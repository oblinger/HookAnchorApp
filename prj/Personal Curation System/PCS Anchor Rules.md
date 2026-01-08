# ANCHOR RULES

The most common stream types are streams of anchors.  Each anchor is usually associated with a folder.  Here are the aspects that may be associated with an anchor folder.

## Anchor Folder File Structure
- An ***`"anchor"`*** is a folder that contains a Markdown file with the same name as the folder.  This is the "anchor markdown" 
- So:   "`.../My Anchor Example Project/My Anchor Example Project.md`" is the folder's anchor markdown.
- The anchor markdown provides links to key parts of the anchor folder


## TLC Naming
- Commonly accessed anchors will have a short acronym for quick access.
- Ideally, these acronyms are three letters long, hence they are called TLC - Three Letter Codes.
- If an anchor folder has a TLC then its anchor markdown (with same name as folder)  should have  "(see [[TLC]])" as its contents
- Then the `TLC.md` will then be the anchor markdown for this anchor folder.


## TLC Index

- The markdown [[SYS/Closet/Three Letter Codes/TLC]] contains a table of all TLCs.
- Occasionally, you should scan ~/ob/kmr and ~/ob/proj to look for anchor folders and check if they also have a TLC file, and then update the table to match what is found.  Here are how the fields are computed:
	- DATE - is the creation date of the TLC.md file itself, and the table should be sorted in reverse order by date.
	- TLC - This is the wiki link to the acronym file, whatever it is (it is guaranteed to be unique across the vault)
	- FULL ANCHOR NAME - Is the full name of the anchor, which is the folder name it is contained in
	- DESC - This is the description of the anchor, it is stored in the fully named anchor markdown with prefix "desc::"
	  You should copy this in either direction and try to keep the most recent copy of this description.
	- If I tell you to "Update the TLCs" I am asking you to scan the "`tlcscan.py`" script to find all TLCs and then update the table as needed.


## CLAUDE
- 

## GIT
- An anchor may have its own git repository at its root, and may be linked to a remote.  
- By default, we use a private repo on GitHub as the remote.

## TMUX
- 
