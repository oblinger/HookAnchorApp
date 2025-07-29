# HookAnchor v0.4.4 Installation Instructions

## For Intel Macs and Apple Silicon Macs

This version includes a universal binary that works on both Intel and Apple Silicon Macs.

## If you get "This app won't run on this Mac" error:

### Method 1: Right-click to open
1. Download and mount the DMG file
2. **Right-click** on the HookAnchor.app (don't double-click)
3. Select "Open" from the context menu
4. Click "Open" in the security dialog

### Method 2: System Preferences
1. Try to open the app normally (it will fail)
2. Go to **System Preferences** → **Security & Privacy** → **General**
3. You'll see a message about HookAnchor being blocked
4. Click **"Open Anyway"**

### Method 3: Terminal (Advanced users)
```bash
# Remove quarantine attribute
xattr -d com.apple.quarantine /Applications/HookAnchor.app
```

## Verification
After installation, you can verify the app supports your architecture:
```bash
file /Applications/HookAnchor.app/Contents/MacOS/hookanchor
# Should show: "Mach-O universal binary with 2 architectures"
```

## macOS Version Requirements
- Requires macOS 11.0 (Big Sur) or later
- Works on Intel Macs (x86_64)
- Works on Apple Silicon Macs (arm64/M1/M2/M3)