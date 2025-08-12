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
    /// Popup window settings (includes scanner settings now)
    pub popup_settings: PopupSettings,
    /// Launcher behavior settings
    pub launcher_settings: Option<LauncherSettings>,
    /// Unified functions section (both simple and JavaScript)
    pub functions: Option<HashMap<String, serde_yaml::Value>>,
    /// Grabber rules for capturing application context
    pub grabber_rules: Option<Vec<crate::grabber::GrabberRule>>,
    /// Key bindings for all actions (legacy)
    pub keybindings: Option<HashMap<String, String>>,
    /// Templates for creating new commands (legacy - migrated to actions)
    pub templates: Option<HashMap<String, crate::core::template_creation::Template>>,
    /// Unified actions section (new)
    pub actions: Option<HashMap<String, crate::core::unified_actions::Action>>,
}

/// Popup settings section of the configuration file
#[derive(Debug, Clone, Serialize, Deserialize)]
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
    /// Maximum window size for dialogs and popups in "widthxheight" format (default: "1700x1100")
    pub max_window_size: Option<String>,
    /// Default window size for popups in "widthxheight" format (default: "600x400")
    pub default_window_size: Option<String>,
    /// Maximum log file size in bytes before clearing (default: 1MB = 1,000,000 bytes)
    pub max_log_file_size: Option<u64>,
    /// Keep app running in background for instant popup (default: false)
    pub run_in_background: Option<bool>,
    /// Markdown scanning roots - directories to scan for markdown files
    pub markdown_roots: Option<Vec<String>>,
    /// Path where orphan anchor files are created (default: ~/ob/kmr/SYS/Closet/Orphans)
    pub orphans_path: Option<String>,
    /// Directory patterns to skip during scanning (glob patterns)
    pub skip_directory_patterns: Option<Vec<String>>,
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
}

impl Default for LauncherSettings {
    fn default() -> Self {
        LauncherSettings {
            js_timeout_ms: Some(5000),
            obsidian_app_name: Some("Obsidian".to_string()),
            obsidian_vault_name: Some("kmr".to_string()),
            obsidian_vault_path: Some("~/Documents".to_string()),
            flip_focus: Some(false),
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
            max_window_size: Some("1700x1100".to_string()),
            default_window_size: Some("600x400".to_string()),
            max_log_file_size: Some(1_000_000), // 1MB default
            run_in_background: Some(false), // Default to false for safety
            markdown_roots: None,
            orphans_path: Some("/Users/oblinger/ob/kmr/SYS/Closet/Orphans".to_string()),
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
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            popup_settings: PopupSettings::default(),
            launcher_settings: Some(LauncherSettings::default()),
            functions: None,
            grabber_rules: None,
            keybindings: None,
            templates: None,
            actions: None,
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
        
        let parse_start = std::time::Instant::now();
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
                
                // Normalize template keys for efficient matching
                normalize_template_keys(&mut config);
                
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
pub(super) fn load_config() -> Config {
    let start = std::time::Instant::now();
    let result = match load_config_with_error() {
        ConfigResult::Success(config) => config,
        ConfigResult::Error(_) => {
            // Fall back to default config for backward compatibility
            // The error will be shown by popup if it uses load_config_with_error
            create_default_config()
        }
    };
    let elapsed = start.elapsed();
    // Direct write to avoid recursion during config loading
    if let Ok(mut file) = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open("/tmp/hookanchor_timing.log") {
        use std::io::Write;
        let _ = writeln!(file, "CONFIG_TIMING: Total config load time: {:?} ({} microseconds)", elapsed, elapsed.as_micros());
    }
    result
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
    
    // Handle legacy markdown_roots at top level - migrate to popup_settings
    if let Some(roots) = yaml.get("markdown_roots")
        .and_then(|v| serde_yaml::from_value::<Vec<String>>(v.clone()).ok()) {
        popup_settings.markdown_roots = Some(roots);
    }
    
    // Handle legacy scanner_settings - migrate to popup_settings
    if let Some(scanner) = yaml.get("scanner_settings") {
        if let Some(orphans_path) = scanner.get("orphans_path").and_then(|v| v.as_str()) {
            popup_settings.orphans_path = Some(orphans_path.to_string());
        }
        if let Some(patterns) = scanner.get("skip_directory_patterns")
            .and_then(|v| serde_yaml::from_value::<Vec<String>>(v.clone()).ok()) {
            popup_settings.skip_directory_patterns = Some(patterns);
        }
    }
    
    // Extract launcher_settings if it exists
    let launcher_settings = yaml.get("launcher_settings")
        .and_then(|v| serde_yaml::from_value(v.clone()).ok());
    
    // Extract grabber_rules if it exists
    let grabber_rules = yaml.get("grabber_rules")
        .and_then(|v| serde_yaml::from_value(v.clone()).ok());
    
    // Extract keybindings if it exists
    let keybindings = yaml.get("keybindings")
        .and_then(|v| serde_yaml::from_value(v.clone()).ok());
    
    // Extract templates if it exists (legacy)
    let templates = yaml.get("templates")
        .and_then(|v| serde_yaml::from_value(v.clone()).ok());
    
    // Extract actions if it exists (new unified system)
    let mut actions: Option<HashMap<String, crate::core::unified_actions::Action>> = 
        yaml.get("actions")
            .and_then(|v| serde_yaml::from_value(v.clone()).ok());
    
    // If we have templates but no actions, migrate templates to actions
    if actions.is_none() && templates.is_some() {
        actions = migrate_templates_to_actions(&templates);
    }
    
    // If we have keybindings but no corresponding actions, migrate them
    if let Some(ref mut action_map) = actions {
        if let Some(ref kb) = keybindings {
            migrate_keybindings_to_actions(kb, action_map);
        }
    }
    
    Ok(Config {
        popup_settings,
        launcher_settings,
        functions,
        grabber_rules,
        keybindings,
        templates,
        actions,
    })
}

/// Creates a default configuration
pub(crate) fn create_default_config() -> Config {
    Config {
        popup_settings: PopupSettings::default(),
        launcher_settings: Some(LauncherSettings::default()),
        functions: Some(HashMap::new()),
        grabber_rules: Some(vec![]),
        keybindings: None,
        templates: None,
        actions: None,
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

/// Migrate templates to unified actions
fn migrate_templates_to_actions(
    templates: &Option<HashMap<String, crate::core::template_creation::Template>>
) -> Option<HashMap<String, crate::core::unified_actions::Action>> {
    let templates = templates.as_ref()?;
    let mut actions = HashMap::new();
    
    for (name, template) in templates {
        let mut params = HashMap::new();
        
        // Core template fields
        params.insert("name".to_string(), serde_json::Value::String(template.name.clone()));
        params.insert("action".to_string(), serde_json::Value::String(template.action.clone()));
        params.insert("arg".to_string(), serde_json::Value::String(template.arg.clone()));
        params.insert("patch".to_string(), serde_json::Value::String(template.patch.clone()));
        params.insert("flags".to_string(), serde_json::Value::String(template.flags.clone()));
        
        // Optional fields
        if template.edit {
            params.insert("edit".to_string(), serde_json::Value::Bool(true));
        }
        if let Some(ref file) = template.file {
            params.insert("file".to_string(), serde_json::Value::String(file.clone()));
        }
        if let Some(ref contents) = template.contents {
            params.insert("contents".to_string(), serde_json::Value::String(contents.clone()));
        }
        if let Some(grab) = template.grab {
            params.insert("grab".to_string(), serde_json::Value::Number(grab.into()));
        }
        if template.validate_previous_folder {
            params.insert("validate_previous_folder".to_string(), serde_json::Value::Bool(true));
        }
        if template.file_rescan {
            params.insert("file_rescan".to_string(), serde_json::Value::Bool(true));
        }
        
        let action = crate::core::unified_actions::Action {
            action_type: "template".to_string(),
            description: template.description.clone(),
            key: template.key.clone(),
            keystroke: None, // Will be computed later
            params,
        };
        
        actions.insert(name.clone(), action);
    }
    
    Some(actions)
}

/// Migrate keybindings to popup actions
fn migrate_keybindings_to_actions(
    keybindings: &HashMap<String, String>,
    actions: &mut HashMap<String, crate::core::unified_actions::Action>,
) {
    for (action_name, key) in keybindings {
        // Skip if this action already exists (e.g., from templates)
        if actions.contains_key(action_name) {
            continue;
        }
        
        // Create a popup action based on the keybinding name
        let mut params = HashMap::new();
        
        let (action_type, popup_action) = match action_name.as_str() {
            "exit_app" => ("popup", Some("exit")),
            "navigate_down" => {
                params.insert("dx".to_string(), serde_json::Value::Number(0.into()));
                params.insert("dy".to_string(), serde_json::Value::Number(1.into()));
                ("popup", Some("navigate"))
            },
            "navigate_up" => {
                params.insert("dx".to_string(), serde_json::Value::Number(0.into()));
                params.insert("dy".to_string(), serde_json::Value::Number((-1).into()));
                ("popup", Some("navigate"))
            },
            "navigate_left" => {
                params.insert("dx".to_string(), serde_json::Value::Number((-1).into()));
                params.insert("dy".to_string(), serde_json::Value::Number(0.into()));
                ("popup", Some("navigate"))
            },
            "navigate_right" => {
                params.insert("dx".to_string(), serde_json::Value::Number(1.into()));
                params.insert("dy".to_string(), serde_json::Value::Number(0.into()));
                ("popup", Some("navigate"))
            },
            "execute_command" => ("popup", Some("execute")),
            "force_rebuild" => ("popup", Some("rebuild")),
            "show_folder" => ("popup", Some("show_folder")),
            "open_editor" | "edit_active_command" => ("popup", Some("edit_command")),
            "show_keys" => ("popup", Some("show_help")),
            "tmux_activate" => ("tmux", None),
            _ => continue, // Skip unknown keybindings
        };
        
        if let Some(popup_action_name) = popup_action {
            params.insert("popup_action".to_string(), serde_json::Value::String(popup_action_name.to_string()));
        }
        
        let action = crate::core::unified_actions::Action {
            action_type: action_type.to_string(),
            description: Some(format!("Keyboard action: {}", action_name)),
            key: Some(key.clone()),
            keystroke: None, // Will be computed later
            params,
        };
        
        actions.insert(action_name.clone(), action);
    }
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
        self.get_action_for_key_with_modifiers(key_name, &std::collections::HashSet::new())
    }
    
    /// Check if any action is bound to the given key with modifiers (using new Modifiers struct)
    /// Returns the action name if found, None otherwise
    /// NOTE: This is a legacy method - new code should use the KeyRegistry system
    pub fn get_action_for_key_with_modifiers_struct(&self, key_name: &str, modifiers: &crate::core::key_processing::Modifiers) -> Option<&str> {
        use crate::core::key_processing::Keystroke;
        
        // Try to create a keystroke from the key name and modifiers
        if let Ok(target_keystroke) = Keystroke::from_key_string(key_name) {
            let target_with_modifiers = Keystroke::new(target_keystroke.key, modifiers.clone());
            
            if let Some(ref keybindings) = self.keybindings {
                for (action, bound_key) in keybindings {
                    if let Ok(bound_keystroke) = Keystroke::from_key_string(bound_key) {
                        if bound_keystroke == target_with_modifiers {
                            return Some(action);
                        }
                    }
                }
            }
        }
        
        None
    }
    
    /// Check if any action is bound to the given key with modifiers (legacy HashSet version)
    /// Returns the action name if found, None otherwise
    pub fn get_action_for_key_with_modifiers(&self, key_name: &str, modifiers: &std::collections::HashSet<String>) -> Option<&str> {
        // Convert HashSet to Modifiers struct and use new method
        let modifiers_struct = crate::core::key_processing::Modifiers {
            ctrl: modifiers.contains("Ctrl"),
            alt: modifiers.contains("Alt"),
            shift: modifiers.contains("Shift"),
            cmd: modifiers.contains("Cmd"),
        };
        self.get_action_for_key_with_modifiers_struct(key_name, &modifiers_struct)
    }
    
    /// Check if the given key is bound to a template
    /// Returns the template name if found, None otherwise
    pub fn get_template_for_key(&self, key_name: &str) -> Option<&str> {
        self.get_template_for_key_with_modifiers(key_name, &std::collections::HashSet::new())
    }
    
    /// Check if the given key with modifiers is bound to a template
    /// Returns the template name if found, None otherwise
    /// NOTE: This is a legacy method - new code should use get_template_for_keystroke
    pub fn get_template_for_key_with_modifiers(&self, key_name: &str, modifiers: &std::collections::HashSet<String>) -> Option<&str> {
        use crate::core::key_processing::{Keystroke, Modifiers};
        
        // Convert HashSet modifiers to Modifiers struct
        let modifiers_struct = Modifiers {
            ctrl: modifiers.contains("Ctrl"),
            alt: modifiers.contains("Alt"),
            shift: modifiers.contains("Shift"),
            cmd: modifiers.contains("Cmd"),
        };
        
        // Try to create a keystroke from the key name and modifiers
        if let Ok(target_keystroke) = Keystroke::from_key_string(key_name) {
            let target_with_modifiers = Keystroke::new(target_keystroke.key, modifiers_struct);
            
            if let Some(ref templates) = self.templates {
                crate::utils::debug_log("TEMPLATE_BIND", &format!("Checking {} templates for key '{}' with modifiers {:?}", templates.len(), key_name, modifiers));
                for (template_name, template) in templates {
                    if let Some(ref keystroke) = template.keystroke {
                        crate::utils::debug_log("TEMPLATE_BIND", &format!("  Template '{}' has keystroke {:?}", template_name, keystroke));
                        if *keystroke == target_with_modifiers {
                            crate::utils::debug_log("TEMPLATE_BIND", &format!("✓ MATCH: Template '{}' matches key '{}'", template_name, key_name));
                            return Some(template_name);
                        }
                    } else {
                        crate::utils::debug_log("TEMPLATE_BIND", &format!("  Template '{}' has no keystroke binding", template_name));
                    }
                }
            }
        }
        None
    }
    
    /// Check if the given egui event matches any template using the Keystroke system
    /// This provides efficient matching by comparing against pre-computed keystrokes
    pub fn get_template_for_keystroke(&self, event: &egui::Event) -> Option<&str> {
        if let Some(ref templates) = self.templates {
            for (template_name, template) in templates {
                if let Some(ref keystroke) = template.keystroke {
                    if keystroke.matches_event(event) {
                        return Some(template_name);
                    }
                }
            }
        }
        None
    }
}

/// Convert template keys to Keystroke objects for efficient matching
fn normalize_template_keys(config: &mut Config) {
    use crate::core::key_processing::Keystroke;
    
    if let Some(ref mut templates) = config.templates {
        for (template_name, template) in templates.iter_mut() {
            if let Some(ref key_str) = template.key {
                match Keystroke::from_key_string(key_str) {
                    Ok(keystroke) => {
                        template.keystroke = Some(keystroke);
                        crate::utils::detailed_log("CONFIG", &format!("✅ Template '{}' key '{}' → {:?}", template_name, key_str, template.keystroke));
                    }
                    Err(e) => {
                        crate::utils::log_error(&format!("Failed to parse key '{}' for template '{}': {}", key_str, template_name, e));
                    }
                }
            }
        }

    }
}