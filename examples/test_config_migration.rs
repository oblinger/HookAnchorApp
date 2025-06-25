//! Test Config Migration
//!
//! Tests that the config migration works correctly for legacy config files

use anchor_selector::load_config;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Testing Config Migration ===\n");
    
    // Load the user's actual config (which is in legacy format)
    println!("🔍 Attempting to load user config...");
    let config = load_config();
    
    println!("📋 Loaded config successfully!");
    println!("  popup_settings.max_rows: {}", config.popup_settings.max_rows);
    println!("  popup_settings.max_columns: {}", config.popup_settings.max_columns);
    println!("  popup_settings.use_new_launcher: {}", config.popup_settings.use_new_launcher);
    
    if let Some(debug_log) = &config.popup_settings.debug_log {
        println!("  popup_settings.debug_log: {}", debug_log);
    } else {
        println!("  popup_settings.debug_log: None");
    }
    
    match &config.listed_actions {
        Some(actions) => {
            println!("  listed_actions: {} actions", actions.len());
            for action in actions {
                println!("    - {}", action);
            }
        },
        None => {
            println!("  listed_actions: None (will use defaults)");
        }
    }
    
    match &config.js_functions {
        Some(functions) => {
            println!("  js_functions: {} functions", functions.len());
            for name in functions.keys() {
                println!("    - {}", name);
            }
        },
        None => {
            println!("  js_functions: None (will use defaults)");
        }
    }
    
    println!("\n🚀 Config migration test completed!");
    
    Ok(())
}