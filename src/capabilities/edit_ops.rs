//! Edit Operations
//!
//! Functions for creating, updating, and deleting commands.
//! This module provides the command CRUD operations.

use crate::core::Command;

// =============================================================================
// COMMAND CREATION
// =============================================================================

/// Parameters for creating a new command.
#[derive(Debug, Clone, Default)]
pub struct CreateCommandParams {
    /// Command name (required)
    pub name: String,
    /// Action type (e.g., "url", "folder", "markdown")
    pub action: String,
    /// Action argument (URL, path, etc.)
    pub arg: String,
    /// Patch/category name
    pub patch: String,
    /// Command flags (e.g., "A" for anchor)
    pub flags: String,
}

impl CreateCommandParams {
    /// Create params for a new command with minimal required fields.
    pub fn new(name: &str, action: &str, arg: &str) -> Self {
        Self {
            name: name.to_string(),
            action: action.to_string(),
            arg: arg.to_string(),
            patch: String::new(),
            flags: String::new(),
        }
    }

    /// Set the patch for this command.
    pub fn with_patch(mut self, patch: &str) -> Self {
        self.patch = patch.to_string();
        self
    }

    /// Set the flags for this command.
    pub fn with_flags(mut self, flags: &str) -> Self {
        self.flags = flags.to_string();
        self
    }

    /// Mark this command as an anchor.
    pub fn as_anchor(mut self) -> Self {
        if !self.flags.contains('A') {
            self.flags.push('A');
        }
        self
    }
}

/// Create a new command from parameters.
///
/// # Arguments
/// * `params` - Command creation parameters
///
/// # Returns
/// The newly created Command (not yet saved)
pub fn create_command(params: CreateCommandParams) -> Command {
    Command::new(
        params.patch,
        params.name,
        params.action,
        params.arg,
        params.flags,
    )
}

/// Create an alias command that points to another command.
///
/// # Arguments
/// * `alias_name` - Name for the alias
/// * `target_name` - Name of command to alias to
/// * `patch` - Optional patch for the alias
///
/// # Returns
/// The alias Command (not yet saved)
pub fn create_alias(alias_name: &str, target_name: &str, patch: &str) -> Command {
    Command::new(
        patch.to_string(),
        alias_name.to_string(),
        "alias".to_string(),
        target_name.to_string(),
        String::new(),
    )
}

// =============================================================================
// COMMAND VALIDATION
// =============================================================================

/// Validation error types.
#[derive(Debug, Clone, PartialEq)]
pub enum ValidationError {
    /// Command name is empty
    EmptyName,
    /// Command name contains invalid characters
    InvalidName(String),
    /// Action type is empty
    EmptyAction,
    /// Action type is not recognized
    UnknownAction(String),
    /// Argument is required for this action type but empty
    EmptyArgument,
    /// Duplicate command name exists
    DuplicateName(String),
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValidationError::EmptyName => write!(f, "Command name cannot be empty"),
            ValidationError::InvalidName(reason) => write!(f, "Invalid command name: {}", reason),
            ValidationError::EmptyAction => write!(f, "Action type cannot be empty"),
            ValidationError::UnknownAction(action) => write!(f, "Unknown action type: {}", action),
            ValidationError::EmptyArgument => write!(f, "Argument is required for this action type"),
            ValidationError::DuplicateName(name) => write!(f, "Command '{}' already exists", name),
        }
    }
}

/// Known action types that are valid.
#[allow(dead_code)]
const KNOWN_ACTIONS: &[&str] = &[
    "url", "folder", "markdown", "doc", "text", "app", "open", "open_app",
    "cmd", "alias", "tmux", "js", "applescript", "template",
];

/// Validate a command before saving.
///
/// # Arguments
/// * `command` - Command to validate
/// * `existing_commands` - List of existing commands (for duplicate check)
/// * `is_new` - Whether this is a new command (vs update)
///
/// # Returns
/// Ok(()) if valid, Err(ValidationError) otherwise
pub fn validate_command(
    command: &Command,
    existing_commands: &[Command],
    is_new: bool,
) -> Result<(), ValidationError> {
    // Check name
    if command.command.trim().is_empty() {
        return Err(ValidationError::EmptyName);
    }

    // Check for invalid characters in name
    if command.command.contains('\n') || command.command.contains('\t') {
        return Err(ValidationError::InvalidName("contains newline or tab".to_string()));
    }

    // Check action
    if command.action.trim().is_empty() {
        return Err(ValidationError::EmptyAction);
    }

    // Warn about unknown actions (but don't reject - could be custom)
    // if !KNOWN_ACTIONS.contains(&command.action.as_str()) {
    //     // Just a warning, not an error
    // }

    // Check argument for actions that require it
    let requires_arg = matches!(command.action.as_str(),
        "url" | "folder" | "markdown" | "doc" | "text" | "alias" | "tmux"
    );
    if requires_arg && command.arg.trim().is_empty() {
        return Err(ValidationError::EmptyArgument);
    }

    // Check for duplicates (only for new commands)
    if is_new {
        let name_lower = command.command.to_lowercase();
        if existing_commands.iter().any(|c| c.command.to_lowercase() == name_lower) {
            return Err(ValidationError::DuplicateName(command.command.clone()));
        }
    }

    Ok(())
}

// =============================================================================
// COMMAND MODIFICATION
// =============================================================================

/// Update a command's properties.
///
/// # Arguments
/// * `command` - Command to modify (mutated in place)
/// * `action` - New action type (None to keep current)
/// * `arg` - New argument (None to keep current)
/// * `patch` - New patch (None to keep current)
/// * `flags` - New flags (None to keep current)
pub fn update_command(
    command: &mut Command,
    action: Option<&str>,
    arg: Option<&str>,
    patch: Option<&str>,
    flags: Option<&str>,
) {
    if let Some(a) = action {
        command.action = a.to_string();
    }
    if let Some(a) = arg {
        command.arg = a.to_string();
    }
    if let Some(p) = patch {
        command.patch = p.to_string();
    }
    if let Some(f) = flags {
        command.flags = f.to_string();
    }
}

/// Set or clear the anchor flag on a command.
///
/// # Arguments
/// * `command` - Command to modify
/// * `is_anchor` - Whether command should be an anchor
pub fn set_anchor_flag(command: &mut Command, is_anchor: bool) {
    command.set_anchor(is_anchor);
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_command_params() {
        let params = CreateCommandParams::new("test", "url", "https://example.com")
            .with_patch("MyPatch")
            .as_anchor();

        assert_eq!(params.name, "test");
        assert_eq!(params.action, "url");
        assert_eq!(params.arg, "https://example.com");
        assert_eq!(params.patch, "MyPatch");
        assert!(params.flags.contains('A'));
    }

    #[test]
    fn test_create_command() {
        let params = CreateCommandParams::new("test", "url", "https://example.com");
        let cmd = create_command(params);

        assert_eq!(cmd.command, "test");
        assert_eq!(cmd.action, "url");
        assert_eq!(cmd.arg, "https://example.com");
    }

    #[test]
    fn test_create_alias() {
        let cmd = create_alias("fb", "Fireball", "Aliases");

        assert_eq!(cmd.command, "fb");
        assert_eq!(cmd.action, "alias");
        assert_eq!(cmd.arg, "Fireball");
        assert_eq!(cmd.patch, "Aliases");
    }

    #[test]
    fn test_validate_empty_name() {
        let cmd = Command::new("".to_string(), "".to_string(), "url".to_string(), "https://x.com".to_string(), "".to_string());
        assert_eq!(validate_command(&cmd, &[], true), Err(ValidationError::EmptyName));
    }

    #[test]
    fn test_validate_empty_action() {
        let cmd = Command::new("".to_string(), "test".to_string(), "".to_string(), "arg".to_string(), "".to_string());
        assert_eq!(validate_command(&cmd, &[], true), Err(ValidationError::EmptyAction));
    }

    #[test]
    fn test_validate_duplicate() {
        let existing = vec![
            Command::new("".to_string(), "test".to_string(), "url".to_string(), "https://x.com".to_string(), "".to_string()),
        ];
        let cmd = Command::new("".to_string(), "TEST".to_string(), "url".to_string(), "https://y.com".to_string(), "".to_string());

        assert_eq!(
            validate_command(&cmd, &existing, true),
            Err(ValidationError::DuplicateName("TEST".to_string()))
        );
    }
}
