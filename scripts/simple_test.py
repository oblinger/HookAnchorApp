#!/usr/bin/env python3
"""Simple, accurate keystroke capture test"""

import subprocess
import time
import re
from pathlib import Path

def run_test(test_string, delay=0.02):
    print(f"🧪 Testing: '{test_string}' (delay: {delay}s)")
    
    # Clear old logs by adding marker
    log_path = Path.home() / ".config/hookanchor/anchor.log"
    marker = f"TEST_START_{int(time.time())}"
    
    with open(log_path, 'a') as f:
        f.write(f"2025-07-30 12:30:{int(time.time()) % 60:02d} MARKER: {marker}\n")
    
    # Launch app
    process = subprocess.Popen(["/Applications/HookAnchor.app/Contents/MacOS/popup"])
    time.sleep(0.3)  # Wait for startup
    
    # Send keystrokes
    for char in test_string:
        cmd = f'osascript -e "tell application \\"System Events\\" to keystroke \\"{char}\\""'
        subprocess.run(cmd, shell=True)
        time.sleep(delay)
    
    time.sleep(0.2)  # Wait for processing
    
    # Close
    subprocess.run('osascript -e "tell application \\"System Events\\" to key code 53"', shell=True)
    time.sleep(0.3)
    
    # Clean up
    try:
        process.terminate()
        process.wait(timeout=2)
    except:
        pass
    
    # Analyze logs
    with open(log_path, 'r') as f:
        lines = f.readlines()
    
    # Find our marker and get subsequent lines
    marker_found = False
    captured_texts = []
    
    for line in lines:
        if marker in line:
            marker_found = True
            continue
        
        if marker_found and "KEYSTROKE_TEST: Captured text:" in line:
            match = re.search(r"Captured text: '([^']*)'", line)
            if match:
                captured_texts.append(match.group(1))
    
    # Get final captured text
    final_text = captured_texts[-1] if captured_texts else ""
    
    # Results
    sent_chars = len(test_string)
    captured_chars = len(final_text)
    success_rate = (captured_chars / sent_chars * 100) if sent_chars > 0 else 0
    
    print(f"   📊 Sent: {sent_chars} chars ('{test_string}')")
    print(f"   ✅ Captured: {captured_chars} chars ('{final_text}')")
    print(f"   📈 Success rate: {success_rate:.1f}%")
    
    # Check for exact match
    if final_text == test_string:
        print(f"   🎉 PERFECT MATCH!")
    elif success_rate >= 80:
        print(f"   ✅ GOOD capture rate")
    else:
        print(f"   ⚠️ Some keystrokes missed")
    
    print()
    return success_rate

if __name__ == "__main__":
    print("🚀 Simple Keystroke Capture Test")
    print("=" * 35)
    
    tests = [
        ("hello", 0.05),
        ("world", 0.03), 
        ("quick", 0.02),
        ("fast", 0.01),
    ]
    
    results = []
    for test_string, delay in tests:
        rate = run_test(test_string, delay)
        results.append(rate)
        time.sleep(1)  # Pause between tests
    
    avg_rate = sum(results) / len(results)
    print(f"📊 Average success rate: {avg_rate:.1f}%")
    
    if avg_rate >= 95:
        print("🎉 EXCELLENT performance!")
    elif avg_rate >= 80:
        print("✅ GOOD performance!")
    else:
        print("⚠️ Needs improvement")