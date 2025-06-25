use anchor_selector::{execute_command, load_config};

fn main() {
    println!("Testing popup integration with new launcher...");
    
    // Check current config
    let config = load_config();
    println!("use_new_launcher: {}", config.settings.use_new_launcher);
    
    // Test command execution
    println!("Testing 'finder' command...");
    execute_command("finder");
    
    println!("Testing 'github' command...");
    execute_command("github");
    
    println!("Testing 'home' command...");
    execute_command("home");
    
    println!("Integration test complete!");
}