use std::env;
use std::fs;
use std::path::Path;
use regex::Regex;

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
    
    // Regex to parse: (optional GROUP !)? COMMAND : (whitespace)* ACTION (whitespace)+ ARGUMENT
    let re = Regex::new(r"^(?:(.+?)!\s*)?(.+?):\s*(\S+)\s+(.*)$").unwrap();
    
    contents
        .lines()
        .filter_map(|line| {
            if line.trim().is_empty() {
                return None;
            }
            
            if let Some(captures) = re.captures(line) {
                let group = captures.get(1).map_or(String::new(), |m| m.as_str().trim().to_string());
                let command = captures.get(2).map_or(String::new(), |m| m.as_str().trim().to_string());
                let action = captures.get(3).map_or(String::new(), |m| m.as_str().trim().to_string());
                let arg = captures.get(4).map_or(String::new(), |m| m.as_str().trim().to_string());
                
                Some(Command {
                    group,
                    command,
                    action,
                    arg,
                    full_line: line.to_string(),
                })
            } else {
                // Try to parse lines that have no argument (just action)
                let re_no_arg = Regex::new(r"^(?:(.+?)!\s*)?(.+?):\s*(\S+)\s*$").unwrap();
                if let Some(captures) = re_no_arg.captures(line) {
                    let group = captures.get(1).map_or(String::new(), |m| m.as_str().trim().to_string());
                    let command = captures.get(2).map_or(String::new(), |m| m.as_str().trim().to_string());
                    let action = captures.get(3).map_or(String::new(), |m| m.as_str().trim().to_string());
                    
                    Some(Command {
                        group,
                        command,
                        action,
                        arg: String::new(),
                        full_line: line.to_string(),
                    })
                } else {
                    eprintln!("Failed to parse line: {}", line);
                    None
                }
            }
        })
        .collect()
}

pub fn filter_commands(commands: &[Command], search_text: &str, debug: bool) -> Vec<Command> {
    let search_lower = search_text.to_lowercase();
    let search_words: Vec<&str> = search_lower.split_whitespace().collect();
    
    if debug {
        println!("Debug: searching for words: {:?} (original: '{}')", search_words, search_text);
    }
    
    // Separate into 4 categories for better relevance
    let mut exact_matches: Vec<Command> = Vec::new();        // Exact command name match
    let mut command_start_matches: Vec<Command> = Vec::new(); // Matches at start of command
    let mut word_start_matches: Vec<Command> = Vec::new();   // Matches at start of any other word
    let mut prefix_matches: Vec<Command> = Vec::new();       // Partial prefix matches
    
    for cmd in commands {
        let cmd_lower = cmd.command.to_lowercase();
        
        // Debug specific command
        if debug && cmd_lower.contains("selector") && search_words.contains(&"s") {
            println!("Debug: checking '{}' against words: {:?}", cmd_lower, search_words);
        }
        
        // Check if all search words match sequentially in the command
        if matches_word_sequence(&cmd_lower, &search_words, debug) {
            // Determine priority based on where the match starts
            if search_words.len() == 1 {
                let search_word = search_words[0];
                let cmd_no_separators = cmd_lower.replace(' ', "").replace('_', "");
                let search_no_separators = search_word.replace(' ', "").replace('_', "");
                
                // 1. Exact match (highest priority)
                if cmd_no_separators == search_no_separators {
                    exact_matches.push(cmd.clone());
                }
                // 2. Matches at start of command (high priority) 
                else if cmd_lower.starts_with(search_word) {
                    command_start_matches.push(cmd.clone());
                }
                // 3. Matches after word boundary
                else if cmd_lower.contains(&format!(" {}", search_word)) || cmd_lower.contains(&format!("_{}", search_word)) {
                    word_start_matches.push(cmd.clone());
                }
                // 4. Partial prefix matches (lowest priority)
                else {
                    prefix_matches.push(cmd.clone());
                }
            } else {
                // Multi-word searches get medium priority
                word_start_matches.push(cmd.clone());
            }
        }
    }
    
    // Sort each category - for very short searches, prefer longer, more meaningful commands
    let prefer_longer = search_words.get(0).map_or(false, |w| w.len() <= 2);
    
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
    if search_words.get(0).map_or(false, |w| w.len() <= 2) {
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

fn matches_word_sequence(cmd_text: &str, search_words: &[&str], debug: bool) -> bool {
    if search_words.is_empty() {
        return true;
    }
    
    // Handle both space-separated search and continuous search
    if search_words.len() == 1 {
        // For single "word" searches, try flexible matching
        return matches_flexible_sequence(cmd_text, search_words[0], debug);
    } else {
        // For multi-word searches, use exact word matching
        let cmd_words: Vec<&str> = cmd_text.split(|c| c == ' ' || c == '_').collect();
        
        if debug && cmd_text.contains("selector") {
            println!("Debug: cmd_words: {:?}, search_words: {:?}", cmd_words, search_words);
        }
        
        let mut search_idx = 0;
        for cmd_word in cmd_words {
            if search_idx >= search_words.len() {
                break;
            }
            
            if cmd_word.starts_with(search_words[search_idx]) {
                search_idx += 1;
                if debug && cmd_text.contains("selector") {
                    println!("Debug: matched '{}' with '{}', search_idx now {}", cmd_word, search_words[search_idx-1], search_idx);
                }
            }
        }
        
        search_idx == search_words.len()
    }
}

fn matches_flexible_sequence(cmd_text: &str, search_text: &str, debug: bool) -> bool {
    // Split command into words using space and underscore as delimiters
    let cmd_words: Vec<&str> = cmd_text.split(|c| c == ' ' || c == '_').collect();
    
    
    // Try to match the search text by consuming characters from command words in sequence
    let search_chars: Vec<char> = search_text.chars().collect();
    let mut search_pos = 0;
    let mut word_idx = 0;
    let mut char_idx_in_word = 0;
    
    while search_pos < search_chars.len() && word_idx < cmd_words.len() {
        let current_word = cmd_words[word_idx];
        let word_chars: Vec<char> = current_word.chars().collect();
        
        
        if char_idx_in_word < word_chars.len() {
            // Try to match current character in current word
            if word_chars[char_idx_in_word] == search_chars[search_pos] {
                search_pos += 1;
                char_idx_in_word += 1;
            } else {
                // Character doesn't match, try jumping to next word if we've matched at least one char in current word
                if char_idx_in_word > 0 {
                    word_idx += 1;
                    char_idx_in_word = 0;
                } else {
                    // Can't match first character of word, try next word
                    word_idx += 1;
                    char_idx_in_word = 0;
                }
            }
        } else {
            // Reached end of current word, move to next word
            word_idx += 1;
            char_idx_in_word = 0;
        }
    }
    
    let result = search_pos == search_chars.len();
    result
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