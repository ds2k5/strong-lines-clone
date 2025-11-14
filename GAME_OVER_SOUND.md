# Game Over Sound Effect Implementation

## Summary
Added game over sound effect (`assets/negative_beeps-6008.mp3`) that plays when the player loses the game.

## Changes Made

### 1. Audio Manager Enhancement (`src/audio_manager.rs`)
- Added `OutputStreamHandle` to the `AudioManager` struct to enable one-shot sound effects
- Added `play_sound_effect()` method that:
  - Creates a temporary sink for sound effects
  - Plays the sound independently from background music
  - Automatically cleans up after playback
  - Respects the mute setting (won't play if muted)

### 2. Game Over Integration (`src/main.rs`)

#### Two Game Over Scenarios:
1. **Lives depleted** (`check_collisions` function):
   - Plays sound when lives reach 0 after enemy collision
   - Line 604: `audio.manager.play_sound_effect("assets/negative_beeps-6008.mp3");`

2. **Timeout** (`update_level_timer` function):
   - Plays sound when level timer runs out
   - Line 816: `audio.manager.play_sound_effect("assets/negative_beeps-6008.mp3");`

## Technical Details

### Sound Effect Implementation
```rust
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
```

### Key Features:
- ✅ Sound plays independently from background music
- ✅ Respects mute toggle (speaker button)
- ✅ Automatically cleans up after playing
- ✅ Non-blocking (doesn't interfere with game flow)
- ✅ Triggers on both game over conditions (lives depleted and timeout)

## Audio File
- **Location**: `assets/negative_beeps-6008.mp3`
- **Format**: MP3
- **Trigger**: Plays when `game_state.game_over` becomes `true`
- **Scenarios**: 
  - Enemy hits player while drawing (lives reach 0)
  - Level timer expires (timeout)

## Testing
To test the game over sound:
1. Build and run: `cargo run --release`
2. Either:
   - Let enemies hit you while drawing until lives reach 0, OR
   - Wait for the 2-minute level timer to expire
3. The `negative_beeps-6008.mp3` sound will play
4. Test with mute enabled - sound should NOT play when muted
