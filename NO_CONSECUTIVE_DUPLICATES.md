# No Consecutive Duplicate Images

## Feature Overview ✅

The game now ensures that **the same image is never selected twice in a row**.

## How It Works

### Image Selection Logic

**Level 1:**
```
Scan directory → Find all images → Pick random → Store path
```

**Level 2+:**
```
Scan directory → Find all images → Exclude previous → Pick random → Store path
```

### Example Flow

```
Level 1: salt-9889192_1920.jpg selected
         ↓
Level 2: Exclude salt image → Pick from remaining → dandelion selected
         ↓
Level 3: Exclude dandelion → Pick from remaining → Could be salt again (OK!)
         ↓
Level 4: Exclude salt → Pick from remaining → dandelion selected
```

## Implementation Details

### 1. Track Current Image

```rust
struct BackgroundImage {
    handle: Handle<Image>,
    revealed_percentage: f32,
    threshold_reached: bool,
    current_image_path: String,  // ← NEW: Track what's loaded
}
```

### 2. Modified Selection Function

```rust
fn get_random_image_path(exclude_path: Option<&str>) -> String {
    // Scan directory
    // Find all images
    // If exclude_path provided, filter it out
    // Select random from remaining
}
```

### 3. Level 1 (No Exclusion)

```rust
load_random_image() {
    let image = get_random_image_path(None);  // No exclusion
    bg_image.current_image_path = image;      // Store it
}
```

### 4. Level 2+ (With Exclusion)

```rust
advance_level() {
    let previous = bg_image.current_image_path;
    let image = get_random_image_path(Some(&previous));  // Exclude previous
    bg_image.current_image_path = image;                 // Store new one
}
```

## Edge Cases Handled

### Case 1: Only One Image Available

```
Situation: User has only 1 image in folder
Behavior: Cannot exclude it (would have no images left)
Action: Warns in console, uses same image
Console: ⚠️ Only one image available - cannot avoid repetition
```

### Case 2: Two Images Available

```
Situation: User has 2 images
Level 1: Image A
Level 2: Must be Image B (A excluded)
Level 3: Must be Image A (B excluded)
Level 4: Must be Image B (A excluded)
Result: Perfect alternation!
```

### Case 3: Three+ Images Available

```
Situation: User has 3+ images
Level 1: Image A
Level 2: Randomly picks from B, C (A excluded)
Level 3: Randomly picks from available (excludes Level 2's choice)
Result: True randomness with no immediate repeats
```

## Console Output

### Normal Operation (Multiple Images)

```
📁 Found 2 images in directory
🎲 Randomly selected: images/salt-9889192_1920.jpg
Loading background image: images/salt-9889192_1920.jpg

(Level completes...)

⏭️ Advancing to Level 2
🔄 Resetting for Level 2...
🚫 Excluding previous image: images/salt-9889192_1920.jpg
📁 Found 1 images in directory
🎲 Randomly selected: images/inside-dandlion-9865287_1920.jpg
Loading new image for level 2: images/inside-dandlion-9865287_1920.jpg
```

### Edge Case (Only One Image)

```
📁 Found 1 images in directory
🎲 Randomly selected: images/only-image.jpg
Loading background image: images/only-image.jpg

(Level completes...)

⏭️ Advancing to Level 2
🔄 Resetting for Level 2...
🚫 Excluding previous image: images/only-image.jpg
⚠️ Only one image available - cannot avoid repetition
📁 Found 1 images in directory
🎲 Randomly selected: images/only-image.jpg
Loading new image for level 2: images/only-image.jpg
```

## Benefits

1. ✅ **Variety** - Players see different images each level
2. ✅ **No boring repeats** - Same image won't appear twice in a row
3. ✅ **Smart handling** - Works with any number of images (1, 2, 3+)
4. ✅ **Clear feedback** - Console shows what's happening
5. ✅ **True randomness** - Still random, just excludes previous

## Testing Scenarios

### Test 1: Two Images

```bash
# Ensure you have exactly 2 images
ls assets/images/*.jpg assets/images/*.png
# Should show 2 files

# Run game
./run.sh

# Expected:
# - Level 1: Image A
# - Level 2: Image B (guaranteed different)
# - Level 3: Image A (guaranteed different from Level 2)
# Perfect alternation!
```

### Test 2: Three Images

```bash
# Add a third image
cp ~/another-pic.png assets/images/

# Run game
./run.sh

# Expected:
# - Level 1: Random pick (A, B, or C)
# - Level 2: Random from other 2 (excludes Level 1's choice)
# - Level 3: Random from other 2 (excludes Level 2's choice)
# True randomness with no consecutive repeats!
```

### Test 3: One Image (Edge Case)

```bash
# Keep only 1 image
cd assets/images
rm inside-dandlion-9865287_1920.jpg
# Only salt-9889192_1920.jpg remains

# Run game
./run.sh

# Expected:
# - Warning in console
# - Same image used each level (unavoidable)
```

## Comparison

### Before (Could Repeat)

```
Level 1: salt.jpg
Level 2: salt.jpg  ← Same! (boring)
Level 3: dandelion.jpg
Level 4: dandelion.jpg  ← Same! (boring)
```

### After (No Consecutive Repeats)

```
Level 1: salt.jpg
Level 2: dandelion.jpg  ← Different! (guaranteed)
Level 3: salt.jpg  ← OK (not consecutive)
Level 4: dandelion.jpg  ← Different from Level 3!
```

## Code Locations

- **Tracking**: `BackgroundImage.current_image_path`
- **Selection**: `get_random_image_path(exclude: Option<&str>)`
- **Level 1**: `load_random_image()` - calls with `None`
- **Level 2+**: `advance_level()` - calls with `Some(&previous)`

## Recommendation

**Add at least 3 images for best experience!**

With 3+ images, you get:
- True randomness
- No consecutive repeats
- Maximum variety

```bash
# Add more images for better variety!
cp ~/pic1.jpg assets/images/
cp ~/pic2.png assets/images/
cp ~/pic3.jpg assets/images/
# Now have 5 images total - great variety!
```

## Summary

✅ Same image never appears twice in a row
✅ Works with any number of images (1, 2, 3+)
✅ Maintains randomness (just excludes previous)
✅ Clear console feedback
✅ Smart edge case handling

Play and enjoy the variety! 🎲🖼️
