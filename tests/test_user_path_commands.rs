use hookanchor::launcher::launch;

#[test]
fn test_commands_requiring_user_path() {
    println!("Testing commands that require user PATH...");
    
    // Test a command that might be in user's custom PATH
    // cargo is usually in ~/.cargo/bin which should be in user PATH
    let result = launch("cmd which cargo");
    match result {
        Ok(_) => {
            println!("✅ 'which cargo' command executed successfully");
        },
        Err(e) => {
            println!("⚠️  'which cargo' failed (may not be installed): {:?}", e);
        }
    }
    
    // Test pwd command (should always work)
    let result = launch("cmd pwd");
    match result {
        Ok(_) => {
            println!("✅ 'pwd' command executed successfully");
        },
        Err(e) => {
            println!("❌ 'pwd' command failed: {:?}", e);
        }
    }
    
    // Test ls command (should always work)
    let result = launch("cmd ls /");
    match result {
        Ok(_) => {
            println!("✅ 'ls /' command executed successfully");
        },
        Err(e) => {
            println!("❌ 'ls /' command failed: {:?}", e);
        }
    }
    
    println!("✅ User PATH command tests completed");
}