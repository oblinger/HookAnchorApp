use hookanchor::load_commands;

fn main() {
    println!("Testing command parsing...\n");
    
    let commands = load_commands();
    
    // Show first few commands to see if semicolon is in arg
    for (i, cmd) in commands.iter().take(5).enumerate() {
        println!("Command {}: {}", i, cmd.command);
        println!("  Action: '{}'", cmd.action);
        println!("  Arg: '{}'", cmd.arg);
        println!("  Flags: '{}'", cmd.flags);
        println!("  Full line: '{}'", cmd.full_line);
        
        // Check if arg ends with semicolon
        if cmd.arg.ends_with(';') {
            println!("  ⚠️  WARNING: Arg ends with semicolon!");
        }
        println!();
    }
}