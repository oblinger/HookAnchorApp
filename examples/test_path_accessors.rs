use hookanchor::{Command, load_config};

fn main() {
    println!("Testing Command path accessor methods...");
    
    // Load actual config
    let config = load_config();
    
    // Test commands for different actions
    let test_commands = vec![
        // OBS command (relative path)
        Command {
            patch: String::new(),
            command: "Test OBS".to_string(),
            action: "obs".to_string(),
            arg: "RR/Stat/ANOVA.md".to_string(),
            flags: String::new(),
            full_line: String::new(),
        },
        // Anchor command (absolute path)
        Command {
            patch: String::new(),
            command: "Test Anchor".to_string(),
            action: "anchor".to_string(),
            arg: "~/Documents/Project/anchor.md".to_string(),
            flags: String::new(),
            full_line: String::new(),
        },
        // Folder command (relative path)
        Command {
            patch: String::new(),
            command: "Test Folder".to_string(),
            action: "folder".to_string(),
            arg: "MyProject/subfolder".to_string(),
            flags: String::new(),
            full_line: String::new(),
        },
        // Non-path command
        Command {
            patch: String::new(),
            command: "Test Chrome".to_string(),
            action: "chrome".to_string(),
            arg: "https://example.com".to_string(),
            flags: String::new(),
            full_line: String::new(),
        },
    ];
    
    println!("Config vault path: {:?}", 
        config.launcher_settings.as_ref()
            .and_then(|s| s.obsidian_vault_path.as_ref()));
    
    for (i, cmd) in test_commands.iter().enumerate() {
        println!("\nTest {}: '{}' (action: {})", i + 1, cmd.command, cmd.action);
        println!("  Raw arg: '{}'", cmd.arg);
        println!("  Is path-based: {}", cmd.is_path_based());
        
        if let Some(file_path) = cmd.get_absolute_file_path(&config) {
            println!("  Absolute file path: {}", file_path.display());
        } else {
            println!("  Absolute file path: None");
        }
        
        if let Some(folder_path) = cmd.get_absolute_folder_path(&config) {
            println!("  Absolute folder path: {}", folder_path.display());
        } else {
            println!("  Absolute folder path: None");
        }
    }
}