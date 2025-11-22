#!/bin/bash
# Test fatal error dialog

echo "========================================="
echo "Testing Fatal Error Dialog"
echo "========================================="
echo ""
echo "This test will show a fatal error dialog."
echo "USER ACTION REQUIRED:"
echo "  1. Verify dialog appears with red/stop icon"
echo "  2. Verify title says 'Fatal Error'"
echo "  3. Verify message is displayed correctly"
echo "  4. Click 'Exit' button"
echo "  5. Verify script terminates (does not continue)"
echo ""
echo "Press Enter to launch dialog..."
read

~/ob/proj/HookAnchor/HookAnchorApp/target/release/HookAnchorDialog --fatal "This is a test fatal error. The application would terminate after you click Exit."

echo ""
echo "If you see this message, the fatal error did NOT terminate the process!"
echo "TEST FAILED: Fatal error should have stopped execution."
