

### --- TRACKER
-- Want to see slices of work to see where to optimize
-- 5-slice model by day by week w. percentages
-- 20-100 slice model by day by week

- Compute m/c/s for each entry and show in report
- 0min gets recorded and hide window
- record entries for prior days
- show reports (just try it)


- [ ] tracker bug hit delete & time get screwed
- [ ] time tracker, don't merge if text comment exists ???
- [ ] pick new list name for 'wait' with unique first letter



# Notes

# User Manual

## Quick Start
Run `trackR` or `python3 tracker.py` to launch the time tracker GUI.

## Keyboard Shortcuts

### Time Entry
- **0-9** - Adjust time duration (two digits: hours + 10-minute increments)
- **Letters** - Select time bucket/category (defined in tracker_info.txt)
- **SPACE** - Accept current entry and log it
- **RETURN** - Hide window (minimize)
- **`** - Accept entry BEFORE the last one (replace last entry)

### Time Adjustment
- **[** + HHMM - Set BEGIN time (military format)
- **]** + HHMM - Set END time (military format)  
- **=** - Set start time to current time

### Management
- **-** - Delete last entry
- **'** - Add comment/note to current entry
- **.** - Reset Pomodoro timer (25 min)
- **,** - Refresh screen

### Reporting
- **\\** - Open interactive report interface
- **|** - Generate graph summary
- **/** - Show category help/info
- **?** - Show help screen

### Interactive Report Commands
- **d/w/m** - Change x-axis scale (Day/Week/Month)
- **n/p** - Navigate Next/Previous time window
- **c/g** - Toggle Categories/Groups view
- **SPACE + key** - Filter by specific category
- **x/ESC** - Exit report

# Configuration File

## Location
`/Users/oblinger/.config/tracker/` 

## Format

### Categories (prefix with `.`)
Define time buckets with a shortcut key:
```
.key CategoryName
```
Example: `.m Mit` creates 'Mit' category accessible with 'm' key

### Groups (prefix with `=`)
Group multiple categories together:
```
=GroupName = Category1, Category2, Category3
```
Example: `=Work = Mit, AF, Chew` groups these categories under 'Work'

### Documentation (prefix with `'`)
Add descriptive text about categories:
```
'This text appears in help screens
```

## Data Files
- **tracker.dat** - JSON time entry data (auto-created)
- **tracker.csv** - Daily summary reports
- **tracker.txt** - Text format reports


# LOG

## 2025-09-18  Old hierarchy


 , Spirit

=== THESE ARE THE DOCS PRINTED BY '/' ===
  123456789 12345678 12345678 12345678 12345678 12345678 12345678 12345678---
' SELF:      Self, Fried, Out, Project, Read
' PERSONAL:  Personal, Career, Eli, Family, Home, Run, Spirit
' WORK:      Work, Chew, Email, First, zMit, Prime, Repeat, Secondary
' COMMS:     Comms, Talk, zMeeting
' DAILY:     Daily, Tidy, Wake
' LOUNGE:    Lounge, Browse, Eat, Frittered, Watch
' GAP:       Gap, Sleep, Nap
