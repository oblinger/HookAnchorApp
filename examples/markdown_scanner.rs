//! Markdown Scanner Example
//!
//! Demonstrates using JavaScript business logic to scan markdown files
//! and generate commands based on frontmatter metadata.

use hookanchor::js_runtime::execute_business_logic;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Markdown Command Scanner ===\n");
    
    // Business logic entirely in JavaScript
    let scanner_script = r#"
        // Business Logic: Scan markdown files for commands
        function scanMarkdownCommands() {
            log("Starting markdown scan...");
            
            // Configuration - easy to modify without recompiling
            const searchDirs = [
                expandHome("~/documents"),
                expandHome("~/notes"),
                expandHome("~/projects")
            ];
            
            const skipPatterns = [
                "draft",
                "template",
                "archive",
                ".git",
                "node_modules"
            ];
            
            let foundCommands = [];
            
            for (let i = 0; i < searchDirs.length; i++) {
                const dir = searchDirs[i];
                if (!fileExists(dir)) {
                    debug("Directory not found: " + dir);
                    continue;
                }
                
                log("Scanning: " + dir);
                const files = listFiles(dir, "recursive");
                
                for (let j = 0; j < files.length; j++) {
                    const file = files[j];
                    
                    // Skip if matches skip pattern
                    let skip = false;
                    for (let k = 0; k < skipPatterns.length; k++) {
                        if (file.indexOf(skipPatterns[k]) >= 0) {
                            skip = true;
                            break;
                        }
                    }
                    if (skip) continue;
                    
                    // Process only markdown files
                    if (getExtension(file) === "md") {
                        const cmd = extractCommandFromMarkdown(file);
                        if (cmd) {
                            foundCommands.push(cmd);
                        }
                    }
                }
            }
            
            log("Found " + foundCommands.length + " commands");
            return foundCommands;
        }
        
        function extractCommandFromMarkdown(filepath) {
            try {
                const content = readFile(filepath);
                const frontmatter = parseFrontmatter(content);
                
                if (frontmatter && frontmatter.command) {
                    // Business rules for command generation
                    const command = {
                        command: frontmatter.command,
                        action: frontmatter.action || inferAction(filepath),
                        arg: filepath,
                        group: frontmatter.group || inferGroup(filepath),
                        full_line: generateFullLine(frontmatter, filepath)
                    };
                    
                    log("Found command: " + command.command + " in " + basename(filepath));
                    return command;
                }
            } catch (e) {
                debug("Error processing file: " + filepath);
            }
            return null;
        }
        
        function parseFrontmatter(content) {
            if (content.indexOf("---") === 0) {
                const endIndex = content.indexOf("\n---", 4);
                if (endIndex > 0) {
                    const yamlText = content.substring(4, endIndex);
                    try {
                        const parsed = parseYaml(yamlText);
                        // parseYaml returns JSON string, so parse it
                        return JSON.parse(parsed);
                    } catch (e) {
                        debug("Failed to parse YAML frontmatter");
                        return null;
                    }
                }
            }
            return null;
        }
        
        function inferAction(filepath) {
            // Business logic: infer action from file type/location
            if (filepath.indexOf("/projects/") >= 0) {
                return "folder";
            } else if (filepath.indexOf("/scripts/") >= 0) {
                return "cmd";
            } else {
                return "doc";
            }
        }
        
        function inferGroup(filepath) {
            // Business logic: infer group from directory structure
            const parts = filepath.split("/");
            for (let i = parts.length - 2; i >= 0; i--) {
                const part = parts[i];
                if (part !== "documents" && part !== "notes" && part !== "projects") {
                    return part;
                }
            }
            return "docs";
        }
        
        function generateFullLine(frontmatter, filepath) {
            // Generate the full command line format
            let line = "";
            if (frontmatter.group) {
                line = frontmatter.group + " ! ";
            }
            line += frontmatter.command + " : ";
            line += frontmatter.action || inferAction(filepath);
            line += " " + filepath;
            return line;
        }
        
        // Execute the scan
        const commands = scanMarkdownCommands();
        
        // Would normally save these commands, but for demo just return count
        "Found " + commands.length + " commands"
    "#;
    
    // Execute the JavaScript business logic
    match execute_business_logic(scanner_script) {
        Ok(result) => {
            println!("\nResult: {}", result);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
    
    println!("\n=== Benefits of this approach ===");
    println!("✅ Business logic is easy to modify without recompiling");
    println!("✅ Configuration (dirs, patterns) can be tweaked on the fly");
    println!("✅ Complex parsing rules are readable and maintainable");
    println!("✅ No external JavaScript runtime required");
    println!("✅ Sandboxed - only has access to provided functions");
    
    Ok(())
}