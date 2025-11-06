// Utility functions

use bevy::prelude::*;
use std::fs;
use std::path::{Path, PathBuf};

/// Load all image files from a directory
pub fn load_images_from_directory<P: AsRef<Path>>(path: P) -> Vec<PathBuf> {
    let mut images = Vec::new();
    
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.flatten() {
            let path = entry.path();
            if let Some(ext) = path.extension() {
                let ext = ext.to_string_lossy().to_lowercase();
                if ext == "png" || ext == "jpg" || ext == "jpeg" {
                    images.push(path);
                }
            }
        }
    }
    
    images
}

/// Load all audio files from a directory
pub fn load_audio_from_directory<P: AsRef<Path>>(path: P) -> Vec<PathBuf> {
    let mut audio_files = Vec::new();
    
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.flatten() {
            let path = entry.path();
            if let Some(ext) = path.extension() {
                let ext = ext.to_string_lossy().to_lowercase();
                if ext == "mp3" || ext == "ogg" || ext == "wav" {
                    audio_files.push(path);
                }
            }
        }
    }
    
    audio_files
}

/// Calculate percentage of area revealed
pub fn calculate_revealed_percentage(
    revealed_pixels: usize,
    total_pixels: usize,
) -> f32 {
    if total_pixels == 0 {
        return 0.0;
    }
    (revealed_pixels as f32 / total_pixels as f32) * 100.0
}

/// Get current timestamp as string
pub fn get_timestamp() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    
    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();
    
    format!("{}", duration.as_secs())
}
