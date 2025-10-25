# Template Creation System Plan

## Overview
A comprehensive system for creating new HookAnchor entries using templates with variable expansion and flexible configuration.

## Implementation Phases

### Phase 1: Core Infrastructure (v0.5.2)
1. **Create `template_creation.rs` module**
   - Basic Template struct
   - Variable expansion engine
   - Template registry

2. **Update config structure**
   - Add `templates` section to config.yaml
   - Define Template struct in config.rs
   - Implement default template

3. **Basic variable expansion**
   - Implement `{{variable}}` syntax parser
   - Support basic variables: input, selected_name, selected_path
   - Add date/time variables

### Phase 2: Template Processing (v0.5.3)
1. **Template field processing**
   - Process all command fields (name, action, arg, patch, flags)
   - Variable substitution in each field
   - Command validation

2. **Keybinding integration**
   - Add `template_create` action
   - Default binding to '-' key
   - Hook into popup UI

3. **Basic testing**
   - Unit tests for variable expansion
   - Integration test for default template

### Phase 3: Advanced Features (v0.5.4)
1. **File/folder creation**
   - Process `file` field for folder creation
   - Process `contents` field for file content
   - Handle path expansion

2. **Grab functionality**
   - Implement delayed grab with countdown
   - Set grabbed values as variables
   - Allow override in template

3. **Editor integration**
   - Process `edit` field
   - Launch command editor before save
   - Handle cancellation

### Phase 4: Extended Templates (v0.5.5)
1. **Multiple templates**
   - Template selection UI
   - Per-template keybindings
   - Template inheritance

2. **JavaScript integration**
   - Execute JS functions in variable expansion
   - Access to existing JS runtime
   - Custom functions

## Template Structure

```yaml
templates:
  default:
    name: "{{input}}"
    action: "anchor"
    arg: "/path/to/{{input}}/{{input}}.md"
    patch: "{{selected_patch}}"
    flags: ""
    edit: false
    
  note:
    name: "{{YYYY}}-{{MM}}-{{DD}} {{input}}"
    action: "markdown"
    arg: "/notes/{{YYYY}}/{{MM}}/{{name}}.md"
    patch: "Notes"
    file: "/notes/{{YYYY}}/{{MM}}"
    contents: "# {{name}}\n\nCreated: {{YYYY}}-{{MM}}-{{DD}} {{hh}}:{{mm}}\n\n"
    
  grab_template:
    grab: 3  # seconds
    name: "{{input}}"
    action: "{{grabbed_action}}"
    arg: "{{grabbed_arg}}"
    edit: true
```

## Variable Categories

1. **Context Variables**
   - `input` - Search box content
   - `selected_name` - Selected command name
   - `selected_path` - Full path of selected command
   - `selected_folder` - Folder of selected command
   - `selected_patch` - Patch of selected command
   - `previous_name` - Previously selected command

2. **Time Variables**
   - `YYYY` - 4-digit year
   - `YY` - 2-digit year
   - `M` - Month (1-12)
   - `MM` - Month (01-12)
   - `MMM` - Month name (Jan-Dec)
   - `D` - Day (1-31)
   - `DD` - Day (01-31)
   - `h` - Hour (0-23)
   - `hh` - Hour (00-23)
   - `m` - Minute (0-59)
   - `mm` - Minute (00-59)
   - `s` - Second (0-59)
   - `ss` - Second (00-59)

3. **Grab Variables** (when grab is used)
   - `grabbed_action` - Action from grabbed window
   - `grabbed_arg` - Argument from grabbed window

4. **Custom Variables** (future)
   - JavaScript function results
   - User-defined variables

## Testing Strategy

1. **Unit Tests**
   - Variable expansion edge cases
   - Date/time formatting
   - Template validation

2. **Integration Tests**
   - Default template creation
   - File/folder creation
   - Grab functionality

3. **Manual Testing**
   - UI interaction
   - Keybinding response
   - Editor integration