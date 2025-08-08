# Keyboard Input Handling

## Overview

The keyboard input system provides sophisticated handling of keyboard events including multi-modifier support, shifted punctuation normalization, and customizable key bindings.

## Architecture

```rust
pub struct Keystroke {
    pub key: egui::Key,
    pub modifiers: Modifiers,
}

pub struct Modifiers {
    pub alt: bool,
    pub ctrl: bool,
    pub shift: bool,
    pub command: bool,
}
```

## Key Processing Pipeline

```
Raw Event → Normalize → Match Binding → Execute Action
    ↓           ↓            ↓              ↓
  egui     Handle Shift   Config Map    Command/Nav
```

## Shifted Punctuation Handling

The system handles keyboard variations for shifted punctuation:

```rust
// Problem: Different keyboards send different codes
"+" key might be: Equals+Shift OR Plus
"?" key might be: Slash+Shift OR Questionmark

// Solution: Normalize in matches_key_event()
if self.modifiers.shift {
    match self.key {
        egui::Key::Equals => accepts Plus or Equals,
        egui::Key::Slash => accepts Questionmark or Slash,
        // etc.
    }
}
```

## Key Binding System

### Configuration Format
```yaml
keybindings:
  exit_app: "Escape"
  navigate_down: "j"           # Vim-style
  execute_command: "Enter"
  copy_as_markdown: "cmd+shift+c"
  force_rescan: "Backtick"
```

### Key String Parsing
```rust
// Parse modifier+key combinations
"cmd+shift+c" → Keystroke {
    key: Key::C,
    modifiers: { command: true, shift: true }
}
```

## Navigation Keys

### Grid Navigation
```
ArrowUp/k    - Move selection up
ArrowDown/j  - Move selection down
ArrowLeft/h  - Move selection left
ArrowRight/l - Move selection right
Home         - First item
End          - Last item
PageUp       - Previous page
PageDown     - Next page
```

### Vim Mode Support
```rust
if config.vim_mode {
    "h" → ArrowLeft
    "j" → ArrowDown
    "k" → ArrowUp
    "l" → ArrowRight
}
```

## Special Keys

### Action Keys
- **Enter**: Execute selected command
- **Escape**: Hide window / Cancel
- **Delete**: Remove command
- **Tab**: Next field / Autocomplete

### Modifier Keys
- **Equals (=)**: Open command editor
- **Plus (+)**: Start grabber mode
- **Slash (/)**: Show in folder
- **Backtick (~)**: Force rescan

## Key Registry System

```rust
pub struct KeyRegistry {
    bindings: HashMap<String, Keystroke>,
}

impl KeyRegistry {
    pub fn get_keystroke(&self, action: &str) -> Option<&Keystroke>
    pub fn handle_event(&self, event: &KeyEvent) -> Option<Action>
}
```

## Event Handling Flow

```rust
fn handle_keyboard_input(&mut self, ctx: &egui::Context) {
    // 1. Get input state
    let input = ctx.input();
    
    // 2. Check each configured key
    for event in &input.events {
        if let Event::Key { key, pressed, modifiers } = event {
            // 3. Match against bindings
            if let Some(action) = registry.match_event(key, modifiers) {
                // 4. Execute action
                self.execute_action(action);
            }
        }
    }
}
```

## Custom Key Handlers

### Template Keys
```rust
// Dynamic key bindings from templates
if key == "t" && modifiers.cmd {
    if let Some(template) = find_template_for_key("t") {
        launch_template(template);
    }
}
```

### Multi-Key Sequences (Future)
```rust
// Vim-style leader keys
"<leader>ff" → Find files
"<leader>fg" → Find grep
```

## Platform Differences

### macOS
- Command (⌘) is primary modifier
- Option (⌥) for alternates
- Control (^) less common

### Future: Windows/Linux
- Ctrl is primary modifier
- Alt for alternates
- Super/Win key available

## Testing

```rust
#[test]
fn test_shifted_punctuation() {
    let plus = Keystroke::from_key_string("shift+equals");
    assert!(plus.matches_key(Key::Plus));
    assert!(plus.matches_key(Key::Equals));
}
```

## Performance

- **Event Debouncing**: Prevent key repeat floods
- **Lazy Matching**: Stop on first match
- **Cached Lookups**: Pre-compiled key maps

## Related Documentation
- [Configuration](configuration.md#keyboard-bindings)
- [Popup System](popup-system.md)
- [Command System](command-system.md)