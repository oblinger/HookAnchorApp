# tmux Cheatsheet

All tmux keybindings organized by category. Default prefix key is `C-b` (Ctrl+b).

## Session Management

| Key | Action |
|-----|--------|
| `tmux new-session -s name` | Create new session |
| `tmux list-sessions` | List sessions |
| `tmux attach -t name` | Attach to session |
| `tmux kill-session -t name` | Kill session |
| `prefix d` | Detach from session |
| `prefix s` | List sessions (interactive) |
| `prefix $` | Rename session |
| `prefix (` | Previous session |
| `prefix )` | Next session |

## Window Management

### Creating & Closing
| Key | Action |
|-----|--------|
| `prefix c` | Create new window |
| `prefix &` | Kill window |
| `prefix ,` | Rename window |

### Navigation
| Key | Action |
|-----|--------|
| `prefix 0-9` | Switch to window by number |
| `prefix n` | Next window |
| `prefix p` | Previous window |
| `prefix l` | Last window |
| `prefix w` | List windows (interactive) |
| `prefix f` | Find window by name |

### Window Layout
| Key | Action |
|-----|--------|
| `prefix .` | Move window |
| `prefix '` | Jump to window by number |

## Pane Management

### Creating & Closing
| Key | Action |
|-----|--------|
| `prefix %` | Split vertically |
| `prefix "` | Split horizontally |
| `prefix x` | Kill pane |
| `prefix !` | Break pane to new window |

### Navigation
| Key | Action |
|-----|--------|
| `prefix o` | Next pane |
| `prefix ;` | Last pane |
| `prefix {` | Swap with previous pane |
| `prefix }` | Swap with next pane |
| `prefix q` | Show pane numbers |
| `prefix q 0-9` | Jump to pane by number |

### Arrow Key Navigation
| Key | Action |
|-----|--------|
| `prefix ↑` | Move to pane above |
| `prefix ↓` | Move to pane below |
| `prefix ←` | Move to pane left |
| `prefix →` | Move to pane right |

### Resizing
| Key | Action |
|-----|--------|
| `prefix C-↑` | Resize pane up |
| `prefix C-↓` | Resize pane down |
| `prefix C-←` | Resize pane left |
| `prefix C-→` | Resize pane right |
| `prefix M-↑` | Resize pane up (5 rows) |
| `prefix M-↓` | Resize pane down (5 rows) |
| `prefix M-←` | Resize pane left (5 columns) |
| `prefix M-→` | Resize pane right (5 columns) |

### Layouts
| Key | Action |
|-----|--------|
| `prefix Space` | Cycle through layouts |
| `prefix M-1` | Even horizontal layout |
| `prefix M-2` | Even vertical layout |
| `prefix M-3` | Main horizontal layout |
| `prefix M-4` | Main vertical layout |
| `prefix M-5` | Tiled layout |

## Copy Mode & Scrolling

### Entering Copy Mode
| Key | Action |
|-----|--------|
| `prefix [` | Enter copy mode |
| `prefix PgUp` | Enter copy mode and scroll up |

### Navigation in Copy Mode (vi-mode)
| Key | Action |
|-----|--------|
| `h/j/k/l` | Move cursor left/down/up/right |
| `w/b` | Move word forward/backward |
| `0/$` | Move to start/end of line |
| `g/G` | Move to start/end of buffer |
| `C-f/C-b` | Page down/up |
| `C-d/C-u` | Half page down/up |

### Selection & Copy (vi-mode)
| Key | Action |
|-----|--------|
| `Space` | Start selection |
| `Enter` | Copy selection and exit |
| `Escape` | Exit copy mode |
| `v` | Start selection |
| `V` | Select line |
| `y` | Copy selection |

### Navigation in Copy Mode (emacs-mode)
| Key | Action |
|-----|--------|
| `C-f/C-b` | Move cursor right/left |
| `C-n/C-p` | Move cursor down/up |
| `M-f/M-b` | Move word forward/backward |
| `C-a/C-e` | Move to start/end of line |
| `M-</M->` | Move to start/end of buffer |

### Search in Copy Mode
| Key | Action |
|-----|--------|
| `/` | Search forward |
| `?` | Search backward |
| `n` | Next search result |
| `N` | Previous search result |

## Paste & Buffer Management

| Key | Action |
|-----|--------|
| `prefix ]` | Paste most recent buffer |
| `prefix =` | List all paste buffers |
| `prefix -` | Delete most recent buffer |
| `prefix #` | List all paste buffers (interactive) |

## Configuration & Help

| Key | Action |
|-----|--------|
| `prefix ?` | List all keybindings |
| `prefix :` | Enter command mode |
| `prefix r` | Reload config file (if configured) |
| `prefix t` | Show clock |
| `prefix ~` | Show messages |

## Command Mode

| Command | Action |
|---------|--------|
| `:list-keys` | List all keybindings |
| `:list-commands` | List all commands |
| `:source-file ~/.tmux.conf` | Reload config |
| `:set -g prefix C-a` | Change prefix key |
| `:bind r source-file ~/.tmux.conf` | Set reload binding |

## Status Line & Monitoring

| Key | Action |
|-----|--------|
| `prefix i` | Display window info |
| `prefix m` | Toggle mouse mode |

## Miscellaneous

| Key | Action |
|-----|--------|
| `prefix C-z` | Suspend tmux |
| `prefix z` | Zoom/unzoom pane |

## External Commands

| Command | Action |
|---------|--------|
| `tmux ls` | List sessions |
| `tmux new -s name` | New session with name |
| `tmux attach -t name` | Attach to session |
| `tmux kill-session -t name` | Kill session |
| `tmux rename-session -t old new` | Rename session |
| `tmux list-windows` | List windows |
| `tmux list-panes` | List panes |
| `tmux capture-pane -p` | Print pane contents |

## Custom Keybindings (Common)

These are commonly added custom bindings:

| Key | Action | Config |
|-----|--------|--------|
| `prefix r` | Reload config | `bind r source-file ~/.tmux.conf` |
| `prefix |` | Split vertically | `bind | split-window -h` |
| `prefix -` | Split horizontally | `bind - split-window -v` |
| `prefix h/j/k/l` | Vim-style pane navigation | `bind h/j/k/l select-pane -L/-D/-U/-R` |

## Tips

- **Mouse Mode**: Enable with `:set -g mouse on` for mouse scrolling and pane selection
- **Vi Mode**: Enable with `:set -g mode-keys vi` for vi-style copy mode navigation
- **Synchronize Panes**: `:setw synchronize-panes on` to type in all panes simultaneously
- **Capture Pane**: `:capture-pane -t 0 -p` to print pane contents
- **Send Keys**: `:send-keys -t 0 'command' Enter` to send commands to panes