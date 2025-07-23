use hookanchor::{Command, create_patches_hashmap};

fn main() {
    // Create test commands with patches
    let commands = vec![
        Command {
            patch: "Web".to_string(),
            command: "google".to_string(),
            action: "url".to_string(),
            arg: "https://google.com".to_string(),
            flags: String::new(),
            full_line: "Web! google : url; https://google.com".to_string(),
        },
        Command {
            patch: "Web".to_string(),
            command: "web".to_string(),  // This should be linked to the Web patch
            action: "chrome".to_string(),
            arg: "https://example.com".to_string(),
            flags: String::new(),
            full_line: "Web! web : chrome; https://example.com".to_string(),
        },
        Command {
            patch: "Dev".to_string(),
            command: "code".to_string(),
            action: "app".to_string(),
            arg: "Visual Studio Code".to_string(),
            flags: String::new(),
            full_line: "Dev! code : app; Visual Studio Code".to_string(),
        },
        Command {
            patch: "Dev".to_string(),
            command: "dev".to_string(),  // This should be linked to the Dev patch
            action: "folder".to_string(),
            arg: "~/Projects".to_string(),
            flags: String::new(),
            full_line: "Dev! dev : folder; ~/Projects".to_string(),
        },
        Command {
            patch: String::new(),  // No patch
            command: "terminal".to_string(),
            action: "app".to_string(),
            arg: "Terminal".to_string(),
            flags: String::new(),
            full_line: "terminal : app; Terminal".to_string(),
        },
    ];

    println!("Testing Patch Dispatcher Functionality");
    println!("======================================");
    
    // Create patches hashmap
    let patches = create_patches_hashmap(&commands);
    
    println!("\nCreated {} patches:", patches.len());
    
    for (patch_name, patch) in &patches {
        println!("\nPatch: '{}'", patch_name);
        if let Some(linked_cmd) = &patch.linked_command {
            println!("  → Linked to command: '{}' (action: {}, arg: {})", 
                linked_cmd.command, linked_cmd.action, linked_cmd.arg);
        } else {
            println!("  → No matching command found");
        }
    }
    
    // Test specific lookups
    println!("\n\nTesting specific patch lookups:");
    
    if let Some(web_patch) = patches.get("web") {
        println!("✓ Found 'web' patch");
        if let Some(cmd) = &web_patch.linked_command {
            println!("  Command: {} → {} {}", cmd.command, cmd.action, cmd.arg);
        }
    } else {
        println!("✗ 'web' patch not found");
    }
    
    if let Some(dev_patch) = patches.get("dev") {
        println!("✓ Found 'dev' patch");
        if let Some(cmd) = &dev_patch.linked_command {
            println!("  Command: {} → {} {}", cmd.command, cmd.action, cmd.arg);
        }
    } else {
        println!("✗ 'dev' patch not found");
    }
}