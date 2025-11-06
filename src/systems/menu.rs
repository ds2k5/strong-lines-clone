// Menu systems

use bevy::prelude::*;
use crate::states::GameState;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (
                main_menu_input,
            ).run_if(in_state(GameState::MainMenu)))
            .add_systems(Update, (
                pause_menu_input,
            ).run_if(in_state(GameState::Paused)))
            .add_systems(Update, (
                check_pause_input,
            ).run_if(in_state(GameState::Playing)));
    }
}

fn main_menu_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard.just_pressed(KeyCode::Enter) {
        next_state.set(GameState::Playing);
    }
    
    if keyboard.just_pressed(KeyCode::KeyH) {
        next_state.set(GameState::HighScores);
    }
    
    if keyboard.just_pressed(KeyCode::KeyS) {
        next_state.set(GameState::Settings);
    }
    
    if keyboard.just_pressed(KeyCode::KeyT) {
        next_state.set(GameState::Tutorial);
    }
}

fn pause_menu_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) || keyboard.just_pressed(KeyCode::Enter) {
        next_state.set(GameState::Playing);
    }
    
    if keyboard.just_pressed(KeyCode::KeyQ) {
        next_state.set(GameState::MainMenu);
    }
}

// Add pause handling during gameplay
fn check_pause_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
    current_state: Res<State<GameState>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) && *current_state.get() == GameState::Playing {
        next_state.set(GameState::Paused);
    }
}
