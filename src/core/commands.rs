//! Command management and operations
//! 
//! This module handles all command-related operations including loading, saving,
//! filtering, merging, and manipulation of commands.

use std::fs;
use std::path::{Path, PathBuf};
use std::env;
use std::collections::HashMap;
// sync imports removed - moved to sys_data module
use serde::{Deserialize, Serialize};
use crate::core::data::config::Config;
use crate::prelude::*;

// =============================================================================
// FORMAT VERSION
// =============================================================================

/// Current version of the commands.txt file format
/// Version 1.0: Key-value format with F:= and A:= syntax
/// Version 0.x (legacy): Old format without version header
pub const COMMANDS_FORMAT_VERSION: &str = "1.0";

// =============================================================================
// FLAG CONSTANTS
// =============================================================================

pub const FLAG_ANCHOR: char = 'A';       // command represents an anchor/hierarchy node
pub const FLAG_USER_EDITED: char = 'U';  // Prevents scanner overwrite of user edited entries
pub const FLAG_MERGED: char = 'M';       // Used when displaying merged ... entries
pub const FLAG_INCLUDE: char = 'I';      // Auto-includes all files in folder when viewing patch

// =============================================================================
// COMMAND STRUCT
// =============================================================================

/// Represents a parsed command with its components
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct  Command {
    pub patch: String,
    pub command: String,
    pub action: String,
    pub arg: String,
    pub flags: String,

    // Extended parameters (key-value pairs beyond standard fields)
    // Used for future extensibility (e.g., title, priority, color, etc.)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub other_params: Option<HashMap<String, String>>,

    // Metadata for history tracking
    #[serde(default)]
    pub last_update: i64,        // Unix timestamp - when this command last changed

    // File metadata (only for file-based actions: anchor/file/folder)
    #[serde(default)]
    pub file_size: Option<u64>,  // File size when we last checked
    // full_line removed per PRD - reconstructed when needed via to_new_format()
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
    /// Name of the patch (original case from anchor command)
    pub name: String,
    /// List of anchor commands for this patch (first one is the preferred/primary anchor)
    pub anchor_commands: Vec<Command>,
    /// Commands with the 'I' (include) flag that have this patch
    /// These commands' folders will be included in the prefix menu
    pub include_commands: Vec<Command>,
    /// Path to this patch's history file, if it exists
    pub history_file: Option<PathBuf>,
}

/// Performs case-insensitive lookup of a patch
/// Returns the patch object if found, None otherwise
pub fn get_patch<'a>(patch_name: &str, patches: &'a HashMap<String, Patch>) -> Option<&'a Patch> {
    let patch_name_lower = patch_name.to_lowercase();
    patches.get(&patch_name_lower)
}

impl Patch {
    /// Get the primary (preferred) anchor command for this patch
    /// Returns the first anchor in the list (which should be the preferred one)
    pub fn primary_anchor(&self) -> Option<&Command> {
        self.anchor_commands.first()
    }

    /// Get the parent patch name for this patch
    /// Returns the patch field from the primary anchor command
    pub fn parent_patch_name(&self) -> Option<String> {
        self.primary_anchor()
            .and_then(|cmd| {
                if cmd.patch.is_empty() {
                    None
                } else {
                    Some(cmd.patch.clone())
                }
            })
    }

    /// Get the children patch names for this patch from the patches HashMap
    /// Returns a vector of patch names whose parent is this patch
    pub fn children_patch_names(&self, patches: &HashMap<String, Patch>) -> Vec<String> {
        let self_name_lower = self.name.to_lowercase();
        let mut children = Vec::new();

        for (patch_name_lower, patch) in patches {
            if let Some(parent_name) = patch.parent_patch_name() {
                if parent_name.to_lowercase() == self_name_lower {
                    // Use the original case from the patch's name
                    children.push(patch.name.clone());
                }
            }
        }

        // Sort children alphabetically for consistent ordering
        children.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));
        children
    }

    /// Get the full path from orphans root to this patch
    /// Returns a vector of patch names from root to this patch
    ///
    /// TODO: Refactor get_patch_path() to use this method instead of duplicating logic
    pub fn get_path_from_root(&self, patches: &HashMap<String, Patch>) -> Vec<String> {
        get_patch_path(&self.name, patches)
    }

    /// Get all include folders from both anchor commands with 'I' flag and include_commands
    pub fn get_all_include_folders(&self, config: &crate::core::Config) -> std::collections::HashSet<std::path::PathBuf> {
        let mut include_folders = std::collections::HashSet::new();

        // Check anchor commands with include flag
        for anchor_cmd in &self.anchor_commands {
            if anchor_cmd.flags.contains(FLAG_INCLUDE) {
                if let Some(folder_path) = anchor_cmd.get_absolute_folder_path(config) {
                    include_folders.insert(folder_path);
                }
            }
        }

        // Check include commands
        for include_cmd in &self.include_commands {
            if let Some(folder_path) = include_cmd.get_absolute_folder_path(config) {
                include_folders.insert(folder_path);
            }
        }

        include_folders
    }
}

/// Iterator for walking up the patch hierarchy from child to root
/// Uses max depth limit instead of cycle detection for simplicity
pub struct PatchHierarchyIterator<'a> {
    patches: &'a HashMap<String, Patch>,
    current_patch_name: Option<String>,
    depth: usize,
    max_depth: usize,
}

impl<'a> PatchHierarchyIterator<'a> {
    pub fn new(start_patch: &str, patches: &'a HashMap<String, Patch>, max_depth: usize) -> Self {
        PatchHierarchyIterator {
            patches,
            current_patch_name: Some(start_patch.to_string()),
            depth: 0,
            max_depth,
        }
    }
}

impl<'a> Iterator for PatchHierarchyIterator<'a> {
    type Item = &'a Patch;

    fn next(&mut self) -> Option<Self::Item> {
        // Check depth limit
        if self.depth >= self.max_depth {
            return None;
        }

        // Get current patch name
        let patch_name = self.current_patch_name.as_ref()?;

        // Stop at "orphans"
        if patch_name.to_lowercase() == "orphans" {
            return None;
        }

        // Look up the patch
        let patch_key = patch_name.to_lowercase();
        let patch = self.patches.get(&patch_key)?;

        // Prepare next iteration - move to parent
        self.current_patch_name = patch.parent_patch_name();
        self.depth += 1;

        Some(patch)
    }
}

/// Helper function to create a patch hierarchy iterator
/// Walks from start_patch up to root, yielding each patch along the way
/// max_depth prevents infinite loops (recommended: 100)
pub fn walk_patch_hierarchy<'a>(
    start_patch: &str,
    patches: &'a HashMap<String, Patch>,
    max_depth: usize,
) -> PatchHierarchyIterator<'a> {
    PatchHierarchyIterator::new(start_patch, patches, max_depth)
}


/// Normalize a path by removing trailing slashes (except for root "/")
fn normalize_path(path: PathBuf) -> PathBuf {
    let path_str = path.to_string_lossy();
    if path_str.len() > 1 && path_str.ends_with('/') {
        // Remove trailing slash(es) except for root "/"
        PathBuf::from(path_str.trim_end_matches('/'))
    } else {
        path
    }
}

impl Command {
    /// Create a new Command with default metadata values
    pub fn new(patch: String, command: String, action: String, arg: String, flags: String) -> Self {
        Command {
            patch,
            command,
            action,
            arg,
            flags,
            other_params: None,
            last_update: 0,
            file_size: None,
        }
    }

    /// Returns the absolute file path for the command's argument
    /// Handles relative paths, tilde expansion, and vault-relative paths
    pub fn get_absolute_file_path(&self, config: &Config) -> Option<PathBuf> {
        let path = match self.action.as_str() {
            "markdown" => {
                // Arg is already absolute path for new markdown action
                Some(PathBuf::from(&self.arg))
            },
            "doc" => {
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
            "open" | "open_app" | "app" => {
                // Handle file paths (expand tilde for absolute, or assume relative to current dir)
                // For apps, arg is the path to the .app bundle (e.g., "/Applications/1Password.app")
                if self.arg.starts_with('/') || self.arg.starts_with('~') {
                    Some(PathBuf::from(crate::utils::expand_tilde(&self.arg)))
                } else {
                    // Relative to current working directory
                    Some(env::current_dir().ok()?.join(&self.arg))
                }
            }
            "cmd" => {
                // For cmd actions, try to extract directory from cd commands
                // Look for patterns like "cd /path" or "cd ~/path" or "cd path"
                if let Some(cd_pos) = self.arg.find("cd ") {
                    let after_cd = &self.arg[cd_pos + 3..];
                    // Find the path - it ends at && or ; or end of string
                    let path_end = after_cd.find(" &&")
                        .or_else(|| after_cd.find(" ;"))
                        .or_else(|| after_cd.find(';'))
                        .unwrap_or(after_cd.len());
                    
                    let path_str = after_cd[..path_end].trim().trim_matches('"');
                    if !path_str.is_empty() {
                        return Some(normalize_path(PathBuf::from(crate::utils::expand_tilde(path_str))));
                    }
                }
                None
            }
            _ => None // Not a file-based action
        };
        
        // Normalize all paths to remove trailing slashes
        path.map(normalize_path)
    }
    
    /// Returns the absolute folder path for the command
    /// For file-based commands, returns the parent directory
    /// For folder commands and directory-based anchors, returns the folder itself
    pub fn get_absolute_folder_path(&self, config: &Config) -> Option<PathBuf> {
        match self.action.as_str() {
            "folder" => {
                // For folder commands, return the folder itself (already normalized)
                self.get_absolute_file_path(config)
            }
            _ => {
                // For other file-based commands, return the parent directory
                // Parent path shouldn't have trailing slashes, but normalize just in case
                self.get_absolute_file_path(config)
                    .and_then(|p| p.parent().map(|parent| normalize_path(parent.to_path_buf())))
            }
        }
    }
    
    /// Checks if this command refers to a file or folder
    pub fn is_path_based(&self) -> bool {
        matches!(self.action.as_str(), "markdown" | "folder" | "doc" | "open" | "open_app" | "app")
    }

    /// Gets the value of a flag by its key character
    /// Returns Some("") if the flag exists, None if not
    /// In new format, flags are single characters only (e.g., "UA" not "U,A")
    pub fn get_flag(&self, key: char) -> Option<String> {
        if self.flags.is_empty() {
            return None;
        }

        // In new format, flags are concatenated single letters (e.g., "UA")
        // Just check if the character is present
        if self.flags.contains(key) {
            return Some(String::new());
        }
        None
    }
    
    /// Sets a flag by its key character
    /// In new format, flags are single characters only (no values)
    /// If value is provided, it's ignored (for backward compatibility)
    pub fn set_flag(&mut self, key: char, _value: &str) {
        // In new format, flags are just concatenated single letters (e.g., "UA")
        // If flag doesn't exist, add it
        if !self.flags.contains(key) {
            self.flags.push(key);
        }
        self.update_full_line();
    }
    
    /// Removes a flag by its key character
    pub fn remove_flag(&mut self, key: char) {
        if self.flags.is_empty() {
            return;
        }

        // In new format, flags are concatenated single letters (e.g., "UA")
        // Just remove the character from the string
        self.flags = self.flags.chars()
            .filter(|&c| c != key)
            .collect();
        self.update_full_line();
    }

    /// Checks if this command is an anchor
    /// Returns true if the anchor flag ('A') is set
    pub fn is_anchor(&self) -> bool {
        self.get_flag(FLAG_ANCHOR).is_some()
    }

    /// Sets or clears the anchor flag
    pub fn set_anchor(&mut self, is_anchor: bool) {
        if is_anchor {
            // Set the anchor flag (empty value means just the flag key)
            self.set_flag(FLAG_ANCHOR, "");
        } else {
            // Remove the anchor flag
            self.remove_flag(FLAG_ANCHOR);
        }
    }

    /// Returns true if the command has a real patch assigned
    /// Returns false if patch is empty or "orphans" (which are semantically equivalent - no patch assigned)
    pub fn has_patch(&self) -> bool {
        !self.patch.is_empty() && self.patch != "orphans"
    }

    /// Get the parent command for this command by walking up the patch hierarchy
    /// Returns None if this command has no patch, or the patch has no parent
    pub fn get_parent_command<'a>(&self, patches: &'a HashMap<String, Patch>) -> Option<&'a Command> {
        let patch = get_patch(&self.patch, patches)?;
        let parent_patch_name = patch.parent_patch_name()?;
        let parent_patch = get_patch(&parent_patch_name, patches)?;
        parent_patch.primary_anchor()
    }

    /// Updates the full_line field to reflect current command state in new format
    /// NOTE: With full_line removed from struct, this is now a no-op
    pub fn update_full_line(&mut self) {
        // No-op: full_line is reconstructed on demand via to_new_format()
    }
    
    /// Converts the command to new format string
    pub fn to_new_format(&self) -> String {
        let mut result = String::new();

        // Add patch if present
        if !self.patch.is_empty() {
            result.push_str(&self.patch);
            result.push_str("! ");
        }

        // Add command and action
        result.push_str(&self.command);
        result.push(':');
        result.push_str(&self.action);
        result.push(';');

        // Add key-value pairs after semicolon
        let mut kv_pairs = Vec::new();

        // Add F:=flags if present
        if !self.flags.is_empty() {
            // Escape any := in flags value
            let escaped_flags = self.flags.replace(":=", "\\:=");
            kv_pairs.push(format!("F:={}", escaped_flags));
        }

        // Add A:=arg if present
        if !self.arg.is_empty() {
            // Escape any := in arg value
            let escaped_arg = self.arg.replace(":=", "\\:=");
            kv_pairs.push(format!("A:={}", escaped_arg));
        }

        // Add other parameters if present
        if let Some(ref params) = self.other_params {
            for (key, value) in params {
                // Escape any := in value
                let escaped_value = value.replace(":=", "\\:=");
                kv_pairs.push(format!("{}:={}", key, escaped_value));
            }
        }

        // Join all KV pairs with space
        if !kv_pairs.is_empty() {
            result.push(' ');
            result.push_str(&kv_pairs.join(" "));
        }

        result
    }
    
    /// Resolves alias chains recursively, returning the final target command
    /// 
    /// If this command is not an alias, returns self.
    /// If this command is an alias, recursively follows the chain until reaching 
    /// a non-alias command or hitting the recursion limit.
    /// 
    /// Errors (recursion limit exceeded, cycles, missing targets) are logged and 
    /// the original command is returned - no error propagation needed.
    /// 
    /// # Arguments
    /// * `commands` - The full list of commands to search for alias targets
    /// 
    /// # Returns
    /// * `Command` - The final resolved command (or original command if resolution fails)
    /// 
    /// # Examples
    /// ```
    /// let resolved = command.resolve_alias(&all_commands);
    /// // Execute resolved command (safe to use directly)
    /// ```
    pub fn resolve_alias(&self, commands: &[Command]) -> Command {
        const MAX_ALIAS_DEPTH: usize = 100;
        match self.resolve_alias_with_depth(commands, 0, MAX_ALIAS_DEPTH, &mut std::collections::HashSet::new()) {
            Ok(resolved) => resolved,
            Err(error_msg) => {
                // Log the error only once per unique error message to avoid spam
                use std::sync::OnceLock;
                use std::collections::HashSet;
                use std::sync::Mutex;

                static LOGGED_ERRORS: OnceLock<Mutex<HashSet<String>>> = OnceLock::new();
                let logged_errors = LOGGED_ERRORS.get_or_init(|| Mutex::new(HashSet::new()));

                if let Ok(mut errors) = logged_errors.lock() {
                    if errors.insert(error_msg.clone()) {
                        // First time seeing this error, log it
                        log_error(&error_msg);
                    }
                }

                // Return the original command
                self.clone()
            }
        }
    }
    
    /// Internal recursive helper for resolve_alias with depth tracking and cycle detection
    fn resolve_alias_with_depth(
        &self, 
        commands: &[Command], 
        depth: usize, 
        max_depth: usize,
        visited: &mut std::collections::HashSet<String>
    ) -> Result<Command, String> {
        // Check recursion depth
        if depth >= max_depth {
            return Err(format!("Recursive aliases: exceeded maximum depth of {} for command '{}'", max_depth, self.command));
        }
        
        // Check for cycles
        if visited.contains(&self.command) {
            return Err(format!("Recursive aliases: cycle detected involving command '{}'", self.command));
        }
        
        // If not an alias, return self
        if self.action != "alias" {
            return Ok(self.clone());
        }
        
        // Add current command to visited set
        visited.insert(self.command.clone());
        
        // Find the target command that this alias points to
        // Try exact match first (fast), then case-insensitive if that fails
        let target_command = commands.iter()
            .find(|cmd| cmd.command == self.arg)
            .or_else(|| {
                let arg_lower = self.arg.to_lowercase();
                commands.iter().find(|cmd| cmd.command.to_lowercase() == arg_lower)
            })
            .ok_or_else(|| {
                format!("Recursive aliases: alias '{}' points to non-existent command '{}'", self.command, self.arg)
            })?;
        
        // Recursively resolve the target
        let resolved = target_command.resolve_alias_with_depth(commands, depth + 1, max_depth, visited)?;
        
        // Remove current command from visited set (backtrack)
        visited.remove(&self.command);
        
        Ok(resolved)
    }
}

/// Returns the path to the commands.txt file
// File path functions moved to storage module
// Use crate::core::data::storage::get_commands_file_path() instead

/// Loads commands from the commands.txt file
/// Creates a hashmap from patch names to Patch structs
/// Creates a patch entry for every anchor command, indexed by the command's lowercase name
/// INTERNAL ONLY - External code should use get_sys_data() to get patches
pub fn create_patches_hashmap(commands: &[Command]) -> HashMap<String, Patch> {
    let mut patches = HashMap::new();
    let config = crate::core::data::get_config();
    let preferred_anchor_type = config.popup_settings.preferred_anchor.as_deref().unwrap_or("markdown");
    
    // First pass: Group all anchor commands by their normalized name
    let mut anchor_groups: HashMap<String, Vec<Command>> = HashMap::new();
    for command in commands {
        if command.is_anchor() {
            let patch_key = command.command.to_lowercase();
            anchor_groups.entry(patch_key).or_insert_with(Vec::new).push(command.clone());
        }
    }
    
    // Second pass: For each group of anchors, select preferred anchor and create patch
    for (patch_key, mut anchor_commands) in anchor_groups {
        if anchor_commands.is_empty() {
            continue;
        }
        
        // Sort anchors to put preferred action type first
        anchor_commands.sort_by(|a, b| {
            let a_action = crate::execute::get_action_for_arg(&a.arg);
            let b_action = crate::execute::get_action_for_arg(&b.arg);
            
            // Preferred action type comes first
            if a_action == preferred_anchor_type && b_action != preferred_anchor_type {
                std::cmp::Ordering::Less
            } else if a_action != preferred_anchor_type && b_action == preferred_anchor_type {
                std::cmp::Ordering::Greater
            } else {
                // Secondary sort by command name for stability
                a.command.cmp(&b.command)
            }
        });
        
        // Create patch with preferred anchor first, but store all anchors
        let primary_anchor = &anchor_commands[0];
        patches.insert(patch_key, Patch {
            name: primary_anchor.command.clone(), // Use primary anchor's case
            anchor_commands,
            include_commands: Vec::new(),
            history_file: None, // Will be populated by history module
        });
    }
    
    // Third pass: Add commands with include flag to their patch's include list
    for command in commands {
        if command.flags.contains(FLAG_INCLUDE) && !command.patch.is_empty() {
            let patch_key = command.patch.to_lowercase();
            if let Some(patch) = patches.get_mut(&patch_key) {
                patch.include_commands.push(command.clone());
            }
        }
    }

    // NOTE: Virtual anchor CREATION removed from here - it was creating entries in the
    // patches HashMap but not adding them to the commands list, causing them to be lost on save.
    // Virtual anchor creation now happens exclusively in validate_and_repair_patches() Phase 3,
    // which properly adds them to both the commands list AND the patches HashMap.

    patches
}

/// Infers the appropriate patch value for a command based on various heuristics
/// 1. First checks if the first word matches any existing patch name (case-insensitive)
/// 2. Then checks if the command refers to a file in a folder associated with a patch
/// 3. If it's an anchor file, walks hierarchy looking for containing patch
/// Returns None if no patch can be inferred
/// Always analyzes the command regardless of any current patch value
pub fn infer_patch(command: &Command, patches: &HashMap<String, Patch>) -> Option<String> {
    crate::core::inference::infer_patch(command, patches)
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
    
    // Check if this is an anchor file FIRST
    let path = Path::new(file_path);
    if crate::utils::is_anchor_file(path) {
        // This IS an anchor file (e.g., RR/Lrn/Lrn.md)
        if let Some(file_stem) = path.file_stem() {
            if let Some(parent) = path.parent() {
                if let Some(parent_name) = parent.file_name() {
                    detailed_log("ANCHOR_INFERENCE", &format!(
                        "File {} is an anchor (stem '{}' matches folder '{}'). Looking for parent patch.",
                        file_path, file_stem.to_string_lossy(), parent_name.to_string_lossy()
                    ));
                }
            }
        }
        // For anchors, walk up the hierarchy to find parent patch
        if let Some(parent) = path.parent() {
            if let Some(result) = infer_patch_from_hierarchy(parent, patches) {
                detailed_log("ANCHOR_INFERENCE", &format!(
                    "Found parent patch '{}' for anchor '{}'",
                    result, exclude_command
                ));
                return Some(result);
            }
            detailed_log("ANCHOR_INFERENCE", &format!(
                "No parent patch found for anchor '{}', will check other methods",
                exclude_command
            ));
        }
    }
    
    // Method 1: Check all path components and return the most specific (deepest) non-self match
    // For paths like "T/Misc/Sleep.md", prefer "Misc" over "T"
    // This works for both relative and absolute paths
    let components: Vec<&str> = file_path.split('/').collect();
    let mut best_matches: Vec<(usize, String)> = Vec::new(); // (depth, patch_name)
    
    for (depth, component) in components.iter().enumerate() {
        if get_patch(component, patches).is_some() {
            // Found a patch match - add to list
            best_matches.push((depth, component.to_string()));
        }
    }
    
    // Sort by depth (deepest first) and return first non-self match
    best_matches.sort_by(|a, b| b.0.cmp(&a.0)); // Sort by depth descending
    
    for (_, patch_name) in best_matches {
        if patch_name.to_lowercase() != exclude_command.to_lowercase() {
            // Return the correct case from the patch's linked command, not the directory name
            if let Some(patch) = get_patch(&patch_name, patches) {
                if let Some(linked_cmd) = patch.primary_anchor() {
                    return Some(linked_cmd.command.clone());
                }
            }
            return Some(patch_name); // Fallback to original if lookup fails
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
        if let Some(linked_cmd) = patch.primary_anchor() {
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
                            // Check if this is an anchor file
                            if crate::utils::is_anchor_file(path) {
                                // This IS an anchor file (e.g., Lrn/Lrn.md)
                                // Walk hierarchy to find the containing patch (should be grandparent)
                                return infer_patch_from_hierarchy(dir, patches);
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


/// Helper function specifically for anchors to find their parent patch
fn infer_patch_from_hierarchy_for_anchor(command: &Command, patches: &HashMap<String, Patch>) -> Option<String> {
    if !command.is_path_based() {
        return None;
    }
    
    let path = Path::new(&command.arg);
    let dir = if path.is_file() {
        path.parent()?
    } else {
        path
    };
    
    infer_patch_from_hierarchy(dir, patches)
}

/// Walks directory hierarchy looking for a patch that contains this folder
fn infer_patch_from_hierarchy(dir: &Path, patches: &HashMap<String, Patch>) -> Option<String> {
    let mut current_dir = dir.parent();
    
    while let Some(parent) = current_dir {
        // Check if any patch is associated with this parent directory
        for patch in patches.values() {
            if let Some(linked_cmd) = patch.primary_anchor() {
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
    // Load all commands and resolve the alias chain
    let all_commands = load_commands_raw();
    let resolved_command = command.resolve_alias(&all_commands);
    
    // If target has a patch, return it
    if !resolved_command.patch.is_empty() {
        return Some(resolved_command.patch.clone());
    }
    
    // If target doesn't have a patch, try to infer one for it
    // (but only if it's not the original command to avoid infinite recursion)
    if resolved_command.command != command.command {
        return infer_patch(&resolved_command, patches);
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
    crate::core::inference::auto_assign_patches(commands);
}

/// Determines if replacing current_patch with inferred_patch would be a degradation
/// 
/// This prevents --infer-all from replacing specific, well-organized patches with generic ones.
/// 
/// # Arguments
/// * `current_patch` - The existing patch on the command
/// * `inferred_patch` - The patch that inference wants to assign
/// 
/// # Returns
/// * `true` if this would be a degradation (should be prevented)
/// * `false` if this would be an improvement or neutral change
fn is_patch_degradation(current_patch: &str, inferred_patch: &str) -> bool {
    // List of generic/low-quality patches that shouldn't replace specific ones
    let generic_patches = [
        "old", "Old", "OLD",
        "misc", "Misc", "MISC", 
        "other", "Other", "OTHER",
        "test", "Test", "TEST",
        "temp", "Temp", "TEMP",
        "archive", "Archive", "ARCHIVE",
        "tmp", "Tmp", "TMP"
    ];
    
    // If the inferred patch is generic, don't replace a non-generic current patch
    if generic_patches.contains(&inferred_patch) && !generic_patches.contains(&current_patch) {
        return true;
    }
    
    // If current patch contains a year (2000-2030) and inferred doesn't, it's likely a degradation
    let current_has_year = current_patch.chars()
        .collect::<Vec<char>>()
        .windows(4)
        .any(|window| {
            let year_str: String = window.iter().collect();
            if let Ok(year) = year_str.parse::<u32>() {
                year >= 2000 && year <= 2030
            } else {
                false
            }
        });
    
    let inferred_has_year = inferred_patch.chars()
        .collect::<Vec<char>>()
        .windows(4)
        .any(|window| {
            let year_str: String = window.iter().collect();
            if let Ok(year) = year_str.parse::<u32>() {
                year >= 2000 && year <= 2030
            } else {
                false
            }
        });
    
    if current_has_year && !inferred_has_year {
        return true;
    }
    
    // If current patch is significantly longer and more specific, prefer it
    if current_patch.len() > inferred_patch.len() + 5 {
        // Only consider it degradation if the current patch has multiple words/parts
        if current_patch.contains(' ') || current_patch.contains('-') || current_patch.contains('_') {
            return true;
        }
    }
    
    // Not a degradation
    false
}

// NOTE: build_folder_to_patch_map() moved to SysData layer
// The folder_to_patch map is now maintained in SysData and queried via get_patch_for_folder()

// NOTE: infer_patch_simple() removed - use crate::core::inference::infer_patch_simple() instead
// The function now queries SysData's folder_to_patch map instead of taking it as a parameter

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
    
    log(&format!("=== PATCH INFERENCE ==="));
    log(&format!("Processing {} commands", commands.len()));

    // Note: folder → patch mapping is now maintained in SysData and queried via get_patch_for_folder()

    for command in commands.iter_mut() {
        // Skip the orphans root command - it should never have a patch
        if command.command == "orphans" {
            continue;
        }
        
        // Skip system-generated virtual anchor commands (empty action) - they should always keep their "orphans" patch
        // Real file-based anchors in orphans SHOULD be processed for inference
        if command.patch == "orphans" && command.is_anchor() && command.action.is_empty() && !command.flags.contains(FLAG_USER_EDITED) {
            detailed_log("PATCH_INFERENCE", &format!(
                "Command '{}' -> SKIPPED (virtual anchor in orphans)",
                command.command
            ));
            continue;
        }
        
        // Run inference on commands that need it
        // Skip user-edited commands UNLESS they don't have a patch assigned
        // (those need help finding the right patch)
        let should_process = !command.flags.contains(FLAG_USER_EDITED)
            || !command.has_patch();

        if should_process {
            detailed_log("PATCH_INFERENCE_DETAIL", &format!(
                "Processing command '{}' (path_based={}, arg='{}', has_patch={})",
                command.command, command.is_path_based(), command.arg, command.has_patch()
            ));

            // LEGACY PATCH INFERENCE - replaced with unified algorithm
            // let inferred_patch = if command.is_path_based() && !command.arg.is_empty() {
            //     detailed_log("PATCH_INFERENCE_DETAIL", &format!(
            //         "  Using infer_patch_simple with arg '{}'",
            //         command.arg
            //     ));
            //     infer_patch_simple(&command.arg, &folder_map)
            // } else {
            //     detailed_log("PATCH_INFERENCE_DETAIL",
            //         "  Using infer_patch (legacy inference)");
            //     // Fall back to old inference for non-path commands
            //     infer_patch(command, patches)
            // };

            // Use unified patch inference algorithm (queries SysData's folder_to_patch map)
            let inferred_patch = crate::core::inference::infer_patch_unified(command, patches);
            
            if let Some(inferred_patch) = inferred_patch {
                // Skip if patch wouldn't change - always skip when values are the same
                if command.patch == inferred_patch {
                    continue;
                }
                
                // CRITICAL: Check for self-reference (cycle prevention)
                if command.is_anchor() && inferred_patch.to_lowercase() == command.command.to_lowercase() {
                    log(&format!("  ❌ CYCLE PREVENTED: Anchor '{}' cannot have itself as patch '{}'", 
                        command.command, inferred_patch));
                    continue;
                }
                
                // ANTI-DEGRADATION PROTECTION: Don't replace specific patches with generic ones
                if !command.patch.is_empty() {
                    if is_patch_degradation(&command.patch, &inferred_patch) {
                        // Skip this change - would degrade patch quality
                        detailed_log("PATCH_INFERENCE", &format!(
                            "Command '{}' -> REJECTED degradation from '{}' to '{}'",
                            command.command, command.patch, inferred_patch
                        ));
                        continue;
                    }
                }
                
                let old_patch_display = if command.patch.is_empty() { "(empty)".to_string() } else { command.patch.clone() };

                // Apply changes - we already checked should_process (non-user-edited)
                // All non-user-edited commands get their patches re-inferred to stay up-to-date
                if apply_changes {
                    // Debug: Log when we're about to assign a patch
                    if inferred_patch.is_empty() {
                        log(&format!("  ⚠️ WARNING: About to assign EMPTY patch to command '{}' (was: '{}')",
                            command.command, old_patch_display));
                    }
                    command.patch = inferred_patch.clone();
                    detailed_log("PATCH_INFERENCE", &format!("Inferred patch for '{}': {} -> {}",
                        command.command, old_patch_display, inferred_patch));
                }

                if print_to_stdout {
                    detailed_log("PATCH_INFERENCE", &format!("{}: {} -> {}", command.command, old_patch_display, inferred_patch));
                }

                patches_assigned += 1;
                
                // Track new patches to add later - use original case for new patch names
                if get_patch(&inferred_patch, patches).is_none() && !new_patches_to_add.contains(&inferred_patch) {
                    // Store the original case for new patches, not lowercase
                    new_patches_to_add.push(inferred_patch.clone());
                }
            }
        }
    }
    
    // Log final state of anchors for debugging
    log(&format!("\n=== FINAL ANCHOR STATES ==="));
    for cmd in commands.iter().filter(|c| c.is_anchor()) {
        if cmd.command.to_lowercase() == "lrn" || cmd.arg.contains("Lrn") {
            log(&format!("  Anchor '{}': patch='{}', arg='{}'", 
                cmd.command, cmd.patch, cmd.arg));
        }
    }
    log(&format!("=== END INFERENCE ===\n"));
    
    (patches_assigned, new_patches_to_add)
}

/// Creates a fast lookup map from command names to their patch references
/// This provides O(1) lookup for finding the patch associated with a command
pub(crate) fn create_command_to_patch_map(commands: &[Command], patches: &HashMap<String, Patch>) -> HashMap<String, String> {
    let mut command_to_patch_map = HashMap::new();
    
    for command in commands {
        if !command.patch.is_empty() {
            if get_patch(&command.patch, patches).is_some() {
                command_to_patch_map.insert(command.command.clone(), command.patch.clone());
            }
        }
    }
    
    command_to_patch_map
}

/// Case-insensitive patch lookup - the canonical way to check if a patch exists
/// Returns true if a patch with the given name exists (case-insensitive)
pub(crate) fn patch_exists(patch_name: &str, patches: &HashMap<String, Patch>) -> bool {
    get_patch(patch_name, patches).is_some()
}

/// Gets the patch for a command using fast lookup
/// Returns None if the command has no patch or the patch doesn't exist
pub fn get_patch_for_command<'a>(command_name: &str, patches: &'a HashMap<String, Patch>) -> Option<&'a Patch> {
    // First try direct patch lookup
    get_patch(command_name, patches)
        .or_else(|| {
            // Try to find by command name if not found by direct lookup
            patches.values()
                .find(|patch| patch.primary_anchor().as_ref()
                    .map_or(false, |cmd| cmd.command.to_lowercase() == command_name.to_lowercase()))
        })
}

/// Normalizes patch case in commands to match the case of their anchor commands
/// Returns the number of patches that were normalized
pub(crate) fn normalize_patch_case(commands: &mut [Command], patches: &HashMap<String, Patch>) -> usize {
    let mut normalized_count = 0;
    
    for command in commands {
        if !command.patch.is_empty() {
            // Find the corresponding patch
            if let Some(patch) = get_patch(&command.patch, patches) {
                // Check if the patch has a linked command to get the proper case
                if let Some(linked_cmd) = patch.primary_anchor() {
                    // Get the properly cased command name from the anchor
                    let proper_case = linked_cmd.command.clone();
                    
                    // IMPORTANT: Only normalize if the current patch is actually incorrect case
                    // Don't shorten longer patch names to shorter anchor names
                    // This prevents "2023 SV Patents" from being normalized to "2023"
                    if command.patch != proper_case && command.patch.to_lowercase() == proper_case.to_lowercase() {
                        detailed_log("AUTO_NORMALIZE", &format!("Normalized patch case for '{}': '{}' -> '{}'", 
                            command.command, command.patch, proper_case));
                        command.patch = proper_case;
                        command.update_full_line();
                        normalized_count += 1;
                    }
                }
            }
        }
    }
    
    normalized_count
}

/// Get the patch path from a command to the root
/// Returns a vector of patch names from the given command up to the root (excluding orphans)
/// Uses the walk_patch_hierarchy iterator with max depth of 100 to prevent infinite loops
pub fn get_patch_path(command_name: &str, patches: &HashMap<String, Patch>) -> Vec<String> {
    let mut path = Vec::new();

    // Walk up the hierarchy using the iterator
    for patch in walk_patch_hierarchy(command_name, patches, 100) {
        // Don't include "orphans" in the path as it's the root
        if patch.name.to_lowercase() != "orphans" {
            // Use the original case from the patch's primary anchor command
            if let Some(primary_cmd) = patch.primary_anchor() {
                path.push(primary_cmd.command.clone());
            } else {
                // Fallback to patch name if no primary anchor
                path.push(patch.name.clone());
            }
        }
    }

    // Reverse the path so it goes from root to command (FOO -> BAR -> BAZ -> CMD)
    path.reverse();
    path
}



/// Create a virtual anchor command for a patch that has no anchor
/// This ensures every patch has at least one anchor command
/// Infers the parent patch by finding the longest matching prefix in existing patches
fn create_virtual_anchor_for_patch(patch_name: &str, _config: &Config, patches: &HashMap<String, Patch>) -> Option<Command> {
    // Infer parent patch by finding longest matching prefix
    let parent_patch = infer_parent_patch_from_name(patch_name, patches);

    detailed_log("VIRTUAL_ANCHOR", &format!(
        "Creating virtual anchor for '{}' with parent '{}'",
        patch_name, parent_patch
    ));

    // Create the virtual anchor command - no file path needed, just blank arg
    Some(Command {
        command: patch_name.to_string(),
        action: String::new(),  // Virtual anchor - blank action (non-executable)
        arg: String::new(), // NEW SYSTEM: Blank arg, no markdown file
        patch: parent_patch,
        flags: "A".to_string(), // Anchor flag (system-generated)
        other_params: None,
        last_update: 0,
        file_size: None,
    })
}

/// Infer parent patch from a patch name by finding the longest matching prefix
/// Example: "2016-00-00 ppp" -> looks for "2016-00", "2016", etc.
fn infer_parent_patch_from_name(patch_name: &str, patches: &HashMap<String, Patch>) -> String {
    let patch_name_lower = patch_name.to_lowercase();

    // Try progressively shorter prefixes, looking for matches in existing patches
    // Split on common separators: space, dash, underscore
    let parts: Vec<&str> = patch_name
        .split(|c: char| c == ' ' || c == '-' || c == '_')
        .collect();

    // Try from longest to shortest prefix
    for len in (1..parts.len()).rev() {
        let prefix = parts[..len].join(" ");
        let prefix_lower = prefix.to_lowercase();

        // Also try with original separators
        if patches.contains_key(&prefix_lower) {
            detailed_log("PARENT_INFERENCE", &format!(
                "Found parent '{}' for patch '{}'", prefix, patch_name
            ));
            return prefix;
        }

        // Try with dashes instead of spaces
        let prefix_dash = parts[..len].join("-");
        let prefix_dash_lower = prefix_dash.to_lowercase();
        if patches.contains_key(&prefix_dash_lower) {
            detailed_log("PARENT_INFERENCE", &format!(
                "Found parent '{}' for patch '{}'", prefix_dash, patch_name
            ));
            return prefix_dash;
        }
    }

    // No parent found - use orphans
    detailed_log("PARENT_INFERENCE", &format!(
        "No parent found for patch '{}' - using orphans", patch_name
    ));
    "orphans".to_string()
}

// GlobalData has been moved to sys_data module as SysData

/// Loads commands from the commands.txt file without any processing
/// This is the raw loading function used by sys_data::load_data()
/// Load commands from commands.txt (delegates to storage layer)
/// DEPRECATED: External code should use crate::core::data::storage::load_commands_raw()
/// This wrapper exists for internal backwards compatibility only
pub(crate) fn load_commands_raw() -> Vec<Command> {
    crate::core::data::load_commands_raw()
}

/// Load commands with all derived data structures (patches, inference, orphan anchors)
/// INTERNAL ONLY - External code should use get_commands() from sys_data
pub(in crate::core) fn load_commands() -> Vec<Command> {
    let (global_data, _) = crate::core::data::get_sys_data();
    (*global_data.commands).clone()
}

/// Load commands with config and patches
/// INTERNAL ONLY - External code should use get_sys_data()
pub(in crate::core) fn load_commands_with_data() -> (crate::core::data::config::Config, Vec<Command>, HashMap<String, Patch>) {
    let (global_data, _) = crate::core::data::get_sys_data();
    (global_data.config, (*global_data.commands).clone(), global_data.patches)
}

/// Load commands with only patches (no inference or orphan creation) - for inference analysis
pub fn load_commands_for_inference() -> (crate::core::data::config::Config, Vec<Command>, HashMap<String, Patch>) {
    // Step 1: Load config
    let config = crate::core::data::get_config();
    
    // Step 2: Load commands
    let commands = load_commands_raw();
    
    // Step 3: Create patches hashmap from anchors only
    let patches = create_patches_hashmap(&commands);
    
    (config, commands, patches)
}

// =============================================================================
// KEY-VALUE PARSER HELPERS (for new format with F:=, A:=, etc.)
// =============================================================================
//
// NOTE: parse_kv_pairs() has been moved to crate::utils::parse_kv_pairs()
// Use that function instead of duplicating the logic here.

/// Parse command line in new KV format: PATCH! NAME:ACTION; F:=flags A:=arg [KEY:=VALUE ...]
fn parse_command_line_v2(line: &str) -> Result<Command, String> {
    let trimmed = line.trim();
    if trimmed.is_empty() {
        return Err("Empty line".to_string());
    }

    // Step 1: Parse PATCH! NAME:ACTION; prefix (same as old format)
    let colon_pos = trimmed.find(':').ok_or("Missing ':' separator")?;
    let (prefix, rest) = trimmed.split_at(colon_pos);
    let rest = &rest[1..]; // Skip the ':'

    // Parse patch and command from prefix
    let (patch, command) = if let Some(exclaim_pos) = prefix.find("! ") {
        let (p, c) = prefix.split_at(exclaim_pos);
        (p.trim().to_string(), c[2..].trim().to_string())
    } else {
        (String::new(), prefix.trim().to_string())
    };

    // Step 2: Split on semicolon to separate action from KV pairs
    let semicolon_pos = rest.find(';').ok_or("Missing ';' separator")?;
    let (action_part, kv_part) = rest.split_at(semicolon_pos);
    let action = action_part.trim().to_string();
    let kv_part = &kv_part[1..]; // Skip the ';'

    // Step 3: Parse KV pairs
    let kv_map = crate::utils::parse_kv_pairs(kv_part);

    // Step 4: Extract standard fields and other params
    // Sanitize flags: remove commas, spaces, and any other invalid characters
    // Flags should only be single letters concatenated together (e.g., "UA", not "U,A" or "U A")
    let flags = kv_map.get("F")
        .map(|f| f.chars()
            .filter(|c| c.is_alphabetic())
            .collect::<String>())
        .unwrap_or_default();
    let arg = kv_map.get("A").cloned().unwrap_or_default();

    // Collect remaining keys into other_params
    let other_params = if kv_map.keys().any(|k| k != "F" && k != "A") {
        let mut params = HashMap::new();
        for (k, v) in kv_map {
            if k != "F" && k != "A" {
                params.insert(k, v);
            }
        }
        Some(params)
    } else {
        None
    };

    Ok(Command {
        patch,
        command,
        action,
        arg,
        flags,
        other_params,
        last_update: 0,
        file_size: None,
    })
}

/// Parses a command line into a Command struct
/// Auto-detects format: uses v2 parser if := is found, otherwise uses legacy parser
pub fn parse_command_line(line: &str) -> Result<Command, String> {
    let trimmed = line.trim();
    if trimmed.is_empty() {
        return Err("Empty line".to_string());
    }

    // Auto-detect format: new format contains := for key-value pairs
    if trimmed.contains(":=") {
        return parse_command_line_v2(line);
    }

    // LEGACY FORMAT BELOW - will be removed after migration

    // Check for old format: [GROUP! ]COMMAND:ACTION FLAGS; ARG
    // Colon is invalid in command names, so we just look for the first colon
    if let Some(colon_pos) = trimmed.find(':') {
        let (prefix, rest) = trimmed.split_at(colon_pos);
        // Skip the colon and any optional spaces after it
        let rest = &rest[1..].trim_start();
        
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
        
        // Debug: Log when we parse a command with an empty patch where we might expect one
        if group.is_empty() && line.contains('!') {
            detailed_log("EMPTY_PATCH_BUG", &format!("WARNING: Parsed command with empty patch despite '!' in line: '{}'", line));
        }
        
        return Ok(Command {
            patch: group,
            command,
            action,
            arg: arg.to_string(),
            flags,
            other_params: None,
            last_update: 0,
            file_size: None,
        });
    }
    
    Err(format!("Invalid command format: missing ' : ' separator"))
}

/// Saves commands to file
/// Deduplicates commands by keeping the best version of each command
/// Priority: commands with patches > commands without patches
/// Then by flags: commands with flags > commands without flags
fn deduplicate_commands(commands: Vec<Command>) -> Vec<Command> {
    use std::collections::HashMap;
    
    let original_count = commands.len();
    let mut best_commands: HashMap<(String, String, String), Command> = HashMap::new();
    
    for command in commands {
        // Use (command_name, action, arg) as key to identify truly identical commands
        // This preserves different files with same base name but different paths
        let key = (command.command.clone(), command.action.clone(), command.arg.clone());
        
        match best_commands.get(&key) {
            Some(existing) => {
                // Keep the better command based on priority
                if is_better_command(&command, existing) {
                    detailed_log("AUTO_DEDUP", &format!("Replacing '{}' (patch:'{}' flags:'{}') with better version (patch:'{}' flags:'{}')",
                        command.command, existing.patch, existing.flags, command.patch, command.flags));
                    best_commands.insert(key, command);
                } else {
                    detailed_log("AUTO_DEDUP", &format!("Keeping existing '{}' (patch:'{}' flags:'{}') over (patch:'{}' flags:'{}')",
                        command.command, existing.patch, existing.flags, command.patch, command.flags));
                }
            }
            None => {
                best_commands.insert(key, command);
            }
        }
    }
    
    let deduplicated_commands = best_commands.into_values().collect::<Vec<_>>();
    let removed_count = original_count - deduplicated_commands.len();
    if removed_count > 0 {
        detailed_log("AUTO_DEDUP", &format!("Removed {} duplicate commands ({} -> {})", removed_count, original_count, deduplicated_commands.len()));
    }
    deduplicated_commands
}

/// Determines if candidate is better than current command
/// Priority: 1. Has patch > no patch, 2. Has flags > no flags, 3. Longer arg > shorter arg
fn is_better_command(candidate: &Command, current: &Command) -> bool {
    // Priority 1: Commands with patches are better than commands without patches
    match (candidate.patch.is_empty(), current.patch.is_empty()) {
        (false, true) => return true,  // candidate has patch, current doesn't
        (true, false) => return false, // current has patch, candidate doesn't
        _ => {} // Both have patches or both don't, continue to next criteria
    }
    
    // Priority 2: Commands with flags are better than commands without flags
    match (candidate.flags.is_empty(), current.flags.is_empty()) {
        (false, true) => return true,  // candidate has flags, current doesn't
        (true, false) => return false, // current has flags, candidate doesn't
        _ => {} // Both have flags or both don't, continue to next criteria
    }
    
    // Priority 3: Commands with longer args (more complete paths) are better
    if candidate.arg.len() > current.arg.len() {
        return true;
    }
    
    // Default: keep current
    false
}

/// Save commands to file (delegates to storage layer for actual file I/O)
/// DEPRECATED: This function is exported for backwards compatibility but delegates to storage
pub fn save_commands_to_file(commands: &[Command]) -> Result<(), Box<dyn std::error::Error>> {
    // Delegate to storage layer which handles the actual file I/O
    crate::core::data::save_commands_to_file(commands)?;
    Ok(())
}

// Old implementation removed - now delegated to storage layer
fn _old_save_implementation_removed(commands: &[Command]) -> Result<(), Box<dyn std::error::Error>> {
    let path = crate::core::data::get_commands_file_path();
    
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
    
    // Deduplicate commands by keeping the one with the most complete information
    // Priority: commands with patches > commands without patches
    // Then by flags: commands with flags > commands without flags
    updated_commands = deduplicate_commands(updated_commands);
    
    // Sort commands by patch string first, then by command name before writing to file
    updated_commands.sort_by(|a, b| {
        match a.patch.cmp(&b.patch) {
            std::cmp::Ordering::Equal => a.command.cmp(&b.command),
            other => other
        }
    });
    
    // Debug: Check what patches look like right before serialization
    let mut empty_patch_count = 0;
    let mut empty_patch_commands = Vec::new();
    for cmd in &updated_commands {
        if cmd.patch.is_empty() {
            empty_patch_count += 1;
            empty_patch_commands.push(cmd.command.clone());
            
            // Only log as potential bug for actions that typically need patches
            let actions_that_need_patches = ["markdown", "doc", "cmd"];
            if actions_that_need_patches.contains(&cmd.action.as_str()) {
                detailed_log("EMPTY_PATCH_BUG", &format!("Command with EMPTY patch during save: '{}' (action: {}, arg: {})", 
                    cmd.command, cmd.action, cmd.arg));
            }
        }
        // Skip logging individual commands
    }
    // Commands ready to save
    
    // Skip logging empty patch commands

    // SAFETY CHECKS: Prevent saving corrupted data
    // Updated for DOC commands: DOC scanning can legitimately add thousands of commands
    if updated_commands.len() > 10000 {
        let error_msg = format!("CORRUPTION DETECTED: Attempting to save {} commands (> 10000 limit). This indicates command inflation. Save operation CANCELLED.", updated_commands.len());
        log_error(&error_msg);
        detailed_log("CORRUPTION", &error_msg);
        return Err("Command count exceeds safety limit".into());
    }
    
    if empty_patch_count > 200 {
        let error_msg = format!("CORRUPTION DETECTED: Attempting to save {} commands with empty patches (> 200 limit). This indicates patch stripping. Save operation CANCELLED.", empty_patch_count);
        log_error(&error_msg);
        detailed_log("CORRUPTION", &error_msg);
        return Err("Empty patch count exceeds safety limit".into());
    }
    
    // Build file contents with version header
    let mut contents = String::new();

    // Add version header as first line
    contents.push_str(&format!("// HookAnchor Commands Format - version:={}\n", COMMANDS_FORMAT_VERSION));

    // Convert all commands to new format and join with newlines
    let commands_text = updated_commands.iter()
        .map(|cmd| cmd.to_new_format())
        .collect::<Vec<_>>()
        .join("\n");

    contents.push_str(&commands_text);

    // Write with better error handling that includes the file path
    if let Err(e) = fs::write(&path, &contents) {
        let error_msg = format!("Cannot write to file '{}': {}", path.display(), e);
        log_error(&error_msg);
        return Err(error_msg.into());
    }
    
    detailed_log("AUTO_SAVE", &format!("Saved {} commands to {}", commands.len(), path.display()));
    Ok(())
}

// REMOVED: Legacy cache functions moved to storage.rs
// Use crate::core::data::storage functions instead


/// Filters commands based on search text with fuzzy matching and patch support
pub(crate) fn filter_commands_with_patch_support(commands: &[Command], search_text: &str, max_results: usize, _word_separators: &str, debug: bool) -> Vec<Command> {
    if search_text.is_empty() {
        return Vec::new();
    }
    
    
    let mut matched_commands: Vec<(i32, &Command)> = Vec::new(); // (match_end_index, command)
    
    for cmd in commands {
        // First try normal command name matching
        let name_match_result = crate::core::display::command_matches_query_with_debug(&cmd.command, search_text, debug);
        
        // Also try patch matching if this might be a patch name (short search text)
        let patch_match_result = if search_text.len() <= 3 {
            // First check for direct patch match (without requiring '!')
            if cmd.patch.eq_ignore_ascii_case(search_text) {
                // Perfect patch match - include this command even if name doesn't match
                if debug && search_text.eq_ignore_ascii_case("ww") {
                    detailed_log("COMMANDS", &format!("Found exact patch match: {} -> {}", cmd.patch, cmd.command));
                }
                0
            } else if cmd.patch.contains('!') {
                // Also check patches with '!' format (extract part before '!')
                let patch_name = cmd.patch.split('!').next().unwrap_or("");
                if patch_name.eq_ignore_ascii_case(search_text) {
                    // Perfect patch match - include this command even if name doesn't match
                    0
                } else {
                    crate::core::display::command_matches_query_with_debug(patch_name, search_text, debug)
                }
            } else {
                -1
            }
        } else {
            -1
        };
        
        // Include command if EITHER name matches OR patch matches
        let match_result = if patch_match_result >= 0 && name_match_result >= 0 {
            // Both patch and command name match - highest priority
            // Check for exact matches: both command and patch equal search text
            if cmd.command.eq_ignore_ascii_case(search_text) && 
               (cmd.patch.eq_ignore_ascii_case(search_text) || 
                cmd.patch.split('!').next().unwrap_or("").eq_ignore_ascii_case(search_text)) {
                -10 // Exact match gets highest priority
            } else {
                // Both match but not exact - use the better of the two scores
                std::cmp::min(patch_match_result, name_match_result)
            }
        } else if patch_match_result >= 0 {
            // Patch matched - use patch match result and prioritize it
            patch_match_result
        } else if name_match_result >= 0 {
            // Only name matched
            name_match_result + 100 // Add penalty to prioritize patch matches
        } else {
            // Neither matched
            -1
        };
        
        if match_result >= 0 {
            matched_commands.push((match_result, cmd));
        }
    }
    
    // Sort by match quality (earlier match end = better match)
    matched_commands.sort_by(|(a_end, a_cmd), (b_end, b_cmd)| {
        // Special case: prioritize exact name matches
        let a_exact = a_cmd.command.eq_ignore_ascii_case(search_text);
        let b_exact = b_cmd.command.eq_ignore_ascii_case(search_text);
        if a_exact && !b_exact {
            return std::cmp::Ordering::Less;
        }
        if !a_exact && b_exact {
            return std::cmp::Ordering::Greater;
        }
        
        // Primary sort: match end position (earlier is better)
        let end_cmp = a_end.cmp(b_end);
        if end_cmp != std::cmp::Ordering::Equal {
            return end_cmp;
        }
        
        // Secondary sort: command name length (shorter is usually better)
        let len_cmp = a_cmd.command.len().cmp(&b_cmd.command.len());
        if len_cmp != std::cmp::Ordering::Equal {
            return len_cmp;
        }
        
        // Tertiary sort: alphabetical by command name
        a_cmd.command.to_lowercase().cmp(&b_cmd.command.to_lowercase())
    });
    
    // Return sorted commands up to max_results
    let results: Vec<Command> = matched_commands.into_iter()
        .take(max_results)
        .map(|(score, cmd)| {
            // Debug logging for AT search results
            if search_text.eq_ignore_ascii_case("AT") && score <= 0 {
                detailed_log("AT_RESULTS", &format!("Result with score {}: cmd={}, patch={}", score, cmd.command, cmd.patch));
            }
            cmd.clone()
        })
        .collect();
    
    // Log if At command is missing from results
    if search_text.eq_ignore_ascii_case("AT") && !results.iter().any(|c| c.command.eq_ignore_ascii_case("At")) {
        detailed_log("AT_MISSING", "At command not in final results!");
    }
    
    results
}

/// Filters commands based on search text with fuzzy matching (legacy function)
pub fn filter_commands(commands: &[Command], search_text: &str, max_results: usize, debug: bool) -> Vec<Command> {
    if search_text.is_empty() {
        return Vec::new();
    }
    
    let mut matched_commands: Vec<(i32, &Command)> = Vec::new(); // (match_end_index, command)
    
    for cmd in commands {
        // Use the core matching function to get match end position
        let name_match_result = crate::core::display::command_matches_query_with_debug(&cmd.command, search_text, debug);
        
        // Also try patch matching for short searches that could be patch names
        let patch_match_result = if search_text.len() <= 3 {
            // First check for direct patch match (without requiring '!')
            if cmd.patch.eq_ignore_ascii_case(search_text) {
                0 // Perfect patch match
            } else if cmd.patch.contains('!') {
                // Also check patches with '!' format (extract part before '!')
                let patch_name = cmd.patch.split('!').next().unwrap_or("");
                if patch_name.eq_ignore_ascii_case(search_text) {
                    0 // Perfect patch match
                } else {
                    crate::core::display::command_matches_query_with_debug(patch_name, search_text, debug)
                }
            } else {
                -1
            }
        } else {
            -1
        };
        
        // Include if either name or patch matches
        let match_result = if patch_match_result >= 0 {
            patch_match_result
        } else if name_match_result >= 0 {
            name_match_result + 100 // Prioritize patch matches
        } else {
            -1
        };
        
        if match_result >= 0 {
            // Debug logging for AT command specifically
            if search_text.eq_ignore_ascii_case("AT") && cmd.command.eq_ignore_ascii_case("At") {
                detailed_log("AT_MATCH", &format!("Adding At command with score {}: cmd='{}', patch='{}', action='{}'", match_result, cmd.command, cmd.patch, cmd.action));
            }
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
        other_params: None,
        last_update: 0,
        file_size: None,
            };
            // Set the merge flag
            merged_command.set_flag(FLAG_MERGED, "");
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
    let match_end_pos = crate::core::display::command_matches_query_with_debug(candidate, search_context, false);
    
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
    fn test_parse_cmd_action_command() {
        let line = "zzz : cmd; echo test";
        let result = parse_command_line(line).unwrap();
        
        assert_eq!(result.command, "zzz");
        assert_eq!(result.action, "cmd");
        assert_eq!(result.arg, "echo test");
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
        other_params: None,
        last_update: 0,
        file_size: None,
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
        other_params: None,
        last_update: 0,
        file_size: None,
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
        other_params: None,
        last_update: 0,
        file_size: None,
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
        other_params: None,
        last_update: 0,
        file_size: None,
                },
            Command {
                patch: String::new(),
                command: "web".to_string(),
                action: "action".to_string(),
                arg: "arg".to_string(),
                flags: String::new(),
        other_params: None,
        last_update: 0,
        file_size: None,
                },
            Command {
                patch: String::new(),
                command: "Webshare".to_string(),
                action: "action".to_string(),
                arg: "arg".to_string(),
                flags: String::new(),
        other_params: None,
        last_update: 0,
        file_size: None,
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
        other_params: None,
        last_update: 0,
        file_size: None,
                },
            Command {
                patch: String::new(),
                command: "test apple".to_string(), // 2 words, alphabetically second
                action: "action".to_string(),
                arg: "arg".to_string(),
                flags: String::new(),
        other_params: None,
        last_update: 0,
        file_size: None,
                },
            Command {
                patch: String::new(),
                command: "testZ".to_string(), // 1 word, alphabetically last
                action: "action".to_string(),
                arg: "arg".to_string(),
                flags: String::new(),
        other_params: None,
        last_update: 0,
        file_size: None,
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
        other_params: None,
        last_update: 0,
        file_size: None,
                },
            Command {
                patch: String::new(),
                command: "test apple".to_string(),
                action: "action".to_string(),
                arg: "arg".to_string(),
                flags: String::new(),
        other_params: None,
        last_update: 0,
        file_size: None,
                },
            Command {
                patch: String::new(),
                command: "test banana".to_string(),
                action: "action".to_string(),
                arg: "arg".to_string(),
                flags: String::new(),
        other_params: None,
        last_update: 0,
        file_size: None,
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
        other_params: None,
        last_update: 0,
        file_size: None,
                },
            Command {
                patch: String::new(),
                command: "test something".to_string(), // "test" matches at position 0
                action: "action".to_string(),
                arg: "arg".to_string(),
                flags: String::new(),
        other_params: None,
        last_update: 0,
        file_size: None,
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
        other_params: None,
        last_update: 0,
        file_size: None,
                },
            Command {
                patch: String::new(),
                command: "WebZ".to_string(), // 1 word, partial match
                action: "action".to_string(),
                arg: "arg".to_string(),
                flags: String::new(),
        other_params: None,
        last_update: 0,
        file_size: None,
                },
            Command {
                patch: String::new(),
                command: "web".to_string(), // exact match
                action: "action".to_string(),
                arg: "arg".to_string(),
                flags: String::new(),
        other_params: None,
        last_update: 0,
        file_size: None,
                },
            Command {
                patch: String::new(),
                command: "Web Browser".to_string(), // 2 words, partial match
                action: "action".to_string(),
                arg: "arg".to_string(),
                flags: String::new(),
        other_params: None,
        last_update: 0,
        file_size: None,
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
