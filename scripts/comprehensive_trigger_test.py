#!/usr/bin/env python3
"""
Comprehensive test of HookAnchor trigger timing including background daemon functionality.
"""

import subprocess
import time
import os
import sys

def kill_existing_popup():
    """Kill any existing popup processes"""
    try:
        subprocess.run(['pkill', '-f', 'popup'], check=False, capture_output=True)
        time.sleep(1)  # Wait for process to fully terminate
    except:
        pass

def check_background_socket():
    """Check if background daemon socket exists"""
    socket_path = os.path.expanduser("~/Library/Caches/hookanchor-popup.sock")
    exists = os.path.exists(socket_path)
    print(f"🔌 Background socket exists: {exists}")
    return exists

def trigger_via_caps_lock():
    """Trigger via caps lock (Karabiner)"""
    print("🚀 Triggering via Caps Lock")
    subprocess.run([
        'osascript', '-e', 
        'tell application "System Events" to key code 57'  # Caps Lock
    ], check=False, capture_output=True)

def send_character_at_delay(char, delay_ms):
    """Send a character after a specific delay"""
    time.sleep(delay_ms / 1000.0)
    print(f"⌨️  Sending '{char}' at {delay_ms}ms")
    subprocess.run([
        'osascript', '-e', 
        f'tell application "System Events" to keystroke "{char}"'
    ], check=False, capture_output=True)

def read_recent_captures():
    """Read recent keystroke captures from log"""
    log_path = os.path.expanduser("~/.config/hookanchor/anchor.log")
    
    if not os.path.exists(log_path):
        return []
        
    try:
        with open(log_path, 'r') as f:
            lines = f.readlines()
        
        # Look for recent keystroke captures (last 30 lines)
        captured_chars = []
        for line in reversed(lines[-30:]):
            if 'KEYSTROKE_TEST: Captured text:' in line:
                try:
                    captured = line.split("Captured text: '")[1].split("'")[0]
                    captured_chars.append(captured)
                except:
                    pass
        
        return list(reversed(captured_chars))
        
    except Exception as e:
        print(f"❌ Error reading log: {e}")
        return []

def test_scenario(name, trigger_func, test_chars):
    """Test a specific trigger scenario"""
    print(f"\n{'='*50}")
    print(f"🧪 {name}")
    print(f"{'='*50}")
    
    # Kill existing processes
    kill_existing_popup()
    
    # Check for background socket before trigger
    socket_before = check_background_socket()
    
    # Trigger the app
    start_time = time.time()
    trigger_func()
    
    # Send characters at specific delays
    delays = [5, 10, 15, 20, 30, 50, 100, 200, 500, 1000]
    
    for i, char in enumerate(test_chars[:len(delays)]):
        send_character_at_delay(char, delays[i])
    
    # Wait for processing
    time.sleep(2)
    
    # Check for background socket after trigger
    socket_after = check_background_socket()
    
    # Check what was captured
    captured = read_recent_captures()
    relevant_captures = [c for c in captured if any(tc in c for tc in test_chars)]
    
    print(f"📊 Results:")
    print(f"   Socket before: {socket_before}")
    print(f"   Socket after: {socket_after}")
    print(f"   Background daemon: {'✅ Running' if socket_after else '❌ Not running'}")
    
    if relevant_captures:
        first_capture = relevant_captures[0]
        first_char = first_capture[0] if first_capture else ""
        
        if first_char in test_chars:
            first_delay = delays[test_chars.index(first_char)]
            print(f"   ✅ First captured: '{first_capture}' (at {first_delay}ms)")
            print(f"   📈 All captures: {relevant_captures}")
            
            # Determine responsiveness
            if first_delay <= 10:
                print(f"   🚀 EXCELLENT: Responsive within 10ms")
            elif first_delay <= 50:
                print(f"   ✅ GOOD: Responsive within 50ms") 
            elif first_delay <= 200:
                print(f"   ⚠️  SLOW: Responsive at {first_delay}ms")
            else:
                print(f"   ❌ VERY SLOW: {first_delay}ms delay")
        else:
            print(f"   ✅ Captured: {relevant_captures}")
    else:
        print(f"   ❌ No characters captured")
    
    return socket_after, relevant_captures

def main():
    print("🎯 COMPREHENSIVE HOOKANCHOR TRIGGER TEST")
    print("Testing both responsiveness and background daemon functionality")
    
    # Test 1: Fresh start (no background daemon)
    daemon_running, captures1 = test_scenario(
        "Test 1: Fresh Start (Cold Start)",
        trigger_via_caps_lock,
        ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j']
    )
    
    if daemon_running:
        # Test 2: Background daemon response (should be instant)
        print(f"\n⏱️  Waiting 2 seconds, then testing background daemon response...")
        time.sleep(2)
        
        daemon_running2, captures2 = test_scenario(
            "Test 2: Background Daemon Response (Should be Instant)",
            trigger_via_caps_lock,
            ['x', 'y', 'z', '1', '2', '3', '4', '5', '6', '7']
        )
        
        print(f"\n🏁 FINAL ANALYSIS:")
        print(f"   Cold start captures: {captures1}")
        print(f"   Background daemon captures: {captures2}")
        
        if captures2 and len(captures2[0]) > 0:
            first_char_bg = captures2[0][0]
            test_chars_bg = ['x', 'y', 'z', '1', '2', '3', '4', '5', '6', '7']
            delays = [5, 10, 15, 20, 30, 50, 100, 200, 500, 1000]
            
            if first_char_bg in test_chars_bg:
                bg_delay = delays[test_chars_bg.index(first_char_bg)]
                print(f"   🎯 Background daemon first response: {bg_delay}ms")
                
                if bg_delay <= 10:
                    print(f"   🚀 SUCCESS: Background daemon eliminates startup delay!")
                else:
                    print(f"   ⚠️  Background daemon still has {bg_delay}ms delay")
    
    else:
        print(f"\n❌ Background daemon not running - check configuration")
    
    # Cleanup
    kill_existing_popup()

if __name__ == "__main__":
    try:
        main()
    except KeyboardInterrupt:
        print("\n⏹️  Test interrupted")
        kill_existing_popup()
    except Exception as e:
        print(f"\n❌ Test failed: {e}")
        kill_existing_popup()