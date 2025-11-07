# Background Music Feature Implementation

## Overview
Added random MP3 playback with persistent mute settings to the Strong Lines game.

## Features Implemented

### 1. Random MP3 Playback
- Automatically scans `assets/sounds/` directory for all MP3 files
- Randomly selects and plays one MP3 file on loop
- Currently includes:
  - `Aetheric - Coconut Kind of Love (freetouse.com).mp3`
  - `Lukrembo - Donut (freetouse.com).mp3`

### 2. Speaker Button UI
- Added a speaker icon button in the top-right corner (🔊/🔇)
- Click to toggle between muted and unmuted states
- Visual feedback:
  - 🔊 when sound is playing
  - 🔇 when sound is muted
- Semi-transparent background for better visibility

### 3. Persistent Settings
- Mute state is saved to `mute_config.json` in the game directory
- Setting is automatically loaded on next game startup
- Your preference persists across sessions

## Technical Details

### Files Modified
1. **Cargo.toml**
   - Added `rodio = "0.19"` dependency for audio playback

2. **src/main.rs**
   - Imported audio_manager module
   - Added `AudioResource` as NonSend resource (audio can't be sent between threads)
   - Created `SpeakerButton` component
   - Added three new systems:
     - `setup_speaker_button` - Creates the UI button
     - `handle_speaker_button` - Handles click events
     - `update_speaker_button_appearance` - Updates icon based on mute state

3. **src/audio_manager.rs** (already existed)
   - Manages audio playback with rodio
   - Handles random file selection
   - Implements mute toggle with persistent storage

### How It Works
1. On game startup:
   - Scans `assets/sounds/` for MP3 files
   - Loads previous mute setting from `mute_config.json`
   - Randomly selects and plays an MP3 on loop
   - Applies saved mute state

2. During gameplay:
   - Speaker button is always visible in top-right corner
   - Clicking toggles mute/unmute
   - State is saved immediately
   - Icon updates to reflect current state

## Adding More Music
Simply drop MP3 files into `assets/sounds/` directory - they will be automatically discovered and added to the random rotation!

## Known Limitations
- Only MP3 format is supported
- Music doesn't change between tracks (loops single selection)
- No volume control (only mute/unmute)
