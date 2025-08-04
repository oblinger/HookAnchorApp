#!/usr/bin/env python3
"""
Test script that mimics the user's exact trigger method and measures character input timing.
This will help identify when the app becomes ready to receive input.
"""

import subprocess
import time
import os
import signal
import sys

def kill_existing_popup():
    """Kill any existing popup processes"""
    try:
        subprocess.run(['pkill', '-f', 'popup'], check=False, capture_output=True)
        time.sleep(0.5)  # Wait for process to fully terminate
    except:
        pass

def trigger_app_via_karabiner():
    """Trigger the app exactly like the user does via Karabiner"""
    print("🚀 Triggering app via Karabiner (simulating caps lock press)")
    # Simulate the caps lock key that triggers Karabiner
    # This should match the user's exact trigger method
    subprocess.run([
        'osascript', '-e', 
        'tell application "System Events" to key code 57'  # Caps Lock key code
    ], check=False, capture_output=True)

def trigger_app_direct():
    """Alternative: trigger app directly"""
    print("🚀 Triggering app directly")
    subprocess.Popen(['/Applications/HookAnchor.app/Contents/MacOS/popup'], 
                     stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)

def send_character_at_delay(char, delay_ms):
    """Send a character after a specific delay"""
    time.sleep(delay_ms / 1000.0)  # Convert ms to seconds
    
    print(f"⌨️  Sending '{char}' at {delay_ms}ms")
    subprocess.run([
        'osascript', '-e', 
        f'tell application "System Events" to keystroke "{char}"'
    ], check=False, capture_output=True)

def read_log_for_captures():
    """Read the log file to see what characters were captured"""
    log_path = os.path.expanduser("~/.config/hookanchor/anchor.log")
    
    if not os.path.exists(log_path):
        print("❌ Log file not found")
        return []
        
    try:
        with open(log_path, 'r') as f:
            lines = f.readlines()
        
        # Look for recent keystroke captures
        captured_chars = []
        for line in reversed(lines[-50:]):  # Check last 50 lines
            if 'KEYSTROKE_TEST: Captured text:' in line:
                # Extract the captured text
                try:
                    captured = line.split("Captured text: '")[1].split("'")[0]
                    captured_chars.append(captured)
                except:
                    pass
        
        return list(reversed(captured_chars))  # Return in chronological order
        
    except Exception as e:
        print(f"❌ Error reading log: {e}")
        return []

def run_timing_test():
    """Run the complete timing test"""
    print("=" * 60)
    print("🧪 HOOKANCHOR TRIGGER TIMING TEST")
    print("=" * 60)
    
    # Kill any existing instances
    kill_existing_popup()
    
    # Clear recent log entries by noting current time
    start_time = time.time()
    
    # Method 1: Try Karabiner trigger (if configured)
    print("\n📋 Test 1: Karabiner Trigger Method")
    try:
        trigger_app_via_karabiner()
        
        # Send characters at different delays
        delays = [10, 20, 30, 50, 100, 200, 500]
        test_chars = ['a', 'b', 'c', 'd', 'e', 'f', 'g']
        
        for i, (char, delay) in enumerate(zip(test_chars, delays)):
            send_character_at_delay(char, delay)
        
        # Wait for processing
        time.sleep(2)
        
        # Check what was captured
        captured = read_log_for_captures()
        if captured:
            print(f"✅ Captured characters: {captured}")
            if captured:
                first_char = captured[0]
                first_delay = delays[test_chars.index(first_char[0])] if first_char and first_char[0] in test_chars else "unknown"
                print(f"🎯 FIRST CAPTURED: '{first_char}' (sent at {first_delay}ms)")
        else:
            print("❌ No characters captured via Karabiner method")
            
    except Exception as e:
        print(f"❌ Karabiner method failed: {e}")
    
    # Wait and kill processes
    time.sleep(1)
    kill_existing_popup()
    time.sleep(1)
    
    # Method 2: Direct trigger
    print("\n📋 Test 2: Direct Trigger Method")
    try:
        trigger_app_direct()
        
        # Send characters at different delays
        delays = [10, 20, 30, 50, 100, 200, 500]
        test_chars = ['x', 'y', 'z', '1', '2', '3', '4']
        
        for i, (char, delay) in enumerate(zip(test_chars, delays)):
            send_character_at_delay(char, delay)
        
        # Wait for processing
        time.sleep(2)
        
        # Check what was captured
        captured = read_log_for_captures()
        if captured:
            # Filter out previous test results
            new_captured = [c for c in captured if any(tc in c for tc in test_chars)]
            print(f"✅ Captured characters: {new_captured}")
            if new_captured:
                first_char = new_captured[0]
                first_delay = delays[test_chars.index(first_char[0])] if first_char and first_char[0] in test_chars else "unknown"
                print(f"🎯 FIRST CAPTURED: '{first_char}' (sent at {first_delay}ms)")
        else:
            print("❌ No characters captured via direct method")
            
    except Exception as e:
        print(f"❌ Direct method failed: {e}")
    
    # Cleanup
    kill_existing_popup()
    
    print("\n" + "=" * 60)
    print("🏁 Test completed! Check the results above.")
    print("💡 The 'FIRST CAPTURED' shows when the app becomes responsive.")
    print("=" * 60)

if __name__ == "__main__":
    try:
        run_timing_test()
    except KeyboardInterrupt:
        print("\n⏹️  Test interrupted by user")
        kill_existing_popup()
        sys.exit(0)
    except Exception as e:
        print(f"\n❌ Test failed: {e}")
        kill_existing_popup()
        sys.exit(1)