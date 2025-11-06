# Dynamic Image Loading - No Hardcoded Names!

## Changes Made ✅

### 1. **Removed Hardcoded Image List**
**Before:**
```rust
let images = vec![
    "images/default.png",
    "images/inside-dandlion-9865287_1920.jpg",
    "images/salt-9889192_1920.jpg",
];
```

**After:**
```rust
// Automatically scans assets/images/ directory
fn get_random_image_path() -> String {
    // Dynamically finds ALL .png, .jpg, .jpeg files
    // No hardcoding needed!
}
```

### 2. **Dynamic Directory Scanning**
The game now:
- Scans `assets/images/` directory at runtime
- Finds all valid image files (PNG, JPG, JPEG)
- Filters out non-image files (README.txt, etc.)
- Randomly selects from available images
- No code changes needed when adding new images!

### 3. **BackgroundSprite Component**
Added a marker component to properly track and despawn background images between levels:
```rust
#[derive(Component)]
struct BackgroundSprite;
```

### 4. **Removed default.png**
- No longer in the image list
- Was never created properly anyway
- Game now only uses the actual image files in the directory

## How It Works

### Startup (Level 1)
```
1. Scan assets/images/ directory
2. Find: inside-dandlion-9865287_1920.jpg, salt-9889192_1920.jpg
3. Randomly select one (e.g., salt image)
4. Load and display

Console output:
📁 Found 2 images in directory
🎲 Randomly selected: images/salt-9889192_1920.jpg
Loading background image: images/salt-9889192_1920.jpg
```

### Level Advancement
```
1. Complete level (10% uncovered)
2. Show image for 10 seconds
3. Despawn old background sprite (using BackgroundSprite component)
4. Scan directory again
5. Randomly select new image
6. Load and display

Console output:
⏭️ Advancing to Level 2
🔄 Resetting for Level 2...
📁 Found 2 images in directory
🎲 Randomly selected: images/inside-dandlion-9865287_1920.jpg
Loading new image for level 2: images/inside-dandlion-9865287_1920.jpg
```

## Adding New Images

**Super Easy - Just Drop Files!**

```bash
# Add any PNG or JPEG image
cp ~/my-cool-image.png /home/developer/rust/strong-lines/assets/images/
cp ~/another-image.jpg /home/developer/rust/strong-lines/assets/images/

# That's it! No code changes needed!
# Game will automatically find and use them
```

## Current Images Available

```
/home/developer/rust/strong-lines/assets/images/
├── inside-dandlion-9865287_1920.jpg  ✅ Will be used
├── salt-9889192_1920.jpg             ✅ Will be used
├── PLACE_IMAGES_HERE.txt             ❌ Ignored (not an image)
└── README.txt                        ❌ Ignored (not an image)
```

## File Type Support

**Supported (automatically detected):**
- `.png` files
- `.jpg` files
- `.jpeg` files

**Ignored:**
- Text files (.txt)
- Any other non-image files

**Case insensitive** - works with .PNG, .JPG, .JPEG too!

## Benefits

1. ✅ **No hardcoding** - Add images anytime without editing code
2. ✅ **Automatic detection** - Game finds all valid images
3. ✅ **True randomness** - Picks from entire collection each level
4. ✅ **Easy expansion** - Drop new images in folder, done!
5. ✅ **Clean code** - No maintenance of image lists
6. ✅ **Proper cleanup** - BackgroundSprite component ensures old images are removed

## Console Debug Output

Watch for these messages to confirm it's working:

```
📁 Found 2 images in directory
🎲 Randomly selected: images/salt-9889192_1920.jpg
Loading background image: images/salt-9889192_1920.jpg
```

If no images found:
```
⚠️ No images found in assets/images! Please add PNG or JPEG files.
```

## Code Structure

### Function: `get_random_image_path()`
```rust
1. Read assets/images/ directory
2. Loop through entries
3. Check if file ends with .png/.jpg/.jpeg
4. Add to list
5. Randomly select one
6. Return path
```

### Usage in Game
- `load_random_image()` - Level 1 startup
- `advance_level()` - Every level after that

Both call `get_random_image_path()` for dynamic scanning!

## Testing

1. **Remove an image:**
```bash
rm /home/developer/rust/strong-lines/assets/images/salt-9889192_1920.jpg
# Game now only finds dandelion image
```

2. **Add more images:**
```bash
cp ~/photo1.png /home/developer/rust/strong-lines/assets/images/
cp ~/photo2.jpg /home/developer/rust/strong-lines/assets/images/
# Game now finds 3 images total
```

3. **Run and verify:**
```bash
./run.sh
# Watch console for: "📁 Found X images in directory"
```

## Level 2 Image Display Fix

The BackgroundSprite component now ensures:
- Old background is properly despawned
- New background is properly spawned
- No orphaned sprites
- Clean level transitions

**This fixes the "show the image did not work in level 2" issue!**

## Summary

✅ Dynamic directory scanning
✅ No hardcoded image names
✅ Add images anytime, no code changes
✅ Proper sprite cleanup with BackgroundSprite
✅ Level 2+ images now display correctly
✅ Random selection from all available images

Just drop images in `assets/images/` and play! 🎮🖼️
