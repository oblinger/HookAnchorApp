use hookanchor::core::{sys_data, Config};
use std::env;

fn main() {
    // Set test config if specified
    if let Ok(config_path) = env::var("HOOKANCHOR_CONFIG") {
        env::set_var("HOOKANCHOR_CONFIG_PATH", config_path);
    }
    
    // Initialize config first
    sys_data::initialize_config();
    
    // Then get sys_data
    let _ = sys_data::get_sys_data();
    let config = sys_data::get_config();
    
    println!("Templates: {:?}", config.templates.as_ref().map(|t| t.len()));
    println!("Actions: {:?}", config.actions.as_ref().map(|a| a.len()));
    
    if let Some(actions) = &config.actions {
        println!("\nAvailable actions:");
        for (name, action) in actions {
            println!("  - {} (type: {})", name, action.action_type);
            if let Some(desc) = &action.description {
                println!("    Description: {}", desc);
            }
        }
        
        // Test expand_string functionality
        println!("\nTesting string expansion:");
        let context = hookanchor::core::unified_actions::ActionContext::new("test input".to_string());
        
        let test_cases = vec![
            "Hello {{input}}!",
            "Length: {{input.length}}",
            "Upper: {{input.toUpperCase()}}",
            "Date: {{YYYY}}-{{MM}}-{{DD}}",
        ];
        
        for test in test_cases {
            match hookanchor::core::unified_actions::expand_string(test, &context) {
                Ok(result) => println!("  '{}' -> '{}'", test, result),
                Err(e) => println!("  '{}' -> ERROR: {}", test, e),
            }
        }
    }
}