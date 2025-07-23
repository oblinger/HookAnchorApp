use hookanchor::{load_data};
use std::path::Path;

fn main() {
    println!("Debugging patch inference order for SCATA command...");
    
    let (_config, commands, patches) = load_data();
    
    // Find the SCATA command
    let scata_command = commands.iter()
        .find(|cmd| cmd.command.contains("2009-05 SCATA Security"))
        .unwrap();
    
    println!("SCATA file path: {}", scata_command.arg);
    
    let file_path = Path::new(&scata_command.arg);
    let dir = file_path.parent().unwrap();
    
    println!("File directory: {:?}", dir);
    
    // Check which patches match this path
    println!("\nChecking patches that could match this path:");
    
    for (patch_key, patch) in &patches {
        if let Some(ref linked_cmd) = patch.linked_command {
            if linked_cmd.is_path_based() {
                let linked_path = Path::new(&linked_cmd.arg);
                let linked_dir = if linked_path.is_file() || linked_cmd.arg.contains('.') {
                    linked_path.parent()
                } else {
                    Some(linked_path)
                };
                
                if let Some(linked_dir) = linked_dir {
                    // Check exact match
                    if dir == linked_dir {
                        println!("  ✅ EXACT MATCH: '{}' -> {:?}", patch.name, linked_dir);
                    }
                    // Check if file is in subdirectory of patch
                    else if let Ok(relative) = dir.strip_prefix(linked_dir) {
                        if !relative.as_os_str().is_empty() {
                            println!("  📂 SUBDIRECTORY: '{}' contains file (via {:?})", patch.name, linked_dir);
                            println!("     Relative path: {:?}", relative);
                        }
                    }
                    // Check if patch dir is subdirectory of file dir (hierarchy walk)
                    else {
                        let mut current_dir = dir.parent();
                        while let Some(parent) = current_dir {
                            if parent == linked_dir {
                                println!("  ⬆️  HIERARCHY: '{}' found by walking up from {:?} to {:?}", patch.name, dir, linked_dir);
                                break;
                            }
                            current_dir = parent.parent();
                        }
                    }
                }
            }
        }
    }
    
    // Test the anchor file detection
    if let Some(file_stem) = file_path.file_stem() {
        let filename = file_stem.to_string_lossy().to_lowercase();
        println!("\nFile stem: '{}'", filename);
        
        // Check if any patch matches this filename
        for (patch_key, patch) in &patches {
            if patch.name.to_lowercase() == filename {
                println!("  📄 ANCHOR FILE: This file matches patch name '{}'", patch.name);
            }
        }
    }
}