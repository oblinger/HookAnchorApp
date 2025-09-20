# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Commands

### Running the Tracker
```bash
# Main execution (with auto-restart loop)
./trackR

# Direct Python execution (single run)
python3 tracker.py
```

## Architecture Overview

### Core Components

**tracker.py** - Monolithic Python application (~1300 lines) that implements:
- Tkinter-based GUI for time tracking interface
- Keyboard-driven interaction model (no mouse required)
- Data persistence using JSON format
- Report generation with pandas/matplotlib
- Interactive reporting interface with terminal-based navigation

### Data Flow

1. **Configuration**: Reads categories and groups from `/Users/oblinger/ob/data/tracker/tracker_info.txt`
2. **Data Storage**: Persists time entries to `/Users/oblinger/ob/data/tracker/tracker.dat` (JSON format)
3. **Reporting**: Generates CSV and text reports to tracker.csv and tracker.txt
4. **UI Events**: Main event loop handles single-character keyboard commands via Tkinter bindings

### Key Design Patterns

- **Command Pattern**: Each keyboard input mapped to exe_* functions (exe_digit, exe_accept_entry, etc.)
- **State Management**: Global variables track current segment, start/end times, and selected category
- **Modal Interface**: Switches between main entry mode and interactive reporting mode
- **Auto-save**: Data automatically persisted after each entry acceptance

### Important File Paths

All data files stored in: `/Users/oblinger/ob/data/tracker/`
- `tracker_info.txt` - Category and group definitions
- `tracker.dat` - JSON time entry database
- `tracker.csv` - Generated daily summaries
- `tracker.txt` - Text format reports

### Dependencies

- Python 3.7+
- tkinter (GUI framework)
- pandas (data analysis and reporting)
- matplotlib (graph generation)
- Standard library: json, datetime, os, subprocess, termios/tty (terminal control)

### Testing Considerations

No formal test suite exists. Manual testing approach:
1. Run `python3 tracker.py` to launch GUI
2. Test keyboard commands per TRACKER_HELP string
3. Verify data persistence in tracker.dat
4. Check report generation with `\` command