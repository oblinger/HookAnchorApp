//! Unified Actions System
//! 
//! This module implements the unified actions architecture where all behaviors
//! (templates, keyboard actions, command actions) are defined as typed actions.
//! Actions are typed by their behavior (what they do) not their invocation method.

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use rquickjs::{Context, Ctx};
use crate::Command;
use crate::core::key_processing::Keystroke;
use crate::utils::{debug_log, log_error};

/// A unified action that can be invoked via keyboard, command, or other actions
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Action {
    /// The type of action (behavior-based: template, popup, open_url, etc.)
    #[serde(rename = "type")]
    pub action_type: String,
    
    /// Optional description of what this action does
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    
    /// Optional keyboard key that triggers this action
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    
    /// Computed keystroke for efficient keyboard matching
    #[serde(skip)]
    pub keystroke: Option<Keystroke>,
    
    /// All other parameters stored as generic JSON for flexibility
    #[serde(flatten)]
    pub params: HashMap<String, JsonValue>,
}

impl Action {
    /// Get the action type
    pub fn action_type(&self) -> &str {
        &self.action_type
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
        if self.action_type != "template" {
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
        
        Some(crate::core::template_creation::Template {
            name,
            action,
            arg,
            patch,
            flags,
            key: self.key.clone(),
            keystroke: None,  // Will be computed if needed
            grab,
            edit,
            file,
            contents: None,  // Not used in unified actions
            description: self.description.clone(),
            validate_previous_folder: false,  // Not used in unified actions
            file_rescan,
        })
    }
}

/// Convert a Command to an Action
pub fn command_to_action(cmd: &Command) -> Action {
    let mut params = HashMap::new();
    
    // Add command fields as parameters
    params.insert("arg".to_string(), JsonValue::String(cmd.arg.clone()));
    params.insert("command_name".to_string(), JsonValue::String(cmd.command.clone()));
    params.insert("patch".to_string(), JsonValue::String(cmd.patch.clone()));
    params.insert("flags".to_string(), JsonValue::String(cmd.flags.clone()));
    
    Action {
        action_type: cmd.action.clone(),
        description: None,
        key: None,
        keystroke: None,
        params,
    }
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
        
        // Add date/time variables
        add_datetime_variables(&mut variables);
        
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

/// Expand all parameters that contain {{...}} JavaScript expressions
fn expand_parameters(
    params: &HashMap<String, String>,
) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
    let mut expanded = HashMap::new();
    
    // Check if any parameter needs expansion
    let needs_expansion = params.values().any(|v| v.contains("{{"));
    
    if !needs_expansion {
        // No expansion needed, return as-is
        return Ok(params.clone());
    }
    
    // Create JavaScript context with all parameters as variables
    let js_ctx = create_js_context_with_params(params)?;
    
    js_ctx.with(|ctx| {
        for (key, value) in params {
            if value.contains("{{") {
                // Expand this parameter
                let expanded_value = expand_template_string(&ctx, value)?;
                expanded.insert(key.clone(), expanded_value);
            } else {
                // No expansion needed
                expanded.insert(key.clone(), value.clone());
            }
        }
        Ok(expanded)
    })
}

/// Expand a single template string with {{...}} expressions
fn expand_template_string(
    ctx: &Ctx<'_>,
    template: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut result = template.to_string();
    let mut last_pos = 0;
    
    // Process all {{...}} expressions
    while let Some(start) = result[last_pos..].find("{{") {
        let start = last_pos + start;
        if let Some(end) = result[start..].find("}}") {
            let end = start + end + 2;
            let expr = result[start + 2..end - 2].trim();
            
            debug_log("EXPAND", &format!("Evaluating expression: {}", expr));
            
            // Evaluate the JavaScript expression
            match ctx.eval::<rquickjs::Value, _>(expr) {
                Ok(value) => {
                    let expanded = js_value_to_string(&value);
                    debug_log("EXPAND", &format!("Expression '{}' expanded to: '{}'", expr, expanded));
                    result.replace_range(start..end, &expanded);
                    last_pos = start + expanded.len();
                }
                Err(e) => {
                    // On error, show dialog and terminate
                    let error_msg = format!(
                        "Couldn't execute this action because of an error in expansion:\n\
                        Expression: {}\n\
                        Error: {}",
                        expr, e
                    );
                    log_error(&error_msg);
                    crate::error_display::queue_user_error(&error_msg);
                    return Err(error_msg.into());
                }
            }
        } else {
            // No matching }} found
            break;
        }
    }
    
    Ok(result)
}

/// Keep the old function for backward compatibility temporarily
/// TODO: Remove after all callers are updated
pub fn expand_string(
    template: &str,
    context: &ActionContext,
) -> Result<String, Box<dyn std::error::Error>> {
    // Convert context to params
    let mut params = HashMap::new();
    params.insert("input".to_string(), context.input.clone());
    if let Some(arg) = &context.arg {
        params.insert("arg".to_string(), arg.clone());
    }
    for (k, v) in &context.variables {
        params.insert(k.clone(), v.clone());
    }
    
    // Use the new expansion
    let expanded_params = expand_parameters(&params)?;
    expanded_params.get(template)
        .cloned()
        .ok_or_else(|| "Template not found in expanded params".into())
}

/// Create a JavaScript context with parameters as variables
fn create_js_context_with_params(
    params: &HashMap<String, String>,
) -> Result<Context, Box<dyn std::error::Error>> {
    let config = crate::core::sys_data::get_config();
    let rt = rquickjs::Runtime::new()?;
    let ctx = Context::full(&rt)?;
    
    ctx.with(|ctx| -> Result<(), Box<dyn std::error::Error>> {
        // Setup all standard built-in functions
        crate::js_runtime::setup_all_builtins(&ctx)?;
        
        // Add all parameters as global variables
        let globals = ctx.globals();
        for (key, value) in params {
            globals.set(key.as_str(), value.clone())?;
        }
        
        // Add date/time variables
        let mut date_vars = HashMap::new();
        add_datetime_variables(&mut date_vars);
        for (key, value) in date_vars {
            globals.set(key.as_str(), value)?;
        }
        
        // Setup user-defined functions from config
        setup_user_functions(&ctx, &config)?;
        
        Ok(())
    })?;
    
    Ok(ctx)
}

/// Create a JavaScript context with action-specific variables (deprecated)
fn create_action_js_context(
    context: &ActionContext,
) -> Result<Context, Box<dyn std::error::Error>> {
    let config = crate::core::sys_data::get_config();
    let rt = rquickjs::Runtime::new()?;
    let ctx = Context::full(&rt)?;
    
    ctx.with(|ctx| -> Result<(), Box<dyn std::error::Error>> {
        // Setup all standard built-in functions
        crate::js_runtime::setup_all_builtins(&ctx)?;
        
        // Add context variables
        setup_action_variables(&ctx, context)?;
        
        // Setup user-defined functions from config
        setup_user_functions(&ctx, &config)?;
        
        Ok(())
    })?;
    
    Ok(ctx)
}

/// Setup action-specific variables in JavaScript context
fn setup_action_variables(
    ctx: &Ctx<'_>,
    context: &ActionContext,
) -> Result<(), Box<dyn std::error::Error>> {
    let globals = ctx.globals();
    
    // Basic variables
    globals.set("input", context.input.clone())?;
    if let Some(arg) = &context.arg {
        globals.set("arg", arg.clone())?;
    } else {
        globals.set("arg", "")?;
    }
    
    // Previous command object
    if let Some(cmd) = &context.previous_command {
        let previous = rquickjs::Object::new(ctx.clone())?;
        previous.set("name", cmd.command.clone())?;
        previous.set("path", cmd.arg.clone())?;
        previous.set("folder", extract_folder_from_path(&cmd.arg).unwrap_or_default())?;
        previous.set("hook", create_hook_url(&cmd.command))?;
        globals.set("previous", previous)?;
    } else {
        // Empty previous object
        let previous = rquickjs::Object::new(ctx.clone())?;
        previous.set("name", "")?;
        previous.set("path", "")?;
        previous.set("folder", "")?;
        previous.set("hook", "")?;
        globals.set("previous", previous)?;
    }
    
    // Selected command object
    if let Some(cmd) = &context.selected_command {
        let selected = rquickjs::Object::new(ctx.clone())?;
        selected.set("name", cmd.command.clone())?;
        selected.set("path", cmd.arg.clone())?;
        selected.set("folder", extract_folder_from_path(&cmd.arg).unwrap_or_default())?;
        selected.set("hook", create_hook_url(&cmd.command))?;
        globals.set("selected", selected)?;
    } else {
        // Empty selected object
        let selected = rquickjs::Object::new(ctx.clone())?;
        selected.set("name", "")?;
        selected.set("path", "")?;
        selected.set("folder", "")?;
        selected.set("hook", "")?;
        globals.set("selected", selected)?;
    }
    
    // Add all custom variables
    for (key, value) in &context.variables {
        globals.set(key.as_str(), value.clone())?;
    }
    
    // Add first_match for folder operations (will be populated by search)
    globals.set("first_match", "")?;
    
    // Add grabbed variables for grab functionality
    globals.set("grabbed_action", "app")?;
    globals.set("grabbed_arg", "Finder")?;
    
    Ok(())
}

/// Setup user-defined functions from config
fn setup_user_functions(
    ctx: &Ctx<'_>,
    config: &crate::Config,
) -> Result<(), Box<dyn std::error::Error>> {
    // First, load functions from config.js (just like js_runtime does)
    crate::js_runtime::load_config_js_functions(ctx)?;
    
    // Then load any additional functions from YAML config
    if let Some(functions) = &config.functions {
        for (name, value) in functions {
        if let Some(js_code) = value.as_str() {
            // It's a JavaScript function definition
            let func_code = format!("(function() {{ {} }})", js_code);
            match ctx.eval::<rquickjs::Value, _>(func_code.as_str()) {
                Ok(func) => {
                    ctx.globals().set(name.as_str(), func)?;
                }
                Err(e) => {
                    log_error(&format!("Failed to load function '{}': {}", name, e));
                }
            }
        }
        }
    }
    
    Ok(())
}

/// Convert JavaScript value to string
fn js_value_to_string(value: &rquickjs::Value) -> String {
    if value.is_null() || value.is_undefined() {
        String::new()
    } else if value.is_bool() {
        value.as_bool().unwrap_or(false).to_string()
    } else if value.is_number() {
        value.as_number().unwrap_or(0.0).to_string()
    } else if let Some(s) = value.as_string() {
        s.to_string().unwrap_or_default()
    } else {
        // Try to stringify complex objects
        format!("{:?}", value)
    }
}

/// Extract folder path from a file path
fn extract_folder_from_path(path: &str) -> Option<String> {
    use std::path::Path;
    Path::new(path)
        .parent()
        .and_then(|p| p.to_str())
        .map(|s| s.to_string())
}

/// Create a hook:// URL from a command name
fn create_hook_url(name: &str) -> String {
    // Simple URL-safe encoding: lowercase, remove spaces and special chars
    let safe_name = name
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == '-' || *c == '_')
        .collect::<String>();
    format!("hook://{}", safe_name)
}

/// Add date/time variables to the context
fn add_datetime_variables(variables: &mut HashMap<String, String>) {
    use chrono::{Local, Datelike, Timelike};
    let now = Local::now();
    
    // Year
    variables.insert("YYYY".to_string(), format!("{:04}", now.year()));
    variables.insert("YY".to_string(), format!("{:02}", now.year() % 100));
    
    // Month
    variables.insert("M".to_string(), format!("{}", now.month()));
    variables.insert("MM".to_string(), format!("{:02}", now.month()));
    variables.insert("MMM".to_string(), now.format("%b").to_string());
    
    // Day
    variables.insert("D".to_string(), format!("{}", now.day()));
    variables.insert("DD".to_string(), format!("{:02}", now.day()));
    
    // Hour
    variables.insert("h".to_string(), format!("{}", now.hour()));
    variables.insert("hh".to_string(), format!("{:02}", now.hour()));
    
    // Minute
    variables.insert("m".to_string(), format!("{}", now.minute()));
    variables.insert("mm".to_string(), format!("{:02}", now.minute()));
    
    // Second
    variables.insert("s".to_string(), format!("{}", now.second()));
    variables.insert("ss".to_string(), format!("{:02}", now.second()));
}

/// Execute an action with the given parameters
/// 
/// # Parameters
/// - `action`: The action definition containing type and static parameters
/// - `arg`: Optional primary argument (typically from user input or command)
/// - `variables`: Optional additional variables for template expansion
pub fn execute_action(
    action: &Action,
    arg: Option<&str>,
    variables: Option<HashMap<String, String>>,
) -> Result<String, Box<dyn std::error::Error>> {
    debug_log("ACTION", &format!("Executing action type: {}", action.action_type));
    
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
    
    // Expand all parameters that contain {{...}} expressions
    let expanded_params = expand_parameters(&params)?;
    
    // Dispatch based on action type
    match action.action_type.as_str() {
        "template" => execute_template_action(action, &expanded_params),
        "popup" => execute_popup_action(action, &expanded_params),
        "open_url" => execute_open_url_action(&expanded_params),
        "open_app" => execute_open_app_action(&expanded_params),
        "open_folder" => execute_open_folder_action(&expanded_params),
        "open_file" => execute_open_file_action(&expanded_params),
        "shell" | "cmd" => execute_shell_action(&expanded_params),
        "javascript" => execute_javascript_action(&expanded_params),
        "obsidian" => execute_obsidian_action(&expanded_params),
        "1password" => execute_1password_action(&expanded_params),
        "slack" => execute_slack_action(&expanded_params),
        "tmux" => execute_tmux_action(&expanded_params),
        "type_text" => execute_type_text_action(&expanded_params),
        "alias" => execute_alias_action(&expanded_params),
        "builtin" => execute_builtin_action(action, &expanded_params),
        _ => {
            // Try to find a user-defined action handler
            execute_custom_action(&action.action_type, &expanded_params)
        }
    }
}

// Action type implementations

fn execute_template_action(
    action: &Action,
    params: &HashMap<String, String>,
) -> Result<String, Box<dyn std::error::Error>> {
    debug_log("ACTION", "Executing template action");
    
    // Create a Command from the template parameters
    let mut command = Command {
        command: params.get("name").cloned().unwrap_or_default(),
        action: params.get("action").cloned().unwrap_or_default(),
        arg: params.get("arg").cloned().unwrap_or_default(),
        patch: params.get("patch").cloned().unwrap_or_default(),
        flags: params.get("flags").cloned().unwrap_or_default(),
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
        debug_log("ACTION", &format!("Created file: {}", expanded_path));
    }
    
    // Handle additional folder creation
    if let Some(folder) = params.get("file") {
        let expanded_folder = crate::utils::expand_tilde(folder);
        std::fs::create_dir_all(&expanded_folder)?;
        debug_log("ACTION", &format!("Created folder: {}", expanded_folder));
    }
    
    // Add the command to commands.txt
    let mut sys_data = crate::core::sys_data::get_sys_data();
    crate::core::commands::add_command(command.clone(), &mut sys_data.commands)?;
    
    Ok(format!("Created command: {}", command.command))
}

fn execute_popup_action(
    _action: &Action,
    params: &HashMap<String, String>,
) -> Result<String, Box<dyn std::error::Error>> {
    let popup_action = params.get("popup_action")
        .ok_or("popup action requires popup_action parameter")?;
    
    debug_log("ACTION", &format!("Executing popup action: {}", popup_action));
    
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
    debug_log("ACTION", &format!("Opening URL: {} in browser: {:?}", url, browser));
    
    // Use the launcher system to open URL
    let result = if let Some(browser_name) = browser {
        crate::utils::log(&format!("OPEN_URL_ACTION: Using browser: {}", browser_name));
        debug_log("ACTION", &format!("Calling open_with_app('{}', '{}')", browser_name, url));
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
    let app = params.get("app")
        .ok_or("open_app action requires app parameter")?;
    
    let args = params.get("arg").or_else(|| params.get("args"));
    
    debug_log("ACTION", &format!("Launching app: {} with args: {:?}", app, args));
    
    let result = crate::utils::launch_app_with_arg(app, args.as_ref().map(|s| s.as_str()));
    
    // Handle NON_BLOCKING_SUCCESS as success
    match result {
        Ok(_) => Ok(format!("Launched app: {}", app)),
        Err(e) if e.to_string().contains("NON_BLOCKING_SUCCESS") => {
            Ok(format!("Launched app: {}", app))
        }
        Err(e) => Err(e.into())
    }
}

fn execute_open_folder_action(
    params: &HashMap<String, String>,
) -> Result<String, Box<dyn std::error::Error>> {
    let path = params.get("path")
        .ok_or("open_folder action requires path parameter")?;
    
    debug_log("ACTION", &format!("Opening folder: {}", path));
    
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
    let path = params.get("path")
        .ok_or("open_file action requires path parameter")?;
    
    debug_log("ACTION", &format!("Opening file: {}", path));
    
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
    
    debug_log("ACTION", &format!("Executing shell command: {} (windowed: {}, gui: {})", actual_command, windowed, is_gui));
    
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

fn execute_javascript_action(
    params: &HashMap<String, String>,
) -> Result<String, Box<dyn std::error::Error>> {
    let function_name = params.get("code")
        .ok_or("javascript action requires code parameter")?;
    
    debug_log("ACTION", &format!("Executing JavaScript function: {}", function_name));
    
    // Get parameters that will be passed to the JavaScript function
    let arg = params.get("arg").unwrap_or(&String::new()).clone();
    let input = params.get("input").unwrap_or(&String::new()).clone();
    
    // Simply call the function that's already loaded in the JavaScript runtime
    // The function is registered as a global and expects the arguments directly
    let js_code = format!(r#"
        // Check if function exists
        if (typeof {func} === 'undefined') {{
            throw new Error('JavaScript function "{func}" not found. Make sure it is exported from config.js');
        }}
        
        // Call the function with simple arguments
        // The wrapper in js_runtime.rs will create the context object
        {func}('{arg}', '{input}', null, null, null);
    "#, 
        func = function_name,
        arg = arg.replace("\\", "\\\\").replace("'", "\\'").replace("\n", "\\n").replace("\"", "\\\""),
        input = input.replace("\\", "\\\\").replace("'", "\\'").replace("\n", "\\n").replace("\"", "\\\"")
    );
    
    debug_log("ACTION", &format!("Calling JavaScript: {}", js_code));
    
    crate::js_runtime::execute_business_logic(&js_code)
}

fn execute_obsidian_action(
    params: &HashMap<String, String>,
) -> Result<String, Box<dyn std::error::Error>> {
    let file = params.get("file")
        .ok_or("obsidian action requires file parameter")?;
    
    let default_vault = crate::core::sys_data::get_config().launcher_settings
        .as_ref()
        .and_then(|s| s.obsidian_vault_name.as_ref())
        .unwrap_or(&"kmr".to_string())
        .clone();
    let vault = params.get("vault")
        .unwrap_or(&default_vault);
    
    debug_log("ACTION", &format!("Opening in Obsidian: {} (vault: {})", file, vault));
    
    let url = format!("obsidian://open?vault={}&file={}", 
        urlencoding::encode(vault),
        urlencoding::encode(file)
    );
    
    let obsidian_app = &crate::core::sys_data::get_config().launcher_settings
        .as_ref()
        .and_then(|s| s.obsidian_app_name.as_ref())
        .unwrap_or(&"Obsidian".to_string())
        .clone();
    crate::utils::launch_app_with_arg(obsidian_app, Some(&url))?;
    
    Ok(format!("Opened in Obsidian: {}", file))
}

fn execute_1password_action(
    params: &HashMap<String, String>,
) -> Result<String, Box<dyn std::error::Error>> {
    let query = params.get("query")
        .ok_or("1password action requires query parameter")?;
    
    debug_log("ACTION", &format!("Searching 1Password for: {}", query));
    
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

fn execute_slack_action(
    params: &HashMap<String, String>,
) -> Result<String, Box<dyn std::error::Error>> {
    let channel = params.get("channel")
        .ok_or("slack action requires channel parameter")?;
    
    debug_log("ACTION", &format!("Navigating to Slack channel: {}", channel));
    
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

fn execute_tmux_action(
    params: &HashMap<String, String>,
) -> Result<String, Box<dyn std::error::Error>> {
    let path = params.get("path")
        .ok_or("tmux action requires path parameter")?;
    
    let session = params.get("session");
    
    debug_log("ACTION", &format!("Activating tmux at: {} (session: {:?})", path, session));
    
    // TODO: Implement tmux activation logic
    
    Ok(format!("Activated tmux at: {}", path))
}

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
    
    debug_log("ACTION", &format!("Typing {} characters", text.len()));
    
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
    
    debug_log("ACTION", &format!("Resolving alias to: {}", target));
    
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
    debug_log("ACTION", &format!("Executing aliased command: {} (action: {})", 
        target_cmd.command, target_cmd.action));
    
    // Use execute_via_server to run the resolved command
    crate::execute_via_server(&target_cmd);
    
    Ok(format!("Executed alias to: {}", target))
}

fn execute_builtin_action(
    action: &Action,
    params: &HashMap<String, String>,
) -> Result<String, Box<dyn std::error::Error>> {
    debug_log("ACTION", &format!("Executing builtin action"));
    
    // Get the builtin name from the action params
    let builtin_name = action.params.get("builtin")
        .and_then(|v| v.as_str())
        .ok_or("builtin action missing 'builtin' parameter")?;
    
    // Create an eval environment and execute the builtin
    let mut env = crate::eval::Environment::new()
        .map_err(|e| format!("Failed to create environment: {}", e))?;
    
    // Set the arg variable if we have one
    if let Some(arg) = params.get("arg") {
        env.variables.insert("arg".to_string(), arg.clone());
    }
    
    // Add other params as variables
    for (key, value) in params {
        env.variables.insert(key.clone(), value.clone());
    }
    
    // Create a function call to the builtin
    let mut func_call = serde_yaml::Mapping::new();
    func_call.insert("fn".into(), builtin_name.into());
    
    // Execute the builtin function
    match env.eval(serde_yaml::Value::Mapping(func_call)) {
        Ok(_) => Ok(format!("Builtin '{}' executed", builtin_name)),
        Err(e) => Err(format!("Failed to execute builtin '{}': {:?}", builtin_name, e).into())
    }
}

fn execute_custom_action(
    action_type: &str,
    params: &HashMap<String, String>,
) -> Result<String, Box<dyn std::error::Error>> {
    debug_log("ACTION", &format!("Looking for custom action handler: action_{}", action_type));
    
    // Look for action_<type> function in JavaScript
    let function_name = format!("action_{}", action_type);
    
    // Create JS context with parameters
    let js_ctx = create_js_context_with_params(params)?;
    
    js_ctx.with(|ctx| {
        // Check if function exists
        let globals = ctx.globals();
        if let Ok(func) = globals.get::<_, rquickjs::Value>(&function_name) {
            if func.is_function() {
                // Call the function with expanded arg
                let arg = params.get("arg").cloned().unwrap_or_default();
                let eval_code = format!("{}('{}')", function_name, arg);
                match ctx.eval::<rquickjs::Value, _>(eval_code.as_str()) {
                    Ok(result) => {
                        let result_str = js_value_to_string(&result);
                        Ok(result_str)
                    }
                    Err(e) => {
                        Err(format!("Error executing custom action '{}': {}", action_type, e).into())
                    }
                }
            } else {
                Err(format!("action_{} is not a function", action_type).into())
            }
        } else {
            Err(format!("Unknown action type: {} (no action_{} function found)", action_type, action_type).into())
        }
    })
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
        });
        
        let result = expand_string("Previous: {{previous.name}}", &context).unwrap();
        assert_eq!(result, "Previous: Test Command");
        
        let result = expand_string("Folder: {{previous.folder}}", &context).unwrap();
        assert_eq!(result, "Folder: /path/to");
    }
}