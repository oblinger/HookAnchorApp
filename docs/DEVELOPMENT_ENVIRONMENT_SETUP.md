# Development Environment Setup

This document describes how to set up HookAnchor for **development** on macOS, where the Karabiner keyboard macro needs to access the latest built binaries without manual copying after each build.

## Overview

The development setup creates a **symlinked app bundle structure** in `/Applications/HookAnchor.app/` that simulates a real installation but points directly to the build folder. This allows:

- Karabiner complex macro to work exactly as in production
- Automatic access to latest built binaries after each `cargo build --release`
- No manual copying or installation steps during development

## Karabiner Macro Configuration

The Karabiner complex macro is configured to trigger HookAnchor via Caps Lock key:

```json
{
    "description": "Caps Lock: Hyper key when combined",
    "manipulators": [
        {
            "from": { "key_code": "caps_lock" },
            "to": [
                {
                    "key_code": "left_shift",
                    "modifiers": ["left_control", "left_option", "left_command"]
                }
            ],
            "to_if_alone": [{ "shell_command": "/Applications/HookAnchor.app/Contents/MacOS/popup" }],
            "type": "basic"
        }
    ]
}
```

**Key Point**: The macro directly calls `/Applications/HookAnchor.app/Contents/MacOS/popup`, expecting the popup binary to be at this exact location.

## Development Setup Steps

### 1. Enable Karabiner Complex Macro

In Karabiner-Elements settings, ensure the "Caps Lock: Hyper key when combined" rule is **enabled**. This rule should have:
- `to_if_alone`: `/Applications/HookAnchor.app/Contents/MacOS/popup`

### 2. Create App Bundle Directory Structure

```bash
sudo mkdir -p /Applications/HookAnchor.app/Contents/MacOS
```

### 3. Create Symlinks to Development Binaries

**Critical**: Instead of copying binaries (which would require copying after each build), create symlinks that always point to the latest built binaries in `target/release/`:

```bash
# Link popup binary (used by Karabiner macro)
sudo ln -sf /Users/oblinger/ob/proj/HookAnchor/target/release/popup /Applications/HookAnchor.app/Contents/MacOS/popup

# Link main dispatcher binary (for completeness)
sudo ln -sf /Users/oblinger/ob/proj/HookAnchor/target/release/ha /Applications/HookAnchor.app/Contents/MacOS/HookAnchor
```

### 4. Verify Setup

Test that the Karabiner macro path works:

```bash
/Applications/HookAnchor.app/Contents/MacOS/popup
```

This should launch the HookAnchor popup window.

### 5. Test Caps Lock Trigger

Press Caps Lock - it should now launch the HookAnchor popup with the latest built code.

## Build Process

After making code changes:

1. Build the release version:
   ```bash
   cargo build --release
   ```

2. Test immediately - no copying required since symlinks automatically point to the new binaries.

## Troubleshooting

### Caps Lock Not Working

1. **Check symlinks exist**:
   ```bash
   ls -la /Applications/HookAnchor.app/Contents/MacOS/
   ```

2. **Verify symlinks point to correct location**:
   ```bash
   readlink /Applications/HookAnchor.app/Contents/MacOS/popup
   ```
   Should output: `/Users/oblinger/ob/proj/HookAnchor/target/release/popup`

3. **Test direct execution**:
   ```bash
   /Applications/HookAnchor.app/Contents/MacOS/popup
   ```

4. **Check Karabiner config**:
   ```bash
   grep -A 5 -B 5 "popup" ~/.config/karabiner/karabiner.json
   ```

### Permission Issues

If you get permission errors:

```bash
sudo chown -R $(whoami) /Applications/HookAnchor.app
```

## Notes

- This setup is for **development only**
- For distribution, use the proper app bundle creation scripts in `scripts/`
- The symlink approach ensures you're always testing the latest built code
- No need to update app bundle after each build

## Alternative: Direct Binary Path

If you prefer not to use the app bundle structure, you could modify the Karabiner macro to call the binary directly:

```json
"to_if_alone": [{ "shell_command": "/Users/oblinger/ob/proj/HookAnchor/target/release/popup" }]
```

However, the app bundle approach is recommended as it matches the production setup more closely.