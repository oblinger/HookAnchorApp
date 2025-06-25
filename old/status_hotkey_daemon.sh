#!/bin/bash

# Check Hotkey Service Daemon Status
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PID_FILE="$SCRIPT_DIR/hotkey_listener.pid"
LOG_FILE="$SCRIPT_DIR/hotkey_listener.log"

echo "Anchor Selector Hotkey Daemon Status"
echo "====================================="

if [ -f "$PID_FILE" ]; then
    PID=$(cat "$PID_FILE")
    if ps -p "$PID" > /dev/null 2>&1; then
        echo "✅ Hotkey listener is running (PID: $PID)"
        echo "   Global hotkey: F10"
        echo "   Press F10 to test"
        
        if [ -f "$LOG_FILE" ]; then
            echo ""
            echo "📋 Recent log entries:"
            tail -n 5 "$LOG_FILE" | sed 's/^/   /'
        fi
    else
        echo "❌ Hotkey listener not running (stale PID file)"
        rm -f "$PID_FILE"
    fi
else
    if pgrep -f "hotkey_listener" > /dev/null; then
        echo "⚠️  Hotkey listener running but no PID file found"
        echo "   Use ./stop_hotkey_daemon.sh to stop it"
    else
        echo "⏹️  Hotkey listener not running"
        echo "   Use ./start_hotkey_daemon.sh to start it"
    fi
fi