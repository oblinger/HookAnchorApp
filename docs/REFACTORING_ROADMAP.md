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
- [ ] Update cli/search.rs to import helpers from command_ops instead of local definitions
- [ ] Analyze cli/server.rs for potential extraction of orchestration logic to systems layer

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
