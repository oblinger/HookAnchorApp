# Dispatcher (CLI Interface)

## Overview

The dispatcher (`ha` command) provides a unified command-line interface for all HookAnchor functionality including direct command execution, popup control, URL handling, and system management.

## Architecture

```rust
// Main entry point: src/dispatcher.rs
pub fn main() {
    let args = parse_arguments();
    match args.mode {
        Mode::Popup => launch_popup(),
        Mode::Execute => execute_command(),
        Mode::Hook => handle_url(),
        Mode::Manage => manage_system(),
    }
}
```

## Command-Line Interface

### Usage
```bash
ha [OPTIONS] [COMMAND]
```

### Options
```
-m, --match <PATTERN>     Search and show popup with pattern
-x, --execute <PATTERN>   Execute first matching command
-f, --folder <PATTERN>    Open folder for matching anchor
-r, --run <CMD>          Run command directly
-a, --add <TYPE> <CMD>   Add new command
    --hook <URL>         Handle hook:// URL
    --rebuild            Rebuild command database
    --rescan             Force filesystem scan
    --status             Show system status
    --version            Show version
    --help               Show help
```

## Modes of Operation

### 1. Popup Mode (Default)
```bash
ha                    # Show popup
ha -m "search term"   # Show with search
```

### 2. Direct Execution
```bash
ha -x "Terminal"      # Execute matching command
ha -r "ls -la"       # Run shell command
```

### 3. URL Handling
```bash
ha --hook "hook://command"   # Handle URL scheme
ha "hook://terminal"         # Auto-detect URL
```

### 4. Anchor Management
```bash
ha -f "project"      # Open project folder
ha --add cmd "test"  # Add new command
```

### 5. System Management
```bash
ha --rebuild          # Rebuild database
ha --rescan          # Force scan
ha --status          # System info
```

## URL Scheme Handling

### Registration
The dispatcher registers as handler for `hook://` URLs:

```xml
<!-- Info.plist -->
<key>CFBundleURLTypes</key>
<array>
    <dict>
        <key>CFBundleURLSchemes</key>
        <array>
            <string>hook</string>
        </array>
    </dict>
</array>
```

### URL Processing
```rust
fn handle_hook_url(url: &str) {
    // Parse: hook://command_name
    let command = extract_command(url);
    
    // Execute directly
    if let Some(cmd) = find_command(command) {
        execute_command(cmd);
    }
}
```

## Command Resolution

### Search Algorithm
1. Exact match (case-insensitive)
2. Prefix match
3. Fuzzy match
4. Patch match

### Execution Flow
```
Pattern → Find Command → Resolve Aliases → Execute
   ↓           ↓               ↓                ↓
Search    Database      Expand          Launcher
```

## Integration Points

### With Popup
```rust
// Launch popup with search
Command::new("popup")
    .arg(search_term)
    .spawn()
```

### With Scanner
```rust
// Trigger scan
scanner::force_scan(&config)
```

### With Commands
```rust
// Direct command access
let commands = commands::load_commands();
commands::execute(&command, &config);
```

## Shell Integration

### Aliases
```bash
# In ~/.zshrc or ~/.bashrc
alias h="ha -m"
alias hx="ha -x"
alias hf="ha -f"
```

### Functions
```bash
# Quick add command
hadd() {
    ha --add cmd "$1" "$2"
}

# Search and execute
hrun() {
    ha -x "$1"
}
```

## Performance

- **Fast Startup**: < 10ms
- **Lazy Loading**: Commands loaded on demand
- **Direct Execution**: No popup overhead
- **Cached State**: Reuse loaded data

## Error Handling

```rust
match execute_command(&cmd) {
    Ok(output) => println!("{}", output),
    Err(e) => {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
```

## Examples

### Daily Usage
```bash
# Open terminal
ha -x terminal

# Search for git commands
ha -m git

# Open project folder
ha -f myproject

# Add quick note
ha --add cmd "note" "echo '$1' >> ~/notes.md"
```

### Automation
```bash
# In scripts
#!/bin/bash
ha -x "Build Project"
ha -x "Run Tests"
ha -x "Deploy"
```

### URL Handler
```bash
# From other apps
open "hook://slack"
open "hook://terminal"
```

## Testing

```bash
# Test URL handling
ha --hook "hook://test"

# Test pattern matching
ha -x "nonexistent" || echo "Correctly failed"

# Test direct execution
ha -r "echo test"
```

## Related Documentation
- [Command System](command-system.md)
- [URL Handling](../URL_HANDLING.md)
- [Popup System](popup-system.md)
- [Configuration](configuration.md)