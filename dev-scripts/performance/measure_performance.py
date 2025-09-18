#!/usr/bin/env python3
"""
Advanced keystroke capture performance tester for HookAnchor
Measures timing, analyzes logs, and provides detailed performance metrics
"""

import subprocess
import time
import re
import os
from datetime import datetime
from pathlib import Path

class KeystrokePerformanceTester:
    def __init__(self):
        self.log_path = Path.home() / ".config/hookanchor/anchor.log"
        self.app_path = "/Applications/HookAnchor.app/Contents/MacOS/popup"
        
    def send_keystroke(self, char, delay=0.01):
        """Send a single keystroke using AppleScript"""
        cmd = f'''osascript -e 'tell application "System Events" to keystroke "{char}"' '''
        subprocess.run(cmd, shell=True, capture_output=True)
        time.sleep(delay)
    
    def send_escape(self):
        """Send Escape key to close popup"""
        cmd = '''osascript -e 'tell application "System Events" to key code 53' '''
        subprocess.run(cmd, shell=True, capture_output=True)
    
    def launch_popup(self):
        """Launch the popup application"""
        return subprocess.Popen([self.app_path])
    
    def get_log_entries_since(self, marker_time):
        """Get log entries since a specific timestamp"""
        if not self.log_path.exists():
            return []
        
        entries = []
        with open(self.log_path, 'r') as f:
            lines = f.readlines()
        
        for line in lines:
            # Parse timestamp from log line
            match = re.match(r'(\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2})', line)
            if match:
                log_time = datetime.strptime(match.group(1), '%Y-%m-%d %H:%M:%S')
                if log_time >= marker_time:
                    entries.append(line.strip())
        
        return entries
    
    def analyze_captured_text(self, log_entries):
        """Analyze log entries to find captured text"""
        captured_text = ""
        buffered_text = ""
        
        # Look for keystroke test logs
        for entry in log_entries:
            if "KEYSTROKE_TEST" in entry:
                if "Captured text:" in entry:
                    # Extract captured text
                    match = re.search(r"Captured text: '([^']*)'", entry)
                    if match:
                        captured_text = match.group(1)
                elif "Buffered text:" in entry:
                    # Extract buffered text
                    match = re.search(r"Buffered text: '([^']*)'", entry)
                    if match:
                        buffered_text = match.group(1)
        
        # Return the most complete text found
        return captured_text or buffered_text
    
    def run_single_test(self, test_name, test_string, keystroke_delay=0.02, launch_delay=0.5):
        """Run a single keystroke capture test"""
        print(f"🧪 Running test: {test_name}")
        print(f"   Test string: '{test_string}'")
        print(f"   Keystroke delay: {keystroke_delay}s")
        print(f"   Launch delay: {launch_delay}s")
        
        # Record start time
        start_time = datetime.now()
        
        # Launch popup
        popup_process = self.launch_popup()
        launch_time = time.time()
        
        # Wait for launch delay
        time.sleep(launch_delay)
        
        # Start sending keystrokes and measure timing
        keystroke_start = time.time()
        for char in test_string:
            self.send_keystroke(char, keystroke_delay)
        keystroke_end = time.time()
        
        # Wait a moment for processing
        time.sleep(0.5)
        
        # Close popup
        self.send_escape()
        time.sleep(0.5)
        
        # Clean up process
        try:
            popup_process.terminate()
            popup_process.wait(timeout=2)
        except:
            try:
                popup_process.kill()
            except:
                pass
        
        # Analyze results
        log_entries = self.get_log_entries_since(start_time)
        captured_text = self.analyze_captured_text(log_entries)
        
        # Calculate metrics
        total_chars = len(test_string)
        captured_chars = len(captured_text)
        success_rate = (captured_chars / total_chars * 100) if total_chars > 0 else 0
        
        # Timing metrics
        total_time = keystroke_end - launch_time
        keystroke_time = keystroke_end - keystroke_start
        
        # Results
        print(f"   📊 Results:")
        print(f"      Sent: {total_chars} chars ('{test_string}')")
        print(f"      Captured: {captured_chars} chars ('{captured_text}')")
        print(f"      Success rate: {success_rate:.1f}%")
        print(f"      Total time: {total_time:.3f}s")
        print(f"      Keystroke time: {keystroke_time:.3f}s")
        
        # Performance rating
        if success_rate >= 95:
            print(f"      🎉 EXCELLENT - Nearly perfect capture!")
        elif success_rate >= 80:
            print(f"      ✅ GOOD - Most keystrokes captured")
        elif success_rate >= 60:
            print(f"      ⚠️ FAIR - Some keystrokes missed")
        else:
            print(f"      ❌ POOR - Many keystrokes missed")
        
        print()
        
        return {
            'test_name': test_name,
            'test_string': test_string,
            'captured_text': captured_text,
            'success_rate': success_rate,
            'total_time': total_time,
            'keystroke_time': keystroke_time,
            'total_chars': total_chars,
            'captured_chars': captured_chars
        }
    
    def run_performance_suite(self):
        """Run a comprehensive performance test suite"""
        print("🚀 HookAnchor Keystroke Capture Performance Suite")
        print("=" * 55)
        print()
        
        tests = [
            ("Slow typing", "hello", 0.1, 0.5),
            ("Normal typing", "world", 0.05, 0.5),
            ("Fast typing", "quick", 0.02, 0.5),
            ("Very fast typing", "speed", 0.01, 0.5),
            ("Immediate typing", "now", 0.02, 0.0),
            ("Background test 1", "bg1", 0.02, 0.1),
            ("Background test 2", "bg2", 0.02, 0.1),
        ]
        
        results = []
        for test_name, test_string, keystroke_delay, launch_delay in tests:
            result = self.run_single_test(test_name, test_string, keystroke_delay, launch_delay)
            results.append(result)
            
            # Brief pause between tests
            time.sleep(1)
        
        # Summary analysis
        print("📈 PERFORMANCE SUMMARY")
        print("=" * 25)
        
        total_tests = len(results)
        avg_success_rate = sum(r['success_rate'] for r in results) / total_tests
        perfect_tests = sum(1 for r in results if r['success_rate'] >= 95)
        good_tests = sum(1 for r in results if r['success_rate'] >= 80)
        
        print(f"Total tests: {total_tests}")
        print(f"Average success rate: {avg_success_rate:.1f}%")
        print(f"Perfect tests (≥95%): {perfect_tests}/{total_tests}")
        print(f"Good tests (≥80%): {good_tests}/{total_tests}")
        
        # Background mode analysis
        background_tests = [r for r in results if 'background' in r['test_name'].lower()]
        if len(background_tests) >= 2:
            bg_avg = sum(r['success_rate'] for r in background_tests) / len(background_tests)
            print(f"Background mode avg: {bg_avg:.1f}%")
        
        print()
        print("💡 Recommendations:")
        if avg_success_rate >= 90:
            print("   🎉 Excellent performance! The app responds very quickly.")
        elif avg_success_rate >= 75:
            print("   ✅ Good performance. Minor optimizations could help.")
        elif avg_success_rate >= 60:
            print("   ⚠️ Fair performance. Consider optimizing startup time.")
        else:
            print("   ❌ Poor performance. Significant optimizations needed.")
            print("   - Try enabling background mode in config")
            print("   - Check for blocking operations during startup")
            print("   - Consider daemon mode improvements")
        
        return results

if __name__ == "__main__":
    tester = KeystrokePerformanceTester()
    results = tester.run_performance_suite()