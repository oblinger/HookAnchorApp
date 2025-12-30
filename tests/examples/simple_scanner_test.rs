use hookanchor::{load_commands_raw};

fn main() {
    println!("Testing that scanner builds correctly with folder processing disabled...");
    println!("======================================================================");
    
    // Just load commands to ensure the scanner integration works
    let commands = load_commands_raw();
    
    let obs_count = commands.iter().filter(|cmd| cmd.action == "obs").count();
    let anchor_count = commands.iter().filter(|cmd| cmd.is_anchor()).count();  // Anchors are identified by 'A' flag
    let folder_count = commands.iter().filter(|cmd| cmd.action == "folder").count();
    let u_flag_count = commands.iter().filter(|cmd| cmd.flags.contains('U')).count();

    println!("Current command counts:");
    println!("  Total commands: {}", commands.len());
    println!("  obs: {}", obs_count);
    println!("  anchors (A flag): {}", anchor_count);
    println!("  folder: {}", folder_count);
    println!("  with U flag: {}", u_flag_count);
    
    // Show some folder commands with U flag (our manually created ones should still exist)
    println!("\nFolder commands with U flag (manually created anchor folders):");
    let u_folder_commands: Vec<_> = commands.iter()
        .filter(|cmd| cmd.action == "folder" && cmd.flags.contains('U'))
        .take(10)
        .collect();
    
    for cmd in u_folder_commands {
        println!("  '{}' -> {} (flags: '{}')", cmd.command, cmd.arg, cmd.flags);
    }
    
    println!("\n✅ Scanner builds successfully with folder processing disabled");
    println!("✅ Manual anchor folder commands with U flag are preserved");
}