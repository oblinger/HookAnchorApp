//! Test Anchor Tmux Integration
//!
//! Tests the anchor action with tmux session detection and creation.

use hookanchor::launcher::launch;
use std::fs;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Testing Anchor Tmux Integration ===\n");
    
    // Create a test directory with a .tmuxp.yaml file
    let test_dir = "/tmp/test_anchor_tmux";
    fs::create_dir_all(test_dir)?;
    
    // Create a simple .tmuxp.yaml file
    let tmuxp_config = r#"session_name: test_anchor_tmux
windows:
  - window_name: main
    panes:
      - shell_command:
        - echo "Test tmux session started"
"#;
    
    fs::write(format!("{}/.tmuxp.yaml", test_dir), tmuxp_config)?;
    println!("Created test directory: {}", test_dir);
    println!("Created .tmuxp.yaml with session_name: test_anchor_tmux");
    
    // Test the anchor command
    println!("\nExecuting: anchor {}", test_dir);
    println!("(This will create/attach to a tmux session if tmux and tmuxp are installed)\n");
    
    match launch(&format!("anchor {}", test_dir)) {
        Ok(_) => {
            println!("\n✅ Anchor command executed successfully!");
            println!("\nCheck the debug log (~/.anchor.log) for detailed output.");
            println!("If tmux is installed, you should see session creation/attachment logs.");
        }
        Err(e) => {
            println!("\n❌ Anchor command failed: {:?}", e);
        }
    }
    
    // Clean up
    fs::remove_dir_all(test_dir).ok();
    
    println!("\n=== Test Complete ===");
    println!("\nTo manually test:");
    println!("1. Create a directory with a .tmuxp.yaml file");
    println!("2. Run: anchor /path/to/directory");
    println!("3. Check that tmux session is created and attached");
    
    Ok(())
}