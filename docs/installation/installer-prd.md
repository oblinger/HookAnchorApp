# HookAnchor Installer PRD

## Overview
Hybrid approach: DMG distribution with first-run Setup Assistant

## Core Components

### 1. Distribution
- **Format**: DMG file containing HookAnchor.app
- **Installation**: Drag-and-drop to Applications folder

### 2. First-Run Setup Assistant
Launches automatically on first run to handle configuration.

#### 2.1 Dependency Check
- Detect if Karabiner-Elements is installed
- If missing: Show download link and installation guide
- Verify installation before proceeding

#### 2.2 Configuration Setup
- Create `~/.config/hookanchor/` directory structure
- Install default `config.yaml` 
- Preserve existing config if found (upgrade scenario)

#### 2.3 Karabiner Integration
- Generate complex modification JSON
- Install to `~/.config/karabiner/assets/complex_modifications/hookanchor.json`
- Default shortcut: Cmd+Space
- Detect conflicts and offer alternatives

### 3. Installation Flow
```
1. Download HookAnchor.dmg
2. Drag HookAnchor.app to Applications
3. Launch HookAnchor → Setup Assistant opens
4. Check/install dependencies
5. Create configuration
6. Set up keyboard shortcut
7. Complete setup → Launch main app
```

### 4. Key Files
```
/Applications/HookAnchor.app
~/.config/hookanchor/config.yaml
~/.config/hookanchor/commands.txt
~/.config/karabiner/assets/complex_modifications/hookanchor.json
```

### 5. Karabiner Complex Modification
```json
{
  "title": "HookAnchor",
  "rules": [{
    "description": "Launch HookAnchor with Cmd+Space",
    "manipulators": [{
      "type": "basic",
      "from": {
        "key_code": "spacebar",
        "modifiers": { "mandatory": ["command"] }
      },
      "to": [{
        "shell_command": "/Applications/HookAnchor.app/Contents/MacOS/hookanchor"
      }]
    }]
  }]
}
```

### 6. Implementation Tasks
- [ ] Create DMG build script
- [ ] Build Setup Assistant (Swift or Rust)
- [ ] Implement Karabiner detection
- [ ] Generate default config.yaml
- [ ] Create Karabiner JSON generator
- [ ] Add first-run detection
- [ ] Handle upgrade scenarios
- [ ] Test on macOS 11.0+

### 7. Success Metrics
- Installation under 2 minutes
- Zero-config working state
- Preserve user data on upgrade
- Clear error handling