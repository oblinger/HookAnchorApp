# PRD: Unified Actions System Refactoring

## Executive Summary

This PRD describes a major refactoring to unify template creation, keyboard bindings, and command actions into a single, consistent "actions" system. 

**Key Insight**: Actions are typed by their **behavior** (what they do), not their **invocation method** (where they're called from). Any action can be:
- Bound to a keyboard key
- Referenced by a command
- Called from other actions

This creates a clean, extensible architecture where adding new behaviors is simple and consistent.

## Goals

1. **Unification**: Merge templates, keyboard bindings, and command actions into a single "actions" system
2. **Consistency**: All actions use the same execution path and parameter expansion
3. **Extensibility**: Easy to add new action types without modifying core logic
4. **Simplification**: Reduce configuration complexity and code duplication
5. **Backwards Compatibility**: Existing configs should continue to work with minimal changes

## Current State

Currently, we have three separate systems:
- **Templates**: Defined in `templates:` section, used for creating new commands
- **Keyboard Bindings**: Defined in `keybindings:` section, hardcoded to specific functions
- **Command Actions**: Defined as `action_*` functions, executed when commands run

Each system has its own:
- Configuration format
- Execution logic
- Parameter handling
- Variable expansion

## Proposed Architecture

### 1. Unified Actions Section

Rename `templates:` to `actions:` in config.yaml. Each action can be:
- Bound to a keyboard key via the `key` field
- Associated with a command in commands.txt
- Called from other actions

```yaml
actions:
  # Example: Template action that creates new commands
  note:
    description: "Create dated note in Notes folder"
    type: "template"  # Behavior type: creates a new command
    key: "$"          # Optional: bind to keyboard
    name: "{{YYYY}}-{{MM}}-{{DD}} {{input}}"
    action: "markdown"
    arg: "~/Notes/{{YYYY}}/{{MM}}/{{YYYY}}-{{MM}}-{{DD}} {{input}}.md"
    patch: "Notes"
    edit: true
    file: "~/Notes/{{YYYY}}/{{MM}}"
    contents: |
      # {{YYYY}}-{{MM}}-{{DD}} {{input}}
      
  # Example: Popup navigation action bound to arrow key
  nav_down:
    description: "Navigate down in popup"
    type: "popup"
    key: "ArrowDown"
    popup_action: "navigate"
    dx: 0
    dy: 1
    
  # Example: Open URL action (could be bound to key or used in command)
  open_github:
    description: "Open GitHub homepage"
    type: "open_url"
    url: "https://github.com"
    browser: "Chrome"  # Optional, defaults to system browser
```

### 2. Action Types (Based on Behavior)

Action types are categorized by **what they do**, not where they're invoked:

#### Popup-Specific Action (Single Type for All Popup Operations)
- `popup` - Handles all popup-specific operations (params: popup_action, additional params)
  - popup_action values:
    - `navigate` - Move selection (additional params: dx, dy)
    - `exit` - Exit the application
    - `execute` - Execute selected command
    - `rebuild` - Force rebuild and rescan
    - `show_help` - Display help/keybindings
    - `edit_command` - Edit selected command
    - `show_folder` - Open first matching folder

#### Command Creation Actions  
- `template` - Create new command from template (params: name, action, arg, patch, etc.)
- `choice` - Present menu and execute selected action (params: prompt, choices) **[DEFERRED]**
- `grab` - Capture window/app context (params: delay)

#### Execution Actions
- `open_url` - Open URL in browser (params: url, browser?)
- `open_app` - Launch application (params: app, args?)
- `open_folder` - Open folder in Finder (params: path)
- `open_file` - Open file with default app (params: path)
- `shell` - Execute shell command (params: command, windowed?)
- `javascript` - Execute JavaScript code (params: code)

#### Specialized Actions
- `obsidian` - Open in Obsidian (params: vault, file)
- `1password` - Search 1Password (params: query)
- `slack` - Navigate to Slack channel (params: channel)
- `tmux` - Activate tmux session (params: path, session)
- `type_text` - Type text into focused app (params: file or text)

#### Meta Actions
- `alias` - Used within templates to create command aliases (params: target_command)
  - Creates a new command in commands.txt that points to an existing command
  - Example: Template creates "gh" as alias to existing "GitHub" command
  - Note: This is NOT for action-to-action mapping (that's handled by action definitions)

### 3. Action Execution

For any action with type `foo`:
1. System looks for function `action_foo` (built-in or JavaScript)
2. Merges action parameters with runtime context
3. Expands all `{{...}}` expressions via JavaScript
4. Executes the action function with expanded parameters

### 4. Parameter Expansion System

#### Core Function: `expand_string(s: &str) -> Result<String>`

```rust
// Expands all {{...}} expressions in a string via JavaScript evaluation
fn expand_string(s: &str, context: &ActionContext) -> Result<String> {
    // Pattern: {{expression}}
    // 1. Find all {{...}} expressions
    // 2. For each expression:
    //    - Set up JavaScript context with variables
    //    - Evaluate expression
    //    - Replace with result
    // 3. On error: show dialog and terminate
}
```

#### JavaScript Context Setup

Before evaluating any `{{...}}` expression, populate JavaScript context:

```javascript
// Previous command context
previous = {
    name: "Last Command Name",
    path: "/full/path/to/command.md",
    folder: "/full/path/to",
    hook: "hook://lastcommand"  // URL-safe compressed name
};

// Selected command context  
selected = {
    name: "Selected Command",
    path: "/path/to/selected.md", 
    folder: "/path/to",
    hook: "hook://selected"
};

// Template variables (existing)
input = "user input text";
arg = "command argument";
YYYY = "2025";
MM = "01";
DD = "10";
hh = "14";
mm = "30";

// Action-specific variables
first_match = "first matching folder from search";
grabbed_action = "captured action type";
grabbed_arg = "captured argument";
```

#### Expression Examples

```yaml
# Simple variable replacement
arg: "{{input}}"  # → "user input text"

# Property access
name: "{{previous.name}} Copy"  # → "Last Command Name Copy"

# JavaScript expressions
windowed: "{{arg.startsWith('W ')}}"  # → true/false
path: "{{previous.folder + '/' + input}}"  # → "/full/path/to/newfile"

# Conditional logic
action: "{{grabbed_action || 'folder'}}"  # → grabbed_action or 'folder' as default
```

### 5. Migration Examples

#### Current Keyboard Bindings → Actions
```yaml
# Before (keybindings section)
keybindings:
  exit_app: "Escape"
  navigate_down: "ArrowDown"
  show_folder: "/"

# After (unified actions)
actions:
  exit_app:
    description: "Exit the application"
    type: "popup"
    key: "Escape"
    popup_action: "exit"
    
  navigate_down:
    description: "Move selection down"
    type: "popup"
    key: "ArrowDown"
    popup_action: "navigate"
    dx: 0
    dy: 1
    
  show_folder:
    description: "Open first matching folder"
    type: "popup"
    key: "/"
    popup_action: "show_folder"
```

#### Current Command Actions → Actions
```yaml
# Before (functions section with structures)
functions:
  action_chrome: {fn: open_with, app: "Google Chrome", arg: "{{arg}}"}
  action_obs: {fn: shell_sync, command: "open -a Obsidian..."}

# After (unified actions, used by commands)
actions:
  chrome:
    description: "Open URL in Chrome"
    type: "open_url"
    browser: "Google Chrome"
    url: "{{arg}}"  # arg comes from command
    
  obs:
    description: "Open file in Obsidian"
    type: "obsidian"
    vault: "{{obsidian_vault}}"
    file: "{{arg}}"
```

### 6. Required Parameters by Action Type

Each action type has specific required and optional parameters:

| Action Type | Required Parameters | Optional Parameters | Example |
|------------|-------------------|-------------------|---------|
| `popup` | popup_action | dx, dy (for navigate) | popup_action: "navigate", dx: 0, dy: 1 |
| `template` | name, action, arg | patch, edit, contents, file | name: "{{input}}" |
| `choice` (deferred) | prompt, choices | - | choices: "option1,option2" |
| `grab` | - | delay | delay: 3 |
| `open_url` | url | browser | url: "https://github.com" |
| `open_app` | app | args | app: "Finder" |
| `open_folder` | path | - | path: "~/Documents" |
| `open_file` | path | - | path: "~/file.txt" |
| `shell` | command | windowed | command: "ls -la" |
| `obsidian` | file | vault | file: "Daily/{{YYYY}}-{{MM}}-{{DD}}" |
| `1password` | query | - | query: "{{input}}" |
| `slack` | channel | - | channel: "general" |
| `tmux` | path | session | path: "{{arg}}" |
| `type_text` | file or text | - | file: "~/template.txt" |
| `alias` | target_command | - | target_command: "GitHub" |

### 7. Commands.txt Integration

Commands in commands.txt reference actions by name:
```
# Current format (action name is implicit)
GitHub | chrome | https://github.com | Web

# New format (action name is explicit, same result)
GitHub | chrome | https://github.com | Web

# The 'chrome' action is defined in config.yaml:
# actions:
#   chrome:
#     type: "open_url"
#     browser: "Google Chrome"
#     url: "{{arg}}"
```

When a command is executed:
1. System looks up the action name (e.g., "chrome")
2. Finds the action definition in the `actions:` section
3. Merges command's arg with action parameters
4. Expands all `{{...}}` expressions
5. Executes the action's type handler

### 8. Choice Action Type (DEFERRED)

**NOTE: The choice action type will NOT be implemented in the initial refactoring. We will build and verify the entire unified actions system first, then add choice functionality in a future phase.**

New action type for presenting choices (future feature):
```yaml
actions:
  create_work_item:
    description: "Choose type of work item to create"
    type: "choice"
    key: "Ctrl+W"
    prompt: "What type of work item?"
    choices: "project,task,meeting,note"  # Comma-separated action names
    
  project:
    description: "Create new project"
    type: "template"
    # ... template parameters
```

Implementation approach when we add this feature:

**Selected Approach: Reuse Existing Dialog System** (src/ui/dialog.rs)
- Already exists with button support
- Currently used for error dialogs
- Will extend to show choices as buttons:
```rust
// Example dialog spec for choices
vec![
    "=Choose Work Item Type".to_string(),      // Title
    "'What type of work item?".to_string(),    // Prompt
    "!Project".to_string(),                    // Button choice 1
    "!Task".to_string(),                       // Button choice 2
    "!Meeting".to_string(),                    // Button choice 3
    "!Note".to_string(),                       // Button choice 4
]
```
- Pros: Minimal new code, consistent with error dialogs, dialog system already exists
- Cons: Limited to button-based selection (no arrow key navigation through list)

This approach was chosen because:
1. The Dialog system already exists and works
2. It provides a clean MVP implementation
3. If arrow-key list navigation is needed later, we can enhance or replace

## Implementation Plan

### Phase 1: Core Infrastructure
- [ ] Create `expand_string()` function for JavaScript-based string expansion
- [ ] Add `previous` and `selected` objects to JavaScript context
- [ ] Implement action type dispatch system (`action_<type>` function lookup)

### Phase 2: Template Migration
- [ ] Rename `templates:` to `actions:` in config parser
- [ ] Add `type: "template"` to all existing templates
- [ ] Create `action_template` function with current template logic
- [ ] Ensure backwards compatibility for old configs

### Phase 3: Keyboard Bindings Migration
- [ ] Convert keybindings to actions with `type: "keyboard"`
- [ ] Create `action_keyboard` function
- [ ] Update keyboard event handler to use actions system

### Phase 4: Command Actions Migration  
- [ ] Convert structured functions to actions section
- [ ] Remove structured definitions from functions section
- [ ] Update command execution to use unified system

### Phase 5: Choice Action Type (DEFERRED TO FUTURE)
- [ ] ~~Evaluate popup reuse vs new dialog approach~~ → Will use Dialog system
- [ ] ~~Implement choice UI mechanism~~ → Deferred
- [ ] ~~Create `action_choice` function~~ → Deferred
- [ ] ~~Add example choice actions to default config~~ → Deferred
**NOTE: This phase is deferred. We will implement and verify the core system first.**

### Phase 6: Testing & Documentation
- [ ] Update all documentation
- [ ] Create migration guide
- [ ] Test with various existing configs
- [ ] Performance testing of JavaScript expansion

## Success Criteria

1. All existing functionality works without regression
2. Configuration is simpler and more consistent
3. Adding new action types requires minimal code
4. Performance impact is negligible (<50ms for action execution)
5. Error handling provides clear user feedback

## Backwards Compatibility Strategy

To ensure smooth migration:

1. **Auto-migration on startup**: 
   - If `templates:` section exists, automatically convert to `actions:`
   - If `keybindings:` section exists, generate corresponding actions
   - Save migrated config to backup before overwriting

2. **Legacy function support**:
   - Keep `action_*` function names working during transition
   - When no action definition found, fall back to legacy function lookup
   - Deprecation warnings in logs

3. **Commands.txt compatibility**:
   - Existing commands continue to work unchanged
   - Action lookup falls back to `action_<name>` functions if no action defined

4. **Migration tool**:
   - Provide `ha --migrate-config` command
   - Shows preview of changes before applying
   - Creates timestamped backup

## Open Questions (Resolved)

1. **Choice UI Implementation**: Should we reuse the popup system or create a dedicated choice dialog? 
   - **Decision**: Start with popup reuse as the simplest approach
   - We'll implement using the existing popup system first
   - If complexity becomes an issue, we can refactor to a dedicated dialog later

2. **Performance**: Is per-string JavaScript evaluation fast enough, or should we batch evaluate?
   - **Decision**: JavaScript evaluation is fast enough for interactive use
   - No need to benchmark or optimize preemptively
   - Modern JS engines handle small evaluations efficiently

3. **Error Recovery**: How should we handle JavaScript errors in string expansion?
   - **Decision**: Show error dialog directly to the user
   - Error message: "Couldn't execute this action because of an error in expansion: [error details]"
   - Terminate the action execution immediately
   - Return system to default state after the failed action

4. **Action naming conflicts**: What if user defines action with same name as built-in?
   - **Decision**: User-defined actions overriding built-ins is valid behavior
   - Log a debug-level message (not a warning) when this occurs
   - No persistent warnings - users may intentionally override built-ins
   - This allows users to customize any aspect of the system

## Risks & Mitigations

- **Risk**: Breaking existing configurations
  - **Mitigation**: Extensive testing, backwards compatibility layer

- **Risk**: Performance degradation from JavaScript evaluation
  - **Mitigation**: Benchmark early, optimize if needed, consider caching

- **Risk**: Increased complexity for users
  - **Mitigation**: Clear documentation, good defaults, migration tools

## Timeline Estimate

- Phase 1-2: 2-3 days (core infrastructure and template migration)
- Phase 3-4: 2 days (keyboard and command migration)
- Phase 5: ~~1-2 days~~ **DEFERRED** (choice action type will be added after system verification)
- Phase 6: 1-2 days (testing and documentation)

**Total: 5-7 days of development** (excluding deferred choice action)

## Key Insights

1. **Actions are behavior-based, not invocation-based**: An action's type describes what it does (popup, open_url, template), not where it's called from (keyboard, command, etc.)

2. **Popup actions are consolidated**: All popup-specific behaviors (navigate, exit, rebuild, etc.) are handled by a single `popup` action type with a `popup_action` parameter. This maintains the paradigm that actions should be executable from anywhere (CLI, commands, etc.)

3. **Any action can be invoked multiple ways**:
   - Bound to a keyboard key (via `key` field)
   - Associated with a command (in commands.txt)
   - Called from other actions (via `rewrite` or `choice`)

4. **Some actions are instantiated rarely**: The `popup` action type handles all popup-specific operations, with only about 10-12 instances needed for all keyboard shortcuts

5. **Most command actions won't be explicitly defined**: They'll use generic action types (open_url, shell, etc.) with parameters filled from the command

## Appendix: Complete Migration Example

```yaml
# ============ BEFORE (current system) ============
templates:
  note:
    key: "$"
    name: "{{YYYY}}-{{MM}}-{{DD}} {{input}}"
    action: "markdown"
    arg: "~/Notes/{{YYYY}}/{{MM}}/{{input}}.md"
    
keybindings:
  exit_app: "Escape"
  navigate_down: "ArrowDown"
  show_folder: "/"
  
functions:
  action_chrome: {fn: open_with, app: "Google Chrome", arg: "{{arg}}"}
  action_cmd: |
    const fullCmd = "{{arg}}";
    if (fullCmd.startsWith('W ')) {
      // Terminal window execution
    }

# ============ AFTER (unified actions) ============
actions:
  # Template actions (create new commands)
  note:
    description: "Create dated note"
    type: "template"
    key: "$"
    name: "{{YYYY}}-{{MM}}-{{DD}} {{input}}"
    action: "markdown"
    arg: "~/Notes/{{YYYY}}/{{MM}}/{{input}}.md"
    
  # Popup-specific actions (consolidated into single type)
  nav_up:
    description: "Navigate up"
    type: "popup"
    key: "ArrowUp"
    popup_action: "navigate"
    dx: 0
    dy: -1
    
  nav_down:
    description: "Navigate down"
    type: "popup"
    key: "ArrowDown"
    popup_action: "navigate"
    dx: 0
    dy: 1
    
  nav_left:
    description: "Navigate left"
    type: "popup"
    key: "ArrowLeft"
    popup_action: "navigate"
    dx: -1
    dy: 0
    
  nav_right:
    description: "Navigate right"
    type: "popup"
    key: "ArrowRight"
    popup_action: "navigate"
    dx: 1
    dy: 0
    
  exit_app:
    description: "Exit application"
    type: "popup"
    key: "Escape"
    popup_action: "exit"
    
  execute_selected:
    description: "Execute selected command"
    type: "popup"
    key: "Enter"
    popup_action: "execute"
    
  force_rebuild:
    description: "Force rebuild and rescan"
    type: "popup"
    key: "`"
    popup_action: "rebuild"
    
  show_folder:
    description: "Open first matching folder"
    type: "popup"
    key: "/"
    popup_action: "show_folder"
    
  edit_command:
    description: "Edit selected command"
    type: "popup"
    key: ";"
    popup_action: "edit_command"
    
  show_help:
    description: "Show keyboard shortcuts"
    type: "popup"
    key: "?"
    popup_action: "show_help"
    
  # Command actions (usually not bound to keys, but could be)
  chrome:
    description: "Open in Chrome"
    type: "open_url"
    browser: "Google Chrome"
    url: "{{arg}}"
    
  cmd:
    description: "Execute shell command"
    type: "shell"
    command: "{{arg}}"
    windowed: "{{arg.startsWith('W ')}}"
    
  # Action mappings (replace old 'rewrite' functionality)
  work:
    description: "Open in work browser"
    type: "open_url"
    browser: "Google Chrome Beta"
    url: "{{arg}}"
    
  # Choice action example
  create_work:
    description: "Create work item"
    type: "choice"
    key: "Ctrl+W"
    prompt: "What type of work item?"
    choices: "project,task,meeting"
    
  project:
    description: "Create project"
    type: "template"
    name: "{{YYYY}}-{{MM}} {{input}}"
    action: "anchor"
    arg: "~/Work/{{YYYY}}-{{MM}} {{input}}/README.md"
```