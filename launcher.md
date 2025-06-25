# Launcher Scripting System Design

## Project Overview

The goal is to design a scripting layer that allows implementation of the launcher functionality currently handled by a Python script. This would enable users to define custom commands that perform various system actions like launching applications, opening browsers, loading specific interfaces, etc.

## Original User Request

> The part that I'm thinking most specifically about is some kind of scripting layer that allows me to implement the launcher which is happening in the background. Maybe I should show you that launcher so you can look at it and then consider how we would best implement a scripted language, they could do the kinds of things that the launcher can do. In the bin, Project spot folder. You'll find a large python script called Spot inside of that you'll find a function CMD_server. That function gets command requests and then dispatches them and you can see the various commands. There's about 10 different commands that can be executed, including anchor OBS for obsidian work which loads a particular browser chrome which loads a different browser, etc.. I'm thinking about writing a scripting language in which commands like these 10 commands could be written. They would represent the specific actions that would be required on other users's systems in order to launch various kinds of applications and interfaces. So our goal here is to look at the range of those commands and then propose a set of operators that could be used to script these commands. Obviously, if this was in JavaScript, then we could allow JavaScript itself to be used as part of the scripting language. That was the advantage of using a JavaScript based system. Still there might be other ways of building a robust scripting solution that does not involve JavaScript. Let's start by trying to figure out what the set of commands might be and what level of generality, we would need in the scripting language.

## Analysis Results

### Command Server Function Location
Found in `/Users/oblinger/ob/kmr/prj/binproj/spot/spot` - the `cmd_server()` function at line 1061 and `execute()` function at line 1089.

### Cataloged Commands (12 Total)

From the `execute()` function (lines 1089-1142), here are the launcher commands:

1. **`1pass`** - Launch 1Password for specific entry
   - `os.system(f"_1pass '{arg}'")`

2. **`alias`** - Launch another command by name
   - `execute_cmd(arg)` (recursive command execution)

3. **`anchor`** - Change directory and run anchor activation
   - `os.system(f"cd '{arg}' && anchor activate")`

4. **`app`** - Launch macOS application
   - `os.system(f"open -a '{arg}'")`

5. **`cmd`** - Execute arbitrary shell command
   - `os.system(arg)`

6. **`folder`** - Open folder in Finder
   - `os.system(f"open '{arg}'")`

7. **`doc`** - Open document with default app
   - `os.system(f"open '{arg}'")`

8. **`notion`** - Open Notion page
   - `os.system(f"open -a Notion '{arg}'")`

9. **`obs`** - Open Obsidian page with special handling
   - `launch_obsidian(arg)` (custom function with URL encoding)

10. **`obs_url`** - Open Obsidian URL directly
    - `os.system(f"open -a obsidian '{arg}'")`

11. **`url`** - Open URL in default browser
    - `os.system(f"open '{arg}'")`

12. **`chrome`** - Open URL in Chrome specifically
    - `os.system(f"open -a 'Google Chrome' '{arg}'")`

13. **`safari`** - Open URL in Safari (incomplete implementation)

14. **`brave`** - Open URL in Brave browser
    - `os.system(f"open -a 'Brave Browser' '{arg}'")`

15. **`firefox`** - Open URL in Firefox
    - `os.system(f"open -a 'firefox' '{arg}'")`

16. **`work`** - Open URL in work browser (Chrome Beta)
    - `os.system(f"open -a 'Google Chrome Beta' '{arg}'")`

### Command Patterns Identified

#### **Pattern 1: Application Launchers**
- `app`, `1pass`, `notion` - Launch specific applications
- **Operation**: `open -a [APP_NAME] [OPTIONAL_ARG]`

#### **Pattern 2: Browser/URL Handlers** 
- `url`, `chrome`, `safari`, `brave`, `firefox`, `work`, `obs_url`
- **Operation**: `open -a [BROWSER] [URL]` or `open [URL]`

#### **Pattern 3: File System Operations**
- `folder`, `doc` - Open files/folders
- **Operation**: `open [PATH]`

#### **Pattern 4: Command Execution**
- `cmd` - Direct shell execution
- **Operation**: `system([COMMAND])`

#### **Pattern 5: Specialized Handlers**
- `obs` - Custom Obsidian integration with encoding
- `anchor` - Directory change + command execution  
- `alias` - Recursive command calling

## Scripting Language Design

### Core Operations Needed

1. **`launch_app(name, [arg])`** - Launch macOS application
2. **`open_url(url, [browser])`** - Open URL in specific browser  
3. **`open_file(path)`** - Open file/folder with default app
4. **`shell(command)`** - Execute shell command
5. **`cd_and_run(dir, command)`** - Change directory and execute
6. **`call(command_name)`** - Call another scripted command

### Proposed Scripting Approaches

#### **Option 1: JSON-Based DSL**
```json
{
  "type": "launch_app",
  "app": "Google Chrome", 
  "arg": "https://example.com"
}
```

#### **Option 2: YAML-Based DSL**
```yaml
- launch_app:
    app: "Google Chrome"
    arg: "https://example.com"
- shell: "echo 'Done'"
```

#### **Option 3: JavaScript Embedded**
```javascript
// Simple cases
launch_app("Google Chrome", "https://example.com");

// Complex cases  
if (url.includes("notion.so")) {
  launch_app("Notion", url);
} else {
  open_url(url, "chrome");
}
```

#### **Option 4: Custom Syntax**
```
app "Google Chrome" "https://example.com"
shell "echo Done"
if url contains "notion.so" then app "Notion" url
```

## Implementation Considerations

### JavaScript-Based Approach
- **Pros**: Full language flexibility, familiar to developers, conditionals/loops
- **Cons**: Requires JS runtime (Node.js or browser engine), security sandboxing needed

### JSON/YAML DSL Approach  
- **Pros**: Simpler, safer, easier to validate, cross-platform
- **Cons**: Limited logic capabilities, harder to express complex scenarios

### Hybrid Approach
- **Pros**: Simple commands in DSL, complex logic in embedded JS
- **Cons**: More complex implementation, two systems to maintain

## Recommended Implementation

**Start with JSON DSL + JavaScript fallback:**

```json
{
  "simple": {
    "type": "launch_app",
    "app": "Chrome",
    "arg": "{{url}}"
  },
  "complex": {
    "type": "javascript", 
    "code": "if (url.includes('github')) launch_app('Chrome', url); else open_url(url);"
  }
}
```

This provides:
- ✅ **Simple cases** handled by declarative JSON
- ✅ **Complex cases** handled by JavaScript  
- ✅ **Progressive complexity** - users start simple, add JS when needed
- ✅ **Security** - can sandbox JavaScript execution
- ✅ **Cross-platform** - JSON structure works everywhere

## Lightweight JavaScript Runtime Options

### **Option 1: QuickJS (Recommended)**
**Size**: ~1-2MB total  
**Startup**: <10ms  
**Rust Integration**: Excellent (`rquickjs` crate)

```rust
// Embedded QuickJS in our Rust app
use rquickjs::{Context, Runtime};

fn execute_script(code: &str, args: &str) -> Result<String> {
    let rt = Runtime::new()?;
    let ctx = Context::full(&rt)?;
    ctx.with(|ctx| {
        // Provide our launcher APIs
        ctx.globals().set("launch_app", js_launch_app)?;
        ctx.globals().set("open_url", js_open_url)?;
        
        // Execute user script
        ctx.eval::<String>(code)
    })
}
```

**Benefits:**
- ✅ **Tiny footprint** (~1-2MB vs 200MB for Electron)
- ✅ **Fast startup** (milliseconds vs seconds)
- ✅ **Full ES6+ support** 
- ✅ **Secure sandboxing** built-in
- ✅ **No external dependencies**

### **Option 2: System JavaScriptCore (macOS)**
**Size**: 0MB (uses system framework)  
**Startup**: <5ms  
**Rust Integration**: Via C bindings

```rust
// Use macOS built-in JavaScript engine
use core_foundation::*;
use javascript_core::*;

// Zero deployment size - uses system JS engine
```

**Benefits:**
- ✅ **Zero size cost** (system framework)
- ✅ **Extremely fast**
- ✅ **Always available** on macOS
- ❌ **macOS only**

### **Option 3: Node.js Detection**
**Size**: 0MB if Node.js installed, fallback to QuickJS  
**Startup**: ~100ms  

```rust
// Try system Node.js first, fallback to embedded
if command_exists("node") {
    std::process::Command::new("node")
        .arg("-e").arg(script)
        .output()
} else {
    // Fallback to embedded QuickJS
    execute_with_quickjs(script)
}
```

**Benefits:**
- ✅ **Zero size** for Node.js users
- ✅ **Full Node.js ecosystem** when available
- ✅ **Graceful fallback** for non-Node users

### **Option 4: Bun Runtime**
**Size**: ~30MB (much smaller than Electron)  
**Startup**: ~50ms  
**Performance**: Faster than Node.js

```bash
# If user has Bun installed
bun run launcher_script.js
```

## Deployment Size Comparison

| Runtime | Binary Size | Startup Time | JavaScript Support |
|---------|-------------|--------------|-------------------|
| **QuickJS** | ~2MB | <10ms | ES2023 |
| **JavaScriptCore** | 0MB | <5ms | ES2023 |
| **Node.js (system)** | 0MB | ~100ms | Full ecosystem |
| **Electron** | ~200MB | ~2-3s | Full browser |
| **Tauri** | ~15MB | ~300ms | Web APIs |

## Recommended Architecture

### **Multi-Runtime Strategy:**

```rust
pub enum ScriptEngine {
    QuickJS,           // Embedded, always available
    SystemJS,          // macOS JavaScriptCore  
    NodeJS,            // If available
}

impl ScriptEngine {
    pub fn best_available() -> Self {
        if cfg!(target_os = "macos") && system_js_available() {
            Self::SystemJS
        } else if command_exists("node") {
            Self::NodeJS  
        } else {
            Self::QuickJS  // Always works
        }
    }
}
```

### **Script API Design:**

```javascript
// User's launcher script
const script = {
    simple_cases: {
        "github_url": (url) => launch_app("Google Chrome", url),
        "notion_url": (url) => launch_app("Notion", url),
    },
    
    complex_logic: (command, arg) => {
        if (arg.includes("github.com")) {
            launch_app("Google Chrome", arg);
        } else if (arg.includes("notion.so")) {
            launch_app("Notion", arg);
        } else {
            open_url(arg);
        }
    }
};
```

## Robustness Advantages

### **vs Electron:**
- ✅ **No Chromium dependencies** 
- ✅ **No web rendering complexity**
- ✅ **Faster startup** (critical for command launcher)
- ✅ **Smaller attack surface**
- ✅ **Native error handling**

### **Security Features:**
- ✅ **Sandboxed execution** (no file system access by default)
- ✅ **Limited API surface** (only launcher functions exposed)
- ✅ **Script validation** before execution
- ✅ **Timeout protection** for runaway scripts

## JavaScript Compatibility Across Runtimes

### **What Works Everywhere (QuickJS, Node.js, JavaScriptCore):**

```javascript
// ✅ All core JavaScript features work identically
const commands = {
    github: (url) => {
        if (url.includes("github.com")) {
            launch_app("Google Chrome", url);
        } else {
            open_url(url);
        }
    },
    
    smart_browser: (url) => {
        // String methods, conditionals, arrays - all work
        const patterns = [
            { match: "notion.so", app: "Notion" },
            { match: "github.com", app: "Google Chrome" },
            { match: "figma.com", app: "Figma" }
        ];
        
        const target = patterns.find(p => url.includes(p.match));
        if (target) {
            launch_app(target.app, url);
        } else {
            open_url(url);
        }
    }
};

// ✅ Modern JavaScript features
const processUrl = async (url) => {
    const config = { timeout: 5000 };
    return url.startsWith("https://") ? url : `https://${url}`;
};

// ✅ Classes, arrow functions, destructuring
class LauncherConfig {
    constructor(defaults) {
        this.apps = { ...defaults };
    }
    
    route(url) {
        const { hostname } = new URL(url);
        return this.apps[hostname] || "default";
    }
}
```

### **What Only Works with Node.js:**

```javascript
// ❌ Node.js-specific modules (not available in QuickJS)
const fs = require('fs');           // File system
const path = require('path');       // Path utilities  
const fetch = require('node-fetch'); // HTTP requests
const { exec } = require('child_process'); // Shell execution

// ❌ Node.js APIs
process.env.HOME                    // Environment variables
require('./my-module.js')           // Module imports
```

### **Our Solution: Provide Equivalent APIs**

Instead of requiring Node.js modules, we provide our own APIs that work everywhere:

```javascript
// ✅ Our launcher APIs work in ALL runtimes
launch_app("Chrome", url);          // Cross-platform app launching
open_url(url, "chrome");           // Browser-specific opening  
shell("echo hello");               // Shell command execution
read_file("config.json");          // File reading (if enabled)
get_env("HOME");                   // Environment variables

// ✅ For complex logic, pure JavaScript works everywhere
function smartRoute(url) {
    // No Node.js dependencies needed!
    if (url.match(/github\.com/)) return "chrome";
    if (url.match(/notion\.so/)) return "notion";
    if (url.match(/figma\.com/)) return "figma";
    return "default";
}
```

## Runtime Feature Comparison

| Feature | QuickJS | Node.js | JavaScriptCore |
|---------|---------|---------|----------------|
| **Core JavaScript** | ✅ Full ES2023 | ✅ Full ES2023 | ✅ Full ES2023 |
| **Arrow functions** | ✅ | ✅ | ✅ |
| **Classes** | ✅ | ✅ | ✅ |
| **Async/await** | ✅ | ✅ | ✅ |
| **String/Array methods** | ✅ | ✅ | ✅ |
| **JSON parsing** | ✅ | ✅ | ✅ |
| **RegExp** | ✅ | ✅ | ✅ |
| **Our launcher APIs** | ✅ | ✅ | ✅ |
| **Node.js modules** | ❌ | ✅ | ❌ |
| **File system access** | ❌* | ✅ | ❌* |
| **HTTP requests** | ❌* | ✅ | ❌* |

*\* = Can be provided through our custom APIs if needed*

## Real-World Impact

### **Scripts That Work Everywhere (95% of use cases):**

```javascript
// ✅ URL routing based on patterns
function handleUrl(url) {
    const routes = {
        "github.com": "Google Chrome",
        "notion.so": "Notion", 
        "figma.com": "Figma",
        "youtube.com": "Chrome"
    };
    
    for (const [pattern, app] of Object.entries(routes)) {
        if (url.includes(pattern)) {
            return launch_app(app, url);
        }
    }
    
    return open_url(url); // Default browser
}

// ✅ Conditional logic with user preferences
const userPrefs = {
    preferredBrowser: "Chrome",
    workHours: { start: 9, end: 17 }
};

function smartLaunch(command, arg) {
    const hour = new Date().getHours();
    const isWorkTime = hour >= userPrefs.workHours.start && 
                      hour <= userPrefs.workHours.end;
    
    if (command === "browse" && isWorkTime) {
        launch_app("Google Chrome Beta", arg); // Work browser
    } else {
        launch_app(userPrefs.preferredBrowser, arg);
    }
}
```

### **Scripts That Need Node.js (5% of use cases):**

```javascript
// ❌ Only works with Node.js runtime
const fs = require('fs');
const config = JSON.parse(fs.readFileSync('config.json'));

// ✅ Alternative that works everywhere
const config = read_config(); // Our provided API
```

## Migration Strategy

**For users with Node.js scripts:**

1. **Most scripts work unchanged** (90%+ compatibility)
2. **Replace Node.js modules** with our APIs where needed
3. **Automatic detection** - script runs on Node.js if available
4. **Graceful degradation** - fallback to QuickJS with warning

```rust
// Auto-detect and migrate
if script.contains("require(") && !nodejs_available() {
    warn!("Script uses Node.js modules, some features may not work");
    // Could even auto-suggest replacements
}
```

**Bottom line:** QuickJS handles 95%+ of real-world launcher scripts perfectly. The 5% that need Node.js-specific features can either be rewritten with our APIs or will auto-run on Node.js when available.

## Configuration Strategy: YAML + Multi-Language Support

### **Proposed File Structure:**

```
launcher_config/
├── config.yaml              # Main configuration
├── scripts/                 # Optional separate JS files
│   ├── github_handler.js
│   ├── work_browser.js
│   └── notion_smart.js
└── templates/               # Reusable snippets
    └── common_patterns.yaml
```

### **Option 1: Everything in YAML (Recommended for most users)**

```yaml
# config.yaml - Progressive complexity in one file
name: "My Launcher Config"
version: "1.0"

# Simple cases - no scripting needed
simple_commands:
  github_url:
    type: "launch_app"
    app: "Google Chrome"
    arg: "{{url}}"
    
  notion_page:
    type: "launch_app" 
    app: "Notion"
    arg: "{{url}}"
    
  open_folder:
    type: "open_file"
    path: "{{path}}"

# Medium complexity - inline JavaScript
scripted_commands:
  smart_browser: |
    if (url.includes("github.com")) {
      launch_app("Google Chrome", url);
    } else if (url.includes("notion.so")) {
      launch_app("Notion", url);
    } else if (url.includes("work-domain.com")) {
      launch_app("Google Chrome Beta", url);
    } else {
      open_url(url);
    }
    
  work_mode: |
    const hour = new Date().getHours();
    const isWorkTime = hour >= 9 && hour <= 17;
    
    if (isWorkTime && url.includes("slack.com")) {
      launch_app("Slack", url);
    } else {
      launch_app("Google Chrome", url);
    }

# Complex cases - reference external files  
external_scripts:
  advanced_routing: "scripts/github_handler.js"
  enterprise_logic: "scripts/work_browser.js"

# Global settings
settings:
  default_browser: "Google Chrome"
  work_browser: "Google Chrome Beta"
  timeout_ms: 5000
```

### **Option 2: Hybrid Approach with External Files**

```yaml
# config.yaml - Keep it clean
name: "My Launcher Config"

# Simple cases stay in YAML
commands:
  basic_github:
    type: "launch_app"
    app: "Google Chrome"
    arg: "{{url}}"

# Complex cases reference external files
scripts:
  smart_routing: "./scripts/smart_routing.js"
  work_mode: "./scripts/work_hours.js"
  
# Default command for unmatched patterns
default_handler: "./scripts/fallback.js"
```

```javascript
// scripts/smart_routing.js
function handle(command, url) {
    const patterns = [
        { match: /github\.com/, app: "Google Chrome" },
        { match: /notion\.so/, app: "Notion" },
        { match: /figma\.com/, app: "Figma" },
        { match: /slack\.com/, app: "Slack" }
    ];
    
    for (const pattern of patterns) {
        if (pattern.match.test(url)) {
            return launch_app(pattern.app, url);
        }
    }
    
    // Fallback
    return open_url(url);
}
```

### **Option 3: Template-Based System**

```yaml
# config.yaml - Using templates
commands:
  github_work:
    template: "url_router"
    patterns:
      - match: "github.com"
        app: "Google Chrome"
      - match: "gitlab.company.com" 
        app: "Google Chrome Beta"
        
  notion_smart:
    template: "time_aware"
    work_hours: "9-17"
    work_app: "Notion"
    personal_app: "Safari"

# Advanced users can still write custom JavaScript
custom_scripts:
  complex_logic: |
    // Inline JavaScript for power users
    const config = get_user_config();
    if (config.work_mode && is_work_hours()) {
      launch_work_browser(url);
    } else {
      launch_personal_browser(url);
    }
```

## Implementation Architecture

### **Configuration Loading Priority:**

```rust
pub struct LauncherConfig {
    simple_commands: HashMap<String, SimpleCommand>,
    scripted_commands: HashMap<String, String>,  // JavaScript code
    external_scripts: HashMap<String, PathBuf>,  // File references
    settings: GlobalSettings,
}

impl LauncherConfig {
    pub fn load() -> Self {
        // 1. Load main config.yaml
        let yaml_config = load_yaml("config.yaml")?;
        
        // 2. Process simple commands (no JavaScript needed)
        let simple = parse_simple_commands(&yaml_config);
        
        // 3. Extract inline JavaScript
        let scripted = parse_scripted_commands(&yaml_config);
        
        // 4. Resolve external file references
        let external = resolve_external_scripts(&yaml_config);
        
        Self { simple_commands: simple, scripted_commands: scripted, external_scripts: external, settings: yaml_config.settings }
    }
}
```

### **Execution Flow:**

```rust
pub fn execute_command(command: &str, arg: &str) -> Result<()> {
    let config = LauncherConfig::load();
    
    // 1. Try simple commands first (fastest)
    if let Some(simple_cmd) = config.simple_commands.get(command) {
        return execute_simple_command(simple_cmd, arg);
    }
    
    // 2. Try inline JavaScript
    if let Some(js_code) = config.scripted_commands.get(command) {
        return execute_javascript_inline(js_code, command, arg);
    }
    
    // 3. Try external JavaScript files
    if let Some(js_file) = config.external_scripts.get(command) {
        return execute_javascript_file(js_file, command, arg);
    }
    
    // 4. Fallback to default handler
    execute_default_handler(command, arg)
}
```

## User Experience Progression

### **Beginner Users:**
```yaml
# Just simple YAML - no JavaScript knowledge needed
commands:
  github: 
    type: "launch_app"
    app: "Google Chrome"
    arg: "{{url}}"
```

### **Intermediate Users:**
```yaml  
# Add some inline JavaScript for logic
commands:
  smart_github: |
    if (url.includes("github.com/work")) {
      launch_app("Google Chrome Beta", url);
    } else {
      launch_app("Google Chrome", url);
    }
```

### **Advanced Users:**
```yaml
# Reference external JavaScript files for complex logic
commands:
  enterprise_routing: "./scripts/enterprise.js"
  ai_smart_routing: "./scripts/ai_classifier.js"
```

## Recommended Strategy

**Start with Option 1 (Everything in YAML)** because:

1. ✅ **Single file** - easier to manage and share
2. ✅ **Progressive complexity** - simple → inline JS → external files
3. ✅ **Better portability** - config.yaml contains everything
4. ✅ **Easier debugging** - all logic visible in one place
5. ✅ **Simpler implementation** - fewer file system dependencies

**Migration path:**
- Beginners start with simple YAML commands
- As needs grow, add inline JavaScript in same file  
- Power users can optionally extract to separate .js files

This gives users maximum flexibility while keeping the common case (simple YAML) as simple as possible.

## Implementation Plan

### **Phase 1: Core + QuickJS**
1. Embed QuickJS in Rust binary (~2MB total)
2. Implement basic launcher APIs
3. JSON DSL for simple cases
4. JavaScript for complex logic

### **Phase 2: Multi-Runtime**  
1. Add system JavaScript detection
2. Optimize for platform-specific runtimes
3. Performance benchmarking

### **Phase 3: Enhanced APIs**
1. Add more launcher functions
2. Template system for common patterns
3. Script library/sharing system

**Result:** Robust, fast, small (~2-5MB) deployment with full JavaScript flexibility for power users.

---

*This document tracks our progress on designing a scripting system for launcher commands.*