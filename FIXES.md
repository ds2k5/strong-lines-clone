# CHANGELOG - Strong Lines Fixes

## Fixed Issues

### 1. Enemy Bouncing Mechanics ✅

**Problem**: Enemies were not properly bouncing off borders and claimed areas.

**Solution**: 
- Implemented **predictive collision detection**
- Checks enemy's NEXT position before moving
- Determines which direction (X, Y, or both) should bounce
- Separates horizontal and vertical collision detection

**Now enemies bounce off:**
- Window borders (all 4 walls)
- Claimed areas (where player successfully drew)
- Active drawing lines (yellow trail)

### 2. Background Image Reveal ✅

**Problem**: Background image was not visible when areas were uncovered.

**Solution**:
- Background image spawned at z=-1.0 (behind everything)
- Created 80x80 grid of dark overlay sprites at z=0.0
- As player claims areas, overlay sprites are despawned
- Background image progressively revealed underneath

### 3. Visual Feedback Enhancement ✅

**Added**: 
- Enemies flash **WHITE** for 0.2 seconds when they bounce
- Makes it easy to see collision detection working
- Helps players understand enemy behavior

## Technical Improvements

### Collision Detection System
```rust
// Predictive system - checks NEXT position
let next_pos = current + velocity * delta_time;
let next_grid = world_to_grid(next_pos);

// Check what will be hit
if will_hit_wall(next_pos) { bounce_x or bounce_y }
if will_hit_claimed(next_grid) { bounce_x or bounce_y }
if will_hit_drawing_line(next_grid) { bounce_x or bounce_y }

// Then move with corrected velocity
```

### Image Reveal System
```
Layer Stack (bottom to top):
- Background Image (z=-1.0) - Always there
- Dark Overlay Grid (z=0.0) - Removed when claimed
- Player/Enemies (z=0.5-1.0)
- Drawing Grid Gizmos (on top)
```

## Testing Checklist

✅ Enemies bounce off top wall
✅ Enemies bounce off bottom wall  
✅ Enemies bounce off left wall
✅ Enemies bounce off right wall
✅ Enemies bounce off claimed areas
✅ Enemies bounce off active drawing line
✅ Enemies flash white on bounce
✅ Background image visible in claimed areas
✅ Starting edges are pre-claimed
✅ Player loses life when hit while drawing

## How to Run

```bash
cd /home/developer/rust/strong-lines
./run.sh
```

Or:
```bash
cargo run --release
```

## Adding Custom Images

Replace `/home/developer/rust/strong-lines/assets/images/default.png` with your own 800x600 image!
