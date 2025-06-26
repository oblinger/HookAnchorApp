//! Built-in Functions for the Evaluation System
//!
//! This module contains all the built-in functions that can be called from the evaluator.
//! Each function handles its own argument processing and template substitution using
//! helper functions.

use std::collections::HashMap;
use crate::eval::{EvalError, Environment};
use crate::utils::{debug_log, launch_app_with_arg, open_url, open_folder, execute_shell_command, open_with_app};

/// Setup all built-in functions in the environment's function registry
pub fn setup_builtin_functions(env: &mut Environment) {
    // launch_app function (handles both app launching and open_with)
    env.functions.insert("launch_app".to_string(), Box::new(|env, args| {
        let name = get_substituted_string_arg(args, "name", env)
            .or_else(|| get_substituted_string_arg(args, "app", env))
            .ok_or_else(|| EvalError::InvalidAction("Missing 'name' or 'app' argument".to_string()))?;
        let arg = get_substituted_string_arg(args, "arg", env);
        
        debug_log("BUILTIN", &format!("Launching app: {} with arg: {:?}", name, arg));
        
        let output = launch_app_with_arg(&name, arg.as_deref())
            .map_err(|e| EvalError::ExecutionError(format!("Failed to launch app '{}': {}", name, e)))?;
        
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(EvalError::ExecutionError(format!("App launch failed: {}", stderr)));
        }
        
        Ok(())
    }));
    
    // open_with function  
    env.functions.insert("open_with".to_string(), Box::new(|env, args| {
        let app = get_substituted_string_arg(args, "app", env).unwrap_or_default();
        let arg = get_substituted_string_arg(args, "arg", env)
            .ok_or_else(|| EvalError::InvalidAction("Missing 'arg' argument".to_string()))?;
        
        debug_log("BUILTIN", &format!("Opening '{}' with app: '{}'", arg, app));
        
        let output = open_with_app(&app, &arg)
            .map_err(|e| EvalError::ExecutionError(format!("Failed to open '{}': {}", arg, e)))?;
        
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(EvalError::ExecutionError(format!("Open failed: {}", stderr)));
        }
        
        Ok(())
    }));
    
    // open_url function
    env.functions.insert("open_url".to_string(), Box::new(|env, args| {
        let url = get_substituted_string_arg(args, "url", env)
            .ok_or_else(|| EvalError::InvalidAction("Missing 'url' argument".to_string()))?;
        
        debug_log("BUILTIN", &format!("Opening URL: {}", url));
        
        let output = open_url(&url)
            .map_err(|e| EvalError::ExecutionError(format!("Failed to open URL '{}': {}", url, e)))?;
        
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(EvalError::ExecutionError(format!("URL open failed: {}", stderr)));
        }
        
        Ok(())
    }));
    
    // open_folder function
    env.functions.insert("open_folder".to_string(), Box::new(|env, args| {
        let path = get_substituted_string_arg(args, "path", env)
            .ok_or_else(|| EvalError::InvalidAction("Missing 'path' argument".to_string()))?;
        
        debug_log("BUILTIN", &format!("Opening folder: {}", path));
        
        let output = open_folder(&path)
            .map_err(|e| EvalError::ExecutionError(format!("Failed to open folder '{}': {}", path, e)))?;
        
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(EvalError::ExecutionError(format!("Folder open failed: {}", stderr)));
        }
        
        Ok(())
    }));
    
    // shell function
    env.functions.insert("shell".to_string(), Box::new(|env, args| {
        let command = get_substituted_string_arg(args, "command", env)
            .or_else(|| get_substituted_string_arg(args, "cmd", env))
            .ok_or_else(|| EvalError::InvalidAction("Missing 'command' or 'cmd' argument".to_string()))?;
        
        debug_log("BUILTIN", &format!("Running shell command: {}", command));
        
        let output = execute_shell_command(&command)
            .map_err(|e| EvalError::ExecutionError(format!("Failed to execute command '{}': {}", command, e)))?;
        
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(EvalError::ExecutionError(format!("Shell command failed: {}", stderr)));
        }
        
        Ok(())
    }));
    
    // javascript function for executing JS code
    env.functions.insert("javascript".to_string(), Box::new(|env, args| {
        let code = get_substituted_string_arg(args, "code", env)
            .ok_or_else(|| EvalError::InvalidAction("Missing 'code' argument".to_string()))?;
        
        debug_log("BUILTIN", &format!("Executing JavaScript: {}", code.trim()));
        
        // Set up JavaScript context with variables
        env.js_context.with(|ctx| -> Result<(), EvalError> {
            for (key, value) in &env.variables {
                ctx.globals().set(key, value)?;
            }
            ctx.globals().set("WORKING_DIR", env.working_dir.to_str().unwrap_or(""))?;
            
            // Setup built-in functions from js_runtime module
            crate::js_runtime::setup_all_builtins(&ctx)
                .map_err(|e| EvalError::ExecutionError(format!("Failed to setup built-ins: {}", e)))?;
            
            // Execute the JavaScript code
            ctx.eval::<(), _>(code.as_str())
                .map_err(|e| EvalError::ExecutionError(format!("JavaScript execution failed: {}", e)))?;
            
            Ok(())
        })?;
        
        Ok(())
    }));
}

/// Helper function to substitute templates in a single value
fn substitute_template_in_value(value: &serde_yaml::Value, env: &Environment) -> serde_yaml::Value {
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

/// Helper function to get a string argument with template substitution
fn get_substituted_string_arg(args: &HashMap<String, serde_yaml::Value>, key: &str, env: &Environment) -> Option<String> {
    args.get(key).and_then(|v| {
        let substituted = substitute_template_in_value(v, env);
        match substituted {
            serde_yaml::Value::String(s) => Some(s),
            _ => None
        }
    })
}

// debug_log function moved to utils module

// expand_tilde function moved to utils module