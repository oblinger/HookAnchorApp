//! Business Logic Module
//!
//! Manages JavaScript-based business logic scripts that can be
//! modified without recompiling the application.
//!
//! # Available JavaScript Built-in Functions
//!
//! All business logic scripts have access to these built-in functions:
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

use crate::js_runtime::execute_business_logic;

/// Executes a business logic script from the embedded scripts directory
pub fn run_business_script(script_name: &str) -> Result<String, Box<dyn std::error::Error>> {
    // For now, use embedded scripts. Later could load from filesystem.
    let script = match script_name {
        "markdown_scanner" => include_str!("business_logic/markdown_scanner.js"),
        "activate" => include_str!("business_logic/activate.js"),
        _ => return Err(format!("Unknown business script: {}", script_name).into()),
    };
    
    execute_business_logic(script)
}

/// Scans for markdown-based commands using JavaScript business logic
pub fn scan_markdown_commands() -> Result<Vec<crate::Command>, Box<dyn std::error::Error>> {
    // For now, return empty vector to get it compiling
    // Full implementation would parse JS results into Command structs
    Ok(Vec::new())
}

/// Updates the command list with markdown-based commands
pub fn update_commands_from_markdown() -> Result<usize, Box<dyn std::error::Error>> {
    // For demonstration, just run the script and print results
    let result = run_business_script("markdown_scanner")?;
    println!("Markdown scanner result: {}", result);
    
    // In full implementation, would parse results and update commands
    Ok(0)
}

/// Activates an anchor project using JavaScript business logic
pub fn activate_anchor(anchor_path: &str) -> Result<String, Box<dyn std::error::Error>> {
    // Load the activate script
    let script = include_str!("business_logic/activate.js");
    
    // Create JavaScript context with the activate function and call it
    let full_script = format!(
        r#"
        {}
        
        // Call the activate function with the provided path
        const activateFunction = activate;
        activateFunction("{}");
        "success"
        "#,
        script,
        anchor_path.replace("\"", "\\\"") // Escape quotes in path
    );
    
    execute_business_logic(&full_script)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_business_script_loading() {
        // This would test in a real implementation
        // For now, just verify the module compiles
        assert!(true);
    }
}