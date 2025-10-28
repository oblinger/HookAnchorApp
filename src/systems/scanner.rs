//! # File Scanner Module
//!
//! Scans the filesystem to discover markdown files, applications, and cloud resources (Notion),
//! creating commands for each discovered item while tracking file changes in history.
//!
//! ## How Scanning Works
//!
//! The scanner uses a shared command list that gets updated through the following steps:
//!
//! ```text
//! 1. scan_new_files(commands, sys_data) → Discover new files
//!    - Walks configured filesystem roots (e.g., ~/ob/kmr, /Applications)
//!    - Creates commands for markdown files, .app bundles, and DOC files
//!    - Records "created" history entries using file birth times
//!    - Sets file_size metadata on each command
//!    - Does NOT run inference - just discovery
//!    - Returns updated command list
//!
//! 2. scan_modified_files(&mut commands) → Detect file changes
//!    - Checks current file size against stored file_size metadata
//!    - Records "modified" history entries when size changed
//!    - Updates file_size and last_update timestamps
//!    - Returns count of files modified
//!
//! 3. load_manual_edits(&mut commands) → Apply user edits
//!    - Loads commands.txt (user's manual edits)
//!    - Merges user edits into command list (1pass, work, chrome, alias, url, etc.)
//!    - Preserves file_size metadata from filesystem scan
//!    - Returns count of edits applied
//!
//! 4. load_data(commands) → Run inference (done by caller, not scanner)
//!    - Assigns patches to commands without patches
//!    - Creates virtual anchor commands
//!    - Normalizes patch case
//!    - Saves complete command set to disk
//! ```
//!
//! ## Key Methods
//!
//! - **`scan_new_files()`** - Discovers files not yet tracked, creates commands, records creation history
//! - **`scan_modified_files()`** - Detects size changes in tracked files, records modification history
//! - **`load_manual_edits()`** - Merges user's manual command edits from commands.txt
//! - **`scan_check()`** - Background scanning at configured intervals
//!
//! ## Guarantees
//!
//! - **No duplicate history entries**: Each file scanned exactly once per rescan
//! - **Accurate timestamps**: Uses file birth time for "created", modification time for "modified"
//! - **User edits preserved**: Commands with 'U' flag never automatically deleted or modified
//! - **Single source of truth**: Cache tracks current file state, commands.txt stores user edits

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command as ProcessCommand;
use std::collections::{hash_map::DefaultHasher, HashSet};
use std::hash::{Hash, Hasher};
use crate::core::Command;
use crate::core::Config;
use crate::core::commands::{FLAG_USER_EDITED, FLAG_ANCHOR};
use crate::utils::detailed_log;
use crate::execute::get_action;
use chrono::Local;

/// Action types that are automatically generated and removed by the scanner
/// These commands will be removed during rescanning unless they have the 'U' (user-edited) flag
/// NOTE: "anchor" is NOT included - we preserve existing anchors during rescan so patch inference can work
pub const SCANNER_GENERATED_ACTIONS: &[&str] = &["markdown", "folder", "app", "open_app", "doc"];


// TODO: DEAD CODE - Remove this function
// This is only used by delete_anchors which is also dead code
/// Check if a command is a Notion anchor
/// Simple check that doesn't require accessing the actions module
fn is_notion_anchor(cmd: &Command) -> bool {
    cmd.is_anchor() &&
    (cmd.arg.contains("notion.so") || cmd.patch == "Notion Root")
}

// TODO: DEAD CODE - Remove this function
// Previously used to delete file-based anchors, but scanner now creates markdown files
// with action:"markdown" + 'A' flag instead of action:"anchor"
// Only remaining use is for Notion anchor deletion which is currently disabled
/// Delete anchor commands based on whether they are Notion anchors or not
///
/// # Arguments
/// * `commands` - Mutable reference to the command list
/// * `delete_notion_anchors` - If true, delete Notion anchors. If false, delete non-Notion anchors
/// * `verbose` - Whether to print verbose output
///
/// Returns the number of commands deleted
fn delete_anchors(commands: &mut Vec<Command>, delete_notion_anchors: bool, verbose: bool) -> usize {
    let initial_count = commands.len();

    commands.retain(|cmd| {
        // Skip if not an anchor
        if cmd.action != "anchor" {
            return true;
        }

        // Always keep user-edited commands
        if cmd.flags.contains(FLAG_USER_EDITED) {
            return true;
        }

        // Always keep special system anchors
        if cmd.command == "orphans" || cmd.command == "Notion Root" {
            return true;
        }

        // Always keep virtual anchors (anchors with no file path)
        // Virtual anchors are created by patch inference for orphaned commands
        if cmd.arg.is_empty() {
            return true;
        }

        // Check if this is a Notion anchor
        let is_notion = is_notion_anchor(cmd);

        // Delete based on the flag
        if delete_notion_anchors {
            // We want to delete Notion anchors, so keep non-Notion
            !is_notion
        } else {
            // We want to delete non-Notion anchors, so keep Notion
            is_notion
        }
    });
    
    let deleted = initial_count - commands.len();
    if deleted > 0 && verbose {
        if delete_notion_anchors {
            crate::utils::print(&format!("   Removed {} non-user-edited Notion anchor commands", deleted));
        } else {
            crate::utils::print(&format!("   Removed {} non-user-edited file anchor commands", deleted));
        }
    }
    deleted
}

/// Main function for automatic background scanning
/// Checks if filesystem scan should be performed and executes it if needed.
/// This function should be called on application exit, not startup.
pub fn scan_check(commands: Vec<Command>) -> Vec<Command> {
    let (sys_data, _) = crate::core::data::get_sys_data();
    let mut state = crate::core::data::get_state();
    
    let current_time = Local::now().timestamp();
    
    // Get scan interval from config (default to 10 seconds)
    let scan_interval = sys_data.config.popup_settings.scan_interval_seconds.unwrap_or(10) as i64;
    
    // Check if enough time has passed since last scan
    let should_scan = match state.last_scan_time {
        Some(last_time) => (current_time - last_time) >= scan_interval,
        None => true, // Never scanned before
    };
    
    if !should_scan {
        return commands; // Not time to scan yet
    }
    
    // Scanner should not manage log file size - that's done when popup opens
    
    // Performing filesystem scan
    // Perform filesystem scan
    let _file_roots = match &sys_data.config.popup_settings.file_roots {
        Some(roots) => roots,
        None => {
            detailed_log("SCANNER", "ERROR: No file_roots configured in config file");
            return commands; // Return without scanning if no roots configured
        }
    };
    
    
    let scanned_commands = scan(commands, &sys_data);
    
    // Calculate checksum of the scan results
    let new_checksum = calculate_commands_checksum(&scanned_commands);
    
    // Check if checksum has changed
    let checksum_changed = match &state.last_scan_checksum {
        Some(old_checksum) => old_checksum != &new_checksum,
        None => true, // First scan
    };
    
    // Update state with scan time and checksum
    state.last_scan_time = Some(current_time);
    state.last_scan_checksum = Some(new_checksum);

    // Save updated state
    if let Err(e) = crate::core::data::set_state(&state) {
        crate::utils::log_error(&format!("Failed to save scan state: {}", e));
    }
    
    // Save commands only if checksum changed
    if checksum_changed {
        // Use sys_data::set_commands - runs patch inference and saves
        if let Err(e) = crate::core::set_commands(scanned_commands.clone()) {
            crate::utils::log_error(&format!("Failed to save updated commands: {}", e));
        }
    }

    // Return the scanned commands
    scanned_commands
}

/// Internal scan function that orchestrates all scanning operations
fn scan(commands: Vec<Command>, sys_data: &crate::core::data::SysData) -> Vec<Command> {
    scan_new_files(commands, sys_data, false)
}

/// Loads user's manual edits from commands.txt and merges them into the command list
/// This is the FINAL step in rescan - user edits override filesystem scanning
/// For each command in commands.txt:
/// - If exists in commands: Update metadata if different
/// - If NOT in commands: Add it (user manually added)
/// Returns number of commands added/updated from manual edits
pub fn load_manual_edits(commands: &mut Vec<Command>, verbose: bool) -> Result<usize, Box<dyn std::error::Error>> {
    if verbose {
        crate::utils::print("\n📝 Loading manual edits from commands.txt...");
    }

    // Load commands.txt
    let txt_commands = crate::core::commands::load_commands_raw();
    crate::utils::log(&format!("LOAD_MANUAL_EDITS: Loaded {} commands from commands.txt", txt_commands.len()));

    let mut edits_applied = 0;
    let mut commands_added = 0;
    let mut commands_updated = 0;
    // History recording is now automatic in sys_data::set_commands()
    // No need to track or record history here

    // For each command in commands.txt, check if we need to add or update it
    for txt_cmd in txt_commands {
        // DEDUPLICATION: Skip scanner-generated commands from commands.txt that don't have U flag
        // The scanner will have already created fresh versions with correct data
        // IMPORTANT: Only apply this to scanner-generated actions, not manual commands (alias, launcher, etc.)
        let is_scanner_generated = SCANNER_GENERATED_ACTIONS.contains(&txt_cmd.action.as_str());
        if is_scanner_generated && !txt_cmd.flags.contains(FLAG_USER_EDITED) {
            crate::utils::detailed_log("MANUAL_EDITS", &format!("Skipping scanner-generated command '{}' from commands.txt (no U flag)", txt_cmd.command));
            continue;
        }

        // Find corresponding command in our list (match by patch, command name, and action)
        let existing_idx = commands.iter().position(|c|
            c.patch == txt_cmd.patch && c.command == txt_cmd.command && c.action == txt_cmd.action
        );

        match existing_idx {
            Some(idx) => {
                // Command exists - check if it changed
                let existing = &commands[idx];

                // Only update if something actually changed (besides file_size which we track separately)
                if existing.arg != txt_cmd.arg || existing.flags != txt_cmd.flags {
                    if verbose {
                        crate::utils::print(&format!("   ✏️  Updated manual edit: '{}'", txt_cmd.command));
                    }
                    crate::utils::log(&format!("LOAD_MANUAL_EDITS: Updating command '{}' (action: {}) - arg or flags changed", txt_cmd.command, txt_cmd.action));

                    let old_cmd = existing.clone();

                    // Preserve file_size and last_update from filesystem scan
                    let mut updated_cmd = txt_cmd.clone();
                    updated_cmd.file_size = existing.file_size;
                    updated_cmd.last_update = existing.last_update;

                    commands[idx] = updated_cmd.clone();
                    edits_applied += 1;
                    commands_updated += 1;
                    // History will be recorded automatically when set_commands is called
                }
            }
            None => {
                // New command from manual edit - add it
                if verbose {
                    crate::utils::print(&format!("   ➕ Added manual command: '{}'", txt_cmd.command));
                }
                crate::utils::log(&format!("LOAD_MANUAL_EDITS: Adding new command '{}' (action: {}) from manual edit", txt_cmd.command, txt_cmd.action));
                commands.push(txt_cmd.clone());
                edits_applied += 1;
                commands_added += 1;
                // History will be recorded automatically when set_commands is called
            }
        }
    }

    if verbose {
        if edits_applied > 0 {
            crate::utils::print(&format!("   ✅ Applied {} manual edit(s)", edits_applied));
        } else {
            crate::utils::print("   ✅ No manual edits to apply");
        }
    }

    // Only log if actual changes were made
    if commands_added > 0 || commands_updated > 0 {
        crate::utils::log(&format!("LOAD_MANUAL_EDITS: Applied {} edits ({} added, {} updated)",
            edits_applied, commands_added, commands_updated));
    } else {
        crate::utils::detailed_log("MANUAL_EDITS", "No changes needed - cache and commands.txt are in sync");
    }

    Ok(edits_applied)
}

/// Scans filesystem to detect MODIFIED files and update history
/// Compares current file sizes against stored file_size metadata
/// Records "modified" history entries for files that changed
/// Returns number of files that were modified
pub fn scan_modified_files(commands: &mut Vec<Command>, verbose: bool) -> Result<usize, Box<dyn std::error::Error>> {
    if verbose {
        crate::utils::print("\n🔍 Scanning for file modifications...");
    }

    let mut file_changes = 0;

    for cmd in commands.iter_mut() {
        // Only check file-based commands
        if !cmd.is_path_based() {
            continue;
        }

        // Get file path
        let file_path = match cmd.get_absolute_file_path(&crate::core::data::get_config()) {
            Some(path) => path,
            None => continue,
        };

        // Check if file exists and get current size
        let current_metadata = match std::fs::metadata(&file_path) {
            Ok(meta) => meta,
            Err(_) => continue, // File doesn't exist or can't be read
        };

        let current_size = current_metadata.len();
        let current_mtime = current_metadata.modified()
            .ok()
            .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
            .map(|d| d.as_secs() as i64);

        // Check if file size changed
        // IMPORTANT: Only record modifications if we have BOTH old and new file sizes
        // If old file_size is None, this is a newly discovered file (should be handled by scan_new_files)
        // This prevents recording duplicate "modified" entries on every rescan
        if let Some(old_size) = cmd.file_size {
            if old_size != current_size {
                if verbose {
                    crate::utils::print(&format!("   📝 File size changed: '{}' ({} -> {} bytes)",
                        cmd.command, old_size, current_size));
                }

                crate::utils::log(&format!("SCAN_MODIFIED: File size changed for '{}' ({} -> {})",
                    cmd.command, old_size, current_size));

                // Create updated command with new file size
                let old_cmd = cmd.clone();
                cmd.file_size = Some(current_size);
                if let Some(mtime) = current_mtime {
                    cmd.last_update = mtime;
                }

                // History will be recorded automatically when set_commands is called
                file_changes += 1;
            }
        } else {
            // File exists but we don't have a stored size - update it but don't record as modification
            cmd.file_size = Some(current_size);
            if let Some(mtime) = current_mtime {
                cmd.last_update = mtime;
            }
        }
    }

    if verbose {
        if file_changes > 0 {
            crate::utils::print(&format!("   ✅ Detected {} file modification(s)", file_changes));
        } else {
            crate::utils::print("   ✅ No file modifications detected");
        }
    }

    crate::utils::log(&format!("SCAN_MODIFIED: Detected {} file changes", file_changes));

    Ok(file_changes)
}

/// Scans filesystem for NEW files not yet in the command list
/// Used for the --rescan command line option
/// This function discovers new files and adds them to the command list
/// AND records "created" history entries with file birth times
/// Scans the filesystem for new files and creates commands for them
///
/// This is the top-level entry point for filesystem scanning during rescan operations.
///
/// Steps:
/// 1. Calls scan_files() which:
///    - Deletes all scanner-generated commands (markdown, folder, app, doc, anchor)
///    - Preserves user-edited commands (those with 'U' flag)
///    - Scans configured file_roots directories recursively
///    - Creates new commands for:
///      * Markdown files (with 'a' flag if they're anchor files)
///      * App bundles (.app directories)
///      * Doc files (configured extensions)
///    - Leaves all command patches empty (will be filled by validate_and_repair_patches)
/// 2. Records "created" history entries for newly discovered files
/// 3. Returns the updated commands list
///
/// Note: This function does NOT assign patches to commands. That happens later in
/// validate_and_repair_patches() which is called by flush() after set_commands().
pub fn scan_new_files(commands: Vec<Command>, sys_data: &crate::core::data::SysData, verbose: bool) -> Vec<Command> {
    let empty_vec = vec![];
    let file_roots = sys_data.config.popup_settings.file_roots.as_ref().unwrap_or(&empty_vec);

    if verbose {
        crate::utils::print("\n🔍 Starting filesystem scan...");
        crate::utils::print(&format!("   Scanning roots: {:?}", file_roots));
    }

    // First scan files (markdown and apps)
    let initial_count = commands.len();
    let mut commands = scan_files(commands, file_roots, &sys_data.config);

    let new_count = commands.len();
    let files_added = new_count.saturating_sub(initial_count);

    if verbose {
        crate::utils::print(&format!("   Scan complete: {} commands total", new_count));
        if files_added > 0 {
            crate::utils::print(&format!("   Added {} new commands", files_added));
        }
    }

    // Still need to set file_size metadata so scan_modified_files knows they're up-to-date
    // CRITICAL: This prevents detecting files as modified right after creation
    // Note: scan_files may delete commands before adding new ones, so we can't use initial_count
    // Instead, we need to identify which commands are file-based and newly scanned
    for cmd in &mut commands {
        // Only update for file-based commands
        if !cmd.is_path_based() {
            continue;
        }

        // Get file path
        let file_path = match cmd.get_absolute_file_path(&sys_data.config) {
            Some(path) => path,
            None => continue,
        };

        // Get file metadata (size and mtime)
        if let Ok(meta) = std::fs::metadata(&file_path) {
            cmd.file_size = Some(meta.len());

            if let Ok(mtime) = meta.modified() {
                if let Ok(duration) = mtime.duration_since(std::time::UNIX_EPOCH) {
                    cmd.last_update = duration.as_secs() as i64;
                }
            }
        }
    }
    // History will be recorded automatically when set_commands is called

    // Then scan cloud services (Notion, Google Drive)
    crate::utils::log("☁️  Scanning cloud services...");
    let scan_result = crate::cloud_scanner::scan_cloud_services();
    
    // Only process Notion commands if we actually got pages (meaning scan was enabled)
    if !scan_result.notion_pages.is_empty() {
        // Delete existing Notion anchors during full scans
        if !scan_result.is_incremental {
            delete_anchors(&mut commands, true, verbose);  // true = delete Notion anchors
        } else {
            if verbose {
                crate::utils::print("   Incremental Notion scan - preserving existing commands");
            }
            crate::utils::log("[NOTION] Incremental scan - existing commands preserved");
        }
    } else {
        // No Notion pages means scanning was disabled or failed
        if verbose {
            crate::utils::detailed_log("SCANNER", "Notion scanning disabled or returned no pages");
        }
    }
    
    // Only process Notion pages if we have any
    if !scan_result.notion_pages.is_empty() {
        // First, create the Notion Root anchor if it doesn't exist
        let notion_root_exists = commands.iter().any(|c| c.command == "Notion Root" && c.is_anchor());
        if !notion_root_exists {
            commands.push(Command {
                command: "Notion Root".to_string(),
                action: String::new(),  // Virtual anchor - blank action (non-executable)
                arg: String::new(),  // Virtual anchor, no actual file
                flags: "A".to_string(),  // Anchor flag
                patch: "orphans".to_string(),  // Notion Root is under orphans
                other_params: None,
                last_update: 0,
                file_size: None,
            });
            if verbose {
                crate::utils::print("   Created Notion Root anchor");
            }
        }

        // Create/update notion anchor commands for each scanned page
        let mut notion_added = 0;
        let mut notion_updated = 0;
        for page in scan_result.notion_pages {
        // Create a command name from the page title (sanitize it)
        let command_name = page.title
            .chars()
            .filter(|c| c.is_alphanumeric() || *c == ' ' || *c == '-' || *c == '_')
            .collect::<String>()
            .trim()
            .replace(' ', " ");  // Keep spaces for readability
        
        // For now, all Notion pages are children of Notion Root
        // TODO: In future, parse parent_path to create proper hierarchy
        let parent_patch = "Notion Root".to_string();
        
        // Check if a command with this name already exists as an anchor
        if let Some(existing_cmd) = commands.iter_mut().find(|cmd|
            cmd.command.eq_ignore_ascii_case(&command_name) &&
            (cmd.is_anchor() || cmd.action == "notion")
        ) {
            // Update existing command to be an anchor if it was "notion"
            if existing_cmd.action == "notion" {
                existing_cmd.action = "url".to_string();
                existing_cmd.set_flag(crate::core::commands::FLAG_ANCHOR, "");  // Set anchor flag
                existing_cmd.patch = parent_patch.clone();
                notion_updated += 1;
            }
            // Update the URL if it changed
            if existing_cmd.arg != page.url {
                crate::utils::detailed_log("SCANNER", &format!("Updating Notion anchor URL for '{}': {} -> {}", 
                    command_name, existing_cmd.arg, page.url));
                existing_cmd.arg = page.url;
                notion_updated += 1;
            }
            // Update patch if different
            if existing_cmd.patch != parent_patch {
                existing_cmd.patch = parent_patch;
                notion_updated += 1;
            }
        } else if !commands.iter().any(|cmd| cmd.command.eq_ignore_ascii_case(&command_name)) {
            // No command with this name exists at all, create new anchor
            commands.push(Command {
                command: command_name.clone(),
                action: "url".to_string(),  // Notion pages are URLs
                arg: page.url,              // URL as the arg
                flags: "A".to_string(),     // Anchor flag
                patch: parent_patch,        // Parent page as the patch
                other_params: None,
                last_update: 0,
                file_size: None,
            });
            notion_added += 1;
        } else {
            // Command exists but it's not an anchor/notion command, skip
            crate::utils::detailed_log("SCANNER", &format!("Skipping Notion page '{}' - non-anchor command exists", command_name));
        }
    }
    
        if (notion_added > 0 || notion_updated > 0) && verbose {
            if notion_added > 0 {
                crate::utils::print(&format!("   Added {} new Notion anchor commands", notion_added));
            }
            if notion_updated > 0 {
                crate::utils::print(&format!("   Updated {} existing Notion anchor commands", notion_updated));
            }
        }
    } // End of Notion pages processing
    
    // Then scan contacts - DISABLED for performance
    // commands = scan_contacts(commands);
    
    // Patches without anchors will simply remain without anchors

    // NOTE: Do NOT run inference here! The caller (run_rescan_command) will run inference
    // AFTER load_manual_edits() loads all the non-file commands. Running inference here
    // would cause load_data() to save the incomplete command set to disk, wiping out
    // all manually-created commands (1pass, work, chrome, alias, url, etc.)

    if verbose {
        crate::utils::print("\n🎉 File scan complete! (Inference will run after loading manual edits)");
    }

    // Return the scanned commands WITHOUT running inference
    commands
}

/// Statistics from merge-based scanning
#[derive(Debug, Default)]
struct ScanStats {
    created: usize,
    updated: usize,
    deleted: usize,
    unchanged: usize,
}

/// Check if a file path is within any of the configured scan roots
fn is_within_scan_roots(file_path: &Path, scan_roots: &[String]) -> bool {
    for root in scan_roots {
        let expanded_root = expand_home(root);
        if file_path.starts_with(&expanded_root) {
            return true;
        }
    }
    false
}

/// Check if a command has changed by comparing key fields
fn has_changed(old: &Command, new: &Command) -> bool {
    old.action != new.action ||
    old.arg != new.arg ||
    old.file_size != new.file_size
}

/// Update or create a command in the command list
/// Returns true if command was added or updated, false if unchanged
fn update_or_create_command(
    new_cmd: Command,
    commands: &mut Vec<Command>,
    config: &Config,
) -> bool {
    // Get absolute file path for comparison
    let file_path = match new_cmd.get_absolute_file_path(config) {
        Some(path) => path.to_string_lossy().to_string(),
        None => return false, // Skip if we can't get path
    };

    // Find existing command by file path
    let existing_idx = commands.iter().position(|cmd| {
        if let Some(existing_path) = cmd.get_absolute_file_path(config) {
            existing_path.to_string_lossy() == file_path
        } else {
            false
        }
    });

    match existing_idx {
        Some(idx) => {
            let existing = &commands[idx];

            // Check if user-edited - preserve if so
            if existing.flags.contains(FLAG_USER_EDITED) {
                crate::utils::detailed_log("UPDATE_CMD", &format!(
                    "Unchanged: '{}' - user-edited, preserving",
                    existing.command
                ));
                return false;
            }

            // Check for changes
            if has_changed(existing, &new_cmd) {
                crate::utils::log(&format!(
                    "Updated: '{}' - file modified",
                    new_cmd.command
                ));
                commands[idx] = new_cmd;
                return true;
            } else {
                crate::utils::detailed_log("UPDATE_CMD", &format!(
                    "Unchanged: '{}' - no changes detected",
                    existing.command
                ));
                return false;
            }
        }
        None => {
            // New command
            crate::utils::log(&format!(
                "Created: '{}' - new file discovered at {}",
                new_cmd.command, file_path
            ));
            commands.push(new_cmd);
            return true;
        }
    }
}

/// Phase 1: Discover all files in scan roots and build HashMap
/// Returns HashMap keyed by absolute file path
fn discover_files(file_roots: &[String], config: &Config) -> std::collections::HashMap<String, Command> {
    use std::collections::HashMap;

    let mut discovered = HashMap::new();

    for root in file_roots {
        let expanded_root = expand_home(root);
        let root_path = Path::new(&expanded_root);

        if !root_path.exists() || !root_path.is_dir() {
            crate::utils::detailed_log("SCANNER", &format!("Skipping non-existent root: {}", expanded_root));
            continue;
        }

        crate::utils::detailed_log("SCANNER", &format!("Discovering files in: {}", expanded_root));
        discover_files_recursive(root_path, root_path, &mut discovered, config);
    }

    crate::utils::detailed_log("SCANNER", &format!("Discovered {} files total", discovered.len()));
    discovered
}

/// Recursively discover files in a directory
fn discover_files_recursive(
    dir: &Path,
    vault_root: &Path,
    discovered: &mut std::collections::HashMap<String, Command>,
    config: &Config
) {
    let entries = match fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(e) => {
            crate::utils::detailed_log("SCANNER", &format!("Cannot read directory {}: {}", dir.display(), e));
            return;
        }
    };

    for entry in entries {
        let entry = match entry {
            Ok(e) => e,
            Err(_) => continue,
        };

        let path = entry.path();

        // Skip hidden files/folders (starting with .)
        if let Some(name) = path.file_name() {
            if let Some(name_str) = name.to_str() {
                if name_str.starts_with('.') {
                    continue;
                }
            }
        }

        if path.is_dir() {
            // Check if it's an .app bundle
            if let Some(ext) = path.extension() {
                if ext == "app" {
                    // Create app command
                    if let Some(app_name) = path.file_stem() {
                        if let Some(name_str) = app_name.to_str() {
                            let app_path = path.to_string_lossy().to_string();

                            // Get file metadata for size (for .app bundles, this will be the bundle directory size)
                            let file_size = fs::metadata(&path)
                                .ok()
                                .map(|m| m.len());

                            let command = Command {
                                command: format!("{} App", name_str),
                                action: "app".to_string(),
                                arg: app_path.clone(),
                                patch: String::new(), // Will be assigned during inference
                                flags: String::new(),
                                other_params: None,
                                last_update: 0,
                                file_size,
                            };
                            discovered.insert(app_path, command);
                        }
                    }
                    continue; // Don't recurse into .app bundles
                }
            }

            // Recurse into regular directories
            discover_files_recursive(&path, vault_root, discovered, config);
        } else if path.is_file() {
            // Check file extension
            if let Some(ext) = path.extension() {
                let ext_str = ext.to_str().unwrap_or("");

                let action = match ext_str {
                    "md" => "markdown",
                    "doc" | "docx" | "txt" | "rtf" | "pdf" | "ppt" | "pptx" | "xls" | "xlsx" | "csv" | "numbers" | "pages" | "key" => "doc",
                    _ => continue, // Skip other file types
                };

                // Create command for this file
                if let Some(file_name) = path.file_stem() {
                    if let Some(name_str) = file_name.to_str() {
                        let file_path_str = path.to_string_lossy().to_string();

                        // Get file metadata for size
                        let file_size = fs::metadata(&path)
                            .ok()
                            .map(|m| m.len());

                        let command = Command {
                            command: name_str.to_string(),
                            action: action.to_string(),
                            arg: file_path_str.clone(),
                            patch: String::new(), // Will be assigned during inference
                            flags: String::new(),
                            other_params: None,
                            last_update: 0,
                            file_size,
                        };
                        discovered.insert(file_path_str, command);
                    }
                }
            }
        }
    }
}

/// Merge-based scanning: intelligently merge discovered files with existing commands
/// This eliminates history churn by only recording actual changes
fn scan_files_merge_based(
    mut commands: Vec<Command>,
    file_roots: &[String],
    config: &Config
) -> (Vec<Command>, ScanStats) {
    use std::collections::HashMap;

    let mut stats = ScanStats::default();

    // Phase 1: Discover all files in scan roots
    crate::utils::detailed_log("SCANNER", "Phase 1: Discovering files in scan roots...");
    let mut discovered_files = discover_files(file_roots, config);

    // Phase 2: Process existing commands
    crate::utils::detailed_log("SCANNER", "Phase 2: Processing existing commands...");
    commands.retain_mut(|cmd| {
        // Skip non-scanner-generated actions (alias, url, 1pass, work, chrome, etc.)
        if !SCANNER_GENERATED_ACTIONS.contains(&cmd.action.as_str()) {
            stats.unchanged += 1;
            return true; // Keep
        }

        // Get absolute file path for this command
        let file_path = match cmd.get_absolute_file_path(config) {
            Some(path) => path,
            None => {
                // Command doesn't have a valid path - keep it unchanged
                stats.unchanged += 1;
                return true;
            }
        };

        // Check if file is within scan roots
        if !is_within_scan_roots(&file_path, file_roots) {
            // Outside scan scope - don't touch
            stats.unchanged += 1;
            crate::utils::detailed_log("SCANNER", &format!(
                "Keeping '{}' - outside scan roots", cmd.command
            ));
            return true;
        }

        // File is within scan roots - check if it still exists
        let file_path_str = file_path.to_string_lossy().to_string();
        if let Some(new_cmd) = discovered_files.remove(&file_path_str) {
            // File still exists

            // If user-edited, preserve the existing command completely
            if cmd.flags.contains(FLAG_USER_EDITED) {
                stats.unchanged += 1;
                crate::utils::detailed_log("SCANNER", &format!(
                    "Preserving user-edited '{}' (U flag)", cmd.command
                ));
                return true;
            }

            // Not user-edited - check for changes
            if has_changed(cmd, &new_cmd) {
                // Update command with new data
                let old_action = cmd.action.clone();
                let old_size = cmd.file_size;
                cmd.action = new_cmd.action;
                cmd.arg = new_cmd.arg;
                cmd.file_size = new_cmd.file_size;
                cmd.last_update = new_cmd.last_update;

                stats.updated += 1;
                crate::utils::log(&format!(
                    "Updated: '{}' - action:{:?}→{:?}, size:{:?}→{:?}",
                    cmd.command, old_action, cmd.action, old_size, cmd.file_size
                ));
            } else {
                // No change
                stats.unchanged += 1;
            }

            return true; // Keep command
        } else {
            // File no longer exists - delete command even if user-edited
            stats.deleted += 1;
            crate::utils::log(&format!(
                "Deleted: '{}' - file no longer exists at {}",
                cmd.command, file_path_str
            ));
            return false; // Remove from list
        }
    });

    // Phase 3: Add remaining discovered files (new files not in existing commands)
    // IMPORTANT: Process anchor files FIRST so they're in folder_map for child files
    crate::utils::detailed_log("SCANNER", "Phase 3a: Adding new anchor files first...");

    // Separate anchors and non-anchors
    let mut anchor_files = Vec::new();
    let mut regular_files = Vec::new();

    for (file_path, cmd) in discovered_files {
        let path = Path::new(&file_path);
        if crate::utils::is_anchor_file(path) {
            anchor_files.push((file_path, cmd));
        } else {
            regular_files.push((file_path, cmd));
        }
    }

    // Add anchors first - we'll run full inference after all anchors are added
    for (file_path, cmd) in anchor_files {
        stats.created += 1;
        crate::utils::log(&format!(
            "Created: '{}' (anchor) - new file discovered at {}",
            cmd.command, file_path
        ));
        commands.push(cmd);
    }

    // Run inference now that all anchors are in the list
    // This ensures folder_map includes the new anchors for regular file inference
    crate::utils::detailed_log("SCANNER", "Running inference on anchors...");
    if let Err(e) = crate::core::set_commands(commands.clone()) {
        crate::utils::log_error(&format!("Failed to run inference on anchors: {}", e));
    }

    // Reload commands with inferred patches
    let (sys_data, _) = crate::core::get_sys_data();
    commands = sys_data.commands;

    // Now add regular files - anchors are in folder_map
    crate::utils::detailed_log("SCANNER", "Phase 3b: Adding new regular files...");
    for (file_path, cmd) in regular_files {
        stats.created += 1;
        crate::utils::log(&format!(
            "Created: '{}' - new file discovered at {}",
            cmd.command, file_path
        ));
        commands.push(cmd);
    }

    // Log summary
    crate::utils::log(&format!(
        "Scan complete: +{} created, ~{} updated, -{} deleted, ={} unchanged",
        stats.created, stats.updated, stats.deleted, stats.unchanged
    ));

    (commands, stats)
}

/// Scans the configured file roots and returns an updated command list
fn scan_files(commands: Vec<Command>, file_roots: &[String], config: &Config) -> Vec<Command> {
    // Use merge-based scanning to eliminate history churn
    // This intelligently merges discovered files with existing commands,
    // only logging actual changes (created/updated/deleted)
    let (commands, _stats) = scan_files_merge_based(commands, file_roots, config);

    // OLD APPROACH (deleted): The old code deleted all non-user-edited scanner commands
    // then recreated them, causing massive history churn for unchanged files.
    // The new merge-based approach preserves existing commands and their patches,
    // only making changes when files are actually added, modified, or removed.

    commands
}

// OLD IMPLEMENTATION: Preserved below for reference, can be deleted after testing
// The old scan_files implementation that caused history churn
#[allow(dead_code)]
fn scan_files_old_delete_recreate_approach(mut commands: Vec<Command>, file_roots: &[String], config: &Config) -> Vec<Command> {

    // Create a lookup map to preserve existing patches when regenerating commands
    let mut existing_patches: std::collections::HashMap<String, String> = std::collections::HashMap::new();
    for cmd in &commands {
        if (cmd.action == "markdown" || cmd.is_anchor() || cmd.action == "folder" || cmd.action == "app" || cmd.action == "open_app" || cmd.action == "doc") && !cmd.patch.is_empty() {
            existing_patches.insert(cmd.command.clone(), cmd.patch.clone());
        }
    }

    // Count commands before removal
    let _markdown_before = commands.iter().filter(|cmd| cmd.action == "markdown").count();
    let _anchor_before = commands.iter().filter(|cmd| cmd.is_anchor()).count();
    let _folder_before = commands.iter().filter(|cmd| cmd.action == "folder").count();
    let _app_before = commands.iter().filter(|cmd| cmd.action == "app" || cmd.action == "open_app").count();
    let _user_edited_scanner_commands = commands.iter()
        .filter(|cmd| SCANNER_GENERATED_ACTIONS.contains(&cmd.action.as_str()) && cmd.flags.contains(FLAG_USER_EDITED))
        .count();

    // Simple rule: Delete all scanner-generated commands that don't have the U (user-edited) flag
    // The scanner will recreate them if the files still exist
    // This eliminates all special cases and complexity
    commands.retain(|cmd| {
        let is_scanner_generated = SCANNER_GENERATED_ACTIONS.contains(&cmd.action.as_str());
        let is_user_edited = cmd.flags.contains(FLAG_USER_EDITED);

        if is_scanner_generated && !is_user_edited {
            // Delete scanner-generated commands without U flag
            crate::utils::detailed_log("SCANNER", &format!("Removing scanner-generated command '{}' (action: {}) for rescan", cmd.command, cmd.action));
            return false;
        }

        if is_scanner_generated && is_user_edited {
            // Keep user-edited scanner-type commands
            crate::utils::detailed_log("SCANNER", &format!("Keeping user-edited command: '{}' (action: {})", cmd.command, cmd.action));
        }

        // Keep all user-edited commands and non-scanner-generated commands
        true
    });

    let _preserved_user_edited = commands.iter()
        .filter(|cmd| SCANNER_GENERATED_ACTIONS.contains(&cmd.action.as_str()) && cmd.flags.contains(FLAG_USER_EDITED))
        .count();

    // Create a set of existing command names for collision detection (lowercase for case-insensitive comparison)
    // This is done AFTER removing old scanner-generated commands to avoid false collisions
    // Include ALL remaining commands to properly detect collisions with user commands and non-scanner commands
    let mut existing_commands: HashSet<String> = commands.iter()
        .map(|cmd| cmd.command.to_lowercase())
        .collect();

    // Create a set of file paths that are already handled by remaining commands
    // This is done AFTER deletion to allow recreation of deleted anchors
    // Skip empty args - they don't represent handled files
    let mut handled_files: HashSet<String> = HashSet::new();
    for cmd in &commands {
        if !cmd.arg.is_empty() && (cmd.action == "markdown" || cmd.is_anchor() || cmd.action == "folder" || cmd.action == "app" || cmd.action == "open_app" || cmd.action == "doc") {
            handled_files.insert(cmd.arg.clone());
        }
    }

    // STEP 2: Build initial folder_map from remaining user-edited anchors
    // This map will be populated incrementally as we discover anchors during scanning
    let mut folder_map: std::collections::HashMap<PathBuf, String> = std::collections::HashMap::new();
    for cmd in &commands {
        if cmd.is_anchor() && !cmd.arg.is_empty() {
            // Get the folder path for this anchor
            if let Some(folder_path) = cmd.get_absolute_folder_path(config) {
                if let Ok(canonical_folder) = folder_path.canonicalize() {
                    folder_map.insert(canonical_folder, cmd.command.clone());
                    crate::utils::log(&format!("Initial folder_map: '{}' -> '{}'", folder_path.display(), cmd.command));
                }
            }
        }
    }
    crate::utils::log(&format!("Built initial folder_map with {} user-edited anchors", folder_map.len()));

    // Collect folders during scanning
    // COMMENTED OUT: Folder scanning disabled for now - only creating commands for markdown files
    // let mut found_folders: Vec<PathBuf> = Vec::new();

    // STEP 3: Scan for new files with depth-first two-pass algorithm
    for root in file_roots {
        let expanded_root = expand_home(root);
        let root_path = Path::new(&expanded_root);
        
        crate::utils::print(&format!("📂 Scanning directory: {}", expanded_root));
        
        if root_path.exists() && root_path.is_dir() {
            let _commands_before_scan = commands.len();
            // COMMENTED OUT: Passing empty Vec instead of found_folders since folder scanning is disabled
            let mut dummy_folders = Vec::new();
            scan_directory_with_root(&root_path, &root_path, &mut commands, &mut existing_commands, &mut handled_files, &mut dummy_folders, &existing_patches, &mut folder_map, config);
            let _commands_after_scan = commands.len();
            crate::utils::print(&format!("   ✅ Found {} new commands in {}", _commands_after_scan - _commands_before_scan, expanded_root));
        } else {
            crate::utils::print(&format!("   ⚠️ Directory not found: {}", expanded_root));
        }
    }
    
    // COMMENTED OUT: Process collected folders at the end - folder scanning disabled
    // detailed_log("SCANNER", &format!("Processing {} collected folders", found_folders.len()));
    // for folder_path in found_folders {
    //     if let Some(folder_name) = folder_path.file_name() {
    //         if let Some(name_str) = folder_name.to_str() {
    //             // Check if a command already exists with this name (case-insensitive)
    //             if !existing_commands.contains(&name_str.to_lowercase()) {
    //                 // Create folder command without "folder" suffix
    //                 let command_name = name_str.to_string();
    //                 let full_path = folder_path.to_string_lossy().to_string();
    //                 let full_line = format!("{} : folder {};", command_name, full_path);
    //                 
    //                 let folder_command = Command {
    //                     patch: String::new(),
    //                     command: command_name.clone(),
    //                     action: "folder".to_string(),
    //                     arg: full_path,
    //                     flags: String::new(),
    //                     full_line,
    //                 };
    //                 
    //                 commands.push(folder_command);
    //                 existing_commands.insert(command_name.to_lowercase());
    //                 if is_scanner_debug_enabled(config) {
    //                     detailed_log("SCANNER", &format!("Added folder command: {}", name_str));
    //                 }
    //             } else {
    //                 if is_scanner_debug_enabled(config) {
    //                     detailed_log("SCANNER", &format!("Skipping folder '{}' - command already exists", name_str));
    //                 }
    //             }
    //         }
    //     }
    // }
    
    // Count final results
    let _markdown_after = commands.iter().filter(|cmd| cmd.action == "markdown").count();
    let _anchor_after = commands.iter().filter(|cmd| cmd.is_anchor()).count();
    let _folder_after = commands.iter().filter(|cmd| cmd.action == "folder").count();
    let _app_after = commands.iter().filter(|cmd| cmd.action == "app" || cmd.action == "open_app").count();

    commands
}


/// Infer patch for a file by walking up its directory tree and looking in folder_map
fn infer_patch_from_folder_map(file_path: &Path, folder_map: &std::collections::HashMap<PathBuf, String>) -> String {
    let mut current = file_path.parent();

    while let Some(dir) = current {
        if let Ok(canonical) = dir.canonicalize() {
            if let Some(patch) = folder_map.get(&canonical) {
                crate::utils::detailed_log("PATCH_INFER", &format!("Inferred patch '{}' for '{}' from folder '{}'",
                    patch, file_path.display(), canonical.display()));
                return patch.clone();
            }
        }
        current = dir.parent();
    }

    // No parent folder found in map - assign to orphans
    crate::utils::detailed_log("PATCH_INFER", &format!("No folder match for '{}' - assigning to orphans", file_path.display()));
    "orphans".to_string()
}

/// Recursively scans a directory for files and folders with vault root tracking
fn scan_directory_with_root(dir: &Path, vault_root: &Path, commands: &mut Vec<Command>, existing_commands: &mut HashSet<String>, handled_files: &mut HashSet<String>, found_folders: &mut Vec<PathBuf>, existing_patches: &std::collections::HashMap<String, String>, folder_map: &mut std::collections::HashMap<PathBuf, String>, config: &Config) {
    scan_directory_with_root_protected(dir, vault_root, commands, existing_commands, handled_files, found_folders, existing_patches, folder_map, config, &mut HashSet::new(), 0)
}

/// Helper function for two-pass scanning
/// When anchor_only=true: Only process anchor markdown files, add them to folder_map
/// When anchor_only=false: Process all non-anchor files, skip anchors (already processed)
fn scan_directory_pass(dir: &Path, vault_root: &Path, commands: &mut Vec<Command>, existing_commands: &mut HashSet<String>, handled_files: &mut HashSet<String>, found_folders: &mut Vec<PathBuf>, existing_patches: &std::collections::HashMap<String, String>, folder_map: &mut std::collections::HashMap<PathBuf, String>, config: &Config, visited: &mut HashSet<PathBuf>, depth: usize, anchor_only: bool) {
    // Prevent infinite loops with depth limit
    if depth > 20 {
        return;
    }

    // Get canonical path to detect symbolic link loops
    let canonical_dir = match dir.canonicalize() {
        Ok(path) => path,
        Err(_) => {
            return;
        }
    };

    // Check if we've already visited this directory (prevents infinite loops)
    if visited.contains(&canonical_dir) {
        return;
    }
    visited.insert(canonical_dir.clone());

    // In Pass 1 (anchor_only=true), we need to process files BEFORE recursing into subdirectories
    // This ensures parent anchor files are found before child directories are scanned
    // Collect subdirectories to process after files
    let mut subdirs_to_scan: Vec<PathBuf> = Vec::new();

    // Scan directory entries
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.filter_map(Result::ok) {
            let path = entry.path();

            // Skip hidden files and directories (those starting with .)
            if let Some(file_name) = path.file_name() {
                if let Some(name_str) = file_name.to_str() {
                    if name_str.starts_with('.') {
                        continue;
                    }

                    // Skip directories based on config patterns
                    if path.is_dir() && should_skip_directory(&name_str, config) {
                        continue;
                    }
                } else {
                    // Log files with non-UTF8 names
                    crate::utils::detailed_log("SCANNER", &format!("Skipping non-UTF8 filename: {:?}", path));
                    continue;
                }
            }

            // Check if it's a symlink and skip directory symlinks to prevent infinite loops
            let metadata = match entry.metadata() {
                Ok(metadata) => metadata,
                Err(_) => continue, // Skip if we can't read metadata
            };

            // Process directories first (but skip directory symlinks)
            if path.is_dir() {
                // Skip symlinked directories to prevent infinite loops
                if metadata.file_type().is_symlink() {
                    continue;
                }

                // Check if this is a .app bundle
                if let Some(file_name) = path.file_name() {
                    if let Some(name_str) = file_name.to_str() {
                        if name_str.ends_with(".app") {
                            // Only process apps in non-anchor pass
                            if !anchor_only {
                                // This is an app bundle - create an APP command
                                if let Some(command) = process_app_bundle(&path, existing_commands, folder_map) {
                                    crate::utils::detailed_log("SCANNER", &format!("Found app bundle: {} -> command: {}",
                                        path.display(), command.command));

                                    // Check if this app is already handled
                                    if handled_files.contains(&command.arg) {
                                        crate::utils::detailed_log("SCANNER", &format!("Skipping '{}' - app already handled: {}",
                                            command.command, path.display()));
                                    } else {
                                        crate::utils::detailed_log("SCANNER", &format!("Found app: {}", command.command));
                                        crate::utils::detailed_log("SCANNER", &format!("Creating new APP command '{}' -> {} {}",
                                            command.command, command.action, command.arg));

                                        existing_commands.insert(command.command.to_lowercase());
                                        handled_files.insert(command.arg.clone());
                                        commands.push(command);
                                    }
                                }
                            }
                            // Don't scan inside .app bundles
                            continue;
                        }
                    }
                }

                // Recursively scan subdirectories (but not .app bundles)
                scan_directory_pass(&path, vault_root, commands, existing_commands, handled_files, found_folders, existing_patches, folder_map, config, visited, depth + 1, anchor_only);
            } else {
                // Process files (markdown files and DOC files)
                let mut file_processed = false;

                crate::utils::detailed_log("SCANNER", &format!("Processing file: {}", path.display()));

                // Try to process as markdown file first
                if let Some(command) = process_markdown_with_root(&path, vault_root, commands, existing_commands, handled_files, folder_map) {
                    // Check if this is an anchor based on the command's is_anchor() method
                    let is_anchor_file = command.is_anchor();

                    // Filter based on pass type
                    if (anchor_only && !is_anchor_file) || (!anchor_only && is_anchor_file) {
                        // Skip this file - wrong pass for this file type
                        continue;
                    }

                    // Always log found markdown files for debugging
                    crate::utils::log(&format!("📄 Found markdown: {} -> cmd: '{}' action: '{}'",
                        path.display(), command.command, command.action));

                    // Check if this file is already handled by an existing command (O(1) lookup)
                    if handled_files.contains(&command.arg) {
                        crate::utils::log(&format!("  ⏭️  Skipping '{}' - file already handled: {}",
                            command.command, path.display()));
                    } else {
                        // Log that we're creating a new command
                        crate::utils::log(&format!("  ✅ Creating new {} command '{}' -> {}",
                            command.action, command.command, command.arg));

                        // Add to existing commands set to prevent future collisions (lowercase)
                        existing_commands.insert(command.command.to_lowercase());
                        // Add to handled files set to prevent creating duplicate commands for this file
                        handled_files.insert(command.arg.clone());

                        // Debug: Log command details
                        crate::utils::log(&format!("   📋 Command details: cmd='{}' action='{}' flags='{}' is_anchor={}",
                            command.command, command.action, command.flags, command.is_anchor()));

                        // If this is an anchor markdown in a subdirectory matching its name, map that subdirectory
                        // Example: @Aeolus/@Aeolus.md should map @Aeolus/ -> @Aeolus
                        // But @BTOD.md (in At/Org/) should NOT map At/Org/ -> @BTOD
                        if command.is_anchor() {
                            if let Some(parent) = path.parent() {
                                if let Some(parent_dir_name) = parent.file_name().and_then(|n| n.to_str()) {
                                    crate::utils::log(&format!("   🔍 Checking folder_map: anchor='{}' parent_dir='{}'",
                                        command.command, parent_dir_name));
                                    // Only add to folder_map if the parent directory name matches the anchor name
                                    // This prevents files like @BTOD.md from mapping their entire parent directory
                                    if parent_dir_name == command.command {
                                        if let Ok(canonical_parent) = parent.canonicalize() {
                                            folder_map.insert(canonical_parent.clone(), command.command.clone());
                                            crate::utils::log(&format!("   🏷️  Added anchor to folder_map: '{}' -> '{}'",
                                                canonical_parent.display(), command.command));
                                        }
                                    } else {
                                        crate::utils::log(&format!("   ⏭️  Skipping folder_map for '{}' - parent dir '{}' doesn't match anchor name",
                                            command.command, parent_dir_name));
                                    }
                                }
                            }
                        }

                        commands.push(command);
                    }
                    file_processed = true;
                }

                // If not a markdown file and not anchor_only pass, try to process as DOC file
                if !file_processed && !anchor_only {
                    crate::utils::detailed_log("SCANNER", &format!("File not processed as markdown, trying DOC: {}", path.display()));
                    if let Some(command) = process_doc_file(&path, existing_commands, folder_map, config) {
                        // Always log found DOC files for debugging
                        // DOC command created successfully
                        crate::utils::log(&format!("📄 Found doc: {} -> cmd: '{}' action: '{}'",
                            path.display(), command.command, command.action));

                        // Check if this file is already handled by an existing command (O(1) lookup)
                        if handled_files.contains(&command.arg) {
                            crate::utils::log(&format!("  ⏭️  Skipping '{}' - file already handled: {}",
                                command.command, path.display()));
                        } else {
                            // Log that we're creating a new command
                            crate::utils::log(&format!("  ✅ Creating new {} command '{}' -> {}",
                                command.action, command.command, command.arg));

                            // Add to existing commands set to prevent future collisions (lowercase)
                            existing_commands.insert(command.command.to_lowercase());
                            // Add to handled files set to prevent creating duplicate commands for this file
                            handled_files.insert(command.arg.clone());
                            commands.push(command);
                        }
                    }
                }
            }
        }
    }
}

/// Protected version with loop detection and depth limiting
/// TWO-PASS PER DIRECTORY ALGORITHM:
/// At each directory level, loops through entries twice:
/// - Pass 1 (is_pass1=true): Process only anchor files, add to folder_map
/// - Pass 2 (is_pass1=false): Process non-anchor files, recurse into subdirectories
/// Uses boolean logic to skip inappropriate entries in each pass
fn scan_directory_with_root_protected(dir: &Path, vault_root: &Path, commands: &mut Vec<Command>, existing_commands: &mut HashSet<String>, handled_files: &mut HashSet<String>, found_folders: &mut Vec<PathBuf>, existing_patches: &std::collections::HashMap<String, String>, folder_map: &mut std::collections::HashMap<PathBuf, String>, config: &Config, visited: &mut HashSet<PathBuf>, depth: usize) {
    // Prevent infinite loops with depth limit
    if depth > 20 {
        return;
    }

    // Get canonical path to detect symbolic link loops
    let canonical_dir = match dir.canonicalize() {
        Ok(path) => path,
        Err(_) => {
            return;
        }
    };

    // Check if we've already visited this directory (prevents infinite loops)
    if visited.contains(&canonical_dir) {
        return;
    }
    visited.insert(canonical_dir.clone());

    // Collect all entries in this directory once
    let entries: Vec<_> = match fs::read_dir(dir) {
        Ok(entries) => entries.filter_map(Result::ok).collect(),
        Err(_) => return,
    };

    // Two passes over the same entries
    for is_pass1 in [true, false] {
        for entry in &entries {
            let path = entry.path();

            // Skip hidden files
            if let Some(file_name) = path.file_name() {
                if let Some(name_str) = file_name.to_str() {
                    if name_str.starts_with('.') {
                        continue;
                    }
                    // Skip directories based on config patterns
                    if path.is_dir() && should_skip_directory(&name_str, config) {
                        continue;
                    }
                }
            }

            // Get metadata
            let metadata = match entry.metadata() {
                Ok(m) => m,
                Err(_) => continue,
            };

            // Handle directories
            if path.is_dir() {
                // Skip symlinked directories
                if metadata.file_type().is_symlink() {
                    continue;
                }

                // Handle .app bundles (only in Pass 2)
                if let Some(file_name) = path.file_name() {
                    if let Some(name_str) = file_name.to_str() {
                        if name_str.ends_with(".app") {
                            if !is_pass1 {
                                if let Some(command) = process_app_bundle(&path, existing_commands, folder_map) {
                                    if !handled_files.contains(&command.arg) {
                                        crate::utils::detailed_log("SCANNER", &format!("Found app: {}", command.command));
                                        existing_commands.insert(command.command.to_lowercase());
                                        handled_files.insert(command.arg.clone());
                                        commands.push(command);
                                    }
                                }
                            }
                            continue; // Don't recurse into .app bundles
                        }
                    }
                }

                // Recurse into subdirectories only in Pass 2 (after all anchors in current dir are mapped)
                if !is_pass1 {
                    scan_directory_with_root_protected(&path, vault_root, commands, existing_commands, handled_files, found_folders, existing_patches, folder_map, config, visited, depth + 1);
                }
            } else {
                // Process files (markdown and DOC)

                // Check if this is an anchor file using the specialized function
                let is_anchor_file = crate::utils::is_anchor_file(&path);

                // Skip if wrong pass for this file type
                // Pass 1: only anchor files; Pass 2: only non-anchor files
                if (is_pass1 && !is_anchor_file) || (!is_pass1 && is_anchor_file) {
                    continue;
                }

                // Try to process as markdown file
                if let Some(command) = process_markdown_with_root(&path, vault_root, commands, existing_commands, handled_files, folder_map) {
                    if !handled_files.contains(&command.arg) {
                        if is_pass1 {
                            crate::utils::detailed_log("SCANNER", &format!("PASS1: Found anchor: {} -> cmd: '{}' patch: '{}'", path.display(), command.command, command.patch));
                        } else {
                            crate::utils::detailed_log("SCANNER", &format!("PASS2: Found markdown: {} -> cmd: '{}' patch: '{}'", path.display(), command.command, command.patch));
                        }

                        existing_commands.insert(command.command.to_lowercase());
                        handled_files.insert(command.arg.clone());

                        // If this is an anchor, add to folder_map using the specialized anchor detection
                        if is_anchor_file {
                            if let Some(parent) = path.parent() {
                                // Use case-insensitive comparison like is_anchor_file does
                                if let Some(parent_dir_name) = parent.file_name().and_then(|n| n.to_str()) {
                                    if parent_dir_name.eq_ignore_ascii_case(&command.command) {
                                        if let Ok(canonical_parent) = parent.canonicalize() {
                                            folder_map.insert(canonical_parent.clone(), command.command.clone());
                                            crate::utils::detailed_log("SCANNER", &format!("Added to folder_map: '{}' -> '{}'", canonical_parent.display(), command.command));
                                        }
                                    }
                                }
                            }
                        }

                        commands.push(command);
                    }
                } else if !is_pass1 {
                    // Try DOC file (only in Pass 2)
                    if let Some(command) = process_doc_file(&path, existing_commands, folder_map, config) {
                        if !handled_files.contains(&command.arg) {
                            crate::utils::detailed_log("SCANNER", &format!("PASS2: Found doc: {} -> cmd: '{}'", path.display(), command.command));
                            existing_commands.insert(command.command.to_lowercase());
                            handled_files.insert(command.arg.clone());
                            commands.push(command);
                        }
                    }
                }
            }
        }
    }
}


/// Processes an .app bundle and returns an APP command
fn process_app_bundle(path: &Path, existing_commands: &HashSet<String>, folder_map: &std::collections::HashMap<PathBuf, String>) -> Option<Command> {
    // Get the app name from the path
    let app_name_with_ext = path.file_name()?.to_str()?;
    
    // Remove the .app extension to get the base name
    let app_name = if app_name_with_ext.ends_with(".app") {
        &app_name_with_ext[..app_name_with_ext.len() - 4]
    } else {
        return None;
    };
    
    // Create command name with "App" suffix
    let preferred_name = format!("{} App", app_name);
    
    // Check for name collisions
    let command_name = if existing_commands.contains(&preferred_name.to_lowercase()) {
        // If collision exists, add a number suffix
        let mut counter = 2;
        loop {
            let numbered_name = format!("{} App {}", app_name, counter);
            if !existing_commands.contains(&numbered_name.to_lowercase()) {
                break numbered_name;
            }
            counter += 1;
        }
    } else {
        preferred_name
    };
    
    let full_path = path.to_string_lossy().to_string();

    // Infer patch from folder hierarchy
    let patch = infer_patch_from_folder_map(path, folder_map);

    Some(Command {
        patch,
        command: command_name,
        action: "app".to_string(),
        arg: full_path,
        flags: String::new(),
        other_params: None,
        last_update: 0,
        file_size: None,
    })
}

/// Processes a path and returns a command if it's a markdown file with vault root for relative paths
fn process_markdown_with_root(path: &Path, _vault_root: &Path, commands: &mut Vec<Command>, existing_commands: &HashSet<String>, handled_files: &HashSet<String>, folder_map: &std::collections::HashMap<PathBuf, String>) -> Option<Command> {
    // Check if it's a file and has .md extension
    if !path.is_file() {
        return None;
    }

    let extension = path.extension()?.to_str()?;
    if extension.to_lowercase() != "md" {
        return None;
    }

    // Check if this file is already handled by an existing command
    let full_path = path.to_string_lossy().to_string();
    if handled_files.contains(&full_path) {
        crate::utils::detailed_log("SCANNER", &format!("Skipping {} - already handled by existing command", full_path));
        return None;
    }

    // Get the base name without extension
    let file_name = match path.file_stem()?.to_str() {
        Some(name) => name,
        None => {
            crate::utils::detailed_log("SCANNER", &format!("Skipping file with invalid UTF-8 name: {}", path.display()));
            return None;
        }
    };

    // Check if this is an anchor file (filename matches parent folder name)
    let is_anchor = crate::utils::is_anchor_file(path);

    // Always use "markdown" action for .md files (anchor status is indicated by 'a' flag)
    let action = "markdown";

    // Create command name without suffix, but check for collisions (case-insensitive)
    let preferred_name = file_name.to_string();

    // Special handling for files ending with "App" - always add " Markdown" suffix
    // This prevents collision with actual .app commands
    let command_name = if file_name.ends_with(" App") || file_name.ends_with("App") {
        // For App markdown files, always use " Markdown" suffix to avoid confusion with actual apps
        format!("{} Markdown", file_name)
    } else if existing_commands.contains(&preferred_name.to_lowercase()) {
        // Check if this is an anchor file with a collision
        if is_anchor {
            // Find the conflicting user command and add 'a' flag to it
            for cmd in commands.iter_mut() {
                if cmd.command.eq_ignore_ascii_case(&preferred_name) && cmd.flags.contains(FLAG_USER_EDITED) {
                    cmd.set_flag(FLAG_ANCHOR, "");
                    crate::utils::log(&format!("🏷️  Added anchor flag to user command '{}' (patch: {})", cmd.command, cmd.patch));
                    return None; // Don't create duplicate - user's command is now the anchor
                }
            }
            // If we get here, collision wasn't with user command
            // Anchor files always use their natural name, even with collisions
            crate::utils::log(&format!("🎯 Creating anchor '{}' despite collision (non-user command)", preferred_name));
            preferred_name
        } else {
            // Non-anchor with collision, add suffix
            format!("{} markdown", file_name)
        }
    } else {
        // Use the preferred name without suffix
        preferred_name
    };
    
    let full_path = path.to_string_lossy().to_string();

    // All markdown files use absolute path as argument
    let arg = full_path.clone();

    // Set the anchor flag if this is an anchor file
    let flags = if is_anchor {
        crate::core::commands::FLAG_ANCHOR.to_string()
    } else {
        String::new()
    };

    // Determine the patch for this command using folder_map
    // This assigns patches DURING scanning based on directory hierarchy
    let patch = infer_patch_from_folder_map(path, folder_map);

    Some(Command {
        patch,
        command: command_name,
        action: action.to_string(),
        arg,
        flags,
        other_params: None,
        last_update: 0,
        file_size: None,
    })
}

/// Processes a path and returns a DOC command if it matches the configured file extensions
fn process_doc_file(path: &Path, existing_commands: &HashSet<String>, folder_map: &std::collections::HashMap<PathBuf, String>, config: &Config) -> Option<Command> {
    // Check if it's a file
    if !path.is_file() {
        return None;
    }
    
    // Get file extension
    let extension = path.extension()?.to_str()?;
    let extension_lower = extension.to_lowercase();
    
    // Get the whitelist of DOC file extensions from config
    let doc_extensions = match &config.popup_settings.doc_file_extensions {
        Some(exts) => {
            crate::utils::detailed_log("SCANNER", &format!("DOC extensions whitelist: {}", exts));
            exts
        },
        None => {
            crate::utils::detailed_log("SCANNER", "No DOC extensions configured in config");
            return None; // No DOC extensions configured
        }
    };
    
    // Check if this file extension is in the whitelist
    let is_doc_extension = doc_extensions
        .split(',')
        .map(|ext| ext.trim().to_lowercase())
        .any(|ext| ext == extension_lower);
    
    crate::utils::detailed_log("SCANNER", &format!("File '{}' extension '{}' in whitelist: {}", path.display(), extension_lower, is_doc_extension));
    
    if !is_doc_extension {
        return None;
    }
    
    // Get the base name without extension
    let file_name = match path.file_stem()?.to_str() {
        Some(name) => name,
        None => {
            crate::utils::detailed_log("SCANNER", &format!("Skipping file with invalid UTF-8 name: {}", path.display()));
            return None;
        }
    };
    
    // Create command name, checking for collisions (case-insensitive)
    let preferred_name = file_name.to_string();
    
    let command_name = if existing_commands.contains(&preferred_name.to_lowercase()) {
        // If collision exists, add extension suffix to distinguish from other files
        format!("{} {}", file_name, extension_lower)
    } else {
        // Use the preferred name without suffix
        preferred_name
    };
    
    let full_path = path.to_string_lossy().to_string();

    // Infer patch from folder hierarchy
    let patch = infer_patch_from_folder_map(path, folder_map);

    Some(Command {
        patch,
        command: command_name,
        action: "doc".to_string(),
        arg: full_path,
        flags: String::new(),
        other_params: None,
        last_update: 0,
        file_size: None,
    })
}


/// Check if a directory should be skipped based on config patterns
fn should_skip_directory(dir_name: &str, config: &Config) -> bool {
    // Get skip patterns from config
    let skip_patterns = match &config.popup_settings.skip_directory_patterns {
        Some(patterns) => patterns,
        None => return false,
    };
    
    // Check each pattern
    for pattern in skip_patterns {
        // Simple glob pattern matching
        if pattern.contains('*') {
            // Convert glob pattern to simple regex-like matching
            let pattern_lower = pattern.to_lowercase();
            let dir_lower = dir_name.to_lowercase();
            
            // Handle patterns like "*trash*"
            if pattern_lower.starts_with('*') && pattern_lower.ends_with('*') {
                let inner = &pattern_lower[1..pattern_lower.len()-1];
                if dir_lower.contains(inner) {
                    return true;
                }
            }
            // Handle patterns like "*.Trash*" 
            else if pattern_lower.contains("*.") {
                // For now, treat as contains match after removing asterisks
                let cleaned = pattern_lower.replace('*', "");
                if dir_lower.contains(&cleaned) {
                    return true;
                }
            }
            // Handle other patterns
            else {
                let cleaned = pattern_lower.replace('*', "");
                if dir_lower.contains(&cleaned) {
                    return true;
                }
            }
        } else {
            // Exact match (case-insensitive)
            if dir_name.eq_ignore_ascii_case(pattern) {
                return true;
            }
        }
    }
    
    false
}

/// Expands ~ to home directory
fn expand_home(path: &str) -> String {
    if path.starts_with("~/") {
        if let Ok(home) = std::env::var("HOME") {
            return path.replacen("~", &home, 1);
        }
    }
    path.to_string()
}

/// Calculates a checksum from the scan results to detect changes
fn calculate_commands_checksum(commands: &[Command]) -> String {
    let mut hasher = DefaultHasher::new();
    
    // Filter only scan-generated commands (markdown, anchor, contact, folder, app, open_app, doc)
    let mut scan_commands: Vec<_> = commands.iter()
        .filter(|cmd| cmd.action == "markdown" || cmd.is_anchor() || cmd.action == "contact" || cmd.action == "folder" || cmd.action == "app" || cmd.action == "open_app" || cmd.action == "doc")
        .collect();
    
    // Sort for consistent checksum - use to_new_format() for comparison
    scan_commands.sort_by(|a, b| a.to_new_format().cmp(&b.to_new_format()));
    
    // Hash all scan-generated command lines
    for cmd in scan_commands {
        cmd.to_new_format().hash(&mut hasher);
    }
    
    format!("{:x}", hasher.finish())
}

// PLEASE KEEP - Expected to integrate later for contact scanning functionality
/// Scans macOS contacts and creates commands for contacts with phone or email
fn scan_contacts(mut commands: Vec<Command>) -> Vec<Command> {
    // First, remove all existing contact commands
    commands.retain(|cmd| cmd.action != "contact");
    
    // Use simple, fast AppleScript to get first 10 contacts for testing
    let script = r#"
tell application "Contacts"
    set output to ""
    set maxContacts to 10
    
    repeat with i from 1 to maxContacts
        try
            set p to person i
            set fullName to name of p
            set contactId to id of p
            set output to output & fullName & "|" & contactId & "\n"
        on error
            -- Skip contacts that cause errors
        end try
    end repeat
    
    return output
end tell
"#;
    
    // Execute the AppleScript with timeout
    let output = ProcessCommand::new("timeout")
        .arg("10")  // 10 second timeout
        .arg("osascript")
        .arg("-e")
        .arg(script)
        .output();
    
    if let Ok(result) = output {
        if result.status.success() {
            let contacts_str = String::from_utf8_lossy(&result.stdout);
            
            // Parse each line
            for line in contacts_str.lines() {
                if line.is_empty() {
                    continue;
                }
                
                // Split by pipe to get name and ID
                let parts: Vec<&str> = line.split('|').collect();
                if parts.len() == 2 {
                    let full_name = parts[0].trim();
                    let contact_id = parts[1].trim();
                    
                    // Create command name: @FirstName LastName ctc
                    let command_name = format!("@{} ctc", full_name);
                    
                    commands.push(Command {
                        patch: String::new(),
                        command: command_name,
                        action: "contact".to_string(),
                        arg: contact_id.to_string(),
                        flags: String::new(),
                        other_params: None,
                        last_update: 0,
                        file_size: None,
                    });
                }
            }
        } else {
            // If contacts scanning fails, fall back to sample data
            let sample_contacts = vec![
                ("John Doe", "12345"),
                ("Jane Smith", "67890"), 
                ("Bob Johnson", "11111"),
            ];
            
            for (name, id) in sample_contacts {
                let command_name = format!("@{} ctc", name);
                commands.push(Command {
                    patch: String::new(),
                    command: command_name,
                    action: "contact".to_string(),
                    arg: id.to_string(),
                    flags: String::new(),
                    other_params: None,
                    last_update: 0,
                    file_size: None,
                });
            }
        }
    }
    
    commands
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use crate::utils::is_anchor_file;

    #[test]
    fn test_is_anchor_file() {
        // Test case: ProjectName/ProjectName.md (should be anchor)
        let path = PathBuf::from("/home/user/ProjectName/ProjectName.md");
        assert!(is_anchor_file(&path));
        
        // Test case: ProjectName/readme.md (should not be anchor)
        let path = PathBuf::from("/home/user/ProjectName/readme.md");
        assert!(!is_anchor_file(&path));
        
        // Test case: projectname/PROJECTNAME.md (should be anchor - case insensitive)
        let path = PathBuf::from("/home/user/projectname/PROJECTNAME.md");
        assert!(is_anchor_file(&path));
    }
}
