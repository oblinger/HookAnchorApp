#!/bin/bash

# Setup F10 as a global shortcut to launch Anchor Selector
# Uses macOS NSUserKeyEquivalents to programmatically set keyboard shortcuts

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
LAUNCHER_PATH="$SCRIPT_DIR/launch_popup.sh"

echo "Setting up F10 shortcut for Anchor Selector"
echo "==========================================="

# Check if launcher exists
if [ ! -f "$LAUNCHER_PATH" ]; then
    echo "❌ Launcher script not found: $LAUNCHER_PATH"
    exit 1
fi

echo "✅ Launcher script found: $LAUNCHER_PATH"

# F10 function key Unicode value is 0xF70D
# In NSUserKeyEquivalents format, this becomes \UF70D
echo "📝 Setting up global shortcut: F10 -> $LAUNCHER_PATH"

# Method 1: Try to set a global shortcut (may not work for custom scripts)
echo "Attempting to set global F10 shortcut..."
defaults write -g NSUserKeyEquivalents -dict-add "Launch Anchor Selector" "\UF70D"

# Method 2: Alternative approach using Apple's Services
echo "Setting up via Services (alternative method)..."

# Create a Services definition that can be bound to F10
# This requires creating a .workflow or .app that runs the script

SERVICE_DIR="$HOME/Library/Services"
SERVICE_NAME="Launch Anchor Selector.workflow"
SERVICE_PATH="$SERVICE_DIR/$SERVICE_NAME"

mkdir -p "$SERVICE_DIR"

# Create a simple Automator workflow equivalent
# Note: This creates the directory structure, but a full .workflow requires Automator
if [ ! -d "$SERVICE_PATH" ]; then
    echo "📝 To complete setup with Services:"
    echo "   1. Open Automator"
    echo "   2. Create new 'Service'"
    echo "   3. Add 'Run Shell Script' action"
    echo "   4. Set script to: $LAUNCHER_PATH"
    echo "   5. Save as 'Launch Anchor Selector'"
    echo "   6. Go to System Settings > Keyboard > Shortcuts > Services"
    echo "   7. Find 'Launch Anchor Selector' and set shortcut to F10"
fi

# Method 3: Instructions for manual setup
echo ""
echo "🔧 Manual Setup Instructions (Most Reliable):"
echo "=============================================="
echo "Since programmatic shortcut assignment has limitations for custom scripts,"
echo "here's how to set it up manually:"
echo ""
echo "1. Go to System Settings > Keyboard > Keyboard Shortcuts"
echo "2. Click 'App Shortcuts' in the left sidebar"  
echo "3. Click the '+' button"
echo "4. Set:"
echo "   - Application: All Applications"
echo "   - Menu Title: (leave empty or enter a description)"
echo "   - Keyboard Shortcut: Press F10"
echo "5. Or use the Services method described above"
echo ""
echo "📋 Launcher path to use: $LAUNCHER_PATH"
echo ""

# Method 4: Create an AppleScript app that can be bound easier
APP_NAME="AnchorSelector.app"
APP_PATH="$SCRIPT_DIR/$APP_NAME"

echo "🍎 Creating AppleScript app for easier binding..."
echo "Creating: $APP_PATH"

# Create the app bundle structure
mkdir -p "$APP_PATH/Contents/MacOS"
mkdir -p "$APP_PATH/Contents/Resources"

# Create Info.plist
cat > "$APP_PATH/Contents/Info.plist" << EOF
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleExecutable</key>
    <string>AnchorSelector</string>
    <key>CFBundleIdentifier</key>
    <string>com.local.anchorselector</string>
    <key>CFBundleName</key>
    <string>AnchorSelector</string>
    <key>CFBundlePackageType</key>
    <string>APPL</string>
    <key>CFBundleVersion</key>
    <string>1.0</string>
</dict>
</plist>
EOF

# Create executable script
cat > "$APP_PATH/Contents/MacOS/AnchorSelector" << EOF
#!/bin/bash
exec "$LAUNCHER_PATH"
EOF

chmod +x "$APP_PATH/Contents/MacOS/AnchorSelector"

echo "✅ Created $APP_NAME"
echo ""
echo "🎯 Easy Setup Option:"
echo "===================="
echo "1. Double-click $APP_NAME to test it"
echo "2. If it works, you can bind F10 to this app using:"
echo "   - System Settings > Keyboard > Shortcuts > App Shortcuts"
echo "   - Or use a third-party tool like BetterTouchTool/Karabiner"
echo ""
echo "Test the app now: open '$APP_PATH'"