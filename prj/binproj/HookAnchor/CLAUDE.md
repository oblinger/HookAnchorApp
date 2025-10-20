## ⚠️⚠️⚠️ CRITICAL: NEVER COPY BINARIES ⚠️⚠️⚠️
**THERE MUST BE ONLY ONE COPY OF EACH BINARY ON THE ENTIRE SYSTEM**
- The ONLY binaries are in: `/Users/oblinger/ob/proj/HookAnchor/target/release/`
- `/Applications/HookAnchor.app` contains SYMLINKS, not copies
- NEVER use `cp` to copy HookAnchor or popup_server binaries
- If symlinks are broken, recreate them - DO NOT COPY BINARIES
- Read the DO_NOT_COPY_BINARIES.md files before doing anything with binaries

## 🚨🚨🚨 ABSOLUTELY CRITICAL: NO println!/eprintln! FOR DEBUGGING 🚨🚨🚨
**NEVER EVER add eprintln! or println! for debugging purposes in ANY file!**
**The user CANNOT see terminal output when running the GUI applications!**

### ⛔ FORBIDDEN - DO NOT ADD DEBUG PRINTS:
- **NEVER add `eprintln!("DEBUG: ...")` anywhere** - User cannot see these!
- **NEVER add debugging println!/eprintln! in installer_gui.rs** - It's a GUI app!
- **NEVER add debugging println!/eprintln! in popup code** - It's a GUI app!
- **NEVER use println!/eprintln! to debug issues** - Use proper logging instead!

### ✅ ALLOWED to use println!/eprintln!:
1. **CLI commands** (`src/ha.rs`, `src/cmd.rs`) - User expects terminal output
2. **CLI scanner verbose mode** (`scan_new_files` function only) - User requested verbose output with `--rescan`
3. **Command server** (`src/execute/execution_server.rs`) - ONLY for:
   - One line per command executed (with timestamp)
   - Error messages to stderr
   - Nothing else!

### ❌ NEVER use println!/eprintln! in:
1. **Popup/GUI code** (`src/ui/*`, `src/popup_launcher.rs`) - No console output at all!
2. **installer_gui.rs** - IT'S A GUI APP! User won't see terminal output!
3. **Library code** (`src/core/*`, `src/utils/*`, `src/systems/*`) - Use logging functions
4. **Background services** - Use logging functions
5. **Any new code by default** - Use logging unless it's a CLI command

### 📝 Use these logging functions instead:
- `crate::utils::log()` - Normal logging (goes to ~/.config/hookanchor/anchor.log)
- `crate::utils::detailed_log()` - Verbose/debug logging (only when verbose mode enabled)
- `crate::utils::log_error()` - Error logging

**Remember: Logs go to `~/.config/hookanchor/anchor.log`, not to the console!**

## Configuration
- `~/.config/hookanchor` is where the config info is stored
- It's important that we're not hard coding keys into the application, instead all control keys should be specified in the key binding section in the config file

## Error Display System
- `show_error_dialog()` function in popup.rs displays errors to users via dialog popup
- `queue_user_error()` function in error_display.rs allows any part of the application to queue errors for display
- Error queue is automatically checked in popup update loop and displays errors to user when shell server is unavailable

## Code Architecture Philosophy
- **NO LEGACY CODE OR MULTIPLE CODE PATHS** - Always refactor to a single, clean implementation
- **NO BACKWARD COMPATIBILITY LAYERS** - When improving systems, remove old code entirely
- **NO FALLBACKS WITHOUT PERMISSION** - Do not add fallback logic or multiple code paths unless explicitly requested by the user
- Do not maintain multiple parameter names, function names, or config keys for backward compatibility
- When changing a config key or API, update ALL references rather than supporting both old and new
- Prefer breaking changes with clear migration over maintaining compatibility code
- The codebase should have ONE way to do each thing, not multiple legacy options

## Launcher System
- Single unified launcher system using launcher::launch() with YAML/JavaScript evaluation
- Removed legacy `/tmp/cmd_file` system and `use_new_launcher` config flag
- Both GUI and CLI use the same execution path through execute_command() → launcher::launch()

## Build Process
- Always build release versions of the code each time. This is important because I'm using keyboard maestro and it's launching the release version each time. Thus, if you only build a build release, I'll end up running Old code.

## 🔄 CRITICAL: SERVER RESTART AFTER REBUILD 🔄
**ALWAYS restart the server after rebuilding binaries!**

### Rule: After every `cargo build --release`, IMMEDIATELY run:
```bash
~/ob/proj/HookAnchor/target/release/ha --restart
```

### Why this is critical:
- The popup_server runs continuously in the background
- It loads the binary code into memory at startup
- Simply rebuilding does NOT update the running server
- The running server will continue using the OLD code until restarted
- This causes confusion when code changes don't take effect

### Development workflow:
1. Make code changes
2. `cargo build --release`
3. `~/ob/proj/HookAnchor/target/release/ha --restart` ← **NEVER SKIP THIS**
4. Test the changes

### Signs you forgot to restart:
- Code changes don't take effect
- New logging statements don't appear
- Bug fixes don't work
- Features behave as before

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

## "Let's Discuss" Rule

**ABSOLUTE RULE: When the user says "let's discuss" - STOP and DISCUSS ONLY**

### If the user starts with "let's discuss":
- **DO NOT** write code
- **DO NOT** create/modify files
- **DO NOT** change configurations
- **DO NOT** implement anything

### You CAN and SHOULD:
- Search/read existing code (gathering information)
- Present options with pros/cons
- Propose alternatives
- Analyze the problem
- React to ideas
- Ask clarifying questions

### Wait for explicit permission like:
- "Go ahead and implement option 2"
- "Please proceed with..."
- "Let's do that"
- Any clear implementation request

### Example:
User: "Let's discuss how to handle uninstall when the app is broken"
❌ BAD: Creating uninstall.sh...
✅ GOOD: Here are several approaches we could take: [options]