# Changelog
The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.10.0] - 2025-01-20

### Changed
- **BREAKING**: Refactored template variable system from flat variables to object-based access
  - Changed from `{{previous_name}}` to `{{last_executed.name}}`
  - Changed from `{{YYYY}}` to `{{date.year}}`
  - All variables now use clean object notation for better organization
- Replaced volatile "previous" command tracking with stable "last_executed" from persistent state
- Converted JavaScript-based actions to first-class action types
  - Changed from `type: javascript` with `code: action_doc` to simply `type: doc`
  - Actions now have cleaner definitions with just `description` and `type`
- Removed redundant `tmux_activate` and `activate_anchor` action types
  - Their functionality can be achieved through regular `cmd` actions

### Added
- JavaScript evaluation support for template variable expansion
- `use_existing` flag for templates to edit existing commands vs creating new ones
- Better error messages including file paths for template operations

### Fixed
- Fixed alias template to use reliable `{{last_executed.name}}` instead of volatile `{{previous.name}}`
- Popup window position persistence improvements

### Removed
- Removed legacy flat variable names (last_executed_name, selected_name, etc.)
- Removed backward compatibility variables
- Removed duplicate action handlers (1password, slack, tmux, type_text)

## [0.9.0] - 2025-01-20

### Changed
- Complete Actions-based architecture refactor
  - All functionality now routes through unified Action system
  - Consistent action handling between CLI and GUI
  - Template system integration with actions

### Added
- Unified actions system with centralized configuration in config.yaml
- Template actions for creating new commands and files
- Execute method to server for action routing consistency
- JavaScript evaluation in templates

### Fixed
- Command editor cursor position bug
- Auto-add U flag to user-edited scanner commands

## [0.8.0] - 2025-01-10

### Changed
- Implemented safe URL handling architecture
  - URL schemes now handled via Apple Events, not command line arguments
  - Prevents system lockups from incorrect URL handling

### Added
- Comprehensive URL handling documentation
- Safety checks for URL processing

## [0.7.0] - 2025-01-07

### Changed
- Major supervisor architecture improvements
- Dispatcher architecture implementation
- Refactored executor into eval-based dispatcher with JavaScript runtime

### Added
- Clean server output handling
- Improved tmux session management
- Better error handling and logging

### Fixed
- tmux PATH issues in anchor action
- tmux session switching improvements

## [0.6.0] - 2025-01-06

### Added
- Comprehensive uninstall functionality with config preservation
- Automatic version archiving in build process
- Inverse file operation with perfect round-trip conversion

### Changed
- Relocated project from kmr/prj to proj directory
- Improved build distribution script

## [0.5.0] - 2025-01-05

### Added
- Idle timeout to hide popup after inactivity
- Improved popup keyboard handlers

### Fixed
- Popup position issues
- tmux activation handling
