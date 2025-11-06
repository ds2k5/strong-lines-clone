// Components module - ECS components for game entities

use bevy::prelude::*;

// Player component
#[derive(Component)]
pub struct Player {
    pub speed: f32,
    pub is_drawing: bool,
}

// Enemy component
#[derive(Component)]
pub struct Enemy {
    pub velocity: Vec2,
    pub speed: f32,
    pub enemy_type: EnemyType,
}

#[derive(Clone, Copy, PartialEq)]
pub enum EnemyType {
    Bouncer,    // Bounces off walls
    EdgeFollower, // Follows edges of revealed areas
}

// Drawing line component
#[derive(Component)]
pub struct DrawingLine {
    pub start_pos: Vec2,
    pub points: Vec<Vec2>, // All points in the line
}

// Line segment marker
#[derive(Component)]
pub struct LineSegment;

// Revealed area component (for the uncovered parts)
#[derive(Component)]
pub struct RevealedArea;

// Covered area component (black overlay)
#[derive(Component)]
pub struct CoveredArea;

// Border marker component
#[derive(Component)]
pub struct BorderMarker;

// Background image marker component
#[derive(Component)]
pub struct BackgroundImage;
