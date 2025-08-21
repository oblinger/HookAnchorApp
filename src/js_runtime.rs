//! JavaScript Runtime with Rich Built-ins
//!
//! Provides a sandboxed JavaScript environment with carefully selected
//! built-in functions for business logic. No external JS interpreters needed.
//!
//! # Available JavaScript Built-in Functions
//!
//! ## Logging Functions
//! - `log(message)` - General logging output
//! - `debug(message)` - Debug logging output  
//! - `error(message)` - Error logging output
//!
//! ## File Operations
//! - `readFile(path)` - Read file contents as string
//! - `writeFile(path, content)` - Write content to file
//! - `fileExists(path)` - Check if file exists (returns boolean)
//! - `isDirectory(path)` - Check if path is directory (returns boolean)
//! - `listFiles(directory, pattern)` - List files in directory with optional pattern
//!
//! ## Path Utilities
//! - `joinPath(part1, part2)` - Join path components
//! - `dirname(path)` - Get directory name from path
//! - `basename(path)` - Get base filename from path
//! - `expandHome(path)` - Expand ~ in paths to home directory
//! - `getExtension(path)` - Get file extension from path
//!
//! ## Text Processing
//! - `testRegex(text, pattern)` - Test text against regex pattern (returns boolean)
//!
//! ## Data Parsing
//! - `parseYaml(text)` - Parse YAML text to JSON string
//!
//! ## Configuration Access
//! - `getObsidianApp()` - Get configured Obsidian application name
//! - `getObsidianVault()` - Get configured Obsidian vault name
//! - `getObsidianVaultPath()` - Get configured Obsidian vault path
//!
//! ## Core System Primitives
//! - `launch_app(app_name, arg)` - Launch macOS application with optional argument
//! - `open_folder(path)` - Open folder in Finder (or configured folder app)
//! - `open_url(url, browser)` - Open URL in browser (optional browser name)
//! - `shell(command)` - Execute shell command in background (non-blocking)
//! - `shell_sync(command)` - Execute shell command and wait for completion (blocking)
//! - `shellWithExitCode(command)` - Execute shell command and return {stdout, stderr, exitCode}
//! - `commandExists(command)` - Check if command is available in PATH (returns boolean)
//! - `change_directory(path)` - Change working directory
//! - `launch(command_name)` - Recursively call another launcher command
//! - `activateApp(app_name)` - Bring application to foreground
//! - `runAppleScript(script)` - Execute AppleScript and return result
//! - [REMOVED] spawnDetached - use shell("command &") for background processes
//! - `appIsRunning(app_name)` - Check if application is currently running (returns boolean)
//!
//! # Usage in Config Files
//!
//! These functions can be used in:
//! - `js_functions` section of config.yaml
//! - Embedded JavaScript business logic scripts
//! - User-customizable action definitions
//! - Per-directory activation scripts

use rquickjs::{Context, Runtime, Function, Ctx};
use std::path::{Path, PathBuf};
use std::fs;
use regex::Regex;
use std::os::unix::process::ExitStatusExt;
use crate::Config;
use crate::utils::expand_tilde;

/// Creates a JavaScript runtime with all business logic built-ins configured
pub fn create_business_logic_runtime() -> Result<Context, Box<dyn std::error::Error>> {
    let config = crate::core::sys_data::get_config();
    create_business_logic_runtime_with_config(&config)
}

/// Creates a JavaScript runtime with built-ins and user-defined functions from config
pub fn create_business_logic_runtime_with_config(config: &Config) -> Result<Context, Box<dyn std::error::Error>> {
    let rt = Runtime::new()?;
    let ctx = Context::full(&rt)?;
    
    ctx.with(|ctx| -> Result<(), Box<dyn std::error::Error>> {
        setup_logging(&ctx)?;
        setup_file_operations(&ctx)?;
        setup_path_utilities(&ctx)?;
        setup_text_processing(&ctx)?;
        setup_data_parsing(&ctx)?;
        setup_launcher_builtins(&ctx)?;
        setup_config_access(&ctx, config)?;
        setup_user_functions(&ctx, config)?;
        Ok(())
    })?;
    
    Ok(ctx)
}

/// Setup all built-in functions in a JavaScript context (for use by other modules)
pub fn setup_all_builtins(ctx: &rquickjs::Ctx<'_>) -> Result<(), Box<dyn std::error::Error>> {
    setup_logging(ctx)?;
    setup_file_operations(ctx)?;
    setup_path_utilities(ctx)?;
    setup_text_processing(ctx)?;
    setup_data_parsing(ctx)?;
    setup_launcher_builtins(ctx)?;
    Ok(())
}

/// Execute JavaScript code in a configured runtime
pub fn execute_business_logic(script: &str) -> Result<String, Box<dyn std::error::Error>> {
    let ctx = create_business_logic_runtime()?;
    
    ctx.with(|ctx| {
        // Execute the script and capture result
        match ctx.eval::<rquickjs::Value, _>(script) {
            Ok(value) => {
                // Convert result to string for returning
                if let Some(s) = value.as_string() {
                    match s.to_string() {
                        Ok(str) => Ok(str),
                        Err(_) => Ok("undefined".to_string()),
                    }
                } else if value.is_number() {
                    Ok(value.as_number().unwrap().to_string())
                } else {
                    Ok("undefined".to_string())
                }
            }
            Err(e) => Err(format!("JavaScript execution error: {}", e).into()),
        }
    })
}

// =============================================================================
// Logging Functions
// =============================================================================

fn setup_logging(ctx: &Ctx<'_>) -> Result<(), Box<dyn std::error::Error>> {
    // log(message) - General logging to file (always logs, not just in verbose mode)
    ctx.globals().set("log", Function::new(ctx.clone(), |msg: String| {
        // Use regular log for JavaScript - these are important user-defined logs
        crate::utils::log(&msg);
    })?)?;
    
    // debug(message) - Debug logging (only in verbose mode)
    ctx.globals().set("debug", Function::new(ctx.clone(), |msg: String| {
        crate::utils::detailed_log("JS", &format!("[DEBUG] {}", msg));
    })?)?;
    
    // error(message) - Error logging
    ctx.globals().set("error", Function::new(ctx.clone(), |msg: String| {
        crate::utils::log_error(&format!("[JS] {}", msg));
    })?)?;
    
    Ok(())
}

// =============================================================================
// File Operations
// =============================================================================

fn setup_file_operations(ctx: &Ctx<'_>) -> Result<(), Box<dyn std::error::Error>> {
    // readFile(path) -> string
    let read_file_fn = Function::new(ctx.clone(), js_read_file)?;
    ctx.globals().set("readFile", read_file_fn)?;
    
    // writeFile(path, content) -> void
    let write_file_fn = Function::new(ctx.clone(), js_write_file)?;
    ctx.globals().set("writeFile", write_file_fn)?;
    
    // fileExists(path) -> boolean
    ctx.globals().set("fileExists", Function::new(ctx.clone(), |path: String| {
        let expanded = expand_tilde(&path);
        Path::new(&expanded).exists()
    })?)?;
    
    // file_exists(path) -> boolean (alias for fileExists)
    ctx.globals().set("file_exists", Function::new(ctx.clone(), |path: String| {
        let expanded = expand_tilde(&path);
        Path::new(&expanded).exists()
    })?)?;
    
    // isDirectory(path) -> boolean
    ctx.globals().set("isDirectory", Function::new(ctx.clone(), |path: String| {
        let expanded = expand_tilde(&path);
        Path::new(&expanded).is_dir()
    })?)?;
    
    // listFiles(directory, pattern) -> array
    ctx.globals().set("listFiles", Function::new(ctx.clone(), |dir: String, pattern: String| {
        let expanded = expand_tilde(&dir);
        let mut files = Vec::new();
        
        if pattern == "recursive" {
            // Recursive directory walk
            walk_directory(&expanded, &mut files);
        } else {
            // Single directory with optional pattern
            if let Ok(entries) = fs::read_dir(&expanded) {
                for entry in entries.flatten() {
                    if let Some(path) = entry.path().to_str() {
                        if pattern.is_empty() || path.contains(&pattern) {
                            files.push(path.to_string());
                        }
                    }
                }
            }
        }
        
        files
    })?)?;
    
    Ok(())
}

// Separate function to handle errors properly
fn js_read_file(ctx: Ctx<'_>, path: String) -> rquickjs::Result<String> {
    let expanded = expand_tilde(&path);
    match fs::read_to_string(&expanded) {
        Ok(content) => Ok(content),
        Err(e) => Err(ctx.throw(rquickjs::Value::from_exception(
            rquickjs::Exception::from_message(ctx.clone(), &format!("Failed to read file: {}", e))?
        ))),
    }
}

fn js_write_file(ctx: Ctx<'_>, path: String, content: String) -> rquickjs::Result<()> {
    let expanded = expand_tilde(&path);
    match fs::write(&expanded, content) {
        Ok(_) => Ok(()),
        Err(e) => Err(ctx.throw(rquickjs::Value::from_exception(
            rquickjs::Exception::from_message(ctx.clone(), &format!("Failed to write file: {}", e))?
        ))),
    }
}

// =============================================================================
// Path Utilities
// =============================================================================

fn setup_path_utilities(ctx: &Ctx<'_>) -> Result<(), Box<dyn std::error::Error>> {
    // joinPath(part1, part2) -> string
    ctx.globals().set("joinPath", Function::new(ctx.clone(), |part1: String, part2: String| {
        let mut path = PathBuf::new();
        path.push(part1);
        path.push(part2);
        path.to_string_lossy().to_string()
    })?)?;
    
    // dirname(path) -> string
    ctx.globals().set("dirname", Function::new(ctx.clone(), |path: String| {
        Path::new(&path)
            .parent()
            .and_then(|p| p.to_str())
            .unwrap_or("")
            .to_string()
    })?)?;
    
    // basename(path) -> string
    ctx.globals().set("basename", Function::new(ctx.clone(), |path: String| {
        Path::new(&path)
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("")
            .to_string()
    })?)?;
    
    // expandHome(path) -> string
    ctx.globals().set("expandHome", Function::new(ctx.clone(), |path: String| {
        expand_tilde(&path)
    })?)?;
    
    // getExtension(path) -> string
    ctx.globals().set("getExtension", Function::new(ctx.clone(), |path: String| {
        Path::new(&path)
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("")
            .to_string()
    })?)?;
    
    Ok(())
}

// =============================================================================
// Text Processing
// =============================================================================

fn setup_text_processing(ctx: &Ctx<'_>) -> Result<(), Box<dyn std::error::Error>> {
    // testRegex(text, pattern) -> boolean
    ctx.globals().set("testRegex", Function::new(ctx.clone(), |text: String, pattern: String| {
        match Regex::new(&pattern) {
            Ok(re) => re.is_match(&text),
            Err(_) => false,
        }
    })?)?;
    
    Ok(())
}

// =============================================================================
// Data Parsing
// =============================================================================

fn setup_data_parsing(ctx: &Ctx<'_>) -> Result<(), Box<dyn std::error::Error>> {
    // parseYaml(text) -> object (as JSON string)
    let parse_yaml_fn = Function::new(ctx.clone(), js_parse_yaml)?;
    ctx.globals().set("parseYaml", parse_yaml_fn)?;
    
    Ok(())
}

// =============================================================================
// Launcher Built-ins
// =============================================================================

fn setup_launcher_builtins(ctx: &Ctx<'_>) -> Result<(), Box<dyn std::error::Error>> {
    // launch_app(app_name, arg) -> launches macOS application
    ctx.globals().set("launch_app", Function::new(ctx.clone(), |app: String, arg: Option<String>| {
        let mut cmd = std::process::Command::new("open");
        cmd.arg("-a").arg(&app);
        let arg_display = arg.clone().unwrap_or_default();
        if let Some(ref arg_val) = arg {
            if !arg_val.is_empty() {
                cmd.arg(arg_val);
            }
        }
        match cmd.output() {
            Ok(_) => format!("Launched {} {}", app, arg_display),
            Err(e) => format!("Failed to launch {}: {}", app, e),
        }
    })?)?;
    
    // open_folder(path) -> opens folder in Finder
    ctx.globals().set("open_folder", Function::new(ctx.clone(), |path: String| {
        let expanded = expand_tilde(&path);
        match std::process::Command::new("open").arg(&expanded).output() {
            Ok(_) => format!("Opened folder: {}", expanded),
            Err(e) => format!("Failed to open folder {}: {}", expanded, e),
        }
    })?)?;
    
    // open_url(url, browser) -> opens URL in specific browser
    ctx.globals().set("open_url", Function::new(ctx.clone(), |url: String, browser: Option<String>| {
        let mut cmd = std::process::Command::new("open");
        let browser_display = browser.clone().unwrap_or("default browser".to_string());
        if let Some(ref browser_name) = browser {
            cmd.arg("-a").arg(browser_name);
        }
        cmd.arg(&url);
        match cmd.output() {
            Ok(_) => format!("Opened URL: {} in {}", url, browser_display),
            Err(e) => format!("Failed to open URL {}: {}", url, e),
        }
    })?)?;
    
    // ============================================================================
    // SHELL EXECUTION FUNCTIONS
    // ============================================================================
    // All shell execution functions are consolidated here for easier maintenance
    // We're keeping the old functions commented for reference while refactoring
    
    // error(message) -> displays an error to the user
    ctx.globals().set("error", Function::new(ctx.clone(), |message: String| {
        // Queue the error for display in the UI
        crate::error_display::queue_user_error(&message);
        crate::utils::log(&format!("JS_ERROR: {}", message));
        // Return empty string since this is a void function
        String::new()
    })?)?;
    
    // shell(command) -> executes shell command without waiting (detached)
    ctx.globals().set("shell", Function::new(ctx.clone(), |command: String| {
        // Execute directly since we're already on the server
        match crate::utils::shell_simple(&command, false) {
            Ok(_) => {
                crate::utils::verbose_log("JS_SHELL", &format!("Command started: {}", command));
                format!("Command started: {}", command)
            },
            Err(e) => {
                let error_msg = format!("Failed to start command '{}': {}", command, e);
                crate::utils::verbose_log("JS_SHELL", &error_msg);
                error_msg
            }
        }
    })?)?;
    
    // shell_sync(command) -> executes shell command and waits for completion
    ctx.globals().set("shell_sync", Function::new(ctx.clone(), |command: String| {
        // Execute directly since we're already on the server
        match crate::utils::shell_simple(&command, true) {
            Ok(output) => {
                let stdout = String::from_utf8_lossy(&output.stdout);
                let stderr = String::from_utf8_lossy(&output.stderr);
                
                crate::utils::verbose_log("JS_SHELL_SYNC", &format!("Command: {}", command));
                if !stdout.is_empty() {
                    crate::utils::verbose_log("JS_SHELL_SYNC", &format!("STDOUT: {}", stdout.trim()));
                }
                if !stderr.is_empty() {
                    crate::utils::verbose_log("JS_SHELL_SYNC", &format!("STDERR: {}", stderr.trim()));
                }
                
                if !stderr.is_empty() {
                    format!("Command executed with stderr: {}", stderr)
                } else {
                    format!("Command executed: {}", stdout.trim())
                }
            },
            Err(e) => {
                let error_msg = format!("Command failed '{}': {}", command, e);
                crate::utils::verbose_log("JS_SHELL_SYNC", &error_msg);
                error_msg
            }
        }
    })?)?;
    
    // shellWithExitCode(command) -> executes shell command and returns detailed result
    ctx.globals().set("shellWithExitCode", Function::new(ctx.clone(), |command: String| {
        // Execute directly since we're already on the server
        use std::process::Command;
        
        let output = Command::new("sh")
            .arg("-c")
            .arg(&command)
            .output()
            .unwrap_or_else(|e| {
                // Return a fake output with error
                std::process::Output {
                    status: std::process::ExitStatus::from_raw(1),
                    stdout: Vec::new(),
                    stderr: format!("Failed to execute: {}", e).into_bytes(),
                }
            });
        
        let exit_code = output.status.code().unwrap_or(-1);
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        
        // Properly escape JSON strings
        let escaped_stdout = stdout.replace('\\', r#"\\"#).replace('"', r#"\""#).replace('\n', r#"\n"#).replace('\r', r#"\r"#).replace('\t', r#"\t"#);
        let escaped_stderr = stderr.replace('\\', r#"\\"#).replace('"', r#"\""#).replace('\n', r#"\n"#).replace('\r', r#"\r"#).replace('\t', r#"\t"#);
        
        format!(r#"{{"stdout":"{}","stderr":"{}","exitCode":{}}}"#, 
            escaped_stdout, escaped_stderr, exit_code)
    })?)?;
    
    // OLD FUNCTIONS - COMMENTED OUT FOR REFERENCE DURING REFACTORING
    // These will be replaced by the unified shell API
    /*
    // spawnDetached(command) -> spawns a completely detached process that won't block shutdown
    ctx.globals().set("spawnDetached", Function::new(ctx.clone(), |command: String| {
        use std::process::{Command, Stdio};
        
        // Use nohup to detach the process completely from the parent
        // The & at the end runs it in background, and nohup ensures it survives parent death
        // Redirecting all I/O to /dev/null ensures no blocking on I/O
        let shell_cmd = format!("nohup {} >/dev/null 2>&1 & disown", command);
        
        match Command::new("sh")
            .arg("-c")
            .arg(&shell_cmd)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn() {
                Ok(_) => format!("Spawned detached command: {}", command),
                Err(e) => format!("Failed to spawn detached command '{}': {}", command, e),
            }
    })?)?;
    
    // spawnDetachedWithArgs(command, arg1, arg2, ...) -> spawns detached process with separate arguments
    ctx.globals().set("spawnDetachedWithArgs", Function::new(ctx.clone(), |args: Vec<String>| {
        use std::process::{Command, Stdio};
        
        if args.is_empty() {
            return "Error: No command provided".to_string();
        }
        
        let command = &args[0];
        let cmd_args = &args[1..];
        
        // Build the command string for nohup
        let full_command = if cmd_args.is_empty() {
            command.clone()
        } else {
            format!("{} {}", command, cmd_args.join(" "))
        };
        
        let shell_cmd = format!("nohup {} >/dev/null 2>&1 & disown", full_command);
        
        match Command::new("sh")
            .arg("-c")
            .arg(&shell_cmd)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn() {
                Ok(_) => format!("Spawned detached: {}", full_command),
                Err(e) => format!("Failed to spawn detached '{}': {}", full_command, e),
            }
    })?)?;
    */
    
    // REMOVED: spawnDetached - not needed, can use shell() with & for background processes
    // Example: shell("command &") or shell("nohup command >/dev/null 2>&1 & disown")
    
    // ============================================================================
    // END SHELL EXECUTION FUNCTIONS
    // ============================================================================
    
    // change_directory(path) -> changes working directory
    ctx.globals().set("change_directory", Function::new(ctx.clone(), |path: String| {
        let expanded = expand_tilde(&path);
        match std::env::set_current_dir(&expanded) {
            Ok(_) => format!("Changed directory to: {}", expanded),
            Err(e) => format!("Failed to change directory to {}: {}", expanded, e),
        }
    })?)?;
    
    // launch(command_name) -> recursively calls another command
    ctx.globals().set("launch", Function::new(ctx.clone(), |command: String| {
        // Parse command into action and args
        let parts: Vec<&str> = command.splitn(2, ' ').collect();
        let action_name = parts[0];
        let args = parts.get(1).copied().unwrap_or("");
        
        // Look up action in config
        let config = crate::core::sys_data::get_config();
        if let Some(actions) = &config.actions {
            if let Some(action_def) = actions.get(action_name) {
                match crate::core::unified_actions::execute_action(
                    action_def,
                    Some(args),
                    None
                ) {
                    Ok(result) => format!("Successfully launched: {} - {}", command, result),
                    Err(e) => format!("Failed to launch '{}': {}", command, e),
                }
            } else {
                format!("Action '{}' not found in configuration", action_name)
            }
        } else {
            format!("No actions configured")
        }
    })?)?;
    
    // DUPLICATE REMOVED - shellWithExitCode is now in the consolidated section above
    
    // commandExists(command) -> checks if command is available in PATH
    ctx.globals().set("commandExists", Function::new(ctx.clone(), |command: String| {
        // Execute directly since we're already on the server
        use std::process::Command;
        
        let output = Command::new("which")
            .arg(&command)
            .output()
            .unwrap_or_else(|_| {
                std::process::Output {
                    status: std::process::ExitStatus::from_raw(1),
                    stdout: Vec::new(),
                    stderr: Vec::new(),
                }
            });
        
        output.status.success()
    })?)?;
    
    // activateApp(app_name) -> brings application to foreground
    ctx.globals().set("activateApp", Function::new(ctx.clone(), |app_name: String| {
        let script = format!(r#"tell application "{}" to activate"#, app_name);
        match std::process::Command::new("osascript").args(["-e", &script]).output() {
            Ok(_) => format!("Activated {}", app_name),
            Err(e) => format!("Failed to activate {}: {}", app_name, e),
        }
    })?)?;
    
    // runAppleScript(script) -> executes AppleScript and returns result
    ctx.globals().set("runAppleScript", Function::new(ctx.clone(), |script: String| {
        match std::process::Command::new("osascript").args(["-e", &script]).output() {
            Ok(output) => {
                let stdout = String::from_utf8_lossy(&output.stdout);
                if output.status.success() {
                    stdout.trim().to_string()
                } else {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    format!("AppleScript error: {}", stderr.trim())
                }
            },
            Err(e) => format!("Failed to execute AppleScript: {}", e),
        }
    })?)?;
    
    
    // appIsRunning(app_name) -> checks if application is currently running
    ctx.globals().set("appIsRunning", Function::new(ctx.clone(), |app_name: String| {
        let script = format!(r#"tell application "System Events" to exists (processes where name is "{}")"#, app_name);
        match std::process::Command::new("osascript").args(["-e", &script]).output() {
            Ok(output) => {
                let result_str = String::from_utf8_lossy(&output.stdout);
                let result = result_str.trim();
                result == "true"
            },
            Err(_) => false,
        }
    })?)?;
    
    Ok(())
}

/// Setup configuration access functions
pub fn setup_config_access(ctx: &Ctx<'_>, config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    // Get launcher settings with defaults
    let launcher_settings = config.launcher_settings.as_ref()
        .cloned()
        .unwrap_or_default();
    
    // Clone config values for closures
    let obsidian_app_name = launcher_settings.obsidian_app_name
        .unwrap_or_else(|| "Obsidian".to_string());
    let obsidian_vault_name = launcher_settings.obsidian_vault_name
        .unwrap_or_else(|| "kmr".to_string());
    let obsidian_vault_path = launcher_settings.obsidian_vault_path
        .map(|p| crate::utils::expand_tilde(&p))
        .unwrap_or_else(|| {
            let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
            format!("{}/Documents", home)
        });
    
    let get_obsidian_app_fn = Function::new(ctx.clone(), move |_ctx: Ctx<'_>| -> rquickjs::Result<String> {
        Ok(obsidian_app_name.clone())
    })?;
    
    let get_obsidian_vault_fn = Function::new(ctx.clone(), move |_ctx: Ctx<'_>| -> rquickjs::Result<String> {
        Ok(obsidian_vault_name.clone())
    })?;
    
    let get_obsidian_vault_path_fn = Function::new(ctx.clone(), move |_ctx: Ctx<'_>| -> rquickjs::Result<String> {
        Ok(obsidian_vault_path.clone())
    })?;
    
    ctx.globals().set("getObsidianApp", get_obsidian_app_fn)?;
    ctx.globals().set("getObsidianVault", get_obsidian_vault_fn)?;
    ctx.globals().set("getObsidianVaultPath", get_obsidian_vault_path_fn)?;
    
    Ok(())
}

/// Setup user-defined functions from configuration
/// Load JavaScript functions from config.js file
pub fn load_config_js_functions(ctx: &Ctx<'_>) -> Result<(), Box<dyn std::error::Error>> {
    let config_js_path = expand_tilde("~/.config/hookanchor/config.js");
    crate::utils::detailed_log("JS", &format!("Looking for config.js at: {}", config_js_path));
    if Path::new(&config_js_path).exists() {
        match fs::read_to_string(&config_js_path) {
            Ok(config_js_content) => {
                crate::utils::detailed_log("JS", &format!("Loading config.js, {} bytes", config_js_content.len()));
                // Create a wrapper that loads the module and exposes functions globally
                let wrapper = format!(r#"
                    (function() {{
                        // Create a module.exports object
                        const module = {{ exports: {{}} }};
                        
                        // Execute the config.js content
                        {}
                        
                        // Create a context object with all builtins
                        const createContext = function(arg, input, previous, grabbed, date) {{
                            return {{
                                arg: arg || '',
                                input: input || '',
                                previous: previous || {{ name: '', folder: '', patch: '' }},
                                grabbed: grabbed || {{ action: '', arg: '' }},
                                date: date || {{ YYYY: '', MM: '', DD: '' }},
                                builtins: {{
                                    log: log,
                                    debug: debug,
                                    error: error,
                                    readFile: readFile,
                                    writeFile: writeFile,
                                    file_exists: fileExists,
                                    isDirectory: isDirectory,
                                    listFiles: listFiles,
                                    joinPath: joinPath,
                                    dirname: dirname,
                                    basename: basename,
                                    expandHome: expandHome,
                                    getExtension: getExtension,
                                    testRegex: testRegex,
                                    parseYaml: parseYaml,
                                    getObsidianApp: getObsidianApp,
                                    getObsidianVault: getObsidianVault,
                                    getObsidianVaultPath: getObsidianVaultPath,
                                    launch_app: launch_app,
                                    open_folder: open_folder,
                                    open_url: open_url,
                                    shell: shell,
                                    shell_sync: shell_sync,
                                    shellWithExitCode: shellWithExitCode,
                                    commandExists: commandExists,
                                    change_directory: change_directory,
                                    launch: launch,
                                    activateApp: activateApp,
                                    runAppleScript: runAppleScript,
                                    // spawnDetached removed - use shell("command &") instead
                                    appIsRunning: appIsRunning,
                                    encodeURIComponent: encodeURIComponent
                                }}
                            }};
                        }};
                        
                        // Expose each function globally with a wrapper that creates the context
                        let functionCount = 0;
                        for (const [name, func] of Object.entries(module.exports)) {{
                            if (typeof func === 'function') {{
                                globalThis[name] = function(arg, input, previous, grabbed, date) {{
                                    const ctx = createContext(arg, input, previous, grabbed, date);
                                    return func(ctx);
                                }};
                                functionCount++;
                                log('JS: Registered function: ' + name);
                            }}
                        }}
                        log('JS: Total functions registered: ' + functionCount);
                    }})();
                "#, config_js_content);
                
                // Execute the wrapper to define all functions
                match ctx.eval::<(), _>(wrapper.as_str()) {
                    Ok(_) => {
                        crate::utils::detailed_log("JS", "Successfully loaded functions from config.js");
                    }
                    Err(e) => {
                        crate::utils::log_error(&format!("Failed to load functions from config.js: {}", e));
                    }
                }
            }
            Err(e) => {
                crate::utils::detailed_log("JS", &format!("config.js not found or not readable: {}", e));
            }
        }
    }
    
    Ok(())
}

/// Setup user-defined functions from configuration (includes both config.js and YAML)
fn setup_user_functions(ctx: &Ctx<'_>, _config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    // Load functions from config.js only
    load_config_js_functions(ctx)?;
    Ok(())
}

fn js_parse_yaml(ctx: Ctx<'_>, yaml_text: String) -> rquickjs::Result<String> {
    match serde_yaml::from_str::<serde_yaml::Value>(&yaml_text) {
        Ok(value) => {
            // Convert YAML value to JSON string for JS
            match serde_json::to_string(&value) {
                Ok(json) => Ok(json),
                Err(e) => Err(ctx.throw(rquickjs::Value::from_exception(
                    rquickjs::Exception::from_message(ctx.clone(), &format!("Failed to convert YAML to JSON: {}", e))?
                ))),
            }
        }
        Err(e) => Err(ctx.throw(rquickjs::Value::from_exception(
            rquickjs::Exception::from_message(ctx.clone(), &format!("Failed to parse YAML: {}", e))?
        ))),
    }
}

// =============================================================================
// Helper Functions
// =============================================================================

// expand_tilde function moved to utils module

fn walk_directory(dir: &str, files: &mut Vec<String>) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            if let Some(path) = entry.path().to_str() {
                if entry.path().is_dir() {
                    walk_directory(path, files);
                } else {
                    files.push(path.to_string());
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_js_execution() {
        let result = execute_business_logic("2 + 2").unwrap();
        assert_eq!(result, "4");
    }

    #[test]
    fn test_logging_functions() {
        let script = r#"
            log("Hello from JS");
            debug("Debug message");
            error("Error message");
            "done"
        "#;
        let result = execute_business_logic(script).unwrap();
        assert_eq!(result, "done");
    }

    #[test]
    fn test_path_utilities() {
        let script = r#"
            const home = expandHome("~");
            const joined = joinPath(home, "documents");
            const dir = dirname(joined);
            const base = basename(joined);
            base
        "#;
        let result = execute_business_logic(script).unwrap();
        assert_eq!(result, "documents");
    }
}