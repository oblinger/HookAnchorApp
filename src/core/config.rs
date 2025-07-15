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
pub struct Config {
    /// Popup window settings
    pub popup_settings: PopupSettings,
    /// Launcher behavior settings
    pub launcher_settings: Option<LauncherSettings>,
    /// Unified functions section (both simple and JavaScript)
    pub functions: Option<HashMap<String, serde_yaml::Value>>,
    /// Markdown scanning roots
    pub markdown_roots: Option<Vec<String>>,
    /// Grabber rules for capturing application context
    pub grabber_rules: Option<Vec<crate::grabber::GrabberRule>>,
    /// Key bindings for all actions
    pub keybindings: Option<HashMap<String, String>>,
}

/// Popup settings section of the configuration file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PopupSettings {
    pub max_rows: usize,
    pub max_columns: usize,
    pub debug_log: Option<String>,
    /// Enable verbose debug logging for JavaScript functions and shell commands (default: false)
    pub verbose_logging: Option<bool>,
    /// Enable detailed scanner debug output (default: true)
    pub debug_scanner: Option<bool>,
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
}

/// Launcher settings section of the configuration file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LauncherSettings {
    pub default_browser: Option<String>,
    pub work_browser: Option<String>, 
    pub timeout_ms: Option<u64>,
    pub obsidian_app_name: Option<String>,
    pub obsidian_vault_name: Option<String>,
    pub application_folder: Option<String>,
    pub obsidian_vault_path: Option<String>,
}

impl Default for LauncherSettings {
    fn default() -> Self {
        LauncherSettings {
            default_browser: Some("Google Chrome".to_string()),
            work_browser: Some("Google Chrome Beta".to_string()),
            timeout_ms: Some(5000),
            obsidian_app_name: Some("Obsidian".to_string()),
            obsidian_vault_name: Some("kmr".to_string()),
            application_folder: Some("/Applications/HookAnchor.app".to_string()),
            obsidian_vault_path: Some("~/Documents".to_string()),
        }
    }
}

/// Copies the default config file to the user's config directory
fn copy_default_config(target_path: &Path) -> Result<(), std::io::Error> {
    // First try to find default config relative to the executable
    let exe_path = env::current_exe().unwrap_or_default();
    let exe_dir = exe_path.parent().unwrap_or(Path::new("."));
    
    // Look for default config in several possible locations
    let possible_paths = vec![
        // In the same directory as the executable
        exe_dir.join("resources/common/default_config.yaml"),
        // Up one level from the executable (for development)
        exe_dir.join("../resources/common/default_config.yaml"),
        // Up two levels (for target/release/ha structure)
        exe_dir.join("../../resources/common/default_config.yaml"),
        // Check in current working directory (for development)
        std::env::current_dir().unwrap_or_default().join("resources/common/default_config.yaml"),
    ];
    
    for default_path in &possible_paths {
        if default_path.exists() {
            // Create parent directory if it doesn't exist
            if let Some(parent) = target_path.parent() {
                fs::create_dir_all(parent)?;
            }
            
            // Read default config
            let contents = fs::read_to_string(default_path)?;
            
            // Update the JavaScript file path in the default config
            let project_root = exe_dir.join("../..").canonicalize().unwrap_or_else(|_| exe_dir.to_path_buf());
            let updated_contents = contents.replace(
                "resources/common/tmux_activation.js",
                &format!("{}/resources/common/tmux_activation.js", project_root.display())
            );
            
            // Write to target location
            fs::write(target_path, updated_contents)?;
            
            println!("Created default config file at: {}", target_path.display());
            println!("Please customize it with your settings.");
            
            return Ok(());
        }
    }
    
    Err(std::io::Error::new(
        std::io::ErrorKind::NotFound,
        "Default config file not found in any expected location"
    ))
}

impl Default for PopupSettings {
    fn default() -> Self {
        PopupSettings {
            max_rows: 10,
            max_columns: 1,
            debug_log: None,
            verbose_logging: Some(false), // Default to disabled
            debug_scanner: Some(true), // Default to enabled
            listed_actions: Some("app,url,folder,cmd,chrome,anchor".to_string()),
            merge_similar: true,
            word_separators: " ._-".to_string(),
            scan_interval_seconds: Some(10),
        }
    }
}

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
    
    // If user config doesn't exist, try to copy from default
    if !config_path.exists() {
        if let Err(e) = copy_default_config(&config_path) {
            return ConfigResult::Error(format!(
                "Config file not found: {}\n\
                Failed to copy default config: {}\n\n\
                Please create a config file with your settings.",
                config_path.display(), e
            ));
        }
    }
    
    // Check for user config
    if let Ok(contents) = fs::read_to_string(&config_path) {
        match serde_yaml::from_str::<Config>(&contents) {
            Ok(mut config) => {
                // Apply defaults for optional fields
                if config.popup_settings.merge_similar {
                    // Already set
                } else {
                    config.popup_settings.merge_similar = false;
                }
                
                if config.popup_settings.word_separators.is_empty() {
                    config.popup_settings.word_separators = " ._-".to_string();
                }
                
                return ConfigResult::Success(config);
            }
            Err(e) => {
                // Try to load legacy format first
                if let Ok(config) = load_legacy_config(&contents) {
                    return ConfigResult::Success(config);
                }
                
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
            Please create a config file with markdown_roots specified.",
            config_path.display()
        ));
    }
}

/// Loads configuration from YAML file or returns default if file doesn't exist (compatibility wrapper)
pub fn load_config() -> Config {
    match load_config_with_error() {
        ConfigResult::Success(config) => config,
        ConfigResult::Error(_) => {
            // Fall back to default config for backward compatibility
            // The error will be shown by popup if it uses load_config_with_error
            create_default_config()
        }
    }
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
    
    // Extract functions if it exists
    let functions = yaml.get("functions")
        .and_then(|v| serde_yaml::from_value(v.clone()).ok());
    
    // Extract markdown_roots if it exists
    let markdown_roots = yaml.get("markdown_roots")
        .and_then(|v| serde_yaml::from_value(v.clone()).ok());
    
    // Extract launcher_settings if it exists
    let launcher_settings = yaml.get("launcher_settings")
        .and_then(|v| serde_yaml::from_value(v.clone()).ok());
    
    // Extract grabber_rules if it exists
    let grabber_rules = yaml.get("grabber_rules")
        .and_then(|v| serde_yaml::from_value(v.clone()).ok());
    
    // Extract keybindings if it exists
    let keybindings = yaml.get("keybindings")
        .and_then(|v| serde_yaml::from_value(v.clone()).ok());
    
    Ok(Config {
        popup_settings,
        launcher_settings,
        functions,
        markdown_roots,
        grabber_rules,
        keybindings,
    })
}

/// Creates a default configuration
fn create_default_config() -> Config {
    Config {
        popup_settings: PopupSettings::default(),
        launcher_settings: Some(LauncherSettings::default()),
        functions: Some(HashMap::new()),
        markdown_roots: Some(vec![]),
        grabber_rules: Some(vec![]),
        keybindings: None,
    }
}

impl Config {
    /// Check if a specific action is bound to the given key name
    /// Returns true if the action is bound to this key, false otherwise
    pub fn is_key_bound_to_action(&self, key_name: &str, action_name: &str) -> bool {
        if let Some(ref keybindings) = self.keybindings {
            if let Some(bound_key) = keybindings.get(action_name) {
                return bound_key == key_name;
            }
        }
        false
    }
    
    /// Check if any action is bound to the given key name
    /// Returns the action name if found, None otherwise
    pub fn get_action_for_key(&self, key_name: &str) -> Option<&str> {
        if let Some(ref keybindings) = self.keybindings {
            for (action, bound_key) in keybindings {
                if bound_key == key_name {
                    return Some(action);
                }
            }
        }
        None
    }
}