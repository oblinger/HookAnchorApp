# Anchor Selector Project Status

## Current State: Clean and Production Ready

The project has been cleaned up and organized into a maintainable structure with embedded default configuration and comprehensive launcher system.

## Core Architecture

### Main Binaries
- **`popup`** - Main GUI application using egui/eframe
- **`cmd`** - Command-line interface

### Core Modules
- **`lib.rs`** - Main library with command parsing, filtering, and execution
- **`launcher.rs`** - New Rust launcher system with embedded YAML config
- **`eval.rs`** - Action execution engine with JavaScript support
- **`popup.rs`** - GUI implementation
- **`command_editor.rs`** - Command editing interface

### Configuration
- **`default_config.yaml`** - Embedded default configuration
- **`~/.config/anchor_selector/config.yaml`** - User configuration (auto-created)

## Key Features Implemented

### ✅ Launcher System
- **JavaScript execution** with rquickjs runtime
- **Action types**: App, URL, Folder, Shell, OpenWith, JavaScript
- **Template substitution** with `{{arg}}` patterns
- **Debug logging** to `~/.anchor.log`
- **Embedded default config** using `include_str!`

### ✅ 1Password Integration
- **Character injection** using Quick Access (Cmd+Shift+Space)
- **Robust fallback** approaches for different 1Password versions
- **Pure character injection** - no URL schemes that bypass autofill

### ✅ GUI Features
- **Multi-column layout** with configurable max_columns
- **Dynamic window sizing** based on content
- **Fuzzy matching** with intelligent prioritization
- **Submenu detection** for hierarchical commands
- **Command editing** with full CRUD operations

### ✅ Command Management
- **Flexible parsing** of command formats with/without groups
- **Smart filtering** with exact, prefix, and partial matching
- **Priority ranking** by match quality and relevance

## Test Infrastructure

### Active Test Binaries
- `test_launcher.rs` - Basic launcher validation
- `test_action_types.rs` - Comprehensive action type testing
- `test_js.rs` - JavaScript action execution tests
- `test_1pass_schemes.rs` - 1Password integration validation
- `test_popup_integration.rs` - GUI integration tests

### Legacy Tests (old_tests/)
- Moved experimental and redundant test files
- Preserved for reference but not part of main build
- Includes timing tests, debug utilities, and experimental hotkey listeners

## Configuration Structure

```yaml
# Application settings
settings:
  max_rows: 15
  max_columns: 3
  use_new_launcher: true
  debug_log: "~/.anchor.log"

# Launcher settings  
launcher_settings:
  default_browser: "Google Chrome"
  work_browser: "Google Chrome Beta"
  timeout_ms: 5000

# Simple actions (direct execution)
simple_actions:
  app: !App { name: "{{arg}}" }
  url: !Url { url: "{{arg}}" }
  chrome: !OpenWith { app: "Google Chrome", arg: "{{arg}}" }
  1pass: !Shell { command: "osascript ..." }  # Character injection

# JavaScript actions (complex logic)
js_actions:
  obs: |
    const encoded = encodeURIComponent("{{arg}}");
    const url = `obsidian://open?vault=kmr&file=${encoded}`;
    launch_app("Obsidian", url);
  anchor: |
    shell("cd '{{arg}}' && /Users/oblinger/bin/anchor activate");
```

## Dependencies (Minimal and Focused)

```toml
eframe = "0.29"      # GUI framework
egui = "0.29"        # Immediate mode GUI
regex = "1.0"        # Command parsing
serde = "1.0"        # Configuration serialization
serde_yaml = "0.9"   # YAML configuration
rquickjs = "0.9"     # JavaScript execution
```

## Build Commands

```bash
# Build all binaries
cargo build --release

# Main GUI application
cargo run --release --bin popup

# Test launcher system
cargo run --release --bin test_launcher

# Test JavaScript actions
cargo run --release --bin test_js
```

## Recent Major Changes

1. **Embedded Configuration** - Replaced hardcoded config with embedded YAML
2. **Source Tree Cleanup** - Moved legacy tests to old_tests/ directory
3. **Dependency Reduction** - Removed unused hotkey and debugging dependencies
4. **Code Polish** - Fixed warnings, added documentation, improved organization
5. **1Password Integration** - Implemented pure character injection approach
6. **JavaScript Support** - Full JavaScript action execution with shell/launch functions

## Next Steps

The project is now in a clean, maintainable state suitable for:
- Production deployment
- Feature additions
- Easy onboarding of new developers
- Reliable testing and validation