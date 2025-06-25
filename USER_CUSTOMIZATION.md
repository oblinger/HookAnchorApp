# User Customization Guide

The Anchor Selector launcher provides a powerful JavaScript-based customization system that allows users to modify any action behavior without recompiling the application.

## Quick Start

### 1. View Available Functions
All JavaScript actions have access to 20+ built-in functions. See [`JAVASCRIPT_API.md`](JAVASCRIPT_API.md) for the complete reference.

### 2. Edit Config File
Customize actions in `~/.config/anchor_selector/config.yaml`:

```yaml
js_actions:
  # Replace any action with custom JavaScript
  anchor: |
    const path = "{{arg}}";
    log("Custom anchor activation for: " + path);
    
    // Your custom logic here
    change_directory(path);
    open_folder(path);
    
    // Environment-aware setup
    if (fileExists(path + "/.tmuxp.yaml")) {
        start_tmux_session(path + "/.tmuxp.yaml");
        activate_iterm();
    } else if (fileExists("CLAUDE.md")) {
        start_claude_code();
    }
```

### 3. Command Context Access

**JavaScript actions have access to complete command data:**

```javascript
// Individual variables for convenience
ARG          // Command argument (e.g., "/tmp" for "anchor /tmp")
ACTION       // Action name (e.g., "anchor", "obs", "url")
FULL_COMMAND // Complete command (e.g., "anchor /tmp")
COMMAND      // JSON object with full metadata

// Example usage
const cmd = JSON.parse(COMMAND);
log("Executing " + ACTION + " with arg: " + ARG);
log("Command timestamp: " + cmd.timestamp);
```

### 4. Available Built-in Functions

**Essential functions for customization:**

```javascript
// Logging
log(message), debug(message), error(message)

// File & Directory Operations  
fileExists(path), isDirectory(path), readFile(path), writeFile(path, content)
expandHome(path), basename(path), dirname(path), joinPath(part1, part2)

// Application & System Control
launch_app(app_name, arg), open_folder(path), open_url(url, browser) 
shell(command), change_directory(path)

// Development Tools
has_tmux_session(name), start_tmux_session(config), activate_iterm(), start_claude_code()

// Text & Data Processing
testRegex(text, pattern), parseYaml(text)

// Recursive Actions
launch(command_name)
```

## Customization Examples

### Environment-Aware Folder Opening
```javascript
// Custom folder action that adapts to user preferences
const path = "{{arg}}";

// Check user config for preferred folder app
const configPath = expandHome("~/.launcher_preferences.yaml");
let folderApp = "Finder"; // Default

if (fileExists(configPath)) {
    const config = JSON.parse(parseYaml(readFile(configPath)));
    folderApp = config.folder_app || "Finder";
}

launch_app(folderApp, path);
```

### Smart URL Handler
```javascript
// Intelligent URL routing based on domain
const url = "{{arg}}";

if (testRegex(url, "github\\.com")) {
    // Work repository - use work browser
    launch_app("Google Chrome Beta", url);
} else if (testRegex(url, "notion\\.so")) {
    // Notion pages - use Notion app
    launch_app("Notion", url);
} else if (testRegex(url, "figma\\.com")) {
    // Design work - use Figma app
    launch_app("Figma", url);
} else {
    // Everything else - default browser
    open_url(url);
}
```

### Project-Aware Activation
```javascript
// Smart project activation with tool detection
const projectPath = "{{arg}}";
change_directory(projectPath);

const projectName = basename(projectPath);
log("Activating project: " + projectName);

// 1. Always open the folder
open_folder(projectPath);

// 2. Detect project type and set up accordingly
if (fileExists("package.json")) {
    log("Node.js project detected");
    shell("npm install"); // Ensure dependencies
    
    if (fileExists(".tmuxp.yaml")) {
        start_tmux_session(".tmuxp.yaml");
        activate_iterm();
    } else {
        launch_app("Visual Studio Code", projectPath);
    }
    
} else if (fileExists("Cargo.toml")) {
    log("Rust project detected");
    shell("cargo check"); // Quick compile check
    
    if (fileExists("CLAUDE.md")) {
        start_claude_code();
    } else {
        launch_app("RustRover", projectPath);
    }
    
} else if (fileExists(".tmuxp.yaml")) {
    log("Generic development project with tmux");
    start_tmux_session(".tmuxp.yaml");
    activate_iterm();
    
} else {
    log("Simple folder activation");
    // Just the folder opening is enough
}
```

### Multi-Platform Actions
```javascript
// Cross-platform compatible actions
const isLinux = shell("uname").includes("Linux");
const isMacOS = shell("uname").includes("Darwin");
const path = "{{arg}}";

if (isMacOS) {
    // macOS-specific behavior
    open_folder(path); // Uses Finder
    
    if (fileExists(path + "/.tmuxp.yaml")) {
        start_tmux_session(path + "/.tmuxp.yaml");
        launch_app("iTerm2");
    }
    
} else if (isLinux) {
    // Linux-specific behavior
    shell("xdg-open '" + path + "'"); // Use system file manager
    
    if (fileExists(path + "/.tmuxp.yaml")) {
        start_tmux_session(path + "/.tmuxp.yaml");
        shell("gnome-terminal"); // Or user's preferred terminal
    }
}
```

### User Preference System
```javascript
// Create a user preference system
function getUserPreferences() {
    const prefFile = expandHome("~/.anchor_preferences.yaml");
    
    if (fileExists(prefFile)) {
        const yaml = readFile(prefFile);
        return JSON.parse(parseYaml(yaml));
    }
    
    // Default preferences
    return {
        folder_app: "Finder",
        terminal_app: "iTerm2", 
        editor_app: "Visual Studio Code",
        default_browser: "Google Chrome",
        work_browser: "Google Chrome Beta"
    };
}

// Use preferences in actions
const prefs = getUserPreferences();
const path = "{{arg}}";

launch_app(prefs.folder_app, path);

if (path.includes("work")) {
    open_url("https://company.slack.com", prefs.work_browser);
}
```

## Advanced Features

### Conditional Tool Installation
```javascript
// Check if tools are available, offer to install if missing
const requiredTools = ["tmux", "node", "git"];

for (const tool of requiredTools) {
    if (!shell("which " + tool).includes(tool)) {
        log("Missing tool: " + tool);
        
        if (tool === "tmux" && shell("which brew").includes("brew")) {
            log("Installing tmux via Homebrew...");
            shell("brew install tmux");
        }
    }
}
```

### Dynamic Configuration Loading
```javascript
// Load per-directory configuration
const projectPath = "{{arg}}";
const localConfig = joinPath(projectPath, ".anchor_config.yaml");

if (fileExists(localConfig)) {
    log("Found local configuration");
    const config = JSON.parse(parseYaml(readFile(localConfig)));
    
    // Use project-specific settings
    if (config.custom_activation) {
        // Execute custom activation script
        eval(config.custom_activation);
        return;
    }
}

// Fall back to default behavior
change_directory(projectPath);
open_folder(projectPath);
```

### Error Handling and Fallbacks
```javascript
// Robust error handling with fallbacks
const path = "{{arg}}";

try {
    // Try primary method
    launch_app("Path Finder", path);
    log("Opened with Path Finder");
} catch (e) {
    try {
        // Fallback to system default
        open_folder(path);
        log("Opened with system default");
    } catch (e2) {
        // Last resort
        shell("open '" + path + "'");
        log("Opened via shell command");
    }
}
```

## File Locations

- **User Config**: `~/.config/anchor_selector/config.yaml`
- **Source Code**: `src/js_runtime.rs` (built-in functions)
- **Action Scripts**: `src/business_logic/` directory
- **API Reference**: [`JAVASCRIPT_API.md`](JAVASCRIPT_API.md)

## Key Benefits

✅ **No Recompilation Required** - Edit actions without rebuilding  
✅ **Environment Adaptive** - Actions detect and adapt to available tools  
✅ **User-Specific** - Each user can customize for their workflow  
✅ **Rich API** - 20+ built-in functions for comprehensive control  
✅ **Cross-Platform Ready** - Environment detection enables platform-specific logic  
✅ **Extensible** - Add new actions or modify existing ones easily  

This system transforms the launcher from a fixed-function tool into a flexible, user-customizable automation platform that adapts to each user's unique environment and preferences.