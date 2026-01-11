# WhisperFlow

A keyboard-triggered dictation workflow using Superwhisper, with intelligent output routing.

## Objective

Single-key (numpad Enter) toggle for dictation: press to start Superwhisper, press again to close it and route the transcribed text to the appropriate destination (tmux session or focused email compose window).

## Current Approach (Keyboard Maestro)

The existing macro:
1. Numpad Enter triggers the flow
2. Checks if Superwhisper window is open via AppleScript:
   ```
   osascript -e 'tell application "System Events" to tell process "superwhisper" to count of windows'
   ```
3. If NOT open → send Shift+F11 to open Superwhisper
4. If open (user finished dictating) → send Shift+F11 to close, wait for clipboard, then:
   - `tmux send-keys -l "$(pbpaste)"` to paste content
   - `tmux send-keys Enter` to submit

## Planned Approach (JXA Script + Utility Library)

### Main Script: `whisperflow.js`

JXA script that:
1. Detects Superwhisper state (open/closed)
2. Toggles Superwhisper via hotkey
3. On close, routes clipboard content based on context:
   - **Email focused** → paste directly into compose window
   - **Default** → send to current tmux session

### Utility Library: `oblib.js`

Reusable JXA library providing Keyboard Maestro-like primitives:

| Function | Description |
|----------|-------------|
| `sendKeystroke(key, modifiers)` | Simulate keyboard input |
| `pause(seconds)` | Wait for specified duration |
| `getActiveApp()` | Get bundle ID of frontmost application |
| `getWindowTitle()` | Get title of focused window |
| `getClipboard()` | Get clipboard contents |
| `setClipboard(text)` | Set clipboard contents |
| `tmuxSendKeys(text, options)` | Send text to tmux session |
| `tmuxSendEnter()` | Send Enter to tmux |
| `isAppRunning(bundleId)` | Check if app is running |
| `countWindows(processName)` | Count windows for process |
| `activateApp(bundleId)` | Bring app to front |

### Context Detection

Email apps to detect (by bundle ID):
- `com.apple.mail` - Apple Mail
- `com.google.Chrome` (with Gmail URL in title)
- `com.microsoft.Outlook`

### Trigger Options

1. **Raycast Script Command** - Place in `~/.config/raycast/script-commands/`
2. **Automator Quick Action** - System Settings → Keyboard → Keyboard Shortcuts → Services
3. **Keyboard Maestro** - Call the JXA script from Execute Shell action

### First-Run Setup (Automator)

When the script is double-clicked for the first time (no Automator Quick Action exists yet):

1. **Display setup dialog** explaining:
   - "WhisperFlow needs to be set up as a Quick Action to work with a keyboard shortcut."
   - "Steps: 1) Save the Automator workflow that will open. 2) Assign a hotkey in the Settings window."

2. **Auto-open**:
   - Create and open an Automator Quick Action template (pre-configured to run the script)
   - Open `x-apple.systempreferences:com.apple.Keyboard-Settings.extension?Shortcuts` (Keyboard Shortcuts pane)

The script detects first-run by checking if `~/Library/Services/WhisperFlow.workflow` exists.

## File Structure

```
~/prj/binproj/WhisperFlow/
├── whisperflow.md      # This document (PRD)
├── whisperflow.js      # Main JXA script
├── oblib.js            # Utility library
├── WhisperFlow.kmmacros # Keyboard Maestro backup
└── WhisperFlow/        # Xcode project
    └── WhisperFlow.app # Built application
```

## Swift App Architecture: ScriptBridge

### Problem with Script-Based Approaches

JXA/AppleScript automation requires Accessibility permissions to send keystrokes. Due to macOS TCC (Transparency, Consent, & Control) protections:
- `osascript` cannot easily be granted Accessibility permissions
- Automator Quick Actions run in a sandboxed context
- There's no supported way to programmatically grant these permissions (SIP-protected)
- Permissions don't inherit to subprocesses (KM calling osascript doesn't help)

### Solution: Swift App as Scriptable Bridge

Instead of the Swift app just being a container that runs scripts, it becomes a **service** that provides privileged operations to scripts:

1. **Swift app has Accessibility permissions** (granted once by user)
2. **Exposes scriptable API** via URL scheme and/or AppleScript dictionary
3. **JXA scripts call the app** for privileged operations (keystrokes, etc.)
4. **Logic stays in JavaScript** - editable, readable, no recompilation needed

### Architecture

```
WhisperFlow.app/
├── Contents/
│   ├── MacOS/
│   │   └── WhisperFlow         # Swift binary with scriptable bridge
│   ├── Resources/
│   │   ├── script.js           # Main JXA script (user-editable)
│   │   └── config.yaml         # Configuration (user-editable)
│   └── Info.plist              # URL scheme registration
```

### URL Scheme API

The app registers `whisperflow://` URL scheme:

| URL | Action |
|-----|--------|
| `whisperflow://keystroke?key=f11&modifiers=shift` | Send Shift+F11 |
| `whisperflow://keystroke?keycode=103&modifiers=shift` | Send keycode 103 + Shift |
| `whisperflow://type?text=hello` | Type text as keystrokes |
| `whisperflow://paste` | Paste clipboard (Cmd+V) |
| `whisperflow://run` | Execute the bundled script.js |
| `whisperflow://setup` | Show hotkey configuration UI |

### JXA Script Usage

Scripts call the app for privileged operations:

```javascript
// In whisperflow.js or oblib.js
function sendKeystroke(key, modifiers) {
    const mods = (modifiers || []).join(",")
    shell(`open "whisperflow://keystroke?key=${key}&modifiers=${mods}"`)
}

function typeText(text) {
    const encoded = encodeURIComponent(text)
    shell(`open "whisperflow://type?text=${encoded}"`)
}
```

### config.yaml

```yaml
name: WhisperFlow
version: 1.0

hotkey:
  key: keypadPlus      # Or: return, f11, etc.
  modifiers: []        # Optional: [shift, command, option, control]

script: script.js      # JXA script to run on hotkey

# Accessibility actions this app provides
capabilities:
  - keystroke          # Send arbitrary keystrokes
  - type               # Type text
  - paste              # Paste clipboard
```

### First-Run Behavior

1. **On first launch**:
   - Request Accessibility permission (standard macOS dialog)
   - Show hotkey picker: "Press the key combination for WhisperFlow"
   - Save to config.yaml
2. **On subsequent launches**:
   - Register global hotkey from config
   - Run as menu bar app (no dock icon)
   - Respond to hotkey → execute script.js
   - Respond to URL scheme → perform privileged action

### Distribution

**Unsigned (developer machine):**
- Build and run locally
- Grant Accessibility permission once
- Works immediately

**Signed (for distribution):**
- Recipients drag to /Applications
- Double-click to launch
- Grant Accessibility when prompted
- Press desired hotkey when asked
- Done

### Implementation Files

```
WhisperFlow/
├── Package.swift           # Swift package manifest
├── Sources/
│   └── WhisperFlow/
│       ├── main.swift      # Entry point
│       ├── AppDelegate.swift
│       ├── HotkeyManager.swift
│       ├── URLHandler.swift
│       ├── KeystrokeSender.swift
│       └── ScriptRunner.swift
└── Resources/
    ├── script.js
    └── config.yaml
```

### Advantages

| Aspect | Pure JXA Scripts | Swift ScriptBridge |
|--------|------------------|-------------------|
| Accessibility | Blocked by TCC | Works reliably |
| Hotkey registration | Manual/fragile | Programmatic |
| Script logic | In JS (good) | Still in JS (good) |
| Privileged operations | Fail silently | Work via URL scheme |
| Distribution | Complex setup | Drag and drop |
| Editing scripts | Direct file edit | Edit in .app/Contents/Resources |
| Recompilation needed | N/A | Only for app changes, not scripts |
