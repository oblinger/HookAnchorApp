use hookanchor::{Command, filter_commands};

/// Helper to create a test command
fn make_cmd(name: &str, action: &str) -> Command {
    Command {
        group: String::new(),
        command: name.to_string(),
        action: action.to_string(),
        arg: String::new(),
        full_line: format!("{} : {}", name, action),
    }
}

#[test]
fn test_display_order_vs_selection_sync() {
    println!("Testing display order vs selection synchronization...");
    
    // Create commands that will be reordered
    let commands = vec![
        make_cmd("2017-09 Elastic Block Chain", "folder"),
        make_cmd("EOCI Consciousness", "app"),
        make_cmd("Economics Report", "url"),
        make_cmd("EOCI Corp AI", "cmd"),
        make_cmd("Electric Vehicle Manual", "doc"),
    ];
    
    // Filter with "ec" 
    let filtered = filter_commands(&commands, "ec", 20, false);
    
    println!("\nOriginal filtered order:");
    for (i, cmd) in filtered.iter().enumerate() {
        println!("{}: {} (action: {})", i, cmd.command, cmd.action);
    }
    
    // Simulate the display reordering that popup.rs does
    let mut prefix_matches = Vec::new();
    let mut early_matches = Vec::new();
    let mut later_matches = Vec::new();
    
    for cmd in &filtered {
        let cmd_lower = cmd.command.to_lowercase();
        if cmd_lower.starts_with("ec") {
            prefix_matches.push(cmd);
        } else if let Some(pos) = cmd_lower.find("ec") {
            if pos < 5 {
                early_matches.push(cmd);
            } else {
                later_matches.push(cmd);
            }
        }
    }
    
    println!("\nDisplay order after reordering:");
    let mut display_order: Vec<&Command> = Vec::new();
    display_order.extend(&prefix_matches);
    display_order.extend(&early_matches);
    display_order.extend(&later_matches);
    
    for (i, cmd) in display_order.iter().enumerate() {
        println!("{}: {} (action: {})", i, cmd.command, cmd.action);
    }
    
    // Test: Verify that selecting index 0 in display gives us the right command
    if !display_order.is_empty() {
        let display_cmd_at_0 = display_order[0];
        println!("\nCommand at display index 0: {} ({})", 
                 display_cmd_at_0.command, display_cmd_at_0.action);
        
        // If we were using filtered[0] instead of display_order[0], we'd get the wrong command
        let filtered_cmd_at_0 = &filtered[0];
        if display_cmd_at_0.command != filtered_cmd_at_0.command {
            println!("✅ Display order differs from filtered order (as expected)");
            println!("   Filtered[0]: {}", filtered_cmd_at_0.command);
            println!("   Display[0]: {}", display_cmd_at_0.command);
        }
    }
    
    // Verify the reordering is working
    assert!(!display_order.is_empty(), "Should have some display commands");
    
    // The first displayed command should be either a prefix match or early match
    let first_display = display_order[0];
    let first_lower = first_display.command.to_lowercase();
    let is_good_match = first_lower.starts_with("ec") || 
                        first_lower.find("ec").map_or(false, |pos| pos < 5);
    
    assert!(is_good_match, 
            "First display command '{}' should be a prefix or early match for 'ec'", 
            first_display.command);
    
    println!("\n✅ Display sync test passed");
}

#[test]
fn test_command_identity_preservation() {
    println!("Testing that command identity is preserved through reordering...");
    
    let cmd1 = make_cmd("EOCI Test", "app");
    let cmd2 = make_cmd("EOCI Test", "folder"); // Same name, different action
    let cmd3 = make_cmd("Economic Report", "url");
    
    let commands = vec![cmd1.clone(), cmd2.clone(), cmd3.clone()];
    let filtered = filter_commands(&commands, "eoc", 20, false);
    
    // Both EOCI Test commands should be in filtered results
    let eoci_app = filtered.iter().find(|c| c.command == "EOCI Test" && c.action == "app");
    let eoci_folder = filtered.iter().find(|c| c.command == "EOCI Test" && c.action == "folder");
    
    assert!(eoci_app.is_some(), "Should find EOCI Test with app action");
    assert!(eoci_folder.is_some(), "Should find EOCI Test with folder action");
    
    println!("✅ Command identity preserved correctly");
}