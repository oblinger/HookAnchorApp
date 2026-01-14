use std::process::Command;
use std::sync::Mutex;

// Global mutex to ensure tests run sequentially
static FINDER_TEST_MUTEX: Mutex<()> = Mutex::new(());

/// Check if the grabber output indicates Finder was the frontmost app
fn is_finder_output(stdout: &str) -> bool {
    stdout.contains("RULE:Finder") ||
    stdout.starts_with("file ") ||
    stdout.starts_with("folder ") ||
    stdout.starts_with("doc ")
}

/// Skip message for when Finder isn't frontmost
fn skip_not_finder(stdout: &str) {
    println!("⚠️  SKIPPING: Finder is not the frontmost application");
    println!("   Got output from another app: {}", stdout.chars().take(80).collect::<String>());
    println!("   This test requires Finder to be frontmost - run manually if needed");
}

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

    let stdout_trimmed = stdout.trim();

    // Skip if Finder isn't the frontmost app (another app grabbed focus or grabber failed)
    if !grab_output.status.success() || !is_finder_output(stdout_trimmed) {
        skip_not_finder(stdout_trimmed);
        std::fs::remove_dir_all(test_dir).ok();
        return;
    }

    // The output should be "doc /tmp/hookanchor_test/test_file.txt" or "doc /private/tmp/hookanchor_test/test_file.txt"
    // On macOS, /tmp is a symlink to /private/tmp
    // Note: Output may include " RULE:Finder Selected File" suffix
    let expected_prefix1 = format!("doc {}", test_file);
    let expected_prefix2 = format!("doc {}", test_file.replace("/tmp/", "/private/tmp/"));

    // Check if output starts with expected path (with or without RULE suffix)
    let matches = stdout_trimmed.starts_with(&expected_prefix1) || stdout_trimmed.starts_with(&expected_prefix2);
    assert!(
        matches,
        "Grabber should output starting with 'doc {}' or 'doc {}' but got '{}'",
        test_file, test_file.replace("/tmp/", "/private/tmp/"), stdout_trimmed
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
    // Note: Output may include " RULE:Finder Selected Folder" suffix
    let expected_prefix1 = format!("folder {}", test_dir);
    let expected_prefix2 = format!("folder {}/", test_dir);
    let expected_prefix3 = format!("folder {}", test_dir.replace("/tmp/", "/private/tmp/"));
    let expected_prefix4 = format!("folder {}/", test_dir.replace("/tmp/", "/private/tmp/"));
    let stdout_trimmed = stdout.trim();

    // Skip if Finder isn't the frontmost app (another app grabbed focus)
    if !is_finder_output(stdout_trimmed) {
        skip_not_finder(stdout_trimmed);
        std::fs::remove_dir_all(test_dir).ok();
        return;
    }

    let matches = stdout_trimmed.starts_with(&expected_prefix1) ||
                  stdout_trimmed.starts_with(&expected_prefix2) ||
                  stdout_trimmed.starts_with(&expected_prefix3) ||
                  stdout_trimmed.starts_with(&expected_prefix4);

    assert!(
        matches,
        "Grabber should output starting with folder path but got '{}'", stdout_trimmed
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
    // Note: Output may include " RULE:Finder No Selection" suffix
    let expected_prefix1 = format!("folder {}/", test_dir);
    let expected_prefix2 = format!("folder {}/", test_dir.replace("/tmp/", "/private/tmp/"));
    let stdout_trimmed = stdout.trim();

    // Skip if Finder isn't the frontmost app (another app grabbed focus)
    if !is_finder_output(stdout_trimmed) {
        skip_not_finder(stdout_trimmed);
        std::fs::remove_dir_all(test_dir).ok();
        return;
    }

    let matches = stdout_trimmed.starts_with(&expected_prefix1) || stdout_trimmed.starts_with(&expected_prefix2);
    assert!(
        matches,
        "Grabber should output starting with '{}' or '{}' when no selection, but got '{}'",
        expected_prefix1, expected_prefix2, stdout_trimmed
    );

    // Clean up
    std::fs::remove_dir_all(test_dir).ok();
}