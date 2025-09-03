# Task 101: Cloud Document Scanning (Notion & Google Drive)

## Overview
Implement document scanning for both Notion and Google Drive to index titles and locations of cloud documents alongside filesystem roots.

## Approach
- **Notion**: Use official API for efficient bulk access
- **Google Drive**: Use Google Drive API v3 for folder traversal and metadata

## Configuration Design
```yaml
scan_roots:
  - type: filesystem
    path: ~/Documents
  - type: notion
    root: ""  # Empty = scan all accessible pages, or specify workspace/page ID
    api_key: "${NOTION_API_KEY}"
  - type: google_drive
    root: "root"  # 'root' for My Drive or specific folder ID
    credentials: "${GOOGLE_CREDENTIALS_PATH}"
```

## Implementation Steps

### Phase 1: Logging Only (Current Task)
1. Add API clients (Notion SDK, Google APIs)
2. Extend scanner to handle different root types
3. Implement traversal for each service that ONLY logs findings:
   - Filesystem: Keep existing behavior unchanged
   - Notion: Log each page found (one line per page)
   - Google Drive: Log each document found (one line per doc)
4. Log format: `[SERVICE] path/to/document - Title (ID: xxx, Modified: YYYY-MM-DD)`

### Phase 2: Future Integration
- Unify indexing between filesystem, Notion, and Drive items
- Add documents to the actual anchor index

## Notion API Endpoints
- `GET /search` - Find all accessible pages
- `GET /databases/{id}/query` - List database entries
- `GET /blocks/{id}/children` - Traverse page hierarchies

## Google Drive API Endpoints
- `files.list` - List files/folders with query filters
  - Query: `'folder_id' in parents and trashed = false`
  - Fields: `id, name, mimeType, modifiedTime, parents`
- `files.get` - Get specific file metadata
- Pagination via `pageToken` for large folders

## Data to Index
- Document title/name
- Parent folder/page hierarchy
- Last modified time
- URL/ID for quick access
- Document type (notion page, gdoc, gsheet, folder, etc.)

## Authentication
### Notion
- API key from integration settings
- Workspace-level permissions

### Google Drive
- OAuth 2.0 flow for user consent
- Service account for server-side access
- Scopes: `drive.readonly` or `drive.metadata.readonly`

## Performance Considerations
- **Notion**: Rate limit of 3 requests/second
- **Google Drive**: 
  - 1000 requests per 100 seconds per user
  - Batch requests supported (up to 100 calls)
- Implement caching and incremental updates
- Use parallel requests where possible

## Example Log Output
```
[NOTION] /Engineering/Design Docs - System Architecture (ID: abc123, Modified: 2025-09-01)
[NOTION] /Engineering/Design Docs/API Design - REST API v2 (ID: def456, Modified: 2025-08-30)
[GDRIVE] /Projects/Q3 Planning - Quarterly Goals (ID: 1x2y3z, Modified: 2025-09-02)
[GDRIVE] /Projects/Q3 Planning/Budget.gsheet - Budget Spreadsheet (ID: 4a5b6c, Modified: 2025-08-28)
```

## Testing Approach
1. Start with minimal API permissions (read-only)
2. Test with a small subset of documents first
3. Verify log output format and completeness
4. Monitor API usage to avoid rate limits
5. Ensure no changes to existing filesystem scanning