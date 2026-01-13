# HookAnchor Code Organization

## Current Organization

### Overview

The codebase has approximately 34,000 lines of Rust across several modules. The largest files are:

| File | Lines | Purpose |
|------|-------|---------|
| `ui/popup.rs` | 5,826 | Popup UI + mixed business logic |
| `cmd.rs` | 2,525 | CLI + mixed business logic |
| `core/commands.rs` | 1,782 | Command management |
| `systems/scanner.rs` | 1,714 | Filesystem scanning |
| `bin/history_viewer.rs` | 1,194 | History viewer GUI |
| `core/key_processing.rs` | 1,178 | Keyboard input handling |
| `js/runtime.rs` | 1,130 | JavaScript execution |
| `core/display.rs` | 1,062 | Display/rendering logic |
| `core/template_creation.rs` | 986 | Template variable expansion |

### Current Module Structure

```
src/
├── lib.rs                    # Library entry point
├── prelude.rs                # Common imports
├── cmd.rs                    # CLI (2,525 lines - MIXED CONCERNS)
│
├── bin/                      # Standalone binaries
│   ├── ha.rs                 # Main dispatcher
│   ├── popup_server.rs       # Popup server entry
│   ├── history_viewer.rs     # History GUI
│   ├── installer_gui.rs      # Installer GUI
│   └── dialog_viewer.rs      # Dialog viewer
│
├── core/                     # Core business logic (WELL ORGANIZED)
│   ├── data/                 # Data layer
│   │   ├── config.rs         # Configuration loading
│   │   ├── sys_data.rs       # System data singleton
│   │   └── state.rs          # Persistent state
│   ├── commands.rs           # Command CRUD operations
│   ├── command_ops.rs        # Command operations
│   ├── display.rs            # Display/filtering logic
│   ├── inference.rs          # Path/patch inference
│   ├── key_processing.rs     # Key binding logic
│   └── template_creation.rs  # Template expansion
│
├── execute/                  # Execution layer (WELL ORGANIZED)
│   ├── actions.rs            # Action type handlers
│   ├── execute.rs            # Execution entry points
│   ├── execution_server.rs   # IPC server
│   └── execution_server_management.rs
│
├── ui/                       # UI layer (MIXED CONCERNS)
│   ├── popup.rs              # Popup (5,826 lines - TOO LARGE)
│   ├── popup_state.rs        # Popup state management
│   ├── command_editor.rs     # Command editor dialog
│   ├── anchor_tree_navigator.rs
│   ├── dialog.rs             # Dialog utilities
│   ├── layout.rs             # Layout calculations
│   └── helpers.rs
│
├── systems/                  # System integrations
│   ├── scanner.rs            # Filesystem scanning
│   ├── grabber.rs            # Window/selection grabbing
│   ├── hotkey.rs             # Global hotkey registration
│   ├── restart.rs            # Process restart logic
│   ├── popup_server.rs       # Server management
│   ├── cloud_scanner.rs      # Notion/cloud scanning
│   ├── obsidian.rs           # Obsidian integration
│   └── setup_assistant.rs    # First-run setup
│
├── js/                       # JavaScript runtime
│   └── runtime.rs            # Boa JS engine wrapper
│
└── utils/                    # Utilities
    ├── utilities.rs          # General utilities
    ├── logging.rs            # Logging functions
    └── ...
```

### Problems with Current Organization

1. **popup.rs is 5,826 lines** - Contains UI rendering, event handling, template execution, command execution, anchor management, navigation, and more. Too many concerns mixed together.

2. **cmd.rs is 2,525 lines** - Contains CLI parsing, help text, AND duplicates of operations that also exist in popup.rs.

3. **Duplicate code paths** - Operations like "create template context" exist in multiple forms:
   - `TemplateContext::create_basic_template()` (used by popup)
   - `TemplateContext::new()` (was dead code, now removed)
   - Similar patterns exist for anchor handling, command execution, etc.

4. **No capabilities layer** - Business operations are embedded in UI code rather than in purpose-built modules. This makes it hard to:
   - See all related operations together
   - Notice duplication
   - Test operations independently
   - Reuse operations between popup and CLI

---

## Proposed Organization

### Design Principles

1. **Thin UI layers** - popup.rs and cmd.rs should primarily:
   - Handle events/input
   - Call into capability modules
   - Render output
   - Contain minimal business logic

2. **Capability modules** - Group related operations by purpose:
   - All template operations in one place
   - All anchor operations in one place
   - All navigation operations in one place
   - etc.

3. **Single source of truth** - Each operation should have exactly ONE implementation that both popup and CLI use.

### Proposed Module Structure

```
src/
├── lib.rs
├── prelude.rs
│
├── bin/                      # Standalone binaries (unchanged)
│   ├── ha.rs
│   ├── popup_server.rs
│   ├── history_viewer.rs
│   └── ...
│
├── core/                     # Core data layer (unchanged, well organized)
│   ├── data/
│   │   ├── config.rs
│   │   ├── sys_data.rs
│   │   └── state.rs
│   ├── commands.rs
│   └── inference.rs
│
├── capabilities/             # NEW: Purpose-built operation modules
│   ├── mod.rs
│   │
│   ├── template_ops.rs       # All template operations
│   │   - create_context()
│   │   - expand_template()
│   │   - execute_template()
│   │   - handle_template_key()
│   │
│   ├── anchor_ops.rs         # All anchor operations
│   │   - set_anchor()
│   │   - get_anchor()
│   │   - clear_anchor()
│   │   - extract_folder_from_command()
│   │   - get_anchor_context()
│   │
│   ├── command_ops.rs        # Command execution operations
│   │   - find_command()
│   │   - execute_command()
│   │   - resolve_command()
│   │   - command_to_action()
│   │
│   ├── navigation_ops.rs     # Navigation operations
│   │   - navigate_up()
│   │   - navigate_down()
│   │   - navigate_hierarchy()
│   │   - enter_prefix_menu()
│   │   - exit_prefix_menu()
│   │
│   ├── display_ops.rs        # Display/filtering operations
│   │   - filter_commands()
│   │   - sort_commands()
│   │   - build_display_list()
│   │   - calculate_layout()
│   │
│   └── edit_ops.rs           # Editing operations
│       - create_command()
│       - update_command()
│       - delete_command()
│       - create_alias()
│
├── execute/                  # Execution layer (mostly unchanged)
│   ├── actions.rs
│   ├── execute.rs
│   └── execution_server.rs
│
├── ui/                       # UI layer (SIMPLIFIED)
│   ├── popup.rs              # Thin: events → capabilities → render
│   ├── popup_state.rs        # UI state only
│   ├── render.rs             # NEW: Rendering logic extracted
│   ├── command_editor.rs
│   └── ...
│
├── cli/                      # NEW: CLI layer (extracted from cmd.rs)
│   ├── mod.rs
│   ├── commands.rs           # CLI command handlers (thin)
│   └── help.rs               # Help text
│
├── systems/                  # System integrations (unchanged)
│   └── ...
│
├── js/                       # JavaScript runtime (unchanged)
│   └── runtime.rs
│
└── utils/                    # Utilities (unchanged)
    └── ...
```

### Migration Strategy

1. **Phase 1: Create capabilities/ module**
   - Start with `anchor_ops.rs` - extract all anchor logic
   - Then `template_ops.rs` - consolidate template handling
   - Update popup.rs and cmd.rs to call these modules

2. **Phase 2: Extract more capabilities**
   - `navigation_ops.rs` from popup.rs
   - `display_ops.rs` from core/display.rs + popup.rs
   - `command_ops.rs` consolidating execution paths

3. **Phase 3: Thin out UI layers**
   - popup.rs becomes event handling + capability calls + rendering
   - Extract cmd.rs into cli/ module with thin command handlers

4. **Phase 4: Cleanup**
   - Remove dead code
   - Consolidate duplicate implementations
   - Add tests for capability modules

### Benefits

1. **Duplication becomes visible** - All anchor code in one file means you see both implementations side by side.

2. **Easier to reason about** - "What can the system do with anchors?" → read anchor_ops.rs

3. **Testable** - Capability modules can be unit tested without UI.

4. **Consistent behavior** - Popup and CLI use the same code paths.

5. **Smaller files** - popup.rs shrinks from 5,800 lines to perhaps 1,500 (events + render).

---

## Current Priority: anchor_ops.rs

Given the recent bugs with anchor handling, the first extraction should be `anchor_ops.rs`:

```rust
// capabilities/anchor_ops.rs

/// Set the active anchor with folder context
pub fn set_anchor(name: String, folder: Option<String>) -> Result<()>

/// Get current anchor info from state
pub fn get_anchor() -> Option<AnchorInfo>

/// Extract folder path from a command's arg
pub fn extract_folder_from_command(cmd: &Command) -> Option<String>

/// Clear the active anchor
pub fn clear_anchor() -> Result<()>
```

This consolidates:
- `set_active_anchor()` from sys_data.rs
- `extract_folder_from_command()` from popup.rs
- Anchor state reading scattered across template_creation.rs, popup.rs
