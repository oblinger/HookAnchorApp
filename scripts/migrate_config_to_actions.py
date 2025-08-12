#!/usr/bin/env python3
"""
Migrate config.yaml from legacy templates/keybindings to unified actions system.
"""

import yaml
import sys
import os
import shutil
from datetime import datetime
from collections import OrderedDict

def yaml_str_presenter(dumper, data):
    """Custom presenter for multiline strings."""
    if '\n' in data:
        return dumper.represent_scalar('tag:yaml.org,2002:str', data, style='|')
    return dumper.represent_scalar('tag:yaml.org,2002:str', data)

yaml.add_representer(str, yaml_str_presenter)

def migrate_templates_to_actions(templates):
    """Convert templates section to actions format."""
    if not templates:
        return {}
    
    actions = {}
    
    for name, template in templates.items():
        action = {
            'type': 'template',
            'description': f"Template: {template.get('name', name)}"
        }
        
        # Add key binding if present
        if 'key' in template:
            action['key'] = template['key']
        
        # Core template fields
        if 'name' in template:
            action['name'] = template['name']
        if 'action' in template:
            action['action'] = template['action']
        if 'arg' in template:
            action['arg'] = template['arg']
        if 'patch' in template:
            action['patch'] = template['patch']
        if 'flags' in template:
            action['flags'] = template['flags']
            
        # Optional fields
        if template.get('edit'):
            action['edit'] = True
        if 'file' in template:
            action['file'] = template['file']
        if 'contents' in template:
            action['contents'] = template['contents']
        if 'grab' in template:
            action['grab'] = template['grab']
        if template.get('validate_previous_folder'):
            action['validate_previous_folder'] = True
        if template.get('file_rescan'):
            action['file_rescan'] = True
            
        actions[name] = action
    
    return actions

def migrate_keybindings_to_actions(keybindings):
    """Convert keybindings to popup actions."""
    if not keybindings:
        return {}
    
    actions = {}
    
    # Mapping of keybinding names to popup actions
    keybinding_mappings = {
        'exit_app': ('exit', 'Exit application'),
        'execute': ('execute', 'Execute selected command'),
        'navigate_up': ('navigate', 'Navigate up', {'dx': 0, 'dy': -1}),
        'navigate_down': ('navigate', 'Navigate down', {'dx': 0, 'dy': 1}),
        'navigate_left': ('navigate', 'Navigate left', {'dx': -1, 'dy': 0}),
        'navigate_right': ('navigate', 'Navigate right', {'dx': 1, 'dy': 0}),
        'show_folder': ('show_folder', 'Open first matching folder'),
        'edit_command': ('edit_command', 'Edit selected command'),
        'force_rebuild': ('rebuild', 'Force rebuild and rescan'),
        'show_help': ('show_help', 'Show keyboard shortcuts'),
        'cycle_patch': ('cycle_patch', 'Cycle through patches'),
        'search_patches': ('search_patches', 'Search patches'),
        'grab': ('grab', 'Grab application context'),
    }
    
    for binding_name, key in keybindings.items():
        if binding_name in keybinding_mappings:
            mapping = keybinding_mappings[binding_name]
            popup_action = mapping[0]
            description = mapping[1]
            
            action = {
                'type': 'popup',
                'description': description,
                'key': key,
                'popup_action': popup_action
            }
            
            # Add extra params if present (for navigate actions)
            if len(mapping) > 2:
                action.update(mapping[2])
            
            # Use a clearer name for the action
            action_name = f"kb_{binding_name}"
            actions[action_name] = action
    
    return actions

def merge_actions(existing_actions, template_actions, keybinding_actions):
    """Merge all actions, preserving existing ones."""
    merged = {}
    
    # Start with template actions
    merged.update(template_actions)
    
    # Add keybinding actions (with kb_ prefix to avoid conflicts)
    merged.update(keybinding_actions)
    
    # Override with any existing actions (user customizations)
    if existing_actions:
        merged.update(existing_actions)
    
    return merged

def load_config(config_path):
    """Load the config file."""
    with open(config_path, 'r') as f:
        return yaml.safe_load(f)

def save_config(config, config_path):
    """Save the config file."""
    with open(config_path, 'w') as f:
        yaml.dump(config, f, default_flow_style=False, sort_keys=False, 
                  allow_unicode=True, width=120)

def backup_config(config_path):
    """Create a backup of the config file."""
    timestamp = datetime.now().strftime("%Y%m%d_%H%M%S")
    backup_path = f"{config_path}.backup_{timestamp}"
    shutil.copy2(config_path, backup_path)
    return backup_path

def main():
    config_path = os.path.expanduser("~/.config/hookanchor/config.yaml")
    
    if not os.path.exists(config_path):
        print(f"Error: Config file not found at {config_path}")
        sys.exit(1)
    
    print(f"Loading config from {config_path}")
    config = load_config(config_path)
    
    # Check if already migrated
    if 'actions' in config and not 'templates' in config:
        print("✓ Config already uses unified actions system")
        return
    
    # Create backup
    print("Creating backup...")
    backup_path = backup_config(config_path)
    print(f"✓ Backup saved to {backup_path}")
    
    # Migrate templates to actions
    template_actions = {}
    if 'templates' in config:
        print(f"Migrating {len(config['templates'])} templates to actions...")
        template_actions = migrate_templates_to_actions(config['templates'])
        print(f"✓ Converted {len(template_actions)} templates")
    
    # Migrate keybindings to actions
    keybinding_actions = {}
    if 'keybindings' in config:
        print(f"Migrating {len(config['keybindings'])} keybindings to actions...")
        keybinding_actions = migrate_keybindings_to_actions(config['keybindings'])
        print(f"✓ Converted {len(keybinding_actions)} keybindings")
    
    # Merge with existing actions
    existing_actions = config.get('actions', {})
    merged_actions = merge_actions(existing_actions, template_actions, keybinding_actions)
    
    # Update config
    config['actions'] = merged_actions
    
    # Remove old sections
    if 'templates' in config:
        del config['templates']
        print("✓ Removed legacy templates section")
    
    # Keep keybindings for now (for backward compatibility during transition)
    # if 'keybindings' in config:
    #     del config['keybindings']
    #     print("✓ Removed legacy keybindings section")
    
    # Save migrated config
    print(f"Saving migrated config to {config_path}")
    save_config(config, config_path)
    print(f"✓ Migration complete! {len(merged_actions)} total actions")
    
    # Show summary
    action_types = {}
    for action in merged_actions.values():
        action_type = action.get('type', 'unknown')
        action_types[action_type] = action_types.get(action_type, 0) + 1
    
    print("\nAction types summary:")
    for action_type, count in sorted(action_types.items()):
        print(f"  {action_type}: {count}")
    
    print(f"\nYour original config is backed up at:\n  {backup_path}")
    print("\nIf something doesn't work correctly, you can restore with:")
    print(f"  cp {backup_path} {config_path}")

if __name__ == "__main__":
    main()