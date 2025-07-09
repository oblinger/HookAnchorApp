# Anchor Selector Configuration Documentation

## Table of Contents

1. [Configuration File Overview](#configuration-file-overview)
2. [Commands File Format](#commands-file-format)
3. [Built-in Functions](#built-in-functions)
4. [Template Variables](#template-variables)
5. [Examples](#examples)

---

## Configuration File Overview

The Anchor Selector uses configuration files located in:
```
~/.config/hookanchor/
├── config.yaml      # Application settings and function definitions
└── commands.txt     # Command definitions (one per line)
```

### Commands File Format

The `commands.txt` file contains command definitions, one per line, in the format:
```
COMMAND : ACTION ARG
```
or with optional grouping:
```
GROUP ! COMMAND : ACTION ARG
```

Examples:
```
finder : app Finder
github : url https://github.com
dev ! server : cmd npm run dev
```

The configuration supports two types of function definitions:
- **Simple Functions**: Direct YAML function calls using the `{fn: function_name}` format  
- **JavaScript Functions**: Custom JavaScript code with access to rich built-in APIs

---

## Built-in Functions

#### Top-Level Built-in Functions (YAML Config)

| Function                   | Description                                          |
| -------------------------- | ---------------------------------------------------- |
| **launch_app(name, arg?)** | Launch an application with optional arguments        |
| **open_with(app, arg)**    | Open a file or URL with a specific application       |
| **open_url(url)**          | Open a URL in the default browser                    |
| **open_folder(path)**      | Open a folder in Finder (supports ~ expansion)       |
| **shell(command)**         | Execute shell commands and return output             |
| **javascript(code)**       | Execute JavaScript code with access to built-in APIs |

#### JavaScript Helper Functions

| Function                           |                                                            |
| ---------------------------------- | ---------------------------------------------------------- |
|                                    |                                                            |
|                                    | **Logging Functions**                                      |
| **log(message)**                   | General logging output to debug log                        |
| **debug(message)**                 | Debug-level logging output                                 |
| **error(message)**                 | Error-level logging output                                 |
|                                    |                                                            |
|                                    | **File Operations**                                        |
| **readFile(path)**                 | Read file contents as string                               |
| **writeFile(path, content)**       | Write content to file                                      |
| **fileExists(path)**               | Check if file exists, returns boolean                      |
| **isDirectory(path)**              | Check if path is directory, returns boolean                |
| **listFiles(directory, pattern?)** | List files in directory with optional pattern              |
|                                    |                                                            |
|                                    | **Path Utilities**                                         |
| **joinPath(part1, part2)**         | Join path components properly                              |
| **dirname(path)**                  | Get directory name from path                               |
| **basename(path)**                 | Get base filename from path                                |
| **expandHome(path)**               | Expand ~ in paths to home directory                        |
| **getExtension(path)**             | Get file extension from path                               |
|                                    |                                                            |
|                                    | **Data Processing**                                        |
| **testRegex(text, pattern)**       | Test text against regex pattern, returns boolean           |
| **parseJSON(text)**                | Parse JSON string to object                                |
| **toJSON(object)**                 | Convert object to JSON string                              |
| **parseYAML(text)**                | Parse YAML string to object                                |
| **toYAML(object)**                 | Convert object to YAML string                              |
|                                    |                                                            |
|                                    | **System Operations**                                      |
| **getEnv(name)**                   | Get environment variable value                             |
| **setEnv(name, value)**            | Set environment variable                                   |
| **getCurrentDirectory()**          | Get current working directory                              |
| **changeDirectory(path)**          | Change working directory                                   |
| **launch(command_name)**           | Recursively call another launcher command                  |
| **activateApp(app_name)**          | Bring application to foreground                            |
| **runAppleScript(script)**         | Execute AppleScript and return result                      |
| **spawnDetached(cmd, args)**       | Start process without waiting for completion               |
| **appIsRunning(app_name)**         | Check if application is currently running, returns boolean |

---

## Template Variables

Template variables allow dynamic substitution of values at runtime:
- **{{arg}}** - The argument provided by the user

---

## Examples

### Simple Function Configuration

```yaml
popup_settings:
  max_rows: 10
  use_new_launcher: true
  debug_log: "~/hookanchor_debug.log"

simple_functions:
  app: {fn: launch_app, name: "{{arg}}"}
  url: {fn: open_url, url: "{{arg}}"}
  folder: {fn: open_folder, path: "{{arg}}"}
  cmd: {fn: shell, command: "{{arg}}"}
  chrome: {fn: open_with, app: "Google Chrome", arg: "{{arg}}"}
  finder: {fn: launch_app, name: "Finder"}
  downloads: {fn: open_folder, path: "~/Downloads"}
```

### JavaScript Function Configuration

```yaml
js_functions:
  # File type handler using built-in functions
  smart_open: |
    const file = "{{arg}}";
    const ext = getExtension(file);
    
    if (fileExists(file)) {
      switch(ext) {
        case ".md":
          launch_app("Typora", file);
          break;
        case ".js":
        case ".py":
          launch_app("VS Code", file);
          break;
        default:
          launch_app("Finder", dirname(file));
      }
    } else {
      error("File not found: " + file);
    }

  # Environment-based browser selection
  browse: |
    const url = "{{arg}}";
    const isDev = getEnv("NODE_ENV") === "development";
    
    if (isDev) {
      launch_app("Google Chrome Beta", url);
    } else {
      launch_app("Safari", url);
    }

  # Project launcher with validation
  project: |
    const name = "{{arg}}";
    const projectPath = joinPath(expandHome("~/projects"), name);
    
    if (isDirectory(projectPath)) {
      changeDirectory(projectPath);
      launch_app("VS Code", projectPath);
      activateApp("VS Code");
      log("Opened project: " + name);
    } else {
      error("Project not found: " + name);
    }
```

### Commands File Example

```
# ~/.config/hookanchor/commands.txt

# Basic commands
finder : app Finder
chrome : app Google Chrome
vscode : app Visual Studio Code

# URLs
github : url https://github.com
google : url https://google.com

# Folders
downloads : folder ~/Downloads
documents : folder ~/Documents

# Grouped commands
dev ! server : cmd npm run dev
dev ! build : cmd npm run build
dev ! test : cmd npm test

# Custom functions
smart : smart_open
browse : browse
proj : project
```
