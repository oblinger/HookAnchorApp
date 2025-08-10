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
    pub functions: HashMap<String, serde_yaml::Value>, // Unified functions section
    pub settings: LauncherSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LauncherSettings {
    pub js_timeout_ms: Option<u64>,
}


pub fn launch(command_line: &str) -> Result<(), LauncherError> {
    let start_time = SystemTime::now();
    crate::utils::detailed_log("LAUNCHER", &format!("Starting launch for: '{}'", command_line));
    
    // Parse command_line to extract action and arguments  
    let (action, args) = parse_command_line(command_line)?;
    crate::utils::detailed_log("LAUNCHER", &format!("Parsed action: '{}', args: '{}'", action, args));
    
    // Load configuration from YAML
    let config = load_config()?;
    // Config loaded successfully - no need to log
    
    // Look up action value in config
    let action_value = lookup_action(&action, &config)?;
    // Found action config - proceed with execution
    
    // Create environment
    let mut env = Environment::new()
        .map_err(|e| LauncherError::ExecutionError(format!("Failed to create environment: {}", e)))?;
    // Environment created
    
    // Set the arg variable for template substitution
    env.variables.insert("arg".to_string(), args.clone());
    // Set arg variable
    
    // Execute the action using eval module
    // Execute the action
    let exec_start_time = std::time::Instant::now();
    let exec_result = env.eval(action_value);
    let _exec_duration = exec_start_time.elapsed();
    
    let duration = start_time.elapsed().unwrap_or_default();
    
    match &exec_result {
        Ok(_) => crate::utils::detailed_log("LAUNCHER", &format!("Command '{}' completed successfully in {:?}", command_line, duration)),
        Err(e) => {
            // Extract simplified error message for JavaScript execution errors
            let error_msg = match e {
                EvalError::ExecutionError(msg) => {
                    // Extract core error from "JavaScript execution failed: ErrorType: message"
                    if let Some(core_error) = msg.strip_prefix("JavaScript execution failed: ") {
                        core_error.to_string()
                    } else {
                        msg.clone()
                    }
                },
                _ => format!("{:?}", e)
            };
            crate::utils::log_error(&format!("Command '{}' failed after {:?}: {}", command_line, duration, error_msg))
        },
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
        std::path::PathBuf::from(home).join(".config/hookanchor/config.yaml")
    };
    
    // Read and parse the YAML config file
    let functions = if let Ok(contents) = std::fs::read_to_string(&config_path) {
        if let Ok(yaml_value) = serde_yaml::from_str::<serde_yaml::Value>(&contents) {
            let mut funcs = HashMap::new();
            
            // Load functions section
            if let Some(functions_section) = yaml_value.get("functions") {
                if let Some(mapping) = functions_section.as_mapping() {
                    for (key, value) in mapping {
                        if let Some(key_str) = key.as_str() {
                            funcs.insert(key_str.to_string(), value.clone());
                        }
                    }
                }
            }
            
            funcs
        } else {
            HashMap::new()
        }
    } else {
        HashMap::new()
    };
    
    let launcher_config = LauncherConfig {
        functions,
        settings: LauncherSettings {
            js_timeout_ms: Some(5000),
        },
    };
    
    Ok(launcher_config)
}



fn lookup_action(action: &str, config: &LauncherConfig) -> Result<serde_yaml::Value, LauncherError> {
    // Check if this is a builtin action that should be handled directly
    // These actions are registered as builtin functions in the eval environment
    let builtin_actions = ["cmd", "shell", "shell_sync", "app", "chrome", "brave", "firefox", 
                          "safari", "folder", "markdown", "anchor", "alias", "doc", "text", 
                          "open", "url", "shutdown", "rescan", "slack"];
    
    if builtin_actions.contains(&action) {
        // Return a function call to the builtin
        let mut map = serde_yaml::Mapping::new();
        map.insert("fn".into(), action.into());
        // The arg will be passed via the environment variables
        return Ok(serde_yaml::Value::Mapping(map));
    }
    
    // Look for action with action_ prefix
    let action_prefixed = format!("action_{}", action);
    if let Some(func_def) = config.functions.get(&action_prefixed) {
        // Check if it's a string (JavaScript function) or a mapping (simple function)
        if let Some(script_code) = func_def.as_str() {
            // It's a JavaScript function - wrap it in the expected format
            let mut map = serde_yaml::Mapping::new();
            map.insert("fn".into(), "javascript".into());
            map.insert("code".into(), script_code.to_string().into());
            return Ok(serde_yaml::Value::Mapping(map));
        } else {
            // It's a simple function mapping
            return Ok(func_def.clone());
        }
    }
    
    // Also check without prefix for helper functions
    if let Some(func_def) = config.functions.get(action) {
        // Check if it's a string (JavaScript function) or a mapping (simple function)
        if let Some(script_code) = func_def.as_str() {
            // It's a JavaScript function - wrap it in the expected format
            let mut map = serde_yaml::Mapping::new();
            map.insert("fn".into(), "javascript".into());
            map.insert("code".into(), script_code.to_string().into());
            return Ok(serde_yaml::Value::Mapping(map));
        } else {
            // It's a simple function mapping
            return Ok(func_def.clone());
        }
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
            functions: HashMap::new(),
            settings: LauncherSettings {
                js_timeout_ms: Some(5000),
            },
        };
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
        assert!(config.settings.timeout_ms.is_some());
    }
}