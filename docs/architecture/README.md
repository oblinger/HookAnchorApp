# HookAnchor System Architecture

## Overview

HookAnchor is a high-performance command launcher and automation system for macOS that provides instant access to commands, applications, and system functions through a keyboard-driven interface. The system achieves sub-50ms launch times through a novel supervisor architecture that keeps the UI process running and uses native OS APIs for window management.

## System Design Principles

1. **Instant Response**: Sub-50ms from keyboard trigger to visible UI
2. **Modular Architecture**: Clear separation between supervisor, UI, and command processing
3. **Native Performance**: Swift supervisor for OS integration, Rust for core logic
4. **Extensible**: JavaScript-based command system with template support
5. **Keyboard-First**: All operations optimized for keyboard interaction

## High-Level Architecture

```
┌─────────────────────────────────────────────────────────┐
│                    User Interaction                      │
│        (Keyboard Shortcut / URL Scheme / CLI)           │
└─────────────┬───────────────────────┬───────────────────┘
              │                       │
              ▼                       ▼
┌──────────────────────┐  ┌──────────────────────────────┐
│   Swift Supervisor   │  │     CLI Dispatcher (ha)      │
│   [supervisor.md]    │  │    [dispatcher.md]           │
│                      │  │                              │
│ • Window Management  │  │ • Command-line Interface     │
│ • Process Lifecycle  │  │ • URL Scheme Handler         │
│ • Native OS APIs     │  │ • Direct Command Execution   │
└──────────┬───────────┘  └──────────────────────────────┘
           │                              
           ▼                              
┌─────────────────────────────────────────────────────────┐
│              Rust Popup Application                      │
│                [popup-system.md]                         │
│                                                          │
│  ┌──────────────┐  ┌─────────────┐  ┌──────────────┐  │
│  │   UI Layer   │  │   Command   │  │   Scanner    │  │
│  │    (egui)    │  │   System    │  │   System     │  │
│  └──────────────┘  └─────────────┘  └──────────────┘  │
│                                                          │
│  ┌──────────────┐  ┌─────────────┐  ┌──────────────┐  │
│  │   Template   │  │  JavaScript │  │   Config     │  │
│  │    System    │  │   Executor  │  │   Manager    │  │
│  └──────────────┘  └─────────────┘  └──────────────┘  │
└─────────────────────────────────────────────────────────┘
```

## Core Components

### 1. Swift Supervisor
**[→ Detailed Documentation](supervisor.md)**

The Swift supervisor is a lightweight native macOS application that provides instant window control:
- Launches and manages the popup_server process (via `popup --server`)
- Uses native Cocoa APIs for sub-10ms window visibility
- Handles system events (reopen, URL schemes)
- Monitors and restarts popup_server if needed

### 2. Rust Popup Application
**[→ Detailed Documentation](popup-system.md)**

The main application logic written in Rust for performance and cross-platform potential:
- **popup**: Lightweight launcher that starts popup_server
- **popup_server**: The actual GUI application
  - Renders UI using egui framework
  - Processes user input and searches
  - Executes commands and actions
  - Manages application state

### 3. Command System
**[→ Detailed Documentation](command-system.md)**

The extensible command and anchor system:
- Command definition and storage
- Anchor-based organization
- Patch system for command grouping
- Fuzzy search and filtering
- Merge and deduplication logic

### 4. Configuration System
**[→ Detailed Documentation](configuration.md)**

YAML-based configuration with JavaScript extensions:
- User preferences and settings
- Keyboard bindings
- Template definitions
- JavaScript functions
- Theme and appearance

### 5. Scanner System
**[→ Detailed Documentation](scanner-system.md)**

Filesystem scanning for automatic command discovery:
- Monitors configured directories
- Creates commands from markdown files
- Detects anchors and patches
- Manages orphan consolidation

### 6. Launcher System
**[→ Detailed Documentation](launcher-system.md)**

Command execution and JavaScript evaluation:
- Shell command execution
- JavaScript function evaluation
- Application launching
- URL opening
- Template processing

### 7. Keyboard & Input Handling
**[→ Detailed Documentation](keyboard-input.md)**

Sophisticated keyboard event processing:
- Multi-modifier support
- Shifted punctuation handling
- Navigation and selection
- Special action keys
- Customizable bindings

## Data Flow

### 1. Launch Sequence
```
User Trigger → Supervisor → Show Window → Load State → Display UI
     ↓                                         ↓
 (< 5ms)                                  (Already loaded)
```

### 2. Command Execution
```
User Input → Search/Filter → Select → Execute → Hide Window
     ↓            ↓            ↓         ↓
  (Instant)    (< 10ms)    (Enter)  (Launch)
```

### 3. Background Scanning
```
Timer → Check Directories → Find Changes → Update Commands → Save State
  ↓           ↓                 ↓              ↓
(5min)   (Configured)      (MD files)     (In memory)
```

## Performance Characteristics

| Operation | Target | Actual | Method |
|-----------|--------|--------|--------|
| Window Show | < 50ms | ~10ms | Native Swift APIs |
| Search Update | < 20ms | ~5ms | Fuzzy matching |
| Command Launch | < 100ms | ~50ms | Direct execution |
| Config Load | < 50ms | ~20ms | Cached YAML |
| Scanner Run | < 1s | ~200ms | Incremental scan |

## File System Layout

```
~/.config/hookanchor/
├── config.yaml           # User configuration
├── commands.yaml         # Command database
├── anchor.log           # Application logs
└── app_state.json       # Window position/state

~/Documents/HookAnchor/   # Default anchor location
├── Anchors/             # Markdown anchors
│   ├── ProjectName/
│   │   ├── ProjectName.md
│   │   └── patches/
└── Orphans/             # Unmatched commands
```

## Security Considerations

- No network access required
- Commands execute with user privileges
- JavaScript sandboxed to specific functions
- No persistent background services
- Configuration in user space only

## Platform Strategy

### Current (macOS)
- Swift supervisor for native integration
- Optimized for macOS keyboard shortcuts
- URL scheme support (hook://)
- Accessibility API integration

### Future (Cross-platform)
- Windows: C# or Rust supervisor with tray
- Linux: X11/Wayland integration
- Core Rust logic remains unchanged
- Platform-specific launchers only

## Related Documentation

- [Build & Deployment](../BUILD.md)
- [Configuration Guide](../CONFIGURATION.md)
- [URL Handling](../URL_HANDLING.md)
- [Development Setup](../DEVELOPMENT.md)