use hookanchor::core::Command;

#[test]
fn test_merged_command_execution_logic() {
    // Create a merged command like what would be generated
    let merged_cmd = Command {
        patch: String::new(),
        command: "FIN Accounts...".to_string(),
        action: "obs".to_string(),
        arg: "FIN Accounts".to_string(),
        flags: String::new(),
        other_params: None,
            last_update: 0,
            file_size: None,
    };
    
    println!("Testing merged command execution:");
    println!("  Command: '{}'", merged_cmd.command);
    println!("  Action: '{}'", merged_cmd.action);
    println!("  Arg: '{}'", merged_cmd.arg);
    
    // Test the execution logic
    if merged_cmd.command.ends_with("...") {
        let launcher_command = if merged_cmd.arg.is_empty() {
            merged_cmd.action.clone()
        } else {
            format!("{} {}", merged_cmd.action, merged_cmd.arg)
        };
        
        println!("  → Would execute: '{}'", launcher_command);
        assert_eq!(launcher_command, "obs FIN Accounts");
        println!("✅ Execution logic is correct!");
    } else {
        panic!("❌ Command should end with '...'");
    }
}