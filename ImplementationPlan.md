# Strong Lines - Implementation Plan

## Project Overview
A Qix-style puzzle game clone built with Bevy 0.17.2 where players draw lines to uncover hidden images while avoiding bouncing enemies.

## Technical Specifications

### Engine & Dependencies
- **Bevy Version**: 0.17.2
- **Rust Edition**: 2021
- **Target Platform**: Desktop (Linux, Windows, MacOS)
- **Resolution**: 1920x1080 (configurable)
- **Window Mode**: Toggleable fullscreen (F11)

### Core Dependencies
```toml
bevy = "0.17.2"
bevy_kira_audio = "0.21"  # Audio management
rusqlite = "0.32"          # SQLite for highscores
rand = "0.8"               # Random selection
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

## Game Architecture

### 1. Game States (Bevy States)
```rust
#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
enum GameState {
    #[default]
    MainMenu,
    Settings,
    Tutorial,
    Playing,
    Paused,
    GameOver,
    Highscores,
}
```

### 2. Core Systems

#### 2.1 Main Menu System
- **Components**: Logo, Play Button, Settings, Highscores, Exit
- **Features**:
  - Animated gradient background
  - Keyboard navigation (Up/Down/Enter)
  - Mouse click support
  - Display current highscore

#### 2.2 Settings System
- **Audio Settings**:
  - Music selection from available tracks
  - Volume slider (0-100%)
  - Mute toggle
  - Sound effects volume
- **Graphics Settings**:
  - Fullscreen toggle
  - Resolution selection
- **Controls Display**:
  - Show key bindings
  - Cannot be changed (arrow keys fixed)

#### 2.3 Tutorial System
- **Interactive Level**:
  - Safe practice area (no enemies at start)
  - Step-by-step instructions overlay
  - Introduces mechanics progressively:
    1. Movement with arrow keys
    2. Drawing lines
    3. Completing squares to reveal image
    4. Enemy introduction (one slow enemy)
    5. Lives and scoring explanation
  - "Skip Tutorial" button
  - Completion bonus points
  - Transitions to Level 1 after completion

#### 2.4 Game Core System

##### Player Component
```rust
struct Player {
    position: Vec2,
    is_drawing: bool,
    current_line: Vec<Vec2>,
    lives: u8,
    drawing_speed: f32,
}
```

##### Line Drawing Mechanics
- **Grid-Based Movement**: 10x10 pixel grid for clean lines
- **Drawing State**:
  - Moving on empty space = drawing new line
  - Moving on completed area = safe (no drawing)
  - Cannot cross own drawing line
- **Line Completion**:
  - Must return to safe area or complete a closed shape
  - Auto-fills rectangular regions
  - Calculates percentage revealed

##### Enemy System
```rust
#[derive(Component)]
enum EnemyType {
    Bouncer { velocity: Vec2, speed: f32 },
    EdgeFollower { speed: f32, clockwise: bool },
    Chaser { speed: f32, aggro_range: f32 },
}

struct Enemy {
    enemy_type: EnemyType,
    position: Vec2,
    radius: f32,
}
```

**Progressive Difficulty**:
- **Level 1-3**: 2-3 Bouncers (slow)
- **Level 4-7**: 3-4 Bouncers + 1 EdgeFollower
- **Level 8-12**: 4-5 Bouncers + 2 EdgeFollowers
- **Level 13-15**: 5-6 Bouncers + 2 EdgeFollowers + 1 Chaser

**Enemy Behavior**:
- **Bouncers**: Bounce off walls and completed areas
- **EdgeFollowers**: Follow perimeter of uncovered regions
- **Chasers**: Slowly move toward player when in range
- All enemies kill player on contact with drawing line
- Enemies cannot enter completed (safe) areas

#### 2.5 Scoring System
```rust
struct Score {
    base_points: u32,        // Percentage revealed (1 point per 1%)
    time_bonus: u32,         // Seconds remaining * 10
    life_bonus: u32,         // Lives remaining * 500
    combo_multiplier: f32,   // Consecutive levels without losing life
}

fn calculate_final_score(score: &Score) -> u32 {
    let base = score.base_points + score.time_bonus + score.life_bonus;
    (base as f32 * score.combo_multiplier) as u32
}
```

**Scoring Details**:
- **Base Points**: 1 point per 1% revealed (max 100)
- **Time Bonus**: Remaining seconds × 10
- **Life Bonus**: Remaining lives × 500
- **Combo Multiplier**: 
  - 1.0× base
  - 1.5× after 3 consecutive perfect levels
  - 2.0× after 5 consecutive perfect levels
  - 2.5× after 8+ consecutive perfect levels
- **Win Condition**: Reveal 75% of image
- **Loss Condition**: 0 lives OR time runs out

#### 2.6 Level System
```rust
struct LevelConfig {
    level_number: u8,
    time_limit: f32,        // Seconds
    image_path: String,
    enemy_count: EnemySpawns,
    required_percentage: f32,
}
```

**Level Progression**:
- 15 total levels
- Time limits: 120s (early) → 90s (mid) → 60s (late)
- Required percentage: 75% for all levels
- Images selected randomly from `assets/images/` folder
- No image repeats in same playthrough
- Background music randomly selected per level from `assets/sounds/`

#### 2.7 Visual System

##### Overlay Rendering
```rust
struct RevealOverlay {
    covered_tiles: HashSet<(i32, i32)>,
    tile_size: f32,
    animation_progress: f32,
}
```

**Animated Gradient Overlay**:
- Covered areas: Dark animated gradient (black → dark blue → black)
- Uncovered areas: Show actual background image
- Drawing line: Bright white/yellow trail
- Smooth fade transition when revealing
- Grid pattern subtle overlay (optional toggle)

##### Visual Effects
- **Particle effects** when completing regions
- **Screen shake** when losing a life
- **Pulsing outline** on player position
- **Enemy trails** (fading path behind enemies)
- **Progress bar** showing % revealed
- **Lives display** (heart icons)
- **Timer** (changes color when < 30s)

#### 2.8 Audio System
```rust
struct AudioSettings {
    music_volume: f32,
    sfx_volume: f32,
    current_track: Option<String>,
    music_enabled: bool,
}
```

**Music Management**:
- Load all MP3 files from `assets/sounds/` on startup
- Display list in settings menu
- Random selection per level (if no manual selection)
- Seamless looping
- Fade in/out on transitions

**Sound Effects**:
- Line drawing sound (subtle whoosh)
- Area completion (satisfying "pop")
- Enemy collision (dramatic hit)
- Life lost (negative tone)
- Level complete (victory fanfare)
- Button clicks (UI feedback)

#### 2.9 Asset Management

##### Image Requirements
- **Location**: `assets/images/*.{png,jpg,jpeg}`
- **Resolution Support**: 
  - Automatically scaled to fit 1920×1080
  - Maintains aspect ratio
  - Letterbox/pillarbox if needed
- **Recommended**: High-contrast images work best
- **Minimum**: 1280×720 resolution images

##### Audio Requirements
- **Location**: `assets/sounds/*.mp3`
- **Format**: MP3 (Ogg Vorbis also supported)
- **Recommended**: 
  - Background music: 2-5 minute loops
  - Moderate tempo for game pacing

##### Font Requirements
- **Location**: `assets/fonts/*.ttf`
- **Default**: Will use Bevy's built-in font if none provided
- **Recommended**: Retro/pixel fonts for authentic feel

#### 2.10 Highscore System

##### Database Schema (SQLite)
```sql
CREATE TABLE IF NOT EXISTS highscores (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    player_name TEXT NOT NULL,
    score INTEGER NOT NULL,
    level_reached INTEGER NOT NULL,
    completion_percentage REAL NOT NULL,
    play_date DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_score ON highscores(score DESC);
```

##### Highscore Features
- **Top 10 Display**: Show on main menu and highscore screen
- **Player Name Entry**: On game over (if top 10 score)
- **Stats Tracked**:
  - Final score
  - Highest level reached
  - Total percentage revealed
  - Date/time of play
- **Database Location**: `./highscores.db` (in game directory)
- **Automatic Cleanup**: Keep only top 100 scores

### 3. User Interface

#### 3.1 Main Menu UI
```
┌────────────────────────────────────┐
│         STRONG LINES               │
│                                     │
│         ► Play Game                │
│           Tutorial                 │
│           Settings                 │
│           Highscores               │
│           Exit                     │
│                                     │
│   High Score: 45,320 - Player      │
└────────────────────────────────────┘
```

#### 3.2 In-Game HUD
```
┌────────────────────────────────────┐
│ Lives: ♥♥♥  Level: 5   Time: 1:23  │
│ Score: 12,450    Revealed: 68%     │
│                                     │
│    [GAMEPLAY AREA WITH IMAGE]      │
│                                     │
│ [Progress Bar: ████████░░░░░ 68%]  │
└────────────────────────────────────┘
```

#### 3.3 Pause Menu
- Translucent overlay
- Resume / Settings / Main Menu / Quit
- Current score and stats visible
- ESC key toggles

#### 3.4 Settings Menu
```
Audio Settings:
  Music Volume: [====|====] 50%
  SFX Volume:   [=======|=] 70%
  Current Track: ▼ [Random]
    - track1.mp3
    - track2.mp3
  Mute: [ ]

Graphics:
  Fullscreen: [X] (F11 to toggle)
  Resolution: 1920x1080

Controls: (Read-Only)
  Arrow Keys: Move
  ESC: Pause
  F11: Toggle Fullscreen
```

### 4. Controls

#### Keyboard Mapping
- **Arrow Keys**: Move player (Up/Down/Left/Right)
- **ESC**: Pause game / Back to previous menu
- **F11**: Toggle fullscreen
- **Enter**: Confirm selection in menus
- **Space**: Skip tutorial (in tutorial mode)

#### Menu Navigation
- **Arrow Keys**: Navigate menu items
- **Enter**: Select
- **ESC**: Back
- **Mouse**: Click buttons (alternative input)

### 5. File Structure

```
strong-lines/
├── Cargo.toml
├── Cargo.lock
├── ImplementationPlan.md
├── README.md
├── highscores.db              # Created at runtime
├── assets/
│   ├── images/               # Background images (PNG/JPG)
│   │   ├── PLACE_IMAGES_HERE.txt
│   │   └── (user adds 1920x1080 images)
│   ├── sounds/               # Background music (MP3)
│   │   ├── PLACE_SOUNDS_HERE.txt
│   │   └── (user adds MP3 files)
│   └── fonts/                # Optional custom fonts
│       └── (optional TTF files)
└── src/
    ├── main.rs               # Entry point, app setup
    ├── states.rs             # Game state definitions
    ├── systems/
    │   ├── mod.rs
    │   ├── menu.rs           # Main menu system
    │   ├── settings.rs       # Settings system
    │   ├── tutorial.rs       # Tutorial system
    │   ├── game.rs           # Core game loop
    │   ├── pause.rs          # Pause menu
    │   └── gameover.rs       # Game over screen
    ├── components/
    │   ├── mod.rs
    │   ├── player.rs         # Player component
    │   ├── enemy.rs          # Enemy components
    │   ├── line.rs           # Line drawing
    │   └── overlay.rs        # Reveal overlay
    ├── resources/
    │   ├── mod.rs
    │   ├── score.rs          # Score tracking
    │   ├── level.rs          # Level configuration
    │   ├── assets.rs         # Asset management
    │   └── audio.rs          # Audio settings
    ├── database/
    │   ├── mod.rs
    │   └── highscore.rs      # SQLite operations
    └── utils/
        ├── mod.rs
        ├── collision.rs      # Collision detection
        └── grid.rs           # Grid calculations
```

### 6. Development Phases

#### Phase 1: Foundation (Week 1)
- [ ] Set up Bevy project with dependencies
- [ ] Create game state system
- [ ] Implement basic main menu
- [ ] Create window management (fullscreen toggle)
- [ ] Set up asset loading system

#### Phase 2: Core Gameplay (Week 2)
- [ ] Implement player movement on grid
- [ ] Create line drawing mechanics
- [ ] Implement area reveal calculation
- [ ] Add collision detection
- [ ] Create basic enemy (Bouncer)

#### Phase 3: Visual Systems (Week 3)
- [ ] Implement animated gradient overlay
- [ ] Add image loading and scaling
- [ ] Create reveal animation
- [ ] Implement HUD (lives, score, timer)
- [ ] Add particle effects

#### Phase 4: Enemy AI (Week 4)
- [ ] Implement EdgeFollower enemy
- [ ] Implement Chaser enemy
- [ ] Create progressive difficulty system
- [ ] Balance enemy spawning

#### Phase 5: Audio & Polish (Week 5)
- [ ] Integrate bevy_kira_audio
- [ ] Implement music selection system
- [ ] Add sound effects
- [ ] Create settings menu
- [ ] Volume controls

#### Phase 6: Progression (Week 6)
- [ ] Implement level system
- [ ] Create scoring system
- [ ] Add combo multipliers
- [ ] Implement time limit
- [ ] Create level transitions

#### Phase 7: Tutorial & Database (Week 7)
- [ ] Create interactive tutorial
- [ ] Set up SQLite database
- [ ] Implement highscore system
- [ ] Add player name entry
- [ ] Create highscore display

#### Phase 8: Testing & Refinement (Week 8)
- [ ] Balance difficulty curve
- [ ] Optimize performance
- [ ] Bug fixes
- [ ] Polish UI/UX
- [ ] Playtest and iterate

### 7. Technical Considerations

#### Performance Optimization
- Use Bevy's built-in sprite batching
- Implement spatial hashing for collision detection
- Limit particle count for older hardware
- Use LOD for complex overlays
- Profile regularly with `cargo flamegraph`

#### Collision Detection Strategy
```rust
// Use AABB for line segments
struct AABB {
    min: Vec2,
    max: Vec2,
}

// Check enemy vs drawing line
fn check_collision(enemy: &Enemy, line: &[Vec2]) -> bool {
    // Segment-circle collision
    for i in 0..line.len()-1 {
        if segment_circle_collision(line[i], line[i+1], enemy.position, enemy.radius) {
            return true;
        }
    }
    false
}
```

#### Grid System
- 10×10 pixel tiles for clean movement
- HashSet for fast tile lookup
- Flood fill algorithm for area calculation
- Bresenham's line algorithm for drawing

#### Memory Management
- Texture atlas for UI elements
- Asset streaming for large images
- Unload unused assets between levels
- Connection pool for SQLite

### 8. Testing Strategy

#### Unit Tests
- Grid calculations
- Collision detection
- Score calculation
- Area reveal percentage
- Database operations

#### Integration Tests
- State transitions
- Asset loading
- Audio playback
- Highscore persistence

#### Playtesting Focus Areas
- Difficulty curve balance
- Control responsiveness
- Enemy AI fairness
- Performance on various hardware
- UI clarity

### 9. Known Challenges & Solutions

#### Challenge 1: Area Calculation
**Problem**: Efficiently calculate revealed percentage
**Solution**: Use flood fill with grid tiles, cache results

#### Challenge 2: Enemy-Line Collision
**Problem**: Fast collision detection while player draws
**Solution**: Only check active drawing line, use spatial partitioning

#### Challenge 3: Image Scaling
**Problem**: Various aspect ratios
**Solution**: Letterbox/pillarbox with maintained aspect ratio

#### Challenge 4: Audio Management
**Problem**: Smooth transitions between tracks
**Solution**: Crossfade with bevy_kira_audio tweens

### 10. Future Enhancements (Post-MVP)
- Online leaderboards
- Level editor
- Custom color schemes
- Power-ups (slow time, freeze enemies, shield)
- Cooperative multiplayer
- Achievement system
- Steam integration
- Mobile port (touch controls)

---

## Getting Started

### Prerequisites
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Verify installation
rustc --version
cargo --version
```

### Build & Run
```bash
cd ~/rust/strong-lines
cargo build --release
cargo run --release
```

### Adding Assets
1. Place 1920×1080 images in `assets/images/`
2. Place MP3 music files in `assets/sounds/`
3. (Optional) Add TTF fonts to `assets/fonts/`

### Development Mode
```bash
# Fast compile for testing
cargo run

# With performance profiling
cargo install flamegraph
cargo flamegraph
```

---

## License Compliance
- Use only license-free/public domain images and music
- Recommended sources:
  - **Images**: Unsplash, Pexels, Pixabay (CC0)
  - **Music**: Free Music Archive, ccMixter (CC BY)
- Always verify license before adding assets
- Document sources in ASSETS_CREDITS.md

---

## Implementation Timeline
**Estimated Total**: 8 weeks (part-time development)
**Minimum Viable Product**: 4-5 weeks
**Fully Featured**: 8 weeks

## Success Metrics
- [ ] Smooth 60 FPS gameplay
- [ ] No crashes during 30-minute session
- [ ] Tutorial completion rate > 80%
- [ ] Average session length > 15 minutes
- [ ] Positive playtest feedback
