//! System data management and operations
//! 
//! This module handles all system-wide data operations including loading, saving,
//! caching, and managing the global application state.

use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};
use super::config::Config;
use super::commands::{Command, Patch};

/// System application data structure containing all loaded data
#[derive(Clone, Debug)]
pub struct SysData {
    pub config: Config,
    pub commands: Vec<Command>,
    pub patches: HashMap<String, Patch>,
}

// Global application data - loaded once and reused
static SYS_DATA: OnceLock<Mutex<Option<SysData>>> = OnceLock::new();

/// Gets a reference to the sys data, loading it if necessary
pub fn get_sys_data() -> SysData {
    // Check if we already have sys data cached
    let sys = SYS_DATA.get_or_init(|| Mutex::new(None));
    
    // Try to acquire the lock without blocking to detect potential deadlock
    match sys.try_lock() {
        Ok(mut sys_data) => {
            if let Some(ref data) = *sys_data {
                crate::utils::debug_log("GET_SYS_DATA", "Returning cached sys data");
                return data.clone();
            }
            // Not cached, need to load
            crate::utils::debug_log("GET_SYS_DATA", "No cached data, loading from disk");
            drop(sys_data); // Release the lock before calling load_data
            load_data(Vec::new(), false)
        }
        Err(_) => {
            // Mutex is locked, this indicates a potential deadlock
            crate::utils::debug_log("GET_SYS_DATA", "ERROR: Mutex is locked, potential deadlock detected");
            // Return a fallback - load without caching
            load_data_no_cache(Vec::new(), false)
        }
    }
}

/// Gets just the config for functions that only need configuration
pub fn get_config() -> Config {
    // Load config directly to avoid circular dependency
    super::config::load_config()
}

/// Clears the sys data, forcing the next access to reload from disk
/// This is useful for testing or when external changes to the data files are made
pub fn clear_sys_data() {
    if let Some(sys) = SYS_DATA.get() {
        let mut sys_data = sys.lock().unwrap();
        *sys_data = None;
        crate::utils::debug_log("CLEAR_SYS_DATA", "Cleared sys data cache");
    }
}

/// Load data without caching - used as fallback when cache is locked
fn load_data_no_cache(commands_override: Vec<Command>, verbose: bool) -> SysData {
    // Load config directly without caching
    let config = super::config::load_config();
    
    // Load commands (from disk or use override)
    let commands = if !commands_override.is_empty() {
        commands_override
    } else {
        super::commands::load_commands_raw()
    };
    
    // Create basic patches hashmap
    let patches = super::commands::create_patches_hashmap(&commands);
    
    SysData {
        config,
        commands,
        patches,
    }
}

/// Comprehensive data loading function that performs all necessary steps in order:
/// 1. Load configuration
/// 2. Load commands (from disk or use override)
/// 3. Create patches hashmap from anchor commands
/// 4. Create orphan anchors if needed
/// 5. Run patch inference for commands without patches
/// 6. Create fast lookup maps
/// 7. Normalize patch case
/// 8. Save changes if needed
/// 9. Return fully processed data
pub fn load_data(commands_override: Vec<Command>, verbose: bool) -> SysData {
    // Check if we already have sys data and no commands override
    let sys = SYS_DATA.get_or_init(|| Mutex::new(None));
    let mut sys_data = sys.lock().unwrap();
    
    let use_override = !commands_override.is_empty();
    
    if !use_override {
        if let Some(ref data) = *sys_data {
            crate::utils::debug_log("LOAD_DATA", "Returning cached sys data - no disk reads performed");
            if verbose {
                println!("⚡ Using cached data - no disk reads needed");
            }
            return data.clone();
        }
        crate::utils::debug_log("LOAD_DATA", "First call - loading from disk and performing full initialization");
        if verbose {
            println!("📂 Loading data from disk...");
        }
    } else {
        crate::utils::debug_log("LOAD_DATA", &format!("Using provided commands override ({} commands)", commands_override.len()));
        if verbose {
            println!("🔧 Using provided commands override ({} commands)", commands_override.len());
        }
    }
    
    // Step 1: Load config
    if verbose {
        println!("🔧 Step 1: Loading configuration...");
    }
    let config = super::config::load_config();
    
    let mut commands = if use_override {
        commands_override
    } else {
        let raw_commands = super::commands::load_commands_raw();
        if verbose {
            println!("📋 Step 2: Loaded {} commands from disk", raw_commands.len());
            let empty_patch_count = raw_commands.iter().filter(|c| c.patch.is_empty()).count();
            println!("            {} commands have empty patches", empty_patch_count);
        }
        raw_commands
    };

    // Step 2.5: 
    
    // Step 3: Create patches hashmap from anchor commands
    if verbose {
        println!("🏷️  Step 3: Creating patches hashmap...");
    }
    let mut patches = super::commands::create_patches_hashmap(&commands);
    if verbose {
        println!("   Found {} patches from anchor commands", patches.len());
    }
    
    // Step 3.1: Ensure orphans root patch exists
    if verbose {
        println!("🌳 Step 3.1: Ensuring orphans root patch exists...");
    }
    if let Err(e) = super::commands::ensure_orphans_root_patch(&mut patches, &mut commands, &config) {
        eprintln!("Warning: Failed to ensure orphans root patch: {}", e);
    }
    
    // Step 3.2: Create orphan anchor files and commands for patches without anchors
    // This must happen BEFORE patch inference to catch patches that need anchors
    if verbose {
        println!("🔍 Step 3.2: Looking for orphan patches...");
    }
    let orphan_patches = super::commands::find_orphan_patches(&patches, &commands);
    let orphans_created = if !orphan_patches.is_empty() {
        if verbose {
            println!("   Found {} orphan patches: {:?}", orphan_patches.len(), orphan_patches);
        } else {
            eprintln!("Found {} orphan patches: {:?}", orphan_patches.len(), orphan_patches);
        }
        match super::commands::create_orphan_anchors(&config, &orphan_patches, &mut commands) {
            Ok(count) => {
                if verbose {
                    println!("   ✅ Created {} orphan anchor files and commands", count);
                } else {
                    eprintln!("Created {} orphan anchor files and commands", count);
                }
                // Recreate patches hashmap to include newly created anchor commands
                patches = super::commands::create_patches_hashmap(&commands);
                count
            }
            Err(e) => {
                eprintln!("Warning: Failed to create orphan anchors: {}", e);
                0
            }
        }
    } else {
        if verbose {
            println!("   No orphan patches found");
        }
        0
    };
    
    // Step 4: Infer patches for commands without patches
    if verbose {
        println!("🧩 Step 4: Running patch inference for commands without patches...");
    }
    let (patches_assigned, new_patches_to_add) = super::commands::run_patch_inference(
        &mut commands, 
        &patches, 
        true,  // apply_changes = true (normal operation)
        verbose, // print_to_stdout = verbose
        false  // overwrite_patch = false (only fill empty patches)
    );
    if verbose {
        println!("   Assigned patches to {} commands", patches_assigned);
        println!("   Need to add {} new patches", new_patches_to_add.len());
    }
    
    // Add new patches to hashmap
    for patch_key in new_patches_to_add {
        if !patches.contains_key(&patch_key) {
            // Find the first command whose name matches this patch name
            let matching_command = commands.iter().find(|cmd| {
                cmd.command.to_lowercase() == patch_key
            });
            
            patches.insert(patch_key.clone(), Patch {
                name: patch_key.clone(),
                linked_command: matching_command.cloned(),
            });
        }
    }
    
    // Step 4.5: Create fast lookup maps
    if verbose {
        println!("🗺️  Step 4.5: Creating command-to-patch lookup map...");
    }
    let command_to_patch = super::commands::create_command_to_patch_map(&commands, &patches);
    if verbose {
        println!("   Created fast lookup map for {} commands", command_to_patch.len());
    }
    
    // Step 4.6: Normalize patch case to match anchor commands
    if verbose {
        println!("🔤 Step 4.6: Normalizing patch case to match anchor commands...");
    }
    let normalized_patches = super::commands::normalize_patch_case(&mut commands, &patches);
    if verbose {
        if normalized_patches > 0 {
            println!("   Normalized case for {} patch references", normalized_patches);
        } else {
            println!("   No case normalization needed");
        }
    }
    
    // Step 5: Save commands if any changes were made
    if verbose {
        println!("💾 Step 5: Saving changes if needed...");
    }
    if patches_assigned > 0 || orphans_created > 0 || normalized_patches > 0 {
        crate::utils::debug_log("SAVE_DEBUG", &format!("Saving commands after changes - patches_assigned: {}, orphans_created: {}, normalized_patches: {}", 
            patches_assigned, orphans_created, normalized_patches));
        
        // Debug: Check a few commands before saving
        for (i, cmd) in commands.iter().enumerate() {
            if i < 5 {
                crate::utils::debug_log("SAVE_DEBUG", &format!("Command {} before save: patch='{}', command='{}', action='{}'", 
                    i, cmd.patch, cmd.command, cmd.action));
            }
            if cmd.command == "Patents" {
                crate::utils::debug_log("SAVE_DEBUG", &format!("Patents command before save: patch='{}', command='{}', action='{}'", 
                    cmd.patch, cmd.command, cmd.action));
            }
        }
        
        if let Err(e) = super::commands::save_commands_to_file(&commands) {
            eprintln!("Warning: Failed to save commands after changes: {}", e);
        } else {
            if verbose {
                println!("   ✅ Saved {} commands with changes", commands.len());
            }
            // Don't clear cache here - we're already updating it
            // clear_sys_data() would cause deadlock since we're holding the mutex
        }
    } else {
        if verbose {
            println!("   ⏭️  No changes to save");
        }
    }
    
    // Store in sys data for future calls (only if not using commands override)
    let sys_data_struct = SysData {
        config,
        commands,
        patches,
    };
    
    if !use_override {
        *sys_data = Some(sys_data_struct.clone());
    }
    
    if verbose {
        println!("\n✅ Data loading complete!");
        println!("   Total commands: {}", sys_data_struct.commands.len());
        println!("   Total patches: {}", sys_data_struct.patches.len());
        let commands_with_patches = sys_data_struct.commands.iter()
            .filter(|c| !c.patch.is_empty())
            .count();
        println!("   Commands with patches: {}", commands_with_patches);
        println!("   Commands without patches: {}", sys_data_struct.commands.len() - commands_with_patches);
    }
    
    sys_data_struct
}
