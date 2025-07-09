use anchor_selector::{load_commands, filter_commands, merge_similar_commands, load_config};

#[test]
fn debug_command_loading_pipeline() {
    println!("\n=== Debugging Command Loading Pipeline ===");
    
    // 1. Check config
    let config = load_config();
    println!("Config merge_similar: {:?}", config.popup_settings.merge_similar);
    
    // 2. Load raw commands
    let all_commands = load_commands();
    println!("Total commands loaded: {}", all_commands.len());
    
    // 3. Look for Findem commands specifically
    println!("\nSearching for commands containing 'findem' (case insensitive):");
    for (i, cmd) in all_commands.iter().enumerate() {
        if cmd.command.to_lowercase().contains("findem") {
            println!("  {}: '{}' | action: '{}' | arg: '{}' | full_line: '{}'", 
                i, cmd.command, cmd.action, cmd.arg, cmd.full_line);
        }
    }
    
    // 4. Filter for "fin" 
    let filtered = filter_commands(&all_commands, "fin", 100, false);
    println!("\nAfter filtering for 'fin': {} commands", filtered.len());
    
    println!("Commands containing 'findem' after filtering:");
    for cmd in &filtered {
        if cmd.command.to_lowercase().contains("findem") {
            println!("  - '{}' | action: '{}' | arg: '{}' | full_line: '{}'", 
                cmd.command, cmd.action, cmd.arg, cmd.full_line);
        }
    }
    
    // 5. Apply merging (should be disabled)
    let config = anchor_selector::load_config();
    let merged = merge_similar_commands(filtered, &config);
    println!("\nAfter merge_similar_commands: {} commands", merged.len());
    
    println!("Commands containing 'findem' or 'dem' after merging:");
    for cmd in &merged {
        if cmd.command.to_lowercase().contains("findem") || cmd.command.to_lowercase().contains("dem") {
            println!("  - '{}' | action: '{}' | arg: '{}' | full_line: '{}'", 
                cmd.command, cmd.action, cmd.arg, cmd.full_line);
        }
    }
    
    // 6. Show first 20 commands from each stage
    println!("\nFirst 20 filtered commands:");
    for (i, cmd) in filtered.iter().take(20).enumerate() {
        println!("  {}: '{}'", i, cmd.command);
    }
    
    println!("\nFirst 20 merged commands:");
    for (i, cmd) in merged.iter().take(20).enumerate() {
        println!("  {}: '{}'", i, cmd.command);
    }
}

#[test]
fn debug_raw_file_reading() {
    println!("\n=== Debug Raw File Reading ===");
    
    // Try to read the commands file directly
    let commands_path = std::path::Path::new(&std::env::var("HOME").unwrap())
        .join(".config/anchor_selector/commands.txt");
    
    if let Ok(content) = std::fs::read_to_string(&commands_path) {
        println!("Commands file path: {:?}", commands_path);
        println!("File size: {} bytes", content.len());
        
        // Look for Findem lines specifically
        println!("\nLines containing 'Findem':");
        for (line_num, line) in content.lines().enumerate() {
            if line.contains("Findem") {
                println!("  Line {}: {}", line_num + 1, line);
            }
        }
        
        // Show total line count
        let line_count = content.lines().count();
        println!("\nTotal lines in file: {}", line_count);
        
        // Show first few lines that match "FIN"
        println!("\nFirst 10 lines containing 'FIN':");
        let mut count = 0;
        for (line_num, line) in content.lines().enumerate() {
            if line.to_uppercase().contains("FIN") && count < 10 {
                println!("  Line {}: {}", line_num + 1, line);
                count += 1;
            }
        }
    } else {
        println!("Could not read commands file at {:?}", commands_path);
    }
}