#!/bin/bash

echo "=== Testing HookAnchor --grab after build ==="
echo ""

# First build
echo "1. Building release version..."
chmod +x build_new_release.sh
./build_new_release.sh

if [[ $? -ne 0 ]]; then
    echo "❌ Build failed, cannot test"
    exit 1
fi

echo ""
echo "2. Testing --grab functionality..."
echo ""

# Test that --grab option exists
echo "Checking if --grab option is available:"
./target/release/ha --help | grep grab
if [[ $? -eq 0 ]]; then
    echo "✅ --grab option found!"
else
    echo "❌ --grab option not found - build may not have included our changes"
    exit 1
fi

echo ""
echo "3. Manual test setup:"
echo "   • Open Finder"
echo "   • Select a file (highlight it in blue)"  
echo "   • Make sure Finder is the frontmost application"
echo "   • Then run: ./target/release/ha --grab 0"
echo ""
echo "Expected output if working:"
echo "   doc /path/to/selected/file.txt"
echo ""
echo "Output if still broken:"
echo "   folder /path/to/current/folder/"
echo ""

# Create test file for demonstration
TEST_FILE="/tmp/hookanchor_test_file.txt"
echo "Sample test content" > "$TEST_FILE"
echo "Created test file: $TEST_FILE"
echo "You can navigate to /tmp in Finder and select this file to test."
echo ""
echo "Quick test command:"
echo "  ./target/release/ha --grab 0"