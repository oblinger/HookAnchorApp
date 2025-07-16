#!/bin/bash

# Migration script to convert obs commands to markdown with absolute paths
# This converts relative vault paths to absolute paths for the new markdown action

set -e

COMMANDS_FILE="/Users/oblinger/.config/hookanchor/commands.txt"
CONFIG_FILE="/Users/oblinger/.config/hookanchor/config.yaml"
BACKUP_FILE="/Users/oblinger/.config/hookanchor/commands.txt.pre_markdown_migration"

echo "🔄 HookAnchor: Migrating obs commands to markdown action"
echo "=================================================="

# Check if files exist
if [[ ! -f "$COMMANDS_FILE" ]]; then
    echo "❌ Commands file not found: $COMMANDS_FILE"
    exit 1
fi

if [[ ! -f "$CONFIG_FILE" ]]; then
    echo "❌ Config file not found: $CONFIG_FILE"
    exit 1
fi

# Extract vault path from config (handle both quoted and unquoted values)
VAULT_PATH=$(grep -A10 "launcher_settings:" "$CONFIG_FILE" | grep "obsidian_vault_path:" | sed 's/.*obsidian_vault_path: *["\x27]*\([^"\x27]*\)["\x27]*.*/\1/' | head -1)

if [[ -z "$VAULT_PATH" ]]; then
    echo "❌ Could not find obsidian_vault_path in config"
    exit 1
fi

# Expand tilde in vault path
VAULT_PATH=$(eval echo "$VAULT_PATH")

echo "✅ Vault path found: $VAULT_PATH"

# Count obs commands before migration
OBS_COUNT=$(grep -c ": obs " "$COMMANDS_FILE" || true)
echo "📊 Found $OBS_COUNT obs commands to migrate"

if [[ "$OBS_COUNT" -eq 0 ]]; then
    echo "✅ No obs commands found - migration not needed"
    exit 0
fi

# Create backup
echo "💾 Creating backup: $BACKUP_FILE"
cp "$COMMANDS_FILE" "$BACKUP_FILE"

# Show commands to be migrated
echo ""
echo "📋 Commands to be migrated:"
echo "-------------------------"
grep ": obs " "$COMMANDS_FILE"
echo "-------------------------"
echo ""

# Create temporary file for migration
TEMP_FILE="/tmp/commands_migration.txt"
cp "$COMMANDS_FILE" "$TEMP_FILE"

# Process each obs command
while IFS= read -r line; do
    if [[ "$line" =~ (.+)\ :\ obs\ (.+)\;\ (.+) ]]; then
        COMMAND_NAME="${BASH_REMATCH[1]}"
        FLAGS="${BASH_REMATCH[2]}"
        RELATIVE_PATH="${BASH_REMATCH[3]}"
        
        # Convert relative path to absolute
        ABSOLUTE_PATH="$VAULT_PATH/$RELATIVE_PATH"
        
        # Create new line with markdown action
        NEW_LINE="$COMMAND_NAME : markdown $FLAGS; $ABSOLUTE_PATH"
        
        echo "🔄 Migrating: $COMMAND_NAME"
        echo "   From: obs; $RELATIVE_PATH"
        echo "   To:   markdown; $ABSOLUTE_PATH"
        
        # Replace in temp file
        sed -i '' "s|$line|$NEW_LINE|" "$TEMP_FILE"
    fi
done < <(grep ": obs " "$COMMANDS_FILE")

# Verify migration
NEW_MARKDOWN_COUNT=$(grep -c ": markdown " "$TEMP_FILE" || true)
REMAINING_OBS_COUNT=$(grep -c ": obs " "$TEMP_FILE" || true)

echo ""
echo "📊 Migration Results:"
echo "   Original obs commands: $OBS_COUNT"
echo "   New markdown commands: $NEW_MARKDOWN_COUNT"
echo "   Remaining obs commands: $REMAINING_OBS_COUNT"

if [[ "$NEW_MARKDOWN_COUNT" -eq "$OBS_COUNT" ]] && [[ "$REMAINING_OBS_COUNT" -eq 0 ]]; then
    echo "✅ Migration successful - replacing commands file"
    mv "$TEMP_FILE" "$COMMANDS_FILE"
    
    echo ""
    echo "🎉 Migration complete!"
    echo "📋 Migrated commands:"
    echo "-------------------"
    grep ": markdown " "$COMMANDS_FILE" | while read -r line; do
        echo "  ✅ $line"
    done
    echo "-------------------"
    echo ""
    echo "💾 Backup saved at: $BACKUP_FILE"
    echo "🔄 You can now rebuild HookAnchor to use the new commands"
    
else
    echo "❌ Migration verification failed"
    echo "   Expected $OBS_COUNT markdown commands, got $NEW_MARKDOWN_COUNT"
    echo "   Expected 0 remaining obs commands, got $REMAINING_OBS_COUNT"
    rm "$TEMP_FILE"
    exit 1
fi