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
use crate::prelude::*;

/// System application data structure containing all loaded data
/// Note: commands field is Arc<Vec<Command>> - use &*sys_data.commands to get &[Command]
#[derive(Clone, Debug)]
pub struct SysData {
    pub config: Config,
    pub commands: Arc<Vec<Command>>,  // Arc for efficient read-only access (cloning is cheap)
    pub patches: HashMap<String, Patch>,
    folder_to_patch: HashMap<std::path::PathBuf, String>,  // Maps folder paths to patch names
}

// Global application data - loaded once and reused
static SYS_DATA: OnceLock<Mutex<Option<SysData>>> = OnceLock::new();

// Global config - loaded once at startup
// Private to this module - external code should use get_config() instead
pub(crate) static CONFIG: OnceLock<Config> = OnceLock::new();

// Track the modification time of commands.txt when we last loaded it
// Used to auto-reload when the file changes (e.g., from server process updates)
static LAST_COMMANDS_MTIME: OnceLock<Mutex<Option<std::time::SystemTime>>> = OnceLock::new();

/// Update the stored mtime of commands.txt to current file mtime
/// Called after loading commands to prevent immediate re-reload
fn update_commands_mtime() {
    let commands_path = super::storage::get_commands_file_path();
    if let Ok(metadata) = std::fs::metadata(&commands_path) {
        if let Ok(mtime) = metadata.modified() {
            let mtime_lock = LAST_COMMANDS_MTIME.get_or_init(|| Mutex::new(None));
            let mut stored = mtime_lock.lock().unwrap();
            *stored = Some(mtime);
        }
    }
}

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
        folder_to_patch: std::collections::HashMap::new(),
    });

    log("SYS_DATA: Minimal initialization complete (empty commands)");
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
        log(&format!(
            "CONFIG_INIT: Missing config files - running first-time setup (yaml:{}, txt:{}, js:{})",
            config_yaml_exists, commands_txt_exists, config_js_exists
        ));

        log("\n🚀 First-time setup detected - initializing HookAnchor configuration...");
        log("   Missing files will be created:");
        if !config_yaml_exists {
            log("   • config.yaml (application configuration)");
        }
        if !commands_txt_exists {
            log("   • commands.txt (command definitions)");
        }
        if !config_js_exists {
            log("   • config.js (JavaScript functions)");
        }

        // Launch GUI installer for automatic installation
        log("   Launching GUI installer...");

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
                    log("   GUI installer launched successfully");
                    log("   Please complete the setup and restart HookAnchor");
                    std::process::exit(0); // Exit to let user complete setup
                },
                Err(e) => {
                    log_error(&format!("\n⚠️  Failed to launch GUI installer: {}", e));
                    log_error("   You can run setup manually with: ha --install");
                }
            }
        } else {
            log_error(&format!("\n⚠️  GUI installer not found at: {}", installer_path.display()));
            log_error("   You can run setup manually with: ha --install");
        }
    }

    // Load config using the existing load_config_with_error for proper error handling
    match super::config::load_config_with_error() {
        super::config::ConfigResult::Success(config) => {
            CONFIG.set(config).map_err(|_| "Config already initialized".to_string())?;
            let elapsed = start.elapsed();
            log(&format!("CONFIG_INIT: Config initialized at startup in {:?} ({} microseconds)", elapsed, elapsed.as_micros()));
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
/// Automatically reloads from disk if commands.txt has been modified since last load
/// Returns (SysData, was_reloaded) - was_reloaded is true if data was refreshed from disk
/// Panics if initialize() hasn't been called yet
pub fn get_sys_data() -> (SysData, bool) {
    // Check if commands.txt has been modified since we last loaded
    let commands_path = super::storage::get_commands_file_path();
    let mut was_reloaded = false;

    if let Ok(metadata) = std::fs::metadata(&commands_path) {
        if let Ok(current_mtime) = metadata.modified() {
            let mtime_lock = LAST_COMMANDS_MTIME.get_or_init(|| Mutex::new(None));
            let stored_mtime = mtime_lock.lock().unwrap().clone();

            if let Some(prev_mtime) = stored_mtime {
                if current_mtime > prev_mtime {
                    // Commands file changed since we last loaded - reload from disk
                    detailed_log("SYS_DATA", "Commands.txt modified - auto-reloading from disk");
                    drop(stored_mtime); // Release any references

                    // Reload commands from disk
                    let commands = super::storage::load_commands_raw();
                    update_commands(commands);

                    // Update stored mtime
                    let mut mtime = mtime_lock.lock().unwrap();
                    *mtime = Some(current_mtime);

                    was_reloaded = true;
                    log(&format!("SYS_DATA: Auto-reloaded {} commands from disk",
                        get_commands_arc().len()));
                }
            }
        }
    }

    let sys = SYS_DATA.get_or_init(|| Mutex::new(None));
    let sys_data = sys.lock().unwrap();

    match *sys_data {
        Some(ref data) => (data.clone(), was_reloaded),
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

        // Sync anchor tree folders to match patch hierarchy
        if let Err(e) = crate::systems::scanner::update_anchor_tree_folders(&data.patches, &data.config) {
            crate::prelude::log_error(&format!("Failed to sync anchor tree folders: {}", e));
        }

        log("SYS_DATA: Commands updated, old Arc invalidated");
    }
}

/// Clear all commands from singleton and delete both commands.txt and cache files
/// Used during delete-history to start completely fresh
pub fn clear_commands() -> Result<(), Box<dyn std::error::Error>> {
    log("CLEAR_COMMANDS: Clearing singleton and deleting files");

    // Update singleton to empty
    update_commands(Vec::new());

    // Delete commands.txt
    let commands_path = super::storage::get_commands_file_path();
    if commands_path.exists() {
        std::fs::remove_file(&commands_path)?;
        log(&format!("CLEAR_COMMANDS: Deleted commands.txt at {:?}", commands_path));
    }

    // Delete cache
    let cache_path = super::storage::get_commands_cache_path();
    if cache_path.exists() {
        std::fs::remove_file(&cache_path)?;
        log(&format!("CLEAR_COMMANDS: Deleted cache at {:?}", cache_path));
    }

    Ok(())
}

/// Reload commands from disk into singleton
/// Reads commands.txt (via load_commands_raw), updates singleton, does NOT save back to disk
/// Used after manually restoring commands.txt from backup
pub fn reload_commands() -> Result<Vec<Command>, Box<dyn std::error::Error>> {
    log("RELOAD_COMMANDS: Loading commands from commands.txt into singleton");

    // Load from commands.txt
    let commands = super::storage::load_commands_raw();
    log(&format!("RELOAD_COMMANDS: Loaded {} commands from commands.txt", commands.len()));

    // Update singleton (but don't save back to disk - we just loaded from there!)
    update_commands(commands.clone());

    // Update the stored mtime so we don't reload again until next change
    update_commands_mtime();

    log("RELOAD_COMMANDS: Updated singleton with loaded commands");

    Ok(commands)
}

/// Create timestamped backup of all command data
/// Backs up commands.txt, commands_cache.json, and history.db (if they exist)
/// Returns the backup directory path for display to the user
pub fn backup_commands() -> Result<std::path::PathBuf, Box<dyn std::error::Error>> {
    use chrono::Local;

    log("BACKUP_COMMANDS: Creating emergency backup");

    // Get paths
    let commands_path = super::storage::get_commands_file_path();
    let cache_path = super::storage::get_commands_cache_path();
    let history_path = super::history::get_history_db_path();

    let config_dir = commands_path.parent()
        .ok_or("Could not get config directory")?;
    let backups_dir = config_dir.join("backups");

    // Create backups directory
    std::fs::create_dir_all(&backups_dir)
        .map_err(|e| format!("Failed to create backups directory: {}", e))?;

    // Create timestamped backup directory
    let timestamp = Local::now().format("%Y%m%d_%H%M%S");
    let backup_dir = backups_dir.join(format!("emergency_restore_{}", timestamp));
    std::fs::create_dir_all(&backup_dir)
        .map_err(|e| format!("Failed to create backup directory: {}", e))?;

    // Backup commands.txt (if exists)
    if commands_path.exists() {
        std::fs::copy(&commands_path, backup_dir.join("commands.txt"))
            .map_err(|e| format!("Failed to backup commands.txt: {}", e))?;
        log(&format!("BACKUP_COMMANDS: Backed up commands.txt"));
    } else {
        // Create marker file to show it didn't exist
        std::fs::write(backup_dir.join("commands.txt.missing"), "commands.txt did not exist at backup time")
            .map_err(|e| format!("Failed to create backup marker: {}", e))?;
        log(&format!("BACKUP_COMMANDS: commands.txt did not exist"));
    }

    // Backup cache (if exists)
    if cache_path.exists() {
        let _ = std::fs::copy(&cache_path, backup_dir.join("commands_cache.json"));
        log(&format!("BACKUP_COMMANDS: Backed up cache"));
    }

    // Backup history (if exists)
    if history_path.exists() {
        let _ = std::fs::copy(&history_path, backup_dir.join("history.db"));
        log(&format!("BACKUP_COMMANDS: Backed up history.db"));
    }

    log(&format!("BACKUP_COMMANDS: Backup complete at {:?}", backup_dir));
    Ok(backup_dir)
}

/// Restore commands from an external file path
/// Physically copies the file to commands.txt, then reloads singleton
/// Returns the number of commands loaded
pub fn restore_commands_from_file(source_path: &std::path::Path) -> Result<usize, Box<dyn std::error::Error>> {
    log(&format!("RESTORE_COMMANDS: Restoring from {:?}", source_path));

    // Validate source file exists
    if !source_path.exists() {
        return Err(format!("Source file does not exist: {}", source_path.display()).into());
    }

    // Get destination path
    let dest_path = super::storage::get_commands_file_path();

    // Copy the file
    let bytes_copied = std::fs::copy(source_path, &dest_path)?;
    log(&format!("RESTORE_COMMANDS: Copied {} bytes to commands.txt", bytes_copied));

    // Reload singleton from the restored file
    let commands = reload_commands()?;
    let count = commands.len();

    log(&format!("RESTORE_COMMANDS: Restored {} commands", count));
    Ok(count)
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
    // Skip build verification during unit tests (tests use `cargo test` not `just build`)
    #[cfg(not(test))]
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
    let commands = match super::storage::load_commands_from_cache() {
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
    // Build folder_to_patch map from anchor commands
    let folder_to_patch = build_folder_to_patch_map(&commands, &get_config());

    *sys_data = Some(SysData {
        config: get_config(),
        commands: Arc::new(commands),  // Wrap in Arc for efficient access
        patches,
        folder_to_patch,
    });

    // Release lock before updating mtime
    drop(sys_data);

    // Set initial mtime so get_sys_data() knows when we last loaded
    update_commands_mtime();

    Ok(())
}


/// Get a copy of patches from the singleton
pub fn get_patches() -> HashMap<String, Patch> {
    let (sys_data, _) = get_sys_data();
    sys_data.patches
}

/// Internal function: Flush commands to disk with validation and repair
/// This ALWAYS validates/repairs patches and saves to both cache and commands.txt
fn flush(commands: &mut Vec<Command>, skip_validation: bool) -> Result<(), Box<dyn std::error::Error>> {
    let flush_start = std::time::Instant::now();
    let initial_count = commands.len();
    log(&format!("⏱️ FLUSH: Starting with {} commands", initial_count));

    // Step 1: Validate and repair patches (ensures data integrity)
    // Can be skipped for small, non-structural changes (e.g., single command edits from UI)
    let after_validation_count = if skip_validation {
        log("⏩ FLUSH: Step 1 (validate/repair): SKIPPED (no structural changes)");
        initial_count
    } else {
        let step1_start = std::time::Instant::now();
        let resolution = crate::core::validate_and_repair_patches(commands, true);
        let patches = resolution.patches;
        log(&format!("⏱️ FLUSH: Step 1 (validate/repair): {:?}", step1_start.elapsed()));

        let after_validation_count = commands.len();
        if after_validation_count != initial_count {
            log(&format!("FLUSH: Validation added/removed {} commands (now {})",
                after_validation_count as i32 - initial_count as i32, after_validation_count));
        }
        after_validation_count
    };

    // Step 2: Deduplicate commands (keeps best version of each unique command name)
    let step2_start = std::time::Instant::now();
    *commands = super::storage::deduplicate_commands(commands.clone());
    log(&format!("⏱️ FLUSH: Step 2 (deduplicate): {:?}", step2_start.elapsed()));

    let after_dedup_count = commands.len();
    if after_dedup_count != after_validation_count {
        log(&format!("FLUSH: Deduplication removed {} duplicate commands (now {})",
            after_validation_count - after_dedup_count, after_dedup_count));
    }

    // Step 3: Save to both cache and commands.txt
    let step3_start = std::time::Instant::now();
    super::storage::save_commands_to_file(commands)?;
    super::storage::save_commands_to_cache(commands)?;
    log(&format!("⏱️ FLUSH: Step 3 (save to disk): {:?}", step3_start.elapsed()));

    // Step 4: Update singleton with new commands (creates new Arc, invalidates old snapshots)
    let step4_start = std::time::Instant::now();
    update_commands(commands.clone());
    log(&format!("⏱️ FLUSH: Step 4 (update singleton): {:?}", step4_start.elapsed()));

    log(&format!("⏱️ FLUSH: TOTAL TIME: {:?}", flush_start.elapsed()));
    Ok(())
}

/// Process a single command change (create, update, or delete)
///
/// This function handles all the side effects of a command change:
/// - Records the change to history
/// - Validates the command if it's structural (anchors, patches, flags)
/// - Returns whether the change is structural (used to decide if full validation is needed)
///
/// # Arguments
/// * `old_cmd` - The previous version of the command (None for new commands)
/// * `new_cmd` - The new version of the command (None for deletions)
/// * `timestamp` - Unix timestamp for when this change occurred
///
/// # Returns
/// * `Ok(bool)` - true if this was a structural change (affects anchors, patches, or flags)
/// * `Err` - if validation or history recording fails
fn process_command_change(
    old_cmd: Option<&Command>,
    new_cmd: Option<&Command>,
    timestamp: i64,
) -> Result<bool, Box<dyn std::error::Error>> {
    match (old_cmd, new_cmd) {
        (Some(old), Some(new)) => {
            // Modified command - record to history
            append_to_history(new, timestamp)?;

            // Check if this is a structural change
            let is_structural = old.is_anchor() || new.is_anchor() ||
                               old.patch != new.patch ||
                               old.flags != new.flags;

            // TODO: Add per-command validation here if needed
            // For now, we'll rely on full validation during rescans

            Ok(is_structural)
        },
        (None, Some(new)) => {
            // New command - record to history
            append_to_history(new, timestamp)?;

            // Check if this is structural
            let is_structural = new.is_anchor() || !new.patch.is_empty();

            // TODO: Add per-command validation here if needed

            Ok(is_structural)
        },
        (Some(old), None) => {
            // Deleted command - record to history with special action
            let mut deleted_cmd = old.clone();
            deleted_cmd.action = "$DELETED$".to_string();
            append_to_history(&deleted_cmd, timestamp)?;

            // Deleting an anchor is structural
            Ok(old.is_anchor())
        },
        (None, None) => {
            // No change
            Ok(false)
        }
    }
}

/// Replace all commands in singleton, run patch inference, and save to disk
/// This is the primary way to perform batch modifications
/// Records all changes to history by comparing against cached commands
/// Always saves (flushes) to disk regardless of whether inference made changes
pub fn set_commands(mut commands: Vec<Command>) -> Result<(), Box<dyn std::error::Error>> {
    let set_commands_start = std::time::Instant::now();
    log(&format!("⏱️ SET_COMMANDS: Starting with {} commands", commands.len()));

    // Get current timestamp
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)?
        .as_secs() as i64;

    // Get cached commands for comparison
    let get_cached_start = std::time::Instant::now();
    let cached_commands = get_commands();
    log(&format!("⏱️ SET_COMMANDS: Get cached commands: {:?}", get_cached_start.elapsed()));

    // Create lookup maps for efficient comparison using dedup keys
    // This ensures we compare commands the same way the system deduplicates them
    let map_start = std::time::Instant::now();
    let cached_map = super::storage::build_command_map(&cached_commands);
    let new_map = super::storage::build_command_map(&commands);
    log(&format!("⏱️ SET_COMMANDS: Create lookup maps: {:?}", map_start.elapsed()));

    // Track changes for logging and validation decisions
    let mut created_count = 0;
    let mut modified_count = 0;
    let mut deleted_count = 0;
    let mut structural_changes = false; // True if anchors, patches, or flags changed

    // Process all new and modified commands
    let history_record_start = std::time::Instant::now();
    for new_cmd in &commands {
        let new_key = super::storage::command_dedup_key(new_cmd);
        if let Some(cached_cmd) = cached_map.get(&new_key) {
            // Command exists - check if it was modified
            if new_cmd.action != cached_cmd.action ||
               new_cmd.arg != cached_cmd.arg ||
               new_cmd.patch != cached_cmd.patch ||
               new_cmd.flags != cached_cmd.flags {
                // Command was modified - process the change
                let is_structural = process_command_change(Some(cached_cmd), Some(new_cmd), timestamp)?;
                structural_changes |= is_structural;
                modified_count += 1;
                detailed_log("HISTORY", &format!(
                    "Modified: '{}' (action: {} -> {}, patch: {} -> {})",
                    new_cmd.command,
                    cached_cmd.action,
                    new_cmd.action,
                    cached_cmd.patch,
                    new_cmd.patch
                ));
            }
        } else {
            // Command is new - process the creation
            let is_structural = process_command_change(None, Some(new_cmd), timestamp)?;
            structural_changes |= is_structural;
            created_count += 1;
            detailed_log("HISTORY", &format!(
                "Created: '{}' (action: {}, patch: {})",
                new_cmd.command,
                new_cmd.action,
                new_cmd.patch
            ));
        }
    }

    // Find deleted commands and process them
    for cached_cmd in &cached_commands {
        let cached_key = super::storage::command_dedup_key(cached_cmd);
        if !new_map.contains_key(&cached_key) {
            // Command was deleted - process the deletion
            let is_structural = process_command_change(Some(cached_cmd), None, timestamp)?;
            structural_changes |= is_structural;
            deleted_count += 1;
            detailed_log("HISTORY", &format!(
                "Deleted: '{}' (was action: {}, patch: {})",
                cached_cmd.command,
                cached_cmd.action,
                cached_cmd.patch
            ));
        }
    }

    log(&format!("⏱️ SET_COMMANDS: Record history: {:?}", history_record_start.elapsed()));

    // Log summary of changes
    if created_count > 0 || modified_count > 0 || deleted_count > 0 {
        log(&format!(
            "SET_COMMANDS: Recorded to history - Created: {}, Modified: {}, Deleted: {}",
            created_count, modified_count, deleted_count
        ));
    }

    // Decide whether we can skip validation
    // Skip validation for small changes (≤3 commands) - these are typically UI saves
    // Only run full validation for bulk operations like rescans (which have many changes)
    //
    // Rationale: Full validation on all 5000+ commands takes ~2 seconds
    // UI saves typically modify 1-3 commands and should be fast
    // Rescans will catch any structural issues that arise from skipped validation
    let total_changes = created_count + modified_count + deleted_count;
    let skip_validation = total_changes <= 3;

    if skip_validation {
        if structural_changes {
            log(&format!(
                "⚡ SET_COMMANDS: Skipping validation ({} structural changes) - UI save optimization. Rescan will validate.",
                total_changes
            ));
        } else {
            log(&format!(
                "⚡ SET_COMMANDS: Skipping validation ({} changes, non-structural) - performance optimization",
                total_changes
            ));
        }
    } else {
        log(&format!(
            "🔧 SET_COMMANDS: Running full validation ({} changes) - bulk operation",
            total_changes
        ));
    }

    // Flush to disk with inference
    let flush_start = std::time::Instant::now();
    let result = flush(&mut commands, skip_validation);
    let flush_time = flush_start.elapsed();
    if skip_validation {
        log(&format!("⏱️ SET_COMMANDS: Flush (validation skipped): {:?}", flush_time));
    } else {
        log(&format!("⏱️ SET_COMMANDS: Flush (includes validation): {:?}", flush_time));
    }
    log(&format!("⏱️ SET_COMMANDS: TOTAL TIME: {:?}", set_commands_start.elapsed()));

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

/// Comprehensive data loading function that performs all necessary steps in order:
/// 1. Load configuration
/// 2. Load commands (from disk or use override)
/// 3. Create patches hashmap from anchor commands
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


/// Set the active anchor - single source of truth for anchor activation
///
/// This function is the centralized way to set the active anchor. All code that
/// needs to activate an anchor should call this function.
///
/// # Arguments
/// * `anchor_name` - The name of the anchor command
/// * `anchor_folder` - Optional folder path associated with the anchor
///
/// # Returns
/// * `Ok(())` if the anchor was successfully set
/// * `Err` if saving state failed
pub fn set_active_anchor(anchor_name: String, anchor_folder: Option<String>) -> Result<(), Box<dyn std::error::Error>> {
    detailed_log("ANCHOR_SET", &format!("Setting active anchor: '{}' with folder: {:?}", anchor_name, anchor_folder));

    let mut state = get_state();
    state.anchor_name = Some(anchor_name.clone());
    state.anchor_timestamp = Some(chrono::Local::now().timestamp());
    state.anchor_folder = anchor_folder;

    match set_state(&state) {
        Ok(()) => {
            detailed_log("ANCHOR_SET", &format!("✅ Successfully set active anchor: '{}'", anchor_name));
            Ok(())
        },
        Err(e) => {
            log_error(&format!("Failed to set active anchor: {}", e));
            Err(e)
        }
    }
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

/// Append a command to history database
///
/// This is the public API for recording command changes to history.
/// Opens the history database, appends the command, and closes the connection.
///
/// # Arguments
/// * `cmd` - The command to record in history
/// * `timestamp` - Unix timestamp for when this change occurred
pub fn append_to_history(cmd: &Command, timestamp: i64) -> Result<(), Box<dyn std::error::Error>> {
    let conn = super::history::initialize_history_db()?;
    super::history::append_command(&conn, cmd, timestamp)?;
    Ok(())
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

// ============================================================================
// FOLDER TO PATCH MAPPING
// ============================================================================

/// Build a hashmap of absolute folder paths to patch names from anchor commands
/// This creates the folder hierarchy that patch inference will use
///
/// Only creates mappings for anchor files with matching subdirectories
/// (e.g., /@Avid Boustani/@Avid Boustani.md) and NOT for standalone anchor files
/// (e.g., /At/@Reed Shaffner.md) which would incorrectly map their parent directory
fn build_folder_to_patch_map(commands: &[Command], config: &Config) -> HashMap<std::path::PathBuf, String> {
    let mut folder_map = HashMap::new();

    // First pass: Add all anchor commands to the map
    for cmd in commands {
        if cmd.is_anchor() && !cmd.arg.is_empty() {
            // Get the file path to check if this is a true anchor file
            if let Some(file_path) = cmd.get_absolute_file_path(config) {
                // Only map folders for anchors that have a matching subdirectory
                // (e.g., /@Avid Boustani/@Avid Boustani.md)
                // NOT for standalone anchor files (e.g., /At/@Reed Shaffner.md)
                if crate::utils::is_anchor_file(&file_path) {
                    // This is a true anchor file with a matching subdirectory
                    // Use the proper accessor that handles both file and folder anchors correctly
                    if let Some(folder_path) = cmd.get_absolute_folder_path(config) {
                        // Canonicalize to handle symlinks and relative paths
                        if let Ok(canonical_folder) = folder_path.canonicalize() {
                            // Map this folder to the anchor's command name (which becomes the patch for its contents)
                            folder_map.insert(canonical_folder, cmd.command.clone());

                            detailed_log("PATCH_MAP", &format!(
                                "Folder '{}' -> patch '{}' (true anchor file with subdirectory)",
                                folder_path.display(), cmd.command
                            ));
                        }
                    }
                } else {
                    // This is a standalone anchor file without a matching subdirectory
                    // Do NOT map its parent directory - it doesn't define a patch for siblings
                    detailed_log("PATCH_MAP", &format!(
                        "Skipping folder mapping for standalone anchor '{}' (no subdirectory)",
                        cmd.command
                    ));
                }
            }
        }
    }

    folder_map
}

/// Query the patch name for a given folder path
/// Returns None if the folder is not mapped to any patch
pub fn get_patch_for_folder(folder_path: &std::path::Path) -> Option<String> {
    let (sys_data, _) = get_sys_data();

    // Try to canonicalize the path to match how it's stored in the map
    if let Ok(canonical) = folder_path.canonicalize() {
        sys_data.folder_to_patch.get(&canonical).cloned()
    } else {
        None
    }
}
