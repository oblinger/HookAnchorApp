#!/usr/bin/env python3
"""
Refactor config to move all structured actions from functions to actions section.
"""

import yaml
import sys
from collections import OrderedDict

def refactor_actions(config):
    """Move structured actions from functions to actions section."""
    
    if 'functions' not in config:
        return config
    
    if 'actions' not in config:
        config['actions'] = {}
    
    functions_to_keep = {}
    
    for name, definition in config['functions'].items():
        if name.startswith('action_'):
            # This is an action definition
            action_name = name[7:]  # Remove 'action_' prefix
            
            # Skip if already in actions
            if action_name in config['actions']:
                continue
            
            if isinstance(definition, dict):
                # Structured action - convert to actions format
                action = convert_structured_to_action(definition, action_name)
                config['actions'][action_name] = action
            elif isinstance(definition, str):
                # JavaScript action - keep in functions for now
                # But also create an action entry that references it
                if action_name not in config['actions']:
                    config['actions'][action_name] = {
                        'type': 'javascript',
                        'description': f"Execute {action_name} action",
                        'function': name  # Reference to the function
                    }
                functions_to_keep[name] = definition
        else:
            # Not an action, keep in functions
            functions_to_keep[name] = definition
    
    # Update functions section
    config['functions'] = functions_to_keep
    
    return config

def convert_structured_to_action(definition, action_name):
    """Convert a structured function definition to an action."""
    
    action = {}
    
    # Determine type based on function name
    fn = definition.get('fn', '')
    
    if fn == 'open_with':
        action['type'] = 'open_url' if action_name in ['chrome', 'safari', 'brave', 'firefox', 'work'] else 'open_app'
        action['description'] = f"Open with {definition.get('app', 'default app')}"
        
        if action['type'] == 'open_url':
            action['browser'] = definition.get('app')
            action['url'] = definition.get('arg', '{{arg}}')
        else:
            action['app'] = definition.get('app', '')
            action['arg'] = definition.get('arg', '{{arg}}')
            
    elif fn == 'launch_app':
        action['type'] = 'open_app'
        action['description'] = f"Launch application"
        action['app'] = definition.get('name', '{{arg}}')
        
    elif fn == 'open_url':
        action['type'] = 'open_url'
        action['description'] = f"Open URL"
        action['url'] = definition.get('url', '{{arg}}')
        
    elif fn == 'shell_sync':
        action['type'] = 'shell'
        action['description'] = f"Execute shell command"
        action['command'] = definition.get('command', '')
        action['sync'] = True
        
    else:
        # Generic conversion
        action['type'] = 'custom'
        action['description'] = f"Custom action: {action_name}"
        action['params'] = definition
    
    return action

def main():
    config_path = "/Users/oblinger/.config/hookanchor/config.yaml"
    
    print(f"Loading config from {config_path}")
    with open(config_path, 'r') as f:
        # Use safe_load to preserve order
        config = yaml.safe_load(f)
    
    print("Refactoring actions...")
    config = refactor_actions(config)
    
    # Count actions
    action_count = len(config.get('actions', {}))
    function_count = len(config.get('functions', {}))
    
    print(f"Actions in actions section: {action_count}")
    print(f"Functions remaining: {function_count}")
    
    # Create backup
    import shutil
    from datetime import datetime
    backup_path = f"{config_path}.backup_{datetime.now().strftime('%Y%m%d_%H%M%S')}"
    shutil.copy2(config_path, backup_path)
    print(f"Backup saved to {backup_path}")
    
    # Save refactored config
    with open(config_path, 'w') as f:
        yaml.dump(config, f, default_flow_style=False, sort_keys=False, allow_unicode=True)
    
    print("✓ Config refactored successfully")
    
    # Show summary of actions
    if 'actions' in config:
        action_types = {}
        for action in config['actions'].values():
            action_type = action.get('type', 'unknown')
            action_types[action_type] = action_types.get(action_type, 0) + 1
        
        print("\nAction types summary:")
        for action_type, count in sorted(action_types.items()):
            print(f"  {action_type}: {count}")

if __name__ == "__main__":
    main()