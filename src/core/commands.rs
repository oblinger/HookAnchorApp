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
pub fn filter_commands(commands: &[Command], search_text: &str, max_results: usize, _debug: bool) -> Vec<Command> {
    if search_text.is_empty() {
        return Vec::new();
    }
    
    let search_lower = search_text.to_lowercase();
    let mut scored_commands: Vec<(f32, &Command)> = Vec::new();
    
    for cmd in commands {
        let cmd_lower = cmd.command.to_lowercase();
        
        // Exact match gets highest score
        if cmd_lower == search_lower {
            scored_commands.push((1000.0, cmd));
            continue;
        }
        
        // Prefix match gets high score
        if cmd_lower.starts_with(&search_lower) {
            let score = 500.0 - (cmd_lower.len() - search_lower.len()) as f32;
            scored_commands.push((score, cmd));
            continue;
        }
        
        // Smart fuzzy matching
        if let Some(score) = fuzzy_match(&cmd_lower, &search_lower) {
            scored_commands.push((score, cmd));
        }
    }
    
    // Sort by score (highest first)
    scored_commands.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
    
    // Return sorted commands up to max_results
    scored_commands.into_iter()
        .take(max_results)
        .map(|(_, cmd)| cmd.clone())
        .collect()
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

/// Merges similar commands based on configuration
pub fn merge_similar_commands(commands: Vec<Command>, config: &Config) -> Vec<Command> {
    if !config.popup_settings.merge_similar {
        return commands;
    }
    
    if commands.is_empty() {
        return commands;
    }
    
    let separators = &config.popup_settings.word_separators;
    let mut merged = Vec::new();
    let mut current_prefix: Option<String> = None;
    let mut current_group: Vec<Command> = Vec::new();
    
    for cmd in commands {
        let prefix = get_command_prefix(&cmd.command, separators);
        
        if Some(&prefix) == current_prefix.as_ref() {
            current_group.push(cmd);
        } else {
            // Process the previous group
            if !current_group.is_empty() {
                add_merged_group(&mut merged, current_group, separators);
            }
            
            // Start new group
            current_prefix = Some(prefix);
            current_group = vec![cmd];
        }
    }
    
    // Don't forget the last group
    if !current_group.is_empty() {
        add_merged_group(&mut merged, current_group, separators);
    }
    
    merged
}

/// Gets the prefix of a command based on word separators
fn get_command_prefix(command: &str, separators: &str) -> String {
    // Find the position of the first separator
    let sep_pos = command.chars()
        .position(|c| separators.contains(c))
        .unwrap_or(command.len());
    
    command[..sep_pos].to_string()
}

/// Adds a group of similar commands to the merged list
fn add_merged_group(merged: &mut Vec<Command>, mut group: Vec<Command>, separators: &str) {
    if group.len() == 1 {
        merged.push(group.into_iter().next().unwrap());
        return;
    }
    
    // Sort group by command name for consistent ordering
    group.sort_by(|a, b| a.command.cmp(&b.command));
    
    // Add separator with the prefix
    let prefix = get_command_prefix(&group[0].command, separators);
    merged.push(Command {
        group: String::new(),
        command: format!("{}---", prefix),
        action: "separator".to_string(),
        arg: String::new(),
        flags: String::new(),
        full_line: String::new(),
    });
    
    // Add all commands in the group
    merged.extend(group);
}

/// Splits commands into submenu sections
pub fn split_commands(commands: &[Command], search_text: &str, separators: &str) -> Vec<Command> {
    if !search_text.contains(' ') {
        return commands.to_vec();
    }
    
    let prefix = search_text.split(' ').next().unwrap();
    get_submenu_commands(commands, prefix, separators)
}

/// Gets the current submenu prefix from search text
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
        let cmd_prefix = get_command_prefix(&cmd.command, separators);
        if cmd_prefix == prefix && cmd.action != "separator" {
            inside_commands.push(cmd.clone());
        } else if !cmd.command.starts_with(prefix) {
            outside_commands.push(cmd.clone());
        }
    }
    
    // Add inside commands first
    result.extend(inside_commands);
    
    // Add separator if we have both inside and outside commands
    if !result.is_empty() && !outside_commands.is_empty() {
        result.push(Command {
            group: String::new(),
            command: "---".to_string(),
            action: "separator".to_string(),
            arg: String::new(),
            flags: String::new(),
            full_line: String::new(),
        });
    }
    
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