#!/bin/bash
set -e

# Build script for macOS targets (ARM64 and x86_64)
# Usage: ./build-macos-all.sh

echo "================================================"
echo "Building strong-lines for macOS"
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

# Set up OSXCross environment
export PATH="$BUILD_DIR/osxcross/target/bin:$PATH"
export SDKROOT="$BUILD_DIR/osxcross/target/SDK/MacOSX26.1.sdk"
export OSXCROSS_SDK_VERSION="26.1"

echo -e "${BLUE}Build directory: $BUILD_DIR${NC}"
echo -e "${BLUE}OSXCross SDK: $SDKROOT${NC}"
echo ""

# Function to build for a specific target
build_target() {
    local target=$1
    local target_name=$2
    
    echo -e "${BLUE}========================================${NC}"
    echo -e "${BLUE}Building for $target_name ($target)${NC}"
    echo -e "${BLUE}========================================${NC}"
    
    if cargo build --release --target "$target" 2>&1 | tee "build-${target}.log"; then
        echo -e "${GREEN}✓ Build successful for $target_name${NC}"
        
        # Show binary info
        local binary_path="$TARGET_DIR/$target/release/strong-lines"
        if [ -f "$binary_path" ]; then
            local size=$(ls -lh "$binary_path" | awk '{print $5}')
            echo -e "${GREEN}  Binary: $binary_path${NC}"
            echo -e "${GREEN}  Size: $size${NC}"
            file "$binary_path" | sed 's/^/  /'
        fi
    else
        echo -e "${RED}✗ Build failed for $target_name${NC}"
        echo -e "${RED}  Check build-${target}.log for details${NC}"
        return 1
    fi
    echo ""
}

# Build ARM64 (Apple Silicon)
build_target "aarch64-apple-darwin" "macOS ARM64 (Apple Silicon)"

# Build x86_64 (Intel)
build_target "x86_64-apple-darwin" "macOS x86_64 (Intel)"

echo -e "${BLUE}========================================${NC}"
echo -e "${BLUE}Build Summary${NC}"
echo -e "${BLUE}========================================${NC}"

# Check if both binaries exist
arm64_binary="$TARGET_DIR/aarch64-apple-darwin/release/strong-lines"
x86_64_binary="$TARGET_DIR/x86_64-apple-darwin/release/strong-lines"

if [ -f "$arm64_binary" ] && [ -f "$x86_64_binary" ]; then
    echo -e "${GREEN}✓ Both binaries built successfully!${NC}"
    echo ""
    echo "ARM64 binary:"
    ls -lh "$arm64_binary" | awk '{print "  " $9 " - " $5}'
    echo ""
    echo "x86_64 binary:"
    ls -lh "$x86_64_binary" | awk '{print "  " $9 " - " $5}'
    echo ""
    echo -e "${BLUE}To create a universal binary, run:${NC}"
    echo "  ./create-universal-binary.sh"
else
    echo -e "${RED}✗ Some builds failed. Check the logs above.${NC}"
    exit 1
fi

echo ""
echo -e "${GREEN}All builds completed successfully!${NC}"
