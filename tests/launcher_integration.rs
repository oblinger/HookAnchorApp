//! Integration tests for the launcher system
//! 
//! Tests the launcher with actual configuration files and command execution.

use anchor_selector::launcher::launch;

#[test]
fn test_basic_app_launch() {
    // Test basic app launching
    let result = launch("app Finder");
    assert!(result.is_ok(), "Failed to launch Finder: {:?}", result);
}

#[test]
fn test_url_opening() {
    // Test URL opening
    let result = launch("url https://github.com");
    assert!(result.is_ok(), "Failed to open URL: {:?}", result);
}

#[test]
fn test_browser_specific_launch() {
    // Test browser-specific launching
    let result = launch("chrome https://anthropic.com");
    assert!(result.is_ok(), "Failed to launch Chrome: {:?}", result);
}

#[test]
fn test_folder_opening() {
    // Test folder opening
    let result = launch("folder /Applications");
    assert!(result.is_ok(), "Failed to open folder: {:?}", result);
}

#[test]
fn test_shell_command() {
    // Test shell command execution
    let result = launch("cmd echo 'Hello from integration test'");
    assert!(result.is_ok(), "Failed to execute shell command: {:?}", result);
}

#[test]
fn test_nonexistent_action() {
    // Test that nonexistent actions fail appropriately
    let result = launch("nonexistent_action_12345");
    assert!(result.is_err(), "Expected error for nonexistent action");
}