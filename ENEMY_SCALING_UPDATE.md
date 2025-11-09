# Enemy Scaling Update

## Summary
Modified the game to progressively add more enemies as the player advances through levels, making the game increasingly challenging.

## Changes Made

### 1. Added `calculate_enemy_count()` function
- **Location**: After constants at the top of `main.rs`
- **Purpose**: Calculates the number of enemies based on the current level
- **Logic**:
  - Levels 1-6: 3 enemies
  - Level 7-9: 4 enemies
  - Level 10-12: 5 enemies
  - Level 13-15: 6 enemies
  - And so on... (+1 enemy every 3 levels after level 6)

### 2. Modified `setup_game()` function
- Now accepts `game_state: Res<GameState>` parameter
- Uses `calculate_enemy_count(game_state.level)` to spawn appropriate number of enemies
- Ensures correct number of enemies spawn when game starts

### 3. Updated `advance_level()` function
- Stores previous level's enemy count before advancing
- Calculates new level's required enemy count
- Spawns additional enemies when needed (e.g., when going from level 6 to 7)
- Displays message when adding enemies: "🔴 Adding X more enemy/enemies! Total enemies: Y"

### 4. Fixed `restart_game()` function
- Changed to despawn all existing enemies
- Spawns fresh set of 3 enemies for level 1
- Ensures clean restart with correct enemy count

### 5. Removed unused constant
- Removed `NUM_ENEMIES` constant (replaced by dynamic calculation)

## Testing the Changes

To test the new enemy scaling:
1. Play through levels 1-6 (should have 3 enemies)
2. Reach level 7 (should spawn 1 additional enemy, total 4)
3. Reach level 10 (should spawn 1 more enemy, total 5)
4. Continue to see the pattern

## Formula
```
Number of enemies = 3 + floor((level - 7) / 3) + 1  for level > 6
                  = 3                              for level <= 6
```

Examples:
- Level 1-6: 3 enemies
- Level 7: 4 enemies (3 + 1)
- Level 8: 4 enemies
- Level 9: 4 enemies
- Level 10: 5 enemies (3 + 2)
- Level 11: 5 enemies
- Level 12: 5 enemies
- Level 13: 6 enemies (3 + 3)
