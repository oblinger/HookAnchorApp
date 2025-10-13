# HookAnchor

(See [[X_HA]])


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


│ > this is still not working.  please show me the code that was active 24 hours ago, lets copy that into our current system.  also add a comment above it describing how it was 
│   called.  (e.g in the server or popup, and using what mechanism)  then lets try to   


- [x] the control W command sets the flags to "__EDIT__" ... that seems wrong.
- [ ] the command 'ask' cmd+P still does not create a new TMUX session.  please look at how the "~/bin/anchor activate" command creates a new session and adapt that code for use in our application.
- [ ] check when auto merging is happening