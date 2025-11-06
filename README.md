# Strong Lines - Qix Clone

A Rust/Bevy implementation of the classic Qix-style arcade game where you reveal hidden images by drawing lines while avoiding bouncing enemies.

## How to Play

### Objective
- Draw lines to claim territory and reveal the hidden image
- **Level 1**: Uncover just **10%** and the **ENTIRE image is revealed!**
- **Level 2**: Uncover **12%** to see the full image
- **Level 3**: Uncover **14%** to see the full image
- Each level adds +2% to the reveal requirement
- **Enemy speed increases 10% per level** - gets progressively harder!
- **10-second display** of complete image before next level starts
- Avoid enemies while drawing lines

### Controls
- **Arrow Keys** or **WASD**: Move your character
- Character is GREEN
- Enemies are RED bouncing dots

### Gameplay
1. **Game starts** - A random image from `assets/images/` is selected and hidden
2. **Goal**: Uncover 10% of the territory (Level 1)
3. Start on the edge of the screen (edges are safe zones)
4. Move into unclaimed (black) territory to start drawing
5. Claimed areas turn slightly gray (but image still hidden)
6. **Hit 10%!** - **ENTIRE image is instantly revealed!** All black overlays disappear!
7. **Game pauses** - View the complete image for 10 seconds (countdown shows)
8. **Auto-advance** - Next level starts automatically with new image and faster enemies
9. While playing:
   - Yellow drawing path shows where you're vulnerable
   - Enemies flash WHITE when bouncing
   - Lose a life if enemy hits you while drawing
10. **Enemies get faster each level** (+10% speed per level)

**The reward**: Uncover just 10-14% and see the WHOLE image!
**The challenge**: Enemies get faster every level!

### Strategy
- **Early game (before threshold)**: Work blind! Claim safely from edges
- **After threshold**: Now you can see - plan your path using the visible image
- Make quick, small captures to play it safe
- Or risk larger captures for faster progression
- Watch enemy movement patterns and bounce behavior
- Plan your path before committing
- Use claimed areas as barriers - enemies bounce off them!
- Your drawing line acts as a temporary wall - enemies can't cross it
- Enemies flash white when bouncing - use this to track their behavior
- **Level strategy**: Rush to threshold to start seeing the image, then optimize your captures

## Key Features

✅ **Complete Image Reveal System**:
   - Uncover 10-14% to WIN the level
   - **ENTIRE image revealed instantly** (not just uncovered parts!)
   - All overlays disappear for pure, clean image view
   - Game pauses for 10-second viewing
   - Auto-advance to next level
✅ **Escalating Difficulty**:
   - Enemy speed increases 10% per level
   - Higher reveal requirements each level (+2%)
   - Level 10 = 28% needed, 190% enemy speed!
✅ **Smart image selection**:
   - Automatically scans directory for images
   - Random selection each level
   - **Never shows same image twice in a row!**
   - Add images anytime - no code changes needed
✅ **Smooth progression** - 10-second pause between levels
✅ **Perfect enemy collision system**:
   - Enemies bounce off window borders
   - Enemies bounce off claimed areas
   - Enemies bounce off active drawing lines
   - Enemies freeze during level completion
   - Visual feedback: White flash on bounce
✅ **Safe edge zones** - Start with edges already claimed
✅ **Clean visuals** - Pure image display, no clutter
✅ **60 FPS gameplay** with predictive collision detection

## Adding Your Own Images

1. Place your images in: `assets/images/`
2. **Supported formats: PNG (.png) and JPEG (.jpg, .jpeg)**
3. **That's it!** The game automatically scans the directory
4. **No code changes needed** - just drop files and play!
5. **Images are randomly selected** each level from whatever is in the folder

### How it works:
- Game scans `assets/images/` directory at startup and each level
- Automatically finds all PNG and JPEG files
- Ignores non-image files (README.txt, etc.)
- Randomly selects one image per level
- **Never selects the same image twice in a row!**
- Add or remove images anytime - no coding required!

### Current images included:
- `inside-dandlion-9865287_1920.jpg` - Dandelion photo
- `salt-9889192_1920.jpg` - Salt crystals photo

### Example - Adding more images:
```bash
# Just copy images to the folder!
cp ~/my-photo.png assets/images/
cp ~/another-pic.jpg assets/images/
# Game will automatically find and use them!
```

### How the reveal works:
- At game start, one image is randomly chosen from the folder
- The entire image is hidden under black squares (80x80 grid)
- Uncover 10% → **ENTIRE image revealed!** (not just parts)
- View complete image for 10 seconds
- Next level: New random image from folder, need 12% to reveal
- Each level requires +2% more to uncover
- At game start, one image is randomly chosen
- The entire image is hidden under black squares (80x80 grid)
- As you claim territory, black squares are removed
- The image appears as a **cutout** - you see only the parts you uncovered!
- Cyan borders show which areas are safe (already claimed)

## Building and Running

### Quick start:
```bash
./run.sh
```

### Or manually:
```bash
cd /home/developer/rust/strong-lines
cargo run --release
```

## Dependencies
- Bevy 0.14 (with JPEG support enabled)
- Rand 0.8

## Image Format Support
- PNG (.png) ✅
- JPEG (.jpg, .jpeg) ✅

## Technical Details

- **Grid System**: 80x80 cells for smooth gameplay
- **Random Image Selection**: Picks from available images in assets/images/ at startup
- **Image Reveal System**:
  - Background image scaled to 800x600 at z=-1.0
  - 6400 black overlay sprites (80x80 grid) at z=0.0 covering the image
  - When area claimed, corresponding overlay sprite is despawned
  - Result: Progressive cutout reveal showing only uncovered portions
- **Z-layering**: 
  - Background Image (z=-1.0) - Always there, scaled to window
  - Black Overlay Grid (z=0.0) - Removed cell-by-cell as areas claimed
  - Player/Enemies (z=0.5-1.0)
  - Drawing Grid Gizmos (on top) - Cyan borders & yellow path
- **Collision**: Real-time grid-based collision with predictive detection
- **Performance**: Optimized sprite despawning for revealed areas
