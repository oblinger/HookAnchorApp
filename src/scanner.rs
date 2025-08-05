//! Scanner module for finding and processing markdown files and contacts
//! 
//! This module scans specified directory roots for markdown files and
//! user contacts, creating commands based on the items found.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command as ProcessCommand;
use std::collections::{hash_map::DefaultHasher, HashSet};
use std::hash::{Hash, Hasher};
use crate::{Command, Config, load_state, save_state, save_commands_to_file, utils::debug_log};
use crate::core::get_action;
use chrono::Local;

/// Helper function to log scanner debug messages
fn scanner_log(message: &str) {
    crate::utils::detailed_log("SCANNER", message);
}

/// Checks if filesystem scan should be performed and executes it if needed
/// This function should be called on application exit, not startup.
pub fn file_scan_check(commands: Vec<Command>) -> Vec<Command> {
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
    let _markdown_roots = match &sys_data.config.markdown_roots {
        Some(roots) => roots,
        None => {
            debug_log("SCANNER", "ERROR: No markdown_roots configured in config file");
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

/// Top-level scan function that orchestrates all scanning operations
pub fn scan(commands: Vec<Command>, sys_data: &crate::core::sys_data::SysData) -> Vec<Command> {
    scan_verbose(commands, sys_data, false)
}

/// Top-level scan function with verbose output
pub fn scan_verbose(commands: Vec<Command>, sys_data: &crate::core::sys_data::SysData, verbose: bool) -> Vec<Command> {
    let empty_vec = vec![];
    let markdown_roots = sys_data.config.markdown_roots.as_ref().unwrap_or(&empty_vec);
    
    if verbose {
        println!("\n🔍 Starting filesystem scan...");
        println!("   Scanning roots: {:?}", markdown_roots);
    }
    
    // First scan markdown files
    let initial_count = commands.len();
    let commands = scan_files(commands, markdown_roots, &sys_data.config);
    
    if verbose {
        let files_added = commands.len().saturating_sub(initial_count);
        println!("   Scan complete: {} commands total", commands.len());
        if files_added > 0 {
            println!("   Added {} new commands", files_added);
        }
    }
    
    // Then scan contacts - DISABLED for performance
    // commands = scan_contacts(commands);
    
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

/// Scans the configured markdown roots and returns an updated command list
pub fn scan_files(mut commands: Vec<Command>, markdown_roots: &[String], config: &Config) -> Vec<Command> {
    
    // Create a set of existing command names for collision detection (lowercase for case-insensitive comparison)
    // Only include non-scan-generated commands to avoid false collisions
    let mut existing_commands: HashSet<String> = commands.iter()
        .filter(|cmd| cmd.action != "markdown" && cmd.action != "anchor" && cmd.action != "folder")
        .map(|cmd| cmd.command.to_lowercase())
        .collect();
    
    // Create a lookup map to preserve existing patches when regenerating commands
    let mut existing_patches: std::collections::HashMap<String, String> = std::collections::HashMap::new();
    for cmd in &commands {
        if (cmd.action == "markdown" || cmd.action == "anchor" || cmd.action == "folder") && !cmd.patch.is_empty() {
            existing_patches.insert(cmd.command.clone(), cmd.patch.clone());
        }
    }
    
    // Count commands before removal
    let _markdown_before = commands.iter().filter(|cmd| cmd.action == "markdown").count();
    let _anchor_before = commands.iter().filter(|cmd| cmd.action == "anchor").count();
    let _folder_before = commands.iter().filter(|cmd| cmd.action == "folder").count();
    let _user_edited_scanner_commands = commands.iter()
        .filter(|cmd| (cmd.action == "markdown" || cmd.action == "anchor" || cmd.action == "folder") && cmd.flags.contains('U'))
        .count();
    
    // Remove existing markdown, anchor, and folder commands from the commands list
    // EXCEPT those with the "U" (user edited) flag - preserve those
    // Also validate that files actually exist for scanner-generated commands
    commands.retain(|cmd| {
        let is_scanner_generated = cmd.action == "markdown" || cmd.action == "anchor" || cmd.action == "folder";
        let is_user_edited = cmd.flags.contains('U');
        
        // If it's scanner-generated, check if file exists (for cleanup of orphan commands)
        if is_scanner_generated {
            // For user-edited commands, still validate file existence to clean up orphans
            let file_exists = std::path::Path::new(&cmd.arg).exists();
            if !file_exists {
                debug_log("SCANNER", &format!("Removing command '{}' - file no longer exists: {}", cmd.command, cmd.arg));
                return false; // Remove this command
            }
            // Keep user-edited commands even if we're about to regenerate scanner commands
            return is_user_edited;
        }
        
        // Keep command if it's NOT scanner-generated
        true
    });
    
    let _preserved_user_edited = commands.iter()
        .filter(|cmd| (cmd.action == "markdown" || cmd.action == "anchor" || cmd.action == "folder") && cmd.flags.contains('U'))
        .count();
    
    // Collect folders during scanning
    // COMMENTED OUT: Folder scanning disabled for now - only creating commands for markdown files
    // let mut found_folders: Vec<PathBuf> = Vec::new();
    
    // Then scan for new markdown files and folders
    for root in markdown_roots {
        let expanded_root = expand_home(root);
        let root_path = Path::new(&expanded_root);
        
        
        if root_path.exists() && root_path.is_dir() {
            let _commands_before_scan = commands.len();
            // COMMENTED OUT: Passing empty Vec instead of found_folders since folder scanning is disabled
            let mut dummy_folders = Vec::new();
            scan_directory_with_root(&root_path, &root_path, &mut commands, &mut existing_commands, &mut dummy_folders, &existing_patches, config);
            let _commands_after_scan = commands.len();
        } else {
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
    
    commands
}


/// Recursively scans a directory for files and folders with vault root tracking
fn scan_directory_with_root(dir: &Path, vault_root: &Path, commands: &mut Vec<Command>, existing_commands: &mut HashSet<String>, found_folders: &mut Vec<PathBuf>, existing_patches: &std::collections::HashMap<String, String>, config: &Config) {
    scan_directory_with_root_protected(dir, vault_root, commands, existing_commands, found_folders, existing_patches, config, &mut HashSet::new(), 0)
}

/// Protected version with loop detection and depth limiting
fn scan_directory_with_root_protected(dir: &Path, vault_root: &Path, commands: &mut Vec<Command>, existing_commands: &mut HashSet<String>, found_folders: &mut Vec<PathBuf>, existing_patches: &std::collections::HashMap<String, String>, config: &Config, visited: &mut HashSet<PathBuf>, depth: usize) {
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
                
                // COMMENTED OUT: Folder collection disabled - only scanning for markdown files
                // found_folders.push(path.clone());
                
                // Recursively scan subdirectories
                scan_directory_with_root_protected(&path, vault_root, commands, existing_commands, found_folders, existing_patches, config, visited, depth + 1);
            } else {
                // Process files (markdown files)
                if let Some(command) = process_markdown_with_root(&path, vault_root, existing_commands, &existing_patches) {
                    // Check if a command with the same name, action, and argument already exists
                    let duplicate_exists = commands.iter().any(|existing_cmd| {
                        existing_cmd.command.to_lowercase() == command.command.to_lowercase() &&
                        existing_cmd.action == command.action &&
                        existing_cmd.arg == command.arg
                    });
                    
                    if duplicate_exists {
                    } else {
                        // Add to existing commands set to prevent future collisions (lowercase)
                        existing_commands.insert(command.command.to_lowercase());
                        commands.push(command);
                    }
                }
            }
        }
    }
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
    let file_name = path.file_stem()?.to_str()?;
    
    // Determine action type using the shared get_action function
    let action = get_action(path);
    
    // Create command name without suffix, but check for collisions (case-insensitive)
    let preferred_name = file_name.to_string();
    let command_name = if existing_commands.contains(&preferred_name.to_lowercase()) {
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
    
    let full_line = format!("{} : {} {};", command_name, action, arg);
    
    // Try to preserve existing patch if this command existed before
    let preserved_patch = existing_patches.get(&command_name).cloned().unwrap_or_else(String::new);
    
    Some(Command {
        patch: preserved_patch,
        command: command_name,
        action: action.to_string(),
        arg,
        flags: String::new(),
        full_line,
    })
}


/// Check if a directory should be skipped based on config patterns
fn should_skip_directory(dir_name: &str, config: &Config) -> bool {
    // Get skip patterns from config
    let skip_patterns = match &config.scanner_settings {
        Some(settings) => match &settings.skip_directory_patterns {
            Some(patterns) => patterns,
            None => return false,
        },
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
    
    // Filter only scan-generated commands (markdown, obs, anchor, contact, folder)
    let mut scan_commands: Vec<_> = commands.iter()
        .filter(|cmd| cmd.action == "markdown" || cmd.action == "anchor" || cmd.action == "contact" || cmd.action == "folder")
        .collect();
    
    // Sort for consistent checksum
    scan_commands.sort_by(|a, b| a.full_line.cmp(&b.full_line));
    
    // Hash all scan-generated command lines
    for cmd in scan_commands {
        cmd.full_line.hash(&mut hasher);
    }
    
    format!("{:x}", hasher.finish())
}

/// Scans macOS contacts and creates commands for contacts with phone or email
pub fn scan_contacts(mut commands: Vec<Command>) -> Vec<Command> {
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
                    let full_line = format!("{} : contact {};", command_name, contact_id);
                    
                    commands.push(Command {
                        patch: String::new(),
                        command: command_name,
                        action: "contact".to_string(),
                        arg: contact_id.to_string(),
                        flags: String::new(),
                        full_line,
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
                let full_line = format!("{} : contact {};", command_name, id);
                
                commands.push(Command {
                    patch: String::new(),
                    command: command_name,
                    action: "contact".to_string(),
                    arg: id.to_string(),
                    flags: String::new(),
                    full_line,
                });
            }
        }
    }
    
    commands
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use crate::core::is_markdown_anchor;

    #[test]
    fn test_is_markdown_anchor() {
        // Test case: ProjectName/ProjectName.md (should be anchor)
        let path = PathBuf::from("/home/user/ProjectName/ProjectName.md");
        assert!(is_markdown_anchor(&path));
        
        // Test case: ProjectName/readme.md (should not be anchor)
        let path = PathBuf::from("/home/user/ProjectName/readme.md");
        assert!(!is_markdown_anchor(&path));
        
        // Test case: projectname/PROJECTNAME.md (should be anchor - case insensitive)
        let path = PathBuf::from("/home/user/projectname/PROJECTNAME.md");
        assert!(is_markdown_anchor(&path));
    }
}