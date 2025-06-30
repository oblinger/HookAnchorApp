# Popup State Cleanup Proposal

## Current State Complexity Issues

### 1. Multiple Command Lists
- **`commands`**: All commands from file (never changes)
- **`filtered_commands`**: After search filtering and merging
- **`display_commands`**: After submenu arrangement and separator insertion

### 2. Index Confusion
- `selected_index` is used to index into `display_commands`
- But some code incorrectly uses it with `filtered_commands`
- In multi-column layout, visual position != array index

### 3. Mixed Concerns
- Filtering logic mixed with display logic
- Submenu detection happens during display
- Column layout calculation repeated in multiple places

## Proposed Cleaner Architecture

### 1. Separate Data Layers
```rust
struct PopupState {
    // Layer 1: Source data
    all_commands: Vec<Command>,
    
    // Layer 2: Search/filter state
    search: SearchState,
    
    // Layer 3: Display state
    display: DisplayState,
    
    // Layer 4: UI state
    selection: SelectionState,
    config: Config,
}
```

### 2. Clear Data Flow
```
all_commands -> filter -> merge -> arrange -> display
                  ↑
             search_text
```

### 3. Selection Management
```rust
struct SelectionState {
    // Visual position (row, col) - what user sees
    visual_pos: (usize, usize),
    
    // Command index - into display_commands
    command_index: usize,
    
    // Bidirectional mapping
    fn from_visual(row: usize, col: usize) -> Option<usize>;
    fn to_visual(index: usize) -> (usize, usize);
}
```

### 4. Display State
```rust
struct DisplayState {
    // Final commands to display
    commands: Vec<Command>,
    
    // Layout information
    layout: Layout,
    
    // Submenu information if applicable
    submenu: Option<SubmenuInfo>,
}

enum Layout {
    Single { rows: usize },
    Multi { 
        rows: usize, 
        cols: usize,
        // Maps (row,col) -> command index
        grid: Vec<Vec<Option<usize>>>
    },
}
```

## Benefits

1. **Clear separation of concerns**
   - Each layer has one responsibility
   - Easy to test each layer independently

2. **No index confusion**
   - Selection tracks both visual and logical position
   - Clear mapping between them

3. **Efficient updates**
   - Only recompute what changed
   - Cache display state until search changes

4. **Easier to debug**
   - Can inspect each layer separately
   - Clear data flow makes bugs obvious

## Migration Strategy

1. **Phase 1**: Fix immediate bugs (DONE - right arrow key)
2. **Phase 2**: Extract display logic into separate struct
3. **Phase 3**: Implement proper selection tracking
4. **Phase 4**: Clean up filtering/merging pipeline
5. **Phase 5**: Remove old code

## Quick Wins Without Major Refactor

1. **Add helper methods**:
   ```rust
   fn get_selected_display_command(&self) -> Option<&Command> {
       let (display_commands, _, _, _) = self.get_display_commands();
       display_commands.get(self.selected_index)
   }
   ```

2. **Document invariants**:
   - `selected_index` always indexes into `display_commands`
   - `display_commands` is computed fresh each time from `filtered_commands`

3. **Consolidate display logic**:
   - Move all display computation into `get_display_commands()`
   - Cache result if search hasn't changed