## Configuration
- `~/.config/hookanchor` is where the config info is stored
- It's important that we're not hard coding keys into the application, instead all control keys should be specified in the key binding section in the config file

## Error Display System
- `show_error_dialog()` function in popup.rs displays errors to users via dialog popup
- `queue_user_error()` function in error_display.rs allows any part of the application to queue errors for display
- Error queue is automatically checked in popup update loop and displays errors to user when shell server is unavailable

## Launcher System
- Single unified launcher system using launcher::launch() with YAML/JavaScript evaluation
- Removed legacy `/tmp/cmd_file` system and `use_new_launcher` config flag
- Both GUI and CLI use the same execution path through execute_command() → launcher::launch()

## Build Process
- Always build release versions of the code each time. This is important because I'm using keyboard maestro and it's launching the release version each time. Thus, if you only build a build release, I'll end up running Old code.

## ⚠️ CRITICAL: URL HANDLING - READ docs/URL_HANDLING.md FIRST ⚠️
**DO NOT modify ANY URL handling without reading docs/URL_HANDLING.md**
**Incorrect URL handling has locked up the entire system multiple times!**

## macOS URL Scheme Handling - CRITICAL KNOWLEDGE
- **macOS does NOT pass URLs via command line arguments when handling URL schemes!**
- **macOS uses Apple Events to pass URLs to app bundles, not command line arguments**
- When a URL like `hook://cnnp` is opened via `open "hook://cnnp"`, macOS:
  1. Launches the app bundle with NO arguments (args.len() == 1, just the program name)
  2. Sends the URL via Apple Events to the running application
- **Any attempt to handle URLs via command line argument checking will fail**
- **URL handling must be implemented in the GUI application using Apple Event handlers**
- This is why wrapper scripts and Info.plist CFBundleExecutable changes don't work for URL handling