// Audio systems

use bevy::prelude::*;
use crate::resources::AudioAssets;
use crate::states::GameState;

pub struct AudioPlugin;

#[derive(Resource)]
struct MusicController {
    current_entity: Option<Entity>,
}

impl Default for MusicController {
    fn default() -> Self {
        Self {
            current_entity: None,
        }
    }
}

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<MusicController>()
            .add_systems(Startup, load_audio)
            .add_systems(OnEnter(GameState::Playing), start_background_music)
            .add_systems(OnExit(GameState::Playing), stop_background_music)
            .add_systems(Update, (
                handle_audio_input,
            ).run_if(in_state(GameState::Playing)));
    }
}

fn load_audio(
    asset_server: Res<AssetServer>,
    mut audio_assets: ResMut<AudioAssets>,
) {
    // Load audio files
    let audio_paths = vec![
        "sounds/Aetheric - Coconut Kind of Love (freetouse.com).mp3",
        "sounds/Lukrembo - Donut (freetouse.com).mp3",
    ];
    
    for path in audio_paths {
        let handle: Handle<AudioSource> = asset_server.load(path);
        audio_assets.background_tracks.push(handle);
    }
}

fn start_background_music(
    mut commands: Commands,
    audio_assets: Res<AudioAssets>,
    mut music_controller: ResMut<MusicController>,
) {
    if !audio_assets.background_tracks.is_empty() && !audio_assets.is_muted {
        let entity = commands.spawn((
            AudioPlayer::new(audio_assets.background_tracks[audio_assets.current_track_index].clone()),
            PlaybackSettings::LOOP.with_volume(bevy::audio::Volume::Linear(0.5)),
        )).id();
        
        music_controller.current_entity = Some(entity);
    }
}

fn stop_background_music(
    mut commands: Commands,
    music_controller: Res<MusicController>,
) {
    if let Some(entity) = music_controller.current_entity {
        commands.entity(entity).despawn();
    }
}

fn handle_audio_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut audio_assets: ResMut<AudioAssets>,
    mut commands: Commands,
    mut music_controller: ResMut<MusicController>,
    audio_sinks: Query<&AudioSink>,
) {
    // M for mute/unmute
    if keyboard.just_pressed(KeyCode::KeyM) {
        audio_assets.is_muted = !audio_assets.is_muted;
        
        if let Some(entity) = music_controller.current_entity {
            if let Ok(sink) = audio_sinks.get(entity) {
                if audio_assets.is_muted {
                    sink.pause();
                } else {
                    sink.play();
                }
            }
        }
    }
    
    // N for next track
    if keyboard.just_pressed(KeyCode::KeyN) {
        if !audio_assets.background_tracks.is_empty() {
            audio_assets.current_track_index = 
                (audio_assets.current_track_index + 1) % audio_assets.background_tracks.len();
            
            // Stop current and play new track
            if let Some(entity) = music_controller.current_entity {
                commands.entity(entity).despawn();
            }
            
            if !audio_assets.is_muted {
                let entity = commands.spawn((
                    AudioPlayer::new(audio_assets.background_tracks[audio_assets.current_track_index].clone()),
                    PlaybackSettings::LOOP.with_volume(bevy::audio::Volume::Linear(0.5)),
                )).id();
                
                music_controller.current_entity = Some(entity);
            }
        }
    }
    
    // P for previous track
    if keyboard.just_pressed(KeyCode::KeyP) {
        if !audio_assets.background_tracks.is_empty() {
            if audio_assets.current_track_index == 0 {
                audio_assets.current_track_index = audio_assets.background_tracks.len() - 1;
            } else {
                audio_assets.current_track_index -= 1;
            }
            
            // Stop current and play new track
            if let Some(entity) = music_controller.current_entity {
                commands.entity(entity).despawn();
            }
            
            if !audio_assets.is_muted {
                let entity = commands.spawn((
                    AudioPlayer::new(audio_assets.background_tracks[audio_assets.current_track_index].clone()),
                    PlaybackSettings::LOOP.with_volume(bevy::audio::Volume::Linear(0.5)),
                )).id();
                
                music_controller.current_entity = Some(entity);
            }
        }
    }
}
