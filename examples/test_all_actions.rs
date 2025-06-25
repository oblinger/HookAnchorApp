//! Comprehensive Action Testing Example
//!
//! Tests all available action types systematically.
//! Useful for validating configuration and system setup.

use anchor_selector::launcher::launch;
use std::{thread, time::Duration};

fn main() {
    println!("=== Testing All Action Types ===\n");

    let actions = vec![
        ("app Finder", "Application Launch"),
        ("url https://github.com", "URL Opening"),
        ("folder /Applications", "Folder Opening"),
        ("cmd echo 'Test command'", "Shell Command"),
        ("chrome https://anthropic.com", "Chrome Opening"),
        ("safari https://apple.com", "Safari Opening"),
        ("obs TEST_PAGE", "Obsidian JavaScript"),
        ("anchor /tmp", "Anchor JavaScript"),
        ("1pass github", "1Password Integration"),
    ];

    let mut passed = 0;
    let mut failed = 0;

    for (action, description) in actions {
        print!("Testing {}: ", description);
        
        match launch(action) {
            Ok(()) => {
                println!("✅ PASS");
                passed += 1;
            },
            Err(e) => {
                println!("❌ FAIL - {:?}", e);
                failed += 1;
            }
        }
        
        // Small delay between actions
        thread::sleep(Duration::from_millis(200));
    }

    println!("\n=== Results ===");
    println!("Passed: {}", passed);
    println!("Failed: {}", failed);
    println!("Total:  {}", passed + failed);
    
    if failed == 0 {
        println!("🎉 All actions working correctly!");
    } else {
        println!("⚠️  Some actions failed - check configuration");
    }
}