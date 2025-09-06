//! Display logic for commands and command filtering
//!
//! ## Display Logic Overview
//!
//! The display system uses a two-phase approach that first determines if there's a submenu,
//! then constructs the appropriate display with sophisticated matching.
//!
//! ### Phase 1: Submenu Detection (`build_submenu`)
//! 1. **Backward Scanning**: Starting with the full input string (e.g., "FBO"), scan backwards
//!    character by character: "FBO" → "FB" → "F"
//! 2. **Command Matching**: For each substring, look for exact command matches (case-insensitive)
//! 3. **Alias Resolution**: Use `resolve_alias()` on each match to get the final target command
//!    - Example: "fb" command with action "alias" resolves to "Fireball" anchor
//! 4. **Anchor Detection**: Stop at the first resolved command that is an anchor
//! 5. **Filter Calculation**: Extract remaining characters for submenu filtering
//!    - Example: "FBO" with "FB" match → remaining_chars = "O"
//! 6. **Return Data**: (submenu_commands, original_command, resolved_command)
//!    - `submenu_commands`: Commands filtered by remaining characters
//!    - `original_command`: The matched command (e.g., "fb" alias)
//!    - `resolved_command`: The final anchor (e.g., "Fireball")
//!
//! ### Phase 2: Display Construction (`get_new_display_commands`)
//! 1. **Submenu Case**: If build_submenu finds an anchor:
//!    - Use sophisticated matching (`filter_commands_with_patch_support`) for all commands
//!    - Remove submenu commands from the results to avoid duplicates
//!    - Sort remaining commands by relevance (exact → prefix → substring)
//!    - Combine: [submenu_commands] + [separator] + [remaining_commands]
//! 2. **No Submenu Case**: Use sophisticated matching with relevance sorting
//!
//! ### Submenu Building (`build_submenu_commands`)
//! 1. **Prefix Matching**: Find commands where prefix matches anchor name
//!    - Uses separator-aware prefix extraction (separators: " ._-!")
//!    - Example: "Fireball! Fireball Gdrive" has prefix "Fireball"
//! 2. **Patch Matching**: Find commands with matching patch field
//! 3. **Include Logic**: Process patch include folders if configured
//! 4. **Filtering**: Apply remaining character filter if present
//!    - Skips separator characters after prefix before matching
//!    - Example: "Fireball! Old Example" → skip "! " → match "Old" with "O"
//!
//! ### Key Features
//! - **Sophisticated Matching**: Uses word boundaries and proper separators, not simple contains
//! - **Separator-Aware**: Handles "!" and other separators in command prefixes
//! - **Proper Filtering**: Correctly filters submenu commands by remaining input
//! - **No Duplicates**: Clean separation between submenu and non-submenu sections
//! - **Unified Code Path**: Same logic for both CLI and GUI

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
                // Found our anchor! Calculate remaining characters for filtering
                let remaining_chars = if end_pos < input.len() {
                    &input[end_pos..]
                } else {
                    ""
                };
                
                // Build the submenu with filtering
                let submenu_commands = build_submenu_commands(&resolved_command, all_commands, patches, remaining_chars);
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
/// * `filter_text` - Characters after the anchor match to filter submenu commands
/// 
/// # Returns
/// * `Vec<Command>` - The commands that should be displayed in the submenu, sorted alphabetically
fn build_submenu_commands(
    anchor_command: &Command,
    all_commands: &[Command], 
    patches: &HashMap<String, Patch>,
    filter_text: &str
) -> Vec<Command> {
    let mut submenu_commands = Vec::new();
    let anchor_name = &anchor_command.command;
    let config = crate::core::sys_data::get_config();
    let separators = &config.popup_settings.word_separators;
    
    // Find all commands that have the anchor name as a prefix
    for cmd in all_commands {
        if cmd.action == "separator" {
            continue;
        }
        
        // Check if command starts with anchor name (followed by separator or end of string)
        let cmd_lower = cmd.command.to_lowercase();
        let anchor_lower = anchor_name.to_lowercase();
        
        let matches_prefix = if cmd_lower == anchor_lower {
            // Exact match
            true
        } else if cmd_lower.starts_with(&anchor_lower) {
            // Check if the character after the anchor name is a separator
            let next_char_pos = anchor_lower.len();
            if next_char_pos < cmd.command.len() {
                let next_char = cmd.command.chars().nth(next_char_pos).unwrap_or(' ');
                separators.contains(next_char)
            } else {
                false
            }
        } else {
            false
        };
        
        if matches_prefix {
            // Apply additional filtering based on remaining characters
            // Only include if filter_text is empty or the command name (after anchor prefix) starts with filter_text
            if filter_text.is_empty() {
                // Avoid duplicates
                if !submenu_commands.iter().any(|existing: &Command| existing.command == cmd.command && existing.action == cmd.action) {
                    submenu_commands.push(cmd.clone());
                }
            } else {
                // Get the part of the command after the anchor name
                if anchor_name.len() < cmd.command.len() {
                    // Find the start of remaining text after anchor name and separators
                    let mut remaining_start = anchor_name.len();
                    
                    // Skip over separator characters after the anchor name
                    while remaining_start < cmd.command.len() {
                        let ch = cmd.command.chars().nth(remaining_start).unwrap_or(' ');
                        if separators.contains(ch) {
                            remaining_start += ch.len_utf8();
                        } else {
                            break;
                        }
                    }
                    
                    if remaining_start < cmd.command.len() {
                        let remaining_part = &cmd.command[remaining_start..];
                        if remaining_part.to_lowercase().starts_with(&filter_text.to_lowercase()) {
                            // Avoid duplicates
                            if !submenu_commands.iter().any(|existing| existing.command == cmd.command && existing.action == cmd.action) {
                                submenu_commands.push(cmd.clone());
                            }
                        }
                    }
                }
            }
        }
    }
    
    // Find all commands that have this anchor as their patch
    let patch_key = anchor_name.to_lowercase();
    for cmd in all_commands {
        if cmd.action == "separator" {
            continue;
        }
        
        if cmd.patch.to_lowercase() == patch_key {
            // Apply additional filtering based on remaining characters
            // Only include if filter_text is empty or the command name starts with filter_text
            if filter_text.is_empty() {
                // Avoid duplicates
                if !submenu_commands.iter().any(|existing: &Command| existing.command == cmd.command && existing.action == cmd.action) {
                    submenu_commands.push(cmd.clone());
                }
            } else {
                // Check if command name starts with the filter text
                if cmd.command.to_lowercase().starts_with(&filter_text.to_lowercase()) {
                    // Avoid duplicates
                    if !submenu_commands.iter().any(|existing| existing.command == cmd.command && existing.action == cmd.action) {
                        submenu_commands.push(cmd.clone());
                    }
                }
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
    
    // Sort with exact anchor name first, then alphabetically
    let anchor_name_lower = anchor_name.to_lowercase();
    submenu_commands.sort_by(|a, b| {
        let a_is_exact = a.command.to_lowercase() == anchor_name_lower;
        let b_is_exact = b.command.to_lowercase() == anchor_name_lower;
        
        // Exact matches come first
        if a_is_exact && !b_is_exact {
            std::cmp::Ordering::Less
        } else if !a_is_exact && b_is_exact {
            std::cmp::Ordering::Greater
        } else {
            // Both are exact or both are not exact - sort alphabetically
            a.command.cmp(&b.command)
        }
    });
    
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
        // Return empty list when input is blank
        return (Vec::new(), false, None, 0);
    }
    
    let input = input.trim();
    
    // Step 1: Try to build a submenu
    if let Some((submenu_commands, original_command, resolved_command)) = build_submenu(input, all_commands, patches) {
        // We have a submenu!
        
        // Step 2: Get all commands that match the input using sophisticated matching
        let config = crate::core::sys_data::get_config();
        let mut prefix_commands = crate::core::commands::filter_commands_with_patch_support(
            all_commands, 
            input, 
            1000, // High limit to get all matches
            &config.popup_settings.word_separators, 
            false
        );
        
        // Step 3: Remove submenu commands from prefix_commands to avoid duplicates
        // Check both literal matches and resolved alias matches
        prefix_commands.retain(|cmd| {
            let cmd_resolved = cmd.resolve_alias(all_commands);
            
            !submenu_commands.iter().any(|submenu_cmd| {
                // Check literal match
                let literal_match = submenu_cmd.command == cmd.command && submenu_cmd.action == cmd.action;
                
                // Check if the resolved alias of cmd matches any submenu command
                let resolved_matches_submenu = submenu_cmd.command == cmd_resolved.command && submenu_cmd.action == cmd_resolved.action;
                
                literal_match || resolved_matches_submenu
            })
        });
        
        // Step 3.5: Sort prefix commands - exact matches first, then prefix matches, then substring matches
        prefix_commands.sort_by(|a, b| {
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
            
            // Both are same type of match - sort alphabetically
            a.command.cmp(&b.command)
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
        // No submenu - use sophisticated matching instead of simple contains()
        let config = crate::core::sys_data::get_config();
        let mut matching_commands = crate::core::commands::filter_commands_with_patch_support(
            all_commands,
            input,
            1000, // High limit to get all matches
            &config.popup_settings.word_separators,
            false
        );
        
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