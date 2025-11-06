# JPEG Support Fix

## Issue
Game was failing to load JPEG images with error:
```
ERROR bevy_asset::server: Failed to load asset 'images/salt-9889192_1920.jpg'
The image format Jpeg is not supported
```

## Solution
Added `jpeg` feature to Bevy in Cargo.toml:

```toml
[dependencies]
bevy = { version = "0.14", features = ["jpeg"] }
rand = "0.8"
```

## Status
✅ Fixed and rebuilt successfully
✅ Game now supports both PNG and JPEG images

## Supported Image Formats

With the current configuration:
- ✅ PNG (.png)
- ✅ JPEG (.jpg, .jpeg)

## Ready to Play!

```bash
cd /home/developer/rust/strong-lines
./run.sh
```

The game will now correctly load all images in the assets/images/ folder!
