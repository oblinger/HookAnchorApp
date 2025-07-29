# HookAnchor - Universal Command Launcher

## Installation

1. **Move the app**: Drag `HookAnchor.app` to your `/Applications` folder
2. **First run security**: If you get "This app won't run on this Mac":
   - **Right-click** the app in Applications and select "Open" (don't double-click)
   - Or go to System Preferences → Security & Privacy → General → Click "Open Anyway"
3. **Grant permissions**: You may need to grant accessibility permissions in System Preferences > Security & Privacy > Privacy > Accessibility

## System Requirements

- macOS 11.0 (Big Sur) or later
- Works on both Intel and Apple Silicon Macs

## Configuration

HookAnchor creates its configuration in `~/.config/hookanchor/config.yaml`

## Usage

- **Launch**: Press your configured hotkey (default varies by system)
- **Search**: Type to find commands, files, and applications
- **Execute**: Press Enter to execute the selected command
- **Navigation**: Use arrow keys to navigate results
- **Special Keys**:
  - `Ctrl+C`: Copy markdown link to clipboard
  - `=`: Open command editor
  - `+`: Start grabber mode
  - `~`: Force rescan

## URL Scheme

HookAnchor supports `hook://` URLs for deep linking:
- `hook://command_name` - Execute a specific command

## Support

For issues and documentation, visit the project repository.
