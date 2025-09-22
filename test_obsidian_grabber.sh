#!/bin/bash

echo "=== Testing Obsidian Grabber ==="
echo "Make sure Obsidian is open and focused on a note, then press Enter to continue..."
read

echo ""
echo "1. Checking if Obsidian is the frontmost app..."
FRONTMOST_APP=$(osascript -e 'tell application "System Events" to get name of first application process whose frontmost is true')
echo "Frontmost app: $FRONTMOST_APP"

if [ "$FRONTMOST_APP" != "Obsidian" ]; then
    echo "⚠️  WARNING: Obsidian is not the frontmost app. Please focus on Obsidian first."
    exit 1
fi

echo ""
echo "2. Getting Obsidian bundle ID..."
BUNDLE_ID=$(osascript -e 'tell application "System Events" to get bundle identifier of first application process whose name is "Obsidian"')
echo "Bundle ID: $BUNDLE_ID"

echo ""
echo "3. Running grabber with debug logging..."
echo "Looking for these key things in the logs:"
echo "   - 'Bundle ID: md.obsidian'"
echo "   - 'Enriching Obsidian context...'"
echo "   - 'Got Obsidian URL: obsidian://...'"
echo "   - 'Evaluating rule Obsidian Note'"
echo ""

# Clear the log marker
echo "=== GRABBER TEST START $(date) ===" >> ~/.config/hookanchor/anchor.log

# Run the grabber
RUST_LOG=debug ~/ob/proj/HookAnchor/target/release/ha -a grab

echo ""
echo "4. Checking the logs for Obsidian-specific activity..."
echo ""

# Show the relevant log entries
tail -200 ~/.config/hookanchor/anchor.log | sed -n '/=== GRABBER TEST START/,$p' | grep -E "GRABBER|GRAB" | grep -E "Bundle ID|Obsidian|md\.obsidian|props\.url|Evaluating rule"

echo ""
echo "=== Test Complete ==="
echo "If you don't see 'Bundle ID: md.obsidian' or 'Enriching Obsidian context', that's the problem!"