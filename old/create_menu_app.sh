#!/bin/bash

echo "🍎 Creating AnchorSelector with Menu Support"
echo "============================================"

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
LAUNCHER_PATH="$SCRIPT_DIR/launch_popup.sh"
APP_NAME="AnchorSelectorWithMenu.app"
APP_PATH="/Applications/$APP_NAME"

# Remove existing app
if [ -d "$APP_PATH" ]; then
    echo "🗑️  Removing existing app..."
    rm -rf "$APP_PATH"
fi

echo "📱 Creating app with menu support..."

# Create app bundle structure
mkdir -p "$APP_PATH/Contents/MacOS"
mkdir -p "$APP_PATH/Contents/Resources"

# Create Info.plist with proper app configuration
cat > "$APP_PATH/Contents/Info.plist" << 'EOF'
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleExecutable</key>
    <string>AnchorSelectorWithMenu</string>
    <key>CFBundleIdentifier</key>
    <string>com.local.anchorselector.menu</string>
    <key>CFBundleName</key>
    <string>AnchorSelectorWithMenu</string>
    <key>CFBundlePackageType</key>
    <string>APPL</string>
    <key>CFBundleVersion</key>
    <string>1.0</string>
    <key>LSUIElement</key>
    <false/>
    <key>NSPrincipalClass</key>
    <string>NSApplication</string>
</dict>
</plist>
EOF

# Create executable AppleScript app that has a menu
cat > "$APP_PATH/Contents/MacOS/AnchorSelectorWithMenu" << EOF
#!/usr/bin/osascript

on run
    try
        do shell script "$LAUNCHER_PATH"
    on error
        display notification "Failed to launch Anchor Selector" with title "Error"
    end try
    
    -- Exit immediately after launching
    quit
end run

-- Add a simple menu handler for App Shortcuts to work
on applicationShouldTerminate(sender)
    return true
end applicationShouldTerminate
EOF

chmod +x "$APP_PATH/Contents/MacOS/AnchorSelectorWithMenu"

# Test the new app
echo "🧪 Testing new app..."
if [ -d "$APP_PATH" ]; then
    echo "✅ Created: $APP_PATH"
    
    # Test launch
    echo "Testing launch..."
    open "$APP_PATH"
    
    echo ""
    echo "🎯 Now set up the shortcut:"
    echo "=========================="
    echo "1. Go to System Settings > Keyboard > Shortcuts > App Shortcuts"
    echo "2. Remove the old 'AnchorSelector' entry"
    echo "3. Click '+' to add new shortcut"
    echo "4. Application: Choose 'AnchorSelectorWithMenu'"
    echo "5. Menu Title: Leave blank"
    echo "6. Keyboard Shortcut: Press Cmd+F9"
    echo ""
    echo "This version should work because it's a proper macOS app"
    echo "that shows up in the dock and has menu support."
    
else
    echo "❌ Failed to create app"
fi