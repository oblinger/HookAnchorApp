# PC Move Anchor

Runbook for moving an anchor folder to a new location.

---

## Quick Reference

**Trigger:** User wants to move an anchor folder

**What happens:** Move the folder and update all systems that index by path.

---

## Checklist

1. [ ] Move the folder itself
2. [ ] Migrate Claude Code sessions
3. [ ] Reindex HookAnchor
4. [ ] Rebuild docs (if applicable)
5. [ ] Update TLC index
6. [ ] Verify git remotes
7. [ ] Scan for hardcoded paths
8. [ ] Test: `claude --continue` works in new location

---

## Step-by-Step

### 1. Move the Folder

```bash
mv "/old/path/{NAME}" "/new/path/{NAME}"
```

### 2. Migrate Claude Code Sessions

Claude Code stores sessions in `~/.claude/projects/` with paths encoded (slashes become dashes). When you move a folder, sessions become orphaned.

**Manual migration:**
```bash
cd ~/.claude/projects/
# Rename the session directory (use -- to handle leading dash)
mv -- -old-path-encoded -new-path-encoded
```

**Example:** Moving `~/ob/proj/My Project` to `~/ob/kmr/prj/My Project`:
```bash
mv -- -Users-oblinger-ob-proj-My-Project -Users-oblinger-ob-kmr-prj-My-Project
```

**Automated migration:** Use the `claude-mv` script:
```bash
claude-mv ~/old/path ~/new/path
```
This handles renaming directories and updating path references in `.jsonl` files.

### 3. Reindex HookAnchor

HookAnchor maintains an index of anchor folders. Trigger a rescan:
```bash
ha --rescan
```

### 4. Rebuild Documentation Index

If the anchor has published docs, rebuild the MkDocs site:
```bash
cd /new/path/{repo}
mkdocs build
```

If using GitHub Pages, push changes to trigger rebuild.

### 5. Update TLC Index

If the anchor has a TLC, update the TLC index table:
- Location: `~/ob/kmr/SYS/Closet/Three Letter Codes/TLC.md`
- Update the path/link to reflect new location

### 6. Update Git Remotes

If the repository was moved, verify remote URLs still work:
```bash
cd /new/path/{repo}
git remote -v
```

### 7. Scan for Hardcoded Paths

Search the codebase for hardcoded references to the old path:
```bash
cd /new/path/{repo}
grep -r "/old/path" .
grep -r "old-path-segment" .
```

**Common places where paths may be hardcoded:**
- `CLAUDE.md` — Project instructions may reference absolute paths
- Config files — pyproject.toml, mkdocs.yml, justfile
- Scripts — Shell scripts, Python scripts with path constants
- Documentation — Examples or tutorials with absolute paths
- Tests — Test fixtures or test data paths

### 8. Verify

After migration, test that Claude Code works:
```bash
cd /new/path/{NAME}
claude --continue
```

---

## Notes

- Git remotes are URL-based, so they typically don't need updating after a local move
- Obsidian wiki links (`[[...]]`) are relative, so they should continue to work
- External links in documentation may need updating if they reference the old path
