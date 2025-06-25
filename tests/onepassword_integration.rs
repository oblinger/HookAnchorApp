//! Integration tests for 1Password character injection
//! 
//! Tests 1Password integration using Quick Access and fallback methods.

use anchor_selector::launcher::launch;
use std::{thread, time::Duration};

#[test]
fn test_onepassword_quick_access() {
    // Test 1Password Quick Access character injection
    let result = launch("1pass github");
    assert!(result.is_ok(), "Failed to execute 1Password action: {:?}", result);
    
    // Allow time for the action to complete
    thread::sleep(Duration::from_millis(500));
}

#[test]
fn test_onepassword_robust_fallback() {
    // Test 1Password robust approach with fallbacks
    let result = launch("1pass_robust github");
    assert!(result.is_ok(), "Failed to execute 1Password robust action: {:?}", result);
    
    // Allow time for the action to complete
    thread::sleep(Duration::from_millis(1000));
}