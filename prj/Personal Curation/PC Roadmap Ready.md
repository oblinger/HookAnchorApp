# PC Roadmap Ready

Runbook for validating that a roadmap is ready for execution.

---

## Quick Reference

**Trigger:** User says "roadmap ready"

**What happens:** Claude validates the roadmap content and structure, surfaces open questions, and runs the validation script.

---

## Validation Steps

### 1. Check Task Order

Review upcoming items in the roadmap:
- Are tasks in a logical order?
- Do we have prerequisites for the next items?
- Are there dependencies that should change the order?

If order seems wrong, propose reordering to the user.

### 2. Check for Missing Work

Scan recent conversations, Notes, and Todo documents:
- Has work been done that isn't reflected in the roadmap?
- Has work been discussed that should be added?
- Are there completed items not marked as done?

Add any missing items to the roadmap.

### 3. Surface Open Questions

Check for open questions in:
- `{NAME} Todo.md` — Open Questions section
- `{NAME} Notes.md` — Unresolved discussion points
- `{NAME} Open Questions.md` — If it exists

For each open question:
- Is it referenced in the roadmap? (Should block a milestone)
- Has it been resolved but not updated?

**If unresolved questions exist:**
1. Organize them into a list
2. Present to user for resolution
3. User must resolve before roadmap is "ready"

### 4. Validate Structure

Run the validation script:
```bash
python ~/ob/kmr/prj/Personal Curation/roadmap_check.py "{NAME} Roadmap.md"
```

Fix any reported errors.

---

## Roadmap Structure Rules

### Checkbox Format

All actionable items must have checkboxes:
- `[ ]` — Not started
- `[x]` — Completed
- `[~]` — Deferred (must have revisit reference)

### Valid Patterns

**Phase headings** (H2) — No checkbox needed:
```markdown
## Phase 1: Foundation
```

**Milestone headings** (H3) — Must have checkbox:
```markdown
### [ ] M1.1 - Repository Setup
### [x] M1.2 - Basic Configuration
### [~] M1.3 - Documentation (Deferred - see M2.5)
```

**Deliverable items** — Must have checkbox:
```markdown
- [ ] Git repository initialized
- [x] Directory structure created
- [~] API docs (Deferred - see M2.5)
```

### Deferred Items

When an item is deferred with `[~]`:

1. **Mark the deferred item** with explanation:
   ```markdown
   ### [~] M1.11 - Documentation Sync (Deferred - see M3.14)
   ```

2. **Add a revisit milestone** in the target phase:
   ```markdown
   ### [ ] M3.14 - Revisit: M1.11 Documentation Sync
   ```

3. **Cross-reference both directions** — The validation script checks this.

---

## Validation Script Output

The script reports:
- Lines missing checkboxes where expected
- Deferred items `[~]` without "Deferred - see" reference
- Deferred items without corresponding revisit milestone
- Invalid checkbox format (not `[ ]`, `[x]`, or `[~]`)

---

## Checklist

Before declaring roadmap ready:

1. [ ] Task order makes sense for upcoming work
2. [ ] All discussed/completed work is reflected
3. [ ] Open questions are surfaced and resolved (or blocking milestones)
4. [ ] Validation script passes with no errors
