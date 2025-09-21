# askAI2 - Direct Anthropic API Integration

askAI2 provides the same interface as askAI but uses Anthropic's Claude API directly instead of tmux sessions. This offers significant advantages in speed, reliability, and cross-platform compatibility.

## Key Advantages over askAI

- **Direct API calls** - No tmux sessions or shell intermediaries
- **Faster responses** - Direct network requests to Anthropic
- **Multiple Claude models** - Choose from Sonnet, Haiku, Opus variants
- **Better error handling** - Clear API error messages
- **Cross-platform** - Works on any platform with Python
- **Structured responses** - Built-in JSON parsing support
- **Session context** - Conversation history without tmux complexity

## Quick Setup

1. **Install dependencies:**
   ```bash
   pip install anthropic pyyaml
   ```

2. **Get your API key:**
   - Visit https://console.anthropic.com/
   - Create an account and generate an API key
   - Your key will start with `sk-ant-api03-`

3. **Set your API key:**
   ```bash
   export ANTHROPIC_API_KEY=sk-ant-api03-your-key-here
   ```

4. **Test the setup:**
   ```bash
   python askAI2.py status
   ```

## CLI Usage

### Basic Commands

```bash
# Simple queries
askAI2 query "What is 2+2?"
askAI2 q "Analyze this Python code: print('hello')"

# Web search (uses same engine as askAI)
askAI2 web_search "Python tutorial"
askAI2 web_search --results 5 "machine learning"
askAI2 web_search --results 3 --json "data science"

# Check status
askAI2 status

# Get help
askAI2 help
askAI2 help models
```

### Model Selection

```bash
# Use specific models
askAI2 q --model claude-3-opus "Complex reasoning task"
askAI2 q --model claude-3-5-haiku "Quick simple question"

# Available models:
# claude-3-5-sonnet-20241022  (recommended, balanced)
# claude-3-5-haiku-20241022   (fast, efficient)
# claude-3-opus-20240229      (most capable)
```

### Advanced Options

```bash
# JSON output for structured queries
askAI2 q --json "Analyze file and return {lines: number, functions: list}"

# Timeout control
askAI2 q --timeout 120 "Long analysis task"

# Verbose logging
askAI2 --verbose q "Debug this issue"
```

## Python Library Usage

### Basic Queries

```python
import askAI2

# Simple query
response = askAI2.query("What is the capital of France?")
print(response)  # "The capital of France is Paris."

# With specific model
response = askAI2.query("Complex analysis", model="claude-3-opus")
```

### Structured Data Extraction

```python
# List of keys
result = askAI2.query("Calculate 15 * 4 and 100 / 5", keys=['multiplication', 'division'])
print(result)  # {'multiplication': 60, 'division': 20}

# Comma-separated keys
result = askAI2.query("Analyze file.py", keys="lines, functions, complexity")

# Typed keys with hints
result = askAI2.query("Count Python files", keys={
    'total_files': 'number',
    'python_files': 'number',
    'largest_file': 'filename'
})
```

### Session Context

```python
# Conversation history
with askAI2.Session() as session:
    overview = session.query("Analyze this codebase structure")
    issues = session.query("Based on that analysis, what are the main issues?")
    fixes = session.query("How would you fix the top 3 issues?")
```

### Web Search Integration

```python
# Same web search as askAI
results = askAI2.web_search("Python tutorial", max_results=5)
for result in results:
    print(f"{result['title']}: {result['url']}")

# Web search in new tab
results = askAI2.web_search("AI frameworks", max_results=3, new_tab=True)
```

## Configuration

### Config File

Create `~/.config/askAI/askAI2.yaml` or `./askAI2.yaml`:

```yaml
settings:
  model: claude-3-5-sonnet-20241022
  max_tokens: 4096
  temperature: 0.7
  default_timeout: 60
  log_level: INFO
  log_queries: true
  log_dir: ./logs
```

### Model Comparison

| Model | Use Case | Speed | Cost | Context |
|-------|----------|-------|------|---------|
| **claude-3-5-sonnet-20241022** | Balanced, recommended | Fast | $3/$15 | 200K |
| **claude-3-5-haiku-20241022** | Quick queries, high volume | Fastest | $0.80/$4 | 200K |
| **claude-3-opus-20240229** | Complex reasoning | Slower | $15/$75 | 200K |

Pricing is per million tokens (input/output).

## Practical Examples

### Code Analysis

```python
# Analyze a Python file
analysis = askAI2.query("Analyze this code", keys={
    'line_count': 'number',
    'function_count': 'number',
    'complexity_score': 'number (1-10)',
    'main_issues': 'list',
    'suggestions': 'list'
})

print(f"Lines: {analysis['line_count']}")
print(f"Functions: {analysis['function_count']}")
print(f"Complexity: {analysis['complexity_score']}/10")
```

### Research Tasks

```python
# Research a topic
research = askAI2.query("Research modern web frameworks", keys={
    'top_frameworks': 'list',
    'popularity_trend': 'string',
    'recommended_for_beginners': 'framework name',
    'enterprise_choice': 'framework name'
})

# Get additional context with web search
web_results = askAI2.web_search("2024 web framework comparison", max_results=5)
```

### Batch Processing

```python
# Process multiple queries
questions = [
    "What is machine learning?",
    "Explain neural networks",
    "What is deep learning?"
]

answers = askAI2.batch_query(questions)
for q, a in zip(questions, answers):
    print(f"Q: {q}")
    print(f"A: {a}\n")
```

## Error Handling

```python
try:
    result = askAI2.query("Complex task", keys=['data', 'analysis'])
except Exception as e:
    if "ANTHROPIC_API_KEY" in str(e):
        print("API key not set or invalid")
    elif "JSON" in str(e):
        print("Claude didn't return valid JSON - try simpler keys")
    else:
        print(f"Other error: {e}")
```

## Migration from askAI

askAI2 is designed as a drop-in replacement:

```python
# askAI (old)
import askAI
response = askAI.query("What is 2+2?")

# askAI2 (new)
import askAI2
response = askAI2.query("What is 2+2?")

# Both web search functions are identical
results = askAI.web_search("query", max_results=5)    # old
results = askAI2.web_search("query", max_results=5)   # new
```

**Key differences:**
- askAI2 requires `ANTHROPIC_API_KEY` instead of tmux sessions
- askAI2 supports multiple Claude models
- askAI2 has faster response times
- askAI2 has better structured query support

## Troubleshooting

### Common Issues

1. **"API key not set"**
   ```bash
   export ANTHROPIC_API_KEY=sk-ant-api03-your-key
   ```

2. **"JSON parsing failed"**
   - Simplify your keys parameter
   - Use more specific type hints
   - Check that Claude understands your request

3. **"Rate limit exceeded"**
   - You've hit Anthropic's API limits
   - Wait and retry, or upgrade your plan

4. **Slow responses**
   - Use `claude-3-5-haiku` for faster queries
   - Reduce `max_tokens` in config
   - Set shorter timeouts

### Getting Help

```bash
askAI2 help          # General help
askAI2 help models   # Model information
askAI2 status        # Check configuration
```

## Development

### Running Tests

```bash
# Install test dependencies
pip install anthropic pyyaml

# Run test suite
python test_askAI2.py

# Run tests with API key (full suite)
export ANTHROPIC_API_KEY=sk-ant-api03-your-key
python test_askAI2.py
```

### Contributing

askAI2 is designed to complement askAI, not replace it. Both tools serve different use cases:

- **askAI**: Complex tmux integration, Claude Code compatibility
- **askAI2**: Direct API access, cross-platform, faster responses