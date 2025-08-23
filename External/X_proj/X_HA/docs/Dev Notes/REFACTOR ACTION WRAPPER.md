# REFACTOR ACTION WRAPPER

## Overview

This document describes the refactoring of the action execution system to use a unified action wrapper architecture where actions are represented as flexible HashMaps and executed through a consistent pipeline in both popup and server contexts.

## Core Design Principles

1. **Actions as HashMaps**: Actions are represented as `HashMap<String, JsonValue>` with a required `type` field
2. **Unified Execution**: Same execution logic in both popup and server
3. **Simple Routing**: Popup routes based on action type (local vs server execution)
4. **Function Lookup**: Functions are resolved by prefixing "action_" to the type
5. **Commands are Actions**: Commands are just named actions stored in commands.txt

## Key API

### Command to Action Conversion
```rust
fn command_to_action(cmd: &Command) -> Action {
    let mut params = HashMap::new();
    params.insert("type".to_string(), JsonValue::String(cmd.action.clone()));
    params.insert("arg".to_string(), JsonValue::String(cmd.arg.clone()));
    params.insert("command_name".to_string(), JsonValue::String(cmd.command.clone()));
    params.insert("patch".to_string(), JsonValue::String(cmd.patch.clone()));
    params.insert("flags".to_string(), JsonValue::String(cmd.flags.clone()));
    
    Action(params)
}
```


### Action Structure
```rust
pub struct Action(HashMap<String, JsonValue>);

impl Action {
    pub fn action_type(&self) -> Option<&str> {
        self.0.get("type")?.as_str()
    }
    
    pub fn get(&self, key: &str) -> Option<&JsonValue> {
        self.0.get(key)
    }
    
    pub fn params(&self) -> &HashMap<String, JsonValue> {
        &self.0
    }
}
```


### Main Execution Entry Point
```rust
// In popup.rs
fn execute(&mut self, action: &Action) {
    match action.action_type() {
        Some("template") | Some("popup") => {
            self.execute_locally(action);
        }
        _ => {
            self.send_action_to_server(action);
        }
    }
}
```


### Action JSON Format
```json
{
  "type": "contact",
  "arg": "Qingling Ni",
  "browser": "Chrome",
  "incognito": true
}
```


## Refactoring Details

### Command Execution Design Decision

#### The Problem
When a command is executed, it has an action field (e.g., "chrome", "cmd", "folder"). How do we convert this to our Action hashmap system?

#### Option 1: Nested Action (Rejected)
```json
{
  "type": "command",
  "command": {
    "name": "CNN Page",
    "action": "chrome",
    "arg": "https://cnn.com"
  }
}
```
This would require recursive execution - the outer action would extract the inner action and execute it.

#### Option 2: Direct Conversion (Chosen)
Convert the command directly into an action:
```json
{
  "type": "chrome",              // From command.action
  "arg": "https://cnn.com",      // From command.arg
  "command_name": "CNN Page",    // Preserve for logging
  "patch": "Web",                // From command.patch
  "flags": ""                    // From command.flags
}
```

#### Why Direct Conversion?
1. **No recursion needed** - One action, one execution
2. **Conceptual clarity** - Commands ARE actions with names
3. **Minimal refactoring** - Command.action becomes Action.type
4. **Preserves all data** - Command metadata available if needed

### Execution Flow Unification

Whether triggered by keyboard or command selection, everything becomes an Action:

```rust
// Key binding triggers action
fn handle_key(key: &str) {
    if let Some(action) = lookup_key_action(key) {
        self.execute(action);  // Direct to execute()
    }
}

// Command selection
fn execute_selected_command(&mut self) {
    let cmd = self.get_selected_command();
    let action = command_to_action(cmd);  // Convert first
    self.execute(&action);                // Same execute() path
}

// Template triggered
fn handle_template(template_name: &str) {
    if let Some(action) = lookup_template_action(template_name) {
        self.execute(action);  // Same execute() path
    }
}
```

### Template vs Server Actions

The routing decision is simple and happens in one place:

```rust
match action.action_type() {
    Some("template") => execute_locally(action),  // Needs UI
    Some("popup") => execute_locally(action),     // Is UI
    _ => send_to_server(action)                   // Everything else
}
```

Templates need:
- Command editor UI
- Grabber access to window state
- User interaction for editing

Server actions need:
- Background execution
- Process spawning
- Long-running operations

### Function Resolution

Both popup and server use identical function lookup:

```rust
pub fn execute_locally(action: &Action) -> Result<String, Error> {
    let function_name = format!("action_{}", action.action_type()?);
    
    // Try builtins first
    if let Some(builtin) = BUILTIN_FUNCTIONS.get(&function_name) {
        return builtin(action.params());
    }
    
    // Try JavaScript
    if js_context.has_function(&function_name) {
        return js_context.call_function(&function_name, action.params());
    }
    
    Err(format!("Unknown action type: {}", action.action_type()?))
}
```

### Action Definition Merging

When a command executes, its parameters merge with the action definition:

```yaml
# In config.yaml
chrome:
  type: open_url
  browser: "Google Chrome"
  incognito: false
```

```rust
// Command: "CNN" with action:"chrome" arg:"https://cnn.com"
// Becomes action:
{
  "type": "open_url",        // From chrome action definition
  "browser": "Google Chrome", // From chrome action definition
  "incognito": false,        // From chrome action definition
  "arg": "https://cnn.com",  // From command
  "command_name": "CNN"      // From command
}
```

This allows action definitions to provide defaults while commands provide specifics.

## Key Functions

### Popup Entry Point
```rust
// In src/ui/popup.rs
fn execute(&mut self, action: &Action) {
    match action.action_type.as_str() {
        // Actions that require UI interaction stay in popup
        "template" | "popup" => {
            self.execute_locally(action);
        }
        // Everything else goes to server
        _ => {
            self.send_action_to_server(action);
        }
    }
}
```

### Send to Server
```rust
// In src/ui/popup.rs
fn send_action_to_server(&self, action: &Action) {
    // Serialize action to JSON
    let action_json = serde_json::to_string(action).unwrap();
    
    // Send via command server client
    let client = CommandClient::new().unwrap();
    client.execute_action_json(&action_json);
}
```

### Server Reception
```rust
// In src/command_server.rs
fn handle_client(stream: UnixStream) {
    // Read JSON from stream
    let action_json = read_json_from_stream(&stream);
    
    // Deserialize to Action
    let action: Action = serde_json::from_str(&action_json)?;
    
    // Execute locally (on server)
    execute_locally(&action);
}
```

### Local Execution (Shared Logic)
```rust
// In src/core/unified_actions.rs (used by both popup and server)
pub fn execute_locally(action: &Action) -> Result<String, Box<dyn Error>> {
    // Build function name: "action_" + type
    let function_name = format!("action_{}", action.action_type);
    
    // Look up function in registry
    if let Some(builtin_fn) = lookup_builtin_function(&function_name) {
        // Execute builtin Rust function
        return builtin_fn(&action.params);
    }
    
    // Try JavaScript functions
    if let Some(js_fn) = lookup_javascript_function(&function_name) {
        // Execute JavaScript function with action params
        return execute_javascript_function(js_fn, &action.params);
    }
    
    // Function not found
    Err(format!("No function found for action type: {}", action.action_type).into())
}
```

### Function Registry
```rust
// In src/core/unified_actions.rs
fn lookup_builtin_function(name: &str) -> Option<ActionFunction> {
    match name {
        "action_shell" => Some(builtin_shell),
        "action_open_url" => Some(builtin_open_url),
        "action_open_app" => Some(builtin_open_app),
        "action_open_folder" => Some(builtin_open_folder),
        "action_type_text" => Some(builtin_type_text),
        _ => None,
    }
}

// JavaScript functions loaded from config.js
fn lookup_javascript_function(name: &str) -> Option<String> {
    // Check if function exists in JavaScript context
    let js_ctx = get_js_context();
    if js_ctx.has_function(name) {
        Some(name.to_string())
    } else {
        None
    }
}
```

## Execution Flow

### Key Binding → Action Execution
1. User presses key (e.g., Cmd+C)
2. Key registry finds matching Action
3. Popup calls `execute(action)`
4. Action routed based on type
5. Executed locally or sent to server

### Command → Action Execution
1. User selects command from list
2. Command's action field used to lookup Action definition
3. Action params populated with command's arg
4. Popup calls `execute(action)`
5. Action routed based on type

### Template Action Flow (Popup Only)
1. Template action triggered (type: "template")
2. Popup's `execute()` sees type is "template"
3. Calls `execute_locally(action)` in popup context
4. Template function runs grabber, opens editor, etc.
5. New command created and saved

### Background Action Flow (Server)
1. Action triggered (type: "cmd", "url", etc.)
2. Popup's `execute()` sees type is not template/popup
3. Sends action JSON to server
4. Server deserializes and calls `execute_locally(action)`
5. Server executes action in background

## JavaScript Integration

Both popup and server create identical JavaScript contexts:

```rust
// In src/js_runtime.rs
pub fn create_standard_js_context(context_name: &str) -> Result<JsContext, Error> {
    let ctx = create_base_runtime()?;
    
    // Register all builtin functions
    setup_logging(&ctx)?;
    setup_file_operations(&ctx)?;
    setup_shell_operations(&ctx)?;
    setup_grabber_functions(&ctx)?;
    
    // Load user functions from config.js
    load_user_functions(&ctx)?;
    
    // Set context identifier
    ctx.eval(&format!("const RUNTIME_CONTEXT = '{}';", context_name))?;
    
    Ok(ctx)
}
```

JavaScript functions are automatically available as `action_*` functions:
- `action_folder` (from config.js)
- `action_cmd` (from config.js)
- `action_markdown` (from config.js)
- `action_contact` (from config.js)
- etc.

## Benefits of This Design

1. **Simplicity**: Single execution path, same logic everywhere
2. **Flexibility**: Actions can have arbitrary parameters
3. **Extensibility**: New action types just need a new function
4. **Consistency**: JavaScript and Rust functions work identically
5. **Clean Separation**: UI actions in popup, background actions in server

## Migration Path

1. Update Action struct to use HashMap for params ✓ (already done)
2. Create `execute()` entry point in popup
3. Create `execute_locally()` shared function
4. Update server to receive Action JSON instead of Command
5. Migrate existing action handlers to new function signature
6. Test all action types in both contexts

## Current Implementation Notes

### Current Action Structure
```rust
// In src/core/unified_actions.rs
pub struct Action {
    #[serde(rename = "type")]
    pub action_type: String,                    
    pub description: Option<String>,            
    pub key: Option<String>,                    
    pub keystroke: Option<Keystroke>,  // Computed, not serialized
    
    #[serde(flatten)]  // All other fields stored here
    pub params: HashMap<String, JsonValue>,     
}
```

With `#[serde(flatten)]`, the Action serializes/deserializes as a single flat JSON object where `type`, `description`, and `key` are pulled out for convenience, but everything lives in the same namespace.