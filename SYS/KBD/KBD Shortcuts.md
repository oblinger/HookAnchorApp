< [[SYS-orig]] 
- [[KBD Global Keys]] 
- [[KBD Obsidian Keys]] 
## = TOPICS =
### - Modifier Combos

| Combo              | Key    | G   | Meaning     | Explanation                | Notes                            |
| ------------------ | ------ | --- | ----------- | -------------------------- | -------------------------------- |
| Ctrl               | CTRL   | A   | "CONTROL"   | App "cursor" control       |                                  |
| Opt                | OPT    | A   |             | App "user defined" control |                                  |
| Cmd                | CMD    | A   | "MODIFY"    | App "global" commands      |                                  |
| Ctrl-Opt           | LEFT2  | A   | "GOTO/VIEW" | LEFT PAIR -                |                                  |
| Opt-Cmd            | RIGHT2 | A   | "CREATE"    | RIGHT PAIR -               |                                  |
| Ctrl-Cmd           | CAPS   | G   | "PLACE"     | global "GO" / page "go"    | Dan's Global "GOTO" Destinations |
| Ctrl-Opt-Cmd       | SLAM   | G   | global "DO" | Ctrl-Opt-Cmd -             | Dan's Global Commands            |
| Ctrl-Opt-Cmd-Shift | TAB    | G?  | ?app "NAV"  | Dan "Cursor" commands      | Dan's Fastest Bindings           |
|                    |        |     |             |                            |                                  |


### - Karabiner
- [Online Macros](https://ke-complex-modifications.pqrs.org/?q=tab),  
#### . Complex Modification 
- to_if_alone_timeout_milliseconds = 1000                // Needs to be long so single tap will register as a TAB
- to_if_held_down_threshold_milliseconds = 50       // Needs to be short so fast-tab chording will register as chorded key

#### . Usage
- If you have a properly formatted JSON file (`pip3 install demjson; jsonlist file.json`) matching the format, you can put it in `~/.config/karabiner/assets/complex_modifications`

#### . Tab key Remap
{ "title": "Tab to Hyper When Held", "rules": [ { "description": "Map Tab to Hyper when held", "manipulators": [ { "type": "basic", "from": { "key_code": "tab", "modifiers": { "optional": [ "any" ] } }, "to_if_alone": [ { "key_code": "tab" } ], "to_if_held_down": [ { "key_code": "left_control", "modifiers": [ "left_command"] } ] } ] } ] }
#### . XXXX
## = Obsidian Builtins = 


| CH    | CTRL                |             |
| ----- | ------------------- | ----------- |
|       |                     |             |
| A     | AT start of line    |             |
| B     | Char BACK           |             |
| C     | COPY to clipboard   |             |
| D     | DELETE char         |             |
| E     | END of line         |             |
| F     | FORWARD char        |             |
| G     |                     |             |
| H     | CHAR backspace      |             |
| I     |                     |             |
| J     |                     |             |
| K     | KILL to end of line |             |
| L     |                     |             |
| M     |                     |             |
| N     | NEXT line           |             |
| O     | OPEN newline        |             |
| P     | PREVIOUS line       |             |
| Q     |                     |             |
| R     |                     |             |
| S     | SAVE note           |             |
| T     | TRANSPOSE chars     |             |
| U     |                     |             |
| V     | PAGE down           |             |
| W     |                     |             |
| X     |                     |             |
| Y     |                     |             |
| Z     | UNDO last edit      |             |
| -     |                     |             |
| +     |                     |             |
| {     | nav BACK            |             |
| }     | nav FORWARD         |             |
| \     |                     |             |
| ;     |                     |             |
| '     |                     |             |
| ,     |                     |             |
| .     |                     |             |
| /     |                     |             |
| ~     |                     |             |
| LT    |                     |             |
| RT    |                     |             |
| UP    |                     |             |
| DN    |                     |             |
| SPACE |                     |             |
| ENTR  |                     | follow LINK |
| INS   |                     |             |
| DEL   |                     |             |
| HOME  |                     |             |
| END   |                     |             |
| PgUp  |                     |             |
| PgDn  |                     |             |
| 1     | ONE pane            |             |
| 2     | TWO vert panes      |             |
| 3     | split pane hor      |             |
| 4     |                     |             |
| 5     |                     |             |
| 6     |                     |             |
| 7     |                     |             |
| 8     |                     |             |
| 9     |                     |             |
| 0     |                     |             |
| F1    |                     |             |
| F2    |                     |             |
| F3    |                     |             |
| F4    |                     |             |
| F5    |                     |             |
| F6    |                     |             |
| F7    |                     |             |
| F8    |                     |             |
| F9    |                     |             |
| F10   |                     |             |
| F11   |                     |             |
| F12   |                     |             |



# # OTHER INFO # 
## === MY STUFF (OUT OF DATE) ===
	   !=shift, ^=control, _/=alt, #=command    a/b ... x/y multiple commands a with y and b with y
### My KEYS
#### CTRL-OPT-CMD
	   C ?
	   D

#### CAPS KEYS (See keyboard maestro)
	CAPS KEYS  --- (MASTER LIST IS REALLY IN KEYBOARD MAESTRO)

	   A  ADDRESS   book  (CoBook)   #Q to search
	   B  BOUNCE    BOUNCE to next step in dev loop  
	  -C  COPY      to clipboard
	   D  DEV       DEV terminal
	   E  EMAIL     send EMAIL to someone
	   F  FINDER    show FINDER
	   G            nstr GOTO    (gmail, go)
	  -H  HIDE      HIDE current window
	   I  IDE
	   J  JUMP      to cmd using alfred
	   K
	   L  LAUNCH    cmd using alfred
	   M  MAIL      view MAIL window
	  -N  NEW       open NEW app instance
	   O  
	   P            ?PLAYER?
	   Q
	   R  READER    view
	   S  STUFF     SHOW my STUFF
	   T  TUNES     control TUNES (whatever app that is)
	   U  UNIX      term view
	  -V  PASTE     from clipboard
	   W  WEB       show WEB browser
	  -X  CUT       to clipboard
	   Y
	   Z

#### CAPS other
	SHIFT-CAPS
	   L  


	-  CAPS-L provides aceess to all 'keyword' triggers.
	   Prefix chars are used to separate sub spaces of these trigger words
	   (Prefix chars are only used when a trigger has multiple forms)

	-  CAPS-L j...     # prefix char for journal files
	-  CAPS-L f...     # prefix for folder files
	-  CAPS-L          # notes/ref files

#### MyArrows

	^_/#   h/j/k/l/; = mostLeft/left/full/right/mostRight   u/n = NW/SW    h/b = small midlow/bottom
	       o/p = most left/right     . / = SE left/right
	       s/f = most left/right     c/v = SE left/right


	^_/#   j/k/l = left/full/right   u/n = NW/SW         h/b = small midlow/bottom
	       o/p = most left/right     . / = SE ledge left-right

	^_/    <- left screen  -> right screen ^ snap back    v full screen
	^_/#   <- full left    -> full right   ^ NW corner    v SW corner
	 _/#   <- most left    -> most right   ^ top of SW    v bottom of SW

#### caps bindings
	MY CAPS KEY BINDINGS
	         (Not current.  See Keyboard Maestro)
	Caps-A   ADDRESS BOOK
	Caps-B
	Caps-C   COPY
	Caps-D   DEV TERM
	Caps-E
	Caps-F   FINDER
	Caps-G   GMAIL
	Caps-H   HIDE
	Caps-I
	Caps-J
	Caps-K
	Caps-L   LAUNCH
	Caps-M   MAIL
	Caps-N   NSTR
	Caps-O
	Caps-P
	Caps-Q
	Caps-R   READER
	Caps-S
	Caps-T   TREE TAB (peep open)
	Caps-U   (unix term)
	Caps-V   PASTE
	Caps-W   WEB
	Caps-X   CUT
	Caps-Y   
	Caps-Z
	Caps-0   0 stars
	Caps-1   1 stars (does not work)
	Caps-2   2 stars
	Caps-3   3 stars
	Caps-4   4 stars
	Caps-5   5 stars
	Caps-7   (go to next NetNewsWire tab, and reload)
	Caps-8   (one screen window arrangement)
	Caps-9   (two screen win
### CAPS LOCK KEYS

#### MAC COMMAND KEY BINDINGS

	Cmd-A   Select All
	Cmd-B
	Cmd-C   Copy
	Cmd-D   (Duplicate)
	Cmd-E   (Eject)
	Cmd-F   Find
	Cmd-G   Goto
	Cmd-H   Hide
	Cmd-I   (Info)
	Cmd-J   (Show Inspector)
	Cmd-K   (Connect to server)
	Cmd-L   (Alias)
	Cmd-M   (Minimize)
	Cmd-N   New
	Cmd-O   Open
	Cmd-P
	Cmd-Q   Quit
	Cmd-R   (show original)
	Cmd-S
	Cmd-T   Show sidebar
	Cmd-U   (utilities)
	Cmd-V   Paste
	Cmd-W   Window close
	Cmd-X   Cut
	Cmd-Y   (slide show)
	Cmd-Z   Undo
	Cmd-,   Preferences
	Cmd-`   Cycle panes
	Cmd-?   Help
	Cmd-[   Back
	Cmd-]   Fwd
	Cmd-    arrows (parent)
	Cmd-
	Cmd-tab Cycle apps
	Cmd-
	Cmd-
	Cmd - H   hide
	Cmd - 

	[ ]   Back / Fwd

### CAPS-L ABBREVIATIONS
	   bm     EMACS [B]ook [M]ark into 
	   nb     EMACS [N]amed [B]ookmark
	d

## === APPLE STUFF ===

### MAC "hard to remember" shortcuts

	+#4     Take snapshot screenshot of portion of screen (put it in file on desktop)  
	        ^ - puts it onto the clipboard

	+^#4    Clipboard

### MAC "UNIVERSAL" STANDARD SHORTCUTS

	POWER  ^+eject (lock) 

	COPT   #X #C #V #!/V     cut/copy/paste/plainPaste text
	FIND   #F
	ZOOM   #! #- #0          increase/decrease/revert zoom level

	^_/#8  InvertScreen

### Mac OS 
	POWER  ^_/#POWER=safe-restart  ^#POWER=instant-restart   _/#=sleep
	cmd ' '  Spotlight

	Command + Option + Control + Eject = Shut Down
	Control + Command + Eject = Restart
	Command + Option + Eject = Sleep

### Finder

	cmd D	Duplicate selected item
	cmd E	Eject
	cmd +G	Go to Folder
	cmd I	Get Info
	cmd _/I	Show Inspector
	cmd ^I	Get Summary Info
	cmd O	Open selected item



	Command-A	Select all items in the front Finder window (or desktop if no window is open)
	Option-Command-A	Deselect all items
	Shift-Command-A	Open the Applications folder
	Command-C	Copy selected item/text to the Clipboard
	Shift-Command-C	Open the Computer window
	Command-1	View as Icon
	Command-2	View as List
	Command-3	View as Columns
	Command-4	View as Cover Flow (Mac OS X 10.5 or later)
	Command-Up Arrow	Open enclosed folder 
	Command-Down Arrow	Open highlighted item


	Shift-Command-D	Open desktop folder
	Command-F	Find any matching Spotlight attribute
	Shift-Command-F	Find Spotlight file name matches
	Option-Command-F	Navigate to the search field in an already-open Spotlight window
	Shift-Command-H	Open the Home folder of the currently logged-in user account
	Shift-Command-I	Open iDisk
	Command-J	Show View Options
	Command-K	Connect to Server
	Shift-Command-K	Open Network window
	Command-L	Make alias of the selected item
	Command-M	Minimize window
	Option-Command-M	Minimize all windows
	Command-N	New Finder window
	Shift-Command-N	New folder
	Option-Command-N	New Smart Folder
	Shift-Command-Q	Log Out
	Option-Shift-Command-Q	Log Out immediately
	Command-R	Show original (of alias)
	Command-T	Add to Sidebar
	Shift-Command-T	Add to Favorites
	Option-Command-T	Hide Toolbar / Show Toolbar in Finder windows
	Shift-Command-U	Open Utilities folder
	Command-V	Paste
	Command-W	Close window
	Option-Command-W	Close all windows
	Command-X	Cut
	Option-Command-Y	Slideshow (Mac OS X 10.5 or later)
	Command-Z	Undo / Redo

	Command-, (Command and the comma key)	Open Finder preferences
	Command-` (the Grave accent key--above Tab key on a US English keyboard layout)	Cycle through open Finder windows
	Command-Shift-?	Open Mac Help
	Option-Shift-Command-Esc (hold for three seconds) - Mac OS X v10.5, v10.6 or later only	Force Quit front-most application
	Command-[	Back
	Command-]	Forward
	Control-Command-Up Arrow	Open enclosed folder in a new window
	Command-Tab	Switch application--cycle forward
	Shift-Command-Tab	Switch application--cycle backward
	Command-Delete	Move to Trash
	Shift-Command-Delete	Empty Trash
	Option-Shift-Command-Delete	Empty Trash without confirmation dialog
	Spacebar (or Command-Y)	Quick Look (Mac OS X 10.5 or later)
	Command key while dragging	Move dragged item to other volume/location (pointer icon changes while key is held--see this article)
	Option key while dragging	Copy dragged item (pointer icon changes while key is held--see this article)
	Option-Command key combination while dragging	Make alias of dragged item (pointer icon changes while key is held--see this article)

### MAC MAIL
	- TRAIGE: !#N NEW-mail-get  #OPEN  [!]#REPLY[ALL]  !#FORWARD   #'<'DELETE
	- MAIL:  !#D-DELIVER Mail  !#N-NEW Mail 
	         #0-msgList  #1-inbox  #2-allmail  #3-sent
	- FMT:   !#T-makePlainTxt
	- TOOLS  #; chkSpell  #: showSpleeGrammar
	- SPECIAL:  #+H showHEADER


	   Cmd       [N]ew  [S]ave  [1]inbox  [2]outbox  [3]drafts  [4]sent

	   Cmd-Shift [A]ttach  [B]ounce  [D]eliver  [F]wd  [H]eaderInfo  [M]boxToggle
	             [R]eply(shift4all)  
	             [Y]2addrBook
	   Cmd-Opt   [U]ncoverOriginal

	   Cmd  [+]larger [-]smaller  [;]spell

	!#E=Redirect  !#BOUNCE  !#YANK-to-addrBook  
	http://dashkards.com/mail

### SPOTLIGHT

	BASIC SPOTLIGHT SHORTCUTS
	cmd ' '    Open Spotlight menu – Command+Space
	cmd _/' '  Open Spotlight in the Finder – Command+Option+Space
	ESC     Clear Spotlight search box – Escape
	ESC ESC Close Spotlight menu – Escape twice

	SPOTLIGHT USAGE & NAVIGATION KEYBOARD SHORTCUTS
	ENT   Open first search item – Return
	^/v   Navigate search results – Arrow Up and Arrow Down
	cmd ENT  Open to location of first search item in Finder – Command+Return
	cmd I    INFO -- Get Info on search item – Command+I
	      Show Quick Look Preview of Spotlight results – Command key or hover with the Mouse cursor (OS X 10.7 and later only)
	cmd _/   Show path/location of search result – Command+Option while hovering over search result
	cmd ^/v  Jump categories in search results

### iPhone

#### Headset
	MUSIC
	o        Play or pause song - press center button once.
	o2       Skip song - press center button rapidly twice.
	o2-hold  Fast forward - press center button twice and hold.
	o3       Back to previous song - Press center button three times
	o3-hold  Rewind - Press center button three times rapidly and hold.
		Volume up - plus button
	- Volume down - minus button

	CALLS
	o        Answer call - press the center button.
	o        Hang up - press the center button again.
	o-hold   Decline an incoming call - hold down the center button for 2 seconds and release. Listen for 2 low beeps that confirm the call was denied.
	o-hold   Place current call on hold while swapping to other call - Press center button. Repeat after delay to swap to other call.
	o-hold   Hang up current call and answer incoming call - Hold center button down for 2 seconds. When you release 2 beeps will let you know old call has been terminated and you will be on the new call.

	SIRI
	o-hold   Hold down the center button until you hear a tone. Use voice control/Siri as directed on your iPhone.


## === OTHER APPS ===
### GLOBAL App Keys

	_/ # R  Switch Resolutions
	_/ # E  Compose SMS
	_/ # M  View SMS

### TRIGGERS FOR SPECIALTY APPS

	^_/# N    NEU file creation

### Feedly

	 [n/p]   next/prev highlighted
	 [v]     view-in-other-tab
	 [gg]    magic menu

### Pocket

	http://help.getpocket.com/customer/portal/articles/805887-keyboard-shortcuts-in-pocket-for-mac

## === PRODUCTIVITY APPS ===
### 1Password
	 cmd \_  \  Show Password mini
	 cmd    \  Fill in password on current page
	 cmd ^\_ \  LOCK 1passord

### Asana
	TAB-T  edit_tags

### Cobook
	  cmd N / cmd DEL   Create / Delete contact
	  cmd L          Toggle edit mode
	  cmd M          Merge
	  cmd E / cmd D     Email / Dial contact

### Evernote

	General Application
	⌘-N -- new note
	⌘-1 -- list view
	⌘-2 -- snippet view
	⌘-3 -- card view


	Editor 

	⌘-' -- edit TAGS
	fn-fn -- start dictation
	⌘-B -I -U -- bold / italicize / underline
	⇧-⌘-F -- simplify formatting
	⇧-⌘-D -- insert date
	⌥-⇧-⌘-D -- insert time
	⇧-⌘-U -- bulleted list
	⇧-⌘-O -- numbered list
	⇧-⌘-L -- insert table
	⌘-K -- add web link



	General Application ALL
	⌘-N -- new note
	⌘-⇧-N -- new notebook
	⌃-⌘-N -- new tag
	⌘-1 -- list view
	⌘-2 -- snippet view
	⌘-3 -- card view
	⌘-M -- minimize window
	⌃-⌘-F -- toggle full screen
	⇧-⌘-P -- page setup
	⌘-P -- print
	⌘-: -- show spelling &grammar
	⌘-; -- spell check
	⌘-0 -- sync status
	⌃-⌘-S -- sync
	⌘-W -- close
	⌘-Q -- quit
	⌘-, -- preferences
	⌘-H -- hide Evernote


	Editor ALL
	fn-fn -- start dictation
	⌘-B -- bold
	⌘-I -- italicize
	⌘-U -- underline
	⌃-⌘-K -- strike through
	⌘-A -- select all
	⌘-X -- cut
	⌘-C -- copy
	⌘-V -- paste
	⇧-⌘-V -- paste without formatting
	⇧-⌘-F -- simplify formatting
	⇧-⌘-D -- insert date
	⌥-⇧-⌘-D -- insert time
	⌘-Z -- undo
	⇧-⌘-Z -- redo
	⇧-⌘-U -- bulleted list
	⇧-⌘-O -- numbered list
	⇧-⌘-L -- insert table
	⌘-{ -- align left
	⌘-} -- align right
	⌘-| -- center
	⇧-⌘ X -- encrypt selected text
	⌘-K -- add link
	⌘-F -- find within note
	⌘-S -- save

	source  http://howto.cnet.com/8301-11310_39-57500610-285/keyboard-shortcuts-in-evernote-for-mac/

### Omni Outliner
	  OPT-Enter    (inside note)
	  ESC          Cycle thru colums
	  Opt-Del      From beginning to point, Del at beginning of line will remove line

## === EDITOR / VIEWERS ===
### CHROME
	     https://support.google.com/chrome/bin/answer.py?hl=en&topic=25799&answer=157179&rd=1

	OPEN   # N # T #!N    window/tab/incog
	       !  #! #!!    Open link inFg/inBgTab/inFgTab

	UNDO   #!T         reopenTab/

	GOTO   #[ #]       goto previous/next history page.  (also del !del)
	       #n ^9       goto nth/last tab
	       #tab,^!tab  next/prev tab
	       #L          focus on address bar

	STOP   #,          Stop Loading
	CLOSE  # W #!W # H   Close tab/win   Hide win
### EMACS   
#### Emacs Org Mode
	http://orgmode.org/orgcard.txt

	http://orgmode.org/manual/Structure-editing.html
	 %!  <--  -->   tree [de]\-]indent
#### Emacs General
	C-SP     set-mark-command		 C-q      quoted-insert
	C-a      beginning-of-line		 C-r      isearch-backward
	C-b      backward-char			 C-s      isearch-forward
	C-c      exit-recursive-edit		 C-t      transpose-chars
	C-d      delete-char			 C-u      universal-argument
	C-e      end-of-line			 C-v      scroll-up
	C-f      forward-char			 C-w      kill-region
	C-h      help-command			 C-x      Control-X-prefix
	TAB      indent-for-tab-command		 C-y      yank
	LFD      newline-and-indent		 C-z      suspend-emacs
	C-k      kill-line			 ESC      ESC-prefix
	C-l      recenter			 C-]      abort-recursive-edit
	RET      newline			 C-_      undo
	C-n      next-line			 SPC .. ~        self-insert-command
	C-o      open-line			 DEL      delete-backward-char
	C-p      previous-line

	C-h v    describe-variable		 C-h d    describe-function
	C-h w    where-is			 C-h k    describe-key
	C-h t    help-with-tutorial		 C-h c    describe-key-briefly
	C-h s    describe-syntax		 C-h b    describe-bindings
	C-h n    view-emacs-news		 C-h a    command-apropos
	C-h C-n  view-emacs-news		 C-h C-d  describe-distribution
	C-h m    describe-mode			 C-h C-c  describe-copying
	C-h l    view-lossage			 C-h ?    help-for-help
	C-h i    info				 C-h C-h  help-for-help
	C-h f    describe-function

	C-x C-a  add-mode-abbrev		 C-x 5    split-window-horizontally
	C-x C-b  list-buffers			 C-x ;    set-comment-column
	C-x C-c  save-buffers-kill-emacs	 C-x <    scroll-left
	C-x C-d  list-directory			 C-x =    what-cursor-position
	C-x C-e  eval-last-sexp			 C-x >    scroll-right
	C-x C-f  find-file			 C-x [    backward-page
	C-x C-h  inverse-add-mode-abbrev	 C-x ]    forward-page
	C-x TAB  indent-rigidly			 C-x ^    enlarge-window
	C-x C-l  downcase-region		 C-x `    next-error
	C-x C-n  set-goal-column		 C-x a    append-to-buffer
	C-x C-o  delete-blank-lines		 C-x b    switch-to-buffer
	C-x C-p  mark-page			 C-x d    dired
	C-x C-q  toggle-read-only		 C-x e    call-last-kbd-macro
	C-x C-r  find-file-read-only		 C-x f    set-fill-column
	C-x C-s  save-buffer			 C-x g    insert-register
	C-x C-t  transpose-lines		 C-x h    mark-whole-buffer
	C-x C-u  upcase-region			 C-x i    insert-file
	C-x C-v  find-alternate-file		 C-x j    register-to-dot
	C-x C-w  write-file			 C-x k    kill-buffer
	C-x C-x  exchange-dot-and-mark		 C-x l    count-lines-page
	C-x C-z  suspend-emacs			 C-x m    mail
	C-x ESC  repeat-complex-command		 C-x n    narrow-to-region
	C-x $    set-selective-display		 C-x o    other-window
	C-x (    start-kbd-macro		 C-x p    narrow-to-page
	C-x )    end-kbd-macro			 C-x q    kbd-macro-query
	C-x +    add-global-abbrev		 C-x r    copy-rectangle-to-register
	C-x -    inverse-add-global-abbrev	 C-x s    save-some-buffers
	C-x .    set-fill-prefix		 C-x u    advertised-undo
	C-x /    dot-to-register		 C-x w    widen
	C-x 0    delete-window			 C-x x    copy-to-register
	C-x 1    delete-other-windows		 C-x {    shrink-window-horizontally
	C-x 2    split-window-vertically	 C-x }    enlarge-window-horizontally
	C-x 4    ctl-x-4-prefix			 C-x DEL  backward-kill-sentence

	ESC C-SP mark-sexp			 ESC =    count-lines-region
	ESC C-a  beginning-of-defun		 ESC >    end-of-buffer
	ESC C-b  backward-sexp			 ESC @    mark-word
	ESC C-c  exit-recursive-edit		 ESC O    ??
	ESC C-d  down-list			 ESC [    backward-paragraph
	ESC C-e  end-of-defun			 ESC \    delete-horizontal-space
	ESC C-f  forward-sexp			 ESC ]    forward-paragraph
	ESC C-h  mark-defun			 ESC ^    delete-indentation
	ESC LFD  indent-new-comment-line	 ESC a    backward-sentence
	ESC C-k  kill-sexp			 ESC b    backward-word
	ESC C-n  forward-list			 ESC c    capitalize-word
	ESC C-o  split-line			 ESC d    kill-word
	ESC C-p  backward-list			 ESC e    forward-sentence
	ESC C-s  isearch-forward-regexp		 ESC f    forward-word
	ESC C-t  transpose-sexps		 ESC g    fill-region
	ESC C-u  backward-up-list		 ESC h    mark-paragraph
	ESC C-v  scroll-other-window		 ESC i    tab-to-tab-stop
	ESC C-w  append-next-kill		 ESC j    indent-new-comment-line
	ESC ESC  ??				 ESC k    kill-sentence
	ESC C-\  indent-region			 ESC l    downcase-word
	ESC SPC  just-one-space			 ESC m    back-to-indentation
	ESC !    shell-command			 ESC q    fill-paragraph
	ESC $    spell-word			 ESC r    move-to-window-line
	ESC %    query-replace			 ESC t    transpose-words
	ESC '    abbrev-prefix-mark		 ESC u    upcase-word
	ESC (    insert-parentheses		 ESC v    scroll-down
	ESC )    move-past-close-and-reindent	 ESC w    copy-region-as-kill
	ESC ,    tags-loop-continue		 ESC x    execute-extended-command
	ESC -    negative-argument		 ESC y    yank-pop
	ESC .    find-tag			 ESC z    zap-to-char
	ESC 0 .. ESC 9  digit-argument		 ESC |    shell-command-on-region
	ESC ;    indent-for-comment		 ESC ~    not-modified
	ESC <    beginning-of-buffer		 ESC DEL  backward-kill-word

	C-x 4 C-f       find-file-other-window	 C-x 4 d  dired-other-window
	C-x 4 .  find-tag-other-window		 C-x 4 f  find-file-other-window
	C-x 4 b  pop-to-buffer			 C-x 4 m  mail-other-window

### NET NEWS READER  

	space   Scroll in reader window, then Goto next
	->      Open current in browser
	i       Mark current entries as read, show next set of entries
	k       Mark all read

	<-      Back to subscrptions
	Shft v  Next unread
	Shft ^  Pref unread
	d       Del read items

	r       Mark as read
	l       Mark read & next unread (lima)
	u       Mark all unread
	m       Mark unread and next unread
	p       Mark all on page as read
	e       Mark as read & collapse
	j       Mark as read & collapse all

	/       Collapse Read items
	       Collapse
	  Collapse all
	    Expand
	  Expand All

### Sublime

	Editing
	LINE     ^+    J=JOIN   [/!]&enter=INS-before/after  L=kill-LINE   K&[K/<]=KILL-to[end/start]  X=del 
	               '['/']'=INDENT-[in/out]    [!]'/'=[BLOCK]-COMMENT-toggle 
	         ^!+   [^/v]=MOVE-up/down   D=DUP
	OTHER    ^     D=Da-WORD   M=MATHING-parens   U=soft-UNDO

	WINDOWS        _/!1=1  _/!2=2  _/!5=4group   ^[!]num=JUMP[MOVE]-To-Group   

	GOTO     P     #P=LOAD FILE  !#P=EXECUTE_CMD

	VIEW           #K#B SIDEBAR

        -
	Ctrl + ⇧ + M	Select all contents of the current parentheses
	Ctrl + Y	Redo, or repeat last keyboard shortcut command
	Ctrl + ⇧ + V	Paste and indent correctly
	Ctrl + Space	Select next auto-complete suggestion
	Ctrl + U	soft undo; jumps to your last change before undoing change when repeated


	http://docs.sublimetext.info/en/latest/reference/keyboard_shortcuts_win.html

	EDITING
	Ctrl + X	Delete line
	Ctrl + ↩	Insert line after
	Ctrl + ⇧ + ↩	Insert line before
	Ctrl + ⇧ + ↑	Move line/selection up
	Ctrl + ⇧ + ↓	Move line/selection down
	Ctrl + L	Select line - Repeat to select next lines
	Ctrl + D	Select word - Repeat select others occurrences
	Ctrl + M	Jump to closing parentheses Repeat to jump to opening parentheses
	Ctrl + ⇧ + M	Select all contents of the current parentheses
	Ctrl + KK	Delete from cursor to end of line
	Ctrl + K + ⌫	Delete from cursor to start of line
	Ctrl + ]	Indent current line(s)
	Ctrl + [	Un-indent current line(s)
	Ctrl + ⇧ + D	Duplicate line(s)
	Ctrl + J	Join line below to the end of the current line
	Ctrl + /	Comment/un-comment current line
	Ctrl + ⇧ + /	Block comment current selection
	Ctrl + Y	Redo, or repeat last keyboard shortcut command
	Ctrl + ⇧ + V	Paste and indent correctly
	Ctrl + Space	Select next auto-complete suggestion
	Ctrl + U	soft undo; jumps to your last change before undoing change when repeated


	GOTO
	Ctrl + P	Quick-open files by name
	Ctrl + R	Goto symbol
	Ctrl + ;	Goto word in current file
	Ctrl + G	Goto line in current file


	GENERAL
	Ctrl + ⇧ + P	Command prompt
	Ctrl + KB	Toggle side bar
	Ctrl + ⇧ + Alt + P	Show scope in status bar


	FIND
	Ctrl + F	Find
	Ctrl + H	Replace
	Ctrl + ⇧ + F	Find in files

	replace ^_/F

	TABS
	Ctrl + ⇧ + t	Open last closed tab
	Ctrl + PgUp	Cycle up through tabs
	Ctrl + PgDn	Cycle down through tabs
	Ctrl + ⇆	Find in files
	Alt + [NUM]	Switch to tab number [NUM] where [NUM] <= number of tabs

	WINDOWS
	Alt + ⇧ + 2	Split view into two columns
	Alt + ⇧ + 1	Revert view to single column
	Alt + ⇧ + 5	Set view to grid (4 groups)
	Ctrl + [NUM]	Jump to group where num is 1-4
	Ctrl + ⇧ + [NUM]	Move file to specified group where num is 1-4

	TABS       


	BOOKMARKS
	Ctrl + F2	Toggle bookmark
	F2	Next bookmark
	⇧ + F2	Previous bookmark
	Ctrl + ⇧ + F2	Clear bookmarks

	CASE 
	Ctrl + KU	Transform to Uppercase
	Ctrl + KL	Transform to Lowercase


#### Specialty
	cmd !P   PACKAGE load

### TEXT MATE

### Word

	-- To enable two veiws on one document.  WordMenu -> Window -> New Window

### COMMENTS <--- misplaced
	Apple-/ Togggle comment
	Alt-Apple-/ Comment a block and comment subset of a line
	“head” tab-key Insert comment header at top of file
	Control-Shift-B Insert comment banner (top of declaration, function, etc)
	“todo” tab-key Insert todo block


### end

## = OLD GLOBAL =

| CH   | COMMAND / [SHIFTED] | ALT  / [SHIFTED]        |
| ---- | ------------------- | ----------------------- |
| A    | select ALL          | --- ABOUT prefix        |
| B    |                     |                         |
| C    |                     | # COPY to clip          |
| D    |                     | DEV terminal            |
| E    |                     | EMAIL message           |
| F    |                     | FINDER                  |
| G    |                     |                         |
| H    |                     | HIDE window             |
| I    | tgl ITALIC          | INFO (obsidian)         |
| J    |                     | JUMP to folder          |
| K    | insert LINK         | KICK window around      |
| L    |                     | LAUNCH                  |
| M    |                     | MAIL                    |
| N    | NEW note / New PANE | # NEW                   |
| O    | OPEN switcher       |                         |
| P    | cmd PALETTE         |                         |
| Q    |                     | QUICK look at web       |
| R    |                     | READER (Feedly)         |
| S    | SAVE file           | STUFF (Asana)           |
| T    | / undo CLOSE        |                         |
| U    |                     |                         |
| V    |                     | # V PASTE               |
| W    | CLOSE pane          | ---WEB prefix           |
| X    |                     |                         |
| Y    |                     |                         |
| Z    |                     | ZOOM (work loop)        |
| -    |                     |                         |
| +    |                     | tgl RIGHT pane          |
| {    |                     | nav BACK                |
| }    |                     | nav FORWARD             |
| \    |                     |                         |
| ;    |                     | DARK mode               |
| '    |                     | LIGHT mode              |
| ,    | SETTINGS            |                         |
| .    |                     |                         |
| /    | tgl COMMENT         |                         |
| ~    |                     |                         |
| LT   |                     |                         |
| RT   |                     |                         |
| UP   |                     |                         |
| DN   |                     |                         |
| ENTR | tgl CHECKBOX        | show LINK in pane       |
| INS  |                     |                         |
| DEL  |                     |                         |
| HOME |                     |                         |
| END  |                     |                         |
| PgUp |                     |                         |
| PgDn |                     |                         |
| 1    | xxxx                | MERGE one window        |
| 2    |                     | SPLIT vertically into 2 |
| 3    |                     | SPLIT horizontally      |
| 4    |                     |                         |
| 5    |                     |                         |
| 6    |                     |                         |
| 7    |                     |                         |
| 8    |                     |                         |
| 9    |                     |                         |
| 0    |                     |                         |
| F1   |                     |                         |
| F2   |                     |                         |
| F3   |                     |                         |
| F4   |                     |                         |
| F5   |                     | PREV daily note         |
| F6   |                     | TODAY'S daily note      |
| F7   |                     | NEXT daily note         |
| F8   |                     |                         |
| F9   |                     |                         |
| F10  |                     |                         |
| F11  |                     |                         |
| F12  |                     |                         |
|      |                     |                         |

