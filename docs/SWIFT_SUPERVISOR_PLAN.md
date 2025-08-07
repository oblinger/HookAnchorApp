# Native Supervisor Architecture Plan

## Overview
Create a lightweight native supervisor app that provides instant window visibility control for the HookAnchor popup, while keeping the core application portable across macOS, Linux, and Windows.

## Current Performance Problem
- Current latency: ~150-200ms from keypress to visible window
- Bottleneck: AppleScript execution takes ~100ms
- Goal: Reduce to <50ms using native APIs

## Core Insight
The supervisor and Rust popup don't need to communicate! The supervisor simply:
1. Launches the Rust popup once as a child process
2. Makes the Rust window visible when triggered
3. Lets the Rust popup hide itself when user presses Escape

No sockets, no IPC, no message passing needed.

## Simplified Architecture

### Components
1. **Native Supervisor** (Platform-specific, ~100-200 lines)
   - Always running in background
   - Launches Rust popup once at startup
   - Controls window visibility using native OS APIs
   - Zero startup cost (already running)

2. **Rust Popup** (Cross-platform, existing code)
   - Runs continuously as child process
   - Hides itself when user presses Escape
   - Doesn't know supervisor exists
   - All existing logic unchanged

### Communication Flow
```
User presses Caps Lock
    ↓ (Karabiner/OS triggers)
Native Supervisor (already running)
    ↓ (finds child window, makes visible ~5ms)
Window becomes visible
```

Total latency: **~20-50ms** (vs current ~200ms)

## Language Choice per Platform

### macOS: Swift
- **Why Swift**: Best access to macOS window APIs
- Handles `applicationShouldHandleReopen` natively
- LSUIElement apps are trivial to create
- Can use `NSRunningApplication` to control child

### Windows: C# or Rust
- **C#**: Natural for Windows tray apps, great WinAPI access
- **Rust**: Could use `windows-rs` crate, keeps tech stack consistent
- System tray integration is straightforward

### Linux: Rust or C
- **Rust**: Consistent with main app, good X11/Wayland bindings
- **C**: Minimal dependencies, direct X11 access
- Many users prefer direct launch anyway

## Implementation Details (macOS/Swift)

### Swift Supervisor Structure
```swift
@main
class AppDelegate: NSApplicationDelegate {
    var rustPopup: NSRunningApplication?
    var popupPID: pid_t = 0
    
    func applicationDidFinishLaunching(_ notification: Notification) {
        // Launch Rust popup once
        launchRustPopup()
        
        // Set up as background app (no dock icon)
        NSApp.setActivationPolicy(.accessory)
    }
    
    func applicationShouldHandleReopen(_ sender: NSApplication, 
                                     hasVisibleWindows: Bool) -> Bool {
        // Called when app is "launched" while already running
        showRustWindow()
        return true
    }
    
    func launchRustPopup() {
        let task = Process()
        task.launchPath = Bundle.main.resourcePath! + "/popup_server"
        task.launch()
        popupPID = task.processIdentifier
    }
    
    func showRustWindow() {
        // Find and show window by PID - native API, ~5ms
        let windows = CGWindowListCopyWindowInfo(.optionAll, kCGNullWindowID)
        // Find window with matching PID and make visible
        // Use NSWorkspace or CGWindow APIs
    }
}
```

### Info.plist Configuration
```xml
<key>LSUIElement</key>
<true/>  <!-- No dock icon -->
<key>LSMultipleInstancesProhibited</key>
<true/>  <!-- Single instance only -->
```

## Cross-Platform Strategy

### Shared Rust Core (95% of code)
- All command processing logic
- Search and filtering
- Template system
- JavaScript execution
- Configuration management
- UI rendering with egui

### Platform-Specific Launchers (5% of code)

#### macOS Implementation
- **Approach**: Swift app that stays running
- **Trigger**: macOS reopen event or URL scheme
- **Window Control**: NSWindow/CGWindow APIs
- **Distribution**: Single .app bundle containing both Swift and Rust binaries

#### Windows Implementation
- **Approach**: System tray app (C# or Rust)
- **Trigger**: Global hotkey or tray icon click
- **Window Control**: Win32 ShowWindow/SetForegroundWindow APIs
- **Distribution**: Single installer with both binaries
- **Alternative**: Use Windows' single-instance mechanism

```csharp
// C# Windows supervisor sketch
class PopupSupervisor : ApplicationContext {
    private Process rustProcess;
    private NotifyIcon trayIcon;
    
    void ShowPopup() {
        // Find window by process ID
        var handle = FindWindowByPID(rustProcess.Id);
        ShowWindow(handle, SW_SHOW);
        SetForegroundWindow(handle);
    }
}
```

#### Linux Implementation
- **Approach**: Lightweight daemon or on-demand launcher
- **Trigger**: Global hotkey via X11/Wayland
- **Window Control**: wmctrl or xdotool commands
- **Distribution**: Package with systemd service file
- **Alternative**: Many Linux users prefer direct launch

```rust
// Rust Linux supervisor sketch
fn show_popup() {
    // Using wmctrl
    Command::new("wmctrl")
        .args(&["-x", "-a", "hookanchor"])
        .spawn();
    
    // Or using X11 directly
    let display = x11::xlib::XOpenDisplay(null());
    // Find and raise window
}
```

### Configuration Approach
```yaml
launcher:
  macos:
    mode: persistent      # Supervisor stays running
    trigger: reopen       # Use macOS reopen event
  windows:
    mode: tray           # System tray app
    trigger: hotkey      # Global hotkey
  linux:
    mode: on-demand      # Launch fresh or daemon
    trigger: command     # Direct CLI command
```

## Benefits Over Current Approach

1. **Speed**: 4-10x faster than AppleScript
2. **Simplicity**: No communication protocol needed
3. **Reliability**: Native APIs are more stable
4. **Portability**: Rust core unchanged across platforms
5. **User Experience**: Feels native on each platform

## Implementation Phases

### Phase 1: macOS Proof of Concept
1. Create Swift app with LSUIElement=true
2. Launch popup_server as child process on startup
3. Implement applicationShouldHandleReopen
4. Find Rust window by PID
5. Show/hide using native APIs
6. Test trigger mechanism

### Phase 2: Integration
1. Replace current `popup` binary with Swift app
2. Update Karabiner config to launch Swift app
3. Ensure Rust popup hides itself properly
4. Add crash detection and restart
5. Package as single .app bundle

### Phase 3: Windows Port
1. Create C# or Rust tray app
2. Implement global hotkey registration
3. Port window finding logic
4. Create installer

### Phase 4: Linux Port
1. Create minimal launcher
2. Implement X11/Wayland window control
3. Package for major distributions
4. Consider systemd integration

## Key Insights

1. **No IPC needed** - Supervisor and Rust don't communicate
2. **Rust stays portable** - No platform-specific code in core
3. **Supervisors are tiny** - ~100-200 lines each
4. **Native feel** - Each platform gets appropriate behavior

## Success Criteria

- Window appears in <50ms from trigger
- Rust code remains unchanged
- Clean separation between supervisor and core
- Works reliably on all three platforms
- Easy to package and distribute

## Architecture Comparison

### Current Architecture
```
Trigger → Launcher → Socket → Server → AppleScript → Window
         (50ms)     (5ms)    (10ms)    (100ms)      = 165ms
```

### New Architecture  
```
Trigger → Supervisor → Native API → Window
         (0ms)        (5-10ms)      = 5-10ms
```

## Similar Successful Examples
- **Spotlight** - Native launcher, instant response
- **Alfred** - Swift supervisor, instant popup
- **Raycast** - Native wrapper around web tech
- **1Password** - Native supervisor, Rust core
- **VS Code** - Platform launchers, Electron core

## FAQ

**Q: Why not just use Rust for the supervisor too?**
A: Native languages have better OS integration. Swift for macOS, C# for Windows, etc. provide cleaner access to platform-specific APIs.

**Q: What if the Rust process crashes?**
A: Supervisor monitors child process and restarts if needed.

**Q: How does this affect development?**
A: During development, can still run Rust directly. Supervisor is only for production.

**Q: What about the config file?**
A: Rust popup still reads config. Supervisor doesn't need configuration.

## Next Steps

When ready to implement:
1. Create new Xcode project for macOS Swift supervisor
2. Implement basic "launch child and show window" logic
3. Test with existing Rust popup_server
4. Package and test full flow
5. Document Windows/Linux approach for future