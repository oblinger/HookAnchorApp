// Tests for command format parsing and serialization
// Verifies the F:= A:= key-value format works correctly

use hookanchor::core::{Command, parse_command_line};

#[test]
fn test_parse_single_flag() {
    let line = "Hook Anchor! Test:markdown; F:=A";
    let result = parse_command_line(line);

    assert!(result.is_ok(), "Failed to parse: {:?}", result.err());
    let cmd = result.unwrap();

    assert_eq!(cmd.patch, "Hook Anchor");
    assert_eq!(cmd.command, "Test");
    assert_eq!(cmd.action, "markdown");
    assert_eq!(cmd.flags, "A");
    assert_eq!(cmd.arg, "");
}

#[test]
fn test_parse_multiple_flags() {
    let line = "Hook Anchor! Test:markdown; F:=UA";
    let result = parse_command_line(line);

    assert!(result.is_ok(), "Failed to parse: {:?}", result.err());
    let cmd = result.unwrap();

    assert_eq!(cmd.flags, "UA");
}

#[test]
fn test_parse_flag_and_arg() {
    let line = "Hook Anchor! Test:markdown; F:=UA A:=/path/to/file.md";
    let result = parse_command_line(line);

    assert!(result.is_ok(), "Failed to parse: {:?}", result.err());
    let cmd = result.unwrap();

    assert_eq!(cmd.flags, "UA");
    assert_eq!(cmd.arg, "/path/to/file.md");
}

#[test]
fn test_parse_arg_only() {
    let line = "Hook Anchor! Test:markdown; A:=/path/to/file.md";
    let result = parse_command_line(line);

    assert!(result.is_ok(), "Failed to parse: {:?}", result.err());
    let cmd = result.unwrap();

    assert_eq!(cmd.flags, "");
    assert_eq!(cmd.arg, "/path/to/file.md");
}

#[test]
fn test_parse_with_comma_in_flags() {
    // Commas in flags should be stripped out by sanitization
    let line = "Hook Anchor! Test:markdown; F:=U,A A:=/path/to/file.md";
    let result = parse_command_line(line);

    assert!(result.is_ok(), "Failed to parse: {:?}", result.err());
    let cmd = result.unwrap();

    assert_eq!(cmd.flags, "UA", "Commas should be removed from flags");
    assert_eq!(cmd.arg, "/path/to/file.md");
}

#[test]
fn test_serialize_flag_only() {
    let cmd = Command {
        patch: "Test".to_string(),
        command: "MyCommand".to_string(),
        action: "markdown".to_string(),
        arg: "".to_string(),
        flags: "A".to_string(),
        other_params: None,
        last_update: 0,
        file_size: None,
    };

    let serialized = cmd.to_new_format();
    assert_eq!(serialized, "Test! MyCommand:markdown; F:=A");
}

#[test]
fn test_serialize_flag_and_arg() {
    let cmd = Command {
        patch: "Test".to_string(),
        command: "MyCommand".to_string(),
        action: "markdown".to_string(),
        arg: "/path/to/file.md".to_string(),
        flags: "UA".to_string(),
        other_params: None,
        last_update: 0,
        file_size: None,
    };

    let serialized = cmd.to_new_format();
    assert_eq!(serialized, "Test! MyCommand:markdown; F:=UA A:=/path/to/file.md");
}

#[test]
fn test_roundtrip_parse_and_serialize() {
    let original = "Hook Anchor! Test:markdown; F:=UA A:=/path/to/file.md";

    let parsed = parse_command_line(original).unwrap();
    let serialized = parsed.to_new_format();

    assert_eq!(serialized, original);
}

#[test]
fn test_roundtrip_with_special_characters_in_path() {
    let original = "Hook Anchor! Test:markdown; A:=/path/with spaces/file.md";

    let parsed = parse_command_line(original).unwrap();
    let serialized = parsed.to_new_format();

    assert_eq!(serialized, original);
}

#[test]
fn test_parse_multiple_kv_pairs() {
    // Test parsing multiple key-value pairs
    let line = "Test! Cmd:action; F:=UA A:=/path W:=value";
    let result = parse_command_line(line);

    assert!(result.is_ok(), "Failed to parse: {:?}", result.err());
    let cmd = result.unwrap();

    assert_eq!(cmd.flags, "UA");
    assert_eq!(cmd.arg, "/path");
    assert!(cmd.other_params.is_some());

    let params = cmd.other_params.unwrap();
    assert_eq!(params.get("W"), Some(&"value".to_string()));
}
