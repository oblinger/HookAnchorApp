# REM (Remote Execution Manager) Specification v1

## Overview
A bidirectional file transfer and remote execution system between GCP and local Mac, using reverse SSH tunnels. The same Python script (`r`) runs on both sides, with different components active based on context.

## Script Name and Usage
- **Script name**: `r` (lowercase)
- **Shebang script**: `#!/usr/bin/env python3`
- Dual usage:
  - Command-line: `r <command> [args]`
  - Python import: `import r; r.pp(data)`

## Commands

### 1. SSH Command (Mac side only)
```bash
r ssh
```
- Captures and displays current directory as "Execution Directory"
- Displays editable gcloud SSH command with default arguments
- Default template:
  ```
  gcloud compute ssh --project "ai-worker-launcher" --zone "us-east1-b" --ssh-key-file=~/.ssh/google_compute_engine ubuntu@dotest
  ```
- User can edit the line (especially ubuntu@dotest at the end) before hitting return
- Automatically appends SSH tunnel flags for reverse/forward ports (hidden from user view)
- Starts background servers for receiving files and commands
- Example output:
  ```
  Execution Directory: /Users/oblinger/projects/data
  SSH Command: gcloud compute ssh --project "ai-worker-launcher" --zone "us-east1-b" --ssh-key-file=~/.ssh/google_compute_engine ubuntu@dotest
  ```

### 2. Change Directory Command (Mac side only)
```bash
r cd <path>
```
- Changes the execution directory for file operations
- All subsequent file receives and command executions use this directory
- Prints confirmation: `Execution Directory changed to: <path>`

### 3. File Operations (GCP side)

#### Send File (GCP → Mac)
```bash
r send <filepath> [destination]
```
- Sends file from GCP to Mac
- Optional destination path on Mac

#### Get File (Mac → GCP)
```bash
r get <mac_filepath> [local_destination]
```
- Retrieves file from Mac to GCP
- Optional local destination path

#### Open File (GCP → Mac)
```bash
r open <filepath>
```
- Convenience command that combines send + view
- Sends file from GCP to Mac
- Automatically opens the file using Mac's default application
- Equivalent to: `r send <filepath>` followed by `r local view <filepath>`

### 4. Remote Execution (GCP → Mac)

#### Local Execute Commands
```bash
r local <action> <args>
```

Current actions:
- **view**: Opens file on Mac using system default application
  ```bash
  r local view <filepath>
  ```
  - Uses Mac's `open` command to launch appropriate viewer based on file type

**Note**: The `local` command is primarily intended to be called remotely from GCP, but can be executed directly on Mac for testing/completeness.

**Important**: Command ordering is preserved - if `send` followed by `local view`, the send completes before view executes.

### 5. Pretty Print (Python API, GCP side)

```python
import r
r.pp(data_structure)
```
- Pretty prints Python data structure with dense formatting
- Uses `pprint` with `compact=True` and optimized width
- Workflow:
  1. Pretty prints to temp file on GCP
  2. Uploads temp file to Mac
  3. Automatically calls `local view` to open the file

## Execution Directory Concept

The "Execution Directory" is the base directory for all file operations on the Mac side:
- Set initially when `r ssh` is executed (captures current directory)
- All received files are placed relative to this directory
- All commands execute with this as the working directory
- Can be changed with `r cd <path>` command
- NOT the literal current directory - maintained separately by the `r` script

## Command Organization

### Mac-side Commands (CLI only)
- `r ssh` - Initialize connection with tunnels
- `r cd <path>` - Change execution directory

### GCP-side Commands (CLI)
- `r send <file>` - Send file to Mac
- `r get <file>` - Get file from Mac
- `r open <file>` - Send and open file on Mac (convenience command)
- `r local view <file>` - Execute view command on Mac (future: other `local` actions)

### GCP-side Commands (Python import)
```python
import r
r.pp(data)  # Pretty print and view on Mac
```

## Architecture

### Ports and Tunnels
- **8888**: File transfer (GCP → Mac)
- **8889**: Command execution (GCP → Mac)
- **8890**: File retrieval (Mac → GCP)

### Mac Side Components
- HTTP server listening on localhost:8888 for file receives
- Command execution server on localhost:8889
- File serving server on localhost:8890

### GCP Side Components
- Client functions for send/get/local operations
- Pretty print formatter with auto-upload

## Implementation Notes

### Code Organization
The Python file should be clearly organized with sections:
```python
# === MAC-SIDE CLI COMMANDS ===
# r ssh, r cd

# === GCP-SIDE CLI COMMANDS ===
# r send, r get, r local

# === GCP-SIDE PYTHON IMPORT API ===
# r.pp()

# === SHARED UTILITIES ===
# Common functions used by both sides
```

### SSH Connection Management
- Base gcloud command stored as template
- SSH tunnel flags added programmatically:
  ```
  --ssh-flag="-R" --ssh-flag="8888:localhost:8888"
  --ssh-flag="-R" --ssh-flag="8889:localhost:8889"  
  --ssh-flag="-L" --ssh-flag="8890:localhost:8890"
  ```

### File Transfer Protocol
- HTTP POST with custom headers:
  - `X-Filename`: Target filename
  - `X-Action`: Action type (file/command)
  - `Content-Type`: File mime type

### Command Queue
- Commands processed sequentially
- Acknowledgment required before next command
- Ensures ordering guarantees

## Future Extensions
- Additional `local` actions beyond `view`
- Bidirectional command execution
- File sync/watch capabilities
- Progress indicators for large files
- Compression for transfers

## Usage Examples

### Basic Workflow
```bash
# On Mac
r ssh
# [Edit and run the gcloud SSH command]

# On GCP
r send results.json
r open results.json

# In Python on GCP
import r
data = {'results': [1, 2, 3], 'status': 'complete'}
r.pp(data)  # Auto-sends and opens on Mac
```

### File Round-trip
```bash
# Get file from Mac, process it, send it back
r get ~/data/input.csv
python process.py input.csv output.csv
r send output.csv ~/data/output.csv
r open ~/data/output.csv
```