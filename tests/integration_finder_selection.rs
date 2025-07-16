use std::process::Command;
use std::sync::Mutex;

// Global mutex to ensure tests run sequentially
static FINDER_TEST_MUTEX: Mutex<()> = Mutex::new(());

#[test]
fn test_finder_file_selection_grabber() {
    // Lock to ensure sequential execution, handle poisoned mutex
    let _guard = match FINDER_TEST_MUTEX.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    };
    
    // Create a test file in a known location with unique name
    let test_dir = "/tmp/hookanchor_test_file_selection";
    let test_file = format!("{}/test_file.txt", test_dir);
    
    // Clean up any previous test artifacts
    std::fs::remove_dir_all(test_dir).ok();
    
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
    
    // The output should be "doc /tmp/hookanchor_test/test_file.txt" or "doc /private/tmp/hookanchor_test/test_file.txt"
    // On macOS, /tmp is a symlink to /private/tmp
    let expected_output1 = format!("doc {}", test_file);
    let expected_output2 = format!("doc {}", test_file.replace("/tmp/", "/private/tmp/"));
    assert!(
        stdout.trim() == expected_output1 || stdout.trim() == expected_output2,
        "Grabber should output 'doc {}' or 'doc {}' but got '{}'", 
        test_file, test_file.replace("/tmp/", "/private/tmp/"), stdout.trim()
    );
    
    // Clean up
    std::fs::remove_dir_all(test_dir).ok();
}

#[test] 
fn test_finder_folder_selection_grabber() {
    // Lock to ensure sequential execution, handle poisoned mutex
    let _guard = match FINDER_TEST_MUTEX.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    };
    
    // Create a test directory with unique name
    let test_dir = "/tmp/hookanchor_test_folder_selection";
    
    // Clean up any previous test artifacts
    std::fs::remove_dir_all(test_dir).ok();
    
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
    
    // The output should be "folder /tmp/hookanchor_test_folder" or "folder /private/tmp/hookanchor_test_folder"
    // It might have a trailing slash
    let expected_output1 = format!("folder {}", test_dir);
    let expected_output2 = format!("folder {}/", test_dir);
    let expected_output3 = format!("folder {}", test_dir.replace("/tmp/", "/private/tmp/"));
    let expected_output4 = format!("folder {}/", test_dir.replace("/tmp/", "/private/tmp/"));
    
    assert!(
        stdout.trim() == expected_output1 || stdout.trim() == expected_output2 || 
        stdout.trim() == expected_output3 || stdout.trim() == expected_output4,
        "Grabber should output folder path but got '{}'", stdout.trim()
    );
    
    // Clean up
    std::fs::remove_dir_all(test_dir).ok();
}

#[test]
fn test_finder_no_selection_grabber() {
    // Lock to ensure sequential execution, handle poisoned mutex
    let _guard = match FINDER_TEST_MUTEX.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    };
    
    // Create a test directory for this test
    let test_dir = "/tmp/hookanchor_test_no_selection";
    
    // Clean up any previous test artifacts
    std::fs::remove_dir_all(test_dir).ok();
    std::fs::create_dir_all(test_dir).expect("Failed to create test directory");
    
    // AppleScript to open Finder to a folder without selecting anything
    let open_finder_script = format!(r#"
        tell application "Finder"
            activate
            open folder POSIX file "{}"
            delay 0.5
            set selection to {{}}
            delay 0.5
        end tell
    "#, test_dir);
    
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
    let expected_output1 = format!("folder {}/", test_dir);
    let expected_output2 = format!("folder {}/", test_dir.replace("/tmp/", "/private/tmp/"));
    assert!(
        stdout.trim() == expected_output1 || stdout.trim() == expected_output2,
        "Grabber should output '{}' or '{}' when no selection, but got '{}'", 
        expected_output1, expected_output2, stdout.trim()
    );
    
    // Clean up
    std::fs::remove_dir_all(test_dir).ok();
}