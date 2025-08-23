# URL Handler Architecture - Proposed Solution

## Problem Statement

The current architecture has a critical flaw: when HookAnchor.app handles URLs directly, it shows the GUI popup, which causes calling applications (like Obsidian) to hang waiting for the URL handler to complete. This has caused system-wide lockups and lost productivity.

## Root Cause

1. **macOS passes URLs via Apple Events, not command line arguments**
2. **The main HookAnchor.app cannot distinguish between:**
   - Being launched normally (should show popup)
   - Being launched to handle a URL (should NOT show popup)
3. **When the popup appears during URL handling, the calling app blocks indefinitely**

## Proposed Architecture: Separate URL Handler App

### Overview

```
┌─────────────────────────────────────────────────────────┐
│                  /Applications/HookAnchor.app           │
│                                                          │
│  Contents/                                              │
│  ├── Info.plist (NO URL registration!)                  │
│  ├── MacOS/                                             │
│  │   ├── HookAnchor        (Swift supervisor)           │
│  │   ├── popup_server      (Rust GUI)                   │
│  │   └── ha               (CLI tool)                    │
│  └── Resources/                                         │
│      └── URLHandler.app/  (NEW: Embedded handler app)   │
│          └── Contents/                                  │
│              ├── Info.plist (Registers hook:// URLs)    │
│              └── MacOS/                                 │
│                  └── url_launcher (tiny Swift/script)   │
└─────────────────────────────────────────────────────────┘
```

### Key Design Principles

1. **Separation of Concerns**
   - Main app: GUI and user interaction
   - URL handler app: URL reception and command execution
   - No crossover between the two

2. **URL Handler Characteristics**
   - **Minimal**: Just receives URL and calls `ha --hook`
   - **Fast**: Exits immediately after dispatching
   - **No GUI**: Never shows any windows
   - **Isolated**: Separate bundle ID and process

3. **Execution Flow**

```
User clicks hook://cnnp in Obsidian
            ↓
macOS launches URLHandler.app
            ↓
URLHandler receives Apple Event with URL
            ↓
URLHandler executes: /path/to/ha --hook "hook://cnnp"
            ↓
URLHandler exits immediately (returns control to Obsidian)
            ↓
ha CLI processes the command in background
```

## Implementation Details

### 1. URLHandler.app Structure

**Location**: `/Applications/HookAnchor.app/Contents/Resources/URLHandler.app`

**Info.plist**:
```xml
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleExecutable</key>
    <string>url_launcher</string>
    <key>CFBundleIdentifier</key>
    <string>com.hookanchor.urlhandler</string>
    <key>CFBundleName</key>
    <string>HookAnchor URL Handler</string>
    <key>CFBundlePackageType</key>
    <string>APPL</string>
    <key>LSBackgroundOnly</key>
    <true/>
    <key>CFBundleURLTypes</key>
    <array>
        <dict>
            <key>CFBundleURLName</key>
            <string>HookAnchor URL</string>
            <key>CFBundleURLSchemes</key>
            <array>
                <string>hook</string>
            </array>
        </dict>
    </array>
</dict>
</plist>
```

### 2. url_launcher Implementation Options

#### Option A: Minimal Swift App
```swift
import AppKit

@main
struct URLLauncher {
    static func main() {
        let app = NSApplication.shared
        let delegate = URLHandlerDelegate()
        app.delegate = delegate
        app.run()
    }
}

class URLHandlerDelegate: NSObject, NSApplicationDelegate {
    func applicationWillFinishLaunching(_ notification: Notification) {
        // Register for URL events
        NSAppleEventManager.shared().setEventHandler(
            self,
            andSelector: #selector(handleURLEvent(_:withReplyEvent:)),
            forEventClass: AEEventClass(kInternetEventClass),
            andEventID: AEEventID(kAEGetURL)
        )
    }
    
    @objc func handleURLEvent(_ event: NSAppleEventDescriptor, 
                              withReplyEvent replyEvent: NSAppleEventDescriptor) {
        guard let urlString = event.paramDescriptor(forKeyword: keyDirectObject)?.stringValue else {
            NSApplication.shared.terminate(nil)
            return
        }
        
        // Execute ha with --hook option (using absolute path to binary)
        let haPath = "/Applications/HookAnchor.app/Contents/MacOS/ha"
        let task = Process()
        task.executableURL = URL(fileURLWithPath: haPath)
        task.arguments = ["--hook", urlString]
        
        // Don't wait - fire and forget
        try? task.run()
        
        // Exit immediately
        NSApplication.shared.terminate(nil)
    }
    
    func applicationDidFinishLaunching(_ notification: Notification) {
        // If launched without URL, just exit
        DispatchQueue.main.asyncAfter(deadline: .now() + 0.1) {
            NSApplication.shared.terminate(nil)
        }
    }
}
```

#### Option B: AppleScript App
```applescript
on open location hookURL
    set haPath to "/Applications/HookAnchor.app/Contents/MacOS/ha"
    do shell script haPath & " --hook " & quoted form of hookURL & " &"
end open location

on run
    -- If opened directly, do nothing
    quit
end run
```

### 3. Main HookAnchor.app Changes

**Remove from Info.plist**:
- Remove ALL `CFBundleURLTypes` entries
- The main app should NEVER register URL schemes

**HookAnchor.swift (Supervisor)**:
- Remove `handleURLEvent` method
- Remove `processURL` method
- Focus solely on caps lock trigger and popup management

### 4. ha CLI Changes

**Add --hook option**:
```rust
// In ha.rs
#[derive(Parser)]
struct Cli {
    // ... existing options ...
    
    /// Handle a hook:// URL directly (for URL handler)
    #[arg(long)]
    hook: Option<String>,
}

// In main()
if let Some(url) = cli.hook {
    // Extract command from hook://command format
    let command = if url.starts_with("hook://") {
        url.strip_prefix("hook://").unwrap()
    } else {
        &url
    };
    
    // Execute the command via -x option logic
    return execute_top_match(command);
}
```

## Migration Plan

### Phase 1: Preparation
1. Document current URL registrations
2. Back up existing configuration
3. Test ha CLI with --hook option

### Phase 2: Implementation
1. Create URLHandler.app as embedded app
2. Remove URL registration from main app
3. Update ha CLI with --hook option
4. Test with simple commands

### Phase 3: Testing
1. Test with `open "hook://test"` 
2. Verify Obsidian doesn't hang
3. Test multiple rapid URL calls
4. Verify caps lock still works normally

### Phase 4: Cleanup
1. Unregister any stale URL handlers
2. Update documentation
3. Create installer script

## Safety Checks

Before ANY changes:
```bash
# 1. Check current URL registrations
lsregister -dump | grep -i "hook:" | grep -B10 -A10 "hook:"

# 2. Back up current app
cp -R /Applications/HookAnchor.app ~/Desktop/HookAnchor.app.backup

# 3. Test in isolation
# Create test bundle in /tmp, not /Applications
```

After implementation:
```bash
# 1. Verify URL handler registration
lsregister -dump | grep -i "com.hookanchor.urlhandler"

# 2. Test URL handling
open "hook://test"

# 3. Monitor for hanging processes
ps aux | grep -E "Obsidian|HookAnchor"
```

## Benefits

1. **No More Hangs**: Calling apps never block
2. **Clean Separation**: URL handling is isolated
3. **Predictable Behavior**: Main app always shows GUI, URL handler never does
4. **Easy Debugging**: Can test URL handler independently
5. **Rollback Safety**: Can disable URL handler without affecting main app

## Risks and Mitigations

| Risk | Mitigation |
|------|------------|
| URL handler crashes | Doesn't affect main app; easy to restart |
| Registration conflicts | Use unique bundle ID; check before registering |
| Path issues | Use absolute paths; verify ha binary exists |
| Permission issues | URL handler runs with user permissions |

## Success Criteria

- [ ] URLs are handled without showing popup
- [ ] Obsidian never hangs when clicking hook:// links
- [ ] Caps lock still triggers popup normally
- [ ] URL handler exits within 100ms
- [ ] No zombie processes
- [ ] Clean uninstall removes URL registration

## Emergency Rollback

If issues occur:
```bash
# 1. Unregister URL handler
lsregister -u "/Applications/HookAnchor.app/Contents/MacOS/URLHandler.app"

# 2. Remove URL handler
rm -rf /Applications/HookAnchor.app/Contents/MacOS/URLHandler.app

# 3. Restore backup if needed
cp -R ~/Desktop/HookAnchor.app.backup /Applications/HookAnchor.app
```

## Notes

- The URL handler MUST be a separate app bundle to get its own bundle ID
- The URL handler MUST exit immediately after dispatching
- The URL handler MUST NOT import or link any GUI frameworks
- The main app MUST NOT have any URL registration in Info.plist