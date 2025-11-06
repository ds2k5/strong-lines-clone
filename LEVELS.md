# Level System - Strong Lines

## How Levels Work

### Level Progression

Each level has a **reveal threshold** - the percentage of territory you must claim before the image starts to reveal.

| Level | Threshold | Challenge |
|-------|-----------|-----------|
| 1 | 40% | Claim 40% blindly before seeing image |
| 2 | 42% | Claim 42% blindly |
| 3 | 44% | Claim 44% blindly |
| 4 | 46% | Claim 46% blindly |
| 5 | 48% | Claim 48% blindly |
| ... | +2% each | Continues increasing |

### Level Phases

#### Phase 1: Blind Phase (0% → Threshold)
```
Status: Image Hidden
Visual: Pure black (unclaimed) and dark gray (claimed but hidden)
Strategy: Work from edges, claim safely, avoid enemies
UI: "Progress: 25% / 40% needed | Image hidden until threshold!"
```

#### Phase 2: Reveal Phase (Threshold → 80%)
```
Status: Image Revealing  
Visual: Image appears in all claimed areas!
Strategy: Now you can see - optimize your captures
UI: "Score: 55% | Image revealing!"
```

#### Phase 3: Level Complete (80%)
```
Status: Level Complete!
Result: Advance to next level
- New random image selected
- Grid reset (edges remain claimed)
- Threshold increases by 2%
- Same lives carry over
```

## Visual States

### Before Threshold (e.g., at 30% in Level 1)
```
Black = Not claimed yet
Dark Gray = Claimed but image still hidden
Yellow = Your drawing path
Cyan border = Safe claimed area edges
```

### After Threshold (e.g., at 50% in Level 1)
```
Black = Not claimed yet  
Image Visible = Claimed and revealed!
Yellow = Your drawing path
Cyan border = Claimed area edges
```

## Difficulty Curve

The level system creates increasing difficulty:

1. **More blind work**: Higher levels require more territory claimed before reveal
2. **Increased risk**: More time spent working without visual feedback
3. **Strategic planning**: Must memorize enemy patterns without image hints
4. **Suspense building**: Longer wait creates more satisfying reveal moment

## Completion Requirements

- **Level complete**: Reach 80% coverage
- **Game over**: Lose all 3 lives
- **Lives**: Carry over between levels (don't reset)

## Console Messages

Watch the console for these messages:

```
Loading background image: images/salt-9889192_1920.jpg
```
- Shows which image was randomly selected

```
🎉 Threshold reached! Image revealing at 40.5%
```
- Confirms when image starts revealing

```
🎊 Level 1 Complete! Moving to Level 2
Loading new image for level 2: images/inside-dandlion-9865287_1920.jpg
```
- Shows level completion and new image

## Strategy Tips

### Early Level (Low Threshold - e.g., Level 1-3)
- Claim 40-44% relatively quickly
- Once revealed, use image details to plan efficient routes
- Balance speed with safety

### Mid Level (Medium Threshold - e.g., Level 5-8)
- 48-54% blind work is challenging
- Focus on safe, systematic claiming from edges
- Memorize enemy bounce patterns
- Plan ahead for post-reveal optimization

### High Level (High Threshold - e.g., Level 10+)
- 58%+ blind claiming requires patience
- Stick to edges and safe zones
- Small, conservative captures
- The reveal becomes extremely satisfying!

## Example Gameplay Flow

```
Level 1 Start
├─ Claim territory: 10% → 20% → 30% (all gray, no image)
├─ Continue: 35% → 40% 
├─ 🎉 THRESHOLD REACHED! Image appears!
├─ Now seeing image: 45% → 55% → 70% → 80%
└─ 🎊 LEVEL COMPLETE!

Level 2 Start
├─ New random image loaded
├─ Threshold now 42%
├─ Claim territory: 10% → 25% → 42%
├─ 🎉 THRESHOLD REACHED!
└─ Continue to 80%...
```

## Why This Design?

This level system adds:
- **Suspense**: Building anticipation before the reveal
- **Challenge**: Harder to navigate without visual cues
- **Progression**: Clear sense of difficulty increase
- **Replayability**: New images and thresholds each level
- **Skill testing**: Tests both blind navigation and visual optimization

The threshold mechanic makes you **earn the reveal** rather than seeing the image immediately!
