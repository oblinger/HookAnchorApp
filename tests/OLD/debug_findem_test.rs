use hookanchor::{Command, split_commands};

fn cmd(command: &str, action: &str, arg: &str) -> Command {
    Command {
        patch: String::new(),
        command: command.to_string(),
        action: action.to_string(),
        arg: arg.to_string(),
        full_line: format!("{} : {} {}", command, action, arg),
    }
}

#[test]
fn debug_findem_exact_case() {
    // Test with the exact case we see in the screenshot
    let commands = vec![
        cmd("FIN", "folder", "~/fin"),
        cmd("FINDEM", "app", "Findem"),  // This should be in outside menu
    ];
    
    println!("=== Testing FINDEM with prefix 'FIN' ===");
    
    for cmd in &commands {
        println!("Command: '{}'", cmd.command);
        println!("  Length: {}", cmd.command.len());
        println!("  Chars: {:?}", cmd.command.chars().collect::<Vec<_>>());
        
        let prefix = "FIN";
        let prefix_end = prefix.len();
        
        if cmd.command.len() > prefix.len() {
            println!("  Command longer than prefix");
            if cmd.command[..prefix_end].eq_ignore_ascii_case(prefix) {
                println!("  Starts with prefix");
                if let Some(ch) = cmd.command.chars().nth(prefix_end) {
                    println!("  Char after prefix: '{}'", ch);
                    if ch == ' ' || ch == '.' || ch == '_' {
                        println!("  -> INSIDE menu (has separator)");
                    } else {
                        println!("  -> OUTSIDE menu (no separator)");
                    }
                } else {
                    println!("  -> No char after prefix");
                }
            } else {
                println!("  Doesn't start with prefix");
                println!("  -> OUTSIDE menu");
            }
        } else if cmd.command.eq_ignore_ascii_case(prefix) {
            println!("  Exact match with prefix");
            println!("  -> INSIDE menu (exact match)");
        } else {
            println!("  Command shorter than prefix");
            println!("  -> OUTSIDE menu");
        }
        println!();
    }
    
    let (inside_menu, outside_menu) = split_commands(&commands, "FIN");
    
    println!("Results:");
    println!("Inside menu ({} items):", inside_menu.len());
    for cmd in &inside_menu {
        println!("  - '{}'", cmd.command);
    }
    println!("Outside menu ({} items):", outside_menu.len());
    for cmd in &outside_menu {
        println!("  - '{}'", cmd.command);
    }
    
    // FINDEM should be in outside menu
    assert_eq!(outside_menu.len(), 1);
    assert_eq!(outside_menu[0].command, "FINDEM");
    
    // FIN should be in inside menu
    assert_eq!(inside_menu.len(), 1);
    assert_eq!(inside_menu[0].command, "FIN");
}

#[test]
fn debug_screenshot_commands() {
    // Test with commands that might match what's in the screenshot
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
        cmd("FINDEM", "app", "Findem"),  // This should be outside
    ];
    
    let (inside_menu, outside_menu) = split_commands(&commands, "FIN");
    
    println!("\n=== Screenshot Commands Test ===");
    println!("Inside menu ({} items):", inside_menu.len());
    for cmd in &inside_menu {
        println!("  - '{}'", cmd.command);
    }
    println!("Outside menu ({} items):", outside_menu.len());
    for cmd in &outside_menu {
        println!("  - '{}'", cmd.command);
    }
    
    // FINDEM should definitely be in outside menu
    assert!(outside_menu.iter().any(|c| c.command == "FINDEM"), 
            "FINDEM should be in outside menu but wasn't found there");
    assert!(!inside_menu.iter().any(|c| c.command == "FINDEM"), 
            "FINDEM should NOT be in inside menu but was found there");
}