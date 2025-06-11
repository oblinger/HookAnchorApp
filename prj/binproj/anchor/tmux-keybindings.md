# tmux Cheat Sheet

**Prefix Key:** `C-b` (Ctrl+b)

## Sessions | Windows | Panes
| Key | Action | Key | Action | Key | Action |
|-----|--------|-----|--------|-----|--------|
| `s` | Choose session | `c` | New window | `%` | Split horizontal |
| `d` | Detach | `n` | Next window | `"` | Split vertical |
| `(` | Prev session | `p` | Prev window | `o` | Next pane |
| `)` | Next session | `l` | Last window | `q` | Show pane # |
| `$` | Rename session | `,` | Rename window | `x` | Kill pane |
| | | `&` | Kill window | `z` | Toggle zoom |
| | | `0-9` | Select window | `!` | Break pane |
| | | `w` | Window list | `{` | Move pane ← |
| | | `f` | Find window | `}` | Move pane → |

## Navigation | Resize | Copy Mode
| Key     | Action      | Key      | Action      | Key    | Action         |
| ------- | ----------- | -------- | ----------- | ------ | -------------- |
| `↑↓←→`  | Select pane | `M-↑↓←→` | Resize pane | `[`    | Enter copy     |
| `Space` | Next layout | `C-↑↓←→` | Resize x5   | `]`    | Paste          |
| `M-1-5` | Set layout  |          |             | =      | Choose buffer  |
|         |             |          |             | `PgUp` | Copy + page up |

## System | Help
| Key | Action |
|-----|--------|
| `:` | Command prompt |
| `?` | List keys |
| `~` | Show messages |
| `r` | Refresh |
| `t` | Clock |
