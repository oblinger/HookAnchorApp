# Rust Popup System

## Overview

The popup system is the core UI application written in Rust using the egui immediate-mode GUI framework. It provides the command palette interface, handles user input, executes commands, and manages all application state.

## Architecture

```
┌──────────────────────────────────────────────────────┐
│                  Popup Application                    │
│                                                       │
│  ┌─────────────────────────────────────────────────┐ │
│  │              Main Entry Points                   │ │
│  │                                                  │ │
│  │  popup_main.rs    - Server mode entry           │ │
│  │  popup_launcher.rs - Client/launcher            │ │
│  └──────────────────┬──────────────────────────────┘ │
│                     │                                 │
│  ┌──────────────────▼──────────────────────────────┐ │
│  │                UI Layer (egui)                   │ │
│  │                                                  │ │
│  │  popup.rs         - Main UI implementation      │ │
│  │  popup_state.rs   - State management            │ │
│  │  display_layout.rs - Grid layout system         │ │
│  │  command_editor.rs - Command editing UI         │ │
│  │  dialog.rs        - Dialog system               │ │
│  └──────────────────┬──────────────────────────────┘ │
│                     │                                 │
│  ┌──────────────────▼──────────────────────────────┐ │
│  │              Core Systems                        │ │
│  │                                                  │ │
│  │  commands.rs      - Command management          │ │
│  │  scanner.rs       - File system scanner         │ │
│  │  launcher.rs      - Command execution           │ │
│  │  config.rs        - Configuration loader        │ │
│  │  template_creation.rs - Template system         │ │
│  └──────────────────────────────────────────────────┘ │
└───────────────────────────────────────────────────────┘
```

## Key Components

### 1. Entry Points

#### popup_main.rs
- Main application entry when running as server
- Initializes egui/eframe
- Creates main window
- Handles `--server` mode for supervisor

#### popup_launcher.rs
- Lightweight launcher binary
- Checks configuration for mode
- Either launches server or connects to existing
- Handles `show`, `status`, `delete` commands

### 2. UI Components

#### PopupApp (popup.rs)
Main application struct containing all UI state:

```rust
pub struct PopupApp {
    popup_state: PopupState,
    command_editor: CommandEditor,
    dialog: Dialog,
    window_size_mode: WindowSizeMode,
    grabber_countdown: Option<u8>,
    loading_state: LoadingState,
    // ... more fields
}
```

**Key Methods:**
- `new()` - Initialize with minimal state for fast startup
- `update()` - Main egui update loop
- `handle_keyboard_input()` - Process keyboard events
- `show_rust_window()` - Make window visible
- `execute_selected_command()` - Run selected command

#### PopupState (popup_state.rs)
Manages command list and search state:

```rust
pub struct PopupState {
    pub commands: Vec<Command>,
    pub filtered_commands: Vec<Command>,
    pub search_text: String,
    pub selected_index: usize,
    pub display_layout: DisplayLayout,
    pub app_state: AppState,
    // ... more fields
}
```

#### DisplayLayout (display_layout.rs)
Grid-based layout system for commands:
- Calculates optimal grid dimensions
- Manages multi-column display
- Handles overflow and scrolling
- Provides navigation helpers

### 3. Core Systems

#### Command System (commands.rs)
- Command structure and operations
- Search and filtering algorithms
- Merge and deduplication logic
- Patch-based grouping
- Submenu detection

#### Scanner System (scanner.rs)
- Background filesystem monitoring
- Markdown file detection
- Anchor identification
- Orphan management
- Incremental scanning

#### Launcher System (launcher.rs)
- Shell command execution
- JavaScript evaluation
- Application launching
- URL opening
- Output capture

## State Management

### Loading States
```rust
enum LoadingState {
    NotLoaded,      // Initial state
    Loading,        // Loading in progress
    Loaded,         // Fully loaded
}
```

### Window Modes
```rust
enum WindowSizeMode {
    Normal,         // Standard command palette
    Editor,         // Command editor expanded
    Dialog,         // Dialog box shown
}
```

### Deferred Loading Strategy
1. Show UI immediately with minimal state
2. Load commands from disk asynchronously
3. Start scanner after UI is responsive
4. Update display as data arrives

## UI Flow

### Main Loop (update method)
```
1. Check loading state
2. Handle deferred loading
3. Process error queue
4. Update countdown timers
5. Handle focus management
6. Render UI based on mode:
   - Normal: Search box + command grid
   - Editor: Command editor interface
   - Dialog: Modal dialog
7. Process keyboard input
8. Execute selected actions
```

### Search & Filter Pipeline
```
User Input → Fuzzy Match → Patch Filter → Merge Similar → Sort → Display
     ↓            ↓             ↓              ↓           ↓        ↓
  (Instant)    (< 5ms)      (< 1ms)        (< 2ms)     (< 1ms)  (< 10ms)
```

## Rendering System

### Grid Layout Algorithm
```rust
1. Calculate available space
2. Determine optimal columns (max 5)
3. Calculate rows needed
4. Position each command in grid
5. Handle selection highlighting
6. Manage scroll state
```

### Visual Hierarchy
- **Search Box**: Top, always visible
- **Command Grid**: Main area, scrollable
- **Status Bar**: Bottom, context hints
- **Overlays**: Editor/Dialog when active

## Performance Optimizations

### 1. Lazy Loading
- Commands loaded after UI shown
- Scanner runs in background
- Config cached in memory

### 2. Incremental Updates
- Only re-filter on input change
- Diff-based command updates
- Minimal redraws with egui

### 3. Memory Management
- Command deduplication
- String interning for patches
- Bounded cache sizes

## Window Management

### Visibility Control
```rust
// Show window
self.request_focus = true;
frame.set_visible(true);
frame.set_minimized(false);
frame.focus();

// Hide window (on Escape)
frame.set_visible(false);
```

### Position Persistence
- Save position on move
- Restore on next launch
- Handle multi-monitor setups

## Error Handling

### Error Display System
```rust
pub fn queue_user_error(title: String, message: String) {
    ERROR_QUEUE.lock().push(QueuedError { title, message });
}
```
- Non-blocking error queue
- Displayed in main loop
- Auto-dismiss or user action

### Crash Recovery
- State saved periodically
- Graceful degradation
- Diagnostic logging

## Configuration Integration

### Runtime Settings
- Keyboard bindings
- Display preferences
- Scanner paths
- Function definitions

### Hot Reload
- Config watched for changes
- Applied without restart
- Validation before apply

## Testing Approach

### Unit Tests
```bash
cargo test --lib
```
- Command operations
- Search algorithms
- Layout calculations

### Integration Tests
```bash
cargo test --test '*'
```
- End-to-end workflows
- Scanner operations
- Config loading

## Performance Metrics

| Operation | Target | Actual | Notes |
|-----------|--------|--------|-------|
| Initial render | < 50ms | ~30ms | First frame |
| Search update | < 16ms | ~5ms | 60 FPS |
| Command execute | < 100ms | ~50ms | Including launch |
| Config reload | < 50ms | ~20ms | YAML parse |
| Scanner cycle | < 1s | ~200ms | Full scan |

## Future Enhancements

1. **Async Command Execution**
   - Non-blocking launches
   - Progress indicators
   - Cancellation support

2. **Enhanced UI**
   - Command previews
   - Rich text support
   - Custom themes

3. **Extended Search**
   - Search history
   - Frecency scoring
   - Semantic matching

## Related Documentation

- [Command System](command-system.md) - Command structure and operations
- [Scanner System](scanner-system.md) - File system monitoring
- [Configuration](configuration.md) - Settings and customization
- [Keyboard Input](keyboard-input.md) - Input handling