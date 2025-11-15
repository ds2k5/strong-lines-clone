#!/bin/bash
# Build script for macOS cross-compilation

# Set PATH to include OSXCross
export PATH="/home/developer/rust/strong-lines-clone/osxcross/target/bin:$PATH"

# Set environment variables for x86_64-apple-darwin
export CC_x86_64_apple_darwin=x86_64-apple-darwin25.1-clang
export CXX_x86_64_apple_darwin=x86_64-apple-darwin25.1-clang++
export AR_x86_64_apple_darwin=x86_64-apple-darwin25.1-ar

# Set environment variables for aarch64-apple-darwin  
export CC_aarch64_apple_darwin=aarch64-apple-darwin25.1-clang
export CXX_aarch64_apple_darwin=aarch64-apple-darwin25.1-clang++
export AR_aarch64_apple_darwin=aarch64-apple-darwin25.1-ar

# Build for the specified target
if [ "$1" == "x86_64" ] || [ "$1" == "intel" ]; then
    echo "Building for macOS x86_64 (Intel)..."
    cargo build --release --target x86_64-apple-darwin
elif [ "$1" == "aarch64" ] || [ "$1" == "arm" ] || [ "$1" == "m1" ]; then
    echo "Building for macOS aarch64 (Apple Silicon)..."
    cargo build --release --target aarch64-apple-darwin
elif [ "$1" == "both" ] || [ "$1" == "all" ]; then
    echo "Building for both architectures..."
    cargo build --release --target x86_64-apple-darwin
    cargo build --release --target aarch64-apple-darwin
else
    echo "Usage: $0 [x86_64|intel|aarch64|arm|m1|both|all]"
    exit 1
fi
