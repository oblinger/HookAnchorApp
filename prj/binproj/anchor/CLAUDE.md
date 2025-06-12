# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This the 'anchor' project documentation and utility repository that combines:
1. Documentation defining a personal project management methodology
2. Folder finding utilities for fast directory navigation through cached folder paths and aliases

## Project Structure

### Core Components
- `anchor` - Python executable script that launches Claude Code and optionally PyCharm
- `anchor.md` - Documentation file describing the Dan Project methodology

### Folder Finding Utilities
- `ffload` - Python script that rebuilds the folder cache (`~/bin/.ff_cache`) for fast folder finding
- `fflookup` - Python script providing shared folder lookup logic with alias support

### Anchor Folder Structure
An anchor folder is a folder containing a markdown with the same name (XXX/XXX.md).
- Anchor folders should have a basename unique among all anchor folders serving as their anchor name
- A markdown file with same name as folder (XXX/XXX.md)
- Optional GitHub-linked git repo
- Optional CLAUDE.md file for Claude Code configuration
- Optional .idea folder for PyCharm configuration

## Script Functionality

The `anchor` script is a command-line tool with the following commands:

### `anchor a` or `anchor --activate`
Activates the project by:
1. Checking if the current directory contains a `.idea` folder (PyCharm project)
2. If found, opening the project in PyCharm using `open -a PyCharm .`
3. Opening the current directory in Finder using `open .`
4. If a `CLAUDE.md` file exists, launching Claude Code using `claude --continue`

This allows for seamless integration between PyCharm IDE and Claude Code for Dan Projects that use PyCharm.

### `anchor n` or `anchor --new <name> [--under|-u <location>]`
Creates a new anchor folder with date prefix and activates it:
1. Prepends current date in YYYY-MM format to the project name
2. Creates the folder in the specified location (default: `~/ob/kmr/prj/`)
3. Creates an empty markdown file with the same name as the folder
4. Changes to the new directory and runs `anchor --activate`

Available locations for `--under`:
- `prj` (default): `~/ob/kmr/prj/`
- `proj`: `~/ob/proj/`
- `binproj`: `~/ob/kmr/prj/binproj/`
- `kmr`: `~/ob/kmr/`
- `ob`: `~/ob/`
- Or any fully qualified path

### `anchor h` or `anchor --help`
Displays usage information and available commands.

The script is designed to be extensible with additional commands in the future.

## Folder Finding System

### Architecture
- **fflookup**: Core lookup logic that searches through cached directories and aliases
  - `find_folder()`: Main search function that checks aliases first, then exact matches, then partial matches
  - Returns tuples of (path, match_type) where match_type can be 'alias', 'exact', 'partial', or None
  - Reads from `~/bin/.ff_cache` file containing directory paths and aliases in format "alias -> path"

- **ffload**: Cache builder that scans directories and creates the cache file
  - Searches base paths: `~/ob/kmr`, `~/ob/proj`
  - Identifies anchor folders (folders with matching .md files)
  - Extracts aliases from markdown files with "Alias [[XXX]]" pattern
  - Builds `~/bin/.ff_cache` with hardcoded entries, directory paths, and aliases

### Cache File Format
The `.ff_cache` file contains:
- Hardcoded user directory aliases (bin, data, ob, kmr, proj)
- Directory paths (one per line)
- Aliases in format: `alias_name -> /full/path/to/directory`

### Search Priority
1. Exact alias matches
2. Exact folder name matches (basename or full path)
3. Partial matches (case-insensitive substring search)

### Anchor Folder Alias System
A folder is an "anchor folder" if:
- It has a markdown file with the same base name as the folder
- The first line of that .md file matches "Alias [[XXX]]" pattern
- An XXX.md file exists in the same folder
- Then "XXX -> /path/to/folder" is added to the cache

## Folder Finding System

### Architecture
- **fflookup**: Core lookup logic that searches through cached directories and aliases
  - `find_folder()`: Main search function that checks aliases first, then exact matches, then partial matches
  - Returns tuples of (path, match_type) where match_type can be 'alias', 'exact', 'partial', or None
  - Reads from `~/bin/.ff_cache` file containing directory paths and aliases in format "alias -> path"

- **ffload**: Cache builder that scans directories and creates the cache file
  - Searches base paths: `~/ob/kmr`, `~/ob/proj`
  - Identifies anchor folders (folders with matching .md files)
  - Extracts aliases from markdown files with "Alias [[XXX]]" pattern
  - Builds `~/bin/.ff_cache` with hardcoded entries, directory paths, and aliases

### Cache File Format
The `.ff_cache` file contains:
- Hardcoded user directory aliases (bin, data, ob, kmr, proj)
- Directory paths (one per line)
- Aliases in format: `alias_name -> /full/path/to/directory`

### Search Priority
1. Exact alias matches
2. Exact folder name matches (basename or full path)
3. Partial matches (case-insensitive substring search)

### Anchor Folder Alias System
A folder is an "anchor folder" if:
- It has a markdown file with the same base name as the folder
- The first line of that .md file matches "Alias [[XXX]]" pattern
- An XXX.md file exists in the same folder
- Then "XXX -> /path/to/folder" is added to the cache

## Commands in ~/bin/ob_zshrc

### `ff` - Folder Finder
Fast directory navigation with fuzzy finding:
- `ff` - Opens fzf browser for all cached directories
- `ff <term>` - Searches for folder by name/alias
  - Exact/alias matches: navigates directly
  - Partial matches: opens fzf with filtered results for selection

### `ana` - Anchor Activate
Like `ff` but calls `anchor --activate` after navigation:
- `ana <term>` - Finds folder and runs `anchor --activate`
- Has same interactive mode as `ff` for ambiguous matches
- Uses fzf for partial matches

### `dp` - Dan Project
Navigates to folder and runs `~/ob/kmr/prj/binproj/dp/dp`:
- `dp <term>` - Finds folder and activates Dan Project
- Only exact/alias matches work (no interactive mode for partials)

All commands use the same `fflookup` backend and tab completion system.

## Dependencies and Integration

### External Dependencies
- Python 3 (for all scripts)
- `~/bin/.ff_cache` file (created by ffload, used by fflookup)
- Base directory structure: `~/ob/kmr/`, `~/ob/proj/`

### System Integration
- Symlinks in `~/bin/` point to this project's scripts
- `~/bin/ob_zshrc` contains commands that use these utilities
- Cache file `~/bin/.ff_cache` must be rebuilt when directory structure changes

### Related Commands in ob_zshrc
Commands that depend on these utilities should be updated when script locations change.

## Development Context

This is part of a larger "BinProj" collection of script projects that are linked by a personal bin folder. The parent directory contains other related projects like "Budg" (budget management) and "km" (knowledge management).

## Memories

- `~/bin/ob_zshrc` contains commands that trigger anchor and folder finding utilities
- The `ff` command in `~/bin/ob_zshrc` has tab completion that extracts all directory basenames from `~/.anchor_cache`, which can create thousands of completion options and cause shell performance issues
- Tab completion for `ff`, `dp`, and `ana` commands uses `_describe` to show directory basenames and alias names
- Python 3 (for all scripts)
- `~/bin/.ff_cache` file (created by ffload, used by fflookup)
- Base directory structure: `~/ob/kmr/`, `~/ob/proj/`

### System Integration
- Symlinks in `~/bin/` point to this project's scripts
- `~/bin/ob_zshrc` contains commands that use these utilities
- Cache file `~/bin/.ff_cache` must be rebuilt when directory structure changes

### Related Commands in ob_zshrc
Commands that depend on these utilities should be updated when script locations change.

## Memories

- `~/bin/ob_zshrc` contains commands that trigger anchor and folder finding utilities