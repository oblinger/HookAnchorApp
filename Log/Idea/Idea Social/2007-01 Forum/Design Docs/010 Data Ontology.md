# Data Ontology
Here are the core data objects underlying the forum platform.

## USER
Represents a participant in the system with political affiliation and roles.
- `user_id` (string) - Unique identifier [NodeBB: uid]
- `username` (string) - Display name
- `email` (string) - Contact email
- `tribes` (array of strings) - Political/ideological affiliations (e.g., ["progressive", "libertarian", "environmental"])
- `roles` (array) - System roles (admin, moderator, participant)
- `created_at` (timestamp) - Account creation date
- `last_active` (timestamp) - Last activity timestamp

## NOTE
Universal content type that maps to NodeBB posts, represents all user contributions.

### Source Data
### nid (string) - Unique Note Identifier
NIDs are guaranteed to be unique across the system.
- By convention, they are expressed as a single letter prefix and then a number.
- Prefix letters:
  - 'r' - Root node (virtual, nid = "r0")
  - 'c' - NodeBB category (e.g., "c1" maps to cid:1)
  - 't' - NodeBB topic (e.g., "t123" maps to tid:123)
  - 'p' - NodeBB post (e.g., "p789" maps to pid:789)
  - 'v' - Variant note (e.g., "v1001" for alternative phrasings)

### parentNid (string)
The nid of this note's parent in the unified tree hierarchy.
- Root node: parentNid = null
- Categories: parentNid = "r0"
- Topics: parentNid = category nid (e.g., "c1")
- Posts: parentNid = topic nid or another post nid for threading
- Variants: parentNid = the note they're a variant of

### noteType (string)
Enumeration defining the note's role and behavior in the system:
- **Structural**: `root`, 
- **Issue** - Maps onto NodeBB Categories
- **Position** - Maps onto Topics
- **Rebuttal** - 
- **Contribution** - 
- **Vote** - 

### uid (string)
NodeBB user ID of the note's author. Maps to NodeBB uid field.

### timestamp (number)
Unix timestamp when the note was created.

### version (number)
Current version number, incremented with each edit.

### metadata (object)
Type-specific additional data stored as JSON object. 


## Note Derived Accessors

### parentNote (Note)
The parent note object, derived by looking up parentNid in the notes cache.

### childrenNotes (List of Note)
List of all direct children, derived by maintaining an inverse mapping of parentNid relationships.

### content (text)
Markdown-formatted text content of the note.  Derived from the associated nodeBB entry.

### pid, cid, tid (number)
NodeBB-specific IDs, extracted from nid and ancestry:
- **pid**: If nid starts with 'p', extract the numeric portion
- **tid**: Walk up ancestors to find first nid starting with 't'
- **cid**: Walk up ancestors to find first nid starting with 'c'

### nodebbReference (object)
Computed reference to the corresponding NodeBB object:
- Categories: `{type: "category", id: 1}`
- Topics: `{type: "topic", id: 123}`
- Posts: `{type: "post", id: 789}`
- Virtual/variants: null


## Note Vote Related Accessors

### votes (List of Note)
List of vote objects

### supportCount (number)
Total support votes, computed from aggregating vote events in the event log.

### opposeCount (number)
Total opposition votes, computed from aggregating vote events in the event log.




## Note Vote Fields

### stance (string)
Enum: support, oppose


### perspective
Enum: True, Mostly True, Misleading, Mostly False, False




# OLDER INFO


### politicalDistribution (object)
Vote breakdown by political affiliation, computed from user votes and their declared affiliations:
```javascript
{
  progressive: 12,
  conservative: 3,
  libertarian: 2,
  centrist: 1
}
```

### qualityScore (number)
Computed quality metric (0-1) based on engagement patterns, constructiveness indicators, and cross-tribal support.

### consensusLevel (number)
Cross-tribal agreement metric (0-1), computed from political distribution of support votes.

### tribalLean (number)
Political lean indicator (-1 to +1), derived from weighted political distribution.

### versionCount (number)
Total number of edits, derived from counting edit events in the event log.

### lastEdit (timestamp)
Timestamp of most recent edit, derived from latest edit event.

### synthesisData (object)
For synthesis-type notes, contains aggregated analysis data:
```javascript
{
  sourceNotes: ["p789", "p790"],
  keyPoints: ["Universal coverage", "Cost reduction"],
  algorithmVersion: "2.1"
}
```





- `note_id` (string) - Unique identifier [NodeBB: pid]
- `parent_note_id` (string) - Parent note (null for root topics)
- `author_id` (user_id) - Author of note
- `note_type` (enum) - topic/issue/position/contribution/alternative/edit_proposal/flag_report
- `stance` (enum) - "support" | "oppose" | "none" (neutral/not applicable)
- `subtype` (string) - Optional subtype, meaning depends on note_type
- `short_name` (string) - Concise reference name, 1-3 words
- `title` (string) - Note headline
- `content` (text) - Markdown-formatted content  
- `metadata` (object) - Type-specific data (vote_direction, edit_diffs, flag_reason, evidence_links, etc.)
- `version` (number) - Version number for edited content
- `created_at` (timestamp) - Creation date
- `updated_at` (timestamp) - Last modification
- ? `status` (enum) - active/edited/flagged/deleted

## Note Type Specializations

### NOTE.topic - Organizational topics that can contain other topics, issues, or positions
- Uses: `title`, `content` (description)
- Parent: Another topic (subtopic) or null (root topic)
- Forms DAG structure of topics at all levels

### NOTE.issue - Specific issue that can have positions taken on it
- Uses: `title`, `content` (issue description)
- Parent: Topic (usually a leaf node in topic DAG)
- Special topic type that accepts positions

### NOTE.position - Stance taken on an issue
- Uses: `title`, `content`
- Parent: Issue (note_type: issue)

### NOTE.stance - Indicates user's stance toward the parent note (support/opposition/none)
- Uses: `content`, `stance`, `subtype`
- Parent: Any note (the note being responded to)
- Stance: "support" | "oppose" | "none" (neutral comment)
- Only one note per user per parent note (editable)
- Only the last note

### NOTE.contribution - Supporting evidence, opposition, or neutral comment
- Uses: `content`, `stance`, `subtype`
- Parent: Any note (the note being responded to)
- Stance: "support" | "oppose" | "none" (neutral comment)
- Subtype (information_type): "comment" | "assertion" | "reference" | "subclaim"
- One contribution per user per parent note (editable)

### NOTE.alternative - Alternative phrasing or variant of existing content
- Uses: `content`, `metadata.original_note_id`
- Parent: Same as original note's parent

### (LATER) NOTE.edit_proposal - Suggested edit to existing content
- Uses: `content`, `metadata.original_content`, `metadata.diff_data`, `metadata.target_note_id`
- Parent: Note being edited

### NOTE.flag_report - Content flagging for moderation
- Uses: `content` (explanation), `metadata.flag_reason`, `metadata.target_note_id`
- Parent: Note being flagged

## Relationships

### Primary Relationships
- Users → Positions (one-to-many: authorship)
- Topics → Positions (one-to-many: categorization)
- Positions → Arguments (one-to-many: debate tree)
- Arguments → Arguments (one-to-many: nested rebuttals)
- Users → Votes (one-to-many: feedback)
- Positions → Edits (one-to-many: version history)

### NodeBB Mapping Strategy
- Users map to NodeBB users with custom fields
- Topics map to NodeBB categories
- Positions map to NodeBB topics (first post)
- Arguments map to NodeBB posts (replies)
- Votes map to NodeBB reputation/voting system
- Custom metadata stored in post/user fields