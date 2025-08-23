# Scanner System

## Overview

The scanner system automatically discovers and indexes markdown files in configured directories, creating commands from their content and maintaining the anchor hierarchy.

## Architecture

```rust
pub struct ScannerSystem {
    config: Arc<Config>,
    last_scan: Instant,
    scan_interval: Duration,
    anchors_path: PathBuf,
    orphans_path: PathBuf,
}
```

## Scanning Process

### 1. Directory Traversal
```
Anchors Path → Walk Directories → Find .md Files → Parse Content
```

### 2. Anchor Detection
- **Valid Anchor**: `FolderName/FolderName.md`
- **Alias Anchor**: `FolderName/AliasName.md`
- **Orphan**: No matching folder structure

### 3. Command Extraction
From markdown files:
- Code blocks: `` `command` ``
- Links: `[name](action:argument)`
- Headers: Used as patch prefixes

### 4. Orphan Management
```rust
// Orphan consolidation logic
if orphan_exists {
    if matching_anchor_found {
        merge_with_anchor()
    } else {
        move_to_orphans_dir()
    }
}
```

## Performance

- **Incremental Scanning**: Only changed files
- **Parallel Processing**: Multi-threaded walk
- **Caching**: File modification times
- **Throttling**: Configurable scan interval

## Configuration

```yaml
scanner_settings:
  anchors_path: "~/Documents/HookAnchor/Anchors"
  orphans_path: "~/Documents/HookAnchor/Orphans"
  scan_interval: 300
  auto_scan: true
  scan_depth: 3
```

## File Processing

### Markdown Parsing
```markdown
# Project Name
patch: project

## Commands
- Build: `cargo build`
- [Documentation](https://docs.rs)
- Open: folder:~/projects/name
```

### Command Generation
```yaml
# Generated commands:
project Build cmd cargo build
project Documentation url https://docs.rs
project Open folder ~/projects/name
```

## Related Documentation
- [Command System](command-system.md)
- [Configuration](configuration.md)