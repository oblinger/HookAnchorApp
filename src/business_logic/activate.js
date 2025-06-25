/**
 * Anchor Activate - User-Customizable Project Activation
 * 
 * This script defines the default behavior for activating an anchor/project.
 * Users can customize this behavior by editing this script or providing 
 * per-directory activation scripts.
 * 
 * # JavaScript API Reference
 * 
 * For complete API documentation, see src/js_runtime.rs module.
 * 
 * ## Most Commonly Used Functions in Activation Scripts
 * - **Logging**: log(), debug(), error()
 * - **File Operations**: fileExists(), isDirectory(), readFile()
 * - **System Control**: launch_app(), shell(), open_folder(), change_directory()
 * - **Path Utilities**: expandHome(), basename(), joinPath()
 * - **Development Tools**: start_tmux_session(), activate_iterm(), start_claude_code()
 * - **Tmux Integration**: has_tmux_session(), start_tmux_session()
 */

// Configuration - users can customize these defaults
const CONFIG = {
    // Which applications to use
    folderApp: "Finder",           // Could be "Path Finder", "Forklift", etc.
    terminalApp: "iTerm2",         // Could be "Terminal", "Alacritty", etc.
    obsidianVault: "kmr",          // Obsidian vault name
    
    // Timing delays (in seconds)
    tmuxDelay: 0.5,               // Delay before activating terminal after tmux
    
    // Feature toggles
    openFolder: true,             // Open folder in Finder
    openObsidian: true,           // Try to open markdown file in Obsidian
    enableTmux: true,             // Enable tmux session handling
    enableClaudeCode: true,       // Enable Claude Code fallback
};

/**
 * Main activate function - called with the anchor path
 */
function activate(anchorPath) {
    log(`=== Activating Anchor: ${anchorPath} ===`);
    
    // Validate and change to the anchor directory
    if (!isDirectory(anchorPath)) {
        error(`Anchor directory does not exist: ${anchorPath}`);
        return;
    }
    
    change_directory(anchorPath);
    const anchorName = basename(anchorPath);
    log(`Changed to directory: ${anchorPath}`);
    
    // Step 1: Open folder (configurable)
    if (CONFIG.openFolder) {
        open_folder(anchorPath);
        log(`Opened folder in ${CONFIG.folderApp}`);
    }
    
    // Step 2: Try to open in Obsidian
    if (CONFIG.openObsidian) {
        openInObsidian(anchorPath, anchorName);
    }
    
    // Step 3: Handle development environment activation
    if (CONFIG.enableTmux && activateTmux(anchorPath, anchorName)) {
        // Tmux session was started/attached, bring terminal to front
        delay(() => {
            activate_iterm();
            log("Activated iTerm2 after tmux setup");
        });
    } else if (CONFIG.enableClaudeCode && fileExists("CLAUDE.md")) {
        // No tmux, but has Claude config - start Claude Code
        start_claude_code();
        log("Started Claude Code (no tmux session)");
    }
    
    log("=== Anchor activation complete ===");
}

/**
 * Try to open the anchor markdown file in Obsidian
 */
function openInObsidian(anchorPath, anchorName) {
    const markdownFile = `${anchorPath}/${anchorName}.md`;
    
    if (!fileExists(markdownFile)) {
        debug(`No markdown file found: ${markdownFile}`);
        return;
    }
    
    // Try to open in Obsidian vault
    // Note: This assumes the anchor is within the Obsidian vault
    // Users can customize this logic for their setup
    
    const vaultRoot = expandHome("~/ob/kmr");
    
    try {
        // Calculate relative path from vault root
        const relativePath = markdownFile.replace(vaultRoot + "/", "");
        const obsidianUri = `obsidian://open?vault=${CONFIG.obsidianVault}&file=${simpleEncode(relativePath)}`;
        
        shell(`open "${obsidianUri}"`);
        log(`Opened in Obsidian: ${relativePath}`);
    } catch (e) {
        // Fallback: open with default app
        shell(`open "${markdownFile}"`);
        log(`Opened with default app: ${markdownFile}`);
    }
}

/**
 * Activate tmux session if .tmuxp.yaml exists
 * Returns true if tmux was activated, false otherwise
 */
function activateTmux(anchorPath, anchorName) {
    const tmuxConfig = `${anchorPath}/.tmuxp.yaml`;
    
    if (!fileExists(tmuxConfig)) {
        debug("No .tmuxp.yaml found, skipping tmux");
        return false;
    }
    
    log(`Found tmux config: ${tmuxConfig}`);
    
    // Check if session already exists
    if (has_tmux_session(anchorName)) {
        log(`Tmux session '${anchorName}' already exists, attaching`);
        // Session exists, just attach to it
        shell(`tmux attach-session -t "${anchorName}"`);
    } else {
        log(`Creating new tmux session: ${anchorName}`);
        // Create new session from config
        start_tmux_session(tmuxConfig);
        
        // Give tmux a moment to fully create the session
        delay(() => {
            shell(`tmux attach-session -t "${anchorName}"`);
        });
    }
    
    return true;
}

/**
 * Utility function for delays - simplified for QuickJS
 * In QuickJS, we can't do real async delays, so we execute immediately
 */
function delay(callback) {
    // Execute immediately since QuickJS doesn't support real setTimeout
    callback();
}

// Simple URL encoding for Obsidian URIs (replaces encodeURIComponent)
function simpleEncode(str) {
    // Basic encoding for most common cases
    return str.replace(/ /g, '%20').replace(/&/g, '%26').replace(/#/g, '%23');
}

// Make activate function available globally
// (don't use 'return' at module level in QuickJS)
this.activate = activate;