//! Scanner module for finding and processing markdown files and contacts
//! 
//! This module scans specified directory roots for markdown files and
//! user contacts, creating commands based on the items found.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command as ProcessCommand;
use std::collections::{hash_map::DefaultHasher, HashSet};
use std::hash::{Hash, Hasher};
use crate::core::Command;
use crate::core::commands::save_commands_to_file;
use crate::core::{Config, load_state, save_state};
use crate::utils::debug_log;
use crate::execute::get_action;
use chrono::Local;

/// Action types that are automatically generated and removed by the scanner
/// These commands will be removed during rescanning unless they have the 'U' (user-edited) flag
pub const SCANNER_GENERATED_ACTIONS: &[&str] = &["markdown", "anchor", "folder", "app", "open_app"];

/// Helper function to log scanner debug messages
fn scanner_log(message: &str) {
    crate::utils::detailed_log("SCANNER", message);
}

/// Main function for automatic background scanning
/// Checks if filesystem scan should be performed and executes it if needed.
/// This function should be called on application exit, not startup.
pub fn scan_check(commands: Vec<Command>) -> Vec<Command> {
    let sys_data = crate::core::sys_data::get_sys_data();
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
            debug_log("SCANNER", "ERROR: No file_roots configured in config file");
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
            // Clear global cache since we've updated the commands file
            crate::core::sys_data::clear_sys_data();
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
    
    // Then scan cloud services (Notion, Google Drive) - Phase 1: Logging only
    crate::utils::log("\n☁️  Scanning cloud services...");
    crate::cloud_scanner::scan_cloud_services();
    
    // Then scan contacts - DISABLED for performance
    // commands = scan_contacts(commands);
    
    // Process orphan merges after scanning files
    if let Some(orphans_path_str) = &sys_data.config.popup_settings.orphans_path {
        let expanded_orphans_path = expand_home(orphans_path_str);
        let orphans_path = std::path::Path::new(&expanded_orphans_path);
        
        // Get vault roots to search for matching anchors
        for root in file_roots {
            let expanded_root = expand_home(root);
            let vault_root = std::path::Path::new(&expanded_root);
            
            // Skip orphan merge checking for /Applications and other system directories
            // Orphan merging only makes sense for markdown vault directories
            if expanded_root.starts_with("/Applications") || 
               expanded_root.starts_with("/System") || 
               expanded_root.starts_with("/Library") {
                continue;
            }
            
            if verbose {
                println!("\n🔄 Checking for orphan anchors to merge in {}...", expanded_root);
            }
            
            // Find orphan folders that should be merged
            let merges = crate::core::commands::find_orphan_folder_merges(orphans_path, vault_root);
            
            if !merges.is_empty() {
                if verbose {
                    println!("   Found {} orphan anchors to merge", merges.len());
                }
                
                // Execute each merge
                for (source, dest) in merges {
                    if verbose {
                        println!("   Merging: {} -> {}", 
                            source.file_name().unwrap_or_default().to_string_lossy(),
                            dest.display());
                    }
                    
                    if let Err(e) = crate::core::commands::merge_folders(&source, &dest) {
                        crate::utils::log_error(&format!("Failed to merge orphan: {}", e));
                        if verbose {
                            println!("   ❌ Merge failed: {}", e);
                        }
                    }
                }
                
                // After merging, we need to rescan to pick up the changes
                // But only if we actually performed merges
                if verbose {
                    println!("   Rescanning after merges...");
                }
                commands = scan_files(commands, file_roots, &sys_data.config);
            } else if verbose {
                println!("   No orphan anchors found to merge");
            }
        }
    }
    
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
    
    // Create a set of file paths that are already handled by existing commands
    // This allows O(1) lookup to prevent creating duplicate commands for the same file
    let mut handled_files: HashSet<String> = HashSet::new();
    for cmd in &commands {
        if cmd.action == "markdown" || cmd.action == "anchor" || cmd.action == "folder" || cmd.action == "app" || cmd.action == "open_app" {
            handled_files.insert(cmd.arg.clone());
        }
    }
    
    // Create a lookup map to preserve existing patches when regenerating commands
    let mut existing_patches: std::collections::HashMap<String, String> = std::collections::HashMap::new();
    for cmd in &commands {
        if (cmd.action == "markdown" || cmd.action == "anchor" || cmd.action == "folder" || cmd.action == "app" || cmd.action == "open_app") && !cmd.patch.is_empty() {
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
    
    // Remove existing markdown, anchor, and folder commands from the commands list
    // BUT ONLY for files within the directories we're about to scan
    // EXCEPT those with the "U" (user edited) flag - preserve those
    // Also validate that files actually exist for scanner-generated commands
    commands.retain(|cmd| {
        let is_scanner_generated = SCANNER_GENERATED_ACTIONS.contains(&cmd.action.as_str());
        let is_user_edited = cmd.flags.contains('U');
        
        // If it's scanner-generated, check if we should keep or remove it
        if is_scanner_generated {
            // Check if file exists (for cleanup of orphan commands)
            let file_exists = std::path::Path::new(&cmd.arg).exists();
            
            // Log file existence checks only in detailed/verbose mode
            crate::utils::detailed_log("SCANNER", &format!("Checking '{}' (action: {}) - file exists: {} at path: {}", 
                cmd.command, cmd.action, file_exists, cmd.arg));
            
            if !file_exists {
                // This is important enough to always log
                crate::utils::log(&format!("SCANNER: ✅ Removing command '{}' - file no longer exists: {}", cmd.command, cmd.arg));
                // Remove from handled_files too so it can be rescanned if the file is recreated
                handled_files.remove(&cmd.arg);
                return false; // Remove this command
            }
            
            // If the file is within scan roots and not user-edited, remove it for rescan
            if is_within_scan_roots(&cmd.arg) && !is_user_edited {
                handled_files.remove(&cmd.arg);
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
    // debug_log("SCANNER", &format!("Processing {} collected folders", found_folders.len()));
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
    //                     debug_log("SCANNER", &format!("Added folder command: {}", name_str));
    //                 }
    //             } else {
    //                 if is_scanner_debug_enabled(config) {
    //                     debug_log("SCANNER", &format!("Skipping folder '{}' - command already exists", name_str));
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
                // Process files (markdown files)
                if let Some(command) = process_markdown_with_root(&path, vault_root, existing_commands, &existing_patches) {
                    // Debug log for found markdown files
                    crate::utils::detailed_log("SCANNER", &format!("Found markdown file: {} -> command: {}", 
                        path.display(), command.command));
                    
                    // Check if this file is already handled by an existing command (O(1) lookup)
                    if handled_files.contains(&command.arg) {
                        crate::utils::detailed_log("SCANNER", &format!("Skipping '{}' - file already handled: {}", 
                            command.command, path.display()));
                    } else {
                        // Log that we're creating a new command (detailed log to avoid clutter)
                        crate::utils::detailed_log("SCANNER", &format!("Creating new command '{}' -> {} {}", 
                            command.command, command.action, command.arg));
                        
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
    
    // Filter only scan-generated commands (markdown, anchor, contact, folder, app, open_app)
    let mut scan_commands: Vec<_> = commands.iter()
        .filter(|cmd| cmd.action == "markdown" || cmd.action == "anchor" || cmd.action == "contact" || cmd.action == "folder" || cmd.action == "app" || cmd.action == "open_app")
        .collect();
    
    // Sort for consistent checksum - use to_new_format() for comparison
    scan_commands.sort_by(|a, b| a.to_new_format().cmp(&b.to_new_format()));
    
    // Hash all scan-generated command lines
    for cmd in scan_commands {
        cmd.to_new_format().hash(&mut hasher);
    }
    
    format!("{:x}", hasher.finish())
}

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