//! Command management and operations
//! 
//! This module handles all command-related operations including loading, saving,
//! filtering, merging, and manipulation of commands.

use std::fs;
use std::path::{Path, PathBuf};
use std::env;
use serde::{Deserialize, Serialize};
use super::config::Config;

/// Represents a parsed command with its components and original line
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Command {
    pub group: String,
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

/// Mapping from flag letters to their word descriptions
pub const FLAG_LETTER_MAPPING: &[(&str, &str)] = &[
    ("M", "merge"),
    // Add more flag mappings here as needed
];

impl Command {
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
        if !self.group.is_empty() {
            result.push_str(&self.group);
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
pub fn load_commands() -> Vec<Command> {
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
            group,
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
    
    // Convert all commands to new format and join with newlines
    let contents = commands.iter()
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
    
    // Step 2: Group commands by matching them against candidates
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
                group: base_command.group.clone(),
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
            group: String::new(),
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
                group: String::new(),
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
        assert_eq!(result.group, "");
    }

    #[test]
    fn test_parse_command_with_group() {
        let line = "Group! test command : action; argument here";
        let result = parse_command_line(line).unwrap();
        
        assert_eq!(result.group, "Group");
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
        assert_eq!(result.group, "");
    }

    #[test]
    fn test_parse_command_with_group_and_flags() {
        let line = "Application! Chrome Test : chrome --incognito; https://example.com";
        let result = parse_command_line(line).unwrap();
        
        assert_eq!(result.group, "Application");
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
        
        assert_eq!(result.group, "Application");
        assert_eq!(result.command, "Netflix 1Pass");
        assert_eq!(result.action, "1pass");
        assert_eq!(result.flags, "--auto-fill");
        assert_eq!(result.arg, "Netflix Account");
    }

    #[test]
    fn test_format_generation_simple() {
        let cmd = Command {
            group: String::new(),
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
            group: String::new(),
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
    fn test_format_generation_with_group_and_flags() {
        let cmd = Command {
            group: "Group".to_string(),
            command: "test command".to_string(),
            action: "action".to_string(),
            arg: "argument here".to_string(),
            flags: "--flag".to_string(),
            full_line: String::new(),
        };
        
        let formatted = cmd.to_new_format();
        assert_eq!(formatted, "Group! test command : action --flag; argument here");
    }

    #[test]
    fn test_roundtrip_parsing() {
        let original = "Application! Test Command : chrome --incognito; https://example.com";
        let parsed = parse_command_line(original).unwrap();
        let reformatted = parsed.to_new_format();
        let reparsed = parse_command_line(&reformatted).unwrap();
        
        assert_eq!(parsed.group, reparsed.group);
        assert_eq!(parsed.command, reparsed.command);
        assert_eq!(parsed.action, reparsed.action);
        assert_eq!(parsed.flags, reparsed.flags);
        assert_eq!(parsed.arg, reparsed.arg);
    }

    #[test]
    fn test_filter_commands_sorting_exact_match_first() {
        let commands = vec![
            Command {
                group: String::new(),
                command: "Web25".to_string(),
                action: "action".to_string(),
                arg: "arg".to_string(),
                flags: String::new(),
                full_line: String::new(),
            },
            Command {
                group: String::new(),
                command: "web".to_string(),
                action: "action".to_string(),
                arg: "arg".to_string(),
                flags: String::new(),
                full_line: String::new(),
            },
            Command {
                group: String::new(),
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
                group: String::new(),
                command: "test zebra final".to_string(), // 3 words, alphabetically first
                action: "action".to_string(),
                arg: "arg".to_string(),
                flags: String::new(),
                full_line: String::new(),
            },
            Command {
                group: String::new(),
                command: "test apple".to_string(), // 2 words, alphabetically second
                action: "action".to_string(),
                arg: "arg".to_string(),
                flags: String::new(),
                full_line: String::new(),
            },
            Command {
                group: String::new(),
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
                group: String::new(),
                command: "test zebra".to_string(),
                action: "action".to_string(),
                arg: "arg".to_string(),
                flags: String::new(),
                full_line: String::new(),
            },
            Command {
                group: String::new(),
                command: "test apple".to_string(),
                action: "action".to_string(),
                arg: "arg".to_string(),
                flags: String::new(),
                full_line: String::new(),
            },
            Command {
                group: String::new(),
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
                group: String::new(),
                command: "some test here".to_string(), // "test" matches at position 5
                action: "action".to_string(),
                arg: "arg".to_string(),
                flags: String::new(),
                full_line: String::new(),
            },
            Command {
                group: String::new(),
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
                group: String::new(),
                command: "Web App Store".to_string(), // 3 words, partial match
                action: "action".to_string(),
                arg: "arg".to_string(),
                flags: String::new(),
                full_line: String::new(),
            },
            Command {
                group: String::new(),
                command: "WebZ".to_string(), // 1 word, partial match
                action: "action".to_string(),
                arg: "arg".to_string(),
                flags: String::new(),
                full_line: String::new(),
            },
            Command {
                group: String::new(),
                command: "web".to_string(), // exact match
                action: "action".to_string(),
                arg: "arg".to_string(),
                flags: String::new(),
                full_line: String::new(),
            },
            Command {
                group: String::new(),
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

// Include merge tests module
#[path = "commands_merge_tests.rs"]
#[cfg(test)]
mod merge_tests;