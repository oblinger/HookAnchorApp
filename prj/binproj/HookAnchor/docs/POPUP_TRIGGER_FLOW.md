# Popup Trigger Flow

This document explains how the HookAnchor popup is triggered when the user presses Caps Lock.

## The Complete Trigger Chain

### 1. User Presses Caps Lock (alone)
When Caps Lock is pressed without any other keys.

### 2. Karabiner-Elements Detects the Keypress
**Configuration:** `~/.config/karabiner/karabiner.json`

Active rule: **"Caps Lock: Hyper key when combined"**
```json
{
  "from": { "key_code": "caps_lock" },
  "to": [
    {
      "key_code": "left_shift",
      "modifiers": ["left_control", "left_option", "left_command"]
    }
  ],
  "to_if_alone": [{ "shell_command": "open -a /Applications/HookAnchor.app" }]
}
```

**Behavior:**
- **When pressed WITH other keys:** Acts as Hyper (Shift+Ctrl+Opt+Cmd)
- **When pressed ALONE:** Executes `open -a /Applications/HookAnchor.app`

### 3. macOS Launches HookAnchor.app
The app bundle at `/Applications/HookAnchor.app` contains symlinks:

```
/Applications/HookAnchor.app/Contents/MacOS/
├── HookAnchor -> /Users/oblinger/ob/proj/HookAnchor/target/release/HookAnchorSupervisor
├── popup_server -> /Users/oblinger/ob/proj/HookAnchor/target/release/HookAnchorPopupServer
├── ha -> /Users/oblinger/ob/proj/HookAnchor/target/release/HookAnchorCommand
└── popup -> /Users/oblinger/ob/proj/HookAnchor/target/release/HookAnchorPopup
```

macOS launches the `HookAnchor` symlink, which points to `HookAnchorSupervisor`.

### 4. Swift Supervisor Receives Application Event
**File:** `swift/Supervisor/HookAnchor.swift`

**Function:** `applicationShouldHandleReopen()` (lines 166-174)

```swift
func applicationShouldHandleReopen(_ sender: NSApplication, hasVisibleWindows flag: Bool) -> Bool {
    NSLog("HookAnchor: ===== CAPS LOCK TRIGGER CHAIN START =====")
    NSLog("HookAnchor: applicationShouldHandleReopen called")
    log(" [CAPS LOCK] Reopen event received - starting popup display chain")
    showRustWindow()
    NSLog("HookAnchor: ===== CAPS LOCK TRIGGER CHAIN END =====")
    return true
}
```

### 5. Swift Calls showRustWindow()
**Function:** `showRustWindow()` (lines 328-413)

This function:
1. Ensures `popup_server` is running
2. Connects to Unix socket at `/tmp/hookanchor_popup.sock`
3. Sends `"show\n"` command to popup_server
4. Falls back to traditional macOS activation methods

```swift
// Send "show" command to popup_server via Unix socket
let socketPath = "/tmp/hookanchor_popup.sock"
// ... socket connection code ...
let command = "show\n"
command.withCString { cString in
    send(sock, cString, strlen(cString), 0)
}
```

### 6. Rust popup_server Receives Socket Command
**File:** `src/popup_launcher.rs`

The popup_server listens on the Unix socket and receives the "show" command, which triggers the window to display.

### 7. egui Window Displays
The Rust GUI (built with egui) shows the popup window.

## Logging

### Swift Supervisor Logs
**Location:** `~/.config/hookanchor/anchor.log`
**Prefix:** `SUPERVISOR:`

**Also logs to:** System Console via NSLog (viewable in Console.app)

**Example log entries:**
```
2025-10-20 22:36:02 SUPERVISOR: Starting up...
2025-10-20 22:36:02 SUPERVISOR: [CAPS LOCK] Reopen event received - starting popup display chain
2025-10-20 22:36:02 SUPERVISOR: [SHOW] Initiating popup display sequence
2025-10-20 22:36:02 SUPERVISOR:  Sent 'show' command to popup_server
2025-10-20 22:36:03 SUPERVISOR:  ✓ Window is visible
```

### Viewing Logs
```bash
# View Swift supervisor logs
tail -f ~/.config/hookanchor/anchor.log | grep SUPERVISOR

# View all logs together
tail -f ~/.config/hookanchor/anchor.log

# View system logs
log stream --predicate 'processImagePath CONTAINS "HookAnchor"' --info
```

## Building the Components

### Swift Supervisor
```bash
cd ~/ob/proj/HookAnchor
./swift/build_supervisor.sh
```

**Output:** `/Users/oblinger/ob/proj/HookAnchor/target/release/HookAnchorSupervisor`

### Rust Components
```bash
cd ~/ob/proj/HookAnchor
just build
```

**Outputs:**
- `target/release/HookAnchorPopupServer` - The GUI
- `target/release/HookAnchorCommand` - CLI tool
- `target/release/HookAnchorHistoryViewer` - History viewer

## Troubleshooting

### Popup Doesn't Open When Pressing Caps Lock

**Check 1: Is HookAnchorSupervisor binary present?**
```bash
ls -la ~/ob/proj/HookAnchor/target/release/HookAnchorSupervisor
```

If missing, rebuild:
```bash
cd ~/ob/proj/HookAnchor
./swift/build_supervisor.sh
```

**Check 2: Is the supervisor running?**
```bash
ps aux | grep HookAnchorSupervisor | grep -v grep
```

If not running:
```bash
open -a /Applications/HookAnchor.app
```

**Check 3: Check the logs**
```bash
tail -50 ~/.config/hookanchor/anchor.log | grep SUPERVISOR
```

If no SUPERVISOR logs appear, the Swift app isn't running or logging properly.

**Check 4: Verify symlinks in app bundle**
```bash
ls -la /Applications/HookAnchor.app/Contents/MacOS/
```

All symlinks should point to valid files in `~/ob/proj/HookAnchor/target/release/`

**Check 5: Test Karabiner manually**
```bash
open -a /Applications/HookAnchor.app
```

This should trigger the same flow as Caps Lock.

### Popup Opens But Disappears Immediately

This could indicate:
1. Socket communication failing
2. popup_server process not responding
3. Window visibility issues

Check popup_server logs:
```bash
tail -50 ~/.config/hookanchor/anchor.log | grep -v SUPERVISOR
```

## Why Did the Binary Go Missing?

### Root Cause Analysis

**The Swift supervisor build is NOT integrated into the main build system.**

When you run `just build` or `cargo build --release`, it only builds the Rust components:
- HookAnchorCommand (ha CLI)
- HookAnchorPopupServer (GUI)
- HookAnchorHistoryViewer

**The Swift HookAnchorSupervisor is a separate build** that must be run explicitly:
```bash
./swift/build_supervisor.sh
```

### How the Binary Disappeared

The binary likely disappeared due to one of these scenarios:

1. **`cargo clean` was run** - This deletes the entire `target/` directory, including Swift binaries placed there
2. **Selective cleanup** - Running `rm -rf target/release/` or similar commands
3. **New machine/clone** - The Swift binary was never built after cloning the repo
4. **Forgot separate build step** - Built Rust components but forgot Swift supervisor exists

### Git History Evidence

The Swift supervisor was added in commit `c2d106e` (2024-09-04):
```
c2d106e Implement Swift supervisor for instant popup window control
```

**Critical Design Gap:** The Justfile has a `test-supervisor` command but no `build-supervisor` command, and `just build` doesn't include the Swift build.

This means:
- Developers naturally run `just build` and assume everything is built
- The Swift binary is **not** included in the build
- The app bundle has symlinks pointing to a missing binary
- No error occurs until you try to launch the app

### The Fix (IMPLEMENTED)

**As of now, `just build` automatically includes the Swift supervisor build.**

The Justfile now has a smart `build-supervisor` target that:
- **Checks if rebuild is needed** - Only rebuilds if source changed or binary is missing
- **Uses timestamp comparison** - Compares `swift/Supervisor/HookAnchor.swift` vs `target/release/HookAnchorSupervisor`
- **Fast when unchanged** - Skips rebuild if binary is up to date

```justfile
# Build Swift supervisor (only if source changed or binary missing)
build-supervisor:
    # Checks: is binary missing OR is source newer than binary?
    # If yes: rebuilds
    # If no: skips (prints "✅ Swift supervisor up to date")

# Build all release binaries (with build verification)
build: build-supervisor
    # Builds Swift first (conditionally), then Rust components
```

**Now developers just run:**
```bash
just build  # Builds both Rust and Swift (Swift only if needed)
```

**Output when Swift is up to date:**
```
✅ Swift supervisor up to date
🔨 Building Rust components with Just (tracked build)...
```

**Output when Swift needs rebuild:**
```
🔨 Building Swift supervisor (source changed or binary missing)...
Building Swift Supervisor (Universal Binary)...
✅ Build complete!
🔨 Building Rust components with Just (tracked build)...
```

## System Architecture Diagram

```
User Presses Caps Lock
         ↓
  Karabiner-Elements
    (detects alone keypress)
         ↓
  open -a /Applications/HookAnchor.app
         ↓
  HookAnchorSupervisor (Swift)
    - applicationShouldHandleReopen()
    - showRustWindow()
         ↓
  Unix Socket: /tmp/hookanchor_popup.sock
    - Sends: "show\n"
         ↓
  HookAnchorPopupServer (Rust)
    - Receives socket command
    - Shows egui window
         ↓
    Popup Displays!
```

## Related Files

- **Swift Source:** `swift/Supervisor/HookAnchor.swift`
- **Build Script:** `swift/build_supervisor.sh`
- **Karabiner Config:** `~/.config/karabiner/karabiner.json`
- **App Bundle:** `/Applications/HookAnchor.app`
- **Logs:** `~/.config/hookanchor/anchor.log`
- **Socket:** `/tmp/hookanchor_popup.sock`
