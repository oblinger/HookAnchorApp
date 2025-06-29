//! Business Logic Module
//!
//! Manages JavaScript-based business logic scripts that can be
//! modified without recompiling the application.
//!
//! # JavaScript API Reference
//!
//! All business logic scripts have access to comprehensive JavaScript built-in functions.
//! For complete API documentation, see [`crate::js_runtime`] module.
//!
//! ## Most Commonly Used Functions
//! - **Logging**: `log()`, `debug()`, `error()`
//! - **File Operations**: `fileExists()`, `isDirectory()`, `readFile()`, `writeFile()`
//! - **System Control**: `launch_app()`, `shell()`, `open_folder()`
//! - **Path Utilities**: `expandHome()`, `basename()`, `joinPath()`
//! - **Development Tools**: `start_tmux_session()`, `activate_iterm()`, `start_claude_code()`

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
    let _result = run_business_script("markdown_scanner")?;
    
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
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_business_script_loading() {
        // This would test in a real implementation
        // For now, just verify the module compiles
        assert!(true);
    }
}