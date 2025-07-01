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
            if flag_part.starts_with(key) && flag_part.len() > 1 {
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
        
        // Add arg if present
        if !self.arg.is_empty() {
            result.push(' ');
            result.push_str(&self.arg);
        }
        
        // Add flags if present
        if !self.flags.is_empty() {
            result.push_str(" ,");
            result.push_str(&self.flags);
        }
        
        // Add trailing semicolon (new format requirement)
        result.push(';');
        
        result
    }
}

/// Returns the path to the commands.txt file
pub fn get_commands_file_path() -> PathBuf {
    let home = env::var("HOME").unwrap_or_else(|_| ".".to_string());
    Path::new(&home).join(".config/anchor_selector/commands.txt")
}

/// Returns the path to the backups folder
pub fn get_backups_folder_path() -> PathBuf {
    let home = env::var("HOME").unwrap_or_else(|_| ".".to_string());
    Path::new(&home).join(".config/anchor_selector/backups")
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
    
    // Remove trailing semicolon if present (new format compatibility)
    let trimmed = if trimmed.ends_with(';') {
        &trimmed[..trimmed.len() - 1]
    } else {
        trimmed
    };
    
    // Check for new format: [GROUP! ]COMMAND : ACTION [ARG] [,FLAGS]
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
        
        // Parse action, arg, and flags from rest
        let (action_arg, flags) = if let Some(comma_pos) = rest.find(" ,") {
            let (aa, f) = rest.split_at(comma_pos);
            (aa.trim(), f[2..].trim()) // Skip " ,"
        } else {
            (rest.trim(), "")
        };
        
        // Split action and arg
        let (action, arg) = if let Some(space_pos) = action_arg.find(' ') {
            let (a, arg_part) = action_arg.split_at(space_pos);
            (a.trim().to_string(), arg_part.trim().to_string())
        } else {
            (action_arg.to_string(), String::new())
        };
        
        return Ok(Command {
            group,
            command,
            action,
            arg,
            flags: flags.to_string(),
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
    
    // Sort by: 1) match position (earlier matches first), 2) alphabetical order
    matched_commands.sort_by(|a, b| {
        match a.0.cmp(&b.0) {
            std::cmp::Ordering::Equal => a.1.command.cmp(&b.1.command), // Alphabetical if same match length
            other => other // Earlier match position first
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

/// Fuzzy matching algorithm
fn fuzzy_match(text: &str, pattern: &str) -> Option<f32> {
    let mut pattern_chars = pattern.chars();
    let mut current_pattern_char = pattern_chars.next()?;
    let mut score = 0.0;
    let mut consecutive_matches = 0;
    let mut last_match_index = None;
    
    for (i, ch) in text.chars().enumerate() {
        if ch == current_pattern_char {
            // Bonus for consecutive matches
            consecutive_matches += 1;
            score += 10.0 * consecutive_matches as f32;
            
            // Bonus for match after word boundary
            if i == 0 || text.chars().nth(i - 1).map_or(false, |c| !c.is_alphanumeric()) {
                score += 20.0;
            }
            
            // Penalty for distance from last match
            if let Some(last_idx) = last_match_index {
                let gap = i - last_idx - 1;
                score -= gap as f32 * 0.5;
            }
            
            last_match_index = Some(i);
            
            // Move to next pattern character
            match pattern_chars.next() {
                Some(ch) => current_pattern_char = ch,
                None => return Some(score), // All pattern chars matched
            }
        } else {
            consecutive_matches = 0;
        }
    }
    
    None // Not all pattern characters were found
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
/// Example: "FIN Budget md" → Some("FIN Budget")
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

/// Adds a group of similar commands to the merged list
fn add_merged_group(merged: &mut Vec<Command>, mut group: Vec<Command>, separators: &str) {
    if group.len() == 1 {
        merged.push(group.into_iter().next().unwrap());
        return;
    }
    
    // Sort group by command name for consistent ordering
    group.sort_by(|a, b| a.command.cmp(&b.command));
    
    // Create a merged command with " ..." representing the group
    let prefix = get_command_prefix(&group[0].command, separators);
    
    // Use the first command as the base for the merged command
    let base_command = &group[0];
    let mut merged_command = Command {
        group: base_command.group.clone(),
        command: format!("{} ...", prefix), // This shows as "FIN.Budget ..." in the UI
        action: base_command.action.clone(),
        arg: base_command.arg.clone(),
        flags: base_command.flags.clone(),
        full_line: format!("{} ...", prefix),
    };
    // Set the merge flag
    merged_command.set_flag('M', "");
    merged.push(merged_command);
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
    
    // If search text contains space, use the first part as prefix
    if search_text.contains(' ') {
        return Some(search_text.split(' ').next().unwrap().to_string());
    }
    
    // Don't auto-detect if search text is very short (causes flickering)
    if search_text.len() < 2 {
        return None;
    }
    
    // Auto-detect submenu based on command prefixes
    let mut prefix_counts: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
    
    for cmd in commands {
        if cmd.action == "separator" {
            continue;
        }
        let cmd_prefix = get_command_prefix(&cmd.command, separators);
        if cmd_prefix.to_lowercase().starts_with(&search_text.to_lowercase()) {
            *prefix_counts.entry(cmd_prefix).or_insert(0) += 1;
        }
    }
    
    // Find the best matching prefix (exact match preferred, then longest match)
    let mut best_prefix: Option<String> = None;
    let mut best_count = 0;
    
    for (prefix, count) in prefix_counts {
        if count >= 2 {
            let is_exact_match = prefix.to_lowercase() == search_text.to_lowercase();
            let should_use = best_prefix.is_none() || 
                            is_exact_match || 
                            (count > best_count) ||
                            (count == best_count && prefix.len() > best_prefix.as_ref().unwrap().len());
                            
            if should_use {
                // Normalize case to match search text case for consistent display
                let normalized_prefix = if is_exact_match {
                    search_text.to_string()
                } else {
                    // Use the original prefix but try to match search text case
                    if search_text.chars().next().unwrap_or('a').is_uppercase() {
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
    let launcher_cmd = format!("{} {}", command.action, command.arg);
    
    match crate::launcher::launch(&launcher_cmd) {
        Ok(()) => {
            // For alias commands, we need to handle the special case
            if command.action == "alias" {
                // The alias command should execute another command
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