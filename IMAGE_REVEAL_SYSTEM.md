# Image Reveal System - How It Works

## Visual Layers

```
┌─────────────────────────────────┐
│   BLACK OVERLAY (6400 sprites)  │  ← z=0.0 - Removed as you claim
│         Covers image             │
├─────────────────────────────────┤
│   BACKGROUND IMAGE              │  ← z=-1.0 - Always there
│   (randomly selected)            │
└─────────────────────────────────┘
```

## What You'll See

### At Game Start:
- Everything is BLACK (except edges which start revealed)
- You can see a bit of the image at the edges
- A random image from assets/images/ was selected

### While Playing:
- **Black areas** = Hidden (not yet uncovered)
- **Image visible** = Uncovered (you claimed this area!)
- **Cyan border** = Edge of claimed area (safe zone)
- **Yellow squares** = Your current drawing path (vulnerable!)

### The Cutout Effect:
When you complete a drawing loop:
1. Black overlay sprites in that area are DESPAWNED
2. Background image shows through
3. You see ONLY the parts you uncovered
4. It's like peeling away a black covering!

## Image Selection

The game randomly picks from:
- default.png
- inside-dandlion-9865287_1920.jpg  
- salt-9889192_1920.jpg

**Add more images** to assets/images/ and they'll automatically be included!

## Testing Checklist

When you run the game, you should see:
✅ Console message: "Loading background image: images/[filename]"
✅ Edges already revealed (you can see a bit of the image)
✅ Rest of screen is black
✅ As you draw and claim areas, black disappears
✅ Background image appears in cutout shape
✅ Cyan borders around claimed areas
✅ Yellow trail while drawing
✅ Enemies bouncing (flashing white)

## Example Gameplay Flow

```
1. Start game → "Loading background image: images/salt-9889192_1920.jpg"
2. See edges revealed (salt crystal texture visible at borders)
3. Move into black area → yellow trail appears
4. Complete loop back to edge → BLACK REMOVED!
5. Salt crystal image now visible in that claimed area
6. Repeat to reveal more of the image
```

## Troubleshooting

**If you don't see the image:**
- Check console for "Loading background image:" message
- Make sure images exist in assets/images/
- Try claiming a larger area (the grid is 80x80, so each cell is small)
- Look at the edges - should show image immediately

**If everything is black:**
- Normal! That's the starting state
- Claim some territory and it will reveal

**If you want to test with a specific image:**
- Edit src/main.rs, function load_random_image
- Comment out the random selection
- Set: `let random_image = "images/your-image.png";`
