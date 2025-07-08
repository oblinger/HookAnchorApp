//! Grabber module for capturing active application context
//! 
//! This module captures information about the currently focused application
//! and matches it against grabber rules to create commands for reopening
//! the same document/URL/folder later.

use std::process::Command as ProcessCommand;
use serde::{Deserialize, Serialize};
use crate::{Command, Config};
use rquickjs::{Runtime, Context, Value};

/// Information captured about the active application
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppContext {
    /// Application name (e.g., "Google Chrome")
    pub app_name: String,
    /// Bundle identifier (e.g., "com.google.Chrome")
    pub bundle_id: String,
    /// Window title
    pub window_title: String,
    /// Additional properties that might be useful
    pub properties: serde_json::Value,
}

/// A grabber rule that matches against app context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrabberRule {
    /// Name of this rule (for debugging)
    pub name: String,
    /// JavaScript expression that returns the arg if matched, or null if not
    pub matcher: String,
    /// Action type for the created command (e.g., "url", "folder", "obs")
    pub action: String,
    /// Optional group for the created command
    pub group: Option<String>,
}

/// Captures information about the currently focused application
pub fn capture_active_app() -> Result<AppContext, String> {
    
    // Use AppleScript to get information about the frontmost application
    let script = r#"
        tell application "System Events"
            set frontApp to first application process whose frontmost is true
            set appName to name of frontApp
            set bundleId to bundle identifier of frontApp
            
            -- Try to get window title
            set windowTitle to ""
            try
                set windowTitle to name of window 1 of frontApp
            end try
            
            -- Output as pipe-delimited for easy parsing
            return appName & "|" & bundleId & "|" & windowTitle
        end tell
    "#;
    
    
    let output = ProcessCommand::new("osascript")
        .arg("-e")
        .arg(script)
        .output()
        .map_err(|e| format!("Failed to run AppleScript: {}", e))?;
    
    if !output.status.success() {
        return Err(format!("AppleScript failed: {}", String::from_utf8_lossy(&output.stderr)));
    }
    
    let result = String::from_utf8_lossy(&output.stdout).trim().to_string();
    
    let parts: Vec<&str> = result.split('|').collect();
    
    if parts.len() < 3 {
        return Err("Failed to parse AppleScript output".to_string());
    }
    
    let context = AppContext {
        app_name: parts[0].to_string(),
        bundle_id: parts[1].to_string(),
        window_title: parts[2].to_string(),
        properties: serde_json::json!({}),
    };
    
    
    Ok(context)
}

/// Get additional browser-specific information
pub fn get_browser_info(bundle_id: &str) -> Option<String> {
    
    let browser_script = match bundle_id {
        "com.google.Chrome" => {
            Some(r#"
                tell application "Google Chrome"
                    if (count of windows) > 0 then
                        return URL of active tab of window 1
                    else
                        return ""
                    end if
                end tell
            "#)
        },
        "com.apple.Safari" => {
            Some(r#"
                tell application "Safari"
                    if (count of windows) > 0 then
                        return URL of current tab of window 1
                    else
                        return ""
                    end if
                end tell
            "#)
        },
        "com.brave.Browser" => {
            Some(r#"
                tell application "Brave Browser"
                    if (count of windows) > 0 then
                        return URL of active tab of window 1
                    else
                        return ""
                    end if
                end tell
            "#)
        },
        "org.mozilla.firefox" => {
            Some(r#"
                tell application "Firefox"
                    -- Firefox doesn't have good AppleScript support
                    -- We'll use window title as a fallback
                    return ""
                end tell
            "#)
        },
        _ => None,
    };
    
    if let Some(script) = browser_script {
        
        if let Ok(output) = ProcessCommand::new("osascript")
            .arg("-e")
            .arg(script)
            .output()
        {
            if output.status.success() {
                let url = String::from_utf8_lossy(&output.stdout).trim().to_string();
                if !url.is_empty() {
                    return Some(url);
                }
            }
        }
    }
    
    None
}

/// Get Finder-specific information including selection
pub fn get_finder_info() -> Option<serde_json::Value> {
    
    let script = r#"
        tell application "Finder"
            if (count of windows) > 0 then
                set currentPath to POSIX path of (target of window 1 as alias)
                
                -- Get selected items
                set selectedItems to selection
                if (count of selectedItems) > 0 then
                    set firstSelection to item 1 of selectedItems
                    set selectionPath to POSIX path of (firstSelection as alias)
                    
                    -- Check if selection is a folder
                    set itemClass to class of firstSelection
                    if itemClass is folder then
                        set isFolder to "true"
                    else
                        set isFolder to "false"
                    end if
                    
                    return currentPath & "|" & selectionPath & "|" & isFolder
                else
                    return currentPath & "||"
                end if
            else
                return ""
            end if
        end tell
    "#;
    
    
    if let Ok(output) = ProcessCommand::new("osascript")
        .arg("-e")
        .arg(script)
        .output()
    {
        if output.status.success() {
            let result = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if !result.is_empty() {
                let parts: Vec<&str> = result.split('|').collect();
                let mut finder_info = serde_json::json!({});
                
                if parts.len() >= 1 && !parts[0].is_empty() {
                    finder_info["path"] = serde_json::Value::String(parts[0].to_string());
                }
                
                if parts.len() >= 2 && !parts[1].is_empty() {
                    finder_info["selection"] = serde_json::Value::String(parts[1].to_string());
                }
                
                if parts.len() >= 3 && !parts[2].is_empty() {
                    finder_info["selectionIsDirectory"] = serde_json::Value::Bool(parts[2] == "true");
                }
                
                return Some(finder_info);
            }
        }
    }
    
    None
}

/// Extract Slack channel/DM name from window title
fn extract_slack_info(window_title: &str) -> Option<String> {
    // Slack window title format: "[Channel/DM Name] - [Workspace Name] - Slack"
    // or sometimes just "[Channel/DM Name] - Slack"
    let parts: Vec<&str> = window_title.split(" - ").collect();
    if parts.len() >= 2 && parts.last() == Some(&"Slack") {
        // Return the first part which is the channel/DM name
        Some(parts[0].to_string())
    } else {
        None
    }
}

/// Enrich the app context with additional information based on the app
pub fn enrich_context(mut context: AppContext) -> AppContext {
    
    // Add browser URL if applicable
    if context.bundle_id.contains("Chrome") || 
       context.bundle_id.contains("Safari") || 
       context.bundle_id.contains("brave") ||
       context.bundle_id.contains("firefox") {
        if let Some(url) = get_browser_info(&context.bundle_id) {
            context.properties["url"] = serde_json::Value::String(url);
        }
    }
    
    // Add Finder path and selection info if applicable
    if context.bundle_id == "com.apple.finder" {
        crate::utils::debug_log("GRABBER", "Enriching Finder context...");
        if let Some(finder_info) = get_finder_info() {
            crate::utils::debug_log("GRABBER", &format!("Got Finder info: {:?}", finder_info));
            // Merge the finder info into properties
            if let Some(obj) = finder_info.as_object() {
                for (key, value) in obj {
                    context.properties[key] = value.clone();
                    crate::utils::debug_log("GRABBER", &format!("Added props.{}: {:?}", key, value));
                }
            }
        } else {
            crate::utils::debug_log("GRABBER", "Failed to get Finder info");
        }
    }
    
    // Add Slack channel info if applicable
    if context.bundle_id == "com.tinyspeck.slackmacgap" {
        if let Some(channel_info) = extract_slack_info(&context.window_title) {
            context.properties["channel"] = serde_json::Value::String(channel_info);
        }
    }
    
    context
}

/// Match the app context against grabber rules and return the first match
pub fn match_grabber_rules(
    context: &AppContext,
    rules: &[GrabberRule],
    _config: &Config,
) -> Option<(String, Command)> {
    if rules.is_empty() {
        crate::utils::debug_log("GRABBER", "No grabber rules configured");
        return None;
    }
    
    crate::utils::debug_log("GRABBER", &format!("Attempting {} grabber rules", rules.len()));
    
    let rt = Runtime::new().ok()?;
    let ctx = Context::full(&rt).ok()?;
    
    // Set up the context in JavaScript
    let context_json = serde_json::to_string(context).ok()?;
    let setup_script = format!(r#"
        const context = {};
        const app = context.app_name;
        const bundleId = context.bundle_id;
        const title = context.window_title;
        const props = context.properties;
    "#, context_json);
    
    ctx.with(|ctx| {
        if ctx.eval::<(), _>(setup_script.as_bytes()).is_err() {
            return None;
        }
        
        // Try each rule
        for rule in rules.iter() {
            
            match ctx.eval::<Value, _>(rule.matcher.as_bytes()) {
                Ok(value) => {
                    // Check what type of value was returned
                    if value.is_null() || value.is_undefined() {
                        // Rule didn't match, continue to next rule
                    } else if let Some(str_ref) = value.as_string() {
                        // String return - use current behavior
                        if let Ok(arg) = str_ref.to_string() {
                            crate::utils::debug_log("GRABBER", &format!("Matched rule '{}' with string: '{}'", rule.name, arg));
                            
                            // Rule matched and returned a string argument
                            let command = Command {
                                group: rule.group.clone().unwrap_or_default(),
                                command: String::new(), // Will be filled by user
                                action: rule.action.clone(),
                                arg: arg.clone(),
                                flags: String::new(),
                                full_line: String::new(), // Will be computed
                            };
                            
                            return Some((rule.name.clone(), command));
                        }
                    } else if let Some(obj) = value.as_object() {
                        // Object return - extract command fields
                        crate::utils::debug_log("GRABBER", &format!("Matched rule '{}' with object", rule.name));
                        
                        // Extract action, arg, and group from the object
                        let action = obj.get::<_, String>("action").ok()
                            .or_else(|| rule.action.clone().into());
                        let arg = obj.get::<_, String>("arg").ok()
                            .unwrap_or_default();
                        let group = obj.get::<_, String>("group").ok()
                            .or_else(|| rule.group.clone())
                            .unwrap_or_default();
                        
                        if let Some(action) = action {
                            crate::utils::debug_log("GRABBER", &format!("  action: '{}', arg: '{}', group: '{}'", action, arg, group));
                            
                            let command = Command {
                                group,
                                command: String::new(), // Will be filled by user
                                action,
                                arg,
                                flags: String::new(),
                                full_line: String::new(), // Will be computed
                            };
                            
                            return Some((rule.name.clone(), command));
                        } else {
                            crate::utils::debug_log("GRABBER", "  Error: object missing 'action' field");
                        }
                    }
                    // Other return types (bool, number) are treated as no match
                }
                Err(e) => {
                    // Log JavaScript errors for debugging
                    crate::utils::debug_log("GRABBER", &format!("Rule '{}' error: {:?}", rule.name, e));
                }
            }
        }
        
        crate::utils::debug_log("GRABBER", "No rules matched");
        None
    })
}

/// Output the captured application context (always shown)
fn output_grabber_context_summary(context: &AppContext) {
    crate::utils::debug_log("GRABBER", "=== CAPTURED APPLICATION CONTEXT ===");
    crate::utils::debug_log("GRABBER", &format!("App: '{}'", context.app_name));
    crate::utils::debug_log("GRABBER", &format!("Bundle ID: '{}'", context.bundle_id));
    crate::utils::debug_log("GRABBER", &format!("Title: '{}'", context.window_title));
    if let Some(props_obj) = context.properties.as_object() {
        if !props_obj.is_empty() {
            for (key, value) in props_obj {
                crate::utils::debug_log("GRABBER", &format!("props.{}: '{}'", key, value.as_str().unwrap_or("(complex value)")));
            }
        }
    }
}

/// Generate rule template text for display in dialog
pub fn generate_rule_template_text(context: &AppContext) -> String {
    let mut template = String::new();
    
    template.push_str("=== CAPTURED APPLICATION CONTEXT ===\n");
    template.push_str(&format!("app = \"{}\"\n", context.app_name));
    template.push_str(&format!("bundleId = \"{}\"\n", context.bundle_id));
    template.push_str(&format!("title = \"{}\"\n", context.window_title));
    
    // Show properties as individual props.key variables if any exist
    if let Some(props_obj) = context.properties.as_object() {
        if !props_obj.is_empty() {
            for (key, value) in props_obj {
                template.push_str(&format!("props.{} = \"{}\"\n", key, value.as_str().unwrap_or("(complex value)")));
            }
        }
    }
    template.push_str("\n");
    
    template.push_str("=== RULE TEMPLATE FOR THIS APP ===\n");
    template.push_str("Copy this template to your config.yaml grabber_rules section:\n\n");
    template.push_str(&format!("  - name: \"{} Rule\"\n", context.app_name));
    template.push_str("    matcher: ");
    
    // Generate a smart matcher based on the context
    if context.bundle_id.contains("chrome") || context.bundle_id.contains("brave") {
        template.push_str(&format!("\"bundleId === '{}' && props.url ? props.url : null\"\n", context.bundle_id));
    } else if context.bundle_id == "com.apple.Safari" {
        template.push_str(&format!("\"bundleId === '{}' && props.url ? props.url : null\"\n", context.bundle_id));
    } else if context.bundle_id == "com.apple.finder" {
        template.push_str(&format!("\"bundleId === '{}' && props.path ? props.path : null\"\n", context.bundle_id));
    } else if context.window_title.len() > 3 {
        template.push_str(&format!("\"bundleId === '{}' && title ? title : null\"\n", context.bundle_id));
    } else {
        template.push_str(&format!("\"bundleId === '{}' ? app : null\"\n", context.bundle_id));
    }
    
    template.push_str("    action: \"doc\"  # Change to appropriate action type\n");
    template.push_str("    # Optional group for organizing commands:\n");
    template.push_str("    # group: \"Grabbed\"\n");
    
    template
}



/// Result of a grab operation
#[derive(Debug)]
pub enum GrabResult {
    /// Rule matched - open command editor with this command
    RuleMatched(String, Command),
    /// No rule matched - show template dialog with this text
    NoRuleMatched(AppContext),
}

/// Perform a grab operation: capture context and match against rules
pub fn grab(config: &Config) -> Result<GrabResult, String> {
    // Capture the active application context
    let context = capture_active_app()?;
    let context = enrich_context(context);
    
    // Always output the context summary first, regardless of rules
    output_grabber_context_summary(&context);
    
    // Always log the template dialog content so users can see it in logs
    let template_text = generate_rule_template_text(&context);
    crate::utils::debug_log("GRABBER", "");
    crate::utils::debug_log("GRABBER", "=== TEMPLATE INFORMATION (ALWAYS LOGGED) ===");
    for line in template_text.lines() {
        crate::utils::debug_log("GRABBER", line);
    }
    crate::utils::debug_log("GRABBER", "");
    
    // Try to match against rules if they exist
    if let Some(rules_vec) = config.grabber_rules.as_ref() {
        if let Some((rule_name, command)) = match_grabber_rules(&context, rules_vec, config) {
            crate::utils::debug_log("GRABBER", &format!("Opening command editor with matched rule: '{}'", rule_name));
            return Ok(GrabResult::RuleMatched(rule_name, command));
        } else {
            crate::utils::debug_log("GRABBER", "No rules matched - showing template dialog");
        }
    } else {
        crate::utils::debug_log("GRABBER", "No grabber rules configured");
    }
    
    Ok(GrabResult::NoRuleMatched(context))
}