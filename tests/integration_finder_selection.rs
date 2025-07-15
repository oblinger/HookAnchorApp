use std::process::Command;

#[test]
fn test_finder_file_selection_grabber() {
    // Create a test file in a known location
    let test_dir = "/tmp/hookanchor_test";
    let test_file = format!("{}/test_file.txt", test_dir);
    
    // Create test directory and file
    std::fs::create_dir_all(test_dir).expect("Failed to create test directory");
    std::fs::write(&test_file, "test content").expect("Failed to create test file");
    
    // AppleScript to open Finder and select the test file
    let select_file_script = format!(r#"
        tell application "Finder"
            activate
            open folder POSIX file "{}"
            delay 0.5
            select POSIX file "{}"
            delay 0.5
        end tell
    "#, test_dir, test_file);
    
    // Execute the AppleScript to set up Finder
    let setup_output = Command::new("osascript")
        .arg("-e")
        .arg(&select_file_script)
        .output()
        .expect("Failed to execute AppleScript");
    
    if !setup_output.status.success() {
        panic!("AppleScript failed: {}", String::from_utf8_lossy(&setup_output.stderr));
    }
    
    // Small delay to ensure Finder is ready
    std::thread::sleep(std::time::Duration::from_millis(500));
    
    // Run the grabber to capture the selected file
    let grab_output = Command::new("./target/release/ha")
        .arg("--grab")
        .arg("0")  // No additional delay
        .output()
        .expect("Failed to execute grabber");
    
    let stdout = String::from_utf8_lossy(&grab_output.stdout);
    let stderr = String::from_utf8_lossy(&grab_output.stderr);
    
    println!("Grabber stdout: {}", stdout);
    println!("Grabber stderr: {}", stderr);
    
    // Check if the grabber captured the correct file
    assert!(grab_output.status.success(), "Grabber should succeed");
    
    // The output should be "doc /tmp/hookanchor_test/test_file.txt"
    let expected_output = format!("doc {}", test_file);
    assert_eq!(stdout.trim(), expected_output, 
        "Grabber should output 'doc {}' but got '{}'", test_file, stdout.trim());
    
    // Clean up
    std::fs::remove_dir_all(test_dir).ok();
}

#[test] 
fn test_finder_folder_selection_grabber() {
    // Create a test directory
    let test_dir = "/tmp/hookanchor_test_folder";
    std::fs::create_dir_all(test_dir).expect("Failed to create test directory");
    
    // AppleScript to open and select the folder
    let select_folder_script = format!(r#"
        tell application "Finder"
            activate
            open folder POSIX file "{}"
            delay 0.5
            set selection to folder POSIX file "{}"
            delay 0.5
        end tell
    "#, test_dir, test_dir);
    
    // Execute the AppleScript
    let setup_output = Command::new("osascript")
        .arg("-e")
        .arg(&select_folder_script)
        .output()
        .expect("Failed to execute AppleScript");
    
    if !setup_output.status.success() {
        panic!("AppleScript failed: {}", String::from_utf8_lossy(&setup_output.stderr));
    }
    
    // Small delay to ensure Finder is ready
    std::thread::sleep(std::time::Duration::from_millis(500));
    
    // Run the grabber
    let grab_output = Command::new("./target/release/ha")
        .arg("--grab")
        .arg("0")
        .output()
        .expect("Failed to execute grabber");
    
    let stdout = String::from_utf8_lossy(&grab_output.stdout);
    let stderr = String::from_utf8_lossy(&grab_output.stderr);
    
    println!("Grabber stdout: {}", stdout);
    println!("Grabber stderr: {}", stderr);
    
    assert!(grab_output.status.success(), "Grabber should succeed");
    
    // The output should be "folder /tmp/hookanchor_test_folder"
    let expected_output = format!("folder {}", test_dir);
    assert_eq!(stdout.trim(), expected_output,
        "Grabber should output 'folder {}' but got '{}'", test_dir, stdout.trim());
    
    // Clean up
    std::fs::remove_dir_all(test_dir).ok();
}

#[test]
fn test_finder_no_selection_grabber() {
    // AppleScript to open Finder to a folder without selecting anything
    let open_finder_script = r#"
        tell application "Finder"
            activate
            open folder POSIX file "/tmp"
            delay 0.5
            set selection to {}
            delay 0.5
        end tell
    "#;
    
    // Execute the AppleScript
    let setup_output = Command::new("osascript")
        .arg("-e")
        .arg(open_finder_script)
        .output()
        .expect("Failed to execute AppleScript");
    
    if !setup_output.status.success() {
        panic!("AppleScript failed: {}", String::from_utf8_lossy(&setup_output.stderr));
    }
    
    // Small delay to ensure Finder is ready
    std::thread::sleep(std::time::Duration::from_millis(500));
    
    // Run the grabber
    let grab_output = Command::new("./target/release/ha")
        .arg("--grab")
        .arg("0")
        .output()
        .expect("Failed to execute grabber");
    
    let stdout = String::from_utf8_lossy(&grab_output.stdout);
    let stderr = String::from_utf8_lossy(&grab_output.stderr);
    
    println!("Grabber stdout: {}", stdout);
    println!("Grabber stderr: {}", stderr);
    
    assert!(grab_output.status.success(), "Grabber should succeed");
    
    // When no selection, it should grab the current folder
    let expected_output = "folder /tmp/";
    assert_eq!(stdout.trim(), expected_output,
        "Grabber should output 'folder /tmp/' when no selection, but got '{}'", stdout.trim());
}