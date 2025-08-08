#!/bin/bash

# Create a shell app bundle in /Applications that symlinks to our release binaries
# This allows macOS and Karabiner to see it as a proper app bundle while keeping single binaries

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
SOURCE_APP="$PROJECT_ROOT/target/release/HookAnchor.app"
DEST_APP="/Applications/HookAnchor.app"

if [ ! -d "$SOURCE_APP" ]; then
    echo "❌ Error: HookAnchor.app not found at $SOURCE_APP"
    echo "Please build the app first"
    exit 1
fi

echo "Creating app bundle structure in /Applications with symlinks..."

# Create the app bundle structure
mkdir -p "$DEST_APP/Contents/MacOS"
mkdir -p "$DEST_APP/Contents/Resources"

# Copy Info.plist (this needs to be a real file, not a symlink)
cp "$SOURCE_APP/Contents/Info.plist" "$DEST_APP/Contents/Info.plist"

# Create symlinks to the actual binaries
ln -sf "$SOURCE_APP/Contents/MacOS/HookAnchor" "$DEST_APP/Contents/MacOS/HookAnchor"
ln -sf "$SOURCE_APP/Contents/MacOS/popup_server" "$DEST_APP/Contents/MacOS/popup_server"

# Symlink the Resources folder
ln -sf "$SOURCE_APP/Contents/Resources" "$DEST_APP/Contents/Resources"

# Create icon symlink if it exists
if [ -f "$SOURCE_APP/Contents/Resources/AppIcon.icns" ]; then
    ln -sf "$SOURCE_APP/Contents/Resources/AppIcon.icns" "$DEST_APP/Contents/Resources/AppIcon.icns"
fi

echo "✅ Created app bundle with symlinks in /Applications"
echo "   Binaries remain in: $SOURCE_APP/Contents/MacOS/"
echo "   App accessible at: /Applications/HookAnchor.app"

# Test that the symlinks work
if [ -x "$DEST_APP/Contents/MacOS/HookAnchor" ]; then
    echo "✅ Symlinks verified - app should work properly"
else
    echo "❌ Warning: Symlinks may not be working correctly"
fi