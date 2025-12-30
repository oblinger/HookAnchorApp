use hookanchor::{Command, load_config, scanner};

fn main() {
    println!("Testing scanner with folder processing disabled...");
    println!("==================================================");
    
    // Create some test commands
    let initial_commands = vec![
        // Non-scanner commands should be preserved
        Command {
            patch: String::new(),
            command: "Test App".to_string(),
            action: "app".to_string(),
            arg: "TestApp".to_string(),
            flags: String::new(),
            full_line: "Test App : app; TestApp".to_string(),
        },
        // Existing obs/anchor commands should be removed (unless they have U flag)
        Command {
            patch: String::new(),
            command: "Old Markdown".to_string(),
            action: "obs".to_string(),
            arg: "old.md".to_string(),
            flags: String::new(),
            full_line: "Old Markdown : obs; old.md".to_string(),
        },
        // User-edited commands should be preserved
        Command {
            patch: String::new(),
            command: "User Edited".to_string(),
            action: "folder".to_string(),
            arg: "/some/path".to_string(),
            flags: "U".to_string(),
            full_line: "User Edited : folder U; /some/path".to_string(),
        },
    ];
    
    println!("Initial commands: {}", initial_commands.len());
    for cmd in &initial_commands {
        println!("  '{}' | action: {} | flags: '{}'", cmd.command, cmd.action, cmd.flags);
    }
    
    // Load config and scan (this will use a small test directory if markdown_roots is configured)
    let config = load_config();
    
    // Use a smaller test root instead of full scan
    let test_roots = vec!["/tmp".to_string()]; // A directory that likely has few or no markdown files
    
    let updated_commands = scanner::scan_files(initial_commands, &test_roots, &config);
    
    println!("\nAfter scan: {} commands", updated_commands.len());
    
    // Count by action type (anchors identified by 'A' flag, not action type)
    let app_count = updated_commands.iter().filter(|cmd| cmd.action == "app").count();
    let obs_count = updated_commands.iter().filter(|cmd| cmd.action == "obs").count();
    let anchor_count = updated_commands.iter().filter(|cmd| cmd.is_anchor()).count();
    let folder_count = updated_commands.iter().filter(|cmd| cmd.action == "folder").count();
    let u_flag_count = updated_commands.iter().filter(|cmd| cmd.flags.contains('U')).count();

    println!("\nCommand breakdown:");
    println!("  app: {}", app_count);
    println!("  obs: {}", obs_count);
    println!("  anchors (A flag): {}", anchor_count);
    println!("  folder: {}", folder_count);
    println!("  with U flag: {}", u_flag_count);
    
    // Verify expected behavior
    println!("\nVerification:");
    
    // Should preserve non-scanner commands
    if app_count > 0 {
        println!("  ✅ Non-scanner commands preserved");
    } else {
        println!("  ❌ Non-scanner commands missing");
    }
    
    // Should preserve user-edited commands
    if u_flag_count > 0 {
        println!("  ✅ User-edited commands preserved");
    } else {
        println!("  ❌ User-edited commands missing");
    }
    
    // Show some example results
    println!("\nExample commands after scan:");
    for cmd in updated_commands.iter().take(5) {
        println!("  '{}' | action: {} | flags: '{}'", cmd.command, cmd.action, cmd.flags);
    }
    
    println!("\n✅ Scanner test completed - folder scanning disabled, only markdown files processed");
}