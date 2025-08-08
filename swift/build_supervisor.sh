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

# If app bundle exists, update the binary inside it
APP_BUNDLE="$BUILD_DIR/HookAnchor.app"
if [ -d "$APP_BUNDLE" ]; then
    echo "Updating app bundle..."
    cp "$BUILD_DIR/HookAnchor" "$APP_BUNDLE/Contents/MacOS/HookAnchor"
    # Also ensure popup_server is there
    if [ -f "$BUILD_DIR/popup_server" ]; then
        cp "$BUILD_DIR/popup_server" "$APP_BUNDLE/Contents/MacOS/popup_server"
    fi
    echo "✅ App bundle updated: $APP_BUNDLE"
fi

echo "Build complete!"
echo "To test, run: $BUILD_DIR/HookAnchor"