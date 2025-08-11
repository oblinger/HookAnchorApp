#!/bin/bash

# Functional test for HookAnchor command execution pipeline
# Completely self-contained - returns success/failure for each test

set -e  # Exit on any error

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
TEST_OUTPUT_DIR="/tmp/hookanchor_test_$$"  # Use PID for unique dir
COMMANDS_FILE="$HOME/.config/hookanchor/commands.txt"
COMMANDS_BACKUP="$HOME/.config/hookanchor/commands.txt.test_backup_$$"
LOG_FILE="/tmp/hookanchor_test_$$.log"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Track test results
TESTS_PASSED=0
TESTS_FAILED=0

echo "🧪 HookAnchor Command Execution Test Suite"
echo "=========================================="
echo "Test directory: $TEST_OUTPUT_DIR"
echo "Log file: $LOG_FILE"
echo ""

# Function to log debug info
debug_log() {
    echo "[DEBUG] $1" >> "$LOG_FILE"
}

# Clean up function
cleanup() {
    # Restore original commands.txt
    if [ -f "$COMMANDS_BACKUP" ]; then
        # Remove test commands (everything after our marker)
        sed -i '' '/# TEST COMMANDS - AUTO GENERATED/,$d' "$COMMANDS_FILE" 2>/dev/null || true
        rm -f "$COMMANDS_BACKUP"
        debug_log "Cleaned up test commands from commands.txt"
    fi
    
    # Clean up test output directory
    rm -rf "$TEST_OUTPUT_DIR"
    
    # Report results
    echo ""
    echo "=========================================="
    echo "Test Results:"
    echo -e "${GREEN}Passed: $TESTS_PASSED${NC}"
    echo -e "${RED}Failed: $TESTS_FAILED${NC}"
    
    if [ $TESTS_FAILED -eq 0 ]; then
        echo -e "${GREEN}✓ All tests passed!${NC}"
        rm -f "$LOG_FILE"  # Clean up log if all tests passed
        exit 0
    else
        echo -e "${RED}✗ Some tests failed. Check log at: $LOG_FILE${NC}"
        exit 1
    fi
}

# Set up trap to ensure cleanup happens
trap cleanup EXIT

# Initialize
rm -rf "$TEST_OUTPUT_DIR"
mkdir -p "$TEST_OUTPUT_DIR"
debug_log "Created test directory: $TEST_OUTPUT_DIR"

# Backup existing commands.txt
if [ -f "$COMMANDS_FILE" ]; then
    cp "$COMMANDS_FILE" "$COMMANDS_BACKUP"
    debug_log "Backed up commands.txt to $COMMANDS_BACKUP"
fi

# Add test commands to commands.txt
{
    echo ""
    echo "# TEST COMMANDS - AUTO GENERATED"
    echo "TEST_ECHO : cmd; echo 'TEST_SUCCESS' > $TEST_OUTPUT_DIR/test_echo.txt"
    echo "TEST_TOUCH : cmd; touch $TEST_OUTPUT_DIR/test_touch_file.txt"
    echo "TEST_WRITE : cmd; echo 'Content from TEST_WRITE' > $TEST_OUTPUT_DIR/test_write.txt"
    echo "TEST_DATE : cmd; date > $TEST_OUTPUT_DIR/test_date.txt"
    echo "TEST_PWD : cmd; pwd > $TEST_OUTPUT_DIR/test_pwd.txt"
    echo "TEST_MULTI : cmd; echo 'Line1' > $TEST_OUTPUT_DIR/test_multi.txt && echo 'Line2' >> $TEST_OUTPUT_DIR/test_multi.txt"
    echo "TEST_SHELL : shell; echo 'SHELL_SUCCESS' > $TEST_OUTPUT_DIR/test_shell.txt"
} >> "$COMMANDS_FILE"

debug_log "Added test commands to commands.txt"

# Force a rescan to pick up the new commands
~/bin/ha --rescan >/dev/null 2>&1

# Give the system a moment to complete rescan
sleep 2

# Function to run a test
run_test() {
    local test_name="$1"
    local command="$2"
    local expected_file="$3"
    local expected_content="$4"
    local test_flag="$5"  # -r or -x
    
    echo -n "Testing $test_name (flag: $test_flag)... "
    debug_log "Running test: $test_name with command: $command"
    
    # Run the command and capture output
    local output
    local exit_code
    output=$(~/bin/ha "$test_flag" "$command" 2>&1) || exit_code=$?
    
    debug_log "Command output: $output"
    debug_log "Exit code: ${exit_code:-0}"
    
    # Give command time to complete (since they run async)
    # Commands are spawned in background, so we need to wait
    sleep 2
    
    # Check if expected file was created
    if [ -f "$expected_file" ]; then
        debug_log "File exists: $expected_file"
        if [ -n "$expected_content" ]; then
            # Check content if specified
            local actual_content=$(cat "$expected_file" 2>/dev/null)
            debug_log "Expected content: '$expected_content'"
            debug_log "Actual content: '$actual_content'"
            
            if grep -q "$expected_content" "$expected_file" 2>/dev/null; then
                echo -e "${GREEN}✓ PASSED${NC}"
                TESTS_PASSED=$((TESTS_PASSED + 1))
                return 0
            else
                echo -e "${RED}✗ FAILED${NC} - Content mismatch"
                echo "  Expected: '$expected_content'"
                echo "  Got: '$actual_content'"
                TESTS_FAILED=$((TESTS_FAILED + 1))
                return 1
            fi
        else
            echo -e "${GREEN}✓ PASSED${NC}"
            TESTS_PASSED=$((TESTS_PASSED + 1))
            return 0
        fi
    else
        echo -e "${RED}✗ FAILED${NC} - File not created: $expected_file"
        debug_log "File does not exist: $expected_file"
        debug_log "Directory contents: $(ls -la $TEST_OUTPUT_DIR 2>&1 || echo 'Dir not accessible')"
        TESTS_FAILED=$((TESTS_FAILED + 1))
        return 1
    fi
}

echo "Running Direct Execution Tests (-r flag):"
echo "-----------------------------------------"

# Run direct execution tests
run_test "TEST_ECHO" "TEST_ECHO" "$TEST_OUTPUT_DIR/test_echo.txt" "TEST_SUCCESS" "-r"
run_test "TEST_TOUCH" "TEST_TOUCH" "$TEST_OUTPUT_DIR/test_touch_file.txt" "" "-r"
run_test "TEST_WRITE" "TEST_WRITE" "$TEST_OUTPUT_DIR/test_write.txt" "Content from TEST_WRITE" "-r"
run_test "TEST_DATE" "TEST_DATE" "$TEST_OUTPUT_DIR/test_date.txt" "" "-r"
run_test "TEST_PWD" "TEST_PWD" "$TEST_OUTPUT_DIR/test_pwd.txt" "" "-r"
run_test "TEST_MULTI" "TEST_MULTI" "$TEST_OUTPUT_DIR/test_multi.txt" "Line1" "-r"
run_test "TEST_SHELL" "TEST_SHELL" "$TEST_OUTPUT_DIR/test_shell.txt" "SHELL_SUCCESS" "-r"

# Clear output directory for search tests
rm -rf "$TEST_OUTPUT_DIR"/*

echo ""
echo "Running Search Execution Tests (-x flag):"
echo "-----------------------------------------"

# Run search execution tests
run_test "TEST_ECHO (search)" "test_echo" "$TEST_OUTPUT_DIR/test_echo.txt" "TEST_SUCCESS" "-x"
run_test "TEST_TOUCH (search)" "test_touch" "$TEST_OUTPUT_DIR/test_touch_file.txt" "" "-x"
run_test "TEST_WRITE (search)" "test_write" "$TEST_OUTPUT_DIR/test_write.txt" "Content from TEST_WRITE" "-x"

echo ""
echo "Testing Real Commands:"
echo "----------------------"

# Test ha Config command - just check it runs without error
echo -n "Testing 'ha Config' via -r flag... "
if ~/bin/ha -r "ha Config" >/dev/null 2>&1; then
    echo -e "${GREEN}✓ PASSED${NC}"
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    echo -e "${RED}✗ FAILED${NC}"
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

echo -n "Testing 'hacon' search via -x flag... "
if ~/bin/ha -x "hacon" >/dev/null 2>&1; then
    echo -e "${GREEN}✓ PASSED${NC}"
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    echo -e "${RED}✗ FAILED${NC}"
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

# Test will cleanup and report via trap