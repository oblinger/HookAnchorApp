# Launcher System

## Overview

The launcher system executes commands with support for shell execution, JavaScript evaluation, application launching, and URL opening. It provides a unified interface for all command actions.

## Architecture

```rust
pub fn launch(
    action: &str,
    arg: &str,
    flags: &str,
    config: &Config,
) -> Result<String, String>
```

## Action Types

### Shell Commands (`cmd`)
```rust
// Basic execution
launch("cmd", "ls -la", "", config)

// With window flag
launch("cmd", "npm test", "W", config)  // Keep terminal open
```

### Application Launch (`app`)
```rust
launch("app", "Visual Studio Code", "", config)
launch("app", "Terminal", "", config)
```

### URL Opening (`url`)
```rust
launch("url", "https://github.com", "", config)
```

### Folder Opening (`folder`)
```rust
launch("folder", "~/Documents", "", config)
```

## JavaScript Evaluation

### Variable Expansion
```javascript
// In command arguments
"echo {{js:new Date().toISOString()}}"
"open {{js:process.env.HOME}}/projects"
```

### Available Context
```javascript
{
    Date, Math, JSON,
    env: process.env,
    home: os.homedir(),
    user: os.userInfo().username
}
```

## Shell Execution

### Process Creation
```rust
Command::new(shell)
    .arg("-c")
    .arg(command)
    .env_clear()
    .envs(environment)
    .output()
```

### Output Handling
- **Capture**: Store stdout/stderr
- **Stream**: Real-time output
- **Window**: Keep terminal open

## Flag System

| Flag | Description | Actions |
|------|-------------|----------|
| W | Window mode | cmd |
| C | Copy to clipboard | all |
| S | Silent execution | cmd |
| B | Background execution | cmd |

## Error Handling

```rust
match launch(action, arg, flags, config) {
    Ok(output) => process_output(output),
    Err(error) => show_error_dialog(error),
}
```

## Performance

- **Async Execution**: Non-blocking launches
- **Output Buffering**: Efficient capture
- **Process Pooling**: Reuse shells
- **Timeout Protection**: Configurable limits

## Related Documentation
- [Command System](command-system.md)
- [Configuration](configuration.md)
- [JavaScript Runtime](configuration.md#javascript-runtime)