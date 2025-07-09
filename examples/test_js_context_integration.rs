//! Test JavaScript Context Integration
//!
//! Tests JavaScript actions with the enhanced command context.

use hookanchor::js_runtime::execute_business_logic;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Testing JavaScript Command Context Integration ===\n");
    
    // Simulate what happens when a JavaScript action is executed
    // This mimics the enhanced context that's now available
    
    let test_script = r#"
        // Simulate the enhanced context variables that are now available
        // In real usage, these are set by the launcher
        const ARG = "/tmp/test-project";
        const ACTION = "anchor";  
        const FULL_COMMAND = "anchor /tmp/test-project";
        const COMMAND = JSON.stringify({
            "command": "anchor /tmp/test-project",
            "action": "anchor", 
            "arg": "/tmp/test-project",
            "full_command": "anchor /tmp/test-project",
            "action_type": "javascript",
            "timestamp": Math.floor(Date.now() / 1000)
        });
        
        log("=== Enhanced Command Context Demo ===");
        
        // 1. Access individual command components
        log("Command argument (ARG): " + ARG);
        log("Action name (ACTION): " + ACTION);
        log("Full command (FULL_COMMAND): " + FULL_COMMAND);
        
        // 2. Parse complete command metadata
        const cmdData = JSON.parse(COMMAND);
        log("Command timestamp: " + cmdData.timestamp);
        log("Action type: " + cmdData.action_type);
        
        // 3. Conditional logic based on command context
        if (ACTION === "anchor") {
            log("This is an anchor activation - setting up development environment");
            
            const projectPath = ARG;
            const projectName = basename(projectPath);
            
            log("Project path: " + projectPath);
            log("Project name: " + projectName);
            
            // Environment-aware setup
            if (fileExists(projectPath + "/.tmuxp.yaml")) {
                log("Found tmux config - would start development session");
            } else if (fileExists(projectPath + "/CLAUDE.md")) {
                log("Found Claude config - would start Claude Code");
            } else {
                log("Simple folder activation");
            }
            
        } else if (ACTION === "obs") {
            log("This is an Obsidian action - would handle note opening");
            
        } else if (ACTION === "url") {
            log("This is a URL action - would handle browser routing");
        }
        
        // 4. Smart behavior based on command patterns
        if (FULL_COMMAND.includes("work")) {
            log("Work-related command detected - using work browser/tools");
        }
        
        // 5. Logging with context
        log("Command executed at: " + new Date(cmdData.timestamp * 1000).toISOString());
        
        log("=== Context Demo Complete ===");
        "success"
    "#;
    
    match execute_business_logic(test_script) {
        Ok(result) => {
            println!("\n✅ JavaScript context integration test completed!");
            println!("Result: {}", result);
            
            println!("\n=== Key Capabilities Demonstrated ===");
            println!("🔍 Full command introspection - JavaScript knows exactly what was run");
            println!("📊 Metadata access - timestamps, action types, etc.");
            println!("🧠 Smart conditional logic - different behavior per action type");
            println!("🏗️ Environment detection - adapt to project structure");
            println!("⏰ Execution tracking - when commands were run");
            
            println!("\n=== Real-World Applications ===");
            println!("• Different anchor behavior for work vs personal projects");
            println!("• URL routing based on domain patterns and user context");
            println!("• Development environment setup based on project type detection");
            println!("• Logging and analytics with full command context");
            println!("• Error handling with complete failure context");
        }
        Err(e) => {
            println!("❌ JavaScript context test failed: {}", e);
        }
    }
    
    Ok(())
}