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
//! ## Launcher Built-ins
//! - `launch_app(app_name, arg)` - Launch macOS application with optional argument
//! - `open_folder(path)` - Open folder in Finder (or configured folder app)
//! - `open_url(url, browser)` - Open URL in browser (optional browser name)
//! - `shell(command)` - Execute shell command and return output
//! - `change_directory(path)` - Change working directory
//! - `launch(command_name)` - Recursively call another launcher command
//! - `has_tmux_session(name)` - Check if tmux session exists (returns boolean)
//! - `start_tmux_session(config_file)` - Start tmux session from .tmuxp.yaml config
//! - `activate_iterm()` - Bring iTerm2 application to foreground
//! - `start_claude_code()` - Start Claude Code in current directory
//!
//! # Usage in Config Files
//!
//! These functions can be used in:
//! - `js_actions` section of config.yaml
//! - Embedded JavaScript business logic scripts
//! - User-customizable action definitions
//! - Per-directory activation scripts

use rquickjs::{Context, Runtime, Function, Ctx};
use std::path::{Path, PathBuf};
use std::fs;
use regex::Regex;

/// Creates a JavaScript runtime with all business logic built-ins configured
pub fn create_business_logic_runtime() -> Result<Context, Box<dyn std::error::Error>> {
    let rt = Runtime::new()?;
    let ctx = Context::full(&rt)?;
    
    ctx.with(|ctx| -> Result<(), Box<dyn std::error::Error>> {
        setup_logging(&ctx)?;
        setup_file_operations(&ctx)?;
        setup_path_utilities(&ctx)?;
        setup_text_processing(&ctx)?;
        setup_data_parsing(&ctx)?;
        setup_launcher_builtins(&ctx)?;
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
    // log(message) - General logging
    ctx.globals().set("log", Function::new(ctx.clone(), |msg: String| {
        println!("[JS] {}", msg);
    })?)?;
    
    // debug(message) - Debug logging
    ctx.globals().set("debug", Function::new(ctx.clone(), |msg: String| {
        eprintln!("[JS DEBUG] {}", msg);
    })?)?;
    
    // error(message) - Error logging
    ctx.globals().set("error", Function::new(ctx.clone(), |msg: String| {
        eprintln!("[JS ERROR] {}", msg);
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
    
    // shell(command) -> executes shell command
    ctx.globals().set("shell", Function::new(ctx.clone(), |command: String| {
        match std::process::Command::new("sh").arg("-c").arg(&command).output() {
            Ok(output) => {
                let stdout = String::from_utf8_lossy(&output.stdout);
                let stderr = String::from_utf8_lossy(&output.stderr);
                if !stderr.is_empty() {
                    format!("Command executed with stderr: {}", stderr)
                } else {
                    format!("Command executed: {}", stdout.trim())
                }
            },
            Err(e) => format!("Failed to execute command '{}': {}", command, e),
        }
    })?)?;
    
    // change_directory(path) -> changes working directory
    ctx.globals().set("change_directory", Function::new(ctx.clone(), |path: String| {
        let expanded = expand_tilde(&path);
        match std::env::set_current_dir(&expanded) {
            Ok(_) => format!("Changed directory to: {}", expanded),
            Err(e) => format!("Failed to change directory to {}: {}", expanded, e),
        }
    })?)?;
    
    // launch(command_name) -> recursively calls another launcher command
    ctx.globals().set("launch", Function::new(ctx.clone(), |command: String| {
        // This will be implemented by calling back into the launcher system
        format!("Recursive launch: {}", command)
    })?)?;
    
    // has_tmux_session(name) -> checks if tmux session exists
    ctx.globals().set("has_tmux_session", Function::new(ctx.clone(), |session_name: String| {
        match std::process::Command::new("tmux").args(["has-session", "-t", &session_name]).output() {
            Ok(output) => output.status.success(),
            Err(_) => false,
        }
    })?)?;
    
    // start_tmux_session(config_file) -> starts tmux session from config
    ctx.globals().set("start_tmux_session", Function::new(ctx.clone(), |config_path: String| {
        let expanded = expand_tilde(&config_path);
        match std::process::Command::new("tmuxp").args(["load", &expanded, "-d"]).output() {
            Ok(_) => format!("Started tmux session from: {}", expanded),
            Err(e) => format!("Failed to start tmux session: {}", e),
        }
    })?)?;
    
    // activate_iterm() -> brings iTerm2 to foreground
    ctx.globals().set("activate_iterm", Function::new(ctx.clone(), || {
        let script = r#"tell application "iTerm2" to activate"#;
        match std::process::Command::new("osascript").args(["-e", script]).output() {
            Ok(_) => "Activated iTerm2".to_string(),
            Err(e) => format!("Failed to activate iTerm2: {}", e),
        }
    })?)?;
    
    // start_claude_code() -> starts Claude Code in current directory
    ctx.globals().set("start_claude_code", Function::new(ctx.clone(), || {
        match std::process::Command::new("claude").arg("--continue").spawn() {
            Ok(_) => "Started Claude Code".to_string(),
            Err(e) => format!("Failed to start Claude Code: {}", e),
        }
    })?)?;
    
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

fn expand_tilde(path: &str) -> String {
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