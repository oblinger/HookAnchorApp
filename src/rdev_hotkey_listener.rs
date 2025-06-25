use rdev::{listen, Event, EventType, Key};
use std::process::Command;
use std::env;

fn callback(event: Event) {
    match event.event_type {
        EventType::KeyPress(Key::F10) => {
            println!("F10 pressed! Launching anchor selector...");
            
            // Reconstruct the popup path (simple approach)
            if let Ok(current_exe) = env::current_exe() {
                if let Some(parent) = current_exe.parent() {
                    let popup_path = parent.join("popup");
                    
                    // Launch the anchor selector popup
                    match Command::new(&popup_path).spawn() {
                        Ok(child) => {
                            println!("Successfully launched anchor selector (PID: {:?})", child.id());
                        },
                        Err(e) => {
                            eprintln!("Failed to launch anchor selector: {}", e);
                            eprintln!("Tried to launch: {}", popup_path.display());
                        }
                    }
                }
            }
        },
        _ => {
            // Ignore other events
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Setting up rdev hotkey listener for F10...");
    println!("Press F10 to launch anchor selector");
    println!("Press Ctrl+C to exit listener");
    
    // Get the path to the anchor selector binary
    let current_exe = env::current_exe()?;
    let popup_path = current_exe
        .parent()
        .ok_or("Could not get parent directory")?
        .join("popup");
    
    
    println!("Will launch: {}", popup_path.display());
    println!("");
    println!("📝 Note: On macOS, this requires Accessibility permissions:");
    println!("   System Settings > Privacy & Security > Accessibility");
    println!("   Add Terminal or this binary to the list");
    println!("");
    
    // Start listening for events
    println!("Starting event listener...");
    if let Err(error) = listen(callback) {
        eprintln!("Error listening for events: {:?}", error);
        eprintln!("");
        eprintln!("This likely means Accessibility permissions are not granted.");
        eprintln!("Please go to System Settings > Privacy & Security > Accessibility");
        eprintln!("and add Terminal (or this binary) to the list.");
    }
    
    Ok(())
}