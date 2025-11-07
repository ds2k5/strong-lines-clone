# Strong Lines - Audio Controls Quick Guide

```
┌────────────────────────────────────────────────────────┐
│ Level 1 | Progress: 5% / 10% to WIN | Lives: 3    [🔊] │  ← Speaker Button
│                                                          │     (Top-Right)
│                                                          │
│                                                          │
│                    GAME AREA                             │
│                 (Background Image)                       │
│                                                          │
│                                                          │
│                                                          │
│                                                          │
└────────────────────────────────────────────────────────┘
```

## How to Use

### Toggle Sound
**Click the speaker icon** in the top-right corner:

**Sound ON (Playing):**
- 🟢 Green background
- 🟢 Green border  
- 🟢 Green-tinted speaker icon
- Music is playing

**Sound OFF (Muted):**
- 🔴 Red background
- 🔴 Red border
- 🔴 Red-tinted speaker icon
- Music is muted

### Your Setting is Saved
When you quit and restart the game, your last mute/unmute choice is remembered!

### Adding More Music
1. Drop MP3 files into: `assets/sounds/`
2. Game will randomly pick one to play each time you start

### Current Music Files
- Aetheric - Coconut Kind of Love
- Lukrembo - Donut

---

## Visual Design

**Button Specifications:**
- Size: 60x60 pixels
- Icon: 40x40 pixels speaker symbol (from Pixabay)
- Position: 10px from top, 10px from right
- Border: 2px thick (color changes with state)
- Always visible over gameplay

**Color States:**
- **Green Theme** = Sound is playing
- **Red Theme** = Sound is muted

---

## Technical Notes

**Assets:**
- Speaker icon: `assets/speaker_icon.png` (from Pixabay)
- Config file: `mute_config.json` (in game directory)
  - Contains: `true` (muted) or `false` (playing)
  - Automatically created on first run

**Features:**
- Image tinting for visual feedback
- Color-coded borders and backgrounds
- Persistent settings across sessions
- Random MP3 selection from assets/sounds/
