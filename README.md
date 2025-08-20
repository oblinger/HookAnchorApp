# HookAnchor

Universal command launcher for macOS with fuzzy search and intelligent command management.

## Quick Start

### Build
```bash
cargo build --release
```

### Run
```bash
# GUI mode
./target/release/ha

# CLI mode  
./target/release/ha -h  # prints help msg
./target/release/ha -x "execute top match"
```

### Build Scripts
```bash
# Quick development build
./scripts/build/build_release.sh

# Versioned release build
./scripts/build/build_release_version.sh 0.2.0
```

## Configuration

Config stored in `~/.config/hookanchor/config.yaml`. Run setup assistant on first launch.

## Documentation

### User Documentation
- **[User Guide](docs/User%20Docs/USER_GUIDE.md)** - Complete guide for using HookAnchor
- **[Configuration Reference](docs/User%20Docs/CONFIG_REFERENCE.md)** - Config file documentation
- **[Templates and Scripting](docs/User%20Docs/TEMPLATES_AND_SCRIPTING.md)** - Template system and JavaScript
- **[JavaScript Template Variables](docs/User%20Docs/TEMPLATE_JS_VARIABLES.md)** - Available JS objects in templates

### API Documentation
- **[API Reference](docs/api/API_REFERENCE.md)** - Complete API documentation
- **[JavaScript API](docs/api/JAVASCRIPT_API.md)** - JavaScript functions and built-ins

### System Architecture
- **[Architecture Overview](docs/System/ARCHITECTURE.md)** - System design and components
- **[Command System](docs/System/command-system.md)** - Command processing and execution
- **[Popup System](docs/System/popup-system.md)** - GUI popup interface
- **[Scanner System](docs/System/scanner-system.md)** - File system scanning
- **[Launcher System](docs/System/launcher-system.md)** - Command launcher implementation
- **[Configuration System](docs/System/configuration.md)** - Config file handling
- **[URL Handler](docs/System/URL_HANDLER_ARCHITECTURE.md)** - URL scheme handling

### Build and Development
- **[Build Distribution](docs/Build/BUILD_DISTRIBUTION.md)** - Creating release builds
- **[Development Environment](docs/Build/DEVELOPMENT_ENV.md)** - Dev setup guide
- **[Installer PRD](docs/Build/installer-prd.md)** - Installer requirements
- **[Installer Safety](docs/Build/installer-safety.md)** - Safe installation practices

### Development Notes
- **[URL Handling](docs/Dev%20Notes/URL_HANDLING.md)** - Critical URL handling notes
- **[Template System](docs/Dev%20Notes/TEMPLATE_SYSTEM_SUMMARY.md)** - Template implementation
- **[Actions Refactoring](docs/Dev%20Notes/PRD-Actions-Refactoring.md)** - Actions system design
- **[Project Status](docs/Dev%20Notes/PROJECT_STATUS.md)** - Current development status

## Project Structure

- `src/` - Rust source code
- `tests/` - All test files and examples
- `scripts/` - Build and utility scripts
- `resources/` - Icons and configuration templates
- `docs/` - Documentation
- `archive/` - Legacy code

## Requirements

- macOS
- Rust 1.70+
- Karabiner-Elements (for hotkey integration)