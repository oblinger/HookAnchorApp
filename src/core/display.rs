//! Display logic for commands and command filtering
//!
//! ## Display Logic Overview
//!
//! The display system uses a two-phase approach that first determines if there's a prefix menu,
//! then constructs the appropriate display with sophisticated matching.
//!
//! ### Phase 1: Prefix Menu Detection (`build_prefix_menu`)
//! 1. **Backward Scanning**: Starting with the full input string (e.g., "FBO"), scan backwards
//!    character by character: "FBO" → "FB" → "F"
//! 2. **Command Matching**: For each substring, look for exact command matches (case-insensitive)
//! 3. **Alias Resolution**: Use `resolve_alias()` on each match to get the final target command
//!    - Example: "fb" command with action "alias" resolves to "Fireball" anchor
//! 4. **Anchor Detection**: Stop at the first resolved command that is an anchor
//! 5. **Filter Calculation**: Extract remaining characters for prefix menu filtering
//!    - Example: "FBO" with "FB" match → remaining_chars = "O"
//! 6. **Return Data**: (prefix_menu_commands, original_command, resolved_command)
//!    - `prefix_menu_commands`: Commands filtered by remaining characters
//!    - `original_command`: The matched command (e.g., "fb" alias)
//!    - `resolved_command`: The final anchor (e.g., "Fireball")
//!
//! ### Phase 2: Display Construction (`get_new_display_commands`)
//! 1. **Prefix Menu Case**: If build_prefix_menu finds an anchor:
//!    - Use sophisticated matching (`filter_commands_with_patch_support`) for all commands
//!    - Remove prefix menu commands from the results to avoid duplicates
//!    - Sort remaining commands by relevance (exact → prefix → substring)
//!    - Combine: [prefix_menu_commands] + [separator] + [remaining_commands]
//! 2. **No Prefix Menu Case**: Use sophisticated matching with relevance sorting
//!
//! ### Prefix Menu Building (`build_prefix_menu_commands`)
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
//! - **Proper Filtering**: Correctly filters prefix menu commands by remaining input
//! - **No Duplicates**: Clean separation between prefix menu and non-prefix menu sections
//! - **Unified Code Path**: Same logic for both CLI and GUI

use super::{Command, Patch};
use std::collections::HashMap;


/// Case-insensitive alphabetical comparison for command sorting
///
/// Sorting rules:
/// 1. Commands starting with alphabetic characters (A-Z, a-z) come first
/// 2. Commands starting with numeric characters (0-9) come after alphabetic
/// 3. Within each group, sort case-insensitively alphabetically
/// 4. Tie-breaker: Shorter command name wins if lowercase text is identical
///
/// Examples:
/// - "Apple" < "Zebra" < "123 Project" < "999 Tasks"
/// - "oblinge" < "Oblinger 83b" (shorter wins in case-insensitive tie)
fn compare_commands_case_insensitive(a: &Command, b: &Command) -> std::cmp::Ordering {
    let a_lower = a.command.to_lowercase();
    let b_lower = b.command.to_lowercase();

    // Check if commands start with digits
    let a_starts_digit = a_lower.chars().next().map_or(false, |c| c.is_ascii_digit());
    let b_starts_digit = b_lower.chars().next().map_or(false, |c| c.is_ascii_digit());

    // If one starts with digit and other doesn't, alphabetic comes first
    if a_starts_digit && !b_starts_digit {
        return std::cmp::Ordering::Greater; // a (digit) comes after b (alpha)
    } else if !a_starts_digit && b_starts_digit {
        return std::cmp::Ordering::Less; // a (alpha) comes before b (digit)
    }

    // Both start with digit or both start with alpha - sort normally
    match a_lower.cmp(&b_lower) {
        std::cmp::Ordering::Equal => a.command.len().cmp(&b.command.len()), // Shorter wins if same text
        other => other,
    }
}

/// Sort commands by relevance to the input string
///
/// **UNIVERSAL SORTING FUNCTION**
/// This function is used throughout the system for ALL command sorting:
/// - Prefix menu commands (sorted by filter text, or anchor name if filter is empty)
/// - Non-prefix-menu commands (sorted by full input)
/// - All commands when no prefix menu exists (sorted by full input)
///
/// SORTING RULES:
/// ==============
/// Priority 1: Exact matches (case-insensitive)
///   - Commands whose name exactly matches the full input
///   - Example: input "texter" → "texter" anchor comes first
///   - Example: input "FB" → "fb" alias comes first
///
/// Priority 2: Prefix matches (case-insensitive)
///   - Commands whose name starts with the input string
///   - Example: input "FB" → "FBX Something" comes before "Some FB Text"
///   - Example: input "task" → "task1_prd_text_capture_app" before "refactoring task"
///
/// Priority 3: Substring matches
///   - Commands that contain the input but don't start with it
///   - These use sophisticated matching (word boundaries, separators)
///   - Example: input "FB" → "Some FB Text" appears last
///
/// Priority 4: Alphabetical ordering (case-insensitive)
///   - Within each priority level, sort alphabetically
///   - Commands starting with alphabetic characters sort before numeric characters
///   - Case is ignored for comparison
///   - Tie-breaker: Shorter command name wins if lowercase text is identical
///
/// SPECIAL CASE: Empty input string
///   - When input is empty, all commands are treated as prefix matches
///   - Falls through to alphabetical sorting
///   - This happens for prefix menus when typing exactly the anchor name (e.g., "texter")
///   - In this case, the caller should pass the anchor name as input for proper exact-match sorting
///
/// # Arguments
/// * `commands` - Mutable reference to vector of commands to sort
/// * `input` - The input string to compare against (filter text, anchor name, or full input)
fn sort_commands_by_relevance(commands: &mut Vec<Command>, input: &str) {
    commands.sort_by(|a, b| {
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

        // Both are same type of match - sort case-insensitively
        compare_commands_case_insensitive(a, b)
    });
}

/// Skip leading date-like characters (digits, dashes, underscores, spaces) from the beginning of a string
/// This allows commands like "2025-09-12 Fireball Integration SOW" to match the "Fireball" anchor
/// by ignoring the date prefix and starting the match from "Fireball"
fn skip_leading_date_chars(s: &str) -> &str {
    let mut start_idx = 0;
    for (i, ch) in s.char_indices() {
        if ch.is_ascii_digit() || ch == '-' || ch == '_' || ch == ' ' {
            start_idx = i + ch.len_utf8();
        } else {
            break;
        }
    }
    &s[start_idx..]
}

/// Core matching function that returns the index where the match ends
/// Returns the position of the first unmatched character, or -1 if no match
/// This is the canonical separator-aware matching function used throughout the system
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

/// Simple boolean version of the canonical matching function
pub fn command_matches_query(command: &str, query: &str) -> bool {
    command_matches_query_with_debug(command, query, false) >= 0
}

/// Check if a prefix has fully matched to the END of a command name
///
/// This is stricter than command_matches_query - it requires that the match
/// extends all the way to the end of the command, not just a partial match.
/// Normalizes both strings by removing separators and converting to lowercase
/// before comparing lengths.
///
/// # Arguments
/// * `prefix` - The search prefix to match
/// * `command` - The command name to match against
///
/// # Returns
/// * `bool` - true if prefix fully matches to end of command, false otherwise
///
/// # Examples
/// * `prefix_fully_matches_anchor("facebook", "facebook")` → true
/// * `prefix_fully_matches_anchor("facebookpage", "facebook")` → true
/// * `prefix_fully_matches_anchor("face", "facebook")` → false (partial only)
/// * `prefix_fully_matches_anchor("face", "Hugging Face")` → false (partial only)
fn prefix_fully_matches_anchor(prefix: &str, command: &str) -> bool {
    // First check if it matches at all
    if !command_matches_query(command, prefix) {
        return false;
    }

    // Normalize both strings: lowercase + remove separators
    let separators = " ._-";
    let prefix_normalized: String = prefix.to_lowercase()
        .chars()
        .filter(|c| !separators.contains(*c))
        .collect();
    let command_normalized: String = command.to_lowercase()
        .chars()
        .filter(|c| !separators.contains(*c))
        .collect();

    // Check if normalized prefix length >= normalized command length
    prefix_normalized.len() >= command_normalized.len()
}


/// Build prefix menu by scanning backwards from input string to find first anchor
///
/// Scans backwards from the full input string, looking for commands that resolve to anchors.
/// Returns the prefix menu commands, original command (for prefix index), and resolved command (for breadcrumbs).
///
/// # Arguments
/// * `input` - The input string from the search box
/// * `all_commands` - All available commands in the system
/// * `patches` - All available patches for include logic
///
/// # Returns
/// * `Option<(Vec<Command>, Command, Command)>` - (prefix_menu_commands, original_command, resolved_command) or None if no prefix menu found
pub fn build_prefix_menu(
    input: &str,
    all_commands: &[Command],
    patches: &HashMap<String, Patch>
) -> Option<(Vec<Command>, Command, Command)> {

    if input.trim().is_empty() {
        return None;
    }

    let input = input.trim();

    
    // Try all possible prefixes of the input string, starting from longest
    // This allows "svplan" to match "SV Plan" and "RESD" to match "RES"
    for prefix_len in (1..=input.len()).rev() {
        let prefix = &input[..prefix_len];

        // Collect all matching anchors at this prefix length using sophisticated matching
        let mut matching_anchors: Vec<(Command, Command)> = Vec::new(); // (original, resolved)

        for cmd in all_commands {
            // Try matching with sophisticated matching (handles spaces, word boundaries, etc.)
            let cmd_trimmed = skip_leading_date_chars(&cmd.command);

            // Check if prefix fully matches to END of command (not just partial match)
            if prefix_fully_matches_anchor(prefix, &cmd.command) || prefix_fully_matches_anchor(prefix, &cmd_trimmed) {
                let resolved = cmd.resolve_alias(all_commands);
                if resolved.is_anchor() {
                    crate::utils::detailed_log("BUILD_PREFIX_MENU", &format!("Found matching anchor for prefix '{}': {} -> {}",
                        prefix, cmd.command, resolved.command));
                    matching_anchors.push((cmd.clone(), resolved));
                }
            }
        }

        // If we found any matching anchors, choose the one with the shortest command name
        // This ensures we match the most input characters with the most specific anchor
        if !matching_anchors.is_empty() {
            // Sort by resolved command name length (shortest first)
            matching_anchors.sort_by(|a, b| a.1.command.len().cmp(&b.1.command.len()));
            let (original_command, resolved_command) = &matching_anchors[0];

            // Calculate remaining chars for filtering
            let remaining_chars = if prefix_len < input.len() {
                &input[prefix_len..].trim_start()
            } else {
                ""
            };

            crate::utils::detailed_log("BUILD_PREFIX_MENU", &format!("Selected longest anchor='{}' for input='{}', filter='{}'",
                resolved_command.command, input, remaining_chars));

            let prefix_menu_commands = build_prefix_menu_commands(&resolved_command, all_commands, patches, remaining_chars);
            return Some((prefix_menu_commands, original_command.clone(), resolved_command.clone()));
        }
    }
    
    None
}

/// Build the list of commands that should be in a prefix menu for the given anchor
///
/// # Arguments
/// * `anchor_command` - The resolved anchor command
/// * `all_commands` - All available commands in the system
/// * `patches` - All available patches for include logic
/// * `filter_text` - Characters after the anchor match to filter prefix menu commands
///
/// # Returns
/// * `Vec<Command>` - The commands that should be displayed in the prefix menu, sorted alphabetically
fn build_prefix_menu_commands(
    anchor_command: &Command,
    all_commands: &[Command],
    patches: &HashMap<String, Patch>,
    filter_text: &str
) -> Vec<Command> {
    let mut prefix_menu_commands = Vec::new();
    let anchor_name = &anchor_command.command;
    let config = crate::core::data::get_config();
    let separators = &config.popup_settings.word_separators;

    // Add the anchor command itself only if it matches the filter
    // If filter_text is empty, always include the anchor (full match)
    // If filter_text exists, only include if anchor name matches it
    if filter_text.is_empty() || command_matches_query_with_debug(&anchor_command.command, filter_text, false) >= 0 {
        prefix_menu_commands.push(anchor_command.clone());
    }

    // Find all commands that have the anchor name as a prefix
    for cmd in all_commands {
        if cmd.action == "separator" {
            continue;
        }

        // Check if command starts with anchor name (followed by separator or end of string)
        // First, skip any leading digits, dashes, underscores, and spaces in the command name
        let cmd_trimmed = skip_leading_date_chars(&cmd.command);
        let cmd_lower = cmd_trimmed.to_lowercase();
        let anchor_lower = anchor_name.to_lowercase();

        // Log tracker info specifically (detailed log only)
        if cmd.command.to_lowercase().contains("tracker info") && !filter_text.is_empty() {
            crate::utils::detailed_log("PREFIX_MENU", &format!("Checking 'tracker info' - cmd_trimmed='{}', anchor_name='{}', filter_text='{}'",
                cmd_trimmed, anchor_name, filter_text));
        }

        let matches_prefix = if cmd_lower.starts_with(&anchor_lower) && cmd_lower != anchor_lower {
            // Check if the character after the anchor name is a separator
            let next_char_pos = anchor_lower.len();
            if next_char_pos < cmd_trimmed.len() {
                let next_char = cmd_trimmed.chars().nth(next_char_pos).unwrap_or(' ');
                let is_separator = separators.contains(next_char);


                is_separator
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
                if !prefix_menu_commands.iter().any(|existing: &Command| existing.command == cmd.command && existing.action == cmd.action) {
                    prefix_menu_commands.push(cmd.clone());
                }
            } else {
                // Get the part of the command after the anchor name (using trimmed command)
                if anchor_name.len() < cmd_trimmed.len() {
                    // Find the start of remaining text after anchor name and separators
                    let mut remaining_start = anchor_name.len();

                    // Skip over separator characters after the anchor name
                    while remaining_start < cmd_trimmed.len() {
                        let ch = cmd_trimmed.chars().nth(remaining_start).unwrap_or(' ');
                        if separators.contains(ch) {
                            remaining_start += ch.len_utf8();
                        } else {
                            break;
                        }
                    }

                    if remaining_start < cmd_trimmed.len() {
                        let remaining_part = &cmd_trimmed[remaining_start..];


                        // Two-phase matching as identified by user:
                        // Phase 1: Check if remaining part after anchor matches filter using sophisticated matching
                        let phase1_match = command_matches_query_with_debug(remaining_part, filter_text, false) >= 0;

                        // Phase 2: Check if full command matches full input (anchor + filter) using sophisticated matching
                        let full_input = format!("{} {}", anchor_name, filter_text);
                        let phase2_match = command_matches_query_with_debug(&cmd.command, &full_input, false) >= 0;


                        if phase1_match || phase2_match {
                            // Avoid duplicates
                            if !prefix_menu_commands.iter().any(|existing| existing.command == cmd.command && existing.action == cmd.action) {
                                prefix_menu_commands.push(cmd.clone());
                            }
                        }
                    }
                }
            }
        }
    }

    let patch_key = anchor_name.to_lowercase();

    // Execute include logic: if the patch has include commands or anchor commands with 'I' flag,
    // add all commands from those folders that match the prefix
    if let Some(patch) = patches.get(&patch_key) {
        let config = crate::core::data::get_config();
        let include_folders = patch.get_all_include_folders(&config);

        if !include_folders.is_empty() {
            // Build the prefix to match against (anchor name + space)
            let prefix_with_space = format!("{} ", anchor_name);
            let prefix_lower = prefix_with_space.to_lowercase();

            for cmd in all_commands {
                if cmd.action == "separator" {
                    continue;
                }

                // Skip if already in prefix_menu_commands
                if prefix_menu_commands.iter().any(|existing| existing.command == cmd.command && existing.action == cmd.action) {
                    continue;
                }

                // Check if this command is in one of the include folders
                if let Some(cmd_folder) = cmd.get_absolute_folder_path(&config) {
                    if include_folders.contains(&cmd_folder) {
                        // ONLY add to prefix menu if command actually starts with the prefix
                        let cmd_lower = cmd.command.to_lowercase();

                        if cmd_lower.starts_with(&prefix_lower) {
                            // Apply additional filter if provided
                            if filter_text.is_empty() {
                                prefix_menu_commands.push(cmd.clone());
                            } else {
                                // Check if command name starts with the filter text
                                if cmd_lower.starts_with(&filter_text.to_lowercase()) {
                                    prefix_menu_commands.push(cmd.clone());
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    // When filter_text is empty, sort by relevance to anchor name (so exact anchor matches appear first)
    // When filter_text is not empty, sort by relevance to filter text
    let sort_key = if filter_text.is_empty() {
        anchor_name
    } else {
        filter_text
    };
    sort_commands_by_relevance(&mut prefix_menu_commands, sort_key);

    // Log final result for tracker
    if anchor_name.to_lowercase() == "tracker" && !filter_text.is_empty() {
        let command_names: Vec<&str> = prefix_menu_commands.iter().map(|c| c.command.as_str()).collect();
        crate::utils::log(&format!("PREFIX_MENU_RESULT: anchor='{}', filter='{}', commands={:?}",
            anchor_name, filter_text, command_names));
    }


    prefix_menu_commands
}

/// New display commands function using build_prefix_menu approach
///
/// This is the new approach that:
/// 1. First tries to build a prefix menu using build_prefix_menu()
/// 2. Gets all commands matching the input prefix (without alias resolution)
/// 3. Removes prefix menu commands from the main list to avoid duplicates
/// 4. Returns the combined display information
///
/// # Arguments
/// * `input` - The input string from the search box
/// * `all_commands` - All available commands in the system
/// * `patches` - All available patches
///
/// # Returns
/// * `(Vec<Command>, bool, Option<(Command, Command)>, usize)` -
///   (display_commands, is_prefix_menu, prefix_menu_info, prefix_menu_count)
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
    
    // Step 1: Try to build a prefix menu
    if let Some((prefix_menu_commands, original_command, resolved_command)) = build_prefix_menu(input, all_commands, patches) {

        // We have a prefix menu!
        
        // Step 2: Get all commands that match the input using sophisticated matching
        let config = crate::core::data::get_config();
        let mut prefix_commands = crate::core::commands::filter_commands_with_patch_support(
            all_commands, 
            input, 
            1000, // High limit to get all matches
            &config.popup_settings.word_separators, 
            false
        );
        
        // Step 3: Remove prefix menu commands from prefix_commands to avoid duplicates
        // Check both literal matches and resolved alias matches
        prefix_commands.retain(|cmd| {
            // crate::utils::detailed_log("DISPLAY_FILTER", &format!("Checking command for dedup: {} (action: {})", cmd.command, cmd.action));
            let cmd_resolved = cmd.resolve_alias(all_commands);
            // crate::utils::detailed_log("DISPLAY_FILTER", &format!("Resolved to: {} (action: {})", cmd_resolved.command, cmd_resolved.action));
            
            !prefix_menu_commands.iter().any(|prefix_menu_cmd| {
                // Check literal match
                let literal_match = prefix_menu_cmd.command == cmd.command && prefix_menu_cmd.action == cmd.action;

                // Check if the resolved alias of cmd matches any prefix menu command
                let resolved_matches_prefix_menu = prefix_menu_cmd.command == cmd_resolved.command && prefix_menu_cmd.action == cmd_resolved.action;
                
                literal_match || resolved_matches_prefix_menu
            })
        });

        // Sort non-prefix-menu commands by relevance: exact → prefix → substring → alphabetical
        sort_commands_by_relevance(&mut prefix_commands, input);

        // Step 4: Combine prefix menu + separator + remaining prefix commands
        let mut final_commands = prefix_menu_commands.clone();
        
        
        if !prefix_commands.is_empty() {
            // Add separator between prefix menu and other commands (even if prefix menu is empty)
            final_commands.push(Command {
                patch: String::new(),
                command: "============".to_string(),
                action: "separator".to_string(),
                arg: String::new(),
                flags: String::new(),
        other_params: None,
                last_update: 0,
                file_size: None,
            });
        }
        
        final_commands.extend(prefix_commands);
        
        return (final_commands, true, Some((original_command, resolved_command)), prefix_menu_commands.len());
    } else {
        // No prefix menu - use sophisticated matching instead of simple contains()
        let config = crate::core::data::get_config();
        let mut matching_commands = crate::core::commands::filter_commands_with_patch_support(
            all_commands,
            input,
            1000, // High limit to get all matches
            &config.popup_settings.word_separators,
            false
        );

        // Sort all commands by relevance: exact → prefix → substring → alphabetical
        sort_commands_by_relevance(&mut matching_commands, input);

        return (matching_commands, false, None, 0);
    }
}