//! Inference Commands
//!
//! Functions for analyzing and applying patch inference.

use crate::core::{load_commands_for_inference, run_patch_inference};
use crate::utils::logging::print;
use crate::prelude::*;

/// Show which commands would have their patches changed by inference (--infer).
pub fn run_infer_patches(args: &[String]) {
    // Check if a specific command name was provided
    if args.len() >= 3 {
        let command_name = &args[2];
        run_infer_single_command(command_name);
        return;
    }

    print("Analyzing patch inference changes...");

    // Load current commands (without inference and virtual anchor creation)
    let (_config, mut commands, patches) = load_commands_for_inference();

    print("\n=== COMMAND PATCH CHANGES ===");

    // Run inference without applying changes, but print to stdout
    let (changes_found, new_patches_to_add) = run_patch_inference(
        &mut commands,
        &patches,
        false, // apply_changes = false (just analyze)
        true,  // print_to_stdout = true (show proposed changes)
        true   // overwrite_patch = true (show all potential changes)
    );

    if changes_found == 0 {
        print("  No commands would have their patches changed.");
    }

    // Show orphan patches that would be created
    if !new_patches_to_add.is_empty() {
        print("\n=== PATCHES NEEDING VIRTUAL ANCHORS ===");
        for new_patch in &new_patches_to_add {
            print(&format!("  New patch needing virtual anchor: {}", new_patch));
        }
    }

    print("\n=== SUMMARY ===");
    print(&format!("  Commands that would change: {}", changes_found));
    print(&format!("  Patches needing virtual anchors: {}", new_patches_to_add.len()));

    if changes_found == 0 && new_patches_to_add.is_empty() {
        print("  No changes would be made.");
    } else {
        print("  Use --infer-all to apply these changes (normal startup only fills empty patches).");
    }
}

/// Show patch inference for a specific command.
fn run_infer_single_command(command_name: &str) {
    // Get current commands from singleton
    let (sys_data, _) = crate::core::get_sys_data();
    let commands_arc = sys_data.commands;
    let patches = sys_data.patches;

    // NOTE: folder_to_patch map is in SysData, queried via get_patch_for_folder()

    // Find the command by name
    let found_command = commands_arc.iter().find(|cmd| cmd.command == command_name);

    match found_command {
        Some(command) => {
            print(&format!("Command: {}", command.command));
            print(&format!("Current patch: '{}'", command.patch));

            // Test patch inference on this specific command
            match crate::core::inference::infer_patch_unified(command, &patches) {
                Some(inferred_patch) => {
                    print(&format!("Inferred patch: '{}'", inferred_patch));

                    if command.patch == inferred_patch {
                        print("✅ No change needed - current patch matches inferred patch");
                    } else if command.patch.is_empty() {
                        print(&format!("📄 Would assign patch: '{}' -> '{}'", command.patch, inferred_patch));
                    } else {
                        print(&format!("🔄 Would change patch: '{}' -> '{}'", command.patch, inferred_patch));
                    }
                }
                None => {
                    if command.patch.is_empty() {
                        print("No patch could be inferred (would remain empty)");
                    } else {
                        print(&format!("No patch could be inferred (would keep current: '{}')", command.patch));
                    }
                }
            }
        }
        None => {
            print(&format!("Command '{}' not found.", command_name));

            // Show similar commands as suggestions
            let similar_commands: Vec<&crate::core::Command> = commands_arc.iter()
                .filter(|cmd| cmd.command.to_lowercase().contains(&command_name.to_lowercase()))
                .take(5)
                .collect();

            if !similar_commands.is_empty() {
                print("\nSimilar commands found:");
                for cmd in similar_commands {
                    print(&format!("  {}", cmd.command));
                }
            }
        }
    }
}

/// Show patch inference changes and prompt user to apply them (--infer-all).
pub fn run_infer_all_patches(_args: &[String]) {
    print("Analyzing patch inference changes...");

    // Load current commands (without inference and virtual anchor creation)
    let (_config, mut commands, patches) = load_commands_for_inference();

    print("\n=== COMMAND PATCH CHANGES ===");

    // First run: Show changes without applying
    let (changes_found, new_patches_to_add) = run_patch_inference(
        &mut commands,
        &patches,
        false, // apply_changes = false (just analyze)
        true,  // print_to_stdout = true (show proposed changes)
        true   // overwrite_patch = true (show all potential changes including overwriting existing patches)
    );

    if changes_found == 0 {
        print("  No commands would have their patches changed.");
    }

    // Show orphan patches that would be created
    if !new_patches_to_add.is_empty() {
        print("\n=== PATCHES NEEDING VIRTUAL ANCHORS ===");
        for new_patch in &new_patches_to_add {
            print(&format!("  New patch needing virtual anchor: {}", new_patch));
        }
    }

    if changes_found == 0 && new_patches_to_add.is_empty() {
        print("\nNo changes would be made.");
        return;
    }

    // Prompt user for confirmation
    print("\n=== SUMMARY ===");
    print(&format!("  Commands that would change: {}", changes_found));
    print(&format!("  Patches needing virtual anchors: {}", new_patches_to_add.len()));
    print!("\nApply all changes? (y/n): ");
    use std::io::{self, Write};
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let input = input.trim().to_lowercase();

    if input == "y" || input == "yes" {
        // Get commands from singleton and apply changes
        let (sys_data, _) = crate::core::get_sys_data();
        let mut commands = (*sys_data.commands).clone();
        let patches = sys_data.patches;

        // Second run: Apply changes without printing (already shown above)
        let (applied_count, _) = run_patch_inference(
            &mut commands,
            &patches,
            true,  // apply_changes = true (apply the changes)
            false, // print_to_stdout = false (don't print since we already showed them)
            true   // overwrite_patch = true (apply all changes including overwriting existing patches)
        );

        // Save the updated commands to file
        if applied_count > 0 {
            match crate::core::set_commands(commands) {
                Ok(()) => {
                    print(&format!("Updated {} commands and saved to file.", applied_count));
                }
                Err(e) => {
                    log_error(&format!("Updated {} commands but failed to save: {}", applied_count, e));
                }
            }
        } else {
            print("No commands were actually updated.");
        }
    } else {
        print("No changes applied.");
    }
}
