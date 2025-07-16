#!/bin/bash

# HookAnchor Integration Test Runner
# Runs all integration tests that require the app to be installed and functional

set -e

PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
TESTS_DIR="$PROJECT_ROOT/tests"

echo "🧪 HookAnchor Integration Test Runner"
echo "====================================="

# Check prerequisites
if [[ ! -d "/Applications/HookAnchor.app" ]]; then
    echo "❌ Error: HookAnchor.app not installed"
    echo "   Run: make install"
    exit 1
fi

echo "📋 Prerequisites: ✅ HookAnchor.app installed"
echo ""

# Count integration tests (tests starting with "integration_")
INTEGRATION_COUNT=$(find "$TESTS_DIR" -name "integration_*.rs" | wc -l | tr -d ' ')
echo "🔍 Found $INTEGRATION_COUNT integration test file(s)"

if [[ "$INTEGRATION_COUNT" -eq 0 ]]; then
    echo "⚠️  No integration tests found (looking for integration_*.rs in $TESTS_DIR)"
    exit 0
fi

echo ""
echo "🔨 Building release version (required for grabber tests)..."
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

# Build release version first (required for tests that use --grab)
if ! cargo build --release; then
    echo "❌ Failed to build release version"
    exit 1
fi

echo "✅ Release build completed"
echo ""
echo "⏳ Running integration tests..."
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

# Run all integration tests
cd "$PROJECT_ROOT"

# Find all integration test files and run them
FAILED_TESTS=0
TOTAL_TESTS=0

for test_file in "$TESTS_DIR"/integration_*.rs; do
    if [[ -f "$test_file" ]]; then
        test_name=$(basename "$test_file" .rs)
        TOTAL_TESTS=$((TOTAL_TESTS + 1))
        
        echo ""
        echo "🧪 Running: $test_name"
        echo "────────────────────────────────────────"
        
        # Run cargo test with the test name
        # Use single threading for Finder tests to prevent conflicts
        if [[ "$test_name" == *"finder"* ]]; then
            TEST_CMD="cargo test --test $test_name -- --test-threads=1"
        else
            TEST_CMD="cargo test --test $test_name"
        fi
        
        if $TEST_CMD; then
            echo "✅ $test_name: PASSED"
        else
            echo "❌ $test_name: FAILED"
            FAILED_TESTS=$((FAILED_TESTS + 1))
        fi
    fi
done

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "📊 Integration Test Results"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "   Total tests: $TOTAL_TESTS"
echo "   Passed: $((TOTAL_TESTS - FAILED_TESTS))"
echo "   Failed: $FAILED_TESTS"

if [[ "$FAILED_TESTS" -eq 0 ]]; then
    echo ""
    echo "🎉 All integration tests PASSED!"
    exit 0
else
    echo ""
    echo "💥 $FAILED_TESTS integration test(s) FAILED"
    echo ""
    echo "🔧 Troubleshooting:"
    echo "   • Check that HookAnchor.app is properly installed"
    echo "   • Verify URL scheme registration: open 'hook://test'"
    echo "   • Check app bundle configuration: make test-integration"
    echo "   • View logs: tail -f ~/.anchor.log"
    exit 1
fi