use hookanchor::core::{load_commands_with_data, Command};

fn main() {
    println!("Testing meditation patch assignment and cycle detection...\n");
    
    // Load all data
    let (_config, commands, patches) = load_commands_with_data();
    
    // Find the meditation commands
    let meditation_misc: Option<&Command> = commands.iter()
        .find(|cmd| cmd.command == "meditation" && cmd.patch == "Misc");
        
    let meditation_meditation: Option<&Command> = commands.iter()
        .find(|cmd| cmd.command == "meditation" && cmd.patch == "meditation");
        
    println!("Meditation commands found:");
    if let Some(cmd) = meditation_misc {
        println!("\n1. Misc patch meditation:");
        println!("   Command: '{}'", cmd.command);
        println!("   Patch: '{}'", cmd.patch);
        println!("   Path: '{}'", cmd.arg);
        println!("   Flags: '{}'", cmd.flags);
    }
    
    if let Some(cmd) = meditation_meditation {
        println!("\n2. meditation patch meditation:");
        println!("   Command: '{}'", cmd.command);
        println!("   Patch: '{}'", cmd.patch);
        println!("   Path: '{}'", cmd.arg);
        println!("   Flags: '{}'", cmd.flags);
    }
    
    // Check the meditation patch
    if let Some(meditation_patch) = patches.get("meditation") {
        println!("\n\nMeditation patch details:");
        if let Some(ref linked_cmd) = meditation_patch.linked_command {
            println!("   Linked to: '{}'", linked_cmd.command);
            println!("   Linked patch: '{}'", linked_cmd.patch);
            println!("   Linked path: '{}'", linked_cmd.arg);
            
            // This is the key issue - if the meditation patch links to a command
            // that itself has "Misc" as its patch, we have a cycle or redirection
            if linked_cmd.patch == "Misc" {
                println!("\n   WARNING: Meditation patch links to a command in Misc patch!");
                println!("   This could cause patch assignment confusion.");
            }
        }
    }
    
    // Check if Misc patch exists and what it links to
    if let Some(misc_patch) = patches.get("misc") {
        println!("\n\nMisc patch details:");
        if let Some(ref linked_cmd) = misc_patch.linked_command {
            println!("   Linked to: '{}'", linked_cmd.command);
            println!("   Linked patch: '{}'", linked_cmd.patch);
            println!("   Linked path: '{}'", linked_cmd.arg);
        }
    }
    
    // Trace through the patch hierarchy
    println!("\n\nPatch hierarchy analysis:");
    
    // Start with meditation patch
    let mut current_patch = "meditation".to_string();
    let mut depth = 0;
    let mut visited = std::collections::HashSet::new();
    
    loop {
        if visited.contains(&current_patch) {
            println!("{}CYCLE DETECTED at '{}'!", "  ".repeat(depth), current_patch);
            break;
        }
        
        let patch = match patches.get(&current_patch) {
            Some(p) => p,
            None => {
                println!("{}Patch '{}' not found", "  ".repeat(depth), current_patch);
                break;
            }
        };
        
        visited.insert(current_patch.clone());
        
        println!("{}'{}' patch", "  ".repeat(depth), current_patch);
        
        if let Some(ref linked_cmd) = patch.linked_command {
            if !linked_cmd.patch.is_empty() {
                println!("{}→ links to command with patch '{}'", "  ".repeat(depth + 1), linked_cmd.patch);
                current_patch = linked_cmd.patch.to_lowercase();
                depth += 1;
            } else {
                println!("{}→ links to command with no patch (root)", "  ".repeat(depth + 1));
                break;
            }
        } else {
            println!("{}→ no linked command", "  ".repeat(depth + 1));
            break;
        }
        
        if depth > 10 {
            println!("{}Maximum depth reached - possible infinite loop", "  ".repeat(depth));
            break;
        }
    }
}