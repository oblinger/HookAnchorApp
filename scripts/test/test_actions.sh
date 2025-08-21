#!/bin/bash

# Functional test for HookAnchor actions with grabber verification
# Tests both background commands and application launches

set -e  # Exit on any error

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
TEST_OUTPUT_DIR="/tmp/hookanchor_test_$$"
LOG_FILE="/tmp/hookanchor_test_$$.log"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Track test results
TESTS_PASSED=0
TESTS_FAILED=0

echo "🧪 HookAnchor Action Test Suite"
echo "================================"
echo "Test directory: $TEST_OUTPUT_DIR"
echo "Log file: $LOG_FILE"
echo ""

# Function to log debug info
debug_log() {
    echo "[DEBUG] $1" >> "$LOG_FILE"
}

# Clean up function
cleanup() {
    # Clean up test output directory
    rm -rf "$TEST_OUTPUT_DIR"
    
    # Report results
    echo ""
    echo "================================"
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

# Function to test a shell command that creates a file
test_shell_command() {
    local test_name="$1"
    local command="$2"
    local expected_file="$3"
    local expected_content="$4"
    
    echo -n "Testing $test_name... "
    debug_log "Running test: $test_name"
    
    # Create a temporary command in commands.txt
    local temp_cmd="TEST_${test_name// /_}"
    echo "${temp_cmd} : cmd; $command" >> ~/.config/hookanchor/commands.txt
    debug_log "Added command: ${temp_cmd}"
    
    # Rescan to pick up the new command
    ~/bin/ha --rescan >/dev/null 2>&1
    sleep 1
    
    # Execute the command
    local output
    output=$(~/bin/ha -r "$temp_cmd" 2>&1) || true
    debug_log "Command output: $output"
    
    # Give command time to complete (since they run async)
    sleep 2
    
    # Check if expected file was created
    if [ -f "$expected_file" ]; then
        debug_log "File exists: $expected_file"
        if [ -n "$expected_content" ]; then
            # Check content if specified
            local actual_content=$(cat "$expected_file" 2>/dev/null)
            debug_log "Expected: '$expected_content', Got: '$actual_content'"
            
            if grep -q "$expected_content" "$expected_file" 2>/dev/null; then
                echo -e "${GREEN}✓ PASSED${NC}"
                TESTS_PASSED=$((TESTS_PASSED + 1))
                # Clean up
                rm -f "$expected_file"
                sed -i '' "/^${temp_cmd} :/d" ~/.config/hookanchor/commands.txt
                return 0
            else
                echo -e "${RED}✗ FAILED${NC} - Content mismatch"
                TESTS_FAILED=$((TESTS_FAILED + 1))
                sed -i '' "/^${temp_cmd} :/d" ~/.config/hookanchor/commands.txt
                return 1
            fi
        else
            echo -e "${GREEN}✓ PASSED${NC}"
            TESTS_PASSED=$((TESTS_PASSED + 1))
            rm -f "$expected_file"
            sed -i '' "/^${temp_cmd} :/d" ~/.config/hookanchor/commands.txt
            return 0
        fi
    else
        echo -e "${RED}✗ FAILED${NC} - File not created"
        debug_log "File does not exist: $expected_file"
        TESTS_FAILED=$((TESTS_FAILED + 1))
        sed -i '' "/^${temp_cmd} :/d" ~/.config/hookanchor/commands.txt
        return 1
    fi
}

# Function to test an application launch using grabber
test_app_launch() {
    local test_name="$1"
    local command="$2"
    local expected_app="$3"
    local wait_time="${4:-3}"  # Time to wait for app to launch
    
    echo -n "Testing $test_name (with grabber verification)... "
    debug_log "Running app launch test: $test_name"
    
    # Execute the command
    local output
    output=$(~/bin/ha -r "$command" 2>&1) || true
    debug_log "Command output: $output"
    
    # Wait for app to launch
    debug_log "Waiting ${wait_time}s for app to launch..."
    sleep "$wait_time"
    
    # Use grabber to check active app
    local active_app
    active_app=$(~/bin/ha --grab 0 2>&1 | grep "Active application:" | sed 's/Active application: //')
    debug_log "Grabbed active app: $active_app"
    
    # Check if the expected app is active
    if [[ "$active_app" == *"$expected_app"* ]]; then
        echo -e "${GREEN}✓ PASSED${NC} (Active: $active_app)"
        TESTS_PASSED=$((TESTS_PASSED + 1))
        return 0
    else
        echo -e "${RED}✗ FAILED${NC} - Expected '$expected_app', got '$active_app'"
        TESTS_FAILED=$((TESTS_FAILED + 1))
        return 1
    fi
}

# Function to test 1Password action specifically
test_1password() {
    local test_name="$1"
    local search_term="$2"
    
    echo -n "Testing $test_name... "
    debug_log "Running 1Password test: $test_name with search term: $search_term"
    
    # Execute the 1Password command
    local output
    output=$(~/bin/ha -r "SimpleNote 1Pass" 2>&1) || true
    debug_log "Command output: $output"
    
    # Wait for 1Password Quick Access to appear
    sleep 2
    
    # Check if 1Password is in focus (Quick Access or main app)
    local active_app
    active_app=$(~/bin/ha --grab 0 2>&1 | grep "Active application:" | sed 's/Active application: //')
    debug_log "Active app after 1pass command: $active_app"
    
    if [[ "$active_app" == *"1Password"* ]] || [[ "$active_app" == *"Quick Access"* ]]; then
        echo -e "${GREEN}✓ PASSED${NC} (1Password opened)"
        TESTS_PASSED=$((TESTS_PASSED + 1))
        # Close 1Password Quick Access
        osascript -e 'tell application "System Events" to key code 53' 2>/dev/null  # ESC key
        return 0
    else
        echo -e "${RED}✗ FAILED${NC} - 1Password did not open (Active: $active_app)"
        TESTS_FAILED=$((TESTS_FAILED + 1))
        return 1
    fi
}

echo "================================"
echo "Phase 1: Background Shell Tests"
echo "================================"

# Test basic shell commands
test_shell_command "Echo to file" \
    "echo 'TEST_SUCCESS' > $TEST_OUTPUT_DIR/test_echo.txt" \
    "$TEST_OUTPUT_DIR/test_echo.txt" \
    "TEST_SUCCESS"

test_shell_command "Create file" \
    "touch $TEST_OUTPUT_DIR/test_touch.txt" \
    "$TEST_OUTPUT_DIR/test_touch.txt" \
    ""

test_shell_command "Write date" \
    "date > $TEST_OUTPUT_DIR/test_date.txt" \
    "$TEST_OUTPUT_DIR/test_date.txt" \
    ""

echo ""
echo "================================"
echo "Phase 2: Application Launch Tests"
echo "================================"
echo "Note: These tests will briefly take focus"
echo ""

# Test 1Password action
test_1password "1Password Quick Access" "SimpleNote"

# Test SimpleNote app launch if it exists
if [ -d "/Applications/Simplenote.app" ]; then
    test_app_launch "SimpleNote App" "Simplenote App" "Simplenote" 3
fi

# Test ha Config (should open Sublime Text)
test_app_launch "ha Config (Sublime)" "ha Config" "Sublime Text" 3

echo ""
echo "================================"
echo "Phase 3: Hook URL Tests"
echo "================================"

# Test hook URL for SimpleNote
echo -n "Testing hook://simplenote1pass... "
open "hook://simplenote1pass" 2>/dev/null
sleep 3
active_app=$(~/bin/ha --grab 0 2>&1 | grep "Active application:" | sed 's/Active application: //')
debug_log "Active app after hook URL: $active_app"

if [[ "$active_app" == *"1Password"* ]]; then
    echo -e "${GREEN}✓ PASSED${NC} (1Password opened via hook)"
    TESTS_PASSED=$((TESTS_PASSED + 1))
    osascript -e 'tell application "System Events" to key code 53' 2>/dev/null  # ESC key
else
    echo -e "${RED}✗ FAILED${NC} - Hook did not open 1Password (Active: $active_app)"
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

# Cleanup and report via trap