use std::collections::HashMap;
use std::path::PathBuf;
use rquickjs::{Context, Runtime};
use serde_yaml;
use crate::utils::debug_log;

#[derive(Debug, Clone, PartialEq)]
pub enum EvalError {
    ExecutionError(String),
    InvalidAction(String),
    SystemError(String),
}

impl From<rquickjs::Error> for EvalError {
    fn from(err: rquickjs::Error) -> Self {
        EvalError::SystemError(format!("JS Error: {}", err))
    }
}

impl From<serde_yaml::Error> for EvalError {
    fn from(err: serde_yaml::Error) -> Self {
        EvalError::SystemError(format!("YAML Error: {}", err))
    }
}

// ActionSpec enum removed - now using generic serde_yaml::Value evaluation


pub struct Environment {
    pub config_path: PathBuf,
    pub working_dir: PathBuf,
    pub variables: HashMap<String, String>,
    // JavaScript runtime - created once and reused
    #[allow(dead_code)]
    js_runtime: Runtime,
    pub js_context: Context,
    // Function registry for built-in functions
    pub functions: HashMap<String, Box<dyn Fn(&mut Environment, &HashMap<String, serde_yaml::Value>) -> Result<(), EvalError> + Send + Sync>>,
}

impl Environment {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let js_runtime = Runtime::new()?;
        let js_context = Context::full(&js_runtime)?;
        
        let mut env = Self {
            config_path: PathBuf::from("config.yaml"),
            working_dir: std::env::current_dir()?,
            variables: HashMap::new(),
            js_runtime,
            js_context,
            functions: HashMap::new(),
        };
        
        // Setup built-in function registry
        env.setup_builtin_functions();
        
        Ok(env)
    }
    
    /// Unified eval function - handles both YAML values and JS values uniformly
    /// This is the core of the Lisp-like evaluation system
    pub fn eval(&mut self, expr: serde_yaml::Value) -> Result<serde_yaml::Value, EvalError> {
        match expr {
            // If it's a mapping (object) with a 'fn' key, it's a function call
            serde_yaml::Value::Mapping(ref map) => {
                if let Some(fn_name_value) = map.get(&serde_yaml::Value::String("fn".to_string())) {
                    if let serde_yaml::Value::String(fn_name) = fn_name_value {
                        // Extract arguments into a HashMap (raw, no substitution)
                        let mut args = HashMap::new();
                        for (key, value) in map {
                            if let serde_yaml::Value::String(key_str) = key {
                                if key_str != "fn" {
                                    args.insert(key_str.clone(), value.clone());
                                }
                            }
                        }
                        
                        // Dispatch to function with raw arguments
                        if self.functions.contains_key(fn_name) {
                            // Extract the function and call it
                            let func_key = fn_name.clone();
                            if let Some(func) = self.functions.remove(&func_key) {
                                let result = func(self, &args);
                                // Restore the function
                                self.functions.insert(func_key, func);
                                debug_log("EVAL", &format!("Executed function: {}", fn_name));
                                result?;
                            }
                            Ok(serde_yaml::Value::Null) // Functions return null for now
                        } else {
                            Err(EvalError::InvalidAction(format!("Unknown function: {}", fn_name)))
                        }
                    } else {
                        Err(EvalError::InvalidAction("Function name must be a string".to_string()))
                    }
                } else {
                    // Object without 'fn' key - return as-is
                    Ok(expr)
                }
            },
            // Primitives evaluate to themselves
            _ => Ok(expr)
        }
    }
    
    /// Setup built-in function registry
    fn setup_builtin_functions(&mut self) {
        crate::builtin_fns::setup_builtin_functions(self);
    }
    
}


/// Helper function for template substitution (public for tests and external use)
pub fn substitute_template_in_args(args: &HashMap<String, serde_yaml::Value>, env: &Environment) -> HashMap<String, serde_yaml::Value> {
    let mut result = HashMap::new();
    for (key, value) in args {
        let substituted = substitute_template_in_value(value, env);
        result.insert(key.clone(), substituted);
    }
    result
}

/// Helper function to substitute templates in a single value (public for external use)
pub fn substitute_template_in_value(value: &serde_yaml::Value, env: &Environment) -> serde_yaml::Value {
    match value {
        serde_yaml::Value::String(s) => {
            let mut result = s.clone();
            for (key, replacement) in &env.variables {
                let pattern = format!("{{{{{}}}}}", key);
                result = result.replace(&pattern, replacement);
            }
            serde_yaml::Value::String(result)
        },
        _ => value.clone()
    }
}

// Legacy JavaScript execution function removed - now handled by javascript function in registry

// Legacy JavaScript helper functions removed - now handled by js_runtime module

// debug_log function moved to utils module

// expand_tilde function moved to utils module

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eval_launch_app() {
        let mut env = Environment::new().unwrap();
        let mut map = serde_yaml::Mapping::new();
        map.insert("fn".into(), "launch_app".into());
        map.insert("name".into(), "Finder".into());
        let action = serde_yaml::Value::Mapping(map);
        let result = env.eval(action);
        assert!(result.is_ok());
    }

    #[test]
    fn test_eval_open_with_browser() {
        let mut env = Environment::new().unwrap();
        env.variables.insert("arg".to_string(), "https://github.com".to_string());
        let mut map = serde_yaml::Mapping::new();
        map.insert("fn".into(), "open_with".into());
        map.insert("app".into(), "Google Chrome".into());
        map.insert("arg".into(), "{{arg}}".into());
        let action = serde_yaml::Value::Mapping(map);
        let result = env.eval(action);
        assert!(result.is_ok());
    }

    #[test]
    fn test_eval_url_action() {
        let mut env = Environment::new().unwrap();
        let mut map = serde_yaml::Mapping::new();
        map.insert("fn".into(), "open_url".into());
        map.insert("url".into(), "https://github.com".into());
        let action = serde_yaml::Value::Mapping(map);
        let result = env.eval(action);
        assert!(result.is_ok());
    }

    #[test]
    fn test_eval_folder_action() {
        let mut env = Environment::new().unwrap();
        let mut map = serde_yaml::Mapping::new();
        map.insert("fn".into(), "open_folder".into());
        map.insert("path".into(), "/Users".into());
        let action = serde_yaml::Value::Mapping(map);
        let result = env.eval(action);
        assert!(result.is_ok());
    }

    #[test]
    fn test_eval_shell_action() {
        let mut env = Environment::new().unwrap();
        let mut map = serde_yaml::Mapping::new();
        map.insert("fn".into(), "shell".into());
        map.insert("command".into(), "echo hello".into());
        let action = serde_yaml::Value::Mapping(map);
        let result = env.eval(action);
        assert!(result.is_ok());
    }

    #[test]
    fn test_eval_javascript_action() {
        let mut env = Environment::new().unwrap();
        let mut map = serde_yaml::Mapping::new();
        map.insert("fn".into(), "javascript".into());
        map.insert("code".into(), "log('Hello from JavaScript');".into());
        let action = serde_yaml::Value::Mapping(map);
        let result = env.eval(action);
        assert!(result.is_ok(), "JavaScript action should execute successfully: {:?}", result);
    }

    #[test]
    fn test_environment_creation() {
        let env = Environment::new().unwrap();
        assert_eq!(env.config_path, PathBuf::from("config.yaml"));
        assert!(env.variables.is_empty());
    }

    #[test]
    fn test_template_substitution() {
        let mut env = Environment::new().unwrap();
        env.variables.insert("arg".to_string(), "TestValue".to_string());
        
        let args = {
            let mut map = std::collections::HashMap::new();
            map.insert("name".to_string(), serde_yaml::Value::String("{{arg}}".to_string()));
            map
        };
        
        let substituted = substitute_template_in_args(&args, &env);
        if let Some(serde_yaml::Value::String(name)) = substituted.get("name") {
            assert_eq!(name, "TestValue");
        } else {
            panic!("Expected substituted name to be 'TestValue'");
        }
    }

    #[test]
    fn test_function_mapping_structure() {
        let mut map = serde_yaml::Mapping::new();
        map.insert("fn".into(), "launch_app".into());
        map.insert("name".into(), "Finder".into());
        let app_value = serde_yaml::Value::Mapping(map);
        
        if let serde_yaml::Value::Mapping(map) = app_value {
            assert_eq!(map.get("fn"), Some(&serde_yaml::Value::String("launch_app".to_string())));
            assert_eq!(map.get("name"), Some(&serde_yaml::Value::String("Finder".to_string())));
        } else {
            panic!("Expected mapping for app command");
        }
    }
}