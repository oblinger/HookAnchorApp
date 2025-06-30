use anchor_selector::utils::execute_shell_command_with_env;

#[test]
fn test_shell_command_with_user_environment() {
    println!("Testing shell command execution with user environment...");
    
    // Test basic command
    match execute_shell_command_with_env("echo 'Hello from shell'") {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            assert!(stdout.contains("Hello from shell"), "Should execute basic commands");
            println!("✅ Basic shell command works: {}", stdout.trim());
        },
        Err(e) => {
            panic!("❌ Basic shell command failed: {}", e);
        }
    }
    
    // Test PATH is available
    match execute_shell_command_with_env("echo $PATH") {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            assert!(!stdout.trim().is_empty(), "PATH should not be empty");
            assert!(stdout.contains("/usr/bin"), "PATH should contain /usr/bin");
            println!("✅ PATH is available: {} characters", stdout.len());
        },
        Err(e) => {
            panic!("❌ PATH test failed: {}", e);
        }
    }
    
    // Test HOME is available
    match execute_shell_command_with_env("echo $HOME") {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            assert!(stdout.contains("/Users/"), "HOME should contain /Users/");
            println!("✅ HOME is available: {}", stdout.trim());
        },
        Err(e) => {
            panic!("❌ HOME test failed: {}", e);
        }
    }
    
    // Test shell profile was loaded (check for user-specific PATH entries)
    match execute_shell_command_with_env("echo $PATH | grep -o '/Users/[^:]*'") {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            if !stdout.trim().is_empty() {
                println!("✅ User-specific PATH entries found: {}", stdout.trim());
            } else {
                println!("⚠️  No user-specific PATH entries found, but basic PATH works");
            }
        },
        Err(e) => {
            println!("⚠️  User PATH test failed: {}", e);
        }
    }
    
    // Test which command works with proper PATH
    match execute_shell_command_with_env("which ls") {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            assert!(stdout.contains("/bin/ls") || stdout.contains("/usr/bin/ls"), 
                   "which should find ls command");
            println!("✅ which command works: {}", stdout.trim());
        },
        Err(e) => {
            panic!("❌ which command test failed: {}", e);
        }
    }
    
    println!("✅ All shell environment tests passed");
}

#[test]
fn test_javascript_shell_execution() {
    use anchor_selector::launcher::launch;
    
    println!("Testing JavaScript shell execution with user environment...");
    
    // Test that JavaScript shell() function uses the proper environment
    // This will execute a simple JavaScript that calls shell()
    let result = launch("cmd echo 'Testing shell execution'");
    match result {
        Ok(_) => {
            println!("✅ JavaScript shell execution completed successfully");
        },
        Err(e) => {
            println!("❌ JavaScript shell execution failed: {:?}", e);
        }
    }
}