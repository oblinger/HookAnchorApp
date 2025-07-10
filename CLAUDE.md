## Configuration
- `~/.config/hookanchor` is where the config info is stored
- It's important that we're not hard coding keys into the application, instead all control keys should be specified in the key binding section in the config file

## Build Process
- Always build release versions of the code each time. This is important because I'm using keyboard maestro and it's launching the release version each time. Thus, if you only build a build release, I'll end up running Old code.