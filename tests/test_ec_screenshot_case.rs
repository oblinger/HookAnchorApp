use hookanchor::{Command, filter_commands};

/// Helper to create a test command
fn make_cmd(name: &str) -> Command {
    Command {
        patch: String::new(),
        command: name.to_string(),
        action: "test".to_string(),
        arg: String::new(),
        full_line: format!("{} : test", name),
    }
}

#[test]
fn test_ec_search_exact_screenshot_case() {
    println!("Testing exact 'ec' search case from screenshot...");
    
    // Commands visible in the screenshot
    let commands = vec![
        make_cmd("2017-09 Elastic Block Chain"),
        make_cmd("2017-09 Elastic Block Chain Note"),
        make_cmd("2017-09 Elastic Block Chain Obsidian"),
        make_cmd("2024-06-13 - Juan EC2 setup..."),
        make_cmd("EOCA Civilizing tendency"),
        make_cmd("EOCA Civilizing tendency Obsidian"),
        make_cmd("EOCB End Run – Cyber Model"),
        make_cmd("EOCB End Run – Cyber Model Obsidian"),
        make_cmd("EOCB Engines Of Control Book"),
        make_cmd("EOCB Engines Of Control Book Obsidian"),
        make_cmd("EOCI Closure"),
        make_cmd("EOCI Closure Obsidian"),
        make_cmd("EOCI Consciousness"),
        make_cmd("EOCI Consciousness Obsidian"),
        make_cmd("EOCI Corp AI"),
        make_cmd("EOCI Corp AI Obsidian"),
        make_cmd("EOCI Corp Decision Making..."),
        make_cmd("EOCI Executive Creep..."),
        make_cmd("EOCI Human Reasoning Conjecture"),
        make_cmd("EOCI Human Reasoning Conjecture Obsidian"),
        make_cmd("EOCI Pax Corporana"),
        make_cmd("EOCI Pax Corporana Obsidian"),
        make_cmd("EOCI Perilous Cyberspace"),
        make_cmd("EOCI Perilous Cyberspace Obsidian"),
    ];
    
    // Filter with "ec"
    let filtered = filter_commands(&commands, "ec", 30, false);
    
    println!("\nFiltered commands for 'ec':");
    for (i, cmd) in filtered.iter().enumerate() {
        let prefix_match = cmd.command.to_lowercase().starts_with("ec");
        println!("{:2}: {} {}", 
                 i, 
                 cmd.command,
                 if prefix_match { "✓ PREFIX" } else { "" });
    }
    
    // Simulate the display ordering that popup.rs would apply
    let mut prefix_matches = Vec::new();
    let mut other_matches = Vec::new();
    
    for cmd in &filtered {
        if cmd.command.to_lowercase().starts_with("ec") {
            prefix_matches.push(cmd);
        } else {
            other_matches.push(cmd);
        }
    }
    
    println!("\nAfter proper ordering (what should be displayed):");
    println!("=== Prefix matches first ===");
    for (i, cmd) in prefix_matches.iter().enumerate() {
        println!("{:2}: {}", i, cmd.command);
    }
    
    println!("\n=== Other matches after ===");
    for (i, cmd) in other_matches.iter().enumerate() {
        println!("{:2}: {}", i + prefix_matches.len(), cmd.command);
    }
    
    // The key assertion: "2017-09 Elastic Block Chain" should NOT be first
    assert!(!filtered.is_empty(), "Should have filtered results");
    
    // After ordering, commands starting with "ec" should come first
    let first_after_ordering = if !prefix_matches.is_empty() {
        &prefix_matches[0].command
    } else if !other_matches.is_empty() {
        &other_matches[0].command
    } else {
        panic!("No matches found");
    };
    
    println!("\n✅ First command after ordering: {}", first_after_ordering);
    assert!(!first_after_ordering.starts_with("2017"), 
            "2017-09 Elastic Block Chain should NOT be first!");
    
    // Should have some EOCA/EOCB/EOCI commands as prefix matches
    assert!(!prefix_matches.is_empty(), "Should have prefix matches for 'ec'");
    
    println!("✅ Test passed - prefix ordering is correct");
}