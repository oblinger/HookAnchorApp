# PRD: Uniform Action System Refactoring

## Overview
Refactor the HookAnchor action system to use a uniform function naming convention and move JavaScript functions to a separate config.js file for better maintainability and extensibility.

## Current State

### Problems
1. **Inconsistent action handling**: Different action types are handled through different code paths:
   - Some use inline Rust functions (e.g., `execute_open_url_action`)
   - Some reference JavaScript functions with `action_` prefix
   - Some use builtin functions
   - Type "javascript" actions reference separate functions

2. **Poor JavaScript ergonomics**: JavaScript functions in YAML require extensive escaping and lack syntax highlighting

3. **Limited extensibility**: Users cannot easily add new action types without modifying Rust code

4. **Confusing type system**: Actions like "slack", "1password" are type:"javascript" but conceptually they are their own types

## Proposed Solution

### 1. Uniform Function Naming Convention
Every action type maps directly to a function with the prefix `action_`:
- `type: "open_url"` → calls function `action_open_url`
- `type: "slack"` → calls function `action_slack`
- `type: "cmd"` → calls function `action_cmd`
- `type: "template"` → calls function `action_template`

### 2. Unified Function Lookup
Replace the current `execute_action` match statement with:
```rust
pub fn execute_action(action: &Action, context: &ActionContext) -> Result<String> {
    let function_name = format!("action_{}", action.action_type);
    
    // Create environment with both builtins and JavaScript functions
    let mut env = create_unified_environment(context)?;
    
    // Execute the function (builtin or JavaScript)
    env.call_function(&function_name, &action.params)
}
```

### 3. Move JavaScript to config.js
Create a new file `~/.config/hookanchor/config.js`:
```javascript
// Action functions for different types
function action_slack(params, context) {
    const url = params.url || context.selectedCommand?.url;
    const slackUrl = convertToSlackUrl(url);
    shell(`open "${slackUrl}"`);
    return `Opened in Slack: ${slackUrl}`;
}

function action_1password(params, context) {
    const item = params.item || context.arg;
    const vault = params.vault || getDefault1PasswordVault();
    const url = `onepassword://open/${vault}/${item}`;
    shell(`open "${url}"`);
    return `Opened in 1Password: ${item}`;
}

function action_obsidian(params, context) {
    const file = params.file || context.arg;
    const vault = getObsidianVault();
    const url = `obsidian://open?vault=${vault}&file=${encodeURIComponent(file)}`;
    shell(`open "${url}"`);
    return `Opened in Obsidian: ${file}`;
}

// Helper functions
function getObsidianVault() {
    return CONFIG.obsidianVault || "Notes";
}

function convertToSlackUrl(webUrl) {
    // Convert https://app.slack.com/... to slack://...
    // Implementation here
}
```

### 4. Convert Existing Handlers to action_ Functions

Current inline Rust handlers become builtin functions:
- `execute_open_url_action` → builtin `action_open_url`
- `execute_template_action` → builtin `action_template`
- `execute_popup_action` → builtin `action_popup`
- `execute_shell_action` → builtin `action_shell`

### 5. Simplified Config.yaml

Actions in config.yaml become much simpler:
```yaml
actions:
  # Simple actions just specify their type
  slack:
    type: slack
    description: "Open Slack URLs"
    
  1password:
    type: 1password
    description: "Open 1Password items"
    
  obsidian:
    type: obsidian
    description: "Open files in Obsidian"
    
  # Template actions still work the same
  sub_anchor:
    type: template
    description: "Create sub-anchor"
    template: "{{previous_folder}}/{{input}}/{{input}}.md"
    
  # Can still pass parameters
  chrome:
    type: open_url
    browser: "Google Chrome"
    description: "Open URL in Chrome"
```

## Benefits

1. **Uniform system**: One way to handle all action types
2. **User extensibility**: Users can add new types by defining `action_XXX` functions in config.js
3. **Better JavaScript DX**: Proper syntax highlighting, no escaping, proper debugging
4. **Override capability**: Users can override builtin `action_XXX` functions with JavaScript versions
5. **Simpler mental model**: Type name directly maps to function name
6. **Self-documenting**: Function names clearly indicate what they handle

### Additional Simplification Benefits

7. **Single execution function**: Replace multiple wrappers with one `execute_command(&Command)`
8. **Cleaner code paths**: Remove unnecessary indirection layers
9. **Easier maintenance**: Fewer functions to update when making changes
10. **Better testability**: Test one function instead of multiple entry points
11. **Consistent behavior**: No risk of CLI/popup/server diverging in behavior
12. **Smaller Command struct**: Remove redundant `full_line` field
13. **Less memory usage**: No duplicate storage of command information
14. **Cleaner serialization**: Smaller JSON payloads over network
15. **Single source of truth**: `to_new_format()` defines the canonical representation

## Implementation Steps

### Phase 1: Infrastructure
1. Create unified function registry for builtins and JavaScript
2. Implement `execute_function()` that looks up functions by name
3. Add config.js loading support
4. **Merge internal server functions**:
   - Combine `handle_client` + `execute_command_with_env` → `server_dispatch`
5. **Merge execution functions**:
   - Combine `launcher::launch` + `execute_action` → `unified_execute`

### Phase 2: Migration
1. Convert all action handlers to `action_XXX` functions
2. Register all builtin `action_XXX` functions in registry
3. Move JavaScript from config.yaml to config.js
4. Update JavaScript functions to uniform `action_XXX(context)` signature
5. **Remove intermediate layers**:
   - Remove old `launcher::launch` (replaced by `unified_execute`)
   - Remove old `execute_action` (merged into `unified_execute`)
   - Remove `handle_client` and `execute_command_with_env` (merged)

### Phase 3: Cleanup
1. Remove old execute_XXX_action handler functions
2. Remove functions section from config.yaml
3. Update default config generation
4. **Remove `full_line` field from Command struct**
5. **Simplify call sites**:
   - Update all callers to use simplified chain
   - Remove unnecessary wrappers and indirection

## Migration Guide

### For Users
1. Backup existing config.yaml
2. Run migration script: `ha --migrate-config`
3. Review generated config.js
4. Custom functions: rename from `action_cmd` to `action_XXX` format

### For Developers
- New action types: just add `action_XXX` function in config.js
- Override builtins: define `action_XXX` in config.js to override

## Testing Plan

1. Verify all existing actions work after refactoring
2. Test user-defined action types
3. Test builtin override capability
4. Performance testing for function lookup
5. Test config.js syntax error handling

## Risks and Mitigations

- **Risk**: Breaking existing user configs
  - **Mitigation**: Provide migration script and backwards compatibility mode

- **Risk**: JavaScript execution errors harder to debug
  - **Mitigation**: Better error messages with line numbers from config.js

- **Risk**: Performance impact of unified lookup
  - **Mitigation**: Cache function references after first lookup

## Timeline

- Day 1: Infrastructure and builtin conversion
- Day 2: JavaScript migration and config.js support
- Day 3: Testing and documentation
- Day 4: Release with migration guide

## Success Criteria

1. All existing functionality works without regression
2. Users can add new action types without touching Rust code
3. JavaScript functions have proper syntax highlighting in editors
4. System is simpler to understand and extend
5. Performance is not degraded

## Unified Command Execution Chain

### Current Command Flow Analysis

The current system has multiple layers with some redundancy:

```
Multiple Entry Points (10+ locations)
    ↓
execute_via_server (necessary - shared entry point)
    ↓
[Server: handle_client] (internal only)
    ↓
[Server: execute_command_with_env] (internal only, 1:1 with handle_client)
    ↓
launcher::launch (mostly just parsing)
    ↓
execute_action (actual execution)
```

### Proposed Simplified Command Flow

Merge redundant layers for a cleaner architecture:

```
Multiple Entry Points (10+ locations)
    ↓
execute_via_server (kept - genuinely needed)
    ↓
server_dispatch (merged handle_client + execute_command_with_env)
    ↓
unified_execute (merged launcher::launch + execute_action)
```

### Before/After Comparison

| Layer | Before (6 functions) | After (3 functions) | Benefit |
|-------|---------------------|---------------------|---------|
| Entry | `execute_via_server` | `execute_via_server` | Kept - needed by many callers |
| Server Internal 1 | `handle_client` | `server_dispatch` | Merged - removes 1:1 redundancy |
| Server Internal 2 | `execute_command_with_env` | ↓ | Merged into server_dispatch |
| Parsing | `launcher::launch` | `unified_execute` | Merged - removes parsing overhead |
| Execution | `execute_action` | ↓ | Merged into unified_execute |
| Registry | Multiple type handlers | `execute_function` | Single registry for all functions |

**Result: 50% reduction in layers (6 → 3 functions)**

## Final Proposed API

This section shows the complete final API after all refactoring is complete.

### Core Data Structures
```rust
// Command - what to execute
struct Command {
    patch: String,      // Group/category
    command: String,    // Display name
    action: String,     // Action type (e.g., "1pass", "app", "cmd")
    arg: String,        // Primary argument
    flags: String,      // Modifiers (e.g., "G" for synchronous)
    // full_line removed - reconstructed when saving
}

// Parameters - everything else
type Params = HashMap<String, String>;
```

### Main Execution Chain (3 functions total)
```rust
// 1. Entry point - handles server connection and retries
pub fn execute_via_server(command: &Command) -> () {
    // Never returns errors - handles all retries internally
    // If server can't start after N attempts:
    //   - Shows error dialog to user
    //   - Exits popup if in GUI mode
    // Otherwise guarantees command delivery
}

// 2. Server dispatch - handles sync/async
fn server_dispatch(command: Command, params: Params) -> () {
    if command.flags.contains("G") {
        // Synchronous execution for shell_sync
        let result = execute(command, params);
        send_response(result);
    } else {
        // Async execution (most commands)
        spawn_async(|| execute(command, params));
        send_response("ACK");
    }
}

// 3. Execution - looks up and runs the action
fn execute(command: Command, mut params: Params) -> Option<String> {
    // Add command.arg to params
    params.insert("arg", command.arg);
    
    // Look up action and execute
    let function_name = format!("action_{}", command.action);
    match execute_function(&function_name, &params) {
        Ok(result) => Some(result),
        Err(e) => {
            log_error(&e);
            None
        }
    }
}
```

### Function Registry
```rust
// All functions have the same signature
type ActionFunction = fn(&HashMap<String, String>) -> Result<String>;

fn execute_function(name: &str, params: &Params) -> Result<String> {
    // Check builtin functions
    if let Some(func) = BUILTIN_FUNCTIONS.get(name) {
        return func(params);
    }
    
    // Check JavaScript functions
    if let Some(js_code) = JS_FUNCTIONS.get(name) {
        return execute_javascript(js_code, params);
    }
    
    Err(format!("Function {} not found", name))
}
```

### Action Function Examples
```rust
// Builtin Rust functions
fn action_open_url(params: &Params) -> Result<String> {
    let url = params.get("arg").ok_or("Missing URL")?;
    open_url(url)?;
    Ok("Opened URL")
}

fn action_shell(params: &Params) -> Result<String> {
    let cmd = params.get("arg").ok_or("Missing command")?;
    shell_execute(cmd)?;
    Ok("Executed")
}
```

```javascript
// JavaScript functions (from config.js)
function action_1pass(params) {
    const query = params.arg;
    shell_sync(`osascript -e '...'`);
    return "OK";
}

function action_slack(params) {
    const channel = params.channel || params.arg;
    open_url(`slack://channel?id=${channel}`);
    return "OK";
}
```

### Environment
- **Working Directory**: Always `~/.config/hookanchor` (set at server startup)
- **Environment Variables**: Inherited from server process
- **No passing of environment data** - it's all implicit

---

## Current State (for reference)

### Command Flow Signatures - SIMPLIFIED

#### 1. Entry Points - REMAIN MULTIPLE BUT SIMPLIFIED
```rust
// These remain as-is since they're called from many places:
// - CLI commands
// - Popup UI actions  
// - JavaScript runtime
// - URL handlers

// CURRENT (incorrect):
execute_via_server(&Command) -> Result<CommandResponse>
// Callers check for errors and try to handle them

// PROPOSED (correct):
execute_via_server(&Command) -> ()
// This function NEVER fails from caller's perspective
// It handles ALL retry logic internally:
// 1. Try to send command to server
// 2. If server is down, restart it (with retries)
// 3. If server can't be restarted after N attempts:
//    - Show error dialog to user
//    - Exit popup if in GUI mode
// 4. Otherwise, command is guaranteed delivered

// Benefits:
// - Callers become simpler - just fire and forget
// - No error handling code scattered throughout
// - Consistent server restart behavior
// - User sees errors only when truly unrecoverable
```

#### 2. Command Structure - SIMPLIFIED
```rust
// Current (with redundant field):
struct Command {
    patch: String,      // Group/category
    command: String,    // Display name
    action: String,     // Action type (e.g., "1pass", "app", "cmd")
    arg: String,        // Primary argument
    flags: String,      // Modifiers (e.g., "G" for GUI/synchronous)
    full_line: String,  // REDUNDANT - reconstructed from fields when saving
}

// Proposed (cleaner):
struct Command {
    patch: String,      // Group/category
    command: String,    // Display name
    action: String,     // Action type (e.g., "1pass", "app", "cmd")
    arg: String,        // Primary argument
    flags: String,      // Modifiers (e.g., "G" for GUI/synchronous)
}
// to_new_format() method reconstructs the line for saving
```

#### 3. Network Transport (Client → Server) - SIMPLIFIED
```json
// Proposed (without redundant full_line):
{
  "patch": "1Pass",
  "command": "SimpleNote 1Pass",
  "action": "1pass",
  "arg": "SimpleNote",
  "flags": ""
}
```

#### 4. Server Dispatch - MERGED & SIMPLIFIED
```rust
// Current: Two separate functions + separate env/dir handling
fn handle_client(stream, inherited_env: HashMap, base_working_dir: PathBuf) {
    let command = parse_from_stream(stream);
    let result = execute_command_with_env(command, inherited_env, base_working_dir);
    send_response(stream, result);
}

fn execute_command_with_env(command, env, dir) -> CommandResponse {
    launcher::launch(&format!("{} {}", command.action, command.arg))
}

// Proposed: Single dispatch with unified parameters
fn server_dispatch(command: Command, params: HashMap<String, String>) -> CommandResponse {
    // params contains only action-specific parameters
    // Environment variables are inherited automatically from server process
    // Working directory is always ~/.config/hookanchor
    
    if command.flags.contains("G") {
        // Synchronous: Wait for result (used by shell_sync)
        let result = unified_execute(command, params);
        CommandResponse::Success(result)  // Return actual result
    } else {
        // Async: Fire and forget (most commands)
        spawn_async(|| {
            unified_execute(command, params);  // Result is discarded
        });
        CommandResponse::Success("ACK")  // Just acknowledge receipt
    }
}

// At server startup, just set working directory once:
std::env::set_current_dir(config_dir())?;  // ~/.config/hookanchor
// That's it! No environment variable capture needed
```

#### 5. Unified Execution - SIMPLIFIED
```rust
// Current: Complex multi-layer execution
pub fn launch(command_line: &str) -> Result<String> {
    let (action_name, args) = parse_command(command_line);
    let action = get_action(action_name)?;
    let context = ActionContext { ... };  // Complex context object
    execute_action(&action, &context)
}

// PROPOSED: Single unified execution function
pub fn unified_execute(command: &Command, params: HashMap<String, String>) -> Option<String> {
    // This replaces both launcher::launch AND execute_action
    
    // Look up action definition
    let action = match get_action(&command.action) {
        Some(a) => a,
        None => {
            log_error(&format!("Unknown action: {}", command.action));
            return None;
        }
    };
    
    // Add command.arg to params
    let mut final_params = params;
    final_params.insert("arg", command.arg.clone());
    
    // Get the function name
    let function_name = format!("action_{}", action.action_type);
    
    // Execute directly via unified registry
    match execute_function(&function_name, &final_params) {
        Ok(result) => Some(result),  // For sync commands
        Err(e) => {
            log_error(&format!("Action failed: {}", e));
            None
        }
    }
}

// No more execute_action as separate function - merged into unified_execute!

// Execution environment (super simple!):
// - Working directory: ALWAYS ~/.config/hookanchor (set once at server startup)
// - Environment vars: Inherited automatically from server process
// - Parameters: Only action-specific data passed in HashMap
```

#### 6. Unified Function Registry
```rust
// Single registry for all action functions (builtin + JavaScript)
pub fn execute_function(
    function_name: &str, 
    params: &HashMap<String, String>
) -> Result<String> {
    // Check builtin functions first
    if let Some(builtin_fn) = BUILTIN_FUNCTIONS.get(function_name) {
        return builtin_fn(params);
    }
    
    // Check JavaScript functions from config.js
    if let Some(js_code) = JS_FUNCTIONS.get(function_name) {
        return execute_javascript(js_code, params);
    }
    
    // Error if function not found
    Err(format!("Function '{}' not found", function_name))
}

// Register builtin functions - all have same signature!
const BUILTIN_FUNCTIONS: HashMap<&str, fn(&HashMap<String, String>) -> Result<String>> = {
    "action_open_url" => builtin_open_url,
    "action_shell" => builtin_shell,
    "action_template" => builtin_template,
    "action_open_app" => builtin_open_app,
    // ...
};

// JavaScript execution also takes same params
fn execute_javascript(code: &str, params: &HashMap<String, String>) -> Result<String> {
    // Set up JS context with params as variables
    // Execute the JavaScript code
}
```

#### 7. Function Implementations
```rust
// Builtin functions (Rust) - just take parameters!
fn builtin_open_url(params: &HashMap<String, String>) -> Result<String> {
    let url = params.get("arg").ok_or("Missing URL")?;
    open_url(&url)?;
    Ok(format!("Opened URL: {}", url))
}

fn builtin_shell(params: &HashMap<String, String>) -> Result<String> {
    let cmd = params.get("arg").ok_or("Missing command")?;
    shell_execute(&cmd)?;
    Ok("Command executed")
}

// JavaScript functions (from config.js) - same signature!
function action_1pass(params) {
    const query = params.arg || params.query;
    shell_sync(`osascript -e '...'`);
    return "1Password action completed";
}

function action_cmd(params) {
    const cmd = params.arg;
    if (cmd.startsWith('W ')) {
        // Windowed execution
    }
    return "Command executed";
}

// Both builtin and JavaScript functions have the same interface:
// (params: HashMap<String, String>) -> Result<String>
```

### Unified Parameter System

The entire system is radically simplified to just two data structures:

```rust
// Throughout the entire system, we only need:
struct Command { ... }  // What to do
HashMap<String, String> // Action-specific parameters

// That's it! No environment object, no working directory passing

// Execution environment is fixed:
// - Working directory: ALWAYS ~/.config/hookanchor
// - Environment vars: Inherited from server process

// Parameters are purely action-specific:
"arg"         -> Primary argument
"input"       -> User input
"browser"     -> Specific browser to use
"vault"       -> Obsidian vault name
// ... any other action-specific parameters
```

This means:
- **No working directory management** - always ~/.config/hookanchor
- **No environment variable passing** - inherited automatically
- **No separate Env object** - not needed
- **Single data structure (HashMap)** for action parameters only
- **Predictable execution** - same environment every time
- **Minimal data passing** - only what's specific to the action

### Simplified API Benefits

The new simplified `execute_action` API provides major improvements:

#### Before (Complex ActionContext):
```rust
// Overly complex context object
let mut context = ActionContext::new(input);
context = context.with_arg(arg);
context = context.with_selected(selected_cmd);
context = context.with_previous(previous_cmd);
for (k, v) in extra_params {
    context.add_variable(k, v);
}
execute_action(action, &context)
```

#### After (Simple Parameters):
```rust
// Just pass what you need
let mut variables = HashMap::new();
variables.insert("input", input);
// Add any other needed variables
execute_action(action, Some(arg), Some(variables))
```

**Benefits:**
1. **Simpler interface** - Just 3 parameters instead of complex object
2. **No unnecessary abstraction** - ActionContext was over-engineered
3. **Easier to test** - Just pass a HashMap, no complex object setup
4. **More flexible** - Callers decide exactly what to pass
5. **Less memory** - No storage of rarely-used fields
6. **Clearer intent** - Parameters show exactly what's being passed

### System Invariants

1. **Command Object Consistency**: A `Command` struct is used throughout the system

2. **Single Execution Path**: All commands go through `execute_via_server` → `server_dispatch` → `unified_execute`

3. **Fixed Working Directory**: All commands execute in `~/.config/hookanchor` - no directory passing needed

4. **Uniform Function Naming**: Every action maps to `action_XXX` function

5. **Single Function Registry**: All functions (builtin and JavaScript) in one registry

6. **Synchronous Flag ("G")**: Commands with "G" flag execute synchronously, others async

7. **Simple Parameter Passing**: Actions receive `arg` + `variables` HashMap, not complex contexts

8. **Absolute Paths**: Commands should use absolute paths for file operations since working directory is fixed

### Known Exceptions

1. **Popup Commands**: Popup-type actions (action: "popup") are handled specially in the popup UI code and don't go through the server

2. **Direct CLI Flags**: Some CLI operations bypass the command system entirely:
   - `--grab`: Direct grabber invocation
   - `--rescan`: Direct filesystem scan
   - `--restart`: Server management
   - `--test-grabber`: Testing functionality

3. **Alias Commands**: Type "alias" has special handling to recursively resolve to target commands

4. **Template Actions**: Type "template" creates intermediate commands rather than executing directly

5. **Shell Commands**: When action is "shell", the arg becomes the entire shell command to execute

### Unified System Benefits

- **Single code path**: All commands follow the same execution chain
- **Consistent parameter handling**: Same expansion rules everywhere  
- **Easy extensibility**: Add new action types by adding action_XXX functions
- **Clear debugging**: Can trace any command through the same flow
- **Testability**: Single execution path makes testing comprehensive
- **Simplified entry points**: One `execute_command()` function instead of multiple wrappers
- **Reduced code duplication**: No need for separate CLI/popup/server execution paths

## Future Enhancements

1. TypeScript support for config.ts with type checking
2. Action composition: actions calling other actions
3. Action marketplace: share custom actions
4. Visual action builder UI