use anchor_selector::launcher::launch;

fn main() {
    println!("Testing JavaScript actions directly...");
    
    // Test anchor command with full path
    println!("Testing anchor action with fixed path...");
    match launch("anchor /tmp") {
        Ok(()) => println!("✅ Anchor action worked"),
        Err(e) => println!("❌ Anchor action failed: {:?}", e),
    }
    
    // Test Obsidian action with corrected vault name
    println!("Testing Obsidian action...");
    match launch("obs TEST_PAGE") {
        Ok(()) => println!("✅ Obsidian action worked"),
        Err(e) => println!("❌ Obsidian action failed: {:?}", e),
    }
}