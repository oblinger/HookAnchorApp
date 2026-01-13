//! Maintenance Commands
//!
//! Functions for system maintenance, diagnostics, and testing.

use std::collections::HashMap;
use crate::utils;
use crate::utils::logging::print;
use crate::execute::{execute_on_server, make_action};
use crate::prelude::*;

/// Test the grabber functionality - capture context and try to match rules (--test-grabber).
pub fn run_test_grabber() {
    print("Testing grabber functionality...");
    print("==================================");

    // Load config
    let config = crate::core::data::get_config();
    print(&format!("Loaded config with {} grabber rules",
        config.grabber_rules.as_ref().map(|r| r.len()).unwrap_or(0)));

    // Try to capture and test context
    print("\nCapturing active application context...");
    match crate::systems::grab_debug(&config) {
        Ok(context) => {
            print("SUCCESS: Captured app context");
            print(&format!("  App: '{}'", context.app_name));
            print(&format!("  Bundle ID: '{}'", context.bundle_id));
            print(&format!("  Title: '{}'", context.window_title));

            // Show properties if any
            if let Some(props_obj) = context.properties.as_object() {
                if !props_obj.is_empty() {
                    print("  Properties:");
                    for (key, value) in props_obj {
                        print(&format!("    {}: '{}'", key, value.as_str().unwrap_or("(complex value)")));
                    }
                }
            }
        }
        Err(e) => {
            print(&format!("ERROR: Failed to capture app context: {}", e));
            std::process::exit(1);
        }
    }

    print("\nGrabber test completed successfully!");
}

/// Test accessibility permissions for HookAnchor (--test-permissions).
pub fn run_test_permissions() {
    print("Testing Accessibility Permissions");
    print("==================================");
    print("Checking if HookAnchor has the required accessibility permissions...");
    print("");

    match crate::systems::setup_assistant::SetupAssistant::test_accessibility_permissions() {
        Ok(true) => {
            print("✅ SUCCESS: Accessibility permissions are granted!");
            print("HookAnchor can capture app context and send keystrokes.");
            print("");
            print("This means:");
            print("• Obsidian URL capture should work");
            print("• Notion URL capture should work");
            print("• Window title detection works");
            print("• Grabber functionality is fully operational");
        }
        Ok(false) => {
            print("❌ FAILED: Accessibility permissions are missing!");
            print("");
            print("This explains why:");
            print("• Obsidian grabber defaults to generic app capture");
            print("• Notion URL capture fails");
            print("• Some grabber features don't work");
            print("");
            print("To fix this, run:");
            print("  ha --install");
            print("");
            print("Or manually:");
            print("1. Open System Preferences → Security & Privacy → Privacy → Accessibility");
            print("2. Click the lock and enter your password");
            print("3. Add Terminal (or your current app) to the allowed list");
            print("4. Test again with: ha --test-permissions");
        }
        Err(e) => {
            print(&format!("❌ ERROR: Could not test permissions: {}", e));
            print("This might indicate a system configuration issue.");
        }
    }
}

/// Run grab command to capture active app and output result (--grab).
pub fn run_grab_command(args: &[String]) {
    // Parse optional delay parameter (defaults to 0)
    let delay_seconds = if args.len() > 2 {
        match args[2].parse::<u64>() {
            Ok(d) => d,
            Err(_) => {
                print(&format!("Invalid delay value: {}", args[2]));
                print("Usage: ha --grab [delay_seconds]");
                std::process::exit(1);
            }
        }
    } else {
        0
    };

    // If delay requested, sleep before capturing
    if delay_seconds > 0 {
        print(&format!("Waiting {} seconds before capture...", delay_seconds));
        std::thread::sleep(std::time::Duration::from_secs(delay_seconds));
    }

    // Load config for grabber rules
    let config = crate::core::data::get_config();

    // Perform the grab
    match crate::systems::grabber::grab(&config) {
        Ok(grab_result) => {
            match grab_result {
                crate::systems::grabber::GrabResult::RuleMatched(rule_name, command) => {
                    // Output action, argument, rule name, and suffix for the popup to process
                    // Format: "action arg RULE:rule_name FLAGS:suffix"
                    if command.flags.is_empty() {
                        print(&format!("{} {} RULE:{}", command.action, command.arg, rule_name));
                    } else {
                        print(&format!("{} {} RULE:{} FLAGS:{}", command.action, command.arg, rule_name, command.flags));
                    }
                }
                crate::systems::grabber::GrabResult::NoRuleMatched(context) => {
                    // No rule matched - output context information
                    print("No grabber rule matched for:");
                    print(&format!("  App: {}", context.app_name));
                    print(&format!("  Bundle ID: {}", context.bundle_id));
                    print(&format!("  Title: {}", context.window_title));
                    if let Some(props) = context.properties.as_object() {
                        for (key, value) in props {
                            print(&format!("  props.{}: {}", key, value.as_str().unwrap_or("(complex)")));
                        }
                    }
                    std::process::exit(1);
                }
            }
        }
        Err(e) => {
            log_error(&format!("Error capturing active app: {}", e));
            std::process::exit(1);
        }
    }
}

/// Rescan filesystem for command discovery (--rescan).
pub fn run_rescan_command() {
    print("🚀 HookAnchor Rescan - Verbose Mode");
    print("====================================");

    // Load configuration
    let config = crate::core::data::get_config();

    // Debug output
    print("📋 Config loaded - checking file_roots...");

    // Get file roots
    let _file_roots = match &config.popup_settings.file_roots {
        Some(roots) => {
            print(&format!("✅ File roots found: {:?}", roots));
            roots.clone()
        },
        None => {
            print("❌ No file roots configured in config file");
            print(&format!("    popup_settings exists: {}", config.popup_settings.file_roots.is_some()));
            std::process::exit(1);
        }
    };

    // === NEW CLEAN SINGLETON ARCHITECTURE ===
    // This eliminates duplicate history entries by using a single shared list

    print("\n📂 Step 1: Loading last known state from cache...");

    // Load from cache through sys_data
    let mut commands = crate::core::get_commands();
    if !commands.is_empty() {
        print(&format!("   ✅ Loaded {} commands from cache", commands.len()));
    } else {
        print("   Starting with empty commands");
    }

    print("\n✏️  Step 2: Merging manual edits from commands.txt...");

    // Load commands.txt and merge on top of cache state
    // This captures any manual edits while preserving file sizes and history from cache
    match crate::systems::load_manual_edits(&mut commands, true) {
        Ok(edits_count) => {
            print(&format!("   ✅ Merged {} edits from commands.txt", edits_count));
        }
        Err(e) => {
            print(&format!("   ⚠️  Error loading manual edits: {}", e));
        }
    }
    print(&format!("   📊 Total commands after merge: {}", commands.len()));

    // Track HA alias after loading manual edits
    let ha_after_load = commands.iter().find(|c| c.command == "HA" && c.action == "alias");
    if let Some(ha) = ha_after_load {
        log(&format!("ALIAS_TRACK: HA alias found after load_manual_edits - patch:'{}' flags:'{}' arg:'{}'", ha.patch, ha.flags, ha.arg));
    } else {
        log("ALIAS_TRACK: HA alias NOT found after load_manual_edits");
    }

    print("\n🔍 Step 3: Scanning filesystem (discovering new files, removing stale entries)...");

    // Scan filesystem - this will:
    // - Add newly discovered files
    // - Remove commands for files that no longer exist
    // - Preserve user-edited commands
    let (global_data, _) = crate::core::data::get_sys_data();

    let scanned_commands = crate::systems::scan_new_files(
        commands.clone(),
        &global_data,
        true
    );

    commands = scanned_commands;
    print(&format!("   ✅ Scan complete - now tracking {} total commands", commands.len()));

    // Track HA alias after filesystem scan
    let ha_after_scan = commands.iter().find(|c| c.command == "HA" && c.action == "alias");
    if let Some(ha) = ha_after_scan {
        log(&format!("ALIAS_TRACK: HA alias found after scan_new_files - patch:'{}' flags:'{}' arg:'{}'", ha.patch, ha.flags, ha.arg));
    } else {
        log("ALIAS_TRACK: HA alias NOT found after scan_new_files");
    }

    print("\n📝 Step 4: Detecting file modifications...");

    // Check if any file sizes changed and record history
    match crate::systems::scan_modified_files(&mut commands, true) {
        Ok(modified_count) => {
            print(&format!("   ✅ Processed modifications ({} files changed)", modified_count));
        }
        Err(e) => {
            print(&format!("   ⚠️  Error scanning modifications: {}", e));
        }
    }

    // Track HA alias after modification scan
    let ha_after_mod = commands.iter().find(|c| c.command == "HA" && c.action == "alias");
    if let Some(ha) = ha_after_mod {
        log(&format!("ALIAS_TRACK: HA alias found after scan_modified_files - patch:'{}' flags:'{}' arg:'{}'", ha.patch, ha.flags, ha.arg));
    } else {
        log("ALIAS_TRACK: HA alias NOT found after scan_modified_files");
    }

    // Check if delete_broken_aliases is enabled in config (default: true)
    let delete_broken = config.scanner_settings
        .as_ref()
        .and_then(|s| s.delete_broken_aliases)
        .unwrap_or(true);

    if delete_broken {
        print("\n🧹 Step 5: Cleaning up invalid aliases...");

        // Delete aliases that point to non-existent commands
        match crate::systems::delete_invalid_aliases(&mut commands, true) {
            Ok(removed_count) => {
                if removed_count > 0 {
                    print(&format!("   ✅ Removed {} invalid alias(es)", removed_count));
                } else {
                    print("   ✅ All aliases are valid");
                }
            }
            Err(e) => {
                print(&format!("   ⚠️  Error validating aliases: {}", e));
            }
        }
    } else {
        print("\n⏭️  Step 5: Skipping alias cleanup (delete_broken_aliases: false)");
    }

    print("\n🔄 Step 6: Running inference and saving...");

    // Save final state through sys_data (runs inference + saves to cache + commands.txt)
    let command_count = commands.len();
    match crate::core::set_commands(commands) {
        Ok(_) => {
            print(&format!("   ✅ Inference complete and saved {} commands", command_count));
        }
        Err(e) => {
            print(&format!("   ⚠️  Error during inference/save: {}", e));
        }
    }

    // Reload commands for summary (after save moved ownership)
    let commands = crate::core::get_commands();

    // Track HA alias after set_commands (final state)
    let ha_after_save = commands.iter().find(|c| c.command == "HA" && c.action == "alias");
    if let Some(ha) = ha_after_save {
        log(&format!("ALIAS_TRACK: HA alias found after set_commands - patch:'{}' flags:'{}' arg:'{}'", ha.patch, ha.flags, ha.arg));
    } else {
        log("ALIAS_TRACK: HA alias NOT found after set_commands - IT WAS LOST!");
    }

    print("\n📊 Step 8: Final Summary:");
    print(&format!("   Total commands: {}", commands.len()));

    // Count commands by action type
    let mut action_counts: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
    for cmd in &commands {
        *action_counts.entry(cmd.action.clone()).or_insert(0) += 1;
    }

    print("\n   Commands by action type:");
    let mut sorted_actions: Vec<_> = action_counts.iter().collect();
    sorted_actions.sort_by_key(|(action, _)| (*action).clone());

    for (action, count) in sorted_actions {
        print(&format!("     {}: {}", action, count));
    }

    print("\n✅ Rescan complete!");
}

/// Execute a launcher command - used by launchctl asuser to run commands in GUI session (--execute-launcher-command).
pub fn run_execute_launcher_command(args: &[String]) {
    if args.len() < 3 {
        print("Usage: ha --execute-launcher-command <launcher_command>");
        std::process::exit(1);
    }

    let launcher_command = &args[2];

    // Visual separator for launcher command execution
    detailed_log("", "=================================================================");
    detailed_log("LAUNCHER_CMD", &format!("Executing in GUI session: '{}'", launcher_command));

    // Execute the launcher command via server (consistent with all execution)
    // Parse the launcher command to create a Command object
    let parts: Vec<&str> = launcher_command.split_whitespace().collect();
    let (action, arg) = if parts.len() > 1 {
        (parts[0], parts[1..].join(" "))
    } else {
        (launcher_command.as_str(), String::new())
    };

    // Create action directly and execute
    let action_obj = make_action(action, &arg);
    let mut variables = HashMap::new();
    variables.insert("arg".to_string(), arg.clone());
    let _ = execute_on_server(&action_obj, Some(variables));
    detailed_log("LAUNCHER_CMD", "Command completed");
}

/// Run rebuild command: restart server and rescan filesystem (--rebuild).
pub fn run_rebuild_command() {
    // Clear log file before starting rebuild
    crate::utils::clear_debug_log();

    // Generate a unique build identifier (timestamp-based)
    let build_timestamp = chrono::Local::now();
    let build_id = build_timestamp.format("%Y%m%d_%H%M%S").to_string();

    // Log the rebuild header with timestamp and build ID
    log(&"=".repeat(80));
    detailed_log("SYSTEM", &format!("REBUILD SESSION: {}", build_id));
    log(&format!("TIMESTAMP: {}", build_timestamp.format("%Y-%m-%d %H:%M:%S%.3f")));
    log(&"=".repeat(80));
    detailed_log("REBUILD", &format!("Starting rebuild session {}", build_id));

    print("🏗️  HookAnchor Rebuild - Full Reset");
    print("===================================");

    // Step 1: Build the release binary
    print("\n🔨 Step 1/3: Building release binary...");
    let build_start = std::time::Instant::now();

    // Get project directory from executable location
    let project_dir = match crate::utils::build_verification::get_project_dir() {
        Some(dir) if dir.join("Cargo.toml").exists() => dir,
        _ => {
            print("  ❌ Cannot determine project directory from executable location");
            print("     Expected: <project>/target/release/ha");
            std::process::exit(1);
        }
    };

    // Run cargo build --release using standard rustup location
    let home = std::env::var("HOME").unwrap_or_default();
    let cargo = format!("{}/.cargo/bin/cargo", home);

    let build_output = std::process::Command::new(&cargo)
        .arg("build")
        .arg("--release")
        .current_dir(&project_dir)
        .output();

    match build_output {
        Ok(output) => {
            if output.status.success() {
                let build_time = build_start.elapsed();
                print(&format!("  ✅ Build successful ({:.1}s)", build_time.as_secs_f32()));
            } else {
                let stderr = String::from_utf8_lossy(&output.stderr);
                print("  ❌ Build failed:");
                print(&format!("{}", stderr));
                print("\n⚠️  Cannot proceed with rebuild due to compilation errors");
                std::process::exit(1);
            }
        }
        Err(e) => {
            print(&format!("  ❌ Failed to run cargo build: {}", e));
            print("\n⚠️  Make sure cargo is installed and in PATH");
            std::process::exit(1);
        }
    }

    print("\n🔄 Step 2/3: Restarting command server...");

    // Clear the socket file to ensure clean start (using centralized path function)
    let socket_path = crate::systems::get_execution_server_socket_path();
    if socket_path.exists() {
        if let Err(e) = std::fs::remove_file(&socket_path) {
            print(&format!("  ⚠️  Failed to remove socket file: {}", e));
        }
    }

    // Restart the server
    match crate::execute::activate_command_server(true) {
        Ok(()) => {
            print("  ✅ Server restart initiated");
            print("  📱 A new Terminal window should open with the server daemon");
        },
        Err(e) => {
            print(&format!("  ❌ Failed to restart server: {}", e));
            return;
        }
    }

    print("\n📁 Step 3/3: Rescanning filesystem...");

    // Run filesystem rescan
    run_rescan_command();

    print("\n🎉 Rebuild complete!");
}

/// Launch history viewer (--search).
pub fn run_search_command() {
    use std::process::Command;

    detailed_log("SEARCH_CMD", "Search command requested - launching history viewer");

    // Find history viewer binary
    let exe_dir = utils::get_binary_dir();
    let viewer_path = exe_dir.join("HookAnchorHistoryViewer");

    if !viewer_path.exists() {
        log_error(&format!("History viewer not found at: {:?}", viewer_path));
        print(&format!("History viewer not found at: {:?}", viewer_path));
        std::process::exit(1);
    }

    detailed_log("SEARCH_CMD", &format!("Launching history viewer: {:?}", viewer_path));

    // Launch history viewer as a background process
    match Command::new(&viewer_path)
        .spawn() {
        Ok(_) => {
            detailed_log("SEARCH_CMD", "History viewer launched successfully");
            print("History viewer launched");
        }
        Err(e) => {
            log_error(&format!("Failed to launch history viewer: {}", e));
            print(&format!("Failed to launch history viewer: {}", e));
            std::process::exit(1);
        }
    }
}

/// Delete history database and command cache (--delete-history).
///
/// This CLI command performs a complete reset by calling `delete_history()` from the
/// data layer, which executes the following steps:
///
/// 1. **Delete history database** (~/.config/hookanchor/history.db)
///    - Removes all command execution history
///    - Removes all file modification tracking records
///
/// 2. **Delete command cache** (~/.config/hookanchor/commands_cache.json)
///    - Forces full filesystem rescan on next startup
///    - Ensures cache and history stay in sync (no orphaned cache entries)
///
/// After deletion, this command performs a two-step rebuild:
/// - Step 1: Scan filesystem with empty commands to record file creation timestamps
/// - Step 2: Run full rescan to merge manual edits from commands.txt
///
/// This operation is irreversible. Historical execution data is lost forever, but the
/// next rescan will rebuild the cache from scratch by scanning all file_roots.
pub fn run_delete_history(args: &[String]) {
    use std::io::{self, Write};
    use std::path::PathBuf;

    // Parse arguments
    let force = args.iter().any(|arg| arg == "--force");

    // Find --use-commands <path> argument
    let mut use_commands_path: Option<PathBuf> = None;
    let mut iter = args.iter();
    while let Some(arg) = iter.next() {
        if arg == "--use-commands" {
            if let Some(path) = iter.next() {
                use_commands_path = Some(PathBuf::from(path));
            } else {
                print_and_log("Error: --use-commands requires a file path");
                std::process::exit(1);
            }
        }
    }

    // Validate --use-commands path if provided
    if let Some(ref path) = use_commands_path {
        if !path.exists() {
            print_and_log(&format!("Error: Specified commands file does not exist: {}", path.display()));
            std::process::exit(1);
        }
        if !path.is_file() {
            print_and_log(&format!("Error: Path is not a file: {}", path.display()));
            std::process::exit(1);
        }
    }

    // Show what will happen
    if !force {
        print_and_log("");
        print_and_log("⚠️  WARNING: This will:");
        print_and_log("  1. Create automatic backup of current state");
        print_and_log("  2. Stop all servers");
        print_and_log("  3. Delete history database and cache");
        print_and_log("  4. Clear commands.txt");
        print_and_log("  5. Scan filesystem to record creation dates");
        if let Some(ref path) = use_commands_path {
            print_and_log(&format!("  6. Restore commands from: {}", path.display()));
        }
        print_and_log("  7. Merge and rebuild with final filesystem scan");
        print_and_log("  8. Restart all servers");
        print_and_log("");
        print_and_log("Type 'yes' to confirm: ");

        // Flush stdout to ensure prompt appears
        io::stdout().flush().unwrap();

        // Read user input
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input != "yes" {
            print_and_log("Operation cancelled.");
            return;
        }
    }

    print_and_log("");
    print_and_log("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    print_and_log("  HISTORY REBUILD WITH RESTORATION");
    print_and_log("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    print_and_log("");

    // Get config_dir path for later steps
    let _config_dir = dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("/tmp"))
        .join("hookanchor");

    // Step 1: Create automatic backup
    print_and_log("📦 Step 1/8: Creating automatic backup...");
    let backup_dir = match crate::core::backup_commands() {
        Ok(dir) => {
            print_and_log(&format!("  ✓ Backup created: {}", dir.display()));
            dir
        }
        Err(e) => {
            print_and_log(&format!("  ✗ Failed to create backup: {}", e));
            print_and_log("  Operation aborted for safety");
            std::process::exit(1);
        }
    };

    // Step 2: Stop all servers via supervisor
    print_and_log("");
    print_and_log("🛑 Step 2/8: Stopping all servers...");
    if let Err(e) = crate::systems::supervisor_command("stop") {
        print_and_log(&format!("  ✗ Failed to stop servers: {}", e));
        print_and_log("  Continuing anyway...");
    } else {
        print_and_log("  ✓ All servers stopped");
    }

    // Step 3: Delete history and cache
    print_and_log("");
    print_and_log("🗑️  Step 3/8: Deleting history database and cache...");
    match crate::core::data::delete_history() {
        Ok((history_deleted, cache_deleted)) => {
            if history_deleted {
                print_and_log("  ✓ Deleted history database");
            } else {
                print_and_log("  ℹ History database not found");
            }
            if cache_deleted {
                print_and_log("  ✓ Deleted command cache");
            } else {
                print_and_log("  ℹ Command cache not found");
            }
        }
        Err(e) => {
            print_and_log(&format!("  ✗ Error: {}", e));
            print_and_log(&format!("  Backup preserved at: {}", backup_dir.display()));
            std::process::exit(1);
        }
    }

    // Step 4: Clear commands via data layer (clears singleton + both files)
    print_and_log("");
    print_and_log("🧹 Step 4/8: Clearing commands via data layer...");
    match crate::core::clear_commands() {
        Ok(()) => {
            print_and_log("  ✓ Singleton and files cleared");
        }
        Err(e) => {
            print_and_log(&format!("  ✗ Failed to clear commands: {}", e));
            print_and_log(&format!("  Backup preserved at: {}", backup_dir.display()));
            std::process::exit(1);
        }
    }

    // Step 5: Initial filesystem scan (records creation dates)
    print_and_log("");
    print_and_log("🔍 Step 5/8: Scanning filesystem to record creation dates...");
    let (global_data, _) = crate::core::data::get_sys_data();
    let scanned_commands = crate::systems::scan_new_files(Vec::new(), &global_data, true);

    // Save via data layer (updates singleton + both files)
    if let Err(e) = crate::core::set_commands(scanned_commands.clone()) {
        print_and_log(&format!("  ⚠️  Failed to save scanned commands: {}", e));
        print_and_log(&format!("  Backup preserved at: {}", backup_dir.display()));
        std::process::exit(1);
    } else {
        print_and_log(&format!("  ✓ Recorded creation history for {} files", scanned_commands.len()));
    }

    // Step 6: Restore commands.txt if --use-commands specified
    if let Some(ref source_path) = use_commands_path {
        print_and_log("");
        print_and_log("📂 Step 6/8: Restoring commands from backup...");
        print_and_log(&format!("  Source: {}", source_path.display()));

        match crate::core::restore_commands_from_file(source_path) {
            Ok(count) => {
                print_and_log(&format!("  ✓ Restored {} commands", count));
            }
            Err(e) => {
                print_and_log(&format!("  ✗ Failed to restore: {}", e));
                print_and_log(&format!("  Backup preserved at: {}", backup_dir.display()));
                std::process::exit(1);
            }
        }
    } else {
        print_and_log("");
        print_and_log("📂 Step 6/8: No historical commands specified (starting fresh)");
    }

    // Step 7: Final rescan (merges commands.txt and does cleanup scan)
    print_and_log("");
    print_and_log("🔨 Step 7/8: Merging and rebuilding with final filesystem scan...");
    run_rescan_command();

    // Step 8: Start all servers via supervisor
    print_and_log("");
    print_and_log("🚀 Step 8/8: Starting all servers...");
    if let Err(e) = crate::systems::supervisor_command("start") {
        print_and_log(&format!("  ✗ Failed to start servers: {}", e));
        print_and_log("  You may need to restart manually with: ha --supervisor start");
    } else {
        print_and_log("  ✓ All servers started");
    }

    // Summary
    print_and_log("");
    print_and_log("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    print_and_log("✅ HISTORY REBUILD COMPLETE");
    print_and_log("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    print_and_log("");
    if let Some(ref path) = use_commands_path {
        print_and_log(&format!("  Restored from: {}", path.display()));
    }
    print_and_log(&format!("  Backup saved: {}", backup_dir.display()));
    print_and_log("");
}

/// Run comprehensive system diagnostics (--diagnose).
pub fn run_diagnose() {
    print("HookAnchor System Diagnostics");
    print("==============================");
    print("");

    let mut all_ok = true;

    // 1. Test Accessibility Permission (native)
    print("1. Accessibility Permission (native CGEvent)");
    print("   Required for: sending keystrokes, 1Password integration");
    let ax_ok = crate::utils::test_accessibility_permission();
    if ax_ok {
        print("   ✅ GRANTED - Can send keystrokes via CGEvent");
    } else {
        print("   ❌ DENIED - Cannot send keystrokes");
        print("   → Add binaries to System Settings → Privacy & Security → Accessibility");
        all_ok = false;
    }
    print("");

    // 2. Test Automation Permission (osascript)
    print("2. Automation Permission (System Events)");
    print("   Required for: controlling other applications");
    let permissions = crate::utils::get_missing_permissions();
    let automation_missing = permissions.iter().any(|p| p.name == "Automation (System Events)");
    if !automation_missing {
        print("   ✅ GRANTED - Can control System Events");
    } else {
        print("   ❌ DENIED - Cannot control System Events");
        print("   → Add binaries to System Settings → Privacy & Security → Automation");
        all_ok = false;
    }
    print("");

    // 3. Check binary signatures
    print("3. Code Signature Status");
    let binary_dir = crate::utils::get_binary_dir();
    let binaries = [("popup", "popup"), ("ha", "HookAnchorCommand"), ("HookAnchorDialog", "HookAnchorDialog")];

    for (display_name, binary) in &binaries {
        let binary_path = binary_dir.join(binary);
        if binary_path.exists() {
            let output = std::process::Command::new("codesign")
                .args(["-dv", "--verbose=2"])
                .arg(&binary_path)
                .output();

            match output {
                Ok(result) => {
                    let stderr = String::from_utf8_lossy(&result.stderr);
                    if stderr.contains("Authority=Developer ID Application") {
                        print(&format!("   ✅ {} - Developer ID signed", display_name));
                    } else if stderr.contains("Authority=Apple Development") {
                        print(&format!("   ⚠️  {} - Development signed (not notarized)", display_name));
                    } else if stderr.contains("adhoc") || stderr.contains("linker-signed") {
                        print(&format!("   ⚠️  {} - Ad-hoc signed (local dev)", display_name));
                    } else if stderr.contains("code object is not signed") {
                        print(&format!("   ❌ {} - Unsigned", display_name));
                    } else {
                        print(&format!("   ?  {} - Unknown signature status", display_name));
                    }
                }
                Err(_) => {
                    print(&format!("   ?  {} - Could not check signature", display_name));
                }
            }
        } else {
            print(&format!("   ❌ {} - Binary not found at {}", display_name, binary_path.display()));
        }
    }
    print("");

    // 4. Check server status
    print("4. Command Server Status");

    // Check if popup_server is running
    let ps_output = std::process::Command::new("pgrep")
        .args(["-lf", "popup_server"])
        .output();

    match ps_output {
        Ok(result) => {
            let stdout = String::from_utf8_lossy(&result.stdout);
            if stdout.trim().is_empty() {
                print("   ⚠️  No server process found");
                print("   → Run: ha --restart");
            } else {
                print("   ✅ Server process running");
                for line in stdout.lines().take(3) {
                    let short = line.split('/').last().unwrap_or(line);
                    print(&format!("      {}", short));
                }
            }
        }
        Err(_) => {
            print("   ?  Could not check server status");
        }
    }
    print("");

    // 5. Check config files
    print("5. Configuration Files");
    let config_dir = dirs::home_dir()
        .map(|p| p.join(".config/hookanchor"))
        .unwrap_or_else(|| std::path::PathBuf::from("/Users/default/.config/hookanchor"));

    let config_files = ["config.yaml", "config.js", "commands.txt"];
    for file in &config_files {
        let path = config_dir.join(file);
        if path.exists() {
            print(&format!("   ✅ {} exists", file));
        } else {
            print(&format!("   ❌ {} missing", file));
            all_ok = false;
        }
    }
    print("");

    // Summary
    print("==============================");
    if all_ok {
        print("✅ All diagnostics passed!");
    } else {
        print("⚠️  Some issues detected - see above for details");
    }
}
