# PC Archive

Runbook for archiving old code, documentation, or other materials.

---

## Quick Reference

**Trigger:** User wants to archive something (old code, deprecated docs, legacy implementation)

**What happens:** Move the item to a `Yore/` subfolder with a date prefix.

---

## Why "Yore"?

The `Yore/` subfolder stores archived materials that are no longer active but worth preserving. The name "Yore" (meaning "long ago") sorts late alphabetically, keeping archives at the bottom of directory listings.

---

## When to Archive

- Old versions of code being replaced
- Deprecated documentation
- Legacy implementations kept for reference
- Backup copies before major refactoring

---

## Archive Naming

Prefix archived items with the date in `YYYY-MM-DD` format:

```
Yore/
├── 2026-01-12 OldProjectName/
├── 2025-11-15 DeprecatedModule/
└── 2025-08-20 LegacyDocs/
```

---

## Procedure

### 1. Create Yore folder if needed

```bash
mkdir -p Yore
```

### 2. Move with date prefix

```bash
mv "OldProject" "Yore/$(date +%Y-%m-%d) OldProject"
```

### 3. Update references

- Update any symlinks that pointed to the archived item
- Update any imports or includes in code
- Update documentation that referenced the old location

### 4. Document the archive

Add a note in the anchor page or README about:
- What was archived
- Why it was archived
- Where to find it if needed

---

## Example

Archiving an old implementation:

```bash
# Create archive folder
mkdir -p Yore

# Move with date
mv "src/old_auth.py" "Yore/$(date +%Y-%m-%d) old_auth.py"

# Or for a directory
mv "legacy_module/" "Yore/$(date +%Y-%m-%d) legacy_module/"
```

---

## Notes

- Don't delete things you might need — archive them instead
- The date prefix makes it easy to see when things were archived
- Yore/ can exist in any folder where you need to archive things
- Consider adding a brief README in Yore/ explaining what's there
