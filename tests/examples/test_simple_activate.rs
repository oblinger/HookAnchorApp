//! Test Simple Activate
//!
//! Tests a simplified version of the activate functionality.

use hookanchor::js_runtime::execute_business_logic;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Testing Simple Activate ===\n");
    
    let script = r#"
        function simpleActivate(path) {
            log("=== Activating Anchor: " + path + " ===");
            
            // Check if the path exists and is a directory
            if (!isDirectory(path)) {
                error("Path does not exist or is not a directory: " + path);
                return false;
            }
            
            // Change to the directory
            const cdResult = change_directory(path);
            log("Change directory result: " + cdResult);
            
            // Open the folder
            const openResult = open_folder(path);
            log("Open folder result: " + openResult);
            
            // Check for tmux config
            const tmuxConfig = path + "/.tmuxp.yaml";
            if (fileExists(tmuxConfig)) {
                log("Found tmux config: " + tmuxConfig);
                // Note: Not actually starting tmux in test
            } else {
                log("No tmux config found");
            }
            
            // Check for Claude config
            if (fileExists("CLAUDE.md")) {
                log("Found CLAUDE.md file");
            } else {
                log("No CLAUDE.md file found");
            }
            
            log("=== Activation complete ===");
            return true;
        }
        
        // Test with /tmp directory
        const result = simpleActivate("/tmp");
        result ? "success" : "failed"
    "#;
    
    match execute_business_logic(script) {
        Ok(result) => {
            println!("✅ Simple activate completed!");
            println!("Result: {}", result);
        }
        Err(e) => {
            println!("❌ Simple activate failed: {}", e);
        }
    }
    
    Ok(())
}