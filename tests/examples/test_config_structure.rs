//! Test New Config Structure
//!
//! Tests the updated configuration structure with popup_settings and listed_actions.

use hookanchor::{load_config, Config, PopupSettings};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Testing Updated Config Structure ===\n");
    
    // Test 1: Load config and verify structure
    println!("1. Loading configuration...");
    let config = load_config();
    
    println!("✅ Config loaded successfully");
    println!("   Max rows: {}", config.popup_settings.max_rows);
    println!("   Max columns: {}", config.popup_settings.max_columns);
    println!("   Use new launcher: {}", config.popup_settings.use_new_launcher);
    println!("   Debug log: {:?}", config.popup_settings.debug_log);
    
    // Test 2: Check listed_actions
    println!("\n2. Checking listed_actions configuration...");
    match &config.listed_actions {
        Some(actions) => {
            println!("✅ Found {} configured actions:", actions.len());
            for (i, action) in actions.iter().enumerate() {
                println!("   {}. {}", i + 1, action);
            }
        }
        None => {
            println!("⚠️  No listed_actions found, using defaults");
        }
    }
    
    // Test 3: Test default config creation
    println!("\n3. Testing default config structure...");
    let default_config = Config::default();
    
    println!("✅ Default config created successfully");
    println!("   Default max rows: {}", default_config.popup_settings.max_rows);
    println!("   Default actions count: {}", 
             default_config.listed_actions.as_ref().map_or(0, |v| v.len()));
    
    // Test 4: Verify YAML parsing
    println!("\n4. Testing YAML serialization...");
    let yaml_content = r#"
popup_settings:
  max_rows: 12
  max_columns: 2
  use_new_launcher: true
  debug_log: "~/.test.log"

listed_actions:
  - "app"
  - "url"
  - "folder"
  - "chrome"
  - "anchor"
"#;
    
    let parsed_config: Config = serde_yaml::from_str(yaml_content)?;
    println!("✅ YAML parsing successful");
    println!("   Parsed max rows: {}", parsed_config.popup_settings.max_rows);
    println!("   Parsed actions: {:?}", parsed_config.listed_actions);
    
    println!("\n=== Key Benefits of New Structure ===");
    println!("🔧 Renamed settings to popup_settings for clarity");
    println!("📋 Added listed_actions for configurable command editor dropdown");
    println!("🎛️ Users can now customize which actions appear in editor");
    println!("📝 Users can reorder actions by preference");
    println!("➕ Users can add custom actions to the dropdown");
    
    println!("\n=== Example User Customization ===");
    println!("# config.yaml");
    println!("popup_settings:");
    println!("  max_rows: 20        # More rows");
    println!("  max_columns: 4      # More columns");
    println!("");
    println!("listed_actions:");
    println!("  - \"anchor\"          # Most used first");
    println!("  - \"app\"");
    println!("  - \"chrome\"");
    println!("  - \"my_custom_action\" # User's custom action");
    
    Ok(())
}