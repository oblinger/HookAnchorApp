use std::time::Instant;
use hookanchor::{load_commands, load_config};

fn main() {
    println!("Testing startup timing...");
    
    let start = Instant::now();
    let _commands = load_commands();
    let commands_time = start.elapsed();
    println!("load_commands(): {:?}", commands_time);
    
    let start = Instant::now();
    let _config = load_config();
    let config_time = start.elapsed();
    println!("load_config(): {:?}", config_time);
    
    println!("Total: {:?}", commands_time + config_time);
}