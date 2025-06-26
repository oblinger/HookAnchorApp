# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview
The anchor selector pops up a gooey that allows the user to type in a command. As they're typing this command in the list of commands with that prefix are listed below the input box up to 10 as the user types more characters it narrows this list down until there's only one entry when they hit. Enter the app exits and print the selected command on standard out.

The commands are in the file ~/.config/anchor_selector/commands.txt and each line has one of two formats:
- `GROUP ! COMMAND : ACTION ARG` (with group)
- `COMMAND : ACTION ARG` (without group)

## Development

### Build and Run
```bash
source ~/.cargo/env  # if needed
cargo run
```

Always build using: `cargo build --release --bin popup`

### Project Structure
- `src/main.rs` - Main application code using egui framework
- `Cargo.toml` - Dependencies (eframe, egui)

### Dependencies
- **eframe 0.29** - GUI framework  
- **egui 0.29** - Immediate mode GUI library

### Features Implemented
- Real-time command filtering as user types
- Keyboard navigation (↑/↓ arrows)
- Mouse click selection
- Enter key automatically selects first filtered result
- Outputs selected command line to stdout and exits
- Supports both command formats (with/without group)

### Development Environment
- `.tmuxp.yaml` - tmux session configuration
- Start development session: `tmuxp load .`