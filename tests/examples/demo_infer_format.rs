fn main() {
    println!("Demo of --infer command output format:");
    println!();
    
    // Show the format you requested: "command name: old patch -> new patch"
    println!("Example outputs:");
    println!("Test Command: (empty) -> Application");  // Adding new patch
    println!("Another Test: RR -> T");                 // Changing existing patch  
    println!("Third Test: CV -> SV");                  // Another change
    println!();
    println!("The format is:");
    println!("  <command_name>: <current_patch> -> <inferred_patch>");
    println!();
    println!("Where:");
    println!("  - (empty) means the command currently has no patch");
    println!("  - Only commands that would change are shown");
    println!("  - Changes include both adding new patches and changing existing ones");
}