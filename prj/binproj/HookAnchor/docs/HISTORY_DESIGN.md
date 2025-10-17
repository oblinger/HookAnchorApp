2# History Module Design

## Overview
The history module maintains markdown files that track changes to markdown files in the vault. Each history file records new files and new entries (H1/H2 headings) from markdown files within its scope. History files are associated with anchors and follow the patch hierarchy.

## Core Concepts

### History Files
- A markdown file becomes a "history file" if its filename (without extension) ends with "history" (case-insensitive) and it is in the same folder as a patch structure.
- Example: `Project History.md`, `work history.md`, `HISTORY.md`
- History files track changes to all markdown files within their anchor's scope

### Scope and Hierarchy
- A history file tracks changes for:
  1. Files directly in the anchor's folder
  2. Files in any subfolder of the anchor
  3. Files in subfolders that have their own anchors (which become child patches)
- Changes propagate UP the patch hierarchy:
  - When a file changes, it's added to the containing anchor's history
  - Then added to the parent patch's history (if parent has a history file)
  - Continues up to the root of the patch DAG

### History Cache
- Location: `~/.config/hookanchor/history_cache.json`
- Tracks all markdown files in the vault with their metadata:
  - File path (key)
  - Current file size (for detecting modifications)
  - Last modification time (for detecting Log section changes)
  - **Last history size** (size when last whole-file entry was added - for growth tracking)

### Detection of New Entries

#### New Files
- Any markdown file that appears in the vault for the first time
- Entry format: `YYYY-MM-DD\t[[filename]]`

#### File Growth Entries
- When an existing file grows by more than `history_increment_size` bytes (default: 1000)
- Adds another entry for the same file
- Entry format: `YYYY-MM-DD\t[[filename]]`
- Only tracks **growth** (size increases), not all changes
- Independent of Log Section entries below

#### Log Section Entries
- Files can have a "# Log" section (configurable via `history.anchor_log_indicator`)
- Any H1 or H2 tags under the Log section are treated as new entries (configurable via `history.anchor_levels`)
- These are tracked **separately** from whole-file entries above
- New H1/H2 tags are detected whenever the file is modified
- Entry format varies:
  - If H tag starts with date: `YYYY-MM-DD\t[[filename#heading]]\trest of heading text after date`
  - If H tag has no date: `YYYY-MM-DD\t[[filename#heading]]\theading text`

## Configuration

### New Section: `history` in config.yaml

```yaml
history:
  # String to look for that indicates a log section in markdown files
  anchor_log_indicator: "# Log"

  # Heading levels to treat as log entries (1 = H1, 2 = H2, etc.)
  anchor_levels: [1, 2]

  # Minimum file size increase (in bytes) to add a new whole-file entry
  # If a file grows by this many bytes, add another entry to history
  history_increment_size: 1000

  # Enable/disable history tracking entirely
  enabled: true
```

## Patch Hierarchy Overview

The patch system forms a DAG (Directed Acyclic Graph) where:
- Each `Command` has a `patch: String` field that names its parent patch
- Each `Patch` has anchor commands (files like `ProjectName/ProjectName.md`)
- To traverse from patch to parent: `patch.primary_anchor().patch` gives parent name
- The root is "orphans" - patches with empty patch field or patch="orphans"

### Folder→Patch Mapping

For efficient file→patch lookup:
1. Build a HashMap mapping folder paths to patch names
2. For each file, look up its folder
3. If not found, try parent folder recursively
4. Once patch found, walk up hierarchy using patch.primary_anchor().patch

## Data Structures

### FolderToPatchMap
```rust
/// Pre-built mapping from folder paths to patch names for O(1) lookup
type FolderToPatchMap = HashMap<PathBuf, String>;
```

### HistoryCache
```rust
/// Cache tracking all markdown files and their modification state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryCache {
    /// Map from file path to file metadata
    pub files: HashMap<PathBuf, FileMetadata>,
}

/// Metadata for a single file in the history cache
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileMetadata {
    /// Current file size in bytes
    pub size: u64,

    /// Last modification time (Unix timestamp)
    pub mtime: i64,

    /// Size when last whole-file history entry was added
    /// Used to track growth for history_increment_size threshold
    pub last_history_size: u64,
}
```

### HistoryEntry
```rust
/// Represents a single entry to be added to history files
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HistoryEntry {
    /// Date of the entry (from file mtime or H tag)
    pub date: String,  // Format: YYYY-MM-DD

    /// Wiki link to the file or heading
    pub link: String,  // Format: [[filename]] or [[filename#heading]]

    /// Optional additional text after the link
    pub text: Option<String>,
}
```

### PatchHistory
```rust
/// Tracks which history files need updating and what entries to add
struct PatchHistory {
    /// Path to the history markdown file
    history_path: PathBuf,

    /// List of entries to add to this history
    entries: Vec<HistoryEntry>,
}
```

### Patch Enhancement
```rust
/// Add to existing Patch struct in commands.rs
impl Patch {
    /// Path to this patch's history file, if it exists
    pub history_file: Option<PathBuf>,
}
```

## Key Functions

### Public API

#### `update_histories(sys_data: &SysData, rebuild_all: bool) -> Result<(), Box<dyn Error>>`
**Main entry point** - called from `run_rescan_command()` in cmd.rs.
- Takes existing SysData (commands and patches already loaded)
- Scans all markdown files in vault
- Detects changes based on cache
- Updates all relevant history files
- **Parameters:**
  - `sys_data`: The loaded SysData with commands and patches HashMap
  - `rebuild_all`: If true, ignore cache and rebuild all histories from scratch
- **Returns:** Ok(()) or error
- **Called from**: `run_rescan_command()` in cmd.rs after `scan_verbose()` completes

#### `get_history_for_patch(patch_name: &str) -> Result<Vec<HistoryEntry>, Box<dyn Error>>`
Retrieves the history entries for a specific patch.
- Reads the patch's history file
- Parses entries into structured format
- **Parameters:**
  - `patch_name`: Name of the patch to get history for
- **Returns:** List of history entries or error

### Existing Functions to Use (DO NOT REIMPLEMENT)

#### `build_folder_to_patch_map(commands: &[Command]) -> HashMap<PathBuf, String>`
**FROM inference.rs:315** - Already implemented, just call it!
- Builds HashMap from absolute folder paths to patch names
- Uses anchor commands to create the mapping

#### `infer_patch_simple(file_path: &str, folder_map: &HashMap<PathBuf, String>) -> Option<String>`
**FROM inference.rs:342** - Already implemented, just call it!
- Given a file path, walks up directories using folder_map to find containing patch
- Returns patch name or None

#### `get_patch_path(patch_name: &str, patches: &HashMap<String, Patch>) -> Vec<String>`
**FROM commands.rs:1167** - Already implemented, just call it!
- Given a patch name, returns all ancestor patches up to root
- Excludes "orphans" from result

### New Internal Functions to Implement

#### `load_history_cache() -> HistoryCache`
Loads the history cache from disk.
- **Location:** `~/.config/hookanchor/history_cache.json`
- **Returns:** Existing cache or new empty cache if file doesn't exist

#### `save_history_cache(cache: &HistoryCache) -> Result<(), Box<dyn Error>>`
Saves the history cache to disk.
- **Parameters:**
  - `cache`: The cache to save
- **Returns:** Ok(()) or error

#### `scan_markdown_files(config: &Config) -> Result<Vec<PathBuf>, Box<dyn Error>>`
Scans all configured file roots for markdown files.
- **Parameters:**
  - `config`: Application configuration
- **Returns:** List of all markdown file paths
- **Note:** Uses config.popup_settings.file_roots for scan roots

#### `detect_new_file(file_path: &Path, cache: &HistoryCache) -> bool`
Checks if a file is new (not in cache).
- **Parameters:**
  - `file_path`: Path to check
  - `cache`: Current history cache
- **Returns:** true if file is new

#### `detect_file_growth(file_path: &Path, cache: &HistoryCache, threshold: u64) -> Result<bool, Box<dyn Error>>`
Checks if a file has grown by more than the threshold since last whole-file entry.
- Compares current size with `last_history_size` in cache
- **Parameters:**
  - `file_path`: Path to check
  - `cache`: Current history cache
  - `threshold`: Minimum growth in bytes (from config.history.history_increment_size)
- **Returns:** true if file grew by more than threshold bytes

#### `detect_file_modification(file_path: &Path, cache: &HistoryCache) -> Result<bool, Box<dyn Error>>`
Checks if a file has been modified since last scan (for Log section parsing).
- Compares mtime with cached value
- **Parameters:**
  - `file_path`: Path to check
  - `cache`: Current history cache
- **Returns:** true if file was modified (mtime changed)

#### `parse_log_section(file_path: &Path, config: &Config) -> Result<Vec<HistoryEntry>, Box<dyn Error>>`
Parses a markdown file's Log section for new entries.
- Looks for log section indicator (e.g., "# Log")
- Extracts H1/H2 entries based on config
- Parses dates from heading text if present
- **Parameters:**
  - `file_path`: Path to markdown file
  - `config`: Configuration (for log indicator and levels)
- **Returns:** List of history entries found in log section

#### `parse_heading_date(heading_text: &str) -> Option<String>`
Extracts date from heading text if it starts with YYYY-MM-DD format.
- **Parameters:**
  - `heading_text`: The heading text to parse
- **Returns:** Date string or None

#### `create_file_entry(file_path: &Path, mtime: i64) -> HistoryEntry`
Creates a history entry for a new file.
- **Parameters:**
  - `file_path`: Path to the file
  - `mtime`: File modification time
- **Returns:** HistoryEntry with wiki link to file

#### `create_heading_entry(file_path: &Path, heading_text: &str, mtime: i64) -> HistoryEntry`
Creates a history entry for a log heading.
- Handles date extraction from heading
- Creates wiki link with heading anchor
- **Parameters:**
  - `file_path`: Path to the file
  - `heading_text`: The heading text (without # markers)
  - `mtime`: File modification time (used if no date in heading)
- **Returns:** HistoryEntry with wiki link to heading

#### `collect_history_updates(entries: Vec<HistoryEntry>, patch_name: &str, patches: &HashMap<String, Patch>) -> HashMap<PathBuf, Vec<HistoryEntry>>`
Collects which history files need which entries.
- Adds entries to containing anchor's history
- Propagates up through parent patches
- Only adds to patches that have history files
- **Parameters:**
  - `entries`: Entries to add
  - `patch_name`: Starting patch name
  - `patches`: HashMap of all patches
- **Returns:** Map from history file path to list of entries to add

#### `update_history_files(updates: HashMap<PathBuf, Vec<HistoryEntry>>, rebuild_all: bool) -> Result<(), Box<dyn Error>>`
Writes updated entries to history files.
- Sorts entries in reverse chronological order
- If rebuild_all: overwrites file with new entries
- If incremental: prepends new entries to existing content
- **Parameters:**
  - `updates`: Map of history files to update
  - `rebuild_all`: Whether to rebuild from scratch
- **Returns:** Ok(()) or error

#### `format_history_entry(entry: &HistoryEntry) -> String`
Formats a history entry as a markdown line.
- Format without text: `YYYY-MM-DD\t[[link]]`
- Format with text: `YYYY-MM-DD\t[[link]]\ttext`
- **Parameters:**
  - `entry`: The entry to format
- **Returns:** Formatted string

#### `identify_history_files(patches: &HashMap<String, Patch>) -> HashMap<String, PathBuf>`
Identifies which patches have history files.
- Checks each patch's anchor for files ending in "history.md" (case-insensitive)
- Updates Patch.history_file field
- **Parameters:**
  - `patches`: HashMap of all patches
- **Returns:** Map from patch name to history file path

## Algorithm Flow

### Main Scan Process (`update_histories`)

Called from `run_rescan_command()` **after** `load_data()` and `scan_verbose()` complete.

1. **Initialization** (uses data already loaded by caller)
   - Receive SysData with commands and patches already loaded
   - Build folder→patch map: `build_folder_to_patch_map(&sys_data.commands)`
   - Load history cache from `~/.config/hookanchor/history_cache.json`
   - Identify which patches have history files (check for files ending in "history.md")

2. **File Discovery**
   - Scan all file roots for markdown files
   - Skip hidden files and directories
   - Skip files matching skip_directory_patterns

3. **Change Detection** (for each file)
   - Check if file is new (not in cache) → creates whole-file entry
   - Check if file has grown by `history_increment_size` bytes → creates whole-file entry
   - Check if file is modified (mtime changed) → parse for new Log entries
   - Skip unchanged files (unless rebuild_all is true)

4. **Entry Collection** (for each changed/new file)
   - **Whole-file entries:**
     - If new file: create file entry with current mtime
     - If file grew by threshold: create file entry with current mtime
     - Update `last_history_size` in cache to current size
   - **Log section entries:**
     - If file modified: parse log section for new H1/H2 entries
     - Compare with previously seen entries to identify new ones
   - **Find containing patch using existing infrastructure:**
     1. Call `infer_patch_simple(file_path, &folder_map)` from inference.rs
     2. This walks up the directory tree to find the containing patch
   - If no patch found, skip file
   - **Walk up patch hierarchy using existing infrastructure:**
     1. Call `get_patch_path(patch_name, &sys_data.patches)` from commands.rs
     2. This returns all ancestor patches up to "orphans"
   - Add entries to all patches in the hierarchy that have history files

5. **History Update**
   - For each history file with new entries:
     - Sort entries in reverse chronological order
     - If rebuild_all: overwrite file
     - If incremental: prepend to existing content
   - Save updated entries to disk

6. **Cache Update**
   - Update cache with new file metadata
   - Save cache to disk

### Rebuild Process (`rebuild_all = true`)

When rebuild_all is true:
1. Treat ALL files as new
2. Parse log sections from ALL files
3. Clear existing history file contents
4. Write sorted entries to history files
5. Update cache with all current file states

## Error Handling

- File I/O errors: Log and continue with next file
- Parse errors: Log warning and skip entry
- Missing patch: Log warning and skip file
- Cache corruption: Start with fresh cache

## Testing Considerations

### Test Cases Needed

1. **New file detection**
   - New markdown file created → appears in history

2. **Log section parsing**
   - H1/H2 entries correctly extracted
   - Date parsing from headings
   - Non-log headings ignored

3. **Hierarchy propagation**
   - Entry added to containing anchor
   - Entry propagated to parent patches
   - Stops at root of DAG

4. **Cache management**
   - Cache persists across runs
   - Modified files detected correctly
   - Unchanged files skipped

5. **Rebuild mode**
   - All histories regenerated from scratch
   - Old entries removed
   - Sorted correctly

6. **Edge cases**
   - File outside any anchor → skipped
   - Circular patch references → handled
   - Missing history files → skipped
   - Malformed log sections → skipped

## Integration Points

### Called from `run_rescan_command()` in cmd.rs

The history update is called **automatically during rescan**, which happens during rebuild:

**Rebuild Process** (`run_rebuild_command()` at cmd.rs:1464):
1. **Step 1**: Build release binary (`cargo build --release`)
2. **Step 2**: Restart command server
3. **Step 3**: Call `run_rescan_command()` ← History update happens here

**Rescan Process** (`run_rescan_command()` at cmd.rs:1105):
1. Load config
2. Load existing commands from disk
3. **Call `load_data(commands, verbose=true)`** ← Loads SysData with patches HashMap
4. **Call `scan_verbose(commands, &global_data, true)`** ← Scans filesystem
5. **→ NEW: Call `update_histories(&global_data, rebuild_all=false)`** ← ADD THIS HERE
6. Print final summary

**Single Call Site**: Add one function call in `run_rescan_command()` after `scan_verbose()` completes:
```rust
// In run_rescan_command() after scan_verbose()
print("\n📝 Updating history files...");
if let Err(e) = crate::systems::history::update_histories(&global_data, false) {
    print(&format!("   ⚠️  History update failed: {}", e));
} else {
    print("   ✅ History files updated");
}
```

**Why this location?**
- SysData is already loaded (has `commands` and `patches` HashMap)
- Filesystem scan is complete (all new files detected)
- Called automatically on every `--rebuild` command
- Verbose mode already enabled for user feedback

### Config Integration
Add `history` section to config.yaml with settings

## Future Enhancements

1. **Incremental log parsing**: Only parse new log entries since last scan
2. **History queries**: Search/filter history entries
3. **History stats**: Count of entries per time period
4. **History export**: Export to different formats
5. **History UI**: Show history in popup
6. **History templates**: Customize entry format
