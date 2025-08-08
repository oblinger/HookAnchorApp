# Command System & Anchors

## Overview

The command system is the heart of HookAnchor, providing a flexible, extensible way to organize and execute actions. Commands can be simple shell scripts, complex JavaScript functions, application launches, or URL opens. The anchor system provides hierarchical organization through markdown files.

## Command Structure

### Core Command Type
```rust
pub struct Command {
    pub patch: String,      // Group identifier (e.g., "dev", "work")
    pub command: String,    // Display name
    pub action: String,     // Action type (cmd, app, url, folder, etc.)
    pub arg: String,        // Action argument
    pub flags: String,      // Special flags (W=window, C=copy, etc.)
    pub full_line: String,  // Original line from file
}
```

### Command Format in Files
```yaml
# In commands.yaml
patch command_name action argument [flags]

# Examples:
dev Terminal app Terminal
work Slack Meeting url https://slack.com/meeting
proj Build Project cmd cargo build --release W
```

## Action Types

### 1. Shell Commands (`cmd`)
Execute shell commands with optional JavaScript preprocessing:
```yaml
dev Build cmd cargo build --release
dev Test cmd npm test W
```
- `W` flag: Keep window open after execution
- JavaScript variables expanded

### 2. Applications (`app`)
Launch macOS applications:
```yaml
tools Calculator app Calculator
dev VS Code app "Visual Studio Code"
```

### 3. URLs (`url`)
Open web URLs in default browser:
```yaml
web GitHub url https://github.com
docs MDN url https://developer.mozilla.org
```

### 4. Folders (`folder`)
Open folders in Finder:
```yaml
nav Downloads folder ~/Downloads
nav Documents folder ~/Documents
```

### 5. Aliases (`alias`)
Reference other commands:
```yaml
quick Build alias dev:Build_Project
```

### 6. Special Actions
- `separator`: Visual separator in menu
- `patch`: Group commands under a patch
- `scan`: Trigger filesystem scan

## Anchor System

### Concept
Anchors are markdown files that serve as organizational hubs for related commands. They enable:
- Hierarchical project organization
- Automatic command discovery
- Context-aware command grouping

### Directory Structure
```
~/Documents/HookAnchor/Anchors/
├── ProjectA/
│   ├── ProjectA.md       # Anchor file
│   ├── Build.md          # Related command
│   └── patches/
│       ├── dev.md        # Patch definitions
│       └── test.md
├── Work/
│   ├── Work.md           # Work anchor
│   └── Meetings.md       # Work commands
└── Tools/
    └── Tools.md          # Tools anchor
```

### Anchor Files
Must follow naming convention: `FolderName/FolderName.md`

Example anchor file:
```markdown
# ProjectA

Main project for client work.

## Quick Actions
- Build: cmd:cargo build
- Test: cmd:cargo test
- Deploy: url:https://deploy.example.com
```

### Alias Anchors
When markdown filename differs from folder name:
```
HookAnchor/
├── HA.md         # Alias for HookAnchor
└── README.md
```

## Patch System

### Purpose
Patches group related commands for:
- Submenu organization
- Visual grouping
- Context switching

### Patch Definition
```yaml
# Simple patch
dev Terminal app Terminal

# Patch with priority (! suffix)
dev! Important Command cmd echo "priority"

# Multi-level patches
dev:frontend Build UI cmd npm run build
dev:backend Build API cmd cargo build
```

### Patch Behavior
1. **Submenu Creation**: Commands with same patch show in submenu
2. **Priority Sorting**: `!` suffix commands appear first
3. **Exact Match First**: Patch name always appears first in its submenu

## Search & Filtering

### Fuzzy Matching Algorithm
```rust
1. Check each character of search term
2. Find characters in order (not necessarily consecutive)
3. Score based on:
   - Match position (earlier = better)
   - Consecutive matches (bonus)
   - Word boundaries (bonus)
   - Case match (bonus)
```

### Search Priority
1. **Exact matches** (case-insensitive)
2. **Prefix matches**
3. **Word boundary matches**
4. **Fuzzy matches**
5. **Patch matches** (for short searches ≤3 chars)

### Submenu Detection
```rust
// Triggered when:
- Multiple commands share prefix
- Multiple commands share patch
- Search matches patch name
```

## Command Merging

### Similar Command Detection
Commands are merged when:
1. Same prefix up to word boundary
2. Same action type
3. Similar arguments

### Merge Representation
```
Terminal ...          # Merged from multiple Terminal commands
Build ...            # Merged from Build Debug, Build Release
```

### Merge Expansion
- Click to expand merged group
- Shows all original commands
- Maintains individual execution

## Orphan Management

### What Are Orphans?
Commands without a matching anchor folder:
- Created from scattered markdown files
- Leftover from deleted projects
- Temporary or experimental commands

### Orphan Processing
```
1. Scanner finds .md files
2. Check for matching anchor
3. No anchor → move to Orphans/
4. Consolidate if similar exists
5. Create from template if enabled
```

### Orphan Merging
When orphan matches existing anchor:
```rust
// Check for:
1. Exact folder name match
2. Alias anchor (HA.md → HookAnchor)
3. Move orphan to anchor location
4. Update command database
```

## Command Lifecycle

### 1. Discovery
```
File System → Scanner → Parse Markdown → Create Command
```

### 2. Storage
```
Commands → Dedup → Sort → Save to commands.yaml
```

### 3. Loading
```
Read YAML → Parse → Validate → Build Index → Memory
```

### 4. Execution
```
Select → Resolve Aliases → Expand Templates → Execute
```

## Performance Optimizations

### Caching Strategy
- Commands cached in memory
- Incremental scanner updates
- Lazy patch resolution

### Deduplication
```rust
// Remove duplicates based on:
1. Exact command match
2. Same patch + name + action
3. Prefer commands with flags
```

### Indexing
- Pre-built search indices
- Patch lookup tables
- Alias resolution cache

## File Formats

### commands.yaml
```yaml
# Generated file - do not edit manually
# Format: patch command action argument flags

dev Terminal app Terminal
dev Build cmd cargo build W
work Slack app Slack
work Meeting url https://meet.google.com
```

### Markdown Commands
```markdown
# Project Name

## Commands
- Build: `cargo build --release`
- Test: `cargo test`
- [Deploy](https://deploy.example.com)
- Open in [VS Code](cmd:code .)
```

## Advanced Features

### JavaScript Integration
Commands can use JavaScript functions:
```yaml
dev Time cmd {{js:new Date().toLocaleTimeString()}}
dev Random cmd echo {{js:Math.random()}}
```

### Template System
Commands can reference templates:
```yaml
template:github_issue Create Issue
```

### Conditional Commands
Using JavaScript for conditions:
```yaml
dev Build cmd {{js:isDev() ? 'npm run dev' : 'npm run build'}}
```

## Error Handling

### Invalid Commands
- Logged but not displayed
- User notification for critical errors
- Graceful degradation

### Missing Resources
- Applications: Suggest alternatives
- URLs: Validate before launch
- Folders: Create if configured

## Testing

### Unit Tests
```bash
cargo test commands::
```
- Command parsing
- Search algorithm
- Merge logic

### Integration Tests
```bash
./test_command_system
```
- End-to-end command flow
- Scanner integration
- Execution verification

## Future Enhancements

1. **Command History**
   - Track usage frequency
   - Suggest based on context
   - Learn user patterns

2. **Smart Grouping**
   - AI-based categorization
   - Dynamic patch creation
   - Context awareness

3. **Command Sharing**
   - Export/import command sets
   - Team synchronization
   - Version control

## Related Documentation

- [Scanner System](scanner-system.md) - How commands are discovered
- [Configuration](configuration.md) - Command customization
- [Launcher System](launcher-system.md) - Command execution
- [Popup System](popup-system.md) - Command display