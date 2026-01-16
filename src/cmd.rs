use crate::utils::logging::print;
use crate::cli::help::{print_help, print_help_vars, print_help_config, print_help_fns};
use crate::cli::url_handlers::{handle_hook_option, handle_hook_url};
use crate::cli::search::{
    run_match_command, run_single_path_command, run_paths_command,
    run_single_named_folder_command, run_named_folders_command,
};
use crate::cli::execute::{
    run_exec_command, run_execute_top_match, run_test_command, run_action_directly,
};
use crate::cli::server::{
    run_start_server, run_start_server_daemon, run_process_health,
    run_process_status, run_restart_server, run_supervisor_command,
};
use crate::cli::install::{run_install, run_uninstall};
use crate::cli::inference::{run_infer_patches, run_infer_all_patches};
use crate::cli::maintenance::{
    run_test_grabber, run_test_permissions, run_grab_command,
    run_rescan_command, run_execute_launcher_command, run_rebuild_command,
    run_search_command, run_delete_history, run_diagnose,
};
use crate::cli::define::run_define;

/// Main entry point for command-line mode
pub fn run_command_line_mode(args: Vec<String>) {
    // Check for hook:// URL as first argument
    if args.len() >= 2 && args[1].starts_with("hook://") {
        handle_hook_url(&args[1]);
        return;
    }
    
    // Check for help flags with optional subcommands
    if args.len() >= 2 && (args[1] == "-h" || args[1] == "--help") {
        if args.len() >= 3 {
            match args[2].to_lowercase().as_str() {
                "vars" => {
                    print_help_vars();
                    return;
                }
                "config" => {
                    print_help_config();
                    return;
                }
                "fns" => {
                    print_help_fns();
                    return;
                }
                _ => {
                    print(&format!("Unknown help topic: {}", args[2]));
                    print("Available topics: vars, config, fns");
                    std::process::exit(1);
                }
            }
        }
        print_help(&args[0]);
        return;
    }
    
    if args.len() < 2 {
        print_help(&args[0]);
        std::process::exit(1);
    }
    
    match args[1].as_str() {
        "-m" | "--match" => run_match_command(&args),
        "-r" | "--run_fn" => run_exec_command(&args),
        "-x" | "--execute" => run_execute_top_match(&args),
        "-p" => run_single_path_command(&args),
        "-P" => run_paths_command(&args),
        "-f" => run_single_named_folder_command(&args),
        "-F" => run_named_folders_command(&args),
        "-c" | "--command" => run_test_command(&args),  // Renamed: test command with action and arg
        "-a" | "--action" => run_action_directly(&args),  // New: execute action directly
        "--hook" => handle_hook_option(&args),
        "--user-info" => print_user_info(),
        "--test-grabber" => run_test_grabber(),
        "--test-permissions" => run_test_permissions(),
        "--diagnose" => run_diagnose(),
        "--grab" => run_grab_command(&args),
        "--infer" => run_infer_patches(&args),
        "--infer-all" => run_infer_all_patches(&args),
        "--rescan" => run_rescan_command(),
        "--rebuild" => run_rebuild_command(),
        "--start-server" => run_start_server(),
        "--start-server-daemon" => run_start_server_daemon(),
        "--restart" => run_restart_server(),
        "--supervisor" => run_supervisor_command(&args),
        "--process-health" => run_process_health(),
        "--process-status" => run_process_status(),
        "--install" => run_install(&args),
        "--uninstall" => run_uninstall(),
        "--execute-launcher-command" => run_execute_launcher_command(&args),
        "--search" => run_search_command(),
        "--delete-history" => run_delete_history(&args),
        "--define" | "-d" => run_define(&args),
        _ => {
            print(&format!("Unknown command: {}", args[1]));
            print("Use -h or --help for usage information");
            std::process::exit(1);
        }
    }
}

// Helper: Resolve alias to target command
fn print_user_info() {
    print("=== User Environment Information ===");
    
    // Environment variables
    print(&format!("USER: {:?}", std::env::var("USER")));
    print(&format!("LOGNAME: {:?}", std::env::var("LOGNAME")));
    print(&format!("HOME: {:?}", std::env::var("HOME")));
    print(&format!("SHELL: {:?}", std::env::var("SHELL")));
    print(&format!("PATH: {:?}", std::env::var("PATH")));
    
    // Try whoami command
    use std::process::Command;
    match Command::new("whoami").output() {
        Ok(output) => {
            let whoami_user = String::from_utf8_lossy(&output.stdout).trim().to_string();
            print(&format!("whoami: '{}'", whoami_user));
        }
        Err(e) => {
            print(&format!("whoami failed: {}", e));
        }
    }
    
    // Try id command for more user info
    match Command::new("id").output() {
        Ok(output) => {
            let id_output = String::from_utf8_lossy(&output.stdout).trim().to_string();
            print(&format!("id: {}", id_output));
        }
        Err(e) => {
            print(&format!("id failed: {}", e));
        }
    }
    
    // Try to get user's shell profile paths
    if let Ok(home) = std::env::var("HOME") {
        print(&format!("Checking shell profiles in {}", home));
        let profiles = vec![".zshrc", ".bash_profile", ".bashrc", ".profile"];
        for profile in profiles {
            let path = format!("{}/{}", home, profile);
            if std::path::Path::new(&path).exists() {
                print(&format!("  {} exists", path));
            } else {
                print(&format!("  {} not found", path));
            }
        }
    }
}


