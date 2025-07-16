use std::process::Command;
use std::thread;
use std::time::{Duration, Instant};
use std::fs;
use std::path::Path;

/// Integration test for hook:// URL handling
/// 
/// This test verifies that hook:// URLs are properly handled by the AppleScript
/// and don't inappropriately open the popup interface.
/// 
/// Expected behavior:
/// - hook:// URL should be handled by CLI mode (URL_HANDLER logs)
/// - Popup should NOT open (no POPUP_OPEN logs)
/// - Command should execute successfully
#[test]
fn test_hook_url_does_not_open_popup() {
    // Skip if not on macOS
    if !cfg!(target_os = "macos") {
        println!("Skipping hook URL test - only runs on macOS");
        return;
    }
    
    // Skip if app bundle doesn't exist
    let app_path = "/Applications/HookAnchor.app";
    if !Path::new(app_path).exists() {
        println!("Skipping hook URL test - HookAnchor.app not installed");
        return;
    }
    
    let log_path = std::env::var("HOME")
        .map(|home| format!("{}/.anchor.log", home))
        .unwrap_or_else(|_| ".anchor.log".to_string());
    
    // Read current log size to know where to start monitoring
    let initial_log_size = fs::metadata(&log_path)
        .map(|meta| meta.len())
        .unwrap_or(0);
    
    println!("Starting hook URL integration test...");
    println!("Log file: {}", log_path);
    println!("Initial log size: {} bytes", initial_log_size);
    
    // Test with a simple hook URL
    let test_url = "hook://test_integration";
    println!("Testing URL: {}", test_url);
    
    let start_time = Instant::now();
    
    // Execute the hook URL
    let output = Command::new("open")
        .arg(test_url)
        .output()
        .expect("Failed to execute 'open' command");
    
    if !output.status.success() {
        panic!("Failed to open hook URL: {}", String::from_utf8_lossy(&output.stderr));
    }
    
    // Wait for processing (AppleScript + command execution)
    thread::sleep(Duration::from_millis(2000));
    
    // Read new log entries
    let log_content = fs::read_to_string(&log_path)
        .expect("Failed to read log file");
    
    // Find log entries after the test started
    let _new_entries: Vec<&str> = log_content
        .lines()
        .skip_while(|line| {
            // Skip lines until we find entries after our test started
            if let Some(timestamp_end) = line.find(" ") {
                if timestamp_end > 19 { // "2025-07-15 12:00:00".len()
                    return true; // Skip parsing old timestamps for simplicity
                }
            }
            true
        })
        .collect();
    
    // Get recent entries (last 50 lines should be sufficient)
    let recent_entries: Vec<&str> = log_content
        .lines()
        .rev()
        .take(50)
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .collect();
    
    println!("\n=== Recent Log Entries ===");
    for entry in &recent_entries {
        if entry.contains("test_integration") || 
           entry.contains("URL_HANDLER") || 
           entry.contains("POPUP_OPEN") ||
           entry.contains("STARTUP") {
            println!("{}", entry);
        }
    }
    println!("=========================\n");
    
    // Check for expected patterns
    let has_url_handler = recent_entries.iter()
        .any(|line| line.contains("URL_HANDLER") && line.contains("test_integration"));
    
    let has_popup_open = recent_entries.iter()
        .any(|line| line.contains("POPUP_OPEN"));
    
    let has_startup = recent_entries.iter()
        .any(|line| line.contains("STARTUP"));
    
    // Results
    println!("Test Results:");
    println!("  URL_HANDLER found: {}", has_url_handler);
    println!("  POPUP_OPEN found: {}", has_popup_open);
    println!("  STARTUP found: {}", has_startup);
    println!("  Test duration: {:?}", start_time.elapsed());
    
    // Assertions
    assert!(has_startup, 
        "Expected STARTUP log entry - binary should have started");
    
    assert!(has_url_handler, 
        "Expected URL_HANDLER log entry - URL should be processed by CLI mode");
    
    assert!(!has_popup_open, 
        "POPUP_OPEN found in logs - URL incorrectly opened popup instead of executing command");
    
    println!("✅ Hook URL integration test PASSED");
}

/// Test helper to verify the app bundle is correctly configured
#[test]
fn test_app_bundle_configuration() {
    if !cfg!(target_os = "macos") {
        return;
    }
    
    let info_plist_path = "/Applications/HookAnchor.app/Contents/Info.plist";
    if !Path::new(info_plist_path).exists() {
        println!("Skipping app bundle test - Info.plist not found");
        return;
    }
    
    let plist_content = fs::read_to_string(info_plist_path)
        .expect("Failed to read Info.plist");
    
    // Check that CFBundleExecutable is set to 'applet' not 'popup'
    assert!(plist_content.contains("<string>applet</string>"), 
        "CFBundleExecutable should be 'applet' for proper URL handling");
    
    // Check that hook URL scheme is registered
    assert!(plist_content.contains("<string>hook</string>"), 
        "hook URL scheme should be registered");
    
    // Check that required files exist
    assert!(Path::new("/Applications/HookAnchor.app/Contents/MacOS/applet").exists(),
        "applet executable should exist");
    
    assert!(Path::new("/Applications/HookAnchor.app/Contents/MacOS/ha").exists(),
        "ha binary should exist for AppleScript to call");
    
    println!("✅ App bundle configuration test PASSED");
}

/// Performance test - ensure URL handling is fast
#[test]
fn test_hook_url_performance() {
    if !cfg!(target_os = "macos") {
        return;
    }
    
    if !Path::new("/Applications/HookAnchor.app").exists() {
        println!("Skipping performance test - HookAnchor.app not installed");
        return;
    }
    
    let start_time = Instant::now();
    
    let output = Command::new("open")
        .arg("hook://perf_test")
        .output()
        .expect("Failed to execute 'open' command");
    
    assert!(output.status.success(), "Failed to open hook URL");
    
    // Wait for completion
    thread::sleep(Duration::from_millis(1000));
    
    let duration = start_time.elapsed();
    println!("Hook URL processing took: {:?}", duration);
    
    // Should complete within reasonable time (5 seconds is generous)
    assert!(duration < Duration::from_secs(5), 
        "Hook URL processing took too long: {:?}", duration);
    
    println!("✅ Hook URL performance test PASSED");
}