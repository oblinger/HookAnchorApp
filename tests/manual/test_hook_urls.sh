#!/bin/bash

# Quick manual test script for hook:// URL handling
# This can be run anytime to verify URL handling is working correctly

set -e

PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
LOG_FILE="$HOME/.anchor.log"
APP_PATH="/Applications/HookAnchor.app"

echo "🧪 HookAnchor URL Handling Test"
echo "==============================="

# Check prerequisites
if [[ ! -d "$APP_PATH" ]]; then
    echo "❌ Error: HookAnchor.app not found at $APP_PATH"
    exit 1
fi

if [[ ! -f "$APP_PATH/Contents/MacOS/applet" ]]; then
    echo "❌ Error: applet executable not found"
    exit 1
fi

if [[ ! -f "$APP_PATH/Contents/MacOS/ha" ]]; then
    echo "❌ Error: ha binary not found"
    exit 1
fi

echo "✅ Prerequisites check passed"
echo ""

# Get current log size
INITIAL_LOG_SIZE=0
if [[ -f "$LOG_FILE" ]]; then
    INITIAL_LOG_SIZE=$(wc -c < "$LOG_FILE")
fi

echo "📋 Initial log size: $INITIAL_LOG_SIZE bytes"
echo "🔗 Testing URL: hook://test_manual"
echo ""

# Execute test URL
echo "⏳ Opening hook URL..."
open "hook://test_manual"

# Wait for processing
echo "⏳ Waiting for processing..."
/bin/sleep 3

# Check results
echo ""
echo "📊 Analyzing results..."

if [[ ! -f "$LOG_FILE" ]]; then
    echo "❌ Log file not found - something is wrong"
    exit 1
fi

# Get recent log entries (last 20 lines)
echo ""
echo "📄 Recent log entries:"
echo "----------------------"
tail -n 20 "$LOG_FILE" | grep -E "(URL_HANDLER|POPUP_OPEN|STARTUP|test_manual)" || echo "No relevant entries found"
echo "----------------------"
echo ""

# Check for specific patterns
HAS_URL_HANDLER=$(tail -n 20 "$LOG_FILE" | grep -c "URL_HANDLER.*test_manual" 2>/dev/null || true)
HAS_POPUP_OPEN=$(tail -n 20 "$LOG_FILE" | grep -c "POPUP_OPEN" 2>/dev/null || true) 
HAS_STARTUP=$(tail -n 20 "$LOG_FILE" | grep -c "STARTUP" 2>/dev/null || true)

# Ensure variables are numeric
HAS_URL_HANDLER=${HAS_URL_HANDLER:-0}
HAS_POPUP_OPEN=${HAS_POPUP_OPEN:-0}
HAS_STARTUP=${HAS_STARTUP:-0}

echo "🔍 Analysis Results:"
echo "   URL_HANDLER entries: $HAS_URL_HANDLER"
echo "   POPUP_OPEN entries: $HAS_POPUP_OPEN"
echo "   STARTUP entries: $HAS_STARTUP"
echo ""

# Determine result
if [[ "$HAS_POPUP_OPEN" -gt 0 ]]; then
    echo "❌ FAILURE: Popup opened incorrectly"
    echo "   The URL should execute the command, not open the popup"
    exit 1
elif [[ "$HAS_URL_HANDLER" -eq 0 ]]; then
    echo "❌ FAILURE: No URL_HANDLER entries found"
    echo "   The URL may not have been processed correctly"
    exit 1
elif [[ "$HAS_STARTUP" -gt 0 && "$HAS_URL_HANDLER" -gt 0 ]]; then
    echo "✅ SUCCESS: Hook URL handled correctly"
    echo "   • Binary started"
    echo "   • URL processed by CLI mode"
    echo "   • No popup opened"
    echo "   • Command not found (expected for test command)"
else
    echo "⚠️  WARNING: Unexpected results"
    echo "   Please check the log entries above"
    exit 1
fi

echo ""
echo "🎉 Hook URL test completed successfully!"