#!/bin/bash

echo "🔍 Debug App Shortcuts Configuration"
echo "====================================="

echo "1. Checking current NSUserKeyEquivalents settings:"
echo "=================================================="
defaults read -g NSUserKeyEquivalents 2>/dev/null || echo "No global shortcuts found"

echo ""
echo "2. Checking AnchorSelector app shortcut specifically:"
echo "===================================================="
defaults read -g NSUserKeyEquivalents 2>/dev/null | grep -i anchor || echo "No AnchorSelector shortcuts found"

echo ""
echo "3. Testing AnchorSelector app bundle:"
echo "====================================="
APP_PATH="/Applications/AnchorSelector.app"
if [ -d "$APP_PATH" ]; then
    echo "✅ App exists: $APP_PATH"
    echo "   Bundle executable: $APP_PATH/Contents/MacOS/AnchorSelector"
    if [ -x "$APP_PATH/Contents/MacOS/AnchorSelector" ]; then
        echo "   ✅ Executable is valid"
    else
        echo "   ❌ Executable not found or not executable"
    fi
else
    echo "❌ App not found: $APP_PATH"
fi

echo ""
echo "4. Testing direct execution:"
echo "============================"
echo "Testing launcher script..."
LAUNCHER="$PWD/launch_popup.sh"
if [ -x "$LAUNCHER" ]; then
    echo "✅ Launcher script exists and is executable"
    echo "   Path: $LAUNCHER"
else
    echo "❌ Launcher script issue: $LAUNCHER"
fi

echo ""
echo "5. App Shortcut Requirements:"
echo "============================="
echo "For App Shortcuts to work, the app needs:"
echo "1. To be in /Applications (✅ done)"
echo "2. To have a menu or be 'shortcut-able' (❓ check this)"
echo "3. The shortcut format must match exactly"
echo ""

echo "🔧 Alternative Approach - Create a proper menu item:"
echo "===================================================="
echo "App Shortcuts work by triggering menu items. Since AnchorSelector"
echo "doesn't have a menu, the shortcut may not work."
echo ""
echo "Let's try a different approach using Services instead:"
echo ""

echo "6. Creating a Service-based shortcut:"
echo "====================================="

SERVICE_DIR="$HOME/Library/Services"
SERVICE_NAME="Launch Anchor Selector.workflow"
SERVICE_PATH="$SERVICE_DIR/$SERVICE_NAME"

mkdir -p "$SERVICE_DIR"

if [ ! -d "$SERVICE_PATH" ]; then
    echo "Creating Service workflow..."
    
    # Create basic Automator service structure
    mkdir -p "$SERVICE_PATH/Contents"
    
    # Create a minimal Info.plist for the service
    cat > "$SERVICE_PATH/Contents/Info.plist" << 'EOF'
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleIdentifier</key>
    <string>com.apple.Automator.LaunchAnchorSelector</string>
    <key>CFBundleName</key>
    <string>Launch Anchor Selector</string>
    <key>CFBundleVersion</key>
    <string>1.0</string>
</dict>
</plist>
EOF

    echo "✅ Service structure created"
    echo ""
    echo "📝 To complete the Service setup:"
    echo "1. Open Automator"
    echo "2. Choose 'Quick Action' (or 'Service' in older macOS)"
    echo "3. Drag 'Run Shell Script' from the library"
    echo "4. Set the script to: $LAUNCHER"
    echo "5. Save as 'Launch Anchor Selector'"
    echo "6. Go to System Settings > Keyboard > Shortcuts > Services"
    echo "7. Find 'Launch Anchor Selector' and assign Cmd+F9"
    
else
    echo "✅ Service already exists: $SERVICE_PATH"
fi

echo ""
echo "🎯 Immediate Test Options:"
echo "=========================="
echo "1. Test app directly: open '$APP_PATH'"
echo "2. Test launcher: $LAUNCHER"
echo "3. Check if Cmd+F9 works now"
echo ""
echo "If none work, the issue might be that App Shortcuts"
echo "don't work with apps that don't have menus."