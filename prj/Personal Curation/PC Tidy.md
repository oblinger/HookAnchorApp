# PC Tidy

Runbook for validating and correcting anchor folder structure.

---

## Quick Reference

**Trigger:** User says "tidy" (optionally with anchor name)

**What happens:** Claude scans the anchor and validates/corrects each item in the checklist below.

---

## Tidy Checklist

Execute each of these items when tidying an anchor:

### 1. Anchor Page Links

Ensure all documentation is linked directly or indirectly from the anchor page.

- The anchor page should be a dense, scannable index
- If any chunk of documentation or links starts to get too large, move it to a subpage and link to that subpage
- Goal: full discoverability while keeping anchor page concise

### 2. Roadmap Content

Verify the roadmap contains only high-level milestone descriptions.

- **Detailed discussion** → Move to Notes document, reference from roadmap
- **Detailed task lists** → Move to Todo document, reference from roadmap
- Roadmap should be scannable, not a wall of text

### 3. Link Table Rows

Check the anchor page link table:

- All links should be valid (no broken links)
- Remove or update outdated links
- Ensure external links (repo, docs) are current
- Each row should have appropriate content

### 4. TLC Index Entry

If the anchor has a TLC:

- Verify it appears in the [[TLC]] index table
- Check that date, link, full name, and description are correct
- Location: `~/ob/kmr/SYS/Closet/Three Letter Codes/TLC.md`

---

## How to Find Anchors

Use the `ha` (HookAnchor) command to find anchor paths:

```bash
ha -p ASP              # Returns path to the ASP anchor folder
ha -p "Alien Biology"  # Find by full name
```

---

## Anchor Structure Reference

An anchor should have:

```
{anchor}/
├── {NAME} Docs/
│   ├── {NAME}.md          # Primary anchor page
│   ├── {NAME} PRD.md
│   ├── {NAME} Roadmap.md
│   ├── {NAME} Todo.md
│   └── ...
└── {kebab-name}/          # Repository (if code project)
```

For private repos, the repository IS the anchor (no wrapper folder).

---

## Common Issues to Fix

- **Orphaned docs** — Files in Docs folder not linked from anchor page
- **Bloated roadmap** — Detailed content that should be in Notes or Todo
- **Stale links** — External links pointing to old URLs
- **Missing TLC entry** — Anchor has TLC but isn't in the index
- **Inconsistent naming** — Files not prefixed with {NAME}
