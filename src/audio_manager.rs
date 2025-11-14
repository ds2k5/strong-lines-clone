use rodio::{Sink, Decoder, Source, OutputStreamHandle};
use rand::seq::SliceRandom;
use std::fs::File;
use std::io::{BufReader, Write, Read};
use std::path::PathBuf;

const CONFIG_FILE: &str = "mute_config.json";

pub struct AudioManager {
    sink: Sink,
    muted: bool,
    sound_files: Vec<PathBuf>,
    stream_handle: OutputStreamHandle,
}

impl AudioManager {
    pub fn new(stream_handle: &rodio::OutputStreamHandle, sound_files: Vec<PathBuf>) -> Self {
        let sink = Sink::try_new(stream_handle).unwrap();
        let muted = Self::load_muted_state().unwrap_or(false);
        let mut manager = Self { 
            sink, 
            muted, 
            sound_files,
            stream_handle: stream_handle.clone(),
        };
        manager.play_random_sound();
        if manager.muted {
            manager.sink.pause();
        }
        manager
    }

    fn load_muted_state() -> Option<bool> {
        let mut file = File::open(CONFIG_FILE).ok()?;
        let mut content = String::new();
        file.read_to_string(&mut content).ok()?;
        Some(content.trim() == "true")
    }

    fn save_muted_state(&self) {
        let mut file = File::create(CONFIG_FILE).expect("Failed to create config file");
        let _ = file.write_all(if self.muted { b"true" } else { b"false" });
    }

    pub fn play_random_sound(&mut self) {
        if self.sound_files.is_empty() { return; }
        let chosen = self.sound_files.choose(&mut rand::thread_rng()).unwrap();
        let file = BufReader::new(File::open(chosen).expect("Failed to open sound file"));
        let source = Decoder::new(file).expect("Failed to decode").repeat_infinite();
        self.sink.append(source);
    }

    pub fn toggle_mute(&mut self) {
        self.muted = !self.muted;
        if self.muted {
            self.sink.pause();
        } else {
            self.sink.play();
        }
        self.save_muted_state();
    }

    pub fn is_muted(&self) -> bool {
        self.muted
    }

    pub fn play_sound_effect(&self, sound_path: &str) {
        if self.muted {
            return; // Don't play sound effects when muted
        }
        
        // Create a new sink for the sound effect (one-shot)
        if let Ok(effect_sink) = Sink::try_new(&self.stream_handle) {
            if let Ok(file) = File::open(sound_path) {
                let file = BufReader::new(file);
                if let Ok(source) = Decoder::new(file) {
                    effect_sink.append(source);
                    effect_sink.detach(); // Let it play independently and cleanup when done
                }
            }
        }
    }
}

