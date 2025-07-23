//! Basic Launcher Usage Example
//!
//! Demonstrates how to use the hookanchor launcher system
//! for executing various types of actions.

use hookanchor::launcher::launch;

fn main() {
    println!("=== HookAnchor Launcher Examples ===\n");

    // Example 1: Launch an application
    println!("1. Launching Finder...");
    match launch("app Finder") {
        Ok(()) => println!("   ✅ Finder launched successfully"),
        Err(e) => println!("   ❌ Failed: {:?}", e),
    }

    // Example 2: Open a URL
    println!("\n2. Opening GitHub...");
    match launch("url https://github.com") {
        Ok(()) => println!("   ✅ GitHub opened successfully"),
        Err(e) => println!("   ❌ Failed: {:?}", e),
    }

    // Example 3: Browser-specific opening
    println!("\n3. Opening Anthropic in Chrome...");
    match launch("chrome https://anthropic.com") {
        Ok(()) => println!("   ✅ Chrome opened successfully"),
        Err(e) => println!("   ❌ Failed: {:?}", e),
    }

    // Example 4: Execute shell command
    println!("\n4. Running shell command...");
    match launch("cmd echo 'Hello from launcher example'") {
        Ok(()) => println!("   ✅ Shell command executed"),
        Err(e) => println!("   ❌ Failed: {:?}", e),
    }

    // Example 5: JavaScript action (Obsidian)
    println!("\n5. Opening Obsidian page...");
    match launch("obs EXAMPLE_PAGE") {
        Ok(()) => println!("   ✅ Obsidian action executed"),
        Err(e) => println!("   ❌ Failed: {:?}", e),
    }

    println!("\n=== Examples complete ===");
}