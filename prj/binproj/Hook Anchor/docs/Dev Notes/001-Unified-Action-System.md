# Unified Action System

## Execution Flow: Function Call Chain

### Primary Execution Pathway (Popup → Server → Actions)

**Command Selection in Popup:**
```
User selects command
    ↓
popup::execute_selected_command()
    ↓
popup::execute_selected_command_impl()
    ↓
actions::command_to_action() [Convert Command → Action]
    ↓
popup::execute(action) [ROUTER: Where should this run?]
    ↓
Route Decision:
    ├─ Template/Popup actions → actions::execute_locally(action) [in popup process]
    └─ All other actions → popup::send_action_to_server(action)
                               ↓
                           CommandClient::send_json(action_json)
                               ↓
                           command_server::handle_client() [receives JSON]
                               ↓
                           Parse as Action (new) or Command (legacy)
                               ↓
                           command_server::execute_command_with_env()
                               ↓
                           ⚠️ BYPASS: hardcoded is_launcher_command check
                               ↓
                           actions::execute_locally(action) [THE execution function]
                               ↓
                           Dispatch by action type:
                               ├─ JavaScript actions → execute_js_function("action_xxx")
                               ├─ Builtin actions → execute_xxx_action()
                               └─ Custom actions → execute_custom_action()
```

### JavaScript Function Execution
```
execute_js_function_action(function_name, params)
    ↓
js_runtime::execute_business_logic(js_code)
    ↓
JavaScript function in config.js (e.g., action_notion())
```

### Direct Local Execution (Templates/Popup Actions)
```
popup::execute(action)
    ↓
actions::execute_locally(action) [in popup process]
    ↓
For templates:
    └─ handle_template_create_named_impl(template_name)
For popup actions:
    └─ Direct UI manipulation
```

### Key Functions Summary

- **Entry Points:**
  - `popup::execute_selected_command()` - Command selection
  - `popup::execute(action)` - Router/dispatcher
  
- **Routing:**
  - `popup::execute()` - Decides WHERE to execute
  - `popup::send_action_to_server()` - Routes to background server
  
- **Server Processing:**
  - `command_server::handle_client()` - Receive and parse
  - `command_server::execute_command_with_env()` - Server-side dispatch
  
- **Actual Execution:**
  - `actions::execute_locally()` - **THE single execution function**
  - `actions::command_to_action()` - Convert Command → Action

## Proposed Solution: True Unified Action System

### Architecture Overview

Create a truly unified action execution system where **every command** flows through a single, consistent pipeline that supports both JavaScript and Rust function execution with flexible parameter passing.

### Actions Module Architecture

#### Module Interface
The `actions` module provides a minimal, clean interface with only two entry points:

**Public API (for entire crate):**
```rust
pub(crate) fn execute_locally(action: &Action) -> Result<String>
pub(crate) fn command_to_action(cmd: &Command) -> Action
```

#### Execution Flow for Server Actions
```
popup::execute()
  → execution_server::send_for_execution()
    → [socket communication]
    → execution_server::handle_client()
      → actions::execute_locally()  // ALWAYS!
```

#### Execution Flow for Local Actions (Templates/Popup)
```
popup::execute()
  → actions::execute_locally()  // Direct call for UI actions
```

#### Internal Dispatch Within execute_locally()
```
actions::execute_locally()
  → match action.type:
      "template"    → execute_template_action()     [Rust builtin]
      "open_url"    → execute_open_url_action()     [Rust builtin]
      "shell"       → execute_shell_action()        [Rust builtin]
      "open_app"    → execute_open_app_action()     [Rust builtin]
      "open_folder" → execute_open_folder_action()  [Rust builtin]
      _             → execute_js_function_action("action_<type>")
                      → js_runtime::execute_business_logic()
                        → config.js::action_<type>()
```

#### Complete Call Tree
```
User selects command
  → popup::execute_selected_command()
    → actions::command_to_action()
    → popup::execute()
      → Route decision:
          ├─ UI actions (template/popup):
          │    → actions::execute_locally()
          │
          └─ Server actions (everything else):
               → execution_server::send_for_execution()
                 → [private: serialize, socket send]
                   → execution_server::handle_client()
                     → actions::execute_locally()
```

### Core Design Principles

1. **Actions as HashMaps**: Actions are represented as `HashMap<String, JsonValue>` with a required `type` field
2. **Unified Execution**: Same execution logic in both popup and server contexts
3. **Simple Routing**: Popup routes based on action type (local vs server execution)
4. **Function Lookup**: Functions are resolved by prefixing "action_" to the type
5. **Commands are Actions**: Commands are just named actions stored in commands.txt
6. **Single Execution Path**: ALL commands go through `unified_actions.rs`

### Action Structure (Action Wrapper Design)

```rust
pub struct Action {
    #[serde(rename = "type")]
    pub action_type: String,
    
    #[serde(flatten)]
    pub params: HashMap<String, JsonValue>,
}

impl Action {
    pub fn action_type(&self) -> Option<&str> {
        Some(&self.action_type)
    }
    
    pub fn get(&self, key: &str) -> Option<&JsonValue> {
        self.params.get(key)
    }
    
    pub fn params(&self) -> &HashMap<String, JsonValue> {
        &self.params
    }
}
```

### Command to Action Conversion

```rust
fn command_to_action(cmd: &Command) -> Action {
    let mut params = HashMap::new();
    params.insert("arg".to_string(), JsonValue::String(cmd.arg.clone()));
    params.insert("command_name".to_string(), JsonValue::String(cmd.command.clone()));
    params.insert("patch".to_string(), JsonValue::String(cmd.patch.clone()));
    params.insert("flags".to_string(), JsonValue::String(cmd.flags.clone()));
    
    Action {
        action_type: cmd.action.clone(),
        params,
    }
}
```

### Unified Function Registry

```rust
pub fn execute_action(action: &Action) -> Result<String> {
    let function_name = format!("action_{}", action.action_type);
    
    // Try builtin functions first
    if let Some(builtin_fn) = BUILTIN_FUNCTIONS.get(&function_name) {
        return builtin_fn(&action.params);
    }
    
    // Try JavaScript functions
    if js_context.has_function(&function_name) {
        return js_context.call_function(&function_name, &action.params);
    }
    
    Err(format!("Unknown action type: {}", action.action_type))
}
```

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

### Popup Entry Point

```rust
// In src/ui/popup.rs
fn execute(&mut self, action: &Action) {
    match action.action_type() {
        // Actions that require UI interaction stay in popup
        Some("template") | Some("popup") => {
            self.execute_locally(action);
        }
        // Everything else goes to server
        _ => {
            self.send_action_to_server(action);
        }
    }
}
```

### Uniform Function Naming Convention

Every action type maps directly to a function with the prefix `action_`:
- `type: "notion"` → calls function `action_notion`
- `type: "slack"` → calls function `action_slack`
- `type: "cmd"` → calls function `action_cmd`
- `type: "contact"` → calls function `action_contact`

### JavaScript Functions in config.js

```javascript
// ~/.config/hookanchor/config.js
function action_notion(params) {
    const url = params.arg;
    shell(`open "${url}"`);
    return "Notion page opened";
}

function action_slack(params) {
    const url = params.url || params.arg;
    const slackUrl = convertToSlackUrl(url);
    shell(`open "${slackUrl}"`);
    return `Opened in Slack: ${slackUrl}`;
}

function action_1pass(params) {
    const item = params.arg;
    // 1Password logic here
    return "1Password opened";
}

function action_contact(params) {
    const name = params.arg;
    const browser = params.browser || "Google Chrome";
    // Contact logic here
    return `Contact opened: ${name}`;
}
```

### Action JSON Format Examples

```json
{
  "type": "contact",
  "arg": "Qingling Ni",
  "browser": "Chrome",
  "incognito": true
}

{
  "type": "notion",
  "arg": "https://notion.so/page-url",
  "command_name": "CVPROJ",
  "patch": "Fireball"
}

{
  "type": "cmd",
  "arg": "echo hello",
  "command_name": "Test Command"
}
```

## Implementation Plan

### Phase 1: Remove All Bypass Routes (IMMEDIATE)

**Remove hardcoded routing from `command_server.rs`:**
```rust
// REMOVE this entirely:
let is_launcher_command = matches!(...);

// ALL commands go through unified_actions.rs:
execute_action(&action)  // No exceptions
```

**Remove hardcoded routing from `commands.rs`:**
```rust
// REMOVE uses_client_environment() function entirely
// Let unified_actions decide execution environment
```

### Phase 2: Verify Single Execution Path

After removing bypasses, **every command** must flow through:
```
Command → unified_actions.rs → execute_action() → action_* function
```

### Phase 3: Complete Action Wrapper Integration

1. **Implement Action struct** with HashMap params
2. **Create command_to_action conversion**
3. **Move all JavaScript to config.js**
4. **Implement unified function registry**
5. **Remove old execute_XXX_action functions**

## Implementation Steps

### Immediate Fix (Day 1)
1. **Remove hardcoded routing** from `command_server.rs` and `commands.rs`
2. **Force all commands through `unified_actions.rs`**
3. **Test that notion/slack/1pass commands work** through unified system
4. **Verify no commands are being intercepted**

### Complete Unification (Day 2-3)
1. **Implement Action struct** with HashMap params
2. **Create command_to_action conversion**
3. **Move all JavaScript to config.js**
4. **Implement unified function registry**
5. **Remove old execute_XXX_action functions**

### Testing & Cleanup (Day 4)
1. **Test every action type** works through unified system
2. **Remove all old routing code**
3. **Verify performance is acceptable**
4. **Update documentation**

## Success Criteria

1. **Single execution path**: ALL commands go through `unified_actions.rs`
2. **No hardcoded routing**: No action type lists anywhere except unified registry
3. **User extensibility**: Users can add `action_foo` functions in config.js
4. **Zero regressions**: All existing functionality works identically
5. **Architectural integrity**: System matches design documents
6. **Flexible parameters**: Actions can have arbitrary parameters via HashMap

## Benefits of True Unification

1. **Architectural integrity**: System matches design
2. **Single source of truth**: One place handles all commands  
3. **Easy debugging**: Can trace any command through same flow
4. **User extensibility**: Add new action types without touching Rust
5. **Maintainability**: No scattered hardcoded routing logic
6. **Testing**: One execution path to test comprehensively
7. **Flexibility**: Actions can have arbitrary parameters
8. **Consistency**: JavaScript and Rust functions work identically

## Timeline

- **Hour 1**: Remove hardcoded bypasses from command_server.rs and commands.rs
- **Hour 2**: Test that all commands flow through unified_actions.rs  
- **Day 1**: Verify notion, slack, 1pass work through unified system
- **Day 2-3**: Complete Action struct and config.js migration
- **Day 4**: Final testing and cleanup

---

## Current Crisis: Multiple Execution Pathways (PROBLEMS)

**CRITICAL PROBLEM**: Despite days of architectural work to create a "unified" action system, the current implementation has **THREE SEPARATE EXECUTION PATHWAYS** that completely circumvent the unified system:

### Problem 1: Hardcoded Route Bypassing in `command_server.rs`
```rust
// This hardcoded list routes commands to "launcher system" instead of unified_actions
let is_launcher_command = matches!(
    command.action.as_str(),
    "app" | "url" | "cmd" | "chrome" | "safari" | "brave" | "firefox" | "work" | 
    "notion" | "obs" | "obs_url" | "1pass" | "anchor" | "folder" | 
    "doc" | "markdown" | "text" | "slack" | "contact" | "alias" |
    "activate_anchor" | "tmux_activate"
);
```

### Problem 2: Direct Client Execution Bypass in `commands.rs`
```rust
// Another hardcoded list that bypasses unified system
fn uses_client_environment(action: &str) -> bool {
    match action {
        "app" | "url" | "folder" | "doc" | "chrome" | "safari" | "brave" | "firefox" | 
        "work" | "notion" | "obs_url" | "1pass" | "contact" | "open_with" => true,
        // ...
    }
}
```

### Problem 3: The Actual Unified System That Gets Bypassed
```rust
// This is supposed to be THE ONLY execution path, but gets bypassed!
pub fn execute_action(action: &Action) -> Result<String> {
    match action.action_type() {
        "folder" => execute_js_function_action("action_folder", &expanded_params),
        "cmd" => execute_js_function_action("action_cmd", &expanded_params),
        "notion" => execute_js_function_action("action_notion", &expanded_params),
        // ... THIS IS WHERE ALL COMMANDS SHOULD GO
    }
}
```

## Architectural Failure Analysis

The unified action system was designed as a **single entry point** for all command execution:

```
All Commands → unified_actions.rs → execute_action() → action_* functions
```

But the actual implementation has **three competing pathways**:

```
Commands → command_server.rs (launcher bypass) → ???
Commands → commands.rs (client bypass) → ???  
Commands → unified_actions.rs → execute_action() (intended path, rarely used)
```

**Result**: The "unified" system handles almost no commands because they're all intercepted before reaching it.

## Root Cause: Incremental Development Without Architectural Discipline

1. **Original system**: Had hardcoded routing in multiple places
2. **Unified system added**: Created alongside old system instead of replacing it
3. **Old routing preserved**: Hardcoded lists still intercept commands
4. **Testing failure**: New system never actually tested because old routing still active

## Current Crisis Resolution

The immediate crisis is that **the unified system doesn't actually unify anything** because old routing mechanisms still intercept commands. 

**Priority 1**: Remove ALL hardcoded routing bypasses so the unified system actually gets used.

**Priority 2**: Complete the unified architecture so it works correctly for all action types.

This refactoring is not optional - the current system is a broken architecture that wastes development time and creates maintenance nightmares.

---

## Refactoring Changes List

### 1. Rename Functions for Clarity
- **Rename `unified_actions::execute_action()` → `actions::execute_locally()`**
  - "execute_locally" means "execute in this process" (popup or server)
  - Remove redundant "action" from name
  - Make it `pub(crate)` for crate-only visibility
  
- **Rename module `unified_actions` → `actions`**
  - Simpler, cleaner name
  - It's already unified by design

### 2. Update Function Signatures
```rust
// In src/core/actions.rs (renamed from unified_actions.rs)
pub(crate) fn execute_locally(action: &Action) -> Result<String> {
    // THE single execution function
    // Works in both popup and server processes
}

// In src/ui/popup.rs
fn execute(&mut self, action: &Action) {
    // Router function - decides WHERE to execute
    match action.action_type() {
        "template" | "popup" => {
            actions::execute_locally(action)  // Execute in popup
        }
        _ => {
            self.send_action_to_server(action)  // Send to server
        }
    }
}
```

### 3. Remove Bypass Routes
- **Delete hardcoded `is_launcher_command` check in `command_server.rs`**
- **Delete `uses_client_environment()` function in `commands.rs`**
- **Route ALL commands through `actions::execute_locally()`**

### 4. Visibility Control
**Rust Visibility Options:**
- `pub(crate)` - Visible anywhere in the crate (best option)
- `pub(super)` - Visible to parent module only
- `pub(in path)` - Visible in specific module path

**Recommended:** Use `pub(crate)` for `execute_locally()`:
```rust
pub(crate) fn execute_locally(action: &Action) -> Result<String>
```
This makes it:
- ✅ Callable from `popup.rs` 
- ✅ Callable from `command_server.rs`
- ❌ Not part of public API
- ❌ Not callable from outside the crate

### 5. Update All Call Sites
- Change `unified_actions::execute_action()` → `actions::execute_locally()`
- Change `unified_actions::` → `actions::`
- Update imports from `use crate::core::unified_actions` → `use crate::core::actions`

### 6. Consolidate Execution Logic
- Move any execution logic from `popup::execute_locally()` into `actions::execute_locally()`
- Make popup's version just call through to `actions::execute_locally()`
- Ensure server always calls `actions::execute_locally()` directly

### 7. Simplify Action Dispatch Logic
**Current (Confusing):**
- Some JavaScript actions hardcoded in match statement
- Some JavaScript actions go through "custom action" handler
- Redundant and inconsistent

**New (Simple):**
```rust
pub(crate) fn execute_locally(action: &Action) -> Result<String> {
    match action.action_type.as_str() {
        // Short list of Rust builtin actions
        "template" => execute_template_action(action, params),
        "open_url" => execute_open_url_action(params),
        "shell" => execute_shell_action(params),
        "open_app" => execute_open_app_action(params),
        "open_folder" => execute_open_folder_action(params),
        
        // EVERYTHING else is JavaScript with action_ prefix
        _ => {
            let function_name = format!("action_{}", action.action_type);
            execute_js_function_action(&function_name, params)
        }
    }
}
```

**Benefits:**
- Clear separation: Rust builtins vs JavaScript functions
- No redundant hardcoding of JavaScript actions
- Any new action type automatically works if `action_<type>` exists in JavaScript
- Remove confusing `execute_custom_action()` function entirely

### 8. Fix Visibility Scoping
**Make these `pub(crate)` (crate-only visibility):**
```rust
// Only these should be accessible from outside the module
pub(crate) fn execute_locally(action: &Action) -> Result<String>
pub(crate) fn command_to_action(cmd: &Command) -> Action
```

**Keep these `pub` (public API):**
```rust
pub struct Action { ... }  // Needed everywhere
impl Action {
    pub fn action_type(&self) -> &str { ... }
    pub fn get_string(&self, key: &str) -> Option<&str> { ... }
    // other helper methods
}
```

**Make these private (module-internal only):**
```rust
fn execute_template_action(...)  // No pub, private to module
fn execute_open_url_action(...)  // No pub, private to module
fn execute_shell_action(...)     // No pub, private to module
fn execute_js_function_action(...) // No pub, private to module
// ... all other execution helpers
```

**Result:**
- Outside code can only call `execute_locally()` and `command_to_action()`
- Internal execution functions are hidden implementation details
- Clean, minimal public interface

### 9. Encapsulate Server Communication
**Rename `command_server` → `execution_server`:**
- "Command" is a data structure, not what we're executing
- We're executing Actions, so "execution_server" is clearer

**Move all server communication into execution_server module:**
```rust
// In execution_server.rs
pub(crate) fn send_for_execution(action: &Action) -> Result<String> {
    // All private implementation:
    // - Create client
    // - Serialize to JSON  
    // - Send via socket
    // - Handle response
}
```

**Update popup.rs to use clean API:**
```rust
fn execute(&mut self, action: &Action) {
    match action.action_type() {
        "template" | "popup" => {
            actions::execute_locally(action)  // Local execution
        }
        _ => {
            execution_server::send_for_execution(action)  // Server execution
        }
    }
}
```

**Benefits:**
- Popup doesn't know about sockets or serialization
- Clean separation of concerns
- Single entry point for server communication
- All server internals are private

### 10. Test Coverage
- Verify all action types work through new path
- Test both popup-local and server execution
- Ensure no regressions in functionality