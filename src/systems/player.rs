// Player movement and line drawing systems

use bevy::prelude::*;
use crate::components::{Player, DrawingLine, LineSegment};
use crate::states::GameState;

pub struct PlayerPlugin;

const HALF_WIDTH: f32 = 960.0;  // 1920 / 2
const HALF_HEIGHT: f32 = 540.0; // 1080 / 2
const BORDER_THRESHOLD: f32 = 10.0; // Distance from edge to be considered "on border"

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (
                player_movement,
                player_line_drawing,
            ).run_if(in_state(GameState::Playing)));
    }
}

fn is_on_border(pos: Vec2) -> bool {
    pos.x >= HALF_WIDTH - BORDER_THRESHOLD || pos.x <= -HALF_WIDTH + BORDER_THRESHOLD ||
    pos.y >= HALF_HEIGHT - BORDER_THRESHOLD || pos.y <= -HALF_HEIGHT + BORDER_THRESHOLD
}

fn player_movement(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&mut Transform, &mut Player)>,
    time: Res<Time>,
) {
    for (mut transform, player) in &mut player_query {
        let mut direction = Vec3::ZERO;
        
        if keyboard.pressed(KeyCode::ArrowUp) {
            direction.y += 1.0;
        }
        if keyboard.pressed(KeyCode::ArrowDown) {
            direction.y -= 1.0;
        }
        if keyboard.pressed(KeyCode::ArrowLeft) {
            direction.x -= 1.0;
        }
        if keyboard.pressed(KeyCode::ArrowRight) {
            direction.x += 1.0;
        }
        
        if direction.length() > 0.0 {
            direction = direction.normalize();
            let new_pos = transform.translation + direction * player.speed * time.delta_secs();
            
            // Clamp to screen bounds
            transform.translation.x = new_pos.x.clamp(-HALF_WIDTH, HALF_WIDTH);
            transform.translation.y = new_pos.y.clamp(-HALF_HEIGHT, HALF_HEIGHT);
        }
    }
}

fn player_line_drawing(
    mut commands: Commands,
    mut player_query: Query<(&Transform, &mut Player)>,
    mut current_line_query: Query<(Entity, &mut DrawingLine)>,
) {
    for (transform, mut player) in &mut player_query {
        let pos = Vec2::new(transform.translation.x, transform.translation.y);
        let on_border = is_on_border(pos);
        
        // If player just moved onto border, finish any current line
        if on_border {
            if player.is_drawing {
                player.is_drawing = false;
                // Despawn current line entity
                for (entity, _) in &current_line_query {
                    commands.entity(entity).despawn();
                }
            }
        } else {
            // Player is in play area
            if !player.is_drawing {
                // Start a new line
                player.is_drawing = true;
                commands.spawn(DrawingLine {
                    start_pos: pos,
                    points: vec![pos],
                });
            } else {
                // Continue drawing line
                for (_, mut line) in &mut current_line_query {
                    // Add point if moved enough distance
                    if let Some(last_point) = line.points.last() {
                        if last_point.distance(pos) > 5.0 {
                            line.points.push(pos);
                            
                            // Spawn visual line segment
                            if line.points.len() >= 2 {
                                let start = line.points[line.points.len() - 2];
                                let end = pos;
                                spawn_line_segment(&mut commands, start, end);
                            }
                        }
                    }
                }
            }
        }
    }
}

fn spawn_line_segment(commands: &mut Commands, start: Vec2, end: Vec2) {
    let midpoint = (start + end) / 2.0;
    let direction = end - start;
    let length = direction.length();
    let angle = direction.y.atan2(direction.x);
    
    commands.spawn((
        LineSegment,
        Sprite {
            color: Color::srgb(0.0, 1.0, 0.0), // Green line
            custom_size: Some(Vec2::new(length, 3.0)),
            ..default()
        },
        Transform::from_xyz(midpoint.x, midpoint.y, 9.0)
            .with_rotation(Quat::from_rotation_z(angle)),
        Visibility::default(),
    ));
}
