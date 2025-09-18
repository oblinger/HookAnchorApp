# HookAnchor - Universal Command Launcher

## Installation

1. **Drag to Applications**: Drag `HookAnchor.app` to your `/Applications` folder
2. **First Launch**: Right-click and select "Open" (don't double-click) to bypass Gatekeeper
3. **Set up Caps Lock**: Use Karabiner-Elements to map Caps Lock to launch HookAnchor
4. **Grant Permissions**: Allow accessibility permissions when prompted

## Features

- **Instant Launch**: Press Caps Lock to open the command palette
- **URL Support**: Handle `hook://` URLs from any application
- **Smart Search**: Fuzzy matching for commands and files
- **Extensible**: Add custom commands via config.yaml

## Configuration

Configuration is stored in `~/.config/hookanchor/config.yaml`

## Uninstall

1. Delete `/Applications/HookAnchor.app`
2. Remove `~/.config/hookanchor/` directory
3. Remove Karabiner-Elements rule if configured

## Support

Report issues at: https://github.com/oblinger/hookanchor
