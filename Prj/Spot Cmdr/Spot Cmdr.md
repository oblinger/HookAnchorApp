
- [SPOT Folder](spot://SPOT~Folder) 


## JUMP PAGES

A **JUMP PAGE** is any prefix string in spot_cmds.txt supplied before the "!" mark.
- It is associated with exactly one jump page within the kmr folder tree having the same name.
	- MISSING - If the page does not exist, a blank one is created under the JJ folder.
	- DUP - If more than one, then the new blank page has pointers to all versions of the page.

The **JUMP BLOCK** is a region of text within a Jump Page with a bunch of links.
- On jump page XXX the jump block will begin with a line:  .[ [ XXX ] ].
  Or being with the same line w/o the dots.  In either case it ends at the first blank line.
- The JUMP ENTRIES are the links within the jump block

A **JUMP MENU** is a jump page whose name also serves as a prefix submenu
- An ALL CAPS jump name is a jump menu.
- Any spot cmds with this all caps prefix or "!" name are items within this menu

## COMMANDS
- PLACE PAGE - ensures that jump page 'x' exists, and sets 'y' as its parent.


## AUTO UPDATES
- All spot commands should be added to their respective jump page.  (Could be two pages if its ! prefix and caps prefix are different).
- Remove any dead OBS links and spot links from within a jump block
- ??? Add jump menu items from corresponding jump blocks as they are added
- ==> Anytime  is edited

Such pages auto translate all 


1. scan spot_cmds if any jump pages missing, then add blank pages as needed.
2. scan changes jump pages, add linkages found in the jump block if page is a jump menu.
	1. ensure commands link their junior parents
	2. ensures commands connected
3. 