#!/bin/bash

# Launch Hotkey Service for Anchor Selector
# This script starts the global hotkey listener in the background

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
HOTKEY_BINARY="$SCRIPT_DIR/target/release/hotkey_listener"
POPUP_BINARY="$SCRIPT_DIR/target/release/popup"

echo "Anchor Selector Hotkey Service Launcher"
echo "======================================="

# Check if binaries exist
if [ ! -f "$HOTKEY_BINARY" ]; then
    echo "❌ Hotkey listener binary not found: $HOTKEY_BINARY"
    echo "Building hotkey listener..."
    cd "$SCRIPT_DIR"
    cargo build --release --bin hotkey_listener
    if [ $? -ne 0 ]; then
        echo "❌ Build failed"
        exit 1
    fi
fi

if [ ! -f "$POPUP_BINARY" ]; then
    echo "❌ Popup binary not found: $POPUP_BINARY"
    echo "Building popup..."
    cd "$SCRIPT_DIR"
    cargo build --release --bin popup
    if [ $? -ne 0 ]; then
        echo "❌ Build failed"
        exit 1
    fi
fi

echo "✅ Binaries found:"
echo "   Hotkey listener: $HOTKEY_BINARY"
echo "   Popup: $POPUP_BINARY"
echo ""

# Check for existing hotkey listener process
if pgrep -f "hotkey_listener" > /dev/null; then
    echo "⚠️  Hotkey listener already running. Stopping existing instance..."
    pkill -f "hotkey_listener"
    sleep 1
fi

echo "🚀 Starting hotkey listener..."
echo "   Global hotkey: Cmd+Alt+F12"
echo "   Press Cmd+Alt+F12 to launch Anchor Selector"
echo ""
echo "📝 Note: On macOS, you may need to grant Accessibility permissions:"
echo "   System Settings > Privacy & Security > Accessibility"
echo ""

# Start the hotkey listener
exec "$HOTKEY_BINARY"