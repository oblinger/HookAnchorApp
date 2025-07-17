//! Command management and operations
//! 
//! This module handles all command-related operations including loading, saving,
//! filtering, merging, and manipulation of commands.

use std::fs;
use std::path::{Path, PathBuf};
use std::env;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use super::config::Config;

/// Represents a parsed command with its components and original line
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Command {
    pub patch: String,
    pub command: String,
    pub action: String,
    pub arg: String,
    pub flags: String,
    pub full_line: String,
}

/// Represents the target of a command execution
#[derive(Debug, Clone, PartialEq)]
pub enum CommandTarget {
    /// A specific command to execute
    Command(Command),
    /// A command that launches another anchor command  
    Alias(String),
}

/// Represents a Patch (Dispatcher) that associates with a command
#[derive(Debug, Clone, PartialEq)]
pub struct Patch {
    /// Name of the patch (lowercase)
    pub name: String,
    /// The first command that matches this patch name
    pub linked_command: Option<Command>,
}

/// Mapping from flag letters to their word descriptions
pub const FLAG_LETTER_MAPPING: &[(&str, &str)] = &[
    ("M", "merge"),
    ("U", "user edited"),
    // Add more flag mappings here as needed
];

impl Command {
    /// Returns the absolute file path for the command's argument
    /// Handles relative paths, tilde expansion, and vault-relative paths
    pub fn get_absolute_file_path(&self, config: &Config) -> Option<PathBuf> {
        match self.action.as_str() {
            "markdown" => {
                // Arg is already absolute path for new markdown action
                Some(PathBuf::from(&self.arg))
            },
            "anchor" | "doc" => {
                // Already absolute, just expand tilde
                Some(PathBuf::from(crate::utils::expand_tilde(&self.arg)))
            }
            "folder" => {
                // Handle both relative and absolute paths
                if self.arg.starts_with('/') || self.arg.starts_with('~') {
                    Some(PathBuf::from(crate::utils::expand_tilde(&self.arg)))
                } else {
                    // Relative to vault root
                    let launcher_settings = config.launcher_settings.as_ref()?;
                    let vault_path = launcher_settings.obsidian_vault_path.as_ref()?;
                    let expanded_vault = crate::utils::expand_tilde(vault_path);
                    Some(Path::new(&expanded_vault).join(&self.arg))
                }
            }
            "open" => {
                // Handle file paths (expand tilde for absolute, or assume relative to current dir)
                if self.arg.starts_with('/') || self.arg.starts_with('~') {
                    Some(PathBuf::from(crate::utils::expand_tilde(&self.arg)))
                } else {
                    // Relative to current working directory
                    Some(env::current_dir().ok()?.join(&self.arg))
                }
            }
            _ => None // Not a file-based action
        }
    }
    
    /// Returns the absolute folder path for the command
    /// For file-based commands, returns the parent directory
    /// For folder commands, returns the folder itself
    pub fn get_absolute_folder_path(&self, config: &Config) -> Option<PathBuf> {
        match self.action.as_str() {
            "folder" => {
                // For folder commands, return the folder itself
                self.get_absolute_file_path(config)
            }
            _ => {
                // For file-based commands, return the parent directory
                self.get_absolute_file_path(config)
                    .and_then(|p| p.parent().map(|parent| parent.to_path_buf()))
            }
        }
    }
    
    /// Checks if this command refers to a file or folder
    pub fn is_path_based(&self) -> bool {
        matches!(self.action.as_str(), "markdown" | "anchor" | "folder" | "doc" | "open")
    }

    /// Gets the value of a flag by its key character
    /// Returns the string after the flag key, or None if the flag doesn't exist
    pub fn get_flag(&self, key: char) -> Option<String> {
        if self.flags.is_empty() {
            return None;
        }
        
        // Split by commas and look for the flag
        for flag_part in self.flags.split(',') {
            let flag_part = flag_part.trim();
            if flag_part.starts_with(key) {
                // Return the part after the flag key (empty string if flag is just the key)
                return Some(flag_part[1..].to_string());
            }
        }
        None
    }
    
    /// Sets the value of a flag by its key character
    /// If the flag exists, updates its value; if not, adds it
    pub fn set_flag(&mut self, key: char, value: &str) {
        let new_flag = format!("{}{}", key, value);
        
        if self.flags.is_empty() {
            self.flags = new_flag;
            self.update_full_line();
            return;
        }
        
        // Check if flag already exists and replace it
        let mut flag_parts: Vec<String> = self.flags.split(',')
            .map(|s| s.trim().to_string())
            .collect();
        
        let mut found = false;
        for part in flag_parts.iter_mut() {
            if part.starts_with(key) {
                *part = new_flag.clone();
                found = true;
                break;
            }
        }
        
        if !found {
            flag_parts.push(new_flag);
        }
        
        self.flags = flag_parts.join(",");
        self.update_full_line();
    }
    
    /// Removes a flag by its key character
    pub fn remove_flag(&mut self, key: char) {
        if self.flags.is_empty() {
            return;
        }
        
        let flag_parts: Vec<String> = self.flags.split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.starts_with(key))
            .collect();
        
        self.flags = flag_parts.join(",");
        self.update_full_line();
    }
    
    /// Updates the full_line field to reflect current command state in new format
    pub fn update_full_line(&mut self) {
        self.full_line = self.to_new_format();
    }
    
    /// Converts the command to new format string
    pub fn to_new_format(&self) -> String {
        let mut result = String::new();
        
        // Add group if present
        if !self.patch.is_empty() {
            result.push_str(&self.patch);
            result.push_str("! ");
        }
        
        // Add command and action
        result.push_str(&self.command);
        result.push_str(" : ");
        result.push_str(&self.action);
        
        // Add flags if present (before semicolon)
        if !self.flags.is_empty() {
            result.push(' ');
            result.push_str(&self.flags);
        }
        
        // Add semicolon to separate flags from arg
        result.push(';');
        
        // Add arg if present (after semicolon)
        if !self.arg.is_empty() {
            result.push(' ');
            result.push_str(&self.arg);
        }
        
        result
    }
}

/// Returns the path to the commands.txt file
pub fn get_commands_file_path() -> PathBuf {
    let home = env::var("HOME").unwrap_or_else(|_| ".".to_string());
    Path::new(&home).join(".config/hookanchor/commands.txt")
}

/// Returns the path to the backups folder
pub fn get_backups_folder_path() -> PathBuf {
    let home = env::var("HOME").unwrap_or_else(|_| ".".to_string());
    Path::new(&home).join(".config/hookanchor/backups")
}

/// Creates a backup of the commands file before saving
pub fn backup_commands_file() -> Result<(), Box<dyn std::error::Error>> {
    let commands_path = get_commands_file_path();
    let backups_path = get_backups_folder_path();
    
    // Create backups directory if it doesn't exist
    fs::create_dir_all(&backups_path)?;
    
    // Only backup if the commands file exists
    if commands_path.exists() {
        let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S");
        let backup_name = format!("commands_{}.txt", timestamp);
        let backup_path = backups_path.join(backup_name);
        
        fs::copy(&commands_path, backup_path)?;
    }
    
    Ok(())
}

/// Loads commands from the commands.txt file
/// Creates a hashmap from patch names to Patch structs
/// Only creates patches for anchor commands where the markdown file name matches the folder name
pub fn create_patches_hashmap(commands: &[Command]) -> HashMap<String, Patch> {
    let mut patches = HashMap::new();
    
    // Scan for anchor commands where file name matches containing folder name
    for command in commands {
        if command.action == "anchor" {
            // Extract file path and check if file name matches folder name
            let file_path = &command.arg;
            
            // Get the file name without extension
            if let Some(last_slash) = file_path.rfind('/') {
                let file_part = &file_path[last_slash + 1..];
                let file_name = if let Some(dot_pos) = file_part.rfind('.') {
                    &file_part[..dot_pos]
                } else {
                    file_part
                };
                
                // Get the folder name (parent directory)
                let folder_part = &file_path[..last_slash];
                let folder_name = if let Some(parent_slash) = folder_part.rfind('/') {
                    &folder_part[parent_slash + 1..]
                } else {
                    folder_part
                };
                
                // Check if file name matches folder name (case-insensitive)
                if file_name.to_lowercase() == folder_name.to_lowercase() {
                    let patch_name = command.command.to_lowercase();
                    
                    // Create patch if it doesn't exist yet
                    if !patches.contains_key(&patch_name) {
                        patches.insert(patch_name.clone(), Patch {
                            name: patch_name,
                            linked_command: Some(command.clone()),
                        });
                    }
                }
            }
        }
    }
    
    patches
}

/// Infers the appropriate patch value for a command based on various heuristics
/// 1. First checks if the first word matches any existing patch name (case-insensitive)
/// 2. Then checks if the command refers to a file in a folder associated with a patch
/// 3. If it's an anchor file, walks hierarchy looking for containing patch
/// Returns None if no patch can be inferred
/// Always analyzes the command regardless of any current patch value
pub fn infer_patch(command: &Command, patches: &HashMap<String, Patch>) -> Option<String> {
    // Method 1: Alias commands inherit patch from their target (HIGHEST PRIORITY)
    if command.action == "alias" {
        if let Some(target_patch) = infer_patch_from_alias_target(command, patches) {
            return Some(target_patch);
        }
    }
    
    // Method 2: File/folder-based inference (HIGH PRIORITY for path-based commands)
    // Check if the command is path-based and extract folder information
    if command.is_path_based() {
        if let Some(inferred_patch) = infer_patch_from_command(command, patches) {
            return Some(inferred_patch);
        }
    }
    
    // Method 3: Check if first word matches any patch name
    if let Some(space_idx) = command.command.find(' ') {
        let first_word = &command.command[..space_idx];
        let first_word_lower = first_word.to_lowercase();
        
        // Look for exact patch name match (case-insensitive)
        if let Some(patch) = patches.get(&first_word_lower) {
            // Check if this would create a self-assignment (command assigned to its own patch)
            let proposed_patch = if let Some(ref linked_cmd) = patch.linked_command {
                if let Some(linked_space_idx) = linked_cmd.command.find(' ') {
                    linked_cmd.command[..linked_space_idx].to_string()
                } else {
                    linked_cmd.command.clone()
                }
            } else {
                first_word.to_string()
            };
            
            // Prevent self-assignment: don't assign a command to its own patch
            if proposed_patch.to_lowercase() != command.command.to_lowercase() {
                return Some(proposed_patch);
            }
            // If it would be self-assignment, skip this method and try others
        }
    }
    
    // Method 4: Default patch rules
    
    // Rule 4a: Cmd action gets "Cmd" patch (ONLY if current patch is empty)
    if command.action == "cmd" && command.patch.is_empty() {
        return Some("Cmd".to_string());
    }
    
    // Rule 4b: Browser/Web actions get "Web" patch (ONLY if current patch is empty)
    let web_actions = ["chrome", "brave", "firefox", "url", "safari"];
    if web_actions.contains(&command.action.as_str()) && command.patch.is_empty() {
        return Some("Web".to_string());
    }
    
    // Rule 4c: Year-based patch inference (LOWEST PRIORITY FALLBACK)
    // Only applies when no other rule matches and current patch is empty
    if command.patch.is_empty() {
        if let Some(year_patch) = infer_patch_from_year_prefix(&command.command) {
            return Some(year_patch);
        }
    }
    
    None
}

/// Infers patch from command using its path accessors
fn infer_patch_from_command(command: &Command, patches: &HashMap<String, Patch>) -> Option<String> {
    // Use the file path method with self-assignment prevention
    infer_patch_from_file_path_with_exclusion(&command.arg, patches, &command.command)
}

/// Infers patch from file path with self-assignment prevention
fn infer_patch_from_file_path_with_exclusion(file_path: &str, patches: &HashMap<String, Patch>, exclude_command: &str) -> Option<String> {
    use std::path::Path;
    
    // Skip if not a file path (URLs, app names, etc.)
    if file_path.starts_with("http") || file_path.starts_with("https") || !file_path.contains('/') {
        return None;
    }
    
    // Method 1: Check all path components and return the most specific (deepest) non-self match
    // For paths like "T/Misc/Sleep.md", prefer "Misc" over "T"
    // This works for both relative and absolute paths
    let components: Vec<&str> = file_path.split('/').collect();
    let mut best_matches: Vec<(usize, String)> = Vec::new(); // (depth, patch_name)
    
    for (depth, component) in components.iter().enumerate() {
        let component_lower = component.to_lowercase();
        if patches.contains_key(&component_lower) {
            // Found a patch match - add to list
            best_matches.push((depth, component.to_string()));
        }
    }
    
    // Sort by depth (deepest first) and return first non-self match
    best_matches.sort_by(|a, b| b.0.cmp(&a.0)); // Sort by depth descending
    
    for (_, patch_name) in best_matches {
        if patch_name.to_lowercase() != exclude_command.to_lowercase() {
            return Some(patch_name);
        }
    }
    
    // If no component matches found, try the directory hierarchy method
    let path = Path::new(file_path);
    
    // Get the directory containing the file
    let dir = if path.is_file() || file_path.contains('.') {
        path.parent()?
    } else {
        path
    };
    
    // Method 2: Check if any patch's linked command refers to a file in the same directory or parent directory
    for patch in patches.values() {
        if let Some(ref linked_cmd) = patch.linked_command {
            if linked_cmd.is_path_based() {
                let linked_path = Path::new(&linked_cmd.arg);
                let linked_dir = if linked_path.is_file() || linked_cmd.arg.contains('.') {
                    linked_path.parent()
                } else {
                    Some(linked_path)
                };
                
                if let Some(linked_dir) = linked_dir {
                    // Check if directories match exactly
                    if dir == linked_dir {
                        // Prevent self-assignment
                        if patch.name.to_lowercase() != exclude_command.to_lowercase() {
                            // Check if this is an anchor file (same name as patch)
                            if let Some(file_stem) = path.file_stem() {
                                if file_stem.to_string_lossy().to_lowercase() == patch.name {
                                    // This is an anchor file, walk hierarchy for containing patch
                                    return infer_patch_from_hierarchy(dir, patches);
                                }
                            }
                            
                            // Not an anchor file, return this patch
                            return Some(patch.name.clone());
                        }
                    }
                }
            }
        }
    }
    
    None
}

/// Legacy function: Infers patch from file path by checking folder associations
/// This maintains the existing logic for raw path analysis without config dependency
fn infer_patch_from_file_path(file_path: &str, patches: &HashMap<String, Patch>) -> Option<String> {
    use std::path::Path;
    
    // Skip if not a file path (URLs, app names, etc.)
    if file_path.starts_with("http") || file_path.starts_with("https") || !file_path.contains('/') {
        return None;
    }
    
    // Method 1: Check all path components and return the most specific (deepest) match
    // For paths like "T/Misc/Sleep.md", prefer "Misc" over "T"
    // This works for both relative and absolute paths
    let components: Vec<&str> = file_path.split('/').collect();
    let mut best_match: Option<(usize, String)> = None; // (depth, patch_name)
    
    for (depth, component) in components.iter().enumerate() {
        let component_lower = component.to_lowercase();
        if patches.contains_key(&component_lower) {
            // Found a patch match - update if this is deeper than previous match
            if best_match.is_none() || depth > best_match.as_ref().unwrap().0 {
                best_match = Some((depth, component.to_string()));
            }
        }
    }
    
    if let Some((_, patch_name)) = best_match {
        return Some(patch_name);
    }
    
    let path = Path::new(file_path);
    
    // Get the directory containing the file
    let dir = if path.is_file() || file_path.contains('.') {
        path.parent()?
    } else {
        path
    };
    
    // Method 2: Check if any patch's linked command refers to a file in the same directory or parent directory
    for patch in patches.values() {
        if let Some(ref linked_cmd) = patch.linked_command {
            if linked_cmd.is_path_based() {
                let linked_path = Path::new(&linked_cmd.arg);
                let linked_dir = if linked_path.is_file() || linked_cmd.arg.contains('.') {
                    linked_path.parent()
                } else {
                    Some(linked_path)
                };
                
                if let Some(linked_dir) = linked_dir {
                    // Check if directories match exactly
                    if dir == linked_dir {
                        // Check if this is an anchor file (same name as patch)
                        if let Some(file_stem) = path.file_stem() {
                            if file_stem.to_string_lossy().to_lowercase() == patch.name {
                                // This is an anchor file, walk hierarchy for containing patch
                                return infer_patch_from_hierarchy(dir, patches);
                            }
                        }
                        
                        // Not an anchor file, return this patch
                        return Some(patch.name.clone());
                    }
                    
                    // Method 3: Check if the file is in a subdirectory of the patch's directory
                    // This handles cases like Instabase being under T/Career/NJ/...
                    if let Ok(relative) = dir.strip_prefix(linked_dir) {
                        if !relative.as_os_str().is_empty() {
                            // The file is in a subdirectory of the patch's directory
                            // Return the patch name in the correct case
                            if let Some(ref linked_cmd) = patch.linked_command {
                                // Extract the patch name from the linked command
                                // For example, if linked command is "T", return "T"
                                if linked_cmd.command.to_lowercase() == patch.name {
                                    return Some(linked_cmd.command.clone());
                                }
                            }
                            return Some(patch.name.clone());
                        }
                    }
                }
            }
        }
    }
    
    None
}

/// Walks directory hierarchy looking for a patch that contains this folder
fn infer_patch_from_hierarchy(dir: &Path, patches: &HashMap<String, Patch>) -> Option<String> {
    let mut current_dir = dir.parent();
    
    while let Some(parent) = current_dir {
        // Check if any patch is associated with this parent directory
        for patch in patches.values() {
            if let Some(ref linked_cmd) = patch.linked_command {
                if linked_cmd.is_path_based() {
                    let linked_path = Path::new(&linked_cmd.arg);
                    let linked_dir = if linked_path.is_file() || linked_cmd.arg.contains('.') {
                        linked_path.parent()
                    } else {
                        Some(linked_path)
                    };
                    
                    if let Some(linked_dir) = linked_dir {
                        if parent == linked_dir {
                            return Some(patch.name.clone());
                        }
                    }
                }
            }
        }
        
        current_dir = parent.parent();
    }
    
    None
}

/// Infers patch for alias commands by looking up the target command's patch
fn infer_patch_from_alias_target(command: &Command, patches: &HashMap<String, Patch>) -> Option<String> {
    // Load all commands to find the alias target
    let all_commands = load_commands_raw();
    
    // Find the target command that the alias points to
    let target_commands = filter_commands(&all_commands, &command.arg, 1, false);
    
    if let Some(target_command) = target_commands.first() {
        // If target has a patch, return it
        if !target_command.patch.is_empty() {
            return Some(target_command.patch.clone());
        }
        
        // If target doesn't have a patch, try to infer one for it
        // (but don't recurse infinitely - only one level)
        if target_command.action != "alias" {
            return infer_patch(target_command, patches);
        }
    }
    
    None
}

/// Infers patch from year prefix in command name (2000-3000 followed by separator)
fn infer_patch_from_year_prefix(command_name: &str) -> Option<String> {
    // Check if command starts with 4-digit year between 2000 and 3000
    if command_name.len() >= 5 {
        let year_part = &command_name[0..4];
        
        // Check if it's a valid 4-digit number
        if let Ok(year) = year_part.parse::<u32>() {
            // Check if year is in valid range (2000-3000)
            if year >= 2000 && year <= 3000 {
                // Check if followed by a separator character
                let separator_char = command_name.chars().nth(4).unwrap();
                if separator_char == '-' || separator_char == '_' || separator_char == '.' || separator_char == ' ' {
                    return Some(year.to_string());
                }
            }
        }
    }
    
    None
}

/// Automatically assigns patch names to commands based on shared prefixes
/// For commands without a patch, if their name starts with a prefix (up to first space)
/// that's shared with at least one other command, that prefix becomes the patch name.
/// The patch name preserves the case of the first occurrence of the prefix.
pub fn auto_assign_patches(commands: &mut Vec<Command>) {
    use std::collections::HashMap;
    
    // First, collect all prefixes and count them
    let mut prefix_counts: HashMap<String, usize> = HashMap::new();
    let mut prefix_first_case: HashMap<String, String> = HashMap::new();
    
    // Scan all commands to find prefixes
    for cmd in commands.iter() {
        if let Some(space_idx) = cmd.command.find(' ') {
            let prefix = &cmd.command[..space_idx];
            let prefix_lower = prefix.to_lowercase();
            
            // Count this prefix
            *prefix_counts.entry(prefix_lower.clone()).or_insert(0) += 1;
            
            // Store the first case we see of this prefix
            prefix_first_case.entry(prefix_lower).or_insert_with(|| prefix.to_string());
        }
    }
    
    // Now assign patches to commands without patches
    for cmd in commands.iter_mut() {
        // Skip if command already has a patch
        if !cmd.patch.is_empty() {
            continue;
        }
        
        // Check if this command has a prefix
        if let Some(space_idx) = cmd.command.find(' ') {
            let prefix_lower = cmd.command[..space_idx].to_lowercase();
            
            // If this prefix is shared by at least 2 commands, assign it as patch
            if let Some(count) = prefix_counts.get(&prefix_lower) {
                if *count >= 2 {
                    // Use the first case we saw of this prefix
                    if let Some(patch_name) = prefix_first_case.get(&prefix_lower) {
                        cmd.patch = patch_name.clone();
                    }
                }
            }
        }
    }
}

/// Run patch inference on commands with configurable behavior
/// 
/// # Arguments
/// * `commands` - Mutable reference to commands vector
/// * `patches` - Reference to patches hashmap for inference
/// * `apply_changes` - If true, actually update command patches; if false, only analyze
/// * `print_to_stdout` - If true, print change summaries to stdout
/// * `overwrite_patch` - If true, consider all commands; if false, only process commands with empty patches
/// 
/// # Returns
/// * Number of patches that were assigned/would be assigned
/// * Vector of new patch keys that were added/would be added to hashmap
pub fn run_patch_inference(
    commands: &mut Vec<Command>, 
    patches: &HashMap<String, Patch>,
    apply_changes: bool,
    print_to_stdout: bool,
    overwrite_patch: bool
) -> (usize, Vec<String>) {
    let mut patches_assigned = 0;
    let mut new_patches_to_add = Vec::new();
    
    for command in commands.iter_mut() {
        // Check if we should process this command based on overwrite_patch setting
        let should_process = overwrite_patch || command.patch.is_empty();
        
        if should_process {
            if let Some(inferred_patch) = infer_patch(command, patches) {
                // Skip if patch wouldn't change - always skip when values are the same
                if command.patch == inferred_patch {
                    continue;
                }
                
                let old_patch_display = if command.patch.is_empty() { "(empty)".to_string() } else { command.patch.clone() };
                
                // Apply changes based on the overwrite_patch setting
                // In normal operation (overwrite_patch = false), only fill empty patches
                // In --infer-all mode (overwrite_patch = true), overwrite existing patches too
                if apply_changes && (overwrite_patch || command.patch.is_empty()) {
                    command.patch = inferred_patch.clone();
                    if crate::core::config::load_config().popup_settings.debug_log.is_some() {
                        eprintln!("Switched command '{}' patch: {} -> {}", 
                                command.command, old_patch_display, inferred_patch);
                    }
                }
                
                if print_to_stdout {
                    println!("{}: {} -> {}", command.command, old_patch_display, inferred_patch);
                }
                
                patches_assigned += 1;
                
                // Track new patches to add later
                let patch_key = inferred_patch.to_lowercase();
                if !patches.contains_key(&patch_key) && !new_patches_to_add.contains(&patch_key) {
                    new_patches_to_add.push(patch_key);
                }
            }
        }
    }
    
    (patches_assigned, new_patches_to_add)
}

/// Find patches that don't have an associated anchor command
pub fn find_orphan_patches(patches: &HashMap<String, Patch>, commands: &[Command]) -> Vec<String> {
    let mut orphan_patches = Vec::new();
    
    for (patch_name, _patch) in patches {
        // Check if there's any anchor command for this patch
        let has_anchor = commands.iter().any(|cmd| {
            cmd.action == "anchor" && cmd.patch.to_lowercase() == patch_name.to_lowercase()
        });
        
        if !has_anchor {
            orphan_patches.push(patch_name.clone());
        }
    }
    
    orphan_patches
}

/// Create orphan anchor files and commands for patches without anchors
pub fn create_orphan_anchors(
    config: &Config, 
    orphan_patches: &[String], 
    commands: &mut Vec<Command>
) -> Result<usize, Box<dyn std::error::Error>> {
    let orphans_path = config.scanner_settings
        .as_ref()
        .and_then(|s| s.orphans_path.as_ref())
        .ok_or("orphans_path not configured")?;
    
    let mut created_count = 0;
    
    for patch_name in orphan_patches {
        // Expand tilde in the path
        let expanded_path = if orphans_path.starts_with("~/") {
            if let Ok(home) = env::var("HOME") {
                orphans_path.replace("~", &home)
            } else {
                orphans_path.clone()
            }
        } else {
            orphans_path.clone()
        };
        
        // Create the folder path for this patch
        let patch_folder = Path::new(&expanded_path).join(patch_name);
        
        // Create the directory if it doesn't exist
        if !patch_folder.exists() {
            fs::create_dir_all(&patch_folder)?;
            eprintln!("Created orphan patch folder: {}", patch_folder.display());
        }
        
        // Create the markdown file
        let markdown_file = patch_folder.join(format!("{}.md", patch_name));
        
        if !markdown_file.exists() {
            let markdown_content = format!(
                "# {}\n\nThis is an auto-generated anchor file for patch '{}'.\n\nAdd your content here.\n",
                patch_name, patch_name
            );
            
            fs::write(&markdown_file, markdown_content)?;
            eprintln!("Created orphan anchor file: {}", markdown_file.display());
        }
        
        // Create the anchor command
        let anchor_command = Command {
            patch: patch_name.clone(),
            command: patch_name.clone(),
            action: "anchor".to_string(),
            arg: markdown_file.to_string_lossy().to_string(),
            flags: "A".to_string(), // Auto-generated flag
            full_line: format!("{} : anchor {}", patch_name, markdown_file.to_string_lossy()),
        };
        
        // Add the command to the list
        commands.push(anchor_command);
        created_count += 1;
        
        eprintln!("Created orphan anchor command for patch: {}", patch_name);
    }
    
    Ok(created_count)
}

/// Comprehensive data loading function that performs all necessary steps in order:
/// 1. Loads config file
/// 2. Loads commands from commands.txt file
/// 3. Computes patch data structure (hashmap)
/// 4. Infers patches for commands without patches
/// 5. Creates orphan anchor files and commands for patches without anchors
/// 6. Saves commands if any changes were made
pub fn load_data() -> (crate::core::config::Config, Vec<Command>, HashMap<String, Patch>) {
    // Step 1: Load config
    let config = crate::core::config::load_config();
    
    // Step 2: Load commands
    let mut commands = load_commands_raw();
    
    // Step 3: Compute patch data structure
    let mut patches = create_patches_hashmap(&commands);
    
    // Step 4: Infer patches for commands without patches
    let (patches_assigned, new_patches_to_add) = run_patch_inference(
        &mut commands, 
        &patches, 
        true,  // apply_changes = true (normal operation)
        false, // print_to_stdout = false (use logging instead)
        false  // overwrite_patch = false (only fill empty patches)
    );
    
    // Add new patches to hashmap
    for patch_key in new_patches_to_add {
        if !patches.contains_key(&patch_key) {
            // Find the first command whose name matches this patch name
            let linked_command = commands.iter()
                .find(|cmd| cmd.command.to_lowercase() == patch_key)
                .cloned();
            
            patches.insert(patch_key.clone(), Patch {
                name: patch_key,
                linked_command,
            });
        }
    }
    
    // Step 5: Create orphan anchor files and commands for patches without anchors
    let orphan_patches = find_orphan_patches(&patches, &commands);
    let orphans_created = if !orphan_patches.is_empty() {
        eprintln!("Found {} orphan patches: {:?}", orphan_patches.len(), orphan_patches);
        match create_orphan_anchors(&config, &orphan_patches, &mut commands) {
            Ok(count) => {
                eprintln!("Created {} orphan anchor files and commands", count);
                count
            }
            Err(e) => {
                eprintln!("Warning: Failed to create orphan anchors: {}", e);
                0
            }
        }
    } else {
        0
    };
    
    // Step 6: Save commands if any changes were made
    if patches_assigned > 0 || orphans_created > 0 {
        if let Err(e) = save_commands_to_file(&commands) {
            eprintln!("Warning: Failed to save commands after changes: {}", e);
        }
    }
    
    (config, commands, patches)
}

/// Loads commands from the commands.txt file without any processing
/// This is the raw loading function used by load_data()
pub fn load_commands_raw() -> Vec<Command> {
    let path = get_commands_file_path();
    
    if !path.exists() {
        eprintln!("Warning: commands.txt not found at {:?}", path);
        return vec![];
    }
    
    match fs::read_to_string(&path) {
        Ok(contents) => {
            let mut commands = Vec::new();
            for (line_num, line) in contents.lines().enumerate() {
                if line.trim().is_empty() {
                    continue;
                }
                
                match parse_command_line(line) {
                    Ok(command) => commands.push(command),
                    Err(e) => eprintln!("Warning: Failed to parse line {} in commands.txt: {} - Line: '{}'", 
                        line_num + 1, e, line),
                }
            }
            commands
        }
        Err(e) => {
            eprintln!("Error reading commands.txt: {}", e);
            vec![]
        }
    }
}

/// Legacy function for backward compatibility - now uses load_data()
pub fn load_commands() -> Vec<Command> {
    let (_config, commands, _patches) = load_data();
    commands
}

/// Parses a command line into a Command struct
pub fn parse_command_line(line: &str) -> Result<Command, String> {
    let trimmed = line.trim();
    if trimmed.is_empty() {
        return Err("Empty line".to_string());
    }
    
    // Check for new format: [GROUP! ]COMMAND : ACTION FLAGS; ARG
    if let Some(colon_pos) = trimmed.find(" : ") {
        let (prefix, rest) = trimmed.split_at(colon_pos);
        let rest = &rest[3..]; // Skip " : "
        
        // Parse group and command from prefix
        let (group, command) = if let Some(exclaim_pos) = prefix.find("! ") {
            let (g, c) = prefix.split_at(exclaim_pos);
            (g.trim().to_string(), c[2..].trim().to_string()) // Skip "! "
        } else {
            (String::new(), prefix.trim().to_string())
        };
        
        // Parse action, flags, and arg from rest using semicolon separator
        let (action_flags, arg) = if let Some(semicolon_pos) = rest.find(';') {
            let (af, a) = rest.split_at(semicolon_pos);
            (af.trim(), a[1..].trim()) // Skip ";"
        } else {
            // No semicolon found - treat everything as action
            (rest.trim(), "")
        };
        
        // Split action and flags (flags are space-separated after action)
        let action_flags_parts: Vec<&str> = action_flags.split_whitespace().collect();
        let action = if action_flags_parts.is_empty() {
            String::new()
        } else {
            action_flags_parts[0].to_string()
        };
        
        let flags = if action_flags_parts.len() > 1 {
            action_flags_parts[1..].join(" ")
        } else {
            String::new()
        };
        
        return Ok(Command {
            patch: group,
            command,
            action,
            arg: arg.to_string(),
            flags,
            full_line: line.to_string(),
        });
    }
    
    Err(format!("Invalid command format: missing ' : ' separator"))
}

/// Saves commands to file
pub fn save_commands_to_file(commands: &[Command]) -> Result<(), Box<dyn std::error::Error>> {
    // Create backup before saving
    backup_commands_file()?;
    
    let path = get_commands_file_path();
    
    // Ensure the directory exists
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    
    // Update cmd commands without patches to have "Cmd" patch
    let mut updated_commands = commands.to_vec();
    for cmd in &mut updated_commands {
        if cmd.action == "cmd" && cmd.patch.is_empty() {
            cmd.patch = "Cmd".to_string();
        }
    }
    
    // Sort commands by patch string first, then by command name before writing to file
    updated_commands.sort_by(|a, b| {
        match a.patch.cmp(&b.patch) {
            std::cmp::Ordering::Equal => a.command.cmp(&b.command),
            other => other
        }
    });
    
    // Convert all commands to new format and join with newlines
    let contents = updated_commands.iter()
        .map(|cmd| cmd.to_new_format())
        .collect::<Vec<_>>()
        .join("\n");
    
    fs::write(&path, contents)?;
    Ok(())
}

/// Adds a new command to the list and saves it
pub fn add_command(new_command: Command, commands: &mut Vec<Command>) -> Result<(), Box<dyn std::error::Error>> {
    commands.push(new_command);
    save_commands_to_file(commands)?;
    Ok(())
}

/// Deletes a command from the list and saves
pub fn delete_command(command_to_delete: &str, commands: &mut Vec<Command>) -> Result<(), Box<dyn std::error::Error>> {
    commands.retain(|cmd| cmd.command != command_to_delete);
    save_commands_to_file(commands)?;
    Ok(())
}

/// Filters commands based on search text with fuzzy matching
pub fn filter_commands(commands: &[Command], search_text: &str, max_results: usize, debug: bool) -> Vec<Command> {
    if search_text.is_empty() {
        return Vec::new();
    }
    
    let mut matched_commands: Vec<(i32, &Command)> = Vec::new(); // (match_end_index, command)
    
    for cmd in commands {
        // Use the core matching function to get match end position
        let match_result = command_matches_query_with_debug(&cmd.command, search_text, debug);
        
        if match_result >= 0 {
            matched_commands.push((match_result, cmd));
        }
    }
    
    // Sort by: 1) exact match first, 2) match position (earlier matches first), 3) word count (fewer words first), 4) alphabetical order
    matched_commands.sort_by(|a, b| {
        // Check for exact matches (case-insensitive)
        let is_exact_a = a.1.command.to_lowercase() == search_text.to_lowercase();
        let is_exact_b = b.1.command.to_lowercase() == search_text.to_lowercase();
        
        match (is_exact_a, is_exact_b) {
            (true, false) => std::cmp::Ordering::Less,    // a is exact, b is not - a comes first
            (false, true) => std::cmp::Ordering::Greater, // b is exact, a is not - b comes first
            _ => {
                // Both exact or both not exact - use match position
                match a.0.cmp(&b.0) {
                    std::cmp::Ordering::Equal => {
                        // If match position is the same, prefer commands with fewer words first
                        let word_count_a = count_words(&a.1.command, " ._-");
                        let word_count_b = count_words(&b.1.command, " ._-");
                        match word_count_a.cmp(&word_count_b) {
                            std::cmp::Ordering::Equal => a.1.command.cmp(&b.1.command), // Alphabetical if same word count
                            other => other // Fewer words first
                        }
                    },
                    other => other // Earlier match position first
                }
            }
        }
    });
    
    // Return sorted commands up to max_results
    matched_commands.into_iter()
        .take(max_results)
        .map(|(_, cmd)| cmd.clone())
        .collect()
}

/// Core matching function that returns the index where the match ends
/// Returns the position of the first unmatched character, or -1 if no match
pub fn command_matches_query_with_debug(command: &str, query: &str, _debug: bool) -> i32 {
    if query.is_empty() {
        return command.len() as i32;
    }
    
    let command_lower = command.to_lowercase();
    let query_lower = query.to_lowercase();
    let separators = " ._-";
    
    let cmd_chars: Vec<char> = command_lower.chars().collect();
    let query_chars: Vec<char> = query_lower.chars().collect();
    
    let mut cmd_idx = 0;
    let mut query_idx = 0;
    let mut last_match_pos = 0;
    
    while cmd_idx < cmd_chars.len() && query_idx < query_chars.len() {
        let cmd_char = cmd_chars[cmd_idx];
        let query_char = query_chars[query_idx];
        
        if cmd_char == query_char {
            // Characters match, advance both
            cmd_idx += 1;
            query_idx += 1;
            last_match_pos = cmd_idx;
        } else if separators.contains(cmd_char) {
            // Skip separator in command
            cmd_idx += 1;
        } else if separators.contains(query_char) {
            // Skip separator in query (handles "Book R" matching "Book To Read")
            query_idx += 1;
        } else {
            // No match - try to find next word boundary in command
            // This allows flexible matching across words
            let mut found_separator = false;
            while cmd_idx < cmd_chars.len() && !found_separator {
                if separators.contains(cmd_chars[cmd_idx]) {
                    found_separator = true;
                    cmd_idx += 1; // Skip the separator
                    break;
                }
                cmd_idx += 1;
            }
            
            if !found_separator {
                // No more word boundaries, no match
                return -1;
            }
        }
    }
    
    // If we matched all query characters, return the position
    if query_idx == query_chars.len() {
        last_match_pos as i32
    } else {
        -1
    }
}

/// Simple boolean version of the matching function
pub fn command_matches_query(command: &str, query: &str) -> bool {
    command_matches_query_with_debug(command, query, false) >= 0
}


/// Merges similar commands based on word removal approach (backward compatibility)
pub fn merge_similar_commands(commands: Vec<Command>, config: &Config) -> Vec<Command> {
    merge_similar_commands_with_context(commands, config, "")
}

/// Merges similar commands with awareness of search context
pub fn merge_similar_commands_with_context(commands: Vec<Command>, config: &Config, search_context: &str) -> Vec<Command> {
    if !config.popup_settings.merge_similar {
        return commands;
    }
    
    if commands.is_empty() {
        return commands;
    }
    
    let separators = &config.popup_settings.word_separators;
    
    // Step 1: Generate valid candidate strings by removing last word from each command
    let mut valid_candidates = std::collections::HashSet::new();
    for cmd in &commands {
        if let Some(candidate) = remove_last_word(&cmd.command, separators) {
            // Use position-based validation: merge only if there's a separator after the match position
            if is_valid_merge_candidate_by_position(&candidate, search_context, separators) {
                valid_candidates.insert(candidate);
            }
        }
    }
    
    // Step 2: Patch commands by matching them against candidates
    let mut groups: std::collections::HashMap<String, Vec<Command>> = std::collections::HashMap::new();
    let mut unmatched_commands = Vec::new();
    
    for cmd in commands {
        let mut matched = false;
        
        // Try direct match against candidates
        if valid_candidates.contains(&cmd.command) {
            groups.entry(cmd.command.clone()).or_insert_with(Vec::new).push(cmd.clone());
            matched = true;
        } else {
            // Try removing 1 word and matching against candidates
            if let Some(shortened) = remove_last_word(&cmd.command, separators) {
                if valid_candidates.contains(&shortened) {
                    groups.entry(shortened).or_insert_with(Vec::new).push(cmd.clone());
                    matched = true;
                }
            }
        }
        
        if !matched {
            unmatched_commands.push(cmd);
        }
    }
    
    // Step 3: Create final result
    let mut result = Vec::new();
    
    // Add merged groups (2+ commands) and single commands
    for (candidate, mut group) in groups {
        if group.len() >= 2 {
            // Create merged entry with " ..."
            group.sort_by(|a, b| a.command.cmp(&b.command));
            let base_command = &group[0];
            let mut merged_command = Command {
                patch: base_command.patch.clone(),
                command: format!("{} ...", candidate),
                action: base_command.action.clone(),
                arg: base_command.arg.clone(),
                flags: base_command.flags.clone(),
                full_line: format!("{} ...", candidate),
            };
            // Set the merge flag
            merged_command.set_flag('M', "");
            result.push(merged_command);
        } else {
            // Single command, add as-is
            result.extend(group);
        }
    }
    
    // Add unmatched commands
    result.extend(unmatched_commands);
    
    // Sort the final result to ensure stable ordering
    result.sort_by(|a, b| {
        // First by command length (shorter first)
        match a.command.len().cmp(&b.command.len()) {
            std::cmp::Ordering::Equal => a.command.cmp(&b.command), // Then alphabetically
            other => other
        }
    });
    
    result
}

/// Gets the prefix of a command based on word separators
pub fn get_command_prefix(command: &str, separators: &str) -> String {
    // Find the position of the first separator
    let sep_pos = command.chars()
        .position(|c| separators.contains(c))
        .unwrap_or(command.len());
    
    command[..sep_pos].to_string()
}

/// Capitalizes the first character of a string
fn capitalize_first_char(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + &chars.as_str().to_lowercase(),
    }
}

/// Removes the last word from a command string, returning the prefix
/// Example: "FIN Budget *" → Some("FIN Budget")
/// Example: "FIN" → None (can't remove last word)
fn remove_last_word(command: &str, separators: &str) -> Option<String> {
    // Find the last separator position
    let last_sep_pos = command.char_indices()
        .rev()
        .find(|(_, c)| separators.contains(*c))
        .map(|(pos, _)| pos);
    
    if let Some(pos) = last_sep_pos {
        let prefix = &command[..pos];
        if prefix.is_empty() {
            None
        } else {
            Some(prefix.to_string())
        }
    } else {
        // No separator found, can't remove last word
        None
    }
}

/// Counts the number of words in a string based on separators
fn count_words(text: &str, separators: &str) -> usize {
    if text.trim().is_empty() {
        return 0;
    }
    
    let mut word_count = 1; // Start with 1 for the first word
    for c in text.chars() {
        if separators.contains(c) {
            word_count += 1;
        }
    }
    word_count
}

/// Checks if a candidate is valid for merging based on match position
/// Returns true if the candidate has at least one separator after where search_context matches
fn is_valid_merge_candidate_by_position(candidate: &str, search_context: &str, separators: &str) -> bool {
    if search_context.is_empty() {
        return true; // Empty search context allows all merges
    }
    
    // Find where the search context matches in the candidate using our core matching function
    let match_end_pos = command_matches_query_with_debug(candidate, search_context, false);
    
    if match_end_pos < 0 {
        return false; // Search doesn't match this candidate
    }
    
    let match_end_pos = match_end_pos as usize;
    
    // Check if there's at least one separator after the match position
    let remaining_text = &candidate[match_end_pos..];
    
    for c in remaining_text.chars() {
        if separators.contains(c) {
            return true; // Found at least one separator after match
        }
    }
    
    false // No separator found after match position
}


/// Splits commands into submenu sections
pub fn split_commands(commands: &[Command], search_text: &str, separators: &str) -> Vec<Command> {
    if !search_text.contains(' ') {
        return commands.to_vec();
    }
    
    let prefix = search_text.split(' ').next().unwrap();
    get_submenu_commands(commands, prefix, separators)
}

/// Gets the current submenu prefix from search text and available commands
pub fn get_current_submenu_prefix_from_commands(commands: &[Command], search_text: &str, separators: &str) -> Option<String> {
    if search_text.is_empty() {
        return None;
    }
    
    // Extract the prefix to check (either the full search text or the part before space)
    let prefix_to_check = if search_text.contains(' ') {
        search_text.split(' ').next().unwrap()
    } else {
        search_text
    };
    
    // Don't auto-detect if prefix is very short (causes flickering)
    if prefix_to_check.len() < 2 {
        return None;
    }
    
    // Auto-detect submenu based on command prefixes
    // Use case-insensitive grouping to fix the case sensitivity bug
    let mut prefix_data: std::collections::HashMap<String, (usize, String)> = std::collections::HashMap::new();
    
    for cmd in commands {
        if cmd.action == "separator" {
            continue;
        }
        let cmd_prefix = get_command_prefix(&cmd.command, separators);
        // Only count exact prefix matches to avoid flickering between similar prefixes
        if cmd_prefix.to_lowercase() == prefix_to_check.to_lowercase() {
            let normalized_key = cmd_prefix.to_lowercase();
            let (count, best_case) = prefix_data.entry(normalized_key).or_insert((0, cmd_prefix.clone()));
            *count += 1;
            
            // Update best_case to prefer exact case match with prefix_to_check, then original case
            if cmd_prefix == prefix_to_check {
                *best_case = cmd_prefix;
            } else if best_case.to_lowercase() != prefix_to_check.to_lowercase() {
                *best_case = cmd_prefix;
            }
        }
    }
    
    // Find the best matching prefix (exact match preferred, then longest match)
    let mut best_prefix: Option<String> = None;
    let mut best_count = 0;
    
    for (_normalized_key, (count, prefix)) in prefix_data {
        if count >= 2 {
            let is_exact_match = prefix.to_lowercase() == prefix_to_check.to_lowercase();
            let should_use = best_prefix.is_none() || 
                            is_exact_match || 
                            (count > best_count) ||
                            (count == best_count && prefix.len() > best_prefix.as_ref().unwrap().len());
                            
            if should_use {
                // Normalize case to match prefix_to_check case for consistent display
                let normalized_prefix = if is_exact_match {
                    prefix_to_check.to_string()
                } else {
                    // Use the original prefix but try to match prefix_to_check case
                    if prefix_to_check.chars().next().unwrap_or('a').is_uppercase() {
                        capitalize_first_char(&prefix)
                    } else {
                        prefix.to_lowercase()
                    }
                };
                best_prefix = Some(normalized_prefix);
                best_count = count;
            }
        }
    }
    
    best_prefix
}

/// Gets the current submenu prefix from search text (legacy function for backward compatibility)
pub fn get_current_submenu_prefix(search_text: &str) -> Option<String> {
    if search_text.contains(' ') {
        Some(search_text.split(' ').next().unwrap().to_string())
    } else {
        None
    }
}

/// Gets commands for a submenu with the given prefix
pub fn get_submenu_commands(commands: &[Command], prefix: &str, separators: &str) -> Vec<Command> {
    let mut result = Vec::new();
    let mut inside_commands = Vec::new();
    let mut outside_commands = Vec::new();
    
    for cmd in commands {
        if cmd.action == "separator" {
            continue; // Skip existing separators
        }
        
        let cmd_prefix = get_command_prefix(&cmd.command, separators);
        if cmd_prefix.eq_ignore_ascii_case(prefix) {
            inside_commands.push(cmd.clone());
        } else {
            outside_commands.push(cmd.clone());
        }
    }
    
    // Sort inside commands by name for consistent ordering
    inside_commands.sort_by(|a, b| a.command.cmp(&b.command));
    
    // Add inside commands first
    result.extend(inside_commands);
    
    // Always add separator if we have inside commands (even if no outside commands)
    if !result.is_empty() {
        result.push(Command {
            patch: String::new(),
            command: "---".to_string(),
            action: "separator".to_string(),
            arg: String::new(),
            flags: String::new(),
            full_line: String::new(),
        });
    }
    
    // Sort outside commands by name for consistent ordering
    outside_commands.sort_by(|a, b| a.command.cmp(&b.command));
    
    // Add outside commands
    result.extend(outside_commands);
    
    result
}

/// Migrates commands to the new format (if needed)
pub fn migrate_commands_to_new_format(commands: &mut [Command]) {
    for cmd in commands.iter_mut() {
        cmd.update_full_line();
    }
}

/// Executes a command and returns the executed command (handling aliases)
pub fn execute_command(command: &Command) -> CommandTarget {
    execute_command_with_depth(command, 0)
}

/// Internal function to execute commands with recursion depth tracking
fn execute_command_with_depth(command: &Command, depth: u32) -> CommandTarget {
    const MAX_ALIAS_DEPTH: u32 = 10; // Prevent infinite recursion
    
    // Handle alias command type: execute the target command instead
    if command.action == "alias" {
        if depth >= MAX_ALIAS_DEPTH {
            crate::utils::debug_log("ALIAS", &format!("Maximum alias depth ({}) exceeded for '{}'", MAX_ALIAS_DEPTH, command.command));
            return CommandTarget::Command(command.clone());
        }
        
        // Load all commands to find the target
        let all_commands = load_commands();
        
        // Filter commands to find the best match for the alias target
        let filtered = filter_commands(&all_commands, &command.arg, 1, false);
        
        if !filtered.is_empty() {
            // Execute the first matching command
            let target_command = &filtered[0];
            crate::utils::debug_log("ALIAS", &format!("Alias '{}' executing target: {} (depth: {})", command.command, target_command.command, depth));
            return execute_command_with_depth(target_command, depth + 1); // Recursive call with depth tracking
        } else {
            crate::utils::debug_log("ALIAS", &format!("Alias '{}' target '{}' not found", command.command, command.arg));
            return CommandTarget::Command(command.clone());
        }
    }
    
    let launcher_cmd = format!("{} {}", command.action, command.arg);
    
    // Log command execution in the requested format
    crate::utils::debug_log("EXECUTE", &format!("'{}' AS '{}' ON '{}'", command.command, command.action, command.arg));
    
    // Log client environment only for actions that actually use it
    if uses_client_environment(&command.action) {
        log_client_environment();
    }
    
    // Save the last executed command for add_alias functionality
    use crate::core::state::save_last_executed_command;
    let _ = save_last_executed_command(&command.command);
    
    match crate::launcher::launch(&launcher_cmd) {
        Ok(()) => {
            // For rewrite commands, we need to handle the special case
            if command.action == "rewrite" {
                // The rewrite command should execute another command
                CommandTarget::Alias(command.arg.clone())
            } else {
                CommandTarget::Command(command.clone())
            }
        }
        Err(e) => {
            eprintln!("Failed to execute command: {:?}", e);
            CommandTarget::Command(command.clone())
        }
    }
}

/// Get commands formatted for display with submenu logic and separators
/// This is the single source of truth for how commands should be displayed
/// in the popup, CLI tools, and other interfaces.
pub fn get_display_commands(
    commands: &[Command], 
    search_text: &str, 
    config: &crate::core::config::Config,
    max_results: usize
) -> Vec<Command> {
    get_display_commands_with_options(commands, search_text, config, max_results, false)
}

/// Get filtered, merged, and sorted commands for display with options for alias expansion
pub fn get_display_commands_with_options(
    commands: &[Command], 
    search_text: &str, 
    config: &crate::core::config::Config,
    max_results: usize,
    expand_aliases: bool
) -> Vec<Command> {
    if search_text.is_empty() {
        return Vec::new();
    }
    
    // Apply initial filtering
    let filtered = filter_commands(commands, search_text, max_results * 2, false);
    
    // Check for submenu mode
    let final_commands = if let Some(menu_prefix) = get_current_submenu_prefix_from_commands(&filtered, search_text, &config.popup_settings.word_separators) {
        // SUBMENU MODE: Split first, then merge and sort each list separately
        let mut inside_commands = Vec::new();
        let mut outside_commands = Vec::new();
        
        // Split into inside and outside lists
        for cmd in filtered {
            if cmd.action == "separator" {
                continue; // Skip any existing separators
            }
            
            let cmd_prefix = get_command_prefix(&cmd.command, &config.popup_settings.word_separators);
            if cmd_prefix.eq_ignore_ascii_case(&menu_prefix) {
                inside_commands.push(cmd);
            } else {
                outside_commands.push(cmd);
            }
        }
        
        // Apply merging to each list separately if enabled
        if config.popup_settings.merge_similar {
            inside_commands = merge_similar_commands_with_context(inside_commands, config, search_text);
            outside_commands = merge_similar_commands_with_context(outside_commands, config, "");
        }
        
        // Combine: inside + separator + outside
        let mut result = inside_commands;
        
        if !result.is_empty() && !outside_commands.is_empty() {
            // Add single separator between inside and outside
            result.push(Command {
                patch: String::new(),
                command: "---".to_string(),
                action: "separator".to_string(),
                arg: String::new(),
                flags: String::new(),
                full_line: String::new(),
            });
        }
        
        result.extend(outside_commands);
        result
        
    } else {
        // NORMAL MODE: Don't merge or create separators when not in submenu mode
        // Just return the filtered commands sorted by our matching logic
        filtered
    };
    
    // Limit final results
    let mut limited_commands = final_commands;
    limited_commands.truncate(max_results);
    
    // If expand_aliases is true, expand alias commands while keeping their names
    if expand_aliases {
        limited_commands = limited_commands.into_iter().map(|cmd| {
            if cmd.action == "alias" {
                // Find the target command
                if let Some(target_cmd) = commands.iter().find(|c| c.command == cmd.arg) {
                    // Create a new command with the alias's name but the target's action/arg
                    Command {
                        patch: cmd.patch,
                        command: cmd.command, // Keep the alias's name
                        action: target_cmd.action.clone(), // Use target's action
                        arg: target_cmd.arg.clone(), // Use target's arg
                        flags: target_cmd.flags.clone(), // Use target's flags
                        full_line: cmd.full_line, // Keep alias's full line for display
                    }
                } else {
                    // Target not found, keep the alias as-is
                    cmd
                }
            } else {
                // Not an alias, return as-is
                cmd
            }
        }).collect();
    }
    
    limited_commands
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_command() {
        let line = "test : action; arg";
        let result = parse_command_line(line).unwrap();
        
        assert_eq!(result.command, "test");
        assert_eq!(result.action, "action");
        assert_eq!(result.arg, "arg");
        assert_eq!(result.flags, "");
        assert_eq!(result.patch, "");
    }

    #[test]
    fn test_parse_command_with_patch() {
        let line = "Patch! test command : action; argument here";
        let result = parse_command_line(line).unwrap();
        
        assert_eq!(result.patch, "Patch");
        assert_eq!(result.command, "test command");
        assert_eq!(result.action, "action");
        assert_eq!(result.arg, "argument here");
        assert_eq!(result.flags, "");
    }

    #[test]
    fn test_parse_command_with_flags() {
        let line = "test : action flag1 flag2; argument";
        let result = parse_command_line(line).unwrap();
        
        assert_eq!(result.command, "test");
        assert_eq!(result.action, "action");
        assert_eq!(result.flags, "flag1 flag2");
        assert_eq!(result.arg, "argument");
        assert_eq!(result.patch, "");
    }

    #[test]
    fn test_parse_command_with_group_and_flags() {
        let line = "Application! Chrome Test : chrome --incognito; https://example.com";
        let result = parse_command_line(line).unwrap();
        
        assert_eq!(result.patch, "Application");
        assert_eq!(result.command, "Chrome Test");
        assert_eq!(result.action, "chrome");
        assert_eq!(result.flags, "--incognito");
        assert_eq!(result.arg, "https://example.com");
    }

    #[test]
    fn test_parse_command_action_only() {
        let line = "test : action;";
        let result = parse_command_line(line).unwrap();
        
        assert_eq!(result.command, "test");
        assert_eq!(result.action, "action");
        assert_eq!(result.arg, "");
        assert_eq!(result.flags, "");
    }

    #[test]
    fn test_parse_command_with_flags_no_arg() {
        let line = "test : action flag1 flag2;";
        let result = parse_command_line(line).unwrap();
        
        assert_eq!(result.command, "test");
        assert_eq!(result.action, "action");
        assert_eq!(result.flags, "flag1 flag2");
        assert_eq!(result.arg, "");
    }

    #[test]
    fn test_parse_rewrite_command() {
        let line = "zzz : rewrite; cnn";
        let result = parse_command_line(line).unwrap();
        
        assert_eq!(result.command, "zzz");
        assert_eq!(result.action, "rewrite");
        assert_eq!(result.arg, "cnn");
        assert_eq!(result.flags, "");
    }

    #[test]
    fn test_parse_complex_1pass_command() {
        let line = "Application! Netflix 1Pass : 1pass --auto-fill; Netflix Account";
        let result = parse_command_line(line).unwrap();
        
        assert_eq!(result.patch, "Application");
        assert_eq!(result.command, "Netflix 1Pass");
        assert_eq!(result.action, "1pass");
        assert_eq!(result.flags, "--auto-fill");
        assert_eq!(result.arg, "Netflix Account");
    }

    #[test]
    fn test_format_generation_simple() {
        let cmd = Command {
            patch: String::new(),
            command: "test".to_string(),
            action: "action".to_string(),
            arg: "argument".to_string(),
            flags: String::new(),
            full_line: String::new(),
        };
        
        let formatted = cmd.to_new_format();
        assert_eq!(formatted, "test : action; argument");
    }

    #[test]
    fn test_format_generation_with_flags() {
        let cmd = Command {
            patch: String::new(),
            command: "test".to_string(),
            action: "action".to_string(),
            arg: "argument".to_string(),
            flags: "flag1 flag2".to_string(),
            full_line: String::new(),
        };
        
        let formatted = cmd.to_new_format();
        assert_eq!(formatted, "test : action flag1 flag2; argument");
    }

    #[test]
    fn test_format_generation_with_patch_and_flags() {
        let cmd = Command {
            patch: "Patch".to_string(),
            command: "test command".to_string(),
            action: "action".to_string(),
            arg: "argument here".to_string(),
            flags: "--flag".to_string(),
            full_line: String::new(),
        };
        
        let formatted = cmd.to_new_format();
        assert_eq!(formatted, "Patch! test command : action --flag; argument here");
    }

    #[test]
    fn test_roundtrip_parsing() {
        let original = "Application! Test Command : chrome --incognito; https://example.com";
        let parsed = parse_command_line(original).unwrap();
        let reformatted = parsed.to_new_format();
        let reparsed = parse_command_line(&reformatted).unwrap();
        
        assert_eq!(parsed.patch, reparsed.patch);
        assert_eq!(parsed.command, reparsed.command);
        assert_eq!(parsed.action, reparsed.action);
        assert_eq!(parsed.flags, reparsed.flags);
        assert_eq!(parsed.arg, reparsed.arg);
    }

    #[test]
    fn test_filter_commands_sorting_exact_match_first() {
        let commands = vec![
            Command {
                patch: String::new(),
                command: "Web25".to_string(),
                action: "action".to_string(),
                arg: "arg".to_string(),
                flags: String::new(),
                full_line: String::new(),
            },
            Command {
                patch: String::new(),
                command: "web".to_string(),
                action: "action".to_string(),
                arg: "arg".to_string(),
                flags: String::new(),
                full_line: String::new(),
            },
            Command {
                patch: String::new(),
                command: "Webshare".to_string(),
                action: "action".to_string(),
                arg: "arg".to_string(),
                flags: String::new(),
                full_line: String::new(),
            },
        ];

        let result = filter_commands(&commands, "web", 10, false);
        
        // Exact match "web" should come first
        assert_eq!(result[0].command, "web");
        assert_eq!(result[1].command, "Web25");
        assert_eq!(result[2].command, "Webshare");
    }

    #[test]
    fn test_filter_commands_sorting_word_count_before_alphabetical() {
        let commands = vec![
            Command {
                patch: String::new(),
                command: "test zebra final".to_string(), // 3 words, alphabetically first
                action: "action".to_string(),
                arg: "arg".to_string(),
                flags: String::new(),
                full_line: String::new(),
            },
            Command {
                patch: String::new(),
                command: "test apple".to_string(), // 2 words, alphabetically second
                action: "action".to_string(),
                arg: "arg".to_string(),
                flags: String::new(),
                full_line: String::new(),
            },
            Command {
                patch: String::new(),
                command: "testZ".to_string(), // 1 word, alphabetically last
                action: "action".to_string(),
                arg: "arg".to_string(),
                flags: String::new(),
                full_line: String::new(),
            },
        ];

        let result = filter_commands(&commands, "test", 10, false);
        
        // Should be sorted by word count first (fewer words first), then alphabetical
        assert_eq!(result[0].command, "testZ");         // 1 word comes first
        assert_eq!(result[1].command, "test apple");    // 2 words comes second  
        assert_eq!(result[2].command, "test zebra final"); // 3 words comes last
    }

    #[test]
    fn test_filter_commands_sorting_alphabetical_within_same_word_count() {
        let commands = vec![
            Command {
                patch: String::new(),
                command: "test zebra".to_string(),
                action: "action".to_string(),
                arg: "arg".to_string(),
                flags: String::new(),
                full_line: String::new(),
            },
            Command {
                patch: String::new(),
                command: "test apple".to_string(),
                action: "action".to_string(),
                arg: "arg".to_string(),
                flags: String::new(),
                full_line: String::new(),
            },
            Command {
                patch: String::new(),
                command: "test banana".to_string(),
                action: "action".to_string(),
                arg: "arg".to_string(),
                flags: String::new(),
                full_line: String::new(),
            },
        ];

        let result = filter_commands(&commands, "test", 10, false);
        
        // All have same word count, should be sorted alphabetically
        assert_eq!(result[0].command, "test apple");
        assert_eq!(result[1].command, "test banana");
        assert_eq!(result[2].command, "test zebra");
    }

    #[test]
    fn test_filter_commands_sorting_match_position_priority() {
        let commands = vec![
            Command {
                patch: String::new(),
                command: "some test here".to_string(), // "test" matches at position 5
                action: "action".to_string(),
                arg: "arg".to_string(),
                flags: String::new(),
                full_line: String::new(),
            },
            Command {
                patch: String::new(),
                command: "test something".to_string(), // "test" matches at position 0
                action: "action".to_string(),
                arg: "arg".to_string(),
                flags: String::new(),
                full_line: String::new(),
            },
        ];

        let result = filter_commands(&commands, "test", 10, false);
        
        // Earlier match position should come first
        assert_eq!(result[0].command, "test something"); // match at position 0
        assert_eq!(result[1].command, "some test here"); // match at position 5
    }

    #[test]
    fn test_filter_commands_comprehensive_sorting() {
        let commands = vec![
            Command {
                patch: String::new(),
                command: "Web App Store".to_string(), // 3 words, partial match
                action: "action".to_string(),
                arg: "arg".to_string(),
                flags: String::new(),
                full_line: String::new(),
            },
            Command {
                patch: String::new(),
                command: "WebZ".to_string(), // 1 word, partial match
                action: "action".to_string(),
                arg: "arg".to_string(),
                flags: String::new(),
                full_line: String::new(),
            },
            Command {
                patch: String::new(),
                command: "web".to_string(), // exact match
                action: "action".to_string(),
                arg: "arg".to_string(),
                flags: String::new(),
                full_line: String::new(),
            },
            Command {
                patch: String::new(),
                command: "Web Browser".to_string(), // 2 words, partial match
                action: "action".to_string(),
                arg: "arg".to_string(),
                flags: String::new(),
                full_line: String::new(),
            },
        ];

        let result = filter_commands(&commands, "web", 10, false);
        
        // Expected order:
        // 1. Exact match first: "web"
        // 2. Then by word count: 1-word "WebZ", 2-word "Web Browser", 3-word "Web App Store"
        assert_eq!(result[0].command, "web");           // exact match
        assert_eq!(result[1].command, "WebZ");          // 1 word
        assert_eq!(result[2].command, "Web Browser");   // 2 words
        assert_eq!(result[3].command, "Web App Store"); // 3 words
    }
}

/// Helper function to determine if an action uses the client environment
/// Returns true for actions that execute directly in the client process
fn uses_client_environment(action: &str) -> bool {
    match action {
        // Actions that use direct client execution (open commands, app launching)
        "app" | "url" | "folder" | "doc" | "chrome" | "safari" | "brave" | "firefox" | 
        "work" | "notion" | "obs_url" | "1pass" | "contact" | "open_with" => true,
        
        // Actions that might use client environment through JavaScript
        "anchor" | "slack" | "shutdown" | "rescan" => true,
        
        // Actions that use server environment (shell commands)
        "cmd" => false,
        
        // New markdown action uses JavaScript environment
        "markdown" => true,
        
        // JavaScript actions - these could use either, but many use client operations
        _ => true, // Default to showing client env for custom JavaScript functions
    }
}

/// Log client environment information (only when verbose logging is enabled)
fn log_client_environment() {
    crate::utils::verbose_log("CLIENT_ENV", &format!("PWD: {:?}", std::env::var("PWD")));
    crate::utils::verbose_log("CLIENT_ENV", &format!("PATH: {:?}", std::env::var("PATH")));
    crate::utils::verbose_log("CLIENT_ENV", &format!("USER: {:?}", std::env::var("USER")));
    crate::utils::verbose_log("CLIENT_ENV", &format!("HOME: {:?}", std::env::var("HOME")));
    crate::utils::verbose_log("CLIENT_ENV", &format!("DISPLAY: {:?}", std::env::var("DISPLAY")));
    crate::utils::verbose_log("CLIENT_ENV", &format!("TERM: {:?}", std::env::var("TERM")));
    crate::utils::verbose_log("CLIENT_ENV", &format!("SHELL: {:?}", std::env::var("SHELL")));
    crate::utils::verbose_log("CLIENT_ENV", &format!("Process ID: {}", std::process::id()));
    crate::utils::verbose_log("CLIENT_ENV", &format!("Current exe: {:?}", std::env::current_exe()));
}

// Include merge tests module
#[path = "commands_merge_tests.rs"]
#[cfg(test)]
mod merge_tests;