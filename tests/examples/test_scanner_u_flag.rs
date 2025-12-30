use hookanchor::{load_commands_raw, Command};

fn main() {
    println!("Testing scanner preservation of U flag commands...");
    println!("=================================================");
    
    // Load current commands
    let commands = load_commands_raw();
    
    // Find commands with U flag that are scanner-generated types
    // Note: anchors are identified by 'A' flag, not action type
    let u_flag_scanner_commands: Vec<_> = commands.iter()
        .filter(|cmd| {
            let is_scanner_type = cmd.action == "obs" || cmd.is_anchor() || cmd.action == "folder";
            let has_u_flag = cmd.flags.contains('U');
            is_scanner_type && has_u_flag
        })
        .collect();
    
    println!("Found {} scanner-type commands with U flag:", u_flag_scanner_commands.len());
    for cmd in &u_flag_scanner_commands {
        println!("  '{}' | action: {} | flags: '{}' | arg: {}", 
            cmd.command, cmd.action, cmd.flags, cmd.arg);
    }
    
    // Test the retain logic manually
    println!("\nTesting retain logic:");
    let mut test_commands = commands.clone();
    
    let before_count = test_commands.len();
    let scanner_before = test_commands.iter()
        .filter(|cmd| cmd.action == "obs" || cmd.is_anchor() || cmd.action == "folder")
        .count();
    let u_flag_scanner_before = test_commands.iter()
        .filter(|cmd| {
            let is_scanner_type = cmd.action == "obs" || cmd.is_anchor() || cmd.action == "folder";
            let has_u_flag = cmd.flags.contains('U');
            is_scanner_type && has_u_flag
        })
        .count();
    
    println!("Before retain: {} total, {} scanner-type, {} with U flag", 
        before_count, scanner_before, u_flag_scanner_before);
    
    // Apply the new retain logic (anchors identified by 'A' flag, not action type)
    test_commands.retain(|cmd| {
        let is_scanner_generated = cmd.action == "obs" || cmd.is_anchor() || cmd.action == "folder";
        let is_user_edited = cmd.flags.contains('U');

        // Keep command if it's NOT scanner-generated OR if it's user-edited
        !is_scanner_generated || is_user_edited
    });
    
    let after_count = test_commands.len();
    let scanner_after = test_commands.iter()
        .filter(|cmd| cmd.action == "obs" || cmd.is_anchor() || cmd.action == "folder")
        .count();
    let u_flag_scanner_after = test_commands.iter()
        .filter(|cmd| {
            let is_scanner_type = cmd.action == "obs" || cmd.is_anchor() || cmd.action == "folder";
            let has_u_flag = cmd.flags.contains('U');
            is_scanner_type && has_u_flag
        })
        .count();
    
    println!("After retain: {} total, {} scanner-type, {} with U flag", 
        after_count, scanner_after, u_flag_scanner_after);
    
    // Verify that all U flag commands were preserved
    if u_flag_scanner_before == u_flag_scanner_after {
        println!("✅ SUCCESS: All {} U flag scanner commands were preserved!", u_flag_scanner_after);
    } else {
        println!("❌ ERROR: Expected {} U flag commands, but only {} preserved", 
            u_flag_scanner_before, u_flag_scanner_after);
    }
    
    // Show what was removed
    let removed_commands = scanner_before - scanner_after;
    println!("Removed {} scanner-generated commands without U flag", removed_commands);
    
    // Show a few examples of preserved U flag commands
    if !u_flag_scanner_commands.is_empty() {
        println!("\nExamples of preserved U flag commands:");
        for cmd in u_flag_scanner_commands.iter().take(5) {
            println!("  '{}' ({})", cmd.command, cmd.action);
        }
    }
}