// UI systems

use bevy::prelude::*;
use crate::resources::GameData;
use crate::states::GameState;

pub struct UIPlugin;

#[derive(Component)]
struct MainMenuUI;

#[derive(Component)]
struct GameUI;

#[derive(Component)]
struct ScoreText;

#[derive(Component)]
struct LivesText;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::MainMenu), setup_main_menu)
            .add_systems(OnExit(GameState::MainMenu), cleanup_main_menu)
            .add_systems(OnEnter(GameState::Playing), setup_game_ui)
            .add_systems(OnExit(GameState::Playing), cleanup_game_ui)
            .add_systems(Update, (
                update_score_display,
                update_lives_display,
            ).run_if(in_state(GameState::Playing)));
    }
}

fn setup_main_menu(
    mut commands: Commands,
) {
    // Title
    commands.spawn((
        MainMenuUI,
        Text::new("STRONG LINES"),
        TextFont {
            font_size: 80.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(660.0),
            top: Val::Px(200.0),
            ..default()
        },
    ));
    
    // Instructions
    commands.spawn((
        MainMenuUI,
        Text::new("Press ENTER to Start\nPress H for Highscores\nPress S for Settings\nPress T for Tutorial"),
        TextFont {
            font_size: 30.0,
            ..default()
        },
        TextColor(Color::srgb(0.8, 0.8, 0.8)),
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(650.0),
            top: Val::Px(400.0),
            ..default()
        },
    ));
}

fn cleanup_main_menu(
    mut commands: Commands,
    query: Query<Entity, With<MainMenuUI>>,
) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

fn setup_game_ui(
    mut commands: Commands,
) {
    // Score display
    commands.spawn((
        GameUI,
        ScoreText,
        Text::new("Score: 0"),
        TextFont {
            font_size: 30.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(20.0),
            top: Val::Px(20.0),
            ..default()
        },
    ));
    
    // Lives display
    commands.spawn((
        GameUI,
        LivesText,
        Text::new("Lives: 3"),
        TextFont {
            font_size: 30.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(20.0),
            top: Val::Px(60.0),
            ..default()
        },
    ));
    
    // Controls hint
    commands.spawn((
        GameUI,
        Text::new("Arrow Keys: Move | ESC: Pause | M: Mute | N: Next Track | P: Prev Track"),
        TextFont {
            font_size: 20.0,
            ..default()
        },
        TextColor(Color::srgb(0.7, 0.7, 0.7)),
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(20.0),
            bottom: Val::Px(20.0),
            ..default()
        },
    ));
}

fn cleanup_game_ui(
    mut commands: Commands,
    query: Query<Entity, With<GameUI>>,
) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

fn update_score_display(
    game_data: Res<GameData>,
    mut query: Query<&mut Text, With<ScoreText>>,
) {
    for mut text in &mut query {
        **text = format!("Score: {} | {}%", game_data.score, game_data.revealed_percentage as u32);
    }
}

fn update_lives_display(
    game_data: Res<GameData>,
    mut query: Query<&mut Text, With<LivesText>>,
) {
    for mut text in &mut query {
        **text = format!("Lives: {}", game_data.lives);
    }
}
