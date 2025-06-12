# tmux Cheatsheet

All tmux keybindings organized by category. Default key is prefix `C-b` (Ctrl+b).

| Session & External                  | Window Management        | Pane Navigation & Resize  | Copy Mode & Buffers            | Juan                               |
| ----------------------------------- | ------------------------ | ------------------------- | ------------------------------ | ---------------------------------- |
| **Sessions**                        | **Create/Kill**          | **Navigation**            | **Copy Mode**                  | **Juan's Custom**                  |
| `tmux new -s name` - New session    | `c` - Create window      | `o` - Next pane           | `[` - Enter copy mode          | `C` - Customize options            |
| `tmux ls` - List sessions           | `&` - Kill window        | `;` - Last pane           | `PgUp` - Enter copy + scroll   | `D` - Choose/detach client         |
| `tmux attach -t name` - Attach      | `,` - Rename window      | `q` - Show pane numbers   | `h/j/k/l` - Move cursor (vi)   | `E` - Spread panes evenly          |
| `tmux kill-session -t name` - Kill  | **Navigate**             | `q 0-9` - Jump to pane    | `w/b` - Word forward/back (vi) | `L` - Switch to last client        |
| `d` - Detach                        | `0-9` - Switch by number | `↑↓←→` - Arrow navigation | `0/$` - Start/end line (vi)    | `M` - Clear marked pane            |
| `s` - List (interactive)            | `n` - Next window        | **Swap & Arrange**        | `g/G` - Start/end buffer (vi)  | `m` - Toggle marked pane           |
| `$` - Rename session                | `p` - Previous window    | `{` - Swap with previous  | `Space` - Start selection (vi) | `M-6` - Main-horiz-mirrored        |
| `(` - Previous session              | `l` - Last window        | `}` - Swap with next      | `Enter` - Copy & exit          | `M-7` - Main-vert-mirrored         |
| `)` - Next session                  | `w` - List (interactive) | `!` - Break to window     | `Escape` - Exit copy mode      | `M-n` - Next window w/ alert       |
| **External Commands**               | `f` - Find by name       | **Resizing**              | `v` - Start selection (vi)     | `M-o` - Rotate panes reverse       |
| `tmux rename-session -t old new`    | `.` - Move window        | `C-↑↓←→` - Resize arrows  | `V` - Select line (vi)         | `M-p` - Prev window w/ alert       |
| `tmux list-windows` - List windows  | `'` - Jump by number     | `M-↑↓←→` - Resize 5x      | `y` - Copy selection (vi)      | `C-o` - Rotate panes               |
| `tmux list-panes` - List panes      | **Layouts**              | **Create/Kill**           | **Search**                     | `DC` - Reset visible follows       |
| `tmux capture-pane -p` - Print pane | `Space` - Cycle layouts  | `%` - Split vertically    | `/` - Search forward           | `S-↑↓←→` - Move visible part       |
|                                     | `M-1` - Even horizontal  | `"` - Split horizontally  | `?` - Search backward          |                                    |
|                                     | `M-2` - Even vertical    | `x` - Kill pane           | `n` - Next result              |                                    |
|                                     | `M-3` - Main horizontal  |                           | `N` - Previous result          |                                    |
|                                     | `M-4` - Main vertical    |                           | **Buffers**                    |                                    |
|                                     | `M-5` - Tiled            |                           | `]` - Paste buffer             |                                    |
|                                     |                          |                           | = - List buffers               |                                    |
|                                     |                          |                           | `-` - Delete buffer            |                                    |
|                                     |                          |                           | `#` - List (interactive)       |                                    |
l

| Config & Help | Misc & Monitoring | Command Mode | Custom Bindings |
|---------------|-------------------|--------------|-----------------|
| **Help** | **Misc** | **Commands** | **Common Custom** |
| `?` - List keybindings | `z` - Zoom/unzoom pane | `:list-keys` - List keys | `r` - Reload config |
| `:` - Enter command mode | `C-z` - Suspend tmux | `:list-commands` - List commands | `|` - Split vertical |
| `t` - Show clock | **Monitoring** | `:source-file ~/.tmux.conf` - Reload | `-` - Split horizontal |
| `~` - Show messages | `i` - Display window info | `:set -g C-a` - Change | `prefix h/j/k/l` - Vim navigation |
| `r` - Reload config | `m` - Toggle mouse mode | `:bind r source-file ~/.tmux.conf` - Set reload | |

## Tips

- **Mouse Mode**: `:set -g mouse on` for mouse scrolling and pane selection
- **Vi Mode**: `:set -g mode-keys vi` for vi-style copy mode navigation  
- **Synchronize Panes**: `:setw synchronize-panes on` to type in all panes
- **Capture Pane**: `:capture-pane -t 0 -p` to print pane contents
- **Send Keys**: `:send-keys -t 0 'command' Enter` to send commands to panes