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

# Compile HookAnchor (Swift supervisor)
swiftc -O \
    -parse-as-library \
    -o "$BUILD_DIR/HookAnchor" \
    "$SWIFT_DIR/HookAnchor.swift" \
    -framework Cocoa \
    -framework Foundation

echo "HookAnchor supervisor built at: $BUILD_DIR/HookAnchor"

# Make it executable
chmod +x "$BUILD_DIR/HookAnchor"

echo "Build complete!"
echo "To test, run: $BUILD_DIR/HookAnchor"