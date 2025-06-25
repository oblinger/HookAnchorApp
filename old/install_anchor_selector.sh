#!/bin/bash

# Install AnchorSelector to make it visible in System Settings
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
SOURCE_APP="$SCRIPT_DIR/AnchorSelector.app"
TARGET_APP="/Applications/AnchorSelector.app"

echo "Installing AnchorSelector to Applications folder"
echo "==============================================="

# Check if source app exists
if [ ! -d "$SOURCE_APP" ]; then
    echo "❌ Source app not found: $SOURCE_APP"
    echo "Run ./setup_f10_shortcut.sh first"
    exit 1
fi

echo "📱 Source app found: $SOURCE_APP"

# Remove existing installation if present
if [ -d "$TARGET_APP" ]; then
    echo "🗑️  Removing existing installation..."
    rm -rf "$TARGET_APP"
fi

# Copy to Applications folder
echo "📁 Installing to: $TARGET_APP"
cp -R "$SOURCE_APP" "$TARGET_APP"

# Verify installation
if [ -d "$TARGET_APP" ]; then
    echo "✅ Installation successful!"
    echo ""
    echo "🎯 Now you can set up F10 shortcut:"
    echo "1. Go to System Settings > Keyboard > Shortcuts"
    echo "2. Click 'App Shortcuts'"
    echo "3. Click '+' button"
    echo "4. Application: Click dropdown and find 'AnchorSelector'"
    echo "5. Menu Title: (leave blank)"
    echo "6. Keyboard Shortcut: Press F10"
    echo ""
    echo "✨ AnchorSelector should now appear in the Applications list!"
    echo ""
    echo "🧪 Test the installed app:"
    echo "open '$TARGET_APP'"
else
    echo "❌ Installation failed"
    exit 1
fi