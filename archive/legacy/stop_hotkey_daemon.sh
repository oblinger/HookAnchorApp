#!/bin/bash

# Stop Hotkey Service Daemon
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PID_FILE="$SCRIPT_DIR/hotkey_listener.pid"

echo "Stopping Anchor Selector Hotkey Daemon"
echo "======================================="

if [ ! -f "$PID_FILE" ]; then
    echo "❌ No PID file found. Daemon may not be running."
    # Try to kill any running instance anyway
    if pgrep -f "hotkey_listener" > /dev/null; then
        echo "🔍 Found running hotkey_listener process, killing it..."
        pkill -f "hotkey_listener"
        echo "✅ Stopped hotkey listener"
    else
        echo "ℹ️  No hotkey_listener process found"
    fi
    exit 0
fi

PID=$(cat "$PID_FILE")
if ps -p "$PID" > /dev/null 2>&1; then
    echo "🛑 Stopping hotkey listener (PID: $PID)..."
    kill "$PID"
    
    # Wait a moment and check if it stopped
    sleep 1
    if ps -p "$PID" > /dev/null 2>&1; then
        echo "⚠️  Process still running, force killing..."
        kill -9 "$PID"
    fi
    
    rm -f "$PID_FILE"
    echo "✅ Hotkey listener stopped"
else
    echo "❌ Process $PID not found (may have already stopped)"
    rm -f "$PID_FILE"
fi