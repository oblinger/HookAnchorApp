# JavaScript API Reference

This document lists all available JavaScript built-in functions provided by the Anchor Selector launcher for use in business logic, config files, and action customization.

## Logging Functions

- `log(message)` - General logging output
- `debug(message)` - Debug logging output  
- `error(message)` - Error logging output

**Example:**
```javascript
log("Starting application launch");
debug("Debug info: path = " + path);
error("Failed to find configuration file");
```

## File Operations

- `readFile(path)` - Read file contents as string
- `writeFile(path, content)` - Write content to file
- `fileExists(path)` - Check if file exists (returns boolean)
- `isDirectory(path)` - Check if path is directory (returns boolean)
- `listFiles(directory, pattern)` - List files in directory with optional pattern

**Example:**
```javascript
if (fileExists("config.json")) {
    const config = readFile("config.json");
    log("Found config: " + config);
}

if (isDirectory("/Applications")) {
    const apps = listFiles("/Applications", ".app");
    log("Found " + apps.length + " applications");
}
```

## Path Utilities

- `joinPath(part1, part2)` - Join path components
- `dirname(path)` - Get directory name from path
- `basename(path)` - Get base filename from path
- `expandHome(path)` - Expand ~ in paths to home directory
- `getExtension(path)` - Get file extension from path

**Example:**
```javascript
const home = expandHome("~");
const documentsPath = joinPath(home, "Documents");
const fileName = basename("/Users/john/file.txt"); // Returns "file.txt"
const ext = getExtension("document.pdf"); // Returns "pdf"
```

## Text Processing

- `testRegex(text, pattern)` - Test text against regex pattern (returns boolean)

**Example:**
```javascript
if (testRegex(url, "github\\.com")) {
    launch_app("Google Chrome", url);
} else if (testRegex(url, "notion\\.so")) {
    launch_app("Notion", url);
}
```

## Data Parsing

- `parseYaml(text)` - Parse YAML text to JSON string

**Example:**
```javascript
if (fileExists(".config.yaml")) {
    const yamlText = readFile(".config.yaml");
    const config = JSON.parse(parseYaml(yamlText));
    log("Loaded config with " + Object.keys(config).length + " keys");
}
```

## Launcher Built-ins

- `launch_app(app_name, arg)` - Launch macOS application with optional argument
- `open_folder(path)` - Open folder in Finder (or configured folder app)
- `open_url(url, browser)` - Open URL in browser (optional browser name)
- `shell(command)` - Execute shell command and return output
- `change_directory(path)` - Change working directory
- `launch(command_name)` - Recursively call another launcher command
- `has_tmux_session(name)` - Check if tmux session exists (returns boolean)
- `start_tmux_session(config_file)` - Start tmux session from .tmuxp.yaml config
- `activate_iterm()` - Bring iTerm2 application to foreground
- `start_claude_code()` - Start Claude Code in current directory

**Example:**
```javascript
// Launch applications
launch_app("Google Chrome", "https://github.com");
launch_app("Finder"); // No argument needed

// Work with folders
open_folder(expandHome("~/Documents"));
change_directory("/tmp");

// Execute shell commands
const result = shell("ls -la");
log("Directory listing: " + result);

// Development environment setup
if (fileExists(".tmuxp.yaml")) {
    if (!has_tmux_session("myproject")) {
        start_tmux_session(".tmuxp.yaml");
    }
    activate_iterm();
} else if (fileExists("CLAUDE.md")) {
    start_claude_code();
}

// Recursive command calling
launch("app Finder"); // Calls another launcher command
```

## Usage in Config Files

These functions can be used in the `js_actions` section of your config.yaml:

```yaml
js_actions:
  # Custom action with environment detection
  smart_open: |
    const path = "{{arg}}";
    
    if (isDirectory(path)) {
        open_folder(path);
        
        // Check for development environment
        const oldDir = shell("pwd").trim();
        change_directory(path);
        
        if (fileExists("package.json")) {
            log("Node.js project detected");
            shell("npm install");
        } else if (fileExists("Cargo.toml")) {
            log("Rust project detected"); 
            shell("cargo check");
        }
        
        change_directory(oldDir);
    } else {
        launch_app("TextEdit", path);
    }
  
  # Complex URL handling
  smart_browser: |
    const url = "{{arg}}";
    
    if (testRegex(url, "github\\.com")) {
        launch_app("Google Chrome", url);
    } else if (testRegex(url, "notion\\.so")) {
        launch_app("Notion", url);
    } else if (testRegex(url, "figma\\.com")) {
        launch_app("Figma", url);
    } else {
        open_url(url); // Default browser
    }
```

## Advanced Examples

### Environment-Aware Activation
```javascript
// Different behavior based on what's available
const projectPath = "{{arg}}";
change_directory(projectPath);

const projectName = basename(projectPath);
const tmuxConfig = joinPath(projectPath, ".tmuxp.yaml");

if (fileExists(tmuxConfig)) {
    log("Development environment detected");
    if (!has_tmux_session(projectName)) {
        start_tmux_session(tmuxConfig);
    }
    activate_iterm();
    
    // Wait a bit then open in editor
    shell("sleep 1");
    if (fileExists("CLAUDE.md")) {
        start_claude_code();
    }
} else {
    log("Simple folder opening");
    open_folder(projectPath);
}
```

### Conditional Tool Selection
```javascript
// Use different tools based on user preferences or availability
const userConfig = expandHome("~/.launcher_config");

if (fileExists(userConfig)) {
    const config = JSON.parse(parseYaml(readFile(userConfig)));
    const folderApp = config.folder_app || "Finder";
    launch_app(folderApp, "{{arg}}");
} else {
    // Default behavior
    open_folder("{{arg}}");
}
```

### Multi-Step Workflows
```javascript
// Complex multi-step action
const url = "{{arg}}";

log("Processing URL: " + url);

// Step 1: Determine the right browser
let browser = "Google Chrome"; // Default
if (testRegex(url, "company\\.com")) {
    browser = "Google Chrome Beta"; // Work browser
}

// Step 2: Open the URL
open_url(url, browser);

// Step 3: Set up related workspace
if (testRegex(url, "github\\.com")) {
    // GitHub URL - might want to clone locally
    const repoMatch = url.match(/github\.com\/([^\/]+\/[^\/]+)/);
    if (repoMatch) {
        log("GitHub repository detected: " + repoMatch[1]);
        // Could automatically clone, open in editor, etc.
    }
}
```

## Command Context Variables

JavaScript actions have access to complete command context data:

### Template Substitution (YAML)
- `{{arg}}` - Command argument (replaced during YAML processing)

### JavaScript Global Variables
- `ARG` - Command argument (e.g., "/tmp" for "anchor /tmp")
- `ACTION` - Action name (e.g., "anchor", "obs", "url")
- `FULL_COMMAND` - Complete original command (e.g., "anchor /tmp")
- `COMMAND` - JSON object with complete command metadata

**Example:**
```yaml
js_actions:
  smart_action: |
    // Multiple ways to access command data
    log("Using template: {{arg}}");          // YAML template substitution
    log("Using ARG variable: " + ARG);       // JavaScript variable
    log("Action name: " + ACTION);           // Action being executed
    log("Full command: " + FULL_COMMAND);    // Complete user input
    
    // Parse the complete command context
    const cmd = JSON.parse(COMMAND);
    log("Command timestamp: " + cmd.timestamp);
    log("Action type: " + cmd.action_type);
    
    // Use context for conditional logic
    if (ACTION === "anchor") {
        log("This is an anchor activation command");
        // Anchor-specific logic here
    }
```

### Command Context Object (COMMAND)
The `COMMAND` variable contains a JSON object with:
```json
{
  "command": "anchor /tmp",      // Full original command
  "action": "anchor",            // Action name
  "arg": "/tmp",                 // Command argument
  "full_command": "anchor /tmp", // Same as command
  "action_type": "javascript",   // Type of action
  "timestamp": 1640995200        // Unix timestamp
}
```

## Error Handling

```javascript
try {
    const result = shell("some-command");
    log("Success: " + result);
} catch (e) {
    error("Command failed: " + e);
    // Fallback behavior
    launch_app("Terminal");
}
```

This comprehensive API enables users to create highly customized, environment-aware launcher actions that adapt to their specific tools and workflows.