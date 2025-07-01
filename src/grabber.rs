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
    
    Ok(AppContext {
        app_name: parts[0].to_string(),
        bundle_id: parts[1].to_string(),
        window_title: parts[2].to_string(),
        properties: serde_json::json!({}),
    })
}

/// Get additional browser-specific information
pub fn get_browser_info(bundle_id: &str) -> Option<String> {
    let browser_script = match bundle_id {
        "com.google.Chrome" => Some(r#"
            tell application "Google Chrome"
                if (count of windows) > 0 then
                    return URL of active tab of window 1
                else
                    return ""
                end if
            end tell
        "#),
        "com.apple.Safari" => Some(r#"
            tell application "Safari"
                if (count of windows) > 0 then
                    return URL of current tab of window 1
                else
                    return ""
                end if
            end tell
        "#),
        "com.brave.Browser" => Some(r#"
            tell application "Brave Browser"
                if (count of windows) > 0 then
                    return URL of active tab of window 1
                else
                    return ""
                end if
            end tell
        "#),
        "org.mozilla.firefox" => Some(r#"
            tell application "Firefox"
                -- Firefox doesn't have good AppleScript support
                -- We'll use window title as a fallback
                return ""
            end tell
        "#),
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

/// Get Finder-specific information
pub fn get_finder_info() -> Option<String> {
    let script = r#"
        tell application "Finder"
            if (count of windows) > 0 then
                set thePath to POSIX path of (target of window 1 as alias)
                return thePath
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
            let path = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if !path.is_empty() {
                return Some(path);
            }
        }
    }
    
    None
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
    
    // Add Finder path if applicable
    if context.bundle_id == "com.apple.finder" {
        if let Some(path) = get_finder_info() {
            context.properties["path"] = serde_json::Value::String(path);
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
        for rule in rules {
            // Evaluate the matcher
            match ctx.eval::<Value, _>(rule.matcher.as_bytes()) {
                Ok(value) => {
                    if let Some(str_ref) = value.as_string() {
                        if let Ok(arg) = str_ref.to_string() {
                            // Rule matched and returned a string argument
                            let command = Command {
                                group: rule.group.clone().unwrap_or_default(),
                                command: String::new(), // Will be filled by user
                                action: rule.action.clone(),
                                arg,
                                flags: String::new(),
                                full_line: String::new(), // Will be computed
                            };
                            return Some((rule.name.clone(), command));
                        }
                    }
                    // If not a string, continue to next rule
                }
                Err(_) => {
                    // Error evaluating rule, skip it
                    continue;
                }
            }
        }
        
        None
    })
}

/// Perform a grab operation: capture context and match against rules
pub fn grab(config: &Config) -> Result<(String, Command), String> {
    // Capture the active application context
    let context = capture_active_app()?;
    let context = enrich_context(context);
    
    // Get grabber rules from config
    let rules = config.grabber_rules.as_ref()
        .ok_or("No grabber rules configured")?;
    
    // Match against rules
    match_grabber_rules(&context, rules, config)
        .ok_or_else(|| format!("No grabber rule matched for {}", context.app_name))
}