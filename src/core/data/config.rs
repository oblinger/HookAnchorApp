//! Configuration management
//! 
//! This module handles loading and managing application configuration from YAML files.

use std::fs;
use std::path::{Path, PathBuf};
use std::env;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_yaml;

/// Main configuration structure containing all application settings
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    /// Popup window settings (includes scanner settings now)
    pub popup_settings: PopupSettings,
    /// Launcher behavior settings
    pub launcher_settings: Option<LauncherSettings>,
    /// Grabber rules for capturing application context
    pub grabber_rules: Option<Vec<crate::systems::grabber::GrabberRule>>,
    /// Grabber suffix map - map from suffix strings to regex patterns
    /// When a grabbed URL/path matches a regex, the corresponding suffix is appended
    pub grabber_suffix_map: Option<HashMap<String, String>>,
    /// Unified actions section (new)
    pub actions: Option<HashMap<String, crate::execute::Action>>,
    /// History viewer settings (top-level)
    pub history_viewer: Option<HistoryViewerSettings>,
}

/// Popup settings section of the configuration file
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct PopupSettings {
    pub max_rows: usize,
    pub max_columns: usize,
    /// Enable verbose debug logging for JavaScript functions and shell commands (default: false)
    pub verbose_logging: Option<bool>,
    /// Comma-separated list of actions shown in command editor dropdown
    /// Example: "app,url,folder,cmd,chrome,anchor"
    pub listed_actions: Option<String>,
    /// Enable merging of similar commands (ending with "...")
    pub merge_similar: bool,
    /// Characters used as word separators for command parsing and merging
    /// Default: " ._-" (space, dot, underscore, dash)
    pub word_separators: String,
    /// Seconds between automatic filesystem scans (default: 10)
    pub scan_interval_seconds: Option<u64>,
    /// Seconds of inactivity before popup automatically closes (default: 60)
    pub idle_timeout_seconds: Option<u64>,
    /// Seconds for grabber countdown (default: 5)
    pub countdown_seconds: Option<u8>,
    /// Seconds to remember last executed anchor as ghost input (default: 180)
    pub ghost_timeout_seconds: Option<u64>,
    /// Maximum window size for dialogs and popups in "widthxheight" format (default: "1700x1100")
    pub max_window_size: Option<String>,
    /// Default window size for popups in "widthxheight" format (default: "600x400")
    pub default_window_size: Option<String>,
    /// Maximum log file size in bytes before clearing (default: 1MB = 1,000,000 bytes)
    pub max_log_file_size: Option<u64>,
    /// Keep app running in background for instant popup (default: false)
    pub run_in_background: Option<bool>,
    /// File scanning roots - directories to scan for markdown files and applications
    pub file_roots: Option<Vec<String>>,
    /// Comma-separated file extensions for which to create DOC commands during scanning
    pub doc_file_extensions: Option<String>,
    /// Directory patterns to skip during scanning (glob patterns)
    pub skip_directory_patterns: Option<Vec<String>>,
    /// When renaming a command, also rename the associated document file if the names match (default: false)
    pub rename_doc: Option<bool>,
    /// When renaming an anchor command, also rename the folder if the names match (default: false)
    pub rename_folder: Option<bool>,
    /// When renaming a command that is a patch, update all commands with that patch (default: false)
    pub rename_patch: Option<bool>,
    /// When renaming a command, update all commands that have it as a prefix (default: false)
    pub rename_prefix: Option<bool>,
    /// Preferred action type for selecting primary anchor when multiple anchors exist for same patch
    /// Options: "markdown", "doc", "anchor", "text", etc. (default: "markdown")
    pub preferred_anchor: Option<String>,
    /// Maximum length for command names in display (prevents wide columns, default: 30)
    pub max_characters: Option<usize>,
}

/// History tracking settings
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct HistorySettings {
    /// Maximum number of history entries to load in the history viewer (default: 50000)
    pub viewable_history_limit: Option<usize>,
    /// Width of the tree navigation sidebar in pixels (default: 250)
    pub tree_sidebar_width: Option<f32>,
    /// Minimum width of the tree navigation sidebar in pixels (default: 50)
    pub tree_sidebar_min_width: Option<f32>,
    /// Indent per level in the tree in pixels (default: 10)
    pub tree_indent_pixels: Option<f32>,
    /// Show indent guide lines in the tree (default: true)
    pub tree_show_guides: Option<bool>,
    /// Enable peek-on-hover: when hovering over tree items, temporarily show their history (default: true)
    pub peek_on_hover: Option<bool>,
}

impl Default for HistorySettings {
    fn default() -> Self {
        HistorySettings {
            viewable_history_limit: Some(50000),
            tree_sidebar_width: Some(250.0),
            tree_sidebar_min_width: Some(50.0),
            tree_indent_pixels: Some(10.0),
            tree_show_guides: Some(true),
            peek_on_hover: Some(true),
        }
    }
}

/// History viewer settings (top-level section)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct HistoryViewerSettings {
    /// Maximum number of history entries to load in the history viewer (default: 50000)
    pub viewable_history_limit: Option<usize>,
    /// Width of the tree navigation sidebar in pixels (default: 250)
    pub tree_sidebar_width: Option<f32>,
    /// Minimum width of the tree navigation sidebar in pixels (default: 50)
    pub tree_sidebar_min_width: Option<f32>,
    /// Indent per level in the tree in pixels (default: 10)
    pub tree_indent_pixels: Option<f32>,
    /// Show indent guide lines in the tree (default: true)
    pub tree_show_guides: Option<bool>,
    /// Enable peek-on-hover: when hovering over tree items, temporarily show their history (default: true)
    pub peek_on_hover: Option<bool>,
    /// Key bindings for history viewer
    pub key_bindings: Option<HistoryViewerKeyBindings>,
}

/// Key bindings for history viewer
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct HistoryViewerKeyBindings {
    /// Key for editing selected command (default: ";")
    pub edit_selection: Option<String>,
}

impl Default for HistoryViewerSettings {
    fn default() -> Self {
        HistoryViewerSettings {
            viewable_history_limit: Some(50000),
            tree_sidebar_width: Some(250.0),
            tree_sidebar_min_width: Some(50.0),
            tree_indent_pixels: Some(10.0),
            tree_show_guides: Some(true),
            peek_on_hover: Some(true),
            key_bindings: Some(HistoryViewerKeyBindings::default()),
        }
    }
}

impl Default for HistoryViewerKeyBindings {
    fn default() -> Self {
        HistoryViewerKeyBindings {
            edit_selection: Some(";".to_string()),
        }
    }
}

/// Launcher settings section of the configuration file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LauncherSettings {
 
    pub js_timeout_ms: Option<u64>,
    pub obsidian_app_name: Option<String>,
    pub obsidian_vault_name: Option<String>,
    pub obsidian_vault_path: Option<String>,
    /// Controls whether grabber flips focus during countdown (default: false)
    /// When true: flips to previous app and back during countdown
    /// When false: user manually changes focus during countdown
    pub flip_focus: Option<bool>,
    /// Enable JavaScript TMUX activation instead of Rust
    pub use_javascript_tmux_activation: Option<String>,
    /// Command to run in tmux session after activation
    pub tmux_startup_command: Option<String>,
}

impl Default for LauncherSettings {
    fn default() -> Self {
        LauncherSettings {
            js_timeout_ms: Some(5000),
            obsidian_app_name: Some("Obsidian".to_string()),
            obsidian_vault_name: Some("kmr".to_string()),
            obsidian_vault_path: Some("~/Documents".to_string()),
            flip_focus: Some(false),
            use_javascript_tmux_activation: None,
            tmux_startup_command: None,
        }
    }
}


impl Default for PopupSettings {
    fn default() -> Self {
        PopupSettings {
            max_rows: 20,
            max_columns: 3,
            verbose_logging: Some(false),
            listed_actions: Some("app,url,folder,cmd,chrome,anchor".to_string()),
            merge_similar: true,
            word_separators: " ._-".to_string(),
            scan_interval_seconds: Some(10),
            idle_timeout_seconds: Some(60),
            countdown_seconds: Some(5),
            ghost_timeout_seconds: Some(180),
            max_window_size: Some("1700x1100".to_string()),
            default_window_size: Some("600x400".to_string()),
            max_log_file_size: Some(1_000_000), // 1MB default
            run_in_background: Some(false), // Default to false for safety
            file_roots: None,
            doc_file_extensions: Some("pdf,doc,docx,xls,xlsx,ppt,pptx,txt,rtf,pages,numbers,key".to_string()),
            skip_directory_patterns: Some(vec![
                "node_modules".to_string(),
                "target".to_string(),
                "__pycache__".to_string(),
                ".git".to_string(),
                ".svn".to_string(),
                "*[Tt]rash*".to_string(),
                "*_TO_TRASH_*".to_string(),
                "*.Trash*".to_string(),
                "*[Rr]ecycle*".to_string(),
            ]),
            rename_doc: Some(false),
            rename_folder: Some(false),
            rename_patch: Some(false),
            rename_prefix: Some(false),
            preferred_anchor: Some("markdown".to_string()),
            max_characters: Some(30),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            popup_settings: PopupSettings::default(),
            launcher_settings: Some(LauncherSettings::default()),
            grabber_rules: None,
            grabber_suffix_map: None,
            actions: None,
            history_viewer: Some(HistoryViewerSettings::default()),
        }
    }
}

// Legacy copy_default_config function removed - config creation is now handled
// by the installer in setup_assistant.rs which runs automatically on first launch


/// Returns the path to the YAML config file
pub fn get_config_file_path() -> PathBuf {
    let home = env::var("HOME").unwrap_or_else(|_| ".".to_string());
    Path::new(&home).join(".config/hookanchor/config.yaml")
}

/// Configuration loading result
#[derive(Debug)]
pub enum ConfigResult {
    Success(Config),
    Error(String),
}

/// Loads configuration from YAML file or returns error details if parsing fails
pub fn load_config_with_error() -> ConfigResult {
    let config_path = get_config_file_path();
    
    // If config doesn't exist, the automatic installer in initialize_config()
    // should have already created it. If we still don't have one, return error.
    if !config_path.exists() {
        return ConfigResult::Error(format!(
            "Config file not found: {}\n\n\
            The installer should have created this automatically.\n\
            Try running: ha --install",
            config_path.display()
        ));
    }
    
    // Check for user config
    let read_start = std::time::Instant::now();
    if let Ok(contents) = fs::read_to_string(&config_path) {
        let read_elapsed = read_start.elapsed();
        // Direct write to avoid recursion during config loading
        if let Ok(mut file) = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open("/tmp/hookanchor_timing.log") {
            use std::io::Write;
            let _ = writeln!(file, "CONFIG_TIMING: Config file read in {:?} ({} microseconds)", read_elapsed, read_elapsed.as_micros());
        }
        
        // Parse config with migrations and defaults
        let parse_start = std::time::Instant::now();
        match parse_config_contents(&contents) {
            Ok(config) => {
                // Normalize template keys for efficient matching
                
                let parse_elapsed = parse_start.elapsed();
                // Direct write to avoid recursion during config loading
                if let Ok(mut file) = std::fs::OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open("/tmp/hookanchor_timing.log") {
                    use std::io::Write;
                    let _ = writeln!(file, "CONFIG_TIMING: Config parsed in {:?} ({} microseconds)", parse_elapsed, parse_elapsed.as_micros());
                }
                
                return ConfigResult::Success(config);
            }
            Err(e) => {
                // Failed to parse - return detailed error
                let error_message = format!(
                    "YAML parsing error in config file:\n{}\n\n\
                    Config file location: {}\n\n\
                    Please check the YAML syntax. Common issues:\n\
                    • Missing colons after keys\n\
                    • Incorrect indentation\n\
                    • Invalid characters or text without proper YAML structure",
                    e, config_path.display()
                );
                return ConfigResult::Error(error_message);
            }
        }
    } else {
        return ConfigResult::Error(format!(
            "Config file not found: {}\n\n\
            Please create a config file with file_roots specified.",
            config_path.display()
        ));
    }
}


/// Parses configuration from YAML string, handling migrations and providing defaults
fn parse_config_contents(contents: &str) -> Result<Config, Box<dyn std::error::Error>> {
    // Parse as raw YAML value first
    let yaml: serde_yaml::Value = serde_yaml::from_str(contents)?;
    
    // Extract the popup_settings section if it exists
    let mut popup_settings = if let Some(settings_value) = yaml.get("popup_settings") {
        serde_yaml::from_value(settings_value.clone()).unwrap_or_default()
    } else {
        PopupSettings::default()
    };
    
    // Extract listed_actions if it exists at top level (preserve user's custom actions)
    if let Some(listed_actions_str) = yaml.get("listed_actions").and_then(|v| v.as_str()) {
        popup_settings.listed_actions = Some(listed_actions_str.to_string());
    }
    
    // Handle file_roots at top level - migrate to popup_settings
    if let Some(roots) = yaml.get("file_roots")
        .and_then(|v| serde_yaml::from_value::<Vec<String>>(v.clone()).ok()) {
        popup_settings.file_roots = Some(roots);
    }
    
    
    // Extract launcher_settings if it exists
    let launcher_settings = yaml.get("launcher_settings")
        .and_then(|v| serde_yaml::from_value(v.clone()).ok());
    
    // Extract grabber_rules if it exists
    let grabber_rules = yaml.get("grabber_rules")
        .and_then(|v| serde_yaml::from_value(v.clone()).ok());
    
    // Extract actions if it exists (unified system)
    let actions: Option<HashMap<String, crate::execute::Action>> =
        yaml.get("actions")
            .and_then(|v| serde_yaml::from_value(v.clone()).ok());

    // Extract history_viewer if it exists
    let history_viewer = yaml.get("history_viewer")
        .and_then(|v| serde_yaml::from_value(v.clone()).ok());

    Ok(Config {
        popup_settings,
        launcher_settings,
        grabber_rules,
        grabber_suffix_map: yaml.get("grabber_suffix_map")
            .and_then(|v| serde_yaml::from_value(v.clone()).ok()),
        actions,
        history_viewer,
    })
}

/// Creates a default configuration
pub(crate) fn create_default_config() -> Config {
    Config {
        popup_settings: PopupSettings::default(),
        launcher_settings: Some(LauncherSettings::default()),
        grabber_rules: Some(vec![]),
        grabber_suffix_map: None,
        actions: None,
        history_viewer: Some(HistoryViewerSettings::default()),
    }
}


/// Parse a window size string in "widthxheight" format (e.g., "1700x1100")
/// Returns (width, height) or None if parsing fails
fn parse_window_size(size_str: &str) -> Option<(u32, u32)> {
    let parts: Vec<&str> = size_str.split('x').collect();
    if parts.len() == 2 {
        if let (Ok(width), Ok(height)) = (parts[0].parse::<u32>(), parts[1].parse::<u32>()) {
            return Some((width, height));
        }
    }
    None
}


impl PopupSettings {
    /// Get maximum window width from max_window_size
    pub fn get_max_window_width(&self) -> u32 {
        if let Some(ref size_str) = self.max_window_size {
            if let Some((width, _)) = parse_window_size(size_str) {
                return width;
            }
        }
        1700 // default fallback
    }
    
    /// Get maximum window height from max_window_size
    pub fn get_max_window_height(&self) -> u32 {
        if let Some(ref size_str) = self.max_window_size {
            if let Some((_, height)) = parse_window_size(size_str) {
                return height;
            }
        }
        1100 // default fallback
    }
    
    /// Get default window width, parsing from default_window_size
    pub fn get_default_window_width(&self) -> u32 {
        if let Some(ref size_str) = self.default_window_size {
            if let Some((width, _)) = parse_window_size(size_str) {
                return width;
            }
        }
        600 // hardcoded default
    }
    
    /// Get default window height, parsing from default_window_size
    pub fn get_default_window_height(&self) -> u32 {
        if let Some(ref size_str) = self.default_window_size {
            if let Some((_, height)) = parse_window_size(size_str) {
                return height;
            }
        }
        400 // hardcoded default
    }
}

impl Config {
}

