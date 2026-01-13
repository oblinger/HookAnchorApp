//! Legacy Commands
//!
//! Functions for comparing legacy command files with current state.
//! Used for debugging and migration purposes.

use crate::core::Command;
use crate::utils::logging::print;
use crate::prelude::*;
use crate::cli::maintenance::run_rescan_command;

/// Comparison result between two command sets.
#[derive(Debug)]
struct ComparisonResult {
    unchanged: Vec<String>,
    changed: Vec<CommandChange>,
    removed: Vec<Command>,
    added: Vec<Command>,
}

/// A single command change showing old and new values.
#[derive(Debug)]
struct CommandChange {
    command_name: String,
    old: Command,
    new: Command,
}

/// Compare two command vectors and return detailed comparison.
fn compare_commands(old_commands: &[Command], new_commands: &[Command]) -> ComparisonResult {
    use std::collections::HashMap;

    // Build hash maps for quick lookup using the same deduplication key
    // This ensures we're comparing commands the same way the system deduplicates them
    let old_map: HashMap<String, &Command> = old_commands.iter()
        .map(|cmd| (crate::core::data::command_dedup_key(cmd), cmd))
        .collect();

    let new_map: HashMap<String, &Command> = new_commands.iter()
        .map(|cmd| (crate::core::data::command_dedup_key(cmd), cmd))
        .collect();

    let mut unchanged = Vec::new();
    let mut changed = Vec::new();
    let mut removed = Vec::new();
    let mut added = Vec::new();

    // Check old commands
    for (key, old_cmd) in &old_map {
        if let Some(new_cmd) = new_map.get(key) {
            // Command exists in both - check if changed
            if commands_equal(old_cmd, new_cmd) {
                unchanged.push(old_cmd.command.clone());
            } else {
                changed.push(CommandChange {
                    command_name: old_cmd.command.clone(),
                    old: (*old_cmd).clone(),
                    new: (*new_cmd).clone(),
                });
            }
        } else {
            // Command was removed
            removed.push((*old_cmd).clone());
        }
    }

    // Check for added commands
    for (key, new_cmd) in &new_map {
        if !old_map.contains_key(key) {
            added.push((*new_cmd).clone());
        }
    }

    ComparisonResult {
        unchanged,
        changed,
        removed,
        added,
    }
}

/// Check if two commands are equal (comparing all relevant fields).
fn commands_equal(cmd1: &Command, cmd2: &Command) -> bool {
    cmd1.command == cmd2.command &&
    cmd1.action == cmd2.action &&
    cmd1.arg == cmd2.arg &&
    cmd1.flags == cmd2.flags &&
    cmd1.patch == cmd2.patch
}

/// Print comparison results to console and log.
fn print_comparison(title: &str, result: &ComparisonResult) {
    print("");
    print(&format!("=== {} ===", title));
    print("");

    print("📊 Summary:");
    print(&format!("  Unchanged: {}", result.unchanged.len()));
    print(&format!("  Changed:   {}", result.changed.len()));
    print(&format!("  Removed:   {}", result.removed.len()));
    print(&format!("  Added:     {}", result.added.len()));
    print("");

    if !result.changed.is_empty() {
        print(&format!("🔄 Changed Commands ({}):", result.changed.len()));
        for change in &result.changed {
            print(&format!("  {}", change.command_name));

            if change.old.action != change.new.action {
                print(&format!("    action:  '{}' → '{}'", change.old.action, change.new.action));
            }
            if change.old.arg != change.new.arg {
                print(&format!("    arg:     '{}' → '{}'", change.old.arg, change.new.arg));
            }
            if change.old.flags != change.new.flags {
                print(&format!("    flags:   '{}' → '{}'", change.old.flags, change.new.flags));
            }
            if change.old.patch != change.new.patch {
                print(&format!("    patch:   '{}' → '{}'", change.old.patch, change.new.patch));
            }
        }
        print("");
    }

    if !result.removed.is_empty() {
        print(&format!("➖ Removed Commands ({}):", result.removed.len()));
        for cmd in result.removed.iter().take(20) {
            print(&format!("  {} ({}:{})", cmd.command, cmd.patch, cmd.action));
        }
        if result.removed.len() > 20 {
            print(&format!("  ... and {} more", result.removed.len() - 20));
        }
        print("");
    }

    if !result.added.is_empty() {
        print(&format!("➕ Added Commands ({}):", result.added.len()));
        for cmd in result.added.iter().take(20) {
            print(&format!("  {} ({}:{})", cmd.command, cmd.patch, cmd.action));
        }
        if result.added.len() > 20 {
            print(&format!("  ... and {} more", result.added.len() - 20));
        }
        print("");
    }
}

/// Load legacy commands and compare with current state (--load-legacy-and-compare).
pub fn run_load_legacy_and_compare(args: &[String]) {
    if args.len() < 3 {
        print("Usage: ha --load-legacy-and-compare <path>");
        print("Example: ha --load-legacy-and-compare ~/.config/hookanchor/commands_20251017_232108.txt");
        std::process::exit(1);
    }

    let legacy_path = &args[2];
    let legacy_path_expanded = crate::utils::expand_tilde(legacy_path);

    print("🔍 Legacy Commands Load and Compare");
    print("===================================");
    print("");
    print(&format!("📁 Legacy file: {}", legacy_path_expanded));
    print("");

    // Step 1: Rescan
    print("🔄 Step 1: Performing initial rescan...");
    run_rescan_command();
    print("✅ Initial rescan complete");
    print("");

    // Step 2: Load legacy commands
    print("📥 Step 2: Loading legacy commands from file...");
    let legacy_commands = load_commands_from_file(&legacy_path_expanded);
    print(&format!("✅ Loaded {} legacy commands", legacy_commands.len()));
    print("");

    // Step 3: Save legacy commands (which triggers flush)
    print("💾 Step 3: Saving legacy commands (triggers flush)...");
    let commands_before_save = legacy_commands.clone();
    match crate::core::set_commands(legacy_commands) {
        Ok(()) => {
            print("✅ Legacy commands saved and flushed");
        }
        Err(e) => {
            print(&format!("❌ Failed to save commands: {}", e));
            std::process::exit(1);
        }
    }
    print("");

    // Step 4: First comparison - check if save/flush corrupted data
    print("🔬 Step 4: First comparison (legacy vs. saved)...");
    let commands_after_save = crate::core::get_commands();
    let comparison1 = compare_commands(&commands_before_save, &commands_after_save);
    print_comparison("POST-SAVE COMPARISON", &comparison1);

    // Step 5: Rescan again
    print("🔄 Step 5: Performing second rescan...");
    run_rescan_command();
    print("✅ Second rescan complete");
    print("");

    // Step 6: Second comparison - check if rescan corrupted data
    print("🔬 Step 6: Second comparison (saved vs. rescanned)...");
    let commands_after_rescan = crate::core::get_commands();
    let comparison2 = compare_commands(&commands_after_save, &commands_after_rescan);
    print_comparison("POST-RESCAN COMPARISON", &comparison2);

    print("✅ Legacy load and compare complete!");
}

/// Load commands from a specific file path.
fn load_commands_from_file(path: &str) -> Vec<Command> {
    use std::fs;

    if !std::path::Path::new(path).exists() {
        print(&format!("❌ File not found: {}", path));
        std::process::exit(1);
    }

    match fs::read_to_string(path) {
        Ok(contents) => {
            let mut commands = Vec::new();
            for (line_num, line) in contents.lines().enumerate() {
                let trimmed = line.trim();

                // Skip empty lines and comments
                if trimmed.is_empty() || trimmed.starts_with("//") {
                    continue;
                }

                match crate::core::commands::parse_command_line(line) {
                    Ok(command) => {
                        commands.push(command);
                    },
                    Err(e) => {
                        log_error(&format!("Failed to parse line {} in {}: {} - Line: '{}'",
                            line_num + 1, path, e, line));
                    }
                }
            }
            commands
        }
        Err(e) => {
            print(&format!("❌ Failed to read file: {}", e));
            std::process::exit(1);
        }
    }
}
