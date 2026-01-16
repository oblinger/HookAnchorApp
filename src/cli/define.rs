//! Define Command
//!
//! CLI command to define new commands from the command line.
//!
//! Usage:
//!   ha --define name:=MyCommand action:=chrome arg:=https://example.com
//!   ha --define n:=MyCommand a:=chrome r:=https://example.com p:=Web
//!
//! Short aliases:
//!   n = name, a = action, r = arg, p = patch, f = flags

use crate::utils::logging::print;
use crate::utils::params::parse_kv_pairs;
use crate::core::Command;
use crate::capabilities::command_ops::save_command_atomic;
use std::collections::HashMap;

/// Run the define command (--define)
pub fn run_define(args: &[String]) {
    // Join all args after --define into a single string for parsing
    let input = if args.len() > 2 {
        args[2..].join(" ")
    } else {
        print("❌ Missing parameters for --define");
        print("");
        print_usage();
        return;
    };

    // Parse the key-value pairs
    let params = parse_kv_pairs(&input);

    // Expand short aliases to full names
    let params = expand_aliases(params);

    // Validate required fields
    let name = match params.get("name") {
        Some(n) if !n.is_empty() => n.clone(),
        _ => {
            print("❌ Missing required field: name (or n)");
            print("");
            print_usage();
            return;
        }
    };

    let action = match params.get("action") {
        Some(a) if !a.is_empty() => a.clone(),
        _ => {
            print("❌ Missing required field: action (or a)");
            print("");
            print_usage();
            return;
        }
    };

    let arg = match params.get("arg") {
        Some(r) if !r.is_empty() => r.clone(),
        _ => {
            print("❌ Missing required field: arg (or r)");
            print("");
            print_usage();
            return;
        }
    };

    // Optional fields with defaults
    let patch = params.get("patch").cloned().unwrap_or_else(|| {
        // Try to infer patch from arg path
        infer_patch_from_arg(&arg, &action)
    });

    let flags = params.get("flags").cloned().unwrap_or_default();

    // Expand tilde in arg
    let arg = crate::utils::expand_tilde(&arg);

    // Create the command
    let command = Command {
        patch: patch.clone(),
        command: name.clone(),
        action: action.clone(),
        arg: arg.clone(),
        flags,
        other_params: None,
        last_update: 0,
        file_size: None,
    };

    // Save the command
    match save_command_atomic(command.clone(), None) {
        Ok(()) => {
            print(&format!("✅ Command '{}' defined successfully", name));
            print("");
            print(&format!("   Patch:  {}", patch));
            print(&format!("   Action: {}", action));
            print(&format!("   Arg:    {}", arg));
            if !command.flags.is_empty() {
                print(&format!("   Flags:  {}", command.flags));
            }
        }
        Err(e) => {
            print(&format!("❌ Failed to save command: {}", e));
        }
    }
}

/// Expand short aliases to full field names
fn expand_aliases(mut params: HashMap<String, String>) -> HashMap<String, String> {
    // Short -> Long mappings
    let aliases = [
        ("n", "name"),
        ("a", "action"),
        ("r", "arg"),
        ("p", "patch"),
        ("f", "flags"),
    ];

    for (short, long) in aliases {
        if let Some(value) = params.remove(short) {
            // Only insert if long form not already present
            if !params.contains_key(long) {
                params.insert(long.to_string(), value);
            }
        }
    }

    params
}

/// Try to infer patch name from the argument path
fn infer_patch_from_arg(arg: &str, action: &str) -> String {
    // For file-based actions, try to extract a reasonable patch name
    match action {
        "folder" | "doc" | "file" | "anchor" => {
            // Expand tilde first
            let expanded = crate::utils::expand_tilde(arg);
            let path = std::path::Path::new(&expanded);

            // Try to get parent folder name as patch
            if let Some(parent) = path.parent() {
                if let Some(name) = parent.file_name() {
                    return name.to_string_lossy().to_string();
                }
            }
            // Fall back to file/folder name itself
            if let Some(name) = path.file_name() {
                return name.to_string_lossy().to_string();
            }
        }
        "chrome" | "safari" | "firefox" => {
            return "Web".to_string();
        }
        _ => {}
    }

    // Default fallback
    "Commands".to_string()
}

/// Print usage information
fn print_usage() {
    print("Usage: ha --define name:=NAME action:=ACTION arg:=ARG [patch:=PATCH] [flags:=FLAGS]");
    print("");
    print("Required fields:");
    print("  name:=   (or n:=)  Command display name");
    print("  action:= (or a:=)  Action type (chrome, doc, folder, anchor, etc.)");
    print("  arg:=    (or r:=)  Argument (URL, file path)");
    print("");
    print("Optional fields:");
    print("  patch:=  (or p:=)  Patch/dispatcher (inferred from path if omitted)");
    print("  flags:=  (or f:=)  Flags: A=anchor, U=user-edited, I=include");
    print("");
    print("Examples:");
    print("  ha --define n:=Google a:=chrome r:=https://google.com p:=Web");
    print("  ha --define name:=MyDocs action:=folder arg:=~/Documents");
    print("  ha --define n:=ProjectRoot a:=anchor r:=~/projects/myapp f:=A");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expand_aliases() {
        let mut params = HashMap::new();
        params.insert("n".to_string(), "MyName".to_string());
        params.insert("a".to_string(), "chrome".to_string());
        params.insert("r".to_string(), "https://example.com".to_string());

        let expanded = expand_aliases(params);

        assert_eq!(expanded.get("name"), Some(&"MyName".to_string()));
        assert_eq!(expanded.get("action"), Some(&"chrome".to_string()));
        assert_eq!(expanded.get("arg"), Some(&"https://example.com".to_string()));
    }

    #[test]
    fn test_expand_aliases_preserves_long_form() {
        let mut params = HashMap::new();
        params.insert("name".to_string(), "LongName".to_string());
        params.insert("n".to_string(), "ShortName".to_string()); // Should be ignored

        let expanded = expand_aliases(params);

        // Long form takes precedence
        assert_eq!(expanded.get("name"), Some(&"LongName".to_string()));
    }

    #[test]
    fn test_infer_patch_chrome() {
        assert_eq!(infer_patch_from_arg("https://google.com", "chrome"), "Web");
        assert_eq!(infer_patch_from_arg("https://github.com", "safari"), "Web");
    }

    #[test]
    fn test_infer_patch_folder() {
        // Should return parent folder name
        let patch = infer_patch_from_arg("/Users/test/Documents/MyProject", "folder");
        assert_eq!(patch, "Documents");
    }

    #[test]
    fn test_define_and_cleanup() {
        // This test requires SysData to be initialized
        // Skip if running in unit test mode without full initialization
        if std::panic::catch_unwind(|| crate::core::get_commands()).is_err() {
            println!("Skipping persistence test - SysData not initialized");
            return;
        }

        // Create a unique test command name
        let test_name = format!("__TestDefine_{}", std::process::id());

        // Create the command
        let command = Command {
            patch: "TestPatch".to_string(),
            command: test_name.clone(),
            action: "chrome".to_string(),
            arg: "https://test-cleanup.example.com".to_string(),
            flags: String::new(),
            other_params: None,
            last_update: 0,
            file_size: None,
        };

        // Save it
        let save_result = save_command_atomic(command, None);
        assert!(save_result.is_ok(), "Failed to save test command");

        // Verify it exists
        let commands = crate::core::get_commands();
        let found = commands.iter().any(|c| c.command == test_name);
        assert!(found, "Test command should exist after save");

        // Clean up - delete the command
        let delete_result = crate::core::delete_command(&test_name);
        assert!(delete_result.is_ok(), "Failed to delete test command");

        // Verify it's gone
        let commands = crate::core::get_commands();
        let found = commands.iter().any(|c| c.command == test_name);
        assert!(!found, "Test command should be deleted");
    }
}
