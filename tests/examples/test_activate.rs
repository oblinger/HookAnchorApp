//! Test Activate Functionality
//!
//! Tests the new JavaScript-based activate functionality.

use hookanchor::business_logic::activate_anchor;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Testing Anchor Activate Functionality ===\n");
    
    // Test with a safe test directory (current directory)
    let test_path = "/tmp";
    
    println!("Testing activate with path: {}", test_path);
    println!("This will demonstrate the JavaScript-based activate functionality.\n");
    
    match activate_anchor(test_path) {
        Ok(result) => {
            println!("✅ Activate completed successfully!");
            println!("Result: {}", result);
        }
        Err(e) => {
            eprintln!("❌ Activate failed: {}", e);
        }
    }
    
    println!("\n=== Test Complete ===");
    println!("Note: This demonstrates the user-customizable activate functionality.");
    println!("Users can edit src/business_logic/activate.js to customize behavior.");
    
    Ok(())
}