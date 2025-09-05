//! System data management and operations
//! 
//! This module handles all system-wide data operations including loading, saving,
//! caching, and managing the global application state.

use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};
use super::config::Config;
use super::{Command, Patch};

// =============================================================================
// GLOBAL CONSTANTS
// =============================================================================
// Put all hardcoded constants here instead of scattering them throughout the codebase.
// This makes them easy to find and modify in one place.

/// Default log file path - hardcoded for reliability during early initialization
/// Note: This is the ONLY place where the log path is defined. The debug_log config
/// parameter has been removed as it was not being used.
pub const DEFAULT_LOG_PATH: &str = "~/.config/hookanchor/anchor.log";

/// Maximum log file size if config is not loaded (10MB)
pub const DEFAULT_MAX_LOG_SIZE: u64 = 10_000_000;

/// System application data structure containing all loaded data
#[derive(Clone, Debug)]
pub struct SysData {
    pub config: Config,
    pub commands: Vec<Command>,
    pub patches: HashMap<String, Patch>,
}

// Global application data - loaded once and reused
static SYS_DATA: OnceLock<Mutex<Option<SysData>>> = OnceLock::new();

// Global config - loaded once at startup
pub static CONFIG: OnceLock<Config> = OnceLock::new();

/// Initialize the global config at application startup
/// This MUST be called before any other operations
pub fn initialize_config() -> Result<(), String> {
    let start = std::time::Instant::now();
    
    // Load config using the existing load_config_with_error for proper error handling
    match super::config::load_config_with_error() {
        super::config::ConfigResult::Success(config) => {
            CONFIG.set(config).map_err(|_| "Config already initialized".to_string())?;
            let elapsed = start.elapsed();
            crate::utils::log(&format!("CONFIG_INIT: Config initialized at startup in {:?} ({} microseconds)", elapsed, elapsed.as_micros()));
            Ok(())
        }
        super::config::ConfigResult::Error(err) => {
            // Use default config but return error for display
            CONFIG.set(super::config::create_default_config()).map_err(|_| "Config already initialized".to_string())?;
            Err(err)
        }
    }
}

/// Gets a reference to the sys data, loading it if necessary
pub fn get_sys_data() -> SysData {
    // Check if we already have sys data cached
    let sys = SYS_DATA.get_or_init(|| Mutex::new(None));
    
    // Try to acquire the lock without blocking to detect potential deadlock
    match sys.try_lock() {
        Ok(sys_data) => {
            if let Some(ref data) = *sys_data {
                // Cached data found, return it
                return data.clone();
            }
            // Not cached, need to load
            drop(sys_data); // Release the lock before calling load_data
            load_data(Vec::new(), false)
        }
        Err(_) => {
            // Mutex is locked, this indicates a potential deadlock
            crate::utils::detailed_log("GET_SYS_DATA", "ERROR: Mutex is locked, potential deadlock detected");
            // Return a fallback - load without caching
            load_data_no_cache(Vec::new(), false)
        }
    }
}

/// Gets just the config for functions that only need configuration
/// Panics if initialize_config() hasn't been called yet
pub fn get_config() -> Config {
    CONFIG.get().expect("Config not initialized! Call initialize_config() at startup").clone()
}

/// Clears the sys data, forcing the next access to reload from disk
/// This is useful for testing or when external changes to the data files are made
pub fn clear_sys_data() {
    if let Some(sys) = SYS_DATA.get() {
        let mut sys_data = sys.lock().unwrap();
        *sys_data = None;
        crate::utils::detailed_log("CLEAR_SYS_DATA", "Cleared sys data cache");
    }
}

/// Load data without caching - used as fallback when cache is locked
fn load_data_no_cache(commands_override: Vec<Command>, _verbose: bool) -> SysData {
    // Use the pre-initialized config
    let config = get_config();
    
    // Load commands (from disk or use override)
    let commands = if !commands_override.is_empty() {
        commands_override
    } else {
        crate::core::commands::load_commands_raw()
    };
    
    // Create basic patches hashmap
    let patches = crate::core::commands::create_patches_hashmap(&commands);
    
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
            // Cached data available, no disk reads needed
            if verbose {
                println!("⚡ Using cached data - no disk reads needed");
            }
            return data.clone();
        }
        if verbose {
            println!("📂 Loading data from disk...");
        }
    } else {
        crate::utils::detailed_log("LOAD_DATA", &format!("Using provided commands override ({} commands)", commands_override.len()));
        if verbose {
            println!("🔧 Using provided commands override ({} commands)", commands_override.len());
        }
    }
    
    // Step 1: Get the pre-initialized config
    if verbose {
        println!("🔧 Step 1: Using pre-initialized configuration...");
    }
    let config = get_config();
    
    let mut commands = if use_override {
        commands_override
    } else {
        let raw_commands = crate::core::commands::load_commands_raw();
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
    let mut patches = crate::core::commands::create_patches_hashmap(&commands);
    if verbose {
        println!("   Found {} patches from anchor commands", patches.len());
    }
    
    // Step 3.1: REMOVED - No longer creating orphans root
    // Orphan management is no longer needed
    
    // Step 3.2: REMOVED - No longer creating orphan anchors
    // Patches without anchors will simply remain without anchors
    let orphans_created = 0;
    
    // Step 4: Infer patches for commands without patches
    if verbose {
        println!("🧩 Step 4: Running patch inference for commands without patches...");
    }
    let (patches_assigned, new_patches_to_add) = crate::core::commands::run_patch_inference(
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
    
    // Add new patches to hashmap (patch_key is now in original case)
    for patch_name in new_patches_to_add {
        let patch_key = patch_name.to_lowercase(); // Convert to lowercase for hashmap key
        if !patches.contains_key(&patch_key) {
            // Find the first command whose name matches this patch name (case-insensitive)
            let matching_command = commands.iter().find(|cmd| {
                cmd.command.to_lowercase() == patch_key
            });
            
            patches.insert(patch_key, Patch {
                name: patch_name.clone(), // Store original case
                anchor_commands: if let Some(cmd) = matching_command.cloned() { vec![cmd] } else { vec![] },
                include_commands: Vec::new(),
            });
        }
    }
    
    // Step 4.5: Create fast lookup maps
    if verbose {
        println!("🗺️  Step 4.5: Creating command-to-patch lookup map...");
    }
    let command_to_patch = crate::core::commands::create_command_to_patch_map(&commands, &patches);
    if verbose {
        println!("   Created fast lookup map for {} commands", command_to_patch.len());
    }
    
    // Step 4.6: Normalize patch case to match anchor commands
    if verbose {
        println!("🔤 Step 4.6: Normalizing patch case to match anchor commands...");
    }
    let normalized_patches = crate::core::commands::normalize_patch_case(&mut commands, &patches);
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
        // Save commands with changes
        
        if let Err(e) = crate::core::commands::save_commands_to_file(&commands) {
            crate::utils::log_error(&format!("Failed to save commands after changes: {}", e));
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
