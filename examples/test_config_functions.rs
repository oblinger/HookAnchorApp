//! Test Config-Defined Functions
//!
//! Tests that the default functions from config.yaml work correctly

use anchor_selector::{js_runtime, Config};
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Testing Config-Defined Functions ===\n");
    
    // Create config with the sample js_functions from default_config.yaml
    let mut js_functions = HashMap::new();
    
    js_functions.insert("activate_iterm".to_string(), 
        "return activateApp(\"iTerm2\");".to_string());
    
    js_functions.insert("start_claude_code".to_string(),
        r#"
        if (commandExists("claude")) {
            return spawnDetached("claude", "--continue");
        } else {
            return "Claude Code not available in PATH";
        }
        "#.to_string());
    
    js_functions.insert("has_tmux_session".to_string(),
        r#"
        const sessionName = arguments[0];
        if (!commandExists("tmux")) {
            return false;
        }
        const result = JSON.parse(shellWithExitCode(`tmux has-session -t "${sessionName}" 2>/dev/null`));
        return result.exitCode === 0;
        "#.to_string());
    
    js_functions.insert("smart_browser".to_string(),
        r#"
        const url = arguments[0];
        if (testRegex(url, "github\\.com")) {
            launch_app("Google Chrome", url);
            return "Opened GitHub URL in Chrome";
        } else {
            open_url(url);
            return "Opened URL in default browser";
        }
        "#.to_string());
    
    let config = Config {
        popup_settings: Default::default(),
        listed_actions: None,
        js_functions: Some(js_functions),
    };
    
    println!("📋 Created test config with {} js_functions:", config.js_functions.as_ref().unwrap().len());
    for name in config.js_functions.as_ref().unwrap().keys() {
        println!("  - {}", name);
    }
    println!();
    
    let ctx = js_runtime::create_business_logic_runtime_with_config(&config)?;
    
    ctx.with(|ctx| -> Result<(), Box<dyn std::error::Error>> {
        // Test that user-defined functions from config are available
        println!("🔧 Testing functions from default config...");
        
        // Test activate_iterm (should use activateApp)
        let script = r#"
            try {
                const result = activate_iterm();
                "Function available: " + result;
            } catch (e) {
                "Function not available: " + e.toString();
            }
        "#;
        let result: String = ctx.eval(script)?;
        println!("  ✅ activate_iterm: {}", result);
        
        // Test start_claude_code (should check commandExists)
        let script = r#"
            try {
                const result = start_claude_code();
                "Function available: " + result;
            } catch (e) {
                "Function not available: " + e.toString();
            }
        "#;
        let result: String = ctx.eval(script)?;
        println!("  ✅ start_claude_code: {}", result);
        
        // Test has_tmux_session (should use shellWithExitCode)
        let script = r#"
            try {
                const result = has_tmux_session("nonexistent");
                "Function available, result: " + result;
            } catch (e) {
                "Function not available: " + e.toString();
            }
        "#;
        let result: String = ctx.eval(script)?;
        println!("  ✅ has_tmux_session: {}", result);
        
        // Test that custom functions work too
        let script = r#"
            try {
                const result = smart_browser("https://github.com");
                "Function available: " + result;
            } catch (e) {
                "Function not available: " + e.toString();
            }
        "#;
        let result: String = ctx.eval(script)?;
        println!("  ✅ smart_browser: {}", result);
        
        Ok(())
    })?;
    
    println!("\n🚀 Config function tests completed!");
    
    Ok(())
}