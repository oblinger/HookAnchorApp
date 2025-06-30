use anchor_selector::{load_commands, filter_commands, merge_similar_commands, get_current_submenu_prefix, split_commands};

#[test]
fn debug_fin_accounts_execution() {
    println!("\n=== Debug FIN Accounts Execution ===");
    
    let search_text = "fin account";
    
    // Step 1: Load and filter commands
    let all_commands = load_commands();
    let filtered = filter_commands(&all_commands, search_text, 100, false);
    
    println!("Commands after filtering for '{}':", search_text);
    for (i, cmd) in filtered.iter().enumerate() {
        if cmd.command.to_lowercase().contains("account") {
            println!("  {}: '{}' | action: '{}' | arg: '{}'", i, cmd.command, cmd.action, cmd.arg);
        }
    }
    
    // Step 2: Apply merging like the UI does
    if let Some(menu_prefix) = get_current_submenu_prefix(&filtered, search_text) {
        println!("\nSubmenu prefix: '{}'", menu_prefix);
        
        let (inside_menu, outside_menu) = split_commands(&filtered, &menu_prefix);
        
        // Merge each list separately
        let merged_inside = merge_similar_commands(&inside_menu, search_text);
        let merged_outside = merge_similar_commands(&outside_menu, search_text);
        
        println!("\nInside menu commands with 'account':");
        for (i, cmd) in merged_inside.iter().enumerate() {
            if cmd.command.to_lowercase().contains("account") {
                println!("  {}: '{}' | action: '{}' | arg: '{}'", i, cmd.command, cmd.action, cmd.arg);
            }
        }
        
        println!("\nOutside menu commands with 'account':");
        for (i, cmd) in merged_outside.iter().enumerate() {
            if cmd.command.to_lowercase().contains("account") {
                println!("  {}: '{}' | action: '{}' | arg: '{}'", i, cmd.command, cmd.action, cmd.arg);
            }
        }
        
        // Show what would be the first command executed
        if !merged_inside.is_empty() {
            println!("\n🔍 First command that would execute: '{}' -> action: '{}', arg: '{}'", 
                merged_inside[0].command, merged_inside[0].action, merged_inside[0].arg);
        }
    } else {
        println!("No submenu detected");
    }
}