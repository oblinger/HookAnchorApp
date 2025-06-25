use anchor_selector::launcher::launch;
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
    
    println!("\nNote: Watch for:");
    println!("- 1Password Quick Access window opening");
    println!("- Search term being typed");
    println!("- Entry being selected and website opening");
    println!("- 1Password maintaining autofill context for the opened website");
}