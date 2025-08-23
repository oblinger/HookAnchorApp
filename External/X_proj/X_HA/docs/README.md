# HookAnchor Documentation

HookAnchor is a universal command launcher for macOS that provides instant access to applications, files, URLs, and custom actions through a keyboard-driven popup interface.

## Quick Start

- **New Users**: Start with [User Docs/README.md](User%20Docs/README.md)
- **Developers**: See [System/README.md](System/README.md)
- **Configuration**: See [User Docs/CONFIG_REFERENCE.md](User%20Docs/CONFIG_REFERENCE.md)

## Documentation Structure

### [User Docs/](User%20Docs/)
End-user documentation for installing, configuring, and using HookAnchor:
- [User Guide](User%20Docs/USER_GUIDE.md) - Installation and basic usage
- [Configuration Reference](User%20Docs/CONFIG_REFERENCE.md) - Complete config.yaml reference
- [Templates & Scripting](User%20Docs/TEMPLATES_AND_SCRIPTING.md) - Advanced customization

### [System/](System/)
Technical documentation for system architecture and internals:
- [Architecture](System/ARCHITECTURE.md) - Overall system design
- [Command System](System/command-system.md) - Command processing
- [Popup System](System/popup-system.md) - UI implementation
- [Scanner System](System/scanner-system.md) - File scanning
- [URL Handler](System/URL_HANDLER_ARCHITECTURE.md) - URL scheme handling

### [API/](api/)
API reference documentation:
- [Rust API](api/API_REFERENCE.md) - Internal Rust API
- [JavaScript API](api/JAVASCRIPT_API.md) - Config scripting API

### [Build/](Build/)
Build and distribution documentation:
- [Development Environment](Build/DEVELOPMENT_ENV.md) - Setup guide
- [Distribution Build](Build/BUILD_DISTRIBUTION.md) - Creating releases

### [Dev Notes/](Dev%20Notes/)
Development notes, proposals, and historical documentation for contributors.

## Key Features

- **Instant Popup**: Background process for zero-latency activation
- **Smart Search**: Fuzzy matching with intelligent ranking
- **Template System**: Dynamic command generation with variables
- **JavaScript Scripting**: Custom automation functions
- **URL Scheme**: `hook://` protocol for integration
- **Markdown Integration**: Obsidian and markdown file support
- **Multi-column Layout**: Efficient command browsing
- **Context Grabber**: Capture information from active windows

## Version

Current version: **0.10.0** (see [CHANGELOG.md](../CHANGELOG.md))

## Support

- Issues: [GitHub Issues](https://github.com/oblinger/hookanchor/issues)
- Logs: `~/.config/hookanchor/anchor.log`
- Config: `~/.config/hookanchor/config.yaml`