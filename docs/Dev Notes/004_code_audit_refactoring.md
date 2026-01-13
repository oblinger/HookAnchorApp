# HookAnchor Code Audit - Refactoring Opportunities

This document catalogs potential refactoring opportunities found during a systematic audit of the HookAnchor codebase. Each finding is documented but NOT implemented - this is a planning document.

## Categories of Findings

- **COMPAT**: Backward compatibility code that may no longer be needed
- **DRY**: Code duplication that could be consolidated
- **DEADCODE**: Unused code that could be removed
- **MERGE**: Multiple code paths that could be unified
- **PARAM**: Unused or redundant parameters
- **SIMPLIFY**: Overly complex code that could be simplified

---

## 1. src/core/data/ - Data Storage Module

### DEADCODE-001: `mark_commands_modified()` function never called
- **File**: `sys_data.rs:258-262`
- **Description**: Function sets `COMMANDS_MODIFIED` flag but is never called anywhere
- **Also dead**: Static `COMMANDS_MODIFIED` at line 35
- **Action**: Remove function and static

### DEADCODE-002: `load_data_no_cache()` function never called
- **File**: `sys_data.rs:744-767`
- **Description**: Private function that loads data without caching, never used
- **Action**: Remove function

### DEADCODE-003: `HistorySettings` struct never used
- **File**: `config.rs:114-143`
- **Description**: `HistorySettings` struct defined with Default impl but never used anywhere. `HistoryViewerSettings` (lines 145-163) is the actual one in use with nearly identical fields.
- **Action**: Remove `HistorySettings` struct and its Default impl

### COMPAT-001: `skip_directory_patterns` deprecated but still used
- **File**: `config.rs:83`
- **Description**: Marked as "DEPRECATED: Use skip_patterns instead" but still used in:
  - `scanner.rs:1876` (fallback logic)
  - `cmd.rs:2109` (help text)
- **Action**: Complete migration to `skip_patterns`, remove fallback logic

### COMPAT-002: Deprecated wrapper functions in commands.rs
- **File**: `commands.rs:1236-1239` and `1483-1491`
- **Description**: `load_commands_raw()` marked as DEPRECATED with wrapper that just calls `crate::core::data::load_commands_raw()`. Same for `save_commands_to_file()`.
- **Callers still using wrappers**:
  - `scanner.rs:311`
  - `execute/actions.rs:624`
- **Action**: Update callers to use `crate::core::data::load_commands_raw()` directly, remove wrappers

### DRY-001: `backup_commands_file()` and `backup_cache_file()` nearly identical
- **File**: `storage.rs:37-54` and `storage.rs:57-74`
- **Description**: Both functions do the same thing (backup to timestamped file) with only the source file different
- **Action**: Consider consolidating into generic `backup_file(source_path, prefix)` function

### DRY-002: Debug logging for "Patents" command
- **File**: `storage.rs:103-111`
- **Description**: Specific debug logging for "Patents" command and first 5 commands - appears to be leftover debugging
- **Action**: Remove or gate behind verbose flag

### SIMPLIFY-001: Empty `impl Config {}` block
- **File**: `config.rs:578-579`
- **Description**: Empty implementation block with no methods
- **Action**: Remove

---

## 2. src/core/ - Core Business Logic

### DEADCODE-004: `update_full_line()` is a no-op
- **File**: `commands.rs:408`
- **Description**: Method `update_full_line()` on Command has empty body - does nothing
- **Callers**: Called from multiple places expecting it to do something
- **Action**: Remove method and all call sites, or implement if needed

### DEADCODE-005: Folder renaming code disabled with `&& false`
- **File**: `command_ops.rs:141-195`
- **Description**: Section 2 "Handle folder renames" has `&& false` condition that disables it completely
- **Also has**: TODO at line 144 about updating anchor flag check
- **Action**: Either implement properly or remove entirely

### DEADCODE-006: `check_and_apply_alias()` is empty
- **File**: `application_state.rs:111-113`
- **Description**: Method body is a comment saying functionality was removed with rewrite action
- **Action**: Remove method and any call sites

### DEADCODE-007: Legacy `ApplicationState` display methods simplified
- **File**: `application_state.rs:117-119` and `154-159`
- **Description**: `get_display_commands()` and `recompute_filtered_commands()` have simplified implementations with comments saying "Legacy ApplicationState is not used"
- **Action**: Verify if ApplicationState is used anywhere, remove if not

### PARAM-001: `_previous_command` parameter unused
- **File**: `template_creation.rs:253`
- **Description**: Parameter `_previous_command: Option<&Command>` is prefixed with underscore and explicitly noted as "kept for API compatibility but not used"
- **Note at line 355**: "We no longer track 'previous' command"
- **Action**: Remove parameter from `TemplateContext::new()` and update all callers

### DRY-003: Patch inference functions duplicated
- **File**: `commands.rs` and `inference.rs`
- **Description**: Multiple similar patch inference functions exist in both files:
  - `infer_patch_from_alias_target()` (commands.rs:805-845)
  - `infer_patch_from_year_prefix()` (commands.rs:825-845)
  - `is_patch_degradation()` (commands.rs:866-922)
  - Similar functions exist in inference.rs
- **Action**: Consolidate patch inference logic into inference.rs, remove duplicates from commands.rs

### MERGE-001: Multiple patch inference entry points
- **File**: `inference.rs`
- **Description**: Has both `infer_patch()` and `infer_patch_unified()` functions that do similar things
- **Action**: Merge into single function with clear interface

---

## 3. src/execute/ - Action Execution

### DEADCODE-008: Archived code block in execution_server.rs
- **File**: `execution_server.rs:296-353`
- **Description**: Large block of commented-out code labeled "ARCHIVED: These execute functions were part of the old execution path before refactoring"
- **Action**: Remove archived code block entirely

### PARAM-002: Unused parameters in `handle_client()`
- **File**: `execution_server.rs:188-192`
- **Description**: Parameters `_inherited_env` and `_base_working_dir` are prefixed with underscore, indicating they're unused but kept for signature compatibility
- **Action**: Determine if these parameters are needed; if not, remove from signature

### NOTE: execute_template_action calls no-op
- **File**: `actions.rs:294`
- **Description**: Calls `command.update_full_line()` which is a no-op (see DEADCODE-004)
- **Action**: Remove call when DEADCODE-004 is addressed

---

## 4. src/js/ - JavaScript Runtime

*Clean - no major issues found*

---

## 5. src/systems/ - Scanner & File Watching

### COMPAT-003: skip_directory_patterns fallback in scanner
- **File**: `scanner.rs:1874-1880`
- **Description**: `should_skip_path()` has fallback logic: tries `skip_patterns` first, falls back to deprecated `skip_directory_patterns`
- **Related to**: COMPAT-001 in config.rs
- **Action**: After removing deprecated config field, simplify scanner to only use `skip_patterns`

---

## 6. src/ui/ - User Interface

*Clean - well organized with good module structure*

---

## 7. src/utils/ - Utilities

### DEADCODE-009: `dialog_old.rs` entire file
- **File**: `dialog_old.rs`
- **Description**: File header says "OLD DIALOG SYSTEM - TO BE REMOVED" and "DEPRECATED: Use dialog2.rs instead". Module is already commented out in mod.rs (line 27)
- **Action**: Delete entire file `dialog_old.rs`

---

## 8. src/bin/ - Binary Entry Points

*Clean - main.rs and other entry points are well documented*

---

## 9. swift/ - Swift Supervisor

### SIMPLIFY-002: Hardcoded developer path in HookAnchor.swift
- **File**: `swift/Supervisor/HookAnchor.swift:49`
- **Description**: `getHaBinaryPath()` has hardcoded fallback path `/Users/oblinger/ob/proj/HookAnchor/HookAnchorApp/target/release/HookAnchorCommand` for development
- **Action**: Consider using environment variable or removing hardcoded path

---

## 10. Cross-Module Patterns

### DRY-004: expand_tilde/expand_home duplicated
- **Files**: Multiple locations
- **Description**: Home directory expansion (`~/` → `/Users/...`) is implemented in:
  - `utils/utilities.rs`: `expand_tilde()`
  - `scanner.rs:1910-1917`: `expand_home()`
  - Various other places inline
- **Action**: Consolidate to single `expand_tilde()` in utils module

### Note: update_full_line() calls throughout codebase
- **Issue**: `command.update_full_line()` is called in many places but is a no-op
- **Related to**: DEADCODE-004
- **Action**: Search for and remove all `update_full_line()` calls when fixing DEADCODE-004

---

## Summary

| Category | Count | Priority |
|----------|-------|----------|
| DEADCODE | 9     | High     |
| COMPAT   | 3     | Medium   |
| DRY      | 4     | Low      |
| SIMPLIFY | 2     | Low      |
| MERGE    | 1     | Medium   |
| PARAM    | 2     | Low      |

**Total findings: 21**

### Priority recommendations:
1. **High priority (DEADCODE)**: Remove dead code first - low risk, immediate cleanup
2. **Medium priority (COMPAT/MERGE)**: Remove backward compatibility code after verifying no configs use old fields
3. **Low priority (DRY/SIMPLIFY/PARAM)**: Refactor for cleaner code

---

*Last updated: 2025-12-30*
*Modules audited: ALL (complete)*
