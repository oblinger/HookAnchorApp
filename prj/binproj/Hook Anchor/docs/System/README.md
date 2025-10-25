# HookAnchor System Documentation

This directory contains technical documentation for HookAnchor's internal architecture and system components.

## Primary Documentation

### 🏗️ [ARCHITECTURE.md](ARCHITECTURE.md)
**Complete system architecture overview** - Start here to understand how HookAnchor works. Covers:
- Multi-process architecture with Swift supervisor and Rust components
- URL handler isolation for stability
- Inter-process communication via Unix sockets
- Startup sequences and lifecycle management
- Key design decisions and rationale

## Core Components

### Process & Lifecycle Management

#### [supervisor.md](supervisor.md)
Swift supervisor that manages the application lifecycle:
- Handles macOS reopen events from Karabiner
- Manages popup_server process lifetime
- Sends visibility commands via Unix socket
- Implements single-instance enforcement

#### [URL_HANDLER_ARCHITECTURE.md](URL_HANDLER_ARCHITECTURE.md)
Critical documentation for URL handling system:
- Separate URLHandler.app prevents system hangs
- Apple Events processing without GUI
- Safety considerations and lessons learned
- **⚠️ Must read before modifying URL handling**

### User Interface

#### [popup-system.md](popup-system.md)
Rust egui-based popup window implementation:
- Window visibility management
- Command palette and search interface
- Grid layout and selection system
- Socket server for IPC commands

### Command Processing

#### [command-system.md](command-system.md)
Command parsing, storage, and execution:
- Command data structures and aliases
- Patch-based organization
- Search and matching algorithms
- Integration with scanner and launcher

#### [launcher-system.md](launcher-system.md)
JavaScript and shell command execution engine:
- Action types (app, url, folder, cmd, etc.)
- JavaScript evaluation with sandboxed runtime
- Template variable expansion
- Shell environment management

#### [Command Inference.md](Command%20Inference.md)
Intelligent patch assignment for commands:
- Inference rules and heuristics
- Path-based patch detection
- Subdirectory navigation logic

### Data Management

#### [scanner-system.md](scanner-system.md)
File system scanning and command discovery:
- Markdown file indexing
- Anchor detection and validation
- Orphan management
- Automatic rescan scheduling

#### [configuration.md](configuration.md)
YAML configuration system:
- Settings schema and defaults
- Keyboard bindings configuration
- JavaScript function definitions
- Template system configuration
- **Includes keyboard input handling details**

### Command-Line Interface

#### [dispatcher.md](dispatcher.md)
CLI tool (`ha`) for direct command execution:
- Command-line argument parsing
- Pattern matching and execution
- System management commands
- Integration with popup and scanner
- **Note**: URL handling now via separate URLHandler.app

## Deprecated Documentation

The following documentation has been consolidated:
- **keyboard-input.md** → Merged into [configuration.md](configuration.md#keyboard-bindings)

## Quick Reference

### Key Processes
| Process | Language | Purpose | Location |
|---------|----------|---------|----------|
| HookAnchor | Swift | Main app, lifecycle manager | `/Applications/HookAnchor.app/Contents/MacOS/` |
| popup_server | Rust | GUI window and commands | Spawned by supervisor |
| url_launcher | Swift | URL scheme handler | `.../Resources/URLHandler.app/Contents/MacOS/` |
| ha | Rust | CLI interface | `.../MacOS/ha` |

### Key Files
| File | Purpose |
|------|---------|
| `~/.config/hookanchor/config.yaml` | User configuration |
| `/tmp/hookanchor_popup.sock` | IPC socket for popup control |
| `~/.config/hookanchor/command_server.sock` | Command execution server |
| `~/.config/hookanchor/anchor.log` | Application logs |

### Communication Flow
```
Caps Lock → Karabiner → HookAnchor.app → Supervisor → Socket → popup_server
hook:// URL → URLHandler.app → ha --hook → Command execution
```

## Architecture Principles

1. **Process Isolation**: URL handling separated from main app for stability
2. **Single Instance**: Only one popup_server process running
3. **Socket IPC**: Simple text protocol for cross-language communication
4. **Lazy Loading**: Components initialized only when needed
5. **No Polling**: Event-driven architecture without timers
6. **Fast Response**: < 100ms popup display time

## Related Documentation

- [Build Documentation](../Build/) - Building and distribution
- [User Documentation](../User%20Docs/) - End-user configuration guides
- [API Reference](../api/) - Code-level API documentation
- [Development Notes](../dev-notes/) - Implementation details and proposals