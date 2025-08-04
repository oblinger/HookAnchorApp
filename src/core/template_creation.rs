//! Template Creation System
//! 
//! Provides a flexible template-based system for creating new commands
//! with variable expansion and customizable fields.

use std::collections::HashMap;
use chrono::{Local, Datelike, Timelike};
use serde::{Deserialize, Serialize};
use crate::Command;

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
    
    /// If true, open command editor before creating
    #[serde(default)]
    pub edit: bool,
    
    /// Optional folder path to create (supports variable expansion)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<String>,
    
    /// Optional file contents to create (supports variable expansion)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contents: Option<String>,
    
    /// Optional seconds to wait before grabbing window
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grab: Option<u32>,
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
        }
        
        // Previous command variables
        if let Some(cmd) = previous_command {
            variables.insert("previous_name".to_string(), cmd.command.clone());
            variables.insert("previous_path".to_string(), cmd.arg.clone());
            variables.insert("previous_patch".to_string(), cmd.patch.clone());
            
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
    
    /// Expand variables in a string
    pub fn expand(&self, template: &str) -> String {
        let mut result = template.to_string();
        let mut last_pos = 0;
        
        // Process the string from left to right to avoid re-processing replacements
        while let Some(start) = result[last_pos..].find("{{") {
            let start = last_pos + start;
            if let Some(end) = result[start..].find("}}") {
                let end = start + end + 2;
                let var_name = &result[start + 2..end - 2].trim();
                
                if let Some(value) = self.variables.get(*var_name) {
                    result.replace_range(start..end, value);
                    // Move position to after the replacement
                    last_pos = start + value.len();
                } else {
                    // Unknown variable - skip past it to avoid infinite loop
                    last_pos = end;
                }
            } else {
                // No matching }} found, break to avoid infinite loop
                break;
            }
        }
        
        result
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
        full_line: String::new(),
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
        std::fs::write(&file_path, contents)?;
    }
    
    // Create additional folder if specified (for cases where file and contents point to different things)
    if let Some(folder_template) = &template.file {
        let folder_path = context.expand(folder_template);
        create_folder_if_needed(&folder_path)?;
    }
    
    // TODO: Implement edit functionality if template.edit is true
    if template.edit {
        // This will be implemented in Phase 3
        // For now, we'll skip the editor
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
                crate::utils::debug_log("TEMPLATE", &format!("Failed to create directory: {}", e));
                return Err(e.into());
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
            full_line: String::new(),
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