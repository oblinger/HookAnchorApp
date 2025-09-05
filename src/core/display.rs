//! Display logic for commands and command filtering
//!
//! This module handles basic display-related operations for commands including:
//! - Basic query matching and scoring
//! - Submenu prefix detection and navigation
//! - Command prefix extraction

use super::Command;

/// Check if a command matches a query string (compatibility wrapper)
pub fn command_matches_query_with_debug(command: &str, query: &str, debug: bool) -> i32 {
    if command_matches_query_string(command, query, debug) {
        1
    } else {
        0
    }
}

/// Check if a command string matches a query 
pub fn command_matches_query_string(command: &str, query: &str, _debug: bool) -> bool {
    if query.trim().is_empty() {
        return true;
    }
    
    let query_lower = query.to_lowercase();
    let command_lower = command.to_lowercase();
    
    // Exact match (highest priority)
    if command_lower == query_lower {
        return true;
    }
    
    // Prefix match (high priority) 
    if command_lower.starts_with(&query_lower) {
        return true;
    }
    
    // Word boundary matches
    let command_words: Vec<&str> = command_lower.split_whitespace().collect();
    let query_words: Vec<&str> = query_lower.split_whitespace().collect();
    
    // Check if all query words match command words (in order)
    if query_words.len() <= command_words.len() {
        let mut query_idx = 0;
        for cmd_word in &command_words {
            if query_idx < query_words.len() && cmd_word.starts_with(query_words[query_idx]) {
                query_idx += 1;
            }
        }
        if query_idx == query_words.len() {
            return true;
        }
    }
    
    // Substring match (lower priority)
    command_lower.contains(&query_lower)
}

/// Check if a Command struct matches a query
pub fn command_matches_query(command: &str, query: &str) -> bool {
    command_matches_query_string(command, query, false)
}

/// Extract the prefix from a command string with custom separators
pub fn get_command_prefix(command: &str, separators: &str) -> String {
    // If no separators specified, use first word
    if separators.is_empty() {
        return command.split_whitespace()
            .next()
            .unwrap_or(command)
            .to_string();
    }
    
    // Find the first separator and extract prefix
    let mut end_pos = command.len();
    for sep_char in separators.chars() {
        if let Some(pos) = command.find(sep_char) {
            end_pos = end_pos.min(pos);
        }
    }
    
    command[..end_pos].trim().to_string()
}

/// Split commands based on search text and separators
pub fn split_commands(commands: &[Command], search_text: &str, separators: &str) -> Vec<Command> {
    if search_text.is_empty() {
        return commands.to_vec();
    }
    
    // Find commands that match the search text
    commands.iter()
        .filter(|cmd| command_matches_query_string(&cmd.command, search_text, false))
        .cloned()
        .collect()
}

/// Get current submenu prefix from commands using search text and separators
pub fn get_current_submenu_prefix_from_commands(
    commands: &[Command], 
    search_text: &str, 
    separators: &str
) -> Option<String> {
    if commands.is_empty() || search_text.is_empty() {
        return None;
    }
    
    // Get all command prefixes
    let prefixes: Vec<String> = commands.iter()
        .map(|cmd| get_command_prefix(&cmd.command, separators))
        .collect();
    
    // Find the most common prefix that matches the search text
    let mut prefix_counts = std::collections::HashMap::new();
    for prefix in &prefixes {
        if command_matches_query_string(prefix, search_text, false) {
            *prefix_counts.entry(prefix.clone()).or_insert(0) += 1;
        }
    }
    
    // Return the most common matching prefix
    prefix_counts.into_iter()
        .max_by_key(|(_, count)| *count)
        .map(|(prefix, _)| prefix)
}

/// Get current submenu prefix from search text
pub fn get_current_submenu_prefix(search_text: &str) -> Option<String> {
    if search_text.is_empty() {
        return None;
    }
    
    // For simple case, return the search text itself as potential prefix
    Some(search_text.trim().to_string())
}