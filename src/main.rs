use bevy::prelude::*;
use rand::Rng;
use std::collections::HashSet;
use std::fs;

mod audio_manager;
use audio_manager::AudioManager;

const WINDOW_WIDTH: f32 = 800.0;
const WINDOW_HEIGHT: f32 = 600.0;
const GRID_SIZE: usize = 80;
const CELL_SIZE: f32 = WINDOW_WIDTH / GRID_SIZE as f32;
const PLAYER_SPEED: f32 = 200.0;
const BASE_ENEMY_SPEED: f32 = 150.0;
const NUM_ENEMIES: usize = 3;
const LEVEL_DISPLAY_TIME: f32 = 10.0; // Show completed image for 10 seconds

#[derive(Component)]
struct Player {
    is_drawing: bool,
}

#[derive(Component)]
struct Enemy {
    velocity: Vec2,
    bounce_timer: f32, // For visual feedback when bouncing
}

#[derive(Component)]
struct DrawingLine {
    points: Vec<(i32, i32)>,
}

#[derive(Component)]
struct RevealedCell {
    grid_x: usize,
    grid_y: usize,
}

#[derive(Component)]
struct BackgroundSprite;

#[derive(Component)]
struct SpeakerButton;

#[derive(Component)]
struct SpeakerIcon;

// NonSend resource because audio streams cannot be sent between threads
struct AudioResource {
    manager: AudioManager,
    _stream: rodio::OutputStream,
    _stream_handle: rodio::OutputStreamHandle,
}

#[derive(Resource)]
struct GameGrid {
    claimed: [[bool; GRID_SIZE]; GRID_SIZE],
    drawing_path: Vec<(i32, i32)>,
}

#[derive(Resource)]
struct GameState {
    score: u32,
    lives: i32,
    game_over: bool,
    level: u32,
    reveal_threshold: f32, // Percentage needed before image reveals
    level_complete_timer: Option<f32>, // Timer for showing image before next level
    ready_to_advance: bool, // Flag to trigger level advancement
}

#[derive(Resource)]
struct BackgroundImage {
    handle: Handle<Image>,
    revealed_percentage: f32,
    threshold_reached: bool, // Track if we've hit the threshold
    current_image_path: String, // Track which image is currently loaded
}

fn main() {
    // Initialize audio before starting Bevy app
    let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
    
    // Collect all MP3 files from assets/sounds/
    let mut sound_files = Vec::new();
    if let Ok(entries) = fs::read_dir("assets/sounds") {
        for entry in entries.flatten() {
            if let Some(path_str) = entry.path().to_str() {
                if path_str.to_lowercase().ends_with(".mp3") {
                    sound_files.push(entry.path());
                }
            }
        }
    }
    
    let audio_manager = AudioManager::new(&stream_handle, sound_files);
    
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Strong Lines".to_string(),
                resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .insert_non_send_resource(AudioResource {
            manager: audio_manager,
            _stream,
            _stream_handle: stream_handle,
        })
        .insert_resource(GameGrid {
            claimed: {
                let mut grid = [[false; GRID_SIZE]; GRID_SIZE];
                // Mark the edges as claimed (safe starting zone)
                for i in 0..GRID_SIZE {
                    grid[0][i] = true; // Bottom edge
                    grid[GRID_SIZE - 1][i] = true; // Top edge
                    grid[i][0] = true; // Left edge
                    grid[i][GRID_SIZE - 1] = true; // Right edge
                }
                grid
            },
            drawing_path: Vec::new(),
        })
        .insert_resource(GameState {
            score: 0,
            lives: 3,
            game_over: false,
            level: 1,
            reveal_threshold: 10.0, // Level 1: 10%
            level_complete_timer: None,
            ready_to_advance: false,
        })
        .add_systems(Startup, (setup_game, load_random_image, setup_speaker_button))
        .add_systems(Update, (
            player_movement,
            enemy_movement,
            update_enemy_visuals,
            check_collisions,
            draw_grid,
            update_overlay_appearance,
            reveal_background,
            check_level_completion,
            hide_entities_during_completion,
            advance_level,
        ))
        .add_systems(Update, (
            update_ui,
            handle_speaker_button,
            update_speaker_button_appearance,
        ))
        .run();
}

fn setup_game(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    
    // Spawn player at the edge
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgb(0.0, 1.0, 0.0),
                custom_size: Some(Vec2::new(CELL_SIZE, CELL_SIZE)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, -WINDOW_HEIGHT / 2.0 + CELL_SIZE, 1.0),
            ..default()
        },
        Player { is_drawing: false },
    ));
    
    // Spawn enemies
    let mut rng = rand::thread_rng();
    for _ in 0..NUM_ENEMIES {
        let x = rng.gen_range(-WINDOW_WIDTH/4.0..WINDOW_WIDTH/4.0);
        let y = rng.gen_range(-WINDOW_HEIGHT/4.0..WINDOW_HEIGHT/4.0);
        let vx = rng.gen_range(-1.0..1.0);
        let vy = rng.gen_range(-1.0..1.0);
        let velocity = Vec2::new(vx, vy).normalize() * BASE_ENEMY_SPEED;
        
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgb(1.0, 0.0, 0.0),
                    custom_size: Some(Vec2::new(CELL_SIZE * 1.5, CELL_SIZE * 1.5)),
                    ..default()
                },
                transform: Transform::from_xyz(x, y, 0.5),
                ..default()
            },
            Enemy { 
                velocity,
                bounce_timer: 0.0,
            },
        ));
    }
    
    // Spawn UI text
    commands.spawn(
        TextBundle::from_section(
            "Score: 0 | Lives: 3",
            TextStyle {
                font_size: 30.0,
                color: Color::WHITE,
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        }),
    );
}

fn load_random_image(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Dynamically scan and select random image (no exclusion for first level)
    let random_image = get_random_image_path(None);
    
    println!("Loading background image: {}", random_image);
    
    let handle: Handle<Image> = asset_server.load(&random_image);
    
    // Spawn the background image sprite (behind everything at z=-1.0)
    // Scale to fit window size
    commands.spawn((
        SpriteBundle {
            texture: handle.clone(),
            transform: Transform::from_xyz(0.0, 0.0, -1.0),
            sprite: Sprite {
                custom_size: Some(Vec2::new(WINDOW_WIDTH, WINDOW_HEIGHT)),
                ..default()
            },
            ..default()
        },
        BackgroundSprite,
    ));
    
    // Spawn dark overlay sprites for each grid cell (will be removed as areas are claimed)
    // These block the background image until removed
    for y in 0..GRID_SIZE {
        for x in 0..GRID_SIZE {
            let world_x = x as f32 * CELL_SIZE - WINDOW_WIDTH / 2.0 + CELL_SIZE / 2.0;
            let world_y = y as f32 * CELL_SIZE - WINDOW_HEIGHT / 2.0 + CELL_SIZE / 2.0;
            
            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::srgb(0.0, 0.0, 0.0), // Pure black to hide image
                        custom_size: Some(Vec2::new(CELL_SIZE, CELL_SIZE)),
                        ..default()
                    },
                    transform: Transform::from_xyz(world_x, world_y, 0.0),
                    ..default()
                },
                RevealedCell { grid_x: x, grid_y: y },
            ));
        }
    }
    
    commands.insert_resource(BackgroundImage {
        handle,
        revealed_percentage: 0.0,
        threshold_reached: false,
        current_image_path: random_image,
    });
}

fn player_movement(
    mut player_query: Query<(&mut Transform, &mut Player)>,
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut grid: ResMut<GameGrid>,
    game_state: Res<GameState>,
) {
    if game_state.game_over || game_state.level_complete_timer.is_some() {
        return; // Freeze during game over or level completion display
    }
    
    let (mut transform, mut player) = player_query.single_mut();
    let mut direction = Vec2::ZERO;
    
    if keyboard.pressed(KeyCode::ArrowLeft) || keyboard.pressed(KeyCode::KeyA) {
        direction.x -= 1.0;
    }
    if keyboard.pressed(KeyCode::ArrowRight) || keyboard.pressed(KeyCode::KeyD) {
        direction.x += 1.0;
    }
    if keyboard.pressed(KeyCode::ArrowUp) || keyboard.pressed(KeyCode::KeyW) {
        direction.y += 1.0;
    }
    if keyboard.pressed(KeyCode::ArrowDown) || keyboard.pressed(KeyCode::KeyS) {
        direction.y -= 1.0;
    }
    
    if direction.length() > 0.0 {
        direction = direction.normalize();
        let movement = direction * PLAYER_SPEED * time.delta_seconds();
        transform.translation += movement.extend(0.0);
        
        // Clamp to window bounds
        transform.translation.x = transform.translation.x.clamp(
            -WINDOW_WIDTH / 2.0 + CELL_SIZE / 2.0,
            WINDOW_WIDTH / 2.0 - CELL_SIZE / 2.0,
        );
        transform.translation.y = transform.translation.y.clamp(
            -WINDOW_HEIGHT / 2.0 + CELL_SIZE / 2.0,
            WINDOW_HEIGHT / 2.0 - CELL_SIZE / 2.0,
        );
        
        // Check if on edge or in claimed territory
        let grid_x = ((transform.translation.x + WINDOW_WIDTH / 2.0) / CELL_SIZE) as i32;
        let grid_y = ((transform.translation.y + WINDOW_HEIGHT / 2.0) / CELL_SIZE) as i32;
        
        let is_on_edge = grid_x == 0 || grid_x == GRID_SIZE as i32 - 1 
            || grid_y == 0 || grid_y == GRID_SIZE as i32 - 1;
        let is_on_claimed = grid.claimed[grid_y.max(0).min(GRID_SIZE as i32 - 1) as usize]
            [grid_x.max(0).min(GRID_SIZE as i32 - 1) as usize];
        
        if is_on_edge || is_on_claimed {
            if player.is_drawing && !grid.drawing_path.is_empty() {
                // Complete the drawing
                complete_area(&mut grid);
                player.is_drawing = false;
            }
        } else {
            if !player.is_drawing {
                player.is_drawing = true;
                grid.drawing_path.clear();
            }
            grid.drawing_path.push((grid_x, grid_y));
        }
    }
}

fn complete_area(grid: &mut GameGrid) {
    // Mark the drawn path as claimed
    for &(x, y) in &grid.drawing_path {
        if x >= 0 && x < GRID_SIZE as i32 && y >= 0 && y < GRID_SIZE as i32 {
            grid.claimed[y as usize][x as usize] = true;
        }
    }
    
    // Simple flood fill to claim enclosed areas
    let mut to_fill = HashSet::new();
    for &(x, y) in &grid.drawing_path {
        // Check adjacent cells
        for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let nx = x + dx;
            let ny = y + dy;
            if nx >= 0 && nx < GRID_SIZE as i32 && ny >= 0 && ny < GRID_SIZE as i32 {
                if !grid.claimed[ny as usize][nx as usize] {
                    to_fill.insert((nx, ny));
                }
            }
        }
    }
    
    // Simple area fill (this is simplified - a real implementation would be more sophisticated)
    for (x, y) in to_fill {
        grid.claimed[y as usize][x as usize] = true;
    }
    
    grid.drawing_path.clear();
}

fn enemy_movement(
    mut enemy_query: Query<(&mut Transform, &mut Enemy)>,
    time: Res<Time>,
    grid: Res<GameGrid>,
    game_state: Res<GameState>,
) {
    // Freeze enemies during level completion display
    if game_state.level_complete_timer.is_some() {
        return;
    }
    
    // Calculate speed multiplier based on level (10% faster per level)
    let speed_multiplier = 1.0 + (game_state.level - 1) as f32 * 0.1;
    
    for (mut transform, mut enemy) in enemy_query.iter_mut() {
        // Calculate next position with level-based speed
        let scaled_velocity = enemy.velocity * speed_multiplier;
        let next_pos = transform.translation + scaled_velocity.extend(0.0) * time.delta_seconds();
        
        // Check grid position for next location
        let next_grid_x = ((next_pos.x + WINDOW_WIDTH / 2.0) / CELL_SIZE) as i32;
        let next_grid_y = ((next_pos.y + WINDOW_HEIGHT / 2.0) / CELL_SIZE) as i32;
        
        let mut should_bounce_x = false;
        let mut should_bounce_y = false;
        
        // Check if hitting left or right walls
        if next_pos.x <= -WINDOW_WIDTH / 2.0 + CELL_SIZE {
            should_bounce_x = true;
        }
        if next_pos.x >= WINDOW_WIDTH / 2.0 - CELL_SIZE {
            should_bounce_x = true;
        }
        
        // Check if hitting top or bottom walls
        if next_pos.y <= -WINDOW_HEIGHT / 2.0 + CELL_SIZE {
            should_bounce_y = true;
        }
        if next_pos.y >= WINDOW_HEIGHT / 2.0 - CELL_SIZE {
            should_bounce_y = true;
        }
        
        // Check if hitting claimed areas (uncovered parts)
        if next_grid_x >= 0 && next_grid_x < GRID_SIZE as i32 && next_grid_y >= 0 && next_grid_y < GRID_SIZE as i32 {
            if grid.claimed[next_grid_y as usize][next_grid_x as usize] {
                // Determine which direction caused the collision
                let current_grid_x = ((transform.translation.x + WINDOW_WIDTH / 2.0) / CELL_SIZE) as i32;
                let current_grid_y = ((transform.translation.y + WINDOW_HEIGHT / 2.0) / CELL_SIZE) as i32;
                
                if next_grid_x != current_grid_x {
                    should_bounce_x = true;
                }
                if next_grid_y != current_grid_y {
                    should_bounce_y = true;
                }
            }
        }
        
        // Check if hitting drawing path (active line)
        for &(path_x, path_y) in &grid.drawing_path {
            if next_grid_x == path_x && next_grid_y == path_y {
                let current_grid_x = ((transform.translation.x + WINDOW_WIDTH / 2.0) / CELL_SIZE) as i32;
                let current_grid_y = ((transform.translation.y + WINDOW_HEIGHT / 2.0) / CELL_SIZE) as i32;
                
                if path_x != current_grid_x {
                    should_bounce_x = true;
                }
                if path_y != current_grid_y {
                    should_bounce_y = true;
                }
                break;
            }
        }
        
        // Apply bounces
        if should_bounce_x {
            enemy.velocity.x = -enemy.velocity.x;
            enemy.bounce_timer = 0.2; // Flash for 0.2 seconds
        }
        if should_bounce_y {
            enemy.velocity.y = -enemy.velocity.y;
            enemy.bounce_timer = 0.2; // Flash for 0.2 seconds
        }
        
        // Update bounce timer
        if enemy.bounce_timer > 0.0 {
            enemy.bounce_timer -= time.delta_seconds();
        }
        
        // Move enemy with updated velocity
        transform.translation += scaled_velocity.extend(0.0) * time.delta_seconds();
    }
}

fn check_collisions(
    player_query: Query<(&Transform, &Player)>,
    enemy_query: Query<&Transform, With<Enemy>>,
    mut game_state: ResMut<GameState>,
    mut grid: ResMut<GameGrid>,
) {
    if game_state.game_over {
        return;
    }
    
    let (player_transform, player) = player_query.single();
    
    // Check if drawing and hit by enemy
    if player.is_drawing {
        for enemy_transform in enemy_query.iter() {
            let distance = player_transform.translation.distance(enemy_transform.translation);
            if distance < CELL_SIZE * 2.0 {
                // Hit while drawing!
                game_state.lives -= 1;
                grid.drawing_path.clear();
                
                if game_state.lives <= 0 {
                    game_state.game_over = true;
                }
                return;
            }
        }
    }
}

fn draw_grid(
    mut gizmos: Gizmos,
    grid: Res<GameGrid>,
    bg_image: Res<BackgroundImage>,
) {
    // Only draw grid elements if image hasn't been revealed yet
    if !bg_image.threshold_reached {
        // Draw thin borders around claimed areas (so players know what's safe)
        for y in 0..GRID_SIZE {
            for x in 0..GRID_SIZE {
                if grid.claimed[y][x] {
                    let world_x = x as f32 * CELL_SIZE - WINDOW_WIDTH / 2.0 + CELL_SIZE / 2.0;
                    let world_y = y as f32 * CELL_SIZE - WINDOW_HEIGHT / 2.0 + CELL_SIZE / 2.0;
                    
                    // Draw just the border outline, not a filled rectangle
                    gizmos.rect_2d(
                        Vec2::new(world_x, world_y),
                        0.0,
                        Vec2::new(CELL_SIZE, CELL_SIZE),
                        Color::srgba(0.0, 1.0, 1.0, 0.3), // Cyan outline
                    );
                }
            }
        }
    }
    
    // Always draw current drawing path (bright yellow filled)
    for &(x, y) in &grid.drawing_path {
        let world_x = x as f32 * CELL_SIZE - WINDOW_WIDTH / 2.0 + CELL_SIZE / 2.0;
        let world_y = y as f32 * CELL_SIZE - WINDOW_HEIGHT / 2.0 + CELL_SIZE / 2.0;
        
        gizmos.rect_2d(
            Vec2::new(world_x, world_y),
            0.0,
            Vec2::new(CELL_SIZE, CELL_SIZE),
            Color::srgb(1.0, 1.0, 0.0),
        );
    }
}

fn update_ui(
    mut text_query: Query<&mut Text>,
    game_state: Res<GameState>,
    grid: Res<GameGrid>,
    bg_image: Res<BackgroundImage>,
) {
    let mut claimed_count = 0;
    let total_cells = GRID_SIZE * GRID_SIZE;
    
    for row in &grid.claimed {
        for &cell in row {
            if cell {
                claimed_count += 1;
            }
        }
    }
    
    let percentage = (claimed_count as f32 / total_cells as f32 * 100.0) as u32;
    let speed_multiplier = 1.0 + (game_state.level - 1) as f32 * 0.1;
    
    for mut text in text_query.iter_mut() {
        if game_state.game_over {
            text.sections[0].value = format!(
                "GAME OVER! Final Score: {}% | Level {} | Press R to Restart", 
                percentage, game_state.level
            );
        } else if let Some(timer) = game_state.level_complete_timer {
            // Showing completed full image
            text.sections[0].value = format!(
                "🎊🎊🎊 LEVEL {} COMPLETE! 🎊🎊🎊 | 💯 VIEWING FULL IMAGE 💯 | Next: Level {} in {:.1}s",
                game_state.level, game_state.level + 1, timer
            );
        } else if !bg_image.threshold_reached {
            text.sections[0].value = format!(
                "Level {} | Progress: {}% / {}% to WIN | Lives: {} | Speed: {:.0}% | Uncover {}% to see FULL image!",
                game_state.level, percentage, game_state.reveal_threshold as u32, game_state.lives, speed_multiplier * 100.0, game_state.reveal_threshold as u32
            );
        } else {
            // This shouldn't happen long since level completes at threshold
            text.sections[0].value = format!(
                "Level {} | {}% uncovered | Lives: {} | Speed: {:.0}%",
                game_state.level, percentage, game_state.lives, speed_multiplier * 100.0
            );
        }
    }
}

fn reveal_background(
    mut commands: Commands,
    grid: Res<GameGrid>,
    overlay_query: Query<(Entity, &RevealedCell)>,
    mut bg_image: ResMut<BackgroundImage>,
    game_state: Res<GameState>,
) {
    // Calculate claimed percentage
    let mut claimed_count = 0;
    let total_cells = GRID_SIZE * GRID_SIZE;
    
    for row in &grid.claimed {
        for &cell in row {
            if cell {
                claimed_count += 1;
            }
        }
    }
    
    let percentage = (claimed_count as f32 / total_cells as f32) * 100.0;
    bg_image.revealed_percentage = percentage;
    
    // Check if threshold is reached
    if !bg_image.threshold_reached && percentage >= game_state.reveal_threshold {
        bg_image.threshold_reached = true;
        println!("🎉 Threshold reached at {}%! Revealing ENTIRE image!", percentage);
        
        // Remove ALL overlay sprites to show the complete image
        let mut removed_count = 0;
        for (entity, _cell) in overlay_query.iter() {
            commands.entity(entity).despawn();
            removed_count += 1;
        }
        println!("✅ Removed {} overlay sprites - image should now be fully visible!", removed_count);
    }
}

fn update_enemy_visuals(
    mut enemy_query: Query<(&mut Sprite, &Enemy)>,
) {
    for (mut sprite, enemy) in enemy_query.iter_mut() {
        if enemy.bounce_timer > 0.0 {
            // Flash white when bouncing
            sprite.color = Color::srgb(1.0, 1.0, 1.0);
        } else {
            // Normal red color
            sprite.color = Color::srgb(1.0, 0.0, 0.0);
        }
    }
}

fn check_level_completion(
    mut game_state: ResMut<GameState>,
    bg_image: Res<BackgroundImage>,
    time: Res<Time>,
) {
    // Level is complete when threshold is reached (show whole image)
    if bg_image.threshold_reached && game_state.level_complete_timer.is_none() {
        println!("🎊 Level {} Complete! Showing FULL image for {} seconds...", game_state.level, LEVEL_DISPLAY_TIME);
        println!("👁️  Player and enemies will be hidden so you can see the image clearly!");
        game_state.level_complete_timer = Some(LEVEL_DISPLAY_TIME);
    }
    
    // Count down timer
    if let Some(timer) = game_state.level_complete_timer.as_mut() {
        *timer -= time.delta_seconds();
        
        // Timer finished - set flag to advance to next level
        if *timer <= 0.0 {
            game_state.level_complete_timer = None;
            game_state.ready_to_advance = true;
            println!("⏭️  Advancing to Level {}", game_state.level + 1);
        }
    }
}

fn update_overlay_appearance(
    mut overlay_query: Query<(&mut Sprite, &RevealedCell)>,
    grid: Res<GameGrid>,
    bg_image: Res<BackgroundImage>,
) {
    // Update overlay sprites to show claimed vs unclaimed areas before threshold
    if !bg_image.threshold_reached {
        for (mut sprite, cell) in overlay_query.iter_mut() {
            if grid.claimed[cell.grid_y][cell.grid_x] {
                // Claimed but not revealed yet - darker gray
                sprite.color = Color::srgb(0.15, 0.15, 0.15);
            } else {
                // Not claimed - pure black
                sprite.color = Color::srgb(0.0, 0.0, 0.0);
            }
        }
    }
}

fn advance_level(
    mut game_state: ResMut<GameState>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    overlay_query: Query<Entity, With<RevealedCell>>,
    bg_sprite_query: Query<Entity, With<BackgroundSprite>>,
    mut grid: ResMut<GameGrid>,
    mut bg_image: ResMut<BackgroundImage>,
) {
    // Only advance when ready_to_advance flag is set
    if !game_state.ready_to_advance {
        return;
    }
    
    println!("🔄 Resetting for Level {}...", game_state.level + 1);
    
    // Level up
    game_state.level += 1;
    game_state.reveal_threshold = 10.0 + (game_state.level - 1) as f32 * 2.0;
    game_state.ready_to_advance = false;
    
    // Reset background image state
    bg_image.revealed_percentage = 0.0;
    bg_image.threshold_reached = false;
    
    // Despawn all existing overlay sprites
    for entity in overlay_query.iter() {
        commands.entity(entity).despawn();
    }
    
    // Despawn existing background sprite
    for entity in bg_sprite_query.iter() {
        commands.entity(entity).despawn();
    }
    
    // Reset grid (keep edges claimed)
    grid.claimed = {
        let mut new_grid = [[false; GRID_SIZE]; GRID_SIZE];
        // Mark the edges as claimed (safe starting zone)
        for i in 0..GRID_SIZE {
            new_grid[0][i] = true; // Bottom edge
            new_grid[GRID_SIZE - 1][i] = true; // Top edge
            new_grid[i][0] = true; // Left edge
            new_grid[i][GRID_SIZE - 1] = true; // Right edge
        }
        new_grid
    };
    grid.drawing_path.clear();
    
    // Get previous image path to exclude it
    let previous_image = bg_image.current_image_path.clone();
    
    // Dynamically scan and load new random image (excluding previous one)
    let random_image = get_random_image_path(Some(&previous_image));
    
    println!("Loading new image for level {}: {}", game_state.level, random_image);
    
    let handle: Handle<Image> = asset_server.load(&random_image);
    
    // Update background image resource
    bg_image.handle = handle.clone();
    bg_image.current_image_path = random_image;
    commands.spawn((
        SpriteBundle {
            texture: handle.clone(),
            transform: Transform::from_xyz(0.0, 0.0, -1.0),
            sprite: Sprite {
                custom_size: Some(Vec2::new(WINDOW_WIDTH, WINDOW_HEIGHT)),
                ..default()
            },
            ..default()
        },
        BackgroundSprite,
    ));
    
    // Spawn new dark overlay sprites
    for y in 0..GRID_SIZE {
        for x in 0..GRID_SIZE {
            let world_x = x as f32 * CELL_SIZE - WINDOW_WIDTH / 2.0 + CELL_SIZE / 2.0;
            let world_y = y as f32 * CELL_SIZE - WINDOW_HEIGHT / 2.0 + CELL_SIZE / 2.0;
            
            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::srgb(0.0, 0.0, 0.0),
                        custom_size: Some(Vec2::new(CELL_SIZE, CELL_SIZE)),
                        ..default()
                    },
                    transform: Transform::from_xyz(world_x, world_y, 0.0),
                    ..default()
                },
                RevealedCell { grid_x: x, grid_y: y },
            ));
        }
    }
    
    bg_image.handle = handle;
}

fn hide_entities_during_completion(
    mut player_query: Query<&mut Visibility, (With<Player>, Without<Enemy>)>,
    mut enemy_query: Query<&mut Visibility, With<Enemy>>,
    game_state: Res<GameState>,
) {
    // Hide player and enemies during level completion display
    let should_hide = game_state.level_complete_timer.is_some();
    
    for mut visibility in player_query.iter_mut() {
        *visibility = if should_hide {
            Visibility::Hidden
        } else {
            Visibility::Visible
        };
    }
    
    for mut visibility in enemy_query.iter_mut() {
        *visibility = if should_hide {
            Visibility::Hidden
        } else {
            Visibility::Visible
        };
    }
}

fn get_random_image_path(exclude_path: Option<&str>) -> String {
    let images_dir = "assets/images";
    let mut image_files = Vec::new();
    
    // Try to read the directory
    if let Ok(entries) = fs::read_dir(images_dir) {
        for entry in entries.flatten() {
            if let Ok(file_type) = entry.file_type() {
                if file_type.is_file() {
                    if let Some(path_str) = entry.path().to_str() {
                        let path_lower = path_str.to_lowercase();
                        // Check for valid image extensions
                        if path_lower.ends_with(".png") 
                            || path_lower.ends_with(".jpg") 
                            || path_lower.ends_with(".jpeg") {
                            // Convert to asset path format (relative to assets folder)
                            if let Some(rel_path) = path_str.strip_prefix("assets/") {
                                image_files.push(rel_path.to_string());
                            }
                        }
                    }
                }
            }
        }
    }
    
    // If no images found, return a fallback
    if image_files.is_empty() {
        println!("⚠️  No images found in {}! Please add PNG or JPEG files.", images_dir);
        return "images/placeholder.png".to_string();
    }
    
    // Filter out the excluded path if provided
    if let Some(exclude) = exclude_path {
        image_files.retain(|path| path != exclude);
        
        // If we filtered out all images (only 1 image available), add them back
        if image_files.is_empty() {
            println!("⚠️  Only one image available - cannot avoid repetition");
            if let Ok(entries) = fs::read_dir(images_dir) {
                for entry in entries.flatten() {
                    if let Ok(file_type) = entry.file_type() {
                        if file_type.is_file() {
                            if let Some(path_str) = entry.path().to_str() {
                                let path_lower = path_str.to_lowercase();
                                if path_lower.ends_with(".png") 
                                    || path_lower.ends_with(".jpg") 
                                    || path_lower.ends_with(".jpeg") {
                                    if let Some(rel_path) = path_str.strip_prefix("assets/") {
                                        image_files.push(rel_path.to_string());
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            println!("🚫 Excluding previous image: {}", exclude);
        }
    }
    
    // Select random image
    let mut rng = rand::thread_rng();
    let selected = &image_files[rng.gen_range(0..image_files.len())];
    
    println!("📁 Found {} images in directory", image_files.len());
    println!("🎲 Randomly selected: {}", selected);
    
    selected.clone()
}

fn setup_speaker_button(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Load speaker icon image
    let speaker_image: Handle<Image> = asset_server.load("speaker_icon.png");
    
    // Create speaker button in top-right corner
    commands.spawn((
        ButtonBundle {
            style: Style {
                width: Val::Px(60.0),
                height: Val::Px(60.0),
                position_type: PositionType::Absolute,
                top: Val::Px(10.0),
                right: Val::Px(10.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                border: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            background_color: BackgroundColor(Color::srgba(0.1, 0.2, 0.1, 0.9)), // Dark green when playing
            border_color: BorderColor(Color::srgba(0.0, 1.0, 0.0, 0.8)), // Green border
            ..default()
        },
        SpeakerButton,
    ))
    .with_children(|parent| {
        parent.spawn((
            ImageBundle {
                style: Style {
                    width: Val::Px(40.0),
                    height: Val::Px(40.0),
                    ..default()
                },
                image: UiImage::new(speaker_image),
                ..default()
            },
            SpeakerIcon,
        ));
    });
}

fn handle_speaker_button(
    interaction_query: Query<
        &Interaction,
        (Changed<Interaction>, With<SpeakerButton>),
    >,
    mut audio_resource: NonSendMut<AudioResource>,
) {
    for interaction in interaction_query.iter() {
        if *interaction == Interaction::Pressed {
            audio_resource.manager.toggle_mute();
        }
    }
}

fn update_speaker_button_appearance(
    mut button_query: Query<(&mut BackgroundColor, &mut BorderColor), With<SpeakerButton>>,
    mut icon_query: Query<&mut UiImage, With<SpeakerIcon>>,
    audio_resource: NonSend<AudioResource>,
) {
    // Update button appearance based on mute state
    let is_muted = audio_resource.manager.is_muted();
    
    for (mut bg_color, mut border_color) in button_query.iter_mut() {
        if is_muted {
            *bg_color = BackgroundColor(Color::srgba(0.3, 0.1, 0.1, 0.9)); // Dark red when muted
            *border_color = BorderColor(Color::srgba(1.0, 0.0, 0.0, 0.8)); // Red border
        } else {
            *bg_color = BackgroundColor(Color::srgba(0.1, 0.2, 0.1, 0.9)); // Dark green when playing
            *border_color = BorderColor(Color::srgba(0.0, 1.0, 0.0, 0.8)); // Green border
        }
    }
    
    // Update icon color/tint based on mute state
    for mut ui_image in icon_query.iter_mut() {
        if is_muted {
            ui_image.color = Color::srgba(1.0, 0.3, 0.3, 1.0); // Red tint when muted
        } else {
            ui_image.color = Color::srgba(0.3, 1.0, 0.3, 1.0); // Green tint when playing
        }
    }
}
