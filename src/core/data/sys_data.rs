//! System Data - Internal Implementation
//!
//! This module implements the data layer singleton and provides functions for
//! accessing commands, config, patches, and state. See `mod.rs` for the complete
//! public interface documentation.
//!
//! External code should access this via `crate::core::data` which re-exports
//! the public functions listed in mod.rs.


use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};
use super::config::Config;
use crate::core::{Command, Patch};

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
// Private to this module - external code should use get_config() instead
pub(crate) static CONFIG: OnceLock<Config> = OnceLock::new();

// Global flag to track if commands have been modified and need reload
static COMMANDS_MODIFIED: OnceLock<std::sync::atomic::AtomicBool> = OnceLock::new();

/// Initialize the global config at application startup
/// This is private to sys_data - external code should use get_config() which handles initialization
fn initialize_config() -> Result<(), String> {
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

/// Gets a reference to the sys data from the singleton
/// Panics if initialize() hasn't been called yet
pub fn get_sys_data() -> (SysData, bool) {
    let sys = SYS_DATA.get_or_init(|| Mutex::new(None));
    let sys_data = sys.lock().unwrap();

    match *sys_data {
        Some(ref data) => (data.clone(), false),
        None => panic!("SysData not initialized! Call initialize() at startup"),
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

// ============================================================================
// NEW API - Command State Management
// ============================================================================

/// Initialize the system by loading config and cache into singleton
/// Call this once at application startup
pub fn initialize() -> Result<(), String> {
    // ==========================================================================
    // STEP 1: Verify build consistency
    // ==========================================================================
    // This ensures we're running code built with 'just build' and that
    // the binary matches the source code in the filesystem
    let verification_passed = crate::core::build_verification::verify_and_log();

    if !verification_passed {
        crate::utils::log_error("");
        crate::utils::log_error("❌ ❌ ❌ BUILD VERIFICATION FAILED ❌ ❌ ❌");
        crate::utils::log_error("");
        crate::utils::log_error("   The binary is corrupted or improperly built!");
        crate::utils::log_error("");
        crate::utils::log_error("   STOP! Rebuild immediately with:");
        crate::utils::log_error("   cd ~/ob/proj/HookAnchor && just build");
        crate::utils::log_error("");
        crate::utils::log_error("❌ ❌ ❌ ❌ ❌ ❌ ❌ ❌ ❌ ❌ ❌ ❌ ❌ ❌ ❌");
        crate::utils::log_error("");
    }

    // ==========================================================================
    // STEP 2: Initialize configuration
    // ==========================================================================
    initialize_config()?;

    // ==========================================================================
    // STEP 3: Load commands from cache
    // ==========================================================================
    let commands = match crate::core::commands::load_commands_from_cache() {
        Some(cached_commands) => cached_commands,
        None => Vec::new(), // No cache - start empty, will be populated by rescan
    };

    // Create initial patches hashmap from commands
    let patches = crate::core::commands::create_patches_hashmap(&commands);

    // ==========================================================================
    // STEP 4: Store in singleton
    // ==========================================================================
    let sys = SYS_DATA.get_or_init(|| Mutex::new(None));
    let mut sys_data = sys.lock().unwrap();
    *sys_data = Some(SysData {
        config: get_config(),
        commands,
        patches,
    });

    Ok(())
}

/// Get a copy of commands from the singleton
pub fn get_commands() -> Vec<Command> {
    let (sys_data, _) = get_sys_data();
    sys_data.commands
}

/// Get a copy of patches from the singleton
pub fn get_patches() -> HashMap<String, Patch> {
    let (sys_data, _) = get_sys_data();
    sys_data.patches
}

/// Internal function: Flush commands to disk with validation and repair
/// This ALWAYS validates/repairs patches and saves to both cache and commands.txt
fn flush(commands: &mut Vec<Command>) -> Result<(), Box<dyn std::error::Error>> {
    // Step 1: Validate and repair patches (ensures data integrity)
    let resolution = crate::core::validate_and_repair_patches(commands, false);
    let patches = resolution.patches;

    // Step 2: Save to both cache and commands.txt
    super::storage::save_commands_to_file(commands)?;
    super::storage::save_commands_to_cache(commands)?;

    // Step 3: Update singleton with new commands and patches
    let sys = SYS_DATA.get_or_init(|| Mutex::new(None));
    let mut sys_data = sys.lock().unwrap();
    *sys_data = Some(SysData {
        config: get_config(),
        commands: commands.clone(),
        patches,
    });

    Ok(())
}

/// Replace all commands in singleton, run patch inference, and save to disk
/// This is the primary way to perform batch modifications
/// Records all changes to history by comparing against cached commands
/// Always saves (flushes) to disk regardless of whether inference made changes
pub fn set_commands(mut commands: Vec<Command>) -> Result<(), Box<dyn std::error::Error>> {
    // Initialize history database
    let conn = crate::systems::history::initialize_history_db()?;

    // Get current timestamp
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)?
        .as_secs() as i64;

    // Get cached commands for comparison
    let cached_commands = get_commands();

    // Create lookup maps for efficient comparison
    use std::collections::HashMap;
    let mut cached_map: HashMap<String, &Command> = HashMap::new();
    for cmd in &cached_commands {
        cached_map.insert(cmd.command.clone(), cmd);
    }

    let mut new_map: HashMap<String, &Command> = HashMap::new();
    for cmd in &commands {
        new_map.insert(cmd.command.clone(), cmd);
    }

    // Track changes for logging
    let mut created_count = 0;
    let mut modified_count = 0;
    let mut deleted_count = 0;

    // Find new and modified commands
    for new_cmd in &commands {
        if let Some(cached_cmd) = cached_map.get(&new_cmd.command) {
            // Command exists - check if it was modified
            if new_cmd.action != cached_cmd.action ||
               new_cmd.arg != cached_cmd.arg ||
               new_cmd.patch != cached_cmd.patch ||
               new_cmd.flags != cached_cmd.flags {
                // Command was modified - record to history
                crate::systems::history::record_command_modified(
                    &conn,
                    cached_cmd,
                    new_cmd,
                    timestamp
                )?;
                modified_count += 1;
                crate::utils::detailed_log("HISTORY", &format!(
                    "Modified: '{}' (action: {} -> {}, patch: {} -> {})",
                    new_cmd.command,
                    cached_cmd.action,
                    new_cmd.action,
                    cached_cmd.patch,
                    new_cmd.patch
                ));
            }
        } else {
            // Command is new - record creation to history
            crate::systems::history::record_command_created(&conn, new_cmd, timestamp)?;
            created_count += 1;
            crate::utils::detailed_log("HISTORY", &format!(
                "Created: '{}' (action: {}, patch: {})",
                new_cmd.command,
                new_cmd.action,
                new_cmd.patch
            ));
        }
    }

    // Find deleted commands
    for cached_cmd in &cached_commands {
        if !new_map.contains_key(&cached_cmd.command) {
            // Command was deleted - log it (no history recording function for deletions yet)
            deleted_count += 1;
            crate::utils::detailed_log("HISTORY", &format!(
                "Deleted: '{}' (action: {}, patch: {})",
                cached_cmd.command,
                cached_cmd.action,
                cached_cmd.patch
            ));
        }
    }

    // Log summary of changes
    if created_count > 0 || modified_count > 0 || deleted_count > 0 {
        crate::utils::log(&format!(
            "SET_COMMANDS: Recorded to history - Created: {}, Modified: {}, Deleted: {}",
            created_count, modified_count, deleted_count
        ));
    }

    // Flush to disk with inference
    flush(&mut commands)
}

/// Add a single command, record in history, run inference, and save
/// Convenience function for single-command additions from UI
pub fn add_command(cmd: Command) -> Result<(), Box<dyn std::error::Error>> {
    // Initialize history database
    let conn = crate::systems::history::initialize_history_db()?;

    // Get current timestamp
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)?
        .as_secs() as i64;

    // Record creation in history
    crate::systems::history::record_command_created(&conn, &cmd, timestamp)?;

    // Get current commands and add new one
    let mut commands = get_commands();
    commands.push(cmd);

    // Flush to disk with inference
    flush(&mut commands)?;

    Ok(())
}

/// Delete a command by name, run inference, and save
/// Convenience function for single-command deletions from UI
pub fn delete_command(cmd_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Get current commands and remove the specified one
    let mut commands = get_commands();
    commands.retain(|c| c.command != cmd_name);

    // Flush to disk with inference
    flush(&mut commands)?;

    Ok(())
}

/// Load data without caching - used as fallback when cache is locked
fn load_data_no_cache(commands_override: Vec<Command>, _verbose: bool) -> SysData {
    // Use the pre-initialized config
    let config = get_config();
    
    // Load commands (from disk or use override)
    let commands = if !commands_override.is_empty() {
        commands_override
    } else {
        super::storage::load_commands_raw()
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

    // Step 3: Validate and repair patches (ensures data integrity)
    if verbose {
        println!("🧩 Step 3: Validating and repairing patches...");
    }
    let resolution = crate::core::validate_and_repair_patches(&mut commands, verbose);
    let patches = resolution.patches;
    let changes_made = resolution.changes_made;

    // Step 4: Save commands (flush) - but ONLY if not using override
    // When using override, caller is responsible for saving (don't overwrite commands.txt during temp processing)
    if !use_override {
        if verbose {
            println!("💾 Step 4: Saving to disk...");
        }
        // Always save - ensures deduplication, formatting, consistency
        if let Err(e) = super::storage::save_commands_to_file(&commands) {
            crate::utils::log_error(&format!("Failed to save commands: {}", e));
        } else if let Err(e) = super::storage::save_commands_to_cache(&commands) {
            crate::utils::log_error(&format!("Failed to save cache: {}", e));
        } else {
            if verbose {
                if changes_made {
                    println!("   ✅ Saved {} commands with {} inference changes", commands.len(),
                        if changes_made { "patch" } else { "no" });
                } else {
                    println!("   ✅ Saved {} commands (no inference changes needed)", commands.len());
                }
            }
            // Don't clear cache here - we're already updating it
            // clear_sys_data() would cause deadlock since we're holding the mutex
        }
    } else if verbose {
        println!("💾 Step 4: Skipping save (using commands override for temporary processing)");
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

// =============================================================================
// STATE MANAGEMENT
// =============================================================================

/// Get application state
pub fn get_state() -> super::state::AppState {
    super::state::load_state()
}

/// Save application state
pub fn set_state(state: &super::state::AppState) -> Result<(), Box<dyn std::error::Error>> {
    super::state::save_state(state)
}
