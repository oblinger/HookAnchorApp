use anchor_selector::launcher::launch;

fn main() {
    println!("Testing 1Password URL schemes...");
    
    // Test 1Password search with URL action
    println!("Testing 1Password search action...");
    match launch("1pass github") {
        Ok(()) => println!("✅ 1Password URL action worked"),
        Err(e) => println!("❌ 1Password URL action failed: {:?}", e),
    }
    
    println!("\nNote: Check if 1Password opened and searched for 'github'");
}