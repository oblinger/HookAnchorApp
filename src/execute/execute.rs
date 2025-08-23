//! Main execution interface for the execute module
//! 
//! This module provides the public-facing execution functions that route
//! actions to the appropriate handlers (local, server, or UI).

use std::collections::HashMap;
use serde_json::Value as JsonValue;
use super::Action;
use crate::core::Command;
use crate::utils::debug_log;

/// Execute an action on the server
/// This function always sends the action to the command server for execution
/// Returns either an ACK (for non-blocking) or actual execution results (for G flag/GUI commands)
pub fn execute_on_server(action: &Action) -> Result<String, Box<dyn std::error::Error>> {
    debug_log("EXECUTE", &format!("Sending action to server: {}", action.action_type()));
    
    // Send to server and get response (ACK or full result)
    let response = super::execution_server::send_for_execution(action)?;
    
    // Return server response (either ACK or execution output)
    Ok(response.stdout)
}

/// Execute an action on the server with additional parameters
/// This clones the action, merges in the arg and variables, then sends to server
pub fn execute_on_server_with_parameters(
    action: &Action,
    arg: Option<&str>,
    variables: Option<HashMap<String, JsonValue>>
) -> Result<String, Box<dyn std::error::Error>> {
    // Clone the action so we can modify it
    let mut modified_action = action.clone();
    
    // Merge in the arg parameter if provided
    if let Some(arg_value) = arg {
        modified_action.params.insert("arg".to_string(), JsonValue::String(arg_value.to_string()));
    }
    
    // Merge in any additional variables
    if let Some(vars) = variables {
        for (key, value) in vars {
            modified_action.params.insert(key, value);
        }
    }
    
    // Now execute on server with the modified action
    execute_on_server(&modified_action)
}



/// Convert a Command to an Action
/// This is the bridge between the old Command system and the new Action system
/// It resolves action types through the config so that actions with a 'type' field
/// get properly routed to their handler (e.g., anchor -> markdown)
pub fn command_to_action(cmd: &Command) -> Action {
    // Log the command being converted
    debug_log("COMMAND_TO_ACTION", &format!("Converting command '{}' with action '{}'", cmd.command, cmd.action));
    
    let mut params = HashMap::new();
    
    // Determine the action type and inherit any config-defined properties
    if cmd.action.is_empty() {
        // If action is empty, try to infer from other fields
        let inferred_type = if cmd.is_path_based() {
            if cmd.arg.ends_with(".md") {
                "markdown"
            } else if std::path::Path::new(&cmd.arg).is_dir() {
                "folder"
            } else {
                "document"
            }
        } else {
            "app" // Default fallback
        };
        params.insert("action_type".to_string(), JsonValue::String(inferred_type.to_string()));
    } else {
        // Look up the action in config to resolve its type and inherit properties
        // Start with the resolved params from config (includes type, browser, etc.)
        params = resolve_action_from_config(&cmd.action);
    }
    
    // Now overlay the command's specific parameters (these take precedence)
    params.insert("arg".to_string(), JsonValue::String(cmd.arg.clone()));
    params.insert("patch".to_string(), JsonValue::String(cmd.patch.clone()));
    params.insert("command_name".to_string(), JsonValue::String(cmd.command.clone()));
    params.insert("flags".to_string(), JsonValue::String(cmd.flags.clone()));
    
    // Log the final action type and browser if present
    let final_type = params.get("action_type")
        .and_then(|v| v.as_str())
        .unwrap_or("unknown");
    let browser = params.get("browser")
        .and_then(|v| v.as_str())
        .unwrap_or("default");
    debug_log("COMMAND_TO_ACTION", &format!("Final action type: '{}', browser: '{}'", final_type, browser));
    
    Action { params }
}

/// Resolve an action name through the config to get its actual type and properties
/// For example, "anchor" might have type: "markdown" in the config
fn resolve_action_from_config(action_name: &str) -> HashMap<String, JsonValue> {
    let config = crate::core::sys_data::get_config();
    
    if let Some(actions) = &config.actions {
        if let Some(action_def) = actions.get(action_name) {
            // Found the action in config - use all its properties
            // The action_def is already an Action (HashMap), so we can copy its params
            let mut resolved_params = action_def.params.clone();
            
            // Check if we need to resolve the type
            let resolved_type = action_def.action_type();
            if resolved_type != action_name {
                debug_log("ACTION_RESOLVE", 
                    &format!("Resolved action '{}' to type '{}'", action_name, resolved_type));
            }
            
            // Ensure the action_type is set (might already be there as "action_type")
            if !resolved_params.contains_key("action_type") {
                resolved_params.insert("action_type".to_string(), JsonValue::String(resolved_type.to_string()));
            }
            
            return resolved_params;
        }
    }
    
    // Action not found in config - use the name as-is
    debug_log("ACTION_RESOLVE", 
        &format!("Action '{}' not found in config, using as-is", action_name));
    let mut params = HashMap::new();
    params.insert("action_type".to_string(), JsonValue::String(action_name.to_string()));
    params
}

/// Unified command server activation function
/// 
/// Ensures the command server is running, with optional restart capability.
/// This is the single public interface for all server management needs.
/// 
/// # Arguments
/// * `restart` - If true, stops any existing server and starts a fresh one
/// 
/// # Returns
/// * Ok(()) if server is running (either already running or successfully started)
/// * Err if server could not be started
/// 
/// # Examples
/// ```
/// // Ensure server is running (start if needed)
/// activate_command_server(false)?;
/// 
/// // Force restart the server
/// activate_command_server(true)?;
/// ```
pub fn activate_command_server(restart: bool) -> Result<(), Box<dyn std::error::Error>> {
    debug_log("SERVER_ACTIVATE", &format!("Activating command server (restart={})", restart));
    
    if restart {
        // Kill any existing server first
        debug_log("SERVER_ACTIVATE", "Restart requested - killing existing server");
        let _ = super::execution_server_management::kill_existing_server();
        // Reset the session check flag so we'll verify the new server
        super::execution_server_management::reset_server_check();
    }
    
    // Now ensure server is running (will start if needed)
    super::execution_server_management::start_server_if_needed()?;
    
    debug_log("SERVER_ACTIVATE", "Command server activation complete");
    Ok(())
}

/// Run the command server process
/// 
/// This function is used when the binary is launched as the server daemon.
/// It starts the server and keeps the process alive.
/// 
/// # Returns
/// This function never returns normally - it runs until the process is killed
pub fn run_command_server() -> Result<(), Box<dyn std::error::Error>> {
    debug_log("SERVER_RUN", "Starting command server daemon");
    
    // Start the persistent server and get its PID
    let server_pid = super::execution_server::start_persistent_server()?;
    
    // Save PID to state
    if let Err(e) = crate::core::state::save_server_pid(server_pid) {
        crate::utils::log_error(&format!("Could not save server PID: {}", e));
    }
    
    debug_log("SERVER_RUN", &format!("Command server daemon running with PID: {}", server_pid));
    
    // Keep the process alive - this is the daemon
    loop {
        std::thread::sleep(std::time::Duration::from_secs(3600)); // Sleep 1 hour at a time
    }
}
