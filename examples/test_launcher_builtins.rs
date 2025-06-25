//! Test Launcher Built-ins
//!
//! Tests the individual JavaScript built-in functions.

use anchor_selector::js_runtime::execute_business_logic;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Testing Launcher Built-ins ===\n");
    
    // Test 1: Basic logging
    println!("Test 1: Basic logging");
    let script1 = r#"
        log("Hello from JavaScript!");
        debug("Debug message");
        "logging test complete"
    "#;
    
    match execute_business_logic(script1) {
        Ok(result) => println!("✅ Logging test: {}", result),
        Err(e) => println!("❌ Logging test failed: {}", e),
    }
    
    // Test 2: File operations
    println!("\nTest 2: File operations");
    let script2 = r#"
        const exists = fileExists("/tmp");
        const isDir = isDirectory("/tmp");
        log("File exists: " + exists);
        log("Is directory: " + isDir);
        "file operations complete"
    "#;
    
    match execute_business_logic(script2) {
        Ok(result) => println!("✅ File operations test: {}", result),
        Err(e) => println!("❌ File operations test failed: {}", e),
    }
    
    // Test 3: Path utilities
    println!("\nTest 3: Path utilities");
    let script3 = r#"
        const home = expandHome("~");
        const base = basename("/tmp/test.txt");
        log("Home: " + home);
        log("Basename: " + base);
        "path utilities complete"
    "#;
    
    match execute_business_logic(script3) {
        Ok(result) => println!("✅ Path utilities test: {}", result),
        Err(e) => println!("❌ Path utilities test failed: {}", e),
    }
    
    // Test 4: Simple launcher functions
    println!("\nTest 4: Launcher functions (non-destructive)");
    let script4 = r#"
        // Test shell command that's safe
        const result = shell("echo 'Hello from shell'");
        log("Shell result: " + result);
        "launcher functions complete"
    "#;
    
    match execute_business_logic(script4) {
        Ok(result) => println!("✅ Launcher functions test: {}", result),
        Err(e) => println!("❌ Launcher functions test failed: {}", e),
    }
    
    println!("\n=== All Tests Complete ===");
    
    Ok(())
}