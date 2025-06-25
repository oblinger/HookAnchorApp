//! Launcher System Test Binary
//!
//! Validates basic launcher functionality with both unit tests and manual testing.
//! Tests various action types to ensure they execute without errors.

use anchor_selector::launcher::launch;

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn test_launch_finder_integration() {
        // This should actually launch Finder using the default config
        let result = launch("finder");
        assert!(result.is_ok(), "Failed to launch finder: {:?}", result);
    }

    #[test]
    fn test_launch_github_integration() {
        // This should actually open GitHub in browser using the default config
        let result = launch("github");
        assert!(result.is_ok(), "Failed to launch github: {:?}", result);
    }

    #[test]
    fn test_launch_home_integration() {
        // This should actually open home directory using the default config
        let result = launch("home");
        assert!(result.is_ok(), "Failed to launch home: {:?}", result);
    }

    #[test]
    fn test_launch_nonexistent_action() {
        // This should return ActionNotFound error
        let result = launch("nonexistent_action");
        assert!(result.is_err(), "Expected error for nonexistent action");
    }
}

fn main() {
    println!("Testing launcher system...");
    
    // Test each default command
    println!("Testing finder command...");
    match launch("finder") {
        Ok(()) => println!("✅ Finder launched successfully"),
        Err(e) => println!("❌ Finder failed: {:?}", e),
    }
    
    println!("Testing github command...");
    match launch("github") {
        Ok(()) => println!("✅ GitHub opened successfully"),
        Err(e) => println!("❌ GitHub failed: {:?}", e),
    }
    
    println!("Testing home command...");
    match launch("home") {
        Ok(()) => println!("✅ Home directory opened successfully"),
        Err(e) => println!("❌ Home directory failed: {:?}", e),
    }
    
    println!("Testing nonexistent command...");
    match launch("nonexistent") {
        Ok(()) => println!("❌ Should have failed"),
        Err(e) => println!("✅ Correctly failed: {:?}", e),
    }
    
    println!("Launcher system test complete!");
}