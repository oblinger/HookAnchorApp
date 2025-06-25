//! Test Full Anchor Action
//!
//! Demonstrates the complete user-customizable anchor action system.

use anchor_selector::js_runtime::execute_business_logic;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Testing Complete Anchor Action System ===\n");
    
    // This simulates what happens when a user runs: anchor /tmp
    // It demonstrates the complete user-customizable JavaScript action system
    
    let anchor_action_script = r#"
        // This is the anchor action from default_config.yaml
        // Users can completely customize this behavior
        
        const path = "/tmp";  // This would be {{arg}} in real usage
        log("Activating anchor: " + path);
        
        // Basic activation functionality (user-customizable)
        change_directory(path);
        open_folder(path);
        
        const anchorName = basename(path);
        const tmuxConfig = path + "/.tmuxp.yaml";
        
        if (fileExists(tmuxConfig)) {
            log("Found tmux config, would start session: " + anchorName);
            // start_tmux_session(tmuxConfig);  // Commented out for safety in test
            // activate_iterm();
        } else if (fileExists("CLAUDE.md")) {
            log("Found CLAUDE.md, would start Claude Code");
            // start_claude_code();  // Commented out for safety in test
        } else {
            log("No special development environment found");
        }
        
        log("User can completely customize this behavior by editing the action script");
        "anchor action complete"
    "#;
    
    println!("Executing anchor action script...\n");
    
    match execute_business_logic(anchor_action_script) {
        Ok(result) => {
            println!("\n✅ Anchor action completed successfully!");
            println!("Result: {}", result);
        }
        Err(e) => {
            println!("\n❌ Anchor action failed: {}", e);
        }
    }
    
    println!("\n=== Key Benefits Achieved ===");
    println!("🎯 No external script dependency - everything is internal");
    println!("🔧 User-customizable - edit JavaScript to change behavior"); 
    println!("🏗️  Rich built-ins - file ops, app launching, shell commands");
    println!("🌐 Environment-aware - different logic for different setups");
    println!("⚡ Fast execution - embedded JavaScript runtime");
    
    println!("\n=== Customization Examples ===");
    println!("- Change folder app: modify open_folder() calls");
    println!("- Add custom tools: use shell() to launch your tools");
    println!("- Conditional logic: if (isLinux()) vs if (isMacOS())");
    println!("- Per-directory behavior: load custom scripts based on path");
    
    Ok(())
}