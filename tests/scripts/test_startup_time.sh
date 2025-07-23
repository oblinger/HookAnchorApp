#!/bin/bash

echo "Testing startup time with early UI display..."

# Clear debug log
rm -f /tmp/ha_debug.log

# Start timer
START_TIME=$(date +%s.%N)

# Launch HookAnchor and immediately send escape to close it
timeout 2s ./target/release/ha &
PID=$!

# Wait a tiny bit for UI to appear
/bin/sleep 0.1

# Kill the process
kill $PID 2>/dev/null

# Calculate elapsed time
END_TIME=$(date +%s.%N)
ELAPSED=$(echo "$END_TIME - $START_TIME" | bc)

echo "Total time to show UI: ${ELAPSED} seconds"
echo ""
echo "Debug log excerpts:"
grep -E "(STARTUP|DEFERRED_LOAD)" /tmp/ha_debug.log | tail -20