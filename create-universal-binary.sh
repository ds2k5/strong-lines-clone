#!/bin/bash
set -e

# Script to create a universal macOS binary
# This combines ARM64 and x86_64 binaries into one

echo "================================================"
echo "Creating Universal macOS Binary"
echo "================================================"
echo ""

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# Build directory
BUILD_DIR="$(pwd)"
TARGET_DIR="$BUILD_DIR/target"

# Binary paths
ARM64_BINARY="$TARGET_DIR/aarch64-apple-darwin/release/strong-lines"
X86_64_BINARY="$TARGET_DIR/x86_64-apple-darwin/release/strong-lines"
UNIVERSAL_DIR="$TARGET_DIR/universal-apple-darwin/release"
UNIVERSAL_BINARY="$UNIVERSAL_DIR/strong-lines"

# Check if both binaries exist
if [ ! -f "$ARM64_BINARY" ]; then
    echo -e "${RED}✗ ARM64 binary not found at: $ARM64_BINARY${NC}"
    echo -e "${BLUE}  Run ./build-macos-all.sh first${NC}"
    exit 1
fi

if [ ! -f "$X86_64_BINARY" ]; then
    echo -e "${RED}✗ x86_64 binary not found at: $X86_64_BINARY${NC}"
    echo -e "${BLUE}  Run ./build-macos-all.sh first${NC}"
    exit 1
fi

echo -e "${GREEN}✓ Both binaries found${NC}"
echo ""

# Create output directory
mkdir -p "$UNIVERSAL_DIR"

# Use lipo to create universal binary
echo -e "${BLUE}Creating universal binary with lipo...${NC}"

if "$TARGET_DIR/aarch64-apple-darwin/release/../../../osxcross/target/bin/lipo" \
    -create \
    "$ARM64_BINARY" \
    "$X86_64_BINARY" \
    -output "$UNIVERSAL_BINARY"; then
    
    echo -e "${GREEN}✓ Universal binary created successfully!${NC}"
    echo ""
    
    # Show binary info
    echo -e "${BLUE}Binary information:${NC}"
    ls -lh "$UNIVERSAL_BINARY" | awk '{print "  Size: " $5}'
    echo ""
    
    # Verify architectures
    echo -e "${BLUE}Architectures in universal binary:${NC}"
    file "$UNIVERSAL_BINARY" | sed 's/^/  /'
    echo ""
    
    echo -e "${GREEN}Universal binary location:${NC}"
    echo -e "  ${UNIVERSAL_BINARY}"
    echo ""
    
    # Copy assets if they exist
    if [ -d "$BUILD_DIR/assets" ]; then
        echo -e "${BLUE}Preparing distribution package...${NC}"
        
        # Create distribution directory
        DIST_DIR="$BUILD_DIR/dist/strong-lines-macos-universal"
        mkdir -p "$DIST_DIR"
        
        # Copy binary
        cp "$UNIVERSAL_BINARY" "$DIST_DIR/"
        
        # Copy assets
        cp -r "$BUILD_DIR/assets" "$DIST_DIR/"
        
        # Copy README if exists
        [ -f "$BUILD_DIR/README.md" ] && cp "$BUILD_DIR/README.md" "$DIST_DIR/"
        
        # Copy license if exists
        [ -f "$BUILD_DIR/license.txt" ] && cp "$BUILD_DIR/license.txt" "$DIST_DIR/"
        
        echo -e "${GREEN}✓ Distribution package created at: $DIST_DIR${NC}"
        echo ""
        
        # Create zip archive
        echo -e "${BLUE}Creating zip archive...${NC}"
        cd "$BUILD_DIR/dist"
        zip -r "strong-lines-macos-universal.zip" "strong-lines-macos-universal" > /dev/null
        cd "$BUILD_DIR"
        
        echo -e "${GREEN}✓ Archive created: dist/strong-lines-macos-universal.zip${NC}"
    fi
    
else
    echo -e "${RED}✗ Failed to create universal binary${NC}"
    exit 1
fi

echo ""
echo -e "${GREEN}Done! Universal binary ready for distribution.${NC}"
