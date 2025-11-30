//! Display logic for commands and command filtering
//!
//! ## Display Logic Overview
//!
//! The display system uses a two-phase approach that first determines if there's a prefix menu,
//! then constructs the appropriate display with sophisticated matching.
//!
//! ### Phase 1: COMMAND SELECTION: Prefix Menu Detection (`build_prefix_menu`)
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
//! - **Sophisticated Matching**: Uses word boundaries and proper separators, NOT simple .contains()
//! - **Separator-Aware**: Handles "!" and other separators in command prefixes
//! - **Proper Filtering**: All filtering (prefix menu, folder files) uses `command_matches_query()`
//! - **No Duplicates**: Folder files are deduplicated against ALL prefix menu commands
//! - **Filter Text for Files**: Folder files are filtered by remaining input after anchor match
//! - **Unified Code Path**: Same matching logic for both CLI and GUI
//!
//! # MENU CONSTRUCTION SPECIFICATION
//!
//! This section defines how the menu system works. These rules specify the intended behavior
//! and serve as the specification for the test suite in `tests/display_tests.rs`.
//!
//! ## PART 1: PREFIX MENU IDENTIFICATION
//!
//! **Backward Scanning for Anchors**
//! - Scan backwards from full input string, trying progressively shorter prefixes
//! - Input "PJPP" tries: "PJPP" → "PJP" → "PJ" → "P"
//! - Stop at first prefix that matches an anchor command (case-insensitive)
//! - If match found: use it as prefix menu anchor, remaining chars become filter
//! - If no match found: no prefix menu (proceed to global matching)
//!
//! **Anchor Matching Rules**
//! - Commands with action="alias" resolve to their target before checking if anchor
//! - Only commands with anchor flag ('a' or 'A') qualify as anchors
//! - Date prefixes (digits, dashes, underscores) are skipped when matching
//! - **If multiple anchors match, choose shortest ORIGINAL command name** (e.g., "HA" over "HA2")
//!   - This ensures the most specific match, even if "HA" is an alias to a longer name
//! - **Anchor matching is EXACT**: Input must match anchor consecutively (no character skipping)
//! - Only separator differences allowed (spaces, dots, underscores, hyphens are ignored)
//!
//! **Example:** Input "ha" → matches both "HA" and "HA2", chooses "HA" (shorter original name)
//! **Example:** Input "PJPP" → matches "PJ" anchor → filter = "PP"
//! **Example:** Input "stat" → matches "Status" anchor, but NOT "SV Tasks" (would skip 'V')
//!
//!
//! ## PART 2: COMMAND MATCHING
//!
//! **Matching Process** - Applied to ALL commands in the system:
//!
//! - Characters from input string match against beginnings of words in command name
//! - Characters must match in order (left-to-right)
//! - Can match multiple consecutive characters from same word
//! - Can jump to next word after matching at least one character
//! - Matching always starts at word boundaries (not mid-word)
//! - Matching is case-insensitive
//! - Word boundaries determined by separators (typically " ._-!")
//!
//! **Examples:**
//! - Input "PJD" matches "PJ Directories" (P,J from "PJ", D from "Directories")
//! - Input "ProDir" matches "Project Directories" (Pro from "Project", Dir from "Directories")
//! - Input "PD" matches "PJ Directories" (P from "PJ", D from "Directories", skipped between)
//! - Input "Dir" matches "PJ Directories" (skipped "PJ", matched "Dir" from "Directories")
//!
//! **Result Prioritization** - Matching commands are placed in three tiers:
//!
//! 1. **Exact Matches** (top) - Command name equals input (ignoring whitespace, case-insensitive)
//! 2. **No Words Skipped** (middle) - Matched at least one character from every word in command
//! 3. **Words Skipped** (bottom) - Skipped one or more words in the command name
//!
//!
//! ## PART 3: PREFIX MENU CONSTRUCTION
//!
//! When a prefix menu anchor is identified (Part 1), build its command list from:
//!
//! **A. Patch-Based Members**
//! - Commands with `patch` field = anchor name (case-insensitive)
//! - Example: Command "PP" with patch="PJ" belongs in "PJ" menu
//!
//! **B. Name-Based Members**
//! - Commands whose name starts with `anchor_name + separator`
//! - Separator must be one of word_separators (typically " ._-!")
//! - Example: "PJ Directories" belongs in "PJ" menu
//!
//! **C. Include Folder Members** (if configured)
//! - When anchor has 'I' flag or patch has include_folders configured
//! - Commands physically located in specified folders
//! - Must have names starting with `anchor_name + space`
//! - **Filtering**: When filter text exists (e.g., "shopsh" → anchor="shop", filter="sh"):
//!   - Extract the part of command name AFTER anchor prefix and separators
//!   - Example: "Shop Shutdown Shell" → extract "Shutdown Shell"
//!   - Apply fuzzy matching (Part 2 rules) to the extracted part using filter text
//!   - Example: "Shutdown Shell" matches filter "sh" (word "Shutdown" starts with "sh")
//!   - This ensures file-based commands filter consistently with other prefix menu commands
//!
//! **D. Commands from Broader Matching**
//! - Commands from Part 2 (both matching types) that also qualify via A, B, or C
//! - These go INTO the prefix menu, NOT the global section
//! - Prevents duplication between prefix menu and global menu
//!
//! **Special Rules:**
//! - Anchor command itself always included in its own menu
//! - Separator commands (action="separator") excluded everywhere
//! - Each (command_name, action) pair appears at most once
//!
//!
//! ## PART 4: SORTING & ORDERING
//!
//! **Priority Tiers** (from Part 2, applied to all command lists):
//!
//! 1. **Exact Matches** - command name equals input (ignoring whitespace, case-insensitive)
//! 2. **No Words Skipped** - matched at least one character from every word
//! 3. **Words Skipped** - skipped one or more words during matching
//! 4. **Alphabetical** - within each tier, sort alphabetically (case-insensitive)
//!    - Alpha characters before numeric characters
//!    - Shorter names win ties when lowercase names identical
//!
//! **Important**: Commands that do not skip any words (Tier 2) are sorted entirely before
//! commands that skip one or more words (Tier 3). This ensures that more complete matches
//! appear higher in the results.
//!
//! **What to Sort Against:**
//! - **Prefix menu commands**: Sort by filter text (if exists) or anchor name (if no filter)
//!   - Example: Input "PJPP" → sort by "PP"
//!   - Example: Input "PJ" → sort by "PJ"
//!
//! - **Global menu commands**: Sort by full input string
//!   - Example: Input "PJPP" → sort by "PJPP"
//!
//!
//! ## PART 5: FINAL MENU ASSEMBLY
//!
//! **With Prefix Menu:**
//! 1. Prefix menu commands (sorted per Part 4)
//! 2. Separator "============" (only if global matches exist)
//! 3. Global matches (sorted per Part 4)
//!    - Global matches = commands from Part 2 NOT already in prefix menu
//!    - Deduplicated against prefix menu (checks literal and alias-resolved matches)
//!
//! **Without Prefix Menu:**
//! 1. All matching commands from Part 2 (sorted per Part 4)
//! 2. No separator
//!
//! **Empty Input:**
//! - Returns empty result immediately (no processing)
//!
//!
//! ## SPECIAL CASES
//!
//! **Empty Prefix Menu Hiding**
//! - When filter text eliminates all prefix menu commands, the prefix menu is hidden entirely
//! - Example: Input "shopxyz" matches "shop" anchor with filter "xyz"
//!   - No Shop commands match "xyz" → prefix menu becomes empty
//!   - System hides prefix menu, separator, and breadcrumb
//!   - Returns only global matches (if any) as if no prefix menu was detected
//!   - Result: `is_prefix_menu = false`, `filter_text = ""`
//! - This provides smooth UX as user types: prefix menu appears, then disappears when no matches remain
//! - User sees seamless transition from prefix menu → empty → global matches
//!
//! **Exact Match Selection**
//! - When prefix menu exists AND an exact match appears in results (case-insensitive, ignoring separators):
//!   - `default_selection_index` points to exact match instead of first item
//!   - Example: Input "philz" matches "Phone" anchor but also has exact "Philz" command
//!   - Selection jumps to "Philz" for better UX when user types complete command name
use super::{Command, Patch, Config};
use std::collections::HashMap;
use crate::prelude::*;


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
/// Determine if a match between query and command skipped any words
/// Returns true if at least one character from every word was matched
/// Returns false if one or more words were entirely skipped
fn matches_all_words(command: &str, query: &str) -> bool {
    if query.is_empty() {
        return true;
    }

    let command_lower = command.to_lowercase();
    let query_lower = query.to_lowercase();
    let separators = " ._-";

    // Split command into words
    let words: Vec<&str> = command_lower
        .split(|c: char| separators.contains(c))
        .filter(|w| !w.is_empty())
        .collect();

    if words.is_empty() {
        return false;
    }

    let query_chars: Vec<char> = query_lower.chars().collect();
    let mut query_idx = 0;

    // For each word, check if at least one character was matched
    for word in &words {
        let word_chars: Vec<char> = word.chars().collect();
        let mut matched_in_word = false;

        // Try to match characters from this word
        for word_char in &word_chars {
            if query_idx < query_chars.len() && *word_char == query_chars[query_idx] {
                matched_in_word = true;
                query_idx += 1;

                // Continue matching subsequent chars from same word if they match
                for &next_word_char in word_chars[word_chars.iter().position(|&c| c == *word_char).unwrap() + 1..].iter() {
                    if query_idx < query_chars.len() && next_word_char == query_chars[query_idx] {
                        query_idx += 1;
                    } else {
                        break;
                    }
                }
                break;
            }
        }

        if !matched_in_word && query_idx < query_chars.len() {
            // We still have query chars to match but didn't match any in this word
            // This word was skipped
            return false;
        }
    }

    // Return true if we matched all query characters
    query_idx == query_chars.len()
}

fn sort_commands_by_relevance(commands: &mut Vec<Command>, input: &str) {
    commands.sort_by(|a, b| {
        // Tier 1: Exact matches
        let a_exact = a.command.eq_ignore_ascii_case(input);
        let b_exact = b.command.eq_ignore_ascii_case(input);

        if a_exact && !b_exact {
            return std::cmp::Ordering::Less;
        } else if !a_exact && b_exact {
            return std::cmp::Ordering::Greater;
        }

        // Tier 2: No words skipped (matched at least one char from every word)
        let a_no_skip = matches_all_words(&a.command, input);
        let b_no_skip = matches_all_words(&b.command, input);

        if a_no_skip && !b_no_skip {
            return std::cmp::Ordering::Less;
        } else if !a_no_skip && b_no_skip {
            return std::cmp::Ordering::Greater;
        }

        // Tier 3: Words skipped (or same tier)
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

/// Check if two strings match exactly (case-insensitive, whitespace-normalized)
///
/// This performs exact matching after normalizing both strings:
/// - Converts to lowercase
/// - Removes all whitespace
///
/// Use this for exact command lookups and alias resolution.
///
/// # Arguments
/// * `str1` - First string to compare
/// * `str2` - Second string to compare
///
/// # Returns
/// * `bool` - true if strings match exactly after normalization
///
/// # Examples
/// * `exact_match("Hook Anchor", "hook anchor")` → true
/// * `exact_match("HookAnchor", "hook anchor")` → true
/// * `exact_match("Hook", "Hook Anchor")` → false
pub fn exact_match(str1: &str, str2: &str) -> bool {
    let normalized1: String = str1.to_lowercase().chars().filter(|c| !c.is_whitespace()).collect();
    let normalized2: String = str2.to_lowercase().chars().filter(|c| !c.is_whitespace()).collect();
    normalized1 == normalized2
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

/// Exact prefix matching for anchors - requires consecutive character match.
/// Only allows ignoring separators (spaces, dots, underscores, hyphens).
///
/// This is stricter than `prefix_fully_matches_anchor()` which uses fuzzy matching.
/// Used specifically for anchor detection to avoid false positives.
///
/// # Examples
/// * `anchor_matches_prefix_exactly("sv", "SV Tasks", " ._-")` → true
/// * `anchor_matches_prefix_exactly("svt", "SV Tasks", " ._-")` → true
/// * `anchor_matches_prefix_exactly("stat", "SV Tasks", " ._-")` → false (would skip 'V')
/// * `anchor_matches_prefix_exactly("stat", "Status", " ._-")` → true
fn anchor_matches_prefix_exactly(command: &str, prefix: &str, separators: &str) -> bool {
    // Normalize: lowercase and remove separators
    let cmd_normalized: String = command.to_lowercase()
        .chars()
        .filter(|c| !separators.contains(*c))
        .collect();
    let prefix_normalized: String = prefix.to_lowercase()
        .chars()
        .filter(|c| !separators.contains(*c))
        .collect();

    // Exact prefix match (no character skipping)
    cmd_normalized.starts_with(&prefix_normalized)
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
/// * `config` - Configuration containing word separators and other settings
///
/// # Returns
/// * `Option<(Vec<Command>, Command, Command, String)>` - (prefix_menu_commands, original_command, resolved_command, filter_text) or None if no prefix menu found
pub fn build_prefix_menu(
    input: &str,
    all_commands: &[Command],
    patches: &HashMap<String, Patch>,
    config: &Config
) -> Option<(Vec<Command>, Command, Command, String)> {

    if input.trim().is_empty() {
        return None;
    }

    let input = input.trim();
    let separators = " ._-";
    let input_normalized: String = input.to_lowercase()
        .chars()
        .filter(|c| !separators.contains(*c))
        .collect();
    let input_normalized_len = input_normalized.len();

    // Phase 1: Check for EXACT match (anchor name == input)
    // This handles cases like input "asf" matching anchor "ASF" exactly
    for cmd in all_commands {
        let cmd_trimmed = skip_leading_date_chars(&cmd.command);
        let cmd_normalized: String = cmd.command.to_lowercase()
            .chars()
            .filter(|c| !separators.contains(*c))
            .collect();
        let cmd_trimmed_normalized: String = cmd_trimmed.to_lowercase()
            .chars()
            .filter(|c| !separators.contains(*c))
            .collect();

        let is_exact_match = cmd_normalized == input_normalized || cmd_trimmed_normalized == input_normalized;

        if is_exact_match {
            let resolved = cmd.resolve_alias(all_commands);
            if resolved.is_anchor() {
                let prefix_menu_commands = build_prefix_menu_commands(&resolved, all_commands, patches, "", config);
                return Some((prefix_menu_commands, cmd.clone(), resolved, String::new()));
            }
        }
    }

    // Phase 2: No exact match found - use prefix matching with filter
    // Try prefix lengths in order: (len-1), len, (len-2), ... to prefer leaving chars for filtering
    let mut prefix_lengths: Vec<usize> = Vec::new();
    if input_normalized_len > 1 {
        prefix_lengths.push(input_normalized_len - 1);
        prefix_lengths.push(input_normalized_len);
        for len in (1..input_normalized_len-1).rev() {
            prefix_lengths.push(len);
        }
    } else {
        prefix_lengths.push(input_normalized_len);
    }

    for &prefix_len in &prefix_lengths {
        let input_prefix = &input_normalized[..prefix_len];

        // Collect all matching anchors at this prefix length
        let mut matching_anchors: Vec<(Command, Command)> = Vec::new();

        for cmd in all_commands {
            let cmd_trimmed = skip_leading_date_chars(&cmd.command);
            let cmd_normalized: String = cmd.command.to_lowercase()
                .chars()
                .filter(|c| !separators.contains(*c))
                .collect();
            let cmd_trimmed_normalized: String = cmd_trimmed.to_lowercase()
                .chars()
                .filter(|c| !separators.contains(*c))
                .collect();

            let matches_original = cmd_normalized.starts_with(input_prefix);
            let matches_trimmed = cmd_trimmed_normalized.starts_with(input_prefix);

            if matches_original || matches_trimmed {
                let resolved = cmd.resolve_alias(all_commands);
                if resolved.is_anchor() {
                    matching_anchors.push((cmd.clone(), resolved));
                }
            }
        }

        if !matching_anchors.is_empty() {
            // Sort by command name length (shortest first)
            matching_anchors.sort_by(|a, b| a.0.command.len().cmp(&b.0.command.len()));
            let (original_command, resolved_command) = &matching_anchors[0];

            // Calculate remaining chars for filtering
            let remaining_chars = if prefix_len < input_normalized_len {
                // Find where the prefix ends in the original input
                let mut consumed_normalized = 0;
                let mut input_pos = 0;
                for ch in input.chars() {
                    if consumed_normalized >= prefix_len {
                        break;
                    }
                    if !separators.contains(ch) {
                        consumed_normalized += 1;
                    }
                    input_pos += ch.len_utf8();
                }
                &input[input_pos..].trim_start()
            } else {
                ""
            };

            detailed_log("BUILD_PREFIX_MENU", &format!("Selected anchor='{}' for input='{}', filter='{}'",
                resolved_command.command, input, remaining_chars));

            let prefix_menu_commands = build_prefix_menu_commands(&resolved_command, all_commands, patches, remaining_chars, config);
            return Some((prefix_menu_commands, original_command.clone(), resolved_command.clone(), remaining_chars.to_string()));
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
/// * `config` - Configuration containing word separators and other settings
///
/// # Returns
/// * `Vec<Command>` - The commands that should be displayed in the prefix menu, sorted alphabetically
fn build_prefix_menu_commands(
    anchor_command: &Command,
    all_commands: &[Command],
    patches: &HashMap<String, Patch>,
    filter_text: &str,
    config: &Config
) -> Vec<Command> {
    let mut prefix_menu_commands = Vec::new();
    let anchor_name = &anchor_command.command;
    let separators = &config.popup_settings.word_separators;

    // Add the anchor command itself only if it matches the filter
    // If filter_text is empty, always include the anchor (full match)
    // If filter_text exists, only include if anchor has patch-based membership AND matches filter
    // (Anchor cannot have name-based membership in its own menu - that's handled by the loop below
    // which explicitly excludes commands where cmd_lower == anchor_lower)
    if filter_text.is_empty() {
        prefix_menu_commands.push(anchor_command.clone());
    } else {
        // Only include anchor if it has patch-based membership (patch field matches anchor name)
        // In this case, match filter against full command name since there's no prefix to skip
        let anchor_lower = anchor_name.to_lowercase();
        let has_matching_patch = anchor_command.patch.to_lowercase() == anchor_lower;

        if has_matching_patch && command_matches_query_with_debug(&anchor_command.command, filter_text, false) >= 0 {
            prefix_menu_commands.push(anchor_command.clone());
        }
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
            detailed_log("PREFIX_MENU", &format!("Checking 'tracker info' - cmd_trimmed='{}', anchor_name='{}', filter_text='{}'",
                cmd_trimmed, anchor_name, filter_text));
        }

        // Check if this command belongs in the prefix menu:
        // 1. Command name starts with anchor name (e.g., "PJ Directories" for anchor "PJ")
        // 2. Command has patch matching anchor name (e.g., "PP" with patch "PJ")
        let has_matching_patch = cmd.patch.to_lowercase() == anchor_lower;

        let name_starts_with_prefix = if cmd_lower.starts_with(&anchor_lower) && cmd_lower != anchor_lower {
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

        let matches_prefix = has_matching_patch || name_starts_with_prefix;
        
        if matches_prefix {
            // Apply additional filtering based on remaining characters
            // Only include if filter_text is empty or the command name (after anchor prefix) starts with filter_text
            if filter_text.is_empty() {
                // Avoid duplicates
                if !prefix_menu_commands.iter().any(|existing: &Command| existing.command == cmd.command && existing.action == cmd.action) {
                    prefix_menu_commands.push(cmd.clone());
                }
            } else {
                // For patch-based membership (no name prefix), use two-phase matching
                if has_matching_patch && !name_starts_with_prefix {
                    // Phase 1: Check if command matches filter using sophisticated matching
                    let phase1_match = command_matches_query_with_debug(&cmd.command, filter_text, false) >= 0;

                    // Phase 2: Check if command matches full input (anchor + filter) using sophisticated matching
                    let full_input = format!("{}{}", anchor_name, filter_text);
                    let phase2_match = command_matches_query_with_debug(&cmd.command, &full_input, false) >= 0;

                    if phase1_match || phase2_match {
                        if !prefix_menu_commands.iter().any(|existing| existing.command == cmd.command && existing.action == cmd.action) {
                            prefix_menu_commands.push(cmd.clone());
                        }
                    }
                } else if anchor_name.len() < cmd_trimmed.len() {
                    // Get the part of the command after the anchor name (using trimmed command)
                    // This handles name-based membership where command starts with anchor name
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
        let include_folders = patch.get_all_include_folders(config);

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
                                // Extract the part AFTER the anchor prefix to match against filter
                                // (same logic as name-based members at lines 699-735)
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

                                    // Use sophisticated fuzzy matching on the remaining part
                                    if command_matches_query(remaining_part, filter_text) {
                                        prefix_menu_commands.push(cmd.clone());
                                    }
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
        detailed_log("PREFIX_MENU", &format!("anchor='{}', filter='{}', commands={:?}",
            anchor_name, filter_text, command_names));
    }


    prefix_menu_commands
}

/// Find the index of an exact match in the command list (case-insensitive, ignoring separators)
/// Returns None if no exact match found or if match is a separator
fn find_exact_match_index(commands: &[Command], input: &str, separators: &str) -> Option<usize> {
    let input_normalized: String = input.to_lowercase()
        .chars()
        .filter(|c| !separators.contains(*c))
        .collect();

    for (index, cmd) in commands.iter().enumerate() {
        // Skip separators
        if cmd.action == "separator" {
            continue;
        }

        let cmd_normalized: String = cmd.command.to_lowercase()
            .chars()
            .filter(|c| !separators.contains(*c))
            .collect();

        if cmd_normalized == input_normalized {
            return Some(index);
        }
    }

    None
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
/// * `config` - Configuration containing word separators and other settings
///
/// # Returns
/// * `(Vec<Command>, bool, Option<(Command, Command)>, usize, Option<usize>)` -
///   (display_commands, is_prefix_menu, prefix_menu_info, prefix_menu_count, default_selection_index)
///
/// ## Special Case: Exact Match Selection
/// When there is a prefix menu AND an exact match exists in the results (case-insensitive,
/// ignoring separators), the default_selection_index will point to that exact match instead
/// of defaulting to index 0. This provides better UX when user types a complete command name.
///
/// **Example:** Input "philz" matches "Phone" anchor but also has exact match "Philz" in results
/// → default_selection_index points to "Philz" instead of first item in prefix menu
pub fn get_new_display_commands(
    input: &str,
    all_commands: &[Command],
    patches: &HashMap<String, Patch>,
    config: &Config
) -> (Vec<Command>, bool, Option<(Command, Command)>, usize, Option<usize>, String) {

    if input.trim().is_empty() {
        // Return empty list when input is blank
        return (Vec::new(), false, None, 0, None, String::new());
    }

    let input = input.trim();

    // Step 1: Try to build a prefix menu using the full input
    // build_prefix_menu will scan backwards internally to find the best anchor match
    // and return the filter text that should be used
    if let Some((prefix_menu_commands, original_command, resolved_command, filter_text)) = build_prefix_menu(input, all_commands, patches, config) {
        // We have a prefix menu!
        detailed_log("DISPLAY", &format!("Found anchor with filter '{}'", filter_text));

        // Note: prefix_menu_commands are already filtered by build_prefix_menu_commands
        // No need to filter again here

        // Step 2: Get all commands that match the FULL input using sophisticated matching
        let mut prefix_commands = crate::core::commands::filter_commands_with_patch_support(
            all_commands,
            input,  // Use full input for broader matches
            1000, // High limit to get all matches
            &config.popup_settings.word_separators,
            false
        );

        // Step 3: Remove prefix menu commands from prefix_commands to avoid duplicates
        // Check both literal matches and resolved alias matches
        prefix_commands.retain(|cmd| {
            let cmd_resolved = cmd.resolve_alias(all_commands);

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

        // Special case: If prefix menu is empty after filtering, don't show prefix menu at all
        // Just return the global matches as if there was no prefix menu
        if prefix_menu_commands.is_empty() {
            detailed_log("DISPLAY", "Prefix menu empty after filtering - hiding prefix menu and showing only global matches");
            return (prefix_commands, false, None, 0, None, String::new());
        }

        // Step 4: Combine prefix menu + separator + remaining prefix commands
        let mut final_commands = prefix_menu_commands.clone();

        if !prefix_commands.is_empty() {
            // Add separator between prefix menu and other commands
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

        // Special case: Check for exact match in the final command list
        // If user typed exact command name (case-insensitive, ignoring separators), select that instead of first item
        let default_selection = find_exact_match_index(&final_commands, input, &config.popup_settings.word_separators);

        return (final_commands, true, Some((original_command, resolved_command)), prefix_menu_commands.len(), default_selection, filter_text);
    }

    // No prefix menu found at any length - use sophisticated matching
    let mut matching_commands = crate::core::commands::filter_commands_with_patch_support(
        all_commands,
        input,
        1000, // High limit to get all matches
        &config.popup_settings.word_separators,
        false
    );

    // Sort all commands by relevance: exact → prefix → substring → alphabetical
    sort_commands_by_relevance(&mut matching_commands, input);

    // No special selection needed for non-prefix-menu case (already sorted with exact matches first)
    // No filter text in non-prefix-menu mode (empty string)
    return (matching_commands, false, None, 0, None, String::new());
}
