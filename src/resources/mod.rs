// Resources module - global game resources

use bevy::prelude::*;
use std::path::PathBuf;

// Game configuration
#[derive(Resource)]
pub struct GameConfig {
    pub target_percentage: f32, // Percentage needed to reveal to win
    pub starting_lives: u32,
    pub window_width: f32,
    pub window_height: f32,
}

impl Default for GameConfig {
    fn default() -> Self {
        Self {
            target_percentage: 75.0,
            starting_lives: 3,
            window_width: 1920.0,
            window_height: 1080.0,
        }
    }
}

// Current game state data
#[derive(Resource)]
pub struct GameData {
    pub score: u32,
    pub lives: u32,
    pub revealed_percentage: f32,
    pub current_level: usize,
    pub time_elapsed: f32,
}

impl Default for GameData {
    fn default() -> Self {
        Self {
            score: 0,
            lives: 3,
            revealed_percentage: 0.0,
            current_level: 0,
            time_elapsed: 0.0,
        }
    }
}

impl GameData {
    pub fn reset(&mut self) {
        self.score = 0;
        self.lives = 3;
        self.revealed_percentage = 0.0;
        self.time_elapsed = 0.0;
    }
}

// Image assets resource
#[derive(Resource, Default)]
pub struct ImageAssets {
    pub background_images: Vec<Handle<Image>>,
    pub current_index: usize,
}

// Audio assets resource
#[derive(Resource, Default)]
pub struct AudioAssets {
    pub background_tracks: Vec<Handle<AudioSource>>,
    pub current_track_index: usize,
    pub is_muted: bool,
}

// Background images resource
#[derive(Resource)]
pub struct BackgroundImages {
    pub images: Vec<PathBuf>,
    pub current_index: usize,
}

// Audio resource
#[derive(Resource)]
pub struct AudioState {
    pub is_muted: bool,
    pub current_track_index: usize,
    pub available_tracks: Vec<PathBuf>,
}
