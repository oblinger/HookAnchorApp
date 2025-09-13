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


## Note Group
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
