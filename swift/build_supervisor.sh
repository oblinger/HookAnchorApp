#!/bin/bash

# Build script for Swift Supervisor

set -e

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
PROJECT_ROOT="$( cd "$SCRIPT_DIR/.." && pwd )"
SWIFT_DIR="$SCRIPT_DIR/Supervisor"
BUILD_DIR="$PROJECT_ROOT/target/release"

echo "Building Swift Supervisor..."

# Create build directory if it doesn't exist
mkdir -p "$BUILD_DIR"

# Build to a temporary file first
TEMP_BINARY="$BUILD_DIR/HookAnchor.tmp"

# Compile HookAnchor (Swift supervisor)
swiftc -O \
    -parse-as-library \
    -o "$TEMP_BINARY" \
    "$SWIFT_DIR/HookAnchor.swift" \
    -framework Cocoa \
    -framework Foundation

# Move the temporary binary to the final location
mv "$TEMP_BINARY" "$BUILD_DIR/HookAnchor"

echo "HookAnchor supervisor built at: $BUILD_DIR/HookAnchor"

# Make it executable
chmod +x "$BUILD_DIR/HookAnchor"

# IMPORTANT: Never copy binaries! The app bundle uses symlinks
echo "✅ Build complete!"
echo "Note: /Applications/HookAnchor.app uses symlinks to this binary"
echo "To test, run: $BUILD_DIR/HookAnchor"