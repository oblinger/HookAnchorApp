use std::process::Command;

fn main() {
    println!("Testing contact access...");
    
    // Test a simple AppleScript to get contact count
    let script = r#"
tell application "Contacts"
    return count of people
end tell
"#;
    
    let output = Command::new("osascript")
        .arg("-e")
        .arg(script)
        .output();
        
    match output {
        Ok(result) => {
            if result.status.success() {
                let count_str = String::from_utf8_lossy(&result.stdout);
                println!("Found {} contacts in Contacts app", count_str.trim());
            } else {
                let error = String::from_utf8_lossy(&result.stderr);
                println!("Error accessing contacts: {}", error);
            }
        }
        Err(e) => {
            println!("Failed to run osascript: {}", e);
        }
    }
}