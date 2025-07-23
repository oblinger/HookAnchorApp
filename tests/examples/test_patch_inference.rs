use hookanchor::{load_data};

fn main() {
    println!("Testing patch inference functionality...");
    
    // Use the new comprehensive load_data function
    println!("Loading data with patch inference...");
    let global_data = load_data(Vec::new());
    let (config, commands, patches) = (global_data.config, global_data.commands, global_data.patches);
    
    println!("Loaded {} commands", commands.len());
    println!("Found {} patches", patches.len());
    
    // Count commands without patches
    let without_patch = commands.iter().filter(|cmd| cmd.patch.is_empty()).count();
    let with_patch = commands.len() - without_patch;
    
    println!("Commands with patches: {}", with_patch);
    println!("Commands without patches: {}", without_patch);
    
    // Show some examples of patches
    println!("\nSample patch assignments:");
    let mut shown = 0;
    for cmd in commands.iter().take(20) {
        if !cmd.patch.is_empty() {
            println!("  '{}' → patch: '{}'", cmd.command, cmd.patch);
            shown += 1;
            if shown >= 10 {
                break;
            }
        }
    }
    
    // Show patch statistics
    use std::collections::HashMap;
    let mut patch_counts: HashMap<String, usize> = HashMap::new();
    for cmd in &commands {
        if !cmd.patch.is_empty() {
            *patch_counts.entry(cmd.patch.clone()).or_insert(0) += 1;
        }
    }
    
    println!("\nTop 10 patches by command count:");
    let mut patches_vec: Vec<_> = patch_counts.into_iter().collect();
    patches_vec.sort_by(|a, b| b.1.cmp(&a.1)); // Sort by count descending
    
    for (patch, count) in patches_vec.iter().take(10) {
        println!("  {}: {} commands", patch, count);
    }
}