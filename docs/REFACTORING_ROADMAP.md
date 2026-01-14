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

### In Progress
- [ ] Continue thinning popup.rs (currently 5,726 lines)

### popup.rs Analysis (5,726 lines)

**Structure breakdown:**
| Section | Lines | Notes |
|---------|-------|-------|
| AnchorSelector struct + fields | 1-135 | State definitions |
| Window/Scanner/Key mgmt | 136-548 | Uses `self` + egui ctx |
| PopupInterface impl | 553-807 | Trait implementation |
| Action execution system | 812-3978 | Business logic + UI interleaved |
| eframe::App impl (update loop) | 4007-5387 | **1,380 lines** - the big one |
| PopupWithControl | 5388-end | Wrapper |

**The challenge:** Almost everything is tightly coupled - methods use `&mut self` and `egui::Context`, business logic (rename, save, delete) is interleaved with UI updates, update loop orchestrates many subsystems.

**Options for further thinning:**
1. **Easy wins** - Look for more standalone utilities like window_utils
2. **Split update loop** - Extract major UI phases (loading, key handling, rendering, command editor handling) into helper methods in separate files
3. **Move business logic to capabilities** - The rename/save/delete operations could be extracted, with popup calling them and handling just the UI callbacks
4. **Leave it** - CLI refactoring complete (cmd.rs: 2,674 → 151 lines). Further popup thinning has diminishing returns.

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
- popup.rs reduced from 5,842 to 5,726 lines
