use hookanchor::core::config::{Config, PopupSettings, LauncherSettings, ScannerSettings};
use hookanchor::core::commands::{
    Command,
    get_current_submenu_prefix_from_commands, 
    get_submenu_commands, 
    get_command_prefix
};

#[cfg(test)]
mod submenu_tests {
    use super::*;

    fn create_test_config() -> Config {
        Config {
            popup_settings: PopupSettings {
                max_rows: 15,
                max_columns: 3,
                debug_log: None,
                verbose_logging: None,
                debug_scanner: None,
                listed_actions: None,
                merge_similar: true,
                word_separators: " ._-".to_string(),
                scan_interval_seconds: None,
                idle_timeout_seconds: None,
                countdown_seconds: Some(5),
            },
            launcher_settings: Some(LauncherSettings::default()),
            scanner_settings: Some(ScannerSettings::default()),
            functions: None,
            markdown_roots: None,
            grabber_rules: None,
            keybindings: None,
        }
    }

    fn create_command(command: &str, action: &str, arg: &str) -> Command {
        Command {
            patch: "".to_string(),
            command: command.to_string(),
            action: action.to_string(),
            arg: arg.to_string(),
            flags: "".to_string(),
            full_line: format!("{} : {} {}", command, action, arg),
        }
    }

    #[test]
    fn test_command_prefix_extraction() {
        let config = create_test_config();
        let word_separators = &config.popup_settings.word_separators;

        // Test basic prefix extraction
        assert_eq!(get_command_prefix("CC Coach", word_separators), "CC");
        assert_eq!(get_command_prefix("CC_Core", word_separators), "CC");
        assert_eq!(get_command_prefix("CC.App", word_separators), "CC");
        assert_eq!(get_command_prefix("CC-Test", word_separators), "CC");
        
        // Test commands without separators
        assert_eq!(get_command_prefix("CCCore", word_separators), "CCCore");
        
        // Test single character commands
        assert_eq!(get_command_prefix("C", word_separators), "C");
        
        // Test empty command
        assert_eq!(get_command_prefix("", word_separators), "");
    }

    #[test]
    fn test_cc_submenu_detection_issue() {
        let config = create_test_config();
        
        // Create the types of commands that might appear for "CC" search
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

        let search_text = "CC";
        let word_separators = &config.popup_settings.word_separators;

        // Test submenu detection
        let submenu_prefix = get_current_submenu_prefix_from_commands(
            &commands, search_text, word_separators
        );

        println!("Submenu prefix for '{}': {:?}", search_text, submenu_prefix);

        // Check which commands have "CC" as prefix
        let cc_prefix_commands: Vec<_> = commands.iter()
            .filter(|cmd| get_command_prefix(&cmd.command, word_separators).to_lowercase() == "cc")
            .collect();

        println!("Commands with 'CC' prefix: {:?}", 
            cc_prefix_commands.iter().map(|c| &c.command).collect::<Vec<_>>());

        // This should tell us if submenu mode is being incorrectly triggered
        if let Some(prefix) = submenu_prefix {
            println!("Submenu mode triggered with prefix: {}", prefix);
            
            // Get the actual submenu split
            let submenu_commands = get_submenu_commands(&commands, &prefix, word_separators);
            
            println!("Total submenu commands: {}", submenu_commands.len());
            for (i, cmd) in submenu_commands.iter().enumerate() {
                println!("  {}: {} (action: {})", i, cmd.command, cmd.action);
            }
            
            // Check for separator
            let separator_pos = submenu_commands.iter().position(|cmd| cmd.action == "separator");
            println!("Separator position: {:?}", separator_pos);
            
        } else {
            println!("No submenu mode triggered");
        }
    }

    #[test]
    fn test_separator_insertion() {
        let config = create_test_config();
        let word_separators = &config.popup_settings.word_separators;

        let commands = vec![
            create_command("CC Coach", "app", "Coach"),
            create_command("CC Core", "app", "Core"), 
            create_command("AA Other", "app", "Other"),
            create_command("BB Another", "app", "Another"),
        ];

        let submenu_commands = get_submenu_commands(&commands, "CC", word_separators);
        
        // Should have: CC Coach, CC Core, ---, AA Other, BB Another
        assert_eq!(submenu_commands.len(), 5);
        
        // Find separator
        let separator_pos = submenu_commands.iter().position(|cmd| cmd.action == "separator");
        assert!(separator_pos.is_some(), "Separator should be present");
        assert_eq!(separator_pos.unwrap(), 2, "Separator should be at position 2");
        
        // Check separator command
        let separator = &submenu_commands[separator_pos.unwrap()];
        assert_eq!(separator.command, "---");
        assert_eq!(separator.action, "separator");
    }

    #[test]
    fn test_no_false_submenu_detection() {
        let config = create_test_config();
        let word_separators = &config.popup_settings.word_separators;

        // Commands that start with "CC" but shouldn't trigger submenu
        let commands = vec![
            create_command("code_core *", "obs", "some/path"),  // starts with 'c', not 'CC'
            create_command("CAR Coach", "app", "Coach"),        // starts with 'CAR', not 'CC'
            create_command("CV Something", "app", "CV"),        // starts with 'CV', not 'CC'
        ];

        let submenu_prefix = get_current_submenu_prefix_from_commands(
            &commands, "CC", word_separators
        );

        // Should NOT detect submenu since no commands actually start with "CC"
        assert!(submenu_prefix.is_none(), 
            "Should not detect submenu when no commands have the 'CC' prefix");
    }

    #[test]
    fn test_minimum_commands_for_submenu() {
        let config = create_test_config();
        let word_separators = &config.popup_settings.word_separators;

        // Only one command with CC prefix - should not trigger submenu
        let commands_one = vec![
            create_command("CC Coach", "app", "Coach"),
            create_command("DD Other", "app", "Other"),
        ];

        let submenu_prefix = get_current_submenu_prefix_from_commands(
            &commands_one, "CC", word_separators
        );
        assert!(submenu_prefix.is_none(), "Should not trigger submenu with only 1 matching command");

        // Two commands with CC prefix - should trigger submenu
        let commands_two = vec![
            create_command("CC Coach", "app", "Coach"),
            create_command("CC Core", "app", "Core"),
            create_command("DD Other", "app", "Other"),
        ];

        let submenu_prefix = get_current_submenu_prefix_from_commands(
            &commands_two, "CC", word_separators
        );
        assert!(submenu_prefix.is_some(), "Should trigger submenu with 2+ matching commands");
        assert_eq!(submenu_prefix.unwrap(), "CC");
    }
}