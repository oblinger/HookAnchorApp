


- [ ] Get 'q2' working right
- [ ] "[" should go back in history
- [ ] yore support
- [ ] ghost entry w/ different input and maybe blank selection
- [ ] ctop - unix top command
- [ ] Fix submenu to it is fully recomputed after the dirty flag was set.  (have the get_sys_data return a boolean)
- [ ] .
- [ ] Add a 'just cmd' button
- [ ] Delete key should prompt to delete file and maybe folder.
- [ ] add support for "close popup" into templates.  move CMD+shift+2 to be a template
- [ ] implement activate tmux in Javascript just as it works in Rust
- [x]  nj lrn should "]" to LRN
- [x] "Forum.PPTX Doc" is pointing to a folder that does not exist in a file. It does not exist
- [x] add a javascript 'console' command that pops up a new console from the 
- [x] "resd" should keep showing the 'res' menu
- [x] change the case of the breadcrumb menu
- [x] Rename command should update the input box too
- [x] make 1pass go faster
- [x] remove legacy code (refactor away comments), warnings
- [x] remove legacy panic fns
- [x] wrong sizing.
- [x] Adding an alias can cause a circularity in the anchor-patch graph.  The user might also manually edit the commands.TXT file and cause a circularity to occur that way as well. Let's think through the best way to handle this.
- [x] Streamline way we manage command reloading.
- [x] remove the "A" flag
- [x] Don't exit after command save, just return to popup
- [x] refactor commands.rs to split complex inference logic into a module inference.rs.  each section in this module will have a comment on the top describging the functioning of its section, and then a group of related functions for that kind of inference.  Patch inference is a large bit of code and I think there is an auto inference as another group.


- [x] svproj should find the submenu even though there is no space in it
- [x] proj sv is added twice and only one can be deleted
- [x] Popup is still drifting downward when it is reopening
- [x] remove legacy and dead code.
- [x] the "]" should 'follow' selected item to new menu
- [x] the '[' key should go to patch parent for current menu.  
- [x] start blank.
- [x] no sub-menu for fireball
- [x] anchor should be listed at the front of the list.
- [x] input should be listed at the front of the commands list.
- [x] the submenu should be filtered by the characters AFTER those that matched the anchor (which might be aliased) 
- [x] Remove commands whose alias_resolution match a command in the submenu as well.
- [x] expand the prefix in the input box. when in an alias submenu
- [x] Nothing is getting saved when creating a new command
- [x] Line break in output....
- [x] Delete button's logic is reversed.

- [ ] - setup should setup config.js
- [ ] - should error if notion does not have permissions; need to exetend grabber
- [x] XXX nope.   anchor should really be a flag on any command rather than a command type
- [ ] - ha use top left not bottom left for state
- [ ] - add 'open' fn to templates

────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────╮



#### Anchor Dialog

- we want to extend the command class for a special kind of command called an anchor.
- and we want to create a special dialogue for dealing with these command classes. 
- anchors are commands, whose action is anchor.
- we will extend the command editor to be the anchor editor as well,  any time the action is set to anchor this editor gets a number of extra Fields and the size of the added window. The vertical size gets larger. It might be simpler for the editor to simply have all these fields available and unused in the case that the action is not anchor because the user can flip back-and-forth and the interface shouldn't forget the values in all those extra fields. Still once a command is saved if it's not saved as an anchor then all the extra fields are lost since they don't exist on non-anchor commands.
- 

#### popup_close -- The issue is that the Rust code needs to be modified to:
  1. Read the close_popup parameter from the config
  2. Actually close the popup when close_popup: true

  However, implementing this requires modifying the Rust code to handle the close_popup parameter, which is a more complex change that requires:
  - Updating the config structs to include the close_popup field
  - Modifying the key handler to check this field
  - Changing the return type to indicate whether to close the popup

  This is a structural change to the Rust application that needs careful implementation. The config changes we made are ready, but the Rust code needs to be updated to
  actually use the close_popup parameter.

  For now, the contact functionality should be working correctly (opening the Contacts app with the right contact selected), but the popup won't automatically close yet. That
  feature would need to be implemented in the Rust code.


#### delete 
│ > Let's add another more complicated command to the command ops module. The delete command operation should be extended to check to see if the ARG for the command actually     │
│   refers to an existing file or folder. And if it does, then it should prompt the user for the Alicia that fileif the name matches the file name that is. If the name matches   │
│   the folder because it's an anchor folder it's an anchor name then it should prompt to delete the folder. Only if the folder is gonna be empty after deleting the associated   │
│   file. Just to be clearif the file referenced is not the same name as the command being deleted, it should still prompt the user asking about whether or not it should delete  │
│   that file. And this deletion operation should follow the same paradigm that the current rename operation does, it should have a dry run flagand it's called the first time    │
│   with the dry run flag in order to get the list of things that it's gonna present to the user. Then if the user presses OK, it's going to actually execute those commands. We  │
│   should think about whether or not this code is sufficiently similar to the reading code that somehow they should be merged togetheror perhaps it is Cleaner to just keep      │
│   them separate even though they are analogous to each other.                                           


#### Refactor
  3 TODO/FIXME removal items

  - src/execute/actions.rs:290 - "TODO: Remove after all callers are updated"
  - src/cmd.rs:1155 - "TODO: Remove Karabiner configuration"
  - src/core/key_processing.rs:385 - "TODO: Remove this once all legacy code is cleaned up"

  Legacy/compatibility mentions (not necessarily actionable)

  - Several "legacy" comments in archived/compatibility code
  - Migration comments in config.rs (functional, not removal targets)
  - Bridge comments between old/new systems (architectural, not removal targets)




## __

- [ ] ama should really be an anchor page AND should launch a website. Need to rethink how anchor pages relate to everything
#### LATER
- [ ] G-doc support.  auto create for 'WW' in confluence and g-docs
- [ ] Clean up the log file, verify that std out and error are sent there.
- [ ] go through the code base and move any non-trivial Constance in the code into variables at the top of cis data.RS
- [ ] refactor to get rid of the NavigationHandler
- [ ] - how would we refactor if a short abbreviation prefix like CV were to change later?  how can we refactor prefixes generally?
- [ ] Rename command (should prompt if matching source file and folder should be changed)
- [ ] Ensure that uninstall does not touch files in the .config/hookanchor folder
- [ ] create a functional test based on a complex command.txt to check auto inferencing and other fancy sub systems
- [ ] Get summary of all functions and clean up code base
- [ ] ! will add priority to certain entries.




#### __
## DONE
- [x] CTRL-2 opens contact
- [x] ajs CMD+A does not launch anchoring
- [x] ensure we have removed processing for javascript in yaml file
- [x] fix relative paths in README in docs
- [x] Work - ai product matrix
- [x] Brave - standup
- [x] Test - Firefox, 
- [x] Safari, 
- [x] obsurl - proj jump
- [x] markdown - mddb markdown
- [x] obs - not used....
- [x] app - activity monitor
- [x] remove: activate_anchor, tmux_activate, rescan 
- [x] text - tt linkedin address
- [x] slack - test77
- [x] 1pass - att 1pass
- [x]   *** EXECUTING JAVASCRIPT FUNCTION: action_text ***
- [x] ha config folder   /   # does not open folder (could require a special case for "cd")
- [x] remove shutdown
- [x] test hook://cnnp
- [x] alias not working
- [x] lets add an AI facet (or another name) this key binding will perform an anchor activate on the folder; rename templates to be actions, and allow a type field for an action.
- [x] my computer stuff is not being scanned
- [x] lets add code to construct default_config.yaml dynamically by trimming my person config file.  mostly just remove reference to my personal folders.  Instead put defaults in their place.  Make this be a simple script that is executed during installation build time. And we won't even have a default config.yaml since that file ends up out of date all the time.
- [x] Verify that rescanning is happening.
- [x] open contact is not working
- X the popup server should read the background flag
- [x] FIX TEMPLATES - the template action which derives template perimeter parameters, potentially by grabbing using the grabber module, and then using command editing module to edit those parameters is doing a number of things which have to happen on on the pop-up, not on the command server. I'm thinking the simplest approach might be to simply have the template command, actually be the second command, in addition to pop up which executes directly in the server directly in the pop-up instead of in the server. It is gonna still need to invoke JavaScript since the grabbers and possibly other code will be written in JavaScript and used during template creation. But I think it's OK if we have a different JavaScript server Running in the pop-up process. Right? What do you think of this design choice? What are the pros and cons and your recommendation?
	- [x] Let's set the "&" key template to create a file, a markdown file in the same folder as the last command, whose file name matches the current input.
	- [x] The the ctrl-2 key so it triggers the creation of new "@" markdown files.  These Will be under the kmr/at folder in the obsidian vault. You can see the format of these markdown files. They all begin with an @ sign (which should be added to the beginning of the input string if it is not already there)  and then the file should always begin with the same few characters which you can see in the existing @files.
	- [x] The "!" that creates the named anchor as a sub folder under the folder of the last command's folder.  (should report and error popup to the user if such a folder is not associated to the last command.)  Create error popup functionality if it does not exist.
	- [x]  The Cmd+W creates a ww sub-folder with anchor based on provided name and YYYY.
- [x] merge list of folder commands into commands.txt
- [x] Shutdown not working
- [x] Update and cleanup the build distribution scripts

REFACTORING ACTIONS
- [x] Please create a new PRD file for Refactoring actions. You can put this in the dev notes section and we can work on it together until we are happy with it and then we will implement it. The aim here is to unify template creation in general action binding on the interface keyboard bindings is what I mean by that, and actions taken by commands into one common unified approach. The idea here is, we will rename the templates section in the config Yamel to be actions. Each one of these actions will still be named by the keyword within the actions section, just like it's done as keywords within the templates section now. Each one of these should also have a description at the top so please move all the descriptions to the top of each action. This describes what the action does. It also will need to have a type, and all of the current templates will be of type template. Because there of type template it means that a function either built-in or JavaScript must be defined that begins action_NAME  where name is the name of the action in this section. So there will be an action either built-in or JavaScript probably built-in called action_template  and this built-in function will execute the current behavior for templates. But other action templates or other action types are possible as well, indeed, each one of the existing actions functions will probably be adapted so that they can operate as an action type in this new formulation. So the action structure becomes the parameters for each action type. I think the simplest way to have these different action types to execute is to have all of the parameters that were automatically defined for each action type to be merged with any parameters from that action within the actions section of the Yamel. This way when executing the built-in or the JavaScript, we simply have a number of parameters that are available for access during the execution. Each one of the key bindings in the system will also use the same mechanism. So you can see currently we have the grab action defined with a key associated to it, the plus key this defines that binding and what action should occur. 

- [x] let's add a function called expand strings. It takes a vector of strings to be expanded, which may have the. {{...}} indicators in the strings, and it expands all of these strings. The way it does this is it actually invokes JavaScript on each of these strings, or rather on "..." portion of the strings and whatever result comes back in JavaScript is inserted into the string. Or the system  Pops up and error dialogue box for the user, and then terminate the application once the dialogue box is closed. That way this function always returns a list of expanded strings. If performance is not that much different, maybe this is just a function that does the expansion of a single string. If performance is similar, then I would do the simpler approach. Either way you're invoking JavaScript and now inside of JavaScript, at least when template expansion is happening we want to create a number of JavaScript objects in memory one of them called. "previous" and "selected" both of these have four operators on them:  previous.name,  previous.path,  previous.folder, and previous.hook.  These are the variables which exist in the existing templating system, hook is a special case where the name has been compressed appropriately so that it could be a URL. Thus it always begins HOOK://...
- [x] add 'type' key to a template, if this key exists then the function 'template_XXX' is run instead of the normal template creation logic.  This allows us to have templates that perform other actions.
- [x] Using this new 'type' field, lets create a new kind of template that pops up a menu of choices, and once the user select a choice then that choice name is looked up as a template and is recursively called instead of this template.  In order to implement this choice menu we're going to reuse the pop-up functionality itself. When this choice is executed, it will save the state of the current pop-up and list in the first line the text field for this template. That text field will contain the question being asked to the user. This will be in the place where the sub menu list normally exists in a normal pop-up. Then the list of options for this choice will be the list of alternative commands in the pop-up. In order to do this, this command will get the string from the template in the field, called choices. That string needs to be a comma separated, list of template names, each of these will be used to create a dummy command with that name in order to populate the pop-up. Once the user selects, one of these commands, the pop-ups contents over return as they were before, and the choice has been made will be returned as a string. Really this choice functionality is pretty general, so we should create a function that passes in the text to display and a list of choices. It execute this choice action and returns from the function with the pop-up state returned as it was before the function began. This is a fairly bizarre usage of the existing pop-up system. If this is gonna require too many contortions in the code we shouldn't do it. Alternatively, we can create a new function that supplement implements a choice which looks similar to the pop-up. But I'm not sure there is a fair bit of code in order to allow a list of choices that can be selected by keyboard arrow, keys, or by mouse. We want to retain that functionality, although we don't necessarily need to retain the multi column functionality if that's too hard before beginning on this operation, give me your opinion about which approach is better, hack this into the existing, pop-up, or create a second bit of code dedicated to simply making a choice box that looks similar to our current pop-up logic.

- [x] In the functions section now there are certain action types, which are actually expressed as structures. We should no longer have structures in the functions section. The idea is all actions from keyboard bindings and from templates and for command actions should all have an action structure inside the new actions section. So the sections like this, they are in functions should be factored and moved over.:  
- [x]   action_chrome: {fn: open_with, app: "Google Chrome", arg: "{{arg}}"}      
- [x] And then for every type of action, there should be a built-in or a JavaScript function, whose name is action_name.
- [x] hook:// is not working - [hook](hook://cnnp)   
OTHER
- [x] logging fn
- [x] escape check; 
- [x] escape should return from dialog or editor back to popup, not exit the app.

- [x] massive cleanup and refactoring of keystroke processing
- [x] cleanup of the key processing loops for the different inferfaces
- [x] fix log file clearing logic
- [x] remove the server log
- [x] Remove the logic for the "+" key
- [x] Remove the logic for the ">" key
- [x] ? should print out a help menu using the dialog
- [x] Get rid of compiler warning messages
- [x] make the help message popup to column and each column should be left justified
- [x] make the popup's vertical size correct based on its dynamic content
- [x] ## Add Item Support
	- [x] We want to develop a comprehensive method for creating new entries using the HA application. Similar to the grabbers and actions, we want this to be an open-ended list of templated creative choices.  This is a complex feature, let's plan it out, a series of increasing capabilities that we do it in multiple steps that we test along the way.
	- [x] Create a new source file called template_creation.rs as the center of this new capability.
	- [x] this will be controlled by a new section in the config file called templates.
	- [x] each template will be a new key with a template section, and the value of the key will be a structure that defines that template.
	- [x] Most of the fields for a template will be a string that will undergo expansion before it is used.  Any "{{...}}" will be expanded by looking up the value of "..." as one of the defined variables, or as the result of executing the associated javascript or build in function.  Predefined variables include:
		- [x] input  -	- [ ] these are the characters that were in the input box before selecting the template to be expanded.
		- [x] selected_name -	- [ ] this is the name of the command that was selected before selecting the template to be expanded
		- [x] selected_path -	- [ ] this is the full path of the command associated with the command selected 
		- [x] selected_folder -	- [ ] this is the full path of the folder associated with the selected command
		- [x] previous_name -	- [ ] this is the name of the previously selected command
		- [x] YYYY YY M MM MMM D DD h hh m mm s ss --	- [ ] variables that expand to year, month, day, hour, min, second in various length formats as typical.
	- [x] template fields:
		- [x] name -	- [ ] The name of the command to be created.
		- [x] action -	- [ ] The action for the command to be created.
		- [x] arg -	- [ ] The arg string for the command to be created.
		- [x] patch -	- [ ] The patch string for the command to be created.
		- [x] flags -	- [ ] The flags string for the command to be created.
		- [x] keybinding -	- [ ] If set, this is used define a key binding that will trigger this template creation.
		- [x] edit -	- [ ] if set, the command editor is triggered on the command before it is created.
		- [x] file -	- [ ] The folder that should be created for this command.
		- [x] contents -	- [ ] The contents for the file being created.
		- [x] grab -	- [ ] number of seconds to count before grabbing the top window.  Once grab is done variables will be set for action and arg as the current grabber does.  These can be overridden using template fields, and they can also be accessed as any other variables can be during string expansion.
	- [x] There will be a new keyboard binding for "template_create" which will initially be bound to '-'
		- [x] For now this will trigger the creation of a new command based on the template called 'default' under the templates section.
		- [x] all variables will be instantiated with appropriate values, then 
		- [x] all template fields will be expanded in order to define the command that is getting added to the system.
		- [x] The new command (and optional file/folder) are created, Then finally the new command is saved in with the existing commands.
- [x] in commands.RS there are a lot of functions associated with file paths. We're now using absolute file paths with all of our action types. I think there ought to be zero or one simple accessories that we can use here to handle file pad and get rid of all this stuff. What do you think?
- [x] Create alias ....... auto alias should infer group
- [x] allows mddb to trigger mddy.py in a window that remains open while it runs (press ok to close on error)
- [x] switch all scripts to python
- [x] Rename -f --folders to -p --paths ; then have -f open the named folder or the folder of the last cmd
- [x] Burns CPU even while idle
- [x] Rename Group (cmd prefixes) and groups
- [x] Focus is occasionally not in the input box on launch.
- [x] when an orphan is created, it's patch should be orphan, it definitely should not be itself! 
- [x] The dag defined by commands going to their patch, which themselves are commands that go to their patch should indeed be a dag without cycles.  
       (actually going to allow cycles)
- [x] My thought is to create an accessor on command struct, which is called patch_path and this accessor recursively scans of the hierarchy of patches to their commands to their patches.  This accessor should ensure. That it somehow returns or indicates if the command would create a cycle. Or better yet if the commands patch would end up being part of a cycle, then it can simply set the patch for this command to be orphan. That way this patch path command can be used to destructively ensure there are no cycles within the patch the patch DAG.
- [x] add an uninstall to the menu
- [x] ensure install never overwites any file w/o backing up.  even with force set as true
- [x] check repo
- [x] commit and push
- [x] dialog does not resize window correctly
- [x] the anchor icon has be lost on the app being shown in the dock.  maybe that is ok if the DMG file still has it.  I need to check....
- [x] ff is broken
- [x] The grabber for finder should check if the doc is really a folder, and if it is, then  action should be folder not doc
- [x] BUG: Creating orphans when case mismatch
- [x] When the installer runs, it doesn't seem to actually add any carabiner complex macro to the system. It seems like my current caps lock complex macro is the only thing in there is it actually doing the install?
- [x] Many mac apps have a MAC patch, and all of these should be listed within that sub-menu, but they are not.
- [x] Auto add anchors for all patches under the orphans folder
- [x] Release script (and test that it works)
- [x] add the bill time to the log each time the app is launched, along with a visual separator so we can easily see each time. The app is launched.
- [x] Refactor commands.rs:
- [x] Rename GlobalData to be SysData.
- [x] refactor Grabber to have countdown in config, and to remove window focusing.
- [x] load_data becomes load_sys_data, and get_global_data becomes get_sys_data
- [x] I'll continue with the task to remove unnecessary delays when flip_focus is false. 
- [x] If the interface stays open for a long time, without any user input, it should just exit. (add config parameter)
- [x] OBS to use absolute paths and be called Markdown and only call obsidian if in vault (same as anchor)
- [x] INFER
	- [x] add --infer 2 booleans
	- [x] if arg=URL Web!
	- [x] all infer changes
- [x] deslign an icon for hook & anchor
- [x] Add flag so user edited obs, anchor, folder commands are not erased & add many defaults
- [x] ctrl-c will copy the hook url for the current command to the clipboard
- [x] Alias: remember the last command & timestamp, then if user presses ">" that name is populated 
- [x] add flag for user edited command to obs, anchor, folder.  and manually copy over the list of commands hard coded in the anchor setup.
- [x] ec command does not work
- [x] change the log file so it is deleted daily. (put the last date in state.json)
- [x] put the current URL link onto the clipboard
- [x] finder folder grabber does not work
- [x] obsidian grabber does not work
- [x] update the ff and ana commands to use the new hook system
- [x] pressing enter in the command editor should save the entry
- [x] file scanner should create folder entries even when their is no anchor md
- [x] open-editor should be split into ; and =
- [x] handle aliases inside the folder list function
- [x] Vault is hard coded
- [x] The Finder grabber is locking up
- [x] Grabbing in the finder does not work
- [x] ha -x web  fails because the path cannot file tmux
- [x] obs use relative paths, but the folder binding does not expand them.
- [x] [hook](hook://cnnpage) is broken
- [x] Shutdown is broken
- [x] popup should filter names by group too
- [x] backtick should not get added to the input buffer
- [x] scanner should group names byparent hook folder; 
[[2025-06 HookAnchor]] 

## Rescan
I would like to overhaul the way that rescanning is done:
- this is a complex spec. Let's take it in stages, even if we build the to-do items for all the parts upfront. Feel free to interact with me to clarify each of the steps before you begin.
- both the scan function and the  run patch inference should accept the same string, which is gonna be a, separated, a comma separated string that indicates the specific scan operations and the specific inference operations that should be performed. This becomes a new parameter on a load data so it controls, which of these sub operations are performed. Since this string is gonna be set up to begin and end with a comma one can simply search for the command name with a comma before, and after it and reliably determine whether each command should or should not be executed. 
- there should be a subsection underneath scanner in the config file got an example of this subsection shown below. In this section, we defined a number of scans that can be applied. We can apply those scans using the dash – scan option, and if you notice, they may also have an every flag at the beginning if they do then that particular name scan should be executed once every N seconds.   Here is an example of this section:
- named_scans:
      startup: orphans
      my_name:  --every 3600, files, applications, orphans
- There should be a config file option under scanner, which indicates which of these operations should occur when the application first loads. Of course, in the config file, the string will not begin and end with commas, those will be added before passing the string to load data.
- start up is a special scan name. That's the one that's executed each time. The system first starts up, this is the string that's passed into low data for that first load.
- all other scans are passed to load data either by the rescan command, or at exit of the app we checked to see if any of the named scans are due for a re-execution. Notice, right now we check for re-scanning when the application first starts, but this is wrong. Rescanning takes noticeable time so instead, we should be checking it at the end when we're gonna dismiss the app instead of doing that we can simply minimize the window and then run the appropriate scanner operation.
- find all the places where scanning or patch inference are done and make sure they all are now done within the load data function, we will simply call the load data function from a couple different places in order to get the functionality at boot time, when the every timeout occurs at the end of the application when it's closing, and from the command line when you call rescan
- Let's update the rescan operation so that it checks if the file has shrunk by a large amount and prompts the user before actually performing that operation. Since the interface might be minimized at this point, we might need to add a focus to it or something in order to be able to pop up the dialogue box asking if the deletion is OK.

## Patch

- [ ] PATCH SUPPORT
	- [ ] design format for a patch in markdown.     
	- [ ] Rename Patch to be Anchor
	- [ ] Patch support: Read/Update patch region in a markdown file
	- [ ] Parse commands in it
	- [ ] Add/delete commands based on delta from commands.txt file
	- [ ] Track file changes over time in all patch files
	- [ ] Top level logic to push all cmds into existing patches (if they exist)
	- [ ] LATER Top level logic to trigger patch update when command is changes/added/deleted
	- [ ] LATER Top level logic to notice file updates and check for added or removed links & update cmds


## OTHER SYSTEMS
- [ ] Finish [[T Roll]]
- [ ] cursor positioning in iterm2
- [ ] TMUXIFY - should reliably launch claude in first tab.
- [ ] cut buffer history
- [ ] dates anniversary; bday
- [ ] write up phone contactable plan
- [ ] nicer fonts for iterm2.  see same blogger that taught us tmux/vim
- [ ] Take notes on call recorder [[Record]] 
- [ ] Figure out notifications 
- [ ] scrub <0xa0> non-breaking spaces from clipboard see argentina trip info
- [ ] [[Time Tracker]],
### Email related
- [ ] email notification comes in before message!
- [ ] email messages are repeated in archive as you type it.
- [ ] EMAILS: Verify saving, verify drafts save, verify save sends, merge all AF mail in. back backup of all mail.
      Ensure sent mail is saved. and received mail is saved.  many copies when creating email
- [ ] Remove repeated stuff in email history
- [ ] Hard core backup of all emails.


## OLDER STUFF
### OLD SPOT STUFF
km copy entries
- spot --delete not found as a spotlitght command.  Also it does not seem to have any effect
- SPOT: If a page is renamed the rebuild operation fails since 'update_block' fails

### OLD OTHER STUFF
- [ ] Team Work Proj -- hookperiod will avoid adding 'Webpage to end of name'
- [ ] ask about finding update times for obsidian notes
- [ ] plugin sync using pandoc
- [ ] folding text
- [ ] squirt google search
- [ ] figure out how to get notifications working
- [ ] [btt](https://folivora.ai/buy) - Better Touch Tool
- [ ] F: play with 'state' app
- [ ] To Read  [Scripting Obsidian](https://www.thoughtasylum.com/2021/09/12/automating-your-obsidian-workspace/)  
- [ ] https://github.com/jigish/slate  -  KICK window into position 1, 2, 3 and back  (Mess with SLATE)

