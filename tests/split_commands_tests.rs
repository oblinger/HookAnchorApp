use anchor_selector::{Command, split_commands};

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
fn test_split_commands_findem_not_in_submenu() {
    // Test case: "FINDEM" should NOT be in FIN submenu because there's no space after FIN
    let commands = vec![
        cmd("FIN", "folder", "~/fin"),
        cmd("FIN Budget", "folder", "~/budget"),
        cmd("FIN Analysis", "obs", "Analysis"),
        cmd("FINDEM", "app", "Findem"),
        cmd("Financial Report", "folder", "~/reports"),
    ];
    
    let (inside_menu, outside_menu) = split_commands(&commands, "FIN");
    
    println!("=== FIN submenu test ===");
    println!("Inside menu:");
    for cmd in &inside_menu {
        println!("  - {}", cmd.command);
    }
    println!("Outside menu:");
    for cmd in &outside_menu {
        println!("  - {}", cmd.command);
    }
    
    // Expected inside menu: FIN, FIN Budget, FIN Analysis (commands with FIN + space)
    assert_eq!(inside_menu.len(), 3);
    assert!(inside_menu.iter().any(|c| c.command == "FIN"));
    assert!(inside_menu.iter().any(|c| c.command == "FIN Budget"));
    assert!(inside_menu.iter().any(|c| c.command == "FIN Analysis"));
    
    // Expected outside menu: FINDEM, Financial Report (no space after FIN)
    assert_eq!(outside_menu.len(), 2);
    assert!(outside_menu.iter().any(|c| c.command == "FINDEM"));
    assert!(outside_menu.iter().any(|c| c.command == "Financial Report"));
}

#[test]
fn test_split_commands_exact_match_in_submenu() {
    // Test case: Exact match should be in submenu
    let commands = vec![
        cmd("TEST", "folder", "~/test"),
        cmd("TEST Case", "app", "TestCase"),
        cmd("TESTING", "app", "Testing"),
    ];
    
    let (inside_menu, outside_menu) = split_commands(&commands, "TEST");
    
    println!("\n=== TEST submenu test ===");
    println!("Inside menu:");
    for cmd in &inside_menu {
        println!("  - {}", cmd.command);
    }
    println!("Outside menu:");
    for cmd in &outside_menu {
        println!("  - {}", cmd.command);
    }
    
    // Expected inside menu: TEST, TEST Case (exact match + space separator)
    assert_eq!(inside_menu.len(), 2);
    assert!(inside_menu.iter().any(|c| c.command == "TEST"));
    assert!(inside_menu.iter().any(|c| c.command == "TEST Case"));
    
    // Expected outside menu: TESTING (no space after TEST)
    assert_eq!(outside_menu.len(), 1);
    assert!(outside_menu.iter().any(|c| c.command == "TESTING"));
}

#[test]
fn test_split_commands_with_dots_and_underscores() {
    // Test case: Should work with dots and underscores as separators
    let commands = vec![
        cmd("LOG", "folder", "~/logs"),
        cmd("LOG.daily", "file", "daily.log"),
        cmd("LOG_weekly", "file", "weekly.log"),
        cmd("LOG error", "app", "LogViewer"),
        cmd("LOGIN", "app", "LoginApp"),
        cmd("LOGGED", "app", "LoggedApp"),
    ];
    
    let (inside_menu, outside_menu) = split_commands(&commands, "LOG");
    
    println!("\n=== LOG submenu test ===");
    println!("Inside menu:");
    for cmd in &inside_menu {
        println!("  - {}", cmd.command);
    }
    println!("Outside menu:");
    for cmd in &outside_menu {
        println!("  - {}", cmd.command);
    }
    
    // Expected inside menu: LOG, LOG.daily, LOG_weekly, LOG error (exact + separators)
    assert_eq!(inside_menu.len(), 4);
    assert!(inside_menu.iter().any(|c| c.command == "LOG"));
    assert!(inside_menu.iter().any(|c| c.command == "LOG.daily"));
    assert!(inside_menu.iter().any(|c| c.command == "LOG_weekly"));
    assert!(inside_menu.iter().any(|c| c.command == "LOG error"));
    
    // Expected outside menu: LOGIN, LOGGED (no separator after LOG)
    assert_eq!(outside_menu.len(), 2);
    assert!(outside_menu.iter().any(|c| c.command == "LOGIN"));
    assert!(outside_menu.iter().any(|c| c.command == "LOGGED"));
}

#[test]
fn test_split_commands_case_insensitive() {
    // Test case: Should be case insensitive
    let commands = vec![
        cmd("fin", "folder", "~/fin"),
        cmd("FIN Budget", "folder", "~/budget"),
        cmd("Fin Analysis", "obs", "Analysis"),
        cmd("FINDEM", "app", "Findem"),
    ];
    
    let (inside_menu, outside_menu) = split_commands(&commands, "FIN");
    
    println!("\n=== Case insensitive test ===");
    println!("Inside menu:");
    for cmd in &inside_menu {
        println!("  - {}", cmd.command);
    }
    println!("Outside menu:");
    for cmd in &outside_menu {
        println!("  - {}", cmd.command);
    }
    
    // Expected inside menu: fin, FIN Budget, Fin Analysis (case insensitive + separator)
    assert_eq!(inside_menu.len(), 3);
    assert!(inside_menu.iter().any(|c| c.command == "fin"));
    assert!(inside_menu.iter().any(|c| c.command == "FIN Budget"));
    assert!(inside_menu.iter().any(|c| c.command == "Fin Analysis"));
    
    // Expected outside menu: FINDEM (no separator)
    assert_eq!(outside_menu.len(), 1);
    assert!(outside_menu.iter().any(|c| c.command == "FINDEM"));
}

#[test]
fn test_split_commands_empty_cases() {
    // Test edge cases
    let commands = vec![
        cmd("TEST", "folder", "~/test"),
        cmd("OTHER", "app", "Other"),
    ];
    
    // Empty prefix
    let (inside_menu, outside_menu) = split_commands(&commands, "");
    assert_eq!(inside_menu.len(), 0);
    assert_eq!(outside_menu.len(), 2);
    
    // No matching prefix
    let (inside_menu, outside_menu) = split_commands(&commands, "NONEXISTENT");
    assert_eq!(inside_menu.len(), 0);
    assert_eq!(outside_menu.len(), 2);
    
    // Empty commands list
    let empty_commands = vec![];
    let (inside_menu, outside_menu) = split_commands(&empty_commands, "TEST");
    assert_eq!(inside_menu.len(), 0);
    assert_eq!(outside_menu.len(), 0);
}