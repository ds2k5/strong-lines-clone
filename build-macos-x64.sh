#!/bin/bash

# OSXCross environment for x86_64-apple-darwin17
export PATH="/home/developer/rust/strong-lines-clone/osxcross/target/bin:$PATH"
export OSXCROSS_MP_INC=1
export OSXCROSS_SDK_VERSION=10.13
export OSXCROSS_TARGET=x86_64-apple-darwin17
export OSXCROSS_SDK=/home/developer/rust/strong-lines-clone/osxcross/target/SDK/MacOSX10.13.sdk

# Build
cargo clean
cargo build --target x86_64-apple-darwin --release
