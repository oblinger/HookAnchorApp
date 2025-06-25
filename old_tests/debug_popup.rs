use anchor_selector::{load_config, filter_commands, load_commands};

fn main() {
    println!("=== Debug Popup Startup ===");
    
    // Test config loading
    let config = load_config();
    println!("Config max_rows: {}", config.settings.max_rows);
    
    // Test command loading  
    let commands = load_commands();
    println!("Loaded {} commands", commands.len());
    
    // Test filtering with the config
    let filtered = filter_commands(&commands, "test", config.settings.max_rows, false);
    println!("Filtered to {} results (should respect max_rows: {})", filtered.len(), config.settings.max_rows);
    
    println!("=== Config Details ===");
    println!("Expected max_rows from config: {}", config.settings.max_rows);
    
    // Print first few filtered results
    println!("First few results:");
    for (i, cmd) in filtered.iter().take(5).enumerate() {
        println!("  {}: {}", i+1, cmd.command);
    }
}