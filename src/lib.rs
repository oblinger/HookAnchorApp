//! Anchor Selector Library
//! 
//! A command management and filtering library that provides fuzzy matching
//! and prioritized search for command execution.

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use regex::Regex;
use serde::{Deserialize, Serialize};

// =============================================================================
// Configuration
// =============================================================================

/// Settings section of the configuration file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub max_rows: usize,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            max_rows: 10,
        }
    }
}

/// Application configuration loaded from YAML config file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub settings: Settings,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            settings: Settings::default(),
        }
    }
}

/// Loads configuration from the YAML config file, returns default if file doesn't exist or is invalid
pub fn load_config() -> Config {
    let config_path = get_config_file_path();
    
    if let Ok(contents) = fs::read_to_string(&config_path) {
        match serde_yaml::from_str::<Config>(&contents) {
            Ok(config) => config,
            Err(_) => Config::default()
        }
    } else {
        Config::default()
    }
}

/// Returns the path to the YAML config file
fn get_config_file_path() -> PathBuf {
    let home = env::var("HOME").unwrap_or_else(|_| ".".to_string());
    Path::new(&home).join(".config/anchor_selector/config.yaml")
}


// =============================================================================
// Data Structures
// =============================================================================

/// Represents a parsed command with its components and original line
#[derive(Debug, Clone, PartialEq)]
pub struct Command {
    pub group: String,
    pub command: String,
    pub action: String,
    pub arg: String,
    pub full_line: String,
}

// =============================================================================
// File I/O Operations
// =============================================================================

/// Returns the path to the commands file
fn get_commands_file_path() -> PathBuf {
    let home = env::var("HOME").unwrap_or_else(|_| ".".to_string());
    Path::new(&home).join("ob/data/spot_cmds/spot_cmds.txt")
}

/// Loads commands from the spot_cmds.txt file and parses them into Command structs
/// 
/// # Panics
/// Panics if the commands file cannot be read or doesn't exist
pub fn load_commands() -> Vec<Command> {
    let file_path = get_commands_file_path();
    
    let contents = fs::read_to_string(&file_path)
        .unwrap_or_else(|e| {
            eprintln!("Error reading commands file {}: {}", file_path.display(), e);
            std::process::exit(1);
        });
    
    parse_commands(&contents)
}

/// Parses command text into a vector of Command structs
fn parse_commands(contents: &str) -> Vec<Command> {
    // Regex patterns for parsing command formats
    let re_with_arg = Regex::new(r"^(?:(.+?)!\s*)?(.+?):\s*(\S+)\s+(.*)$").unwrap();
    let re_no_arg = Regex::new(r"^(?:(.+?)!\s*)?(.+?):\s*(\S+)\s*$").unwrap();
    
    contents
        .lines()
        .filter_map(|line| {
            let line = line.trim();
            if line.is_empty() {
                return None;
            }
            
            // Try parsing with argument first
            if let Some(captures) = re_with_arg.captures(line) {
                Some(Command {
                    group: captures.get(1).map_or(String::new(), |m| m.as_str().trim().to_string()),
                    command: captures.get(2).map_or(String::new(), |m| m.as_str().trim().to_string()),
                    action: captures.get(3).map_or(String::new(), |m| m.as_str().trim().to_string()),
                    arg: captures.get(4).map_or(String::new(), |m| m.as_str().trim().to_string()),
                    full_line: line.to_string(),
                })
            }
            // Try parsing without argument
            else if let Some(captures) = re_no_arg.captures(line) {
                Some(Command {
                    group: captures.get(1).map_or(String::new(), |m| m.as_str().trim().to_string()),
                    command: captures.get(2).map_or(String::new(), |m| m.as_str().trim().to_string()),
                    action: captures.get(3).map_or(String::new(), |m| m.as_str().trim().to_string()),
                    arg: String::new(),
                    full_line: line.to_string(),
                })
            }
            // Log and skip unparseable lines
            else {
                eprintln!("Warning: Failed to parse line: {}", line);
                None
            }
        })
        .collect()
}

/// Saves commands to spot_cmds.txt using their original full_line format
pub fn save_commands(commands: &[Command]) -> Result<(), Box<dyn std::error::Error>> {
    let file_path = get_commands_file_path();
    
    let contents = commands.iter()
        .map(|cmd| cmd.full_line.clone())
        .collect::<Vec<String>>()
        .join("\n");
    
    fs::write(&file_path, contents)?;
    Ok(())
}

/// Saves commands to a specified file with proper formatting and newline handling
pub fn save_commands_formatted(commands: &[Command], output_file: &str) -> Result<(), Box<dyn std::error::Error>> {
    let home = env::var("HOME").unwrap_or_else(|_| ".".to_string());
    let file_path = Path::new(&home).join("ob/data/spot_cmds").join(output_file);
    
    let contents = commands.iter()
        .map(|cmd| cmd.full_line.clone())
        .collect::<Vec<String>>()
        .join("\n") + "\n";
    
    fs::write(&file_path, contents)?;
    Ok(())
}

/// Convenience function to save commands to the default spot_cmds.txt file
pub fn save_commands_to_file(commands: &[Command]) -> Result<(), Box<dyn std::error::Error>> {
    save_commands_formatted(commands, "spot_cmds.txt")
}

// =============================================================================
// Command Matching and Filtering
// =============================================================================

/// Returns true if the command name matches the search query using flexible sequence matching
pub fn command_matches_query(command_name: &str, query: &str) -> bool {
    command_matches_query_with_debug(command_name, query, false) >= 0
}

/// Returns match quality as integer: -1 for no match, position of first unmatched non-space/dot char, or string length for complete match
pub fn command_matches_query_with_debug(command_name: &str, query: &str, debug: bool) -> i32 {
    let search_lower = query.to_lowercase();
    let cmd_lower = command_name.to_lowercase();
    
    if search_lower.is_empty() {
        return cmd_lower.len() as i32;
    }
    
    // For queries containing dots, try both word-separated matching and flexible character matching
    if search_lower.contains('.') {
        let search_words: Vec<&str> = search_lower.split(|c| c == ' ' || c == '_' || c == '.').filter(|s| !s.is_empty()).collect();
        
        // Try multi-word matching first
        if search_words.len() > 1 {
            let result = matches_multi_word_sequence_with_position(&cmd_lower, &search_words, debug);
            if result >= 0 {
                return result;
            }
        }
        
        // Also try flexible character matching treating the whole query as one unit
        return matches_flexible_sequence_with_position(&cmd_lower, &search_lower, debug);
    } else {
        // For regular queries, use original logic
        let search_words: Vec<&str> = search_lower.split_whitespace().collect();
        
        if search_words.is_empty() {
            return cmd_lower.len() as i32;
        }
        
        if search_words.len() == 1 {
            // Single word: use flexible character-by-character matching
            matches_flexible_sequence_with_position(&cmd_lower, search_words[0], debug)
        } else {
            // Multiple words: use exact word boundary matching
            matches_multi_word_sequence_with_position(&cmd_lower, &search_words, debug)
        }
    }
}

/// Filters and prioritizes commands based on search text, returning up to max_results best matches
/// 
/// Commands are ranked by relevance:
/// 1. Exact matches (highest priority)
/// 2. Prefix matches at start of command
/// 3. Word boundary matches
/// 4. Partial prefix matches (lowest priority)
pub fn filter_commands(commands: &[Command], search_text: &str, max_results: usize, debug: bool) -> Vec<Command> {
    if search_text.trim().is_empty() {
        return Vec::new();
    }
    
    let search_lower = search_text.to_lowercase();
    let search_words: Vec<&str> = search_lower.split_whitespace().collect();
    
    if debug {
        println!("Debug: searching for words: {:?} (original: '{}')", search_words, search_text);
    }
    
    // Categorize matches by relevance
    let mut exact_matches = Vec::new();
    let mut command_start_matches = Vec::new();
    let mut word_start_matches = Vec::new();
    let mut prefix_matches = Vec::new();
    
    for cmd in commands {
        let cmd_lower = cmd.command.to_lowercase();
        
        if debug && cmd_lower.contains("selector") && search_words.contains(&"s") {
            println!("Debug: checking '{}' against words: {:?}", cmd_lower, search_words);
        }
        
        if command_matches_query_with_debug(&cmd.command, search_text, debug) >= 0 {
            categorize_match(cmd, &cmd_lower, &search_words, &mut exact_matches, 
                           &mut command_start_matches, &mut word_start_matches, &mut prefix_matches);
        }
    }
    
    // Sort and combine results
    let prefer_longer = search_words.get(0).map_or(false, |w| w.len() <= 2);
    
    sort_matches(&mut exact_matches, prefer_longer);
    sort_matches(&mut command_start_matches, prefer_longer);
    sort_matches(&mut word_start_matches, prefer_longer);
    sort_matches(&mut prefix_matches, prefer_longer);
    
    combine_and_limit_results(search_words, exact_matches, command_start_matches, 
                             word_start_matches, prefix_matches, max_results, debug)
}

/// Categorizes a matching command into the appropriate priority bucket
fn categorize_match(cmd: &Command, cmd_lower: &str, search_words: &[&str],
                   exact_matches: &mut Vec<Command>, command_start_matches: &mut Vec<Command>,
                   word_start_matches: &mut Vec<Command>, prefix_matches: &mut Vec<Command>) {
    if search_words.len() == 1 {
        let search_word = search_words[0];
        let cmd_no_separators = cmd_lower.replace(' ', "").replace('_', "");
        let search_no_separators = search_word.replace(' ', "").replace('_', "");
        
        if cmd_no_separators == search_no_separators {
            exact_matches.push(cmd.clone());
        } else if cmd_lower.starts_with(search_word) {
            command_start_matches.push(cmd.clone());
        } else if cmd_lower.contains(&format!(" {}", search_word)) || 
                  cmd_lower.contains(&format!("_{}", search_word)) {
            word_start_matches.push(cmd.clone());
        } else {
            prefix_matches.push(cmd.clone());
        }
    } else {
        // Multi-word searches get medium priority
        word_start_matches.push(cmd.clone());
    }
}

/// Sorts command matches based on length preference
fn sort_matches(matches: &mut Vec<Command>, prefer_longer: bool) {
    if prefer_longer {
        // For short searches, prefer "reasonable" length (8-30 chars)
        matches.sort_by_key(|cmd| {
            let len = cmd.command.len();
            if len >= 8 && len <= 30 {
                0  // Best priority
            } else if len > 30 {
                len  // Long commands get lower priority
            } else {
                1000 + len  // Very short commands get lowest priority
            }
        });
    } else {
        matches.sort_by_key(|cmd| cmd.command.len());
    }
}

/// Combines and limits results based on search strategy
fn combine_and_limit_results(search_words: Vec<&str>, exact_matches: Vec<Command>,
                            command_start_matches: Vec<Command>, word_start_matches: Vec<Command>,
                            prefix_matches: Vec<Command>, max_results: usize, debug: bool) -> Vec<Command> {
    let exact_count = exact_matches.len();
    let cmd_start_count = command_start_matches.len();
    let word_count = word_start_matches.len();
    let prefix_count = prefix_matches.len();
    
    let mut filtered_commands = Vec::new();
    
    // For short searches, interleave categories for variety
    if search_words.get(0).map_or(false, |w| w.len() <= 2) {
        let mut exact_iter = exact_matches.into_iter();
        let mut start_iter = command_start_matches.into_iter();
        let mut word_iter = word_start_matches.into_iter();
        let mut prefix_iter = prefix_matches.into_iter();
        
        // Interleave: 2 exact, 3 cmd-start, 3 word-start, 2 more cmd-start
        filtered_commands.extend(exact_iter.by_ref().take(2));
        filtered_commands.extend(start_iter.by_ref().take(3));
        filtered_commands.extend(word_iter.by_ref().take(3));
        filtered_commands.extend(start_iter.by_ref().take(2));
        
        // Fill remaining slots up to max_results
        while filtered_commands.len() < max_results {
            if let Some(cmd) = word_iter.next().or_else(|| start_iter.next()).or_else(|| prefix_iter.next()) {
                filtered_commands.push(cmd);
            } else {
                break;
            }
        }
    } else {
        // For longer searches, use priority order
        filtered_commands.extend(exact_matches);
        filtered_commands.extend(command_start_matches);
        filtered_commands.extend(word_start_matches);
        filtered_commands.extend(prefix_matches);
        filtered_commands.truncate(max_results);
    }
    
    if debug {
        println!("Debug: found {} exact, {} cmd-start, {} word-start, {} prefix matches, {} total", 
                 exact_count, cmd_start_count, word_count, prefix_count, filtered_commands.len());
    }
    
    filtered_commands
}



/// Returns position of first unmatched non-space/dot char, or command length if fully matched, or -1 if no match
/// Matches characters sequentially, allowing matches to start at word boundaries or continue within words
fn matches_flexible_sequence_with_position(cmd_text: &str, search_text: &str, _debug: bool) -> i32 {
    let search_chars: Vec<char> = search_text.chars().collect();
    let cmd_chars: Vec<char> = cmd_text.chars().collect();
    
    if search_chars.is_empty() {
        return cmd_text.len() as i32;
    }
    
    let mut search_pos = 0;
    let mut cmd_pos = 0;
    let mut last_match_pos = None; // Track position of last match to detect continuous sequences
    
    while search_pos < search_chars.len() && cmd_pos < cmd_chars.len() {
        let cmd_char = cmd_chars[cmd_pos];
        
        if cmd_char == search_chars[search_pos] {
            let can_match = if search_pos == 0 {
                // First character - only allow at word boundaries (start of string or after separator)
                cmd_pos == 0 || {
                    let prev_char = cmd_chars[cmd_pos - 1];
                    prev_char == ' ' || prev_char == '_' || prev_char == '.'
                }
            } else {
                // For subsequent characters, check if we can match here
                let continuing_sequence = if let Some(last_pos) = last_match_pos {
                    // Check if we're continuing from the previous match (adjacent characters)
                    cmd_pos == last_pos + 1
                } else {
                    false
                };
                
                if continuing_sequence {
                    true
                } else {
                    // Check if at word boundary
                    cmd_pos == 0 || {
                        let prev_char = cmd_chars[cmd_pos - 1];
                        prev_char == ' ' || prev_char == '_' || prev_char == '.'
                    }
                }
            };
            
            if can_match {
                last_match_pos = Some(cmd_pos);
                search_pos += 1;
            }
        }
        
        cmd_pos += 1;
    }
    
    if search_pos == search_chars.len() {
        // All search characters matched, find first unmatched non-space/dot character
        while cmd_pos < cmd_chars.len() {
            let ch = cmd_chars[cmd_pos];
            if ch != ' ' && ch != '.' && ch != '_' {
                return cmd_pos as i32;
            }
            cmd_pos += 1;
        }
        // All remaining characters were spaces/dots/underscores, return command length
        return cmd_text.len() as i32;
    } else {
        // Not all search characters matched
        return -1;
    }
}

/// Returns position of first unmatched non-space/dot char for multi-word matching
fn matches_multi_word_sequence_with_position(cmd_text: &str, search_words: &[&str], debug: bool) -> i32 {
    let cmd_words: Vec<&str> = cmd_text.split(|c| c == ' ' || c == '_' || c == '.').filter(|s| !s.is_empty()).collect();
    
    if debug && cmd_text.contains("selector") {
        println!("Debug: cmd_words: {:?}, search_words: {:?}", cmd_words, search_words);
    }
    
    let mut search_idx = 0;
    let mut total_chars_processed = 0;
    
    for cmd_word in &cmd_words {
        if search_idx >= search_words.len() {
            break;
        }
        
        if cmd_word.starts_with(search_words[search_idx]) {
            search_idx += 1;
            if debug && cmd_text.contains("selector") {
                println!("Debug: matched '{}' with '{}', search_idx now {}", 
                        cmd_word, search_words[search_idx-1], search_idx);
            }
        }
        
        // Track position in original string
        total_chars_processed += cmd_word.len();
        // Account for separators (simplified - assumes single separator between words)
        if total_chars_processed < cmd_text.len() {
            total_chars_processed += 1;
        }
    }
    
    if search_idx == search_words.len() {
        // All search words matched, find first unmatched non-space/dot character
        let cmd_chars: Vec<char> = cmd_text.chars().collect();
        let mut pos = total_chars_processed.min(cmd_chars.len());
        
        while pos < cmd_chars.len() {
            let ch = cmd_chars[pos];
            if ch != ' ' && ch != '.' && ch != '_' {
                return pos as i32;
            }
            pos += 1;
        }
        // All remaining characters were spaces/dots/underscores
        return cmd_text.len() as i32;
    } else {
        // Not all search words matched
        return -1;
    }
}

// =============================================================================
// Command List Management
// =============================================================================

/// Removes a command from the list by name, returns true if a command was deleted
pub fn delete_command(commands: &mut Vec<Command>, command_name: &str) -> bool {
    let original_len = commands.len();
    commands.retain(|cmd| cmd.command != command_name);
    commands.len() != original_len
}

/// Adds a new command to the end of the command list
pub fn add_command(commands: &mut Vec<Command>, new_command: Command) {
    commands.push(new_command);
}

/// Updates a command list by removing the original command and adding the new one
pub fn update_command_list(commands: &mut Vec<Command>, new_command: Command, original_command_name: &str) {
    if !original_command_name.is_empty() {
        delete_command(commands, original_command_name);
    }
    add_command(commands, new_command);
}

// =============================================================================
// Submenu Detection and Display
// =============================================================================

/// Analyzes filtered commands to determine if they form a submenu and returns display positions
/// 
/// Returns:
/// - Empty list if no submenu detected (no commands have dots before their match position)
/// - List of integers indicating where to start displaying each command's suffix
///   - For commands with dot before match position: returns the match position
///   - For commands without dot before match position: returns 0 (show full name)
pub fn get_submenu_display_positions(commands: &[Command], search_text: &str) -> Vec<usize> {
    let mut positions = Vec::new();
    let mut has_submenu = false;
    
    for cmd in commands {
        let match_pos = command_matches_query_with_debug(&cmd.command, search_text, false);
        
        if match_pos >= 0 {
            let match_pos = match_pos as usize;
            
            // Check if there's a separator (dot, space, or underscore) before the match position
            if match_pos > 0 {
                if let Some(prev_char) = cmd.command.chars().nth(match_pos - 1) {
                    if prev_char == '.' || prev_char == ' ' || prev_char == '_' {
                        has_submenu = true;
                        positions.push(match_pos);
                    } else {
                        positions.push(0); // Show full name for commands without separator before match
                    }
                } else {
                    positions.push(0);
                }
            } else {
                positions.push(0); // Show full name for commands at start
            }
        } else {
            positions.push(0); // No match, show full name
        }
    }
    
    if has_submenu {
        positions
    } else {
        Vec::new() // Not a submenu
    }
}

/// Extracts the submenu prefix from the first command that has a separator (dot, space, or underscore) before its match position
pub fn get_submenu_prefix(commands: &[Command], search_text: &str) -> Option<String> {
    for cmd in commands {
        let match_pos = command_matches_query_with_debug(&cmd.command, search_text, false);
        
        if match_pos >= 0 {
            let match_pos = match_pos as usize;
            
            // Check if there's a separator (dot, space, or underscore) before the match position
            if match_pos > 0 {
                if let Some(prev_char) = cmd.command.chars().nth(match_pos - 1) {
                    if prev_char == '.' || prev_char == ' ' || prev_char == '_' {
                        // Return the prefix (everything before the separator)
                        return Some(cmd.command[..match_pos - 1].to_string());
                    }
                }
            }
        }
    }
    None
}

// =============================================================================
// Command Execution
// =============================================================================

/// Writes the command to /tmp/cmd_file for execution by external script
pub fn execute_command(command: &str) {
    let content = format!("execute {}\n", command);
    
    if let Err(e) = fs::write("/tmp/cmd_file", content) {
        eprintln!("Error writing to /tmp/cmd_file: {}", e);
    }
}