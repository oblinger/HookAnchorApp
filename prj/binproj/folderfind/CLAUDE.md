# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a folder finding utility that provides fast directory navigation through cached folder paths and aliases. The system works by maintaining a cache file at `~/bin/.ff_cache` that contains directory paths and aliases for quick lookup.

## Architecture

- **ff_lookup.py**: Core lookup logic that searches through cached directories and aliases
  - `find_folder()`: Main search function that checks aliases first, then exact matches, then partial matches
  - Returns tuples of (path, match_type) where match_type can be 'alias', 'exact', 'partial', or None
  - Reads from `~/bin/.ff_cache` file containing directory paths and aliases in format "alias -> path"

- **ffload**: Wrapper script that delegates to `ffload.py` (currently missing) to rebuild the cache file

## Cache File Format

The `.ff_cache` file contains:
- Directory paths (one per line)
- Aliases in format: `alias_name -> /full/path/to/directory`

## Search Priority

1. Exact alias matches
2. Exact folder name matches (basename or full path)
3. Partial matches (case-insensitive substring search)

## Commands

- Run folder lookup: `python3 ff_lookup.py <search_term>`
- The `ffload` script currently references missing `ffload.py` for cache rebuilding

## Missing Components

The `ffload` script references `ffload.py` which is not present in the repository but would be responsible for rebuilding the folder cache.

## Additional Notes

- `~/bin/ob_zshrc` has commands that use these folderfind functions so be sure to refactor those commands as needed