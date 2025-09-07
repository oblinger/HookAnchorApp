//! General utility functions
//!
//! This module contains general-purpose utility functions that don't fit
//! into more specific categories like logging, files, or subprocess management.

use std::process::Command;
use std::path::PathBuf;
use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

/// Cached login shell environment variables
static LOGIN_ENV_CACHE: OnceLock<Mutex<Option<HashMap<String, String>>>> = OnceLock::new();

/// Global static to store the path of the currently running binary
/// This ensures all parts of the application use the same binary path for spawning processes
static BINARY_PATH: OnceLock<PathBuf> = OnceLock::new();

// ============================================================================
// BINARY PATH MANAGEMENT
// ============================================================================

/// Initialize the global binary path - should be called once at application startup
pub fn init_binary_path() {
    if let Ok(exe_path) = std::env::current_exe() {
        let _ = BINARY_PATH.set(exe_path);
    }
}

/// Get the path of the currently running binary
pub fn get_binary_path() -> Option<&'static PathBuf> {
    BINARY_PATH.get()
}

// ============================================================================
// PATH UTILITIES
// ============================================================================

/// Expands ~ in paths to the home directory
/// 
/// This is a common operation needed across multiple modules for handling
/// user-provided paths that may use the ~ shorthand.
pub fn expand_tilde(path: &str) -> String {
    if path.starts_with('~') {
        if let Ok(home) = std::env::var("HOME") {
            path.replacen('~', &home, 1)
        } else {
            path.to_string()
        }
    } else {
        path.to_string()
    }
}

// ============================================================================
// APP LAUNCHING UTILITIES
// ============================================================================

/// Launch an app with optional argument using macOS `open` command
/// 
/// Consolidates the common pattern of launching apps with the `-a` flag
/// Uses non-blocking spawn to prevent UI lockups
pub fn launch_app_with_arg(app: &str, arg: Option<&str>) -> Result<std::process::Output, std::io::Error> {
    
    let mut cmd = Command::new("open");
    cmd.arg("-a").arg(app);
    if let Some(arg_val) = arg {
        if !arg_val.is_empty() {
            cmd.arg(arg_val);
        }
    }
    
    // Log the exact command being executed
    let command_str = format!("open -a \"{}\" {}", app, arg.unwrap_or(""));
    crate::utils::detailed_log("LAUNCH_APP", &format!("LAUNCH_APP: Executing command: {}", command_str));
    
    // Use spawn + detach for non-blocking execution to prevent UI lockups
    match cmd.spawn() {
        Ok(child) => {
            // Register the process for monitoring
            let _process_id = super::subprocess::register_process(child, command_str.clone());
            crate::utils::detailed_log("LAUNCH_APP", &format!("LAUNCH_APP: Process spawned successfully for: {}", app));
            
            // For non-blocking execution, we don't wait for the result
            // The application will open independently without blocking the UI
            Err(std::io::Error::new(
                std::io::ErrorKind::Other, 
                "NON_BLOCKING_SUCCESS: Process spawned successfully"
            ))
        },
        Err(e) => {
            crate::utils::log_error(&format!("LAUNCH_APP: Failed to spawn process for '{}': {}", app, e));
            Err(e)
        }
    }
}

/// Open a URL using macOS `open` command
/// 
/// Consolidates the common pattern of opening URLs  
/// Uses non-blocking spawn to prevent UI lockups
pub fn open_url(url: &str) -> Result<std::process::Output, std::io::Error> {
    
    let child = Command::new("open").arg(url).spawn()?;
    
    // Register the process for monitoring
    let command_str = format!("open {}", url);
    let _process_id = super::subprocess::register_process(child, command_str);
    
    
    // For non-blocking execution, we don't wait for the result
    Err(std::io::Error::new(
        std::io::ErrorKind::Other, 
        "NON_BLOCKING_SUCCESS: URL process spawned successfully"
    ))
}

/// Open a folder using macOS `open` command
/// 
/// Consolidates the common pattern of opening folders in Finder
/// Uses non-blocking spawn to prevent UI lockups
pub fn open_folder(path: &str) -> Result<std::process::Output, std::io::Error> {
    let expanded_path = expand_tilde(path);
    
    let child = Command::new("open").arg(&expanded_path).spawn()?;
    
    // Register the process for monitoring
    let command_str = format!("open {}", expanded_path);
    let _process_id = super::subprocess::register_process(child, command_str);
    
    
    // For non-blocking execution, we don't wait for the result
    Err(std::io::Error::new(
        std::io::ErrorKind::Other, 
        "NON_BLOCKING_SUCCESS: Folder process spawned successfully"
    ))
}

/// Open a file/URL with a specific app using macOS `open` command
/// 
/// Consolidates the open-with pattern used throughout the codebase
/// Uses non-blocking spawn to prevent UI lockups
pub fn open_with_app(app: &str, target: &str) -> Result<std::process::Output, std::io::Error> {
    super::logging::log(&format!("OPEN_WITH_APP: app='{}', target='{}'", app, target));
    
    // Check if this is a browser and we're opening a URL
    let is_browser = matches!(app.to_lowercase().as_str(), 
        "google chrome" | "chrome" | "safari" | "firefox" | "brave" | "brave browser" | "microsoft edge" | "edge");
    let is_url = target.starts_with("http://") || target.starts_with("https://");
    
    if is_browser && is_url {
        // Use AppleScript for browsers to ensure they come to foreground
        super::logging::log(&format!("OPEN_WITH_APP: Using AppleScript for browser: {}", app));
        
        let applescript = format!(
            "tell application \"{}\" to activate\ntell application \"{}\" to open location \"{}\"",
            app, app, target
        );
        
        let mut cmd = Command::new("osascript");
        cmd.arg("-e").arg(&applescript);
        
        // Log the command
        super::logging::log(&format!("OPEN_WITH_APP: Executing AppleScript for browser activation"));
        
        // Try to spawn the command
        let child = match cmd.spawn() {
            Ok(child) => {
                super::logging::log(&format!("OPEN_WITH_APP: Successfully spawned AppleScript process with PID: {:?}", child.id()));
                child
            },
            Err(e) => {
                super::logging::log_error(&format!("OPEN_WITH_APP: Failed to spawn AppleScript command: {}", e));
                // Fall back to regular open command
                super::logging::log(&format!("OPEN_WITH_APP: Falling back to regular open command"));
                let mut fallback_cmd = Command::new("open");
                fallback_cmd.arg("-F").arg("-a").arg(app).arg(target);
                match fallback_cmd.spawn() {
                    Ok(child) => child,
                    Err(e) => return Err(e),
                }
            }
        };
        
        // Register the process for monitoring
        let command_str = format!("osascript -e 'activate {}' -e 'open location {}'", app, target);
        let _process_id = super::subprocess::register_process(child, command_str);
        super::logging::log(&format!("OPEN_WITH_APP: Process registered for monitoring"));
        
    } else {
        // For non-browsers or non-URLs, use the regular open command
        super::logging::log(&format!("OPEN_WITH_APP: Using regular open command"));
        
        let mut cmd = Command::new("open");
        if app.is_empty() {
            cmd.arg(target);
        } else {
            cmd.arg("-F"); // Bring app to foreground
            cmd.arg("-a").arg(app).arg(target);
        }
        
        // Log the full command that will be executed
        let command_str = if app.is_empty() {
            format!("open {}", target)
        } else {
            format!("open -F -a \"{}\" \"{}\"", app, target)
        };
        super::logging::log(&format!("OPEN_WITH_APP: Executing command: {}", command_str));
        
        // Try to spawn the command
        let child = match cmd.spawn() {
            Ok(child) => {
                super::logging::log(&format!("OPEN_WITH_APP: Successfully spawned process with PID: {:?}", child.id()));
                child
            },
            Err(e) => {
                super::logging::log_error(&format!("OPEN_WITH_APP: Failed to spawn open command: {}", e));
                return Err(e);
            }
        };
        
        // Register the process for monitoring
        let _process_id = super::subprocess::register_process(child, command_str);
        super::logging::log(&format!("OPEN_WITH_APP: Process registered for monitoring"));
    }
    
    // Check if the process is actually running
    std::thread::sleep(std::time::Duration::from_millis(100));
    
    // For non-blocking execution, we don't wait for the result
    super::logging::log(&format!("OPEN_WITH_APP: Returning NON_BLOCKING_SUCCESS"));
    Err(std::io::Error::new(
        std::io::ErrorKind::Other, 
        "NON_BLOCKING_SUCCESS: Open with app process spawned successfully"
    ))
}

// ============================================================================
// SHELL EXECUTION UTILITIES
// ============================================================================

/// Capture environment variables from a login shell (cached)
fn get_login_environment() -> Result<HashMap<String, String>, std::io::Error> {
    let cache = LOGIN_ENV_CACHE.get_or_init(|| Mutex::new(None));
    let mut cache_guard = cache.lock().unwrap();
    
    if let Some(ref env) = *cache_guard {
        super::logging::detailed_log("SHELL", "Using cached login environment");
        return Ok(env.clone());
    }
    
    super::logging::detailed_log("SHELL", "Capturing login shell environment for first time");
    
    // Get the user's shell
    let shell = std::env::var("SHELL").unwrap_or_else(|_| "/bin/zsh".to_string());
    
    super::logging::detailed_log("SHELL", &format!("Using shell: {}", shell));
    
    // Run the login shell directly to capture environment
    let output = Command::new(&shell)
        .arg("-l")
        .arg("-c")
        .arg("env")
        .output()?;
    
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Failed to capture login environment: {}", stderr)
        ));
    }
    
    let env_output = String::from_utf8_lossy(&output.stdout);
    let mut env_map = HashMap::new();
    
    for line in env_output.lines() {
        if let Some((key, value)) = line.split_once('=') {
            env_map.insert(key.to_string(), value.to_string());
        }
    }
    
    super::logging::detailed_log("SHELL", &format!("Captured {} environment variables", env_map.len()));
    *cache_guard = Some(env_map.clone());
    
    Ok(env_map)
}

/// Shell execution with simple current environment (original approach)
pub fn shell_simple(command: &str, blocking: bool) -> Result<std::process::Output, std::io::Error> {
    super::logging::detailed_log("SHELL", &format!("SIMPLE: {}", command));
    
    // Detailed logging
    super::logging::detailed_log("SHELL_SIMPLE", "===========================================");
    super::logging::detailed_log("SHELL_SIMPLE", &format!("Command: '{}'", command));
    super::logging::detailed_log("SHELL_SIMPLE", &format!("Blocking: {}", blocking));
    super::logging::detailed_log("SHELL_SIMPLE", &format!("Current dir: {:?}", std::env::current_dir()));
    
    let mut cmd = Command::new("sh");
    cmd.arg("-c").arg(command);
    
    super::logging::detailed_log("SHELL_SIMPLE", "Created Command object with sh -c");
    super::logging::detailed_log("SHELL_SIMPLE", &format!("Full command: sh -c '{}'", command));
    
    if blocking {
        super::logging::detailed_log("SHELL_SIMPLE", "Executing in BLOCKING mode (waiting for completion)");
        let result = cmd.output();
        
        match &result {
            Ok(output) => {
                super::logging::detailed_log("SHELL_SIMPLE", &format!("Command completed with exit status: {:?}", output.status));
                super::logging::detailed_log("SHELL_SIMPLE", &format!("Exit code: {:?}", output.status.code()));
                super::logging::detailed_log("SHELL_SIMPLE", &format!("STDOUT length: {} bytes", output.stdout.len()));
                super::logging::detailed_log("SHELL_SIMPLE", &format!("STDERR length: {} bytes", output.stderr.len()));
                
                if !output.stdout.is_empty() {
                    let stdout_str = String::from_utf8_lossy(&output.stdout);
                    super::logging::detailed_log("SHELL_SIMPLE", &format!("STDOUT content: {}", stdout_str.trim()));
                }
                if !output.stderr.is_empty() {
                    let stderr_str = String::from_utf8_lossy(&output.stderr);
                    super::logging::detailed_log("SHELL_SIMPLE", &format!("STDERR content: {}", stderr_str.trim()));
                }
            },
            Err(e) => {
                super::logging::detailed_log("SHELL_SIMPLE", &format!("Command FAILED with error: {}", e));
                super::logging::detailed_log("SHELL_SIMPLE", &format!("Error kind: {:?}", e.kind()));
            }
        }
        
        super::logging::detailed_log("SHELL_SIMPLE", "===========================================");
        result
    } else {
        super::logging::detailed_log("SHELL_SIMPLE", "Executing in NON-BLOCKING mode (spawn and return)");
        match cmd.spawn() {
            Ok(child) => {
                super::logging::detailed_log("SHELL_SIMPLE", &format!("Process spawned successfully with PID: {:?}", child.id()));
                super::logging::detailed_log("SHELL_SIMPLE", "Returning dummy success output for non-blocking call");
                
                // Return dummy success output for non-blocking
                // Use a command that always succeeds to get a proper ExitStatus
                let dummy_output = Command::new("true").output()?;
                super::logging::detailed_log("SHELL_SIMPLE", "===========================================");
                Ok(std::process::Output {
                    status: dummy_output.status,
                    stdout: Vec::new(),
                    stderr: Vec::new(),
                })
            },
            Err(e) => {
                super::logging::detailed_log("SHELL_SIMPLE", &format!("Failed to spawn process: {}", e));
                super::logging::detailed_log("SHELL_SIMPLE", &format!("Error kind: {:?}", e.kind()));
                super::logging::detailed_log("SHELL_SIMPLE", "===========================================");
                Err(e)
            },
        }
    }
}

/// Shell execution with full login shell environment (su - approach)
pub fn shell_login(command: &str, blocking: bool) -> Result<std::process::Output, std::io::Error> {
    super::logging::detailed_log("SHELL", &format!("LOGIN: {}", command));
    
    let current_user = detect_current_user();
    let escaped_command = command.replace("'", "'\"'\"'");
    let final_command = format!("su - {} -c '{}'", current_user, escaped_command);
    
    let mut cmd = Command::new("sh");
    cmd.arg("-c").arg(&final_command);
    
    if blocking {
        cmd.output()
    } else {
        match cmd.spawn() {
            Ok(_) => {
                let dummy_output = Command::new("true").output()?;
                Ok(std::process::Output {
                    status: dummy_output.status,
                    stdout: Vec::new(),
                    stderr: Vec::new(),
                })
            },
            Err(e) => Err(e),
        }
    }
}

/// Shell execution with hybrid approach: current GUI environment + login shell env vars
pub fn shell_hybrid(command: &str, blocking: bool) -> Result<std::process::Output, std::io::Error> {
    super::logging::detailed_log("SHELL", &format!("HYBRID: {}", command));
    
    // Check if tmux is requested
    if command.contains("tmux") {
        super::logging::detailed_log("SHELL", "tmux command detected, checking availability");
    }
    
    let mut cmd = Command::new("sh");
    cmd.arg("-c").arg(command);
    
    // Try to get login environment, but fall back to simple execution if it fails
    match get_login_environment() {
        Ok(login_env) => {
            super::logging::detailed_log("SHELL", &format!("Applying {} login environment variables", login_env.len()));
            
            // Debug: Log PATH from login environment
            if let Some(path) = login_env.get("PATH") {
                super::logging::detailed_log("SHELL", &format!("Login PATH: {}", path));
            }
            
            // Apply login environment variables to current process environment
            for (key, value) in &login_env {
                cmd.env(key, value);
            }
        },
        Err(e) => {
            super::logging::detailed_log("SHELL", &format!("Failed to get login environment ({}), falling back to simple execution", e));
            // Continue with current environment
        }
    }
    
    if blocking {
        cmd.output()
    } else {
        match cmd.spawn() {
            Ok(_) => {
                let dummy_output = Command::new("true").output()?;
                Ok(std::process::Output {
                    status: dummy_output.status,
                    stdout: Vec::new(),
                    stderr: Vec::new(),
                })
            },
            Err(e) => Err(e),
        }
    }
}

/// Execute a shell command using the user's shell with proper environment
/// 
/// Uses the hybrid approach by default (current GUI environment + login shell env vars)
pub fn execute_shell_command(command: &str) -> Result<std::process::Output, std::io::Error> {
    shell_hybrid(command, true)
}

#[derive(Debug, Clone)]
pub struct ShellOptions {
    pub blocking: bool,      // true = wait for completion, false = spawn and return
    pub login_shell: bool,   // true = use login shell with full environment
    pub inherit_env: bool,   // true = inherit current environment variables
}

impl Default for ShellOptions {
    fn default() -> Self {
        Self {
            blocking: true,
            login_shell: true,
            inherit_env: true,
        }
    }
}

/// Core shell command execution with proper user environment
/// 
/// This function is shared between the utils module and JavaScript runtime
/// to ensure consistent shell execution behavior across the application
pub fn execute_shell_command_with_env(command: &str) -> Result<std::process::Output, std::io::Error> {
    shell_hybrid(command, true)
}

/// Unified shell command execution with configurable options
/// 
/// This replaces all the various shell execution functions with a single, 
/// configurable approach that properly handles user environments
pub fn execute_shell_command_unified(command: &str, blocking: bool, detached: bool) -> Result<std::process::Output, std::io::Error> {
    let options = ShellOptions {
        blocking,
        login_shell: true,
        inherit_env: true,
    };
    execute_shell_with_options(command, options, detached)
}

pub fn execute_shell_with_options(command: &str, options: ShellOptions, detached: bool) -> Result<std::process::Output, std::io::Error> {
    super::logging::detailed_log("SHELL", &format!("Executing command with options: {:?}", options));
    super::logging::detailed_log("SHELL", &format!("Command: {}", command));
    
    // Detect current user - try multiple methods for reliability
    let current_user = detect_current_user();
    super::logging::detailed_log("SHELL", &format!("Detected user: {}", current_user));
    
    // Check if this is a GUI command that needs direct execution
    let is_gui_command = is_gui_command(command);
    super::logging::detailed_log("SHELL", &format!("Is GUI command: {}", is_gui_command));
    
    // Build the command based on options
    let final_command = if options.login_shell && !is_gui_command {
        // Use login shell to get proper environment, but not for GUI commands
        // Escape single quotes in the original command
        let escaped_command = command.replace("'", "'\"'\"'");
        format!("su - {} -c '{}'", current_user, escaped_command)
    } else {
        // Use simple shell execution for GUI commands or when login_shell is false
        command.to_string()
    };
    
    super::logging::detailed_log("SHELL", &format!("Final command: {}", final_command));
    
    // Execute with appropriate method
    if detached {
        execute_detached(&final_command, options)
    } else if options.blocking {
        execute_blocking(&final_command, options)
    } else {
        execute_non_blocking(&final_command, options)
    }
}

/// Detects if a command is a GUI command that needs direct execution (not through su -)
fn is_gui_command(command: &str) -> bool {
    // List of common GUI commands that need direct execution
    let gui_commands = [
        "open",           // macOS open command
        "osascript",      // AppleScript execution
        "automator",      // Automator workflows
        "say",            // Text-to-speech
        "afplay",         // Audio playback
        "qlmanage",       // Quick Look
        "screencapture",  // Screen capture
        "caffeinate",     // Keep system awake
    ];
    
    // Extract the first word (command name) from the full command
    let command_name = command.trim().split_whitespace().next().unwrap_or("");
    
    // Check if it's a known GUI command
    gui_commands.iter().any(|&gui_cmd| command_name == gui_cmd)
}

fn detect_current_user() -> String {
    // Try multiple methods to detect the current user
    
    // Method 1: USER environment variable
    if let Ok(user) = std::env::var("USER") {
        if !user.is_empty() {
            super::logging::detailed_log("SHELL", &format!("Found user via USER env var: {}", user));
            return user;
        }
    }
    
    // Method 2: LOGNAME environment variable  
    if let Ok(user) = std::env::var("LOGNAME") {
        if !user.is_empty() {
            super::logging::detailed_log("SHELL", &format!("Found user via LOGNAME env var: {}", user));
            return user;
        }
    }
    
    // Method 3: whoami command
    if let Ok(output) = Command::new("whoami").output() {
        if output.status.success() {
            let user = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if !user.is_empty() {
                super::logging::detailed_log("SHELL", &format!("Found user via whoami: {}", user));
                return user;
            }
        }
    }
    
    // Method 4: Parse from HOME directory
    if let Ok(home) = std::env::var("HOME") {
        if let Some(user) = home.split('/').last() {
            if !user.is_empty() {
                super::logging::detailed_log("SHELL", &format!("Found user via HOME parsing: {}", user));
                return user.to_string();
            }
        }
    }
    
    // Fallback: assume current user from id command
    if let Ok(output) = Command::new("id").arg("-un").output() {
        if output.status.success() {
            let user = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if !user.is_empty() {
                super::logging::detailed_log("SHELL", &format!("Found user via id -un: {}", user));
                return user;
            }
        }
    }
    
    // Final fallback
    super::logging::detailed_log("SHELL", "Could not detect user, using fallback 'user'");
    "user".to_string()
}

fn execute_blocking(command: &str, options: ShellOptions) -> Result<std::process::Output, std::io::Error> {
    use std::process::Command;
    
    let mut cmd = Command::new("sh");
    cmd.arg("-c").arg(command);
    
    if options.inherit_env {
        // Inherit minimal environment
        if let Ok(home) = std::env::var("HOME") {
            cmd.env("HOME", home);
        }
        if let Ok(path) = std::env::var("PATH") {
            cmd.env("PATH", path);
        }
    }
    
    cmd.output()
}

fn execute_non_blocking(command: &str, options: ShellOptions) -> Result<std::process::Output, std::io::Error> {
    use std::process::{Command, Stdio};
    
    let mut cmd = Command::new("sh");
    cmd.arg("-c")
       .arg(command)
       .stdin(Stdio::null())
       .stdout(Stdio::null())
       .stderr(Stdio::null());
    
    if options.inherit_env {
        if let Ok(home) = std::env::var("HOME") {
            cmd.env("HOME", home);
        }
        if let Ok(path) = std::env::var("PATH") {
            cmd.env("PATH", path);
        }
    }
    
    // For non-blocking, we spawn and return immediately
    match cmd.spawn() {
        Ok(_) => {
            super::logging::detailed_log("SHELL", "Non-blocking command spawned successfully");
            // For non-blocking commands, just run a quick successful command to get a real ExitStatus
            let dummy_output = Command::new("true").output().unwrap_or_else(|_| {
                std::process::Output {
                    status: std::process::ExitStatus::default(),
                    stdout: Vec::new(),
                    stderr: Vec::new(),
                }
            });
            Ok(dummy_output)
        }
        Err(e) => Err(e)
    }
}

fn execute_detached(command: &str, options: ShellOptions) -> Result<std::process::Output, std::io::Error> {
    use std::process::{Command, Stdio};
    
    let mut cmd = Command::new("sh");
    cmd.arg("-c")
       .arg(command)
       .stdin(Stdio::null())
       .stdout(Stdio::null()) 
       .stderr(Stdio::null());
       
    if options.inherit_env {
        if let Ok(home) = std::env::var("HOME") {
            cmd.env("HOME", home);
        }
        if let Ok(path) = std::env::var("PATH") {
            cmd.env("PATH", path);
        }
    }
    
    match cmd.spawn() {
        Ok(_) => {
            super::logging::detailed_log("SHELL", "Detached command spawned successfully");
            // For detached commands, just run a quick successful command to get a real ExitStatus
            let dummy_output = Command::new("true").output().unwrap_or_else(|_| {
                std::process::Output {
                    status: std::process::ExitStatus::default(),
                    stdout: Vec::new(),
                    stderr: Vec::new(),
                }
            });
            Ok(dummy_output)
        }
        Err(e) => Err(e)
    }
}