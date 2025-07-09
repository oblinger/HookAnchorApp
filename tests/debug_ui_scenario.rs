use anchor_selector::{load_commands, filter_commands, load_config};
use anchor_selector::core::commands::{merge_similar_commands, get_current_submenu_prefix, split_commands};

#[test]
fn debug_ui_scenario_step_by_step() {
    println!("\n=== Debug UI Scenario Step by Step ===");
    
    // Simulate exact UI flow: type "fin" and process
    let search_text = "fin";
    
    // Step 1: Load commands 
    let all_commands = load_commands();
    println!("1. Loaded {} commands", all_commands.len());
    
    // Step 2: Filter commands (like UI does)
    let filtered = filter_commands(&all_commands, search_text, 10, false);
    println!("2. After filtering for '{}': {} commands", search_text, filtered.len());
    
    // Step 3: Apply merge (should be no-op since disabled)
    let config = load_config();
    let merged = merge_similar_commands(filtered, &config);
    println!("3. After merging: {} commands", merged.len());
    
    // Step 4: Check for submenu prefix
    let submenu_prefix = get_current_submenu_prefix(search_text);
    println!("4. Submenu prefix: {:?}", submenu_prefix);
    
    // Step 5: Split commands (if there's a submenu)
    if let Some(prefix) = submenu_prefix {
        let split_result = split_commands(&merged, &prefix, &config.popup_settings.word_separators);
        
        println!("5. Split results:");
        println!("   Total commands: {} commands", split_result.len());
        
        // Look for Findem specifically
        println!("\n=== Findem Analysis ===");
        
        // Check if Findem is in the split results
        for (i, cmd) in split_result.iter().enumerate() {
            if cmd.command.contains("Findem") {
                println!("FOUND IN SPLIT RESULTS [{}]: '{}'", i, cmd.command);
            }
        }
    } else {
        println!("5. No submenu detected");
    }
}