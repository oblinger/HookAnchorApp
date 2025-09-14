# NodeBB Integration Architecture

This document describes how the political discussion forum will integrate with and extend the NodeBB platform.

## Core Integration Commitments

### **Commitment 1: Issues as NodeBB Categories**

Forum issues will be represented as NodeBB categories.

- Each political issue becomes a top-level NodeBB category
- Examples: "Healthcare", "Climate Policy", "Economic Policy", "Immigration"
- Leverages NodeBB's existing category management, permissions, and organization
- Allows standard NodeBB category-level moderation and access controls
- Provides natural organizational structure for position statements and discussions

### **Commitment 2: Unread Messages with Specialized Filtering for News Feeds**

The forum will use NodeBB's unread message system with specialized filtering to implement both default and contextual news feeds.

**Default News Feed:**
- Uses NodeBB's native unread message tracking
- Shows unread content from all subscribed categories/topics
- Respects user's subscription preferences
- Leverages existing NodeBB notification and tracking infrastructure

**Contextual News Feeds:**
- Custom filtering of unread messages based on current tree position
- Shows unread content from current node and all descendants
- Available from any point in the discussion hierarchy
- Examples:
  - At "Healthcare" category: Shows all unread healthcare discussions
  - At "Universal Healthcare" topic: Shows all unread arguments in that position tree
  - At specific argument: Shows all unread rebuttals/refinements to that argument

**Technical Implementation:**
- Extends NodeBB's existing read/unread tracking system
- Custom query logic for contextual filtering
- Maintains performance through proper indexing and caching
- Preserves NodeBB's subscription and notification mechanisms

### **Commitment 3: Generic Enhanced Note Viewer Released as Plugin**

The forum will use a generic, open-source NodeBB plugin that provides enhanced note display capabilities without containing proprietary business logic.

**Plugin Capabilities:**
- Replaces default NodeBB post display with interactive note viewer
- Renders highlighted text segments with connected annotations
- Displays comments, references, and metadata linked to specific text intervals
- Provides dynamic navigation through argument tree structures
- Supports voting/support/oppose UI elements with vote counts
- Handles real-time updates of political metrics and engagement data

**Generic Design:**
- Consumes standardized metadata packets from external APIs
- No hardcoded political logic or scoring algorithms
- Template override system that works with any NodeBB installation
- Configurable annotation types and display styles
- RESTful API client for fetching enhancement data
- Open source release builds community trust and adoption

**Technical Architecture:**
- Uses NodeBB's `filter:parse.post` and template override hooks
- JavaScript-based interactive UI with highlighting and navigation
- Metadata packet structure defines all annotations and enhancements
- Plugin serves as pure rendering engine without business logic

### **Commitment 4: Proprietary Voting and Structure Analysis Metadata Server**

A separate, private backend service will handle all proprietary political analysis, vote calculations, and argument structure computation.

**Core Responsibilities:**
- Political affiliation tracking and tribal analysis algorithms
- Vote synthesis and support/opposition calculations
- Argument tree relationship computation and hierarchy building
- Dynamic political metrics generation (progressive/conservative ratios)
- Position refinement and alternative tracking logic
- User engagement scoring and quality assessments

**Metadata Generation:**
- Processes NodeBB post data to generate enhancement packets
- Calculates real-time political metrics and vote distributions
- Builds argument relationship maps and tree structures
- Identifies text intervals requiring annotations
- Generates comment/reference linkage data
- Produces contextual navigation hints for tree traversal

**API Design:**
- JWT Token Forwarding for autentication.
- RESTful endpoints serving standardized metadata packets
- Webhook integration for real-time NodeBB post updates
- Scalable architecture supporting multiple forum instances
- Authentication and rate limiting for API access
- Caching layer for performance optimization

**IP Protection:**
- All proprietary algorithms and scoring logic remain private
- Voting synthesis and political analysis methods protected
- Monetization through backend licensing model
- No business logic exposed in public plugin code

### **Commitment 5: Hybrid Approach to Post Relationships and Hierarchy**

The forum will use a combination of NodeBB's native post threading capabilities and external metadata tracking to implement hierarchical argument structures.

**NodeBB Native Capabilities Used:**
- Leverages `toPid` field for immediate parent-child post relationships
- Uses NodeBB's basic threading support for single-level replies
- Extends posts with custom fields via plugin architecture
- Maintains compatibility with NodeBB's standard discussion flow

**Custom Extensions via Plugin:**
- Adds `argumentType` field to posts (support, oppose, refinement, alternative)
- Adds `argumentLevel` field to track depth in argument hierarchy
- Adds `parentArgumentId` for complex multi-parent relationships
- Hooks into post creation/editing events for metadata synchronization

**External Metadata Server Tracking:**
- Complete argument tree paths and multi-level hierarchies
- Full ancestry tracking beyond immediate parent relationships
- Cross-reference relationships between related arguments
- Argument clustering and relationship mapping
- Political lean calculations based on tree position

**Data Synchronization:**
```javascript
// NodeBB post data
{
  "pid": 12345,
  "toPid": 11111,           // Native NodeBB parent reference
  "argumentType": "support", // Custom field added by plugin
  "content": "I agree because..."
}

// Metadata server computes and stores
{
  "postId": 12345,
  "fullAncestryPath": [1000, 5000, 11111, 12345],
  "argumentDepth": 3,
  "crossReferences": [9999, 8888],
  "politicalMetrics": {...}
}
```

### **Commitment 6: External Version History and Edit Tracking**

Since NodeBB lacks comprehensive version history, the forum will implement external versioning through the metadata server.

**NodeBB Limitations Acknowledged:**
- NodeBB only tracks that edits occurred and timestamps
- No native content version history or diff viewing
- Cannot revert to previous versions through NodeBB alone
- Edit tracking shows who edited and when, but not what changed

**External Version System Implementation:**
- Metadata server captures full post content on every edit
- Stores complete version history with timestamps and authors
- Implements diff generation between versions
- Provides version comparison and rollback capabilities
- Tracks edit patterns for quality and moderation purposes

**Version Tracking Architecture:**
- Webhook fires on NodeBB post edit events
- Metadata server retrieves and stores new content version
- Maintains version chain with parent-child version relationships
- Exposes version history through API for plugin display

**Integration with Enhanced Note Viewer:**
- Plugin can display version history timeline
- Shows diff highlights between selected versions
- Indicates controversial edits or major changes
- Provides UI for viewing historical versions

**Technical Implementation:**
```javascript
// Version history structure in metadata server
{
  "postId": 12345,
  "versions": [
    {
      "versionId": "v1",
      "content": "Original content...",
      "timestamp": 1234567890,
      "authorId": 123,
      "editType": "minor"
    },
    {
      "versionId": "v2",
      "content": "Edited content...",
      "timestamp": 1234567990,
      "authorId": 123,
      "diff": "... diff from v1 ...",
      "editType": "major"
    }
  ],
  "currentVersion": "v2"
}
```
