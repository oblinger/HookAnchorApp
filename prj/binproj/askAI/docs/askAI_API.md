# askAI Python Library API Documentation

## Overview

The askAI module provides a Python library interface for interacting with Claude Code through tmux sessions. This allows you to programmatically send queries to Claude and receive responses within your Python applications.

## Installation

```bash
# Install the askAI module (assuming it's in your Python path)
import askAI
```

## Basic Usage

### Simple Query

```python
import askAI

# Send a basic query to Claude
response = askAI.query("What files are in this directory?")
print(response)
```

### Query with Options

```python
import askAI

# Query with custom timeout and session name
response = askAI.query(
    prompt="Analyze this code for security issues",
    timeout=60,
    session_name="custom-session"
)
print(response)
```

## API Reference

### Functions

#### `askAI.query(prompt, timeout=None, session_name="askAI-session")`

Send a single query to Claude via tmux session.

**Parameters:**
- `prompt` (str): The query to send to Claude
- `timeout` (int, optional): Maximum time to wait for response in seconds. Uses config file default if not specified
- `session_name` (str, optional): Name of the tmux session. Default: "askAI-session"

**Returns:**
- `str`: Claude's response

**Example:**
```python
response = askAI.query("Explain this function", timeout=45)
```

#### `askAI.batch_query(prompts, timeout=None, session_name="askAI-session")`

Send multiple queries to Claude sequentially.

**Parameters:**
- `prompts` (List[str]): List of queries to send
- `timeout` (int, optional): Maximum time per query in seconds. Uses config file default if not specified
- `session_name` (str, optional): Name of the tmux session. Default: "askAI-session"

**Returns:**
- `List[str]`: List of responses corresponding to each query

**Example:**
```python
queries = [
    "Find all TODO comments",
    "List security issues",
    "Generate documentation"
]
responses = askAI.batch_query(queries, timeout=60)
for query, response in zip(queries, responses):
    print(f"Q: {query}")
    print(f"A: {response}\n")
```

### Classes

#### `askAI.Session(session_name="askAI-session")`

Context manager for managing askAI sessions with persistent state.

**Parameters:**
- `session_name` (str, optional): Name of the tmux session. Default: "askAI-session"

**Methods:**
- `query(prompt, timeout=None)`: Send a query within the session context (uses config file default timeout)

**Example:**
```python
with askAI.Session() as session:
    response1 = session.query("What is the current directory?")
    response2 = session.query("List the Python files here")
    response3 = session.query("Based on the files you just listed, what is this project about?")
```

## Advanced Usage

### Custom Session Management

```python
import askAI

# Use a custom session name for isolation
session_name = "my-analysis-session"

# First query establishes context
response1 = askAI.query(
    "Analyze the structure of this Python project",
    session_name=session_name
)

# Subsequent queries can reference previous context
response2 = askAI.query(
    "Based on your analysis, suggest improvements",
    session_name=session_name
)
```

### Error Handling

```python
import askAI

try:
    response = askAI.query("Complex analysis task", timeout=120)
    print(response)
except TimeoutError:
    print("Query timed out")
except ConnectionError:
    print("Could not connect to askAI session")
except Exception as e:
    print(f"Error: {e}")
```

### Batch Processing with Error Handling

```python
import askAI

queries = [
    "Find all Python files",
    "Check for syntax errors",
    "Generate test cases"
]

successful_responses = []
failed_queries = []

for query in queries:
    try:
        response = askAI.query(query, timeout=60)
        successful_responses.append((query, response))
    except Exception as e:
        failed_queries.append((query, str(e)))

print(f"Successful: {len(successful_responses)}")
print(f"Failed: {len(failed_queries)}")
```

## Integration Examples

### Code Analysis Script

```python
#!/usr/bin/env python3
import askAI
import os
import sys

def analyze_codebase(directory):
    """Analyze a codebase using askAI."""

    os.chdir(directory)

    with askAI.Session("code-analysis") as session:
        # Get overview
        overview = session.query("Analyze the structure of this codebase")

        # Find issues
        issues = session.query("Find potential bugs and security issues")

        # Get suggestions
        suggestions = session.query("Suggest improvements and optimizations")

        return {
            'overview': overview,
            'issues': issues,
            'suggestions': suggestions
        }

if __name__ == "__main__":
    if len(sys.argv) != 2:
        print("Usage: python analyze.py <directory>")
        sys.exit(1)

    results = analyze_codebase(sys.argv[1])

    print("=== CODEBASE ANALYSIS ===")
    print(f"Overview:\n{results['overview']}\n")
    print(f"Issues:\n{results['issues']}\n")
    print(f"Suggestions:\n{results['suggestions']}")
```

### Documentation Generator

```python
#!/usr/bin/env python3
import askAI
import glob

def generate_docs():
    """Generate documentation for Python files."""

    python_files = glob.glob("**/*.py", recursive=True)

    for file_path in python_files:
        print(f"Generating docs for {file_path}...")

        query = f"Generate comprehensive documentation for the file {file_path}"
        docs = askAI.query(query, timeout=120)

        # Save documentation
        doc_path = file_path.replace('.py', '_docs.md')
        with open(doc_path, 'w') as f:
            f.write(f"# Documentation for {file_path}\n\n")
            f.write(docs)

        print(f"Documentation saved to {doc_path}")

if __name__ == "__main__":
    generate_docs()
```

## Configuration

### Configuration File

Configuration is managed through a config file at `~/.config/askAI/askAI.yaml` or command-line options:

```yaml
# askAI Configuration

settings:
  session_name: askAI-session
  default_timeout: 60
  log_level: INFO
  claude_command: claude
  log_queries: true
  log_dir: ./logs
  response_end_marker: "<<<CLAUDE_END>>>"
  stability_wait: 2
  max_response_length: 10000
```

The configuration file uses a `settings` section to organize parameters, allowing for additional sections to be added in the future.

## Best Practices

1. **Use descriptive session names** for different types of work to maintain context separation
2. **Set appropriate timeouts** based on query complexity
3. **Handle exceptions** properly in production code
4. **Use context managers** (`Session`) for related queries that benefit from shared context
5. **Batch similar queries** using `batch_query()` for efficiency
6. **Monitor session health** and implement retry logic for critical applications

## Troubleshooting

### Common Issues

1. **"Session not found" errors**: Ensure the tmux session is running
2. **Timeout errors**: Increase timeout for complex queries
3. **Import errors**: Ensure askAI.py is in your Python path
4. **Permission errors**: Ensure askAI.py has execute permissions

### Debugging

Enable debug logging to troubleshoot issues:

```python
import logging
import askAI

logging.basicConfig(level=logging.DEBUG)
response = askAI.query("Your query")
```

## API Status

**Current Version:** 0.1.0

**Implementation Status:**
- ⬜ `query()` function - Placeholder implementation
- ⬜ `batch_query()` function - Placeholder implementation
- ⬜ `Session` class - Placeholder implementation
- ⬜ Error handling - Not implemented
- ⬜ Configuration support - Not implemented
- ⬜ Logging - Basic logging setup

This API documentation describes the intended interface. The actual implementation is still in development.