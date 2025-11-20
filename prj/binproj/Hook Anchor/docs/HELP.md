# HookAnchor Help Documentation

# Installation

## SETUP -- Installing and configuring HookAnchor

The HookAnchor installer automates most setup tasks:

**Automatic Installation:**
- Copies Karabiner-Elements complex modifications to `~/.config/karabiner/assets/complex_modifications/`
- Adds files into the configuration folder  `~/.config/hookanchor/` including:
	`config.yaml`, `config.js`, `commands.txt`, shell integration script, and uninstaller

**Manual Steps Required:**
- **Accessibility Permissions**: Grant accessibility permissions in System Preferences → Security & Privacy → Accessibility. HookAnchor needs these to capture context from applications and send keystrokes.
- **Karabiner-Elements**: Download from https://karabiner-elements.pqrs.org/ if not installed. After installation, open Karabiner-Elements → Complex Modifications → Add Rule → enable the HookAnchor rule.

**Alternative Activation**: 
- Instead of Karabiner, configure any hotkey tool to trigger `open -a /Applications/HookAnchor.app` or set a custom key combination in `config.yaml` under `keybindings`.

## UNINSTALL -- Removing HookAnchor

1. **Stop Processes**: Open Activity Monitor, search for "HookAnchor", and quit any running processes (HookAnchorPopupServer, HookAnchorCommand, etc.)
2. **Remove Application**: Delete `/Applications/HookAnchor.app`
3. **Optional Cleanup**: Remove configuration files from `~/.config/hookanchor/` and Karabiner complex modifications from `~/.config/karabiner/assets/complex_modifications/hookanchor.json`


# User Documentation

## USAGE -- Hook Anchor Usage Basics

The popup window provides real-time command search and selection:

**Basic Operation:**
- Type characters to filter commands by name
- Arrow keys navigate selections
- Enter executes the highlighted command
- Escape closes the popup

**Prefix Menus:** When typing exactly matches an anchor name (ignoring separators like spaces, dots, underscores), a special menu appears showing:
- All commands with that prefix
- All commands from the same patch group
- This prefix menu is shown at the front of the list of commands separated by an equals sign (=) from regular matches
- Note: Matching is strict - input must match consecutively without skipping characters
- When multiple anchors match (e.g., "ha" matches both "HA" and "HA2"), the shortest command name is chosen (e.g., "HA")

**Dragging:** You can drag the popup by clicking below the input box. 



## KEYBOARD -- Keyboard Shortcuts

**Navigation Keys:**
- **Arrow Keys** - Navigate selection in command list
- **Enter** - Execute the selected command
- **Escape** - Close popup window
- **Space** - Add ghost/selected anchor to input box
- **\\** - Navigate up to parent patch in hierarchy
- **]** - Navigate down into selected anchor submenu

**Action Keys:**
- **/** - Show folder for selected anchor
- **?** - Show all keyboard shortcuts
- **`` ` ``** - Force rebuild (restart server + rescan)
- **Cmd+T** - Activate TMUX session for selected folder

**Template Creation Keys:**
- **>** - Create alias for last executed command
- **!** - Create anchor command
- **&** - Create doc command
- **+** - Create command with grabbed content (5-second countdown)
- **;** - Duplicate selected command
- **=** - Create command with custom input

**Project Creation:**
- **Cmd+Opt+P** - Create new project with YYYY prefix
- **Cmd+Opt+A** - Create new @ contact entry
- **Cmd+Opt+O** - Create new organization @ entry
- **Cmd+Opt+D** - Create new DOC documentation file
- **Cmd+Opt+W** - Create anchor with year prefix
- **Cmd+Opt+Q** - Create anchor with full date prefix
- **Cmd+Opt+S** - Create markdown with date prefix
- **Cmd+Opt+E** - Clear log file

Key bindings are defined in `config.yaml` in the `actions` section with a `key:` field and can be customized to use any modifier combination (Cmd, Opt, Ctrl, Shift) with letters, numbers, or special keys.

## EDITOR -- Using the Command Editor

The command editor allows creating and modifying commands through a GUI interface. Access via Cmd+E on a selected command or through template creation.

**Fields:**
- **Name**: Command identifier for searching
- **Action**: Action type (shell, js, template, etc.)
- **Argument**: Path, command, or parameter for the action
- **Patch**: Group/category for organization
- **Flags**: Modifiers (G for GUI, W for working directory, etc.)

**Automatic Inference**: HookAnchor automatically infers action types based on file extensions and content. Shell scripts get `shell` action, JavaScript files get `js`, directories get `anchor`, etc. This can be overridden manually.


## ACTIONS -- Built-in Action Types

Actions determine how HookAnchor executes commands. Each command in the database specifies its action type, which controls the execution method. The following action types are available when creating or editing commands:

**File and Folder Actions:**
- **anchor** - Opens a folder or markdown file in your configured editor/app. Anchors form the hierarchical navigation structure and can contain sub-anchors for organizing related content.
- **folder** - Opens a folder in Finder or configured file manager. Similar to anchor but specifically for directory navigation without markdown files.
- **doc** - Opens any document file in its default application (PDFs, Word docs, spreadsheets, etc.). The system determines the appropriate application based on file extension.
- **markdown** - Opens a markdown file in your configured markdown editor. Specifically for .md files that aren't anchors.
- **text** - Opens plain text files in your configured text editor. For non-markdown text files like logs, configs, source code.

**Application Actions:**
- **app** - Launches a macOS application by name. The argument should be the application name as it appears in /Applications.
- **console** - Opens Terminal or configured console application with optional working directory.

**Command Execution:**
- **shell** - Executes shell commands in a subprocess with full environment. Commands run in background by default unless W flag is set for working directory.
- **cmd** - Alias for shell action, executes terminal commands. Commonly used for quick shell scripts and system commands.
- **js** - Executes JavaScript functions defined in config.js. Provides access to system functions and custom business logic.

**Command Management:**
- **alias** - References another command by name, creating a shortcut. When executed, runs the target command instead. Useful for creating multiple access points to the same command.
- **template** - Creates new commands dynamically using variable expansion and templates. Templates are defined in config.yaml and can include grabber integration.

**Integration Actions:**
- **1pass** - Opens items in 1Password password manager. Argument specifies the item name or ID to display.
- **contact** - Opens contact information in Contacts app or configured contact manager.
- **notion** - Opens Notion pages by URL or ID. Requires Notion integration to be configured.
- **obs_url** - Opens Obsidian URLs for deep linking into specific notes or searches.
- **slack** - Opens Slack channels, direct messages, or threads by URL.

**Browser Actions:**
- **url** - Opens URL in default browser. Generic web link handler.
- **brave** - Opens URL specifically in Brave browser.
- **chrome** - Opens URL specifically in Google Chrome.
- **firefox** - Opens URL specifically in Mozilla Firefox.
- **safari** - Opens URL specifically in Safari browser.
- **work** - Opens URL in designated work browser profile or application.

**System Actions:**
- **popup** - Internal action for popup window operations. Used by keyboard shortcuts for navigation and UI control.
- **grabber** - Captures context from the currently active application (URLs, titles, selected text). Often used with templates for creating context-aware commands.


## CREATE -- Creating New Commands, Files, Folders

Templates enable rapid creation of new commands, files, and folders through key bindings:

**Template Triggers**: Key bindings in `config.yaml` activate specific templates. When triggered, templates use variable expansion to create customized content based on current context.

**Grabber Integration**: The grabber captures context from active applications (current document, URL, selection) and makes this data available to templates through variables like `grabbed.url`, `grabbed.title`, `grabbed.selection`. Templates can also include grammar patterns that parse input text to extract structured data (names, dates, categories) for intelligent content generation.

**Usage**: Press the configured key binding, optionally provide input when prompted, and the template creates the new command/file with expanded variables. See the TEMPLATES section for detailed configuration.



## ANCHORS -- Working with File Anchors

Anchors provide hierarchical navigation through file and folder structures:

**Selection Persistence**: Selected anchors appear in light gray for several minutes. During this time, keyboard commands operate relative to that anchor context. Press Space to add the anchor name to the input box.

**Anchor Hierarchy**: Anchors form a tree structure based on file system paths and logical groupings. Parent-child relationships determine breadcrumb navigation and context inheritance.

**Prefix Menus**: When typing matches an anchor prefix, HookAnchor generates dynamic menus containing:
- All commands sharing that prefix
- Commands from the same patch group
- Hierarchical breadcrumbs showing the anchor tree position

**Breadcrumb Display**: The top of the popup shows the current anchor context as navigable breadcrumbs, indicating your position in the anchor hierarchy and allowing quick navigation to parent levels.

## SCANNER -- Auto-discovering Commands

The scanner automatically discovers files and creates commands by traversing configured scan paths:

**Automatic Discovery**: Scans directories specified in `config.yaml` under `scan_paths`, creating commands for executable files, scripts, and folders. The scanner runs on startup and can be triggered manually with Cmd+R.

**Patch Inference**: The scanner automatically assigns patch groups based on:
- Directory structure (parent folder names)
- File locations relative to configured paths
- Existing patch patterns and naming conventions
- Manual patch assignments in the configuration

**Orphan Creation**: Commands that cannot be categorized into existing patches are assigned to "orphans" groups. These appear in the orphans section and can be manually reassigned to appropriate patches or used to create new patch categories.


# Configuration Documentation

## FILES -- Configuration File Structure

```
~/.config/hookanchor/
├── config.yaml          # Main configuration settings
├── config.js             # JavaScript functions and customizations
├── commands.txt          # Generated command database
├── state.json           # Application state and history
├── anchor.log           # Execution and debug logs
├── server.log           # Command server logs
└── backups/             # Automatic backups of commands.txt
    └── commands_*.txt
```

**Editable Files:**
- **config.yaml**: User configuration - edit anytime, changes apply immediately
- **config.js**: JavaScript functions - edit anytime, changes apply immediately
- **commands.txt**: Command database - edit anytime, but hit Cmd+R to rebuild cache

**System Files:**
- **state.json**: Managed by HookAnchor, contains last executed command and UI state
- **anchor.log**: Execution logs for debugging
- **backups/**: Automatic backups created before modifying commands.txt

## SETTINGS -- Main Configuration (config.yaml)

Key configuration parameters are organized into sections in config.yaml:

**popup_settings:**
- **verbose_logging**: Enable detailed debug logging to anchor.log (default: false)
- **merge_similar**: Merge commands with similar names in the popup display (default: true)
- **max_rows**: Maximum rows displayed in command list (default: 25)
- **max_columns**: Maximum columns for multi-column display (default: 4)
- **word_separators**: Characters that separate words for submenus and merging (default: `" ._-!"`)
- **scan_interval_seconds**: Auto-rescan filesystem interval (default: 30)
- **idle_timeout_seconds**: Hide popup after inactivity (default: 60)
- **countdown_seconds**: Grabber countdown duration (default: 5)
- **ghost_timeout_seconds**: Number of seconds the last anchor will remain active (shown in grey) in the input window after it was executed (default: 3600)
- **run_in_background**: Keep server running for instant popup (default: true)
- **global_hotkey**: System-wide hotkey to show popup (default: "opt+a")
- **popup_server_retries**: Retry attempts before rebuilding (default: 3)
- **max_window_size**: Maximum dialog size in "widthxheight" format
- **default_window_size**: Default popup size in "widthxheight" format
- **max_log_file_size**: Log file size limit in bytes before clearing
- **file_roots**: Directories to scan for markdown files and apps
- **doc_file_extensions**: File extensions to auto-create DOC commands for
- **scan_exclusion_patterns**: Regex patterns to exclude from scanning
- **listed_actions**: Available action types shown in dropdown menus
- **rename_doc/folder/patch/prefix**: Auto-rename associated data when editing

**launcher_settings:**
- **js_timeout_ms**: JavaScript execution timeout in milliseconds (default: 5000)
- **obsidian_app_name**: Name of Obsidian application
- **obsidian_vault_name**: Main vault for Obsidian markdown viewing
- **obsidian_vault_path**: Path to main Obsidian vault
- **use_javascript_tmux_activation**: Use JS instead of native TMUX activation

**actions:** Section containing all action definitions with their key bindings, types, and parameters

**cloud_scan_roots:** Configuration for cloud document scanning (Notion integration)

## FLOW -- Understanding Hook Anchor Command Execution

HookAnchor command execution follows this structured flow:

**1. User Input**
    → User types text in popup to filter commands
    → User navigates with arrow keys and presses Enter to select
    → Or user presses a keybinding shortcut directly

**2. Action Construction**
    → For command selection: Load command from commands.txt (name|action|arg|patch|flags)
    → Look up action type in config.yaml actions section to get action definition
    → Merge command parameters with action definition parameters
    → For keybindings: Skip command lookup, use action definition directly from config.yaml
    → Final action object contains: action_type, function/script, arguments, flags, and parameters

**3. Variable Expansion**
    → Template variables resolved (grabber data, selected/last commands, system info)
    → Parameters expanded using template context (see VARIABLES section for all variables)
    → Working directory and environment determined

**4. Execution Dispatch**
    → Popup/template actions only: Execute directly within popup process
    → All other actions: Send to background command server for execution
    → CLI commands: Always execute on server (even when invoked from terminal)
    → Server handles shell, JS, app launches, file operations, etc.

**5. Response Handling**
    → Success/failure status captured
    → Output logged to anchor.log
    → State updated (last executed command, history)
    → User feedback provided through popup or notifications

**Keybinding Shortcuts:** Direct keybindings (Cmd+R, Cmd+E, etc.) bypass steps 1-2 and execute actions immediately without command lookup.

## VARIABLES -- Template Variable Expansion

Variable expansion transforms template variables in command arguments before execution. Variables use `{{variable_name}}` syntax and are resolved using the current execution context.

**Available Expansion Variables:**

**System Information:**
- `{{sys_user}}` - Current username
- `{{sys_home}}` - User home directory path
- `{{sys_hostname}}` - Computer hostname
- `{{sys_time}}` - Current timestamp
- `{{sys_date}}` - Current date in YYYY-MM-DD format

**Command Context:**
- `{{_selected_name}}` - Currently selected command name in popup
- `{{_selected_path}}` - Selected command's argument/path
- `{{_selected_patch}}` - Selected command's patch group
- `{{_selected_action}}` - Selected command's action type
- `{{_selected_flags}}` - Selected command's flags
- `{{_selected_folder}}` - Selected command's folder context

**Last Executed Command:**
- `{{_last_executed_name}}` - Previously executed command name
- `{{_last_executed_path}}` - Last command's argument/path
- `{{_last_executed_patch}}` - Last command's patch group
- `{{_last_executed_action}}` - Last command's action type
- `{{_last_executed_flags}}` - Last command's flags
- `{{_last_executed_folder}}` - Last command's folder context

**Grabber Context (from active applications):**
To make these variables available, press **+** (or any template key with a grab countdown) to activate the grabber. A 5-second countdown appears, giving you time to switch to another application. The grabber then captures context from the active window.
- `{{grabbed_action}}` - Inferred action from grabbed context
- `{{grabbed_arg}}` - Grabbed file path or URL
- `{{grabbed_app}}` - Source application name
- `{{grabbed_title}}` - Window or document title
- `{{grabbed_text}}` - Selected text content
- `{{grabbed_suffix}}` - File extension for grabbed content

**User Input Parameters:** When creating templates, you can capture user input. The text typed by the user becomes available as `{{input}}`. Templates can also define patterns to parse this input into numbered variables (`{{1}}`, `{{2}}`, etc.) for structured data extraction.

## COMMANDS -- Commands File Format

The commands database is stored in `~/.config/hookanchor/commands.txt` using a structured text format.

**File Format:** Each command occupies one line with this structure:
```
patch! name:action,flag,...; argument
```

**Format Components:**
- **patch!** - Optional patch/category prefix ending with `!` (can be omitted)
- **name:** - Display name shown in popup for searching
- **flag, ...** - Optional Flags
- **action** - Action type preceded by colon (doc, anchor, app, etc.)
- **; argument** - Semicolon followed by path, URL, or parameters

**Example Commands:**
```
:; test
1Pass! GitHub 1Pass:1pass; GitHub
@Google! GCP:anchor; /Users/oblinger/ob/kmr/At/Org/@Google/GCP/GCP.md
Apps! Safari App:app; Safari
DOC! README:doc; ~/Documents/readme.pdf
Build Project:shell; cargo build --release
```

**Special Cases:**
- Commands without a patch have no prefix before the command name
- The simplest command is just `:; test` (empty name, empty action, test argument)
- Flags are rarely used in the current format and appended after argument when present

**Editing Guidelines:**
- You can edit `commands.txt` directly at any time
- After manual edits, press **`** (backtick) or use Force Rebuild to reload
- The scanner automatically adds new commands but preserves manual edits
- Lines starting with `#` are treated as comments and ignored

**Backup System:** HookAnchor automatically creates timestamped backups in the `~/.config/hookanchor/backups/` folder before any modifications. To restore a previous version, copy a backup file over the current `commands.txt` and use Force Rebuild.

## ACTIONS -- Configuring Custom Actions & Keybindings

Actions define how commands execute. When a command runs, HookAnchor looks up the action type from the command's action field, merges the command's parameters with any user input, performs variable expansion, then dispatches to the appropriate execution handler.

**Action-Command Relationship:**
Commands specify their action type in the second field of commands.txt. The action system maps each action type to specific execution logic - shell commands to terminal execution, JavaScript actions to the JS runtime, app actions to macOS application launching, etc. Parameters from the command's argument field and user input are merged before expansion.

**Keybinding Configuration:**
Keybindings in `config.yaml` use modifier+key format under the `keybindings` section:
```yaml
keybindings:
  "Cmd+R": rescan_commands
  "Cmd+Shift+N": template_note_create
  "Opt+Space": popup_toggle
```

**Modifier Keys:** Use `Cmd`, `Opt`, `Ctrl`, `Shift` combined with letters, numbers, or function keys. Multiple modifiers are connected with `+`. Examples: `Cmd+E`, `Cmd+Shift+R`, `Opt+Ctrl+A`.

**Defining Custom Action Types:**
Advanced users can define new action types by adding JavaScript functions to `config.js` and mapping them in the `actions` section of `config.yaml`. New action types should follow the existing pattern of accepting parameters and returning execution results. Study existing action implementations in the codebase for patterns and required interfaces.



## TEMPLATES -- Template Configuration

Templates automate creation of new commands, files, and folders through variable expansion and customizable content generation.

**Creating New Templates:**
Define templates in the `templates` section of `config.yaml`:
```yaml
templates:
  project_setup:
    target_folder: "${sys_home}/projects/${1}"
    files:
      - "README.md"
      - "src/main.rs"
    grammar: "project ([a-zA-Z0-9_]+)"
```

**Template Components:**
- **target_folder**: Destination directory with variable expansion
- **files**: List of files/folders to create
- **grammar**: Optional regex pattern for parsing user input
- **content**: Template content for created files
- **keybinding**: Optional direct keyboard trigger

**Grabber Integration:**
Associate templates with the grabber system to use context from active applications:
```yaml
templates:
  bookmark_page:
    requires_grabber: true
    target_folder: "${sys_home}/bookmarks"
    content: |
      # ${grabbed_title}
      URL: ${grabbed_arg}
      Date: ${sys_date}
```

**File and Folder Creation:**
Templates can create directory structures and populate files with expanded content. Use variable expansion in file paths and content to customize based on user input and system context. The template engine supports conditional logic and JavaScript evaluation for complex content generation.


## JAVASCRIPT -- Scripting Configuration (config.js)

The JavaScript runtime in `config.js` provides custom scripting capabilities for complex actions and business logic integration.

**JavaScript Integration:**
Functions defined in `config.js` can be called as action types in commands. The JS runtime has access to expansion variables and can perform complex logic, API calls, and system interactions that aren't possible with simple shell commands or built-in actions.

**Available Variables and Functions:**
The JavaScript environment provides these built-in functions:
- `shell(command)` - Execute shell command asynchronously (detached)
- `shell_sync(command)` - Execute shell command and wait for completion
- `shellWithExitCode(command)` - Execute shell command with detailed result object
- `log(message)` - Write to anchor.log file
- `getCurrentFolder()` - Get current working directory context

**Runtime Variables:**
JavaScript functions receive parameters through the global context and have access to expansion variables as structured objects. These variables are not simple strings but objects with multiple accessible properties:

**Command Objects:** Both `previous` and `selected` commands are objects with these properties:
- `cmd.name` - Command display name
- `cmd.action` - Action type (shell, js, anchor, etc.)
- `cmd.arg` - Command argument/path
- `cmd.patch` - Patch group/category
- `cmd.flags` - Command flags
- `cmd.folder` - Extracted folder path from the command

**Context Objects:**
- `grabbed.action` - Inferred action from grabbed context
- `grabbed.arg` - Grabbed file path or URL
- `grabbed.app` - Source application name
- `grabbed.title` - Window or document title
- `grabbed.text` - Selected text content
- `grabbed.suffix` - File extension for grabbed content

**Date Object:**
- `date.YYYY` - Four-digit year
- `date.MM` - Two-digit month
- `date.DD` - Two-digit day

**System Variables:**
- `sys_user`, `sys_home`, `sys_hostname` - Basic system information
- `input`, `command_name`, `user_input`, `ghost_input` - User interaction data

**Built-in Functions:** All system functions are available through the `builtins` object or directly as global functions

**Function Integration:**
To use JavaScript functions as actions:
1. Define the function in `config.js`
2. Map the function name in `config.yaml` actions section
3. Create commands with `js` action type that reference the function name
4. The function receives parameters and can return results or trigger side effects

**Example Usage:**
```javascript
// config.js

// Basic function using simple parameters
function createProjectStructure(name, type) {
    const baseDir = getCurrentFolder() + '/' + name;
    shell_sync(`mkdir -p ${baseDir}/{src,tests,docs}`);
    log(`Created project structure for ${name} (${type})`);
}

// Advanced function using expansion variable objects
function processSelectedCommand(ctx) {
    // Access the previous command's properties
    if (ctx.previous && ctx.previous.name) {
        log(`Last command was: ${ctx.previous.name}`);
        log(`Command action: ${ctx.previous.action}`);
        log(`Command path: ${ctx.previous.arg}`);
        log(`Command patch: ${ctx.previous.patch}`);
        log(`Command folder: ${ctx.previous.folder}`);
    }

    // Access grabbed context from active application
    if (ctx.grabbed && ctx.grabbed.arg) {
        log(`Grabbed URL/path: ${ctx.grabbed.arg}`);
        log(`From app: ${ctx.grabbed.app}`);
        log(`Window title: ${ctx.grabbed.title}`);
        log(`Selected text: ${ctx.grabbed.text}`);
    }

    // Access date components
    const today = `${ctx.date.YYYY}-${ctx.date.MM}-${ctx.date.DD}`;
    log(`Today's date: ${today}`);

    // Use built-in functions
    ctx.builtins.shell_sync(`echo "Processing: ${ctx.input}"`);

    return `Processed ${ctx.command_name} with input: ${ctx.input}`;
}
```

**Accessing Expansion Variables:** In your JavaScript functions, expansion variables are passed as the `ctx` parameter. Each variable is a structured object with multiple properties that you can access using dot notation (e.g., `ctx.previous.name`, `ctx.grabbed.arg`, `ctx.date.YYYY`).



# Troubleshooting

## LAUNCH -- HookAnchor Won't Start

**Check Running Processes:**
- Open Activity Monitor and search for "HookAnchor"
- Kill any existing HookAnchorPopupServer or HookAnchorCommand processes
- Try launching again from `/Applications/HookAnchor.app`

**Verify Installation:**
- Ensure HookAnchor.app is in `/Applications/` folder
- Check that configuration files exist in `~/.config/hookanchor/`
- Run installer again if files are missing

**Common Issues:**
- Accessibility permissions not granted (see PERMISSIONS section)
- Corrupted configuration files (see RESET section)
- Conflicting processes from previous versions

## POPUP -- Popup Window Issues

**Popup Won't Appear:**
- Press configured hotkey (default F10) or trigger via Karabiner
- Check Accessibility permissions in System Preferences
- Verify HookAnchor has permission to control your computer
- Restart HookAnchor if popup server isn't responding

**Popup Appears But Commands Don't Work:**
- Check that command server is running (`ha --process-status`)
- Restart server with `ha --restart`
- Review anchor.log for execution errors
- Verify commands exist in commands.txt

**Display Issues:**
- Adjust popup_timeout in config.yaml if closing too quickly
- Check display settings if popup appears off-screen
- Reset window position by deleting state.json

## PERMISSIONS -- Accessibility Problems

**Granting Accessibility Permissions:**
1. Open System Preferences → Security & Privacy → Accessibility
2. Click the lock icon and enter your password
3. Find HookAnchor in the list or click "+" to add it
4. Check the box next to HookAnchor to grant permissions
5. Restart HookAnchor after granting permissions

**Permission Issues:**
- HookAnchor needs accessibility permissions to capture keystrokes and application context
- Without permissions, hotkeys won't work and grabber functionality fails
- Terminal app may also need permissions if running shell commands
- Some commands may require additional permissions (Full Disk Access, etc.)

## SHELL -- Shell Integration Not Working

**Environment Issues:**
- Check that shell integration script is in your shell profile
- Source the script manually: `source ~/.config/hookanchor/shell_integration.sh`
- Verify PATH includes HookAnchor binary location
- Check terminal_app setting in config.yaml

**Command Execution Problems:**
- Review anchor.log for shell command errors
- Test commands manually in terminal
- Check working directory settings (W flag)
- Verify shell permissions and environment variables

## KARABINER -- Hotkey Not Functioning

**Karabiner-Elements Setup:**
1. Install Karabiner-Elements from https://karabiner-elements.pqrs.org/
2. Open Karabiner-Elements → Complex Modifications
3. Click "Add Rule" and import HookAnchor rule
4. Enable the HookAnchor complex modification
5. Test hotkey (default F10)

**Alternative Hotkey Setup:**
- Configure custom keybinding in config.yaml under keybindings section
- Use any hotkey tool to trigger `open -a HookAnchor.app`
- Set up direct shell command: `/path/to/ha --popup`

## SCANNER -- Missing Commands

**Force Rescan:**
- Press **Cmd+R** in popup to rebuild command cache
- Run `ha --rescan` from terminal for verbose scanning
- Check scan_paths in config.yaml are correct
- Verify files exist and are accessible

**Scan Path Issues:**
- Ensure scan paths point to valid directories
- Check file permissions on scanned folders
- Add new directories to scan_paths in config.yaml
- Remove non-existent paths that cause scan errors

**Command Detection:**
- Scanner looks for executable files, scripts, and folders
- Files need proper extensions (.sh, .py, .js, etc.) or execute permissions
- Folders become anchor commands automatically
- Manual commands can be added directly to commands.txt

## PERFORMANCE -- Slow Performance

**Popup Response:**
- Reduce max_results in config.yaml to limit search results
- Check scan_paths aren't including large directory trees
- Clear command cache and rescan (Cmd+R)
- Review anchor.log for slow operations

**Command Execution:**
- Use background execution for long-running commands
- Check command server status (`ha --process-status`)
- Restart server if accumulating processes (`ha --restart`)
- Optimize shell commands and reduce I/O operations

**System Impact:**
- Monitor HookAnchor processes in Activity Monitor
- Adjust scan frequency if using automatic scanning
- Disable detailed logging if enabled continuously

## NETWORK -- Server Connection Issues

**Command Server Problems:**
- Check server status: `ha --process-status`
- Restart server: `ha --restart`
- Look for socket file: `/tmp/hookanchor_cmd_server.sock`
- Kill zombie processes if server won't start

**Communication Errors:**
- Review server.log for connection issues
- Check filesystem permissions on socket file
- Restart entire HookAnchor application
- Clear temporary files in /tmp/ if needed

## LOGS -- Finding and Reading Logs

**Log Locations:**
- **anchor.log**: Main execution and debug log at `~/.config/hookanchor/anchor.log`
- **server.log**: Command server communication log at `~/.config/hookanchor/server.log`
- **Console.app**: System-level errors and crash reports

**Reading Logs:**
- Use `tail -f ~/.config/hookanchor/anchor.log` to monitor real-time
- Search for specific commands or error patterns
- Check timestamps to correlate with issues
- Clear logs periodically to manage file size

## DEBUG -- Running in Debug Mode

**Enable Detailed Logging:**
Set `enable_logging: true` in config.yaml for verbose output to anchor.log

**Debug Commands:**
- `ha --process-status` - Show running processes
- `ha --rescan` - Verbose command scanning
- `RUST_LOG=debug ha` - Run with debug output
- `ha --test-grabber` - Test context grabbing functionality

**Debug Information:**
- Check build version: `ha --version`
- Review configuration: `ha --show-config`
- Test individual commands: `ha -x "command_name"`
- Monitor system resources during operation

## RESET -- Resetting Configuration

**Reset to Defaults:**
1. Quit HookAnchor completely (Activity Monitor if needed)
2. Backup important customizations from `~/.config/hookanchor/`
3. Delete `~/.config/hookanchor/` folder
4. Run HookAnchor - it will create fresh configuration files
5. Restore custom commands and settings from backup

**Partial Reset:**
- **Commands only**: Delete `commands.txt`, press Cmd+R to rescan
- **UI state**: Delete `state.json` to reset window positions and history
- **Cache**: Delete cache files, rebuild with Cmd+R
- **Logs**: Clear `anchor.log` and `server.log` files

**Configuration Recovery:**
Use backup files in `~/.config/hookanchor/backups/` to restore previous command databases or copy settings from the reference backup.

## RECOVERY -- Recovering from Errors

**Application Crashes:**
- Check Console.app for crash reports
- Review anchor.log for error messages before crash
- Restart with clean state by deleting state.json
- Run installer to repair corrupted files

**Command Database Corruption:**
- Restore from backup: `cp ~/.config/hookanchor/backups/commands_*.txt ~/.config/hookanchor/commands.txt`
- Clear and rescan: Delete commands.txt, press Cmd+R
- Manual repair: Edit commands.txt to fix malformed lines

**System Integration Issues:**
- Re-run installer to restore system integration
- Manually re-add Karabiner complex modifications
- Check and restore accessibility permissions
- Verify file associations and URL handlers

## UPDATES -- Updating HookAnchor

**Installation Process:**
1. Download latest HookAnchor.dmg from distribution source
2. Quit running HookAnchor processes
3. Drag new HookAnchor.app to Applications (replace existing)
4. Launch updated version
5. Configuration files are preserved automatically

**Version Verification:**
- Check version: `ha --version`
- Review changelog for breaking changes
- Test critical functionality after update
- Report issues if functionality regresses

**Configuration Migration:**
Most updates preserve existing configuration. Major version changes may require configuration updates - check release notes for migration instructions.

## SUPPORT -- Getting Help

**Documentation Resources:**
- This help file: Press **Cmd+?** in popup
- Configuration examples in config.yaml comments
- Log files for troubleshooting error details

**Debugging Information:**
When reporting issues, include:
- HookAnchor version (`ha --version`)
- Operating system version
- Relevant log entries from anchor.log
- Steps to reproduce the problem
- Expected vs actual behavior

**Community Support:**
- Check existing issues and solutions in project documentation
- Search logs for error patterns and solutions
- Test with minimal configuration to isolate problems