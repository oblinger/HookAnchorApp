//! # File Scanner Module
//! 
//! This module handles filesystem scanning to discover and create commands for markdown files,
//! applications, and cloud resources (Notion). The scanner maintains existing user edits while
//! refreshing automatically generated content.
//!
//! ## Scanning Process Overview
//! 
//! ### Phase 1: Initialization
//! 1. **Preserve existing patches**: Build a map of command→patch from existing commands
//! 2. **Delete old anchors**: Remove non-Notion anchors (unless user-edited with 'U' flag)
//!    - Special anchors like "orphans" and "Notion Root" are preserved
//!    - This allows anchors to be recreated with fresh data
//! 3. **Clean up scanner commands**: Remove scanner-generated commands (markdown/folder/app)
//!    - Only for files within scan roots
//!    - Preserves user-edited commands ('U' flag)
//!    - Removes commands for non-existent files
//!
//! ### Phase 2: Build Tracking Sets (AFTER deletion)
//! 1. **existing_commands**: Set of command names (lowercase) to prevent name collisions
//! 2. **handled_files**: Set of file paths from remaining commands
//!    - Built AFTER deletion so deleted anchors can be recreated
//!    - Skips empty args (commands without files)
//!    - Prevents creating duplicate commands for the same file
//!
//! ### Phase 3: Filesystem Scan
//! For each configured file root (e.g., ~/ob/kmr, /Applications):
//! 1. **Process .app bundles**: Create app commands with "App" suffix
//! 2. **Process markdown files**:
//!    - Determine if it's an anchor (filename matches parent folder) or regular markdown
//!    - Skip if file already in handled_files
//!    - Skip if command name already in existing_commands
//!    - Preserve patch from existing_patches if available
//! 3. **Recursively scan subdirectories**
//!
//! ### Phase 4: Cloud Services
//! 1. **Notion scanning**: If enabled, fetch pages and create anchor commands
//!    - All Notion pages become anchors under "Notion Root" patch
//!
//! ### Phase 5: Post-Processing (scan_verbose only)
//! 1. Run full inference pipeline via load_data():
//!    - Create patches hashmap from anchors
//!    - Run patch inference (assigns patches to commands with empty patches)
//!    - Normalize patch case
//! 2. Save final commands to commands.txt
//!
//! ## Key Design Decisions
//! 
//! - **handled_files built AFTER deletion**: Allows recreation of deleted anchors
//! - **Skip empty args**: Empty args don't represent files, shouldn't block other commands
//! - **User edits preserved**: 'U' flag prevents automatic deletion/modification
//! - **Patch preservation**: Existing patches are preserved when regenerating commands

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command as ProcessCommand;
use std::collections::{hash_map::DefaultHasher, HashSet};
use std::hash::{Hash, Hasher};
use crate::core::Command;
use crate::core::commands::save_commands_to_file;
use crate::core::{Config, load_state, save_state};
use crate::utils::detailed_log;
use crate::execute::get_action;
use chrono::Local;

/// Action types that are automatically generated and removed by the scanner
/// These commands will be removed during rescanning unless they have the 'U' (user-edited) flag
/// NOTE: "anchor" is handled separately by delete_anchors() function
pub const SCANNER_GENERATED_ACTIONS: &[&str] = &["markdown", "folder", "app", "open_app", "doc"];


/// Check if a command is a Notion anchor
/// Simple check that doesn't require accessing the actions module
fn is_notion_anchor(cmd: &Command) -> bool {
    cmd.action == "anchor" && 
    (cmd.arg.contains("notion.so") || cmd.patch == "Notion Root")
}

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
        if cmd.flags.contains('U') {
            return true;
        }
        
        // Always keep special system anchors
        if cmd.command == "orphans" || cmd.command == "Notion Root" {
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
            println!("   Removed {} non-user-edited Notion anchor commands", deleted);
        } else {
            println!("   Removed {} non-user-edited file anchor commands", deleted);
        }
    }
    deleted
}

/// Main function for automatic background scanning
/// Checks if filesystem scan should be performed and executes it if needed.
/// This function should be called on application exit, not startup.
pub fn scan_check(commands: Vec<Command>) -> Vec<Command> {
    let (sys_data, _) = crate::core::sys_data::get_sys_data();
    let mut state = load_state();
    
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
    if let Err(e) = save_state(&state) {
        crate::utils::log_error(&format!("Failed to save scan state: {}", e));
    }
    
    // Save commands only if checksum changed
    if checksum_changed {
        // Save the scanned commands directly - patch inference should already be handled by load_data()
        // The commands passed to the scanner should already have proper patches from load_data()
        if let Err(e) = save_commands_to_file(&scanned_commands) {
            crate::utils::log_error(&format!("Failed to save updated commands: {}", e));
        } else {
            // Mark commands as modified since we've updated the commands file
            crate::core::mark_commands_modified();
        }
        
        
        // Return the original scanned commands (should already have patches from load_data())
        scanned_commands
    } else {
        // No changes, return original commands
        scanned_commands
    }
}

/// Internal scan function that orchestrates all scanning operations
fn scan(commands: Vec<Command>, sys_data: &crate::core::sys_data::SysData) -> Vec<Command> {
    scan_verbose(commands, sys_data, false)
}

/// Manual CLI rescanning with verbose output
/// Used for the --rescan command line option
pub fn scan_verbose(commands: Vec<Command>, sys_data: &crate::core::sys_data::SysData, verbose: bool) -> Vec<Command> {
    let empty_vec = vec![];
    let file_roots = sys_data.config.popup_settings.file_roots.as_ref().unwrap_or(&empty_vec);
    
    if verbose {
        println!("\n🔍 Starting filesystem scan...");
        println!("   Scanning roots: {:?}", file_roots);
    }
    
    // First scan files (markdown and apps)
    let initial_count = commands.len();
    let mut commands = scan_files(commands, file_roots, &sys_data.config);
    
    if verbose {
        let files_added = commands.len().saturating_sub(initial_count);
        println!("   Scan complete: {} commands total", commands.len());
        if files_added > 0 {
            println!("   Added {} new commands", files_added);
        }
    }
    
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
                println!("   Incremental Notion scan - preserving existing commands");
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
        let notion_root_exists = commands.iter().any(|c| c.command == "Notion Root" && c.action == "anchor");
        if !notion_root_exists {
            commands.push(Command {
                command: "Notion Root".to_string(),
                action: "anchor".to_string(),
                arg: String::new(),  // Virtual anchor, no actual file
                flags: String::new(),
                patch: "orphans".to_string(),  // Notion Root is under orphans
        last_update: 0,
        file_size: None,
            });
            if verbose {
                println!("   Created Notion Root anchor");
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
            (cmd.action == "anchor" || cmd.action == "notion")
        ) {
            // Update existing command to be an anchor if it was "notion"
            if existing_cmd.action == "notion" {
                existing_cmd.action = "anchor".to_string();
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
                action: "anchor".to_string(),  // All Notion pages are anchors
                arg: page.url,                  // URL as the arg
                flags: String::new(),
                patch: parent_patch,             // Parent page as the patch
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
                println!("   Added {} new Notion anchor commands", notion_added);
            }
            if notion_updated > 0 {
                println!("   Updated {} existing Notion anchor commands", notion_updated);
            }
        }
    } // End of Notion pages processing
    
    // Then scan contacts - DISABLED for performance
    // commands = scan_contacts(commands);
    
    // Patches without anchors will simply remain without anchors
    
    // Run full inference pipeline on scanned commands to ensure consistency
    if verbose {
        println!("\n🔄 Running inference pipeline on scanned commands...");
    }
    let global_data = crate::core::sys_data::load_data(commands, verbose);
    
    // Save the processed commands to ensure persistence
    if verbose {
        println!("\n💾 Saving processed commands...");
    }
    
    if let Err(e) = crate::core::commands::save_commands_to_file(&global_data.commands) {
        crate::utils::log_error(&format!("Failed to save commands after scan: {}", e));
        if verbose {
            println!("   ❌ Failed to save: {}", e);
        }
    } else {
        if verbose {
            println!("   ✅ Saved {} commands to disk", global_data.commands.len());
        }
    }
    
    if verbose {
        println!("\n🎉 Scan and inference complete!");
    }
    
    // Return the fully processed commands with all inference completed
    global_data.commands
}

/// Scans the configured file roots and returns an updated command list
fn scan_files(mut commands: Vec<Command>, file_roots: &[String], config: &Config) -> Vec<Command> {
    
    // Create a lookup map to preserve existing patches when regenerating commands
    let mut existing_patches: std::collections::HashMap<String, String> = std::collections::HashMap::new();
    for cmd in &commands {
        if (cmd.action == "markdown" || cmd.action == "anchor" || cmd.action == "folder" || cmd.action == "app" || cmd.action == "open_app" || cmd.action == "doc") && !cmd.patch.is_empty() {
            existing_patches.insert(cmd.command.clone(), cmd.patch.clone());
        }
    }
    
    // Count commands before removal
    let _markdown_before = commands.iter().filter(|cmd| cmd.action == "markdown").count();
    let _anchor_before = commands.iter().filter(|cmd| cmd.action == "anchor").count();
    let _folder_before = commands.iter().filter(|cmd| cmd.action == "folder").count();
    let _app_before = commands.iter().filter(|cmd| cmd.action == "app" || cmd.action == "open_app").count();
    let _user_edited_scanner_commands = commands.iter()
        .filter(|cmd| SCANNER_GENERATED_ACTIONS.contains(&cmd.action.as_str()) && cmd.flags.contains('U'))
        .count();
    
    // Check if a path is within any of the scan roots
    let is_within_scan_roots = |path: &str| -> bool {
        for root in file_roots {
            let expanded_root = expand_home(root);
            if path.starts_with(&expanded_root) {
                return true;
            }
        }
        false
    };
    
    // Delete non-Notion file anchors before rescanning files
    delete_anchors(&mut commands, false, true);  // false = delete non-Notion anchors, true = verbose output for debugging
    
    // Remove existing markdown and folder commands from the commands list
    // BUT ONLY for files within the directories we're about to scan
    // EXCEPT those with the "U" (user edited) flag - preserve those
    // Also validate that files actually exist for scanner-generated commands
    commands.retain(|cmd| {
        let is_scanner_generated = SCANNER_GENERATED_ACTIONS.contains(&cmd.action.as_str());
        let is_user_edited = cmd.flags.contains('U');
        
        // If it's scanner-generated, check if we should keep or remove it
        if is_scanner_generated {
            // Check if file exists (for cleanup of missing file commands)
            let file_exists = std::path::Path::new(&cmd.arg).exists();
            
            // Log file existence checks only in detailed/verbose mode
            crate::utils::detailed_log("SCANNER", &format!("Checking '{}' (action: {}) - file exists: {} at path: {}", 
                cmd.command, cmd.action, file_exists, cmd.arg));
            
            if !file_exists {
                // This is important enough to always log
                crate::utils::detailed_log("SCANNER", &format!("SCANNER: ✅ Removing command '{}' - file no longer exists: {}", cmd.command, cmd.arg));
                return false; // Remove this command
            }
            
            // If the file is within scan roots and not user-edited, remove it for rescan
            if is_within_scan_roots(&cmd.arg) && !is_user_edited {
                crate::utils::detailed_log("SCANNER", &format!("Removing command '{}' for rescan (within scan roots)", cmd.command));
                return false; // Remove this command so it can be rescanned
            }
            
            // Keep commands that are:
            // 1. User-edited, OR
            // 2. Outside the scan roots (won't be rescanned)
            crate::utils::detailed_log("SCANNER", &format!("Keeping command '{}' (user-edited: {}, outside roots: {})", 
                cmd.command, is_user_edited, !is_within_scan_roots(&cmd.arg)));
            return true;
        }
        
        // Keep command if it's NOT scanner-generated
        true
    });
    
    let _preserved_user_edited = commands.iter()
        .filter(|cmd| SCANNER_GENERATED_ACTIONS.contains(&cmd.action.as_str()) && cmd.flags.contains('U'))
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
        if !cmd.arg.is_empty() && (cmd.action == "markdown" || cmd.action == "anchor" || cmd.action == "folder" || cmd.action == "app" || cmd.action == "open_app" || cmd.action == "doc") {
            handled_files.insert(cmd.arg.clone());
        }
    }
    
    // Collect folders during scanning
    // COMMENTED OUT: Folder scanning disabled for now - only creating commands for markdown files
    // let mut found_folders: Vec<PathBuf> = Vec::new();
    
    // Then scan for new files (markdown, folders, and apps)
    for root in file_roots {
        let expanded_root = expand_home(root);
        let root_path = Path::new(&expanded_root);
        
        println!("📂 Scanning directory: {}", expanded_root);
        
        if root_path.exists() && root_path.is_dir() {
            let _commands_before_scan = commands.len();
            // COMMENTED OUT: Passing empty Vec instead of found_folders since folder scanning is disabled
            let mut dummy_folders = Vec::new();
            scan_directory_with_root(&root_path, &root_path, &mut commands, &mut existing_commands, &mut handled_files, &mut dummy_folders, &existing_patches, config);
            let _commands_after_scan = commands.len();
            println!("   ✅ Found {} new commands in {}", _commands_after_scan - _commands_before_scan, expanded_root);
        } else {
            println!("   ⚠️ Directory not found: {}", expanded_root);
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
    let _anchor_after = commands.iter().filter(|cmd| cmd.action == "anchor").count();
    let _folder_after = commands.iter().filter(|cmd| cmd.action == "folder").count();
    let _app_after = commands.iter().filter(|cmd| cmd.action == "app" || cmd.action == "open_app").count();

    commands
}


/// Recursively scans a directory for files and folders with vault root tracking
fn scan_directory_with_root(dir: &Path, vault_root: &Path, commands: &mut Vec<Command>, existing_commands: &mut HashSet<String>, handled_files: &mut HashSet<String>, found_folders: &mut Vec<PathBuf>, existing_patches: &std::collections::HashMap<String, String>, config: &Config) {
    scan_directory_with_root_protected(dir, vault_root, commands, existing_commands, handled_files, found_folders, existing_patches, config, &mut HashSet::new(), 0)
}

/// Protected version with loop detection and depth limiting
fn scan_directory_with_root_protected(dir: &Path, vault_root: &Path, commands: &mut Vec<Command>, existing_commands: &mut HashSet<String>, handled_files: &mut HashSet<String>, found_folders: &mut Vec<PathBuf>, existing_patches: &std::collections::HashMap<String, String>, config: &Config, visited: &mut HashSet<PathBuf>, depth: usize) {
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
                            // This is an app bundle - create an APP command
                            if let Some(command) = process_app_bundle(&path, existing_commands, existing_patches) {
                                crate::utils::detailed_log("SCANNER", &format!("Found app bundle: {} -> command: {}", 
                                    path.display(), command.command));
                                
                                // Check if this app is already handled
                                if handled_files.contains(&command.arg) {
                                    crate::utils::detailed_log("SCANNER", &format!("Skipping '{}' - app already handled: {}", 
                                        command.command, path.display()));
                                } else {
                                    println!("      🎯 Found app: {}", command.command);
                                    crate::utils::detailed_log("SCANNER", &format!("Creating new APP command '{}' -> {} {}", 
                                        command.command, command.action, command.arg));
                                    
                                    existing_commands.insert(command.command.to_lowercase());
                                    handled_files.insert(command.arg.clone());
                                    commands.push(command);
                                }
                            }
                            // Don't scan inside .app bundles
                            continue;
                        }
                    }
                }
                
                // COMMENTED OUT: Folder collection disabled - only scanning for markdown files
                // found_folders.push(path.clone());
                
                // Recursively scan subdirectories (but not .app bundles)
                scan_directory_with_root_protected(&path, vault_root, commands, existing_commands, handled_files, found_folders, existing_patches, config, visited, depth + 1);
            } else {
                // Process files (markdown files and DOC files)
                let mut file_processed = false;
                
                crate::utils::detailed_log("SCANNER", &format!("Processing file: {}", path.display()));
                
                // Try to process as markdown file first
                if let Some(command) = process_markdown_with_root(&path, vault_root, existing_commands, &existing_patches) {
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
                        commands.push(command);
                    }
                    file_processed = true;
                }
                
                // If not a markdown file, try to process as DOC file
                if !file_processed {
                    crate::utils::detailed_log("SCANNER", &format!("File not processed as markdown, trying DOC: {}", path.display()));
                    if let Some(command) = process_doc_file(&path, existing_commands, &existing_patches, config) {
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


/// Processes an .app bundle and returns an APP command
fn process_app_bundle(path: &Path, existing_commands: &HashSet<String>, existing_patches: &std::collections::HashMap<String, String>) -> Option<Command> {
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
    
    // Try to preserve existing patch if this command existed before
    let preserved_patch = existing_patches.get(&command_name).cloned().unwrap_or_else(|| "App".to_string());
    
    Some(Command {
        patch: preserved_patch,
        command: command_name,
        action: "open_app".to_string(),
        arg: full_path,
        flags: String::new(),
        last_update: 0,
        file_size: None,
    })
}

/// Processes a path and returns a command if it's a markdown file with vault root for relative paths
fn process_markdown_with_root(path: &Path, _vault_root: &Path, existing_commands: &HashSet<String>, existing_patches: &std::collections::HashMap<String, String>) -> Option<Command> {
    // Check if it's a file and has .md extension
    if !path.is_file() {
        return None;
    }
    
    let extension = path.extension()?.to_str()?;
    if extension.to_lowercase() != "md" {
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
    
    // Determine action type using the shared get_action function
    let action = get_action(path);
    
    // Create command name without suffix, but check for collisions (case-insensitive)
    let preferred_name = file_name.to_string();
    
    // Special handling for files ending with "App" - always add " Markdown" suffix
    // This prevents collision with actual .app commands
    let command_name = if file_name.ends_with(" App") || file_name.ends_with("App") {
        // For App markdown files, always use " Markdown" suffix to avoid confusion with actual apps
        format!("{} Markdown", file_name)
    } else if existing_commands.contains(&preferred_name.to_lowercase()) {
        // If collision exists, use " markdown" suffix
        format!("{} markdown", file_name)
    } else {
        // Use the preferred name without suffix
        preferred_name
    };
    
    let full_path = path.to_string_lossy().to_string();
    
    // Calculate the argument based on action type
    let arg = if action == "markdown" {
        // For markdown entries, use absolute path (new unified approach)
        full_path.clone()
    } else {
        // For anchor entries, use full path as before
        full_path.clone()
    };
    
    // Try to preserve existing patch if this command existed before
    let preserved_patch = existing_patches.get(&command_name).cloned().unwrap_or_else(String::new);
    
    Some(Command {
        patch: preserved_patch,
        command: command_name,
        action: action.to_string(),
        arg,
        flags: String::new(),
        last_update: 0,
        file_size: None,
    })
}

/// Processes a path and returns a DOC command if it matches the configured file extensions
fn process_doc_file(path: &Path, existing_commands: &HashSet<String>, existing_patches: &std::collections::HashMap<String, String>, config: &Config) -> Option<Command> {
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
    
    // Try to preserve existing patch if this command existed before
    let preserved_patch = existing_patches.get(&command_name).cloned().unwrap_or_else(String::new);
    
    Some(Command {
        patch: preserved_patch,
        command: command_name,
        action: "doc".to_string(),
        arg: full_path,
        flags: String::new(),
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
        .filter(|cmd| cmd.action == "markdown" || cmd.action == "anchor" || cmd.action == "contact" || cmd.action == "folder" || cmd.action == "app" || cmd.action == "open_app" || cmd.action == "doc")
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
