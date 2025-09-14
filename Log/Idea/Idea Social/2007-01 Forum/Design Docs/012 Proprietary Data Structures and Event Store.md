# Proprietary Data Structures and Event Store

This document details the in-memory data structures and SQL event store for the proprietary metadata server that powers the political forum's advanced features.

## In-Memory Data Structures

### Note Types
This is a comprehensive list of note types in the system

#E## Root Note

#### Issue Note

#### Position Note

#### Part Note



### Enhanced Note Object
Each note in the unified tree has a corresponding in-memory object with computed and derived fields:

```javascript
notesCache = {
  "p789": {  // Key matches note ID (could be NodeBB pid, tid, cid, or virtual ID)
    // === Core Identity ===
    timestamp: 1234567890,       // Creation timestamp
    pid: 789,                    // Unique note ID (shared key)
    tid: 123,                    // NodeBB topic ID (if applicable)
    uid: 456,                    // Author user ID
    parentPid: 788,              // Direct parent in unified tree
    nodeType: "support",         // See Note Types below
    nodebbId: "pid:789",         // Reference to NodeBB object (pid:X, tid:X, cid:X, or null)

	// === DERIVED DATA ===
    // === Tree Structure ===
    fullAncestry: [0, 1, 123, 788, 789],  // Complete path from root
    depth: 4,                    // Level in unified tree
    children: [790, 791, 792],   // Direct child notes
    crossReferences: [555, 666], // Related notes elsewhere

    // === Version Tracking ===
    currentVersion: "v3",        // Latest version identifier
    versionCount: 3,             // Total number of edits
    lastEdit: 1234567890,        // Timestamp of last edit
    lastEditor: 789,             // User ID of last editor
    contentHash: "abc123...",    // Hash of current content

    // === Political Metrics ===
    supportCount: 15,            // Total support votes
    opposeCount: 8,              // Total opposition votes
    politicalDistribution: {
      progressive: 12,           // Support by affiliation
      conservative: 3,
      libertarian: 2,
      centrist: 1
    },
    tribalLean: 0.73,           // -1 (full oppose) to +1 (full support)

    // === Quality Metrics ===
    qualityScore: 0.82,         // Computed quality (0-1)
    consensusLevel: 0.65,       // Cross-tribal agreement (0-1)
    engagementScore: 0.91,      // Activity level (0-1)
    controversyIndex: 0.45,     // Disagreement level (0-1)

    // === Synthesis Data ===
    synthesisGroup: "sg_125",   // Cluster of similar arguments
    synthesisRole: "primary",   // primary | supporting | redundant
    mergedInto: null,           // If redundant, which note supersedes
    strengthRating: 0.88,       // Argument strength (0-1)

    // === Display Metadata ===
    highlights: [               // Text segments with annotations
      {
        start: 10,
        end: 25,
        commentIds: ["c1", "c2"],
        highlightType: "contested"
      }
    ],
    annotations: {              // Comments linked to highlights
      "c1": {
        uid: 999,
        text: "This claim needs evidence",
        type: "critique",
        timestamp: 1234567999
      }
    },

    // === Cache Metadata ===
    lastAccessed: 1234567999,   // For LRU cache eviction
    accessCount: 145,           // Heat metric
    cacheVersion: 3,            // Cache invalidation tracking
    isDirty: false              // Needs recomputation
  }
}
```

### User Political State
Track dynamic user political positioning:

```javascript
userPoliticalCache = {
  "456": {  // User ID
    uid: 456,

    // Admin-assigned (from NodeBB)
    declaredAffiliation: "progressive",

    // Computed tribal affiliations by issue
    tribalPositions: {
      "healthcare": {
        tribe: "universal_healthcare_supporters",
        strength: 0.89,  // How strongly aligned
        postCount: 12
      },
      "climate": {
        tribe: "aggressive_action",
        strength: 0.67,
        postCount: 8
      }
    },

    // Engagement patterns
    supportPatterns: {
      progressive: 145,  // Times supported progressive arguments
      conservative: 23,
      libertarian: 12,
      centrist: 45
    },

    // Quality metrics
    argumentQuality: 0.76,
    constructiveness: 0.88,
    bridgeBuilding: 0.34,  // Cross-tribal engagement

    // Activity
    lastActive: 1234567999,
    totalPosts: 89,
    totalVotes: 234
  }
}
```

### Topic Synthesis State
Aggregate state for each topic/issue:

```javascript
topicSynthesisCache = {
  "123": {  // Topic ID
    tid: 123,

    // Argument clusters
    argumentClusters: [
      {
        clusterId: "c1",
        position: "strong_support",
        memberPids: [789, 790, 791],
        bestArgument: 789,  // Highest quality in cluster
        consensusText: "Universal healthcare provides..."
      }
    ],

    // Political breakdown
    tribalDistribution: {
      progressive: { support: 145, oppose: 12 },
      conservative: { support: 23, oppose: 89 },
      libertarian: { support: 15, oppose: 45 },
      centrist: { support: 56, oppose: 34 }
    },

    // Key points extraction
    keyArguments: {
      support: [789, 812, 834],  // Top supporting arguments
      oppose: [790, 815, 836],   // Top opposing arguments
      nuanced: [791, 819, 841]   // Bridge-building arguments
    },

    // Synthesis metrics
    polarization: 0.72,  // How divided the discussion is
    maturity: 0.65,      // How developed the arguments are
    convergence: 0.23    // Whether consensus is emerging
  }
}
```

## SQL Event Store Schema

### Events Table
Primary event log capturing all state changes:

```sql
CREATE TABLE forum_events (
  -- Identity
  event_id BIGSERIAL PRIMARY KEY,
  event_uuid UUID NOT NULL DEFAULT gen_random_uuid(),
  timestamp TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,

  -- Event classification
  event_type VARCHAR(50) NOT NULL,  -- post.created, post.edited, vote.cast, etc.
  event_version INT NOT NULL DEFAULT 1,  -- Schema version

  -- Actor
  user_id INT NOT NULL,  -- Who triggered the event
  user_affiliation VARCHAR(20),  -- User's political affiliation at time of event

  -- Target
  post_id INT,  -- NodeBB pid if applicable
  topic_id INT,  -- NodeBB tid if applicable
  target_type VARCHAR(20),  -- post, topic, user, etc.

  -- Event payload (JSONB for flexibility)
  payload JSONB NOT NULL,
  -- Example payload for post.edited:
  -- {
  --   "oldContent": "Previous text...",
  --   "newContent": "Updated text...",
  --   "diff": "...",
  --   "editType": "major",
  --   "argumentTypeChanged": false
  -- }

  -- Metadata
  session_id VARCHAR(100),  -- For grouping related events
  ip_address INET,  -- For security/audit
  user_agent TEXT,  -- Client information

  -- Processing status
  processed BOOLEAN DEFAULT FALSE,  -- Has been applied to cache
  process_error TEXT,  -- Error if processing failed

  -- Indexes
  INDEX idx_timestamp (timestamp),
  INDEX idx_event_type (event_type),
  INDEX idx_post_id (post_id),
  INDEX idx_topic_id (topic_id),
  INDEX idx_user_id (user_id),
  INDEX idx_processed (processed)
);
```

### Event Types and Payloads

```sql
-- Post created event
INSERT INTO forum_events (event_type, user_id, post_id, topic_id, payload)
VALUES ('post.created', 456, 789, 123, '{
  "content": "Universal healthcare would...",
  "argumentType": "support",
  "parentPid": 788,
  "initialVersion": "v1"
}'::jsonb);

-- Vote cast event
INSERT INTO forum_events (event_type, user_id, post_id, payload)
VALUES ('vote.cast', 999, 789, '{
  "voteType": "support",
  "previousVote": "none",
  "strength": 0.8
}'::jsonb);

-- Post edited event
INSERT INTO forum_events (event_type, user_id, post_id, payload)
VALUES ('post.edited', 456, 789, '{
  "oldContent": "Universal healthcare would...",
  "newContent": "Universal healthcare systems provide...",
  "versionFrom": "v1",
  "versionTo": "v2",
  "editType": "minor"
}'::jsonb);
```

### Derived Tables (Materialized Views)

```sql
-- Current state view (latest version of each post)
CREATE MATERIALIZED VIEW post_current_state AS
SELECT DISTINCT ON (post_id)
  post_id,
  payload->>'content' as current_content,
  payload->>'argumentType' as argument_type,
  payload->>'versionTo' as current_version,
  timestamp as last_modified
FROM forum_events
WHERE event_type IN ('post.created', 'post.edited')
ORDER BY post_id, timestamp DESC;

-- Vote tallies
CREATE MATERIALIZED VIEW vote_summary AS
SELECT
  post_id,
  COUNT(*) FILTER (WHERE payload->>'voteType' = 'support') as support_count,
  COUNT(*) FILTER (WHERE payload->>'voteType' = 'oppose') as oppose_count,
  AVG((payload->>'strength')::float) as avg_strength
FROM forum_events
WHERE event_type = 'vote.cast'
GROUP BY post_id;
```

## Event Processing Pipeline

### Event Ingestion
```javascript
async function ingestEvent(eventData) {
  // 1. Write to SQL event store
  const event = await db.query(
    `INSERT INTO forum_events
     (event_type, user_id, post_id, topic_id, payload)
     VALUES ($1, $2, $3, $4, $5)
     RETURNING *`,
    [eventData.type, eventData.userId, eventData.postId,
     eventData.topicId, eventData.payload]
  );

  // 2. Update in-memory cache
  await updateCache(event);

  // 3. Trigger derived computations
  await recomputeMetrics(event);

  // 4. Emit real-time updates
  websocket.emit('event', event);
}
```

### Cache Rebuild from Events
```javascript
async function rebuildCache() {
  // Clear existing cache
  notesCache = {};
  userPoliticalCache = {};
  topicSynthesisCache = {};

  // Replay all events in order
  const events = await db.query(
    'SELECT * FROM forum_events ORDER BY timestamp ASC'
  );

  for (const event of events) {
    await applyEventToCache(event);
  }

  // Mark cache as rebuilt
  cacheGeneration++;
}
```

## Performance Optimizations

### Cache Eviction Strategy
```javascript
// LRU eviction when memory pressure
function evictLRU() {
  const sorted = Object.entries(notesCache)
    .sort((a, b) => a[1].lastAccessed - b[1].lastAccessed);

  // Keep hot data, evict cold
  const toEvict = sorted.slice(0, Math.floor(sorted.length * 0.2));

  for (const [pid, _] of toEvict) {
    delete notesCache[pid];
  }
}
```

### Event Batching
```javascript
// Batch process events for efficiency
const eventQueue = [];

setInterval(async () => {
  if (eventQueue.length > 0) {
    const batch = eventQueue.splice(0, 100);
    await processBatch(batch);
  }
}, 100); // Process every 100ms
```

## Data Consistency Guarantees

1. **Event Store is Source of Truth**: Can always rebuild from events
2. **Cache is Eventually Consistent**: May lag by milliseconds
3. **Read-after-Write Consistency**: User sees their own changes immediately
4. **Conflict Resolution**: Last-write-wins with full audit trail
5. **Crash Recovery**: Rebuild cache from last checkpoint + new events