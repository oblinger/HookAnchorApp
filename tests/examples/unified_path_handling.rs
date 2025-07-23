use hookanchor::{load_data};

fn main() {
    println!("Demonstrating unified path handling with accessor methods...");
    
    // Load commands using the new unified loading
    let (config, commands, _patches) = load_data();
    
    println!("Loaded {} commands", commands.len());
    
    // Find examples of each path-based action type
    let mut obs_example = None;
    let mut anchor_example = None;
    let mut folder_example = None;
    
    for cmd in &commands {
        if cmd.is_path_based() {
            match cmd.action.as_str() {
                "obs" if obs_example.is_none() => obs_example = Some(cmd),
                "anchor" if anchor_example.is_none() => anchor_example = Some(cmd),
                "folder" if folder_example.is_none() => folder_example = Some(cmd),
                _ => {}
            }
        }
        
        // Break early once we have examples of each
        if obs_example.is_some() && anchor_example.is_some() && folder_example.is_some() {
            break;
        }
    }
    
    // Demonstrate unified path handling
    println!("\n=== UNIFIED PATH HANDLING DEMONSTRATION ===");
    
    let examples = vec![
        ("OBS Command", obs_example),
        ("Anchor Command", anchor_example), 
        ("Folder Command", folder_example),
    ];
    
    for (label, cmd_opt) in examples {
        if let Some(cmd) = cmd_opt {
            println!("\n{}: '{}'", label, cmd.command);
            println!("  Action: {}", cmd.action);
            println!("  Raw arg: '{}'", cmd.arg);
            println!("  Patch: '{}'", cmd.patch);
            
            // BEFORE: We had different logic for each action type scattered across the codebase
            // NOW: Unified access through accessor methods
            
            if let Some(abs_file) = cmd.get_absolute_file_path(&config) {
                println!("  📁 Absolute file path: {}", abs_file.display());
            }
            
            if let Some(abs_folder) = cmd.get_absolute_folder_path(&config) {
                println!("  📂 Absolute folder path: {}", abs_folder.display());
            }
            
            // Check if the path exists
            if let Some(path) = cmd.get_absolute_file_path(&config) {
                println!("  ✅ Path exists: {}", path.exists());
            }
        } else {
            println!("\n{}: No example found", label);
        }
    }
    
    // Show statistics
    let path_based_count = commands.iter().filter(|c| c.is_path_based()).count();
    let total_count = commands.len();
    
    println!("\n=== PATH-BASED STATISTICS ===");
    println!("Path-based commands: {} / {} ({:.1}%)", 
        path_based_count, total_count, 
        (path_based_count as f64 / total_count as f64) * 100.0);
    
    // Count by action type
    let mut action_counts = std::collections::HashMap::new();
    for cmd in &commands {
        if cmd.is_path_based() {
            *action_counts.entry(cmd.action.clone()).or_insert(0) += 1;
        }
    }
    
    println!("Breakdown by action:");
    for (action, count) in action_counts {
        println!("  {}: {} commands", action, count);
    }
}