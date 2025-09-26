# Command Rename Feature

## Overview
When renaming a command in the command editor, HookAnchor can automatically rename associated files, folders, patches, and command prefixes to keep everything in sync.

## Configuration
Add these settings to your `~/.config/hookanchor/config.yaml` under `popup_settings`:

```yaml
popup_settings:
  # Rename associated data when editing command names
  rename_doc:      true   # Rename document files when command name changes
  rename_folder:   true   # Rename anchor folders when command name changes  
  rename_patch:    true   # Update patch names and commands when patch is renamed
  rename_prefix:   false  # Update command prefixes when prefix command is renamed
```

## Features

### 1. Document Renaming (`rename_doc`)
When enabled, renaming a document command (markdown, text, doc, anchor) will also rename the physical file if:
- The file exists
- The file's base name matches the old command name (case-insensitive, ignoring spaces)
- No file with the new name already exists

Example:
- Command: `MyNotes` → `ProjectNotes`
- File: `/path/to/MyNotes.md` → `/path/to/ProjectNotes.md`

### 2. Folder Renaming (`rename_folder`)
When enabled, renaming an anchor command will also rename the folder if:
- It's an anchor command (action = "anchor")
- The folder name matches the old command name (case-insensitive)
- No folder with the new name already exists
- The anchor file inside will also be renamed to match

Example:
- Command: `MyProject` → `NewProject`
- Folder: `/path/MyProject/` → `/path/NewProject/`
- Anchor: `/path/MyProject/MyProject.md` → `/path/NewProject/NewProject.md`

### 3. Patch Renaming (`rename_patch`)
When enabled, renaming a command that is also a patch will:
- Update the patch name in the system
- Update all commands that have this patch to use the new name
- Handle both direct patches and "PatchName!" format

Example:
- Rename patch command: `Work` → `Office`
- All commands with patch `Work` or `Work!` will be updated to `Office` or `Office!`

### 4. Prefix Renaming (`rename_prefix`)
When enabled, renaming a command will update all commands that use it as a prefix:
- Only affects commands that start with the old name followed by a separator
- Separators are defined in `word_separators` config (default: " ._-")

Example:
- Command: `Test` → `Demo`
- `Test File` → `Demo File`
- `Test_Case` → `Demo_Case`
- But `TestCase` remains unchanged (no separator)

## Logging
All rename operations are logged to `~/.config/hookanchor/anchor.log` with the prefix `RENAME:` for debugging.

## Error Handling
- If a rename operation fails (e.g., target already exists), the save continues
- Error messages are logged but don't block the command save
- Physical file/folder renames are only attempted if safe

## Implementation Details
The rename functionality is implemented in:
- `src/core/commands.rs`: `rename_associated_data()` function
- `src/ui/popup.rs`: Integration with command editor save operation

The function uses existing name matching logic that:
- Ignores case differences
- Ignores spaces, underscores, and hyphens for matching
- Uses the same logic as the scanner for consistency