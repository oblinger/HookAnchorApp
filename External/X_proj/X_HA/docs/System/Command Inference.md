# HookAnchor Patch Inference and Command Management Rules

This document describes all the automatic actions that HookAnchor takes with respect to command data, organized by precedence and scenario.

## Section 1: Patch Inference Rules (When Patch is Blank)

When a command has an empty patch field, the system attempts to infer an appropriate patch using these rules in precedence order:

### Rule 1: Direct Command Name Match
- **Condition**: Command name exactly matches an existing patch name (case-insensitive)
- **Action**: Assign the matching patch name
- **Example**: Command "KMR" matches patch "kmr" → assign patch "kmr"

### Rule 2: Parent Directory Match (for file/folder actions)
- **Condition**: Command is a file/folder path and parent directory matches a patch name
- **Action**: Assign the parent directory patch
- **Example**: File "/Users/user/proj/MyProject/file.md" with patch "MyProject" → assign "MyProject"

### Rule 3: Substring Match with Existing Patches
- **Condition**: Command name contains a substring that matches an existing patch
- **Action**: Assign the longest matching patch name
- **Example**: Command "Project Alpha Notes" matches patch "Project Alpha" → assign "Project Alpha"

### Rule 4: Path-based Inference (for anchor/markdown actions)
- **Condition**: Command points to a file path that contains patch-like directory names
- **Action**: Extract patch name from the file path structure
- **Example**: Path "/Users/user/patches/WorkProject/file.md" → infer patch "WorkProject"

### Rule 5: Action-based Grouping
- **Condition**: No path or name matches found
- **Action**: Group by action type with fallback patches
- **Examples**:
  - `action: "notion"` → patch "notion"
  - `action: "work"` → patch "work"
  - `action: "1pass"` → patch "security"

### Rule 6: Default Fallback
- **Condition**: No other rules apply
- **Action**: Assign patch "misc" or "uncategorized"

## Section 2: Patch Update Rules (When Patch is Not Blank)

These rules determine when an existing patch assignment gets updated:

### Rule 1: Case Normalization
- **Condition**: Patch exists but case doesn't match the canonical form
- **Action**: Update patch case to match the first anchor command's case
- **Example**: Patch "myproject" → "MyProject" (if anchor command is "MyProject")

### Rule 2: Patch Hierarchy Optimization
- **Condition**: Current patch is too generic when a more specific patch exists
- **Action**: Replace with more specific patch
- **Example**: Patch "work" → "ProjectAlpha" if command relates to ProjectAlpha

### Rule 3: Parent-Child Patch Resolution
- **Condition**: Command has patch A but should belong to patch B based on file location
- **Action**: Update to the more appropriate parent patch
- **Example**: File in "/ProjectAlpha/subdir/" with patch "misc" → update to "ProjectAlpha"

### Rule 4: Duplicate Patch Consolidation
- **Condition**: Multiple patch variants exist for the same logical project
- **Action**: Consolidate to the canonical patch name
- **Example**: Patches "proj-alpha", "ProjectAlpha", "project_alpha" → all become "ProjectAlpha"

## Section 3: Automatic Command Creation Rules

The system automatically creates new commands in these scenarios:

### Rule 1: Orphan Anchor Creation
- **Trigger**: Patch exists in commands but no corresponding anchor folder/file found
- **Action**: Create anchor folder and markdown file in orphans directory
- **Location**: `{orphans_path}/{patch_name}/{patch_name}.md`
- **Command Format**: `{patch}! {patch} : anchor A; {file_path}`
- **Example**: Patch "NewProject" referenced but no anchor → create orphan anchor

### Rule 2: Markdown File Discovery
- **Trigger**: Markdown files found during filesystem scan without corresponding commands
- **Action**: Create anchor or markdown commands for discovered files
- **Rules**:
  - If filename matches folder name → create anchor command
  - Otherwise → create markdown command
- **Example**: Find `/proj/MyProject/MyProject.md` → create anchor command

### Rule 3: Root Patch Guarantee
- **Trigger**: System startup or patch validation
- **Action**: Ensure "orphans" root patch always exists
- **Purpose**: Prevent cycles in patch hierarchy graph
- **Command**: `Orphans : anchor; {orphans_path}/Orphans/Orphans.md`

### Rule 4: Alias Expansion
- **Trigger**: User creates alias command
- **Action**: Create new command entry with alias pointing to original
- **Format**: `{alias_name} : alias; {original_command}`
- **Example**: User aliases "notes" → "Daily Notes" creates alias command

### Rule 5: URL Handler Command Creation
- **Trigger**: URL scheme registration (hook://, obsidian://, etc.)
- **Action**: Create commands for handling special URL schemes
- **Example**: `hook://project_alpha` → create command that launches Project Alpha

## Section 4: Data Consistency and Cleanup Rules

### Rule 1: Duplicate Command Removal
- **Trigger**: Multiple commands with identical command+action+arg combinations
- **Action**: Remove duplicates, keeping the first occurrence
- **Preservation**: Maintain original case and patch assignments

### Rule 2: Invalid Path Cleanup
- **Trigger**: Commands pointing to non-existent files/folders
- **Action**: Mark for cleanup or update paths if file moved
- **Options**: User confirmation for deletion vs. path correction

### Rule 3: Flag Normalization
- **Trigger**: Commands with inconsistent or invalid flags
- **Action**: Standardize flags (U, A, M, etc.) according to command type
- **Example**: Auto-generated commands get "A" flag

### Rule 4: Action Validation
- **Trigger**: Commands with invalid or deprecated action types
- **Action**: Update to valid actions from configured `listed_actions`
- **Fallback**: Convert unknown actions to "cmd" or "url" as appropriate

## Section 5: Precedence and Conflict Resolution

### Priority Order for Patch Assignment:
1. Explicit user assignment (highest priority)
2. Anchor command match
3. File path inference
4. Directory structure inference
5. Action-based grouping
6. Default fallback (lowest priority)

### Conflict Resolution Rules:
- **Case Conflicts**: First anchor command wins for case
- **Path Conflicts**: More specific path beats generic path
- **Action Conflicts**: Explicit user action beats inferred action
- **Timing**: Later rules can override earlier rules only if more specific

## Section 6: Performance and Optimization

### Caching Strategy:
- Patch lookup maps cached for performance
- File system scans throttled to avoid excessive I/O
- Inference results cached until next data save

### Batch Processing:
- All inference runs in batch mode during data loading
- Changes accumulated and applied atomically
- Single save operation for all changes

### Logging and Debug:
- All automatic actions logged with `AUTO_UPDATE`, `ORPHAN_DEBUG` prefixes
- Patch inference logged with `PATCH_INFERENCE` prefix
- Performance metrics logged for optimization

---

*This document reflects the current state of the HookAnchor inference system as of the latest codebase analysis.*