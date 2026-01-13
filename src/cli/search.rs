//! Search Commands
//!
//! Functions for searching and querying commands.

use crate::core::{filter_commands, Command};
use crate::utils::logging::print;

// =============================================================================
// HELPER FUNCTIONS
// =============================================================================

/// Resolve alias to target command.
pub fn resolve_alias_to_target<'a>(cmd: &'a Command, all_commands: &'a [Command]) -> &'a Command {
    if cmd.action == "alias" {
        // Find the target command
        let target_lower = cmd.arg.to_lowercase();
        if let Some(target_cmd) = all_commands.iter().find(|c|
            c.command.to_lowercase() == target_lower ||
            // Handle patch! prefix format
            (c.command.contains('!') && c.command.split('!').nth(1).map(|s| s.trim().to_lowercase()) == Some(target_lower.clone()))
        ) {
            return target_cmd;
        }
    }
    // Return original if not an alias or target not found
    cmd
}

/// Extract folder path from a command.
pub fn get_command_folder(cmd: &Command) -> Option<String> {
    // Get arg_type from action config instead of hardcoding action names
    let arg_type = crate::execute::get_action_arg_type(&cmd.action);

    match arg_type.as_deref() {
        Some("folder") => {
            // For folder actions, the arg is the folder path
            if !cmd.arg.is_empty() {
                return Some(cmd.arg.clone());
            }
        }
        Some("file") => {
            // For file actions, extract directory from file path in arg
            if !cmd.arg.is_empty() {
                if let Some(parent) = std::path::Path::new(&cmd.arg).parent() {
                    return Some(parent.to_string_lossy().to_string());
                }
            }
        }
        _ => {}
    }
    None
}

/// Extract full file path from a command.
pub fn get_command_path(cmd: &Command) -> Option<String> {
    // Get arg_type from action config instead of hardcoding action names
    let arg_type = crate::execute::get_action_arg_type(&cmd.action);

    // For file/folder actions, the arg is the full path
    match arg_type.as_deref() {
        Some("file") | Some("folder") => {
            if !cmd.arg.is_empty() {
                return Some(cmd.arg.clone());
            }
        }
        _ => {}
    }
    None
}

// =============================================================================
// SEARCH COMMANDS
// =============================================================================

/// Run the match command (-m, --match).
pub fn run_match_command(args: &[String]) {
    if args.len() < 3 {
        print("Usage: ha -m, --match <query> [--exact] [--format=name|folder|path|json]");
        std::process::exit(1);
    }

    let query = &args[2];

    // Parse flags
    let mut exact_mode = false;
    let mut format = "name"; // default format
    let mut debug = false;

    for arg in args.iter().skip(3) {
        if arg == "--exact" {
            exact_mode = true;
        } else if arg.starts_with("--format=") {
            format = arg.strip_prefix("--format=").unwrap_or("name");
        } else if arg == "debug" {
            debug = true;
        }
    }

    let (sys_data, _) = crate::core::data::get_sys_data();
    let config = crate::core::data::get_config();

    // Always use the shared display logic (same as popup)
    let (mut filtered, _is_prefix_menu) = if debug {
        // Legacy debug mode only
        (filter_commands(&sys_data.commands, query, 10, debug), false)
    } else {
        // Use shared popup matching logic - handles aliases, prefix menus, scoring
        let (display_commands, is_prefix_menu, _prefix_menu_info, _prefix_menu_count, _default_selection, _filter_text) =
            crate::core::get_new_display_commands(query, &sys_data.commands, &sys_data.patches, &config);
        (display_commands, is_prefix_menu)
    };

    // Detect if first result is an exact match or prefix menu match
    // For prefix menus, require the first result to actually match the query
    let is_exact_match = !filtered.is_empty() &&
        crate::core::display::exact_match(&filtered[0].command, query);

    // Apply exact mode filtering if requested
    if exact_mode {
        if is_exact_match {
            // Keep only the first result (the exact/prefix menu match)
            filtered.truncate(1);
        } else {
            // No exact match found - filter all results for exact matches
            filtered.retain(|cmd| crate::core::display::exact_match(&cmd.command, query));
        }
    }

    // Determine exit code based on number of matches
    let exit_code = match filtered.len() {
        0 => 2, // No matches
        1 => 0, // Unique match
        _ => 1, // Multiple matches (ambiguous)
    };

    // Output based on format
    match format {
        "name" => {
            // Print first 50 command names
            for cmd in filtered.iter().take(50) {
                print(&format!("{}", cmd.command));
            }
        }
        "folder" => {
            // Print folder paths (resolve aliases, skip commands without folders)
            for cmd in filtered.iter().take(50) {
                let resolved = resolve_alias_to_target(cmd, &sys_data.commands);
                if let Some(folder) = get_command_folder(resolved) {
                    print(&folder);
                }
            }
        }
        "path" => {
            // Print full file paths (resolve aliases, skip commands without file paths)
            for cmd in filtered.iter().take(50) {
                let resolved = resolve_alias_to_target(cmd, &sys_data.commands);
                if let Some(path) = get_command_path(resolved) {
                    print(&path);
                }
            }
        }
        "json" => {
            // Print JSON objects (one per line)
            for cmd in filtered.iter().take(50) {
                if let Ok(json) = serde_json::to_string(cmd) {
                    print(&json);
                }
            }
        }
        _ => {
            print(&format!("Error: Unknown format '{}'", format));
            std::process::exit(1);
        }
    }

    std::process::exit(exit_code);
}

/// Get single folder path for top match (-p).
/// Returns just the folder path of the first matching command.
pub fn run_single_path_command(args: &[String]) {
    if args.len() < 3 {
        print("Usage: ha -p <query>");
        std::process::exit(1);
    }

    let query = &args[2];
    let (sys_data, _) = crate::core::data::get_sys_data();
    let config = crate::core::data::get_config();

    let (filtered, _) = {
        let (display_commands, is_prefix_menu, _, _, _, _) =
            crate::core::get_new_display_commands(query, &sys_data.commands, &sys_data.patches, &config);
        (display_commands, is_prefix_menu)
    };

    // Print first folder path only
    for cmd in filtered.iter().take(1) {
        let resolved = resolve_alias_to_target(cmd, &sys_data.commands);
        if let Some(folder) = get_command_folder(resolved) {
            print(&folder);
            std::process::exit(0);
        }
    }

    // No folder found
    std::process::exit(2);
}

/// Get folder paths for all matching commands (-P).
/// Returns folder paths for up to 50 matches.
pub fn run_paths_command(args: &[String]) {
    if args.len() < 3 {
        print("Usage: ha -P <query>");
        std::process::exit(1);
    }

    let query = &args[2];
    let (sys_data, _) = crate::core::data::get_sys_data();
    let config = crate::core::data::get_config();

    let (filtered, _) = {
        let (display_commands, is_prefix_menu, _, _, _, _) =
            crate::core::get_new_display_commands(query, &sys_data.commands, &sys_data.patches, &config);
        (display_commands, is_prefix_menu)
    };

    // Print folder paths (resolve aliases, skip commands without folders)
    for cmd in filtered.iter().take(50) {
        let resolved = resolve_alias_to_target(cmd, &sys_data.commands);
        if let Some(folder) = get_command_folder(resolved) {
            print(&folder);
        }
    }

    let exit_code = match filtered.len() {
        0 => 2,
        1 => 0,
        _ => 1,
    };
    std::process::exit(exit_code);
}

/// Get single command name with folder path (-f).
/// Returns: COMMAND -> /path/to/folder (first match only).
pub fn run_single_named_folder_command(args: &[String]) {
    if args.len() < 3 {
        print("Usage: ha -f <query>");
        std::process::exit(1);
    }

    let query = &args[2];
    let (sys_data, _) = crate::core::data::get_sys_data();
    let config = crate::core::data::get_config();

    let (filtered, _) = {
        let (display_commands, is_prefix_menu, _, _, _, _) =
            crate::core::get_new_display_commands(query, &sys_data.commands, &sys_data.patches, &config);
        (display_commands, is_prefix_menu)
    };

    // Print first command name with folder path
    for cmd in filtered.iter().take(1) {
        let resolved = resolve_alias_to_target(cmd, &sys_data.commands);
        if let Some(folder) = get_command_folder(resolved) {
            print(&format!("{} -> {}", cmd.command, folder));
            std::process::exit(0);
        }
    }

    // No folder found
    std::process::exit(2);
}

/// Get command names with their folder paths (-F).
/// Prints: COMMAND -> /path/to/folder (all matches).
pub fn run_named_folders_command(args: &[String]) {
    if args.len() < 3 {
        print("Usage: ha -F <query>");
        std::process::exit(1);
    }

    let query = &args[2];
    let (sys_data, _) = crate::core::data::get_sys_data();
    let config = crate::core::data::get_config();

    let (filtered, _) = {
        let (display_commands, is_prefix_menu, _, _, _, _) =
            crate::core::get_new_display_commands(query, &sys_data.commands, &sys_data.patches, &config);
        (display_commands, is_prefix_menu)
    };

    // Print command names with folder paths
    for cmd in filtered.iter().take(50) {
        let resolved = resolve_alias_to_target(cmd, &sys_data.commands);
        if let Some(folder) = get_command_folder(resolved) {
            print(&format!("{} -> {}", cmd.command, folder));
        }
    }

    let exit_code = match filtered.len() {
        0 => 2,
        1 => 0,
        _ => 1,
    };
    std::process::exit(exit_code);
}
