//! Anchor Selector Library
//! 
//! A command management and filtering library that provides fuzzy matching
//! and prioritized search for command execution.
//!
//! For complete configuration documentation including JavaScript API, built-in functions,
//! and usage examples, see `docs.md` in the project root.

// New launcher modules
pub mod eval;
pub mod launcher;
pub mod js_runtime;
pub mod business_logic;
pub mod builtin_fns;
pub mod utils;
pub mod scanner;

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};
use chrono::Local;

// =============================================================================
// State Management
// =============================================================================

/// Application state that persists between runs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppState {
    /// Last saved window position
    pub window_position: Option<(f32, f32)>,
    /// Unix timestamp when this build was created
    pub build_time: Option<i64>,
    /// Unix timestamp when filesystem scan was last performed
    pub last_scan_time: Option<i64>,
    /// Checksum derived from last filesystem scan results
    pub last_scan_checksum: Option<String>,
}

impl Default for AppState {
    fn default() -> Self {
        AppState {
            window_position: None,
            build_time: None,
            last_scan_time: None,
            last_scan_checksum: None,
        }
    }
}

/// Returns the path to the state.json file
fn get_state_file_path() -> PathBuf {
    let home = env::var("HOME").unwrap_or_else(|_| ".".to_string());
    Path::new(&home).join(".config/anchor_selector/state.json")
}

/// Loads application state from state.json file, returns default if file doesn't exist or is invalid
pub fn load_state() -> AppState {
    let state_path = get_state_file_path();
    
    if let Ok(contents) = fs::read_to_string(&state_path) {
        match serde_json::from_str::<AppState>(&contents) {
            Ok(state) => state,
            Err(_) => AppState::default()
        }
    } else {
        AppState::default()
    }
}

/// Saves application state to state.json file
pub fn save_state(state: &AppState) -> Result<(), Box<dyn std::error::Error>> {
    let state_path = get_state_file_path();
    
    // Ensure config directory exists
    if let Some(parent) = state_path.parent() {
        fs::create_dir_all(parent)?;
    }
    
    // Serialize and save
    let json_content = serde_json::to_string_pretty(state)?;
    fs::write(&state_path, json_content)?;
    
    Ok(())
}

/// Updates build time in state.json file
pub fn save_state_with_build_time() -> Result<(), Box<dyn std::error::Error>> {
    let mut state = load_state();
    state.build_time = Some(Local::now().timestamp());
    save_state(&state)
}

// =============================================================================
// Configuration
// =============================================================================

/// Popup settings section of the configuration file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PopupSettings {
    pub max_rows: usize,
    pub max_columns: usize,
    pub use_new_launcher: bool,
    pub debug_log: Option<String>,
    /// Comma-separated list of actions shown in command editor dropdown
    /// Example: "app,url,folder,cmd,chrome,anchor"
    pub listed_actions: Option<String>,
    /// Enable merging of similar commands (ending with "...")
    pub merge_similar: Option<bool>,
    /// Characters used as word separators for command parsing and merging
    /// Default: " ._-" (space, dot, underscore, dash)
    pub word_separators: Option<String>,
    /// Root directory for file system scanning (F7 key functionality)
    pub scan_root: Option<String>,
    /// Seconds between automatic filesystem scans (default: 10)
    pub scan_interval_seconds: Option<u64>,
}

/// Launcher settings section of the configuration file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LauncherSettings {
    pub default_browser: Option<String>,
    pub work_browser: Option<String>, 
    pub timeout_ms: Option<u64>,
    pub obsidian_app_name: Option<String>,
    pub obsidian_vault_name: Option<String>,
}

impl Default for LauncherSettings {
    fn default() -> Self {
        LauncherSettings {
            default_browser: Some("Google Chrome".to_string()),
            work_browser: Some("Google Chrome Beta".to_string()),
            timeout_ms: Some(5000),
            obsidian_app_name: Some("Obsidian".to_string()),
            obsidian_vault_name: Some("kmr".to_string()),
        }
    }
}

impl Default for PopupSettings {
    fn default() -> Self {
        PopupSettings {
            max_rows: 10,
            max_columns: 1,
            use_new_launcher: false, // Default to old launcher for backward compatibility
            debug_log: None,
            listed_actions: Some("app,url,folder,cmd,chrome,anchor".to_string()),
            merge_similar: Some(true), // Enable merging by default
            word_separators: Some(" ._-".to_string()), // space, dot, underscore, dash
            scan_root: Some("~/ob/kmr".to_string()),
            scan_interval_seconds: Some(10), // 10 seconds default
        }
    }
}

/// Application configuration loaded from YAML config file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub popup_settings: PopupSettings,
    /// Launcher application settings
    pub launcher_settings: Option<LauncherSettings>,
    /// User-defined JavaScript functions that extend the global function namespace
    /// These functions are available in all JavaScript contexts (actions, business logic, etc.)
    pub js_functions: Option<std::collections::HashMap<String, String>>,
    /// List of root directories to scan for markdown files
    pub markdown_roots: Option<Vec<String>>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            popup_settings: PopupSettings::default(),
            launcher_settings: Some(LauncherSettings::default()),
            js_functions: None,
            markdown_roots: None,
        }
    }
}

/// Loads configuration from the YAML config file, returns default if file doesn't exist or is invalid
pub fn load_config() -> Config {
    let config_path = get_config_file_path();
    
    if let Ok(contents) = fs::read_to_string(&config_path) {
        // First try to load as new format
        match serde_yaml::from_str::<Config>(&contents) {
            Ok(config) => config,
            Err(_) => {
                // If that fails, try to load as old format and migrate
                match load_legacy_config(&contents) {
                    Ok(migrated_config) => migrated_config,
                    Err(_) => Config::default()
                }
            }
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


/// Loads and migrates configuration from legacy format (settings -> popup_settings)
fn load_legacy_config(contents: &str) -> Result<Config, Box<dyn std::error::Error>> {
    // Parse as raw YAML value first
    let yaml: serde_yaml::Value = serde_yaml::from_str(contents)?;
    
    // Extract the settings section if it exists
    let mut popup_settings = if let Some(settings_value) = yaml.get("settings") {
        serde_yaml::from_value(settings_value.clone()).unwrap_or_default()
    } else {
        PopupSettings::default()
    };
    
    // Extract listed_actions if it exists at top level (preserve user's custom actions)
    if let Some(listed_actions_str) = yaml.get("listed_actions").and_then(|v| v.as_str()) {
        popup_settings.listed_actions = Some(listed_actions_str.to_string());
    }
    
    // Extract js_functions if it exists
    let js_functions = yaml.get("js_functions")
        .and_then(|v| serde_yaml::from_value(v.clone()).ok());
    
    // Extract markdown_roots if it exists
    let markdown_roots = yaml.get("markdown_roots")
        .and_then(|v| serde_yaml::from_value(v.clone()).ok());
    
    // Extract launcher_settings if it exists
    let launcher_settings = yaml.get("launcher_settings")
        .and_then(|v| serde_yaml::from_value(v.clone()).ok());
    
    Ok(Config {
        popup_settings,
        launcher_settings,
        js_functions,
        markdown_roots,
    })
}


// =============================================================================
// Data Structures
// =============================================================================

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
            result.push(',');
            result.push_str(&self.flags);
        }
        
        // End with semicolon
        result.push(';');
        
        result
    }
}

// =============================================================================
// File I/O Operations
// =============================================================================

/// Returns the path to the commands file
fn get_commands_file_path() -> PathBuf {
    let home = env::var("HOME").unwrap_or_else(|_| ".".to_string());
    Path::new(&home).join(".config/anchor_selector/commands.txt")
}

/// Loads commands from the commands.txt file and parses them into Command structs
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
/// Only supports new format: [GROUP !] COMMAND : ACTION [ARG] [,flag1,flag2...];
fn parse_commands(contents: &str) -> Vec<Command> {
    contents
        .lines()
        .filter_map(|line| {
            let line = line.trim();
            if line.is_empty() {
                return None;
            }
            
            parse_command_line(line)
        })
        .collect()
}

/// Parses a single command line in new format
fn parse_command_line(line: &str) -> Option<Command> {
    // All lines must end with semicolon
    if !line.ends_with(';') {
        eprintln!("Warning: Line missing semicolon terminator: {}", line);
        return None;
    }
    
    parse_new_format(line)
}

/// Parses new format: [GROUP !] COMMAND : ACTION [ARG] [,flag1,flag2...];
fn parse_new_format(line: &str) -> Option<Command> {
    // Remove the trailing semicolon
    let line_without_semicolon = &line[..line.len()-1];
    
    // Split by colon to get command part and action part
    let colon_pos = line_without_semicolon.find(':')?;
    let command_part = line_without_semicolon[..colon_pos].trim();
    let action_part = line_without_semicolon[colon_pos+1..].trim();
    
    // Parse command part for group and command
    let (group, command) = if let Some(excl_pos) = command_part.find('!') {
        let group = command_part[..excl_pos].trim().to_string();
        let command = command_part[excl_pos+1..].trim().to_string();
        (group, command)
    } else {
        (String::new(), command_part.to_string())
    };
    
    // Parse action part for action, arg, and flags
    let (action, arg, flags) = parse_action_part(action_part);
    
    Some(Command {
        group,
        command,
        action,
        arg,
        flags,
        full_line: line.to_string(),
    })
}

/// Parses the action part: ACTION [ARG] [,flag1,flag2...]
/// Returns (action, arg, flags)
fn parse_action_part(action_part: &str) -> (String, String, String) {
    // Look for the first comma to separate action/arg from flags
    if let Some(comma_pos) = action_part.find(',') {
        let action_arg_part = action_part[..comma_pos].trim();
        let flags_part = action_part[comma_pos+1..].trim();
        
        let (action, arg) = parse_action_and_arg(action_arg_part);
        (action, arg, flags_part.to_string())
    } else {
        let (action, arg) = parse_action_and_arg(action_part);
        (action, arg, String::new())
    }
}

/// Parses action and argument from a string like "ACTION ARG"
fn parse_action_and_arg(action_arg: &str) -> (String, String) {
    let parts: Vec<&str> = action_arg.splitn(2, ' ').collect();
    if parts.len() == 2 {
        (parts[0].to_string(), parts[1].to_string())
    } else {
        (action_arg.to_string(), String::new())
    }
}

/// Saves commands to commands.txt using their original full_line format
pub fn save_commands(commands: &[Command]) -> Result<(), Box<dyn std::error::Error>> {
    let file_path = get_commands_file_path();
    
    let contents = commands.iter()
        .map(|cmd| cmd.full_line.clone())
        .collect::<Vec<String>>()
        .join("\n");
    
    fs::write(&file_path, contents)?;
    Ok(())
}

/// Creates a backup of the existing file before overwriting
fn backup_file_if_exists(file_path: &Path, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    if file_path.exists() {
        // Create backups directory
        let backup_dir = file_path.parent()
            .ok_or("No parent directory")?
            .join("backups");
        
        if !backup_dir.exists() {
            fs::create_dir_all(&backup_dir)?;
        }
        
        // Generate timestamp
        let timestamp = Local::now().format("%Y-%m-%d_%H-%M-%S");
        let backup_filename = format!("{} {}", timestamp, filename);
        let backup_path = backup_dir.join(backup_filename);
        
        // Copy file to backup
        fs::copy(file_path, backup_path)?;
    }
    Ok(())
}

/// Migrates commands from old format to new format and updates full_line
pub fn migrate_commands_to_new_format(commands: &mut [Command]) {
    for cmd in commands {
        // Convert to new format and update full_line
        cmd.update_full_line();
    }
}

/// Saves commands to a specified file with proper formatting and newline handling
/// Uses new format for all commands
pub fn save_commands_formatted(commands: &[Command], output_file: &str) -> Result<(), Box<dyn std::error::Error>> {
    let home = env::var("HOME").unwrap_or_else(|_| ".".to_string());
    let file_path = Path::new(&home).join(".config/anchor_selector").join(output_file);
    
    // Create backup before overwriting
    backup_file_if_exists(&file_path, output_file)?;
    
    let contents = commands.iter()
        .map(|cmd| cmd.to_new_format())
        .collect::<Vec<String>>()
        .join("\n") + "\n";
    
    fs::write(&file_path, contents)?;
    Ok(())
}

/// Saves commands to a specified file using original full_line format (backward compatibility)
pub fn save_commands_original_format(commands: &[Command], output_file: &str) -> Result<(), Box<dyn std::error::Error>> {
    let home = env::var("HOME").unwrap_or_else(|_| ".".to_string());
    let file_path = Path::new(&home).join(".config/anchor_selector").join(output_file);
    
    // Create backup before overwriting
    backup_file_if_exists(&file_path, output_file)?;
    
    let contents = commands.iter()
        .map(|cmd| cmd.full_line.clone())
        .collect::<Vec<String>>()
        .join("\n") + "\n";
    
    fs::write(&file_path, contents)?;
    Ok(())
}

/// Convenience function to save commands to the default commands.txt file using new format
pub fn save_commands_to_file(commands: &[Command]) -> Result<(), Box<dyn std::error::Error>> {
    save_commands_formatted(commands, "commands.txt")
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
    }
    
    // Categorize matches by relevance
    let mut exact_matches = Vec::new();
    let mut command_start_matches = Vec::new();
    let mut word_start_matches = Vec::new();
    let mut prefix_matches = Vec::new();
    
    for cmd in commands {
        let cmd_lower = cmd.command.to_lowercase();
        
        if debug && cmd_lower.contains("selector") && search_words.contains(&"s") {
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
    let _exact_count = exact_matches.len();
    let _cmd_start_count = command_start_matches.len();
    let _word_count = word_start_matches.len();
    let _prefix_count = prefix_matches.len();
    
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
/// - Empty list if no submenu detected (no prefix of search text matches any anchor)
/// - List of integers indicating where to start displaying each command's suffix
///   - For commands with separator before match position: returns the match position
///   - For commands without separator before match position: returns 0 (show full name)
/// 
/// A submenu is shown when any prefix of the search text matches an anchor entry
/// (the text before a separator like '.', ' ', or '_')
pub fn get_submenu_display_positions(commands: &[Command], search_text: &str) -> Vec<usize> {
    let mut positions = Vec::new();
    let mut submenu_anchor: Option<String> = None;
    
    // First pass: check if any prefix of search text matches any anchor
    // Check from longest to shortest prefix to find the most specific match
    for prefix_len in (1..=search_text.len()).rev() {
        let prefix = &search_text[..prefix_len];
        
        for cmd in commands {
            // Find all separators in the command
            for (i, ch) in cmd.command.char_indices() {
                if ch == '.' || ch == ' ' || ch == '_' {
                    let anchor = &cmd.command[..i];
                    if anchor.to_lowercase() == prefix.to_lowercase() {
                        submenu_anchor = Some(anchor.to_string());
                        break;
                    }
                }
            }
            if submenu_anchor.is_some() {
                break;
            }
        }
        if submenu_anchor.is_some() {
            break;
        }
    }
    
    // If no anchor prefix match, return empty (no submenu)
    if submenu_anchor.is_none() {
        return Vec::new();
    }
    
    // Second pass: calculate display positions for all commands
    for cmd in commands {
        let match_pos = command_matches_query_with_debug(&cmd.command, search_text, false);
        
        if match_pos >= 0 {
            let match_pos = match_pos as usize;
            
            // Check if there's a separator at the match position - this determines if it's a submenu item
            if match_pos < cmd.command.len() {
                if let Some(char_at_match_pos) = cmd.command.chars().nth(match_pos) {
                    if char_at_match_pos == ' ' || char_at_match_pos == '.' || char_at_match_pos == '_' {
                        // There's a separator at match position - this is a submenu item
                        // Trim from position after the separator (match_pos + 1)
                        positions.push(match_pos + 1);
                    } else {
                        // No separator at match position - not a submenu item, show full name
                        positions.push(0);
                    }
                } else {
                    positions.push(0); // Beyond string length
                }
            } else {
                positions.push(0); // Complete match, show full name
            }
        } else {
            positions.push(0); // No match, show full name
        }
    }
    
    positions
}

/// Determines the current submenu prefix from the filtered commands and search text
/// Returns None if not in a submenu
pub fn get_current_submenu_prefix(commands: &[Command], search_text: &str) -> Option<String> {
    // Use existing logic to find submenu prefix
    get_submenu_prefix(commands, search_text)
}

/// Splits commands into those inside and outside the given menu
/// Returns (inside_menu, outside_menu) vectors
pub fn split_commands(commands: &[Command], menu_prefix: &str) -> (Vec<Command>, Vec<Command>) {
    let mut inside_menu = Vec::new();
    let mut outside_menu = Vec::new();
    
    for cmd in commands {
        // Check if command starts with menu_prefix followed by a separator
        if cmd.command.len() > menu_prefix.len() {
            let prefix_end = menu_prefix.len();
            if cmd.command[..prefix_end].eq_ignore_ascii_case(menu_prefix) {
                // Check if there's a separator right after the menu prefix
                if let Some(ch) = cmd.command.chars().nth(prefix_end) {
                    if ch == ' ' || ch == '.' || ch == '_' {
                        inside_menu.push(cmd.clone());
                        continue;
                    }
                }
            }
        } else if cmd.command.eq_ignore_ascii_case(menu_prefix) {
            // Exact match with menu prefix
            inside_menu.push(cmd.clone());
            continue;
        }
        
        // If we get here, it's outside the menu
        outside_menu.push(cmd.clone());
    }
    
    (inside_menu, outside_menu)
}

/// Extracts the submenu prefix when any prefix of search text matches an anchor entry
/// Returns the longest matching anchor prefix found in the search text
pub fn get_submenu_prefix(commands: &[Command], search_text: &str) -> Option<String> {
    // Check if any prefix of search text matches any anchor
    // Check from longest to shortest prefix to find the most specific match
    for prefix_len in (1..=search_text.len()).rev() {
        let prefix = &search_text[..prefix_len];
        
        for cmd in commands {
            // Find all separators in the command
            for (i, ch) in cmd.command.char_indices() {
                if ch == '.' || ch == ' ' || ch == '_' {
                    let anchor = &cmd.command[..i];
                    if anchor.to_lowercase() == prefix.to_lowercase() {
                        // Return the exact anchor text as the prefix
                        return Some(anchor.to_string());
                    }
                }
            }
        }
    }
    None
}

/// Reorders commands for submenu display: suffix-only commands first, then separator, then full-name commands
/// Returns (reordered_commands, reordered_positions, separator_index)
/// separator_index is None if no separator was added
pub fn reorder_commands_for_submenu(commands: &[Command], positions: &[usize]) -> (Vec<Command>, Vec<usize>, Option<usize>) {
    if positions.is_empty() || commands.len() != positions.len() {
        return (commands.to_vec(), positions.to_vec(), None);
    }
    
    let mut suffix_commands = Vec::new();
    let mut suffix_positions = Vec::new();
    let mut full_commands = Vec::new();
    let mut full_positions = Vec::new();
    
    // Separate commands based on display characteristics:
    // - Suffix commands: either show suffix (pos > 0) OR are short single words that match the submenu pattern
    // - Full commands: longer compound names that show full text (pos == 0)
    for (cmd, &pos) in commands.iter().zip(positions.iter()) {
        if pos > 0 {
            // Definitely shows suffix only - these go first
            suffix_commands.push(cmd.clone());
            suffix_positions.push(pos);
        } else {
            // pos == 0 means showing full name - but we need to distinguish:
            // - Short single words (like "FIN") that belong with submenu items
            // - Longer compound commands (like "Findem Note") that are separate
            
            let cmd_words: Vec<&str> = cmd.command.split_whitespace().collect();
            let is_single_short_word = cmd_words.len() == 1 && cmd.command.len() <= 8;
            
            if is_single_short_word {
                // Short single words stay with submenu items
                suffix_commands.push(cmd.clone());
                suffix_positions.push(pos);
            } else {
                // Longer/compound commands go to full section
                full_commands.push(cmd.clone());
                full_positions.push(pos);
            }
        }
    }
    
    // If we have both types, create reordered lists with separator
    if !suffix_commands.is_empty() && !full_commands.is_empty() {
        let mut reordered_commands = suffix_commands;
        let mut reordered_positions = suffix_positions;
        
        // Add separator command
        let separator_index = reordered_commands.len();
        let separator_command = Command {
            group: String::new(),
            command: "---".to_string(),
            action: "separator".to_string(),
            arg: String::new(),
            flags: String::new(),
            full_line: "--- : separator;".to_string(),
        };
        reordered_commands.push(separator_command);
        reordered_positions.push(0); // Separator shows full text
        
        // Add full-name commands
        reordered_commands.extend(full_commands);
        reordered_positions.extend(full_positions);
        
        (reordered_commands, reordered_positions, Some(separator_index))
    } else {
        // Only one type, no reordering needed
        (commands.to_vec(), positions.to_vec(), None)
    }
}

/// Helper function to check if a character is a word separator
fn is_word_separator(ch: char, separators: &str) -> bool {
    separators.contains(ch)
}

/// Helper function to remove the last word from a command string
/// Returns None if the string has no separators (i.e., is a single word)
fn remove_last_word(text: &str, separators: &str) -> Option<String> {
    // Find the last occurrence of any separator
    if let Some(last_sep_pos) = text.rfind(|c| is_word_separator(c, separators)) {
        Some(text[..last_sep_pos].to_string())
    } else {
        None // No separators found, can't remove last word
    }
}

/// Check if a candidate string is valid for merging based on the search text
/// A candidate is invalid if the user has typed past a word boundary into the next word
fn is_valid_merge_candidate(candidate: &str, search_text: &str, active_prefix: &str, separators: &str) -> bool {
    // Candidate must be longer than active prefix
    if candidate.len() <= active_prefix.len() {
        return false;
    }
    
    // For validation, we need to check against the actual search text, not just the submenu prefix
    let candidate_lower = candidate.to_lowercase();
    let search_lower = search_text.to_lowercase();
    
    // Find where the search text matches in the candidate
    if let Some(search_pos) = candidate_lower.find(&search_lower) {
        let match_end = search_pos + search_text.len();
        
        // If search text is empty or very short, candidate is valid
        if search_text.is_empty() || match_end == 0 {
            return true;
        }
        
        // Check one character back from match end
        let check_pos = match_end - 1;
        if check_pos < candidate.len() {
            if let Some(ch) = candidate.chars().nth(check_pos) {
                // If we're not at a separator (i.e., we're inside a word)
                if !is_word_separator(ch, separators) {
                    // Check if there's any separator from match_end to end of string
                    if match_end < candidate.len() {
                        let remaining = &candidate[match_end..];
                        if !remaining.chars().any(|c| is_word_separator(c, separators)) {
                            // No separator found after match point - user has typed into a specific word
                            // This candidate should NOT be merged
                            return false;
                        }
                    }
                }
            }
        }
        
        true
    } else {
        // If search text doesn't match the candidate, it's not valid for merging
        false
    }
}

/// New merge_similar_commands implementation based on word removal approach
pub fn merge_similar_commands(commands: &[Command], search_text: &str) -> Vec<Command> {
    if commands.is_empty() {
        return commands.to_vec();
    }
    
    // Check if merging is disabled in configuration
    let config = load_config();
    if !config.popup_settings.merge_similar.unwrap_or(true) {
        return commands.to_vec();
    }
    
    // Get word separators from config
    let separators = config.popup_settings.word_separators
        .as_deref()
        .unwrap_or(" ._-");
    
    // Step 1: Determine active prefix
    let active_prefix = get_current_submenu_prefix(commands, search_text)
        .unwrap_or_default();
    
    // Step 2: Generate valid candidate strings by removing last word from each command
    let mut valid_candidates = std::collections::HashSet::new();
    for cmd in commands {
        if let Some(candidate) = remove_last_word(&cmd.command, separators) {
            // Use the new validation logic to check if this candidate should be merged
            if is_valid_merge_candidate(&candidate, search_text, &active_prefix, separators) {
                valid_candidates.insert(candidate);
            }
        }
    }
    
    // Step 3: Group commands by matching them against candidates
    let mut groups: std::collections::HashMap<String, Vec<&Command>> = std::collections::HashMap::new();
    let mut unmatched_commands = Vec::new();
    
    for cmd in commands {
        let mut matched = false;
        
        // Try direct match against candidates
        if valid_candidates.contains(&cmd.command) {
            groups.entry(cmd.command.clone()).or_insert_with(Vec::new).push(cmd);
            matched = true;
        } else {
            // Try removing 1 word and matching against candidates
            if let Some(shortened) = remove_last_word(&cmd.command, separators) {
                if valid_candidates.contains(&shortened) {
                    groups.entry(shortened).or_insert_with(Vec::new).push(cmd);
                    matched = true;
                }
            }
        }
        
        if !matched {
            unmatched_commands.push(cmd);
        }
    }
    
    // Step 4: Create final result
    let mut result = Vec::new();
    
    // Process groups
    for (candidate, command_list) in groups {
        if command_list.len() >= 2 {
            // Create merged entry
            let merged_command = Command {
                group: command_list[0].group.clone(),
                command: format!("{}...", candidate),
                action: command_list[0].action.clone(),
                arg: command_list[0].arg.clone(),
                flags: command_list[0].flags.clone(),
                full_line: command_list[0].full_line.clone(),
            };
            result.push(merged_command);
        } else {
            // Single command in group, keep unchanged
            result.extend(command_list.iter().cloned().cloned());
        }
    }
    
    // Add unmatched commands
    result.extend(unmatched_commands.iter().cloned().cloned());
    
    // Sort to maintain consistent order
    result.sort_by(|a, b| a.command.cmp(&b.command));
    
    result
}


// =============================================================================
// Command Execution
// =============================================================================

/// Executes a command using either the new launcher system or the old temp file method
pub fn execute_command(command: &str) {
    let config = load_config();
    
    if config.popup_settings.use_new_launcher {
        // Use new launcher system - first look up the command to get action and arg
        let commands = load_commands();
        
        // Find the command by name
        if let Some(cmd) = commands.iter().find(|c| c.command == command) {
            // Construct launcher command from action and arg
            let launcher_command = if cmd.arg.is_empty() {
                cmd.action.clone()
            } else {
                format!("{} {}", cmd.action, cmd.arg)
            };
            
            use crate::launcher::launch;
            if let Err(e) = launch(&launcher_command) {
                eprintln!("Error executing command with new launcher: {:?}", e);
            }
        } else {
            eprintln!("Command '{}' not found in commands list", command);
        }
    } else {
        // Use old temp file method for backward compatibility
        let content = format!("execute {}\n", command);
        
        if let Err(e) = fs::write("/tmp/cmd_file", content) {
            eprintln!("Error writing to /tmp/cmd_file: {}", e);
        }
    }
}

/// Gets the list of actions for the command editor dropdown
/// Returns the configured actions from popup_settings.listed_actions, or default actions if not configured
pub fn get_listed_actions() -> Vec<String> {
    let config = load_config();
    
    match config.popup_settings.listed_actions {
        Some(actions_str) => {
            // Split by comma and trim whitespace
            actions_str
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect()
        },
        None => vec![
            "app".to_string(),
            "url".to_string(),
            "folder".to_string(), 
            "cmd".to_string(),
            "chrome".to_string(),
            "anchor".to_string(),
        ],
    }
}