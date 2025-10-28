// HookAnchor Default JavaScript Configuration
// 
// This file contains JavaScript functions used by HookAnchor actions.
// It is automatically generated from the developer's personal config during build.
// Personal paths and values have been replaced with generic defaults.
// 
// Copy this to ~/.config/hookanchor/config.js and customize as needed.
// 
// All JavaScript functions have access to these built-in functions:
// - shell(command): Execute shell command asynchronously
// - shell_sync(command): Execute shell command synchronously  
// - log(message): Log message to HookAnchor logs
//

// HookAnchor Configuration Functions
// This file contains JavaScript action implementations that were previously embedded in config.yaml
// Each function receives a context object with the following structure:
// {
//   arg: string,              // The primary argument passed to the action
//   input: string,            // User's search input
//   previous: {               // Previous command information
//     name: string,
//     folder: string,
//     patch: string
//   },
//   grabbed: {                // Grabber information
//     action: string,
//     arg: string
//   },
//   date: {                   // Date components
//     YYYY: string,
//     MM: string,
//     DD: string
//   },
//   builtins: {               // All builtin functions
//     log, open_folder, shell, shell_sync, launch_app, change_directory,
//     readFile, expandHome, getObsidianVault, getObsidianVaultPath,
//     getObsidianApp, joinPath, file_exists, spawnDetached,
//     shellWithExitCode, encodeURIComponent, commandExists, etc.
//   }
// }

// Helper function for printing (mimics Rust's print function)
// Outputs to console and also logs to file with '|' prefix
function print(message) {
  // Print to console (if available)
  console.log(message);
  // Also log to file with '|' prefix to indicate console output
  if (typeof log !== 'undefined') {
    log('| ' + message);
  }
}

module.exports = {


  // Open folder - handles relative and absolute paths
  action_folder: function(ctx) {
    const { log, open_folder, joinPath, getObsidianVaultPath } = ctx.builtins;
    const folderPath = ctx.arg;
    
    detailed_log("FOLDER", `Starting with path: '${folderPath}'`);

    // Check if path is already absolute (starts with / or ~)
    if (folderPath.startsWith('/') || folderPath.startsWith('~')) {
      // Already absolute, use as-is
      detailed_log("FOLDER", `Using absolute path: '${folderPath}'`);
      try {
        open_folder(folderPath);
        detailed_log("FOLDER", `Successfully called open_folder for: '${folderPath}'`);
      } catch (error) {
        detailed_log("FOLDER", `Error opening folder: ${error}`);
      }
    } else {
      // Relative path - join with vault root
      const vaultRoot = getObsidianVaultPath();
      const absolutePath = joinPath(vaultRoot, folderPath);
      detailed_log("FOLDER", `Converting relative path '${folderPath}' to absolute: '${absolutePath}'`);
      try {
        open_folder(absolutePath);
        detailed_log("FOLDER", `Successfully called open_folder for: '${absolutePath}'`);
      } catch (error) {
        detailed_log("FOLDER", `Error opening folder: ${error}`);
      }
    }
    return "Folder opened successfully";
  },


  // Execute command - handles windowed (W flag) execution
  action_cmd: function(ctx) {
    const { log, shell, shell_sync } = ctx.builtins;
    const fullCmd = ctx.arg;

    // Check if command starts with 'W ' flag (handle different separations)
    if (fullCmd.startsWith('W ')) {
      // Remove 'W ' and get actual command  
      const command = fullCmd.substring(2);
      detailed_log("CMD", `Executing in Terminal window (W flag): '${command}'`);
      
      // Use osascript to open Terminal window with command
      // Escape both single and double quotes for AppleScript
      const escapedCmd = command.replace(/\\/g, '\\\\').replace(/"/g, '\\"').replace(/'/g, "'\"'\"'");
      shell(`osascript -e 'tell application "Terminal" to do script "${escapedCmd}"'`);
      
      // Brief delay to ensure Terminal has time to start
      shell_sync("/bin/sleep 0.2");
      
      detailed_log("CMD", `Terminal window opened with command`);
    } else if (fullCmd.startsWith('W; ')) {
      // Handle 'W; ' format (semicolon with space)
      const command = fullCmd.substring(3);
      detailed_log("CMD", `Executing in Terminal window (W; flag): '${command}'`);
      
      // Use osascript to open Terminal window with command
      const escapedCmd = command.replace(/\\/g, '\\\\').replace(/"/g, '\\"').replace(/'/g, "'\"'\"'");
      shell(`osascript -e 'tell application "Terminal" to do script "${escapedCmd}"'`);
      
      // Brief delay to ensure Terminal has time to start  
      shell_sync("/bin/sleep 0.2");
      
      detailed_log("CMD", `Terminal window opened with command`);
    } else if (fullCmd.startsWith('W;')) {
      // Handle 'W;' format (semicolon no space)
      const command = fullCmd.substring(2);
      detailed_log("CMD", `Executing in Terminal window (W; flag): '${command}'`);
      
      // Use osascript to open Terminal window with command
      const escapedCmd = command.replace(/\\/g, '\\\\').replace(/"/g, '\\"').replace(/'/g, "'\"'\"'");
      shell(`osascript -e 'tell application "Terminal" to do script "${escapedCmd}"'`);
      
      // Brief delay to ensure Terminal has time to start  
      shell_sync("/bin/sleep 0.2");
      
      detailed_log("CMD", `Terminal window opened with command`);
    } else {
      // Normal background execution
      detailed_log("CMD", `Executing in background: '${fullCmd}'`);
      shell(fullCmd);
    }
  },

  // Execute command with different modes based on flags:
  // No flags: Background execution (like CMD)
  // I flag: Interactive terminal that stays open
  // C flag: Interactive terminal that auto-closes when command exits
  action_console: function(ctx) {
    const { log, shell, shell_sync } = ctx.builtins;
    const command = ctx.arg;
    const hasInteractiveFlag = ctx.flags && ctx.flags.includes('I');
    const hasCloseFlag = ctx.flags && ctx.flags.includes('C');
    
    // Determine execution mode
    if (!hasInteractiveFlag && !hasCloseFlag) {
      // Mode 1: No flags - background execution (like CMD action)
      detailed_log("CONSOLE", `Executing in background: '${command}'`);
      shell(command);
      return `Background execution: ${command}`;
    } else {
      // Mode 2 & 3: Interactive terminal execution
      const willAutoClose = hasCloseFlag;
      
      detailed_log("CONSOLE", `Opening new terminal window with command: '${command}' (interactive: true, auto-close: ${willAutoClose})`);
      
      // Append exit command if C flag is present (auto-close mode)
      let finalCommand = command;
      if (willAutoClose) {
        finalCommand = `${command}; exit`;
        detailed_log("CONSOLE", `Auto-close enabled - final command: '${finalCommand}'`);
      }
      
      // Escape both single and double quotes for AppleScript
      const escapedCmd = finalCommand.replace(/\\/g, '\\\\').replace(/"/g, '\\"').replace(/'/g, "'\"'\"'");
      
      // Use AppleScript to create new Terminal window and run the command
      shell(`osascript -e 'tell application "Terminal" to do script "${escapedCmd}"'`);
      
      // Brief delay to ensure Terminal has time to start
      shell_sync("/bin/sleep 0.2");
      
      // Activate Terminal to bring it to foreground
      shell(`osascript -e 'tell application "Terminal" to activate'`);
      
      detailed_log("CONSOLE", `New terminal window opened and activated`);
      
      if (willAutoClose) {
        return `Console opened with: ${command} (auto-close)`;
      } else {
        return `Console opened with: ${command} (stays open)`;
      }
    }
  },


  // Open markdown file in Obsidian or default editor
  action_markdown: function(ctx) {
    const { log, shell, launch_app, expandHome, getObsidianVaultPath, getObsidianVault, getObsidianApp, encodeURIComponent } = ctx.builtins;
    const filePath = ctx.arg;
    
    detailed_log("MARKDOWN", `Processing file: '${filePath}'`);

    // Get the Obsidian vault path and expand it
    const vaultPath = getObsidianVaultPath();
    detailed_log("MARKDOWN", `Obsidian vault path: '${vaultPath}'`);

    // Normalize the file path (expand ~ and resolve ..)
    const normalizedFilePath = expandHome(filePath);
    const normalizedVaultPath = expandHome(vaultPath);

    detailed_log("MARKDOWN", `Normalized file path: '${normalizedFilePath}'`);
    detailed_log("MARKDOWN", `Normalized vault path: '${normalizedVaultPath}'`);

    // Check if the file is within the Obsidian vault
    if (normalizedFilePath.startsWith(normalizedVaultPath + '/') || normalizedFilePath === normalizedVaultPath) {
      // File is in Obsidian vault - open in Obsidian
      detailed_log("MARKDOWN", `File is in Obsidian vault, opening in Obsidian`);
      
      // Get the relative path from vault root
      const relativePath = normalizedFilePath.substring(normalizedVaultPath.length + 1);
      detailed_log("MARKDOWN", `Relative path in vault: '${relativePath}'`);
      
      // Remove .md extension if present
      const fileNameWithoutExt = relativePath.replace(/\.md$/, '');
      
      // Encode for URL
      const encoded = encodeURIComponent(fileNameWithoutExt);
      const vault = getObsidianVault();
      const app = getObsidianApp();
      const url = `obsidian://open?vault=${vault}&file=${encoded}`;
      
      detailed_log("MARKDOWN", `Opening Obsidian with URL: ${url}`);
      launch_app(app, url);
    } else {
      // File is outside vault - open with default markdown editor
      detailed_log("MARKDOWN", `File is outside Obsidian vault, opening with default app`);
      shell(`open "${filePath}"`);
    }
  },


  // Type text file contents into active application
  action_text: function(ctx) {
    const { log, readFile, shell, shell_sync } = ctx.builtins;
    const filePath = ctx.arg;
    
    // Get delay parameter from action config, default to 1.0 seconds
    const delay = ctx.params?.delay || 1.0;
    
    detailed_log("TEXT", `Reading file: '${filePath}'`);
    detailed_log("TEXT", `Using delay of ${delay} seconds before typing`);

    // Read the file contents
    let content;
    try {
      content = readFile(filePath);
      detailed_log("TEXT", `Successfully read ${content.length} characters from file`);
    } catch (error) {
      detailed_log("TEXT", `Error reading file: ${error}`);
      throw new Error(`Failed to read text file: ${error}`);
    }

    // Give focus back to the previous application (the one that was active before popup)
    detailed_log("TEXT", `Switching focus to previous application`);
    shell("osascript -e 'tell application \"System Events\" to key code 48 using command down'");
    shell_sync(`/bin/sleep ${delay}`); // Wait for focus switch with configurable delay

    // Type the content using a simpler approach
    detailed_log("TEXT", `Typing content into focused application after ${delay}s delay`);

    // Use AppleScript to type the content as a single string
    // This is faster and should complete more reliably
    const escapedContent = content
      .replace(/\\/g, '\\\\')
      .replace(/"/g, '\\"')
      .replace(/'/g, "\\'");

    const script = `osascript -e 'tell application "System Events" to keystroke "${escapedContent}"'`;

    try {
      shell_sync(script);
      detailed_log("TEXT", `Successfully typed content`);
    } catch (error) {
      detailed_log("TEXT", `Error typing content: ${error}`);
      throw new Error(`Failed to type text: ${error}`);
    }
  },







  // Open Notion page
  action_notion: function(ctx) {
    // Extract URL from context object
    const url = ctx.arg || ctx;
    
    // Use log function from context
    const { log, shell } = ctx.builtins || { log: console.log, shell: shell };
    
    detailed_log("NOTION_ACTION", "NOTION_ACTION: ========== START ==========");
    detailed_log("NOTION_ACTION", `ctx type: ${typeof ctx}`);
    detailed_log("NOTION_ACTION", `ctx.arg: '${ctx.arg}'`);
    detailed_log("NOTION_ACTION", `ctx.input: '${ctx.input}'`);
    detailed_log("NOTION_ACTION", `URL to open: '${url}'`);
    
    if (!url || url === "" || url === "undefined" || url === "null") {
      detailed_log("NOTION_ACTION", "NOTION_ACTION: WARNING - No URL provided or URL is empty/undefined");
      detailed_log("NOTION_ACTION", "NOTION_ACTION: Falling back to just activating Notion app");
      const fallbackCmd = `open -a "Notion"`;
      detailed_log("NOTION_ACTION", `Executing fallback: ${fallbackCmd}`);
      shell(fallbackCmd);
      return "Notion activated (no URL provided)";
    }
    
    // Check if URL looks valid
    if (typeof url === 'string' && !url.startsWith("http")) {
      detailed_log("NOTION_ACTION", `WARNING - URL doesn't start with http: '${url}'`);
      detailed_log("NOTION_ACTION", "NOTION_ACTION: Will try to open anyway...");
    }
    
    // Log the exact command we're about to run
    const openCmd = `open "${url}"`;
    detailed_log("NOTION_ACTION", `Executing command: ${openCmd}`);
    detailed_log("NOTION_ACTION", `URL being opened: ${url}`);
    
    // Execute and capture any output
    const result = shell(openCmd);
    detailed_log("NOTION_ACTION", `Shell command result: ${result}`);
    
    detailed_log("NOTION_ACTION", "NOTION_ACTION: ========== END ==========");
    return "Notion page opened";
  },

  // Open URL in Chrome browser
  action_chrome: function(ctx) {
    const url = ctx.arg;
    log(`[CHROME] ============= START =============`);
    log(`[CHROME] Function called with context:`);
    detailed_log("[CHROME]   ctx.arg", `'${ctx.arg}'`);
    detailed_log("JS", `[CHROME]   ctx.input: '${ctx.input}'`);
    detailed_log("JS", `[CHROME]   ctx.previous: '${ctx.previous}'`);
    detailed_log("JS", `[CHROME]   ctx.selected: '${ctx.selected}'`);
    detailed_log("JS", `[CHROME]   ctx.grabbed: '${ctx.grabbed}'`);
    detailed_log("JS", `[CHROME] URL to open: '${url}'`);
    
    const command = `open -a "Google Chrome" "${url}"`;
    detailed_log("JS", `[CHROME] Shell command to execute: '${command}'`);
    log(`[CHROME] Calling shell() function...`);
    
    try {
      const result = shell(command);
      log(`[CHROME] Shell execution completed`);
      detailed_log("JS", `[CHROME] Shell result: '${result}'`);
      log(`[CHROME] ============= END =============`);
      return `Opened ${url} in Chrome`;
    } catch (error) {
      log(`[CHROME] ERROR: Shell execution failed`);
      detailed_log("JS", `[CHROME] Error details: ${error}`);
      log(`[CHROME] ============= END WITH ERROR =============`);
      throw error;
    }
  },

  // Search and fill from 1Password
  action_1pass: function(ctx) {
    const { shell, shell_sync } = ctx.builtins;
    const searchTerm = ctx.arg;
    
    // Try Quick Access first (most universal)
    try {
      shell_sync("osascript -e 'tell application \"System Events\" to keystroke \" \" using {shift down, command down}'");
      shell_sync("/bin/sleep 0.25");  // Wait for Quick Access to fully open
      
      // Type character by character for better reliability
      shell_sync(`osascript -e 'tell application "System Events"
        repeat with i from 1 to length of "${searchTerm}"
          set currentChar to character i of "${searchTerm}"
          keystroke currentChar
          delay 0.05
        end repeat
      end tell'`);
      
      shell_sync("/bin/sleep 2.0");  // Wait for 1Password to search and show results
      shell_sync("osascript -e 'tell application \"System Events\" to key code 36'");  // Press Enter to select and open
      shell_sync("/bin/sleep 0.5");  // Wait for action to complete
    } catch (e) {
      // Fallback 1: Try menu bar access
      try {
        shell("osascript -e 'tell application \"System Events\" to tell process \"1Password 7 - Password Manager\" to click menu bar item 1 of menu bar 1'");
        shell_sync("/bin/sleep 0.5");
        shell(`osascript -e 'tell application "System Events" to keystroke "${searchTerm}"'`);
        shell("osascript -e 'tell application \"System Events\" to key code 36'");
      } catch (e2) {
        // Fallback 2: Open 1Password app and use search
        shell("osascript -e 'tell application \"1Password\" to activate'");
        shell_sync("/bin/sleep 0.5");
        shell("osascript -e 'tell application \"System Events\" to keystroke \"f\" using command down'");
        shell_sync("/bin/sleep 0.2");
        shell(`osascript -e 'tell application "System Events" to keystroke "${searchTerm}"'`);
        shell("osascript -e 'tell application \"System Events\" to key code 36'");
      }
    }
    return "1Password search initiated";
  },


  // Open document with default application
  action_doc: function(ctx) {
    const { log, shell, shellWithExitCode, expandHome, file_exists } = ctx.builtins;
    const filePath = expandHome(ctx.arg);
    
    detailed_log("DOC", `Opening document: '${filePath}'`);
    
    // Check if file exists
    if (!file_exists(filePath)) {
      detailed_log("DOC", `Warning - File does not exist: ${filePath}`);
      // Try to open anyway, let the OS handle the error
    }
    
    // Open with default application for this file type
    detailed_log("DOC", `Opening with default application using 'open' command`);
    const openCommand = `open "${filePath}"`;
    
    const resultJson = shellWithExitCode(openCommand);
    const result = JSON.parse(resultJson);
    
    if (result.exitCode === 0) {
      detailed_log("DOC", `✓ Successfully opened document with default app`);
      return `Document opened: ${filePath}`;
    } else {
      detailed_log("DOC", `✗ Failed to open document: ${result.stderr}`);
      return `Error opening document: ${filePath}`;
    }
  },

  // Search for contact in Contacts app
  action_contact: function(ctx) {
    const { log, shell, shell_sync, shellWithExitCode } = ctx.builtins;
    
    // Use the search parameter from config (e.g., "{{command}}") 
    // This allows users to control what gets searched for
    const searchTerm = ctx.search || ctx.arg;
    
    // Strip @ prefix if present
    const contactName = searchTerm.startsWith('@') ? searchTerm.substring(1) : searchTerm;
    
    detailed_log("CONTACT", `Starting search for contact: '${contactName}' (from search: '${searchTerm}')`);
    
    // Use AppleScript to open Contacts and search for the contact
    const escapedName = contactName.replace(/"/g, '\\"').replace(/'/g, "'\"'\"'");
    
    // AppleScript to open Contacts, search, and select the contact
    const script = `
      tell application "Contacts"
        -- First, make sure the app is running and visible
        activate
        delay 0.5
        
        -- If minimized, bring it back
        tell application "System Events"
          tell process "Contacts"
            set visible to true
            if (count of windows) > 0 then
              perform action "AXRaise" of window 1
            end if
          end tell
          set frontmost of process "Contacts" to true
        end tell
        delay 1
        
        -- Search for the contact
        set searchResults to every person whose name contains "${escapedName}"
        
        if (count of searchResults) > 0 then
          -- Select the contact using built-in selection
          set selection to item 1 of searchResults
          delay 0.5
          
          set contactCount to count of searchResults
          set selectedName to name of item 1 of searchResults
          return "FOUND_DIRECT:" & contactCount & " matches - Selected: " & selectedName
        else
          -- If no exact match, try searching in the search field
          tell application "System Events"
            tell process "Contacts"
              -- Use Cmd+F to focus search field
              keystroke "f" using command down
              delay 0.2
              -- Clear existing search
              keystroke "a" using command down
              delay 0.1
              keystroke "${escapedName}"
            end tell
          end tell
          return "SEARCH_FIELD:No direct match, using search field"
        end if
      end tell
    `;
    
    try {
      // Use shellWithExitCode to capture the result
      const resultJson = shellWithExitCode(`osascript -e '${script}'`);
      const result = JSON.parse(resultJson);
      
      if (result.exitCode === 0) {
        const output = result.stdout.trim();
        if (output.startsWith("FOUND_DIRECT:")) {
          detailed_log("CONTACT", `✓ Found directly in database - ${output.substring(13)}`);
          return `Found contact directly: ${contactName}`;
        } else if (output.startsWith("SEARCH_FIELD:")) {
          detailed_log("CONTACT", `⚠ No direct match found - falling back to search field for: ${contactName}`);
          return `Searching via search field: ${contactName}`;
        } else {
          detailed_log("CONTACT", `Unexpected response: ${output}`);
        }
      } else {
        detailed_log("CONTACT", `✗ AppleScript failed with exit code ${result.exitCode}: ${result.stderr}`);
        // Fallback: just open Contacts app and let user search manually
        detailed_log("CONTACT", `Falling back to opening Contacts app for manual search`);
        shell('open -a Contacts');
      }
    } catch (error) {
      detailed_log("CONTACT", `✗ Error executing contact search: ${error}`);
      // Fallback: just open Contacts app and let user search manually
      detailed_log("CONTACT", `Falling back to opening Contacts app for manual search`);
      shell('open -a Contacts');
    }
    
    return `Contact search completed: ${contactName}`;
  },


  // Navigate to Slack channel
  action_slack: function(ctx) {
    const { shell, shell_sync } = ctx.builtins;
    const channel = ctx.arg;
    
    // Use AppleScript to activate Slack and navigate to channel
    shell(`osascript -e 'tell application "Slack" to activate'`);
    shell_sync("/bin/sleep 0.5");
    // Open quick switcher with Cmd+K
    shell(`osascript -e 'tell application "System Events" to keystroke "k" using {command down}'`);
    shell_sync("/bin/sleep 0.5");
    // Type the channel name
    shell(`osascript -e 'tell application "System Events" to keystroke "${channel}"'`);
    shell_sync("/bin/sleep 0.5");
    // Press Enter to navigate
    shell(`osascript -e 'tell application "System Events" to keystroke return'`);
  },


  // Force rescan of markdown files
  action_rescan: function(ctx) {
    const { log } = ctx.builtins;
    log("Forcing rescan of markdown files and filesystem...");
    // This will trigger a rescan in the popup application
    return "Rescan triggered";
  },



  // Helper function: Give up focus to previous application
  give_up_focus: function(ctx) {
    const { shell, shell_sync } = ctx.builtins;
    // Give focus to the previous application using Cmd+Tab
    shell("osascript -e 'tell application \"System Events\" to key code 48 using command down'");
    shell_sync("/bin/sleep 0.15"); // Brief delay to ensure focus transfer completes
  },


  // Helper function: Regain focus for the anchor selector
  regain_focus: function(ctx) {
    const { shell, shell_sync } = ctx.builtins;
    // Bring focus back to the anchor selector application
    shell("osascript -e 'tell application \"popup\" to activate'");
    shell_sync("/bin/sleep 0.1"); // Brief delay to ensure focus transfer completes
  },


  // REMOVED: start_claude_code - was never used


  // Restored working tmux_activate action
  action_tmux_activate: function(arg, input, previous, selected, grabbed) {
    // The arg contains the full path to the anchor markdown file
    const fullPath = arg;
    const anchorDir = fullPath.substring(0, fullPath.lastIndexOf('/'));
    
    detailed_log("TMUX_ACTIVATE", `Checking for tmux session in: ${anchorDir}`);
    
    // Check if .tmuxp.yaml exists
    const tmuxpPath = joinPath(anchorDir, ".tmuxp.yaml");
    if (file_exists(tmuxpPath)) {
      detailed_log("TMUX_ACTIVATE", `Found .tmuxp.yaml at ${tmuxpPath}`);
      
      // Change to the anchor directory
      change_directory(anchorDir);
      
      const folderName = anchorDir.split('/').pop();
      detailed_log("TMUX_ACTIVATE", `Activating tmux session '${folderName}'`);
      
      try {
        // Get existing tmux sessions
        const sessionsResult = shellWithExitCode('/opt/homebrew/bin/tmux ls -F "#{session_name}" 2>/dev/null');
        const sessionsData = JSON.parse(sessionsResult);
        const existingSessions = sessionsData.exitCode === 0 ? 
            sessionsData.stdout.trim().split('\n').filter(s => s) : [];
        
        // Create session if it doesn't exist
        if (!existingSessions.includes(folderName)) {
          detailed_log("TMUX_ACTIVATE", `Creating session '${folderName}'`);
          const tmuxpResult = shell_sync(`tmuxp load "${tmuxpPath}" -d`);
          detailed_log("TMUX_ACTIVATE", `tmuxp load result: ${tmuxpResult}`);
          shell_sync("/bin/sleep 0.2");
        }
        
        // Try to attach to or switch to the session (same as anchor command)
        detailed_log("TMUX_ACTIVATE", `Attaching to session '${folderName}'`);
        try {
          // First try switch-client (works when already inside a tmux session)
          detailed_log("TMUX_ACTIVATE", `Trying tmux switch-client to '${folderName}'`);
          const switchResult = shellWithExitCode(`/opt/homebrew/bin/tmux switch-client -t "${folderName}"`);
          const switchData = JSON.parse(switchResult);
          
          if (switchData.exitCode === 0) {
            detailed_log("TMUX_ACTIVATE", `Successfully switched to session '${folderName}'`);
          } else {
            // switch-client failed, try attach-session (works when not inside tmux)
            detailed_log("TMUX_ACTIVATE", `Switch-client failed, trying attach-session to '${folderName}'`);
            const attachResult = shellWithExitCode(`/opt/homebrew/bin/tmux attach-session -t "${folderName}"`);
            const attachData = JSON.parse(attachResult);
            
            if (attachData.exitCode === 0) {
              detailed_log("TMUX_ACTIVATE", `Successfully attached to session '${folderName}'`);
            } else {
              detailed_log("TMUX_ACTIVATE", `Both switch-client and attach-session failed for '${folderName}'`);
              detailed_log("TMUX_ACTIVATE", `Switch error: ${switchData.stderr}`);
              detailed_log("TMUX_ACTIVATE", `Attach error: ${attachData.stderr}`);
            }
          }
        } catch (e) {
          detailed_log("TMUX_ACTIVATE", `Tmux execution error: ${e}`);
        }
        
        // Activate iTerm2 (same as anchor command)
        shell_sync("/bin/sleep 0.5");
        shell('osascript -e \'tell application "iTerm2" to activate\'');
        
      } catch (error) {
        detailed_log("TMUX_ACTIVATE", `Error with tmux session: ${error}`);
      }
    } else {
      detailed_log("TMUX_ACTIVATE", `No .tmuxp.yaml found at ${tmuxpPath}, skipping`);
    }
  },
  
  // Anchor action - behaves exactly like markdown action
  // Anchors and markdown files are both just documents that open the same way
  action_anchor: function(ctx) {
    const { log, error, file_exists, shell } = ctx.builtins;
    const arg = ctx.arg;
    
    // If no argument, silently do nothing (virtual anchors like "Notion Root")
    if (!arg) {
      detailed_log("ANCHOR", "ANCHOR: No argument provided, doing nothing (virtual anchor)");
      return;
    }
    
    detailed_log("ANCHOR", `Processing argument: '${arg}'`);
    
    // Check what type of argument we have
    // 1. If it's a Notion URL, open it
    if (arg.includes('notion.so')) {
      detailed_log("ANCHOR", `Detected Notion URL, opening: '${arg}'`);
      shell(`open "${arg}"`);
      return `Opened Notion page: ${arg}`;
    }
    
    // 2. If it ends with .md, it's a markdown file
    if (arg.endsWith('.md')) {
      detailed_log("ANCHOR", `Detected markdown file, delegating to action_markdown`);
      return this.action_markdown(ctx);
    }
    
    // 3. Check if it's a folder (exists and is a directory)
    // We'll check if the path exists and doesn't have a common file extension
    const commonExtensions = ['.txt', '.pdf', '.doc', '.docx', '.html', '.htm', 
                             '.jpg', '.jpeg', '.png', '.gif', '.mp4', '.mov',
                             '.zip', '.tar', '.gz', '.dmg', '.app'];
    const hasFileExtension = commonExtensions.some(ext => arg.toLowerCase().endsWith(ext));
    
    if (!hasFileExtension && file_exists(arg)) {
      // Likely a folder - delegate to folder action
      detailed_log("ANCHOR", `Detected folder, delegating to action_folder`);
      return this.action_folder(ctx);
    }
    
    // 4. If it has a file extension or appears to be a file path, treat as document
    if (hasFileExtension || arg.includes('.')) {
      detailed_log("ANCHOR", `Detected document, delegating to action_doc`);
      return this.action_doc(ctx);
    }
    
    // 5. Otherwise, generate an error
    error(`Illegal anchor argument: '${arg}' - must be a Notion URL, markdown file, folder, or document`);
    detailed_log("ANCHOR", `ERROR - Unrecognized argument type: '${arg}'`);
  },

  // TEST: Function to test JavaScript error reporting
  action_test_error: function(ctx) {
    const { error, detailed_log } = ctx.builtins;
    detailed_log("TEST_ERROR", "About to throw a JavaScript error for testing");
    
    // This will definitely throw an error
    nonexistent_function_call();
    
    return "This should never be reached";
  },
  
  // NEW: TMUX activation using only the shell commands available to JavaScript
  // This reimplements the Rust version using shellWithExitCode, shell, and error
  action_activate_tmux: function(ctx) {
    const { log, shell, shellWithExitCode, error, file_exists, detailed_log, dirname } = ctx.builtins;
    const fullPath = ctx.arg;
    
    try {
      log("🚀 TMUX_DEBUG: ==================== TMUX ACTIVATION START ====================");
      log(`🚀 TMUX_DEBUG: Received arg: '${fullPath}'`);
      log(`🚀 TMUX_DEBUG: Type of arg: ${typeof fullPath}`);
      
      if (!fullPath) {
        log("🚀 TMUX_DEBUG: ERROR - No path provided");
        error("No path provided for TMUX activation");
        return;
      }
    
      // Extract folder path from the full path (might be a .md file or folder)
    let folder_path;
    if (fullPath.endsWith('.md')) {
      detailed_log("TMUX_DEBUG", "Path ends with .md, extracting parent directory");
      // It's a markdown file, get the parent directory using dirname built-in
      folder_path = dirname(fullPath);
      detailed_log("TMUX_DEBUG", `Extracted folder_path using dirname: '${folder_path}'`);
    } else {
      detailed_log("TMUX_DEBUG", "Path is already a folder");
      // It's already a folder
      folder_path = fullPath;
    }
    
    detailed_log("ACTIVATE_TMUX_JS", `Starting activation for folder: ${folder_path}`);
    detailed_log("TMUX_DEBUG", `Line 703: folder_path = '${folder_path}'`);
    
    // 1. Open folder in Finder (matching Rust implementation)
    detailed_log("TMUX_DEBUG", `Line 715: Opening folder in Finder: '${folder_path}'`);
    try {
      shell(`open "${folder_path}"`);
      detailed_log("TMUX_DEBUG", "Line 716: Finder open command executed successfully");
    } catch (e) {
      detailed_log("TMUX_DEBUG", `Line 716: ERROR opening Finder: ${e}`);
    }
    
    // 2. Check for .tmuxp.yaml
    const tmuxp_path = `${folder_path}/.tmuxp.yaml`;
    detailed_log("TMUX_DEBUG", `Line 718: Checking for .tmuxp.yaml at: '${tmuxp_path}'`);
    
    if (!file_exists(tmuxp_path)) {
      detailed_log("TMUX_DEBUG", "Line 719: No .tmuxp.yaml found");
      log("ACTIVATE_TMUX_JS: No .tmuxp.yaml found");
      
      // Check for CLAUDE.md as fallback
      const claude_path = `${folder_path}/CLAUDE.md`;
      detailed_log("TMUX_DEBUG", `Line 723: Checking for CLAUDE.md at: '${claude_path}'`);
      
      if (file_exists(claude_path)) {
        detailed_log("TMUX_DEBUG", "Line 724: Found CLAUDE.md, launching Claude Code");
        log("ACTIVATE_TMUX_JS: Found CLAUDE.md, launching Claude Code");
        // Change to the directory and launch Claude Code
        const claudeCmd = `cd "${folder_path}" && /opt/homebrew/bin/claude --continue`;
        detailed_log("TMUX_DEBUG", `Line 726: Running command: '${claudeCmd}'`);
        detailed_log("ACTIVATE_TMUX_JS", `Running: ${claudeCmd}`);
        try {
          shell(claudeCmd);
          detailed_log("TMUX_DEBUG", "Line 728: Claude command executed");
        } catch (e) {
          detailed_log("TMUX_DEBUG", `Line 728: ERROR running Claude: ${e}`);
        }
      } else {
        detailed_log("TMUX_DEBUG", "Line 729: No CLAUDE.md found either");
      }
      detailed_log("TMUX_DEBUG", "Line 730: Returning early - no tmuxp.yaml");
      return;
    }
    
    detailed_log("TMUX_DEBUG", `Line 733: Found .tmuxp.yaml at ${tmuxp_path}`);
    detailed_log("ACTIVATE_TMUX_JS", `Found .tmuxp.yaml at ${tmuxp_path}`);
    
    // 3. Get session name from folder name and sanitize it
    const folder_name = folder_path.split('/').pop() || 'session';
    const session_name = folder_name
      .replace(/ /g, '_')
      .replace(/:/g, '_')
      .replace(/\./g, '_')
      .replace(/\[/g, '_')
      .replace(/\]/g, '_');
    
    detailed_log("ACTIVATE_TMUX_JS", `Folder name: '${folder_name}', Session name: '${session_name}'`);
    
    // 4. Check if session exists
    detailed_log("TMUX_DEBUG", `Line 738: Checking if session '${session_name}' exists`);
    const checkCmd = `/opt/homebrew/bin/tmux has-session -t "${session_name}" 2>/dev/null`;
    detailed_log("TMUX_DEBUG", `Line 739: Running command: '${checkCmd}'`);
    
    const checkResult = shellWithExitCode(checkCmd);
    detailed_log("TMUX_DEBUG", `Line 740: Raw checkResult: '${checkResult}'`);
    
    let checkData;
    try {
      checkData = JSON.parse(checkResult);
      detailed_log("TMUX_DEBUG", `Line 742: Parsed checkData - exitCode: ${checkData.exitCode}, stdout: '${checkData.stdout}', stderr: '${checkData.stderr}'`);
    } catch (e) {
      detailed_log("TMUX_DEBUG", `Line 743: ERROR - Failed to parse check result: ${e}`);
      detailed_log("TMUX_DEBUG", `Line 743: Raw result was: '${checkResult}'`);
      detailed_log("ACTIVATE_TMUX_JS", `Failed to parse check result: ${e}`);
      error("Failed to check TMUX session status");
      return;
    }
    
    const session_exists = checkData.exitCode === 0;
    detailed_log("TMUX_DEBUG", `Line 748: Session exists: ${session_exists}`);
    
    if (!session_exists) {
      detailed_log("TMUX_DEBUG", `Line 751: Session does not exist, creating new session '${session_name}'`);
      detailed_log("ACTIVATE_TMUX_JS", `Creating new tmux session '${session_name}'`);
      
      // Create session with tmuxp
      // Change to directory first, then load tmuxp config
      const createCmd = `cd "${folder_path}" && /opt/homebrew/bin/tmuxp load ".tmuxp.yaml" -d 2>&1`;
      detailed_log("TMUX_DEBUG", `Line 755: Running tmuxp command: '${createCmd}'`);
      
      const createResult = shellWithExitCode(createCmd);
      detailed_log("TMUX_DEBUG", `Line 756: Raw createResult: '${createResult}'`);
      
      let createData;
      try {
        createData = JSON.parse(createResult);
        detailed_log("TMUX_DEBUG", `Line 759: Parsed createData - exitCode: ${createData.exitCode}`);
        detailed_log("TMUX_DEBUG", `Line 759: stdout: '${createData.stdout}'`);
        detailed_log("TMUX_DEBUG", `Line 759: stderr: '${createData.stderr}'`);
      } catch (e) {
        detailed_log("TMUX_DEBUG", `Line 760: ERROR - Failed to parse create result: ${e}`);
        detailed_log("TMUX_DEBUG", `Line 760: Raw result was: '${createResult}'`);
        detailed_log("ACTIVATE_TMUX_JS", `Failed to parse create result: ${e}`);
        error("Failed to create TMUX session");
        return;
      }
      
      if (createData.exitCode !== 0) {
        detailed_log("TMUX_DEBUG", `Line 765: tmuxp failed with exit code ${createData.exitCode}`);
        detailed_log("ACTIVATE_TMUX_JS", `tmuxp failed with exit code ${createData.exitCode}`);
        detailed_log("ACTIVATE_TMUX_JS", `stderr: ${createData.stderr}`);
        detailed_log("ACTIVATE_TMUX_JS", `stdout: ${createData.stdout}`);
        error(`Failed to create TMUX session: ${createData.stderr || createData.stdout}`);
        return;
      }
      
      log("ACTIVATE_TMUX_JS: tmuxp completed successfully");
      
      // Wait longer for session to fully initialize
      shell_sync("/bin/sleep 1.5");
      
      // Verify session was created
      const verifyResult = shellWithExitCode(`/opt/homebrew/bin/tmux has-session -t "${session_name}" 2>/dev/null`);
      try {
        const verifyData = JSON.parse(verifyResult);
        if (verifyData.exitCode !== 0) {
          detailed_log("ACTIVATE_TMUX_JS", `WARNING: Session '${session_name}' was NOT created despite tmuxp success`);
          // Try basic tmux fallback
          const basicResult = shellWithExitCode(`/opt/homebrew/bin/tmux new-session -d -s "${session_name}" -c "${folder_path}" 2>&1`);
          const basicData = JSON.parse(basicResult);
          if (basicData.exitCode !== 0) {
            detailed_log("ACTIVATE_TMUX_JS", `Basic tmux also failed: ${basicData.stderr}`);
          }
        }
      } catch (e) {
        detailed_log("ACTIVATE_TMUX_JS", `Failed to verify session creation: ${e}`);
      }
    } else {
      detailed_log("TMUX_DEBUG", `Line 794: Session '${session_name}' already exists, skipping creation`);
      detailed_log("ACTIVATE_TMUX_JS", `Session '${session_name}' already exists`);
    }
    
    // Longer delay to ensure session is fully ready
    detailed_log("TMUX_DEBUG", "Line 798: Sleeping for 1 second to ensure session is ready");
    try {
      shell_sync("/bin/sleep 1.0");
      detailed_log("TMUX_DEBUG", "Line 799: Sleep completed");
    } catch (e) {
      detailed_log("TMUX_DEBUG", `Line 799: ERROR during sleep: ${e}`);
    }
    
    // 5. Check if there are existing TMUX clients and switch/attach accordingly
    log(`ACTIVATE_TMUX_JS: Checking for existing TMUX clients`);
    const clientsResult = shellWithExitCode("/opt/homebrew/bin/tmux list-clients 2>/dev/null");
    let has_clients = false;
    try {
      const clientsData = JSON.parse(clientsResult);
      has_clients = clientsData.exitCode === 0 && clientsData.stdout.trim().length > 0;
      if (has_clients) {
        detailed_log("ACTIVATE_TMUX_JS", `Found TMUX clients: ${clientsData.stdout.trim()}`);
      } else {
        log(`ACTIVATE_TMUX_JS: No TMUX clients found`);
      }
    } catch (e) {
      detailed_log("ACTIVATE_TMUX_JS", `Failed to check clients: ${e}`);
    }
    
    // Different approach based on whether clients exist
    if (has_clients) {
      // There are existing TMUX clients - use switch-client
      // When called from outside TMUX, this switches the most recently used client
      log(`ACTIVATE_TMUX_JS: Using switch-client for existing TMUX clients`);
      const switchResult = shellWithExitCode(`/opt/homebrew/bin/tmux switch-client -t "${session_name}" 2>&1`);
      
      try {
        const switchData = JSON.parse(switchResult);
        if (switchData.exitCode === 0) {
          detailed_log("ACTIVATE_TMUX_JS", `Successfully switched to session '${session_name}'`);
          // Activate Terminal to bring it to foreground
          shell(`osascript -e 'tell application "Terminal" to activate'`);
        } else {
          detailed_log("ACTIVATE_TMUX_JS", `Failed to switch: ${switchData.stderr || switchData.stdout}`);
          error(`Failed to switch to TMUX session: ${switchData.stderr || switchData.stdout}`);
        }
      } catch (e) {
        detailed_log("ACTIVATE_TMUX_JS", `Failed to parse switch result: ${e}`);
      }
    } else {
      // No existing clients - show error (matching Rust behavior)
      log(`ACTIVATE_TMUX_JS: No TMUX clients found - showing error to user`);
      error(
        `Cannot switch to TMUX session '${session_name}': No TMUX client is running.\n\n` +
        `Please open a terminal and run:\n` +
        `tmux attach-session -t '${session_name}'`
      );
    }
    
    // Request exit after successful activation
    detailed_log("TMUX_DEBUG", "Line 908: Function completed successfully, returning exit:true");
    detailed_log("TMUX_DEBUG", "==================== TMUX ACTIVATION END ====================");
    return { exit: true };
    } catch (e) {
      detailed_log("TMUX_DEBUG", `ERROR in action_activate_tmux: ${e}`);
      error(`TMUX activation failed: ${e}`);
    }
  },

};
