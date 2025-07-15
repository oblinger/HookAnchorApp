#!/bin/bash

echo "=== Testing Finder File Selection Grabber ==="
echo ""

# Create test file
TEST_DIR="/tmp/hookanchor_test_manual"
TEST_FILE="$TEST_DIR/test_document.txt"

mkdir -p "$TEST_DIR"
echo "This is a test document" > "$TEST_FILE"

echo "1. Opening Finder and selecting test file..."
osascript -e "
tell application \"Finder\"
    activate
    open folder POSIX file \"$TEST_DIR\"
    delay 0.5
    select POSIX file \"$TEST_FILE\"
    delay 0.5
end tell
"

echo "2. Waiting 2 seconds, then running grabber..."
sleep 2

echo "3. Running grabber..."
echo "Output: "
./target/release/ha --grab 0

echo ""
echo "=== Test Complete ==="
echo ""
echo "Expected output: doc $TEST_FILE"
echo ""
echo "To test with a delay:"
echo "  ./target/release/ha --grab 3   # Wait 3 seconds before capture"