//! Template Operations
//!
//! All template-related business logic consolidated in one place.
//! This module provides a clean API surface for template context creation
//! and variable expansion.
//!
//! Templates use JavaScript evaluation for `{{...}}` expressions, with access
//! to command objects (selected, last_executed, last_anchor), date/time,
//! grabbed content, and environment variables.

use std::collections::HashMap;
use crate::core::Command;
use crate::core::template_creation::TemplateContext;

// Re-export types that callers need
pub use crate::core::template_creation::{
    Template,
    TemplateOperation,
    add_datetime_variables,
};

// =============================================================================
// CONTEXT CREATION
// =============================================================================

/// Create a basic template context with universally available data.
///
/// This is the entry point for creating template contexts. It includes:
/// - Input text
/// - Date/time variables
/// - Last executed command info
/// - Last anchor info
/// - Placeholder grabber variables
/// - Empty selected command (can be filled via add_selected_command)
///
/// # Arguments
/// * `input` - The input text (search text, user input, etc.)
///
/// # Returns
/// A TemplateContext ready for variable expansion
pub fn create_context(input: &str) -> TemplateContext {
    TemplateContext::create_basic_template(input)
}

/// Create a template context with selected command info.
///
/// This is a convenience function that creates a basic context and adds
/// the selected command variables in one call.
///
/// # Arguments
/// * `input` - The input text
/// * `selected` - Optional selected command to include in context
///
/// # Returns
/// A TemplateContext with selected command variables populated
pub fn create_context_with_selection(input: &str, selected: Option<&Command>) -> TemplateContext {
    let mut context = create_context(input);
    context.add_popup_variables(selected);
    context
}

// =============================================================================
// TEMPLATE EXPANSION
// =============================================================================

/// Expand a template string using the given context.
///
/// Processes `{{...}}` expressions using JavaScript evaluation.
/// Variables available in expressions include:
/// - `input`, `raw_input` - input text
/// - `selected.name`, `selected.path`, `selected.folder`, etc.
/// - `last_executed.name`, `last_executed.path`, etc.
/// - `last_anchor.name`, `last_anchor.folder`, etc.
/// - `date.year`, `date.month`, `date.day`, etc.
/// - `env.home`, `env.user`, etc.
/// - `grabbed.action`, `grabbed.arg`, etc.
///
/// # Arguments
/// * `template` - String containing `{{...}}` expressions
/// * `context` - TemplateContext with variable values
///
/// # Returns
/// Expanded string with all `{{...}}` expressions evaluated
pub fn expand(template: &str, context: &TemplateContext) -> String {
    context.expand(template)
}

/// Expand all string parameters in a HashMap.
///
/// Useful for batch expansion of configuration values or action parameters.
///
/// # Arguments
/// * `params` - HashMap of parameter names to values
/// * `context` - TemplateContext for expansion
///
/// # Returns
/// New HashMap with all `{{...}}` expressions expanded
pub fn expand_params(params: &HashMap<String, String>, context: &TemplateContext) -> HashMap<String, String> {
    context.expand_hashmap(params)
}

/// Expand template expressions in an Action's parameters.
///
/// This modifies the action in place, expanding any `{{...}}` templates
/// in string parameters.
///
/// # Arguments
/// * `action` - Action to expand parameters in (modified in place)
/// * `context` - TemplateContext for expansion
pub fn expand_action(action: &mut crate::execute::Action, context: &TemplateContext) {
    context.expand_action_parameters(action)
}

// =============================================================================
// CONTEXT MODIFICATION
// =============================================================================

/// Add a custom variable to the context.
///
/// # Arguments
/// * `context` - TemplateContext to modify
/// * `name` - Variable name
/// * `value` - Variable value
pub fn add_variable(context: &mut TemplateContext, name: &str, value: &str) {
    context.add_variable(name.to_string(), value.to_string());
}

/// Add grabber variables to the context.
///
/// Called after grab functionality has captured window/selection info.
///
/// # Arguments
/// * `context` - TemplateContext to modify
/// * `grabbed_vars` - HashMap of grabbed variable names to values
pub fn add_grabber_variables(context: &mut TemplateContext, grabbed_vars: HashMap<String, String>) {
    context.add_grabber_variables(grabbed_vars);
}

/// Add selected command variables to the context.
///
/// Updates the `selected.*` variables in the context.
///
/// # Arguments
/// * `context` - TemplateContext to modify
/// * `selected` - Optional command to use for selected variables
pub fn add_selected_command(context: &mut TemplateContext, selected: Option<&Command>) {
    context.add_popup_variables(selected);
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // Note: Most template_ops functions require system initialization (get_state, get_sys_data)
    // because create_context loads last_executed and last_anchor from state.
    // Full integration tests for these functions are in tests/integration_*.rs

    // Unit tests for functions that don't require system state:

    #[test]
    fn test_add_datetime_variables() {
        let mut vars = HashMap::new();
        add_datetime_variables(&mut vars);

        // Check that date variables were added
        assert!(vars.contains_key("YYYY"));
        assert!(vars.contains_key("MM"));
        assert!(vars.contains_key("DD"));
        assert!(vars.contains_key("hh"));
        assert!(vars.contains_key("mm"));
        assert!(vars.contains_key("ss"));
    }
}
