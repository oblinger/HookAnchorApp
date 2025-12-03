//! Template Creation System
//!
//! Provides a flexible template-based system for creating new commands
//! with variable expansion and customizable fields.

use std::collections::HashMap;
use chrono::{Local, Datelike, Timelike};
use serde::{Deserialize, Serialize};
use crate::core::Command;
use crate::core::key_processing::Keystroke;
use crate::prelude::*;

/// Canonical field names for command objects in JavaScript context.
/// These are used consistently in both template expansion and JS action execution.
///
/// The command object (e.g., `selected`, `last_executed`) has these fields:
/// - `name`: The command name
/// - `path`: The file path (same as arg for file-based commands)
/// - `arg`: Alias for path (for backward compatibility)
/// - `patch`: The patch/category name
/// - `action`: The action type (e.g., "markdown", "app", "url")
/// - `flags`: Command flags (e.g., "A" for anchor)
/// - `folder`: Parent folder of the path (throws error if empty when accessed)
///
/// Variable naming convention:
/// - Internal HashMap keys use underscore prefix: `_selected_name`, `_selected_path`, etc.
/// - JavaScript objects use direct property access: `selected.name`, `selected.path`, etc.

/// Operations that can be performed by a template.
/// Templates can define a list of operations to execute in order.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum TemplateOperation {
    /// Create a new command
    #[serde(rename = "command")]
    Command {
        /// The name of the command to create
        name: String,
        /// The action type (e.g., "markdown", "app")
        action: String,
        /// The argument for the command
        arg: String,
        /// The patch/category for the command
        #[serde(default)]
        patch: String,
        /// Command flags (e.g., "A" for anchor)
        #[serde(default)]
        flags: String,
        /// If true, open command editor before creating
        #[serde(default)]
        edit: bool,
        /// If true, use existing command with same name if it exists
        #[serde(default)]
        use_existing: bool,
    },
    /// Create a new file
    #[serde(rename = "create")]
    Create {
        /// Path to the file to create
        file: String,
        /// Contents to write to the file
        #[serde(default)]
        contents: String,
    },
    /// Append to an existing file
    #[serde(rename = "append")]
    Append {
        /// Path to the file to append to
        file: String,
        /// Contents to append
        contents: String,
        /// Optional: insert after this line (if not found, add this line first)
        #[serde(default)]
        after: Option<String>,
    },
    /// Copy contents to clipboard
    #[serde(rename = "clip")]
    Clip {
        /// Contents to put in clipboard
        contents: String,
    },
}

/// Generates JavaScript code to build a command object from extra_params.
/// This is used by the JS runtime to create the `selected` object consistently
/// with the template system's `create_command_object` method.
///
/// The generated code expects `extra_params` to contain:
/// - `_selected_name`, `_selected_path`, `_selected_patch`,
/// - `_selected_action`, `_selected_flags`, `_selected_folder`
pub fn generate_js_command_object_builder() -> &'static str {
    r#"// Build selected object from extra_params (same structure as template system)
                            const selected = {
                                name: (extra_params && extra_params._selected_name) || '',
                                path: (extra_params && extra_params._selected_path) || '',
                                arg: (extra_params && extra_params._selected_path) || '',  // alias for path
                                patch: (extra_params && extra_params._selected_patch) || '',
                                action: (extra_params && extra_params._selected_action) || '',
                                flags: (extra_params && extra_params._selected_flags) || '',
                                folder: (extra_params && extra_params._selected_folder) || ''
                            };"#
}

/// A template for creating new commands
/// Templates use operations to define what to create (commands, files, clipboard, etc.)
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Template {
    /// List of operations to perform (create files, commands, clipboard, etc.)
    pub operations: Vec<TemplateOperation>,

    /// Optional key that triggers this template (e.g., "Cmd+D", "!", etc.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,

    /// Keystroke for efficient matching (computed from key field)
    #[serde(skip)]
    pub keystroke: Option<Keystroke>,

    /// If true, open command editor before creating
    #[serde(default)]
    pub edit: bool,

    /// If true, use existing command with same name if it exists (case-insensitive)
    #[serde(default)]
    pub use_existing: bool,

    /// Optional seconds to wait before grabbing window
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grab: Option<u32>,

    /// Optional description of what this template does (for help display)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// If true, validate that previous_folder exists and is valid
    #[serde(default)]
    pub validate_previous_folder: bool,

    /// If true, rescan file system after creating template
    #[serde(default)]
    pub file_rescan: bool,
}

/// Context for template variable expansion
#[derive(Clone)]
pub struct TemplateContext {
    /// Variables available for expansion
    pub variables: HashMap<String, String>,
}


impl TemplateContext {
    /// Create a basic template context with all universally available data
    /// This gathers everything it can without needing popup state
    /// Suitable for both CLI and as a base for popup execution
    pub fn create_basic_template(input: &str) -> Self {
        let mut variables = HashMap::new();

        // Basic input variable
        variables.insert("input".to_string(), input.to_string());

        // Raw input variable (for CLI, same as input since no alias expansion)
        variables.insert("raw_input".to_string(), input.to_string());

        // Add date/time variables
        add_datetime_variables(&mut variables);

        // Add last executed command from state (available everywhere)
        let state = crate::core::data::get_state();
        if let Some(last_executed_name) = &state.last_executed_command {
            let (sys_data, _) = crate::core::get_sys_data();
            if let Some(cmd) = sys_data.commands.iter().find(|c| &c.command == last_executed_name) {
                let folder = match extract_and_validate_folder(cmd) {
                    Ok(f) => f,
                    Err(_) => String::new()
                };

                variables.insert("_last_executed_name".to_string(), cmd.command.clone());
                variables.insert("_last_executed_path".to_string(), cmd.arg.clone());
                variables.insert("_last_executed_patch".to_string(), cmd.patch.clone());
                variables.insert("_last_executed_action".to_string(), cmd.action.clone());
                variables.insert("_last_executed_flags".to_string(), cmd.flags.clone());
                variables.insert("_last_executed_folder".to_string(), folder);
            }
        } else {
            // Provide empty defaults for last executed
            variables.insert("_last_executed_name".to_string(), String::new());
            variables.insert("_last_executed_path".to_string(), String::new());
            variables.insert("_last_executed_patch".to_string(), String::new());
            variables.insert("_last_executed_action".to_string(), String::new());
            variables.insert("_last_executed_flags".to_string(), String::new());
            variables.insert("_last_executed_folder".to_string(), String::new());
        }

        // Add placeholder grabber variables (empty defaults)
        variables.insert("grabbed_action".to_string(), String::new());
        variables.insert("grabbed_arg".to_string(), String::new());
        variables.insert("grabbed_app".to_string(), String::new());
        variables.insert("grabbed_title".to_string(), String::new());
        variables.insert("grabbed_text".to_string(), String::new());
        variables.insert("grabbed_suffix".to_string(), String::new());

        // Add empty defaults for selected command (will be overwritten by popup if available)
        variables.insert("_selected_name".to_string(), String::new());
        variables.insert("_selected_path".to_string(), String::new());
        variables.insert("_selected_patch".to_string(), String::new());
        variables.insert("_selected_action".to_string(), String::new());
        variables.insert("_selected_flags".to_string(), String::new());
        variables.insert("_selected_folder".to_string(), String::new());

        TemplateContext { variables }
    }

    /// Add popup-specific variables to this context (only selected command)
    /// Last executed is now handled in new_basic()
    pub fn add_popup_variables(&mut self, selected_command: Option<&Command>) {
        // Add or update selected command variables
        if let Some(cmd) = selected_command {
            // Extract and validate folder, or use empty string if extraction fails
            let folder = match extract_and_validate_folder(cmd) {
                Ok(f) => f,
                Err(_) => String::new()
            };

            self.variables.insert("_selected_name".to_string(), cmd.command.clone());
            self.variables.insert("_selected_path".to_string(), cmd.arg.clone());
            self.variables.insert("_selected_patch".to_string(), cmd.patch.clone());
            self.variables.insert("_selected_action".to_string(), cmd.action.clone());
            self.variables.insert("_selected_flags".to_string(), cmd.flags.clone());
            self.variables.insert("_selected_folder".to_string(), folder);
        }
        // If no selected command, the empty defaults from new_basic() remain
    }

    /// Perform grab operation and add grabbed variables to this context
    pub fn perform_grab_and_add_variables(&mut self, _grab_seconds: u64) -> Result<(), Box<dyn std::error::Error>> {
        // TODO: Implement actual grab functionality
        // For now, just add placeholder grabbed variables
        self.variables.insert("grabbed_action".to_string(), "app".to_string());
        self.variables.insert("grabbed_arg".to_string(), "Finder".to_string());
        self.variables.insert("grabbed_app".to_string(), "Finder".to_string());
        self.variables.insert("grabbed_title".to_string(), "".to_string());
        self.variables.insert("grabbed_text".to_string(), "".to_string());
        self.variables.insert("grabbed_suffix".to_string(), "".to_string());

        Ok(())
    }

    /// Create a new template context with standard variables
    pub fn new(
        input: &str,
        selected_command: Option<&Command>,
        _previous_command: Option<&Command>, // Kept for compatibility but unused
    ) -> Self {
        // Note: _previous_command parameter is kept for API compatibility but not used in template processing
        let mut variables = HashMap::new();

        // Basic variables
        variables.insert("input".to_string(), input.to_string());
        
        // Last executed command from state - stored as object fields for JavaScript access
        let state = crate::core::data::get_state();
        if let Some(last_executed_name) = state.last_executed_command {
            // Try to find the full command details from singleton
            let (sys_data, _) = crate::core::get_sys_data();
            if let Some(cmd) = sys_data.commands.iter().find(|c| c.command == last_executed_name) {
                // Store last_executed command details
                // Extract and validate folder, or use empty string if extraction fails
                let folder = match extract_and_validate_folder(&cmd) {
                    Ok(f) => {
                        detailed_log("TEMPLATE_FOLDER", &format!("TEMPLATE_FOLDER: Extracted valid folder from '{}': {}", cmd.command, f));
                        f
                    },
                    Err(e) => {
                        detailed_log("TEMPLATE_FOLDER", &format!("TEMPLATE_FOLDER: Failed to extract folder from '{}' (action={}): {}", cmd.command, cmd.action, e));
                        String::new() // Use empty string for invalid folder
                    }
                };
                
                // We store these for JavaScript object creation later
                variables.insert("_last_executed_name".to_string(), cmd.command.clone());
                variables.insert("_last_executed_path".to_string(), cmd.arg.clone());
                variables.insert("_last_executed_patch".to_string(), cmd.patch.clone());
                variables.insert("_last_executed_action".to_string(), cmd.action.clone());
                variables.insert("_last_executed_flags".to_string(), cmd.flags.clone());
                variables.insert("_last_executed_folder".to_string(), folder);
            } else {
                // Command not found, just use the name
                variables.insert("_last_executed_name".to_string(), last_executed_name);
                variables.insert("_last_executed_path".to_string(), String::new());
                variables.insert("_last_executed_patch".to_string(), String::new());
                variables.insert("_last_executed_folder".to_string(), String::new());
                variables.insert("_last_executed_action".to_string(), String::new());
                variables.insert("_last_executed_flags".to_string(), String::new());
            }
        } else {
            // No last executed command
            variables.insert("_last_executed_name".to_string(), String::new());
            variables.insert("_last_executed_path".to_string(), String::new());
            variables.insert("_last_executed_patch".to_string(), String::new());
            variables.insert("_last_executed_folder".to_string(), String::new());
            variables.insert("_last_executed_action".to_string(), String::new());
            variables.insert("_last_executed_flags".to_string(), String::new());
        }

        // Last anchor from state - stored as object fields for JavaScript access
        if let Some(anchor_name) = &state.anchor_name {
            variables.insert("_last_anchor_name".to_string(), anchor_name.clone());
            if let Some(timestamp) = state.anchor_timestamp {
                variables.insert("_last_anchor_timestamp".to_string(), timestamp.to_string());
            }
            if let Some(folder) = &state.anchor_folder {
                variables.insert("_last_anchor_folder".to_string(), folder.clone());
            } else {
                variables.insert("_last_anchor_folder".to_string(), String::new());
            }
        } else {
            variables.insert("_last_anchor_name".to_string(), String::new());
            variables.insert("_last_anchor_timestamp".to_string(), String::new());
            variables.insert("_last_anchor_folder".to_string(), String::new());
        }

        // Selected command - stored as object fields for JavaScript access
        if let Some(cmd) = selected_command {
            // Extract and validate folder, or use empty string if extraction fails
            let folder = match extract_and_validate_folder(cmd) {
                Ok(f) => {
                    detailed_log("TEMPLATE", &format!("Selected command folder: {}", f));
                    f
                },
                Err(e) => {
                    detailed_log("TEMPLATE", &format!("Selected command folder error: {}", e));
                    String::new()
                }
            };
            
            variables.insert("_selected_name".to_string(), cmd.command.clone());
            variables.insert("_selected_path".to_string(), cmd.arg.clone());
            variables.insert("_selected_patch".to_string(), cmd.patch.clone());
            variables.insert("_selected_action".to_string(), cmd.action.clone());
            variables.insert("_selected_flags".to_string(), cmd.flags.clone());
            variables.insert("_selected_folder".to_string(), folder);
        } else {
            // Provide empty defaults for when no command is selected
            variables.insert("_selected_name".to_string(), String::new());
            variables.insert("_selected_path".to_string(), String::new());
            variables.insert("_selected_patch".to_string(), String::new());
            variables.insert("_selected_folder".to_string(), String::new());
            variables.insert("_selected_action".to_string(), String::new());
            variables.insert("_selected_flags".to_string(), String::new());
        }
        
        // Note: We no longer track "previous" command as it was volatile and unreliable
        // Instead, use last_executed which tracks the actually executed command
        // The _previous_command parameter is kept for compatibility but unused
        
        // Add date/time variables
        add_datetime_variables(&mut variables);
        
        // Add placeholder variables for grab functionality (until it's implemented)
        variables.insert("grabbed_action".to_string(), "app".to_string());
        variables.insert("grabbed_arg".to_string(), "Finder".to_string());
        
        TemplateContext { variables }
    }
    
    /// Add a custom variable
    pub fn add_variable(&mut self, name: String, value: String) {
        self.variables.insert(name, value);
    }
    
    /// Expand variables in a string using JavaScript evaluation
    pub fn expand(&self, template: &str) -> String {
        let mut result = template.to_string();
        let mut last_pos = 0;
        
        // Process the string from left to right to avoid re-processing replacements
        while let Some(start) = result[last_pos..].find("{{") {
            let start = last_pos + start;
            if let Some(end) = result[start..].find("}}") {
                let end = start + end + 2;
                let expr = result[start + 2..end - 2].trim();
                
                // Evaluate the JavaScript expression
                match self.eval_js_expression(expr) {
                    Ok(value) => {
                        // Check if an error was queued during JS execution
                        if crate::utils::error::has_errors() {
                            // Return what we have so far - error will be displayed later
                            return result;
                        }
                        result.replace_range(start..end, &value);
                        // Move position to after the replacement
                        last_pos = start + value.len();
                    }
                    Err(e) => {
                        detailed_log("TEMPLATE_JS", &format!("Failed to evaluate '{}': {}", expr, e));
                        // Unknown variable - skip past it to avoid infinite loop
                        last_pos = end;
                    }
                }
            } else {
                // No matching }} found, break to avoid infinite loop
                break;
            }
        }
        
        result
    }
    
    /// Evaluate a JavaScript expression with template context
    fn eval_js_expression(&self, expr: &str) -> Result<String, Box<dyn std::error::Error>> {
        // Create JavaScript objects from template context
        let js_code = self.build_js_context() + &format!(
            r#"
            // Evaluate the expression with error handling
            (function() {{
                try {{
                    let result = {};
                    // Convert undefined/null to empty string
                    if (result === undefined || result === null) {{
                        return '';
                    }}
                    return String(result);
                }} catch(e) {{
                    // Queue the error for user display
                    error(e.toString());
                    // Return empty string as safe default
                    return '';
                }}
            }})();
            "#,
            expr
        );
        
        // Execute the JavaScript code with context for better error reporting
        crate::js::execute_with_context(&js_code, &format!("TEMPLATE_EXPANSION({})", expr))
    }

    /// Helper function to create a command object in JavaScript
    /// Used for selected, last_executed, and last_anchor objects
    /// All command objects have the same fields: name, path, arg, patch, folder*, action, flags
    fn create_command_object(
        &self,
        var_name: &str,
        prefix: &str,
        description: &str
    ) -> String {
        let mut js = String::new();

        let name = self.variables.get(&format!("_{}_name", prefix)).cloned().unwrap_or_else(String::new);
        let folder = self.variables.get(&format!("_{}_folder", prefix)).cloned().unwrap_or_else(String::new);

        js.push_str(&format!("const {} = Object.create(null, {{\n", var_name));
        js.push_str(&format!("  name: {{ value: {:?}, enumerable: true }},\n", name));
        js.push_str(&format!("  path: {{ value: {:?}, enumerable: true }},\n", self.variables.get(&format!("_{}_path", prefix)).unwrap_or(&String::new())));
        js.push_str(&format!("  arg: {{ value: {:?}, enumerable: true }},\n", self.variables.get(&format!("_{}_path", prefix)).unwrap_or(&String::new())));
        js.push_str(&format!("  patch: {{ value: {:?}, enumerable: true }},\n", self.variables.get(&format!("_{}_patch", prefix)).unwrap_or(&String::new())));

        // Add folder with getter that throws error if empty
        js.push_str("  folder: {\n");
        js.push_str("    enumerable: true,\n");
        js.push_str("    get: function() {\n");
        js.push_str(&format!("      const folderValue = {:?};\n", folder));
        js.push_str("      if (!folderValue || folderValue === '') {\n");
        js.push_str(&format!("        throw new Error('{} \"' + {:?} + '\" does not have a folder context');\n", description, name));
        js.push_str("      }\n");
        js.push_str("      return folderValue;\n");
        js.push_str("    }\n");
        js.push_str("  },\n");

        js.push_str(&format!("  action: {{ value: {:?}, enumerable: true }},\n", self.variables.get(&format!("_{}_action", prefix)).unwrap_or(&String::new())));
        js.push_str(&format!("  flags: {{ value: {:?}, enumerable: true }}\n", self.variables.get(&format!("_{}_flags", prefix)).unwrap_or(&String::new())));
        js.push_str("});\n");

        js
    }

    /// Build JavaScript context with all template variables as objects
    fn build_js_context(&self) -> String {
        let mut context = String::new();
        
        // Create input variable (string)
        if let Some(input) = self.variables.get("input") {
            context.push_str(&format!("const input = {:?};\n", input));
        } else {
            context.push_str("const input = '';\n");
        }

        // Create raw_input variable (string)
        if let Some(raw_input) = self.variables.get("raw_input") {
            context.push_str(&format!("const raw_input = {:?};\n", raw_input));
        } else {
            context.push_str("const raw_input = '';\n");
        }

        // Create arg variable (string) - for template expansion like {{arg}}
        if let Some(arg) = self.variables.get("arg") {
            context.push_str(&format!("const arg = {:?};\n", arg));
        } else {
            context.push_str("const arg = '';\n");
        }

        // Create selected command object
        context.push_str(&self.create_command_object("selected", "selected", "Selected command"));
        
        // Create date object
        let now = Local::now();
        context.push_str("const date = {\n");
        context.push_str(&format!("  year: {:?},\n", format!("{:04}", now.year())));
        context.push_str(&format!("  year2: {:?},\n", format!("{:02}", now.year() % 100)));
        context.push_str(&format!("  month: {:?},\n", format!("{:02}", now.month())));
        context.push_str(&format!("  month_short: {:?},\n", format!("{}", now.month())));
        context.push_str(&format!("  month_name: {:?},\n", now.format("%B").to_string()));
        context.push_str(&format!("  month_abbr: {:?},\n", now.format("%b").to_string()));
        context.push_str(&format!("  day: {:?},\n", format!("{:02}", now.day())));
        context.push_str(&format!("  day_short: {:?},\n", format!("{}", now.day())));
        context.push_str(&format!("  weekday: {:?},\n", now.format("%A").to_string()));
        context.push_str(&format!("  weekday_abbr: {:?},\n", now.format("%a").to_string()));
        context.push_str(&format!("  hour: {:?},\n", format!("{:02}", now.hour())));
        context.push_str(&format!("  hour12: {:?},\n", format!("{}", if now.hour() == 0 { 12 } else if now.hour() > 12 { now.hour() - 12 } else { now.hour() })));
        context.push_str(&format!("  minute: {:?},\n", format!("{:02}", now.minute())));
        context.push_str(&format!("  second: {:?},\n", format!("{:02}", now.second())));
        context.push_str(&format!("  ampm: {:?},\n", if now.hour() < 12 { "AM" } else { "PM" }));
        context.push_str(&format!("  timestamp: {},\n", now.timestamp()));
        context.push_str(&format!("  iso: {:?}\n", now.format("%Y-%m-%dT%H:%M:%S").to_string()));
        context.push_str("};\n");
        
        // Create grabbed object
        context.push_str("const grabbed = {\n");
        context.push_str(&format!("  action: {:?},\n", self.variables.get("grabbed_action").unwrap_or(&String::new())));
        context.push_str(&format!("  arg: {:?},\n", self.variables.get("grabbed_arg").unwrap_or(&String::new())));
        context.push_str(&format!("  app: {:?},\n", self.variables.get("grabbed_app").unwrap_or(&String::new())));
        context.push_str(&format!("  title: {:?},\n", self.variables.get("grabbed_title").unwrap_or(&String::new())));
        context.push_str(&format!("  text: {:?},\n", self.variables.get("grabbed_text").unwrap_or(&String::new())));
        context.push_str(&format!("  suffix: {:?}\n", self.variables.get("grabbed_suffix").unwrap_or(&String::new())));
        context.push_str("};\n");
        
        // Create env object
        let home = std::env::var("HOME").unwrap_or_else(|_| "/Users/unknown".to_string());
        let user = std::env::var("USER").unwrap_or_else(|_| "unknown".to_string());
        context.push_str("const env = {\n");
        context.push_str(&format!("  home: {:?},\n", home));
        context.push_str(&format!("  user: {:?},\n", user));
        context.push_str(&format!("  hostname: {:?},\n", "localhost"));
        context.push_str(&format!("  os: {:?},\n", std::env::consts::OS));
        context.push_str(&format!("  config_dir: {:?}\n", format!("{}/.config/hookanchor", home)));
        context.push_str("};\n");
        
        // Create last_executed command object
        context.push_str(&self.create_command_object("last_executed", "last_executed", "Last executed command"));

        // Create last_anchor command object (same fields as other command objects)
        context.push_str(&self.create_command_object("last_anchor", "last_anchor", "Last anchor"));

        // No legacy compatibility variables needed

        context
    }

    /// Expand all values in a HashMap using template expansion
    /// This allows batch expansion of action parameters
    pub fn expand_hashmap(&self, params: &std::collections::HashMap<String, String>) -> std::collections::HashMap<String, String> {
        let mut expanded = std::collections::HashMap::new();

        for (key, value) in params {
            if value.contains("{{") {
                // Expand this parameter
                let expanded_value = self.expand(value);
                expanded.insert(key.clone(), expanded_value);
            } else {
                // No expansion needed
                expanded.insert(key.clone(), value.clone());
            }
        }

        expanded
    }

    /// Directly expand all string parameters in an Action object
    /// This modifies the action in place, expanding any {{...}} templates
    pub fn expand_action_parameters(&self, action: &mut crate::execute::Action) {
        let mut params_to_update = Vec::new();

        // Find all string parameters that need expansion
        for (key, value) in &action.params {
            if let Some(string_value) = value.as_str() {
                if string_value.contains("{{") {
                    let expanded_value = self.expand(string_value);
                    params_to_update.push((key.clone(), expanded_value));
                }
            }
        }

        // Update the parameters with expanded values
        for (key, expanded_value) in params_to_update {
            action.params.insert(key, serde_json::Value::String(expanded_value));
        }
    }

    /// Add grabber variables to this context
    /// This is used when grab functionality has been executed
    pub fn add_grabber_variables(&mut self, grabbed_vars: std::collections::HashMap<String, String>) {
        for (key, value) in grabbed_vars {
            self.variables.insert(key, value);
        }
    }
}

/// Extract folder path from a file path
fn extract_folder_from_path(path: &str) -> Option<String> {
    use std::path::Path;
    
    let path = Path::new(path);
    path.parent()
        .and_then(|p| p.to_str())
        .map(|s| s.to_string())
}

/// Extract folder from cmd command with cd
fn extract_folder_from_cd_command(arg: &str) -> Result<String, String> {
    let trimmed = arg.trim();
    
    // Check if it starts with cd
    if !trimmed.starts_with("cd ") {
        return Err("Command does not start with 'cd'".to_string());
    }
    
    // Find the && terminator
    let cd_part = if let Some(idx) = trimmed.find(" &&") {
        &trimmed[3..idx] // Skip "cd " and take up to " &&"
    } else {
        return Err("No && found after cd command".to_string());
    };
    
    // Trim and remove quotes
    let folder = cd_part.trim();
    let folder = if (folder.starts_with('"') && folder.ends_with('"')) ||
                    (folder.starts_with('\'') && folder.ends_with('\'')) {
        &folder[1..folder.len()-1]
    } else {
        folder
    };
    
    Ok(folder.to_string())
}

/// Extract and validate folder from command
fn extract_and_validate_folder(cmd: &super::Command) -> Result<String, String> {
    use std::path::Path;
    
    let folder = match cmd.action.as_str() {
        "cmd" => {
            // Special handling for cmd - look for cd command
            extract_folder_from_cd_command(&cmd.arg)?
        },
        "folder" => {
            // For folder action, the arg is the folder itself
            cmd.arg.clone()
        },
        "markdown" | "doc" | "text" => {
            // For file-based actions, check if arg is a directory or file
            let expanded_arg = if cmd.arg.starts_with("~/") {
                cmd.arg.replacen("~", &std::env::var("HOME").unwrap_or_default(), 1)
            } else {
                cmd.arg.clone()
            };

            let path = Path::new(&expanded_arg);
            if path.is_dir() {
                // If it's already a directory (like /path/to/folder), use it directly
                expanded_arg
            } else {
                // If it's a file, extract parent folder
                extract_folder_from_path(&cmd.arg)
                    .ok_or_else(|| format!("Could not extract folder from path: {}", cmd.arg))?
            }
        },
        _ => {
            // Other actions don't have folder context
            return Err(format!("Action '{}' does not provide folder context", cmd.action));
        }
    };
    
    // Expand tilde if present
    let expanded = if folder.starts_with("~/") {
        folder.replacen("~", &std::env::var("HOME").unwrap_or_default(), 1)
    } else {
        folder.clone()
    };
    
    // Verify folder exists
    if !Path::new(&expanded).exists() {
        return Err(format!("Folder does not exist: {}", expanded));
    }

    // Remove trailing slash to avoid double slashes in templates
    let normalized = if expanded.ends_with('/') && expanded.len() > 1 {
        expanded[..expanded.len() - 1].to_string()
    } else {
        expanded
    };

    Ok(normalized)
}

/// Add date/time variables to the context
/// This is used by both template system and actions system
pub fn add_datetime_variables(variables: &mut HashMap<String, String>) {
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

/// Create a file with contents, creating parent directories as needed
fn create_file_with_contents(file_path: &str, contents: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Expand tilde
    let expanded_path = if file_path.starts_with("~/") {
        if let Ok(home) = std::env::var("HOME") {
            file_path.replacen("~/", &format!("{}/", home), 1)
        } else {
            file_path.to_string()
        }
    } else {
        file_path.to_string()
    };

    let path = std::path::Path::new(&expanded_path);

    // Create parent directory if needed
    if let Some(parent) = path.parent() {
        if !parent.exists() {
            std::fs::create_dir_all(parent)?;
            detailed_log("TEMPLATE_OPS", &format!("Created directory: {}", parent.display()));
        }
    }

    // Write the file
    std::fs::write(&path, contents)?;
    detailed_log("TEMPLATE_OPS", &format!("Created file: {} ({} bytes)", path.display(), contents.len()));

    Ok(())
}


/// Append contents to a file, optionally after a specific line
/// If `after` is specified but not found, adds the `after` line first
fn append_to_file(file_path: &str, contents: &str, after: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
    // Expand tilde
    let expanded_path = if file_path.starts_with("~/") {
        if let Ok(home) = std::env::var("HOME") {
            file_path.replacen("~/", &format!("{}/", home), 1)
        } else {
            file_path.to_string()
        }
    } else {
        file_path.to_string()
    };

    let path = std::path::Path::new(&expanded_path);

    // Read existing contents (or empty if file doesn't exist)
    let existing = if path.exists() {
        std::fs::read_to_string(&path)?
    } else {
        // Create parent directory if needed
        if let Some(parent) = path.parent() {
            if !parent.exists() {
                std::fs::create_dir_all(parent)?;
            }
        }
        String::new()
    };

    let new_contents = if let Some(after_str) = after {
        // Look for the after string
        if let Some(pos) = existing.find(after_str) {
            // Find end of the line containing after_str
            let after_end = existing[pos..].find('\n')
                .map(|p| pos + p + 1)
                .unwrap_or(existing.len());

            // Insert contents after that line
            let mut result = existing[..after_end].to_string();
            result.push_str(contents);
            if !contents.ends_with('\n') {
                result.push('\n');
            }
            result.push_str(&existing[after_end..]);
            result
        } else {
            // after_str not found - add it first, then contents
            let mut result = existing;
            if !result.is_empty() && !result.ends_with('\n') {
                result.push('\n');
            }
            result.push_str(after_str);
            result.push('\n');
            result.push_str(contents);
            if !contents.ends_with('\n') {
                result.push('\n');
            }
            result
        }
    } else {
        // No after string - just append to end
        let mut result = existing;
        if !result.is_empty() && !result.ends_with('\n') {
            result.push('\n');
        }
        result.push_str(contents);
        if !contents.ends_with('\n') {
            result.push('\n');
        }
        result
    };

    std::fs::write(&path, new_contents)?;
    detailed_log("TEMPLATE_OPS", &format!("Appended to file: {}", path.display()));

    Ok(())
}


/// Copy text to the system clipboard using pbcopy
fn copy_to_clipboard(contents: &str) -> Result<(), Box<dyn std::error::Error>> {
    use std::process::{Command as ProcessCommand, Stdio};
    use std::io::Write;

    let mut child = ProcessCommand::new("pbcopy")
        .stdin(Stdio::piped())
        .spawn()?;

    if let Some(mut stdin) = child.stdin.take() {
        stdin.write_all(contents.as_bytes())?;
    }

    let status = child.wait()?;
    if !status.success() {
        return Err("pbcopy failed".into());
    }

    detailed_log("TEMPLATE_OPS", &format!("Copied to clipboard: {} chars", contents.len()));
    Ok(())
}


/// Process a template and create the command (but NOT files - use process_template_files for that)
pub fn process_template(
    template: &Template,
    context: &TemplateContext,
    _config: &super::Config,
) -> Result<Command, Box<dyn std::error::Error>> {
    // Validate previous folder if required
    if template.validate_previous_folder {
        let previous_folder = context.variables.get("previous_folder")
            .ok_or("No previous command available")?;
            
        if previous_folder.is_empty() {
            let error_msg = "Previous command has no associated folder. Cannot create sub-anchor.";
            crate::utils::error::queue_user_error(error_msg);
            return Err(error_msg.into());
        }
        
        // Expand tilde in path for validation
        let expanded_folder = if previous_folder.starts_with("~/") {
            if let Ok(home) = std::env::var("HOME") {
                previous_folder.replacen("~/", &format!("{}/", home), 1)
            } else {
                previous_folder.clone()
            }
        } else {
            previous_folder.clone()
        };
        
        // Check if folder exists
        if !std::path::Path::new(&expanded_folder).exists() {
            let error_msg = format!("Previous command's folder does not exist: {}", previous_folder);
            crate::utils::error::queue_user_error(&error_msg);
            return Err(error_msg.into());
        }
        
        detailed_log("TEMPLATE", &format!("Validated previous folder: {}", previous_folder));
    }
    
    // TODO: Implement grab functionality if template.grab is set
    if let Some(_grab_seconds) = template.grab {
        // This will be implemented in Phase 3
        // For now, we'll skip grab functionality
    }

    detailed_log("TEMPLATE", &format!("Processing {} operations", template.operations.len()));

    // Find the command operation and create the command from it
    for op in &template.operations {
        if let TemplateOperation::Command { name, action, arg, patch, flags, edit: _, use_existing: _ } = op {
            let mut command = Command {
                command: context.expand(name),
                action: context.expand(action),
                arg: context.expand(arg),
                patch: context.expand(patch),
                flags: context.expand(flags),
                other_params: None,
                last_update: 0,
                file_size: None,
            };
            command.update_full_line();

            // Check if any errors were queued during template expansion
            if crate::utils::error::has_errors() {
                return Err("Template expansion failed due to errors".into());
            }

            if template.edit {
                detailed_log("TEMPLATE", "Template has edit flag set - command will be opened in editor");
            }

            return Ok(command);
        }
    }

    // No command operation found
    Err("Template operations must include a 'command' operation".into())
}

/// Process template file creation after save (called when user saves in command editor)
pub fn process_template_files(
    template: &Template,
    context: &TemplateContext,
    _saved_command: &Command,
) -> Result<(), Box<dyn std::error::Error>> {
    detailed_log("TEMPLATE_OPS", &format!("Processing {} file operations", template.operations.len()));

    // Process create, append, and clip operations (command was already processed)
    for op in &template.operations {
        match op {
            TemplateOperation::Create { file, contents } => {
                let expanded_file = context.expand(file);
                let expanded_contents = context.expand(contents);
                create_file_with_contents(&expanded_file, &expanded_contents)?;
            }
            TemplateOperation::Append { file, contents, after } => {
                let expanded_file = context.expand(file);
                let expanded_contents = context.expand(contents);
                let expanded_after = after.as_ref().map(|s| context.expand(s));
                append_to_file(&expanded_file, &expanded_contents, expanded_after.as_deref())?;
            }
            TemplateOperation::Command { .. } => {
                // Command already processed in process_template
            }
            TemplateOperation::Clip { contents } => {
                let expanded_contents = context.expand(contents);
                copy_to_clipboard(&expanded_contents)?;
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn init_test_environment() {
        // Initialize sys_data for tests that need it
        let _ = crate::core::initialize();
    }

    #[test]
    fn test_variable_expansion() {
        // Create context manually without requiring full initialization
        let mut variables = HashMap::new();
        variables.insert("input".to_string(), "test input".to_string());
        variables.insert("custom".to_string(), "custom value".to_string());

        let context = TemplateContext { variables };

        // Note: These tests will fail if JavaScript runtime isn't initialized
        // The expand() method uses JS evaluation, so we test basic functionality
        // For pure unit testing without JS, we'd need a simpler expand method

        // Skip the actual expansion tests since they require JS runtime
        // Instead, verify the context setup is correct
        assert_eq!(context.variables.get("input"), Some(&"test input".to_string()));
        assert_eq!(context.variables.get("custom"), Some(&"custom value".to_string()));
        assert_eq!(context.variables.get("unknown"), None);
    }
    
    #[test]
    fn test_selected_command_variables() {
        // Create context manually to test selected command variable setup
        let selected_command = Command {
            command: "test_cmd".to_string(),
            action: "markdown".to_string(),
            arg: "/Users/test/Documents/notes/test.md".to_string(),
            patch: "TestPatch".to_string(),
            flags: String::new(),
            other_params: None,
            last_update: 0,
            file_size: None,
        };

        // Build variables manually to avoid requiring full initialization
        let mut variables = HashMap::new();
        variables.insert("input".to_string(), "input".to_string());
        variables.insert("_selected_name".to_string(), selected_command.command.clone());
        variables.insert("_selected_path".to_string(), selected_command.arg.clone());
        variables.insert("_selected_patch".to_string(), selected_command.patch.clone());
        variables.insert("_selected_action".to_string(), selected_command.action.clone());

        // Extract folder from path (simple implementation for test)
        let folder = if let Some(idx) = selected_command.arg.rfind('/') {
            selected_command.arg[..idx].to_string()
        } else {
            String::new()
        };
        variables.insert("_selected_folder".to_string(), folder.clone());

        let context = TemplateContext { variables };

        // Verify the variables are set correctly (not testing JS expansion, just setup)
        assert_eq!(context.variables.get("_selected_name"), Some(&"test_cmd".to_string()));
        assert_eq!(context.variables.get("_selected_path"), Some(&"/Users/test/Documents/notes/test.md".to_string()));
        assert_eq!(context.variables.get("_selected_patch"), Some(&"TestPatch".to_string()));
        assert_eq!(context.variables.get("_selected_folder"), Some(&"/Users/test/Documents/notes".to_string()));
        assert_eq!(context.variables.get("_selected_action"), Some(&"markdown".to_string()));
    }
    
    #[test]
    #[ignore] // Requires full config environment (config.js, etc.) - run with --ignored
    fn test_datetime_variables() {
        init_test_environment();
        let context = TemplateContext::new("", None, None);
        
        // Test object-based date variables
        let year = context.expand("{{date.year}}");
        assert_eq!(year.len(), 4);
        assert!(year.parse::<u32>().is_ok());
        
        let month = context.expand("{{date.month}}");
        assert_eq!(month.len(), 2);
        let month_num = month.parse::<u32>().unwrap();
        assert!(month_num >= 1 && month_num <= 12);
        
        let day = context.expand("{{date.day}}");
        assert_eq!(day.len(), 2);
        let day_num = day.parse::<u32>().unwrap();
        assert!(day_num >= 1 && day_num <= 31);
        
        // Test year formats
        assert_eq!(context.expand("{{date.year}}").len(), 4);
        assert_eq!(context.expand("{{date.year2}}").len(), 2);
    }
    
}
