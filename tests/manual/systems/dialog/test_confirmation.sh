#!/bin/bash
# Test confirmation dialog with custom spec strings

echo "========================================="
echo "Testing Confirmation Dialog"
echo "========================================="
echo ""
echo "This test will show a custom confirmation dialog."
echo "USER ACTION REQUIRED:"
echo "  1. Verify dialog appears with custom title and message"
echo "  2. Verify 'Yes' and 'No' buttons are shown"
echo "  3. Click either button"
echo "  4. Verify JSON output shows which button was clicked"
echo ""
echo "Press Enter to launch dialog..."
read

RESULT=$(~/ob/proj/HookAnchor/HookAnchorApp/target/release/HookAnchorDialog \
    --spec "=Confirm Delete" \
    --spec "#Are you sure?" \
    --spec "'This action cannot be undone" \
    --spec "!Yes" \
    --spec "!No")

echo ""
echo "Dialog result (JSON):"
echo "$RESULT"
echo ""

# Parse the exit button from JSON
BUTTON=$(echo "$RESULT" | grep -o '"exit":"[^"]*"' | cut -d'"' -f4)

echo "Button clicked: $BUTTON"
echo ""

if [ "$BUTTON" = "Yes" ]; then
    echo "✓ User clicked 'Yes' - confirmed action"
elif [ "$BUTTON" = "No" ]; then
    echo "✓ User clicked 'No' - cancelled action"
else
    echo "✗ Unexpected result: $BUTTON"
fi

echo ""
echo "TEST COMPLETED: Check that button value matches what you clicked."
