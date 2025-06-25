use crate::eval::{ActionSpec, Environment, EvalError, execute};
use std::collections::HashMap;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

const DEFAULT_CONFIG: &str = include_str!("default_config.yaml");

#[derive(Debug, Clone, PartialEq)]
pub enum LauncherError {
    ParseError(String),
    ConfigError(String),
    ExecutionError(String),
    ActionNotFound(String),
    EvalError(EvalError),
}

impl From<EvalError> for LauncherError {
    fn from(err: EvalError) -> Self {
        LauncherError::EvalError(err)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LauncherConfig {
    pub simple_actions: HashMap<String, ActionSpec>,
    pub js_actions: HashMap<String, String>,
    pub settings: LauncherSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LauncherSettings {
    pub default_browser: Option<String>,
    pub work_browser: Option<String>,
    pub timeout_ms: Option<u64>,
}

pub fn launch(command_line: &str) -> Result<(), LauncherError> {
    let start_time = SystemTime::now();
    
    // Parse command_line to extract action and arguments  
    let (action, args) = parse_command_line(command_line)?;
    debug_log(&format!("Parsed command '{}' -> action: '{}', args: '{}'", command_line, action, args));
    
    // Load configuration from YAML
    let config = load_config()?;
    
    // Look up action template in config
    let action_template = lookup_action(&action, &config)?;
    debug_log(&format!("Found action template for '{}': {:?}", action, action_template));
    
    // Substitute templates with actual arguments
    let action_spec = action_template.substitute_templates(&args);
    debug_log(&format!("Template substitution result: {:?}", action_spec));
    
    // Create environment
    let env = Environment::new();
    
    // Execute the action using eval module
    debug_log(&format!("Executing action: {:?}", action_spec));
    let exec_result = execute(action_spec, command_line, &env);
    
    let duration = start_time.elapsed().unwrap_or_default();
    
    match &exec_result {
        Ok(()) => debug_log(&format!("Command '{}' completed successfully in {:?}", command_line, duration)),
        Err(e) => debug_log(&format!("Command '{}' failed after {:?}: {:?}", command_line, duration, e)),
    }
    
    exec_result.map_err(LauncherError::from)
}

fn parse_command_line(command_line: &str) -> Result<(String, String), LauncherError> {
    let trimmed = command_line.trim();
    
    if trimmed.is_empty() {
        return Err(LauncherError::ParseError("Empty command line".to_string()));
    }
    
    // Find first whitespace to split action from arguments
    if let Some(space_pos) = trimmed.find(char::is_whitespace) {
        let action = trimmed[..space_pos].to_string();
        let args = trimmed[space_pos..].trim_start().to_string();
        Ok((action, args))
    } else {
        // Single word command with no arguments
        Ok((trimmed.to_string(), String::new()))
    }
}

fn load_config() -> Result<LauncherConfig, LauncherError> {
    let config_path = get_launcher_config_path();
    
    // If config file doesn't exist, create it from default
    if !config_path.exists() {
        // Ensure config directory exists
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| LauncherError::ConfigError(format!("Failed to create config directory: {}", e)))?;
        }
        
        // Write default config to file
        fs::write(&config_path, DEFAULT_CONFIG)
            .map_err(|e| LauncherError::ConfigError(format!("Failed to write default config: {}", e)))?;
        
        debug_log(&format!("Created default config at: {:?}", config_path));
    }
    
    // Read and parse YAML config
    let contents = fs::read_to_string(&config_path)
        .map_err(|e| LauncherError::ConfigError(format!("Failed to read config file: {}", e)))?;
    
    let config: LauncherConfig = serde_yaml::from_str(&contents)
        .map_err(|e| LauncherError::ConfigError(format!("Failed to parse YAML config: {}", e)))?;
    
    Ok(config)
}

fn get_launcher_config_path() -> PathBuf {
    // Use the main config file (same as popup app)
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    PathBuf::from(home).join(".config/anchor_selector/config.yaml")
}


fn lookup_action(action: &str, config: &LauncherConfig) -> Result<ActionSpec, LauncherError> {
    // First try simple commands
    if let Some(action_spec) = config.simple_actions.get(action) {
        return Ok(action_spec.clone());
    }
    
    // Then try JS actions  
    if let Some(script_code) = config.js_actions.get(action) {
        return Ok(ActionSpec::JavaScript { code: script_code.clone() });
    }
    
    // Action not found
    Err(LauncherError::ActionNotFound(format!("Action '{}' not found in configuration", action)))
}

fn debug_log(message: &str) {
    use crate::load_config;
    
    let config = load_config();
    if let Some(debug_path) = &config.settings.debug_log {
        let debug_path = expand_tilde(debug_path);
        
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        
        let log_entry = format!("[{}] LAUNCHER: {}\n", timestamp, message);
        
        if let Ok(mut file) = OpenOptions::new()
            .create(true)
            .append(true)
            .open(debug_path) {
            let _ = file.write_all(log_entry.as_bytes());
        }
    }
}

fn expand_tilde(path: &str) -> String {
    if path.starts_with('~') {
        if let Ok(home) = std::env::var("HOME") {
            path.replacen('~', &home, 1)
        } else {
            path.to_string()
        }
    } else {
        path.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_command() {
        let result = parse_command_line("app Finder");
        assert!(result.is_ok());
        let (action, args) = result.unwrap();
        assert_eq!(action, "app");
        assert_eq!(args, "Finder");
    }

    #[test]
    fn test_parse_quoted_command() {
        let result = parse_command_line("open_with \"Google Chrome\" \"https://github.com\"");
        assert!(result.is_ok());
        let (action, args) = result.unwrap();
        assert_eq!(action, "open_with");
        assert_eq!(args, "\"Google Chrome\" \"https://github.com\"");
    }

    #[test]
    fn test_parse_url_command() {
        let result = parse_command_line("url https://github.com");
        assert!(result.is_ok());
        let (action, args) = result.unwrap();
        assert_eq!(action, "url");
        assert_eq!(args, "https://github.com");
    }

    #[test]
    fn test_parse_folder_command() {
        let result = parse_command_line("folder /Users/John Doe");
        assert!(result.is_ok());
        let (action, args) = result.unwrap();
        assert_eq!(action, "folder");
        assert_eq!(args, "/Users/John Doe");
    }

    #[test]
    fn test_parse_shell_command() {
        let result = parse_command_line("shell echo hello world");
        assert!(result.is_ok());
        let (action, args) = result.unwrap();
        assert_eq!(action, "shell");
        assert_eq!(args, "echo hello world");
    }

    #[test]
    fn test_launch_app_action() {
        // Test basic app launching action type
        let result = launch("app Finder");
        assert!(result.is_ok(), "App action should work: {:?}", result);
    }

    #[test]
    fn test_launch_url_action() {
        // Test URL opening action type
        let result = launch("url https://github.com");
        assert!(result.is_ok(), "URL action should work: {:?}", result);
    }

    #[test]
    fn test_launch_chrome_action() {
        // Test browser-specific action type
        let result = launch("chrome https://anthropic.com");
        assert!(result.is_ok(), "Chrome action should work: {:?}", result);
    }

    #[test]
    fn test_launch_folder_action() {
        // Test folder opening action type
        let result = launch("folder /Applications");
        assert!(result.is_ok(), "Folder action should work: {:?}", result);
    }

    #[test]
    fn test_launch_shell_action() {
        // Test shell command action type
        let result = launch("cmd echo 'Hello from test'");
        assert!(result.is_ok(), "Shell action should work: {:?}", result);
    }

    #[test]
    fn test_invalid_command_returns_parse_error() {
        let result = parse_command_line("");
        assert!(result.is_err());
        if let Err(LauncherError::ParseError(_)) = result {
            // Expected behavior
        } else {
            panic!("Expected ParseError");
        }
    }

    #[test] 
    fn test_action_not_found_returns_error() {
        // Should return ActionNotFound error when action doesn't exist in config
        let result = launch("nonexistent_action some_arg");
        assert!(result.is_err());
    }

    #[test]
    fn test_launcher_config_creation() {
        let config = LauncherConfig {
            simple_actions: HashMap::new(),
            js_actions: HashMap::new(),
            settings: LauncherSettings {
                default_browser: Some("Google Chrome".to_string()),
                work_browser: None,
                timeout_ms: Some(5000),
            },
        };
        assert_eq!(config.settings.default_browser, Some("Google Chrome".to_string()));
        assert_eq!(config.settings.timeout_ms, Some(5000));
    }

    #[test]
    fn test_template_substitution_app() {
        use crate::eval::ActionSpec;
        let template = ActionSpec::App { name: "{{arg}}".to_string() };
        let result = template.substitute_templates("Finder");
        if let ActionSpec::App { name } = result {
            assert_eq!(name, "Finder");
        } else {
            panic!("Expected App action");
        }
    }

    #[test]
    fn test_template_substitution_url() {
        use crate::eval::ActionSpec;
        let template = ActionSpec::Url { url: "{{arg}}".to_string() };
        let result = template.substitute_templates("https://github.com");
        if let ActionSpec::Url { url } = result {
            assert_eq!(url, "https://github.com");
        } else {
            panic!("Expected Url action");
        }
    }

    #[test]
    fn test_template_substitution_open_with() {
        use crate::eval::ActionSpec;
        let template = ActionSpec::OpenWith { 
            app: "Google Chrome".to_string(), 
            arg: "{{arg}}".to_string() 
        };
        let result = template.substitute_templates("https://anthropic.com");
        if let ActionSpec::OpenWith { app, arg } = result {
            assert_eq!(app, "Google Chrome");
            assert_eq!(arg, "https://anthropic.com");
        } else {
            panic!("Expected OpenWith action");
        }
    }

    #[test]
    fn test_default_config_loads_from_yaml() {
        // Test that the embedded YAML config parses correctly
        let config: LauncherConfig = serde_yaml::from_str(DEFAULT_CONFIG)
            .expect("Default config YAML should parse correctly");
        
        // Test that all Python launcher action types are present
        assert!(config.simple_actions.contains_key("app"));
        assert!(config.simple_actions.contains_key("url"));
        assert!(config.simple_actions.contains_key("folder"));
        assert!(config.simple_actions.contains_key("cmd"));
        assert!(config.simple_actions.contains_key("doc"));
        assert!(config.simple_actions.contains_key("chrome"));
        assert!(config.simple_actions.contains_key("safari"));
        assert!(config.simple_actions.contains_key("brave"));
        assert!(config.simple_actions.contains_key("firefox"));
        assert!(config.simple_actions.contains_key("work"));
        assert!(config.simple_actions.contains_key("notion"));
        assert!(config.simple_actions.contains_key("obs_url"));
        assert!(config.simple_actions.contains_key("1pass"));
        
        // Test JavaScript actions
        assert!(config.js_actions.contains_key("obs"));
        assert!(config.js_actions.contains_key("alias"));
        assert!(config.js_actions.contains_key("anchor"));
    }
}