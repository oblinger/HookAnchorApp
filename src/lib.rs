use std::env;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, PartialEq)]
pub struct Command {
    pub group: String,
    pub command: String,
    pub action: String,
    pub arg: String,
    pub full_line: String,
}

pub fn load_commands() -> Vec<Command> {
    let home = env::var("HOME").unwrap_or_else(|_| ".".to_string());
    let file_path = Path::new(&home).join("ob/data/spot_cmds/spot_cmds.txt");
    
    let contents = match fs::read_to_string(&file_path) {
        Ok(contents) => contents,
        Err(e) => {
            eprintln!("Error reading commands file {}: {}", file_path.display(), e);
            std::process::exit(1);
        }
    };
    
    contents
        .lines()
        .filter_map(|line| {
            if line.trim().is_empty() {
                return None;
            }
            
            // Parse format: GROUP ! COMMAND : ACTION ARG or COMMAND : ACTION ARG
            let (group, command, action_arg) = if line.contains('!') {
                let parts: Vec<&str> = line.splitn(2, '!').collect();
                if parts.len() != 2 {
                    return None;
                }
                let group = parts[0].trim().to_string();
                let rest = parts[1].trim();
                
                let cmd_parts: Vec<&str> = rest.splitn(2, ':').collect();
                if cmd_parts.len() != 2 {
                    return None;
                }
                (group, cmd_parts[0].trim().to_string(), cmd_parts[1].trim())
            } else {
                let cmd_parts: Vec<&str> = line.splitn(2, ':').collect();
                if cmd_parts.len() != 2 {
                    return None;
                }
                (String::new(), cmd_parts[0].trim().to_string(), cmd_parts[1].trim())
            };
            
            let mut action_parts = action_arg.split_whitespace();
            let action = action_parts.next().unwrap_or("").to_string();
            let arg = action_parts.collect::<Vec<&str>>().join(" ");
            
            Some(Command {
                group,
                command,
                action,
                arg,
                full_line: line.to_string(),
            })
        })
        .collect()
}

pub fn filter_commands(commands: &[Command], search_text: &str, debug: bool) -> Vec<Command> {
    let search_no_spaces = search_text.to_lowercase().replace(' ', "");
    
    if debug {
        println!("Debug: searching for '{}' (original: '{}')", search_no_spaces, search_text);
    }
    
    // Separate into 4 categories for better relevance
    let mut exact_matches: Vec<Command> = Vec::new();        // Exact command name match
    let mut command_start_matches: Vec<Command> = Vec::new(); // Matches at start of command
    let mut word_start_matches: Vec<Command> = Vec::new();   // Matches at start of any other word
    let mut prefix_matches: Vec<Command> = Vec::new();       // Partial prefix matches
    
    for cmd in commands {
        let cmd_lower = cmd.command.to_lowercase();
        
        // Debug specific command
        if debug && cmd_lower.contains("prime") && search_no_spaces == "p" {
            println!("Debug: checking '{}' against 'p'", cmd_lower);
        }
        
        let cmd_no_spaces = cmd_lower.replace(' ', "");
        
        // 1. Exact match (highest priority)
        if cmd_no_spaces == search_no_spaces {
            exact_matches.push(cmd.clone());
            if debug && cmd_lower.contains("prime") && search_no_spaces == "p" {
                println!("Debug: '{}' matched exactly", cmd_lower);
            }
        }
        // 2. Matches at start of command (high priority) 
        else if matches_at_word_boundary(&cmd_lower, &search_no_spaces, 0) {
            command_start_matches.push(cmd.clone());
            if debug && cmd_lower.contains("prime") && search_no_spaces == "p" {
                println!("Debug: '{}' matched at command start with spaces", cmd_lower);
            }
        } else {
            // 3. Check if it matches after any space (word boundary) - medium priority
            let mut found = false;
            for (i, ch) in cmd_lower.char_indices() {
                if ch == ' ' && i + 1 < cmd_lower.len() {
                    if matches_at_word_boundary(&cmd_lower, &search_no_spaces, i + 1) {
                        word_start_matches.push(cmd.clone());
                        if debug && cmd_lower.contains("prime") && search_no_spaces == "p" {
                            println!("Debug: '{}' matched after space at position {}", cmd_lower, i + 1);
                        }
                        found = true;
                        break;
                    }
                }
            }
            
            // 4. Partial prefix matches (lowest priority)
            if !found && cmd_lower.starts_with(&search_no_spaces) {
                prefix_matches.push(cmd.clone());
                if debug && cmd_lower.contains("prime") && search_no_spaces == "p" {
                    println!("Debug: '{}' matched as prefix", cmd_lower);
                }
            }
        }
    }
    
    // Sort each category - for very short searches, prefer longer, more meaningful commands
    let prefer_longer = search_no_spaces.len() <= 2;
    
    if prefer_longer {
        // For short searches, prefer "reasonable" length (8-30 chars) over very short or very long
        let sort_by_reasonable_length = |cmd: &Command| {
            let len = cmd.command.len();
            if len >= 8 && len <= 30 {
                0  // Best priority
            } else if len > 30 {
                len  // Long commands get lower priority
            } else {
                1000 + len  // Very short commands get lowest priority
            }
        };
        
        exact_matches.sort_by_key(sort_by_reasonable_length);
        command_start_matches.sort_by_key(sort_by_reasonable_length);
        word_start_matches.sort_by_key(sort_by_reasonable_length);
        prefix_matches.sort_by_key(sort_by_reasonable_length);
    } else {
        exact_matches.sort_by_key(|cmd| cmd.command.len());
        command_start_matches.sort_by_key(|cmd| cmd.command.len());
        word_start_matches.sort_by_key(|cmd| cmd.command.len());
        prefix_matches.sort_by_key(|cmd| cmd.command.len());
    }
    
    // Debug output  
    let exact_count = exact_matches.len();
    let cmd_start_count = command_start_matches.len();
    let word_count = word_start_matches.len();
    let prefix_count = prefix_matches.len();
    
    // For short searches, interleave categories to show variety in top 10
    let mut filtered_commands = Vec::new();
    if search_no_spaces.len() <= 2 {
        // Take top items from each category to ensure variety
        let mut exact_iter = exact_matches.into_iter();
        let mut start_iter = command_start_matches.into_iter();
        let mut word_iter = word_start_matches.into_iter();
        let mut prefix_iter = prefix_matches.into_iter();
        
        // Interleave: 2 exact, 3 cmd-start, 3 word-start, 2 more cmd-start
        filtered_commands.extend(exact_iter.by_ref().take(2));
        filtered_commands.extend(start_iter.by_ref().take(3));
        filtered_commands.extend(word_iter.by_ref().take(3));
        filtered_commands.extend(start_iter.by_ref().take(2));
        
        // Fill remaining slots up to 10
        while filtered_commands.len() < 10 {
            if let Some(cmd) = word_iter.next() {
                filtered_commands.push(cmd);
            } else if let Some(cmd) = start_iter.next() {
                filtered_commands.push(cmd);
            } else if let Some(cmd) = prefix_iter.next() {
                filtered_commands.push(cmd);
            } else {
                break;
            }
        }
    } else {
        // For longer searches, use original priority order
        filtered_commands.extend(exact_matches);
        filtered_commands.extend(command_start_matches);
        filtered_commands.extend(word_start_matches);
        filtered_commands.extend(prefix_matches);
    }
    
    if debug {
        println!("Debug: found {} exact, {} cmd-start, {} word-start, {} prefix matches, {} total", 
                 exact_count, cmd_start_count, word_count, prefix_count, filtered_commands.len());
    }
    
    filtered_commands
}

fn matches_at_word_boundary(cmd_text: &str, search_no_spaces: &str, start_pos: usize) -> bool {
    let cmd_chars = cmd_text[start_pos..].chars();
    let mut search_chars = search_no_spaces.chars();
    
    let mut current_search_char = search_chars.next();
    
    for cmd_char in cmd_chars {
        if let Some(search_char) = current_search_char {
            if cmd_char == ' ' {
                // Skip spaces in command text
                continue;
            } else if cmd_char == search_char {
                // Characters match, advance search
                current_search_char = search_chars.next();
            } else {
                // Characters don't match
                return false;
            }
        } else {
            // We've matched all search characters
            return true;
        }
    }
    
    // Check if we matched all search characters
    current_search_char.is_none()
}

pub fn execute_command(command: &str) {
    let content = format!("execute {}\n", command);
    
    match std::fs::write("/tmp/cmd_file", content) {
        Ok(_) => {
            // Successfully wrote to file
        }
        Err(e) => {
            eprintln!("Error writing to /tmp/cmd_file: {}", e);
        }
    }
}

pub fn save_commands(commands: &[Command]) -> Result<(), Box<dyn std::error::Error>> {
    let home = env::var("HOME").unwrap_or_else(|_| ".".to_string());
    let file_path = Path::new(&home).join("ob/data/spot_cmds/spot_cmds.txt");
    
    let contents = commands.iter()
        .map(|cmd| cmd.full_line.clone())
        .collect::<Vec<String>>()
        .join("\n");
    
    fs::write(&file_path, contents)?;
    Ok(())
}

pub fn update_command_list(commands: &mut Vec<Command>, new_command: Command, original_command_name: &str) {
    // Remove the original command if it exists
    if !original_command_name.is_empty() {
        commands.retain(|cmd| cmd.command != original_command_name);
    }
    
    // Add the new command
    commands.push(new_command);
}