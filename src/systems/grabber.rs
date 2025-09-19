//! Grabber module for capturing active application context
//! 
//! This module captures information about the currently focused application
//! and matches it against grabber rules to create commands for reopening
//! the same document/URL/folder later.

use std::process::Command as ProcessCommand;
use serde::{Deserialize, Serialize};
use crate::core::Command;
use crate::core::Config;
use crate::execute::{get_action, get_default_patch_for_action};
use rquickjs::{Runtime, Context, Value};
use regex::Regex;

/// Information captured about the active application
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppContext {
    /// Application name (e.g., "Google Chrome")
    pub app_name: String,
    /// Bundle identifier (e.g., "com.google.Chrome")
    pub bundle_id: String,
    /// Application path (e.g., "/Applications/Google Chrome.app")
    pub app_path: String,
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
    /// Optional group for the created command (stored as patch in Command)
    #[serde(alias = "group")]
    pub patch: Option<String>,
}

/// Apply suffix mapping to a grabbed argument based on config patterns
/// Returns (original_arg, detected_suffix) - arg is NOT modified, only suffix is detected
fn apply_suffix_mapping(arg: &str, config: &Config) -> (String, Option<String>) {
    crate::utils::detailed_log("GRABBER", &format!("Applying suffix mapping to: '{}'", arg));

    if let Some(suffix_map) = config.grabber_suffix_map.as_ref() {
        crate::utils::detailed_log("GRABBER", &format!("Found {} suffix patterns to check", suffix_map.len()));
        for (suffix, pattern) in suffix_map.iter() {
            crate::utils::detailed_log("GRABBER", &format!("Testing pattern '{}' for suffix '{}'", pattern, suffix));
            if let Ok(regex) = Regex::new(pattern) {
                if regex.is_match(arg) {
                    crate::utils::detailed_log("GRABBER", &format!("Suffix match: '{}' -> detected suffix '{}' (arg unchanged)", pattern, suffix));
                    return (arg.to_string(), Some(suffix.clone()));
                } else {
                    crate::utils::detailed_log("GRABBER", &format!("Pattern '{}' did not match", pattern));
                }
            } else {
                crate::utils::detailed_log("GRABBER", &format!("Invalid regex pattern for suffix '{}': {}", suffix, pattern));
            }
        }
    } else {
        crate::utils::detailed_log("GRABBER", "No grabber_suffix_map found in config");
    }

    // No suffix matched, return original
    crate::utils::detailed_log("GRABBER", "No suffix patterns matched, returning original arg");
    (arg.to_string(), None)
}

/// Captures information about the currently focused application
fn capture_active_app() -> Result<AppContext, String> {
    
    // Use AppleScript to get information about the frontmost application
    let script = r#"
        tell application "System Events"
            set frontApp to first application process whose frontmost is true
            set appName to name of frontApp
            set bundleId to bundle identifier of frontApp
            
            -- Get the application file path
            set appPath to ""
            try
                set appFile to application file of frontApp
                set appPath to POSIX path of appFile
            end try
            
            -- Try to get window title
            set windowTitle to ""
            try
                set windowTitle to name of window 1 of frontApp
            end try
            
            -- Output as pipe-delimited for easy parsing
            return appName & "|" & bundleId & "|" & appPath & "|" & windowTitle
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
    
    if parts.len() < 4 {
        return Err("Failed to parse AppleScript output".to_string());
    }
    
    let context = AppContext {
        app_name: parts[0].to_string(),
        bundle_id: parts[1].to_string(),
        app_path: parts[2].to_string(),
        window_title: parts[3].to_string(),
        properties: serde_json::json!({}),
    };
    
    
    Ok(context)
}

/// Get additional browser-specific information
fn get_browser_info(bundle_id: &str) -> Option<String> {
    
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
        "com.google.Chrome.beta" => {
            Some(r#"
                tell application "Google Chrome Beta"
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

/// Get Obsidian active file URL by triggering copy URL shortcut
fn get_obsidian_url() -> Option<String> {
    crate::utils::detailed_log("GRAB", "Getting Obsidian URL via command palette (Cmd+P)");
    crate::utils::detailed_log("GRAB", "🔥 NEW CODE VERSION - Improved error handling active");

    // First, save the current clipboard content
    let original_clipboard = ProcessCommand::new("osascript")
        .arg("-e")
        .arg("get the clipboard as string")
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .unwrap_or_default();

    let script = r#"
        -- Clear clipboard first to detect if command succeeded
        set the clipboard to ""

        -- Open Obsidian command palette with Cmd+P
        tell application "System Events"
            tell process "Obsidian"
                key code 35 using {command down}  -- Cmd+P
            end tell
        end tell

        -- Wait for command palette to fully open
        delay 0.4

        -- Clear the palette first (in case something was typed before)
        tell application "System Events"
            tell process "Obsidian"
                key code 51  -- Delete key
                key code 51  -- Delete key again
                keystroke "a" using {command down}  -- Select all
                key code 51  -- Delete
            end tell
        end tell

        -- Type the command to copy Obsidian URL
        tell application "System Events"
            tell process "Obsidian"
                keystroke "copy obsidian url"
            end tell
        end tell

        -- Small delay for autocomplete
        delay 0.3

        -- Press Enter to execute the command
        tell application "System Events"
            tell process "Obsidian"
                key code 36  -- Enter key
            end tell
        end tell

        -- Wait for clipboard to update
        delay 0.5

        -- Get clipboard content
        try
            set clipboardContent to (the clipboard as string)
            if clipboardContent starts with "obsidian://" then
                return clipboardContent
            else
                -- If no URL, maybe try pressing Escape to close palette
                tell application "System Events"
                    tell process "Obsidian"
                        key code 53  -- Escape key
                    end tell
                end tell
                return ""
            end if
        on error errMsg
            return "ERROR: " & errMsg
        end try
    "#;
    
    if let Ok(output) = ProcessCommand::new("osascript")
        .arg("-e")
        .arg(script)
        .output()
    {
        let result = String::from_utf8_lossy(&output.stdout).trim().to_string();

        if output.status.success() {
            if result.starts_with("obsidian://") {
                crate::utils::detailed_log("GRAB", &format!("Successfully got Obsidian URL: {}", result));
                return Some(result);
            } else if result.starts_with("ERROR:") {
                crate::utils::detailed_log("GRAB", &format!("AppleScript error: {}", result));
            } else if result.is_empty() {
                crate::utils::detailed_log("GRAB", "No Obsidian URL found in clipboard (command may have failed)");
            } else {
                crate::utils::detailed_log("GRAB", &format!("Unexpected clipboard content: {}", result));
            }
        } else {
            let error = String::from_utf8_lossy(&output.stderr);
            crate::utils::detailed_log("GRAB", &format!("AppleScript execution failed: {}", error));
        }
    } else {
        crate::utils::detailed_log("GRAB", "Failed to execute AppleScript");
    }

    None
}

/// Get Notion page URL by triggering copy link shortcut
fn get_notion_url() -> Option<String> {
    crate::utils::detailed_log("GRABBER_NOTION", "=== NOTION URL CAPTURE START ===");
    
    // First, try to read the clipboard in case user already copied the URL
    crate::utils::detailed_log("GRABBER_NOTION", "Step 1: Checking if clipboard already contains Notion URL");
    
    let check_clipboard_script = r#"
        try
            set clipContent to (the clipboard) as string
            if clipContent starts with "https://www.notion.so/" then
                return clipContent
            else
                return ""
            end if
        on error
            return ""
        end try
    "#;
    
    if let Ok(output) = ProcessCommand::new("osascript")
        .arg("-e")
        .arg(check_clipboard_script)
        .output()
    {
        let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if stdout.starts_with("https://www.notion.so/") {
            crate::utils::detailed_log("GRABBER_NOTION", &format!("✅ Found Notion URL in clipboard: {}", stdout));
            crate::utils::detailed_log("GRABBER_NOTION", "=== NOTION URL CAPTURE END (SUCCESS - from clipboard) ===");
            return Some(stdout);
        } else {
            crate::utils::detailed_log("GRABBER_NOTION", "No Notion URL in clipboard, trying keystroke method");
        }
    }
    
    // Try to trigger Notion's copy link shortcut (Cmd+L)
    // This requires Terminal/popup_server to have Accessibility permissions
    crate::utils::detailed_log("GRABBER_NOTION", "Step 2: Attempting to trigger Cmd+L in Notion");
    
    let trigger_copy_script = r#"
        -- Save current clipboard
        set oldClipboard to ""
        try
            set oldClipboard to (the clipboard)
        end try
        
        -- Clear clipboard first to detect if Cmd+L actually works
        set the clipboard to ""
        
        -- Make sure Notion is active and ready
        tell application "Notion" to activate
        delay 0.2
        
        -- Trigger Notion's copy link shortcut (Cmd+L)
        tell application "System Events"
            keystroke "l" using {command down}
        end tell
        
        -- Wait longer for clipboard to update (Notion can be slow)
        delay 1.0
        
        -- Get new clipboard content
        try
            set newClipboard to (the clipboard) as string
            if newClipboard starts with "https://www.notion.so/" then
                return newClipboard
            else if newClipboard is "" then
                -- Clipboard is still empty, Cmd+L didn't work
                -- Restore old clipboard
                try
                    set the clipboard to oldClipboard
                end try
                return "EMPTY"
            else
                -- Got something else, not a Notion URL
                return "NOT_NOTION:" & newClipboard
            end if
        on error errMsg
            return "ERROR: " & errMsg
        end try
    "#;
    
    crate::utils::detailed_log("GRABBER_NOTION", "Executing AppleScript to send Cmd+L");
    
    if let Ok(output) = ProcessCommand::new("osascript")
        .arg("-e")
        .arg(trigger_copy_script)
        .output()
    {
        let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
        
        crate::utils::detailed_log("GRABBER_NOTION", &format!("AppleScript stdout: '{}'", stdout));
        crate::utils::detailed_log("GRABBER_NOTION", &format!("AppleScript stderr: '{}'", stderr));
        crate::utils::detailed_log("GRABBER_NOTION", &format!("AppleScript exit status: {}", output.status.success()));
        
        if !stderr.is_empty() {
            crate::utils::detailed_log("GRABBER_NOTION", &format!("AppleScript error output: {}", stderr));
            if stderr.contains("not allowed to send keystrokes") {
                crate::utils::detailed_log("GRABBER_NOTION", "❌ PERMISSION ERROR: osascript not allowed to send keystrokes");
                crate::utils::detailed_log("GRABBER_NOTION", "This is a known macOS limitation when running from popup_server");
                crate::utils::detailed_log("GRABBER_NOTION", "");
                crate::utils::detailed_log("GRABBER_NOTION", "WORKAROUND: User should manually copy Notion URL first:");
                crate::utils::detailed_log("GRABBER_NOTION", "1. Click in Notion to focus the page");
                crate::utils::detailed_log("GRABBER_NOTION", "2. Press Cmd+L to copy the page URL");  
                crate::utils::detailed_log("GRABBER_NOTION", "3. Then trigger the grabber");
                crate::utils::detailed_log("GRABBER_NOTION", "");
                crate::utils::detailed_log("GRABBER_NOTION", "=== NOTION URL CAPTURE END (FAILED - permissions) ===");
                
                // Show user-friendly message
                crate::utils::detailed_log("GRABBER", "⚠️ Cannot send keystrokes from popup_server");
                crate::utils::detailed_log("GRABBER", "Please copy Notion URL manually first (Cmd+L), then grab");
            }
        }
        
        if output.status.success() {
            crate::utils::detailed_log("GRABBER_NOTION", &format!("AppleScript returned: '{}'", stdout));
            
            if stdout.starts_with("https://www.notion.so/") {
                crate::utils::detailed_log("GRABBER_NOTION", &format!("✅ SUCCESS: Got Notion URL: {}", stdout));
                crate::utils::detailed_log("GRABBER_NOTION", "=== NOTION URL CAPTURE END (SUCCESS) ===");
                return Some(stdout);
            } else if stdout == "EMPTY" {
                crate::utils::detailed_log("GRABBER_NOTION", "❌ FAILED: Clipboard empty after Cmd+L");
                crate::utils::detailed_log("GRABBER_NOTION", "Possible causes: permissions issue or not on a Notion page");
            } else if stdout.starts_with("NOT_NOTION:") {
                let content = &stdout[11..];
                crate::utils::detailed_log("GRABBER", &format!("Cmd+L copied non-Notion content: '{}'", 
                    if content.len() > 50 { 
                        format!("{}...", &content[..50]) 
                    } else { 
                        content.to_string() 
                    }
                ));
            } else if stdout.starts_with("ERROR:") {
                crate::utils::detailed_log("GRABBER", &format!("AppleScript error: {}", &stdout[6..]));
            }
        } else {
            crate::utils::detailed_log("GRABBER", "osascript command failed");
        }
    } else {
        crate::utils::detailed_log("GRABBER", "Failed to execute osascript");
    }
    
    crate::utils::detailed_log("GRABBER_NOTION", "Step 3: Fallback - user needs to copy URL manually");
    crate::utils::detailed_log("GRABBER", "📋 Please copy Notion URL first (Cmd+L), then grab again");
    crate::utils::detailed_log("GRABBER_NOTION", "=== NOTION URL CAPTURE END (FAILED - manual copy needed) ===");
    None
}

/// Get Finder-specific information including selection
fn get_finder_info() -> Option<serde_json::Value> {
    
    let mut finder_info = serde_json::json!({});
    
    // First get the current folder path
    let path_script = r#"
        tell application "Finder"
            try
                if (count of windows) > 0 then
                    set frontWindow to front window
                    set windowPath to (POSIX path of (target of frontWindow as alias))
                    return windowPath
                else
                    return "--ERROR--"
                end if
            on error
                return "--ERROR--"
            end try
        end tell
    "#;
    
    if let Ok(output) = ProcessCommand::new("osascript")
        .arg("-e")
        .arg(path_script)
        .output()
    {
        if output.status.success() {
            let result = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if !result.is_empty() && result != "--ERROR--" {
                finder_info["path"] = serde_json::Value::String(result);
            }
        }
    }
    
    // Then try to get selected item
    let selection_script = r#"
        tell application "Finder"
            try
                set selectedItems to selection
                if (count of selectedItems) > 0 then
                    set selectedItem to item 1 of selectedItems
                    set thePath to POSIX path of (selectedItem as alias)
                    set itemClass to class of selectedItem
                    if itemClass is folder then
                        set isFolder to "true"
                    else
                        set isFolder to "false"
                    end if
                    return thePath & "|" & isFolder
                else
                    return "--NO-SELECTION--"
                end if
            on error errorMessage
                return "--ERROR: " & errorMessage & "--"
            end try
        end tell
    "#;
    
    if let Ok(output) = ProcessCommand::new("osascript")
        .arg("-e")
        .arg(selection_script)
        .output()
    {
        if output.status.success() {
            let result = String::from_utf8_lossy(&output.stdout).trim().to_string();
            crate::utils::detailed_log("GRABBER", &format!("Selection script result: '{}'", result));
            
            if !result.is_empty() && result != "--NO-SELECTION--" && !result.starts_with("--ERROR:") {
                let parts: Vec<&str> = result.split('|').collect();
                if parts.len() >= 2 {
                    crate::utils::detailed_log("GRABBER", &format!("Found selection: '{}', isDirectory: '{}'", parts[0], parts[1]));
                    finder_info["selection"] = serde_json::Value::String(parts[0].to_string());
                    finder_info["selectionIsDirectory"] = serde_json::Value::Bool(parts[1] == "true");
                } else {
                    crate::utils::detailed_log("GRABBER", &format!("Invalid selection result format: '{}'", result));
                }
            } else {
                crate::utils::detailed_log("GRABBER", &format!("No selection detected: '{}'", result));
            }
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            crate::utils::detailed_log("GRABBER", &format!("Selection script failed: {}", stderr));
        }
    } else {
        crate::utils::detailed_log("GRABBER", "Failed to run selection script");
    }
    
    // Return finder_info if we have any data
    if !finder_info.as_object().unwrap().is_empty() {
        Some(finder_info)
    } else {
        None
    }
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
fn enrich_context(mut context: AppContext) -> AppContext {
    
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
    crate::utils::detailed_log("GRABBER", &format!("Bundle ID: '{}', checking for Finder...", context.bundle_id));
    if context.bundle_id == "com.apple.finder" || context.bundle_id == "com.apple.Finder" {
        crate::utils::detailed_log("GRABBER", "Enriching Finder context...");
        if let Some(finder_info) = get_finder_info() {
            crate::utils::detailed_log("GRABBER", &format!("Got Finder info: {:?}", finder_info));
            // Merge the finder info into properties
            if let Some(obj) = finder_info.as_object() {
                for (key, value) in obj {
                    context.properties[key] = value.clone();
                    crate::utils::detailed_log("GRABBER", &format!("Added props.{}: {:?}", key, value));
                }
                
                // Add recommended action based on the selected path
                if let Some(selection_path) = obj.get("selection") {
                    if let Some(path_str) = selection_path.as_str() {
                        let path = std::path::Path::new(path_str);
                        let recommended_action = get_action(path);
                        context.properties["recommendedAction"] = serde_json::Value::String(recommended_action.to_string());
                        crate::utils::detailed_log("GRABBER", &format!("Added recommended action: {}", recommended_action));
                    }
                }
            }
        } else {
            crate::utils::detailed_log("GRABBER", "Failed to get Finder info");
        }
    }
    
    // Add Obsidian URL if applicable
    if context.bundle_id == "md.obsidian" {
        crate::utils::detailed_log("GRABBER", "Enriching Obsidian context...");
        if let Some(url) = get_obsidian_url() {
            crate::utils::detailed_log("GRABBER", &format!("Got Obsidian URL: {}", url));
            context.properties["url"] = serde_json::Value::String(url);
        } else {
            crate::utils::detailed_log("GRABBER", "Failed to get Obsidian URL");
        }
    }
    
    // Add Notion URL if applicable
    if context.bundle_id == "notion.id" {
        crate::utils::detailed_log("GRABBER", "Bundle ID is notion.id - attempting to enrich with URL");
        crate::utils::detailed_log("GRABBER", "Calling get_notion_url()...");
        
        if let Some(url) = get_notion_url() {
            crate::utils::detailed_log("GRABBER", &format!("✅ Got Notion URL: {}", url));
            crate::utils::detailed_log("GRABBER", "Adding URL to context.properties");
            context.properties["url"] = serde_json::Value::String(url.clone());
            crate::utils::detailed_log("GRABBER", &format!("Properties after adding URL: {:?}", context.properties));
        } else {
            crate::utils::detailed_log("GRABBER", "❌ Failed to get Notion URL - properties['url'] will be empty");
            crate::utils::detailed_log("GRABBER", &format!("Properties without URL: {:?}", context.properties));
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
fn match_grabber_rules(
    context: &AppContext,
    rules: &[GrabberRule],
    config: &Config,
) -> Option<(String, Command)> {
    if rules.is_empty() {
        crate::utils::detailed_log("GRABBER", "No grabber rules configured");
        return None;
    }
    
    crate::utils::detailed_log("GRABBER", &format!("Attempting {} grabber rules", rules.len()));
    
    // Load patches data for patch inference
    let (sys_data, _) = crate::core::sys_data::get_sys_data();
    let patches = &sys_data.patches;
    
    let rt = Runtime::new().ok()?;
    let ctx = Context::full(&rt).ok()?;
    
    // Set up the context in JavaScript
    let context_json = serde_json::to_string(context).ok()?;
    let setup_script = format!(r#"
        const context = {json};
        const app = context.app_name;
        const bundleId = context.bundle_id;
        const appPath = context.app_path;
        const title = context.window_title;
        const props = context.properties;
    "#, json = context_json);
    
    ctx.with(|ctx| {
        if ctx.eval::<(), _>(setup_script.as_bytes()).is_err() {
            return None;
        }
        
        // Try each rule
        for rule in rules.iter() {
            crate::utils::detailed_log("GRABBER", &format!("Evaluating rule '{}': {}", rule.name, rule.matcher));
            
            match ctx.eval::<Value, _>(rule.matcher.as_bytes()) {
                Ok(value) => {
                    // Check what type of value was returned
                    if value.is_null() || value.is_undefined() {
                        // Rule didn't match, continue to next rule
                        crate::utils::detailed_log("GRABBER", &format!("  Rule '{}' returned null/undefined - no match", rule.name));
                    } else if let Some(str_ref) = value.as_string() {
                        // String return - use current behavior
                        if let Ok(arg) = str_ref.to_string() {
                            crate::utils::detailed_log("GRABBER_MATCH", &format!("====== RULE MATCH: '{}' ======", rule.name));
                            crate::utils::detailed_log("GRABBER_MATCH", &format!("Matched argument: '{}'", arg));
                            crate::utils::detailed_log("GRABBER_MATCH", &format!("Argument length: {}", arg.len()));
                            crate::utils::detailed_log("GRABBER_MATCH", &format!("Rule action: '{}'", rule.action));
                            crate::utils::detailed_log("GRABBER_MATCH", &format!("Rule patch: '{:?}'", rule.patch));

                            // Apply suffix mapping to the argument
                            let (_, detected_suffix) = apply_suffix_mapping(&arg, config);
                            if let Some(ref suffix) = detected_suffix {
                                crate::utils::detailed_log("GRABBER_MATCH", &format!("Detected suffix '{}' for arg '{}'", suffix, arg));
                            }

                            // Rule matched and returned a string argument
                            let mut command = Command {
                                patch: rule.patch.clone().unwrap_or_default(),
                                command: String::new(), // Will be filled by user
                                action: rule.action.clone(),
                                arg: arg.clone(),
                                flags: detected_suffix.unwrap_or_default(), // Store suffix in flags field temporarily
                            };
                            
                            crate::utils::detailed_log("GRABBER_MATCH", &format!("Created command: action='{}', arg='{}'", command.action, command.arg));
                            
                            // Apply patch inference if no explicit patch was set
                            if command.patch.is_empty() {
                                // First try to get default patch for action type
                                if let Some(default_patch) = crate::execute::get_default_patch_for_action(&command.action) {
                                    crate::utils::detailed_log("GRABBER", &format!("Using default patch '{}' for action '{}'", default_patch, command.action));
                                    command.patch = default_patch.to_string();
                                } else if let Some(inferred_patch) = crate::core::commands::infer_patch(&command, patches) {
                                    crate::utils::detailed_log("GRABBER", &format!("Inferred patch '{}' for grabbed command", inferred_patch));
                                    command.patch = inferred_patch;
                                } else {
                                    crate::utils::detailed_log("GRABBER", "No patch could be inferred for grabbed command");
                                }
                            }
                            
                            return Some((rule.name.clone(), command));
                        }
                    } else if let Some(obj) = value.as_object() {
                        // Object return - extract command fields
                        crate::utils::detailed_log("GRABBER", &format!("Matched rule '{}' with object", rule.name));
                        
                        // Extract action, arg, and group from the object
                        let action = obj.get::<_, String>("action").ok()
                            .or_else(|| rule.action.clone().into());
                        let arg = obj.get::<_, String>("arg").ok()
                            .unwrap_or_default();
                        let group = obj.get::<_, String>("group").ok()
                            .or_else(|| rule.patch.clone())
                            .unwrap_or_default();
                        
                        if let Some(action) = action {
                            crate::utils::detailed_log("GRABBER", &format!("  action: '{}', arg: '{}', patch: '{}'", action, arg, group));

                            // Apply suffix mapping to the argument
                            let (_, detected_suffix) = apply_suffix_mapping(&arg, config);
                            if let Some(ref suffix) = detected_suffix {
                                crate::utils::detailed_log("GRABBER_MATCH", &format!("Detected suffix '{}' for arg '{}'", suffix, arg));
                            }

                            let mut command = Command {
                                patch: group,
                                command: String::new(), // Will be filled by user
                                action,
                                arg: arg.clone(),
                                flags: detected_suffix.unwrap_or_default(), // Store suffix in flags field temporarily
                            };
                            
                            // Apply patch inference if no explicit patch was set
                            if command.patch.is_empty() {
                                // First try to get default patch for action type
                                if let Some(default_patch) = get_default_patch_for_action(&command.action) {
                                    crate::utils::detailed_log("GRABBER", &format!("Using default patch '{}' for action '{}'", default_patch, command.action));
                                    command.patch = default_patch.to_string();
                                } else if let Some(inferred_patch) = crate::core::commands::infer_patch(&command, patches) {
                                    crate::utils::detailed_log("GRABBER", &format!("Inferred patch '{}' for grabbed command", inferred_patch));
                                    command.patch = inferred_patch;
                                } else {
                                    crate::utils::detailed_log("GRABBER", "No patch could be inferred for grabbed command");
                                }
                            }
                            
                            return Some((rule.name.clone(), command));
                        } else {
                            crate::utils::detailed_log("GRABBER", "  Error: object missing 'action' field");
                        }
                    }
                    // Other return types (bool, number) are treated as no match
                }
                Err(e) => {
                    // Log JavaScript errors for debugging
                    crate::utils::detailed_log("GRABBER", &format!("Rule '{}' error: {:?}", rule.name, e));
                }
            }
        }
        
        crate::utils::detailed_log("GRABBER", "No rules matched");
        None
    })
}

/// Output the captured application context (always shown)
fn output_grabber_context_summary(context: &AppContext) {
    crate::utils::detailed_log("GRABBER", "=== CAPTURED APPLICATION CONTEXT ===");
    crate::utils::detailed_log("GRABBER", &format!("App: '{}'", context.app_name));
    crate::utils::detailed_log("GRABBER", &format!("Bundle ID: '{}'", context.bundle_id));
    crate::utils::detailed_log("GRABBER", &format!("App Path: '{}'", context.app_path));
    crate::utils::detailed_log("GRABBER", &format!("Title: '{}'", context.window_title));
    if let Some(props_obj) = context.properties.as_object() {
        if !props_obj.is_empty() {
            for (key, value) in props_obj {
                crate::utils::detailed_log("GRABBER", &format!("props.{}: '{}'", key, value.as_str().unwrap_or("(complex value)")));
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
    template.push_str(&format!("appPath = \"{}\"\n", context.app_path));
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
        // For general apps, return the app path for open_app action
        template.push_str(&format!("\"bundleId === '{}' ? appPath : null\"\n", context.bundle_id));
    }
    
    template.push_str("    action: \"doc\"  # Change to appropriate action type\n");
    template.push_str("    # Optional group for organizing commands:\n");
    template.push_str("    # patch: \"Grabbed\"\n");
    
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

/// Main function for capturing and matching context (used by UI)
pub fn grab(config: &Config) -> Result<GrabResult, String> {
    crate::utils::detailed_log("GRAB", "grabber::grab() starting - capturing active app");
    
    // Capture the active application context
    let capture_start = std::time::Instant::now();
    let context = capture_active_app()?;
    crate::utils::detailed_log("GRAB", &format!("capture_active_app() completed in {}ms", capture_start.elapsed().as_millis()));
    let context = enrich_context(context);
    
    // Always output the context summary first, regardless of rules
    output_grabber_context_summary(&context);
    
    // Always log the template dialog content so users can see it in logs
    let template_text = generate_rule_template_text(&context);
    crate::utils::detailed_log("GRABBER", "");
    crate::utils::detailed_log("GRABBER", "=== TEMPLATE INFORMATION (ALWAYS LOGGED) ===");
    for line in template_text.lines() {
        crate::utils::detailed_log("GRABBER", line);
    }
    crate::utils::detailed_log("GRABBER", "");
    
    // Try to match against rules if they exist
    if let Some(rules_vec) = config.grabber_rules.as_ref() {
        if let Some((rule_name, command)) = match_grabber_rules(&context, rules_vec, config) {
            crate::utils::detailed_log("GRABBER", &format!("Opening command editor with matched rule: '{}'", rule_name));
            return Ok(GrabResult::RuleMatched(rule_name, command));
        } else {
            crate::utils::detailed_log("GRABBER", "No rules matched - using app fallback");
        }
    } else {
        crate::utils::detailed_log("GRABBER", "No grabber rules configured - using app fallback");
    }
    
    // Create a fallback app command if we have an app path
    if !context.app_path.is_empty() {
        let mut command = Command {
            patch: "App".to_string(),  // Default patch for apps
            command: String::new(),     // Will be filled by user
            action: "open_app".to_string(),
            arg: context.app_path.clone(),
            flags: String::new(),
        };
        
        // Try to get default patch for open_app action
        if let Some(default_patch) = get_default_patch_for_action("open_app") {
            command.patch = default_patch.to_string();
        }
        
        crate::utils::detailed_log("GRABBER", &format!("Created fallback app command with path: '{}'", context.app_path));
        return Ok(GrabResult::RuleMatched("App Fallback".to_string(), command));
    }
    
    Ok(GrabResult::NoRuleMatched(context))
}

/// Debug function for CLI testing and rule development
pub fn grab_debug(config: &Config) -> Result<AppContext, String> {
    // Capture and enrich context
    let context = capture_active_app()?;
    println!("📱 Captured context:");
    println!("  App: {}", context.app_name);
    println!("  Bundle: {}", context.bundle_id);
    println!("  Path: {}", context.app_path);
    println!("  Title: {}", context.window_title);
    
    let context = enrich_context(context);
    println!("🔍 Enriched properties: {}", serde_json::to_string_pretty(&context.properties).unwrap_or_default());
    
    // Test against rules if available
    if let Some(rules) = config.grabber_rules.as_ref() {
        if let Some((rule_name, command)) = match_grabber_rules(&context, rules, config) {
            println!("✅ Rule matched: '{}' -> {}", rule_name, command.to_new_format());
        } else {
            println!("❌ No rules matched");
            let template = generate_rule_template_text(&context);
            println!("💡 Suggested rule template:\n{}", template);
        }
    } else {
        println!("ℹ️  No grabber rules configured");
    }
    
    Ok(context)
}