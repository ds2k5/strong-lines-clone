use bevy::prelude::*;

/// Game state management
#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum GameState {
    #[default]
    MainMenu,
    Settings,
    Tutorial,
    Playing,
    Paused,
    LevelComplete,
    GameOver,
    HighScores,
}
