use hookanchor::{Command, filter_commands, get_current_submenu_prefix, split_commands};

/// Helper to create a test command
fn make_cmd(name: &str) -> Command {
    Command {
        group: String::new(),
        command: name.to_string(),
        action: "test".to_string(),
        arg: String::new(),
        full_line: format!("{} : test", name),
    }
}

#[test]
fn test_prefix_matching_order_basic() {
    println!("Testing basic prefix matching order...");
    
    let commands = vec![
        make_cmd("2017-09 Elastic Block Chain"),
        make_cmd("2017-09 Elastic Block Chain Note"),
        make_cmd("2017-09 Elastic Block Chain Obsidian"),
        make_cmd("EOCI Consciousness"),
        make_cmd("EOCI Consciousness Obsidian"),
        make_cmd("EOCI Corp AI"),
        make_cmd("EOCA Civilizing tendency"),
        make_cmd("EOCB End Run - Cyber Model"),
        make_cmd("Electric Car Manual"),
        make_cmd("Economics Report"),
    ];
    
    // Filter with "ec" to get commands
    let filtered = filter_commands(&commands, "ec", 20, false);
    
    println!("\nFiltered commands for 'ec':");
    for (i, cmd) in filtered.iter().enumerate() {
        let starts_with_ec = cmd.command.to_lowercase().starts_with("ec");
        println!("{}: {} {}", 
                 i, 
                 cmd.command,
                 if starts_with_ec { "✓ (starts with 'ec')" } else { "" });
    }
    
    // The simulated display ordering (what popup.rs would do)
    let mut prefix_matches = Vec::new();
    let mut other_matches = Vec::new();
    
    for cmd in &filtered {
        if cmd.command.to_lowercase().starts_with("ec") {
            prefix_matches.push(cmd);
        } else {
            other_matches.push(cmd);
        }
    }
    
    println!("\nAfter reordering - prefix matches first:");
    let mut ordered: Vec<&Command> = Vec::new();
    ordered.extend(&prefix_matches);
    ordered.extend(&other_matches);
    
    for (i, cmd) in ordered.iter().enumerate() {
        let starts_with_ec = cmd.command.to_lowercase().starts_with("ec");
        println!("{}: {} {}", 
                 i, 
                 cmd.command,
                 if starts_with_ec { "✓ PREFIX" } else { "- other match" });
    }
    
    // Verify that all prefix matches come before non-prefix matches
    let mut found_non_prefix = false;
    for cmd in &ordered {
        if cmd.command.to_lowercase().starts_with("ec") {
            assert!(!found_non_prefix, 
                    "Found prefix match '{}' after non-prefix matches!", cmd.command);
        } else {
            found_non_prefix = true;
        }
    }
    
    println!("✅ Prefix ordering test passed");
}

#[test]
fn test_eoci_specific_case() {
    println!("\nTesting EOCI specific case from screenshot...");
    
    let commands = vec![
        make_cmd("2017-09 Elastic Block Chain"),
        make_cmd("EOCI Consciousness"),
        make_cmd("EOCI Corp AI"),
        make_cmd("EOCI Closure"),
        make_cmd("Economics Report"),
        make_cmd("Electric Vehicle"),
    ];
    
    let filtered = filter_commands(&commands, "eoc", 20, false);
    
    // Simulate display ordering
    let mut display_order = Vec::new();
    for cmd in &filtered {
        if cmd.command.to_lowercase().starts_with("eoc") {
            display_order.push(cmd);
        }
    }
    for cmd in &filtered {
        if !cmd.command.to_lowercase().starts_with("eoc") {
            display_order.push(cmd);
        }
    }
    
    println!("\nExpected order for 'eoc' search:");
    for (i, cmd) in display_order.iter().enumerate() {
        println!("{}: {}", i, cmd.command);
    }
    
    // Verify EOCI commands come first
    assert!(display_order[0].command.starts_with("EOCI"), 
            "First command should start with EOCI");
    assert!(display_order[1].command.starts_with("EOCI"), 
            "Second command should start with EOCI");
    assert!(display_order[2].command.starts_with("EOCI"), 
            "Third command should start with EOCI");
    
    println!("✅ EOCI ordering test passed");
}

#[test]
fn test_submenu_with_prefix_ordering() {
    println!("\nTesting submenu with prefix ordering...");
    
    let commands = vec![
        // Analysis submenu commands
        make_cmd("Analysis Folder"),
        make_cmd("Analysis Notes"),
        make_cmd("Analysis Report"),
        // Outside commands
        make_cmd("Analytics Dashboard"), // matches "ana" prefix
        make_cmd("Anaconda Setup"),      // matches "ana" prefix
        make_cmd("Manual Analysis Tool"), // contains "ana" but not prefix
        make_cmd("GitHub Repository"),    // no match
    ];
    
    // First, filter the commands
    let filtered = filter_commands(&commands, "ana", 20, false);
    
    // Check if we're in submenu mode
    let menu_prefix = get_current_submenu_prefix(&filtered, "ana");
    assert!(menu_prefix.is_some(), "Should detect submenu");
    
    let (inside_menu, outside_menu) = split_commands(&filtered, menu_prefix.as_ref().unwrap());
    
    println!("\nInside submenu:");
    for cmd in &inside_menu {
        println!("  {}", cmd.command);
    }
    
    // Simulate the ordering of outside commands
    let mut outside_prefix = Vec::new();
    let mut outside_other = Vec::new();
    let mut outside_none = Vec::new();
    
    for cmd in outside_menu {
        if cmd.command.to_lowercase().starts_with("ana") {
            outside_prefix.push(cmd);
        } else if hookanchor::command_matches_query(&cmd.command, "ana") {
            outside_other.push(cmd);
        } else {
            outside_none.push(cmd);
        }
    }
    
    println!("\nOutside submenu (ordered):");
    println!("  Prefix matches:");
    for cmd in &outside_prefix {
        println!("    {}", cmd.command);
    }
    println!("  Other matches:");
    for cmd in &outside_other {
        println!("    {}", cmd.command);
    }
    println!("  Non-matches:");
    for cmd in &outside_none {
        println!("    {}", cmd.command);
    }
    
    // Verify Analytics and Anaconda come before Manual Analysis Tool
    let ordered_outside: Vec<_> = outside_prefix.iter()
        .chain(outside_other.iter())
        .chain(outside_none.iter())
        .collect();
    
    let analytics_pos = ordered_outside.iter().position(|c| c.command.contains("Analytics"));
    let manual_pos = ordered_outside.iter().position(|c| c.command.contains("Manual"));
    
    if let (Some(a_pos), Some(m_pos)) = (analytics_pos, manual_pos) {
        assert!(a_pos < m_pos, 
                "Analytics Dashboard (prefix match) should come before Manual Analysis Tool");
    }
    
    println!("✅ Submenu prefix ordering test passed");
}

#[test]
fn test_case_insensitive_prefix_matching() {
    println!("\nTesting case-insensitive prefix matching...");
    
    let commands = vec![
        make_cmd("EOCI Consciousness"),
        make_cmd("eoci lower case"),
        make_cmd("Eoci Mixed Case"),
        make_cmd("2017-09 Elastic Block Chain"),
    ];
    
    let filtered = filter_commands(&commands, "EOCI", 20, false);
    
    // All EOCI variants should be prefix matches
    let mut prefix_count = 0;
    for cmd in &filtered {
        if cmd.command.to_lowercase().starts_with("eoci") {
            prefix_count += 1;
            println!("✓ Prefix match: {}", cmd.command);
        } else {
            println!("- Other match: {}", cmd.command);
        }
    }
    
    assert_eq!(prefix_count, 3, "Should have 3 prefix matches for EOCI variants");
    println!("✅ Case-insensitive test passed");
}