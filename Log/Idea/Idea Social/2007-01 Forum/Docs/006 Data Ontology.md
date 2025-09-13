# Data Ontology

## Core Data Types

### USER - Represents a participant in the system with political affiliation and roles.
- `user_id` (string) - Unique identifier [NodeBB: uid]
- `username` (string) - Display name
- `email` (string) - Contact email
- `tribes` (array of strings) - Political/ideological affiliations (e.g., ["progressive", "libertarian", "environmental"])
- `roles` (array) - System roles (admin, moderator, participant)
- `created_at` (timestamp) - Account creation date
- `last_active` (timestamp) - Last activity timestamp

### NOTE - Universal content type that maps to NodeBB posts, represents all user contributions.
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