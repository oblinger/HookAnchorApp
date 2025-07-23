use hookanchor::{load_commands, save_commands_to_file, auto_assign_patches};

fn main() {
    println!("Loading commands from commands.txt...");
    let mut commands = load_commands();
    let original_count = commands.len();
    
    println!("Found {} commands", original_count);
    
    // Count commands without patches before
    let without_patch_before = commands.iter().filter(|cmd| cmd.patch.is_empty()).count();
    println!("Commands without patches: {}", without_patch_before);
    
    // Auto-assign patches based on shared prefixes
    println!("\nAuto-assigning patches based on shared prefixes...");
    auto_assign_patches(&mut commands);
    
    // Count commands without patches after
    let without_patch_after = commands.iter().filter(|cmd| cmd.patch.is_empty()).count();
    println!("Commands without patches after: {}", without_patch_after);
    println!("Patches assigned: {}", without_patch_before - without_patch_after);
    
    // Show some examples of patches assigned
    println!("\nExamples of patches assigned:");
    let mut shown = 0;
    for cmd in &commands {
        if !cmd.patch.is_empty() && cmd.command.contains(' ') {
            if let Some(space_idx) = cmd.command.find(' ') {
                let prefix = &cmd.command[..space_idx];
                if prefix == cmd.patch {
                    println!("  '{}' → patch: '{}'", cmd.command, cmd.patch);
                    shown += 1;
                    if shown >= 10 {
                        break;
                    }
                }
            }
        }
    }
    
    // Save the updated commands
    println!("\nSaving updated commands to commands.txt...");
    if let Err(e) = save_commands_to_file(&commands) {
        eprintln!("Error saving commands: {}", e);
        std::process::exit(1);
    }
    
    println!("Commands successfully updated with patch assignments!");
    
    // Show patch statistics
    use std::collections::HashMap;
    let mut patch_counts: HashMap<String, usize> = HashMap::new();
    for cmd in &commands {
        if !cmd.patch.is_empty() {
            *patch_counts.entry(cmd.patch.clone()).or_insert(0) += 1;
        }
    }
    
    println!("\nPatch statistics:");
    let mut patches: Vec<_> = patch_counts.into_iter().collect();
    patches.sort_by(|a, b| b.1.cmp(&a.1)); // Sort by count descending
    
    for (patch, count) in patches.iter().take(20) {
        println!("  {}: {} commands", patch, count);
    }
}