
[Folder](https://drive.google.com/drive/u/0/folders/1aKeEdnbKsvIqt4-KgFXzCcAuAmUxyiyA) 
- [proposal](https://docs.google.com/document/d/1E4EfjYjdkg883J1qhvp-aibiugZ1CounntZ0DrfhIsA/edit#heading=h.rdueoa6l79t8) - 
- [Figure](https://docs.google.com/presentation/d/1peKBLGXUUUtfxSIKXgCiRZUFeMPWDGaEATWpOYlwbPM/edit#slide=id.p) - 



- [[/Users/oblinger/ob/kmr/Work/SV/WW/WW Registration UX/Game Manager UX.pptx|PPTX]] [gDrive](https://drive.google.com/drive/u/0/folders/1aKeEdnbKsvIqt4-KgFXzCcAuAmUxyiyA) 


- [ ] - [ ] cap city program - aau NE basketball - // many programs at one tournament





GAME is the base unit.
TEAM participated  game base roster
PROGRAM as a container -  collection of teams
	
from the game up.

- Hot Bug Process - 14 Jira projects - 

- tactically we need deliver features - when we claim - 




Ball Detect, Camera Angle, Rules to Move Camera, Recovery from lost calls.

- Martin - Engineering (fixing python)
- vishal - manage AWS - image NAWS - DevOps - 



# GAMES TAXONOMY


GAME - A key SV building block is a GAME.  A game is comprised of zero or more recordings, and optional meta data about who is playing, where, with what jersey colors etc.

FOLDER - A folder is a container of games and/or other folders that forms a non-circular taxonomy.


## KEY ACTIONS

### ACCOUNT/FOLDER CREATION / DELETION
- Account Deletion - Removes all data associated with this account.
- Folder Creation - Each user can freely create folders


### TAXONOMY EDITOR

THREE BUTTONS:  	FOLDER-WIDGET  PLUS-BUTTON  TRASH-WIDGET
PATH: 				Sequence > of > sub > folder > names
CONTENTS:			List of folder contents in reverse order they were added to the folder.  Each entry is on its own row.


CLICKING ON THE
- PLUS-BUTTON: 		Brings up folders-menu to select to a folder/game to add to the current folder
- TRASH-WIDGET: 	Views list of recently deleted folders games as a menu that can be selected from to re-add them to the current folder.
- PATH-WIDGETS:	Current path in taxonomy is displayed as a sequence of selectable folder names with the root folder being the Sports Visio icon.
- A CONTENT ROW:	Will "jump" to the specified folder or game element.

DRAGGING - A GAME OR FOLDER MAY BE DRUG ONTO A GUI ELEMENT:
- TO PLUS-WIDGET: 		If a folder/game is drug to the PLUS-WIDGET the folders-menu is opened to select the target where it should be added
- TO TRASH-WIDGET: 	If folder/game is drug to TRASH-WIDGET it is removed from the current parent folder, and it is added to this users "recently deleted" list
- TO PATH-WIDGET:		If a folder or game is drug into a path widget, folders menu selector is activated to select a target to MOVE it to and remove it from the current location.
- TO FOLDER:				If a folder/game is drug onto another folder icon (in the contents panel) it is MOVED there and removed from its current parent.
- DRAG BETWEEN: 		If a folder/game is drug between the rows within the content pane that pane is reordered.




FOLDER SELECT
- UX brings up list of sub-folders which forms a recursive select folder tree.
- Selection can be restricted to only select (1) GAMES  (2) LEAF FOLDERS  (3) ANY FOLDER




## FOR LATER

TWO-STEP INCLUSION APPROVAL
- Any user with access to a folder/game can add it to another folder.
- Then when the owner of that folder logs in, they are presented with an approval request which they can: deny, grant once, grant automatically.
- Once granted, then the folder/game inclusion become active.
- Will need a "sharing" dialog on a folder to control who can auto add to it.

PERMISSIONS
- Visibility permissions to see other folders / games
- Update permissions to control who can write to specific folders, folder trees




## DETAILS


FOLDER STRUCTURE
- PARENTS - Each folder is contained within a ordered list of parent folders.
  		(The folder-parent structure cannot have any cycles, and there is exactly one top folder containing all others.)
- SHORT NAME - Each folder has a short name that generally distinguishes it from other folders within the same parents.  
  		(it is legal, but not advisable to have two sub-folders with the same short name in a single parent folder.)
- FOLDER PATH - When selecting 