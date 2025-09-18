#!/bin/bash
# Test script to measure keystroke capture performance

echo "🧪 Testing HookAnchor keystroke capture performance..."

# Function to send keystrokes using AppleScript
send_keystrokes() {
    local text="$1"
    local delay="$2"
    
    for (( i=0; i<${#text}; i++ )); do
        char="${text:$i:1}"
        osascript -e "tell application \"System Events\" to keystroke \"$char\""
        sleep "$delay"
    done
}

# Function to send Escape key
send_escape() {
    osascript -e 'tell application "System Events" to key code 53' # Escape key
}

# Test function
run_test() {
    local test_name="$1"
    local test_string="$2"
    local keystroke_delay="$3"
    local launch_delay="$4"
    
    echo "📊 Test: $test_name"
    echo "   String: '$test_string'"
    echo "   Keystroke delay: ${keystroke_delay}s"
    echo "   Launch delay: ${launch_delay}s"
    
    # Clear previous log entries by adding a marker
    echo "$(date '+%Y-%m-%d %H:%M:%S') TEST_MARKER: Starting test '$test_name'" >> ~/.config/hookanchor/anchor.log
    
    # Launch the popup
    /Applications/HookAnchor.app/Contents/MacOS/popup &
    popup_pid=$!
    
    # Wait for specified launch delay
    sleep "$launch_delay"
    
    # Send keystrokes
    send_keystrokes "$test_string" "$keystroke_delay"
    
    # Wait a moment for processing
    sleep 0.5
    
    # Send escape to close
    send_escape
    
    # Wait for popup to close
    sleep 0.5
    
    # Analyze results
    echo "   Results:"
    
    # Extract log entries since our marker
    log_entries=$(sed -n '/TEST_MARKER.*'"$test_name"'/,$p' ~/.config/hookanchor/anchor.log | tail -n +2)
    
    # Count characters that made it into search
    captured_chars=""
    if echo "$log_entries" | grep -q "ALWAYS update search results"; then
        # Look for search updates
        search_updates=$(echo "$log_entries" | grep "update_search" | tail -1)
        if [[ ! -z "$search_updates" ]]; then
            # Extract the search text from the log (this might need adjustment based on log format)
            captured_chars=$(echo "$search_updates" | grep -o "search_text.*" | cut -d"'" -f2 2>/dev/null || echo "")
        fi
    fi
    
    # Calculate success metrics
    total_chars=${#test_string}
    captured_count=${#captured_chars}
    success_rate=$(( captured_count * 100 / total_chars ))
    
    echo "   📈 Sent: $total_chars chars ('$test_string')"
    echo "   ✅ Captured: $captured_count chars ('$captured_chars')"
    echo "   📊 Success rate: $success_rate%"
    
    if [[ $success_rate -eq 100 ]]; then
        echo "   🎉 PERFECT - All keystrokes captured!"
    elif [[ $success_rate -ge 80 ]]; then
        echo "   ✅ GOOD - Most keystrokes captured"
    elif [[ $success_rate -ge 50 ]]; then
        echo "   ⚠️ FAIR - Some keystrokes missed"
    else
        echo "   ❌ POOR - Many keystrokes missed"
    fi
    
    echo ""
    
    # Clean up popup process if still running
    if kill -0 $popup_pid 2>/dev/null; then
        kill $popup_pid 2>/dev/null
    fi
    
    return $success_rate
}

# Run multiple test scenarios
echo "🚀 Starting keystroke capture tests..."
echo ""

# Test 1: Slow typing (simulates careful user)
run_test "Slow typing" "hello" 0.1 0.5

# Test 2: Normal typing speed
run_test "Normal typing" "world" 0.05 0.5

# Test 3: Fast typing (simulates power user)
run_test "Fast typing" "quick" 0.02 0.5

# Test 4: Very fast typing (stress test)
run_test "Very fast typing" "test" 0.01 0.5

# Test 5: Immediate typing (worst case - user types before app loads)
run_test "Immediate typing" "immediate" 0.02 0.0

# Test 6: Background mode test (if enabled)
echo "🔄 Testing background mode (second launch should be faster)..."
run_test "Background mode" "bg" 0.02 0.1

echo "🏁 Test completed!"
echo ""
echo "💡 Tips for improvement:"
echo "   - Success rate < 80%: Consider optimizing startup time"
echo "   - Background mode should have higher success rate than first launch"
echo "   - Check logs in ~/.config/hookanchor/anchor.log for detailed analysis"