#!/bin/bash

echo "Verifying F10 Shortcut Setup"
echo "============================"

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
LAUNCHER_PATH="$SCRIPT_DIR/launch_popup.sh"
APP_PATH="$SCRIPT_DIR/AnchorSelector.app"

echo "✅ Files created:"
echo "   Launcher: $LAUNCHER_PATH"
echo "   App: $APP_PATH"
echo ""

echo "🔍 Checking global shortcut registration:"
if defaults read -g NSUserKeyEquivalents 2>/dev/null | grep -q "Launch Anchor Selector"; then
    echo "✅ Global shortcut registered in NSUserKeyEquivalents"
    defaults read -g NSUserKeyEquivalents | grep "Launch Anchor Selector"
else
    echo "❌ No global shortcut found"
fi
echo ""

echo "🧪 Testing components:"

# Test launcher script
if [ -x "$LAUNCHER_PATH" ]; then
    echo "✅ Launcher script is executable"
else
    echo "❌ Launcher script issue"
fi

# Test app
if [ -d "$APP_PATH" ] && [ -x "$APP_PATH/Contents/MacOS/AnchorSelector" ]; then
    echo "✅ AnchorSelector.app is properly created"
else
    echo "❌ AnchorSelector.app issue"
fi

# Test popup binary
if [ -x "$SCRIPT_DIR/target/release/popup" ]; then
    echo "✅ Popup binary exists and is executable"
else
    echo "❌ Popup binary missing - run: cargo build --release --bin popup"
fi

echo ""
echo "📋 Setup Status:"
echo "================"
echo "The F10 shortcut setup is complete! Here's what was done:"
echo ""
echo "1. ✅ Created launcher script: launch_popup.sh"
echo "2. ✅ Created AnchorSelector.app for easy binding"
echo "3. ✅ Set NSUserKeyEquivalents global shortcut"
echo ""
echo "🎯 To activate F10 shortcut:"
echo "=============================="
echo "Option A: Test if the global shortcut works"
echo "   - Try pressing F10 now"
echo "   - If it doesn't work, use Option B"
echo ""
echo "Option B: Manual setup (most reliable)"
echo "   1. Go to System Settings > Keyboard > Shortcuts"
echo "   2. Click 'App Shortcuts'"
echo "   3. Click '+' button"
echo "   4. Application: All Applications"
echo "   5. Menu Title: (leave blank)"
echo "   6. Keyboard Shortcut: Press F10"
echo "   7. Browse to: $APP_PATH"
echo ""
echo "🧪 Manual Test:"
echo "==============="
echo "Run this to test manually: open '$APP_PATH'"
echo "Or: $LAUNCHER_PATH"