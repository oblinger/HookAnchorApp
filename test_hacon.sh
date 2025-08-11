#!/bin/bash

# Test specifically for HACON command execution

echo "Testing HACON command execution..."

# Test 1: Direct execution of ha Config
echo -n "Test 1 - Direct execution 'ha Config': "
if ~/bin/ha -r "ha Config" >/dev/null 2>&1; then
    echo "✓ PASSED"
else
    echo "✗ FAILED"
    exit 1
fi

# Test 2: Search execution with hacon
echo -n "Test 2 - Search 'hacon': "
if ~/bin/ha -x "hacon" >/dev/null 2>&1; then
    echo "✓ PASSED"
else
    echo "✗ FAILED"
    exit 1
fi

# Test 3: Search execution with partial match
echo -n "Test 3 - Search 'ha con': "
OUTPUT=$(~/bin/ha -m "ha con" 2>&1 | head -1)
if [[ "$OUTPUT" == *"ha Config"* ]]; then
    echo "✓ PASSED"
else
    echo "✗ FAILED - Output: $OUTPUT"
    exit 1
fi

echo ""
echo "All HACON tests passed!"