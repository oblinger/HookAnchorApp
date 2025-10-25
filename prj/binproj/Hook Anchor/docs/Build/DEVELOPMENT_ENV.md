# HookAnchor Development Environment

## Overview

The HookAnchor development environment consists of several integrated components that work together to provide a seamless command launcher experience triggered by the caps lock key.

## Key Components

### 1. Keyboard Trigger Methods

HookAnchor can be triggered through various methods:

#### Option A: macOS Native Shortcuts (Simplest)
- Use System Settings → Keyboard → Keyboard Shortcuts
- Assign any key combination (e.g., ⌥Space) to launch HookAnchor
- No additional software required

#### Option B: Karabiner-Elements (For Caps Lock)
- Intercepts caps lock key press at the system level
- Executes: `open /Applications/HookAnchor.app`
- Prevents caps lock from toggling
- **Configuration**: `~/.config/karabiner/karabiner.json`

#### Option C: Keyboard Maestro
- Create macro to launch `/Applications/HookAnchor.app`
- Supports complex triggers and conditions
- Enable "reopen" for instant triggering

#### Option D: Other Tools
- BetterTouchTool, Alfred, Raycast, Hammerspoon
- Any tool that can launch applications works

### 2. Application Structure

The HookAnchor application (`/Applications/HookAnchor.app`) consists of:

```
/Applications/HookAnchor.app/
├── Contents/
│   ├── Info.plist          # App configuration
│   └── MacOS/
│       ├── HookAnchor       # Swift supervisor (main executable)
│       ├── popup_server     # Rust popup UI server
│       ├── ha              # Command-line interface
│       └── popup           # Popup control utility
```

**IMPORTANT**: All binaries in `/Applications/HookAnchor.app/Contents/MacOS/` are **SYMLINKS** to the actual binaries in `/Users/oblinger/ob/proj/HookAnchor/target/release/`. Never copy binaries - always use symlinks!

### 3. Process Architecture

When your keyboard trigger is activated, the following chain of events occurs:

1. **Trigger** activated (via macOS shortcut/Karabiner/KM/etc.) → executes `open /Applications/HookAnchor.app`
2. **macOS** sends a "reopen" event to HookAnchor (if already running) or launches it
3. **HookAnchor (Swift supervisor)** receives the reopen event
4. **Supervisor** sends "show" command to popup_server via Unix socket (`/tmp/hookanchor_popup.sock`)
5. **popup_server** displays the egui-based popup window
6. **Command server** (`~/.config/hookanchor/command_server.sock`) handles command execution

### 4. Development Setup Script

The `setup_dev_environment.sh` script provides an idempotent way to configure the development environment:

```bash
./setup_dev_environment.sh
```

This script:
- Checks prerequisites (Rust, Swift)
- Builds the project if needed
- Creates config directory and default config
- Creates `/Applications/HookAnchor.app` with proper symlinks
- Sets up `~/bin/ha` command-line access
- Cleans up stale sockets
- Tests the installation

The script is **idempotent** - it can be run multiple times safely and will only make necessary changes.

### 5. Build Process

#### Rust Components
```bash
cargo build --release
```
Builds:
- `ha` - Command-line interface
- `popup_server` - UI server
- `popup` - Popup control utility

#### Swift Supervisor
```bash
./swift/build_supervisor.sh
```
Builds the Swift supervisor that handles macOS events and manages the popup server.

### 6. Configuration Files

- **Config**: `~/.config/hookanchor/config.yaml` - Main configuration
- **Commands**: `~/.config/hookanchor/commands.txt` - Command definitions
- **Log**: `~/.config/hookanchor/anchor.log` - Debug/error logging

### 7. Socket Communication

The system uses Unix domain sockets for IPC:

- **Popup Control**: `/tmp/hookanchor_popup.sock`
  - Commands: "show", "hide", "ping"
  - Used by supervisor to control popup visibility

- **Command Server**: `~/.config/hookanchor/command_server.sock`
  - Handles command execution requests
  - Maintains persistent environment

### 8. Debugging the Popup Display Chain

To debug why the popup might not appear:

1. **Check Karabiner is working**:
   ```bash
   # Monitor Karabiner console
   tail -f ~/.config/karabiner/console.log
   ```

2. **Verify processes are running**:
   ```bash
   ps aux | grep -E "HookAnchor|popup_server"
   ```

3. **Check supervisor is receiving events**:
   ```bash
   tail -f ~/.config/hookanchor/anchor.log | grep -E "Reopen|SUPERVISOR"
   ```

4. **Test popup directly**:
   ```bash
   /Applications/HookAnchor.app/Contents/MacOS/popup show
   ```

5. **Check socket communication**:
   ```bash
   # Test if socket exists and is responsive
   echo "show" | nc -U /tmp/hookanchor_popup.sock
   ```

6. **Monitor full event chain**:
   ```bash
   # Watch the log for the complete flow
   tail -f ~/.config/hookanchor/anchor.log
   ```

### 9. Common Issues and Solutions

#### Popup doesn't appear when pressing caps lock:
1. Check Karabiner is running and configured
2. Verify `/Applications/HookAnchor.app` exists
3. Check processes with Activity Monitor
4. Run `setup_dev_environment.sh` to reset

#### Popup appears but immediately disappears:
- Check for errors in `~/.config/hookanchor/anchor.log`
- Verify command server is running

#### Commands don't execute:
- Check command server socket exists
- Verify `ha --start-server-daemon` is running
- Check for JavaScript errors in log

### 10. Testing the System

After setup or changes, test the complete chain:

```bash
# 1. Kill everything
pkill -f HookAnchor; pkill -f popup_server

# 2. Run setup script
./setup_dev_environment.sh

# 3. Test manually
open /Applications/HookAnchor.app

# 4. Test with caps lock key
# Press caps lock - popup should appear

# 5. Check logs for issues
tail -f ~/.config/hookanchor/anchor.log
```

## Important Notes

- **NEVER COPY BINARIES** - Always use symlinks from the build directory
- The Swift supervisor must be the main executable for macOS event handling
- Socket files may become stale after crashes - delete them if needed
- The system is designed to have only one instance running at a time

## First-Run Setup Assistant (Future)

The planned Setup Assistant will handle initial configuration for new users:

### Planned Features

1. **Dependency Detection**
   - Check for Karabiner-Elements installation
   - Offer to download/install if missing
   - Verify correct version compatibility

2. **Configuration Wizard**
   - Create initial `~/.config/hookanchor/config.yaml`
   - Set up markdown scanning roots
   - Configure Obsidian vault path
   - Choose keyboard trigger (default: Caps Lock)

3. **Karabiner Setup**
   - Generate complex modification JSON
   - Install to Karabiner config directory
   - Detect and resolve conflicts
   - Offer alternative triggers if needed

4. **URL Handler Registration**
   - Register URLHandler.app for hook:// URLs
   - Verify no conflicting handlers exist
   - Clean up any stale registrations

### Current Implementation

Currently, first-run setup is handled by:
- `setup_dev_environment.sh` for developers
- Manual configuration file editing
- Default config from `Resources/default_config.yaml`

The graphical Setup Assistant is planned for a future release to improve the new user experience.

## URL Handler Architecture

Starting with v0.8.0, HookAnchor uses a separated URL handler architecture:

### Key Points

- **Main App** (`HookAnchor.app`) does NOT register URLs
- **URL Handler** (`URLHandler.app`) is embedded in Resources folder
- URL handler processes hook:// URLs without showing popup
- Prevents system hangs from URL processing loops

### Development Setup

When developing, the URL handler uses symlinks to development binaries:
```bash
/Applications/HookAnchor.app/Contents/Resources/URLHandler.app/
└── Contents/MacOS/url_launcher → /Users/.../target/release/url_launcher
```

This allows testing URL handling without rebuilding the entire app bundle.

See [../System/URL_HANDLER_ARCHITECTURE.md](../System/URL_HANDLER_ARCHITECTURE.md) for complete details.