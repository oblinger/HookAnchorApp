# 1Password Integration Scripts

This folder contains scripts for managing 1Password entries and integrating them with HookAnchor's commands.txt file.

## Files

### filter_1password.py
Filters 1Password CSV exports based on specific criteria:
- Keeps only favorited entries
- Requires entries to have URL, username, and password
- Removes credit card entries
- Removes entries containing "coblinger@icloud.com"
- Removes entries containing "CCARD"
- Removes entries without a title/name

**Usage:**
```bash
python3 filter_1password.py
```

### inject_1password_entries.py
Conservative script to add 1Password entries to commands.txt:
- Only adds entries that don't already have similar names in commands.txt
- Uses fuzzy string matching to avoid duplicates
- Creates entries in format: `Name : 1pass; Name`
- Automatically backs up commands.txt before making changes

**Usage:**
```bash
python3 inject_1password_entries.py
```

## Workflow

1. Export your 1Password data to CSV
2. Run `filter_1password.py` to clean the data
3. Run `inject_1password_entries.py` to add new entries to commands.txt

## Notes

- Both scripts are conservative and designed to avoid data loss
- The filter script reads/writes to "1-Password update.csv" by default
- The injection script uses similarity matching (80-85% threshold) to avoid duplicates
- Always creates backups before modifying files