# NodeBB Data Structures, Tables and APIs

This document describes the core NodeBB data structures we'll use, how we'll extend them, and our storage strategy for metadata and derived data.

## NodeBB Core Data Structures

### Categories (Political Issues)
```javascript
// NodeBB category structure
category:<cid> = {
  "cid": 1,
  "name": "Healthcare",
  "slug": "healthcare",
  "description": "Discussions about healthcare policy",
  "parentCid": 0,  // Top-level category
  "topic_count": 45,
  "post_count": 892
}
```

### Topics (Position Statements)
```javascript
// NodeBB topic structure
topic:<tid> = {
  "tid": 123,
  "cid": 1,  // Category ID (Healthcare)
  "uid": 456,  // Author user ID
  "title": "Universal Healthcare Should Be Implemented",
  "slug": "universal-healthcare-should-be-implemented",
  "mainPid": 789,  // Main post ID
  "postcount": 23,
  "viewcount": 156,
  "locked": 0,
  "deleted": 0,
  "timestamp": 1234567890
}
```

### Posts (Arguments/Notes)
```javascript
// NodeBB post structure
post:<pid> = {
  "pid": 789,
  "tid": 123,  // Topic ID
  "uid": 456,  // Author user ID
  "content": "Universal healthcare would provide...",
  "timestamp": 1234567890,
  "toPid": 788,  // Parent post ID (for threading)
  "edited": 0,
  "editor": null,
  "deleted": 0,
  "upvotes": 12,
  "downvotes": 3
}
```

### Users
```javascript
// NodeBB user structure
user:<uid> = {
  "uid": 456,
  "username": "politicaluser",
  "email": "user@example.com",
  "joindate": 1234567890,
  "reputation": 125,
  "postcount": 89,
  // Custom fields we'll add:
  "politicalAffiliation": "progressive",  // Admin-assigned
  "isVerified": true
}
```

## Storage Strategy Analysis

### Option 1: Extend NodeBB Core Objects (Not Recommended)
**Pros:**
- Single source of truth
- No synchronization issues
- Minimal memory overhead

**Cons:**
- Mixing proprietary data with open-source system
- Risk of NodeBB updates breaking custom fields
- Harder to separate IP when scaling

### Option 2: Parallel Event Log (Recommended)
**Architecture:**
```javascript
// Event log entry
{
  "eventId": "evt_123456",
  "timestamp": 1234567890,
  "eventType": "post.created",
  "nodebbPid": 789,
  "metadata": {
    "argumentType": "support",
    "argumentLevel": 2,
    "parentArgumentId": 788,
    "politicalLean": "progressive"
  }
}
```

**Benefits:**
- Complete audit trail
- Event sourcing enables time-travel debugging
- Can rebuild state from any point
- Clean separation of proprietary data

### Option 3: Hybrid Approach (Best Choice)
Combine minimal NodeBB extensions with event log and derived data cache:

**NodeBB Extensions (via Plugin):**
```javascript
// Minimal custom fields on posts
post:<pid> = {
  ...standard NodeBB fields...,
  "argumentType": "support",  // Quick access field
  "metadataVersion": 3  // Version pointer
}
```

**Event Log Database:**
```javascript
// MongoDB collection: forum_events
{
  "_id": "evt_123456",
  "timestamp": 1234567890,
  "eventType": "post.created",
  "nodebbPid": 789,
  "fullContent": "Universal healthcare would...",  // Version tracking
  "metadata": {
    "argumentType": "support",
    "argumentLevel": 2,
    "ancestry": [123, 456, 788, 789]
  }
}
```

**In-Memory Derived Cache:**
```javascript
// Computed and cached in Redis/Memory
argumentTree = {
  "789": {
    "pid": 789,
    "children": [790, 791, 792],
    "supportCount": 15,
    "opposeCount": 8,
    "politicalDistribution": {
      "progressive": 12,
      "conservative": 3,
      "libertarian": 2,
      "centrist": 8
    },
    "synthesisScore": 0.73,
    "lastComputed": 1234567890
  }
}
```

## API Design

### NodeBB API Extensions (via Plugin)

**Get Post with Metadata:**
```javascript
GET /api/v3/posts/:pid/enhanced
Response: {
  ...standard NodeBB post...,
  "metadata": {
    "argumentType": "support",
    "versionCount": 3,
    "lastEditor": 456
  }
}
```

**Create Argument Post:**
```javascript
POST /api/v3/posts/argument
Body: {
  "tid": 123,
  "content": "I support this because...",
  "toPid": 788,
  "argumentType": "support"
}
```

### Proprietary Metadata Server APIs

**Get Argument Tree:**
```javascript
GET /metadata/api/posts/:pid/tree
Headers: {
  "Authorization": "Bearer <JWT>"
}
Response: {
  "pid": 789,
  "fullAncestry": [123, 456, 788, 789],
  "children": [...],
  "politicalMetrics": {...},
  "synthesisData": {...}
}
```

**Get Version History:**
```javascript
GET /metadata/api/posts/:pid/versions
Response: {
  "versions": [
    {
      "versionId": "v1",
      "content": "Original text...",
      "timestamp": 1234567890,
      "author": 456
    },
    ...
  ]
}
```

**Get Enhanced Post Display:**
```javascript
GET /metadata/api/posts/:pid/display
Response: {
  "postId": 789,
  "highlights": [
    {
      "start": 10,
      "end": 25,
      "commentId": "c1",
      "type": "support"
    }
  ],
  "annotations": [...],
  "politicalMetrics": {...},
  "navigationHints": {...}
}
```

## Data Flow Architecture

### Write Path
1. User creates/edits post in NodeBB
2. Plugin webhook fires to metadata server
3. Event logged with full content
4. Derived data updated in cache
5. Real-time updates pushed to clients

### Read Path
1. User requests post view
2. Plugin fetches NodeBB post data
3. Plugin requests metadata from server
4. Enhanced display rendered
5. Cached for performance

## Memory vs Database Trade-offs

### Keep in Memory:
- Frequently accessed derived data
- Argument tree structures
- Recent political metrics
- Active user session data
- Hot post metadata

### Store in Event Log Database:
- All events (creates, edits, votes)
- Version history
- Complete post content versions
- User action audit trail
- Political affiliation changes

### Store in NodeBB Database:
- Minimal argument type indicators
- Version pointers
- Basic relationship markers
- User political affiliation (admin-set)

## Synchronization Strategy

**Event-Driven Updates:**
```javascript
// NodeBB hook
NodeBB.on('post.create', async (postData) => {
  // 1. Add minimal fields to NodeBB
  await posts.setPostField(postData.pid, 'argumentType', 'support');

  // 2. Send to metadata server
  await metadataAPI.logEvent({
    type: 'post.created',
    data: postData
  });

  // 3. Update cache
  await cache.invalidate(`tree:${postData.tid}`);
});
```

## Performance Considerations

### Cache Strategy
- Redis for hot data (< 1 day old)
- Memory for active sessions
- Database for cold storage
- CDN for static metadata

### Query Optimization
- Denormalized read models
- Materialized argument trees
- Pre-computed political metrics
- Indexed event log by timestamp and type