#!/bin/bash

# Build script for creating universal macOS binaries

set -e

PROJECT_DIR="/home/developer/rust/strong-lines-clone"
SDK_ROOT="$PROJECT_DIR/osxcross/target/SDK/MacOSX26.1.sdk"
OSXCROSS_BIN="$PROJECT_DIR/osxcross/target/bin"

echo "====================================="
echo "Building Strong Lines for macOS"
echo "====================================="

# Add OSXcross to PATH
export PATH="$OSXCROSS_BIN:$PATH"

# Export common environment variables
export OSXCROSS_SDK_VERSION=26.1
export SDKROOT="$SDK_ROOT"

# Clean previous builds
echo "Cleaning previous builds..."
cargo clean

# Build for ARM64 (Apple Silicon)
echo ""
echo "====================================="
echo "Building for aarch64-apple-darwin (Apple Silicon)..."
echo "====================================="
export BINDGEN_EXTRA_CLANG_ARGS="-target aarch64-apple-darwin -isysroot $SDK_ROOT -arch arm64"
cross build --target aarch64-apple-darwin --release

if [ $? -eq 0 ]; then
    echo "✓ ARM64 build successful!"
else
    echo "✗ ARM64 build failed!"
    exit 1
fi

# Build for x86_64 (Intel)
echo ""
echo "====================================="
echo "Building for x86_64-apple-darwin (Intel)..."
echo "====================================="
export BINDGEN_EXTRA_CLANG_ARGS="-target x86_64-apple-darwin -isysroot $SDK_ROOT -arch x86_64"
cross build --target x86_64-apple-darwin --release

if [ $? -eq 0 ]; then
    echo "✓ x86_64 build successful!"
else
    echo "✗ x86_64 build failed!"
    exit 1
fi

# Check if lipo is available for creating universal binary
if command -v lipo &> /dev/null; then
    echo ""
    echo "====================================="
    echo "Creating Universal Binary..."
    echo "====================================="
    
    mkdir -p target/universal-apple-darwin/release
    
    lipo -create \
        target/aarch64-apple-darwin/release/strong-lines \
        target/x86_64-apple-darwin/release/strong-lines \
        -output target/universal-apple-darwin/release/strong-lines
    
    if [ $? -eq 0 ]; then
        echo "✓ Universal binary created successfully!"
        echo ""
        echo "Binary locations:"
        echo "  ARM64:     target/aarch64-apple-darwin/release/strong-lines"
        echo "  x86_64:    target/x86_64-apple-darwin/release/strong-lines"
        echo "  Universal: target/universal-apple-darwin/release/strong-lines"
    else
        echo "✗ Failed to create universal binary"
        echo ""
        echo "Individual binaries available at:"
        echo "  ARM64:  target/aarch64-apple-darwin/release/strong-lines"
        echo "  x86_64: target/x86_64-apple-darwin/release/strong-lines"
    fi
else
    echo ""
    echo "Note: lipo not found. Skipping universal binary creation."
    echo ""
    echo "Binaries available at:"
    echo "  ARM64:  target/aarch64-apple-darwin/release/strong-lines"
    echo "  x86_64: target/x86_64-apple-darwin/release/strong-lines"
fi

echo ""
echo "====================================="
echo "Build Complete!"
echo "====================================="
