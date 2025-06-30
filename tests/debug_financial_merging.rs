use anchor_selector::{load_commands, filter_commands, merge_similar_commands, get_current_submenu_prefix, split_commands};

#[test]
fn debug_financial_model_merging() {
    println!("\n=== Debug Financial Model Merging ===");
    
    // Load actual commands and filter for "fin"
    let all_commands = load_commands();
    let filtered = filter_commands(&all_commands, "fin", 100, false);
    
    // Look specifically for "Financial Model" commands
    println!("Commands containing 'Financial Model':");
    let financial_commands: Vec<_> = filtered.iter()
        .filter(|cmd| cmd.command.contains("Financial Model"))
        .collect();
    
    for cmd in &financial_commands {
        println!("  - '{}' | action: {}", cmd.command, cmd.action);
    }
    
    // Test merging just these commands
    let financial_command_vec: Vec<_> = financial_commands.iter().map(|&c| c.clone()).collect();
    println!("\nTesting merge_similar_commands on just Financial Model commands:");
    let merged_financial = merge_similar_commands(&financial_command_vec, "fin");
    
    println!("After merging Financial Model commands:");
    for cmd in &merged_financial {
        println!("  - '{}'", cmd.command);
    }
    
    // Now test what happens in full submenu scenario
    println!("\n=== Full Submenu Scenario ===");
    
    if let Some(menu_prefix) = get_current_submenu_prefix(&filtered, "fin") {
        println!("Submenu prefix: '{}'", menu_prefix);
        
        let (inside_menu, outside_menu) = split_commands(&filtered, &menu_prefix);
        
        // Look for Financial Model commands in outside menu
        println!("\nFinancial Model commands in outside menu:");
        for cmd in &outside_menu {
            if cmd.command.contains("Financial Model") {
                println!("  - '{}'", cmd.command);
            }
        }
        
        // Apply merging to outside menu
        println!("\nAfter merging outside menu:");
        let merged_outside = merge_similar_commands(&outside_menu, "fin");
        
        println!("Financial Model commands after merging outside menu:");
        for cmd in &merged_outside {
            if cmd.command.contains("Financial Model") || cmd.command.contains("Fin Model") || cmd.command.contains("Model...") {
                println!("  - '{}'", cmd.command);
            }
        }
        
        println!("\nAll commands in merged outside menu (first 10):");
        for (i, cmd) in merged_outside.iter().take(10).enumerate() {
            println!("  {}: '{}'", i, cmd.command);
        }
        
        // Check total counts
        println!("\nCounts:");
        println!("  Inside menu: {}", inside_menu.len());
        println!("  Outside menu: {}", outside_menu.len());
        println!("  Merged outside: {}", merged_outside.len());
    }
}