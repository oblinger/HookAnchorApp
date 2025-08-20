//! Template Creation System
//! 
//! Provides a flexible template-based system for creating new commands
//! with variable expansion and customizable fields.

use std::collections::HashMap;
use chrono::{Local, Datelike, Timelike};
use serde::{Deserialize, Serialize};
use serde_json::json;
use crate::Command;
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
pub struct TemplateContext {
    /// Variables available for expansion
    pub variables: HashMap<String, String>,
}

impl TemplateContext {
    /// Create a new template context with standard variables
    pub fn new(
        input: &str,
        selected_command: Option<&Command>,
        previous_command: Option<&Command>,
    ) -> Self {
        let mut variables = HashMap::new();
        
        // Basic variables
        variables.insert("input".to_string(), input.to_string());
        
        // Selected command variables
        if let Some(cmd) = selected_command {
            variables.insert("selected_name".to_string(), cmd.command.clone());
            variables.insert("selected_path".to_string(), cmd.arg.clone());
            variables.insert("selected_patch".to_string(), cmd.patch.clone());
            variables.insert("selected_action".to_string(), cmd.action.clone());
            variables.insert("selected_flags".to_string(), cmd.flags.clone());
            
            // Extract folder from path if it's a file path
            if let Some(folder) = extract_folder_from_path(&cmd.arg) {
                variables.insert("selected_folder".to_string(), folder);
            }
        } else {
            // Provide empty defaults for when no command is selected
            variables.insert("selected_name".to_string(), String::new());
            variables.insert("selected_path".to_string(), String::new());
            variables.insert("selected_patch".to_string(), String::new());
            variables.insert("selected_folder".to_string(), String::new());
            variables.insert("selected_action".to_string(), String::new());
            variables.insert("selected_flags".to_string(), String::new());
        }
        
        // Previous command variables
        if let Some(cmd) = previous_command {
            variables.insert("previous_name".to_string(), cmd.command.clone());
            variables.insert("previous_path".to_string(), cmd.arg.clone());
            variables.insert("previous_patch".to_string(), cmd.patch.clone());
            variables.insert("previous_action".to_string(), cmd.action.clone());
            variables.insert("previous_flags".to_string(), cmd.flags.clone());
            
            // Extract folder from path if it's a file path
            if let Some(folder) = extract_folder_from_path(&cmd.arg) {
                variables.insert("previous_folder".to_string(), folder);
            } else {
                variables.insert("previous_folder".to_string(), String::new());
            }
            
            crate::utils::debug_log("TEMPLATE_CONTEXT", &format!("Set previous_name to: '{}'", cmd.command));
            crate::utils::debug_log("TEMPLATE_CONTEXT", &format!("Set previous_path to: '{}'", cmd.arg));
            crate::utils::debug_log("TEMPLATE_CONTEXT", &format!("Set previous_patch to: '{}'", cmd.patch));
        } else {
            crate::utils::debug_log("TEMPLATE_CONTEXT", "No previous command provided to context");
            // Provide empty defaults for when no command is available
            variables.insert("previous_name".to_string(), String::new());
            variables.insert("previous_path".to_string(), String::new());
            variables.insert("previous_patch".to_string(), String::new());
            variables.insert("previous_folder".to_string(), String::new());
            variables.insert("previous_action".to_string(), String::new());
            variables.insert("previous_flags".to_string(), String::new());
        }
        
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
                        result.replace_range(start..end, &value);
                        // Move position to after the replacement
                        last_pos = start + value.len();
                    }
                    Err(e) => {
                        crate::utils::debug_log("TEMPLATE_JS", &format!("Failed to evaluate '{}': {}", expr, e));
                        // If evaluation fails, try simple variable lookup for backward compatibility
                        if let Some(value) = self.variables.get(expr) {
                            result.replace_range(start..end, value);
                            last_pos = start + value.len();
                        } else {
                            // Unknown variable - skip past it to avoid infinite loop
                            last_pos = end;
                        }
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
            // Evaluate the expression
            (function() {{
                try {{
                    let result = {};
                    // Convert undefined/null to empty string
                    if (result === undefined || result === null) {{
                        return '';
                    }}
                    return String(result);
                }} catch(e) {{
                    log('Template JS error: ' + e.toString());
                    return '';
                }}
            }})();
            "#,
            expr
        );
        
        // Execute the JavaScript code
        crate::js_runtime::execute_business_logic(&js_code)
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
        
        // Create previous object
        context.push_str("const previous = {\n");
        context.push_str(&format!("  name: {:?},\n", self.variables.get("previous_name").unwrap_or(&String::new())));
        context.push_str(&format!("  path: {:?},\n", self.variables.get("previous_path").unwrap_or(&String::new())));
        context.push_str(&format!("  arg: {:?},\n", self.variables.get("previous_path").unwrap_or(&String::new())));
        context.push_str(&format!("  patch: {:?},\n", self.variables.get("previous_patch").unwrap_or(&String::new())));
        context.push_str(&format!("  folder: {:?},\n", self.variables.get("previous_folder").unwrap_or(&String::new())));
        context.push_str(&format!("  action: {:?},\n", self.variables.get("previous_action").unwrap_or(&String::new())));
        context.push_str(&format!("  flags: {:?}\n", self.variables.get("previous_flags").unwrap_or(&String::new())));
        context.push_str("};\n");
        
        // Create selected object
        context.push_str("const selected = {\n");
        context.push_str(&format!("  name: {:?},\n", self.variables.get("selected_name").unwrap_or(&String::new())));
        context.push_str(&format!("  path: {:?},\n", self.variables.get("selected_path").unwrap_or(&String::new())));
        context.push_str(&format!("  arg: {:?},\n", self.variables.get("selected_path").unwrap_or(&String::new())));
        context.push_str(&format!("  patch: {:?},\n", self.variables.get("selected_patch").unwrap_or(&String::new())));
        context.push_str(&format!("  folder: {:?},\n", self.variables.get("selected_folder").unwrap_or(&String::new())));
        context.push_str(&format!("  action: {:?},\n", self.variables.get("selected_action").unwrap_or(&String::new())));
        context.push_str(&format!("  flags: {:?}\n", self.variables.get("selected_flags").unwrap_or(&String::new())));
        context.push_str("};\n");
        
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
        context.push_str(&format!("  text: {:?}\n", self.variables.get("grabbed_text").unwrap_or(&String::new())));
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
        
        // Add backward compatibility aliases for old variable names
        context.push_str("\n// Backward compatibility aliases\n");
        context.push_str(&format!("const previous_name = {:?};\n", self.variables.get("previous_name").unwrap_or(&String::new())));
        context.push_str(&format!("const previous_folder = {:?};\n", self.variables.get("previous_folder").unwrap_or(&String::new())));
        context.push_str(&format!("const previous_patch = {:?};\n", self.variables.get("previous_patch").unwrap_or(&String::new())));
        context.push_str(&format!("const previous_path = {:?};\n", self.variables.get("previous_path").unwrap_or(&String::new())));
        context.push_str(&format!("const selected_name = {:?};\n", self.variables.get("selected_name").unwrap_or(&String::new())));
        context.push_str(&format!("const selected_folder = {:?};\n", self.variables.get("selected_folder").unwrap_or(&String::new())));
        context.push_str(&format!("const selected_patch = {:?};\n", self.variables.get("selected_patch").unwrap_or(&String::new())));
        context.push_str(&format!("const grabbed_action = {:?};\n", self.variables.get("grabbed_action").unwrap_or(&String::new())));
        context.push_str(&format!("const grabbed_arg = {:?};\n", self.variables.get("grabbed_arg").unwrap_or(&String::new())));
        context.push_str(&format!("const YYYY = {:?};\n", format!("{:04}", now.year())));
        context.push_str(&format!("const YY = {:?};\n", format!("{:02}", now.year() % 100)));
        context.push_str(&format!("const MM = {:?};\n", format!("{:02}", now.month())));
        context.push_str(&format!("const DD = {:?};\n", format!("{:02}", now.day())));
        context.push_str(&format!("const hh = {:?};\n", format!("{:02}", now.hour())));
        context.push_str(&format!("const mm = {:?};\n", format!("{:02}", now.minute())));
        context.push_str(&format!("const ss = {:?};\n", format!("{:02}", now.second())));
        
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

/// Add date/time variables to the context
fn add_datetime_variables(variables: &mut HashMap<String, String>) {
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

/// Process a template and create the command and any associated files
pub fn process_template(
    template: &Template,
    context: &TemplateContext,
    _config: &crate::Config,
) -> Result<Command, Box<dyn std::error::Error>> {
    // Validate previous folder if required
    if template.validate_previous_folder {
        let previous_folder = context.variables.get("previous_folder")
            .ok_or("No previous command available")?;
            
        if previous_folder.is_empty() {
            let error_msg = "Previous command has no associated folder. Cannot create sub-anchor.";
            crate::error_display::queue_user_error(error_msg);
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
            crate::error_display::queue_user_error(&error_msg);
            return Err(error_msg.into());
        }
        
        crate::utils::debug_log("TEMPLATE", &format!("Validated previous folder: {}", previous_folder));
    }
    
    // TODO: Implement grab functionality if template.grab is set
    if let Some(_grab_seconds) = template.grab {
        // This will be implemented in Phase 3
        // For now, we'll skip grab functionality
    }
    
    // Create the command
    let mut command = create_command_from_template(template, context);
    
    // Create file with contents if specified
    if let Some(contents_template) = &template.contents {
        let contents = context.expand(contents_template);
        let expanded_arg = context.expand(&command.arg);
        
        // Debug logging
        crate::utils::debug_log("TEMPLATE", &format!("Original arg: {}", command.arg));
        crate::utils::debug_log("TEMPLATE", &format!("Expanded arg: {}", expanded_arg));
        
        // Expand tilde in the file path
        let expanded_path = if expanded_arg.starts_with("~/") {
            if let Ok(home) = std::env::var("HOME") {
                expanded_arg.replacen("~/", &format!("{}/", home), 1)
            } else {
                expanded_arg.clone()
            }
        } else {
            expanded_arg.clone()
        };
        
        let file_path = std::path::Path::new(&expanded_path);
        
        // Create parent directory if it doesn't exist
        if let Some(parent) = file_path.parent() {
            crate::utils::debug_log("TEMPLATE", &format!("Creating parent dir: {}", parent.display()));
            create_folder_if_needed(&parent.to_string_lossy())?;
        }
        
        crate::utils::debug_log("TEMPLATE", &format!("Writing file: {}", file_path.display()));
        if let Err(e) = std::fs::write(&file_path, contents) {
            let error_msg = format!("Cannot write to file '{}': {}", file_path.display(), e);
            crate::utils::log_error(&error_msg);
            return Err(error_msg.into());
        }
    }
    
    // Create additional folder if specified (for cases where file and contents point to different things)
    if let Some(folder_template) = &template.file {
        let folder_path = context.expand(folder_template);
        if let Err(e) = create_folder_if_needed(&folder_path) {
            // Error already logged in create_folder_if_needed
            return Err(e);
        }
    }
    
    // If edit flag is set, return the command for editing
    if template.edit {
        crate::utils::debug_log("TEMPLATE", "Template has edit flag set - command will be opened in editor");
        // Return a special marker in the flags to indicate this should open the editor
        command.flags = format!("{}__EDIT__", command.flags);
    }
    
    // Update the full_line
    command.update_full_line();
    
    Ok(command)
}

/// Create a folder if it doesn't exist
fn create_folder_if_needed(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    crate::utils::debug_log("TEMPLATE", &format!("create_folder_if_needed called with: {}", path));
    
    let expanded_path = if path.starts_with("~/") {
        if let Ok(home) = std::env::var("HOME") {
            let result = path.replacen("~/", &format!("{}/", home), 1);
            crate::utils::debug_log("TEMPLATE", &format!("Expanded tilde path to: {}", result));
            result
        } else {
            crate::utils::debug_log("TEMPLATE", "Could not get HOME environment variable");
            path.to_string()
        }
    } else {
        crate::utils::debug_log("TEMPLATE", &format!("Path doesn't start with ~/, using as-is: {}", path));
        path.to_string()
    };
    
    let path = std::path::Path::new(&expanded_path);
    crate::utils::debug_log("TEMPLATE", &format!("Final path for mkdir: {}", path.display()));
    
    if !path.exists() {
        crate::utils::debug_log("TEMPLATE", "Path doesn't exist, creating directory...");
        match std::fs::create_dir_all(path) {
            Ok(_) => crate::utils::debug_log("TEMPLATE", "Successfully created directory"),
            Err(e) => {
                let error_msg = format!("Cannot create directory '{}': {}", path.display(), e);
                crate::utils::log_error(&error_msg);
                crate::utils::debug_log("TEMPLATE", &error_msg);
                return Err(error_msg.into());
            }
        }
    } else {
        crate::utils::debug_log("TEMPLATE", "Path already exists, skipping creation");
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
    fn test_previous_command_variables() {
        let previous_command = Command {
            command: "test_cmd".to_string(),
            action: "markdown".to_string(),
            arg: "/Users/test/Documents/notes/test.md".to_string(),
            patch: "TestPatch".to_string(),
            flags: String::new(),
        };
        
        let context = TemplateContext::new("input", None, Some(&previous_command));
        
        assert_eq!(context.expand("{{previous_name}}"), "test_cmd");
        assert_eq!(context.expand("{{previous_path}}"), "/Users/test/Documents/notes/test.md");
        assert_eq!(context.expand("{{previous_patch}}"), "TestPatch");
        assert_eq!(context.expand("{{previous_folder}}"), "/Users/test/Documents/notes");
    }
    
    #[test]
    fn test_datetime_variables() {
        let context = TemplateContext::new("", None, None);
        
        // Test that date variables exist and have correct format
        let year = context.expand("{{YYYY}}");
        assert_eq!(year.len(), 4);
        assert!(year.parse::<u32>().is_ok());
        
        let month = context.expand("{{MM}}");
        assert_eq!(month.len(), 2);
        let month_num = month.parse::<u32>().unwrap();
        assert!(month_num >= 1 && month_num <= 12);
        
        let day = context.expand("{{DD}}");
        assert_eq!(day.len(), 2);
        let day_num = day.parse::<u32>().unwrap();
        assert!(day_num >= 1 && day_num <= 31);
    }
    
    #[test]
    fn test_create_command_from_template() {
        let template = Template {
            name: "{{input}} Note".to_string(),
            action: "markdown".to_string(),
            arg: "/notes/{{YYYY}}/{{MM}}/{{input}}.md".to_string(),
            patch: "Notes".to_string(),
            flags: String::new(),
            key: None,
            edit: false,
            file: None,
            contents: None,
            grab: None,
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