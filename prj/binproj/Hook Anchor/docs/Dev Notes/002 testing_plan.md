# Comprehensive Testing Plan for Data Crate

## Overview

This document outlines a comprehensive testing strategy for the `src/core/data/` crate, which serves as the centralized data layer for HookAnchor. The data crate is responsible for all persistent state management including configuration, commands, patches, application state, and history tracking.

## Module Structure

The data crate consists of the following modules:

- `config.rs` - Configuration management (YAML loading, parsing, defaults)
- `history.rs` - SQLite-based command history tracking
- `state.rs` - Application state persistence (JSON-based)
- `storage.rs` - Low-level command storage (disk I/O, caching, deduplication)
- `sys_data.rs` - Singleton data access layer and initialization
- `mod.rs` - Public interface definitions

## Testing Categories

### 1. Configuration Management Tests (`config.rs`)

#### 1.1 Configuration Error Handling
- **Test: Handle missing configuration file**
  - Input: Non-existent config file path
  - Expected: ConfigResult::Error with helpful message
  - Validation: Error message suggests running `ha --install`

- **Test: Handle corrupted YAML syntax**
  - Input: Config file with invalid YAML
  - Expected: ConfigResult::Error with parsing details
  - Validation: Error includes our custom error message and file location

#### 1.2 Configuration Parsing Helpers
- **Test: Parse valid window size string**
  - Input: "1700x1100"
  - Expected: Some((1700, 1100))
  - Validation: Both dimensions correct

- **Test: Parse invalid window size format**
  - Input: "1700x", "invalid", "1700-1100"
  - Expected: None returned, error logged to dialogue system
  - Validation: Returns None, error function called with descriptive message

- **Test: Get window dimensions fallback logs error**
  - Input: PopupSettings with invalid max_window_size format
  - Expected: Default dimensions returned, error logged via dialogue system
  - Validation: Verify 1700x1100 returned, error logged with user's invalid value shown
  - Note: Error logged but not fatal - allows app to continue with defaults

#### 1.3 Default Values
- **Test: PopupSettings default values**
  - Expected: All Option fields have sensible defaults
  - Validation: Verify max_rows=20, max_columns=3, scan_interval=10s, etc.

- **Test: LauncherSettings default values**
  - Expected: Obsidian vault settings, timeouts, etc.
  - Validation: Verify js_timeout_ms=5000, flip_focus=false

- **Test: HistoryViewerSettings default values**
  - Expected: Tree sidebar dimensions, limits, etc.
  - Validation: Verify viewable_history_limit=50000, tree_sidebar_width=250.0

### 2. History Tracking Tests (`history.rs`)

#### 2.1 Database Initialization
- **Test: Create history database from scratch**
  - Input: Non-existent database file
  - Expected: New SQLite database created with proper schema
  - Validation: Verify table structure, indexes exist

- **Test: Open existing history database**
  - Input: Existing history.db file
  - Expected: Database opened successfully
  - Validation: Can query existing records

- **Test: Migrate old schema with change_type column**
  - Input: Database with old schema including change_type
  - Expected: Column removed, data migrated (change_type='deleted' → action='$DELETED$')
  - Validation: Query schema, verify no change_type column, check migrated records

#### 2.2 Command History Recording
- **Test: Append new command to history**
  - Input: New Command with action="markdown"
  - Expected: Entry created with all fields populated
  - Validation: Query database, verify timestamp, patch, command, action, arg

- **Test: Record command modification**
  - Input: Modified command (changed action or patch)
  - Expected: New entry with updated values
  - Validation: Query shows multiple entries for same command name with different values

- **Test: Record command deletion**
  - Input: Command with action="$DELETED$"
  - Expected: Deletion entry recorded
  - Validation: Entry exists with action="$DELETED$"

- **Test: Use file modification time for path-based commands**
  - Input: Command with file_path pointing to existing file
  - Expected: Timestamp matches file's modification time
  - Validation: Compare timestamp to actual file metadata

- **Test: Calculate edit_size for file-based commands**
  - Input: Command with file_path and file_size
  - Expected: edit_size field populated from file_size
  - Validation: Verify edit_size matches command.file_size

- **Test: Handle commands without files**
  - Input: Non-file command (action="app", "url", etc.)
  - Expected: Uses provided timestamp, no edit_size
  - Validation: Timestamp matches input, edit_size is NULL

#### 2.3 History Querying
- **Test: Get history entries with limit**
  - Input: Limit of 100 entries
  - Expected: At most 100 entries returned, newest first
  - Validation: Check count, verify descending timestamp order

- **Test: Exclude deletions from results**
  - Input: exclude_deletions=true
  - Expected: No entries with action="$DELETED$"
  - Validation: Filter results for deletion marker

- **Test: Include deletions in results**
  - Input: exclude_deletions=false
  - Expected: Deletion entries included
  - Validation: Some results have action="$DELETED$"

- **Test: Query empty database**
  - Input: Newly initialized database
  - Expected: Empty Vec returned
  - Validation: Length is 0

- **Test: HistoryEntry is_deletion() method**
  - Input: Entry with action="$DELETED$"
  - Expected: is_deletion() returns true
  - Input: Entry with action="markdown"
  - Expected: is_deletion() returns false

#### 2.4 History Indexes
- **Test: Query performance with timestamp index**
  - Input: Large dataset (10,000+ entries), query by timestamp range
  - Expected: Query completes quickly (< 100ms)
  - Validation: Measure query time with EXPLAIN QUERY PLAN

- **Test: Query performance with file_path index**
  - Input: Query entries for specific file_path
  - Expected: Fast results using index
  - Validation: EXPLAIN QUERY PLAN shows index usage

- **Test: Query performance with patch index**
  - Input: Query all entries for a specific patch
  - Expected: Fast results using index
  - Validation: EXPLAIN QUERY PLAN shows index usage

### 3. State Management Tests (`state.rs`)

#### 3.1 State Persistence
- **Test: Save and load AppState**
  - Input: AppState with various fields populated
  - Expected: Saved to JSON, reloaded with identical values
  - Validation: Compare all fields before/after

- **Test: Save and load HistoryViewerState**
  - Input: HistoryViewerState with filters and window position
  - Expected: All fields preserved
  - Validation: Compare filter strings, dimensions, etc.

- **Test: Handle missing state file**
  - Input: Non-existent state.json
  - Expected: Returns default AppState
  - Validation: All fields match AppState::default()

- **Test: Handle corrupted state JSON**
  - Input: Invalid JSON in state.json
  - Expected: Returns default AppState, logs error
  - Validation: Check that default is returned, log output contains error

- **Test: Create state directory if missing**
  - Input: Save state when ~/.config/hookanchor doesn't exist
  - Expected: Directory created, file saved successfully
  - Validation: Verify directory and file exist

#### 3.2 State Fields
- **Test: Save window position**
  - Input: AppState with window_position = Some((100.0, 200.0))
  - Expected: Position preserved in JSON and reloaded
  - Validation: Exact coordinates match

- **Test: Save last executed command**
  - Input: last_executed_command = Some("MyCommand")
  - Expected: Command name preserved
  - Validation: String matches after reload

- **Test: Save server PID**
  - Input: server_pid = Some(12345)
  - Expected: PID preserved
  - Validation: Number matches after reload

- **Test: Save anchor state**
  - Input: anchor_name, anchor_timestamp, anchor_folder all set
  - Expected: All three fields preserved together
  - Validation: Verify all anchor-related fields

- **Test: Default show_files is true**
  - Input: AppState without show_files in JSON
  - Expected: show_files = true after loading
  - Validation: Check field value

#### 3.3 History Viewer State
- **Test: Save filter strings**
  - Input: patch_filter="MyPatch", name_filter="test"
  - Expected: Both filters preserved
  - Validation: Strings match after reload

- **Test: Save min_edit_size filter**
  - Input: min_edit_size = Some(500)
  - Expected: Filter value preserved
  - Validation: Number matches

- **Test: Save selected action types**
  - Input: selected_action_types = vec!["markdown", "doc"]
  - Expected: List preserved
  - Validation: Vec equality check

- **Test: Save sidebar width**
  - Input: sidebar_width = Some(300.0)
  - Expected: Width preserved
  - Validation: Float matches

### 4. Storage Layer Tests (`storage.rs`)

#### 4.1 File Path Resolution
- **Test: Get commands file path**
  - Expected: ~/.config/hookanchor/commands.txt
  - Validation: Path components correct

- **Test: Get backups folder path**
  - Expected: ~/.config/hookanchor/backups/
  - Validation: Path components correct

- **Test: Get cache file path**
  - Expected: ~/.config/hookanchor/commands_cache.json
  - Validation: Path components correct

#### 4.2 Commands Loading
- **Test: Load commands from valid commands.txt**
  - Input: Well-formed commands.txt with version header
  - Expected: Vec<Command> with all commands parsed
  - Validation: Count matches, spot check command fields

- **Test: Load commands.txt with empty lines**
  - Input: File with blank lines between commands
  - Expected: Empty lines skipped
  - Validation: Only actual commands in result

- **Test: Load commands.txt with parse errors**
  - Input: File with some invalid command lines
  - Expected: Valid commands loaded, errors logged
  - Validation: Check logs for error messages, valid commands present

- **Test: Load missing commands.txt**
  - Input: Non-existent file
  - Expected: Empty Vec, error logged
  - Validation: Length is 0, log contains error

#### 4.3 Commands Saving
- **Test: Save commands to file with backup**
  - Input: Vec<Command>
  - Expected: Backup created in backups/ folder, new file written
  - Validation: Both backup and current file exist, backup is timestamped

- **Test: Save commands with version header**
  - Input: Commands list
  - Expected: First line is version header comment
  - Validation: Read file, verify first line format

- **Test: Save commands sorted by patch then name**
  - Input: Unsorted commands
  - Expected: Output file has commands sorted
  - Validation: Parse file, verify order

- **Test: Auto-add "Cmd" patch to cmd actions without patches**
  - Input: Command with action="cmd", patch=""
  - Expected: Saved with patch="Cmd"
  - Validation: Reload and check patch field

- **Test: Safety check: Reject saving >10,000 commands**
  - Input: 10,001 commands
  - Expected: Error returned, file not saved
  - Validation: Check error message, verify file unchanged

- **Test: Safety check: Reject saving >200 commands with empty patches**
  - Input: 201 commands with empty patches
  - Expected: Error returned, file not saved
  - Validation: Check error message about patch stripping

- **Test: Create config directory if missing**
  - Input: Save when directory doesn't exist
  - Expected: Directory created
  - Validation: Directory exists after save

#### 4.4 Commands Cache
- **Test: Save commands to JSON cache**
  - Input: Vec<Command>
  - Expected: JSON file written with pretty formatting
  - Validation: File exists, valid JSON

- **Test: Load commands from cache**
  - Input: Valid commands_cache.json
  - Expected: Commands loaded successfully
  - Validation: Compare against saved commands

- **Test: Load cache with invalid JSON**
  - Input: Corrupted JSON file
  - Expected: None returned, error logged
  - Validation: Check return value and logs

- **Test: Load missing cache file**
  - Input: Non-existent cache
  - Expected: None returned
  - Validation: No errors, just None

- **Test: Sanitize flags when loading from cache**
  - Input: Cached commands with malformed flags (spaces, commas)
  - Expected: Flags cleaned to only alphabetic characters
  - Validation: Check flags field after loading

#### 4.5 Deduplication
- **Test: Deduplicate exact duplicates**
  - Input: Two identical commands
  - Expected: One command in output
  - Validation: Length reduced by 1

- **Test: Deduplication key: command + action + arg**
  - Input: Commands with same name but different actions
  - Expected: Both kept (not duplicates)
  - Validation: Both commands present

- **Test: Prefer command with patch over no patch**
  - Input: Duplicate commands, one with patch, one without
  - Expected: Version with patch kept
  - Validation: Result has patch field populated

- **Test: Prefer command with flags over no flags**
  - Input: Duplicate commands with identical patches, one has flags
  - Expected: Version with flags kept
  - Validation: Result has flags field populated

- **Test: Prefer anchor commands**
  - Input: Duplicate commands, one is anchor
  - Expected: Anchor version kept
  - Validation: Action is anchor

- **Test: Deduplication with no duplicates**
  - Input: All unique commands
  - Expected: All commands kept, count unchanged
  - Validation: Length matches input

- **Test: Generate dedup key**
  - Input: Command with name="test", action="markdown", arg="~/file.md"
  - Expected: Key is "test:markdown:~/file.md"
  - Validation: String equality

### 5. System Data (Singleton) Tests (`sys_data.rs`)

#### 5.1 Initialization
- **Test: Initialize sys data from scratch**
  - Input: First-time initialization
  - Expected: Config loaded, empty commands (or from cache), patches created
  - Validation: SysData populated, singleton set

- **Test: Initialize with missing config files**
  - Input: No config.yaml, commands.txt, or config.js
  - Expected: Installer GUI launched, process exits
  - Validation: Check that installer_gui is spawned

- **Test: Initialize with corrupted config**
  - Input: Invalid YAML in config file
  - Expected: Error returned, default config used
  - Validation: CONFIG singleton has default values

- **Test: Multiple initialization attempts**
  - Input: Call initialize() twice
  - Expected: Second call returns error (already initialized)
  - Validation: Error message about re-initialization

#### 5.2 Data Access
- **Test: Get config after initialization**
  - Expected: Returns cloned Config
  - Validation: Config values match initialized data

- **Test: Get config before initialization**
  - Expected: Panic with helpful message
  - Validation: Panic message mentions initialization requirement

- **Test: Get sys_data after initialization**
  - Expected: Returns (SysData, bool) tuple
  - Validation: SysData has config, commands, patches

- **Test: Get sys_data before initialization**
  - Expected: Panic with helpful message
  - Validation: Panic message clear

- **Test: Get commands**
  - Expected: Cloned Vec<Command> from singleton
  - Validation: Commands match initialized data

- **Test: Get patches**
  - Expected: Cloned HashMap<String, Patch>
  - Validation: Patches match initialized data

#### 5.3 Command Modification
- **Test: Set commands with history tracking**
  - Input: Modified commands list
  - Expected: Changes recorded to history, patches inferred, saved to disk
  - Validation: History database has new entries, singleton updated, files saved

- **Test: Add single command**
  - Input: New Command
  - Expected: Command added, history recorded, inference run, saved
  - Validation: Command in singleton, history entry exists, file updated

- **Test: Delete command by name**
  - Input: Command name to delete
  - Expected: Command removed, deletion recorded in history, saved
  - Validation: Command not in singleton, history has deletion entry

- **Test: Set commands detects creations**
  - Input: Commands list with new commands vs. cache
  - Expected: New commands recorded as "created" in history
  - Validation: History entries with appropriate log messages

- **Test: Set commands detects modifications**
  - Input: Commands with changed action/patch/flags vs. cache
  - Expected: Modified commands recorded in history
  - Validation: History entries show old vs. new values

- **Test: Set commands detects deletions**
  - Input: Commands missing from new list
  - Expected: Deletions recorded with action="$DELETED$"
  - Validation: History has deletion entries

#### 5.4 Flush and Validation
- **Test: Flush runs patch validation**
  - Input: Commands with missing patches
  - Expected: Patches inferred during flush
  - Validation: Output commands have patches

- **Test: Flush runs deduplication**
  - Input: Commands with duplicates
  - Expected: Duplicates removed during flush
  - Validation: Output has no duplicates

- **Test: Flush saves to both cache and file**
  - Input: Modified commands
  - Expected: Both commands.txt and commands_cache.json updated
  - Validation: Both files exist with matching data

- **Test: Flush updates singleton**
  - Input: Modified commands
  - Expected: Singleton SysData updated with new commands and patches
  - Validation: Next get_sys_data() returns updated data

#### 5.5 State Access
- **Test: Get application state**
  - Expected: Loads state from disk
  - Validation: Returns AppState

- **Test: Set application state**
  - Input: Modified AppState
  - Expected: Saved to state.json
  - Validation: File updated

#### 5.6 History Access
- **Test: Get history entries via sys_data**
  - Input: Limit and exclude_deletions parameters
  - Expected: Trampoline to history module works
  - Validation: Returns Vec<HistoryEntry>

### 6. Integration Tests (Cross-Module)

#### 6.1 Full Data Flow
- **Test: Complete initialization → modification → persistence cycle**
  - Steps:
    1. Initialize sys_data
    2. Get commands
    3. Modify commands
    4. Call set_commands()
    5. Verify persistence to disk
    6. Re-initialize from scratch
    7. Verify data matches
  - Validation: Data survives full cycle

#### 6.2 Config + Commands Integration
- **Test: Config file_roots used by scanner**
  - Input: Config with file_roots set
  - Expected: Scanner uses these paths
  - Validation: Commands created from files in configured roots

- **Test: Config word_separators affects command parsing**
  - Input: Config with custom word_separators
  - Expected: Command parsing respects separators
  - Validation: Commands split correctly

#### 6.3 History + Commands Integration
- **Test: Command lifecycle fully tracked**
  - Steps:
    1. Create command → check history
    2. Modify command → check history
    3. Delete command → check history
  - Expected: All changes recorded
  - Validation: History database has all entries

#### 6.4 State + Commands Integration
- **Test: Last executed command tracked**
  - Steps:
    1. Execute command
    2. Update state.last_executed_command
    3. Save state
    4. Reload state
  - Expected: Last command preserved
  - Validation: State matches

- **Test: Anchor state affects command filtering**
  - Input: State with anchor_name set
  - Expected: Command display respects anchor context
  - Validation: Filtered commands match anchor

### 7. Error Handling and Edge Cases

#### 7.1 Filesystem Errors
- **Test: Handle read-only filesystem**
  - Input: Attempt to save when filesystem is read-only
  - Expected: Graceful error, helpful message
  - Validation: Error explains permission issue

- **Test: Handle out of disk space**
  - Input: Attempt to save when disk is full
  - Expected: Error with disk space message
  - Validation: Original file not corrupted

- **Test: Handle concurrent access**
  - Input: Multiple processes accessing same files
  - Expected: No data corruption
  - Validation: File integrity maintained

#### 7.2 Database Errors
- **Test: Handle corrupted history database**
  - Input: Invalid SQLite file
  - Expected: Attempt to recreate or error
  - Validation: Error message helpful

- **Test: Handle database lock timeout**
  - Input: Database locked by another process
  - Expected: Timeout error with retry suggestion
  - Validation: Error message explains issue

#### 7.3 Data Corruption Detection
- **Test: Detect and prevent command inflation**
  - Input: Suspicious command growth (>10,000)
  - Expected: Save blocked with error
  - Validation: Original data preserved

- **Test: Detect and prevent patch stripping**
  - Input: Mass loss of patches (>200 empty)
  - Expected: Save blocked with error
  - Validation: Error logged, data not saved

#### 7.4 Migration and Backwards Compatibility
- **Test: Load old format commands.txt (without version header)**
  - Input: Legacy commands file format
  - Expected: Commands parsed correctly
  - Validation: All commands loaded

- **Test: Migrate history schema automatically**
  - Input: Old history database with change_type column
  - Expected: Schema migrated on initialization
  - Validation: New schema in place, data preserved

### 8. Performance Tests

#### 8.1 Load Performance
- **Test: Load 1,000 commands performance**
  - Expected: < 100ms
  - Validation: Measure time

- **Test: Load 5,000 commands performance**
  - Expected: < 500ms
  - Validation: Measure time

- **Test: Parse complex config performance**
  - Expected: < 50ms
  - Validation: Measure config loading time

#### 8.2 Query Performance
- **Test: History query with 10,000+ entries**
  - Expected: < 100ms for limited query
  - Validation: Measure query time

- **Test: Command deduplication performance**
  - Input: 5,000 commands with 20% duplicates
  - Expected: < 200ms
  - Validation: Measure dedup time

#### 8.3 Save Performance
- **Test: Save 1,000 commands with backup**
  - Expected: < 200ms
  - Validation: Measure save + backup time

- **Test: Cache serialization performance**
  - Input: 5,000 commands to JSON
  - Expected: < 300ms
  - Validation: Measure JSON write time

## Test Implementation Priority

### Phase 1: Critical Foundation (High Priority)
1. Storage layer: load/save commands (4.2, 4.3)
2. Config error handling and defaults (1.1, 1.3)
3. Config parsing helpers with error logging (1.2)
4. State persistence (3.1)
5. System data initialization (5.1, 5.2)

### Phase 2: Core Functionality (Medium Priority)
6. History recording and querying (2.2, 2.3)
7. Deduplication logic (4.5)
8. Command modification (5.3)
9. Integration tests (6.1, 6.3)

### Phase 3: Polish and Robustness (Lower Priority)
10. Error handling (7.1, 7.2, 7.3)
11. Performance tests (8.1, 8.2, 8.3)
12. Edge cases (remaining tests)

## Test Infrastructure Needs

### Test Utilities
- **Temporary directory management**: Create isolated test environments
- **Mock filesystem**: For testing error conditions
- **Database fixtures**: Pre-populated test databases
- **Config builders**: Fluent API for creating test configs
- **Command factories**: Easy creation of test commands

### Test Data
- Sample valid configurations (minimal, full, edge cases)
- Sample commands.txt files (various sizes, formats)
- Sample history databases (empty, small, large, old schema)
- Corrupted data samples (invalid JSON, bad YAML, corrupt DB)

### Assertion Helpers
- `assert_config_eq!()` - Deep equality for Config
- `assert_command_eq!()` - Compare commands ignoring internal fields
- `assert_file_exists!()` - File existence checks
- `assert_logs_contain!()` - Check log output

## Success Criteria

- **Coverage**: Aim for >90% line coverage in data crate
- **Reliability**: All tests pass consistently (no flaky tests)
- **Performance**: All performance tests meet targets
- **Documentation**: Each test has clear description and validation
- **Maintainability**: Tests easy to understand and modify

## Notes

- All tests should be independent and idempotent
- Use temporary directories for file I/O tests
- Clean up test artifacts after each test
- Consider parallel test execution (tests must not conflict)
- Mock external dependencies where appropriate
- Use property-based testing for complex logic (deduplication, parsing)
