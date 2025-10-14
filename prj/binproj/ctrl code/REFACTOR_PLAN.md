# CTRL Refactoring Plan

## Goal
- Switch from argparse to Typer for cleaner CLI
- Separate data extraction from I/O (output)
- Create reusable components for Chrome page operations
- Group Chrome functions together

## Proposed Function Organization

### Core Utilities (General Purpose)
```python
output_result(data: dict, output_path: Optional[str], format: str)
    """Output dict/list data to stdout or file in JSON/YAML format.

    Args:
        data: Dictionary or list to output
        output_path: File path (None = stdout)
        format: 'json' or 'yaml'
    """
```

### Safari Commands (Browser-agnostic legacy)
```python
search_google(query, max_results, new_tab) -> List[dict]
parse_google_results(max_results) -> List[dict]
navigate_to_url(url, new_tab) -> None
create_new_tab() -> None
open_url_in_new_tab(url) -> None
extract_page_content(url, output_path) -> dict
perform_jsearch(query, output_path) -> List[dict]
extract_list_from_page(output_path, min_items) -> List[dict]
get_current_safari_url() -> str
```

### Playwright Commands
```python
extract_page_with_playwright(url, output_path) -> dict
```

### Shell Commands
```python
execute_shell_command(command) -> None
```

### Typer Command Handlers (CLI layer)
```python
# Main Typer app
app = typer.Typer()

@app.command()
def search(...) -> None
    """Search command handler"""

@app.command()
def navigate(...) -> None
    """Navigate command handler"""

@app.command()
def cpage(...) -> None
    """Chrome page extraction command handler"""
    # Orchestrates: setup_chrome_page + get_chrome_page + output_result

# ... other command handlers
```

### Chrome CDP Functions (Bottom of file - cohesive group)
```python
# Chrome connection/setup
check_chrome_debug_port(port) -> bool
    """Check if Chrome debug port is available"""

launch_chrome_debug(port) -> subprocess.Popen
    """Launch Chrome with remote debugging"""

connect_to_chrome(port) -> Tuple[Playwright, Browser]
    """Connect Playwright to Chrome via CDP"""

get_chrome_tab_by_number(browser, tab_number) -> Page
    """Get specific Chrome tab by number (1, 2, -1, -2, etc.)"""

# NEW FUNCTIONS (refactored separation of concerns)
setup_chrome_page(
    browser: Browser,
    url_or_tab: str,
    port: int = 9222
) -> Page:
    """Set up and return the Chrome page to extract from.

    Handles:
    - Opening new tab if URL provided (http://... or https://...)
    - Selecting existing tab if number provided (1, 2, -1, -2, etc.)
    - Using last tab if '-' or None
    - Scrolling to load lazy content for existing tabs

    Args:
        browser: Connected Playwright browser
        url_or_tab: URL string, tab number, '-', or None
        port: CDP port (for potential connection retry)

    Returns:
        Playwright Page object ready for extraction
    """

get_chrome_page(
    page: Page,
    include_html: bool = False,
    use_legacy_font: bool = False,
    max_font_filter: Optional[int] = None
) -> dict:
    """Extract page data and return as dict (pure data, no I/O).

    Args:
        page: Playwright Page object
        include_html: Include full HTML in output
        use_legacy_font: Use legacy " ~:~ " font format
        max_font_filter: Filter to show only fonts f1-fN (None = no filter)

    Returns:
        Dict with structure:
        {
            'url': str,
            'title': str,
            'metadata': {...},
            'headings': {'h1': [...], 'h2': [...], ...},
            'fonts': {...},  # only if not legacy format
            'text': [...],   # recursive list/dict structure
            'links': [...],
            'images': [...],
            'html': str      # only if include_html=True
        }
    """

extract_page_with_chrome(url, output_path, port, output_format, include_html, font_filter) -> dict:
    """REFACTORED - Orchestrate Chrome page extraction (compatibility wrapper).

    Now calls:
    1. connect_to_chrome(port) -> playwright, browser
    2. setup_chrome_page(browser, url) -> page
    3. get_chrome_page(page, ...) -> data
    4. output_result(data, output_path, output_format)

    This function kept for backward compatibility with existing code.
    New code should call the three functions separately for more flexibility.
    """
```

## Key Changes

### Before (argparse monolith):
```python
def extract_page_with_chrome(url, output_path, port, output_format, include_html, font_filter):
    # Setup Chrome connection
    # Parse URL/tab parameter
    # Open or select page
    # Extract data with JavaScript
    # Output to file/stdout
    # All mixed together - 500+ lines!
```

### After (Typer + separated concerns):
```python
# 1. Data extraction (pure, reusable)
def get_chrome_page(page, **kwargs) -> dict:
    # Just extract and return data
    # No I/O, no printing
    # ~200 lines

# 2. Page setup (reusable)
def setup_chrome_page(browser, url_or_tab) -> Page:
    # Handle URL vs tab number
    # Open new or select existing
    # Scroll for lazy content
    # ~50 lines

# 3. Output formatting (reusable across ALL commands)
def output_result(data, output_path, format):
    # Handle JSON/YAML
    # Handle file/stdout
    # ~20 lines
    # Can be used by cpage, jpage, jsearch, etc.

# 4. Typer command (clean orchestration)
@app.command()
def cpage(url: str = "-", output: Optional[str] = None, ...):
    playwright, browser = connect_to_chrome(port)
    page = setup_chrome_page(browser, url)
    data = get_chrome_page(page, include_html=html, ...)
    output_result(data, output, format='yaml' if yaml else 'json')
```

## Benefits

1. **Reusability**: Other commands can call `get_chrome_page()` internally
2. **Testability**: Pure functions easier to test (no I/O side effects)
3. **Composability**: Mix and match components
4. **Clarity**: Single responsibility per function
5. **Typer**: Type-safe, auto-generated help, less boilerplate

## File Structure After Refactoring

```
__main__.py:
  1. Module docstring
  2. Imports (add typer)
  3. Version
  4. FUNCTION LIST COMMENT (this document as reference)
  5. output_result() - universal output function
  6. Safari commands (search, navigate, etc.)
  7. Playwright commands
  8. Shell commands
  9. Typer app and command handlers
 10. Chrome CDP functions (grouped at bottom):
     - Connection functions
     - setup_chrome_page() NEW
     - get_chrome_page() NEW
     - extract_page_with_chrome() REFACTORED
 11. main() - Typer app entry point
```

## Migration Strategy

1. Add Typer dependency
2. Create `output_result()` function
3. Create `setup_chrome_page()` function
4. Create `get_chrome_page()` function (extract from existing code)
5. Refactor `extract_page_with_chrome()` to use the three functions
6. Create Typer app with command handlers
7. Update `main()` to use Typer instead of argparse
8. Test all commands work identically
9. Add function list comment at top of file

## Questions for Review

1. Does this separation make sense?
2. Should `get_chrome_page()` return different structure?
3. Any other functions that should be split?
4. Should we refactor other commands (jpage, jsearch) similarly?
