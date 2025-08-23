# Popup Visibility Architecture

## CRITICAL CONSTRAINT: Use Only Visible/Invisible Commands

**DO NOT use minimize/unminimize or any other window state commands for hiding/showing the popup.**

The HookAnchor popup uses a simple and reliable visibility system:
- **HIDE**: `ctx.send_viewport_cmd(egui::ViewportCommand::Visible(false))`
- **SHOW**: `ctx.send_viewport_cmd(egui::ViewportCommand::Visible(true))`

This approach has been tested and works well. Any attempts to use:
- Minimize/Unminimize
- Window state changes
- Other visibility mechanisms

...will break the expected behavior and should be avoided.

## Current Issue to Solve

After executing a command, the popup hides correctly but won't show again when requested.
The issue is NOT with the visibility commands themselves, but likely with:
1. The egui update loop after hiding
2. Window focus or activation state
3. The server/client communication

## Architecture Notes

- The popup runs as a persistent server (`popup_server`)
- Commands are sent via Unix socket to show/hide the window
- The window remains in memory when hidden (not destroyed)
- The update loop continues running even when hidden (with slower refresh rate)