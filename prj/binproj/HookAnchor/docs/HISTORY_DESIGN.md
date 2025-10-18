# History System Design

## Overview

The history system tracks all changes to commands over time by maintaining:
1. A snapshot cache of current command state (`commands_cache.json`)
2. An append-only history database (`history.db`)

Changes are detected during `--rescan` by comparing cached state against current state (from `commands.txt` and filesystem).

## Data Model

### Extended Command Structure

```rust
pub struct Command {
    pub patch: String,
    pub command: String,
    pub action: String,
    pub arg: String,
    pub flags: String,

    // Metadata (for ALL commands)
    pub last_update: i64,        // Unix timestamp - when we last saw this command change

    // File metadata (only for file-based actions: anchor/file/folder)
    pub file_size: Option<u64>,  // File size when we last checked
}
```

### Files on Disk

1. **`~/.config/hookanchor/commands.txt`**
   - Human-editable text format (current format)
   - Loaded ONLY during `--rescan`

2. **`~/.config/hookanchor/commands_cache.json`**
   - JSON serialization of `Vec<Command>` with all metadata
   - Snapshot of "last known state" for comparison
   - Updated after every rescan

3. **`~/.config/hookanchor/history.db`** (SQLite)
   - Append-only history of all command changes
   - Queryable with filters (date range, path prefix, text search, patch)

## SQLite Schema

```sql
-- Command change history (append-only)
CREATE TABLE command_history (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    timestamp INTEGER NOT NULL,         -- Unix timestamp
    change_type TEXT NOT NULL,          -- 'created', 'modified', 'deleted'

    -- Command identity
    patch TEXT NOT NULL,
    command TEXT NOT NULL,
    action TEXT NOT NULL,

    -- Command details (NULL for 'deleted' type)
    arg TEXT,
    flags TEXT,
    file_path TEXT,

    -- What changed (for 'modified' type, JSON)
    changed_fields TEXT,                -- ["arg", "flags"]
    old_values TEXT,                    -- {"arg": "old_value"}
    new_values TEXT                     -- {"arg": "new_value"}
);

CREATE INDEX idx_command_history_timestamp ON command_history(timestamp);
CREATE INDEX idx_command_history_file_path ON command_history(file_path);
CREATE INDEX idx_command_history_patch ON command_history(patch);
```

## Rescan Workflow

```rust
fn rescan() {
    // 1. Load cached state (last known state)
    let cached = load_from_json("commands_cache.json");

    // 2. Load commands.txt (detect manual edits)
    let txt_commands = load_from_txt("commands.txt");
    for txt_cmd in txt_commands {
        if differs_from_cache(txt_cmd, cached) {
            update_command(txt_cmd); // Appends to history.db
        }
    }

    // 3. Scan filesystem (detect file changes)
    for cmd in file_based_commands() {
        let current_size = get_file_size(&cmd.arg);
        if current_size != cmd.file_size {
            update_command(cmd.with_new_size(current_size));
        }
    }

    // 4. Save updated cache
    save_to_json("commands_cache.json", &commands);
}
```

## Core Operations

### `update_command(old: Option<Command>, new: Command)`

**ONLY way to modify commands** - enforces history tracking.

1. Determine change type (created/modified/deleted)
2. INSERT entry into `history.db`
3. Update in-memory commands list
4. Mark cache as dirty

### `flush_commands()`

Persist changes to disk:
1. Save to `commands.txt` (human-readable)
2. Save to `commands_cache.json` (with metadata)

## Example Queries

### All changes in date range
```sql
SELECT * FROM command_history
WHERE timestamp BETWEEN ? AND ?
ORDER BY timestamp DESC;
```

### All changes under a path prefix
```sql
SELECT * FROM command_history
WHERE file_path LIKE '/Users/oblinger/ob/kmr/prj%'
ORDER BY timestamp DESC;
```

### All changes to a specific patch
```sql
SELECT * FROM command_history
WHERE patch = 'hookanchor'
ORDER BY timestamp DESC;
```

### Recent command creations
```sql
SELECT * FROM command_history
WHERE change_type = 'created'
ORDER BY timestamp DESC
LIMIT 100;
```

## Implementation Notes

- **Fast startup**: Load from `commands_cache.json`, don't rescan
- **Explicit rescan**: User runs `--rescan` when they want to check for changes
- **Single update path**: All command modifications go through `update_command()`
- **Append-only history**: Never delete from `history.db`, only append
- **No rebuild**: History is permanent, rescan only adds new entries
