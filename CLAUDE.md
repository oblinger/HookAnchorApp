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