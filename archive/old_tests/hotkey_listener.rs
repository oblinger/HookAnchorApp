use global_hotkey::{
    hotkey::{Code, HotKey, Modifiers},
    GlobalHotKeyEvent, GlobalHotKeyManager,
};
use std::process::Command;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Setting up global hotkey listener for F10...");
    
    // Create global hotkey manager
    let manager = GlobalHotKeyManager::new()?;
    
    // Create hotkey for F10 (no modifiers)
    let hotkey = HotKey::new(
        None, // No modifiers - just F10 by itself
        Code::F10,
    );
    
    // Register the hotkey
    manager.register(hotkey)?;
    println!("Registered global hotkey: F10");
    println!("Press F10 to launch anchor selector");
    println!("Press Ctrl+C to exit listener");
    
    // Get the path to the anchor selector binary
    let current_exe = env::current_exe()?;
    let popup_path = current_exe
        .parent()
        .ok_or("Could not get parent directory")?
        .join("popup");
    
    println!("Will launch: {}", popup_path.display());
    
    // Create global hotkey event receiver
    let receiver = GlobalHotKeyEvent::receiver();
    
    // Event loop
    loop {
        if let Ok(event) = receiver.try_recv() {
            println!("Global hotkey triggered: {:?}", event);
            
            // Launch the anchor selector popup
            match Command::new(&popup_path)
                .spawn()
            {
                Ok(mut child) => {
                    println!("Successfully launched anchor selector (PID: {:?})", child.id());
                    // Don't wait for the child process to complete
                    // The popup will handle its own lifecycle
                },
                Err(e) => {
                    eprintln!("Failed to launch anchor selector: {}", e);
                    eprintln!("Tried to launch: {}", popup_path.display());
                }
            }
        }
        
        // Small sleep to prevent high CPU usage
        std::thread::sleep(std::time::Duration::from_millis(10));
    }
}