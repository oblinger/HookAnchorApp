//! Template Creation System
//! 
//! Provides a flexible template-based system for creating new commands
//! with variable expansion and customizable fields.

use std::collections::HashMap;
use chrono::{Local, Datelike, Timelike};
use serde::{Deserialize, Serialize};
use crate::core::Command;
use crate::core::key_processing::Keystroke;

/// A template for creating new commands
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Template {
    /// The name of the command to be created (supports variable expansion)
    pub name: String,
    
    /// The action for the command (supports variable expansion)
    pub action: String,
    
    /// The argument string for the command (supports variable expansion)
    pub arg: String,
    
    /// The patch string for the command (supports variable expansion)
    #[serde(default)]
    pub patch: String,
    
    /// The flags string for the command (supports variable expansion)
    #[serde(default)]
    pub flags: String,
    
    /// Optional key that triggers this template (e.g., "Minus", "N", etc.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    
    /// Keystroke for efficient matching (computed from key field)
    #[serde(skip)]
    pub keystroke: Option<Keystroke>,
    
    /// If true, open command editor before creating
    #[serde(default)]
    pub edit: bool,
    
    /// If true, use existing command with same name if it exists (case-insensitive)
    /// If false or if no matching command exists, create a new command
    #[serde(default)]
    pub use_existing: bool,
    
    /// Optional folder path to create (supports variable expansion)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<String>,
    
    /// Optional file contents to create (supports variable expansion)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contents: Option<String>,
    
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
        let state = crate::core::state::load_state();
        if let Some(last_executed_name) = state.last_executed_command {
            // Try to find the full command details
            let commands = crate::core::commands::load_commands();
            if let Some(cmd) = commands.iter().find(|c| c.command == last_executed_name) {
                // Store last_executed command details
                // Extract and validate folder, or use empty string if extraction fails
                let folder = match extract_and_validate_folder(&cmd) {
                    Ok(f) => {
                        crate::utils::detailed_log("TEMPLATE_FOLDER", &format!("TEMPLATE_FOLDER: Extracted valid folder from '{}': {}", cmd.command, f));
                        f
                    },
                    Err(e) => {
                        crate::utils::detailed_log("TEMPLATE_FOLDER", &format!("TEMPLATE_FOLDER: Failed to extract folder from '{}' (action={}): {}", cmd.command, cmd.action, e));
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
        
        // Selected command - stored as object fields for JavaScript access
        if let Some(cmd) = selected_command {
            // Extract and validate folder, or use empty string if extraction fails
            let folder = match extract_and_validate_folder(cmd) {
                Ok(f) => {
                    crate::utils::detailed_log("TEMPLATE", &format!("Selected command folder: {}", f));
                    f
                },
                Err(e) => {
                    crate::utils::detailed_log("TEMPLATE", &format!("Selected command folder error: {}", e));
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
                        crate::utils::detailed_log("TEMPLATE_JS", &format!("Failed to evaluate '{}': {}", expr, e));
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
        
        // Execute the JavaScript code
        crate::js::execute(&js_code)
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
        
        // Create selected object with getter for folder that throws on empty
        let selected_name = self.variables.get("_selected_name").cloned().unwrap_or_else(String::new);
        let selected_folder = self.variables.get("_selected_folder").cloned().unwrap_or_else(String::new);
        
        context.push_str("const selected = Object.create(null, {\n");
        context.push_str(&format!("  name: {{ value: {:?}, enumerable: true }},\n", selected_name));
        context.push_str(&format!("  path: {{ value: {:?}, enumerable: true }},\n", self.variables.get("_selected_path").unwrap_or(&String::new())));
        context.push_str(&format!("  arg: {{ value: {:?}, enumerable: true }},\n", self.variables.get("_selected_path").unwrap_or(&String::new())));
        context.push_str(&format!("  patch: {{ value: {:?}, enumerable: true }},\n", self.variables.get("_selected_patch").unwrap_or(&String::new())));
        
        // Add folder with getter that throws error if empty
        context.push_str("  folder: {\n");
        context.push_str("    enumerable: true,\n");
        context.push_str("    get: function() {\n");
        context.push_str(&format!("      const folderValue = {:?};\n", selected_folder));
        context.push_str("      if (!folderValue || folderValue === '') {\n");
        context.push_str(&format!("        throw new Error('Selected command \"' + {:?} + '\" does not have a folder context');\n", selected_name));
        context.push_str("      }\n");
        context.push_str("      return folderValue;\n");
        context.push_str("    }\n");
        context.push_str("  },\n");
        
        context.push_str(&format!("  action: {{ value: {:?}, enumerable: true }},\n", self.variables.get("_selected_action").unwrap_or(&String::new())));
        context.push_str(&format!("  flags: {{ value: {:?}, enumerable: true }}\n", self.variables.get("_selected_flags").unwrap_or(&String::new())));
        context.push_str("});\n");
        
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
        
        // Create last_executed object with getter for folder that throws on empty
        let last_name = self.variables.get("_last_executed_name").cloned().unwrap_or_else(String::new);
        let last_folder = self.variables.get("_last_executed_folder").cloned().unwrap_or_else(String::new);
        
        context.push_str("const last_executed = Object.create(null, {\n");
        context.push_str(&format!("  name: {{ value: {:?}, enumerable: true }},\n", last_name));
        context.push_str(&format!("  path: {{ value: {:?}, enumerable: true }},\n", self.variables.get("_last_executed_path").unwrap_or(&String::new())));
        context.push_str(&format!("  arg: {{ value: {:?}, enumerable: true }},\n", self.variables.get("_last_executed_path").unwrap_or(&String::new())));
        context.push_str(&format!("  patch: {{ value: {:?}, enumerable: true }},\n", self.variables.get("_last_executed_patch").unwrap_or(&String::new())));
        
        // Add folder with getter that throws error if empty
        context.push_str("  folder: {\n");
        context.push_str("    enumerable: true,\n");
        context.push_str("    get: function() {\n");
        context.push_str(&format!("      const folderValue = {:?};\n", last_folder));
        context.push_str("      if (!folderValue || folderValue === '') {\n");
        context.push_str(&format!("        throw new Error('Last executed command \"' + {:?} + '\" does not have a folder context');\n", last_name));
        context.push_str("      }\n");
        context.push_str("      return folderValue;\n");
        context.push_str("    }\n");
        context.push_str("  },\n");
        
        context.push_str(&format!("  action: {{ value: {:?}, enumerable: true }},\n", self.variables.get("_last_executed_action").unwrap_or(&String::new())));
        context.push_str(&format!("  flags: {{ value: {:?}, enumerable: true }}\n", self.variables.get("_last_executed_flags").unwrap_or(&String::new())));
        context.push_str("});\n");
        
        // No legacy compatibility variables needed
        
        context
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
        "markdown" | "anchor" | "doc" | "text" => {
            // For file-based actions, extract parent folder
            extract_folder_from_path(&cmd.arg)
                .ok_or_else(|| format!("Could not extract folder from path: {}", cmd.arg))?
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
    
    Ok(expanded)
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

/// Create a command from a template
pub fn create_command_from_template(
    template: &Template,
    context: &TemplateContext,
) -> Command {
    let mut command = Command {
        command: context.expand(&template.name),
        action: context.expand(&template.action),
        arg: context.expand(&template.arg),
        patch: context.expand(&template.patch),
        flags: context.expand(&template.flags),
    };
    command.update_full_line();
    command
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
        
        crate::utils::detailed_log("TEMPLATE", &format!("Validated previous folder: {}", previous_folder));
    }
    
    // TODO: Implement grab functionality if template.grab is set
    if let Some(_grab_seconds) = template.grab {
        // This will be implemented in Phase 3
        // For now, we'll skip grab functionality
    }
    
    // Create the command
    let mut command = create_command_from_template(template, context);
    
    // Check if any errors were queued during template expansion
    if crate::utils::error::has_errors() {
        // Return an error - the queued error will be displayed by the UI
        return Err("Template expansion failed due to errors".into());
    }
    
    // NOTE: File creation is now handled in process_template_files() which should be called
    // after the user saves in the command editor
    
    // Update the full_line
    command.update_full_line();
    
    // Log if template has edit flag (but don't modify the command)
    if template.edit {
        crate::utils::detailed_log("TEMPLATE", "Template has edit flag set - command will be opened in editor");
    }
    
    Ok(command)
}

/// Process template file creation after save (called when user saves in command editor)
pub fn process_template_files(
    template: &Template,
    context: &TemplateContext,
    saved_command: &Command,
) -> Result<(), Box<dyn std::error::Error>> {
    // Create file with contents if specified
    if let Some(contents_template) = &template.contents {
        let contents = context.expand(contents_template);
        
        // Determine file path based on template.file setting
        let file_path_str = if let Some(ref file_template) = template.file {
            // Check for special "true" value to use saved command's arg
            if file_template == "true" {
                // Use the saved command's arg as the file path
                crate::utils::detailed_log("TEMPLATE", "Using saved command arg as file path (file: \"true\")");
                saved_command.arg.clone()
            } else {
                // Use the expanded file template
                context.expand(file_template)
            }
        } else {
            // No file field specified, use the saved command's arg
            saved_command.arg.clone()
        };
        
        // Debug logging
        crate::utils::detailed_log("TEMPLATE", &format!("File path from template: {}", file_path_str));
        
        // Expand tilde in the file path
        let expanded_path = if file_path_str.starts_with("~/") {
            if let Ok(home) = std::env::var("HOME") {
                file_path_str.replacen("~/", &format!("{}/", home), 1)
            } else {
                file_path_str.clone()
            }
        } else {
            file_path_str.clone()
        };
        
        let file_path = std::path::Path::new(&expanded_path);
        
        // Create parent directory if it doesn't exist
        if let Some(parent) = file_path.parent() {
            crate::utils::detailed_log("TEMPLATE", &format!("Creating parent dir: {}", parent.display()));
            create_folder_if_needed(&parent.to_string_lossy())?;
        }
        
        crate::utils::detailed_log("TEMPLATE", &format!("Writing file: {}", file_path.display()));
        
        // Check if file exists and might be read-only
        if file_path.exists() {
            if let Ok(metadata) = std::fs::metadata(&file_path) {
                if metadata.permissions().readonly() {
                    crate::utils::log(&format!("File '{}' is read-only, attempting to make it writable", file_path.display()));
                    // Try to make the file writable
                    let mut perms = metadata.permissions();
                    perms.set_readonly(false);
                    if let Err(e) = std::fs::set_permissions(&file_path, perms) {
                        crate::utils::log_error(&format!("Failed to remove read-only flag from '{}': {}", file_path.display(), e));
                        // Continue anyway - the write might still work on some systems
                    } else {
                        crate::utils::detailed_log("SYSTEM", &format!("Successfully removed read-only flag from '{}'", file_path.display()));
                    }
                }
            }
        }
        
        // Write the file
        match std::fs::write(&file_path, &contents) {
            Ok(_) => {
                crate::utils::detailed_log("TEMPLATE", &format!("TEMPLATE: Created file '{}' ({} bytes)", file_path.display(), contents.len()));
            }
            Err(e) => {
                let error_msg = format!("Cannot write to file '{}': {}", file_path.display(), e);
                crate::utils::log_error(&error_msg);
                
                // Check if it's a permission error and provide a more helpful message
                if e.kind() == std::io::ErrorKind::PermissionDenied {
                    let help_msg = format!(
                        "Permission denied writing to '{}'. Try:\n  • Check file permissions with: ls -la {}\n  • Remove read-only flag with: chmod u+w {}",
                        file_path.display(), file_path.display(), file_path.display()
                    );
                    crate::utils::log_error(&help_msg);
                    return Err(format!("{}\n\n{}", error_msg, help_msg).into());
                }
                
                return Err(error_msg.into());
            }
        }
    }
    
    // Create file if specified but contents wasn't provided
    // The 'file' field should always create a file, not a folder
    if template.contents.is_none() && template.file.is_some() {
        let file_path_str = if let Some(ref file_template) = template.file {
            // Check for special "true" value to use saved command's arg
            if file_template == "true" {
                saved_command.arg.clone()
            } else {
                context.expand(file_template)
            }
        } else {
            saved_command.arg.clone()
        };
        
        // Expand tilde in the file path
        let expanded_path = if file_path_str.starts_with("~/") {
            if let Ok(home) = std::env::var("HOME") {
                file_path_str.replacen("~/", &format!("{}/", home), 1)
            } else {
                file_path_str.clone()
            }
        } else {
            file_path_str.clone()
        };
        
        let file_path = std::path::Path::new(&expanded_path);
        
        // Create parent directory if it doesn't exist
        if let Some(parent) = file_path.parent() {
            crate::utils::detailed_log("TEMPLATE", &format!("Creating parent dir for file: {}", parent.display()));
            create_folder_if_needed(&parent.to_string_lossy())?;
        }
        
        // Create an empty file if it doesn't exist
        if !file_path.exists() {
            if let Err(e) = std::fs::write(&file_path, "") {
                let error_msg = format!("Cannot create file '{}': {}", file_path.display(), e);
                crate::utils::log_error(&error_msg);
                return Err(error_msg.into());
            } else {
                crate::utils::detailed_log("TEMPLATE", &format!("TEMPLATE: Created empty file '{}'", file_path.display()));
            }
        }
    }
    
    Ok(())
}

/// Create a folder if it doesn't exist
fn create_folder_if_needed(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    crate::utils::detailed_log("TEMPLATE", &format!("create_folder_if_needed called with: {}", path));
    
    let expanded_path = if path.starts_with("~/") {
        if let Ok(home) = std::env::var("HOME") {
            let result = path.replacen("~/", &format!("{}/", home), 1);
            crate::utils::detailed_log("TEMPLATE", &format!("Expanded tilde path to: {}", result));
            result
        } else {
            crate::utils::detailed_log("TEMPLATE", "Could not get HOME environment variable");
            path.to_string()
        }
    } else {
        crate::utils::detailed_log("TEMPLATE", &format!("Path doesn't start with ~/, using as-is: {}", path));
        path.to_string()
    };
    
    let path = std::path::Path::new(&expanded_path);
    crate::utils::detailed_log("TEMPLATE", &format!("Final path for mkdir: {}", path.display()));
    
    if !path.exists() {
        crate::utils::detailed_log("TEMPLATE", "Path doesn't exist, creating directory...");
        match std::fs::create_dir_all(path) {
            Ok(_) => crate::utils::detailed_log("TEMPLATE", "Successfully created directory"),
            Err(e) => {
                let error_msg = format!("Cannot create directory '{}': {}", path.display(), e);
                crate::utils::log_error(&error_msg);
                crate::utils::detailed_log("TEMPLATE", &error_msg);
                return Err(error_msg.into());
            }
        }
    } else {
        crate::utils::detailed_log("TEMPLATE", "Path already exists, skipping creation");
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_variable_expansion() {
        let mut context = TemplateContext::new("test input", None, None);
        context.add_variable("custom".to_string(), "custom value".to_string());
        
        assert_eq!(context.expand("{{input}}"), "test input");
        assert_eq!(context.expand("Hello {{input}}!"), "Hello test input!");
        assert_eq!(context.expand("{{custom}}"), "custom value");
        assert_eq!(context.expand("{{unknown}}"), "{{unknown}}");
        assert_eq!(context.expand("{{input}} and {{custom}}"), "test input and custom value");
    }
    
    #[test]
    fn test_selected_command_variables() {
        let selected_command = Command {
            command: "test_cmd".to_string(),
            action: "markdown".to_string(),
            arg: "/Users/test/Documents/notes/test.md".to_string(),
            patch: "TestPatch".to_string(),
            flags: String::new(),
        };
        
        let context = TemplateContext::new("input", Some(&selected_command), None);
        
        // Test object-based access
        assert_eq!(context.expand("{{selected.name}}"), "test_cmd");
        assert_eq!(context.expand("{{selected.path}}"), "/Users/test/Documents/notes/test.md");
        assert_eq!(context.expand("{{selected.patch}}"), "TestPatch");
        assert_eq!(context.expand("{{selected.folder}}"), "/Users/test/Documents/notes");
        assert_eq!(context.expand("{{selected.action}}"), "markdown");
    }
    
    #[test]
    fn test_datetime_variables() {
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
    
    #[test]
    fn test_create_command_from_template() {
        let template = Template {
            name: "{{input}} Note".to_string(),
            action: "markdown".to_string(),
            arg: "/notes/{{date.year}}/{{date.month}}/{{input}}.md".to_string(),
            patch: "Notes".to_string(),
            flags: String::new(),
            key: None,
            edit: false,
            file: None,
            contents: None,
            grab: None,
            description: None,
            use_existing: false,
            validate_previous_folder: false,
            keystroke: None,
            file_rescan: false,
        };
        
        let context = TemplateContext::new("Test", None, None);
        let command = create_command_from_template(&template, &context);
        
        assert_eq!(command.command, "Test Note");
        assert_eq!(command.action, "markdown");
        assert!(command.arg.contains("/notes/"));
        assert!(command.arg.ends_with("/Test.md"));
        assert_eq!(command.patch, "Notes");
    }
}