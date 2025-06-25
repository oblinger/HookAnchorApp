#!/bin/bash

# Start Hotkey Service as Background Daemon
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
HOTKEY_BINARY="$SCRIPT_DIR/target/release/hotkey_listener"
PID_FILE="$SCRIPT_DIR/hotkey_listener.pid"
LOG_FILE="$SCRIPT_DIR/hotkey_listener.log"

echo "Anchor Selector Hotkey Daemon"
echo "============================="

# Check if already running
if [ -f "$PID_FILE" ]; then
    PID=$(cat "$PID_FILE")
    if ps -p "$PID" > /dev/null 2>&1; then
        echo "⚠️  Hotkey listener already running (PID: $PID)"
        echo "   Use ./stop_hotkey_daemon.sh to stop it"
        exit 1
    else
        # Stale PID file
        rm -f "$PID_FILE"
    fi
fi

# Build if needed
if [ ! -f "$HOTKEY_BINARY" ]; then
    echo "Building hotkey listener..."
    cd "$SCRIPT_DIR"
    cargo build --release --bin hotkey_listener popup
fi

echo "🚀 Starting hotkey listener as background daemon..."
echo "   Global hotkey: F10"
echo "   Log file: $LOG_FILE"
echo "   PID file: $PID_FILE"

# Start daemon
nohup "$HOTKEY_BINARY" > "$LOG_FILE" 2>&1 &
PID=$!
echo $PID > "$PID_FILE"

echo "✅ Hotkey listener started (PID: $PID)"
echo "   Press F10 to launch Anchor Selector"
echo "   Use ./stop_hotkey_daemon.sh to stop the service"
echo ""
echo "📝 Note: On macOS, you may need to grant Accessibility permissions:"
echo "   System Settings > Privacy & Security > Accessibility"