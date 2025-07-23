#!/bin/bash

echo "=== HookAnchor Grabber Test ==="
echo ""

# First, let's check if the binary exists and has the --grab option
if [[ ! -f "./target/release/ha" ]]; then
    echo "❌ Binary ./target/release/ha not found"
    echo "   Need to build first with: cargo build --release"
    exit 1
fi

echo "✅ Binary found: ./target/release/ha"
echo ""

# Test basic --grab functionality
echo "1. Testing --grab with help..."
./target/release/ha --help | grep grab
echo ""

# Test immediate grab (no delay)
echo "2. Testing immediate grab (will capture current active window)..."
echo "   Make sure Finder is active and has a file selected!"
echo "   Press Enter to continue..."
read -r

echo "Capturing in 3 seconds..."
sleep 1
echo "2..."
sleep 1  
echo "1..."
sleep 1
echo "Capturing now!"

RESULT=$(./target/release/ha --grab 0)
EXIT_CODE=$?

echo ""
echo "=== RESULT ==="
echo "Exit code: $EXIT_CODE"
echo "Output: '$RESULT'"
echo ""

if [[ $EXIT_CODE -eq 0 ]]; then
    echo "✅ Grab succeeded!"
    echo "Action and argument: $RESULT"
else
    echo "❌ Grab failed with exit code $EXIT_CODE"
fi

echo ""
echo "To test automatic file selection, run:"
echo "  ./test_finder_grab.sh"