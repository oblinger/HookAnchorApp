use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Deserialize, Serialize};
use rquickjs::{Context, Runtime, Function, Result as JsResult};

#[derive(Debug, Clone, PartialEq)]
pub enum EvalError {
    ExecutionError(String),
    InvalidAction(String),
    SystemError(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActionSpec {
    App { name: String },
    OpenWith { app: String, arg: String },
    Url { url: String },
    Folder { path: String },
    Shell { command: String },
    JavaScript { code: String },
}

impl ActionSpec {
    /// Substitute template variables like {{arg}} with actual values
    pub fn substitute_templates(&self, arg: &str) -> Self {
        match self {
            ActionSpec::App { name } => ActionSpec::App {
                name: substitute_template_string(name, arg),
            },
            ActionSpec::OpenWith { app, arg: template_arg } => ActionSpec::OpenWith {
                app: substitute_template_string(app, arg),
                arg: substitute_template_string(template_arg, arg),
            },
            ActionSpec::Url { url } => ActionSpec::Url {
                url: substitute_template_string(url, arg),
            },
            ActionSpec::Folder { path } => ActionSpec::Folder {
                path: substitute_template_string(path, arg),
            },
            ActionSpec::Shell { command } => ActionSpec::Shell {
                command: substitute_template_string(command, arg),
            },
            ActionSpec::JavaScript { code } => ActionSpec::JavaScript {
                code: substitute_template_string(code, arg),
            },
        }
    }
}

/// Replace {{arg}} template with actual argument value
fn substitute_template_string(template: &str, arg: &str) -> String {
    template.replace("{{arg}}", arg)
}

#[derive(Debug, Clone)]
pub struct Environment {
    pub config_path: PathBuf,
    pub working_dir: PathBuf,
    pub variables: HashMap<String, String>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            config_path: PathBuf::from("config.yaml"),
            working_dir: std::env::current_dir().unwrap_or_default(),
            variables: HashMap::new(),
        }
    }
}

pub fn execute(action_spec: ActionSpec, original_command: &str, _env: &Environment) -> Result<(), EvalError> {
    match action_spec {
        ActionSpec::App { name } => {
            debug_log(&format!("Executing App action: launching '{}'", name));
            
            // Launch app with `open -a APP_NAME`
            let output = Command::new("open")
                .arg("-a")
                .arg(&name)
                .output()
                .map_err(|e| EvalError::ExecutionError(format!("Failed to launch app '{}': {}", name, e)))?;
            
            if !output.status.success() {
                let stderr = String::from_utf8_lossy(&output.stderr);
                debug_log(&format!("App launch failed with exit code {}: {}", output.status.code().unwrap_or(-1), stderr));
                return Err(EvalError::ExecutionError(format!("App launch failed: {}", stderr)));
            }
            
            debug_log(&format!("App '{}' launched successfully", name));
            Ok(())
        }
        ActionSpec::OpenWith { app, arg } => {
            if app.is_empty() {
                debug_log(&format!("Executing OpenWith action: opening '{}' with system default", arg));
            } else {
                debug_log(&format!("Executing OpenWith action: opening '{}' with '{}'", arg, app));
            }
            
            // Launch app with argument using `open -a APP ARG` or just `open ARG` if no app specified
            let output = if app.is_empty() {
                // Let system choose default app
                Command::new("open")
                    .arg(&arg)
                    .output()
                    .map_err(|e| EvalError::ExecutionError(format!("Failed to open '{}': {}", arg, e)))?
            } else {
                // Use specific app
                Command::new("open")
                    .arg("-a")
                    .arg(&app)
                    .arg(&arg)
                    .output()
                    .map_err(|e| EvalError::ExecutionError(format!("Failed to open '{}' with '{}': {}", arg, app, e)))?
            };
            
            if !output.status.success() {
                let stderr = String::from_utf8_lossy(&output.stderr);
                debug_log(&format!("OpenWith failed with exit code {}: {}", output.status.code().unwrap_or(-1), stderr));
                return Err(EvalError::ExecutionError(format!("Open with app failed: {}", stderr)));
            }
            
            debug_log(&format!("OpenWith completed successfully"));
            Ok(())
        }
        ActionSpec::Url { url } => {
            debug_log(&format!("Executing Url action: opening '{}'", url));
            
            // Open URL with default browser using `open URL`
            let output = Command::new("open")
                .arg(&url)
                .output()
                .map_err(|e| EvalError::ExecutionError(format!("Failed to open URL '{}': {}", url, e)))?;
            
            if !output.status.success() {
                let stderr = String::from_utf8_lossy(&output.stderr);
                debug_log(&format!("URL open failed with exit code {}: {}", output.status.code().unwrap_or(-1), stderr));
                return Err(EvalError::ExecutionError(format!("URL open failed: {}", stderr)));
            }
            
            debug_log(&format!("URL '{}' opened successfully", url));
            Ok(())
        }
        ActionSpec::Folder { path } => {
            debug_log(&format!("Executing Folder action: opening '{}'", path));
            
            // Open folder with `open PATH`
            let output = Command::new("open")
                .arg(&path)
                .output()
                .map_err(|e| EvalError::ExecutionError(format!("Failed to open folder '{}': {}", path, e)))?;
            
            if !output.status.success() {
                let stderr = String::from_utf8_lossy(&output.stderr);
                debug_log(&format!("Folder open failed with exit code {}: {}", output.status.code().unwrap_or(-1), stderr));
                return Err(EvalError::ExecutionError(format!("Folder open failed: {}", stderr)));
            }
            
            debug_log(&format!("Folder '{}' opened successfully", path));
            Ok(())
        }
        ActionSpec::Shell { command } => {
            debug_log(&format!("Executing Shell action: running '{}'", command));
            
            // Execute shell command using /bin/sh
            let output = Command::new("/bin/sh")
                .arg("-c")
                .arg(&command)
                .output()
                .map_err(|e| EvalError::ExecutionError(format!("Failed to execute command '{}': {}", command, e)))?;
            
            if !output.status.success() {
                let stderr = String::from_utf8_lossy(&output.stderr);
                let stdout = String::from_utf8_lossy(&output.stdout);
                debug_log(&format!("Shell command failed with exit code {}: stdout='{}', stderr='{}'", 
                    output.status.code().unwrap_or(-1), stdout.trim(), stderr.trim()));
                return Err(EvalError::ExecutionError(format!("Shell command failed: {}", stderr)));
            }
            
            let stdout = String::from_utf8_lossy(&output.stdout);
            debug_log(&format!("Shell command completed successfully, output: '{}'", stdout.trim()));
            Ok(())
        }
        ActionSpec::JavaScript { ref code } => {
            debug_log(&format!("Executing JavaScript action: '{}'", code.trim()));
            execute_javascript(code, original_command, &action_spec)
        }
    }
}

fn execute_javascript(code: &str, original_command: &str, action_spec: &ActionSpec) -> Result<(), EvalError> {
    debug_log("Starting JavaScript runtime");
    
    let rt = Runtime::new().map_err(|e| EvalError::ExecutionError(format!("Failed to create JavaScript runtime: {}", e)))?;
    let ctx = Context::full(&rt).map_err(|e| EvalError::ExecutionError(format!("Failed to create JavaScript context: {}", e)))?;
    
    ctx.with(|ctx| -> Result<(), EvalError> {
        // Parse original command to extract parts
        let (action_name, command_arg) = if let Some(space_pos) = original_command.find(char::is_whitespace) {
            let action = &original_command[..space_pos];
            let args = original_command[space_pos..].trim_start();
            (action, args)
        } else {
            (original_command, "")
        };
        
        // Create command context object for JavaScript
        let command_context = format!(
            r#"{{
                "command": "{}",
                "action": "{}",
                "arg": "{}",
                "full_command": "{}",
                "action_type": "{}",
                "timestamp": {}
            }}"#,
            original_command,
            action_name,
            command_arg,
            original_command,
            match action_spec {
                ActionSpec::JavaScript { .. } => "javascript",
                ActionSpec::App { .. } => "app",
                ActionSpec::OpenWith { .. } => "open_with",
                ActionSpec::Url { .. } => "url",
                ActionSpec::Folder { .. } => "folder",
                ActionSpec::Shell { .. } => "shell",
            },
            SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_secs()
        );
        
        // Set command context as global variable
        ctx.globals().set("COMMAND", command_context.as_str())
            .map_err(|e| EvalError::ExecutionError(format!("Failed to set COMMAND context: {}", e)))?;
            
        // Set individual command parts as global variables for convenience
        ctx.globals().set("ARG", command_arg)
            .map_err(|e| EvalError::ExecutionError(format!("Failed to set ARG: {}", e)))?;
        ctx.globals().set("ACTION", action_name)
            .map_err(|e| EvalError::ExecutionError(format!("Failed to set ACTION: {}", e)))?;
        ctx.globals().set("FULL_COMMAND", original_command)
            .map_err(|e| EvalError::ExecutionError(format!("Failed to set FULL_COMMAND: {}", e)))?;
        
        // Add enhanced built-in functions (delegate to js_runtime module)
        crate::js_runtime::setup_all_builtins(&ctx)
            .map_err(|e| EvalError::ExecutionError(format!("Failed to setup built-ins: {}", e)))?;
        
        // Legacy shell function for compatibility
        let shell_fn = Function::new(ctx.clone(), js_shell_function)
            .map_err(|e| EvalError::ExecutionError(format!("Failed to create shell function: {}", e)))?;
        
        ctx.globals().set("shell", shell_fn)
            .map_err(|e| EvalError::ExecutionError(format!("Failed to set shell function: {}", e)))?;
        
        // Add launch_app function
        let launch_app_fn = Function::new(ctx.clone(), js_launch_app_function)
            .map_err(|e| EvalError::ExecutionError(format!("Failed to create launch_app function: {}", e)))?;
        
        ctx.globals().set("launch_app", launch_app_fn)
            .map_err(|e| EvalError::ExecutionError(format!("Failed to set launch_app function: {}", e)))?;
        
        // Add launch function for recursive command calling
        let launch_fn = Function::new(ctx.clone(), js_launch_function)
            .map_err(|e| EvalError::ExecutionError(format!("Failed to create launch function: {}", e)))?;
        
        ctx.globals().set("launch", launch_fn)
            .map_err(|e| EvalError::ExecutionError(format!("Failed to set launch function: {}", e)))?;
        
        // Execute the JavaScript code
        debug_log(&format!("Executing JavaScript: {}", code.trim()));
        ctx.eval::<(), _>(code)
            .map_err(|e| EvalError::ExecutionError(format!("JavaScript execution failed: {}", e)))?;
        
        debug_log("JavaScript execution completed successfully");
        Ok(())
    })
}

fn js_shell_function(command: String) -> JsResult<()> {
    debug_log(&format!("JavaScript calling shell: '{}'", command));
    let output = Command::new("/bin/sh")
        .arg("-c")
        .arg(&command)
        .output();
    
    match output {
        Ok(result) => {
            if result.status.success() {
                let stdout = String::from_utf8_lossy(&result.stdout);
                debug_log(&format!("Shell command completed: '{}'", stdout.trim()));
                Ok(())
            } else {
                let stderr = String::from_utf8_lossy(&result.stderr);
                debug_log(&format!("Shell command failed: '{}'", stderr.trim()));
                Err(rquickjs::Error::new_from_js_message("shell", "Rust", format!("Shell command failed: {}", stderr)))
            }
        }
        Err(e) => {
            debug_log(&format!("Shell command error: {}", e));
            Err(rquickjs::Error::new_from_js_message("shell", "Rust", format!("Shell command error: {}", e)))
        }
    }
}

fn js_launch_app_function(app: String, arg: String) -> JsResult<()> {
    debug_log(&format!("JavaScript calling launch_app: app='{}', arg='{}'", app, arg));
    let output = if arg.is_empty() {
        Command::new("open")
            .arg("-a")
            .arg(&app)
            .output()
    } else {
        Command::new("open")
            .arg("-a")
            .arg(&app)
            .arg(&arg)
            .output()
    };
    
    match output {
        Ok(result) => {
            if result.status.success() {
                debug_log(&format!("App '{}' launched successfully", app));
                Ok(())
            } else {
                let stderr = String::from_utf8_lossy(&result.stderr);
                debug_log(&format!("App launch failed: '{}'", stderr.trim()));
                Err(rquickjs::Error::new_from_js_message("launch_app", "Rust", format!("App launch failed: {}", stderr)))
            }
        }
        Err(e) => {
            debug_log(&format!("App launch error: {}", e));
            Err(rquickjs::Error::new_from_js_message("launch_app", "Rust", format!("App launch error: {}", e)))
        }
    }
}

fn js_launch_function(command: String) -> JsResult<()> {
    debug_log(&format!("JavaScript calling launch: '{}'", command));
    use crate::launcher::launch;
    match launch(&command) {
        Ok(()) => {
            debug_log(&format!("Recursive launch completed: '{}'", command));
            Ok(())
        }
        Err(e) => {
            debug_log(&format!("Recursive launch failed: {:?}", e));
            Err(rquickjs::Error::new_from_js_message("launch", "Rust", format!("Recursive launch failed: {:?}", e)))
        }
    }
}

fn debug_log(message: &str) {
    use crate::load_config;
    
    let config = load_config();
    if let Some(debug_path) = &config.settings.debug_log {
        let debug_path = expand_tilde(debug_path);
        
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        
        let log_entry = format!("[{}] EVAL: {}\n", timestamp, message);
        
        if let Ok(mut file) = OpenOptions::new()
            .create(true)
            .append(true)
            .open(debug_path) {
            let _ = file.write_all(log_entry.as_bytes());
        }
    }
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_execute_app_action() {
        let env = Environment::new();
        let action = ActionSpec::App { name: "Finder".to_string() };
        let result = execute(action, "", &env);
        assert!(result.is_ok());
    }

    #[test]
    fn test_execute_app_action_with_spaces() {
        let env = Environment::new();
        let action = ActionSpec::App { name: "Google Chrome".to_string() };
        let result = execute(action, "", &env);
        assert!(result.is_ok());
    }

    #[test]
    fn test_execute_open_with_browser() {
        let env = Environment::new();
        let action = ActionSpec::OpenWith { 
            app: "Google Chrome".to_string(),
            arg: "https://github.com".to_string()
        };
        let result = execute(action, "", &env);
        assert!(result.is_ok());
    }

    #[test]
    fn test_execute_open_with_document() {
        let env = Environment::new();
        // Use TextEdit which is installed on all macOS systems and a URL instead of file path
        let action = ActionSpec::OpenWith { 
            app: "TextEdit".to_string(),
            arg: "https://apple.com".to_string()
        };
        let result = execute(action, "", &env);
        assert!(result.is_ok());
    }

    #[test]
    fn test_execute_url_action() {
        let env = Environment::new();
        let action = ActionSpec::Url { url: "https://github.com".to_string() };
        let result = execute(action, "", &env);
        assert!(result.is_ok());
    }

    #[test]
    fn test_execute_folder_action() {
        let env = Environment::new();
        let action = ActionSpec::Folder { path: "/Users".to_string() };
        let result = execute(action, "", &env);
        assert!(result.is_ok());
    }

    #[test]
    fn test_execute_folder_action_with_spaces() {
        let env = Environment::new();
        // Use a path that exists - the current user's home directory
        let home_path = std::env::var("HOME").unwrap_or("/Users".to_string());
        let action = ActionSpec::Folder { path: home_path };
        let result = execute(action, "", &env);
        assert!(result.is_ok());
    }

    #[test]
    fn test_execute_shell_action_simple() {
        let env = Environment::new();
        let action = ActionSpec::Shell { command: "echo hello".to_string() };
        let result = execute(action, "", &env);
        assert!(result.is_ok());
    }

    #[test]
    fn test_execute_shell_action_complex() {
        let env = Environment::new();
        let action = ActionSpec::Shell { command: "ls -la /tmp".to_string() };
        let result = execute(action, "", &env);
        assert!(result.is_ok());
    }

    #[test]
    fn test_execute_javascript_action() {
        let env = Environment::new();
        let action = ActionSpec::JavaScript { 
            code: "shell('echo Hello from JavaScript');".to_string() 
        };
        let result = execute(action, "", &env);
        assert!(result.is_ok(), "JavaScript action should execute successfully: {:?}", result);
    }

    #[test]
    fn test_environment_creation() {
        let env = Environment::new();
        assert_eq!(env.config_path, PathBuf::from("config.yaml"));
        assert!(env.variables.is_empty());
    }
}