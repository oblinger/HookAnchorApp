//! Hybrid JavaScript Business Logic Example
//!
//! Demonstrates using JavaScript for business logic with Rust built-ins.
//! Shows how to implement markdown file scanning in JavaScript.

use rquickjs::{Context, Runtime, Function};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Hybrid JS Business Logic Demo ===\n");
    
    // Create JavaScript runtime
    let rt = Runtime::new()?;
    let ctx = Context::full(&rt)?;
    
    ctx.with(|ctx| -> Result<(), Box<dyn std::error::Error>> {
        // Set up built-in functions for JS
        setup_builtins(&ctx)?;
        
        // Business logic in JavaScript
        let business_logic = r#"
            function scanForMarkdownCommands() {
                console_log("Starting markdown scan...");
                
                // This would scan a directory for .md files
                const files = scan_directory("~/documents");
                console_log("Found files: " + files.length);
                
                let commands = [];
                for (let i = 0; i < files.length; i++) {
                    const file = files[i];
                    if (file.endsWith('.md')) {
                        const content = read_file(file);
                        const cmd = extractCommandFromMarkdown(file, content);
                        if (cmd) {
                            commands.push(cmd);
                        }
                    }
                }
                
                console_log("Generated commands: " + commands.length);
                return commands;
            }
            
            function extractCommandFromMarkdown(filepath, content) {
                // Business logic: extract command from markdown frontmatter
                if (content.startsWith('---')) {
                    const lines = content.split('\n');
                    let command = null;
                    let action = "doc";
                    
                    for (let i = 1; i < lines.length; i++) {
                        const line = lines[i];
                        if (line === '---') break;
                        if (line.startsWith('command:')) {
                            command = line.substring(8).trim();
                        }
                        if (line.startsWith('action:')) {
                            action = line.substring(7).trim();
                        }
                    }
                    
                    if (command) {
                        console_log("Found command: " + command);
                        return {
                            command: command,
                            action: action,
                            arg: filepath,
                            patch: "docs"
                        };
                    }
                }
                return null;
            }
            
            // Execute the business logic
            const commands = scanForMarkdownCommands();
            console_log("Scan complete!");
        "#;
        
        // Execute JavaScript business logic
        ctx.eval::<(), _>(business_logic)?;
        
        Ok(())
    })?;
    
    println!("\n=== Demo completed ===");
    Ok(())
}

fn setup_builtins(ctx: &rquickjs::Ctx<'_>) -> Result<(), Box<dyn std::error::Error>> {
    // Console logging
    ctx.globals().set("console_log", Function::new(ctx.clone(), |msg: String| {
        println!("[JS] {}", msg);
    })?)?;
    
    // Mock file system operations
    ctx.globals().set("scan_directory", Function::new(ctx.clone(), |dir: String| {
        println!("[RUST] Scanning directory: {}", dir);
        // In real implementation, this would use std::fs::read_dir
        vec![
            "/path/to/doc1.md".to_string(),
            "/path/to/doc2.md".to_string(),
            "/path/to/README.md".to_string(),
        ]
    })?)?;
    
    ctx.globals().set("read_file", Function::new(ctx.clone(), |path: String| {
        println!("[RUST] Reading file: {}", path);
        // Mock markdown content with frontmatter
        if path.contains("doc1.md") {
            r#"---
command: my_document
action: doc
---
# My Document
This is content."#.to_string()
        } else {
            "# Regular markdown without frontmatter".to_string()
        }
    })?)?;
    
    Ok(())
}