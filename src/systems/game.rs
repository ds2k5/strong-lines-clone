// Game logic systems

use bevy::prelude::*;
use crate::resources::GameData;
use crate::states::GameState;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (
                update_timer,
                check_win_condition,
                check_lose_condition,
            ).run_if(in_state(GameState::Playing)));
    }
}

fn update_timer(
    mut game_data: ResMut<GameData>,
    time: Res<Time>,
) {
    game_data.time_elapsed += time.delta_secs();
}

fn check_win_condition(
    _game_data: Res<GameData>,
    _next_state: ResMut<NextState<GameState>>,
) {
    // TODO: Check if revealed percentage >= target
    // If yes, transition to LevelComplete state
}

fn check_lose_condition(
    game_data: Res<GameData>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if game_data.lives == 0 {
        next_state.set(GameState::GameOver);
    }
}
