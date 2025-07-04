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
}

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
    pub merge_similar: bool,
    /// Characters used as word separators for command parsing and merging
    /// Default: " ._-" (space, dot, underscore, dash)
    pub word_separators: String,
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
            merge_similar: true,
            word_separators: " ._-".to_string(),
            scan_root: None,
            scan_interval_seconds: Some(10),
        }
    }
}

/// Returns the path to the YAML config file
pub fn get_config_file_path() -> PathBuf {
    let home = env::var("HOME").unwrap_or_else(|_| ".".to_string());
    Path::new(&home).join(".config/anchor_selector/config.yaml")
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
    
    Ok(Config {
        popup_settings,
        launcher_settings,
        functions,
        markdown_roots,
        grabber_rules,
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
    }
}