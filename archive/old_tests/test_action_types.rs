use hookanchor::launcher::launch;

fn main() {
    println!("Testing action-type launcher system...");
    
    // Test basic action types (should work like Python launcher)
    
    println!("Testing 'app Finder'...");
    match launch("app Finder") {
        Ok(()) => println!("✅ App launcher worked"),
        Err(e) => println!("❌ App launcher failed: {:?}", e),
    }
    
    println!("Testing 'url https://github.com'...");
    match launch("url https://github.com") {
        Ok(()) => println!("✅ URL launcher worked"),
        Err(e) => println!("❌ URL launcher failed: {:?}", e),
    }
    
    println!("Testing 'chrome https://anthropic.com'...");
    match launch("chrome https://anthropic.com") {
        Ok(()) => println!("✅ Chrome launcher worked"),
        Err(e) => println!("❌ Chrome launcher failed: {:?}", e),
    }
    
    println!("Testing 'folder /Applications'...");
    match launch("folder /Applications") {
        Ok(()) => println!("✅ Folder launcher worked"),
        Err(e) => println!("❌ Folder launcher failed: {:?}", e),
    }
    
    println!("Testing 'cmd echo Hello from new launcher'...");
    match launch("cmd echo Hello from new launcher") {
        Ok(()) => println!("✅ Shell command worked"),
        Err(e) => println!("❌ Shell command failed: {:?}", e),
    }
    
    println!("Action-type system test complete!");
}