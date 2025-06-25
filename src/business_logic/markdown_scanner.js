// Markdown Scanner Business Logic
// This file can be modified without recompiling the Rust application

// Configuration
const CONFIG = {
    searchDirs: [
        "~/documents",
        "~/notes", 
        "~/obsidian",
        "~/projects"
    ],
    
    skipPatterns: [
        "draft",
        "template",
        "archive",
        ".trash",
        ".git",
        "node_modules"
    ],
    
    commandDefaults: {
        action: "doc",
        group: "docs"
    }
};

// Main scanning function
function scanForCommands() {
    log("=== Markdown Command Scanner ===");
    
    let allCommands = [];
    let scannedFiles = 0;
    
    for (let i = 0; i < CONFIG.searchDirs.length; i++) {
        const dir = expandHome(CONFIG.searchDirs[i]);
        
        if (!fileExists(dir)) {
            debug("Skipping non-existent directory: " + dir);
            continue;
        }
        
        log("Scanning directory: " + dir);
        const commands = scanDirectory(dir);
        allCommands = allCommands.concat(commands);
        
        log("  Found " + commands.length + " commands");
    }
    
    log("\nTotal commands found: " + allCommands.length);
    return allCommands;
}

function scanDirectory(dir) {
    const files = listFiles(dir, "recursive");
    const commands = [];
    
    for (let i = 0; i < files.length; i++) {
        const file = files[i];
        
        // Check skip patterns
        if (shouldSkipFile(file)) {
            continue;
        }
        
        // Process markdown files
        if (getExtension(file) === "md") {
            const cmd = processMarkdownFile(file);
            if (cmd) {
                commands.push(cmd);
            }
        }
    }
    
    return commands;
}

function shouldSkipFile(filepath) {
    for (let i = 0; i < CONFIG.skipPatterns.length; i++) {
        if (filepath.indexOf(CONFIG.skipPatterns[i]) >= 0) {
            return true;
        }
    }
    return false;
}

function processMarkdownFile(filepath) {
    try {
        const content = readFile(filepath);
        
        // Try to extract frontmatter
        const frontmatter = extractFrontmatter(content);
        if (frontmatter && frontmatter.command) {
            return createCommand(frontmatter, filepath);
        }
        
        // Try to extract from first heading
        const titleCommand = extractFromTitle(content, filepath);
        if (titleCommand) {
            return titleCommand;
        }
        
    } catch (e) {
        debug("Error processing file: " + filepath + " - " + e);
    }
    
    return null;
}

function extractFrontmatter(content) {
    if (content.indexOf("---") !== 0) {
        return null;
    }
    
    const endIndex = content.indexOf("\n---", 4);
    if (endIndex < 0) {
        return null;
    }
    
    const yamlText = content.substring(4, endIndex);
    try {
        const jsonStr = parseYaml(yamlText);
        return JSON.parse(jsonStr);
    } catch (e) {
        debug("Failed to parse frontmatter: " + e);
        return null;
    }
}

function extractFromTitle(content, filepath) {
    // Look for # Title pattern
    const lines = content.split("\n");
    for (let i = 0; i < Math.min(lines.length, 5); i++) {
        const line = lines[i].trim();
        if (line.indexOf("# ") === 0) {
            const title = line.substring(2).trim();
            if (title.length > 0 && title.length < 50) {
                // Create command from title
                return {
                    command: slugify(title),
                    action: CONFIG.commandDefaults.action,
                    arg: filepath,
                    group: inferGroupFromPath(filepath),
                    full_line: generateCommandLine(slugify(title), CONFIG.commandDefaults.action, filepath)
                };
            }
        }
    }
    return null;
}

function createCommand(frontmatter, filepath) {
    const command = frontmatter.command;
    const action = frontmatter.action || CONFIG.commandDefaults.action;
    const group = frontmatter.group || inferGroupFromPath(filepath);
    
    return {
        command: command,
        action: action,
        arg: filepath,
        group: group,
        full_line: generateCommandLine(command, action, filepath, group)
    };
}

function inferGroupFromPath(filepath) {
    // Extract meaningful group from directory structure
    const parts = filepath.split("/");
    
    // Look for project/category folders
    for (let i = parts.length - 2; i >= 0; i--) {
        const part = parts[i];
        // Skip common root folders
        if (part !== "documents" && part !== "notes" && part !== "obsidian" && part !== "projects") {
            // Use the folder name as group
            return part.toLowerCase();
        }
    }
    
    return CONFIG.commandDefaults.group;
}

function generateCommandLine(command, action, arg, group) {
    let line = "";
    if (group && group !== CONFIG.commandDefaults.group) {
        line = group + " ! ";
    }
    line += command + " : " + action;
    if (arg) {
        line += " " + arg;
    }
    return line;
}

function slugify(text) {
    // Convert text to command-friendly slug
    return text
        .toLowerCase()
        .replace(/[^a-z0-9]+/g, "_")
        .replace(/^_+|_+$/g, "");
}

// Entry point
scanForCommands();