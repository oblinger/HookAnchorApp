//! JavaScript Action Test Binary
//!
//! Tests JavaScript execution in the launcher system including:
//! - Obsidian URL encoding and launch
//! - Directory navigation with anchor activation
//! - 1Password character injection

use hookanchor::launcher::launch;

fn main() {
    println!("Testing JavaScript action execution...");
    
    // Test Obsidian JavaScript action
    println!("\n1. Testing Obsidian JavaScript action...");
    match launch("obs TEST_PAGE") {
        Ok(()) => println!("   ✅ Obsidian action worked"),
        Err(e) => println!("   ❌ Obsidian action failed: {:?}", e),
    }
    
    // Test anchor JavaScript action
    println!("\n2. Testing anchor JavaScript action...");
    match launch("anchor /tmp") {
        Ok(()) => println!("   ✅ Anchor action worked"),
        Err(e) => println!("   ❌ Anchor action failed: {:?}", e),
    }
    
    println!("\n3. Testing 1Password character injection...");
    match launch("1pass github") {
        Ok(()) => println!("   ✅ 1Password action worked"),
        Err(e) => println!("   ❌ 1Password action failed: {:?}", e),
    }
    
    println!("\nAll JavaScript actions tested.");
}