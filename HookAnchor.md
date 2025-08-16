# HookAnchor






VLM 
Digital Twin

## Project Overview
Universal command launcher and anchor system for macOS.

## Key Features
- Popup command interface triggered by hotkeys
- Markdown file scanning and command extraction  
- Obsidian integration for note management
- Configurable actions (apps, URLs, folders, etc.)
- JavaScript function system for complex workflows

## Development
- **Language**: Rust
- **UI Framework**: egui
- **Config**: YAML + JavaScript functions
- **Platform**: macOS

## Quick Actions
- Build: `cargo build --release`
- Test: `./target/release/ha --help`
- Install: Create DMG and drag to Applications
- Config: `~/.config/hookanchor/config.yaml`

## Related
- [docs](docs/docs.md) - Technical documentation
- [README](README.md) - Basic setup instructions
- [CLAUDE](CLAUDE.md) - Development guidelines