#!/bin/bash

# Script to convert PNG icon to macOS .icns format and install it

ICON_PNG="$1"
APP_BUNDLE="/Applications/HookAnchor.app"

if [ -z "$ICON_PNG" ]; then
    echo "Usage: $0 <path_to_icon.png>"
    echo "Example: $0 ~/Desktop/hookanchor_icon.png"
    exit 1
fi

if [ ! -f "$ICON_PNG" ]; then
    echo "Error: Icon file not found: $ICON_PNG"
    exit 1
fi

if [ ! -d "$APP_BUNDLE" ]; then
    echo "Error: HookAnchor app bundle not found at: $APP_BUNDLE"
    exit 1
fi

echo "Converting PNG to .icns format..."

# Create temporary directory for icon conversion
TEMP_DIR=$(mktemp -d)
ICONSET_DIR="$TEMP_DIR/AppIcon.iconset"
mkdir -p "$ICONSET_DIR"

# Generate all required icon sizes
echo "Generating icon sizes..."
sips -z 16 16 "$ICON_PNG" --out "$ICONSET_DIR/icon_16x16.png"
sips -z 32 32 "$ICON_PNG" --out "$ICONSET_DIR/icon_16x16@2x.png"
sips -z 32 32 "$ICON_PNG" --out "$ICONSET_DIR/icon_32x32.png"
sips -z 64 64 "$ICON_PNG" --out "$ICONSET_DIR/icon_32x32@2x.png"
sips -z 128 128 "$ICON_PNG" --out "$ICONSET_DIR/icon_128x128.png"
sips -z 256 256 "$ICON_PNG" --out "$ICONSET_DIR/icon_128x128@2x.png"
sips -z 256 256 "$ICON_PNG" --out "$ICONSET_DIR/icon_256x256.png"
sips -z 512 512 "$ICON_PNG" --out "$ICONSET_DIR/icon_256x256@2x.png"
sips -z 512 512 "$ICON_PNG" --out "$ICONSET_DIR/icon_512x512.png"
sips -z 1024 1024 "$ICON_PNG" --out "$ICONSET_DIR/icon_512x512@2x.png"

# Convert iconset to .icns
echo "Creating .icns file..."
iconutil -c icns "$ICONSET_DIR" -o "$TEMP_DIR/applet.icns"

# Backup original icon
echo "Backing up original icon..."
cp "$APP_BUNDLE/Contents/Resources/applet.icns" "$APP_BUNDLE/Contents/Resources/applet.icns.backup"

# Install new icon
echo "Installing new icon..."
cp "$TEMP_DIR/applet.icns" "$APP_BUNDLE/Contents/Resources/applet.icns"

# Clean up
rm -rf "$TEMP_DIR"

echo "Icon installation complete!"
echo "You may need to restart the app and clear icon cache for changes to take effect."
echo "To clear icon cache, run: sudo rm -rf /Library/Caches/com.apple.iconservices.store"
echo "Original icon backed up to: $APP_BUNDLE/Contents/Resources/applet.icns.backup"