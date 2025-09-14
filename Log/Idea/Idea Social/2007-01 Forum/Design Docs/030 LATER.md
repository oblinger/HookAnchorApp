# LATER - Future Implementation Items

This document captures functionality to be implemented in later phases of development.

## Missing Actions

### **1. Content Quality Voting**
**Gap:** Reader story "VOTE - vote on content quality" isn't covered by current actions
**Missing Action:** `noteVote` - Simple upvote/downvote separate from support/oppose
**Purpose:** Voting = "this is well-written/useful" vs Support = "I agree with this position"

### **2. Moderator Content Management**  
**Gap:** Moderator stories about content management aren't covered
**Missing Actions:**
- `adminReviewFlags` - Review and act on flagged content
- `adminWarnUser` - Send warnings to users for behavior violations
- `adminMovePost` - Reorganize posts between topics to maintain organization
- `adminLockThread` - Temporarily freeze discussions during heated debates

### **3. Content Discovery and Filtering**
**Gap:** No way to surface "highest-rated" or "best arguments" per user stories
**Missing Actions:**
- `navSortByRating` - Sort content by quality votes vs chronological order
- `navFilterByPerspective` - Filter to show only progressive/conservative viewpoints
- `navBestArguments` - Surface highest-rated content across discussions

### **4. Suggested Edit Workflow**
**Gap:** Position Author story "ACCEPT CHANGES" needs complete workflow
**Missing Actions:**
- `noteReviewSuggestions` - Review pending edit suggestions from other users
- `noteAcceptEdit` - Accept a suggested change and apply to original note
- `noteRejectEdit` - Reject a suggested change with optional feedback

### **5. Specialized Feed Customization**
**Gap:** Argument Refiner story "FEED READER - see other refinements being proposed"
**Missing Actions:**
- `navFeedFilterByType` - Filter feed to show only refinements, rebuttals, alternatives, etc.
- `navFeedPersonalized` - Customized feed based on user type and political interests
- `navFeedByContribution` - Show feed filtered by contribution type (assertions, references, subclaims)

## Missing Data Elements

### **Quality Voting System**
**Gap:** User stories mention voting on "content quality" separate from support/oppose
**Missing Data:**
- `note_votes` table - Track quality votes (user_id, note_id, vote_direction, timestamp)
- `quality_score` field on NOTE - Aggregate quality rating
- `vote_type` enum - Distinguish quality votes from support/oppose

### **Watch/Notification Tracking**
**Gap:** noteToggleWatch action needs data persistence
**Missing Data:**
- `user_watches` table - Track which notes users are watching (user_id, note_id, created_at)
- `notification_queue` table - Store pending notifications for users


### **Comment Note Type**
**Gap:** Actions describe neutral comments but ontology only has support/oppose
**Missing Data:**
- Add `comment` to note_type enum
- `NOTE.comment` specialization for neutral contributions

### **Reference URL Handling**
**Gap:** External links need structured storage for assertions and references
**Missing Data:**
- `reference_urls` array field in NOTE metadata
- `url_title` and `url_description` for rich link previews

## Implementation Priority
These actions should be prioritized based on:
1. Core user workflow completion (suggested edit workflow)
2. Content quality and moderation (voting, flagging, warnings)
3. Enhanced discovery and navigation (filtering, sorting)
4. Specialized user experience (personalized feeds)


## DETAILS RELATED TO ALTERNATES


## Note Group (Later)
A note group is a set of notes that the system treats as variants for the same concept. 
- VARIANT NOTES - When a note is created, it can be designated as the variant of another note.
- VARIANT GROUP - Variant designation can be done recursively, generating a tree of variant notes that reference each other. It does not matter which Note is the root of this tree, and in general, the structure of the tree does not matter. We think of all the notes within this tree collectively as a single group of notes, which we call a Note group.



## Nomination
- NOTATION - We use the notation `nominate(u, n)` to indicate that user u is presently nominating note n among all variants for n.
- MOST RECENT VOTE - A user, u, is nominating a note, n, from a set of notes S if the most recent vote for any note in S is a positive vote for n.
- 
A note is nominated by a user 
- NOMINATED VARIANT - The variant receiving the most valid nominations is designated as the nominated variant for the group.  (see below for 'valid nominations')



## Issue Positions
- **USER VOTING** - A user's position on an issue is defined by the most recent support vote they have given to any position on that issue. 
- **POSITION GROUPINGS** - Any support given to a variant of a position is equivalent to providing support to the Root position itself. They represent an equivalence class of positions; support for one of these variants is support for all of them.
- **USER POSITION** - Thus, each user has a defined position on every issue. They are labelled as supporting whichever position group they most recently supported, or they are labelled as having no position on the issue if they have not added support for any position or if they most recently voted to support the "no position" on the issue.
- **NOTE POSITION** - Each note under an issue will also have a defined position regarding that issue.  
	- Each position supports it own position group.
	- Any sub-note whose stance is "support" for a note that supports a position, also supports that same position.
	- Any sub-note whose stance is "oppose" 
- NOMINATED POSITION NOTE - The position note within a group of variants for a given position that has the absolute largest number of nominations among those users holding this position becomes the nominated note for that position group. Which ever position Note most recently has support from a given user is the nomination from that user on this position. A user may also nominate no position on an issue in order to remove their nomination for any position on the issue.
- Each note below an issue can thus be labeled as supporting, opposing, or commenting on a given position.  
