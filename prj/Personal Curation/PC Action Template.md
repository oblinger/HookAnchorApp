# PC Action Template

Template for creating action runbook documents.

---

## Structure

Action runbooks should follow this structure:

```markdown
# PC {Action Name}

**Objectives:**
- What this action accomplishes
- Key benefits or goals

**Key Constraint**: Any critical rule that must always be followed during this action.

## The {ACTION NAME} Procedure

**PRECONDITIONS**
- What must be true before starting
- Required state or setup

**{ACTION NAME} PROCEDURE**
1. Step one
2. Step two
3. ...

# Notes

## {Topic}
Additional details, edge cases, or explanations for specific aspects.

## Implementation Details
Technical specifics, commands, code snippets if needed.
```

---

## Guidelines

### Objectives Section
- Start with clear, bulleted objectives
- Focus on what the user gets out of this action
- Include any efficiency or workflow benefits

### Key Constraint
- If there's ONE rule that must never be violated, state it prominently
- Example from PR Flow: "Claude must always alert the user when stopping"

### Procedure Section
- Use **PRECONDITIONS** for required starting state
- Use **{ACTION NAME} PROCEDURE** for the numbered steps
- Keep steps high-level and scannable
- Link to other actions if they're prerequisites (e.g., `[[PC Roadmap Ready]]`)

### Notes Section
- Use H2 headings for different topics
- Put edge cases, explanations, and "why" content here
- Keep separate from the main procedure for clarity

### Implementation Details
- Include specific commands, code, or technical steps
- Use code blocks for bash commands
- This section is for Claude to execute, not for user reading

---

## Naming Convention

- File: `PC {Action Name}.md`
- Title: `# PC {Action Name}`
- Procedure heading: `## The {ACTION NAME} Procedure`

---

## Example Reference

See [[PC PR Flow]] for a complete example of this template in use.
