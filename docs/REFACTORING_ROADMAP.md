# Refactoring Roadmap

## Phase 3: Thin Out UI Layers (In Progress)

### Completed
- [x] Extracted cli/help.rs - help text functions
- [x] Extracted cli/url_handlers.rs - URL scheme handling
- [x] Extracted cli/search.rs - search/query commands
- [x] Extracted cli/execute.rs - command execution
- [x] Extracted cli/server.rs - server management
- [x] Extracted cli/install.rs - installation commands
- [x] Extracted cli/inference.rs - patch inference
- [x] Extracted cli/maintenance.rs - maintenance/diagnostic commands
- [x] Removed cli/legacy.rs - comparison tool no longer needed
- [x] Extracted ui/window_utils.rs - window positioning utilities from popup.rs
- [x] Moved helper functions (resolve_alias_to_target, get_command_folder, get_command_path) from cli/search.rs to capabilities/command_ops.rs

### Completed - Extract Business Logic from popup.rs
Moved business logic from popup.rs to capabilities layer. UI calls capability functions and handles display/dialogs.

- [x] `save_command_atomic` → `capabilities/command_ops::save_command_atomic`
- [x] `execute_rename_with_ui_update` → `capabilities/command_ops::execute_full_rename` + `FullRenameParams`
- [x] `execute_command_only_rename` → `capabilities/command_ops::execute_command_only_rename`
- [x] Delete handling → already properly extracted in `core::delete_command`

### popup.rs Analysis (5,571 lines)

**Structure breakdown:**
| Section | Lines | Notes |
|---------|-------|-------|
| AnchorSelector struct + fields | 1-135 | State definitions |
| Window/Scanner/Key mgmt | 136-548 | Uses `self` + egui ctx |
| PopupInterface impl | 553-807 | Trait implementation |
| Action execution system | 812-3978 | Business logic + UI interleaved |
| eframe::App impl (update loop) | 4007-5387 | **1,380 lines** - the big one |
| PopupWithControl | 5388-end | Wrapper |

### Decided Against
- cli/server.rs orchestration extraction - kept as-is because user feedback is tightly coupled with each step

### Future Considerations
- [ ] **Consolidate similar methods**: `command_ops.rs` now has `resolve_alias_to_target`, `get_command_folder`, `get_command_path` which are similar to `Command::resolve_alias`, `Command::get_absolute_folder_path`, `Command::get_absolute_file_path`. Consider unifying these to use a single implementation. Key differences:
  - command_ops versions use dynamic action type lookup via `get_action_arg_type()`
  - Command methods use hardcoded action names
  - command_ops returns String; Command methods return PathBuf
  - command_ops doesn't require Config parameter
  - Evaluate whether one approach can replace the other, or if both are needed

### Results
- cmd.rs reduced from 2,674 to 151 lines (thin dispatcher)
- popup.rs reduced from 5,842 to 5,571 lines (-271 lines, business logic to capabilities)

---

## Phase 4: Method Overlap Analysis (Future)

After refactoring, systematically analyze each module for:
- Duplicate or overlapping methods that could be consolidated
- Similar functions with slightly different signatures that could be unified
- Opportunities to create shared abstractions

Key areas to review:
- `capabilities/command_ops.rs` vs `Command` struct methods
- CLI helper functions vs core utilities
- Rename/save/delete operations across modules

---

## Phase 5: Comprehensive Testing (Future)

Establish systematic testing coverage:
- **Unit tests** for capabilities layer functions (pure business logic)
- **Integration tests** for CLI commands
- **UI tests** for popup interactions (if feasible with egui)
- **End-to-end tests** for common workflows

Current state: Limited test coverage exists. Need to assess:
- What tests exist today
- Critical paths that need coverage
- Testing strategy for GUI components
- CI/CD integration for automated testing
