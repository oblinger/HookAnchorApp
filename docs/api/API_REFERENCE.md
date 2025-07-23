k# API Reference

## src/

### ha.rs
| Function | Description |
|---|---|
| `main() -> Result<(), eframe::Error>` | Application entry point, routes to GUI or CLI mode |

### lib.rs
| Function | Description |
|---|---|
| `get_listed_actions() -> Vec<String>` | Gets configured actions for command editor dropdown |

## src/core/

### application_state.rs
| Function | Description |
|---|---|
| `ApplicationState` | Global application state spanning GUI and CLI modes |
| `new() -> Self` | Create new application state by loading from files |
| `new_with_search(String) -> Self` | Create with initial search text |
| `update_search(String)` | Update search text and recompute filtered commands |
| `get_commands() -> &[Command]` | Get reference to all commands |
| `get_commands_mut() -> &mut Vec<Command>` | Get mutable reference to commands |
| `set_commands(Vec<Command>)` | Update command list for deferred scanner updates |
| `get_display_commands() -> (Vec<Command>, bool, Option<String>, usize)` | Get display commands with submenu information |
| `get_hint_text() -> String` | Get hint text for search box |
| `update_window_position((f32, f32))` | Update window position in app state |
| `get_window_position() -> Option<(f32, f32)>` | Get saved window position |
| `is_separator_command(&Command) -> bool` | Check if command is a separator |
| `check_and_apply_alias()` | Apply alias if search text matches rewrite command |

### commands.rs
| Function | Description |
|---|---|
| `Command` | Command with group, command, action, arg, flags, full_line |
| `CommandTarget` | Command execution target (Command or Alias) |
| `load_commands() -> Vec<Command>` | Load commands from file |
| `save_commands_to_file(&[Command]) -> Result<(), Error>` | Save commands to file |
| `add_command(Command, &mut Vec<Command>) -> Result<(), Error>` | Add new command to list |
| `delete_command(&str, &mut Vec<Command>) -> Result<(), Error>` | Delete command by name |
| `parse_command_line(&str) -> Result<Command, String>` | Parse command line to Command struct |
| `filter_commands(&[Command], &str, usize, bool) -> Vec<Command>` | Filter commands with fuzzy matching |
| `command_matches_query(&str, &str) -> bool` | Check if command matches query |
| `merge_similar_commands(Vec<Command>, &Config) -> Vec<Command>` | Merge similar commands ending with ... |
| `execute_command(&Command) -> CommandTarget` | Execute command (handles aliases) |
| `get_display_commands(&[Command], &str, &Config, usize) -> Vec<Command>` | Get commands for display with limits |
| `split_commands(&[Command], &str, &str) -> Vec<Command>` | Split into submenu sections |
| `get_current_submenu_prefix_from_commands(&[Command], &str, &str) -> Option<String>` | Get submenu prefix from commands |
| `migrate_commands_to_new_format(&mut [Command])` | Migrate commands to new format |
| `get_command_prefix(&str, &str) -> String` | Get command prefix based on separators |
| `get_flag(char) -> Option<String>` | Get flag value by key |
| `set_flag(char, &str)` | Set flag value |
| `remove_flag(char)` | Remove flag by key |
| `update_full_line()` | Update full_line field from components |
| `to_new_format() -> String` | Convert command to new format string |
| `get_commands_file_path() -> PathBuf` | Get path to commands.txt |
| `backup_commands_file() -> Result<(), Error>` | Create backup of commands file |

### config.rs
| Function | Description |
|---|---|
| `Config` | Main configuration structure |
| `PopupSettings` | Popup window settings |
| `LauncherSettings` | Launcher behavior settings |
| `load_config() -> Config` | Load configuration from file |
| `load_config_with_error() -> ConfigResult` | Load config with error details |
| `get_config_file_path() -> PathBuf` | Get configuration file path |
| `is_key_bound_to_action(&str, &str) -> bool` | Check if key is bound to action |
| `get_action_for_key(&str) -> Option<&str>` | Get action for key name |
j
### state.rs
| Function | Description |
|---|---|
| `AppState` | Persistent application state |
| `load_state() -> AppState` | Load state from state.json |
| `save_state(&AppState) -> Result<(), Error>` | Save state to state.json |
| `save_state_with_build_time() -> Result<(), Error>` | Update state with current build time |
| `get_state_file_path() -> PathBuf` | Get path to state.json |

## src/ui/

### popup.rs
| Function | Description |
|---|---|
| `AnchorSelector` | Main popup application state |
| `run_gui_with_prompt(&str, ApplicationState) -> Result<(), eframe::Error>` | Run GUI with initial prompt |
| `save_window_position(egui::Pos2)` | Save window position to state |
| `load_window_position() -> Option<egui::Pos2>` | Load saved window position |
| `is_position_visible(egui::Pos2, egui::Vec2) -> bool` | Check if position is visible on screen |
| `center_on_main_display(&egui::Context, egui::Vec2) -> egui::Pos2` | Center window on main display |
| `new() -> Self` | Create new AnchorSelector |
| `new_with_prompt(&str) -> Self` | Create with initial search prompt |

### popup_state.rs
| Function | Description |
|---|---|
| `PopupState` | Core popup state separated from UI |
| `new(Vec<Command>, Config, AppState) -> Self` | Create new popup state |
| `update_search(String)` | Update search text and recompute commands |
| `navigate(Direction) -> bool` | Navigate selection in direction |
| `get_selected_command() -> Option<&Command>` | Get currently selected command |
| `get_command_at_position(usize, usize) -> Option<&Command>` | Get command at grid position |
| `set_selection_to_position(usize, usize) -> bool` | Set selection to grid position |
| `get_commands() -> &[Command]` | Get all available commands |
| `set_commands(Vec<Command>)` | Update command list |
| `get_layout_dimensions() -> (usize, usize)` | Get layout dimensions (rows, cols) |
| `should_display_command(&Command) -> bool` | Check if command should be displayed |
| `get_submenu_info() -> Option<&SubmenuInfo>` | Get current submenu information |
| `get_display_commands_for_rendering() -> Vec<&Command>` | Get commands for UI rendering |
| `navigate_horizontal(i32) -> bool` | Navigate horizontally by offset |
| `navigate_vertical(i32) -> bool` | Navigate vertically by offset |
| `get_hint_text() -> String` | Get hint text for search input |

### layout.rs
| Function | Description |
|---|---|
| `DisplayLayout` | Commands arrangement for display |
| `LayoutArrangement` | Visual arrangement (SingleColumn or MultiColumn) |
| `SubmenuInfo` | Submenu display information |
| `Selection` | Selection position tracking |
| `Direction` | Navigation directions enum |
| `new(Vec<Command>, &Config) -> Self` | Create new display layout |
| `get_command_at_position(usize, usize) -> Option<&Command>` | Get command at visual position |
| `visual_to_index(usize, usize) -> Option<usize>` | Convert visual position to index |
| `index_to_visual(usize) -> Option<(usize, usize)>` | Convert index to visual position |
| `get_dimensions() -> (usize, usize)` | Get layout dimensions |
| `from_index(usize, &DisplayLayout) -> Self` | Create selection from index |
| `navigate(Direction, &DisplayLayout) -> bool` | Navigate selection in direction |
| `get_command(&DisplayLayout) -> Option<&Command>` | Get selected command |
| `reset(&DisplayLayout)` | Reset selection to first valid command |

### command_editor.rs
| Function | Description |
|---|---|
| `CommandEditor` | Command editor dialog state |
| `CommandEditorResult` | Editor result enum |
| `new() -> Self` | Create new command editor |
| `hide()` | Hide the editor dialog |
| `update_commands(&[Command])` | Update internal commands list |
| `edit_command(Option<&Command>, &str)` | Start editing command |
| `open_with_command(Command)` | Open editor with pre-filled command |
| `update(&egui::Context, &Config) -> CommandEditorResult` | Update and render editor |
| `prepare_save_command() -> (Option<String>, Command)` | Prepare command for saving |
| `delete_original_command(&mut Vec<Command>) -> Result<(), String>` | Delete original command from list |

### dialog.rs
| Function | Description |
|---|---|
| `Dialog` | General dialog system |
| `DialogElement` | Dialog element types |
| `DialogRow` | Row of dialog elements |
| `new() -> Self` | Create new dialog |
| `show(Vec<String>)` | Show dialog with specification strings |
| `show_error(&str)` | Show error message dialog |
| `hide()` | Hide the dialog |
| `take_result() -> Option<HashMap<String, String>>` | Take dialog input results |
| `calculate_required_size() -> (f32, f32)` | Calculate required dialog size |
| `update(&egui::Context) -> bool` | Update and render dialog |

## src/

### cmd.rs
| Function | Description |
|---|---|
| `run_command_line_mode(Vec<String>)` | Main CLI entry point |
| `print_help(&str)` | Print help message |

### launcher.rs
| Function | Description |
|---|---|
| `LauncherError` | Launcher error types |
| `LauncherConfig` | Launcher configuration |
| `LauncherSettings` | Launcher settings |
| `launch(&str) -> Result<(), LauncherError>` | Launch command line |

### scanner.rs
| Function | Description |
|---|---|
| `startup_check(Vec<Command>) -> Vec<Command>` | Check if filesystem scan needed on startup |
| `scan(Vec<Command>, &[String]) -> Vec<Command>` | Top-level scan function |
| `scan_files(Vec<Command>, &[String]) -> Vec<Command>` | Scan markdown files for commands |
| `scan_contacts(Vec<Command>) -> Vec<Command>` | Scan macOS contacts |

### utils.rs
| Function | Description |
|---|---|
| `debug_log(&str, &str)` | Debug logging to file |
| `expand_tilde(&str) -> String` | Expand ~ in file paths |
| `launch_app_with_arg(&str, Option<&str>) -> Result<Output, Error>` | Launch app with optional argument |
| `open_url(&str) -> Result<Output, Error>` | Open URL in default browser |
| `open_folder(&str) -> Result<Output, Error>` | Open folder in Finder |
| `execute_shell_command(&str) -> Result<Output, Error>` | Execute shell command |
| `execute_shell_command_with_env(&str) -> Result<Output, Error>` | Execute shell command with environment |
| `open_with_app(&str, &str) -> Result<Output, Error>` | Open file/URL with specific app |

### eval.rs
| Function | Description |
|---|---|
| `Environment` | Evaluation environment |
| `EvalError` | Evaluation error types |
| `substitute_template_in_args(&HashMap<String, serde_yaml::Value>, &Environment) -> HashMap<String, serde_yaml::Value>` | Template substitution in arguments |
| `substitute_template_in_value(&serde_yaml::Value, &Environment) -> serde_yaml::Value` | Template substitution in values |
| `new() -> Result<Self, Error>` | Create new evaluation environment |
| `eval(serde_yaml::Value) -> Result<serde_yaml::Value, EvalError>` | Unified evaluation function |

### js_runtime.rs
| Function | Description |
|---|---|
| `create_business_logic_runtime() -> Result<Context, Error>` | Create JavaScript runtime |
| `create_business_logic_runtime_with_config(&Config) -> Result<Context, Error>` | Create JS runtime with config |
| `setup_all_builtins(&Ctx) -> Result<(), Error>` | Setup all built-in functions |
| `execute_business_logic(&str) -> Result<String, Error>` | Execute JavaScript code |
| `setup_config_access(&Ctx, &Config) -> Result<(), Error>` | Setup config access in JS |

### business_logic.rs
| Function | Description |
|---|---|
| `run_business_script(&str) -> Result<String, Error>` | Execute business logic script |
| `scan_markdown_commands() -> Result<Vec<Command>, Error>` | Scan markdown files for commands |
| `update_commands_from_markdown() -> Result<usize, Error>` | Update commands from markdown |
| `activate_anchor(&str) -> Result<String, Error>` | Activate anchor project |

### builtin_fns.rs
| Function | Description |
|---|---|
| `setup_builtin_functions(&mut Environment)` | Setup all built-in functions |

### grabber.rs
| Function | Description |
|---|---|
| `AppContext` | Captured application information |
| `GrabberRule` | Rule for matching context |
| `GrabResult` | Result of grab operation |
| `capture_active_app() -> Result<AppContext, String>` | Capture active application context |
| `get_browser_info(&str) -> Option<String>` | Get browser-specific information |
| `get_finder_info() -> Option<String>` | Get Finder path information |
| `enrich_context(AppContext) -> AppContext` | Enrich context with additional info |
| `match_grabber_rules(&AppContext, &[GrabberRule], &Config) -> Option<(String, Command)>` | Match context against grabber rules |
| `generate_rule_template_text(&AppContext) -> String` | Generate rule template from context |
| `grab(&Config) -> Result<GrabResult, String>` | Perform context grab operation |