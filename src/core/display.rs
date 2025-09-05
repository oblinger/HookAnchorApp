//! Display logic for commands and command filtering
//!
//! ## Display Logic Overview
//!
//! The display system uses a forward-thinking approach that first determines if there's a submenu,
//! then constructs the appropriate display. This is implemented in two main phases:
//!
//! ### Phase 1: Submenu Detection (`build_submenu`)
//! 1. **Backward Scanning**: Starting with the full input string (e.g., "FBAA"), scan backwards
//!    character by character: "FBAA" → "FBA" → "FB" → "F"
//! 2. **Command Matching**: For each substring, look for exact command matches (case-insensitive)
//! 3. **Alias Resolution**: Use `resolve_alias()` on each match to get the final target command
//! 4. **Anchor Detection**: Stop at the first resolved command that is an anchor
//! 5. **Return Data**: (submenu_commands, original_command, resolved_command)
//!    - `submenu_commands`: All commands that belong in this anchor's submenu
//!    - `original_command`: The matched alias (e.g., "FB") - used for prefix index calculation
//!    - `resolved_command`: The final anchor (e.g., "Facebook") - used for breadcrumbs
//!
//! ### Phase 2: Display Construction (`get_new_display_commands`)
//! 1. **Submenu Case**: If build_submenu finds an anchor:
//!    - Get all commands with simple prefix matching on the original input
//!    - Remove submenu commands from prefix commands to avoid duplicates  
//!    - Combine: [submenu_commands] + [separator] + [remaining_prefix_commands]
//! 2. **No Submenu Case**: Simple substring matching and relevance sorting
//!
//! ### Key Benefits
//! - **Correct prefix trimming**: Uses original command length (FB=2) not resolved (Facebook=8)
//! - **Accurate breadcrumbs**: Uses resolved command for patch hierarchy traversal
//! - **Complete submenu content**: Includes prefix matches, patch matches, and include logic
//! - **No duplicate commands**: Clean separation between submenu and non-submenu sections
//!
//! This module handles basic display-related operations for commands including:
//! - Basic query matching and scoring
//! - Submenu prefix detection and navigation
//! - Command prefix extraction
//! - Submenu construction and breadcrumb generation

use super::{Command, Patch};
use std::collections::HashMap;

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

/// Build submenu by scanning backwards from input string to find first anchor
/// 
/// Scans backwards from the full input string, looking for commands that resolve to anchors.
/// Returns the submenu commands, original command (for prefix index), and resolved command (for breadcrumbs).
/// 
/// # Arguments
/// * `input` - The input string from the search box
/// * `all_commands` - All available commands in the system
/// * `patches` - All available patches for include logic
/// 
/// # Returns
/// * `Option<(Vec<Command>, Command, Command)>` - (submenu_commands, original_command, resolved_command) or None if no submenu found
pub fn build_submenu(
    input: &str, 
    all_commands: &[Command], 
    patches: &HashMap<String, Patch>
) -> Option<(Vec<Command>, Command, Command)> {
    if input.trim().is_empty() {
        return None;
    }
    
    let input = input.trim();
    
    // Scan backwards from full input string to find first anchor
    for end_pos in (1..=input.len()).rev() {
        let test_string = &input[..end_pos];
        
        // Look for exact command match
        if let Some(matching_command) = all_commands.iter().find(|cmd| cmd.command.eq_ignore_ascii_case(test_string)) {
            // Resolve alias to get the final command
            let resolved_command = matching_command.resolve_alias(all_commands);
            
            // Check if resolved command is an anchor
            if resolved_command.action == "anchor" {
                // Found our anchor! Build the submenu
                let submenu_commands = build_submenu_commands(&resolved_command, all_commands, patches);
                return Some((submenu_commands, matching_command.clone(), resolved_command));
            }
        }
    }
    
    None
}

/// Build the list of commands that should be in a submenu for the given anchor
/// 
/// # Arguments
/// * `anchor_command` - The resolved anchor command
/// * `all_commands` - All available commands in the system  
/// * `patches` - All available patches for include logic
/// 
/// # Returns
/// * `Vec<Command>` - The commands that should be displayed in the submenu, sorted alphabetically
fn build_submenu_commands(
    anchor_command: &Command,
    all_commands: &[Command], 
    patches: &HashMap<String, Patch>
) -> Vec<Command> {
    let mut submenu_commands = Vec::new();
    let anchor_name = &anchor_command.command;
    
    // Find all commands that have the anchor name as a prefix
    for cmd in all_commands {
        if cmd.action == "separator" {
            continue;
        }
        
        // Check if command starts with anchor name (case-insensitive)
        if cmd.command.to_lowercase().starts_with(&anchor_name.to_lowercase()) {
            submenu_commands.push(cmd.clone());
        }
    }
    
    // Find all commands that have this anchor as their patch
    let patch_key = anchor_name.to_lowercase();
    for cmd in all_commands {
        if cmd.action == "separator" {
            continue;
        }
        
        if cmd.patch.to_lowercase() == patch_key {
            // Avoid duplicates
            if !submenu_commands.iter().any(|existing| existing.command == cmd.command && existing.action == cmd.action) {
                submenu_commands.push(cmd.clone());
            }
        }
    }
    
    // Execute include logic: if the patch has include commands or anchor commands with 'I' flag,
    // add all commands from those folders
    if let Some(patch) = patches.get(&patch_key) {
        let config = crate::core::sys_data::get_config();
        let include_folders = patch.get_all_include_folders(&config);
        
        if !include_folders.is_empty() {
            for cmd in all_commands {
                if cmd.action == "separator" {
                    continue;
                }
                
                // Skip if already in submenu_commands
                if submenu_commands.iter().any(|existing| existing.command == cmd.command && existing.action == cmd.action) {
                    continue;
                }
                
                // Check if this command is in one of the include folders
                if let Some(cmd_folder) = cmd.get_absolute_folder_path(&config) {
                    if include_folders.contains(&cmd_folder) {
                        submenu_commands.push(cmd.clone());
                    }
                }
            }
        }
    }
    
    // Sort alphabetically (not by length as before)
    submenu_commands.sort_by(|a, b| a.command.cmp(&b.command));
    
    submenu_commands
}

/// New display commands function using build_submenu approach
/// 
/// This is the new approach that:
/// 1. First tries to build a submenu using build_submenu()
/// 2. Gets all commands matching the input prefix (without alias resolution)
/// 3. Removes submenu commands from the main list to avoid duplicates
/// 4. Returns the combined display information
/// 
/// # Arguments
/// * `input` - The input string from the search box
/// * `all_commands` - All available commands in the system
/// * `patches` - All available patches
/// 
/// # Returns
/// * `(Vec<Command>, bool, Option<(Command, Command)>, usize)` - 
///   (display_commands, is_submenu, submenu_info, submenu_count)
pub fn get_new_display_commands(
    input: &str,
    all_commands: &[Command],
    patches: &HashMap<String, Patch>
) -> (Vec<Command>, bool, Option<(Command, Command)>, usize) {
    if input.trim().is_empty() {
        // Return all commands sorted alphabetically
        let mut all_cmds: Vec<Command> = all_commands.iter().cloned().collect();
        all_cmds.sort_by(|a, b| a.command.cmp(&b.command));
        return (all_cmds, false, None, 0);
    }
    
    let input = input.trim();
    
    // Step 1: Try to build a submenu
    if let Some((submenu_commands, original_command, resolved_command)) = build_submenu(input, all_commands, patches) {
        // We have a submenu!
        
        // Step 2: Get all commands that match the input as prefix (not thinking about aliases)
        let mut prefix_commands = Vec::new();
        for cmd in all_commands {
            if cmd.action == "separator" {
                continue;
            }
            // Simple prefix match - no alias resolution
            if cmd.command.to_lowercase().starts_with(&input.to_lowercase()) {
                prefix_commands.push(cmd.clone());
            }
        }
        
        // Step 3: Remove submenu commands from prefix_commands to avoid duplicates
        prefix_commands.retain(|cmd| {
            !submenu_commands.iter().any(|submenu_cmd| {
                submenu_cmd.command == cmd.command && submenu_cmd.action == cmd.action
            })
        });
        
        // Step 4: Combine submenu + separator + remaining prefix commands
        let mut final_commands = submenu_commands.clone();
        
        if !final_commands.is_empty() && !prefix_commands.is_empty() {
            // Add separator between submenu and other commands
            final_commands.push(Command {
                patch: String::new(),
                command: "---".to_string(),
                action: "separator".to_string(),
                arg: String::new(),
                flags: String::new(),
            });
        }
        
        final_commands.extend(prefix_commands);
        
        return (final_commands, true, Some((original_command, resolved_command)), submenu_commands.len());
    } else {
        // No submenu - just return commands that match the input prefix
        let mut matching_commands = Vec::new();
        for cmd in all_commands {
            if cmd.action == "separator" {
                continue;
            }
            // Simple prefix match or substring match
            if cmd.command.to_lowercase().contains(&input.to_lowercase()) {
                matching_commands.push(cmd.clone());
            }
        }
        
        // Sort by relevance (exact matches first, then prefix matches, then substring matches)
        matching_commands.sort_by(|a, b| {
            let a_exact = a.command.eq_ignore_ascii_case(input);
            let b_exact = b.command.eq_ignore_ascii_case(input);
            
            if a_exact && !b_exact {
                return std::cmp::Ordering::Less;
            } else if !a_exact && b_exact {
                return std::cmp::Ordering::Greater;
            }
            
            let a_prefix = a.command.to_lowercase().starts_with(&input.to_lowercase());
            let b_prefix = b.command.to_lowercase().starts_with(&input.to_lowercase());
            
            if a_prefix && !b_prefix {
                return std::cmp::Ordering::Less;
            } else if !a_prefix && b_prefix {
                return std::cmp::Ordering::Greater;
            }
            
            // Both are same type of match, sort alphabetically
            a.command.cmp(&b.command)
        });
        
        return (matching_commands, false, None, 0);
    }
}