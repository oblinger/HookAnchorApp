//! Comprehensive JavaScript API Test
//!
//! Demonstrates many of the available JavaScript built-in functions
//! to show the full power of the user-customizable system.

use anchor_selector::js_runtime::execute_business_logic;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Comprehensive JavaScript API Test ===\n");
    
    let comprehensive_script = r#"
        log("=== JavaScript API Comprehensive Test ===");
        
        // 1. PATH UTILITIES
        log("\n1. Path Utilities:");
        const home = expandHome("~");
        const testPath = joinPath(home, "Documents");
        const fileName = basename("/Users/test/example.txt");
        const dirName = dirname("/Users/test/example.txt");
        const extension = getExtension("document.pdf");
        
        log("  Home: " + home);
        log("  Joined path: " + testPath);
        log("  Basename: " + fileName);
        log("  Dirname: " + dirName);
        log("  Extension: " + extension);
        
        // 2. FILE OPERATIONS
        log("\n2. File Operations:");
        const tmpExists = fileExists("/tmp");
        const tmpIsDir = isDirectory("/tmp");
        
        log("  /tmp exists: " + tmpExists);
        log("  /tmp is directory: " + tmpIsDir);
        
        if (tmpExists && tmpIsDir) {
            const files = listFiles("/tmp", "");
            log("  Files in /tmp: " + files.length + " items");
        }
        
        // 3. TEXT PROCESSING
        log("\n3. Text Processing:");
        const githubTest = testRegex("https://github.com/user/repo", "github\\.com");
        const notionTest = testRegex("https://notion.so/page", "notion\\.so");
        const emailTest = testRegex("user@example.com", "\\w+@\\w+\\.\\w+");
        
        log("  GitHub URL regex test: " + githubTest);
        log("  Notion URL regex test: " + notionTest);
        log("  Email regex test: " + emailTest);
        
        // 4. SHELL OPERATIONS (safe commands only)
        log("\n4. Shell Operations:");
        const echoResult = shell("echo 'Hello from shell'");
        const dateResult = shell("date");
        const pwdResult = shell("pwd");
        
        log("  Echo result: " + echoResult);
        log("  Date result: " + dateResult.trim());
        log("  Current directory: " + pwdResult.trim());
        
        // 5. TMUX OPERATIONS (safe checks only)
        log("\n5. Tmux Operations:");
        const tmuxAvailable = shell("which tmux").includes("tmux");
        log("  Tmux available: " + tmuxAvailable);
        
        if (tmuxAvailable) {
            const hasSession = has_tmux_session("nonexistent-session");
            log("  Has test session: " + hasSession);
        }
        
        // 6. CONDITIONAL LOGIC EXAMPLE
        log("\n6. Conditional Logic Example:");
        const testUrl = "https://github.com/user/project";
        
        if (testRegex(testUrl, "github\\.com")) {
            log("  Would open GitHub URL in development browser");
        } else if (testRegex(testUrl, "notion\\.so")) {
            log("  Would open Notion page");
        } else {
            log("  Would open in default browser");
        }
        
        // 7. CONFIGURATION EXAMPLE
        log("\n7. Configuration Example:");
        const CONFIG = {
            defaultBrowser: "Google Chrome",
            workBrowser: "Google Chrome Beta",
            folderApp: "Finder",
            terminalApp: "iTerm2"
        };
        
        log("  Default browser: " + CONFIG.defaultBrowser);
        log("  Work browser: " + CONFIG.workBrowser);
        log("  Folder app: " + CONFIG.folderApp);
        
        // 8. ENVIRONMENT DETECTION
        log("\n8. Environment Detection:");
        const isLinux = shell("uname").includes("Linux");
        const isMacOS = shell("uname").includes("Darwin");
        const nodeAvailable = shell("which node").includes("node");
        const gitAvailable = shell("which git").includes("git");
        
        log("  Is Linux: " + isLinux);
        log("  Is macOS: " + isMacOS);
        log("  Node.js available: " + nodeAvailable);
        log("  Git available: " + gitAvailable);
        
        // 9. PROJECT TYPE DETECTION EXAMPLE
        log("\n9. Project Type Detection (simulated):");
        function detectProjectType(projectPath) {
            if (fileExists(joinPath(projectPath, "package.json"))) {
                return "Node.js";
            } else if (fileExists(joinPath(projectPath, "Cargo.toml"))) {
                return "Rust";
            } else if (fileExists(joinPath(projectPath, "pom.xml"))) {
                return "Java/Maven";
            } else if (fileExists(joinPath(projectPath, "requirements.txt"))) {
                return "Python";
            } else {
                return "Unknown";
            }
        }
        
        const currentProjectType = detectProjectType(".");
        log("  Current directory project type: " + currentProjectType);
        
        log("\n=== API Test Complete ===");
        log("All " + (9) + " categories of functions tested successfully!");
        
        "comprehensive-api-test-complete"
    "#;
    
    match execute_business_logic(comprehensive_script) {
        Ok(result) => {
            println!("\n✅ Comprehensive API test completed!");
            println!("Final result: {}", result);
            
            println!("\n=== Key Takeaways ===");
            println!("🔧 Rich built-in API with 20+ functions");
            println!("🏗️ Support for complex conditional logic");
            println!("🌐 Environment detection and adaptation");
            println!("📁 File system operations and path utilities");
            println!("🚀 Shell command execution");
            println!("🔍 Text processing with regex support");
            println!("⚙️ Development tool integration (tmux, editors)");
            
            println!("\n=== User Customization Potential ===");
            println!("✨ Users can create sophisticated, environment-aware actions");
            println!("✨ Support for conditional logic based on available tools");
            println!("✨ Project-type detection and automated setup");
            println!("✨ Integration with development workflows");
            println!("✨ Cross-platform compatibility checks");
        }
        Err(e) => {
            println!("❌ API test failed: {}", e);
        }
    }
    
    Ok(())
}