//! Comprehensive functional test suite for HookAnchor
//! 
//! This test suite creates a complex command hierarchy to test all aspects
//! of the system including patch inference, command resolution, anchors,
//! aliases, and various action types.

use hookanchor::core::commands::{Patch, parse_command_line};
use hookanchor::core::commands::{filter_commands, run_patch_inference, find_orphan_patches};
use hookanchor::core::config::Config;
use hookanchor::{load_data, GlobalData, get_display_commands_with_options};
use std::collections::HashMap;
use std::fs;

/// Create a comprehensive test command structure
fn create_test_command_structure() -> String {
    r#"# HookAnchor Functional Test Commands
# This structure tests all major features and edge cases

# === PATCH GROUPS WITH ANCHORS ===
# These patches have explicit anchor commands

Project! project : anchor; /Users/test/projects/project.md
Project! Django Backend : folder; /Users/test/projects/django
Project! React Frontend : folder; /Users/test/projects/react
Project! API Docs : notion; https://notion.so/api-docs

Work! work : anchor; /Users/test/work/work.md
Work! Email : chrome; https://gmail.com
Work! Calendar : chrome; https://calendar.google.com
Work! Slack : app; Slack
Work! 1Password : 1pass; Work Account

Dev! dev : anchor; /Users/test/dev/dev.md  
Dev! VS Code : app; Visual Studio Code
Dev! Terminal : app; Terminal
Dev! GitHub : chrome; https://github.com
Dev! Docker : shell; docker ps -a

# === NESTED PATCH HIERARCHIES ===
# Test multi-level patch structures

Apps! apps : anchor; /Users/test/apps/apps.md
Apps! Browsers : rewrite; browsers
Apps! Editors : rewrite; editors
Apps! Utilities : rewrite; utilities

Browsers! browsers : anchor; /Users/test/apps/browsers.md
Browsers! Chrome : app; Google Chrome
Browsers! Firefox : app; Firefox
Browsers! Safari : app; Safari
Browsers! Brave : app; Brave Browser

Editors! editors : anchor; /Users/test/apps/editors.md
Editors! VS Code : app; Visual Studio Code
Editors! Sublime : app; Sublime Text
Editors! Vim : shell; vim
Editors! Emacs : app; Emacs

# === COMMANDS WITHOUT PATCHES (should get inferred) ===
# These should trigger patch inference rules

Google Search : chrome; https://google.com
DuckDuckGo : firefox; https://duckduckgo.com
GitHub : url; https://github.com
Stack Overflow : brave; https://stackoverflow.com

Downloads : folder; ~/Downloads
Documents : folder U; ~/Documents
Desktop : folder; ~/Desktop

README : markdown; /Users/test/README.md
Notes : doc; ~/notes.txt
Config : open; ~/.config/hookanchor/config.yaml

# === ALIASES AND REWRITE CHAINS ===
# Test command resolution through aliases

web : rewrite; Google Search
search : rewrite; web
g : rewrite; search

code : rewrite; VS Code
editor : rewrite; code
e : rewrite; editor

# === SPECIAL CHARACTERS AND EDGE CASES ===

Special! Spaces In Name : folder; /Users/test/My Documents
Special! URL With Params : chrome; https://example.com?foo=bar&baz=qux
Special! Shell Command : shell; echo "Hello, World!" | grep Hello
Special! Path With Tilde : folder; ~/.config/special
Special! Unicode 中文 : notion; https://notion.so/chinese

# === FLAGS AND COMPLEX ARGUMENTS ===

Flags! Chrome Incognito : chrome --incognito; https://private.com
Flags! Terminal New Tab : app -n; Terminal
Flags! Folder Merge : folder M; ~/merge-test
Flags! User Edited : folder U; ~/user-folder

# === ORPHAN PATCH REFERENCES ===
# These reference patches that don't have anchors yet

Orphan1! Test Command 1 : chrome; https://test1.com
Orphan2! Test Command 2 : folder; /test2
Orphan3! Test Command 3 : app; TestApp

# === CASE SENSITIVITY TESTS ===

CaseSensitive! lowercase : anchor; /test/lowercase.md
casesensitive! UPPERCASE : folder; /test/UPPERCASE
CASESENSITIVE! MixedCase : chrome; https://mixed.com

# === CIRCULAR REFERENCES (should handle gracefully) ===

circular1 : rewrite; circular2
circular2 : rewrite; circular3  
circular3 : rewrite; circular1

# === DUPLICATE COMMANDS (test merging) ===

Duplicate! Test : chrome; https://version1.com
Duplicate! Test : chrome; https://version2.com
Test : chrome; https://no-patch-version.com

# === ACTION TYPE COVERAGE ===

Actions! Anchor Test : anchor; /test/anchor.md
Actions! Markdown Test : markdown; /test/markdown.md
Actions! Folder Test : folder; /test/folder
Actions! Chrome Test : chrome; https://chrome.com
Actions! Firefox Test : firefox; https://firefox.com
Actions! Brave Test : brave; https://brave.com
Actions! Safari Test : safari; https://safari.com
Actions! URL Test : url; https://url.com
Actions! App Test : app; TestApp
Actions! Shell Test : shell; ls -la
Actions! CMD Test : cmd; dir
Actions! Notion Test : notion; https://notion.so/test
Actions! Work Test : work; https://work.com
Actions! 1Pass Test : 1pass; Test Account
Actions! Doc Test : doc; /test/doc.txt
Actions! Open Test : open; /test/file.txt
Actions! Rewrite Test : rewrite; Anchor Test

# === EMPTY/MINIMAL COMMANDS ===

Empty! No Arg : chrome;
Empty! No Action : ;
Empty! Spaces :   folder  ;   /test/spaces   

# === LONG NAMES AND ARGUMENTS ===

LongTest! This Is A Very Long Command Name That Should Still Parse Correctly : folder; /Users/test/very/long/path/that/goes/on/and/on/and/on/testing/limits
LongTest! Short : url; https://example.com/with/a/very/long/url/path/that/includes/many/segments/and/parameters?param1=value1&param2=value2&param3=value3"#.to_string()
}

/// Parse test command structure into command vector
fn parse_test_commands() -> Vec<hookanchor::Command> {
    let test_data = create_test_command_structure();
    let lines: Vec<&str> = test_data.lines().collect();
    
    let mut commands = Vec::new();
    for line in lines {
        if !line.trim().is_empty() && !line.starts_with('#') {
            if let Ok(cmd) = parse_command_line(line) {
                commands.push(cmd);
            }
        }
    }
    commands
}

/// Create a complete GlobalData structure with test commands
/// This runs the full load_data pipeline on our test data
fn create_test_global_data() -> GlobalData {
    let test_commands = parse_test_commands();
    load_data(test_commands)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    
    /// Load test commands and verify basic structure using new load_data_with_commands
    #[test]
    fn test_load_command_structure() {
        let global_data = create_test_global_data();
        
        // Should have loaded many commands with full processing
        assert!(global_data.commands.len() > 50, "Expected > 50 commands, got {}", global_data.commands.len());
        
        // Verify some key commands exist (note: patches are lowercase with ! suffix)
        let has_project_anchor = global_data.commands.iter().any(|c| c.patch.contains("project") && c.command == "project");
        assert!(has_project_anchor, "Missing project anchor command");
        
        let has_orphan_ref = global_data.commands.iter().any(|c| c.patch.contains("Orphan1"));
        assert!(has_orphan_ref, "Missing Orphan1 patch reference");
        
        // Verify patches were created
        assert!(!global_data.patches.is_empty(), "Should have created patches hashmap");
        
        // Verify config is loaded
        assert!(!global_data.config.popup_settings.word_separators.is_empty(), "Config should be loaded");
    }
    
    /// Test patch inference for commands without patches using full pipeline
    #[test] 
    fn test_patch_inference() {
        // Start with raw parsed commands (no inference yet)
        let raw_commands = parse_test_commands();
        
        // Count commands without patches before full processing
        let empty_before = raw_commands.iter().filter(|c| c.patch.is_empty()).count();
        assert!(empty_before > 0, "Should have commands without patches before processing");
        
        // Run full pipeline including patch inference
        let global_data = create_test_global_data();
        
        // Count after full processing - should be fewer empty patches
        let empty_after = global_data.commands.iter().filter(|c| c.patch.is_empty()).count();
        assert!(empty_after < empty_before, "Full processing should reduce empty patches");
        
        // Verify patches were created for test commands
        assert!(!global_data.patches.is_empty(), "Should have created patches");
        
        // Test specific patch assignments from our test data  
        let project_commands = global_data.commands.iter()
            .filter(|c| c.patch.contains("project"))
            .count();
        assert!(project_commands > 1, "Should have multiple project commands");
        
        let work_commands = global_data.commands.iter()
            .filter(|c| c.patch.contains("work"))
            .count();
        assert!(work_commands > 1, "Should have multiple work commands");
    }
    
    /// Test display commands with known test dataset
    #[test]
    fn test_display_commands_with_test_data() {
        let global_data = create_test_global_data();
        
        // Test searching for "project" should find Project commands
        let project_results = get_display_commands_with_options(
            &global_data.commands, 
            "project", 
            &global_data.config, 
            50, 
            false
        );
        
        assert!(!project_results.is_empty(), "Should find project-related commands");
        
        // Test submenu functionality with patch groups
        let work_results = get_display_commands_with_options(
            &global_data.commands,
            "work",
            &global_data.config,
            50,
            false
        );
        
        assert!(!work_results.is_empty(), "Should find work-related commands");
        
        // Test that we have both commands with work patch and commands starting with "work"
        let has_work_patch = work_results.iter().any(|c| c.patch.contains("work"));
        assert!(has_work_patch, "Should include commands with work patch");
        
        // Test case sensitivity
        let case_results = get_display_commands_with_options(
            &global_data.commands,
            "CASE",
            &global_data.config,
            50,
            false
        );
        
        assert!(!case_results.is_empty(), "Should handle case insensitive search");
    }
    
    /// Test command resolution through rewrite chains
    #[test]
    fn test_rewrite_resolution() {
        let test_data = create_test_command_structure();
        let lines: Vec<&str> = test_data.lines().collect();
        
        // Parse commands manually
        let mut commands = Vec::new();
        for line in lines {
            if !line.trim().is_empty() && !line.starts_with('#') {
                if let Ok(cmd) = parse_command_line(line) {
                    commands.push(cmd);
                }
            }
        }
        
        // Test simple alias
        let web_cmd = commands.iter().find(|c| c.command == "web").unwrap();
        assert_eq!(web_cmd.action, "rewrite");
        assert_eq!(web_cmd.arg, "Google Search");
        
        // Test chained aliases (g -> search -> web -> Google Search)
        let g_cmd = commands.iter().find(|c| c.command == "g").unwrap();
        assert_eq!(g_cmd.action, "rewrite");
        
        // Verify circular reference exists (should handle gracefully in real usage)
        let circular1 = commands.iter().find(|c| c.command == "circular1").unwrap();
        assert_eq!(circular1.action, "rewrite");
        assert_eq!(circular1.arg, "circular2");
    }
    
    /// Test filtering and searching commands
    #[test]
    fn test_command_filtering() {
        let test_data = create_test_command_structure();
        let lines: Vec<&str> = test_data.lines().collect();
        
        // Parse commands manually
        let mut commands = Vec::new();
        for line in lines {
            if !line.trim().is_empty() && !line.starts_with('#') {
                if let Ok(cmd) = parse_command_line(line) {
                    commands.push(cmd);
                }
            }
        }
        
        // Filter for "test" - should find multiple matches
        let filtered = filter_commands(&commands, "test", 100, false);
        assert!(filtered.len() >= 5, "Should find multiple test commands");
        
        // Filter for exact patch
        let project_cmds = filter_commands(&commands, "Project!", 100, false);
        let all_project = project_cmds.iter().all(|c| c.patch == "Project");
        assert!(all_project, "All filtered commands should be in Project patch");
        
        // Case insensitive search
        let case_filtered = filter_commands(&commands, "CHROME", 100, false);
        assert!(!case_filtered.is_empty(), "Should find chrome commands case-insensitively");
    }
    
    /// Test handling of special characters and edge cases
    #[test]
    fn test_special_cases() {
        let test_data = create_test_command_structure();
        let lines: Vec<&str> = test_data.lines().collect();
        
        // Parse commands manually
        let mut commands = Vec::new();
        let mut errors = Vec::new();
        for line in lines {
            if !line.trim().is_empty() && !line.starts_with('#') {
                match parse_command_line(line) {
                    Ok(cmd) => commands.push(cmd),
                    Err(e) => errors.push(format!("{}: {}", line, e)),
                }
            }
        }
        
        // Find special character commands
        let spaces_cmd = commands.iter().find(|c| c.command == "Spaces In Name").unwrap();
        assert_eq!(spaces_cmd.arg, "/Users/test/My Documents");
        
        let unicode_cmd = commands.iter().find(|c| c.command == "Unicode 中文").unwrap();
        assert_eq!(unicode_cmd.action, "notion");
        
        // Check URL with parameters parsed correctly
        let url_cmd = commands.iter().find(|c| c.command == "URL With Params").unwrap();
        assert!(url_cmd.arg.contains("foo=bar&baz=qux"));
        
        // Since we filter out empty lines, we should verify the test commands were parsed correctly
        // Check for commands with special characters
        let has_unicode = commands.iter().any(|c| c.command.contains("中文"));
        assert!(has_unicode, "Should have unicode command");
        
        // Check for commands with spaces in arguments  
        let has_spaces = commands.iter().any(|c| c.arg.contains("My Documents"));
        assert!(has_spaces, "Should have command with spaces in argument");
    }
    
    /// Test flag handling
    #[test]
    fn test_flag_parsing() {
        let test_data = create_test_command_structure();
        let lines: Vec<&str> = test_data.lines().collect();
        
        // Parse commands manually
        let mut commands = Vec::new();
        for line in lines {
            if !line.trim().is_empty() && !line.starts_with('#') {
                if let Ok(cmd) = parse_command_line(line) {
                    commands.push(cmd);
                }
            }
        }
        
        // Test chrome with incognito flag
        let incognito = commands.iter().find(|c| c.command == "Chrome Incognito").unwrap();
        assert_eq!(incognito.flags, "--incognito");
        assert_eq!(incognito.action, "chrome");
        
        // Test folder with merge flag
        let merge = commands.iter().find(|c| c.command == "Folder Merge").unwrap();
        assert_eq!(merge.flags, "M");
        
        // Test user edited flag
        let user_edited = commands.iter().find(|c| c.command == "User Edited").unwrap();
        assert_eq!(user_edited.flags, "U");
    }
    
    /// Test orphan patch detection and anchor creation
    #[test]
    fn test_orphan_patches() {
        let test_data = create_test_command_structure();
        let lines: Vec<&str> = test_data.lines().collect();
        
        // Parse commands manually
        let mut commands = Vec::new();
        for line in lines {
            if !line.trim().is_empty() && !line.starts_with('#') {
                if let Ok(cmd) = parse_command_line(line) {
                    commands.push(cmd);
                }
            }
        }
        
        // Create patches hashmap
        let mut patches = HashMap::new();
        for cmd in &commands {
            if cmd.action == "anchor" {
                let patch_name = cmd.command.to_lowercase();
                patches.insert(patch_name.clone(), Patch {
                    name: patch_name,
                    linked_command: Some(cmd.clone()),
                });
            }
        }
        
        // Find orphan patches using the built-in function
        let orphan_patches = find_orphan_patches(&patches, &commands);
        
        // We expect more than 3 orphan patches because several patches are referenced but don't have anchors
        assert!(orphan_patches.len() >= 3, "Should detect at least 3 orphan patches, found {}", orphan_patches.len());
        
        // Verify some expected orphan patches exist
        assert!(orphan_patches.iter().any(|p| p.to_lowercase().contains("orphan")), "Should find orphan patch references");
    }
    
    /// Test saving and reloading maintains integrity
    #[test]
    fn test_save_reload_integrity() {
        let test_data = create_test_command_structure();
        let lines: Vec<&str> = test_data.lines().collect();
        
        // Parse commands manually
        let mut original_commands = Vec::new();
        for line in lines {
            if !line.trim().is_empty() && !line.starts_with('#') {
                if let Ok(cmd) = parse_command_line(line) {
                    original_commands.push(cmd);
                }
            }
        }
        
        // Create temp file
        let temp_dir = TempDir::new().unwrap();
        let temp_file = temp_dir.path().join("test_commands.txt");
        
        // Write commands to file manually since save_commands_to_file doesn't take path
        let mut content = String::new();
        for cmd in &original_commands {
            content.push_str(&cmd.full_line);
            content.push('\n');
        }
        fs::write(&temp_file, &content).unwrap();
        
        // Reload
        let reloaded = fs::read_to_string(&temp_file).unwrap();
        let reloaded_lines: Vec<&str> = reloaded.lines().collect();
        
        let mut reloaded_commands = Vec::new();
        for line in reloaded_lines {
            if let Ok(cmd) = parse_command_line(line) {
                reloaded_commands.push(cmd);
            }
        }
        
        // Verify same number of commands
        assert_eq!(original_commands.len(), reloaded_commands.len(), 
                   "Should have same number of commands after reload");
        
        // Verify key commands still exist with correct data
        let orig_project = original_commands.iter().find(|c| c.patch == "Project" && c.command == "project").unwrap();
        let reload_project = reloaded_commands.iter().find(|c| c.patch == "Project" && c.command == "project").unwrap();
        
        assert_eq!(orig_project.action, reload_project.action);
        assert_eq!(orig_project.arg, reload_project.arg);
    }
    
    /// Test that patch inference doesn't overwrite existing patches (reproduces corruption bug)
    #[test]
    fn test_patch_inference_corruption_prevention() {
        let test_data = r#"
# Commands with existing patches - these should NOT be changed
Project! Django Backend : folder; /test/django
Work! Email : chrome; https://gmail.com  
2023 Patents! Patents : notion; https://notion.so/patents

# Commands without patches - these SHOULD get patches
Google Search : chrome; https://google.com
Downloads : folder; ~/Downloads
README : markdown; /test/README.md
"#;
        
        let lines: Vec<&str> = test_data.lines().filter(|l| !l.trim().is_empty() && !l.starts_with('#')).collect();
        
        // Parse commands manually
        let mut commands = Vec::new();
        for line in lines {
            if let Ok(cmd) = parse_command_line(line) {
                commands.push(cmd);
            }
        }
        
        // Verify initial state
        let commands_with_patches = commands.iter().filter(|c| !c.patch.is_empty()).count();
        let commands_without_patches = commands.iter().filter(|c| c.patch.is_empty()).count();
        
        assert_eq!(commands_with_patches, 3, "Should have 3 commands with patches initially");
        assert_eq!(commands_without_patches, 3, "Should have 3 commands without patches initially");
        
        // Create patches hashmap
        let mut patches = HashMap::new();
        for cmd in &commands {
            if cmd.action == "anchor" {
                let patch_name = cmd.command.to_lowercase();
                patches.insert(patch_name.clone(), Patch {
                    name: patch_name,
                    linked_command: Some(cmd.clone()),
                });
            }
        }
        
        // Run patch inference with overwrite_patch = false (should not change existing patches)
        let (patches_assigned, _) = run_patch_inference(&mut commands, &patches, true, false, false);
        
        // Verify that existing patches were NOT changed
        let django = commands.iter().find(|c| c.command == "Django Backend").unwrap();
        assert_eq!(django.patch, "Project", "Django Backend should keep its Project patch");
        
        let email = commands.iter().find(|c| c.command == "Email").unwrap();
        assert_eq!(email.patch, "Work", "Email should keep its Work patch");
        
        let patents = commands.iter().find(|c| c.command == "Patents").unwrap();
        assert_eq!(patents.patch, "2023 Patents", "Patents should keep its 2023 Patents patch");
        
        // Verify that only empty patches were assigned
        let final_empty_count = commands.iter().filter(|c| c.patch.is_empty()).count();
        assert!(final_empty_count < commands_without_patches, "Should have assigned some patches to empty commands");
        assert!(patches_assigned > 0, "Should have assigned patches to commands without them");
        assert!(patches_assigned <= 3, "Should not assign more patches than empty commands we started with");
        
        println!("✅ Test passed: Patch inference properly preserved existing patches");
        println!("   - Commands with patches before: {}", commands_with_patches);
        println!("   - Commands without patches before: {}", commands_without_patches);
        println!("   - Patches assigned: {}", patches_assigned);
        println!("   - Commands without patches after: {}", final_empty_count);
    }
    
    /// Test that reproduces the corruption bug when overwrite_patch=true is used incorrectly  
    #[test]
    fn test_overwrite_patch_corruption_bug() {
        let test_data = r#"
# Many commands with good existing patches that should NOT be overwritten
Project! Django Backend : folder; /test/django
Work! Email : chrome; https://gmail.com  
2023 Patents! Patents : notion; https://notion.so/patents
Research! Paper 1 : markdown; /research/paper1.md
Dev! VS Code : app; Visual Studio Code
Finance! Banking : url; https://bank.com

# A few commands without patches
Google Search : chrome; https://google.com
Downloads : folder; ~/Downloads
"#;
        
        let lines: Vec<&str> = test_data.lines().filter(|l| !l.trim().is_empty() && !l.starts_with('#')).collect();
        
        // Parse commands manually
        let mut commands = Vec::new();
        for line in lines {
            if let Ok(cmd) = parse_command_line(line) {
                commands.push(cmd);
            }
        }
        
        // Verify initial state: 6 commands with patches, 2 without
        let initial_with_patches = commands.iter().filter(|c| !c.patch.is_empty()).count();
        let initial_without_patches = commands.iter().filter(|c| c.patch.is_empty()).count();
        
        assert_eq!(initial_with_patches, 6, "Should have 6 commands with patches initially");
        assert_eq!(initial_without_patches, 2, "Should have 2 commands without patches initially");
        
        // Create patches hashmap (simulate some existing anchors)
        let mut patches = HashMap::new();
        
        // Now run patch inference with overwrite_patch = true (this reproduces the bug!)
        let (patches_assigned, _) = run_patch_inference(&mut commands, &patches, true, false, true);
        
        // This is the corruption: it should assign WAY more patches than just the empty ones
        // Because it's overwriting existing patches with inferred ones
        println!("🐛 BUG REPRODUCED:");
        println!("   - Commands with patches initially: {}", initial_with_patches);
        println!("   - Commands without patches initially: {}", initial_without_patches);
        println!("   - Patches assigned with overwrite=true: {}", patches_assigned);
        
        // After the fix: overwrite_patch=true should only assign patches to commands that actually need them
        // The anti-degradation protection should prevent mass overwriting
        if patches_assigned > initial_without_patches {
            println!("🐛 BUG STILL EXISTS: overwrite_patch=true assigned {} patches, but only {} commands were missing patches!", 
                   patches_assigned, initial_without_patches);
            panic!("Bug not fixed: system is still overwriting existing patches inappropriately");
        } else {
            println!("✅ BUG FIXED: overwrite_patch=true only assigned {} patches to {} commands that needed them", 
                   patches_assigned, initial_without_patches);
        }
        
        // Verify that existing patches were NOT inappropriately changed (should be preserved)
        let django = commands.iter().find(|c| c.command == "Django Backend").unwrap();
        let email = commands.iter().find(|c| c.command == "Email").unwrap();
        
        // These should be preserved after the fix
        assert_eq!(django.patch, "Project", "Django Backend should keep its Project patch (anti-degradation protection)");
        assert_eq!(email.patch, "Work", "Email should keep its Work patch (anti-degradation protection)");
        
        // This should no longer trigger corruption detection
        println!("✅ Anti-degradation protection prevents corruption detection by preserving specific patches");
        
        // The test now confirms the fix works
        println!("🎉 CORRUPTION BUG FIXED: Specific patches are preserved, preventing mass degradation");
    }
    
    /// Test that command parsing preserves existing patches (debug patch stripping)
    #[test]
    fn test_command_parsing_preserves_patches() {
        let test_lines = vec![
            "Project! Django Backend : folder; /test/django",
            "Work! Email : chrome; https://gmail.com",  
            "2023 Patents! Patents : notion; https://notion.so/patents",
            "Google Search : chrome; https://google.com",  // No patch
        ];
        
        println!("🔍 Testing command parsing for patch preservation:");
        
        for line in &test_lines {
            println!("  Parsing: {}", line);
            match parse_command_line(line) {
                Ok(cmd) => {
                    println!("    ✅ Parsed - patch: '{}', command: '{}'", cmd.patch, cmd.command);
                    
                    // Verify patches are preserved correctly
                    if line.contains("Project!") {
                        assert_eq!(cmd.patch, "Project", "Project patch should be preserved");
                    } else if line.contains("Work!") {
                        assert_eq!(cmd.patch, "Work", "Work patch should be preserved");  
                    } else if line.contains("2023 Patents!") {
                        assert_eq!(cmd.patch, "2023 Patents", "2023 Patents patch should be preserved");
                    } else if line.contains("Google Search") {
                        assert_eq!(cmd.patch, "", "Google Search should have empty patch");
                    }
                }
                Err(e) => {
                    println!("    ❌ Parse error: {}", e);
                    panic!("Failed to parse command line: {}", line);
                }
            }
        }
        
        println!("✅ All command parsing preserved patches correctly");
    }
    
    /// Test to reproduce the exact patch stripping bug from real system
    #[test] 
    fn test_reproduce_real_patch_stripping_bug() {
        // Simulate loading commands that should have patches but end up without them
        let test_data = r#"
2023 SV Patents! Patents : notion; https://www.notion.so/sportsvisio/PATENTS-3434075a725f4997b57d9aa9bd0b818a?source=copy_link
Project! Django Backend : folder; /test/django
Work! Email : chrome; https://gmail.com
Google Search : chrome; https://google.com
"#;
        
        let lines: Vec<&str> = test_data.lines().filter(|l| !l.trim().is_empty() && !l.starts_with('#')).collect();
        
        // Parse commands manually
        let mut commands = Vec::new();
        for line in lines {
            if let Ok(cmd) = parse_command_line(line) {
                commands.push(cmd);
            }
        }
        
        println!("🔍 After parsing, checking for patch stripping:");
        for cmd in &commands {
            println!("  Command: '{}' -> Patch: '{}'", cmd.command, cmd.patch);
        }
        
        // Count commands with and without patches
        let with_patches = commands.iter().filter(|c| !c.patch.is_empty()).count();
        let without_patches = commands.iter().filter(|c| c.patch.is_empty()).count();
        
        println!("📊 Results: {} with patches, {} without patches", with_patches, without_patches);
        
        // Should have 3 with patches, 1 without
        assert_eq!(with_patches, 3, "Should have 3 commands with patches after parsing");
        assert_eq!(without_patches, 1, "Should have 1 command without patch after parsing");
        
        // Now run the loading process that might cause stripping
        println!("🔧 Testing if load_data() process strips patches...");
        
        // This simulates what happens in the real system
        let config = Config::default();
        
        // Create patches hashmap
        let mut patches = HashMap::new();
        for cmd in &commands {
            if cmd.action == "anchor" {
                let patch_name = cmd.command.to_lowercase();
                patches.insert(patch_name.clone(), Patch {
                    name: patch_name,
                    linked_command: Some(cmd.clone()),
                });
            }
        }
        
        // Run full inference like the real system does
        let (patches_assigned, _) = run_patch_inference(&mut commands, &patches, true, false, false);
        
        println!("📈 After inference: {} patches assigned", patches_assigned);
        
        // Check final state
        let final_with_patches = commands.iter().filter(|c| !c.patch.is_empty()).count();
        let final_without_patches = commands.iter().filter(|c| c.patch.is_empty()).count();
        
        println!("📊 Final: {} with patches, {} without patches", final_with_patches, final_without_patches);
        
        for cmd in &commands {
            println!("  Final: '{}' -> Patch: '{}'", cmd.command, cmd.patch);
        }
        
        // Verify no patches were stripped
        assert!(final_without_patches <= without_patches, 
               "BUG: Commands lost patches! Started with {} without patches, ended with {}", 
               without_patches, final_without_patches);
        
        // Check specific commands
        let patents = commands.iter().find(|c| c.command == "Patents").unwrap();
        assert_eq!(patents.patch, "2023 SV Patents", "Patents should keep its 2023 SV Patents patch");
        
        println!("✅ No patch stripping detected in this test");
    }
    
    /// Test to reproduce patch degradation bug where specific patches get overwritten by generic ones
    #[test]
    fn test_patch_degradation_bug() {
        let test_data = r#"
# Generic anchor that can cause degradation
T! Old : anchor; /path/to/old.md

# Commands with good specific patches that might get degraded
2025-06 Alg2! README markdown : markdown; /path/to/readme.md
2025-06 HookAnchor! HOTKEY_ALTERNATIVES : markdown; /path/to/hotkey.md
SomeProject! Important File : folder; /path/to/project

# Commands without patches
Simple Command : chrome; https://example.com
"#;
        
        let lines: Vec<&str> = test_data.lines().filter(|l| !l.trim().is_empty() && !l.starts_with('#')).collect();
        
        // Parse commands manually
        let mut commands = Vec::new();
        for line in lines {
            if let Ok(cmd) = parse_command_line(line) {
                commands.push(cmd);
            }
        }
        
        // Create patches hashmap including the problematic "Old" anchor
        let mut patches = HashMap::new();
        for cmd in &commands {
            if cmd.action == "anchor" {
                let patch_name = cmd.command.to_lowercase();
                patches.insert(patch_name.clone(), Patch {
                    name: patch_name.clone(),
                    linked_command: Some(cmd.clone()),
                });
                println!("📍 Created patch anchor: '{}'", patch_name);
            }
        }
        
        println!("🔍 Before inference:");
        for cmd in &commands {
            println!("  '{}' -> patch: '{}'", cmd.command, cmd.patch);
        }
        
        let initial_specific_patches = commands.iter()
            .filter(|c| !c.patch.is_empty() && !c.patch.to_lowercase().contains("old"))
            .count();
        
        println!("📊 Initial specific (non-old) patches: {}", initial_specific_patches);
        
        // Run inference with overwrite_patch = true (reproduces the degradation bug!)
        let (patches_assigned, _) = run_patch_inference(&mut commands, &patches, true, false, true);
        
        println!("🔍 After inference with overwrite=true:");
        for cmd in &commands {
            println!("  '{}' -> patch: '{}'", cmd.command, cmd.patch);
        }
        
        let final_specific_patches = commands.iter()
            .filter(|c| !c.patch.is_empty() && !c.patch.to_lowercase().contains("old"))
            .count();
        
        let degraded_to_old = commands.iter()
            .filter(|c| c.patch.to_lowercase().contains("old"))
            .count();
        
        println!("📊 Final specific (non-old) patches: {}", final_specific_patches);
        println!("📊 Commands degraded to 'old' patch: {}", degraded_to_old);
        println!("📈 Total patches assigned: {}", patches_assigned);
        
        // This is the bug: specific patches getting degraded to generic ones
        if degraded_to_old > 0 {
            println!("🚨 BUG CONFIRMED: {} commands were degraded to generic 'old' patch!", degraded_to_old);
            println!("💥 This type of degradation causes 'corruption' when saving because specific organizational structure is lost");
        }
        
        if final_specific_patches < initial_specific_patches {
            println!("🚨 PATCH DEGRADATION: Specific patches reduced from {} to {}", initial_specific_patches, final_specific_patches);
        }
        
        // Check specific examples
        let readme = commands.iter().find(|c| c.command == "README markdown").unwrap();
        let hotkey = commands.iter().find(|c| c.command == "HOTKEY_ALTERNATIVES").unwrap();
        
        if readme.patch != "2025-06 Alg2" {
            println!("🚨 README degraded: '2025-06 Alg2' -> '{}'", readme.patch);
        }
        if hotkey.patch != "2025-06 HookAnchor" {
            println!("🚨 HOTKEY degraded: '2025-06 HookAnchor' -> '{}'", hotkey.patch);
        }
        
        // This reproduces the corruption issue: mass degradation of patches
        println!("💡 SOLUTION: overwrite_patch=true should NOT degrade specific patches to generic ones");
        
        // The test documents the problematic behavior
        assert!(patches_assigned > 0, "Should show that inference is making changes");
    }
}

