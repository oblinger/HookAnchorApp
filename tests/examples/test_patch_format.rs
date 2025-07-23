use hookanchor::{Command, save_commands_to_file};

fn main() {
    // Create test commands to show the new patch format
    let commands = vec![
        Command {
            patch: "CV".to_string(),  // New patch field
            command: "CV Ana".to_string(),
            action: "work".to_string(),
            arg: "https://docs.google.com/document/d/1SUE-2Y6zARBB8yQltgbbNtC1M20WqpAOyRdtgJjcWNI/edit".to_string(),
            flags: String::new(),
            full_line: String::new(),
        },
        Command {
            patch: "CV".to_string(),
            command: "CV Confluence".to_string(),
            action: "work".to_string(),
            arg: "https://software-engineering-team.atlassian.net/wiki/spaces/SVAI/overview".to_string(),
            flags: String::new(),
            full_line: String::new(),
        },
        Command {
            patch: String::new(),  // No patch
            command: "Terminal".to_string(),
            action: "app".to_string(),
            arg: "Terminal".to_string(),
            flags: String::new(),
            full_line: String::new(),
        },
    ];
    
    println!("Commands in memory:");
    for cmd in &commands {
        println!("  Patch: '{}', Command: '{}', Action: '{}'", cmd.patch, cmd.command, cmd.action);
    }
    
    println!("\nGenerated file format:");
    for cmd in &commands {
        let formatted = cmd.to_new_format();
        println!("  {}", formatted);
    }
}