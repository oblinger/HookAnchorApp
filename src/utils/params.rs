use std::collections::HashMap;
use crate::prelude::*;

/// Parse key-value pairs from a string using := separator
/// Handles backslash escaping for literal := in values
/// Returns HashMap of key->value pairs
///
/// Format: "KEY1:=value1 KEY2:=value2 ..."
/// Example: "template:=mytemplate priority:=high"
pub fn parse_kv_pairs(input: &str) -> HashMap<String, String> {
    let mut result = HashMap::new();
    let mut current_key = String::new();
    let mut current_value = String::new();
    let mut chars = input.chars().peekable();
    let mut in_value = false;

    while let Some(ch) = chars.next() {
        if !in_value {
            // Looking for key
            if ch == ':' && chars.peek() == Some(&'=') {
                chars.next(); // consume '='
                in_value = true;
            } else if !ch.is_whitespace() {
                current_key.push(ch);
            }
        } else {
            // Looking for value
            if ch == '\\' && chars.peek() == Some(&':') {
                // Escaped := - add literal : to value
                current_value.push(':');
                chars.next(); // consume ':'
                if chars.peek() == Some(&'=') {
                    current_value.push('=');
                    chars.next(); // consume '='
                }
            } else if ch == ':' && chars.peek() == Some(&'=') {
                // Unescaped := means start of next key
                // Scan backwards in current_value to find last space
                // Everything before last space is the actual value
                // Everything after last space is the next key

                let value_str = current_value.as_str();
                if let Some(last_space_pos) = value_str.rfind(' ') {
                    // Split at last space
                    let actual_value = &value_str[..last_space_pos];
                    let next_key = &value_str[last_space_pos + 1..];

                    // Save current pair with corrected value
                    if !current_key.is_empty() {
                        result.insert(current_key.trim().to_string(), actual_value.trim().to_string());
                    }

                    // Set up for next key-value pair
                    current_key = next_key.to_string();
                    current_value.clear();
                    chars.next(); // consume '='
                    // Stay in value mode (in_value remains true)
                } else {
                    // No space found - malformed input, treat whole thing as value
                    detailed_log("PARSE_KV", &format!(
                        "Warning: No space before next key in KV pairs. Key='{}', Value='{}'",
                        current_key, current_value
                    ));
                    if !current_key.is_empty() {
                        result.insert(current_key.trim().to_string(), current_value.trim().to_string());
                    }
                    current_key.clear();
                    current_value.clear();
                    chars.next(); // consume '='
                    in_value = false;
                }
            } else {
                current_value.push(ch);
            }
        }
    }

    // Save last pair
    if !current_key.is_empty() && in_value {
        result.insert(current_key.trim().to_string(), current_value.trim().to_string());
    }

    result
}

/// Format a HashMap of key-value pairs back to string format
/// Output format: "KEY1:=value1 KEY2:=value2 ..."
/// Keys are sorted alphabetically for consistent output
pub fn format_kv_pairs(params: &HashMap<String, String>) -> String {
    if params.is_empty() {
        return String::new();
    }

    let mut pairs: Vec<_> = params.iter().collect();
    pairs.sort_by_key(|(k, _)| *k);

    pairs.iter()
        .map(|(k, v)| format!("{}:={}", k, v))
        .collect::<Vec<_>>()
        .join(" ")
}

/// Known parameter names - use these constants to avoid typos
pub const PARAM_TEMPLATE: &str = "template";
pub const PARAM_PRIORITY: &str = "priority";
pub const PARAM_FLAGS: &str = "F";
pub const PARAM_ARG: &str = "A";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_kv_pairs_basic() {
        let input = "template:=mytemplate priority:=high";
        let result = parse_kv_pairs(input);

        assert_eq!(result.get("template"), Some(&"mytemplate".to_string()));
        assert_eq!(result.get("priority"), Some(&"high".to_string()));
    }

    #[test]
    fn test_parse_kv_pairs_with_spaces_in_value() {
        let input = "template:=my template name priority:=high";
        let result = parse_kv_pairs(input);

        assert_eq!(result.get("template"), Some(&"my template name".to_string()));
        assert_eq!(result.get("priority"), Some(&"high".to_string()));
    }

    #[test]
    fn test_parse_kv_pairs_escaped() {
        let input = "url:=https\\://example.com port:=8080";
        let result = parse_kv_pairs(input);

        assert_eq!(result.get("url"), Some(&"https:=example.com".to_string()));
        assert_eq!(result.get("port"), Some(&"8080".to_string()));
    }

    #[test]
    fn test_format_kv_pairs_basic() {
        let mut params = HashMap::new();
        params.insert("template".to_string(), "mytemplate".to_string());
        params.insert("priority".to_string(), "high".to_string());

        let result = format_kv_pairs(&params);

        // Should be sorted alphabetically
        assert_eq!(result, "priority:=high template:=mytemplate");
    }

    #[test]
    fn test_format_kv_pairs_empty() {
        let params = HashMap::new();
        let result = format_kv_pairs(&params);

        assert_eq!(result, "");
    }

    #[test]
    fn test_roundtrip() {
        let original = "template:=mytemplate priority:=high";
        let parsed = parse_kv_pairs(original);
        let formatted = format_kv_pairs(&parsed);
        let reparsed = parse_kv_pairs(&formatted);

        assert_eq!(parsed, reparsed);
    }
}
