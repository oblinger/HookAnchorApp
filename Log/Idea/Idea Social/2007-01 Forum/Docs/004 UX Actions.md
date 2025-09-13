# UX Actions

This document describes the specific actions and interactions available within each major interface and UX element of the political discussion forum.

## Administrative Actions

### **adminCreateUser** - Create new user accounts

- Creates new user record in the system
- Allows admin to pre-fill basic profile information (name, email, tribal affiliations)
- Assigns initial role (participant, moderator, or admin)
- Automatically generates and sends invitation email to the new user
- User receives secure link to complete registration and set password
- Tracks invitation status (pending, accepted, expired)

### **adminEditUser** - Edit existing user properties and management

- Opens user dashboard showing all user properties (name, email, roles, tribal affiliations)
- Allows editing of any user field including profile information and permissions
- Displays user activity summary and engagement statistics
- Shows invitation status and registration completion date
- **Delete button** - Permanently removes user account and all associated data
- **Resend Invite button** - Generates and sends new invitation email for pending users
- Tracks all administrative changes with timestamp and admin user log


## Note Actions

### **noteDoubleClick** - Opens the reference note in the Note Viewer.

### **noteToggleSupport** / **noteToggleOppose** - Add or remove support/opposition to a position
- TOGGLES -  The toggle support and opposition buttons are both push buttons. It can be clicked on and off, and if you click on one of them, it implicitly clicks the other one off.
- ONE NOTE - there is in most one note which represents either support or opposition for a given user on a given note. So if the user expresses multiple opinions, it's the last one that will count, and the text will be the text associated with the last one.

### **noteOpenAddMenu** - Pops up the add menu
### **noteOpenNavMenu** - Pops up the nav menu



### **noteAddComment** / **noteAddReference** / **noteAddAssertion** - Contribute Info
- INFO - each of these contributes, a kind of information to the parent note. A comment is a general comment, a reference is some URL or book reference, and an assertion is a claim about an asserted fact, typically also with some kind of cited authority for the fact.
- INTERVAL - (LATER) each of these contributions may or may not be associated with an interval of text within the parent note. If it is associated with an interval of text in the parent, then hovering over that interval, which has some kind of highlighting on it or highlight the associated information on the right hand side, where the contributions are listed. And if one hovers over any given contribution the interval that it's associated with will be highlighted.
- COMMENT - comments are special in that there may be an most one comment from a given user on a given parent note. If that same user or to make another comment, they would simply be opened on the editor showing their previous comment and they can edit it and then save it again.

### **noteAddSubclaim** - Add supporting subsidiary argument
- HIERARCHY - Creates a new note that supports the parent note's position with additional reasoning or evidence
- STRUCTURE - The subclaim appears as a child node in the discussion tree, maintaining logical argument flow
- OWNERSHIP - Each user can contribute multiple subclaims to strengthen different aspects of the main argument


### **noteEdit** - Suggest tracked changes to existing content  
- EDIT NOTE - Open the editor on the current note. If this user has the rights to edit the note, they will simply be editing the previous version of the note and when they save the note, this will become the new version of this note.
- SUGGEST EDIT - if they don't have the rights to edit the note, then it will take the existing contents of the note and prepend in all caps "SUGGESTED EDIT" and it will simply create a comment on the note. The notes owner can read that comment and decide what to do with it. The user may highlight one area that they like to make change to, or they may just rewrite the whole thing. Either way, it will be up to the original notes owner to make any changes based on this comment.
- SUGGEST EDIT (LATER) - later, we will allow an editor with a track changes functionality where are specific edits can be saved and approved by the original author.
- VERSIONING - Maintains history of all suggested edits and their acceptance/rejection status.


### **noteAddAlternative** - Propose alternative phrasing or approach
- VARIANT - Creates a competing version of the same position with different wording or reasoning
- CHOICE - Users can support either the original or alternative version, allowing refinement through selection
- EVOLUTION - Popular alternatives may become the preferred version of a position over time


### **noteFlag** - (LATER) Report content for moderation review
- MODERATION - Flags content that may violate community guidelines or forum rules
- REVIEW - Sends the flagged content to moderators for evaluation and potential action
- CATEGORIES - Allows selection of flag reason (spam, harassment, misinformation, etc.)


### **noteToggleWatch** - Subscribe to or mute notifications
- TOGGLE - Simple two-state toggle: not watching ↔ watching
- NOTIFICATIONS - When watching, user receives alerts for new replies, support, or opposition
- SILENCE - When not watching, user receives no notifications for this note



## Navigate Actions

### **navBack** - Navigate back to the previous view
- Generally, this will navigate back to the previous view, whatever that was. That will go back to the viewer on the Notes that have been looked at recently. For example, it will also go back into the newsfeed, if that's where you came from in general, it's going to go back to the view you were on before.

### **navForward** - Navigate forward to the next view
- Navigate forward in the browser history to views that were visited after using the back button. This follows standard browser navigation patterns, allowing users to move forward through their navigation history after going back.

### **navNote** - Switch to view given note
- OPEN VIEWER - Open the viewer on a given note. Really this is a sub action that is triggered by other actions.

### **navParentNote** - Navigate to parent note in hierarchy
- TRAVERSAL - Moves up one level in the discussion tree to view the note that this one responds to
- CONTEXT - Helps users understand the broader conversation flow and argument structure
- BREADCRUMB - Updates navigation breadcrumbs to reflect the new position in the hierarchy


### **navTreeView** - (LATER) Show note's position in topic tree
- OVERVIEW - Displays a visual tree structure showing where the current note fits in the topic hierarchy
- NAVIGATION - Allows clicking on any node in the tree to jump directly to that note
- PERSPECTIVE - Provides bird's-eye view of the entire discussion structure and argument flow


### **noteShare** - (LATER) Share link to note via external platforms
- PLATFORMS - Opens sharing options for social media, email, or messaging apps
- PERMALINK - Generates a permanent link that will always point to this specific note
- CONTEXT - Shared links include enough context for external viewers to understand the note's position


### **noteCopyLink** - Copy direct URL to clipboard
- CLIPBOARD - Copies the permanent URL for the note to the system clipboard
- REFERENCE - Enables easy linking to the note in other discussions or documents
- PRECISION - Links directly to the specific note, not just the general topic or thread



## Global Actions

### **navFeedView** - Return to main discussion feed
- NAVIGATION - Returns to the primary forum dashboard showing recent activity across all topics
- RESET - Clears any filtered views or specific topic focuses to show the complete feed
- ENTRY - Serves as the main entry point for discovering new discussions and trending topics


### **globalSearch** - (LATER) Search forum by keywords or shortnames
- KEYWORDS - Searches note titles, content, and short names for matching text
- SCOPE - Can search across all topics or be filtered to specific categories or time ranges
- RESULTS - Returns ranked results with snippet previews and relevance indicators


### **globalProfile** - (LATER) Access profile settings and preferences
- IDENTITY - Edit display name, bio, and tribal affiliation settings
- PRIVACY - Configure what information is visible to other users
- PREFERENCES - Set notification preferences, display options, and interaction defaults


### **globalSettings** - (LATER) Configure account and display options
- ACCOUNT - Change password, email address, and security settings
- DISPLAY - Customize interface themes, font sizes, and layout preferences
- BEHAVIOR - Set default sorting, filtering, and interaction modes for discussions


### **globalNotifications** - (LATER) View mentions and activity updates
- MENTIONS - Shows when other users reference or reply to your contributions
- ACTIVITY - Displays updates on notes you're watching or have participated in
- MANAGEMENT - Allows marking notifications as read and configuring alert types


### **globalAnalytics** - (LATER) See personal engagement statistics
- METRICS - Shows your contribution count, support/opposition ratios, and engagement levels
- TRENDS - Displays your activity patterns and most active discussion topics
- IMPACT - Indicates how often your contributions receive support or generate responses


### **globalHelp** - Access documentation and support
- DOCUMENTATION - Provides guides for using forum features and understanding the discussion structure
- FAQ - Common questions about political discourse guidelines and technical functionality
- SUPPORT - Contact options for technical issues or community guidelines questions


### **globalLogout** - Sign out of the forum system
- SECURITY - Ends the current session and clears authentication tokens
- CLEANUP - Removes any cached data or temporary settings from the browser
- REDIRECT - Returns to the login page or public forum view depending on configuration

