use anchor_selector::launcher::launch;

#[test]
fn test_shutdown_command_execution() {
    println!("Testing _shutdown command execution...");
    
    // Test the specific failing command
    let result = launch("cmd _shutdown");
    match result {
        Ok(_) => {
            println!("✅ '_shutdown' command executed successfully");
        },
        Err(e) => {
            println!("❌ '_shutdown' command failed: {:?}", e);
            
            // Let's also test if the command exists in PATH
            let which_result = launch("cmd which _shutdown");
            match which_result {
                Ok(_) => {
                    println!("✅ 'which _shutdown' works - command is in PATH");
                },
                Err(e2) => {
                    println!("❌ 'which _shutdown' also failed: {:?}", e2);
                }
            }
        }
    }
    
    // Test that PATH contains the user's bin directory
    let path_result = launch("cmd echo $PATH | grep /Users/oblinger/bin");
    match path_result {
        Ok(_) => {
            println!("✅ User's bin directory is in PATH");
        },
        Err(e) => {
            println!("❌ User's bin directory not found in PATH: {:?}", e);
        }
    }
}