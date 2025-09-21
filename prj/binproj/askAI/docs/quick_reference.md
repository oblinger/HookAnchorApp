# askAI Quick Reference

## Basic Usage

### CLI Commands
```bash
# Session management
askAI start                    # Start Claude session
askAI stop                     # Stop session
askAI status                   # Check if running
askAI attach                   # Connect to session

# Simple queries
askAI query "What is 2+2?"     # Full command
askAI q "What is 2+2?"         # Shortcut

# Web search
askAI web_search "Python docs"              # Google search in Safari
askAI web_search --new-tab "AI tutorials"   # In new tab
askAI web_search --results 5 "Python docs" # Parse 5 search results
askAI web_search --results 3 --json "AI"    # JSON output for programming

# With options
askAI q --timeout 60 "Complex analysis"
askAI q --json "Return structured data"
```

### Python Library
```python
import askAI

# Basic query
response = askAI.query("Count files in current directory")

# With timeout
response = askAI.query("Analyze code", timeout=120)

# Session context for related queries
with askAI.Session() as session:
    overview = session.query("Analyze this codebase")
    issues = session.query("Based on that, find problems")

# Web search with results parsing
results = askAI.web_search("Python tutorial", max_results=3)
# Returns: [{"title": "...", "url": "...", "description": "...", ...}, ...]

# Web search in new tab
results = askAI.web_search("machine learning", max_results=5, new_tab=True)
```

## Web Search & Results Parsing

### CLI Web Search
```bash
# Basic web search (opens in Safari)
askAI web_search "Python tutorial"

# Parse search results (returns structured data)
askAI web_search --results 5 "machine learning"

# JSON output for scripting
askAI web_search --results 3 --json "data science" | jq '.[0].title'

# Open in new Safari tab
askAI web_search --new-tab "AI frameworks"
```

### Library Web Search
```python
import askAI

# Get structured search results
results = askAI.web_search("Python libraries", max_results=5)

# Result structure:
# [
#   {
#     "title": "NumPy — Fundamental package for scientific computing",
#     "url": "https://numpy.org/",
#     "description": "",
#     "displayUrl": "numpy.org",
#     "position": 1
#   },
#   ...
# ]

# Access result data
for result in results:
    print(f"{result['title']}: {result['url']}")

# Search in new tab
results = askAI.web_search("React tutorial", max_results=3, new_tab=True)
```

### Web Search Use Cases
```python
# Research competitors
competitors = askAI.web_search("project management tools", max_results=10)

# Find documentation
docs = askAI.web_search("FastAPI documentation", max_results=3)

# Market research
trends = askAI.web_search("2024 web development trends", max_results=5)

# Technical solutions
solutions = askAI.web_search("Python async database connection", max_results=5)
```

## Structured Data Extraction

### Simple Keys (List)
```python
# Returns: {'total': 42, 'python_files': 12}
result = askAI.query("Count files", keys=['total', 'python_files'])
print(result['total'])  # Access as dict
```

### Simple Keys (String)
```python
# Same as above, comma-separated
result = askAI.query("Count files", keys="total, python_files")
```

### Typed Keys (Dict)
```python
# Type hints help Claude return proper data types
result = askAI.query("Analyze code complexity", keys={
    'score': 'number',           # Returns int/float
    'is_complex': 'boolean',     # Returns True/False
    'language': 'string',        # Returns string
    'functions': 'list'          # Returns array
})
```

### Custom/Semantic Types
```python
# Use descriptive types for better results
result = askAI.query("Where is the White House?", keys={
    'address': 'street address',
    'coordinates': 'latitude,longitude',
    'established': 'date (YYYY-MM-DD)',
    'website': 'URL',
    'contact': 'phone number'
})
```

## Advanced Examples

### Code Analysis
```python
data = askAI.query("Analyze this Python file", keys={
    'line_count': 'number',
    'complexity_score': 'number (1-10)',
    'main_function': 'function name',
    'imports': 'list of modules',
    'has_tests': 'boolean'
})
```

### Data Processing
```python
result = askAI.query("Parse this log entry", keys={
    'timestamp': 'ISO 8601 datetime',
    'log_level': 'severity level',
    'message': 'error message',
    'source_file': 'filename',
    'line_number': 'number'
})
```

### Web Content Analysis
```python
info = askAI.query("Analyze this website content", keys={
    'title': 'page title',
    'description': 'meta description',
    'word_count': 'number',
    'readability_score': 'grade level',
    'primary_topic': 'subject category',
    'target_audience': 'audience type'
})
```

### System Information
```python
specs = askAI.query("What are typical laptop specs?", keys={
    'cpu': 'processor model',
    'ram': 'memory amount',
    'storage': 'storage capacity',
    'price_range': 'price range in USD',
    'battery_life': 'hours',
    'weight': 'weight in pounds'
})
```

## Configuration

### Config File Location
- `~/.config/askAI/askAI.yaml` (primary)
- `./askAI.yaml` (project-local)

### Sample Config
```yaml
settings:
  session_name: askAI-session
  default_timeout: 60
  log_level: INFO
  claude_command: claude
  log_queries: true
  log_dir: ./logs  # Relative to config file
```

## Error Handling

```python
try:
    result = askAI.query("Complex task", keys=['key1', 'key2'])
except Exception as e:
    if "Session not found" in str(e):
        # Start session first
        subprocess.run(['askAI', 'start'])
    elif "JSON" in str(e):
        # Claude didn't return valid JSON
        print("Try simpler query or different keys")
    else:
        print(f"Other error: {e}")
```

## Tips

### Better Results
- Use specific type hints: `'date (YYYY-MM-DD)'` vs `'date'`
- Ask for lists when expecting multiple items
- Include units: `'size in MB'`, `'score (1-10)'`
- Be specific: `'email address'` vs `'contact'`

### Performance
- Use shorter timeouts for simple queries
- Group related queries in Session context
- Prefer structured queries over manual parsing

### Debugging
```bash
askAI attach          # Connect to see what Claude is doing
askAI --debug q "test" # Enable debug logging
```