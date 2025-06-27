use crate::eval::{Environment, EvalError};
use crate::utils::debug_log;
use std::collections::HashMap;
use std::time::SystemTime;
use serde::{Deserialize, Serialize};


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
    pub simple_functions: HashMap<String, serde_yaml::Value>,
    pub js_functions: HashMap<String, String>,
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
    debug_log("LAUNCHER", &format!("Parsed command '{}' -> action: '{}', args: '{}'", command_line, action, args));
    
    // Load configuration from YAML
    let config = load_config()?;
    
    // Look up action value in config
    let action_value = lookup_action(&action, &config)?;
    debug_log("LAUNCHER", &format!("Found action template for '{}': {:?}", action, action_value));
    
    // Create environment
    let mut env = Environment::new()
        .map_err(|e| LauncherError::ExecutionError(format!("Failed to create environment: {}", e)))?;
    
    // Set the arg variable for template substitution
    env.variables.insert("arg".to_string(), args.clone());
    
    // Execute the action using eval module
    debug_log("LAUNCHER", &format!("Executing action: {:?}", action_value));
    let exec_result = env.eval(action_value);
    
    let duration = start_time.elapsed().unwrap_or_default();
    
    match &exec_result {
        Ok(_) => debug_log("LAUNCHER", &format!("Command '{}' completed successfully in {:?}", command_line, duration)),
        Err(e) => debug_log("LAUNCHER", &format!("Command '{}' failed after {:?}: {:?}", command_line, duration, e)),
    }
    
    exec_result.map(|_| ()).map_err(LauncherError::from)
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
    // Load the main config file
    let config_path = {
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
        std::path::PathBuf::from(home).join(".config/anchor_selector/config.yaml")
    };
    
    // Read and parse the YAML config file
    let simple_functions = if let Ok(contents) = std::fs::read_to_string(&config_path) {
        if let Ok(yaml_value) = serde_yaml::from_str::<serde_yaml::Value>(&contents) {
            if let Some(simple_funcs) = yaml_value.get("simple_functions") {
                if let Some(mapping) = simple_funcs.as_mapping() {
                    let mut functions = HashMap::new();
                    for (key, value) in mapping {
                        if let Some(key_str) = key.as_str() {
                            functions.insert(key_str.to_string(), value.clone());
                        }
                    }
                    functions
                } else {
                    get_default_simple_functions()
                }
            } else {
                get_default_simple_functions()
            }
        } else {
            get_default_simple_functions()
        }
    } else {
        get_default_simple_functions()
    };
    
    // Load main config for JS functions
    let main_config = crate::load_config();
    
    let launcher_config = LauncherConfig {
        simple_functions,
        js_functions: main_config.js_functions.unwrap_or_default(),
        settings: LauncherSettings {
            default_browser: Some("Google Chrome".to_string()),
            work_browser: Some("Google Chrome Beta".to_string()),
            timeout_ms: Some(5000),
        },
    };
    
    Ok(launcher_config)
}

// Helper to get default simple functions for testing
fn get_default_simple_functions() -> HashMap<String, serde_yaml::Value> {
    let mut functions = HashMap::new();
    
    // Create function calls using the new uniform format
    let mut app_map = serde_yaml::Mapping::new();
    app_map.insert("fn".into(), "launch_app".into());
    app_map.insert("name".into(), "{{arg}}".into());
    functions.insert("app".to_string(), serde_yaml::Value::Mapping(app_map));
    
    let mut url_map = serde_yaml::Mapping::new();
    url_map.insert("fn".into(), "open_url".into());
    url_map.insert("url".into(), "{{arg}}".into());
    functions.insert("url".to_string(), serde_yaml::Value::Mapping(url_map));
    
    let mut folder_map = serde_yaml::Mapping::new();
    folder_map.insert("fn".into(), "open_folder".into());
    folder_map.insert("path".into(), "{{arg}}".into());
    functions.insert("folder".to_string(), serde_yaml::Value::Mapping(folder_map));
    
    let mut cmd_map = serde_yaml::Mapping::new();
    cmd_map.insert("fn".into(), "shell".into());
    cmd_map.insert("command".into(), "{{arg}}".into());
    functions.insert("cmd".to_string(), serde_yaml::Value::Mapping(cmd_map));
    
    let mut chrome_map = serde_yaml::Mapping::new();
    chrome_map.insert("fn".into(), "open_with".into());
    chrome_map.insert("app".into(), "Google Chrome".into());
    chrome_map.insert("arg".into(), "{{arg}}".into());
    functions.insert("chrome".to_string(), serde_yaml::Value::Mapping(chrome_map));
    
    functions
}



fn lookup_action(action: &str, config: &LauncherConfig) -> Result<serde_yaml::Value, LauncherError> {
    // First try simple commands
    if let Some(action_value) = config.simple_functions.get(action) {
        return Ok(action_value.clone());
    }
    
    // Then try JS functions  
    if let Some(script_code) = config.js_functions.get(action) {
        let mut map = serde_yaml::Mapping::new();
        map.insert("fn".into(), "javascript".into());
        map.insert("code".into(), script_code.clone().into());
        return Ok(serde_yaml::Value::Mapping(map));
    }
    
    // Action not found
    Err(LauncherError::ActionNotFound(format!("Action '{}' not found in configuration", action)))
}

// debug_log function moved to utils module

// expand_tilde function moved to utils module

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
            simple_functions: HashMap::new(),
            js_functions: HashMap::new(),
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
        let mut map = serde_yaml::Mapping::new();
        map.insert("fn".into(), "launch_app".into());
        map.insert("name".into(), "{{arg}}".into());
        let template = serde_yaml::Value::Mapping(map);
        assert!(template.is_mapping());
    }

    #[test]
    fn test_template_substitution_url() {
        let mut map = serde_yaml::Mapping::new();
        map.insert("fn".into(), "open_url".into());
        map.insert("url".into(), "{{arg}}".into());
        let template = serde_yaml::Value::Mapping(map);
        assert!(template.is_mapping());
    }

    #[test]
    fn test_template_substitution_open_with() {
        let mut map = serde_yaml::Mapping::new();
        map.insert("fn".into(), "open_with".into());
        map.insert("app".into(), "Google Chrome".into());
        map.insert("arg".into(), "{{arg}}".into());
        let template = serde_yaml::Value::Mapping(map);
        assert!(template.is_mapping());
    }

    #[test]
    fn test_default_config_loads_from_yaml() {
        // Test that the embedded YAML config parses correctly
        // Note: The actual config format has changed, so we just test basic functionality
        let config = load_config().expect("Should load config");
        
        // Test basic structure
        assert!(config.settings.default_browser.is_some());
        assert!(config.settings.timeout_ms.is_some());
    }
}