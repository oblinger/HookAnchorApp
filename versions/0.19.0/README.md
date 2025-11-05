# HookAnchor - Keyboard Launcher for Knowledge Management

## Quick Installation

1. **Drag to Applications**: Drag `HookAnchor.app` to `/Applications`

2. **First Launch - Bypass macOS Security**:
   - Double-click HookAnchor.app (will be blocked)
   - Open **System Settings** → **Privacy & Security**
   - Scroll to **Security** section
   - Click **"Open Anyway"** next to HookAnchor message
   - Try opening HookAnchor again, click **"Open"** in dialog

   _(This is only needed once for unsigned apps)_

3. **Grant Permissions**: Allow accessibility when prompted

4. **Start Using**: Press **Option+Space** (⌥Space) to open HookAnchor from anywhere!

## Features

- **Built-in Hotkey**: Option+Space works out of the box (configurable)
- **Fuses Local + Online Knowledge**: Access files, URLs, and commands
- **Auto-Discovery**: Scans your system for executables and markdown files
- **Smart Search**: Fuzzy matching for instant access

## Configuration

Config files: `~/.config/hookanchor/`
- `config.yaml` - Change hotkey, customize behavior
- `commands.txt` - Your command list (auto-managed)

## Uninstall

1. Delete `/Applications/HookAnchor.app`
2. Remove `~/.config/hookanchor/` directory
3. Remove Karabiner-Elements rule if configured

## Support

Report issues at: https://github.com/oblinger/hookanchor
