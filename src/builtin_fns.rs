//! Built-in Functions for the Evaluation System
//!
//! This module contains all the built-in functions that can be called from the evaluator.
//! Each function handles its own argument processing and template substitution using
//! helper functions.

use std::collections::HashMap;
use crate::eval::{EvalError, Environment};
use crate::utils::{detailed_log, launch_app_with_arg, open_url, open_folder, open_with_app};

/// Setup all built-in functions in the environment's function registry
pub fn setup_builtin_functions(env: &mut Environment) {
    // launch_app function (handles both app launching and open_with)
    env.functions.insert("launch_app".to_string(), Box::new(|env, args| {
        let name = get_substituted_string_arg(args, "name", env)
            .or_else(|| get_substituted_string_arg(args, "app", env))
            .ok_or_else(|| EvalError::InvalidAction("Missing 'name' or 'app' argument".to_string()))?;
        let arg = get_substituted_string_arg(args, "arg", env);
        
        detailed_log("BUILTIN", &format!("Launching app: {} with arg: {:?}", name, arg));
        
        let launch_start_time = std::time::Instant::now();
        detailed_log("BUILTIN", "About to call launch_app_with_arg (non-blocking)");
        let output_result = launch_app_with_arg(&name, arg.as_deref());
        let launch_duration = launch_start_time.elapsed();
        detailed_log("BUILTIN", &format!("launch_app_with_arg completed in {:?}", launch_duration));
        
        match output_result {
            Ok(output) => {
                // Should not happen with current non-blocking implementation
                if !output.status.success() {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    return Err(EvalError::ExecutionError(format!("App launch failed: {}", stderr)));
                }
            }
            Err(e) => {
                // Check if this is our "success error" from non-blocking execution
                if e.to_string().contains("NON_BLOCKING_SUCCESS") {
                    detailed_log("BUILTIN", "Non-blocking launch successful");
                } else {
                    return Err(EvalError::ExecutionError(format!("Failed to launch app '{}': {}", name, e)));
                }
            }
        }
        
        Ok(())
    }));
    
    // open_with function  
    env.functions.insert("open_with".to_string(), Box::new(|env, args| {
        let app = get_substituted_string_arg(args, "app", env).unwrap_or_default();
        let arg = get_substituted_string_arg(args, "arg", env)
            .ok_or_else(|| EvalError::InvalidAction("Missing 'arg' argument".to_string()))?;
        
        detailed_log("BUILTIN", &format!("Opening '{}' with app: '{}'", arg, app));
        
        let open_start_time = std::time::Instant::now();
        detailed_log("BUILTIN", "About to call open_with_app (non-blocking)");
        let output_result = open_with_app(&app, &arg);
        let open_duration = open_start_time.elapsed();
        detailed_log("BUILTIN", &format!("open_with_app completed in {:?}", open_duration));
        
        match output_result {
            Ok(output) => {
                if !output.status.success() {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    return Err(EvalError::ExecutionError(format!("Open failed: {}", stderr)));
                }
            }
            Err(e) => {
                if e.to_string().contains("NON_BLOCKING_SUCCESS") {
                    detailed_log("BUILTIN", "Non-blocking open successful");
                } else {
                    return Err(EvalError::ExecutionError(format!("Failed to open '{}': {}", arg, e)));
                }
            }
        }
        
        Ok(())
    }));
    
    // open_url function
    env.functions.insert("open_url".to_string(), Box::new(|env, args| {
        let url = get_substituted_string_arg(args, "url", env)
            .ok_or_else(|| EvalError::InvalidAction("Missing 'url' argument".to_string()))?;
        
        detailed_log("BUILTIN", &format!("Opening URL: {}", url));
        
        let url_start_time = std::time::Instant::now();
        detailed_log("BUILTIN", "About to call open_url (non-blocking)");
        let output_result = open_url(&url);
        let url_duration = url_start_time.elapsed();
        detailed_log("BUILTIN", &format!("open_url completed in {:?}", url_duration));
        
        match output_result {
            Ok(output) => {
                if !output.status.success() {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    return Err(EvalError::ExecutionError(format!("URL open failed: {}", stderr)));
                }
            }
            Err(e) => {
                if e.to_string().contains("NON_BLOCKING_SUCCESS") {
                    detailed_log("BUILTIN", "Non-blocking URL open successful");
                } else {
                    return Err(EvalError::ExecutionError(format!("Failed to open URL '{}': {}", url, e)));
                }
            }
        }
        
        Ok(())
    }));
    
    // open_folder function
    env.functions.insert("open_folder".to_string(), Box::new(|env, args| {
        let path = get_substituted_string_arg(args, "path", env)
            .ok_or_else(|| EvalError::InvalidAction("Missing 'path' argument".to_string()))?;
        
        detailed_log("BUILTIN", &format!("Opening folder: {}", path));
        
        let folder_start_time = std::time::Instant::now();
        detailed_log("BUILTIN", "About to call open_folder (non-blocking)");
        let output_result = open_folder(&path);
        let folder_duration = folder_start_time.elapsed();
        detailed_log("BUILTIN", &format!("open_folder completed in {:?}", folder_duration));
        
        match output_result {
            Ok(output) => {
                if !output.status.success() {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    return Err(EvalError::ExecutionError(format!("Folder open failed: {}", stderr)));
                }
            }
            Err(e) => {
                if e.to_string().contains("NON_BLOCKING_SUCCESS") {
                    detailed_log("BUILTIN", "Non-blocking folder open successful");
                } else {
                    return Err(EvalError::ExecutionError(format!("Failed to open folder '{}': {}", path, e)));
                }
            }
        }
        
        Ok(())
    }));
    
    // cmd function - alias for shell, uses command server for consistent environment
    // This handles the "cmd" action from commands.txt
    env.functions.insert("cmd".to_string(), Box::new(|env, args| {
        // The argument is the shell command to execute
        let command = env.variables.get("arg")
            .cloned()
            .or_else(|| get_substituted_string_arg(args, "command", env))
            .or_else(|| get_substituted_string_arg(args, "cmd", env))
            .ok_or_else(|| EvalError::InvalidAction("Missing command argument".to_string()))?;
        
        crate::utils::verbose_log("BUILTIN", &format!("CMD: Executing shell command via server: {}", command));
        
        // Use command server for consistent environment
        match crate::command_server::execute_via_server(&command, None, None, true) {
            Ok(response) => {
                if !response.success {
                    if let Some(error) = response.error {
                        return Err(EvalError::ExecutionError(format!("CMD command failed: {}", error)));
                    } else if !response.stderr.is_empty() {
                        return Err(EvalError::ExecutionError(format!("CMD command failed: {}", response.stderr)));
                    }
                }
                Ok(())
            },
            Err(e) => {
                let error_msg = format!("CMD: Shell server unavailable for command '{}': {}", command, e);
                crate::utils::log_error(&error_msg);
                Err(EvalError::ExecutionError(error_msg))
            }
        }
    }));
    
    // shell function - uses command server for consistent environment
    env.functions.insert("shell".to_string(), Box::new(|env, args| {
        let command = get_substituted_string_arg(args, "command", env)
            .or_else(|| get_substituted_string_arg(args, "cmd", env))
            .ok_or_else(|| EvalError::InvalidAction("Missing 'command' or 'cmd' argument".to_string()))?;
        
        crate::utils::verbose_log("BUILTIN", &format!("Starting shell command via server (non-blocking): {}", command));
        
        // Use command server for consistent environment
        match crate::command_server::execute_via_server(&command, None, None, false) {
            Ok(response) => {
                // Command server already logs stdout/stderr in verbose mode, no need to duplicate
                
                if !response.success {
                    if let Some(error) = response.error {
                        return Err(EvalError::ExecutionError(format!("Shell command failed: {}", error)));
                    } else if !response.stderr.is_empty() {
                        return Err(EvalError::ExecutionError(format!("Shell command failed: {}", response.stderr)));
                    }
                }
                Ok(())
            },
            Err(e) => {
                let error_msg = format!("Shell server unavailable for command '{}': {}", command, e);
                crate::error_display::queue_user_error(&error_msg);
                Err(EvalError::ExecutionError(error_msg))
            }
        }
    }));
    
    // shell_simple function - basic shell execution
    env.functions.insert("shell_simple".to_string(), Box::new(|env, args| {
        let command = get_substituted_string_arg(args, "command", env)
            .or_else(|| get_substituted_string_arg(args, "cmd", env))
            .ok_or_else(|| EvalError::InvalidAction("Missing 'command' or 'cmd' argument".to_string()))?;
        
        detailed_log("BUILTIN", &format!("Starting shell command (simple, non-blocking): {}", command));
        
        let _output = crate::utils::shell_simple(&command, false)
            .map_err(|e| EvalError::ExecutionError(format!("Failed to start shell command '{}': {}", command, e)))?;
        
        Ok(())
    }));
    
    // shell_login function - login shell execution
    env.functions.insert("shell_login".to_string(), Box::new(|env, args| {
        let command = get_substituted_string_arg(args, "command", env)
            .or_else(|| get_substituted_string_arg(args, "cmd", env))
            .ok_or_else(|| EvalError::InvalidAction("Missing 'command' or 'cmd' argument".to_string()))?;
        
        detailed_log("BUILTIN", &format!("Starting shell command (login, non-blocking): {}", command));
        
        let _output = crate::utils::shell_login(&command, false)
            .map_err(|e| EvalError::ExecutionError(format!("Failed to start shell command '{}': {}", command, e)))?;
        
        Ok(())
    }));
    
    // shell_sync function - blocking execution via command server
    env.functions.insert("shell_sync".to_string(), Box::new(|env, args| {
        let command = get_substituted_string_arg(args, "command", env)
            .or_else(|| get_substituted_string_arg(args, "cmd", env))
            .ok_or_else(|| EvalError::InvalidAction("Missing 'command' or 'cmd' argument".to_string()))?;
        
        crate::utils::verbose_log("BUILTIN", &format!("Running shell command via server (blocking): {}", command));
        
        // Use command server for consistent environment
        match crate::command_server::execute_via_server(&command, None, None, true) {
            Ok(response) => {
                // Command server already logs stdout/stderr in verbose mode, no need to duplicate
                
                if !response.success {
                    if let Some(error) = response.error {
                        return Err(EvalError::ExecutionError(format!("Shell command failed: {}", error)));
                    } else if !response.stderr.is_empty() {
                        return Err(EvalError::ExecutionError(format!("Shell command failed: {}", response.stderr)));
                    }
                }
                Ok(())
            },
            Err(e) => {
                let error_msg = format!("Shell server unavailable for command '{}': {}", command, e);
                crate::error_display::queue_user_error(&error_msg);
                Err(EvalError::ExecutionError(error_msg))
            }
        }
    }));
    
    // javascript function for executing JS code
    env.functions.insert("javascript".to_string(), Box::new(|env, args| {
        let code = get_substituted_string_arg(args, "code", env)
            .ok_or_else(|| EvalError::InvalidAction("Missing 'code' argument".to_string()))?;
        
        // Enhanced debugging for JavaScript execution
        detailed_log("JS-DEBUG", "=== JavaScript Execution Debug ===");
        detailed_log("JS-DEBUG", &format!("Code to execute: {}", code));
        detailed_log("JS-DEBUG", &format!("Code length: {} chars", code.len()));
        
        // Check for common problematic patterns
        if code.contains("sub ") || code.contains("sub\"") {
            detailed_log("JS-DEBUG", "WARNING: Code contains 'sub' which might be a shell alias");
        }
        
        detailed_log("BUILTIN", "Executing JavaScript function");
        
        // Set up JavaScript context with variables
        env.js_context.with(|ctx| -> Result<(), EvalError> {
            for (key, value) in &env.variables {
                ctx.globals().set(key, value)?;
            }
            ctx.globals().set("WORKING_DIR", env.working_dir.to_str().unwrap_or(""))?;
            
            // Setup built-in functions from js_runtime module
            crate::js_runtime::setup_all_builtins(&ctx)
                .map_err(|e| EvalError::ExecutionError(format!("Failed to setup built-ins: {}", e)))?;
            
            // Load config and setup config access functions
            let config = crate::core::sys_data::get_config();
            crate::js_runtime::setup_config_access(&ctx, &config)
                .map_err(|e| EvalError::ExecutionError(format!("Failed to setup config access: {}", e)))?;
            
            // Execute the JavaScript code with better error reporting
            match ctx.eval::<rquickjs::Value, _>(code.as_str()) {
                Ok(_) => Ok(()),
                Err(rquickjs::Error::Exception) => {
                    // Try to get the exception details
                    let exception = ctx.catch();
                    
                    // Enhanced error logging
                    detailed_log("JS-ERROR", "=== JavaScript Exception Details ===");
                    detailed_log("JS-ERROR", &format!("Failed code: {}", code));
                    
                    let exception_info = if let Some(message_str) = exception.as_string() {
                        message_str.to_string().unwrap_or_else(|_| "Unknown error".to_string())
                    } else if let Some(obj) = exception.as_object() {
                        // Try to get error properties
                        let message = obj.get::<_, String>("message").unwrap_or_else(|_| "No message".to_string());
                        let name = obj.get::<_, String>("name").unwrap_or_else(|_| "Error".to_string());
                        let stack = obj.get::<_, String>("stack").unwrap_or_else(|_| "No stack trace".to_string());
                        
                        detailed_log("JS-ERROR", &format!("Error Type: {}", name));
                        detailed_log("JS-ERROR", &format!("Message: {}", message));
                        
                        // Try to identify the problematic part of the code
                        if message.contains("expecting ';'") {
                            detailed_log("JS-ERROR", "Syntax error: Missing semicolon or invalid JavaScript syntax");
                            // Try to find where the issue might be
                            if code.contains(" cd ") {
                                detailed_log("JS-ERROR", "Found 'cd' command - this is a shell command, not JavaScript!");
                            }
                            if code.contains(" && ") {
                                detailed_log("JS-ERROR", "Found '&&' operator - mixing shell and JavaScript syntax?");
                            }
                        }
                        
                        // Compact format: extract just the stack lines without full stack trace formatting
                        let stack_lines: Vec<&str> = stack.lines()
                            .skip(1) // Skip the error message line (first line)
                            .filter(|line| line.trim().starts_with("at "))
                            .map(|line| line.trim().trim_start_matches("at "))
                            .collect();
                        
                        if stack_lines.is_empty() {
                            format!("{}: {}", name, message)
                        } else {
                            format!("{}: {}. {}", name, message, stack_lines.join(" "))
                        }
                    } else {
                        format!("JavaScript exception: {:?}", exception)
                    };
                    
                    detailed_log("JS-ERROR", &exception_info);
                    Err(EvalError::ExecutionError(format!("JavaScript execution failed: {}", exception_info)))
                },
                Err(e) => {
                    let error_msg = format!("JavaScript execution failed: {}", e);
                    detailed_log("JS-ERROR", &error_msg);
                    Err(EvalError::ExecutionError(error_msg))
                }
            }
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