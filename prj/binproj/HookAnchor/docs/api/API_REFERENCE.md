# HookAnchor Rust API Reference

This document provides a comprehensive overview of all modules, structs, enums, and public functions in the HookAnchor Rust codebase. The codebase is well-structured with clear separation between core business logic and UI concerns.

## Core Architecture

HookAnchor is organized into the following main architectural layers:

1. **Core Business Logic** (`src/core/`) - Pure business logic, data models, and configuration
2. **UI Layer** (`src/ui/`) - User interface components and rendering logic  
3. **System Integration** (`src/`) - Command execution, scanning, grabbing, and system interaction
4. **JavaScript Runtime** - Embedded JavaScript engine for extensible command processing

## Core Modules (`src/core/`)

### actions.rs
| Function/Struct | Description |
|---|---|
| `get_action()` | Determines action type for file path based on extension and naming conventions |
| `is_markdown_anchor()` | Checks if markdown file is an "anchor" (base name matches parent folder) |
| `is_executable()` | Checks if file has executable permissions |
| `get_default_patch_for_action()` | Returns default patch for a given action type |

### application_state.rs
| Function/Struct | Description |
|---|---|
| `ApplicationState` | Global application state spanning GUI and CLI modes |
| `minimal()` | Creates minimal state for immediate GUI startup |
| `new()` | Creates state by loading from files with error handling |
| `new_with_search()` | Creates state with initial search text |
| `update_search()` | Updates search text and recomputes filtered commands |
| `check_and_apply_alias()` | Applies rewrite command aliases when word separators are used |
| `get_display_commands()` | Gets commands with submenu information for display |

### commands.rs
| Function/Struct | Description |
|---|---|
| `Command` | Core data structure representing a parsed command |
| `CommandTarget` | Enum for command execution targets (Command/Alias) |
| `Patch` | Associates dispatcher with linked command |
| `get_patch()` | Case-insensitive patch lookup |
| `get_absolute_file_path()` | Returns absolute file path with tilde expansion |
| `filter_commands()` | Filters commands based on search query with fuzzy matching |
| `merge_similar_commands()` | Merges commands ending with "..." |
| `load_commands_with_data()` | Comprehensive command loading with all processing steps |
| `infer_patch()` | Infers appropriate patch for a command |
| `get_patch_path()` | Traverses patch hierarchy from command to orphans root |

### config.rs
| Function/Struct | Description |
|---|---|
| `Config` | Main configuration structure from YAML files |
| `PopupSettings` | Popup window configuration settings |
| `LauncherSettings` | Launcher behavior settings |
| `ScannerSettings` | Scanner behavior settings |
| `load_config_with_error()` | Loads configuration with detailed error reporting |
| `get_config_file_path()` | Returns path to YAML config file |
| `is_key_bound_to_action()` | Checks if key is bound to specific action |
| `get_action_for_key()` | Gets action bound to key (simple keys) |
| `get_action_for_key_with_modifiers()` | Gets action bound to key with modifier support |
| `get_template_for_key()` | Gets template bound to key (simple keys) |
| `get_template_for_key_with_modifiers()` | Gets template bound to key with modifier support |

### key_parsing.rs
| Function/Struct | Description |
|---|---|
| `KeyChord` | Represents parsed key combination with modifiers |
| `parse_key_string()` | Parses key strings (ASCII, named keys, chords like "Cmd+C") |
| `key_matches()` | Checks if key event matches configuration string |
| `ascii_to_key_name()` | Converts ASCII characters to standard key names |
| `normalize_modifier()` | Normalizes modifier names (cmd/command → Cmd) |

### state.rs  
| Function/Struct | Description |
|---|---|
| `AppState` | Persistent application state between runs |
| `load_state()` | Loads state from JSON file |
| `save_state()` | Saves state to JSON file |
| `save_state_with_build_time()` | Updates build time in state |
| `save_last_executed_command()` | Updates last executed command for template system |
| `save_server_pid()` | Updates server PID |
| `clear_server_pid()` | Clears server PID |

### sys_data.rs
| Function/Struct | Description |
|---|---|
| `SysData` | System-wide data structure with config, commands, patches |
| `get_sys_data()` | Gets cached system data, loading if necessary |
| `get_config()` | Gets just configuration without full data loading |
| `clear_sys_data()` | Clears cache, forcing reload from disk |
| `load_data()` | Comprehensive data loading with caching |

### template_creation.rs
| Function/Struct | Description |
|---|---|
| `Template` | Template for creating new commands with variable expansion |
| `TemplateContext` | Context for template variable expansion with 15+ variable types |
| `create_command_from_template()` | Creates command from template and context |
| `process_template()` | Processes template and creates associated files |
| `extract_folder_from_path()` | Extracts folder path from file path |
| `add_datetime_variables()` | Adds date/time variables (YYYY, MM, DD, etc.) |

## UI Modules (`src/ui/`)

### command_editor.rs
| Function/Struct | Description |
|---|---|
| `CommandEditor` | Dialog for editing/creating commands |
| `CommandEditorResult` | Result enum for editor actions |
| `edit_command()` | Opens editor with existing command or new command |
| `open_with_command()` | Opens editor with pre-filled command |
| `update()` | Handles UI updates and user input |
| `prepare_save_command()` | Prepares command for saving with validation |
| `delete_original_command()` | Deletes original command from list |

### dialog.rs
| Function/Struct | Description |
|---|---|
| `Dialog` | Generic dialog system with spec string parsing |
| `DialogElement` | Enum for different dialog element types |
| `DialogRow` | Container for dialog elements in a row |
| `show_error()` | Shows error dialog with message |
| `update()` | Handles dialog rendering and user interaction |
| `take_result()` | Gets dialog result and clears internal state |

### layout.rs
| Function/Struct | Description |
|---|---|
| `DisplayLayout` | Manages command arrangement for display |
| `LayoutArrangement` | Enum for single/multi-column arrangements |
| `Selection` | Tracks selection position in visual and logical space |
| `Direction` | Navigation direction enum |
| `get_command_at_position()` | Gets command at visual position |
| `navigate()` | Moves selection in given direction |
| `visual_to_index()` | Converts visual position to logical index |
| `index_to_visual()` | Converts logical index to visual position |

### popup.rs
| Function/Struct | Description |
|---|---|
| `AnchorSelector` | Main popup window application state |
| `WindowSizeMode` | Enum for different window sizing modes |
| `LoadingState` | Enum for deferred initialization states |
| `run_gui_with_prompt()` | Main entry point for GUI mode |
| `handle_template_create_named()` | Handles template creation with variable expansion |
| `execute_grab()` | Executes grabber with countdown |
| `show_error_dialog()` | Shows error dialog to user |
| `key_generates_text()` | Checks if key press generates text |

### popup_state.rs
| Function/Struct | Description |
|---|---|
| `PopupState` | Core popup state separated from UI rendering |
| `new_minimal()` | Creates minimal state for early UI display |
| `update_search()` | Updates search and recomputes commands |
| `navigate()` | Navigates selection |
| `get_selected_command()` | Gets currently selected command |
| `get_command_at_position()` | Gets command at grid position |
| `set_selection_to_position()` | Sets selection to specific position |

## System Integration Modules (`src/`)

### builtin_fns.rs
| Function/Struct | Description |
|---|---|
| `setup_builtin_functions()` | Registers all built-in functions in environment |
| `launch_app` | Built-in function for launching applications |
| `open_with` | Built-in function for opening files with specific apps |
| `open_url` | Built-in function for opening URLs |
| `shell` | Built-in function for shell command execution |
| `javascript` | Built-in function for JavaScript code execution |

### business_logic.rs
| Function/Struct | Description |
|---|---|
| `run_business_script()` | Executes business logic script from embedded scripts |
| `scan_markdown_commands()` | Scans for markdown-based commands |
| `activate_anchor()` | Activates anchor project using JavaScript |

### cmd.rs
| Function/Struct | Description |
|---|---|
| `run_command_line_mode()` | Main entry point for CLI mode |
| `print_help()` | Prints command-line help |
| `handle_hook_url()` | Handles hook:// URL processing |
| `run_match_command()` | Searches and displays matching commands |
| `run_execute_top_match()` | Executes top matching command with state saving |
| `run_infer_patches()` | Shows patch inference changes |
| `run_rescan_command()` | Rescans filesystem with verbose output |
| `run_rebuild_command()` | Full system rebuild (server restart + rescan) |

### command_launcher.rs
| Function/Struct | Description |
|---|---|
| `LauncherConfig` | Configuration for command launcher |
| `LauncherSettings` | Launcher behavior settings |
| `LauncherError` | Error types for launcher operations |
| `launch()` | Main entry point for command launching |
| `parse_command_line()` | Parses command line into action and arguments |
| `load_config()` | Loads launcher configuration from YAML |

### command_server.rs
| Function/Struct | Description |
|---|---|
| `CommandServer` | Background server for command execution |
| `CommandRequest` | Request structure for command execution |
| `CommandResponse` | Response structure with execution results |
| `CommandClient` | Client for communicating with command server |
| `execute_via_server()` | Executes commands via background server |
| `start_persistent_server()` | Starts long-running command server |
| `execute_command_with_depth()` | Executes commands with alias resolution and depth tracking |

### command_server_management.rs
| Function/Struct | Description |
|---|---|
| `start_server_if_needed()` | Starts server with session-based caching |
| `is_process_alive()` | Checks if process with PID is running |
| `start_server_via_terminal()` | Starts server in Terminal window |
| `kill_existing_server()` | Kills existing server process |
| `reset_server_check()` | Resets server status check cache |

### dispatcher.rs
| Function/Struct | Description |
|---|---|
| `main()` | Main dispatcher routing execution based on launch context |
| `launch_popup()` | Launches GUI popup mode |
| `handle_hook_url()` | Processes hook:// URLs via server |

### error_display.rs
| Function/Struct | Description |
|---|---|
| `init_error_queue()` | Initializes global error queue |
| `queue_user_error()` | Queues error for display to user |
| `take_next_error()` | Takes next error from queue |
| `has_errors()` | Checks if errors are queued |
| `clear_errors()` | Clears all errors from queue |

### eval.rs
| Function/Struct | Description |
|---|---|
| `Environment` | JavaScript execution environment with variables |
| `EvalError` | Error types for evaluation operations |
| `new()` | Creates new environment with JavaScript runtime |
| `eval()` | Evaluates YAML-defined actions |

### grabber.rs
| Function/Struct | Description |
|---|---|
| `AppContext` | Information about active application |
| `GrabberRule` | Rule matching against app context |
| `GrabResult` | Result of grab operation |
| `capture_active_app()` | Captures active application context |
| `match_grabber_rules()` | Matches context against rules |
| `grab()` | Main grab function combining capture and matching |

### js_runtime.rs
| Function/Struct | Description |
|---|---|
| `execute_business_logic()` | Executes JavaScript in business logic context |
| `setup_all_builtins()` | Sets up all JavaScript built-in functions |
| `setup_config_access()` | Sets up configuration access functions |
| Built-in JS functions | 50+ built-in functions for file ops, logging, system control |

### lib.rs
| Function/Struct | Description |
|---|---|
| `init_binary_path()` | Initializes global binary path for process spawning |
| `get_binary_path()` | Gets path of currently running binary |
| `get_listed_actions()` | Gets configured actions for command editor dropdown |
| Module exports | Re-exports all major modules and types |

### popup_main.rs
| Function/Struct | Description |
|---|---|
| `main()` | Top-level application coordinator |
| Application routing | Determines GUI vs CLI mode based on arguments |
| Setup integration | Runs setup assistant on first launch |

### process_monitor.rs
| Function/Struct | Description |
|---|---|
| `check_system_health()` | Monitors system health after command execution |
| Health monitoring | Tracks process performance and system resources |

### scanner.rs
| Function/Struct | Description |
|---|---|
| `scan_verbose()` | Scans filesystem with detailed output |
| `startup_check()` | Quick startup validation of commands |
| Filesystem scanning | Recursively scans configured roots for commands |

### setup_assistant.rs
| Function/Struct | Description |
|---|---|
| `SetupAssistant` | Handles installation and configuration |
| `check_and_run_setup()` | Checks if setup needed and runs if required |
| `run_setup()` | Executes setup process |
| `install_hookanchor()` | Installs HookAnchor components |
| `uninstall_hookanchor()` | Uninstalls HookAnchor components |

### url_handler.rs
| Function/Struct | Description |
|---|---|
| URL scheme handling | Handles hook:// URL scheme registration |
| Apple Events | Processes URLs via Apple Events (not command line) |

### utils.rs
| Function/Struct | Description |
|---|---|
| `debug_log()` | Debug logging to configured file |
| `verbose_log()` | Conditional verbose logging |
| `expand_tilde()` | Expands ~ in paths to home directory |
| `launch_app_with_arg()` | Launches macOS app with optional argument |
| `open_url()` | Opens URL in default browser |
| `shell_simple()` | Simple shell command execution |
| `shell_login()` | Shell execution with login environment |

### vault.rs
| Function/Struct | Description |
|---|---|
| Obsidian integration | Functions for working with Obsidian vaults |
| Vault operations | Opening and managing vault files |

## Key Observations and Potential Redundancies

### **Areas of Potential Redundancy:**

1. **Command Loading:** Multiple functions load commands (`load_commands()`, `load_commands_with_data()`, `load_commands_for_inference()`) with different processing steps
2. **Error Handling:** Multiple error types (`EvalError`, `LauncherError`) with overlapping functionality
3. **Configuration Loading:** Config loading spread across multiple modules with different error handling approaches
4. **Shell Execution:** Multiple shell execution functions (`shell_simple()`, `shell_login()`, `shell()`, `shell_sync()`) with different behaviors
5. **State Management:** Both `ApplicationState` and `PopupState` manage similar command filtering and display logic

### **Well-Designed Separations:**

1. **Core/UI Separation:** Clean separation between business logic (`core/`) and UI (`ui/`)
2. **Command Server:** Dedicated background server architecture for consistent command execution
3. **Template System:** Comprehensive template creation system with variable expansion
4. **JavaScript Integration:** Rich JavaScript runtime with 50+ built-in functions
5. **Error Display System:** Global error queue for non-UI components to communicate with users
6. **Key Parsing System:** Unified key parsing with chord support (simple keys + modifiers)

### **Architecture Strengths:**

- **Modular Design:** Clear module boundaries with specific responsibilities
- **Extensibility:** JavaScript runtime allows user customization without recompilation
- **Cross-Platform Considerations:** Proper tilde expansion, path handling, and environment management
- **Robust Error Handling:** Multiple error types and comprehensive error reporting
- **Performance:** Caching mechanisms and efficient command filtering
- **Template System:** Rich variable expansion with 15+ variable types including previous command context

### **Recent Enhancements:**

- **Template Creation System:** Complete template system with variable expansion, file creation, and editor integration
- **Previous Command Variables:** Template access to previously executed command (name, path, patch, folder)
- **Chord Key Support:** Support for modifier key combinations like "Cmd+C", "Ctrl+Shift+A"
- **Enhanced State Management:** Proper state persistence for template system functionality
- **Improved Error Display:** Dynamic error dialog sizing and better user feedback

This codebase demonstrates mature Rust practices with good separation of concerns, comprehensive error handling, and thoughtful architecture for a desktop application with both GUI and CLI interfaces.