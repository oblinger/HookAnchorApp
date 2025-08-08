#!/bin/bash

# Test script for Swift supervisor

echo "Testing Swift Supervisor Implementation"
echo "======================================="

# Kill any existing popup processes
echo "Cleaning up existing processes..."
pkill -f "popup_server" 2>/dev/null
pkill -f "supervisor" 2>/dev/null
/bin/sleep 1

# Start the Swift supervisor
echo "Starting Swift supervisor..."
./target/release/supervisor &
SUPERVISOR_PID=$!
echo "Supervisor started with PID: $SUPERVISOR_PID"

# Wait a moment for it to initialize
/bin/sleep 2

# Check if supervisor is running
if ps -p $SUPERVISOR_PID > /dev/null; then
    echo "✅ Supervisor is running"
else
    echo "❌ Supervisor failed to start"
    exit 1
fi

# Check if popup_server was launched
POPUP_PID=$(ps aux | grep -E "popup_server.*--server" | grep -v grep | awk '{print $2}')
if [ -n "$POPUP_PID" ]; then
    echo "✅ Popup server launched by supervisor (PID: $POPUP_PID)"
else
    echo "❌ Popup server not found"
fi

echo ""
echo "To trigger the popup window:"
echo "  open -a ./target/release/supervisor"
echo ""
echo "To stop the test:"
echo "  kill $SUPERVISOR_PID"
echo ""
echo "Test complete. Supervisor is running in background."