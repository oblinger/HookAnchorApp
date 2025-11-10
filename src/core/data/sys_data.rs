//! System Data - Internal Implementation
//!
//! This module implements the data layer singleton and provides functions for
//! accessing commands, config, patches, and state. See `mod.rs` for the complete
//! public interface documentation.
//!
//! External code should access this via `crate::core::data` which re-exports
//! the public functions listed in mod.rs.


use std::collections::HashMap;
use std::sync::{Arc, Mutex, OnceLock};
use super::config::Config;
use crate::core::{Command, Patch};

/// System application data structure containing all loaded data
/// Note: commands field is Arc<Vec<Command>> - use &*sys_data.commands to get &[Command]
#[derive(Clone, Debug)]
pub struct SysData {
    pub config: Config,
    pub commands: Arc<Vec<Command>>,  // Arc for efficient read-only access (cloning is cheap)
    pub patches: HashMap<String, Patch>,
}

// Global application data - loaded once and reused
static SYS_DATA: OnceLock<Mutex<Option<SysData>>> = OnceLock::new();

// Global config - loaded once at startup
// Private to this module - external code should use get_config() instead
pub(crate) static CONFIG: OnceLock<Config> = OnceLock::new();

// Global flag to track if commands have been modified and need reload
static COMMANDS_MODIFIED: OnceLock<std::sync::atomic::AtomicBool> = OnceLock::new();

/// Initialize minimal sys_data with empty commands for GUI startup
/// This prevents panics when UI tries to access data before it's loaded
pub fn initialize_minimal() -> Result<(), String> {
    // Initialize config first
    initialize_config()?;

    // Create minimal SysData with empty commands
    let sys = SYS_DATA.get_or_init(|| Mutex::new(None));
    let mut sys_data = sys.lock().unwrap();
    *sys_data = Some(SysData {
        config: get_config(),
        commands: Arc::new(Vec::new()),
        patches: std::collections::HashMap::new(),
    });

    crate::utils::log("SYS_DATA: Minimal initialization complete (empty commands)");
    Ok(())
}

/// Initialize the global config at application startup
/// This can be called multiple times safely - only the first call does the work
pub fn initialize_config() -> Result<(), String> {
    // If already initialized, return immediately
    if CONFIG.get().is_some() {
        return Ok(());
    }

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

        crate::utils::log("\n🚀 First-time setup detected - initializing HookAnchor configuration...");
        crate::utils::log("   Missing files will be created:");
        if !config_yaml_exists {
            crate::utils::log("   • config.yaml (application configuration)");
        }
        if !commands_txt_exists {
            crate::utils::log("   • commands.txt (command definitions)");
        }
        if !config_js_exists {
            crate::utils::log("   • config.js (JavaScript functions)");
        }

        // Launch GUI installer for automatic installation
        crate::utils::log("   Launching GUI installer...");

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
                    crate::utils::log("   GUI installer launched successfully");
                    crate::utils::log("   Please complete the setup and restart HookAnchor");
                    std::process::exit(0); // Exit to let user complete setup
                },
                Err(e) => {
                    crate::utils::log_error(&format!("\n⚠️  Failed to launch GUI installer: {}", e));
                    crate::utils::log_error("   You can run setup manually with: ha --install");
                }
            }
        } else {
            crate::utils::log_error(&format!("\n⚠️  GUI installer not found at: {}", installer_path.display()));
            crate::utils::log_error("   You can run setup manually with: ha --install");
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

/// Gets an Arc reference to the commands list (internal, fast access)
/// This is very fast - just clones the Arc pointer (~5ns), not the data
/// Use this for hot paths that only read commands
/// Panics if initialize() hasn't been called yet
pub fn get_commands_arc() -> Arc<Vec<Command>> {
    let sys = SYS_DATA.get_or_init(|| Mutex::new(None));
    let sys_data = sys.lock().unwrap();

    match *sys_data {
        Some(ref data) => Arc::clone(&data.commands),
        None => panic!("SysData not initialized! Call initialize() at startup"),
    }
}

/// Get commands as a Vec - standard access for general use
/// Clones from internal Arc
pub fn get_commands() -> Vec<Command> {
    let commands_arc = get_commands_arc();
    (*commands_arc).clone()
}

/// Updates the commands in the singleton with a new list
/// Creates a new Arc, invalidating old snapshots (old Arc stays alive until last reference drops)
/// Also updates the patches hashmap to stay in sync
/// Call this after flushing commands to disk
pub fn update_commands(new_commands: Vec<Command>) {
    let sys = SYS_DATA.get().expect("SysData not initialized! Call initialize() at startup");
    let mut sys_data = sys.lock().unwrap();

    if let Some(ref mut data) = *sys_data {
        // Create new Arc with updated commands
        data.commands = Arc::new(new_commands.clone());

        // Update patches to stay in sync
        data.patches = crate::core::commands::create_patches_hashmap(&new_commands);

        crate::utils::log("SYS_DATA: Commands updated, old Arc invalidated");
    }
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
    // STEP 1: Initialize configuration (idempotent - safe to call multiple times)
    // ==========================================================================
    initialize_config()?;

    // ==========================================================================
    // STEP 2: Verify build consistency
    // ==========================================================================
    // This ensures we're running code built with 'just build' and that
    // the binary matches the source code in the filesystem.
    // If verification fails, this will show a dialog and terminate the app.
    // NOTE: This must come AFTER config initialization because logging needs config.
    crate::utils::verify_build(true);

    // ==========================================================================
    // STEP 2.5: Verify config version compatibility
    // ==========================================================================
    // Check that config.yaml version is compatible with this build
    // If config is too old, show error dialog and terminate
    // NOTE: This check happens for ALL machines (not just dev machines)
    crate::utils::verify_config_version_or_exit();

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
        commands: Arc::new(commands),  // Wrap in Arc for efficient access
        patches,
    });

    Ok(())
}


/// Get a copy of patches from the singleton
pub fn get_patches() -> HashMap<String, Patch> {
    let (sys_data, _) = get_sys_data();
    sys_data.patches
}

/// Internal function: Flush commands to disk with validation and repair
/// This ALWAYS validates/repairs patches and saves to both cache and commands.txt
fn flush(commands: &mut Vec<Command>) -> Result<(), Box<dyn std::error::Error>> {
    let flush_start = std::time::Instant::now();
    let initial_count = commands.len();
    crate::utils::log(&format!("⏱️ FLUSH: Starting with {} commands", initial_count));

    // Step 1: Validate and repair patches (ensures data integrity)
    let step1_start = std::time::Instant::now();
    let resolution = crate::core::validate_and_repair_patches(commands, true);
    let patches = resolution.patches;
    crate::utils::log(&format!("⏱️ FLUSH: Step 1 (validate/repair): {:?}", step1_start.elapsed()));

    let after_validation_count = commands.len();
    if after_validation_count != initial_count {
        crate::utils::log(&format!("FLUSH: Validation added/removed {} commands (now {})",
            after_validation_count as i32 - initial_count as i32, after_validation_count));
    }

    // Step 2: Deduplicate commands (keeps best version of each unique command name)
    let step2_start = std::time::Instant::now();
    *commands = super::storage::deduplicate_commands(commands.clone());
    crate::utils::log(&format!("⏱️ FLUSH: Step 2 (deduplicate): {:?}", step2_start.elapsed()));

    let after_dedup_count = commands.len();
    if after_dedup_count != after_validation_count {
        crate::utils::log(&format!("FLUSH: Deduplication removed {} duplicate commands (now {})",
            after_validation_count - after_dedup_count, after_dedup_count));
    }

    // Step 3: Save to both cache and commands.txt
    let step3_start = std::time::Instant::now();
    super::storage::save_commands_to_file(commands)?;
    super::storage::save_commands_to_cache(commands)?;
    crate::utils::log(&format!("⏱️ FLUSH: Step 3 (save to disk): {:?}", step3_start.elapsed()));

    // Step 4: Update singleton with new commands (creates new Arc, invalidates old snapshots)
    let step4_start = std::time::Instant::now();
    update_commands(commands.clone());
    crate::utils::log(&format!("⏱️ FLUSH: Step 4 (update singleton): {:?}", step4_start.elapsed()));

    crate::utils::log(&format!("⏱️ FLUSH: TOTAL TIME: {:?}", flush_start.elapsed()));
    Ok(())
}

/// Replace all commands in singleton, run patch inference, and save to disk
/// This is the primary way to perform batch modifications
/// Records all changes to history by comparing against cached commands
/// Always saves (flushes) to disk regardless of whether inference made changes
pub fn set_commands(mut commands: Vec<Command>) -> Result<(), Box<dyn std::error::Error>> {
    let set_commands_start = std::time::Instant::now();
    crate::utils::log(&format!("⏱️ SET_COMMANDS: Starting with {} commands", commands.len()));

    // Initialize history database
    let history_init_start = std::time::Instant::now();
    let conn = super::history::initialize_history_db()?;
    crate::utils::log(&format!("⏱️ SET_COMMANDS: History DB init: {:?}", history_init_start.elapsed()));

    // Get current timestamp
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)?
        .as_secs() as i64;

    // Get cached commands for comparison
    let get_cached_start = std::time::Instant::now();
    let cached_commands = get_commands();
    crate::utils::log(&format!("⏱️ SET_COMMANDS: Get cached commands: {:?}", get_cached_start.elapsed()));

    // Create lookup maps for efficient comparison using dedup keys
    // This ensures we compare commands the same way the system deduplicates them
    let map_start = std::time::Instant::now();
    let cached_map = super::storage::build_command_map(&cached_commands);
    let new_map = super::storage::build_command_map(&commands);
    crate::utils::log(&format!("⏱️ SET_COMMANDS: Create lookup maps: {:?}", map_start.elapsed()));

    // Track changes for logging
    let mut created_count = 0;
    let mut modified_count = 0;
    let mut deleted_count = 0;

    // Record all new and modified commands to history
    let history_record_start = std::time::Instant::now();
    for new_cmd in &commands {
        let new_key = super::storage::command_dedup_key(new_cmd);
        if let Some(cached_cmd) = cached_map.get(&new_key) {
            // Command exists - check if it was modified
            if new_cmd.action != cached_cmd.action ||
               new_cmd.arg != cached_cmd.arg ||
               new_cmd.patch != cached_cmd.patch ||
               new_cmd.flags != cached_cmd.flags {
                // Command was modified - append to history
                super::history::append_command(&conn, new_cmd, timestamp)?;
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
            // Command is new - append to history
            super::history::append_command(&conn, new_cmd, timestamp)?;
            created_count += 1;
            crate::utils::detailed_log("HISTORY", &format!(
                "Created: '{}' (action: {}, patch: {})",
                new_cmd.command,
                new_cmd.action,
                new_cmd.patch
            ));
        }
    }

    // Find deleted commands and record them with special action
    for cached_cmd in &cached_commands {
        let cached_key = super::storage::command_dedup_key(cached_cmd);
        if !new_map.contains_key(&cached_key) {
            // Command was deleted - create deletion entry
            let mut deleted_cmd = cached_cmd.clone();
            deleted_cmd.action = "$DELETED$".to_string();
            super::history::append_command(&conn, &deleted_cmd, timestamp)?;
            deleted_count += 1;
            crate::utils::detailed_log("HISTORY", &format!(
                "Deleted: '{}' (was action: {}, patch: {})",
                cached_cmd.command,
                cached_cmd.action,
                cached_cmd.patch
            ));
        }
    }

    crate::utils::log(&format!("⏱️ SET_COMMANDS: Record history: {:?}", history_record_start.elapsed()));

    // Log summary of changes
    if created_count > 0 || modified_count > 0 || deleted_count > 0 {
        crate::utils::log(&format!(
            "SET_COMMANDS: Recorded to history - Created: {}, Modified: {}, Deleted: {}",
            created_count, modified_count, deleted_count
        ));
    }

    // Flush to disk with inference
    let flush_start = std::time::Instant::now();
    let result = flush(&mut commands);
    crate::utils::log(&format!("⏱️ SET_COMMANDS: Flush (includes inference): {:?}", flush_start.elapsed()));
    crate::utils::log(&format!("⏱️ SET_COMMANDS: TOTAL TIME: {:?}", set_commands_start.elapsed()));

    result
}

/// Add a single command, record in history, run inference, and save
/// Convenience function for single-command additions from UI
pub fn add_command(cmd: Command) -> Result<(), Box<dyn std::error::Error>> {
    // Get current commands and add new one
    let mut commands = get_commands();
    commands.push(cmd);

    // Use set_commands which handles history recording automatically
    set_commands(commands)?;

    Ok(())
}

/// Delete a command by name, record deletion in history, run inference, and save
/// Convenience function for single-command deletions from UI
pub fn delete_command(cmd_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Get current commands and remove the specified one
    let mut commands = get_commands();
    commands.retain(|c| c.command != cmd_name);

    // Use set_commands which handles history recording (including deletion)
    set_commands(commands)?;

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
        commands: Arc::new(commands),
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
                crate::utils::log("⚡ Using cached data - no disk reads needed");
            }
            return data.clone();
        }
        if verbose {
            crate::utils::log("📂 Loading data from disk...");
        }
    } else {
        crate::utils::detailed_log("LOAD_DATA", &format!("Using provided commands override ({} commands)", commands_override.len()));
        if verbose {
            crate::utils::log(&format!("🔧 Using provided commands override ({} commands)", commands_override.len()));
        }
    }

    // Step 1: Get the pre-initialized config
    if verbose {
        crate::utils::log("🔧 Step 1: Using pre-initialized configuration...");
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
                    crate::utils::log(&format!("📋 Step 2: Loaded {} commands from cache", cached_commands.len()));
                }
                cached_commands
            }
            None => {
                if verbose {
                    crate::utils::log("📋 Step 2: No cache found - starting with empty commands");
                    crate::utils::log("            Run --rescan to rebuild from filesystem");
                }
                Vec::new()
            }
        }
    };

    // Step 2.5:

    // Step 3: Validate and repair patches (ensures data integrity)
    if verbose {
        crate::utils::log("🧩 Step 3: Validating and repairing patches...");
    }
    let resolution = crate::core::validate_and_repair_patches(&mut commands, verbose);
    let patches = resolution.patches;
    let changes_made = resolution.changes_made;

    // Step 4: Save commands (flush) - but ONLY if not using override
    // When using override, caller is responsible for saving (don't overwrite commands.txt during temp processing)
    if !use_override {
        if verbose {
            crate::utils::log("💾 Step 4: Saving to disk...");
        }
        // Always save - ensures deduplication, formatting, consistency
        if let Err(e) = super::storage::save_commands_to_file(&commands) {
            crate::utils::log_error(&format!("Failed to save commands: {}", e));
        } else if let Err(e) = super::storage::save_commands_to_cache(&commands) {
            crate::utils::log_error(&format!("Failed to save cache: {}", e));
        } else {
            if verbose {
                if changes_made {
                    crate::utils::log(&format!("   ✅ Saved {} commands with {} inference changes", commands.len(),
                        if changes_made { "patch" } else { "no" }));
                } else {
                    crate::utils::log(&format!("   ✅ Saved {} commands (no inference changes needed)", commands.len()));
                }
            }
            // Don't clear cache here - we're already updating it
            // clear_sys_data() would cause deadlock since we're holding the mutex
        }
    } else if verbose {
        crate::utils::log("💾 Step 4: Skipping save (using commands override for temporary processing)");
    }
    
    // Store in sys data for future calls (only if not using commands override)
    let sys_data_struct = SysData {
        config,
        commands: Arc::new(commands),  // Wrap in Arc for efficient access
        patches,
    };
    
    if !use_override {
        *sys_data = Some(sys_data_struct.clone());
    }

    if verbose {
        crate::utils::log("\n✅ Data loading complete!");
        crate::utils::log(&format!("   Total commands: {}", sys_data_struct.commands.len()));
        crate::utils::log(&format!("   Total patches: {}", sys_data_struct.patches.len()));
        let commands_with_patches = sys_data_struct.commands.iter()
            .filter(|c| !c.patch.is_empty())
            .count();
        crate::utils::log(&format!("   Commands with patches: {}", commands_with_patches));
        crate::utils::log(&format!("   Commands without patches: {}", sys_data_struct.commands.len() - commands_with_patches));
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

// =============================================================================
// HISTORY MANAGEMENT
// =============================================================================

/// Get history entries (trampoline to history module)
///
/// # Arguments
/// * `limit` - Maximum number of entries to return
/// * `exclude_deletions` - If true, filters out entries with action="$DELETED$"
pub fn get_history_entries(limit: usize, exclude_deletions: bool) -> rusqlite::Result<Vec<super::history::HistoryEntry>> {
    super::history::get_history_entries(limit, exclude_deletions)
}

/// Delete history -- This function coordinates a complete reset of the history and cache system 
///
/// # Returns
/// * `Ok((history_deleted, cache_deleted))` - Tuple of booleans indicating what was deleted
/// * `Err(String)` - Error message if deletion fails
pub fn delete_history() -> Result<(bool, bool), String> {
    let history_deleted = super::history::delete_history_db()?;
    let cache_deleted = super::storage::delete_cache()?;
    Ok((history_deleted, cache_deleted))
}
