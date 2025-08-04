#!/usr/bin/env python3
import csv
import re
from difflib import SequenceMatcher

def similarity(a, b):
    """Calculate similarity between two strings (0-1, where 1 is identical)"""
    return SequenceMatcher(None, a.lower(), b.lower()).ratio()

def normalize_name(name):
    """Normalize a name for comparison by removing common variations"""
    # Remove common suffixes and prefixes
    name = re.sub(r'\s*\([^)]*\)', '', name)  # Remove parentheses content
    name = re.sub(r'\s*-\s*.*$', '', name)    # Remove dash and everything after
    name = re.sub(r'\s+', ' ', name).strip()   # Normalize whitespace
    return name.lower()

def inject_1password_entries():
    """
    Conservative script to inject 1Password entries into commands.txt
    Only adds entries that don't have similar names already present
    """
    
    # Read existing commands
    commands_file = "/Users/oblinger/.config/hookanchor/commands.txt"
    onepass_csv = "/Users/oblinger/Desktop/1-Password update.csv"
    
    print("Reading existing commands...")
    existing_commands = []
    existing_names = set()
    onepass_names = set()
    
    try:
        with open(commands_file, 'r', encoding='utf-8') as f:
            for line in f:
                existing_commands.append(line.rstrip('\n'))
                # Extract command names for comparison
                if ':' in line:
                    name = line.split(':')[0].strip()
                    existing_names.add(normalize_name(name))
                    # Also track existing 1pass entries
                    if '1pass;' in line.lower():
                        # Extract the 1pass name
                        parts = line.split(';')
                        if len(parts) > 1:
                            onepass_name = parts[1].strip()
                            onepass_names.add(normalize_name(onepass_name))
    except FileNotFoundError:
        print(f"Commands file not found: {commands_file}")
        return
    
    print(f"Found {len(existing_commands)} existing commands")
    print(f"Found {len(onepass_names)} existing 1Password entries")
    
    # Read 1Password entries
    print("Reading 1Password entries...")
    new_entries = []
    skipped_entries = []
    
    try:
        with open(onepass_csv, 'r', encoding='utf-8') as f:
            reader = csv.DictReader(f)
            for row in reader:
                title = row.get('Title', '').strip()
                if not title:
                    continue
                    
                normalized_title = normalize_name(title)
                
                # Check if this entry already exists (with similarity threshold)
                already_exists = False
                similar_to = None
                
                # First check exact matches in existing 1pass entries
                if normalized_title in onepass_names:
                    already_exists = True
                    similar_to = title
                else:
                    # Check for high similarity matches in all existing commands
                    for existing_name in existing_names:
                        if similarity(normalized_title, existing_name) > 0.8:
                            already_exists = True
                            similar_to = existing_name
                            break
                    
                    # Also check against existing 1pass entries with lower threshold
                    if not already_exists:
                        for onepass_name in onepass_names:
                            if similarity(normalized_title, onepass_name) > 0.85:
                                already_exists = True
                                similar_to = onepass_name
                                break
                
                if already_exists:
                    skipped_entries.append((title, similar_to))
                else:
                    # Create new entry in the format: Name : 1pass; Name
                    new_entry = f"{title} : 1pass; {title}"
                    new_entries.append(new_entry)
    
    except FileNotFoundError:
        print(f"1Password CSV not found: {onepass_csv}")
        return
    
    print(f"\nAnalysis complete:")
    print(f"  - New entries to add: {len(new_entries)}")
    print(f"  - Skipped (similar exists): {len(skipped_entries)}")
    
    if skipped_entries:
        print(f"\nSkipped entries (first 10):")
        for i, (title, similar) in enumerate(skipped_entries[:10]):
            print(f"  - '{title}' (similar to: '{similar}')")
        if len(skipped_entries) > 10:
            print(f"  ... and {len(skipped_entries) - 10} more")
    
    if new_entries:
        print(f"\nNew entries to add (first 10):")
        for i, entry in enumerate(new_entries[:10]):
            print(f"  - {entry}")
        if len(new_entries) > 10:
            print(f"  ... and {len(new_entries) - 10} more")
        
        # Proceed automatically (conservative approach - only adding clearly new entries)
        if True:
            # Backup original file
            import shutil
            backup_file = commands_file + ".backup"
            shutil.copy2(commands_file, backup_file)
            print(f"Backup created: {backup_file}")
            
            # Add new entries to the file
            with open(commands_file, 'a', encoding='utf-8') as f:
                for entry in new_entries:
                    f.write('\n' + entry)
            
            print(f"Successfully added {len(new_entries)} new 1Password entries to commands.txt")
        else:
            print("Operation cancelled.")
    else:
        print("\nNo new entries to add - all 1Password entries already have similar matches in commands.txt")

if __name__ == "__main__":
    inject_1password_entries()