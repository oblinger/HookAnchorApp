# HookAnchor Installer Safety Documentation

## Overview
The HookAnchor installer system is designed to be **completely safe for reinstallation** and preserves all existing user data and configurations.

## Current Architecture (v0.8.0+)

### Build & Distribution Safety
- **Build Script**: `build_distribution.sh` creates clean distributions
- **Temporary Files**: Deleted immediately (not trashed) to prevent URL conflicts
- **URL Handler**: Isolated in separate embedded app to prevent system hangs
- **Universal Binary**: Single package works on Intel and Apple Silicon

## Safety Guarantees

### ✅ Configuration Files (`config.yaml`)
- **Never overwrites existing config files**
- Detects if `~/.config/hookanchor/config.yaml` exists
- If found: Validates content and preserves as-is
- If empty/corrupted: Creates backup before regenerating
- Only creates new config if none exists
- Default config copied from `Resources/default_config.yaml`

### ✅ Commands Database (Deprecated)
- **Legacy file**: `commands.txt` no longer actively used
- Commands now stored in `config.yaml` under anchors/commands sections
- If legacy file exists: Preserved but not modified
- Migration handled automatically on first run

### ✅ Karabiner Integration (`hookanchor.json`)
- **Smart update mechanism**
- Checks if existing modification has current app path
- If current: Leaves completely untouched
- If outdated: Creates backup (`.backup` suffix) then updates
- Only installs if none exists
- Default: Caps Lock trigger (not Cmd+Space to avoid conflicts)

### ✅ Directory Structure
- Uses `create_dir_all()` which never overwrites files
- Creates: `~/.config/hookanchor/`, `backups/`, `logs/`
- Existing files and subdirectories are preserved
- Command server socket preserved if running

### ✅ Application Bundle
- Installed to `/Applications/HookAnchor.app`
- Standard macOS app replacement behavior
- User can choose to keep or replace during drag-and-drop
- URL handler embedded in Resources (won't conflict)

## Reinstallation Scenarios

### Scenario 1: Clean Reinstall (No Existing Config)
```
User has: Nothing
Result: 
  ✓ Creates full default configuration
  ✓ Creates starter commands
  ✓ Installs Karabiner modification
  ✓ Sets up complete environment
```

### Scenario 2: Upgrade Existing Installation
```
User has: config.yaml, commands.txt with 50 commands, Karabiner mod
Result:
  ✓ Preserves existing config.yaml (no changes)
  ✓ Preserves all 50 existing commands (no changes)
  ✓ Updates Karabiner mod path if needed (creates backup)
  ✓ No data loss
```

### Scenario 3: Partial Configuration
```
User has: config.yaml only (no commands.txt)
Result:
  ✓ Preserves existing config.yaml
  ✓ Creates new commands.txt with starter examples
  ✓ Installs/updates Karabiner modification
```

### Scenario 4: Corrupted Files
```
User has: Empty config.yaml, corrupted commands.txt
Result:
  ✓ Backs up empty config → config.yaml.empty.backup
  ✓ Backs up corrupted commands → commands.txt.original.backup
  ✓ Creates fresh files
  ✓ Original files preserved as backups
```

## Backup Strategy

The installer automatically creates backups when needed:

- `config.yaml.empty.backup` - If existing config is empty
- `commands.txt.original.backup` - If existing commands file has no content
- `hookanchor.json.backup` - If Karabiner modification is updated

## First-Run Detection

Setup assistant only runs when:
- `~/.config/hookanchor/config.yaml` does **not** exist
- This means: reinstallation over existing config = **no setup assistant**
- User configuration is completely preserved

## User Data Preservation

**Never Modified:**
- User's custom commands
- User's configuration settings
- User's Karabiner profiles
- User's log files
- User's backup files

**Only Modified When:**
- File doesn't exist → create new
- File is empty/corrupted → backup then recreate
- Karabiner mod has wrong path → backup then update

## Testing Reinstallation Safety

Use the test script to verify safety:

```bash
# Test with existing config (should preserve everything)
./installer/scripts/test_installer.py

# Test clean install (should create defaults)
./installer/scripts/test_installer.py --clean-test
```

## Manual Safety Verification

Before reinstalling, you can verify your data will be preserved:

```bash
# Check what exists
ls -la ~/.config/hookanchor/
ls -la ~/.config/karabiner/assets/complex_modifications/hookanchor.json

# Your files will be preserved if they contain real content
```

## Recovery

In the unlikely event of issues:
1. Backups are created in `~/.config/hookanchor/`
2. Karabiner modifications backed up as `hookanchor.json.backup`
3. All operations are logged for troubleshooting

## Summary

**The installer is completely safe for reinstallation.** It follows the principle of "preserve everything, create only what's missing" and creates backups before any modifications.