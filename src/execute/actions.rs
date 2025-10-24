//! Unified Actions System
//! 
//! This module implements the unified actions architecture where all behaviors
//! (templates, keyboard actions, command actions) are defined as typed actions.
//! Actions are typed by their behavior (what they do) not their invocation method.

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use crate::core::Command;
use crate::core::key_processing::Keystroke;
use crate::utils::detailed_log;

/// A unified action that can be invoked via keyboard, command, or other actions
/// This is now simply a HashMap with accessor methods for common fields
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct Action {
    /// All action data stored as JSON for maximum flexibility
    pub params: HashMap<String, JsonValue>,
}

impl Action {
    /// Get the action type (looks for "action_type" key in params)
    pub fn action_type(&self) -> &str {
        self.params.get("action_type")
            .and_then(|v| v.as_str())
            .unwrap_or("app")  // Default to "app" if no type specified
    }
    
    /// Get the description
    pub fn description(&self) -> Option<&str> {
        self.params.get("description")?.as_str()
    }
    
    /// Get the key string
    pub fn key(&self) -> Option<&str> {
        self.params.get("key")?.as_str()
    }
    
    /// Get the keystroke (computed from key on demand)
    pub fn keystroke(&self) -> Option<Keystroke> {
        self.key().and_then(|key_str| {
            Keystroke::from_key_string(key_str).ok()
        })
    }
    
    /// Get a parameter value
    pub fn get(&self, key: &str) -> Option<&JsonValue> {
        self.params.get(key)
    }
    
    /// Get a string parameter
    pub fn get_string(&self, key: &str) -> Option<&str> {
        self.params.get(key)?.as_str()
    }
    
    /// Get all parameters
    pub fn params(&self) -> &HashMap<String, JsonValue> {
        &self.params
    }
    
    /// Convert this action to a Template if it's a template action
    pub fn to_template(&self) -> Option<crate::core::template_creation::Template> {
        if self.action_type() != "template" {
            return None;
        }
        
        // Extract template fields from params
        let name = self.params.get("name")
            .and_then(|v| v.as_str())
            .unwrap_or("{{input}}")
            .to_string();
            
        let action = self.params.get("action")
            .and_then(|v| v.as_str())
            .unwrap_or("app")
            .to_string();
            
        let arg = self.params.get("arg")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
            
        let patch = self.params.get("patch")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
            
        let flags = self.params.get("flags")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
            
        let grab = self.params.get("grab")
            .and_then(|v| v.as_u64())
            .map(|v| v as u32);
            
        let edit = self.params.get("edit")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);
            
        let file = self.params.get("file")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
            
        let file_rescan = self.params.get("file_rescan")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);
        
        let use_existing = self.params.get("use_existing")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);
        
        Some(crate::core::template_creation::Template {
            name,
            action,
            arg,
            patch,
            flags,
            key: self.key().map(|s| s.to_string()),
            keystroke: None,  // Will be computed if needed
            grab,
            edit,
            use_existing,
            file,
            contents: self.params.get("contents")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string()),
            description: self.description().map(|s| s.to_string()),
            validate_previous_folder: false,  // Not used in unified actions
            file_rescan,
        })
    }
}

/// Helper function to create a simple Action with just type and arg
/// This is the most common pattern in the codebase
pub fn make_action(action_type: &str, arg: &str) -> Action {
    let mut params = HashMap::new();
    params.insert("action_type".to_string(), JsonValue::String(action_type.to_string()));
    if !arg.is_empty() {
        params.insert("arg".to_string(), JsonValue::String(arg.to_string()));
    }
    
    Action { params }
}


/// Context for action execution with JavaScript variable expansion
pub struct ActionContext {
    /// Input text from user
    pub input: String,
    
    /// Currently selected command (if any)
    pub selected_command: Option<Command>,
    
    /// Previously executed command (if any)
    pub previous_command: Option<Command>,
    
    /// Command argument (when action is invoked via command)
    pub arg: Option<String>,
    
    /// Additional variables for expansion
    pub variables: HashMap<String, String>,
}

impl ActionContext {
    /// Create a new action context
    pub fn new(input: String) -> Self {
        let mut variables = HashMap::new();
        
        // Add date/time variables using shared function from template_creation
        crate::core::template_creation::add_datetime_variables(&mut variables);
        
        Self {
            input,
            selected_command: None,
            previous_command: None,
            arg: None,
            variables,
        }
    }
    
    /// Set the selected command
    pub fn with_selected(mut self, command: Command) -> Self {
        self.selected_command = Some(command);
        self
    }
    
    /// Set the previous command
    pub fn with_previous(mut self, command: Command) -> Self {
        self.previous_command = Some(command);
        self
    }
    
    /// Set the command argument
    pub fn with_arg(mut self, arg: String) -> Self {
        self.arg = Some(arg);
        self
    }
    
    /// Add a custom variable
    pub fn add_variable(&mut self, name: String, value: String) {
        self.variables.insert(name, value);
    }
}

// Removed unused expansion functions - expansion now handled by TemplateContext


/// Execute an action locally in the current process
/// 
/// # Parameters
/// - `action`: The action definition containing type and static parameters
/// - `arg`: Optional primary argument (typically from user input or command)
/// - `variables`: Optional additional variables for template expansion
pub(super) fn execute_locally(
    action: &Action,
    arg: Option<&str>,
    variables: Option<HashMap<String, String>>,
) -> Result<String, Box<dyn std::error::Error>> {
    // Get key values for logging
    let action_name = action.get_string("command_name").unwrap_or("(unnamed)").to_string();
    let action_type = action.action_type();
    let action_arg = arg.unwrap_or_else(|| action.get_string("arg").unwrap_or(""));
    let action_flags = action.get_string("flags").unwrap_or("");
    
    // Log main action line
    let main_log = format!("ACTION:{}  TYPE:{}  FLAGS:{}  ARG:{}", action_name, action_type, action_flags, action_arg);
    crate::utils::log(&main_log);
    
    // Log detailed action info (only to detailed log)
    let mut details = format!("ACTION DETAILS  FLAGS:{}", action_flags);
    
    // Add other key-value pairs from action params
    for (key, value) in &action.params {
        if key != "command_name" && key != "action_type" && key != "arg" && key != "flags" {
            let value_str = match value {
                JsonValue::String(s) => s.clone(),
                JsonValue::Bool(b) => b.to_string(),
                JsonValue::Number(n) => n.to_string(),
                _ => "...".to_string(), // Complex values
            };
            details.push_str(&format!(", {}:{}", key, value_str));
        }
    }
    crate::utils::detailed_log("ACTION", &details);
    
    // Merge all parameters into a single HashMap
    let mut params = HashMap::new();
    
    // Add the primary argument if present
    if let Some(arg_value) = arg {
        params.insert("arg".to_string(), arg_value.to_string());
    }
    
    // Add any additional variables
    if let Some(vars) = variables {
        for (key, value) in vars {
            params.insert(key, value);
        }
    }
    
    // Add action's static parameters
    for (key, value) in &action.params {
        let value_str = match value {
            JsonValue::String(s) => s.clone(),
            JsonValue::Bool(b) => b.to_string(),
            JsonValue::Number(n) => n.to_string(),
            _ => continue, // Skip complex values for now
        };
        params.insert(key.clone(), value_str);
    }

    // No expansion here - expansion should be done by caller (popup or CLI)
    // using TemplateContext before calling this function

    // Dispatch based on action type
    match action.action_type() {
        // Virtual anchors - non-executable
        "noop" | "" if params.get("arg").map(|s| s.is_empty()).unwrap_or(true) => {
            // Virtual anchor with blank action and blank arg - do nothing
            crate::utils::detailed_log("EXECUTE", "Virtual anchor - not executing");
            Ok("Virtual anchor (not executable)".to_string())
        }

        // Builtin Rust actions
        "template" => execute_template_action(action, &params),
        "popup" => execute_popup_action(action, &params),
        "open_url" => execute_open_url_action(&params),
        "app" => execute_open_app_action(&params),  // Renamed from open_app
        "open_folder" => execute_open_folder_action(&params),
        "open_file" => execute_open_file_action(&params),
        "shell" => execute_shell_action(&params),
        "obsidian" => execute_obsidian_action(&params),
        "alias" => execute_alias_action(&params),
        
        // Everything else is JavaScript with action_ prefix
        _ => {
            // Check if there's a specific function name in params (for legacy js or js_function type)
            let function_name = if action.action_type() == "js_function" || action.action_type() == "js" {
                // Legacy format: use the function name from params
                action.params.get("function")
                    .and_then(|v| v.as_str())
                    .unwrap_or(&format!("action_{}", action.action_type()))
                    .to_string()
            } else {
                // Modern format: action_type "foo" calls JavaScript function "action_foo()"
                format!("action_{}", action.action_type())
            };
            execute_js_function_action(&function_name, &params)
        }
    }
}

// Action type implementations

fn execute_template_action(
    _action: &Action,
    params: &HashMap<String, String>,
) -> Result<String, Box<dyn std::error::Error>> {
    detailed_log("ACTION", "Executing template action");
    
    // Create a Command from the template parameters
    let mut command = Command {
        command: params.get("name").cloned().unwrap_or_default(),
        action: params.get("action").cloned().unwrap_or_default(),
        arg: params.get("arg").cloned().unwrap_or_default(),
        patch: params.get("patch").cloned().unwrap_or_default(),
        flags: params.get("flags").cloned().unwrap_or_default(),
        other_params: None,
        last_update: 0,
        file_size: None,
    };
    command.update_full_line();
    
    // Handle file creation if specified
    if let Some(contents) = params.get("contents") {
        let file_path = params.get("arg").ok_or("Template with contents requires arg (file path)")?;
        let expanded_path = crate::utils::expand_tilde(file_path);
        
        // Create parent directory if needed
        if let Some(parent) = std::path::Path::new(&expanded_path).parent() {
            std::fs::create_dir_all(parent)?;
        }
        
        // Write the file
        std::fs::write(&expanded_path, contents)?;
        detailed_log("ACTION", &format!("Created file: {}", expanded_path));
    }
    
    // Handle additional folder creation
    if let Some(folder) = params.get("file") {
        let expanded_folder = crate::utils::expand_tilde(folder);
        std::fs::create_dir_all(&expanded_folder)?;
        detailed_log("ACTION", &format!("Created folder: {}", expanded_folder));
    }
    
    // Add the command to commands.txt
    let (mut sys_data, _) = crate::core::data::get_sys_data();
    crate::core::add_command(command.clone())?;
    
    Ok(format!("Created command: {}", command.command))
}

fn execute_popup_action(
    _action: &Action,
    params: &HashMap<String, String>,
) -> Result<String, Box<dyn std::error::Error>> {
    let popup_action = params.get("popup_action")
        .ok_or("popup action requires popup_action parameter")?;
    
    detailed_log("ACTION", &format!("Executing popup action: {}", popup_action));
    
    // Popup actions are handled by the UI layer
    // This function is mainly for CLI testing
    
    Ok(format!("Popup action: {}", popup_action))
}

fn execute_open_url_action(
    params: &HashMap<String, String>,
) -> Result<String, Box<dyn std::error::Error>> {
    let url = params.get("url")
        .ok_or("open_url action requires url parameter")?;
    
    let browser = params.get("browser");
    
    crate::utils::log(&format!("OPEN_URL_ACTION: url='{}', browser={:?}", url, browser));
    detailed_log("ACTION", &format!("Opening URL: {} in browser: {:?}", url, browser));
    
    // Use the launcher system to open URL
    let result = if let Some(browser_name) = browser {
        crate::utils::log(&format!("OPEN_URL_ACTION: Using browser: {}", browser_name));
        detailed_log("ACTION", &format!("Calling open_with_app('{}', '{}')", browser_name, url));
        crate::utils::open_with_app(browser_name, url)
    } else {
        crate::utils::log("OPEN_URL_ACTION: Using default browser");
        crate::utils::open_url(url)
    };
    
    // Handle NON_BLOCKING_SUCCESS as success
    match result {
        Ok(_) => {
            crate::utils::log(&format!("OPEN_URL_ACTION: Success - opened {}", url));
            Ok(format!("Opened URL: {}", url))
        },
        Err(e) if e.to_string().contains("NON_BLOCKING_SUCCESS") => {
            crate::utils::log(&format!("OPEN_URL_ACTION: NON_BLOCKING_SUCCESS - opened {}", url));
            Ok(format!("Opened URL: {}", url))
        }
        Err(e) => {
            crate::utils::log_error(&format!("OPEN_URL_ACTION: Failed - {}", e));
            Err(e.into())
        }
    }
}

fn execute_open_app_action(
    params: &HashMap<String, String>,
) -> Result<String, Box<dyn std::error::Error>> {
    // For open_app, the app path/name can be in either 'app' or 'arg' field
    // Commands from commands.txt put it in 'arg', config actions put it in 'app'
    let app = params.get("app")
        .or_else(|| params.get("arg"))
        .ok_or("open_app action requires app or arg parameter")?;
    
    // Additional arguments if specified separately
    let args = params.get("args");
    
    crate::utils::log(&format!("OPEN_APP: Attempting to launch app: '{}' with args: {:?}", app, args));
    crate::utils::detailed_log("OPEN_APP", &format!("Full params: {:?}", params));
    
    let result = crate::utils::launch_app_with_arg(app, args.as_ref().map(|s| s.as_str()));
    
    // Handle NON_BLOCKING_SUCCESS as success
    match result {
        Ok(_) => {
            crate::utils::detailed_log("OPEN_APP", &format!("OPEN_APP: Successfully launched: {}", app));
            Ok(format!("Launched app: {}", app))
        },
        Err(e) if e.to_string().contains("NON_BLOCKING_SUCCESS") => {
            crate::utils::log(&format!("OPEN_APP: Non-blocking launch successful: {}", app));
            Ok(format!("Launched app: {}", app))
        }
        Err(e) => {
            crate::utils::log_error(&format!("OPEN_APP: Failed to launch '{}': {}", app, e));
            Err(e.into())
        }
    }
}

fn execute_open_folder_action(
    params: &HashMap<String, String>,
) -> Result<String, Box<dyn std::error::Error>> {
    // For open_folder, the path can be in either 'folder', 'path', or 'arg' field
    let path = params.get("folder")
        .or_else(|| params.get("path"))
        .or_else(|| params.get("arg"))
        .ok_or("open_folder action requires folder, path, or arg parameter")?;
    
    detailed_log("ACTION", &format!("Opening folder: {}", path));
    
    let result = crate::utils::open_folder(path);
    
    // Handle NON_BLOCKING_SUCCESS as success
    match result {
        Ok(_) => Ok(format!("Opened folder: {}", path)),
        Err(e) if e.to_string().contains("NON_BLOCKING_SUCCESS") => {
            Ok(format!("Opened folder: {}", path))
        }
        Err(e) => Err(e.into())
    }
}

fn execute_open_file_action(
    params: &HashMap<String, String>,
) -> Result<String, Box<dyn std::error::Error>> {
    // For open_file, the path can be in either 'file', 'path', or 'arg' field
    let path = params.get("file")
        .or_else(|| params.get("path"))
        .or_else(|| params.get("arg"))
        .ok_or("open_file action requires file, path, or arg parameter")?;
    
    detailed_log("ACTION", &format!("Opening file: {}", path));
    
    // Use default app to open file
    let result = crate::utils::open_with_app("", path);
    
    // Handle NON_BLOCKING_SUCCESS as success
    match result {
        Ok(_) => Ok(format!("Opened file: {}", path)),
        Err(e) if e.to_string().contains("NON_BLOCKING_SUCCESS") => {
            Ok(format!("Opened file: {}", path))
        }
        Err(e) => Err(e.into())
    }
}

fn execute_shell_action(
    params: &HashMap<String, String>,
) -> Result<String, Box<dyn std::error::Error>> {
    // For "cmd" actions from commands.txt, the command is in "arg"
    // For "shell" actions, it's in "command"
    let command = params.get("command")
        .or_else(|| params.get("arg"))
        .ok_or("shell/cmd action requires command or arg parameter")?;
    
    let windowed = params.get("windowed")
        .map(|s| s == "true")
        .unwrap_or(false);
    
    // Check for GUI flag (commands that start with "G; ")
    let (actual_command, is_gui) = if command.starts_with("G; ") {
        (&command[3..], true)
    } else {
        (command.as_str(), false)
    };
    
    detailed_log("ACTION", &format!("Executing shell command: {} (windowed: {}, gui: {})", actual_command, windowed, is_gui));
    
    if windowed {
        // Open in terminal window
        let script = format!(
            "tell application \"Terminal\" to do script \"{}\"",
            actual_command.replace("\"", "\\\"")
        );
        crate::utils::shell_simple(&format!("osascript -e '{}'", script), false)?;
    } else {
        // Use blocking execution for GUI commands (G flag)
        crate::utils::shell_simple(actual_command, is_gui)?;
    }
    
    Ok(format!("Executed shell command"))
}


fn execute_js_function_action(
    function_name: &str,
    params: &HashMap<String, String>,
) -> Result<String, Box<dyn std::error::Error>> {
    detailed_log("ACTION", &format!("Executing JavaScript function: {}", function_name));

    // Log extra parameters (excluding standard ones)
    let standard_params = ["action_type", "patch", "command_name", "arg", "flags", "description"];
    let mut extra_params = Vec::new();
    for (key, value) in params {
        if !standard_params.contains(&key.as_str()) {
            extra_params.push(format!("{}='{}'", key, value));
        }
    }
    if !extra_params.is_empty() {
        let param_msg = format!("   {}", extra_params.join(", "));
        crate::utils::log(&param_msg);
    }
    
    // Get parameters that will be passed to the JavaScript function
    let arg = params.get("arg").unwrap_or(&String::new()).clone();
    let input = params.get("input").unwrap_or(&String::new()).clone();
    let command_name = params.get("command_name").unwrap_or(&String::new()).clone();
    let user_input = params.get("user_input").unwrap_or(&String::new()).clone();
    let last_anchor_input = params.get("last_anchor_input").unwrap_or(&String::new()).clone();

    // Simply call the function that's already loaded in the JavaScript runtime
    // The function is registered as a global and expects the arguments directly
    let js_code = format!(r#"
        // Check if function exists
        if (typeof {func} === 'undefined') {{
            throw new Error('JavaScript function "{func}" not found. Make sure it is exported from config.js');
        }}

        // Call the function with simple arguments including last_anchor_input
        // The wrapper in js_runtime.rs will create the context object
        {func}('{arg}', '{input}', null, null, null, '{command_name}', '{user_input}', '{last_anchor_input}');
    "#,
        func = function_name,
        arg = arg.replace("\\", "\\\\").replace("'", "\\'").replace("\n", "\\n").replace("\"", "\\\""),
        input = input.replace("\\", "\\\\").replace("'", "\\'").replace("\n", "\\n").replace("\"", "\\\""),
        command_name = command_name.replace("\\", "\\\\").replace("'", "\\'").replace("\n", "\\n").replace("\"", "\\\""),
        user_input = user_input.replace("\\", "\\\\").replace("'", "\\'").replace("\n", "\\n").replace("\"", "\\\""),
        last_anchor_input = last_anchor_input.replace("\\", "\\\\").replace("'", "\\'").replace("\n", "\\n").replace("\"", "\\\"")
    );
    
    detailed_log("ACTION", &format!("Calling JavaScript: {}", js_code));
    
    match crate::js::execute_with_context(&js_code, &format!("JS_FUNCTION({})", function_name)) {
        Ok(result) => {
            detailed_log("ACTION", &format!("JavaScript execution succeeded: {}", result));
            Ok(result)
        }
        Err(e) => {
            detailed_log("ACTION", &format!("JavaScript execution failed: {}", e));
            Err(e)
        }
    }
}

fn execute_obsidian_action(
    params: &HashMap<String, String>,
) -> Result<String, Box<dyn std::error::Error>> {
    let file = params.get("file")
        .ok_or("obsidian action requires file parameter")?;
    
    let default_vault = crate::core::data::get_config().launcher_settings
        .as_ref()
        .and_then(|s| s.obsidian_vault_name.as_ref())
        .unwrap_or(&"kmr".to_string())
        .clone();
    let vault = params.get("vault")
        .unwrap_or(&default_vault);
    
    detailed_log("ACTION", &format!("Opening in Obsidian: {} (vault: {})", file, vault));
    
    let url = format!("obsidian://open?vault={}&file={}", 
        urlencoding::encode(vault),
        urlencoding::encode(file)
    );
    
    let obsidian_app = &crate::core::data::get_config().launcher_settings
        .as_ref()
        .and_then(|s| s.obsidian_app_name.as_ref())
        .unwrap_or(&"Obsidian".to_string())
        .clone();
    crate::utils::launch_app_with_arg(obsidian_app, Some(&url))?;
    
    Ok(format!("Opened in Obsidian: {}", file))
}


// ============================================================================
// OLD/UNUSED NATIVE IMPLEMENTATIONS 
// Kept for reference - these have been replaced by JavaScript implementations
// ============================================================================

#[allow(dead_code)]
fn execute_1password_action(
    params: &HashMap<String, String>,
) -> Result<String, Box<dyn std::error::Error>> {
    let query = params.get("query")
        .ok_or("1password action requires query parameter")?;
    
    detailed_log("ACTION", &format!("Searching 1Password for: {}", query));
    
    // Use the 1Password Quick Access approach
    let script = format!(
        "tell application \"System Events\" to keystroke \" \" using {{shift down, command down}}' && \
        /bin/sleep 0.5 && \
        osascript -e 'tell application \"System Events\" to keystroke \"{}\"' && \
        /bin/sleep 0.3 && \
        osascript -e 'tell application \"System Events\" to key code 36",
        query
    );
    
    crate::utils::shell_simple(&format!("osascript -e '{}'", script), false)?;
    
    Ok(format!("Searched 1Password for: {}", query))
}

#[allow(dead_code)]
fn execute_slack_action(
    params: &HashMap<String, String>,
) -> Result<String, Box<dyn std::error::Error>> {
    let channel = params.get("channel")
        .ok_or("slack action requires channel parameter")?;
    
    detailed_log("ACTION", &format!("Navigating to Slack channel: {}", channel));
    
    // Activate Slack and navigate to channel
    let script = format!(
        "tell application \"Slack\" to activate' && \
        /bin/sleep 0.5 && \
        osascript -e 'tell application \"System Events\" to keystroke \"k\" using {{command down}}' && \
        /bin/sleep 0.5 && \
        osascript -e 'tell application \"System Events\" to keystroke \"{}\"' && \
        /bin/sleep 0.5 && \
        osascript -e 'tell application \"System Events\" to keystroke return",
        channel
    );
    
    crate::utils::shell_simple(&format!("osascript -e '{}'", script), false)?;
    
    Ok(format!("Navigated to Slack channel: {}", channel))
}

#[allow(dead_code)]
fn execute_tmux_action(
    params: &HashMap<String, String>,
) -> Result<String, Box<dyn std::error::Error>> {
    let path = params.get("path")
        .ok_or("tmux action requires path parameter")?;
    
    let session = params.get("session");
    
    detailed_log("ACTION", &format!("Activating tmux at: {} (session: {:?})", path, session));
    
    // TODO: Implement tmux activation logic
    
    Ok(format!("Activated tmux at: {}", path))
}

#[allow(dead_code)]
fn execute_type_text_action(
    params: &HashMap<String, String>,
) -> Result<String, Box<dyn std::error::Error>> {
    let text = if let Some(file) = params.get("file") {
        // Read from file
        std::fs::read_to_string(crate::utils::expand_tilde(file))?
    } else if let Some(text) = params.get("text") {
        text.clone()
    } else {
        return Err("type_text action requires either file or text parameter".into());
    };
    
    detailed_log("ACTION", &format!("Typing {} characters", text.len()));
    
    // Type the text using AppleScript
    let escaped = text
        .replace("\\", "\\\\")
        .replace("\"", "\\\"")
        .replace("'", "\\'");
    
    let script = format!(
        "tell application \"System Events\" to keystroke \"{}\"",
        escaped
    );
    
    crate::utils::shell_simple(&format!("osascript -e '{}'", script), false)?;
    
    Ok(format!("Typed {} characters", text.len()))
}

fn execute_alias_action(
    params: &HashMap<String, String>,
) -> Result<String, Box<dyn std::error::Error>> {
    let target = params.get("target")
        .or_else(|| params.get("target_command"))
        .or_else(|| params.get("arg"))
        .ok_or("alias action requires target parameter")?;
    
    detailed_log("ACTION", &format!("Resolving alias to: {}", target));
    
    // Find and execute the target command
    let commands = crate::core::commands::load_commands_raw();
    
    // Look for the target command (case-insensitive)
    // First try exact match, then try without patch prefix
    let target_lower = target.to_lowercase();
    let target_cmd = commands.iter()
        .find(|cmd| {
            // Try exact match first
            if cmd.command.to_lowercase() == target_lower {
                return true;
            }
            // If command has a patch (contains '!'), try matching just the part after the patch
            if let Some(exclaim_pos) = cmd.command.find('!') {
                let cmd_without_patch = cmd.command[exclaim_pos + 1..].trim();
                if cmd_without_patch.to_lowercase() == target_lower {
                    return true;
                }
            }
            false
        })
        .ok_or_else(|| format!("Alias target '{}' not found", target))?;
    
    // Execute the target command
    detailed_log("ACTION", &format!("Executing aliased command: {} (action: {})", 
        target_cmd.command, target_cmd.action));
    
    // Execute the resolved command
    let target_action = crate::execute::command_to_action(&target_cmd);
    let mut variables = std::collections::HashMap::new();
    variables.insert("arg".to_string(), target_cmd.arg.clone());
    let _ = crate::execute::execute_on_server(&target_action, Some(variables));
    
    Ok(format!("Executed alias to: {}", target))
}



#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_expand_string_basic() {
        let context = ActionContext::new("test input".to_string());
        
        let result = expand_string("Hello {{input}}!", &context).unwrap();
        assert_eq!(result, "Hello test input!");
    }
    
    #[test]
    fn test_expand_string_javascript() {
        let context = ActionContext::new("test".to_string());
        
        let result = expand_string("Length: {{input.length}}", &context).unwrap();
        assert_eq!(result, "Length: 4");
        
        let result = expand_string("Upper: {{input.toUpperCase()}}", &context).unwrap();
        assert_eq!(result, "Upper: TEST");
    }
    
    #[test]
    fn test_expand_string_with_objects() {
        let mut context = ActionContext::new("input".to_string());
        context.previous_command = Some(Command {
            command: "Test Command".to_string(),
            action: "test".to_string(),
            arg: "/path/to/file.md".to_string(),
            patch: "TestPatch".to_string(),
            flags: String::new(),
        other_params: None,
        last_update: 0,
        file_size: None,
        });
        
        let result = expand_string("Previous: {{previous.name}}", &context).unwrap();
        assert_eq!(result, "Previous: Test Command");
        
        let result = expand_string("Folder: {{previous.folder}}", &context).unwrap();
        assert_eq!(result, "Folder: /path/to");
    }
}

// ============================================================================
// Utility Functions for Scanner and Grabber
// ============================================================================

/// Determine the appropriate action type for a given file path
pub fn get_action(path: &std::path::Path) -> &'static str {
    if path.is_dir() {
        "folder"
    } else {
        let extension = path.extension()
            .and_then(|ext| ext.to_str())
            .map(|s| s.to_lowercase());
        
        match extension.as_deref() {
            Some("md") => {
                // Use the helper function for consistent anchor detection
                if crate::utils::is_anchor_file(path) {
                    "anchor"
                } else {
                    "markdown"
                }
            },
            Some("txt") | Some("text") => "text",
            Some("pdf") | Some("doc") | Some("docx") => "doc",
            Some("app") => "app",
            Some("html") | Some("htm") => "url",
            _ => "doc"  // Default to doc for unknown files
        }
    }
}

/// Determine the appropriate action type for a given argument string
/// This can handle URLs, file paths, and other special cases
pub fn get_action_for_arg(arg: &str) -> &'static str {
    // Check if it's a URL
    if arg.starts_with("http://") || arg.starts_with("https://") {
        // Special case for Notion URLs
        if arg.contains("notion.so") {
            return "notion";
        }
        return "url";
    }
    
    // Check if it's an obsidian URL
    if arg.starts_with("obsidian://") {
        return "obs_url";
    }
    
    // Otherwise treat as a file path
    let path = std::path::Path::new(arg);
    get_action(path)
}



/// Get the default patch for a given action type
pub fn get_default_patch_for_action(action: &str) -> Option<&'static str> {
    match action {
        "folder" => Some("Folders"),
        "markdown" | "anchor" => Some("Anchors"),
        "doc" | "text" => Some("Documents"),
        "app" => Some("Apps"),
        "url" => Some("URLs"),
        _ => None
    }
}
