#!/usr/bin/env python3
import csv
import sys

def filter_1password_csv(input_file, output_file):
    """
    Filter 1Password CSV export to keep only items that:
    - Are favorited (MUST be favorited)
    - Have username, password, URL, AND title/name
    - Are not credit cards (based on title patterns)
    - Do not contain "coblinger@icloud.com" (case insensitive)
    - Do not contain "CCARD" anywhere in the entry
    """
    
    # Credit card patterns to exclude
    credit_card_patterns = [
        'credit card', 'visa', 'mastercard', 'amex', 'american express',
        'discover', 'bank card', 'debit card', 'card ending'
    ]
    
    kept_rows = []
    filtered_count = 0
    
    with open(input_file, 'r', newline='', encoding='utf-8') as infile:
        reader = csv.DictReader(infile)
        
        for row in reader:
            title = row.get('Title', '').strip()
            url = row.get('Url', '').strip()
            username = row.get('Username', '').strip()
            password = row.get('Password', '').strip()
            favorite = row.get('Favorite', '').strip().upper()
            archived = row.get('Archived', '').strip().upper()
            
            # Check if any field contains "coblinger@icloud.com" (case insensitive)
            row_text = ' '.join([title, url, username, password]).lower()
            if 'coblinger@icloud.com' in row_text:
                print(f"Filtered out coblinger@icloud.com entry: {title}")
                filtered_count += 1
                continue
            
            # Check if any field contains "CCARD" (case insensitive)
            if 'ccard' in row_text:
                print(f"Filtered out CCARD entry: {title}")
                filtered_count += 1
                continue
            
            # Skip if it's a credit card (check title for credit card patterns)
            is_credit_card = any(pattern.lower() in title.lower() for pattern in credit_card_patterns)
            if is_credit_card:
                print(f"Filtered out credit card: {title}")
                filtered_count += 1
                continue
            
            # MUST be favorited
            if favorite != 'TRUE':
                print(f"Filtered out not favorited: {title}")
                filtered_count += 1
                continue
            
            # Skip if missing title, username, password, or URL (all four must be present)
            if not title or not username or not password or not url:
                missing_fields = []
                if not title: missing_fields.append("title/name")
                if not username: missing_fields.append("username")
                if not password: missing_fields.append("password")
                if not url: missing_fields.append("URL")
                print(f"Filtered out missing {'/'.join(missing_fields)}: {title if title else '(no name)'}")
                filtered_count += 1
                continue
            
            # Keep this row
            kept_rows.append(row)
    
    # Write filtered results
    if kept_rows:
        with open(output_file, 'w', newline='', encoding='utf-8') as outfile:
            fieldnames = kept_rows[0].keys()
            writer = csv.DictWriter(outfile, fieldnames=fieldnames)
            writer.writeheader()
            writer.writerows(kept_rows)
    
    print(f"\nSummary:")
    print(f"Original entries: {len(kept_rows) + filtered_count}")
    print(f"Kept entries: {len(kept_rows)}")
    print(f"Filtered out: {filtered_count}")
    print(f"Output saved to: {output_file}")

if __name__ == "__main__":
    input_file = "/Users/oblinger/Desktop/1-Password update.csv"
    output_file = "/Users/oblinger/Desktop/1-Password update.csv"
    
    filter_1password_csv(input_file, output_file)