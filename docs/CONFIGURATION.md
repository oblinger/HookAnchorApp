# HookAnchor Configuration Guide[[API_REFERENCE 1]]

HookAnchor is a universal command launcher that uses a YAML configuration file to define its behavior. This guide provides comprehensive documentation for configuring all aspects of the application.

## Table of Contents

1. [Configuration File Location](#configuration-file-location)
2. [Application Settings](#application-settings)
3. [Keyboard Shortcuts](#keyboard-shortcuts)
4. [Key Naming Convention](#key-naming-convention)
5. [Templates](#templates)
6. [Template Variables](#template-variables)
7. [Grabber Rules](#grabber-rules)
8. [Functions](#functions)
9. [Launcher Settings](#launcher-settings)
10. [Scanner Settings](#scanner-settings)
11. [Markdown Roots](#markdown-roots)
12. [Complete Example](#complete-example)
13. [Key Reference](#key-reference)

## Configuration File Location

The main configuration file is located at:
```
~/.config/hookanchor/config.yaml
```

## Application Settings

### Popup Settings

Controls the behavior and appearance of the popup interface:

```yaml
popup_settings:
  max_rows: 25                    # Maximum rows to display in popup
  max_columns: 4                  # Maximum columns for multi-column layout
  debug_log: "~/.config/hookanchor/anchor.log"  # Debug log file path
  server_log: "~/.config/hookanchor/server.log" # Server log file path
  debug_scanner: false            # Enable scanner debug logging
  verbose_logging: true           # Enable verbose debug output
  merge_similar: true             # Merge similar commands in popup
  word_separators: " ._-"         # Characters used for word separation
  scan_interval_seconds: 86400000 # Auto-scan interval (large = disabled)
  idle_timeout_seconds: 60        # Auto-close timeout in seconds
  countdown_seconds: 5            # Grabber countdown duration
  run_in_background: true         # Keep app running for instant popup
  max_log_file_size: 1000000      # Maximum log file size in bytes (1MB) before clearing
  listed_actions: "alias,anchor,app,url,folder,cmd,chrome,safari,brave,firefox,work,notion,obs,obs_url,1pass,rewrite,doc,contact,slack,text,shutdown"
```

### Launcher Settings

Configuration for the command launcher system:

```yaml
launcher_settings:
  application_folder: "/Applications/HookAnchor.app"
  default_browser: "Google Chrome"
  work_browser: "Google Chrome Beta"
  timeout_ms: 5000                # Command execution timeout
  obsidian_app_name: "Obsidian"
  obsidian_vault_name: "kmr"
  obsidian_vault_path: "~/ob/kmr"
```

### Scanner Settings

Configuration for filesystem scanning:

```yaml
scanner_settings:
  orphans_path: "/path/to/orphans/folder"  # Path for orphaned commands
```

## Keyboard Shortcuts

Define keyboard shortcuts for application actions:

```yaml
keybindings:
  exit_app: "Escape"              # Exit the application
  navigate_down: "ArrowDown"      # Move selection down
  navigate_up: "ArrowUp"          # Move selection up
  navigate_left: "ArrowLeft"      # Move selection left
  navigate_right: "ArrowRight"    # Move selection right
  execute_command: "Enter"        # Execute selected command
  force_rebuild: "`"              # Force rebuild (backtick)
  show_folder: "/"                # Launch first folder match
  open_editor: "="                # Open command editor
  edit_active_command: ";"        # Edit currently selected command
  cancel_editor: "Escape"         # Cancel command editor
```

## Key Naming Convention

HookAnchor uses a hybrid approach for naming keys in configuration:

### ASCII Characters (Preferred)
Use the actual character when it's printable and unambiguous:
```yaml
key: "a"        # Letter a
key: "5"        # Number 5
key: "+"        # Plus sign
key: "-"        # Minus/hyphen
key: "="        # Equals sign
key: ">"        # Greater than
key: "<"        # Less than
key: "/"        # Forward slash
key: "\\"       # Backslash (escaped)
key: ";"        # Semicolon
key: "`"        # Backtick
```

### Named Keys
Use descriptive names for non-printable or special keys:
```yaml
key: "Enter"        # Enter key
key: "Escape"       # Escape key
key: "Space"        # Space bar
key: "Tab"          # Tab key
key: "Backspace"    # Backspace key
key: "Delete"       # Delete key
key: "ArrowUp"      # Up arrow
key: "ArrowDown"    # Down arrow
key: "ArrowLeft"    # Left arrow
key: "ArrowRight"   # Right arrow
key: "F1"           # Function key F1
key: "Home"         # Home key
key: "End"          # End key
key: "PageUp"       # Page Up
key: "PageDown"     # Page Down
```

### Modifier Keys (Chord Support)
Chord combinations using standard notation:
```yaml
key: "Cmd+C"        # Command+C (Mac)
key: "Ctrl+S"       # Control+S
key: "Alt+F4"       # Alt+F4
key: "Shift+Tab"    # Shift+Tab
key: "Cmd+Shift+P"  # Command+Shift+P
key: "Ctrl+Shift+A" # Control+Shift+A

# Modifier names are case-insensitive and accept aliases:
key: "cmd+c"        # Same as "Cmd+C"
key: "command+c"    # Same as "Cmd+C"
key: "ctrl+s"       # Same as "Ctrl+S"
key: "control+s"    # Same as "Ctrl+S"
key: "alt+f4"       # Same as "Alt+F4"
key: "option+f4"    # Same as "Alt+F4" (Mac)
```

## Templates

Templates provide a powerful way to create new commands with variable substitution:

```yaml
templates:
  # Basic template example
  basic:
    key: "-"                    # Trigger key (minus)
    name: "{{input}}"           # Command name using input variable
    action: "anchor"            # Command action
    arg: "/path/{{input}}.md"   # Command argument with variable
    patch: "{{selected_patch}}" # Use selected command's patch
    flags: ""                   # Command flags
    edit: true                  # Open editor before creating
    
  # Note template with file creation
  note:
    key: "$"
    name: "{{YYYY}}-{{MM}}-{{DD}} {{input}}"
    action: "markdown"
    arg: "~/Notes/{{YYYY}}/{{MM}}/{{input}}.md"
    patch: "Notes"
    edit: true
    file: "~/Notes/{{YYYY}}/{{MM}}"     # Create folder
    contents: |                          # Create file with content
      # {{YYYY}}-{{MM}}-{{DD}} {{input}}
      
      Created: {{YYYY}}-{{MM}}-{{DD}} {{hh}}:{{mm}}
      
  # Grabber template
  grab:
    key: "%"
    grab: 3                     # Wait 3 seconds before grabbing
    name: "{{input}}"
    action: "{{grabbed_action}}"
    arg: "{{grabbed_arg}}"
    edit: true
    
  # Alias template using previous command
  alias:
    key: "<"
    name: "{{input}}"
    action: "alias"
    arg: "{{previous_name}}"
    patch: "{{previous_patch}}"
    edit: true
```

## Template Variables

Templates support variable substitution using `{{variable}}` syntax:

### Input Variables
- `{{input}}` - User input when template is triggered

### Selected Command Variables
- `{{selected_name}}` - Name of currently selected command
- `{{selected_path}}` - Path/arg of currently selected command
- `{{selected_patch}}` - Patch of currently selected command
- `{{selected_folder}}` - Folder extracted from selected command's path

### Previous Command Variables
- `{{previous_name}}` - Name of last executed command
- `{{previous_path}}` - Path/arg of last executed command
- `{{previous_patch}}` - Patch of last executed command
- `{{previous_folder}}` - Folder extracted from previous command's path

### Date/Time Variables
- `{{YYYY}}` - Four-digit year (e.g., 2025)
- `{{YY}}` - Two-digit year (e.g., 25)
- `{{MM}}` - Two-digit month (01-12)
- `{{M}}` - Month without leading zero (1-12)
- `{{MMM}}` - Month abbreviation (Jan, Feb, etc.)
- `{{DD}}` - Two-digit day (01-31)
- `{{D}}` - Day without leading zero (1-31)
- `{{hh}}` - Two-digit hour (00-23)
- `{{h}}` - Hour without leading zero (0-23)
- `{{mm}}` - Two-digit minute (00-59)
- `{{m}}` - Minute without leading zero (0-59)
- `{{ss}}` - Two-digit second (00-59)
- `{{s}}` - Second without leading zero (0-59)

### Grabber Variables
- `{{grabbed_action}}` - Action determined by grabber rules
- `{{grabbed_arg}}` - Argument captured by grabber

## Grabber Rules

Grabber rules define how to capture context from different applications:

```yaml
grabber_rules:
  # Browser URL capture
  - name: "Chrome URL"
    matcher: "bundleId === 'com.google.Chrome' && props.url ? props.url : null"
    action: "chrome"
    group: "Web"
    
  # File system capture
  - name: "Finder Anchor"
    matcher: "bundleId === 'com.apple.finder' && props.selection && props.recommendedAction === 'anchor' ? props.selection : null"
    action: "anchor"
    group: "Files"
    
  # Application capture with fallback
  - name: "Application Launcher"
    matcher: "app && app.length > 0 ? app : null"
    action: "app"
    group: "Apps"
```

## Functions

Functions define actions that commands can execute:

### Simple Functions
```yaml
functions:
  # Basic app launcher
  action_app: {fn: launch_app, name: "{{arg}}"}
  
  # URL opener
  action_url: {fn: open_url, url: "{{arg}}"}
  
  # Browser-specific actions
  action_chrome: {fn: open_with, app: "Google Chrome", arg: "{{arg}}"}
```

### JavaScript Functions
```yaml
functions:
  # Complex shell command with windowing
  action_cmd: |
    const fullCmd = "{{arg}}";
    if (fullCmd.startsWith('W ')) {
      const command = fullCmd.substring(2);
      shell(`osascript -e 'tell application "Terminal" to do script "${command}"'`);
    } else {
      shell(fullCmd);
    }
    
  # Folder opener with path resolution
  action_folder: |
    const folderPath = "{{arg}}";
    if (folderPath.startsWith('/') || folderPath.startsWith('~')) {
      open_folder(folderPath);
    } else {
      const vaultRoot = getObsidianVaultPath();
      const absolutePath = joinPath(vaultRoot, folderPath);
      open_folder(absolutePath);
    }
```

## Markdown Roots

Define directories to scan for markdown files:

```yaml
markdown_roots:
  - "~/Documents"
  - "~/Notes"
  - "~/Projects"
```

## Complete Example

Here's a complete minimal configuration file:

```yaml
# Application Settings
popup_settings:
  max_rows: 20
  debug_log: "~/.config/hookanchor/anchor.log"
  
# Keyboard Shortcuts  
keybindings:
  exit_app: "Escape"
  execute_command: "Enter"
  
# Templates
templates:
  note:
    key: "$"
    name: "{{YYYY}}-{{MM}}-{{DD}} {{input}}"
    action: "markdown"
    arg: "~/Notes/{{input}}.md"
    edit: true
    
# Basic Functions
functions:
  action_app: {fn: launch_app, name: "{{arg}}"}
  action_url: {fn: open_url, url: "{{arg}}"}
  
# Markdown Scanning
markdown_roots:
  - "~/Documents"
```

## Key Reference

### Printable ASCII Characters
```
Letters: a b c d e f g h i j k l m n o p q r s t u v w x y z
         A B C D E F G H I J K L M N O P Q R S T U V W X Y Z
Numbers: 0 1 2 3 4 5 6 7 8 9
Symbols: ! @ # $ % ^ & * ( ) - _ = + [ ] { } \ | ; : ' " , . < > / ?
Special: ` ~ (backtick, tilde)
```

### Named Keys (egui::Key compatible)
```
Navigation: ArrowUp, ArrowDown, ArrowLeft, ArrowRight
           Home, End, PageUp, PageDown
           
Action:     Enter, Escape, Space, Tab, Backspace, Delete
           Insert, Pause, PrintScreen, ScrollLock
           
Function:   F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12
           F13, F14, F15, F16, F17, F18, F19, F20
           
Numpad:     Num0, Num1, Num2, Num3, Num4, Num5, Num6, Num7, Num8, Num9
           NumLock, NumpadAdd, NumpadSubtract, NumpadMultiply, NumpadDivide
           NumpadDecimal, NumpadEnter, NumpadEquals
```

### Modifier Keys (for future chord support)
```
Primary:    Cmd (Mac), Ctrl (Windows/Linux)
Secondary:  Alt, Option (Mac), Meta 
Modifier:   Shift
```

For the most up-to-date key name reference, see the [egui::Key documentation](https://docs.rs/egui/latest/egui/enum.Key.html).

---

This documentation covers all aspects of HookAnchor configuration. For specific examples and advanced usage patterns, refer to the default configuration file and example templates included with the application.