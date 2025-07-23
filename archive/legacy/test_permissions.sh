#!/bin/bash

echo "Testing macOS Accessibility Permissions"
echo "======================================="

# Test if we can access accessibility features
echo "Checking current accessibility permissions..."

# Try to read accessibility settings
if sudo sqlite3 /Library/Application\ Support/com.apple.TCC/TCC.db "SELECT service,client,allowed FROM access WHERE service='kTCCServiceAccessibility';" 2>/dev/null | grep -q "$(whoami)"; then
    echo "✅ Found accessibility entries in TCC database"
else
    echo "❌ No accessibility entries found for current user"
fi

echo ""
echo "To grant accessibility permissions:"
echo "1. Go to System Settings > Privacy & Security > Accessibility"
echo "2. Click the '+' button"
echo "3. Add Terminal.app (or the specific hotkey_listener binary)"
echo "4. Ensure the toggle is ON"
echo ""
echo "After granting permissions, try running:"
echo "  ./target/release/rdev_hotkey_listener"
echo ""
echo "Then press F10 to test the hotkey"