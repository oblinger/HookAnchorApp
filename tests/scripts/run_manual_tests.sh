#!/bin/bash

# HookAnchor Manual Test Runner
# Runs all manual/interactive tests that require human verification or external validation

set -e

PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
MANUAL_DIR="$PROJECT_ROOT/tests/manual"

echo "🎯 HookAnchor Manual Test Runner"
echo "================================"

# Check if manual test directory exists
if [[ ! -d "$MANUAL_DIR" ]]; then
    echo "❌ Error: Manual test directory not found: $MANUAL_DIR"
    exit 1
fi

# Count manual tests (macOS compatible find)
MANUAL_COUNT=$(find "$MANUAL_DIR" -name "*.sh" -perm +111 2>/dev/null | wc -l | tr -d ' ')
echo "🔍 Found $MANUAL_COUNT manual test script(s)"

if [[ "$MANUAL_COUNT" -eq 0 ]]; then
    echo "⚠️  No manual test scripts found in $MANUAL_DIR"
    echo "   Manual tests should be executable .sh files"
    exit 0
fi

echo ""
echo "📋 Manual tests found:"
for test_script in "$MANUAL_DIR"/*.sh; do
    if [[ -f "$test_script" && -x "$test_script" ]]; then
        test_name=$(basename "$test_script" .sh)
        echo "   • $test_name"
    fi
done

echo ""
echo "⏳ Running manual tests..."
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

# Run all manual test scripts
FAILED_TESTS=0
TOTAL_TESTS=0

for test_script in "$MANUAL_DIR"/*.sh; do
    if [[ -f "$test_script" && -x "$test_script" ]]; then
        test_name=$(basename "$test_script" .sh)
        TOTAL_TESTS=$((TOTAL_TESTS + 1))
        
        echo ""
        echo "🎯 Running: $test_name"
        echo "────────────────────────────────────────"
        
        if "$test_script"; then
            echo "✅ $test_name: PASSED"
        else
            echo "❌ $test_name: FAILED"
            FAILED_TESTS=$((FAILED_TESTS + 1))
        fi
        
        # Add a brief pause between tests
        /bin/sleep 1
    fi
done

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "📊 Manual Test Results"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "   Total tests: $TOTAL_TESTS"
echo "   Passed: $((TOTAL_TESTS - FAILED_TESTS))"
echo "   Failed: $FAILED_TESTS"

if [[ "$FAILED_TESTS" -eq 0 ]]; then
    echo ""
    echo "🎉 All manual tests PASSED!"
    echo ""
    echo "💡 Manual test tips:"
    echo "   • These tests verify end-to-end functionality"
    echo "   • They test real system integration (URLs, apps, etc.)"
    echo "   • Run them after major changes or before releases"
    exit 0
else
    echo ""
    echo "💥 $FAILED_TESTS manual test(s) FAILED"
    echo ""
    echo "🔧 Troubleshooting:"
    echo "   • Check system requirements (apps installed, permissions)"
    echo "   • Verify HookAnchor.app is properly configured"
    echo "   • Run individual tests for detailed error messages"
    echo "   • Check logs: tail -f ~/.anchor.log"
    exit 1
fi