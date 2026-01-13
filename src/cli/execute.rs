//! Execute Commands
//!
//! Functions for executing commands and actions from the CLI.

use std::collections::HashMap;
use crate::utils::logging::print;
use crate::prelude::*;
use crate::execute::{execute_on_server, make_action, command_to_action};

/// Execute a specific command by name (-r, --run_fn).
pub fn run_exec_command(args: &[String]) {
    if args.len() < 3 {
        print("Usage: ha -r, --run_fn <command>");
        std::process::exit(1);
    }

    let command = &args[2];

    // Visual separator for run function execution
    detailed_log("", "=================================================================");
    detailed_log("USER INPUT", &format!("RUN_FN: '{}'", command));

    print(&format!("Executing command function: {}", command));

    // Load system data to find the actual command
    let (sys_data, _) = crate::core::data::get_sys_data();

    // Find the command by name (case-insensitive)
    let matching_cmd = sys_data.commands.iter()
        .find(|cmd| cmd.command.to_lowercase() == command.to_lowercase());

    if let Some(cmd) = matching_cmd {
        // Execute the actual Command object
        // Convert command to action and execute
        let action = command_to_action(&cmd);
        let mut variables = HashMap::new();
        variables.insert("arg".to_string(), cmd.arg.clone());
        let _ = execute_on_server(&action, Some(variables));
        print("Command completed");
    } else {
        print(&format!("Command '{}' not found", command));
        std::process::exit(1);
    }
}

/// Execute the top matching command for a query (-x, --execute).
pub fn run_execute_top_match(args: &[String]) {
    if args.len() < 3 {
        print("Usage: ha -x, --execute <query>");
        std::process::exit(1);
    }

    let query = &args[2];

    // Visual separator for command execution
    detailed_log("", "=================================================================");
    detailed_log("USER INPUT", &format!("CLI: '{}'", query));

    // Client environment logging is now handled automatically in execute_command based on action type

    let (sys_data, _) = crate::core::data::get_sys_data();
    let config = crate::core::data::get_config();

    // First, find if there's an exact match (including aliases)
    let exact_match = sys_data.commands.iter()
        .find(|cmd| cmd.command.eq_ignore_ascii_case(query))
        .cloned();

    // Get display commands which may resolve aliases
    let (display_commands, _, _, _, _, _) = crate::core::get_new_display_commands(query, &sys_data.commands, &sys_data.patches, &config);
    let filtered = display_commands.into_iter().take(1).collect::<Vec<_>>();

    if filtered.is_empty() {
        print(&format!("No commands found matching: {}", query));
        std::process::exit(1);
    }

    // Use the exact match if we have one (preserves original alias), otherwise use filtered result
    let (top_command_obj, original_name) = if let Some(exact) = exact_match.as_ref() {
        // We have an exact match - use it to preserve the original alias name
        (exact.clone(), exact.command.clone())
    } else {
        // No exact match, use the filtered result
        (filtered[0].clone(), filtered[0].command.clone())
    };

    print(&format!("Executing top match: {}", top_command_obj.command));

    // Save the last executed command for add_alias functionality
    detailed_log("STATE_SAVE", &format!("CLI: Attempting to save last executed command: '{}'", original_name));
    let mut state = crate::core::data::get_state();
    state.last_executed_command = Some(original_name.clone());
    match crate::core::data::set_state(&state) {
        Ok(_) => detailed_log("STATE_SAVE", "CLI: Successfully saved last executed command"),
        Err(e) => detailed_log("STATE_SAVE", &format!("CLI: Failed to save last executed command: {}", e)),
    }


    // Use server-based execution for consistent environment
    // Convert command to action and execute with parameters
    let action = command_to_action(&top_command_obj);
    let mut variables = std::collections::HashMap::new();
    variables.insert("arg".to_string(), top_command_obj.arg.clone());

    // Execute and check for errors
    match execute_on_server(&action, Some(variables)) {
        Ok(_) => {
            print("Command completed");
        }
        Err(e) => {
            print_and_log(&format!("❌ Command execution failed: {}", e));
            print_and_log("💡 The command server may not be running.");
            print_and_log("   Try running: ha --restart");
            std::process::exit(1);
        }
    }
}

/// Test a command with action and arg (-c, --command).
pub fn run_test_command(args: &[String]) {
    if args.len() < 3 {
        print("Usage: ha -c, --command <action_name> [--arg <value>] [--input <value>] [--param key=value]...");
        print("Examples:");
        print("  ha -c open_url --arg https://github.com");
        print("  ha -c template --input \"My Note\" --param action=markdown");
        print("  ha -c popup --param popup_action=navigate --param dx=0 --param dy=1");
        std::process::exit(1);
    }

    let action_name = &args[2];

    // Parse additional parameters
    let mut arg_value = String::new();
    let mut input_value = String::new();
    let mut extra_params = std::collections::HashMap::new();

    let mut i = 3;
    while i < args.len() {
        match args[i].as_str() {
            "--arg" => {
                if i + 1 < args.len() {
                    arg_value = args[i + 1].clone();
                    i += 2;
                } else {
                    print("--arg requires a value");
                    std::process::exit(1);
                }
            }
            "--input" => {
                if i + 1 < args.len() {
                    input_value = args[i + 1].clone();
                    i += 2;
                } else {
                    print("--input requires a value");
                    std::process::exit(1);
                }
            }
            "--param" => {
                if i + 1 < args.len() {
                    // Parse key=value
                    if let Some((key, value)) = args[i + 1].split_once('=') {
                        extra_params.insert(key.to_string(), serde_json::Value::String(value.to_string()));
                    } else {
                        print("--param requires key=value format");
                        std::process::exit(1);
                    }
                    i += 2;
                } else {
                    print("--param requires a value");
                    std::process::exit(1);
                }
            }
            _ => {
                // For backward compatibility, treat as arg if no flag
                if i == 3 && !args[i].starts_with("--") {
                    arg_value = args[i].clone();
                }
                i += 1;
            }
        }
    }

    // Visual separator for action testing
    detailed_log("", "=================================================================");
    detailed_log("USER INPUT", &format!("ACTION: '{}' ARG: '{}' INPUT: '{}'", action_name, arg_value, input_value));

    // Try to execute as a unified action first
    let config = crate::core::data::get_config();
    if let Some(actions) = &config.actions {
        if let Some(action) = actions.get(action_name) {
            print(&format!("Testing unified action '{}' (type: {})", action_name, action.action_type()));

            // Prepare variables for action execution (as JsonValue)
            let mut variables = HashMap::new();
            if !input_value.is_empty() {
                variables.insert("input".to_string(), serde_json::Value::String(input_value));
            }

            // Add extra parameters to variables (already JsonValue)
            for (key, value) in extra_params {
                variables.insert(key.clone(), value.clone());
            }

            // Execute the action with simplified parameters - convert variables to String HashMap
            let mut string_variables = std::collections::HashMap::new();

            // Add arg if present
            if !arg_value.is_empty() {
                string_variables.insert("arg".to_string(), arg_value.clone());
            }

            // Convert JsonValue variables to String variables
            for (key, value) in variables {
                if let Some(string_value) = value.as_str() {
                    string_variables.insert(key, string_value.to_string());
                }
            }

            match execute_on_server(action, if string_variables.is_empty() { None } else { Some(string_variables) }) {
                Ok(result) => {
                    print(&format!("Action completed successfully: {}", result));
                }
                Err(e) => {
                    print(&format!("Action failed: {}", e));
                    std::process::exit(1);
                }
            }
            return;
        }
    }

    // Fall back to legacy action testing (for backward compatibility)
    let command_line = format!("{} {}", action_name, arg_value);
    print(&format!("Testing legacy action '{}' with arg '{}': {}", action_name, arg_value, command_line));

    // Use server-based execution for testing actions
    // Create action directly and execute
    let action = make_action(&action_name, &arg_value);
    let mut variables = HashMap::new();
    variables.insert("arg".to_string(), arg_value.clone());
    let _ = execute_on_server(&action, Some(variables));
    print("Action completed");
}

/// Execute an action directly by type (-a, --action).
pub fn run_action_directly(args: &[String]) {
    if args.len() < 3 {
        print("Usage: ha -a, --action <action_type> [--key value]...");
        print("Examples:");
        print("  ha -a markdown --arg /path/to/file.md");
        print("  ha -a cmd --arg \"ls -la\" --flags W");
        std::process::exit(1);
    }

    let action_type = &args[2];

    // Parse all --key value pairs
    let mut params = std::collections::HashMap::new();
    let mut i = 3;
    while i < args.len() {
        if args[i].starts_with("--") {
            let key = args[i].trim_start_matches("--");
            if i + 1 < args.len() {
                params.insert(key.to_string(), args[i + 1].clone());
                i += 2;
            } else {
                print(&format!("Error: {} requires a value", args[i]));
                std::process::exit(1);
            }
        } else {
            print(&format!("Error: Expected --key but got: {}", args[i]));
            std::process::exit(1);
        }
    }

    // Build the action structure based on type and parameters
    use crate::execute::Action;
    use serde_json::Value as JsonValue;

    print(&format!("Executing action '{}' with parameters:", action_type));
    for (key, value) in &params {
        print(&format!("  {}: {}", key, value));
    }

    // Create the action as a HashMap with JsonValue entries
    let mut action_params = std::collections::HashMap::new();
    action_params.insert("action_type".to_string(), JsonValue::String(action_type.to_string()));

    // Add all parameters from command line
    for (key, value) in params {
        action_params.insert(key, JsonValue::String(value));
    }

    let action = Action { params: action_params };

    // Execute through server - convert JsonValue params to String variables
    let mut variables = HashMap::new();

    // Extract all string parameters for template expansion
    for (key, value) in &action.params {
        if let Some(string_val) = value.as_str() {
            variables.insert(key.clone(), string_val.to_string());
        }
    }

    let _ = execute_on_server(&action, if variables.is_empty() { None } else { Some(variables) });
    print(&format!("Action '{}' completed", action_type));
}
