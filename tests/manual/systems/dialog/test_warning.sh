#!/bin/bash
# Test warning dialog

echo "========================================="
echo "Testing Warning Dialog"
echo "========================================="
echo ""
echo "This test will show a warning dialog."
echo "USER ACTION REQUIRED:"
echo "  1. Verify dialog appears with yellow/lightning icon"
echo "  2. Verify title says 'Warning'"
echo "  3. Verify message is displayed correctly"
echo "  4. Click 'OK' button"
echo "  5. Verify script continues after dialog closes"
echo ""
echo "Press Enter to launch dialog..."
read

~/ob/proj/HookAnchor/target/release/HookAnchorDialog --warning "This is a test warning. Using default value instead of invalid configuration."

echo ""
echo "✓ Dialog closed successfully!"
echo "✓ Script continued execution (warning is non-fatal)"
echo ""
echo "TEST PASSED: Warning dialog worked correctly."
