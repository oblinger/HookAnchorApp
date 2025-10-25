# Swift Supervisor Component

## Overview

The Swift supervisor is a lightweight native macOS application that provides instant window visibility control for the HookAnchor popup. It achieves sub-10ms response times by keeping the Rust popup process running continuously and using native Cocoa APIs to show/hide its window.

## Design Rationale

### Problem
- Previous AppleScript-based approach: ~150-200ms latency
- Process launch overhead: ~50ms
- AppleScript execution: ~100ms
- Total user-perceived delay was unacceptable

### Solution
- Native Swift app using Cocoa APIs: ~5-10ms
- Keep popup process running continuously
- Use native window management APIs
- No IPC or message passing needed

## Architecture

```
┌─────────────────────────────────────────┐
│          Swift Supervisor               │
│                                         │
│  ┌──────────────────────────────────┐  │
│  │    AppDelegate (NSApplication)    │  │
│  │                                   │  │
│  │  • applicationDidFinishLaunching  │  │
│  │  • applicationShouldHandleReopen  │  │
│  │  • applicationWillTerminate       │  │
│  └────────────┬─────────────────────┘  │
│               │                         │
│  ┌────────────▼─────────────────────┐  │
│  │     Process Management           │  │
│  │                                   │  │
│  │  • Launch popup --server         │  │
│  │  • Monitor child process         │  │
│  │  • Restart if crashed            │  │
│  └────────────┬─────────────────────┘  │
│               │                         │
│  ┌────────────▼─────────────────────┐  │
│  │     Window Control               │  │
│  │                                   │  │
│  │  • NSRunningApplication          │  │
│  │  • CGWindowListCopyWindowInfo    │  │
│  │  • AXUIElement APIs              │  │
│  └──────────────────────────────────┘  │
└─────────────────────────────────────────┘
```

## Implementation Details

### File Location
- Source: `swift/Supervisor/main.swift`
- Binary: `target/release/supervisor`
- In app bundle: `HookAnchor.app/Contents/MacOS/supervisor`

### Key Components

#### 1. Application Delegate
```swift
@main
class AppDelegate: NSObject, NSApplicationDelegate
```
- Manages application lifecycle
- Handles reopen events (when app is "launched" while running)
- Runs as LSUIElement (no dock icon)

#### 2. Process Management
```swift
var rustPopupProcess: Process?
var rustPopupPID: pid_t = 0
```
- Launches popup with `--server` flag
- Monitors process health
- Automatic restart on crash
- Clean termination on supervisor exit

#### 3. Window Visibility Control

**Primary Method: NSRunningApplication**
```swift
if let app = NSRunningApplication(processIdentifier: rustPopupPID) {
    app.activate(options: [.activateAllWindows])
    if app.isHidden {
        app.unhide()
    }
}
```

**Fallback Method: Accessibility APIs**
```swift
let appElement = AXUIElementCreateApplication(rustPopupPID)
AXUIElementPerformAction(windows[0], kAXRaiseAction)
```

### Launch Sequence

1. **Supervisor Start**
   - Set activation policy to `.accessory` (no dock icon)
   - Determine popup binary path
   - Launch popup process with `--server` flag
   - Start monitoring timer

2. **Popup Launch**
   - Execute: `popup --server` (which internally launches `popup_server`)
   - Capture process ID
   - Wait for initialization (~500ms)
   - Initial window show

3. **Reopen Handling**
   - User triggers keyboard shortcut
   - macOS sends `applicationShouldHandleReopen`
   - Supervisor calls `showRustWindow()`
   - Window appears in ~5-10ms

## Trigger Mechanisms

### 1. Application Reopen
- **Trigger**: `open -a /path/to/supervisor`
- **Handler**: `applicationShouldHandleReopen`
- **Use Case**: Keyboard shortcuts via Karabiner/Keyboard Maestro

### 2. URL Scheme (Future)
- **Trigger**: `hook://command`
- **Handler**: URL event handler
- **Use Case**: Inter-app communication

### 3. Direct Launch
- **Trigger**: Double-click or Spotlight
- **Handler**: Normal launch if not running
- **Use Case**: Manual activation

## Performance Characteristics

| Operation | Time | Method |
|-----------|------|--------|
| Reopen event | < 1ms | Native callback |
| Find window | ~2ms | PID lookup |
| Show window | ~5ms | NSRunningApplication |
| Total latency | ~8ms | End-to-end |

## Error Handling

### Popup Process Crashes
- Detected by periodic timer (5s interval)
- Automatic restart attempted
- Logged to system console

### Window Not Found
- Falls back to multiple detection methods
- Attempts app activation
- Logs diagnostic information

### Path Resolution
- Checks bundle resources first
- Falls back to development paths
- Validates binary existence

## Build Process

### Compilation
```bash
swiftc -O \
    -parse-as-library \
    -o target/release/supervisor \
    swift/Supervisor/main.swift \
    -framework Cocoa \
    -framework Foundation
```

### Requirements
- macOS 11.0+
- Swift 5.0+
- Xcode Command Line Tools

## Configuration

The supervisor requires no configuration. All settings are managed by the Rust popup application.

### Info.plist Keys (When Bundled)
```xml
<key>LSUIElement</key>
<true/>  <!-- No dock icon -->
<key>LSMultipleInstancesProhibited</key>
<true/>  <!-- Single instance only -->
```

## Integration Points

### With Popup Application
- Launches with `--server` flag
- No direct communication
- Popup hides itself on Escape
- Supervisor only controls visibility

### With System
- Registers as background app
- Responds to reopen events
- Uses Accessibility APIs (may require permissions)

## Debugging

### Console Logs
```bash
# View supervisor logs
log show --predicate 'process == "supervisor"' --last 1h

# Or check system log
Console.app → Search "HookAnchor Supervisor"
```

### Common Issues

1. **Popup doesn't appear**
   - Check Accessibility permissions
   - Verify popup binary path
   - Check if popup process is running

2. **High CPU usage**
   - Usually indicates popup crash loop
   - Check popup logs for crash reason

3. **Multiple instances**
   - Kill all and restart
   - Check launch agent configuration

## Future Enhancements

1. **IPC Channel** (Optional)
   - For status queries
   - Configuration updates
   - Metrics collection

2. **Crash Reporting**
   - Detect crash patterns
   - Notify user of issues
   - Automatic fallback modes

3. **Multi-Window Support**
   - Manage multiple popup windows
   - Different contexts/workspaces

## Related Components

- [Popup System](popup-system.md) - The Rust UI application
- [Launcher System](launcher-system.md) - Command execution
- [Main Architecture](README.md) - System overview