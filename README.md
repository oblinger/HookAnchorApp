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
- **[User Guide](../HookAnchor/docs/USER_GUIDE.md)** - Complete guide for using HookAnchor
- **[Configuration Reference](../HookAnchor/docs/CONFIG_REFERENCE.md)** - Config file documentation
- **[Templates and Scripting](../HookAnchor/docs/TEMPLATES_AND_SCRIPTING.md)** - Template system and JavaScript
- **[JavaScript Template Variables](../HookAnchor/docs/TEMPLATE_JS_VARIABLES.md)** - Available JS objects in templates
- **[Action Types](../HookAnchor/docs/HookAnchor_Action_Types.md)** - Available action types

### Development Notes
- **[Code Organization](./docs/CODE_ORGANIZATION.md)** - Current and proposed code architecture

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