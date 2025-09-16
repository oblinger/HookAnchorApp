 # Forum Mechanics

This document describes the mechanics of how the form operates. Specifically, how derived parameters are computed, and how permissions for various actions are derived as well.


## Moderators
Moderators have expanded powers of control within the form system. Typically this allows them to make edits or changes within specific groups of notes.
- Each user has a list of tribes for which they are moderators. 
- This list is empty for most users since they are not moderators for any tribe.
- For those users who are moderators, they are generally moderators for a single tribe.
- And administrators are set up as moderators for every tribe.

## Note Editor
An editor for a note is a user who is allowed to make edits to all user-editable fields of the note. 
- AUTHOR - The author of a note is typically an editor of the note.
- PROXIES - Users may also designate other users as proxies. When they do that, any notes that they are in an editor on become editable by those other users as well.
- MODERATORS - Users who are listed as moderators for the tribe of the author of a note become editors for that note.
- ALLY - (LATER) An ally for a note is a user who is in the same tribe as the author of the Note.  (For the moment, we don't have any actions specific to allies.)
- ACCEPT CHANGE - (LATER) In addition to editing a note, editors are also allowed to accept changes or revisions suggested by others for the note as well. (LATER).


## Voting
- `vote(u, n, s)` -- This notation indicates that the most recent vote from user, `u`, on note, `n`, indicated stance, `s`.
	We say the user **supports/opposes/has-no-stance** on the note.
-  `nominate(u, n, A)` --  This notation indicates the most recent vote for any note within the set of alternate notes, `A`, as a support vote for note, `n`.
	We say the *user* **is nominating** *note* from the *alternatives*.
-  `nominate(u, NONE, A)` -- Indicates a user has not nominated any alternative, or they later expressed opposition to remove any nomination.
	We say the user is **not nominating any note** from the *alternatives*.


## Issue Positioning
- **REFERENDUM** - Each issue implicitly sets up an ongoing referendum among all positions posted under the issue.
- **PARTICIPANT VOTING** - A user's position on an issue is defined by the most recent support vote they have given to any position on that issue. 
- **PARTICIPANT POSITION** - Thus, each participant has a defined position on every issue. It is the note they have most recently nominated, or it is NONE.
- **NOTE POSITION** - A note may also have a relationship to the positions on an issue.  
	A note ***is part*** of a position if it is under that position and authored by a participant holding that position.  
	A note ***is part of the opposition*** of a position if it is under the position, it is authored by someone not holding the position, and it indicates opposition to part of the position, or support for part of the opposition of the position.
- **NOTE POSITION RULES** - Here are the rules for determining the position for all notes under a given position:
	- The position note itself is part of its own position.
	- Any note whose author holds the position and whose parent is not part of the position is also part of that same position.  
	  (Notice, even if a note expresses opposition, it is understood to be a critique of a part of the position, but it also is part of that same position because of its author)
	- Any note whose author hold a different position and whose 

... not done ...
	- Any contribution notes whose author supports
	- Any sub-note whose stance is "support" for a note that supports a position, also supports that same position.
	- Any sub-note whose stance is "oppose" 
- NOMINATED POSITION NOTE - The position note within a group of variants for a given position that has the absolute largest number of nominations among those users holding this position becomes the nominated note for that position group. Which ever position Note most recently has support from a given user is the nomination from that user on this position. A user may also nominate no position on an issue in order to remove their nomination for any position on the issue.
- Each note below an issue can thus be labeled as supporting, opposing, or commenting on a given position.  
