//! Test Enhanced Command Context
//!
//! Tests the enhanced JavaScript context that provides full command data.

use hookanchor::launcher::launch;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Testing Enhanced JavaScript Command Context ===\n");
    
    // Note: This requires the launcher to be configured with a test JS action
    println!("This test demonstrates the enhanced command context available to JavaScript actions.");
    println!("JavaScript actions now have access to:");
    println!("  - ARG: Command argument");
    println!("  - ACTION: Action name");
    println!("  - FULL_COMMAND: Complete original command");
    println!("  - COMMAND: JSON object with complete metadata");
    println!();
    
    // Test with a simple action first
    println!("Testing simple app launch (non-JS action):");
    match launch("app TextEdit") {
        Ok(()) => println!("✅ Simple action completed successfully"),
        Err(e) => println!("❌ Simple action failed: {:?}", e),
    }
    println!();
    
    // Test JavaScript action context
    println!("Testing JavaScript action with enhanced context:");
    println!("This would normally execute a JS action with full command context.");
    println!("See the config file js_actions section for examples of using:");
    println!("  - ARG variable");
    println!("  - ACTION variable");
    println!("  - FULL_COMMAND variable");
    println!("  - COMMAND JSON object");
    
    println!("\n=== Command Context Benefits ===");
    println!("🎯 Full command awareness - JavaScript knows what action is running");
    println!("📝 Access to all data - argument, action name, full command, metadata");
    println!("🕐 Timestamp tracking - when the command was executed");
    println!("🔍 Action type detection - distinguish JS actions from other types");
    println!("🧩 Enhanced logic - conditional behavior based on command context");
    
    println!("\n=== Example Usage in Config ===");
    println!("js_actions:");
    println!("  context_demo: |");
    println!("    log('Action: ' + ACTION);");
    println!("    log('Argument: ' + ARG);");
    println!("    log('Full command: ' + FULL_COMMAND);");
    println!("    const cmd = JSON.parse(COMMAND);");
    println!("    log('Timestamp: ' + cmd.timestamp);");
    
    Ok(())
}