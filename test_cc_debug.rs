use anchor_selector::core::config::{Config, PopupSettings};
use anchor_selector::core::commands::{
    Command,
    get_current_submenu_prefix_from_commands, 
    get_submenu_commands, 
    get_command_prefix
};

fn create_test_config() -> Config {
    Config {
        popup_settings: PopupSettings {
            max_rows: 15,
            max_columns: 3,
            use_new_launcher: true,
            debug_log: None,
            listed_actions: None,
            merge_similar: true,
            word_separators: " ._-".to_string(),
            scan_root: None,
            scan_interval_seconds: None,
        },
        launcher_settings: None,
        functions: None,
        markdown_roots: None,
        grabber_rules: None,
        keybindings: None,
    }
}

fn create_command(command: &str, action: &str, arg: &str) -> Command {
    Command {
        group: "".to_string(),
        command: command.to_string(),
        action: action.to_string(),
        arg: arg.to_string(),
        flags: "".to_string(),
        full_line: format!("{} : {} {}", command, action, arg),
    }
}

fn main() {
    let config = create_test_config();
    
    println!("=== Test 1: Commands WITHOUT CC prefix (like user's screenshot) ===");
    test_scenario_1(&config);
    
    println!("\n=== Test 2: Commands WITH some CC prefixes (expected submenu) ===");
    test_scenario_2(&config);
    
    println!("\n=== Test 3: Mixed case CC commands ===");
    test_scenario_3(&config);
}

fn test_scenario_1(config: &Config) {
    // Create the types of commands that might appear for "CC" search
    // These are similar to what the user showed in the screenshot
    let commands = vec![
        create_command("code_core *", "obs", "prj/binproj/code_core/code_core.md"),
        create_command("CAR Coach App", "app", "CAR Coach"),
        create_command("CV Confluence", "url", "https://confluence.example.com"),
        create_command("Cookie Clicker *", "obs", "games/Cookie Clicker.md"),
        create_command("CODE_OF_CONDUCT *", "obs", "prj/CODE_OF_CONDUCT.md"),
        create_command("Christmas Cards *", "obs", "personal/Christmas Cards.md"),
        create_command("COMS Conferences *", "obs", "work/COMS Conferences.md"),
        create_command("Clozure CL App", "app", "Clozure CL"),
        create_command("Google Cloud Console", "url", "https://console.cloud.google.com"),
        create_command("COM Campaigns OLDER *", "obs", "marketing/COM Campaigns.md"),
    ];

    analyze_commands(&commands, "CC", config);
}

fn test_scenario_2(config: &Config) {
    // Commands with actual CC prefixes that SHOULD trigger submenu
    let commands = vec![
        create_command("CC Core App", "app", "CC Core"),
        create_command("CC Coach Tool", "app", "CC Coach"),
        create_command("code_core *", "obs", "prj/binproj/code_core/code_core.md"),
        create_command("CAR Coach App", "app", "CAR Coach"),
        create_command("CV Confluence", "url", "https://confluence.example.com"),
    ];

    analyze_commands(&commands, "CC", config);
}

fn test_scenario_3(config: &Config) {
    // Test case sensitivity issues
    let commands = vec![
        create_command("cc core", "app", "cc core"),  // lowercase cc
        create_command("Cc Coach", "app", "Cc Coach"), // mixed case
        create_command("CC Tool", "app", "CC Tool"),   // uppercase CC
        create_command("code_core *", "obs", "prj/binproj/code_core/code_core.md"),
    ];

    analyze_commands(&commands, "CC", config);
}

fn analyze_commands(commands: &Vec<Command>, search_text: &str, config: &Config) {
    let word_separators = &config.popup_settings.word_separators;

    println!("Search text: '{}'", search_text);
    println!("Word separators: '{}'", word_separators);
    println!();

    // Check which commands have the search prefix
    println!("Command prefix analysis:");
    let mut cc_count = 0;
    for cmd in commands {
        let prefix = get_command_prefix(&cmd.command, word_separators);
        let matches_search = prefix.to_lowercase() == search_text.to_lowercase();
        if matches_search {
            cc_count += 1;
        }
        println!("  '{}' -> prefix: '{}' -> matches {}: {}", 
            cmd.command, prefix, search_text, matches_search);
    }
    println!("Total commands with '{}' prefix: {}", search_text, cc_count);
    println!();

    // Test submenu detection with debug
    println!("=== Submenu Detection Debug ===");
    let mut prefix_counts: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
    
    for cmd in commands {
        if cmd.action == "separator" {
            continue;
        }
        let cmd_prefix = get_command_prefix(&cmd.command, word_separators);
        let matches = cmd_prefix.to_lowercase() == search_text.to_lowercase();
        println!("  Command: '{}' -> prefix: '{}' -> matches search '{}': {}", 
            cmd.command, cmd_prefix, search_text, matches);
        
        if matches {
            *prefix_counts.entry(cmd_prefix.clone()).or_insert(0) += 1;
            println!("    -> Added to prefix_counts: '{}' (count now: {})", 
                cmd_prefix, prefix_counts[&cmd_prefix]);
        }
    }
    
    println!("Final prefix_counts: {:?}", prefix_counts);
    
    // Check which prefixes qualify (count >= 2)
    println!("Qualifying prefixes (count >= 2):");
    for (prefix, count) in &prefix_counts {
        if *count >= 2 {
            println!("  '{}': {} commands", prefix, count);
        }
    }

    let submenu_prefix = get_current_submenu_prefix_from_commands(
        commands, search_text, word_separators
    );

    println!("Submenu prefix result: {:?}", submenu_prefix);

    if let Some(prefix) = submenu_prefix {
        println!("✓ Submenu mode triggered with prefix: {}", prefix);
        
        // Get the actual submenu split
        let submenu_commands = get_submenu_commands(commands, &prefix, word_separators);
        
        println!("Total submenu commands: {}", submenu_commands.len());
        for (i, cmd) in submenu_commands.iter().enumerate() {
            let is_separator = cmd.action == "separator";
            println!("  {}: {} (action: {}) {}", 
                i, cmd.command, cmd.action, 
                if is_separator { "<-- SEPARATOR" } else { "" });
        }
        
        // Check for separator
        let separator_pos = submenu_commands.iter().position(|cmd| cmd.action == "separator");
        println!("Separator position: {:?}", separator_pos);
        
    } else {
        println!("✗ No submenu mode triggered (this is correct if no commands have '{}' prefix)", search_text);
    }
}