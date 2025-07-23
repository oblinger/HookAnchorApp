# Global Hotkey Setup for Anchor Selector

This document explains how to set up global hotkey support for launching the Anchor Selector with **Cmd+Alt+F12**.

## macOS Permissions Required

On macOS, global hotkey functionality requires **Accessibility** permissions. You'll need to:

1. Go to **System Settings** > **Privacy & Security** > **Accessibility**
2. Click the **+** button and add your Terminal app (or the app running the hotkey listener)
3. Alternatively, add the `hotkey_listener` binary directly

## Usage

### Option 1: Manual Launch
```bash
# Build the hotkey listener
cargo build --release --bin hotkey_listener

# Run the hotkey listener (requires Accessibility permissions)
./target/release/hotkey_listener
```

### Option 2: Background Service
```bash
# Run as background process
nohup ./target/release/hotkey_listener > /dev/null 2>&1 &
```

### Option 3: launchd Service (Recommended)
Create a launchd plist file to automatically start the hotkey listener on login.

## Key Codes Used

The implementation uses these macOS virtual key codes:
- **F12**: `0x45` (decimal 69)
- **Command (⌘)**: `0xE3` (left) / `0xE7` (right)  
- **Alt/Option (⌥)**: `0xE2` (left) / `0xE6` (right)

## How It Works

1. The `hotkey_listener` binary registers a global hotkey for **Cmd+Alt+F12**
2. When triggered, it launches `./target/release/popup`
3. The popup runs independently and exits when done
4. The hotkey listener continues running in the background

## Dependencies

- **global-hotkey**: Cross-platform global hotkey registration
- **Standard Rust**: No additional system dependencies

## Troubleshooting

**"Permission denied" or hotkey doesn't work:**
- Ensure Accessibility permissions are granted
- Try running from Terminal that has Accessibility access
- Check that both `hotkey_listener` and `popup` binaries exist

**"Binary not found" error:**
- Run `cargo build --release` to ensure binaries are built
- Check that `./target/release/popup` exists and is executable

**Multiple instances:**
- The hotkey listener prevents multiple popup instances
- Each hotkey press launches a new popup (previous ones will exit)