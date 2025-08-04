# HookAnchor Documentation

HookAnchor is a universal command launcher for macOS that provides instant access to applications, files, URLs, and custom actions through a keyboard-driven popup interface.

## Core Documentation

### User Documentation
- [**CONFIGURATION.md**](CONFIGURATION.md) - Complete configuration guide with settings, keybindings, templates, and examples
- [**docs.md**](docs.md) - Legacy configuration documentation (consider consolidating)

### API & Development
- [**api/API_REFERENCE.md**](api/API_REFERENCE.md) - Complete module and function reference for all Rust code
- [**api/JAVASCRIPT_API.md**](api/JAVASCRIPT_API.md) - JavaScript function API for config file scripting

### Feature Documentation
- [**TEMPLATE_SYSTEM_SUMMARY.md**](TEMPLATE_SYSTEM_SUMMARY.md) - Template system overview and capabilities
- [**URL_HANDLING.md**](URL_HANDLING.md) - Critical URL scheme handling (read before modifying URL code)

### Technical Documentation
- [**technical/SHELL_ENVIRONMENT_FIX.md**](technical/SHELL_ENVIRONMENT_FIX.md) - Server architecture and environment handling
- [**technical/RUST_BEST_PRACTICES.md**](technical/RUST_BEST_PRACTICES.md) - Coding standards and patterns
- [**technical/launcher.md**](technical/launcher.md) - Launcher system architecture

## Development Notes

### Current Features
- [**dev-notes/ALIAS_FEATURE_SUMMARY.md**](dev-notes/ALIAS_FEATURE_SUMMARY.md) - Alias system implementation
- [**dev-notes/POLISH_SUMMARY.md**](dev-notes/POLISH_SUMMARY.md) - UI polish and improvements
- [**dev-notes/USER_CUSTOMIZATION.md**](dev-notes/USER_CUSTOMIZATION.md) - Customization capabilities

### Project Management
- [**dev-notes/PROJECT_STATUS.md**](dev-notes/PROJECT_STATUS.md) - Current project status and roadmap
- [**dev-notes/TESTING.md**](dev-notes/TESTING.md) - Testing strategies and procedures
- [**TEMPLATE_CREATION_PLAN.md**](TEMPLATE_CREATION_PLAN.md) - Template system development plan

### Legacy/Archive
- [**dev-notes/NEW_LAUNCHER_INTEGRATION.md**](dev-notes/NEW_LAUNCHER_INTEGRATION.md) - Launcher integration notes
- [**dev-notes/STATE_CLEANUP_PROPOSAL.md**](dev-notes/STATE_CLEANUP_PROPOSAL.md) - State management proposals
- [**dev-notes/CLAUDE.md**](dev-notes/CLAUDE.md) - Development session notes
- [**inference.md**](inference.md) - Patch inference system (legacy)
- [**project-notes.md**](project-notes.md) - Miscellaneous project notes

## Installation & Setup

- [**installation/installer-prd.md**](installation/installer-prd.md) - Production installer requirements
- [**installation/installer-safety.md**](installation/installer-safety.md) - Installer security considerations

## Integrations

- [**integrations/karabiner.md**](integrations/karabiner.md) - Karabiner-Elements integration setup

## Quick Start

1. **Configuration**: Start with [CONFIGURATION.md](CONFIGURATION.md) for complete setup guide
2. **Templates**: Learn about creating dynamic commands in the templates section
3. **Development**: Use [api/API_REFERENCE.md](api/API_REFERENCE.md) to understand the codebase
4. **Troubleshooting**: Check [URL_HANDLING.md](URL_HANDLING.md) for URL scheme issues

## File Cleanup Recommendations

The following files may be candidates for consolidation or removal:
- `docs.md` - Superseded by CONFIGURATION.md
- `inference.md` - Legacy patch inference documentation
- `project-notes.md` - Miscellaneous notes that could be organized
- Some dev-notes may be outdated and could be archived

---

For the most up-to-date information, always refer to the main configuration file at `~/.config/hookanchor/config.yaml` and this documentation.