//! Main execution interface for the execute module
//! 
//! This module provides the public-facing execution functions that route
//! actions to the appropriate handlers (local, server, or UI).

use std::collections::HashMap;
use serde_json::Value as JsonValue;
use super::Action;
use crate::core::Command;
use crate::prelude::*;

/// Execute an action on the server with template expansion using provided variables
/// This function performs template expansion before sending to the command server
/// Returns either an ACK (for non-blocking) or actual execution results (for G flag/GUI commands)
pub fn execute_on_server(
    action: &Action,
    variables: Option<HashMap<String, String>>
) -> Result<String, Box<dyn std::error::Error>> {
    detailed_log("EXECUTE", &format!("CLI execution with variables - expanding action: {}", action.action_type()));

    // Clone action and perform CLI template expansion
    let mut expanded_action = action.clone();

    // Create template context for CLI (no popup context, empty input)
    let mut template_context = crate::core::template_creation::TemplateContext::create_basic_template("");

    // Add variables to template context for expansion
    if let Some(vars) = &variables {
        for (key, value) in vars {
            template_context.add_variable(key.clone(), value.clone());
            detailed_log("EXECUTE", &format!("Added variable to template context: '{}' = '{}'", key, value));
        }
    }

    // Expand all action parameters using the template context
    template_context.expand_action_parameters(&mut expanded_action);

    detailed_log("EXECUTE", &format!("Sending expanded action to server: {}", expanded_action.action_type()));

    // Send to server and get response (ACK or full result)
    let response = super::execution_server::send_for_execution(&expanded_action)?;

    // Return server response (either ACK or execution output)
    Ok(response.stdout)
}




/// Convert a Command to an Action
pub fn command_to_action(cmd: &Command) -> Action {
    // Log the command being converted
    detailed_log("COMMAND_TO_ACTION", &format!("Converting command '{}' with action '{}'", cmd.command, cmd.action));
    
    let mut params = HashMap::new();

    // Determine the action type and inherit any config-defined properties
    if cmd.action.is_empty() {
        // Check if this is a virtual anchor (blank action + blank arg) - these are non-executable
        if cmd.arg.is_empty() {
            detailed_log("COMMAND_TO_ACTION", "Virtual anchor (blank action + arg) - not executable");
            params.insert("action_type".to_string(), JsonValue::String("noop".to_string()));
            params.insert("arg".to_string(), JsonValue::String(String::new()));
            params.insert("patch".to_string(), JsonValue::String(cmd.patch.clone()));
            params.insert("command_name".to_string(), JsonValue::String(cmd.command.clone()));
            params.insert("flags".to_string(), JsonValue::String(cmd.flags.clone()));
            return Action { params };
        }

        // If action is empty but arg exists, try to infer from other fields
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
    detailed_log("COMMAND_TO_ACTION", &format!("Final action type: '{}', browser: '{}'", final_type, browser));
    
    Action { params }
}

/// Resolve an action name through the config to get its actual type and properties
/// For example, "anchor" might have type: "markdown" in the config
fn resolve_action_from_config(action_name: &str) -> HashMap<String, JsonValue> {
    let config = crate::core::data::get_config();
    
    if let Some(actions) = &config.actions {
        if let Some(action_def) = actions.get(action_name) {
            // Found the action in config - use all its properties
            // The action_def is already an Action (HashMap), so we can copy its params
            let mut resolved_params = action_def.params.clone();
            
            // Check if we need to resolve the type
            let resolved_type = action_def.action_type();
            if resolved_type != action_name {
                detailed_log("ACTION_RESOLVE", 
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
    detailed_log("ACTION_RESOLVE", 
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
    detailed_log("SERVER_ACTIVATE", &format!("Activating command server (restart={})", restart));
    
    if restart {
        // Kill any existing server first
        detailed_log("SERVER_ACTIVATE", "Restart requested - killing existing server");
        let _ = super::execution_server_management::kill_existing_server();
        // Reset the session check flag so we'll verify the new server
        super::execution_server_management::reset_server_check();
    }
    
    // Now ensure server is running (will start if needed)
    super::execution_server_management::start_server_if_needed()?;
    
    detailed_log("SERVER_ACTIVATE", "Command server activation complete");
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
    detailed_log("SERVER_RUN", "Starting command server daemon");
    
    // Start the persistent server and get its PID
    let server_pid = super::execution_server::start_persistent_server()?;

    // Save PID to state
    let mut state = crate::core::data::get_state();
    state.server_pid = Some(server_pid);
    if let Err(e) = crate::core::data::set_state(&state) {
        log_error(&format!("Could not save server PID: {}", e));
    }
    
    detailed_log("SERVER_RUN", &format!("Command server daemon running with PID: {}", server_pid));
    
    // Keep the process alive - this is the daemon
    loop {
        std::thread::sleep(std::time::Duration::from_secs(3600)); // Sleep 1 hour at a time
    }
}
