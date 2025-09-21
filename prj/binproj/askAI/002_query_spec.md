# askAI Query Implementation Specification

## Overview

This document outlines the implementation strategy for the query system in askAI, which sends queries to Claude via tmux sessions and handles automated responses to permission prompts.

## Configuration: Auto-Allow System

### Config File Structure

```yaml
# askAI Configuration

settings:
  session_name: askAI-session
  default_timeout: 30
  progress_timeout: 10      # No progress for this long = timeout
  prompt_timeout: 300       # Time to wait when prompting user (5 minutes)
  log_level: INFO
  claude_command: claude

auto_allow:
  # List of regex patterns that auto-approve Claude's permission requests
  # When matched, system sends DOWN_ARROW + ENTER to allow and remember
  patterns:
    - "Do you want me to create.*file"
    - "Should I proceed with.*writing.*"
    - "Do you want me to read.*file"
    - "Should I.*search.*directory"
    - "Do you want me to analyze.*"
    - "Should I.*list.*files"
    - "Do you want me to examine.*"
    - "Should I proceed with.*installing.*package"  # Be careful with this one

  # Patterns that should be auto-denied (send 'n' + ENTER)
  deny_patterns:
    - ".*sudo.*"
    - ".*delete.*file"
    - ".*remove.*directory"
    - ".*format.*drive"
```

### Auto-Allow Logic

1. **Pattern Matching**: When Claude outputs a permission request, check against `auto_allow.patterns`
2. **Action**: If matched, inject `DOWN_ARROW` + `ENTER` (allows and remembers choice)
3. **Deny Patterns**: If matched against `deny_patterns`, inject `n` + `ENTER`
4. **No Match**: Wait for manual intervention with extended `prompt_timeout`

## Query Session Monitoring

### Session Status States

The session monitoring system tracks these states:

```python
from enum import Enum
from typing import NamedTuple, Optional

class SessionStatus(Enum):
    WAITING = "waiting"        # Waiting for Claude to start responding
    RUNNING = "running"        # Claude is actively generating response
    PROMPTING = "prompting"    # Claude is asking for permission/input
    DONE = "done"             # Response complete
    TIMEOUT = "timeout"       # No progress within timeout period
    ERROR = "error"           # Error occurred

class QueryResult(NamedTuple):
    status: SessionStatus
    content: str = ""         # Response content or prompt text
    prompt_text: str = ""     # Full text of permission prompt (if prompting)
    elapsed_time: float = 0.0
    error_message: str = ""
```

### Core Monitoring Function

```python
def monitor_query_progress(session_name: str,
                          progress_timeout: int = 10,
                          prompt_timeout: int = 300,
                          poll_interval: float = 0.5) -> QueryResult:
    """
    Monitor query progress with different timeout behaviors.

    Args:
        session_name: tmux session name
        progress_timeout: Timeout when no progress is being made
        prompt_timeout: Timeout when waiting at a prompt for user input
        poll_interval: How often to check session output

    Returns:
        QueryResult with status and content
    """
```

### Monitoring Logic Flow

```
1. Inject query into session
2. Start monitoring loop:

   a) Capture current session output
   b) Compare with previous output to detect progress
   c) Analyze output to determine current state:

      - NEW OUTPUT DETECTED:
        * Reset progress timer
        * Check if response looks complete → DONE
        * Check if output contains permission request → PROMPTING
        * Otherwise continue → RUNNING

      - NO NEW OUTPUT:
        * If last state was PROMPTING:
          - Continue waiting up to prompt_timeout
          - Check for auto_allow pattern match
          - If matched, send response and continue monitoring
        * If last state was RUNNING/WAITING:
          - Check progress_timeout
          - If exceeded → TIMEOUT

      - OUTPUT ANALYSIS:
        * Response completion indicators:
          - Claude prompt appears ("> ")
          - Stable output for X seconds
          - Specific end markers
        * Permission request indicators:
          - "(y/n)" patterns
          - Question ending patterns
          - Waiting cursor/prompt states
```

## Implementation Design

### 1. Query Function Signature

```python
def query(prompt: str,
          timeout: int = 30,
          session_name: str = "askAI-session",
          auto_allow: bool = True,
          log_requests: bool = True) -> str:
    """
    Send query to Claude and return response.

    Args:
        prompt: Query to send to Claude
        timeout: Overall timeout (used for progress_timeout)
        session_name: tmux session name
        auto_allow: Whether to use auto_allow patterns
        log_requests: Whether to log the query/response

    Returns:
        Claude's response text (cleaned of permission dialogs)

    Raises:
        QueryTimeoutError: No progress within timeout
        PromptTimeoutError: Waiting too long for user input at prompt
        SessionError: tmux session issues
    """
```

### 2. Internal Query Implementation

```python
def _execute_query_with_monitoring(session_name: str,
                                  prompt: str,
                                  config: dict) -> QueryResult:
    """Internal query execution with full monitoring."""

    # 1. Inject query
    if not inject_to_session(session_name, prompt):
        return QueryResult(SessionStatus.ERROR, error_message="Failed to inject query")

    # 2. Monitor with timeouts
    start_time = time.time()
    last_output = ""
    last_progress_time = start_time
    current_status = SessionStatus.WAITING

    while True:
        current_output = capture_session_output(session_name)
        current_time = time.time()

        # Check for progress
        if current_output != last_output:
            last_progress_time = current_time
            last_output = current_output

            # Analyze new output
            analysis = analyze_session_output(current_output)

            if analysis.is_complete:
                response = extract_response_text(current_output, prompt)
                return QueryResult(SessionStatus.DONE, content=response,
                                 elapsed_time=current_time - start_time)

            elif analysis.is_prompting:
                current_status = SessionStatus.PROMPTING
                if config.get('auto_allow', True):
                    handled = handle_permission_request(session_name,
                                                      analysis.prompt_text,
                                                      config)
                    if handled:
                        continue  # Continue monitoring after auto-response

            else:
                current_status = SessionStatus.RUNNING

        # Check timeouts
        time_since_progress = current_time - last_progress_time

        if current_status == SessionStatus.PROMPTING:
            if time_since_progress > config['prompt_timeout']:
                return QueryResult(SessionStatus.TIMEOUT,
                                 prompt_text=analysis.prompt_text,
                                 error_message="Prompt timeout - manual intervention required")
        else:
            if time_since_progress > config['progress_timeout']:
                return QueryResult(SessionStatus.TIMEOUT,
                                 error_message="No progress timeout")

        time.sleep(config.get('poll_interval', 0.5))
```

### 3. Output Analysis

```python
class OutputAnalysis(NamedTuple):
    is_complete: bool
    is_prompting: bool
    prompt_text: str = ""
    completion_indicators: list = []

def analyze_session_output(output: str) -> OutputAnalysis:
    """Analyze session output to determine state."""

    # Check for completion indicators
    completion_patterns = [
        r'>\s*$',  # Claude prompt at end
        r'\?\s*for shortcuts\s*$',  # Claude UI indicator
    ]

    # Check for permission request patterns
    prompt_patterns = [
        r'Do you want me to.*\?\s*\(y/n\)',
        r'Should I proceed.*\?\s*\(y/n\)',
        r'.*\[y/n\].*',
        r'Press.*to continue',
    ]

    is_complete = any(re.search(pattern, output, re.MULTILINE)
                     for pattern in completion_patterns)

    for pattern in prompt_patterns:
        match = re.search(pattern, output, re.MULTILINE | re.IGNORECASE)
        if match:
            return OutputAnalysis(
                is_complete=False,
                is_prompting=True,
                prompt_text=match.group(0)
            )

    return OutputAnalysis(is_complete=is_complete, is_prompting=False)
```

### 4. Permission Request Handling

```python
def handle_permission_request(session_name: str,
                            prompt_text: str,
                            config: dict) -> bool:
    """
    Handle permission request based on auto_allow config.

    Returns:
        True if handled automatically, False if needs manual intervention
    """
    auto_allow_patterns = config.get('auto_allow', {}).get('patterns', [])
    deny_patterns = config.get('auto_allow', {}).get('deny_patterns', [])

    # Check deny patterns first (safety)
    for pattern in deny_patterns:
        if re.search(pattern, prompt_text, re.IGNORECASE):
            logging.info(f"Auto-denying: {prompt_text[:50]}... (matched: {pattern})")
            inject_to_session(session_name, "n", send_enter=True)
            return True

    # Check allow patterns
    for pattern in auto_allow_patterns:
        if re.search(pattern, prompt_text, re.IGNORECASE):
            logging.info(f"Auto-allowing: {prompt_text[:50]}... (matched: {pattern})")
            # Send DOWN_ARROW + ENTER to allow and remember
            inject_to_session(session_name, "", send_enter=False)  # Just position
            inject_to_session(session_name, "Down", send_enter=False)  # Down arrow
            inject_to_session(session_name, "", send_enter=True)   # Enter
            return True

    # No match - requires manual intervention
    logging.warning(f"Permission request requires manual intervention: {prompt_text}")
    return False
```

## Logging Strategy

### Log Levels and Content

```python
# Query lifecycle logging
logging.info(f"Starting query: {prompt[:100]}...")
logging.debug(f"Injected query into session {session_name}")
logging.debug(f"Session output: {current_output}")
logging.info(f"Permission request: {prompt_text}")
logging.info(f"Auto-response: {action} (pattern: {pattern})")
logging.info(f"Query completed in {elapsed_time:.2f}s")
logging.warning(f"Query timeout after {elapsed_time:.2f}s")
logging.error(f"Query failed: {error_message}")

# Detailed debug logging
logging.debug(f"Progress check: {time_since_progress:.1f}s since last output")
logging.debug(f"Status transition: {old_status} -> {new_status}")
logging.debug(f"Output analysis: complete={is_complete}, prompting={is_prompting}")
```

### Log File Organization

```
logs/
├── askAI.log                    # Main application log
├── queries/                     # Query-specific logs
│   ├── 2024-09-20_14-30-15.log # Timestamped query logs
│   └── current.log              # Latest query session
└── sessions/                    # Session output captures
    ├── askAI-session.log        # Raw session outputs
    └── responses/               # Cleaned responses
        ├── response_001.txt
        └── response_002.txt
```

## Error Handling

### Exception Hierarchy

```python
class QueryError(Exception):
    """Base exception for query operations"""
    pass

class QueryTimeoutError(QueryError):
    """Query timed out due to no progress"""
    pass

class PromptTimeoutError(QueryError):
    """Timed out waiting for user input at prompt"""
    pass

class SessionError(QueryError):
    """tmux session error"""
    pass

class PermissionDeniedError(QueryError):
    """Action was denied by permission system"""
    pass
```

## Testing Strategy

### Unit Tests

1. **Pattern Matching**: Test auto_allow regex patterns
2. **Output Analysis**: Test completion/prompting detection
3. **Timeout Logic**: Test different timeout scenarios
4. **Permission Handling**: Test auto-response injection

### Integration Tests

1. **End-to-End Query**: Full query with real Claude session
2. **Permission Scenarios**: Test with known permission-requesting queries
3. **Timeout Handling**: Test various timeout conditions
4. **Error Recovery**: Test session failure scenarios

### Mock Testing

```python
def test_query_with_permission_request():
    """Test query that triggers permission request"""
    with mock_session_output([
        "Initial Claude output...",
        "Do you want me to create a test file? (y/n)",
        "File created successfully.",
        "> "  # Claude prompt indicating completion
    ]):
        result = query("Create a test file")
        assert "File created successfully" in result
```

## Future Enhancements

1. **Learning System**: Track which patterns are commonly triggered
2. **Context-Aware Patterns**: Different rules based on query content
3. **User Preference Learning**: Adapt auto_allow based on user manual responses
4. **Response Quality Metrics**: Track if auto-responses affect response quality
5. **Interactive Mode**: Fallback to user prompting with timeout warnings

## Implementation Priority

1. ✅ Basic query injection
2. ⬜ Session output monitoring
3. ⬜ Output analysis (completion/prompting detection)
4. ⬜ Auto-allow configuration loading
5. ⬜ Permission request handling
6. ⬜ Timeout management (progress vs prompt)
7. ⬜ Response extraction and cleaning
8. ⬜ Comprehensive logging
9. ⬜ Error handling and recovery
10. ⬜ Testing and validation