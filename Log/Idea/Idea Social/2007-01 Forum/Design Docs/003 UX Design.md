# UX Design

The documents is organized by major interfaces and/or by design elements within these interfaces.


## === ADMINISTRATIVE INTERFACE ===
Plan: Use NodeBB's built-in Admin Control Panel (ACP)

**Available out-of-the-box:**
- User management - create users, assign roles (admin/moderator/participant)
- Category management - create and organize topic categories  
- Access control - category-specific permissions and restrictions
- Registration settings - invite-only or open registration modes
- Core invitation system - send email invites to new users
- System monitoring and configuration
- Plugin management and settings

**Relevant plugins for enhanced functionality:**
- `nodebb-plugin-user-invitations` - allows users to invite others with admin oversight
- `nodebb-plugin-newuser-invitation` - admin-only invitation control with CSV import

**Custom additions needed:**
- Tribal affiliation field during user creation/invitation process
- Custom user profile fields for political tribes management
- Topic organization templates for political discussion structure
- Scoring algorithm configuration interface for content ranking




## === NOTE VIEWER ===
This is the main interface for the Forum.  From this interface, a user can: navigate the hierarchy, read, edit, add, and vote.

![[node-viewer.svg]]










## === NOTE EDITOR ===
The Note editor has very much the same shape as the Note viewer. It's just that each one of the components is actually an edible box. So the short name is an edible box at the end of the breadcrumbs. The title is an editable box below that, the contents is a large edit window in the middle.  And there is a save and cancel button at the bottom.

![[note-editor.svg]]


















## === NEWS FEED VIEWER ===

The news feed view allows a participant to Scan the list of recent posts under a single node within the tree, or to simply view all of the posts that have occurred within the areas that they've indicated interest. Each one of the posts has a format similar to the node viewer, along with many of the same actions that can be taken from the news feed view.

![[newsfeed.svg]] 










## KEY UX ELEMENTS

### --- THE TOP BAR ---
The top bar for a note provides context and basic actions, organized into one or two lines.

![[top-bar-layout.svg]] 


**Line 1: Parts**
- **Breadcrumbs**	- Path through topic hierarchy (e.g., Healthcare > Universal Coverage > Single Payer)
- **Shortname**		- Nickname for this note (e.g., "Single Payer")
- **Metadata**		- Author, tribes, timestamp, scores (e.g., @user (progressive) • 2h ago • ⭐ 142 🔵🔵)
- **👍 Support**		- Add supporting argument with subtype (opinion/fact/reference)
- **👎 Oppose**		- Add opposing argument with subtype (opinion/fact/reference)  
- **➕ Add**			- Opens submenu for adding different types of content
- **🔍 Navigate**	- Opens the navigation submenu


**Metadata Indicators:**
- **⭐ [number]**	- Total support votes and opposition votes (e.g., ⭐ +142-30)
- **🔵** Progressive lean	- 1-3 symbols showing degree (🔵 = slight, 🔵🔵 = moderate, 🔵🔵🔵 = strong)
- **🔴** Conservative lean	- 1-3 symbols showing degree (🔴 = slight, 🔴🔴 = moderate, 🔴🔴🔴 = strong)
- No symbols = politically neutral content

**Line 2 - Title:**
- **Full Title**		- Optional title line for the note.  (This can actually be multiple lines if the title is very long)


**The "+" Submenu:**
- **💬 Comment**	- Add a comment without indicating support or opposition to a note
- **📊 Fact**			- Add factual evidence that supports the position
- **🔗 Reference**	- Add external source/citation that supports the position  
- **🔸 Sub-claim**	- Add subsidiary argument that supports the main position
- **✏️ Edit**			- (LATER) Suggest edits to existing content (with track changes)
- **🔄 Alternative**	- Propose alternative phrasing or approach
- **🚩 Flag**			- Report content for moderation review


**The "🔍" Navigation Submenu:**
- **👁️ Watching**	- Toggle to  **🔕 Mute**	notifications for new posts under this topic
- **↗️ Parent Post**	- Navigate to parent note (for replies/arguments)
- **🌳 View in Tree**	- Show this note's position in topic hierarchy
- **📤 Share**		- Share link to this note/discussion
- **📋 Copy Link**	- Copy direct link to this specific note




### --- THE ACCOUNT BAR ---
The account bar is at the top of the screen and allows the user to execute various account or global actions.

![[account-bar-layout.svg]]

- **💬 Forum**		- Clicking the forum symbol will return to main feed/dashboard
- **🔍 Search**		- Search the forum by shortname or text string
- **👤 Profile**		- Edit profile, tribes, and user preferences  


**Profile Submenu**  (accessed by clicking on the user's profile icon)
- **⚙️ Settings**		- Account settings and display preferences
- **🔔 Notifications**- View mentions, replies, and followed content updates
- **📊 Analytics**	- (LATER) Personal engagement stats and tribal breakdown
- **❓ Help**		- Documentation and support resources
- **🚪 Logout**		- Sign out of the system

