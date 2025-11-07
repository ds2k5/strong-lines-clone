use rodio::{Sink, Decoder, Source};
use rand::seq::SliceRandom;
use std::fs::File;
use std::io::{BufReader, Write, Read};
use std::path::PathBuf;

const CONFIG_FILE: &str = "mute_config.json";

pub struct AudioManager {
    sink: Sink,
    muted: bool,
    sound_files: Vec<PathBuf>,
}

impl AudioManager {
    pub fn new(stream_handle: &rodio::OutputStreamHandle, sound_files: Vec<PathBuf>) -> Self {
        let sink = Sink::try_new(stream_handle).unwrap();
        let muted = Self::load_muted_state().unwrap_or(false);
        let mut manager = Self { sink, muted, sound_files };
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
}

