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