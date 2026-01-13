//! CLI Module
//!
//! This module contains the command-line interface functionality,
//! organized into focused submodules.

pub mod help;
pub mod url_handlers;
pub mod search;
pub mod execute;
pub mod server;
pub mod install;
pub mod inference;
pub mod maintenance;

// Re-export commonly used items
pub use help::{print_help, print_help_vars, print_help_config, print_help_fns};
pub use url_handlers::{handle_hook_option, handle_hook_url};
pub use search::{
    run_match_command, run_single_path_command, run_paths_command,
    run_single_named_folder_command, run_named_folders_command,
};
pub use execute::{
    run_exec_command, run_execute_top_match, run_test_command, run_action_directly,
};
pub use server::{
    run_start_server, run_start_server_daemon, run_process_health,
    run_process_status, run_restart_server, run_supervisor_command,
};
pub use install::{run_install, run_uninstall};
pub use inference::{run_infer_patches, run_infer_all_patches};
pub use maintenance::{
    run_test_grabber, run_test_permissions, run_grab_command,
    run_rescan_command, run_execute_launcher_command, run_rebuild_command,
    run_search_command, run_delete_history, run_diagnose,
};
