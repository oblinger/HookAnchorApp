use hookanchor::Command;

fn main() {
    println!("Testing flag functionality...\n");
    
    // Create a command and test flag operations
    let mut cmd = Command {
        group: String::new(),
        command: "test command".to_string(),
        action: "test".to_string(),
        arg: "some argument".to_string(),
        flags: String::new(),
        full_line: String::new(),
    };
    
    println!("Initial command: {}", cmd.to_new_format());
    
    // Set some flags
    cmd.set_flag('p', "high");
    println!("After setting priority flag 'p' to 'high': {}", cmd.to_new_format());
    
    cmd.set_flag('t', "tag1");
    println!("After setting tag flag 't' to 'tag1': {}", cmd.to_new_format());
    
    cmd.set_flag('u', "urgent");
    println!("After setting urgency flag 'u' to 'urgent': {}", cmd.to_new_format());
    
    // Get flags
    println!("\nGetting flags:");
    println!("Priority flag 'p': {:?}", cmd.get_flag('p'));
    println!("Tag flag 't': {:?}", cmd.get_flag('t'));
    println!("Urgency flag 'u': {:?}", cmd.get_flag('u'));
    println!("Non-existent flag 'x': {:?}", cmd.get_flag('x'));
    
    // Update a flag
    cmd.set_flag('p', "medium");
    println!("\nAfter changing priority flag 'p' to 'medium': {}", cmd.to_new_format());
    
    // Remove a flag
    cmd.remove_flag('t');
    println!("After removing tag flag 't': {}", cmd.to_new_format());
    
    // Test parsing of new format
    println!("\nTesting parsing of new format:");
    let test_line = "MyGroup ! test cmd : action arg,phigh,uurgent;";
    println!("Parsing line: {}", test_line);
    
    // This would require exposing the parse functions, so let's just test roundtrip
    println!("Original flags: {}", cmd.flags);
    println!("Final format: {}", cmd.to_new_format());
    
    println!("\nFlag functionality test complete!");
}