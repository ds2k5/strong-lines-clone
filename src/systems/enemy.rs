// Enemy AI and movement systems

use bevy::prelude::*;
use crate::components::Enemy;
use crate::states::GameState;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (
                enemy_movement,
                enemy_collision,
            ).run_if(in_state(GameState::Playing)));
    }
}

fn enemy_movement(
    mut enemy_query: Query<(&mut Transform, &mut Enemy)>,
    time: Res<Time>,
) {
    const HALF_WIDTH: f32 = 960.0;  // 1920 / 2
    const HALF_HEIGHT: f32 = 540.0; // 1080 / 2
    
    for (mut transform, mut enemy) in &mut enemy_query {
        // Move enemy - velocity already includes speed
        transform.translation.x += enemy.velocity.x * time.delta_secs();
        transform.translation.y += enemy.velocity.y * time.delta_secs();
        
        // Bounce off walls
        if transform.translation.x >= HALF_WIDTH || transform.translation.x <= -HALF_WIDTH {
            enemy.velocity.x *= -1.0;
            // Clamp position to prevent getting stuck outside bounds
            transform.translation.x = transform.translation.x.clamp(-HALF_WIDTH, HALF_WIDTH);
        }
        
        if transform.translation.y >= HALF_HEIGHT || transform.translation.y <= -HALF_HEIGHT {
            enemy.velocity.y *= -1.0;
            // Clamp position to prevent getting stuck outside bounds
            transform.translation.y = transform.translation.y.clamp(-HALF_HEIGHT, HALF_HEIGHT);
        }
    }
}

fn enemy_collision(
    mut commands: Commands,
    enemy_query: Query<&Transform, With<Enemy>>,
    player_query: Query<&Transform, With<crate::components::Player>>,
    line_query: Query<(Entity, &Transform, &crate::components::LineSegment)>,
    drawing_line_query: Query<&crate::components::DrawingLine>,
    mut game_data: ResMut<crate::resources::GameData>,
    mut next_state: ResMut<NextState<crate::states::GameState>>,
) {
    // Check collision with player while drawing
    if let Ok(player_transform) = player_query.single() {
        let player_pos = Vec2::new(player_transform.translation.x, player_transform.translation.y);
        
        // Only check collision if player is drawing (off border)
        let is_drawing = !is_on_border_pos(player_pos);
        
        if is_drawing {
            for enemy_transform in &enemy_query {
                let enemy_pos = Vec2::new(enemy_transform.translation.x, enemy_transform.translation.y);
                let distance = player_pos.distance(enemy_pos);
                
                // If enemy hits player while drawing, lose a life
                if distance < 20.0 { // Player size (20) + enemy size (15) / 2
                    game_data.lives = game_data.lives.saturating_sub(1);
                    
                    // Despawn all drawn line segments
                    for (entity, _, _) in &line_query {
                        commands.entity(entity).despawn();
                    }
                    
                    // Despawn drawing line marker
                    for _entity in drawing_line_query.iter() {
                        // This is handled by the line query
                    }
                    
                    // Check if game over
                    if game_data.lives == 0 {
                        next_state.set(crate::states::GameState::GameOver);
                    }
                    
                    return; // Exit after hit
                }
            }
        }
    }
    
    // Check collision with completed line segments
    for enemy_transform in &enemy_query {
        let enemy_pos = Vec2::new(enemy_transform.translation.x, enemy_transform.translation.y);
        
        for (_entity, line_transform, _) in &line_query {
            // Simple AABB collision for line segments
            if let Some(size) = line_transform.scale.truncate().try_into().ok() {
                let line_pos = Vec2::new(line_transform.translation.x, line_transform.translation.y);
                let half_size: Vec2 = size;
                
                // Check if enemy overlaps with line segment
                let collision = enemy_pos.x >= line_pos.x - half_size.x / 2.0 
                    && enemy_pos.x <= line_pos.x + half_size.x / 2.0
                    && enemy_pos.y >= line_pos.y - half_size.y / 2.0
                    && enemy_pos.y <= line_pos.y + half_size.y / 2.0;
                
                if collision {
                    // Enemy hit a completed line - lose a life
                    game_data.lives = game_data.lives.saturating_sub(1);
                    
                    // Despawn all line segments
                    for (e, _, _) in &line_query {
                        commands.entity(e).despawn();
                    }
                    
                    if game_data.lives == 0 {
                        next_state.set(crate::states::GameState::GameOver);
                    }
                    
                    return;
                }
            }
        }
    }
}

fn is_on_border_pos(pos: Vec2) -> bool {
    const HALF_WIDTH: f32 = 960.0;
    const HALF_HEIGHT: f32 = 540.0;
    const BORDER_THRESHOLD: f32 = 10.0;
    
    pos.x >= HALF_WIDTH - BORDER_THRESHOLD || pos.x <= -HALF_WIDTH + BORDER_THRESHOLD ||
    pos.y >= HALF_HEIGHT - BORDER_THRESHOLD || pos.y <= -HALF_HEIGHT + BORDER_THRESHOLD
}
