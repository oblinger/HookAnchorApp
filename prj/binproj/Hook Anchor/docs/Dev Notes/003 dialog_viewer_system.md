# Dialog Viewer System - Standalone General-Purpose Dialog

## Public API (Prelude)

Add `use crate::prelude::*;` to any file to access these functions:

### Core Functions

```rust
/// Show fatal error dialog and terminate (never returns)
/// Blocks until user clicks Exit, then panics
fatal_error(&format!("Cannot load config: {}", path));

/// Show warning dialog (blocking)
/// Blocks until user clicks OK, then continues execution
warning(&format!("Using default value for invalid setting: {}", value));

/// Show general-purpose dialog (blocking)
/// Returns HashMap with button clicked ("exit" key) and input values
let result = dialog(&[
    "=Confirm Delete",
    "#Are you sure?",
    "'This cannot be undone",
    "!Yes",
    "!No"
]);
if result.get("exit") == Some(&"Yes".to_string()) {
    // User confirmed
}
```

**All functions are blocking** - they wait for user interaction before returning/exiting.

---

## Overview

This document outlines the design for a standalone dialog viewer binary that **replaces** the existing embedded dialog system (`ui/dialog.rs`). The new system uses the same powerful spec string format but can be launched as an independent process from anywhere - CLI, initialization code, or the popup application.

## Current System Limitations

1. **Dialog embedded in popup**: The existing dialog system (`ui/dialog.rs`) only works within the popup application. Errors during initialization or in CLI contexts cannot be displayed.

2. **Silent fallbacks confuse users**: Configuration parsing failures fall back to defaults silently. Users don't understand why their config values are ignored.

3. **No fatal error mechanism**: Critical errors should show a dialog and stop execution, but currently just log to files.

4. **Initialization errors invisible**: Errors during config loading, database initialization happen before GUI exists.

## Solution: Standalone Dialog Binary

### Architecture

Create a new binary (`HookAnchorDialog`) that:
- **Replaces** the embedded dialog system entirely
- Uses the **same spec string format** (Vec String) as the original
- Can be launched as an **independent process** from anywhere
- Supports **blocking mode** (waits for result) and **non-blocking mode** (fire-and-forget)
- Uses **egui** for consistent appearance

### Components

```
src/
├── dialog_viewer.rs              # New binary: main entry point
└── ui/
    ├── dialog.rs                 # MOVE: Extract core rendering to shared module
    └── dialog_renderer.rs        # NEW: Shared dialog rendering logic
```

### Migration Strategy

1. **Extract rendering logic** from `ui/dialog.rs` into reusable module
2. **Create standalone binary** that uses the same rendering
3. **Replace popup's dialog usage** with calls to standalone binary
4. **Remove** embedded dialog from popup once transition complete

## Command-Line Interface

### Spec String Format (Preserves Order)

The system uses the same spec string format as the original dialog system. Each string starts with a control character:

- `=Title` - Sets dialog window title
- `#Text` - Large heading text (starts new row)
- `'Text` - Label text (normal size)
- `$key,placeholder` - Input field with key and placeholder
- `&Text` - Multi-line text box (auto-sized, read-only)
- `^Text` - Multi-line text box with scrolling
- `!Text` - Button (buttons on same row if consecutive)

### Command-Line Usage

```bash
# General purpose dialog using spec strings (preserves order)
HookAnchorDialog --spec "=Confirm Action" \
                 --spec "#Are you sure?" \
                 --spec "'This will delete the file" \
                 --spec "!Yes" \
                 --spec "!No"

# Multiple spec arguments preserve order
HookAnchorDialog --spec "=Enter Name" \
                 --spec "'Please enter your name:" \
                 --spec "\$name,Your name here" \
                 --spec "!OK" \
                 --spec "!Cancel"

# Read spec strings from stdin (one per line)
echo -e "=Error\n#Configuration Error\n'Invalid window size format\n!OK" | HookAnchorDialog

# Convenience wrappers for common dialogs
HookAnchorDialog --error "Configuration file is corrupted"
HookAnchorDialog --warning "Using default window size"
HookAnchorDialog --info "Database migrated successfully"
HookAnchorDialog --fatal "Cannot initialize: config.yaml missing"
```

### Convenience Flags (Generate Spec Strings)

These flags are shortcuts that generate spec string vectors automatically:

#### `--error <message>`
Generates:
```
["=Error", "#⚠️  Error", "'<message>", "!OK"]
```
Exit code: 1 (after user clicks OK)

#### `--warning <message>`
Generates:
```
["=Warning", "#⚡ Warning", "'<message>", "!OK"]
```
Exit code: 0 (informational)

#### `--info <message>`
Generates:
```
["=Information", "#ℹ️  Information", "'<message>", "!OK"]
```
Exit code: 0 (informational)

#### `--fatal <message>`
Generates:
```
["=Fatal Error", "#🛑 Fatal Error", "'<message>", "^<details if provided>", "!Exit"]
```
Exit code: 1 (after user clicks Exit)
Usage: Critical errors requiring termination

#### `--confirm <message>`
Generates:
```
["=Confirm", "#Confirmation Required", "'<message>", "!Yes", "!No"]
```
Exit code: 0 if "Yes" clicked, 1 if "No" clicked

### Input Formats

The binary accepts spec strings via multiple methods:

1. **Command-line arguments**: Multiple `--spec` flags (order preserved)
2. **JSON array** via `--json`: `["=Title", "#Text", "!OK"]` (array preserves order)
3. **Stdin**: One spec string per line (order preserved)

**Example with JSON array (preserves order)**:
```bash
HookAnchorDialog --json '["=Confirm Delete", "#Are you sure?", "'\''This cannot be undone", "!Yes", "!No"]'
```

### Output Format

For blocking dialogs, results are returned via:
- **Exit code**:
  - 0 = First button clicked, or "OK"/"Yes" clicked
  - 1 = Second button clicked, or "Cancel"/"No" clicked
  - 2+ = Third+ button clicked
- **Stdout (JSON)**: `{"exit": "ButtonText", "field1": "value1", ...}`
  - `exit` key contains the button text clicked
  - Other keys contain input field values (from `$key,placeholder` specs)

## Implementation Details

### Module Structure

**Create `src/utils/dialog.rs`** (new module for dialog functions):

```rust
use std::process::Command;
use std::collections::HashMap;

/// Show fatal error dialog and terminate (never returns)
pub fn fatal_error(message: &str) -> ! {
    crate::utils::log_error(&format!("FATAL: {}", message));

    let exit_code = launch_dialog_blocking(&[
        "=Fatal Error",
        "#🛑 Fatal Error",
        &format!("'{}", message),
        "!Exit"
    ]);

    panic!("Fatal error: {}", message);
}

/// Show warning dialog (blocking)
pub fn warning(message: &str) {
    crate::utils::log(&format!("WARNING: {}", message));

    launch_dialog_blocking(&[
        "=Warning",
        "#⚡ Warning",
        &format!("'{}", message),
        "!OK"
    ]);
}

/// Show general-purpose dialog (blocking)
/// Returns HashMap with "exit" key for button clicked, plus any input field values
pub fn dialog(specs: &[&str]) -> HashMap<String, String> {
    launch_dialog_blocking(specs)
}

// Internal implementation
fn launch_dialog_blocking(specs: &[&str]) -> HashMap<String, String> {
    let exe_dir = crate::utils::get_binary_dir();
    let dialog_path = exe_dir.join("HookAnchorDialog");

    if !dialog_path.exists() {
        crate::utils::log_error(&format!("Dialog binary not found: {:?}", dialog_path));
        let mut result = HashMap::new();
        result.insert("exit".to_string(), "OK".to_string());
        return result;
    }

    let mut cmd = Command::new(&dialog_path);

    // Pass spec strings as multiple --spec arguments
    for spec in specs {
        cmd.arg("--spec");
        cmd.arg(spec);
    }

    // Wait for output
    match cmd.output() {
        Ok(output) => {
            // Parse JSON result from stdout
            if let Ok(json_str) = String::from_utf8(output.stdout) {
                if let Ok(result) = serde_json::from_str::<HashMap<String, String>>(&json_str) {
                    return result;
                }
            }

            // Fallback: just return exit code
            let mut result = HashMap::new();
            result.insert("exit".to_string(), "OK".to_string());
            result
        }
        Err(e) => {
            crate::utils::log_error(&format!("Failed to run dialog: {}", e));
            let mut result = HashMap::new();
            result.insert("exit".to_string(), "OK".to_string());
            result
        }
    }
}
```

### Export via Prelude

**Update `src/utils/mod.rs`**:
```rust
pub(crate) mod dialog;

// Add to public exports:
pub use dialog::{fatal_error, warning, dialog};
```

**Create `src/prelude.rs`**:
```rust
//! Common imports for HookAnchor modules
pub use crate::utils::{fatal_error, warning, dialog, log, log_error, detailed_log};
```

**Reference in `src/lib.rs`**:
```rust
pub mod prelude;
```

Now any file can use: `use crate::prelude::*;`

## Use Cases and Examples

### 1. Configuration Parsing Fallback

**Current code** (config.rs:423-425):
```rust
pub fn get_max_window_width(&self) -> u32 {
    if let Some(ref size_str) = self.max_window_size {
        if let Some((width, _)) = parse_window_size(size_str) {
            return width;
        }
    }
    1700 // default fallback - SILENT!
}
```

**Updated code** (with prelude):
```rust
use crate::prelude::*;

pub fn get_max_window_width(&self) -> u32 {
    if let Some(ref size_str) = self.max_window_size {
        if let Some((width, _)) = parse_window_size(size_str) {
            return width;
        } else {
            // User provided invalid format - warn them
            warning(&format!(
                "Invalid window size format in config: '{}'\n\
                 Expected format: 'WIDTHxHEIGHT' (e.g., '1700x1100')\n\
                 Using default: 1700x1100",
                size_str
            ));
        }
    }
    1700 // default fallback
}
```

### 2. Missing Configuration File

**Current code** (config.rs:274-280):
```rust
if !config_path.exists() {
    return ConfigResult::Error(format!(
        "Config file not found: {}\n\n\
        The installer should have created this automatically.\n\
        Try running: ha --install",
        config_path.display()
    ));
}
```

**Updated code** (with prelude):
```rust
use crate::prelude::*;

if !config_path.exists() {
    fatal_error(&format!(
        "Configuration file not found!\n\n\
         Expected location: {}\n\n\
         The installer should have created this automatically.\n\
         Please run: ha --install",
        config_path.display()
    ));
}
```

### 3. Database Initialization Failure

**Current code** (history.rs:38-40):
```rust
pub(super) fn initialize_history_db() -> SqlResult<Connection> {
    let db_path = get_history_db_path();
    let conn = Connection::open(db_path)?;
    // ... rest of initialization
}
```

**Updated code** (with prelude):
```rust
use crate::prelude::*;

pub(super) fn initialize_history_db() -> SqlResult<Connection> {
    let db_path = get_history_db_path();

    match Connection::open(&db_path) {
        Ok(conn) => Ok(conn),
        Err(e) => {
            warning(&format!(
                "Failed to open history database!\n\n\
                 Database path: {:?}\n\
                 Error: {}\n\n\
                 The database file may be corrupted.\n\
                 Try deleting the file and restarting.",
                db_path, e
            ));
            Err(e)
        }
    }
}
```

### 4. Corrupted Commands File Safety Check

**Current code** (storage.rs:134-139):
```rust
if updated_commands.len() > 10000 {
    let error_msg = format!("CORRUPTION DETECTED: Attempting to save {} commands...", updated_commands.len());
    crate::utils::log_error(&error_msg);
    crate::utils::detailed_log("CORRUPTION", &error_msg);
    return Err("Command count exceeds safety limit".into());
}
```

**Updated code** (with prelude):
```rust
use crate::prelude::*;

if updated_commands.len() > 10000 {
    warning(&format!(
        "⚠️  Data Corruption Detected!\n\n\
         Attempting to save {} commands (normal: ~100-500)\n\n\
         This indicates command inflation/duplication.\n\
         Save operation BLOCKED to protect your data.\n\n\
         Please report this issue with logs from:\n\
         ~/.config/hookanchor/anchor.log",
        updated_commands.len()
    ));
    return Err("Command count exceeds safety limit".into());
}
```

## Implementation Plan

### Phase 1: Core Infrastructure (High Priority)

1. **Extract shared rendering logic**
   - Move dialog rendering from `ui/dialog.rs` to `ui/dialog_renderer.rs`
   - Make spec string parsing reusable
   - Keep HashMap result format

2. **Create dialog_viewer.rs binary**
   - Basic egui window setup
   - Command-line argument parsing (`--spec`, `--json`, convenience flags)
   - Use shared rendering logic
   - Return results via JSON stdout

3. **Add to Cargo.toml**
   - Register new binary: `HookAnchorDialog`
   - Ensure it builds with release

4. **Create dialog helper functions**
   - Create `src/utils/dialog.rs` with three functions:
     - `fatal_error(message)` - blocks, shows dialog, panics
     - `warning(message)` - blocks, shows dialog, returns
     - `dialog(specs: &[&str])` - general purpose, returns HashMap
   - Create `src/prelude.rs` to export these functions
   - All functions are blocking and use the standalone binary

### Phase 2: Integration (Medium Priority)

5. **Update config.rs fallbacks**
   - Window size parsing errors
   - YAML parsing failures
   - Missing config file (fatal)

6. **Update storage.rs safety checks**
   - Command inflation detection
   - Patch stripping detection
   - Backup failures

7. **Update history.rs initialization**
   - Database open failures
   - Schema migration errors

### Phase 3: Advanced Features (Lower Priority)

8. **Confirmation dialogs**
   - JSON input support
   - Multiple buttons
   - Return values via exit code

9. **Timeout support**
   - Auto-close after N seconds
   - Visual countdown timer

10. **Error details scrolling**
    - Long error messages in scrollable text box
    - Stack traces for debugging

## Window Specifications

### Size and Position
- **Default size**: 500x300 pixels
- **Minimum size**: 400x200 pixels
- **Maximum size**: 800x600 pixels (for error details)
- **Position**: Centered on screen
- **Always on top**: Yes (for critical errors)
- **Resizable**: Yes (for viewing error details)

### Styling
- **Font**: Same as popup (system default)
- **Colors**:
  - Error: Red accent
  - Warning: Yellow accent
  - Info: Blue accent
  - Fatal: Dark red background with white text
- **Icon size**: 32x32 pixels
- **Button size**: 100x30 pixels minimum
- **Padding**: 20px around content

## Testing Strategy

### Unit Tests

1. **Argument parsing**
   - Test all dialog types
   - Test JSON input parsing
   - Test invalid arguments

2. **Exit codes**
   - Verify correct codes for each dialog type
   - Test button selection affects exit code

3. **Dialog rendering**
   - Mock egui context
   - Verify layout for each dialog type
   - Test with long messages (scrolling)

### Integration Tests

4. **Launch from utils**
   - Test `show_dialog_with_specs()` successfully spawns process
   - Test `fatal_error()` blocks and returns
   - Test missing binary graceful failure

5. **Real user scenarios**
   - Invalid config.yaml → warning dialog
   - Missing config.yaml → fatal dialog
   - Corrupted database → error dialog
   - Command inflation → error dialog with details

### Manual Testing

6. **Visual appearance**
   - All dialog types look correct
   - Icons render properly
   - Text wrapping works
   - Scrolling works for long messages

7. **User interaction**
   - Buttons respond correctly
   - Window can be closed via X button
   - Escape key closes dialog
   - Enter key triggers default button

## Success Criteria

- ✅ Dialog viewer binary builds and installs correctly
- ✅ Can be launched from any context (CLI, GUI, initialization)
- ✅ All dialog types (error, warning, info, fatal) work correctly
- ✅ Fatal errors block execution and show clear messages
- ✅ Warnings for fallbacks inform users of ignored config values
- ✅ Exit codes allow calling code to check user response
- ✅ General-purpose dialogs work (input fields, confirmations)
- ✅ No crashes or hangs in any error scenario
- ✅ Consistent visual appearance across all dialogs

## Migration Path

### Eventually Replace Embedded Dialog

**Phase 1**: Keep embedded dialog in popup, add standalone binary
- Embedded `ui/dialog.rs` continues to work during transition
- New standalone dialog used for initialization and CLI contexts

**Phase 2**: Migrate popup to use standalone dialog
- Replace direct dialog calls with `show_dialog_with_specs()`
- Remove embedded dialog rendering from popup

**Phase 3**: Complete migration
- Remove `ui/dialog.rs` entirely
- All dialogs use standalone binary

### When to Use Standalone Dialog

Use standalone dialog (`show_dialog_with_specs()`, `error_with_dialog()`, etc.) when:
- Error occurs during initialization (before popup exists)
- Error occurs in CLI commands (`ha --rescan`, etc.)
- Configuration parsing fails
- Fatal errors require termination
- General input/confirmation needed from any context
- Want consistent dialog appearance across entire application

### Deprecate Error Queue System

The existing `queue_user_error()` system can be phased out:
- Currently used for runtime errors within popup
- Can be replaced with `error_with_dialog()` (non-blocking, standalone)
- Removes the complexity of queue management
- Ensures errors are always visible to user (not queued silently)

## Future Enhancements

1. **Sound effects**: Optional beep for error dialogs (configurable)
2. **Error reporting**: "Send error report" button that creates GitHub issue
3. **Multiple dialog stacking**: Queue multiple dialogs if multiple errors occur
4. **Clipboard copy**: Copy error message to clipboard button
5. **Dark mode support**: Match system theme or popup theme
6. **Localization**: Support for multiple languages in error messages

## References

- Current error queue: `src/utils/error.rs`
- Current dialog system: `src/ui/dialog.rs`
- History viewer pattern: `src/history_viewer.rs`
- Config loading: `src/core/data/config.rs`
- Storage safety checks: `src/core/data/storage.rs`
