
### Design

- List types:  
	- Global Lists 
		- explicitly enumerated
		- having menu prefix-form
	- Project Lists
		- not a global list
		- named by the note name
- Throw Command: 
	- Global 2 Global:  	Moves item from one global list to another global list
	- Global 2 Project:		Moves item from global list to project list
	- Project 2 Project: 	Moves the from one project list to another project list
	- Project 2 Global:		Copies item to global list
- Sync Actions:
	- Finds all list notes.  (how?)
	  ==> Scans all and processes them
- Sync One File:
	- Look for updates to recorded cross linkings
	  ==> Push any updates that have occurred since last recording
	- Looks for bare cross linkings.  (those without an ^num mark)
	  ==> Adds cross-linked entry and adds linking marks to both
	- Look 
- Configs
	- Regex showing where the beginning of todo entries are in a note.  Typically occuring after the '---' marks
- Front Matter
	- Tracks which other lists contain projected entries from this list
	- Controls what part of a list if auto projected to the auto projection lists