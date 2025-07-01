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
    crate::utils::debug_log("GRABBER", "=== Starting application capture ===");
    
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
    
    crate::utils::debug_log("GRABBER", "Executing AppleScript to capture active application");
    
    let output = ProcessCommand::new("osascript")
        .arg("-e")
        .arg(script)
        .output()
        .map_err(|e| {
            let error_msg = format!("Failed to run AppleScript: {}", e);
            crate::utils::debug_log("GRABBER", &format!("ERROR: {}", error_msg));
            error_msg
        })?;
    
    if !output.status.success() {
        let error_msg = format!("AppleScript failed: {}", String::from_utf8_lossy(&output.stderr));
        crate::utils::debug_log("GRABBER", &format!("ERROR: {}", error_msg));
        return Err(error_msg);
    }
    
    let result = String::from_utf8_lossy(&output.stdout).trim().to_string();
    crate::utils::debug_log("GRABBER", &format!("AppleScript raw output: '{}'", result));
    
    let parts: Vec<&str> = result.split('|').collect();
    
    if parts.len() < 3 {
        let error_msg = "Failed to parse AppleScript output".to_string();
        crate::utils::debug_log("GRABBER", &format!("ERROR: {}", error_msg));
        crate::utils::debug_log("GRABBER", &format!("Expected 3 parts, got {}: {:?}", parts.len(), parts));
        return Err(error_msg);
    }
    
    let context = AppContext {
        app_name: parts[0].to_string(),
        bundle_id: parts[1].to_string(),
        window_title: parts[2].to_string(),
        properties: serde_json::json!({}),
    };
    
    crate::utils::debug_log("GRABBER", &format!("Captured basic context:"));
    crate::utils::debug_log("GRABBER", &format!("  App Name: '{}'", context.app_name));
    crate::utils::debug_log("GRABBER", &format!("  Bundle ID: '{}'", context.bundle_id));
    crate::utils::debug_log("GRABBER", &format!("  Window Title: '{}'", context.window_title));
    
    Ok(context)
}

/// Get additional browser-specific information
pub fn get_browser_info(bundle_id: &str) -> Option<String> {
    crate::utils::debug_log("GRABBER", &format!("Checking for browser-specific info for bundle: '{}'", bundle_id));
    
    let browser_script = match bundle_id {
        "com.google.Chrome" => {
            crate::utils::debug_log("GRABBER", "Detected Google Chrome - attempting to get URL");
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
            crate::utils::debug_log("GRABBER", "Detected Safari - attempting to get URL");
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
            crate::utils::debug_log("GRABBER", "Detected Brave Browser - attempting to get URL");
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
            crate::utils::debug_log("GRABBER", "Detected Firefox - limited AppleScript support");
            Some(r#"
                tell application "Firefox"
                    -- Firefox doesn't have good AppleScript support
                    -- We'll use window title as a fallback
                    return ""
                end tell
            "#)
        },
        _ => {
            crate::utils::debug_log("GRABBER", &format!("No browser-specific handler for bundle: '{}'", bundle_id));
            None
        },
    };
    
    if let Some(script) = browser_script {
        crate::utils::debug_log("GRABBER", "Executing browser-specific AppleScript");
        
        if let Ok(output) = ProcessCommand::new("osascript")
            .arg("-e")
            .arg(script)
            .output()
        {
            if output.status.success() {
                let url = String::from_utf8_lossy(&output.stdout).trim().to_string();
                crate::utils::debug_log("GRABBER", &format!("Browser AppleScript result: '{}'", url));
                if !url.is_empty() {
                    crate::utils::debug_log("GRABBER", &format!("Successfully extracted URL: {}", url));
                    return Some(url);
                } else {
                    crate::utils::debug_log("GRABBER", "Browser returned empty URL");
                }
            } else {
                let error = String::from_utf8_lossy(&output.stderr);
                crate::utils::debug_log("GRABBER", &format!("Browser AppleScript failed: {}", error));
            }
        } else {
            crate::utils::debug_log("GRABBER", "Failed to execute browser AppleScript");
        }
    }
    
    None
}

/// Get Finder-specific information
pub fn get_finder_info() -> Option<String> {
    crate::utils::debug_log("GRABBER", "Attempting to get Finder path information");
    
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
    
    crate::utils::debug_log("GRABBER", "Executing Finder AppleScript");
    
    if let Ok(output) = ProcessCommand::new("osascript")
        .arg("-e")
        .arg(script)
        .output()
    {
        if output.status.success() {
            let path = String::from_utf8_lossy(&output.stdout).trim().to_string();
            crate::utils::debug_log("GRABBER", &format!("Finder AppleScript result: '{}'", path));
            if !path.is_empty() {
                crate::utils::debug_log("GRABBER", &format!("Successfully extracted Finder path: {}", path));
                return Some(path);
            } else {
                crate::utils::debug_log("GRABBER", "Finder returned empty path");
            }
        } else {
            let error = String::from_utf8_lossy(&output.stderr);
            crate::utils::debug_log("GRABBER", &format!("Finder AppleScript failed: {}", error));
        }
    } else {
        crate::utils::debug_log("GRABBER", "Failed to execute Finder AppleScript");
    }
    
    None
}

/// Enrich the app context with additional information based on the app
pub fn enrich_context(mut context: AppContext) -> AppContext {
    crate::utils::debug_log("GRABBER", "=== Starting context enrichment ===");
    
    // Add browser URL if applicable
    if context.bundle_id.contains("Chrome") || 
       context.bundle_id.contains("Safari") || 
       context.bundle_id.contains("brave") ||
       context.bundle_id.contains("firefox") {
        crate::utils::debug_log("GRABBER", "Bundle ID indicates browser - attempting URL extraction");
        if let Some(url) = get_browser_info(&context.bundle_id) {
            context.properties["url"] = serde_json::Value::String(url.clone());
            crate::utils::debug_log("GRABBER", &format!("Added URL to context: {}", url));
        } else {
            crate::utils::debug_log("GRABBER", "No URL extracted from browser");
        }
    }
    
    // Add Finder path if applicable
    if context.bundle_id == "com.apple.finder" {
        crate::utils::debug_log("GRABBER", "Bundle ID indicates Finder - attempting path extraction");
        if let Some(path) = get_finder_info() {
            context.properties["path"] = serde_json::Value::String(path.clone());
            crate::utils::debug_log("GRABBER", &format!("Added path to context: {}", path));
        } else {
            crate::utils::debug_log("GRABBER", "No path extracted from Finder");
        }
    }
    
    // Log the final enriched context
    crate::utils::debug_log("GRABBER", "=== Final enriched context ===");
    crate::utils::debug_log("GRABBER", &format!("App Name: '{}'", context.app_name));
    crate::utils::debug_log("GRABBER", &format!("Bundle ID: '{}'", context.bundle_id));
    crate::utils::debug_log("GRABBER", &format!("Window Title: '{}'", context.window_title));
    crate::utils::debug_log("GRABBER", &format!("Properties: {}", serde_json::to_string_pretty(&context.properties).unwrap_or_else(|_| "{}".to_string())));
    
    context
}

/// Match the app context against grabber rules and return the first match
pub fn match_grabber_rules(
    context: &AppContext,
    rules: &[GrabberRule],
    _config: &Config,
) -> Option<(String, Command)> {
    crate::utils::debug_log("GRABBER", "=== Starting rule matching ===");
    crate::utils::debug_log("GRABBER", &format!("Found {} grabber rules to evaluate", rules.len()));
    
    if rules.is_empty() {
        crate::utils::debug_log("GRABBER", "ERROR: No grabber rules configured - add some to grabber_rules in config.yaml");
        return None;
    }
    
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
    
    crate::utils::debug_log("GRABBER", "Setting up JavaScript context with captured app data");
    crate::utils::debug_log("GRABBER", &format!("JavaScript context setup script:\n{}", setup_script));
    
    ctx.with(|ctx| {
        if ctx.eval::<(), _>(setup_script.as_bytes()).is_err() {
            crate::utils::debug_log("GRABBER", "ERROR: Failed to setup JavaScript context");
            return None;
        }
        
        crate::utils::debug_log("GRABBER", "JavaScript context setup successful");
        
        // Try each rule
        for (rule_index, rule) in rules.iter().enumerate() {
            crate::utils::debug_log("GRABBER", &format!("=== Evaluating rule {} of {}: '{}' ===", rule_index + 1, rules.len(), rule.name));
            crate::utils::debug_log("GRABBER", &format!("Rule action: '{}'", rule.action));
            crate::utils::debug_log("GRABBER", &format!("Rule group: '{}'", rule.group.as_ref().unwrap_or(&"(none)".to_string())));
            crate::utils::debug_log("GRABBER", &format!("Rule matcher JavaScript:\n{}", rule.matcher));
            
            // Evaluate the matcher
            match ctx.eval::<Value, _>(rule.matcher.as_bytes()) {
                Ok(value) => {
                    crate::utils::debug_log("GRABBER", "Rule JavaScript executed successfully");
                    
                    // Check what type of value was returned
                    if value.is_null() {
                        crate::utils::debug_log("GRABBER", "Rule returned null - no match");
                    } else if value.is_undefined() {
                        crate::utils::debug_log("GRABBER", "Rule returned undefined - no match");
                    } else if let Some(str_ref) = value.as_string() {
                        if let Ok(arg) = str_ref.to_string() {
                            crate::utils::debug_log("GRABBER", &format!("*** RULE MATCHED! *** Returned arg: '{}'", arg));
                            
                            // Rule matched and returned a string argument
                            let command = Command {
                                group: rule.group.clone().unwrap_or_default(),
                                command: String::new(), // Will be filled by user
                                action: rule.action.clone(),
                                arg: arg.clone(),
                                flags: String::new(),
                                full_line: String::new(), // Will be computed
                            };
                            
                            crate::utils::debug_log("GRABBER", &format!("Created command with action='{}', arg='{}'", command.action, command.arg));
                            return Some((rule.name.clone(), command));
                        } else {
                            crate::utils::debug_log("GRABBER", "Rule returned string but failed to convert to String");
                        }
                    } else if value.is_bool() {
                        let bool_val = value.as_bool().unwrap_or(false);
                        crate::utils::debug_log("GRABBER", &format!("Rule returned boolean: {} - no match", bool_val));
                    } else if value.is_number() {
                        crate::utils::debug_log("GRABBER", "Rule returned number - no match");
                    } else {
                        crate::utils::debug_log("GRABBER", "Rule returned unknown type - no match");
                    }
                }
                Err(err) => {
                    crate::utils::debug_log("GRABBER", &format!("ERROR: Rule JavaScript failed to execute: {:?}", err));
                    crate::utils::debug_log("GRABBER", "Check your JavaScript syntax in the rule matcher");
                }
            }
        }
        
        crate::utils::debug_log("GRABBER", "=== No rules matched ===");
        crate::utils::debug_log("GRABBER", "To create a new rule for this app, copy the context info above");
        crate::utils::debug_log("GRABBER", "Example rule template:");
        crate::utils::debug_log("GRABBER", &format!(r#"  - name: "{} Rule"
    matcher: |
      if (bundleId === "{}" && title) {{
        return title; // or extract what you need
      }}
      return null;
    action: "doc"  // or "url", "folder", etc."#, context.app_name, context.bundle_id));
        
        None
    })
}

/// Output a consistent summary block for creating grabber rules
fn output_grabber_summary(context: &AppContext, rules: &[GrabberRule], _config: &Config) {
    crate::utils::debug_log("GRABBER", "");
    crate::utils::debug_log("GRABBER", "################################################################################");
    crate::utils::debug_log("GRABBER", "##################### GRABBER CONTEXT SUMMARY #############################");
    crate::utils::debug_log("GRABBER", "################################################################################");
    crate::utils::debug_log("GRABBER", "");
    crate::utils::debug_log("GRABBER", "=== CAPTURED APPLICATION CONTEXT ===");
    crate::utils::debug_log("GRABBER", &format!("App Name: '{}'", context.app_name));
    crate::utils::debug_log("GRABBER", &format!("Bundle ID: '{}'", context.bundle_id));
    crate::utils::debug_log("GRABBER", &format!("Window Title: '{}'", context.window_title));
    crate::utils::debug_log("GRABBER", &format!("Properties: {}", serde_json::to_string_pretty(&context.properties).unwrap_or_else(|_| "{}".to_string())));
    crate::utils::debug_log("GRABBER", "");
    
    // Try to determine what the action and arg would be with a simple rule match
    let (inferred_action, inferred_arg) = infer_action_and_arg(context);
    
    crate::utils::debug_log("GRABBER", "=== INFERRED RULE SUGGESTION ===");
    crate::utils::debug_log("GRABBER", &format!("Suggested Action: '{}'", inferred_action));
    crate::utils::debug_log("GRABBER", &format!("Suggested Arg: '{}'", inferred_arg.as_deref().unwrap_or("(window title or context-specific)")));
    crate::utils::debug_log("GRABBER", "");
    
    crate::utils::debug_log("GRABBER", "=== RULE TEMPLATE FOR THIS APP ===");
    crate::utils::debug_log("GRABBER", "Copy this template to your config.yaml grabber_rules section:");
    crate::utils::debug_log("GRABBER", "");
    crate::utils::debug_log("GRABBER", &format!("  - name: \"{} Rule\"", context.app_name));
    crate::utils::debug_log("GRABBER", "    matcher: |");
    
    // Generate a smart matcher based on the context
    if context.bundle_id.contains("chrome") || context.bundle_id.contains("brave") {
        crate::utils::debug_log("GRABBER", &format!("      if (bundleId === \"{}\" && props.url) {{", context.bundle_id));
        crate::utils::debug_log("GRABBER", "        return props.url;");
        crate::utils::debug_log("GRABBER", "      }");
    } else if context.bundle_id == "com.apple.Safari" {
        crate::utils::debug_log("GRABBER", &format!("      if (bundleId === \"{}\" && props.url) {{", context.bundle_id));
        crate::utils::debug_log("GRABBER", "        return props.url;");
        crate::utils::debug_log("GRABBER", "      }");
    } else if context.bundle_id == "com.apple.finder" {
        crate::utils::debug_log("GRABBER", &format!("      if (bundleId === \"{}\" && props.path) {{", context.bundle_id));
        crate::utils::debug_log("GRABBER", "        return props.path;");
        crate::utils::debug_log("GRABBER", "      }");
    } else if context.window_title.len() > 3 {
        crate::utils::debug_log("GRABBER", &format!("      if (bundleId === \"{}\" && title) {{", context.bundle_id));
        crate::utils::debug_log("GRABBER", "        return title;  // or extract what you need from title");
        crate::utils::debug_log("GRABBER", "      }");
    } else {
        crate::utils::debug_log("GRABBER", &format!("      if (bundleId === \"{}\") {{", context.bundle_id));
        crate::utils::debug_log("GRABBER", "        return app;  // or return some identifier");
        crate::utils::debug_log("GRABBER", "      }");
    }
    
    crate::utils::debug_log("GRABBER", "      return null;");
    crate::utils::debug_log("GRABBER", &format!("    action: \"{}\"", inferred_action));
    
    if rules.is_empty() {
        crate::utils::debug_log("GRABBER", "    # Optional group for organizing commands:");
        crate::utils::debug_log("GRABBER", "    # group: \"Grabbed\"");
    }
    
    crate::utils::debug_log("GRABBER", "");
    crate::utils::debug_log("GRABBER", "=== JAVASCRIPT VARIABLES AVAILABLE ===");
    crate::utils::debug_log("GRABBER", "In your matcher JavaScript, you can use:");
    crate::utils::debug_log("GRABBER", &format!("  app = \"{}\"", context.app_name));
    crate::utils::debug_log("GRABBER", &format!("  bundleId = \"{}\"", context.bundle_id));
    crate::utils::debug_log("GRABBER", &format!("  title = \"{}\"", context.window_title));
    crate::utils::debug_log("GRABBER", "  props = {");
    for (key, value) in context.properties.as_object().unwrap_or(&serde_json::Map::new()) {
        crate::utils::debug_log("GRABBER", &format!("    {}: \"{}\"", key, value.as_str().unwrap_or("(complex value)")));
    }
    crate::utils::debug_log("GRABBER", "  }");
    crate::utils::debug_log("GRABBER", "");
    crate::utils::debug_log("GRABBER", "################################################################################");
    crate::utils::debug_log("GRABBER", "");
}

/// Infer what action and arg would be appropriate for this context
fn infer_action_and_arg(context: &AppContext) -> (String, Option<String>) {
    // Check for browser
    if context.bundle_id.contains("chrome") || context.bundle_id.contains("brave") {
        if let Some(url) = context.properties.get("url").and_then(|v| v.as_str()) {
            return ("url".to_string(), Some(url.to_string()));
        }
        return ("url".to_string(), Some("props.url".to_string()));
    }
    
    if context.bundle_id == "com.apple.Safari" {
        if let Some(url) = context.properties.get("url").and_then(|v| v.as_str()) {
            return ("safari".to_string(), Some(url.to_string()));
        }
        return ("safari".to_string(), Some("props.url".to_string()));
    }
    
    // Check for Finder
    if context.bundle_id == "com.apple.finder" {
        if let Some(path) = context.properties.get("path").and_then(|v| v.as_str()) {
            return ("folder".to_string(), Some(path.to_string()));
        }
        return ("folder".to_string(), Some("props.path".to_string()));
    }
    
    // Check for known apps
    if context.app_name == "Obsidian" {
        return ("obs".to_string(), Some("(extracted from title)".to_string()));
    }
    
    if context.bundle_id == "com.microsoft.VSCode" {
        return ("doc".to_string(), Some("(extracted from title)".to_string()));
    }
    
    // Default to doc action
    ("doc".to_string(), Some(context.window_title.clone()))
}

/// Perform a grab operation: capture context and match against rules
pub fn grab(config: &Config) -> Result<(String, Command), String> {
    crate::utils::debug_log("GRABBER", "");
    crate::utils::debug_log("GRABBER", "################################################################################");
    crate::utils::debug_log("GRABBER", "##################### GRABBER OPERATION STARTED ###########################");
    crate::utils::debug_log("GRABBER", "################################################################################");
    crate::utils::debug_log("GRABBER", "");
    
    // Capture the active application context
    let context = capture_active_app()?;
    let context = enrich_context(context);
    
    // Get grabber rules from config
    let rules = config.grabber_rules.as_ref()
        .ok_or_else(|| {
            let error_msg = "No grabber rules configured in config.yaml";
            crate::utils::debug_log("GRABBER", &format!("ERROR: {}", error_msg));
            crate::utils::debug_log("GRABBER", "Add a 'grabber_rules:' section to your config.yaml with rules like:");
            crate::utils::debug_log("GRABBER", "grabber_rules:");
            crate::utils::debug_log("GRABBER", "  - name: \"Example Rule\"");
            crate::utils::debug_log("GRABBER", "    matcher: |");
            crate::utils::debug_log("GRABBER", "      if (bundleId === \"com.example.app\") {");
            crate::utils::debug_log("GRABBER", "        return title;");
            crate::utils::debug_log("GRABBER", "      }");
            crate::utils::debug_log("GRABBER", "      return null;");
            crate::utils::debug_log("GRABBER", "    action: \"doc\"");
            error_msg.to_string()
        })?;
    
    crate::utils::debug_log("GRABBER", &format!("Loaded {} grabber rules from config", rules.len()));
    
    // Always output the summary block for rule creation
    output_grabber_summary(&context, rules, config);
    
    // Match against rules
    match match_grabber_rules(&context, rules, config) {
        Some((rule_name, command)) => {
            crate::utils::debug_log("GRABBER", "");
            crate::utils::debug_log("GRABBER", "################################################################################");
            crate::utils::debug_log("GRABBER", "##################### GRABBER SUCCESS! ####################################");
            crate::utils::debug_log("GRABBER", "################################################################################");
            crate::utils::debug_log("GRABBER", &format!("Matched rule: {}", rule_name));
            crate::utils::debug_log("GRABBER", &format!("Generated command action: {}", command.action));
            crate::utils::debug_log("GRABBER", &format!("Generated command arg: {}", command.arg));
            crate::utils::debug_log("GRABBER", "");
            Ok((rule_name, command))
        }
        None => {
            let error_msg = format!("No grabber rule matched for {} ({})", context.app_name, context.bundle_id);
            crate::utils::debug_log("GRABBER", "");
            crate::utils::debug_log("GRABBER", "################################################################################");
            crate::utils::debug_log("GRABBER", "##################### GRABBER NO MATCH #################################");
            crate::utils::debug_log("GRABBER", "################################################################################");
            crate::utils::debug_log("GRABBER", &format!("ERROR: {}", error_msg));
            crate::utils::debug_log("GRABBER", "");
            Err(error_msg)
        }
    }
}