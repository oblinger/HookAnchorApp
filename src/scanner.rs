//! Scanner module for finding and processing markdown files and contacts
//! 
//! This module scans specified directory roots for markdown files and
//! user contacts, creating commands based on the items found.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command as ProcessCommand;
use std::collections::{hash_map::DefaultHasher, HashSet};
use std::hash::{Hash, Hasher};
use crate::{Command, load_config, load_state, save_state, save_commands_to_file, utils::debug_log};
use chrono::Local;

/// Checks if filesystem scan should be performed and executes it if needed
/// This function should be called on every startup.
pub fn startup_check(commands: Vec<Command>) -> Vec<Command> {
    let config = load_config();
    let mut state = load_state();
    
    let current_time = Local::now().timestamp();
    
    // Get scan interval from config (default to 10 seconds)
    let scan_interval = config.popup_settings.scan_interval_seconds.unwrap_or(10) as i64;
    
    // Check if enough time has passed since last scan
    let should_scan = match state.last_scan_time {
        Some(last_time) => (current_time - last_time) >= scan_interval,
        None => true, // Never scanned before
    };
    
    if !should_scan {
        return commands; // Not time to scan yet
    }
    
    // Re-enabling scanner for debugging
    debug_log("SCANNER", "Starting filesystem scan");
    // Perform filesystem scan
    let markdown_roots = match config.markdown_roots {
        Some(roots) => roots,
        None => {
            debug_log("SCANNER", "ERROR: No markdown_roots configured in config file");
            return commands; // Return without scanning if no roots configured
        }
    };
    
    debug_log("SCANNER", &format!("startup_check markdown_roots: {:?}", markdown_roots));
    
    let scanned_commands = scan(commands, &markdown_roots);
    
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
        eprintln!("Warning: Failed to save scan state: {}", e);
    }
    
    // Save commands only if checksum changed
    if checksum_changed {
        if let Err(e) = save_commands_to_file(&scanned_commands) {
            eprintln!("Warning: Failed to save updated commands: {}", e);
        }
    }
    
    scanned_commands
}

/// Top-level scan function that orchestrates all scanning operations
pub fn scan(mut commands: Vec<Command>, markdown_roots: &[String]) -> Vec<Command> {
    // First scan markdown files
    commands = scan_files(commands, markdown_roots);
    
    // Then scan contacts - DISABLED for performance
    // commands = scan_contacts(commands);
    
    commands
}

/// Scans the configured markdown roots and returns an updated command list
pub fn scan_files(mut commands: Vec<Command>, markdown_roots: &[String]) -> Vec<Command> {
    // Re-enabled for debugging - adding extensive logging
    debug_log("SCANNER", &format!("Starting scan_files with {} initial commands", commands.len()));
    debug_log("SCANNER", &format!("Markdown roots to scan: {:?}", markdown_roots));
    
    // Create a set of existing command names for collision detection (lowercase for case-insensitive comparison)
    // Only include non-scan-generated commands to avoid false collisions
    let mut existing_commands: HashSet<String> = commands.iter()
        .filter(|cmd| cmd.action != "obs" && cmd.action != "anchor" && cmd.action != "folder")
        .map(|cmd| cmd.command.to_lowercase())
        .collect();
    
    // Count commands before removal
    let obs_before = commands.iter().filter(|cmd| cmd.action == "obs").count();
    let anchor_before = commands.iter().filter(|cmd| cmd.action == "anchor").count();
    let folder_before = commands.iter().filter(|cmd| cmd.action == "folder").count();
    debug_log("SCANNER", &format!("Before removal: {} obs, {} anchor, {} folder", obs_before, anchor_before, folder_before));
    
    // Remove all existing obs, anchor, and folder commands from the commands list
    // but keep ALL commands in existing_commands set for collision detection
    commands.retain(|cmd| {
        cmd.action != "obs" && cmd.action != "anchor" && cmd.action != "folder"
    });
    
    debug_log("SCANNER", &format!("After removal: {} commands remaining", commands.len()));
    
    // Collect folders during scanning
    let mut found_folders: Vec<PathBuf> = Vec::new();
    
    // Then scan for new markdown files and folders
    for root in markdown_roots {
        let expanded_root = expand_home(root);
        let root_path = Path::new(&expanded_root);
        
        debug_log("SCANNER", &format!("Checking root: {} -> {}", root, expanded_root));
        
        if root_path.exists() && root_path.is_dir() {
            let commands_before_scan = commands.len();
            debug_log("SCANNER", &format!("Scanning directory: {}", expanded_root));
            scan_directory_with_root(&root_path, &root_path, &mut commands, &mut existing_commands, &mut found_folders);
            let commands_after_scan = commands.len();
            debug_log("SCANNER", &format!("Added {} commands from {}", commands_after_scan - commands_before_scan, expanded_root));
        } else {
            debug_log("SCANNER", &format!("Skipping non-existent or non-directory: {}", expanded_root));
        }
    }
    
    // Process collected folders at the end
    debug_log("SCANNER", &format!("Processing {} collected folders", found_folders.len()));
    for folder_path in found_folders {
        if let Some(folder_name) = folder_path.file_name() {
            if let Some(name_str) = folder_name.to_str() {
                // Check if a command already exists with this name (case-insensitive)
                if !existing_commands.contains(&name_str.to_lowercase()) {
                    // Create folder command without "folder" suffix
                    let command_name = name_str.to_string();
                    let full_path = folder_path.to_string_lossy().to_string();
                    let full_line = format!("{} : folder {};", command_name, full_path);
                    
                    let folder_command = Command {
                        group: String::new(),
                        command: command_name.clone(),
                        action: "folder".to_string(),
                        arg: full_path,
                        flags: String::new(),
                        full_line,
                    };
                    
                    commands.push(folder_command);
                    existing_commands.insert(command_name.to_lowercase());
                    debug_log("SCANNER", &format!("Added folder command: {}", name_str));
                } else {
                    debug_log("SCANNER", &format!("Skipping folder '{}' - command already exists", name_str));
                }
            }
        }
    }
    
    // Count final results
    let obs_after = commands.iter().filter(|cmd| cmd.action == "obs").count();
    let anchor_after = commands.iter().filter(|cmd| cmd.action == "anchor").count();
    let folder_after = commands.iter().filter(|cmd| cmd.action == "folder").count();
    debug_log("SCANNER", &format!("Final scan results: {} obs, {} anchor, {} folder (total: {} commands)", 
        obs_after, anchor_after, folder_after, commands.len()));
    
    commands
}


/// Recursively scans a directory for files and folders with vault root tracking
fn scan_directory_with_root(dir: &Path, vault_root: &Path, commands: &mut Vec<Command>, existing_commands: &mut HashSet<String>, found_folders: &mut Vec<PathBuf>) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.filter_map(Result::ok) {
            let path = entry.path();
            
            // Skip hidden files and directories (those starting with .)
            if let Some(file_name) = path.file_name() {
                if let Some(name_str) = file_name.to_str() {
                    if name_str.starts_with('.') {
                        continue;
                    }
                    
                    // Also skip certain system directories
                    let skip_folders = ["node_modules", "target", "__pycache__", ".git", ".svn"];
                    if path.is_dir() && skip_folders.contains(&name_str) {
                        continue;
                    }
                }
            }
            
            // Process directories first
            if path.is_dir() {
                // Instead of creating folder command immediately, collect the folder
                found_folders.push(path.clone());
                
                // Recursively scan subdirectories
                scan_directory_with_root(&path, vault_root, commands, existing_commands, found_folders);
            } else {
                // Process files (markdown files)
                if let Some(command) = process_markdown_with_root(&path, vault_root, existing_commands) {
                    // Add to existing commands set to prevent future collisions (lowercase)
                    existing_commands.insert(command.command.to_lowercase());
                    commands.push(command);
                }
            }
        }
    }
}


/// Processes a path and returns a command if it's a markdown file with vault root for relative paths
fn process_markdown_with_root(path: &Path, vault_root: &Path, existing_commands: &HashSet<String>) -> Option<Command> {
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
    
    // Determine action type
    let action = if is_markdown_anchor(path) {
        "anchor"
    } else {
        "obs"
    };
    
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
    let arg = if action == "obs" {
        // For OBS entries, use relative path from vault root (with .md extension)
        if let Ok(relative_path) = path.strip_prefix(vault_root) {
            relative_path.to_string_lossy().to_string()
        } else {
            // Fallback to just filename with extension if we can't calculate relative path
            format!("{}.md", file_name)
        }
    } else {
        // For anchor entries, use full path as before
        full_path.clone()
    };
    
    let full_line = format!("{} : {} {};", command_name, action, arg);
    
    Some(Command {
        group: String::new(),
        command: command_name,
        action: action.to_string(),
        arg,
        flags: String::new(),
        full_line,
    })
}


/// Checks if a markdown file is an "anchor" (base name matches parent folder name)
fn is_markdown_anchor(path: &Path) -> bool {
    // Get the file stem (base name without extension)
    let file_stem = match path.file_stem() {
        Some(stem) => stem.to_str().unwrap_or(""),
        None => return false,
    };
    
    // Get the parent directory name
    let parent_name = match path.parent() {
        Some(parent) => match parent.file_name() {
            Some(name) => name.to_str().unwrap_or(""),
            None => return false,
        },
        None => return false,
    };
    
    // Compare ignoring case
    file_stem.to_lowercase() == parent_name.to_lowercase()
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
    
    // Filter only scan-generated commands (obs, anchor, contact, folder)
    let mut scan_commands: Vec<_> = commands.iter()
        .filter(|cmd| cmd.action == "obs" || cmd.action == "anchor" || cmd.action == "contact" || cmd.action == "folder")
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
                        group: String::new(),
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
                    group: String::new(),
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
    use super::*;
    use std::path::PathBuf;

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