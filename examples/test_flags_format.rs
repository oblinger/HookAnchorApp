use hookanchor::{Command, parse_command_line};

fn main() {
    println!("Testing command formatting with flags...");
    println!("========================================");
    
    // Test various command formats with flags
    let test_lines = [
        "test1 : action; argument",
        "test2 : action; argument; U",
        "test3 : action; argument; U M",
        "PATCH! test4 : action; argument; U",
        "PATCH! test5 : action;; U",  // No argument, just flags
    ];
    
    for line in test_lines {
        println!("\nParsing: '{}'", line);
        match parse_command_line(line) {
            Ok(cmd) => {
                println!("  Command: '{}'", cmd.command);
                println!("  Action: '{}'", cmd.action);
                println!("  Arg: '{}'", cmd.arg);
                println!("  Flags: '{}'", cmd.flags);
                println!("  Patch: '{}'", cmd.patch);
                
                // Test round-trip formatting
                let formatted = cmd.to_new_format();
                println!("  Formatted: '{}'", formatted);
                
                // Verify round-trip preserves flags
                if let Ok(reparsed) = parse_command_line(&formatted) {
                    if reparsed.flags == cmd.flags {
                        println!("  ✅ Flags preserved in round-trip");
                    } else {
                        println!("  ❌ Flags NOT preserved! Original: '{}', Reparsed: '{}'", 
                            cmd.flags, reparsed.flags);
                    }
                }
            }
            Err(e) => {
                println!("  ❌ Parse error: {}", e);
            }
        }
    }
    
    // Test creating a command with flags programmatically
    println!("\n\nCreating command with flags programmatically:");
    let cmd = Command {
        patch: "CV".to_string(),
        command: "Test Command".to_string(),
        action: "folder".to_string(),
        arg: "/path/to/folder".to_string(),
        flags: "U M".to_string(),
        full_line: String::new(), // Will be generated
    };
    
    let formatted = cmd.to_new_format();
    println!("  Formatted: '{}'", formatted);
    
    // Parse it back
    match parse_command_line(&formatted) {
        Ok(parsed) => {
            println!("  ✅ Successfully parsed back");
            println!("  Flags: '{}'", parsed.flags);
        }
        Err(e) => {
            println!("  ❌ Failed to parse back: {}", e);
        }
    }
}