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

// Global flag to track if commands have been modified and need reload
static COMMANDS_MODIFIED: OnceLock<std::sync::atomic::AtomicBool> = OnceLock::new();

/// Initialize the global config at application startup
/// This MUST be called before any other operations
pub fn initialize_config() -> Result<(), String> {
    let start = std::time::Instant::now();

    // Check if essential config files exist - if not, run the installer
    let config_dir = dirs::home_dir()
        .ok_or_else(|| "Could not find home directory".to_string())?
        .join(".config")
        .join("hookanchor");

    let config_yaml_exists = config_dir.join("config.yaml").exists();
    let commands_txt_exists = config_dir.join("commands.txt").exists();
    let config_js_exists = config_dir.join("config.js").exists();

    // If the essential config files created by the installer are missing, run first-time setup
    if !config_yaml_exists || !commands_txt_exists || !config_js_exists {
        crate::utils::log(&format!(
            "CONFIG_INIT: Missing config files - running first-time setup (yaml:{}, txt:{}, js:{})",
            config_yaml_exists, commands_txt_exists, config_js_exists
        ));

        println!("\n🚀 First-time setup detected - initializing HookAnchor configuration...");
        println!("   Missing files will be created:");
        if !config_yaml_exists {
            println!("   • config.yaml (application configuration)");
        }
        if !commands_txt_exists {
            println!("   • commands.txt (command definitions)");
        }
        if !config_js_exists {
            println!("   • config.js (JavaScript functions)");
        }

        // Launch GUI installer for automatic installation
        println!("   Launching GUI installer...");

        // Find the installer_gui binary in the same directory as the current executable
        let current_exe = std::env::current_exe().unwrap_or_else(|_| std::path::PathBuf::from("ha"));

        // Resolve symlinks to get the actual binary location
        let resolved_exe = std::fs::canonicalize(&current_exe).unwrap_or(current_exe);
        let exe_dir = resolved_exe.parent().unwrap_or_else(|| std::path::Path::new("."));
        let installer_path = exe_dir.join("installer_gui");

        if installer_path.exists() {
            match std::process::Command::new(&installer_path)
                .spawn() {
                Ok(_) => {
                    println!("   GUI installer launched successfully");
                    println!("   Please complete the setup and restart HookAnchor");
                    std::process::exit(0); // Exit to let user complete setup
                },
                Err(e) => {
                    eprintln!("\n⚠️  Failed to launch GUI installer: {}", e);
                    eprintln!("   You can run setup manually with: ha --install");
                }
            }
        } else {
            eprintln!("\n⚠️  GUI installer not found at: {}", installer_path.display());
            eprintln!("   You can run setup manually with: ha --install");
        }
    }

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
/// Returns (data, was_reloaded) where was_reloaded indicates if data was refreshed
pub fn get_sys_data() -> (SysData, bool) {
    // Check if commands have been modified and need reload
    let flag = COMMANDS_MODIFIED.get_or_init(|| std::sync::atomic::AtomicBool::new(false));
    let commands_dirty = flag.swap(false, std::sync::atomic::Ordering::Relaxed);
    
    let sys = SYS_DATA.get_or_init(|| Mutex::new(None));
    
    // Try to acquire the lock without blocking to detect potential deadlock
    match sys.try_lock() {
        Ok(mut sys_data) => {
            // If commands are dirty, clear cache to force reload
            let was_cleared = if commands_dirty {
                crate::utils::detailed_log("GET_SYS_DATA", "Commands modified - clearing cache for reload");
                *sys_data = None;
                true
            } else {
                false
            };
            
            if let Some(ref data) = *sys_data {
                // Cached data found, return it
                return (data.clone(), false);
            }
            // Not cached or cleared due to modifications, need to load
            drop(sys_data); // Release the lock before calling load_data
            (load_data(Vec::new(), false), was_cleared)
        }
        Err(_) => {
            // Mutex is locked, this indicates a potential deadlock
            crate::utils::detailed_log("GET_SYS_DATA", "ERROR: Mutex is locked, potential deadlock detected");
            // Return a fallback - load without caching (assume this is a reload)
            (load_data_no_cache(Vec::new(), false), true)
        }
    }
}

/// Gets just the config for functions that only need configuration
/// Panics if initialize_config() hasn't been called yet
pub fn get_config() -> Config {
    CONFIG.get().expect("Config not initialized! Call initialize_config() at startup").clone()
}

/// Mark that commands have been modified and need to be reloaded
/// This is the standard way to indicate that command data has changed
/// The next call to get_sys_data() will automatically reload
pub fn mark_commands_modified() {
    let flag = COMMANDS_MODIFIED.get_or_init(|| std::sync::atomic::AtomicBool::new(false));
    flag.store(true, std::sync::atomic::Ordering::Relaxed);
    crate::utils::detailed_log("COMMANDS_RELOAD", "Commands marked as modified - will reload on next get_sys_data() call");
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
/// 4. Create virtual anchors if needed
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
        // Load ONLY from cache - commands.txt is only used during manual rebuild
        // If cache doesn't exist, system starts with empty commands
        match crate::core::commands::load_commands_from_cache() {
            Some(cached_commands) => {
                if verbose {
                    println!("📋 Step 2: Loaded {} commands from cache", cached_commands.len());
                }
                cached_commands
            }
            None => {
                if verbose {
                    println!("📋 Step 2: No cache found - starting with empty commands");
                    println!("            Run --rescan to rebuild from filesystem");
                }
                Vec::new()
            }
        }
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
    
    // Step 3.1: REMOVED - No longer creating physical anchor root
    // Virtual anchor management is no longer needed
    
    // Step 3.2: Extract virtual anchor commands from patches and add them to commands list
    let mut virtual_anchors_created = 0;
    for patch in patches.values() {
        for anchor_cmd in &patch.anchor_commands {
            if anchor_cmd.patch == "orphans" && anchor_cmd.action == "anchor" && !anchor_cmd.flags.contains('U') {
                if verbose {
                    println!("   Adding virtual anchor command: {}", anchor_cmd.command);
                }
                commands.push(anchor_cmd.clone());
                virtual_anchors_created += 1;
            }
        }
    }
    
    if verbose && virtual_anchors_created > 0 {
        println!("   Created {} virtual anchor commands", virtual_anchors_created);
    }
    
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
                history_file: None,
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
    if patches_assigned > 0 || virtual_anchors_created > 0 || normalized_patches > 0 {
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
