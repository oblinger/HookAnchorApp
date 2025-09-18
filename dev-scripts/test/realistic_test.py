#!/usr/bin/env python3
"""Realistic keystroke capture test that simulates actual user behavior"""

import subprocess
import time
import re
import threading
from pathlib import Path

def send_immediate_keystrokes(test_string, start_delay=0.0):
    """Send keystrokes immediately after a small delay (simulating user reaction time)"""
    time.sleep(start_delay)
    
    for char in test_string:
        cmd = f'osascript -e "tell application \\"System Events\\" to keystroke \\"{char}\\""'
        subprocess.run(cmd, shell=True, capture_output=True)
        time.sleep(0.02)  # 20ms between keystrokes (fast typing)

def run_realistic_test(test_string, start_delay=0.0):
    """Run test that simulates real user behavior"""
    print(f"🧪 Testing: '{test_string}' (typing starts after {start_delay*1000:.0f}ms)")
    
    # Clear old logs by adding marker
    log_path = Path.home() / ".config/hookanchor/anchor.log"
    marker = f"REALISTIC_TEST_{int(time.time()*1000) % 100000}"
    
    with open(log_path, 'a') as f:
        f.write(f"2025-07-30 12:35:{int(time.time()) % 60:02d} MARKER: {marker}\n")
    
    # Start keystroke thread (this simulates user starting to type immediately)
    keystroke_thread = threading.Thread(target=send_immediate_keystrokes, args=(test_string, start_delay))
    
    # Record app launch time
    launch_start = time.time()
    
    # Launch app AND start typing thread simultaneously (like real usage)
    process = subprocess.Popen(["/Applications/HookAnchor.app/Contents/MacOS/popup"])
    keystroke_thread.start()
    
    # Wait for keystrokes to complete
    keystroke_thread.join()
    
    # Wait a bit more for processing
    time.sleep(0.5)
    
    # Record total time
    total_time = time.time() - launch_start
    
    # Close app
    subprocess.run('osascript -e "tell application \\"System Events\\" to key code 53"', shell=True)
    time.sleep(0.2)
    
    # Clean up process
    try:
        process.terminate()
        process.wait(timeout=2)
    except:
        try:
            process.kill()
        except:
            pass
    
    # Analyze logs
    with open(log_path, 'r') as f:
        lines = f.readlines()
    
    # Find our marker and get subsequent lines
    marker_found = False
    captured_texts = []
    keystroke_logs = []
    
    for line in lines:
        if marker in line:
            marker_found = True
            continue
        
        if marker_found:
            if "KEYSTROKE_TEST: Captured text:" in line:
                match = re.search(r"Captured text: '([^']*)'", line)
                if match:
                    captured_texts.append(match.group(1))
                    keystroke_logs.append(line.strip())
            elif "KEYSTROKE_TEST: Buffered text:" in line:
                match = re.search(r"Buffered text: '([^']*)'", line)
                if match:
                    captured_texts.append(match.group(1))
                    keystroke_logs.append(line.strip())
    
    # Get final captured text
    final_text = captured_texts[-1] if captured_texts else ""
    
    # Calculate metrics
    sent_chars = len(test_string)
    captured_chars = len(final_text)
    success_rate = (captured_chars / sent_chars * 100) if sent_chars > 0 else 0
    
    # Results
    print(f"   📊 Sent: {sent_chars} chars ('{test_string}')")
    print(f"   ✅ Captured: {captured_chars} chars ('{final_text}')")
    print(f"   📈 Success rate: {success_rate:.1f}%")
    print(f"   ⏱️  Total time: {total_time:.2f}s")
    
    # Show keystroke progression
    if len(captured_texts) > 1:
        print(f"   📝 Keystroke progression: {' → '.join(captured_texts)}")
    
    # Performance analysis
    if final_text == test_string:
        print(f"   🎉 PERFECT MATCH!")
    elif success_rate >= 80:
        print(f"   ✅ GOOD - Most keystrokes captured")
    elif success_rate >= 50:
        print(f"   ⚠️ FAIR - Some keystrokes captured")
    else:
        print(f"   ❌ POOR - Many keystrokes missed")
    
    # Time analysis
    if total_time < 0.5:
        print(f"   ⚡ Very fast response time")
    elif total_time < 1.0:
        print(f"   🏃 Good response time")
    else:
        print(f"   🐌 Slow response time")
    
    print()
    return success_rate, total_time

if __name__ == "__main__":
    print("🚀 Realistic Keystroke Capture Test")
    print("=" * 38)
    print("This test simulates real user behavior:")
    print("- App launches via subprocess (like Karabiner)")
    print("- User starts typing immediately (no wait)")
    print("- Measures actual missed keystrokes")
    print()
    
    test_scenarios = [
        ("immediate", "hello", 0.0),      # Start typing immediately
        ("50ms delay", "world", 0.05),    # Slight human reaction delay
        ("100ms delay", "quick", 0.1),    # Normal human reaction
        ("200ms delay", "test", 0.2),     # Slower reaction
    ]
    
    results = []
    times = []
    
    for name, test_string, start_delay in test_scenarios:
        print(f"📋 Scenario: {name}")
        rate, total_time = run_realistic_test(test_string, start_delay)
        results.append(rate)
        times.append(total_time)
        time.sleep(2)  # Pause between tests
    
    print("📊 SUMMARY")
    print("=" * 15)
    avg_rate = sum(results) / len(results)
    avg_time = sum(times) / len(times)
    
    print(f"Average success rate: {avg_rate:.1f}%")
    print(f"Average response time: {avg_time:.2f}s")
    
    # Find worst case (immediate typing)
    immediate_rate = results[0]
    immediate_time = times[0]
    
    print(f"Immediate typing (worst case): {immediate_rate:.1f}% in {immediate_time:.2f}s")
    
    if immediate_rate >= 90:
        print("🎉 EXCELLENT - App captures keystrokes instantly!")
    elif immediate_rate >= 70:
        print("✅ GOOD - App captures most immediate keystrokes")
    elif immediate_rate >= 50:
        print("⚠️ FAIR - Some immediate keystrokes missed")
    else:
        print("❌ POOR - Many immediate keystrokes missed")
        print("💡 Consider optimizing app startup time")