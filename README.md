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

- **[Complete Documentation](docs/docs.md)** - Full user guide and API reference
- **[Installation Guide](docs/installation/)** - Setup and distribution
- **[API Reference](docs/api/)** - JavaScript API and built-in functions
- **[Technical Details](docs/technical/)** - Implementation and architecture
- **[Development Notes](docs/dev-notes/)** - Planning documents and summaries

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