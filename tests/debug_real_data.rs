use anchor_selector::{Command, split_commands, get_current_submenu_prefix, filter_commands, merge_similar_commands, load_commands};

fn cmd(command: &str, action: &str, arg: &str) -> Command {
    Command {
        group: String::new(),
        command: command.to_string(),
        action: action.to_string(),
        arg: arg.to_string(),
        full_line: format!("{} : {} {}", command, action, arg),
    }
}

#[test]
fn debug_real_scenario() {
    // Test with commands that look like what's in the screenshot
    let commands = vec![
        cmd("FIN", "folder", "~/fin"),
        cmd("FIN .Flows", "app", "Flows"),
        cmd("FIN .Log", "app", "Log"),
        cmd("FIN Accounts", "folder", "~/accounts"),
        cmd("FIN Analysis", "obs", "Analysis"),
        cmd("FIN Bills", "folder", "~/bills"),
        cmd("FIN Budget", "folder", "~/budget"),
        cmd("FIN Categories", "folder", "~/categories"),
        cmd("Flows", "app", "FlowsApp"),
        cmd("Folder", "folder", "~/folder"),
        cmd("Log Folder", "folder", "~/logs"),
        cmd("Net", "app", "NetApp"),
        cmd("Note", "obs", "Note"),
        cmd("FINDEM", "app", "Findem"),
    ];
    
    println!("\n=== Debugging Real Scenario ===");
    
    // First, let's see what get_current_submenu_prefix returns
    let submenu_prefix = get_current_submenu_prefix(&commands, "fin");
    println!("Submenu prefix: {:?}", submenu_prefix);
    
    if let Some(prefix) = submenu_prefix {
        // Now let's see how split_commands behaves
        let (inside_menu, outside_menu) = split_commands(&commands, &prefix);
        
        println!("\nInside menu ({} commands):", inside_menu.len());
        for (i, cmd) in inside_menu.iter().enumerate() {
            println!("  {}: {}", i, cmd.command);
        }
        
        println!("\nOutside menu ({} commands):", outside_menu.len());
        for (i, cmd) in outside_menu.iter().enumerate() {
            println!("  {}: {}", i, cmd.command);
        }
        
        // Check specifically for FINDEM
        if inside_menu.iter().any(|c| c.command == "FINDEM") {
            println!("\n❌ ERROR: FINDEM is in inside_menu (should be outside)");
        } else if outside_menu.iter().any(|c| c.command == "FINDEM") {
            println!("\n✅ CORRECT: FINDEM is in outside_menu");
        } else {
            println!("\n❓ MISSING: FINDEM not found in either menu");
        }
    } else {
        println!("No submenu prefix found");
    }
}

#[test] 
fn debug_with_actual_commands() {
    // Load actual commands from the file and test
    let all_commands = load_commands();
    
    // Filter for "fin" like the UI does
    let filtered = filter_commands(&all_commands, "fin", 100, false);
    
    // Apply merging like the UI does
    let merged = merge_similar_commands(&filtered, "fin");
    
    println!("\n=== With Actual Commands ===");
    println!("All commands: {}", all_commands.len());
    println!("Filtered commands: {}", filtered.len());
    println!("Merged commands: {}", merged.len());
    
    // Show first 20 filtered commands
    println!("\nFirst 20 filtered commands:");
    for (i, cmd) in merged.iter().take(20).enumerate() {
        println!("  {}: {}", i, cmd.command);
    }
    
    // Look for commands that might be causing issues
    println!("\nCommands containing 'DEM':");
    for cmd in &merged {
        if cmd.command.to_uppercase().contains("DEM") {
            println!("  - '{}' (len: {}, chars: {:?})", 
                cmd.command, 
                cmd.command.len(),
                cmd.command.chars().collect::<Vec<_>>()
            );
        }
    }
    
    // Check submenu behavior
    if let Some(prefix) = get_current_submenu_prefix(&merged, "fin") {
        println!("\nSubmenu prefix: '{}'", prefix);
        
        let (inside_menu, outside_menu) = split_commands(&merged, &prefix);
        
        println!("\nInside menu commands containing 'DEM':");
        for cmd in &inside_menu {
            if cmd.command.to_uppercase().contains("DEM") {
                println!("  - '{}'", cmd.command);
            }
        }
        
        println!("\nOutside menu commands containing 'DEM':");
        for cmd in &outside_menu {
            if cmd.command.to_uppercase().contains("DEM") {
                println!("  - '{}'", cmd.command);
            }
        }
    }
}