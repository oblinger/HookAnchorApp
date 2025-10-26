#!/bin/bash
# Test dialog with input fields

echo "========================================="
echo "Testing Dialog with Input Fields"
echo "========================================="
echo ""
echo "This test will show a dialog with input fields."
echo "USER ACTION REQUIRED:"
echo "  1. Verify dialog appears with input field"
echo "  2. Enter some text in the 'name' field"
echo "  3. Click 'Submit' button"
echo "  4. Verify JSON output includes your input"
echo ""
echo "Press Enter to launch dialog..."
read

RESULT=$(~/ob/proj/HookAnchor/target/release/HookAnchorDialog \
    --spec "=User Input Test" \
    --spec "#Please enter your information" \
    --spec "'Enter your name:" \
    --spec "\$name,Your name here" \
    --spec "!Submit" \
    --spec "!Cancel")

echo ""
echo "Dialog result (JSON):"
echo "$RESULT"
echo ""

# Parse the values from JSON
BUTTON=$(echo "$RESULT" | grep -o '"exit":"[^"]*"' | cut -d'"' -f4)
NAME=$(echo "$RESULT" | grep -o '"name":"[^"]*"' | cut -d'"' -f4)

echo "Button clicked: $BUTTON"
echo "Name entered: $NAME"
echo ""

if [ "$BUTTON" = "Submit" ] && [ -n "$NAME" ]; then
    echo "✓ User submitted with name: $NAME"
elif [ "$BUTTON" = "Cancel" ]; then
    echo "✓ User cancelled input"
else
    echo "✗ Unexpected result"
fi

echo ""
echo "TEST COMPLETED: Verify the name matches what you typed."
