const fullPath = "{{arg}}";
const anchorDir = fullPath.substring(0, fullPath.lastIndexOf('/'));

log(`ANCHOR: Opening anchor at: ${fullPath}`);
log(`ANCHOR: Directory: ${anchorDir}`);

// Change to the anchor directory
change_directory(anchorDir);

// Open the folder in Finder
open_folder(anchorDir);

// Always open the markdown file in Obsidian first
const baseName = fullPath.split('/').pop().replace(/\.md$/, '');
const encoded = encodeURIComponent(baseName);
const vault = getObsidianVault();
const app = getObsidianApp();
const url = `obsidian://open?vault=${vault}&file=${encoded}`;
log(`ANCHOR: Opening in Obsidian: ${url}`);
launch_app(app, url);

// JavaScript tmux processing (based on working Python implementation)
const tmuxpPath = joinPath(anchorDir, ".tmuxp.yaml");
if (file_exists(tmuxpPath)) {
    log(`ANCHOR: Found .tmuxp.yaml, activating tmux session`);
    
    // Get folder name for session
    const folderName = anchorDir.split('/').pop();
    
    try {
        // Get list of existing tmux sessions
        const sessionsOutput = shell("PATH=/opt/homebrew/bin:$PATH /opt/homebrew/bin/tmux ls 2>/dev/null || true");
        const existingSessions = sessionsOutput.split('\n')
            .filter(line => line.includes(':'))
            .map(line => line.split(':')[0].trim())
            .filter(name => name.length > 0);
        log(`ANCHOR: Existing tmux sessions: ${existingSessions.join(', ')}`);
        
        if (existingSessions.includes(folderName)) {
            log(`ANCHOR: Session '${folderName}' already exists, attaching to it`);
        } else {
            log(`ANCHOR: Creating new tmux session '${folderName}' from profile`);
            // Create session in detached mode using tmuxp with explicit PATH
            shell(`PATH=/opt/homebrew/bin:$PATH /opt/homebrew/bin/tmuxp load "${tmuxpPath}" -d`);
            log(`ANCHOR: Tmux session '${folderName}' created successfully`);
            // Give tmux a moment to fully create the session
            shell_sync("/bin/sleep 0.2");
        }
        
        // Attach to the session - KEY: Use tmux commands directly without capturing output
        log(`ANCHOR: Attaching to tmux session '${folderName}'...`);
        try {
            // Try switch-client first (works when already inside a tmux session)
            // Use spawnDetached for tmux commands to avoid interfering with session control
            const switchCmd = `PATH=/opt/homebrew/bin:$PATH /opt/homebrew/bin/tmux switch-client -t "${folderName}"`;
            try {
                spawnDetached("bash", "-c", switchCmd);
                log(`ANCHOR: Sent switch-client command for session '${folderName}'`);
            } catch (switchError) {
                log(`ANCHOR: Switch-client failed, trying attach-session`);
                // Fall back to attach-session (works when not inside tmux)
                const attachCmd = `PATH=/opt/homebrew/bin:$PATH /opt/homebrew/bin/tmux attach-session -t "${folderName}"`;
                spawnDetached("bash", "-c", attachCmd);
                log(`ANCHOR: Sent attach-session command for session '${folderName}'`);
            }
        } catch (attachError) {
            log(`ANCHOR: Error attaching to session: ${attachError}`);
        }
        
        // Wait a moment then activate iTerm2
        shell_sync("/bin/sleep 0.5");
        shell('osascript -e \'tell application "iTerm2" to activate\'');
        
    } catch (error) {
        log(`ANCHOR: Error managing tmux session: ${error}`);
        log(`ANCHOR: Make sure tmux and tmuxp are installed at /opt/homebrew/bin/`);
    }
} else {
    log(`ANCHOR: No .tmuxp.yaml found, checking for CLAUDE.md fallback`);
    
    // Check for CLAUDE.md and run claude --continue as fallback
    const claudeMdPath = joinPath(anchorDir, "CLAUDE.md");
    if (file_exists(claudeMdPath)) {
        log(`ANCHOR: Found CLAUDE.md, running claude --continue`);
        try {
            spawnDetached("claude", "--continue");
            log(`ANCHOR: Started claude --continue in background`);
        } catch (claudeError) {
            log(`ANCHOR: Failed to start claude --continue: ${claudeError}`);
        }
    } else {
        log(`ANCHOR: No tmux or Claude configuration found, Obsidian only`);
    }
}