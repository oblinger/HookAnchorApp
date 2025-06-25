//! Integration tests for JavaScript action execution
//! 
//! Tests JavaScript actions including Obsidian, anchor, and complex workflows.

use anchor_selector::launcher::launch;

#[test]
fn test_obsidian_javascript_action() {
    // Test Obsidian page opening with URL encoding
    let result = launch("obs TEST_PAGE");
    assert!(result.is_ok(), "Failed to execute Obsidian action: {:?}", result);
}

#[test]
fn test_anchor_javascript_action() {
    // Test anchor activation (directory change + anchor activate)
    let result = launch("anchor /tmp");
    assert!(result.is_ok(), "Failed to execute anchor action: {:?}", result);
}

#[test]
fn test_alias_javascript_action() {
    // Test recursive command calling (alias)
    let result = launch("alias app Finder");
    assert!(result.is_ok(), "Failed to execute alias action: {:?}", result);
}