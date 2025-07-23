//! 1Password Integration Test Binary
//!
//! Tests different approaches to 1Password integration:
//! - Character injection using Quick Access (Cmd+Shift+Space)
//! - Robust fallback approach with multiple methods
//! - Validates that 1Password opens websites to maintain autofill context

use hookanchor::launcher::launch;
use std::thread;
use std::time::Duration;

fn main() {
    println!("Testing 1Password character injection approaches...\n");
    
    // Test 1: Pure character injection using Quick Access (Cmd+Shift+Space)
    println!("1. Testing Quick Access approach: 1pass github");
    println!("   This should open 1Password Quick Access, search, and activate entry");
    match launch("1pass github") {
        Ok(()) => println!("   ✅ Quick Access approach worked"),
        Err(e) => println!("   ❌ Quick Access approach failed: {:?}", e),
    }
    thread::sleep(Duration::from_secs(3));
    
    // Test 2: Robust approach with fallbacks
    println!("\n2. Testing robust approach with fallbacks: 1pass_robust github");
    println!("   This tries Quick Access, then menu bar, then main app");
    match launch("1pass_robust github") {
        Ok(()) => println!("   ✅ Robust approach worked"),
        Err(e) => println!("   ❌ Robust approach failed: {:?}", e),
    }
    
    println!("\nExpected behavior:");
    println!("- 1Password Quick Access window opens");
    println!("- Search term is typed automatically");
    println!("- Entry is selected and website opens in browser");
    println!("- 1Password maintains autofill context for the opened website");
}