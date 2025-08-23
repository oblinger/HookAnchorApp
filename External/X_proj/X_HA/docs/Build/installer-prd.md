# HookAnchor Installer PRD

## Overview
Hybrid approach: DMG distribution with first-run Setup Assistant

## Current Architecture (v0.8.0+)

### Application Bundle Structure
```
HookAnchor.app/
├── Contents/
│   ├── Info.plist               # Main app config (NO URL registration!)
│   ├── MacOS/
│   │   ├── HookAnchor           # Swift supervisor (main executable)
│   │   ├── popup_server         # Rust popup UI server  
│   │   ├── ha                   # Command-line interface
│   │   └── popup                # Popup control utility
│   └── Resources/
│       ├── AppIcon.icns         # Application icon
│       ├── default_config.yaml  # Default configuration
│       └── URLHandler.app/      # Embedded URL handler
│           └── Contents/
│               ├── Info.plist   # Registers hook:// URLs
│               └── MacOS/
│                   └── url_launcher  # Minimal URL processor
```

## Core Components

### 1. Distribution
- **Format**: DMG file containing HookAnchor.app
- **Installation**: Drag-and-drop to Applications folder
- **Binaries**: Universal (Intel + Apple Silicon) via `lipo`
- **Build Script**: `build_distribution.sh` creates complete package

### 2. First-Run Setup Assistant
Launches automatically on first run to handle configuration.

#### 2.1 Keyboard Trigger Setup
- Offer keyboard trigger options:
  - macOS native shortcuts (recommended for simplicity)
  - Karabiner-Elements (for Caps Lock trigger)
  - Keyboard Maestro (if detected)
- Guide user through chosen method
- Skip if user prefers manual setup

#### 2.2 Configuration Setup
- Create `~/.config/hookanchor/` directory structure
- Install default `config.yaml` (from Resources folder)
- Preserve existing config if found (upgrade scenario)
- Never overwrites user customizations

#### 2.3 Trigger Configuration
Based on user's choice in 2.1:
- **macOS Shortcuts**: Guide through System Settings
- **Karabiner**: Generate and install complex modification JSON
- **Keyboard Maestro**: Provide macro configuration
- **Manual**: Show documentation for all options

### 3. Installation Flow
```
1. Download HookAnchor-<version>.dmg
2. Open DMG → Drag HookAnchor.app to Applications
3. Launch HookAnchor → Setup Assistant opens (if first run)
4. Choose keyboard trigger method
5. Create configuration files
6. Configure chosen trigger (macOS/Karabiner/KM)
7. Register URLHandler.app for hook:// URLs
8. Complete setup → App ready to use
```

### 4. Key Files & Locations
```
# Application
/Applications/HookAnchor.app                # Main application bundle
/Applications/HookAnchor.app/.../URLHandler.app  # URL handler (embedded)

# Configuration
~/.config/hookanchor/config.yaml            # User configuration
~/.config/hookanchor/commands.txt           # User commands (deprecated)
~/.config/hookanchor/command_server.sock    # Command server socket

# Karabiner Integration
~/.config/karabiner/assets/complex_modifications/hookanchor.json

# Logs & Cache
~/.config/hookanchor/logs/                  # Application logs
~/.config/hookanchor/command_cache.json     # Command cache
```

### 5. Karabiner Complex Modification
```json
{
  "title": "HookAnchor",
  "rules": [{
    "description": "Launch HookAnchor with Caps Lock",
    "manipulators": [{
      "type": "basic",
      "from": {
        "key_code": "caps_lock"
      },
      "to": [{
        "shell_command": "open /Applications/HookAnchor.app"
      }]
    }]
  }]
}
```

### 6. URL Handler Architecture
- **Separation of Concerns**: Main app doesn't register URLs
- **URLHandler.app**: Minimal Swift app that only handles hook:// URLs
- **Fast Processing**: URL handler executes `ha --hook <url>` and exits
- **No Popup**: URL handling never shows the popup window
- **Safety**: Prevents system hangs from URL processing loops

### 7. Implementation Status
- [x] Create DMG build script (`build_distribution.sh`)
- [x] Build Swift supervisor application
- [x] Implement URL handler separation
- [x] Generate default config.yaml
- [x] Create Karabiner JSON support
- [x] Add first-run detection (checks for config.yaml)
- [x] Handle upgrade scenarios (preserves user data)
- [x] Test on macOS 11.0+ (universal binary support)
- [ ] Build graphical Setup Assistant (currently CLI-based)

### 8. Success Metrics
- Installation under 2 minutes
- Zero-config working state
- Preserve user data on upgrade
- Clear error handling
- No URL handling conflicts
- Instant popup response (< 100ms)