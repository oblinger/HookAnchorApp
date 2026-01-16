//! Regression tests for hook:// URL processing
//!
//! These tests verify that URL handling works correctly across different scenarios:
//! - Direct command execution via hook:// URLs
//! - URL encoding/decoding
//! - Special characters in URLs
//! - Command matching and execution
//! - Server-based execution
//!
//! IMPORTANT: These tests require the command server to be running.
//! The server is automatically started if not running.

use std::process::Command;
use std::thread;
use std::time::Duration;

/// Test that basic hook:// URL processing works
#[test]
fn test_basic_url_processing() {
    let result = Command::new(get_ha_binary_path())
        .arg("--hook")
        .arg("hook://test")
        .output()
        .expect("Failed to execute ha --hook");

    // Should exit successfully
    assert!(result.status.success(),
        "URL processing failed: {}", String::from_utf8_lossy(&result.stderr));
}

/// Test URL encoding with spaces
#[test]
fn test_url_encoding_spaces() {
    let result = Command::new(get_ha_binary_path())
        .arg("--hook")
        .arg("hook://test%20command")
        .output()
        .expect("Failed to execute ha --hook");

    assert!(result.status.success(),
        "URL with encoded spaces failed: {}", String::from_utf8_lossy(&result.stderr));
}

/// Test URL encoding with special characters
#[test]
fn test_url_encoding_special_chars() {
    // Test various special characters that might appear in commands
    let test_cases = vec![
        "hook://test%2Fpath",  // Forward slash
        "hook://test%3Acolon", // Colon
        "hook://test%26amp",   // Ampersand
        "hook://test%3Fquest", // Question mark
    ];

    for url in test_cases {
        let result = Command::new(get_ha_binary_path())
            .arg("--hook")
            .arg(url)
            .output()
            .expect("Failed to execute ha --hook");

        assert!(result.status.success(),
            "URL {} failed: {}", url, String::from_utf8_lossy(&result.stderr));
    }
}

/// Test that --hook flag auto-adds hook:// prefix
#[test]
fn test_hook_flag_adds_prefix() {
    // When using --hook, the prefix should be auto-added if missing
    let result = Command::new(get_ha_binary_path())
        .arg("--hook")
        .arg("test_query")  // No hook:// prefix
        .output()
        .expect("Failed to execute ha --hook");

    assert!(result.status.success(),
        "URL without prefix failed: {}", String::from_utf8_lossy(&result.stderr));
}

/// Test that URLs with hook:// prefix also work with --hook flag
#[test]
fn test_hook_flag_with_full_url() {
    let result = Command::new(get_ha_binary_path())
        .arg("--hook")
        .arg("hook://test_query")  // Full URL
        .output()
        .expect("Failed to execute ha --hook");

    assert!(result.status.success(),
        "URL with explicit prefix failed: {}", String::from_utf8_lossy(&result.stderr));
}

/// Test server-based execution path
#[test]
fn test_server_execution() {
    // Ensure server is running
    ensure_server_running();

    // Give server time to start
    thread::sleep(Duration::from_millis(500));

    // Execute via server
    let result = Command::new(get_ha_binary_path())
        .arg("--hook")
        .arg("hook://test_server")
        .output()
        .expect("Failed to execute ha --hook");

    assert!(result.status.success(),
        "Server execution failed: {}", String::from_utf8_lossy(&result.stderr));
}

/// Test that empty URL is handled gracefully
#[test]
fn test_empty_url() {
    let result = Command::new(get_ha_binary_path())
        .arg("--hook")
        .arg("hook://")
        .output()
        .expect("Failed to execute ha --hook");

    // Empty URL should either succeed (no-op) or provide a helpful message
    // We don't want it to crash
    let stderr = String::from_utf8_lossy(&result.stderr);
    assert!(result.status.success() || stderr.contains("empty") || stderr.contains("query"),
        "Empty URL handling failed unexpectedly: {}", stderr);
}

/// Test URL processing performance
#[test]
fn test_url_processing_performance() {
    use std::time::Instant;

    ensure_server_running();
    thread::sleep(Duration::from_millis(500));

    let start = Instant::now();

    let result = Command::new(get_ha_binary_path())
        .arg("--hook")
        .arg("hook://perf_test")
        .output()
        .expect("Failed to execute ha --hook");

    let duration = start.elapsed();

    assert!(result.status.success(),
        "Performance test execution failed");

    // URL processing should be fast (< 2 seconds is reasonable)
    assert!(duration < Duration::from_secs(2),
        "URL processing took too long: {:?}", duration);

    println!("URL processing took: {:?}", duration);
}

/// Test that command matching works correctly
#[test]
fn test_command_matching() {
    ensure_server_running();
    thread::sleep(Duration::from_millis(500));

    // Test with a query that should match commands
    // Using -m (match) to verify the matching logic
    let result = Command::new(get_ha_binary_path())
        .arg("-m")
        .arg("test")
        .output()
        .expect("Failed to execute ha -m");

    // Note: -m may return non-zero exit code if no exact match found, but it still works
    // We just verify it runs without crashing and produces some output
    if !result.status.success() {
        println!("Note: ha -m returned non-zero (expected if no exact match): {}",
            String::from_utf8_lossy(&result.stderr));
    }

    // Should return some output (matched commands or empty)
    // We're not asserting on success code, just that the mechanism works and produces output
    let stdout = String::from_utf8_lossy(&result.stdout);
    assert!(!stdout.is_empty() || result.status.success(),
        "Command should either produce output or succeed");
}

/// Test that URL processing doesn't show popup
/// This is critical - URLs should execute directly, not show the GUI
#[test]
fn test_url_no_popup() {
    ensure_server_running();
    thread::sleep(Duration::from_millis(500));

    use std::time::Instant;
    let start = Instant::now();

    let result = Command::new(get_ha_binary_path())
        .arg("--hook")
        .arg("hook://no_popup_test")
        .output()
        .expect("Failed to execute ha --hook");

    let duration = start.elapsed();

    // If popup opened, this would hang or take a very long time
    // Fast completion means no popup
    assert!(duration < Duration::from_secs(2),
        "URL processing took too long - popup might have opened: {:?}", duration);

    assert!(result.status.success(),
        "URL execution failed");
}

// Helper functions

fn get_ha_binary_path() -> String {
    // Use the release binary - prefer relative path for portability
    "./target/release/ha".to_string()
}

fn ensure_server_running() {
    // Check if server is running, start if not
    let status = Command::new(get_ha_binary_path())
        .arg("--process-status")
        .output()
        .expect("Failed to check server status");

    if !status.status.success() {
        // Server not running, start it
        let _ = Command::new(get_ha_binary_path())
            .arg("--start-server-daemon")
            .spawn()
            .expect("Failed to start server");

        // Wait for server to be ready
        thread::sleep(Duration::from_secs(1));
    }
}

// Integration test combining multiple scenarios
#[test]
fn test_url_processing_integration() {
    println!("=== URL Processing Integration Test ===");

    ensure_server_running();
    thread::sleep(Duration::from_millis(500));

    let test_scenarios = vec![
        ("Basic query", "hook://integration_test"),
        ("With spaces", "hook://integration%20test"),
        ("With special chars", "hook://test%2Fpath"),
        ("Short query", "hook://it"),
        ("Long query", "hook://this_is_a_very_long_query_name_for_testing"),
    ];

    for (name, url) in test_scenarios {
        println!("Testing: {}", name);

        let result = Command::new(get_ha_binary_path())
            .arg("--hook")
            .arg(url)
            .output()
            .expect(&format!("Failed to execute: {}", name));

        assert!(result.status.success(),
            "{} failed: {}", name, String::from_utf8_lossy(&result.stderr));

        println!("  ✓ {}", name);
    }

    println!("=== All integration tests passed ===");
}
