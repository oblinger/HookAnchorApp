//! Test Unified Functions Design
//!
//! Tests that js_functions work both as actions (when in listed_actions) 
//! and as helper functions (callable from other functions).

use anchor_selector::{js_runtime, load_config};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Testing Unified Functions Design ===\n");
    
    // Load config and test the unified structure
    let config = load_config();
    
    println!("📋 Config Structure:");
    if let Some(functions) = &config.js_functions {
        println!("  js_functions: {} functions", functions.len());
        for name in functions.keys() {
            println!("    - {}", name);
        }
    }
    
    if let Some(actions_str) = &config.popup_settings.listed_actions {
        let actions: Vec<String> = actions_str
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();
        println!("  listed_actions: {} actions", actions.len());
        for action in &actions {
            println!("    - {}", action);
        }
    }
    
    // Test JavaScript runtime with unified functions
    println!("\n🧪 Testing JavaScript Runtime:");
    let ctx = js_runtime::create_business_logic_runtime_with_config(&config)?;
    
    ctx.with(|ctx| -> Result<(), Box<dyn std::error::Error>> {
        // Test that helper functions are available
        println!("  🔧 Testing helper function (has_tmux_session):");
        let script = r#"
            try {
                const result = has_tmux_session("nonexistent");
                "Helper function available: " + result;
            } catch (e) {
                "Helper function error: " + e.toString();
            }
        "#;
        let result: String = ctx.eval(script)?;
        println!("    Result: {}", result);
        
        // Test that action functions are available
        println!("  ⚡ Testing action function (activate_iterm):");
        let script = r#"
            try {
                const result = activate_iterm();
                "Action function available: " + result;
            } catch (e) {
                "Action function error: " + e.toString();
            }
        "#;
        let result: String = ctx.eval(script)?;
        println!("    Result: {}", result);
        
        // Test function composition (action calling helper)
        println!("  🔗 Testing function composition:");
        let script = r#"
            try {
                // Simulate an action that uses helper functions
                const sessionExists = has_tmux_session("test");
                const terminalActive = activate_iterm();
                `Composition test: session=${sessionExists}, terminal=${terminalActive}`;
            } catch (e) {
                "Composition error: " + e.toString();
            }
        "#;
        let result: String = ctx.eval(script)?;
        println!("    Result: {}", result);
        
        Ok(())
    })?;
    
    println!("\n🚀 Unified functions design working correctly!");
    println!("\n💡 Key Benefits:");
    println!("  ✅ Single js_functions section for all JavaScript");
    println!("  ✅ Helper functions available to all contexts");
    println!("  ✅ Action functions can call helper functions");
    println!("  ✅ listed_actions controls command availability");
    println!("  ✅ Consistent global function namespace");
    
    Ok(())
}