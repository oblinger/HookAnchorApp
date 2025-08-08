# Configuration System

## Overview

HookAnchor uses a YAML-based configuration system with JavaScript extensions for maximum flexibility. The configuration controls all aspects of the application including UI behavior, keyboard shortcuts, command execution, and system integration.

## Configuration File Location

```
~/.config/hookanchor/config.yaml
```

Fallback locations:
1. `$XDG_CONFIG_HOME/hookanchor/config.yaml`
2. `~/Library/Application Support/hookanchor/config.yaml`
3. Built-in default configuration

## Configuration Structure

### Complete Schema
```yaml
# Popup UI Settings
popup_settings:
  max_rows: 10
  max_columns: 5
  max_height: 600
  max_width: 800
  bg_color: "#1e1e1e"
  text_color: "#ffffff"
  highlight_color: "#3d3d3d"
  selected_color: "#5a5a5a"
  merge_similar: true
  word_separators: " /-_."
  inactivity_timeout: 30
  run_in_background: false

# Scanner Settings
scanner_settings:
  anchors_path: "~/Documents/HookAnchor/Anchors"
  orphans_path: "~/Documents/HookAnchor/Orphans"
  scan_interval: 300  # seconds
  auto_scan: true
  scan_depth: 3
  create_directories: true

# Keyboard Bindings
keybindings:
  exit_app: "Escape"
  navigate_down: "ArrowDown"
  navigate_up: "ArrowUp"
  navigate_left: "ArrowLeft"
  navigate_right: "ArrowRight"
  execute_command: "Enter"
  force_rescan: "Backtick"
  show_folder: "Slash"
  open_editor: "Equals"
  edit_active_command: "Semicolon"
  delete_command: "Delete"
  copy_as_markdown: "cmd+shift+c"
  start_grabber: "Plus"

# JavaScript Functions
functions:
  action_app:
    fn: launch_app
    name: "{{arg}}"
  action_url:
    fn: open_url
    url: "{{arg}}"
  action_folder:
    fn: open_folder
    path: "{{arg}}"
  action_cmd:
    fn: shell
    command: "{{arg}}"
  action_doc:
    fn: open_with
    app: ""
    arg: "{{arg}}"
  
  # Custom functions
  open_in_vscode:
    fn: shell
    command: "code {{path}}"
  search_google:
    fn: open_url
    url: "https://google.com/search?q={{query}}"

# Template Definitions
templates:
  github_issue:
    prompt: "Issue title"
    command: "GitHub Issue: {{input}}"
    action: "url"
    arg: "https://github.com/user/repo/issues/new?title={{input|urlencode}}"
    patch: "dev"
    
  new_note:
    prompt: "Note title"
    command: "Note: {{input}}"
    action: "cmd"
    arg: "echo '# {{input}}' > ~/Notes/{{input|slugify}}.md"
    patch: "notes"
    edit: true  # Open in editor before executing
    
  timer:
    grab: 3  # Grab context after 3 seconds
    command: "Timer: {{grabbed_text}}"
    action: "cmd"
    arg: "osascript -e 'display notification \"{{grabbed_text}}\" with title \"Timer\"'"
    file_rescan: true  # Rescan after execution

# Launcher Settings
launcher_settings:
  shell: "/bin/zsh"
  environment:
    PATH: "/usr/local/bin:/usr/bin:/bin"
    EDITOR: "vim"
  timeout: 30000  # milliseconds
  capture_output: true

# Development Settings
development:
  debug_mode: false
  log_level: "info"
  show_timing: false
  mock_scanner: false
```

## Configuration Sections

### 1. Popup Settings

Controls the visual appearance and behavior of the popup window:

| Setting | Type | Default | Description |
|---------|------|---------|-------------|
| max_rows | int | 10 | Maximum rows in command grid |
| max_columns | int | 5 | Maximum columns in command grid |
| max_height | int | 600 | Maximum window height in pixels |
| max_width | int | 800 | Maximum window width in pixels |
| merge_similar | bool | true | Merge similar commands |
| word_separators | string | " /-_." | Characters that separate words |
| inactivity_timeout | int | 30 | Hide window after N seconds |
| run_in_background | bool | false | Keep popup process running |

### 2. Scanner Settings

Configures the filesystem scanner:

| Setting | Type | Default | Description |
|---------|------|---------|-------------|
| anchors_path | string | ~/Documents/HookAnchor/Anchors | Anchor files location |
| orphans_path | string | ~/Documents/HookAnchor/Orphans | Orphan commands location |
| scan_interval | int | 300 | Seconds between scans |
| auto_scan | bool | true | Enable automatic scanning |
| scan_depth | int | 3 | Directory recursion depth |
| create_directories | bool | true | Auto-create missing directories |

### 3. Keyboard Bindings

All keyboard shortcuts are configurable:

```yaml
keybindings:
  # Navigation
  navigate_down: "ArrowDown"        # or "j", "ctrl+n"
  navigate_up: "ArrowUp"            # or "k", "ctrl+p"
  
  # Actions
  execute_command: "Enter"          # or "Return"
  open_editor: "Equals"             # or "ctrl+e"
  
  # Modifiers
  copy_as_markdown: "cmd+shift+c"   # Multiple modifiers
  force_quit: "cmd+q"               # Standard macOS
```

**Key Format:**
- Single keys: `"a"`, `"Enter"`, `"Escape"`
- With modifiers: `"cmd+a"`, `"ctrl+shift+x"`
- Special keys: `"ArrowUp"`, `"Backtick"`, `"Delete"`

### 4. JavaScript Functions

Define reusable JavaScript functions:

```yaml
functions:
  # Basic function
  timestamp:
    fn: shell
    command: "echo {{js:new Date().toISOString()}}"
  
  # With parameters
  open_project:
    fn: shell
    command: "cd {{project_path}} && code ."
  
  # Complex logic
  smart_open:
    fn: shell
    command: |
      {{js:
        const ext = arg.split('.').pop();
        return ext === 'md' ? 'typora' : 'open';
      }} "{{arg}}"
```

**Function Components:**
- `fn`: Base function (shell, open_url, launch_app, etc.)
- Parameters: Named parameters with `{{param}}` syntax
- JavaScript: Inline with `{{js:expression}}`

### 5. Templates

Templates generate commands dynamically:

```yaml
templates:
  template_name:
    prompt: "User prompt text"        # What to ask user
    command: "Display name"           # Command name (can use {{input}})
    action: "cmd|url|app"            # Action type
    arg: "command or URL"            # Action argument
    patch: "category"                # Optional patch
    edit: false                      # Open editor first
    grab: 0                          # Grabber countdown
    file_rescan: false               # Rescan after execution
```

**Template Variables:**
- `{{input}}` - User input from prompt
- `{{selected_command}}` - Currently selected command
- `{{previous_command}}` - Previously executed command
- `{{grabbed_text}}` - Text from grabber

**Template Filters:**
- `{{input|urlencode}}` - URL encode
- `{{input|slugify}}` - Convert to slug
- `{{input|lower}}` - Lowercase
- `{{input|upper}}` - Uppercase

## Configuration Loading

### Load Sequence
1. Check user config location
2. Parse YAML file
3. Validate against schema
4. Merge with defaults
5. Initialize JavaScript runtime
6. Cache in memory

### Hot Reload
- Config file watched for changes
- Automatic reload on save
- Validation before applying
- Error notification on invalid config

### Error Handling
```yaml
# Invalid config example
keybindings:
  invalid_key: "not-a-real-key"  # Error: Unknown key

# HookAnchor will:
1. Log error to ~/.config/hookanchor/anchor.log
2. Show notification in UI
3. Continue with previous valid config
```

## JavaScript Runtime

### Available Functions
```javascript
// Built-in objects
Date, Math, JSON, Array, Object, String

// Custom functions
shell(command)          // Execute shell command
open_url(url)          // Open URL in browser
launch_app(name)       // Launch application
open_folder(path)      // Open folder in Finder

// Helpers
urlencode(str)         // URL encode string
slugify(str)           // Create URL slug
expand_path(path)      // Expand ~ and vars
```

### Security
- Sandboxed environment
- No file system access
- No network access
- Limited to defined functions

## Environment Variables

### System Variables
```yaml
launcher_settings:
  environment:
    PATH: "$PATH:/usr/local/bin"
    HOME: "$HOME"
    USER: "$USER"
```

### Custom Variables
```yaml
launcher_settings:
  environment:
    PROJECT_ROOT: "/Users/me/projects"
    GITHUB_TOKEN: "secret-token"
```

## Migration & Upgrades

### Config Version
```yaml
config_version: 2  # Internal version tracking
```

### Auto-Migration
- Old configs automatically upgraded
- Backup created before migration
- User notified of changes

### Breaking Changes
- Documented in release notes
- Migration guide provided
- Compatibility mode available

## Examples

### Minimal Config
```yaml
# Minimal working configuration
popup_settings:
  max_rows: 8
  max_columns: 4

keybindings:
  exit_app: "Escape"
  execute_command: "Enter"
```

### Developer Config
```yaml
# Developer-focused configuration
scanner_settings:
  anchors_path: "~/dev/anchors"
  scan_interval: 60

templates:
  git_commit:
    prompt: "Commit message"
    command: "Commit: {{input}}"
    action: "cmd"
    arg: "git add -A && git commit -m '{{input}}'"
    patch: "git"

development:
  debug_mode: true
  log_level: "debug"
```

### Power User Config
```yaml
# Advanced power user setup
popup_settings:
  merge_similar: false  # See all commands
  inactivity_timeout: 0  # Never auto-hide

keybindings:
  # Vim-style navigation
  navigate_down: "j"
  navigate_up: "k"
  navigate_left: "h"
  navigate_right: "l"
  
functions:
  # Custom workflows
  deploy_prod:
    fn: shell
    command: "cd {{project}} && npm run build && npm run deploy"
```

## Troubleshooting

### Config Not Loading
1. Check file permissions
2. Validate YAML syntax
3. Review logs: `~/.config/hookanchor/anchor.log`

### JavaScript Errors
1. Check syntax in functions
2. Verify variable names
3. Test in isolation

### Performance Issues
1. Reduce scanner frequency
2. Disable merge_similar
3. Decrease max_rows/columns

## Related Documentation

- [Keyboard Input](keyboard-input.md) - Key binding details
- [Launcher System](launcher-system.md) - Function execution
- [Scanner System](scanner-system.md) - Scanner configuration
- [Template System](template-creation.md) - Template details