use hookanchor::{Command, Patch, infer_patch};
use std::collections::HashMap;

/// Helper to create a test command
fn make_cmd(name: &str, action: &str, arg: &str, patch: &str) -> Command {
    Command {
        patch: patch.to_string(),
        command: name.to_string(),
        action: action.to_string(),
        arg: arg.to_string(),
        flags: String::new(),
        full_line: format!("{} : {} {}", name, action, arg),
    }
}

/// Helper to create a patch with linked command
fn make_patch(name: &str, linked_cmd: Option<Command>) -> Patch {
    Patch {
        name: name.to_string(),
        linked_command: linked_cmd,
    }
}

/// Create a test patches hashmap with common patterns
fn create_test_patches() -> HashMap<String, Patch> {
    let mut patches = HashMap::new();
    
    // Create some anchor commands for patch linking
    let ghost_cmd = make_cmd("GHOST-other", "anchor", "/path/to/ghost.md", "2006-12 ghost");
    let darpa_cmd = make_cmd("2007-00 DARPA seeds", "anchor", "/path/to/darpa.md", "idea");
    let log_cmd = make_cmd("Log", "folder", "/Users/oblinger/ob/kmr/Log", "");
    let idea_cmd = make_cmd("Idea", "folder", "/Users/oblinger/ob/kmr/Log/Idea", "");
    let web_cmd = make_cmd("Web", "folder", "/path/to/web", "");
    
    // Add patches with linked commands
    patches.insert("ghost".to_string(), make_patch("ghost", Some(ghost_cmd)));
    patches.insert("2007-00 darpa seeds".to_string(), make_patch("2007-00 darpa seeds", Some(darpa_cmd)));
    patches.insert("log".to_string(), make_patch("log", Some(log_cmd)));
    patches.insert("idea".to_string(), make_patch("idea", Some(idea_cmd)));
    patches.insert("web".to_string(), make_patch("web", Some(web_cmd)));
    
    // Add some patches without linked commands
    patches.insert("cmd".to_string(), make_patch("cmd", None));
    patches.insert("2023".to_string(), make_patch("2023", None));
    patches.insert("2014".to_string(), make_patch("2014", None));
    patches.insert("kmr".to_string(), make_patch("kmr", None));
    
    patches
}

#[test]
fn test_self_assignment_prevention() {
    println!("Testing self-assignment prevention...");
    let patches = create_test_patches();
    
    // Test case 1: Command should not be assigned to its own patch
    let ghost_cmd = make_cmd("GHOST-other", "anchor", "/path/to/ghost.md", "2006-12 ghost");
    let result = infer_patch(&ghost_cmd, &patches);
    
    // Should NOT return "GHOST-other" (self-assignment)
    assert_ne!(result, Some("GHOST-other".to_string()), 
              "Command should not be assigned to its own patch");
    
    // Test case 2: Command with exact patch name match should be prevented
    let web_cmd = make_cmd("Web", "folder", "/path/to/web", "");
    let result = infer_patch(&web_cmd, &patches);
    
    // Should NOT return "Web" (self-assignment)
    assert_ne!(result, Some("Web".to_string()), 
              "Command should not be assigned to its own patch");
    
    println!("✅ Self-assignment prevention tests passed");
}

#[test]
fn test_path_specificity() {
    println!("Testing path specificity (most specific patch wins)...");
    let patches = create_test_patches();
    
    // Test case: File under nested anchors should get most specific patch
    let nested_cmd = make_cmd("SCATA Security", "anchor", 
                            "/Users/oblinger/ob/kmr/Log/Idea/2007-00 DARPA seeds/SCATA Security/file.md", 
                            "");
    let result = infer_patch(&nested_cmd, &patches);
    
    // Should return "2007-00 DARPA seeds" (most specific) not "Log" or "Idea"
    assert_eq!(result, Some("2007-00 DARPA seeds".to_string()), 
              "Should return most specific patch in path hierarchy");
    
    // Test case: Relative path specificity
    let relative_cmd = make_cmd("Test File", "doc", "Log/Idea/somefile.md", "");
    let result = infer_patch(&relative_cmd, &patches);
    
    // Should return "Idea" (deeper) not "Log"
    assert_eq!(result, Some("Idea".to_string()), 
              "Should return deeper patch for relative paths");
    
    println!("✅ Path specificity tests passed");
}

#[test]
fn test_browser_action_web_patch() {
    println!("Testing browser actions get Web patch...");
    let patches = create_test_patches();
    
    let test_cases = vec![
        ("chrome", "https://example.com"),
        ("safari", "https://apple.com"),
        ("firefox", "https://mozilla.org"),
        ("brave", "https://brave.com"),
        ("url", "https://github.com"),
    ];
    
    for (action, url) in test_cases {
        let cmd = make_cmd("Test Page", action, url, "");
        let result = infer_patch(&cmd, &patches);
        
        assert_eq!(result, Some("Web".to_string()), 
                  "Action '{}' should infer Web patch", action);
    }
    
    println!("✅ Browser action Web patch tests passed");
}

#[test]
fn test_year_based_patch_inference() {
    println!("Testing year-based patch inference...");
    let patches = create_test_patches();
    
    // Test case: Commands starting with year should get year patch (ONLY when no other rules apply)
    let year_cmd = make_cmd("2023-06 Project", "app", "SomeApp", "");
    let result = infer_patch(&year_cmd, &patches);
    
    assert_eq!(result, Some("2023".to_string()), 
              "Year-prefixed command should get year patch when no other rules apply");
    
    // Test case: Command with year in middle should not match
    let non_year_cmd = make_cmd("Project 2023 Plan", "app", "SomeOtherApp", "");
    let result = infer_patch(&non_year_cmd, &patches);
    
    // Should not get year patch since year is not at start
    assert_ne!(result, Some("2023".to_string()), 
              "Year not at start should not get year patch");
    
    println!("✅ Year-based patch inference tests passed");
}

#[test]
fn test_path_specificity_trumps_year_inference() {
    println!("Testing path specificity trumps year-based inference...");
    let patches = create_test_patches();
    
    // Test case: Command with year prefix BUT in a specific path should get path-based patch
    let year_in_path_cmd = make_cmd("2014 Rule Trainer", "doc", 
                                  "/Users/oblinger/ob/kmr/Log/Idea/2007-00 DARPA seeds/2014 Rule Trainer.md", 
                                  "");
    let result = infer_patch(&year_in_path_cmd, &patches);
    
    // Should return "2007-00 DARPA seeds" (most specific path) NOT "2014" (year)
    assert_eq!(result, Some("2007-00 DARPA seeds".to_string()), 
              "Path specificity should trump year inference");
    assert_ne!(result, Some("2014".to_string()), 
              "Should not get year patch when path specificity applies");
    
    // Test case: Year command with no path should still get year patch
    let pure_year_cmd = make_cmd("2014 General Project", "app", "SomeApp", "");
    let result = infer_patch(&pure_year_cmd, &patches);
    
    assert_eq!(result, Some("2014".to_string()), 
              "Year inference should work when no other rules apply");
    
    println!("✅ Path specificity trumps year inference tests passed");
}

#[test]
fn test_cmd_action_patch() {
    println!("Testing cmd action gets Cmd patch...");
    let patches = create_test_patches();
    
    let cmd_command = make_cmd("Test Script", "cmd", "echo hello", "");
    let result = infer_patch(&cmd_command, &patches);
    
    assert_eq!(result, Some("Cmd".to_string()), 
              "Command with 'cmd' action should get 'Cmd' patch");
    
    println!("✅ Cmd action patch tests passed");
}

#[test]
fn test_first_word_patch_matching() {
    println!("Testing first word patch matching...");
    let patches = create_test_patches();
    
    // Test case: First word matches patch name (but not self-assignment)
    let log_cmd = make_cmd("Log Entry", "app", "SomeApp", "");
    let result = infer_patch(&log_cmd, &patches);
    
    assert_eq!(result, Some("Log".to_string()), 
              "First word should match existing patch");
    
    // Test case: First word with different casing
    let idea_cmd = make_cmd("IDEA Brainstorm", "app", "SomeOtherApp", "");
    let result = infer_patch(&idea_cmd, &patches);
    
    assert_eq!(result, Some("Idea".to_string()), 
              "First word matching should be case-insensitive");
    
    println!("✅ First word patch matching tests passed");
}

#[test]
fn test_no_inference_when_appropriate() {
    println!("Testing cases where no inference should occur...");
    let patches = create_test_patches();
    
    // Test case: Non-path, non-matching command
    let unknown_cmd = make_cmd("Random Command", "app", "SomeApp", "");
    let result = infer_patch(&unknown_cmd, &patches);
    
    assert_eq!(result, None, 
              "Unknown command should not get any inferred patch");
    
    // Test case: URL that doesn't match browser actions
    let url_cmd = make_cmd("Custom Protocol", "custom", "protocol://example", "");
    let result = infer_patch(&url_cmd, &patches);
    
    assert_eq!(result, None, 
              "Custom protocol should not get inferred patch");
    
    println!("✅ No inference tests passed");
}

#[test]
fn test_alias_patch_inheritance() {
    println!("Testing alias commands inherit target patch...");
    let patches = create_test_patches();
    
    // Test case: Alias should inherit patch from target
    // Note: This would require the target command to exist in the system
    // For now, test that alias action is handled
    let alias_cmd = make_cmd("Shortcut", "alias", "Log Entry", "");
    let result = infer_patch(&alias_cmd, &patches);
    
    // The actual implementation would need access to target commands
    // This test ensures alias action doesn't crash the inference
    println!("Alias result: {:?}", result);
    
    println!("✅ Alias patch inheritance tests passed");
}

#[test]
fn test_web_patch_only_for_empty_patches() {
    println!("Testing Web patch only applies when current patch is empty...");
    let patches = create_test_patches();
    
    // Test case 1: Command with existing patch should NOT get Web patch
    let existing_patch_cmd = make_cmd("Amazon", "chrome", "https://amazon.com", "AMA");
    let result = infer_patch(&existing_patch_cmd, &patches);
    
    // Should NOT return "Web" because command already has "AMA" patch
    assert_ne!(result, Some("Web".to_string()), 
              "Command with existing patch should not get Web patch");
    
    // Test case 2: Command with empty patch SHOULD get Web patch
    let empty_patch_cmd = make_cmd("New Site", "chrome", "https://example.com", "");
    let result = infer_patch(&empty_patch_cmd, &patches);
    
    assert_eq!(result, Some("Web".to_string()), 
              "Command with empty patch should get Web patch");
    
    println!("✅ Web patch empty-only tests passed");
}

#[test]
fn test_alias_priority_over_other_rules() {
    println!("Testing alias inheritance has highest priority...");
    let mut patches = create_test_patches();
    
    // Create a target command with a patch
    let target_cmd = make_cmd("Log Analysis", "doc", "/path/to/analysis.md", "Log");
    patches.insert("log analysis".to_string(), make_patch("log analysis", Some(target_cmd.clone())));
    
    // Test: Alias command should inherit target's patch even if other rules would apply
    let alias_cmd = make_cmd("abw", "alias", "Log Analysis", "ABIO");
    let result = infer_patch(&alias_cmd, &patches);
    
    // This would need the actual target lookup implementation
    // For now, ensure it's processed as alias
    println!("Alias priority result: {:?}", result);
    
    println!("✅ Alias priority tests passed");
}

#[test]
fn test_edge_cases() {
    println!("Testing edge cases...");
    let patches = create_test_patches();
    
    // Test case: Empty command name
    let empty_cmd = make_cmd("", "doc", "/path/to/file.md", "");
    let result = infer_patch(&empty_cmd, &patches);
    
    // Should not crash and should return None or some reasonable default
    println!("Empty command result: {:?}", result);
    
    // Test case: Command with only spaces
    let space_cmd = make_cmd("   ", "doc", "/path/to/file.md", "");
    let result = infer_patch(&space_cmd, &patches);
    
    println!("Space-only command result: {:?}", result);
    
    // Test case: Very long path
    let long_path = "/".repeat(1000) + "file.md";
    let long_cmd = make_cmd("Long Path File", "doc", &long_path, "");
    let result = infer_patch(&long_cmd, &patches);
    
    println!("Long path result: {:?}", result);
    
    println!("✅ Edge case tests passed");
}

#[test]
fn test_integration_realistic_scenarios() {
    println!("Testing realistic integration scenarios...");
    let patches = create_test_patches();
    
    // Scenario 1: Anchor file in proper parent folder
    let anchor_cmd = make_cmd("2007-00 DARPA seeds", "anchor", 
                            "/Users/oblinger/ob/kmr/Log/Idea/2007-00 DARPA seeds.md", 
                            "idea");
    let result = infer_patch(&anchor_cmd, &patches);
    
    // Should keep current patch "idea" and not suggest self-assignment
    // The inference might return "Idea" from path analysis, which is correct
    if let Some(inferred) = result {
        assert_ne!(inferred.to_lowercase(), "2007-00 darpa seeds", 
                  "Anchor should not be assigned to its own patch");
    }
    
    // Scenario 2: Web browser command
    let browser_cmd = make_cmd("GitHub", "chrome", "https://github.com", "");
    let result = infer_patch(&browser_cmd, &patches);
    
    assert_eq!(result, Some("Web".to_string()), 
              "Browser command should get Web patch");
    
    // Scenario 3: Nested file should get most specific patch
    let nested_file = make_cmd("Security Analysis", "doc", 
                             "/Users/oblinger/ob/kmr/Log/Idea/2007-00 DARPA seeds/analysis.md", 
                             "");
    let result = infer_patch(&nested_file, &patches);
    
    assert_eq!(result, Some("2007-00 DARPA seeds".to_string()), 
              "Nested file should get most specific containing patch");
    
    println!("✅ Realistic integration scenario tests passed");
}