use hookanchor::{Command, get_submenu_display_positions, get_submenu_prefix};

fn main() {
    println!("Testing new submenu logic (exact anchor match only):");
    
    // Create test commands with different structures
    let commands = vec![
        Command {
            group: "".to_string(),
            command: "FIN Page".to_string(),
            action: "url".to_string(),
            arg: "https://fin.example.com".to_string(),
            full_line: "FIN Page : url https://fin.example.com".to_string(),
        },
        Command {
            group: "".to_string(),
            command: "FIN Note".to_string(),
            action: "url".to_string(),
            arg: "https://note.example.com".to_string(),
            full_line: "FIN Note : url https://note.example.com".to_string(),
        },
        Command {
            group: "".to_string(),
            command: "FIN Todo".to_string(),
            action: "url".to_string(),
            arg: "https://todo.example.com".to_string(),
            full_line: "FIN Todo : url https://todo.example.com".to_string(),
        },
        Command {
            group: "".to_string(),
            command: "Finance Report".to_string(),
            action: "file".to_string(),
            arg: "/docs/finance.pdf".to_string(),
            full_line: "Finance Report : file /docs/finance.pdf".to_string(),
        }
    ];
    
    println!("\nTest 1: Search 'FIN' (exact anchor match) - should show submenu");
    let positions = get_submenu_display_positions(&commands, "FIN");
    let prefix = get_submenu_prefix(&commands, "FIN");
    println!("Submenu positions: {:?}", positions);
    println!("Submenu prefix: {:?}", prefix);
    println!("Has submenu: {}", !positions.is_empty());
    
    println!("\nTest 2: Search 'fin' (case insensitive exact anchor match) - should show submenu");
    let positions = get_submenu_display_positions(&commands, "fin");
    let prefix = get_submenu_prefix(&commands, "fin");
    println!("Submenu positions: {:?}", positions);
    println!("Submenu prefix: {:?}", prefix);
    println!("Has submenu: {}", !positions.is_empty());
    
    println!("\nTest 3: Search 'Finance' (exact anchor match) - should show submenu");
    let positions = get_submenu_display_positions(&commands, "Finance");
    let prefix = get_submenu_prefix(&commands, "Finance");
    println!("Submenu positions: {:?}", positions);
    println!("Submenu prefix: {:?}", prefix);
    println!("Has submenu: {}", !positions.is_empty());
    
    println!("\nTest 4: Search 'FI' (partial match, not exact anchor) - should NOT show submenu");
    let positions = get_submenu_display_positions(&commands, "FI");
    let prefix = get_submenu_prefix(&commands, "FI");
    println!("Submenu positions: {:?}", positions);
    println!("Submenu prefix: {:?}", prefix);
    println!("Has submenu: {}", !positions.is_empty());
    
    println!("\nTest 5: Search 'FINE' (no exact anchor match) - should NOT show submenu");
    let positions = get_submenu_display_positions(&commands, "FINE");
    let prefix = get_submenu_prefix(&commands, "FINE");
    println!("Submenu positions: {:?}", positions);
    println!("Submenu prefix: {:?}", prefix);
    println!("Has submenu: {}", !positions.is_empty());
    
    println!("\nTest 6: Search 'Page' (suffix, not anchor) - should NOT show submenu");
    let positions = get_submenu_display_positions(&commands, "Page");
    let prefix = get_submenu_prefix(&commands, "Page");
    println!("Submenu positions: {:?}", positions);
    println!("Submenu prefix: {:?}", prefix);
    println!("Has submenu: {}", !positions.is_empty());
}