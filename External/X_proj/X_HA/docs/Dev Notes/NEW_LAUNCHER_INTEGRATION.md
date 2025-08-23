# New Launcher System Integration

## Overview

The anchor selector now supports two execution modes:

1. **Legacy Mode** (default): Writes commands to `/tmp/cmd_file` for external processing
2. **New Launcher Mode**: Executes commands directly using the new Rust-based launcher system

## Configuration

### Enable New Launcher

Add to `~/.config/hookanchor/config.yaml`:

```yaml
settings:
  max_rows: 15
  max_columns: 3
  use_new_launcher: true  # Enable new launcher system
```

### Create Launcher Commands

Create `~/.config/hookanchor/launcher_config.yaml`:

```yaml
simple_commands:
  finder: !App
    name: "Finder"
  
  github: !Url
    url: "https://github.com"
  
  home: !Folder
    path: "/Users/username"
  
  chrome_github: !OpenWith
    app: "Google Chrome"
    arg: "https://github.com"
  
  hello: !Shell
    command: "echo 'Hello World!'"

scripted_commands:
  test_script: "console.log('Hello from JavaScript');"

settings:
  default_browser: "Google Chrome"
  work_browser: "Google Chrome Beta"
  timeout_ms: 5000
```

## Command Types

### 1. App Launcher
```yaml
command_name: !App
  name: "Application Name"
```
Equivalent to: `open -a "Application Name"`

### 2. URL Opening
```yaml
command_name: !Url
  url: "https://example.com"
```
Equivalent to: `open "https://example.com"`

### 3. App with Argument
```yaml
command_name: !OpenWith
  app: "Application Name"
  arg: "argument"
```
Equivalent to: `open -a "Application Name" "argument"`

### 4. Folder Opening
```yaml
command_name: !Folder
  path: "/path/to/folder"
```
Equivalent to: `open "/path/to/folder"`

### 5. Shell Command
```yaml
command_name: !Shell
  command: "shell command here"
```
Equivalent to: `/bin/sh -c "shell command here"`

### 6. JavaScript (Future)
```yaml
scripted_commands:
  command_name: "JavaScript code here"
```

## Command Types

The launcher system supports all major command types:

| Action Type | Launcher Implementation | Example |
|----------------|-------------------|---------|
| `app NAME` | `!App` | Launch applications |
| `url URL` | `!Url` | Open URLs in default browser |
| `chrome URL` | `!OpenWith` | Open URL in specific browser |
| `safari URL` | `!OpenWith` | Open URL in Safari |
| `brave URL` | `!OpenWith` | Open URL in Brave |
| `firefox URL` | `!OpenWith` | Open URL in Firefox |
| `work URL` | `!OpenWith` | Open URL in work browser |
| `notion URL` | `!OpenWith` | Open in Notion app |
| `obs_url URL` | `!OpenWith` | Open in Obsidian |
| `folder PATH` | `!Folder` | Open folders |
| `doc PATH` | `!OpenWith` | Open documents |
| `cmd COMMAND` | `!Shell` | Execute shell commands |
| `1pass NAME` | `!OpenWith` | Launch 1Password |
| `alias NAME` | JavaScript | Call other commands |
| `obs NAME` | JavaScript | Complex Obsidian logic |

## Testing

### Integration Test
```bash
cargo run --bin test_popup_integration
```

### Manual Test
1. Enable new launcher in config
2. Run popup: `cargo run --bin popup`
3. Type command name and press Enter
4. Verify application/URL opens correctly

## Benefits

- ✅ **Faster execution** (pure Rust/JavaScript implementation)
- ✅ **Better error handling** (Rust error types)
- ✅ **No external dependencies** (self-contained)
- ✅ **Type safety** (compile-time validation)
- ✅ **Future extensibility** (JavaScript support planned)

## Backward Compatibility

The system maintains full backward compatibility:
- `use_new_launcher: false` (default) → Uses legacy temp file method
- `use_new_launcher: true` → Uses new Rust launcher system

Existing workflows continue to work unchanged when the flag is disabled.