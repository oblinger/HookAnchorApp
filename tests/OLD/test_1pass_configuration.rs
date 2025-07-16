use hookanchor::launcher::launch;

#[test]
fn test_1pass_actions_found() {
    println!("Testing 1pass action configurations...");
    
    // Test that the new 1pass action (robust version) can be found
    let result = launch("1pass test");
    match result {
        Ok(_) => {
            println!("✅ 1pass (robust) action executed successfully");
        },
        Err(e) => {
            let error_msg = format!("{:?}", e);
            if error_msg.contains("ActionNotFound") {
                panic!("❌ 1pass action still not found in configuration: {}", error_msg);
            } else {
                println!("✅ 1pass (robust) action found (different error: {})", error_msg);
            }
        }
    }
    
    // Test that the old 1pass action is still available
    let result_old = launch("1pass_old test");
    match result_old {
        Ok(_) => {
            println!("✅ 1pass_old action executed successfully");
        },
        Err(e) => {
            let error_msg = format!("{:?}", e);
            if error_msg.contains("ActionNotFound") {
                panic!("❌ 1pass_old action not found in configuration: {}", error_msg);
            } else {
                println!("✅ 1pass_old action found (different error: {})", error_msg);
            }
        }
    }
}