  [MyOrgStartup](MyOrgStartup/MyOrgStartup.md)
  [myorgstartup](myorgstartup.md)
  [myorgstartup](myorgstartup.md)
  [myorgstartup](myorgstartup.md)
  [myorgstartup](myorgstartup.md)
  [myorgstartup](myorgstartup.md)
  [TextPuter](TextPuter.md)  [myorgstartup](myorgstartup.md)  [KMco](KMco.md)
  [KMco](__KMco__.md)  [MyOrgStartup](MyOrgStartup/__MyOrgStartup__.md)  [TextPuter](__TextPuter__.md)
# ### INFO ### 
## === REF ===
### --- Pointers ---
#### all the KM apps out there
https://arieluna.notion.site/Apps-29ffc475bd3a4539b48eeb5bb551d713
[[Screen Shot 2022-03-15 at 8.10.10 AM.png]] 
# ### SYSTEM DESIGN ### 
## === 2021 Design ===
### --- Paid name launcher ---

https://www.fiverr.com/zahid105/convert-your-idea-into-an-ios-app?context_referrer=search_gigs&source=top-bar&ref_ctx_id=91188f88588c5518a4e61285d6c81769&pckg_id=1&pos=3&context_type=auto&funnel=91188f88588c5518a4e61285d6c81769&imp_id=f59ca032-65d1-4433-9681-88774ab5fdc8


Zahid,
I believe this app to be rather trivial in scope.  Still I am interested in getting it to operate across many edge cases, and also to have a simple but good build-chain that I can use to build the full application ontop of what you deliver.  Please let me know how you plan to price this combination of triviality plus robustness.  

OBJECTIVE:
Provide a MacOS application that can serve as the default web browser and forward all traffic to the previous default web browser unless the URL begins with n://... in this second case the url is passed to a python script for further processing.

A key objective is that this browser acts in every way as if the default browser was still loaded as the defualt browser, EXCEPT when urls with the specified "N://" prefix are provided.  In that special case, the application passes the URL to a python script for further processing.

IMPORTANT REQUIREMENTS
- SWAPPING -- At the bottom of this job spec is the stub python program you will be using in your development.  I should be able to replace this stub script with myown script and in this way specify what should happen when the user opens a URL that begins with "n://"
- PYTHON -- I will be supplying a trivial script that will write the url to the output file "/tmp/url.txt" anytime a "n://" ... url is provided.  Otherwise it will 

PYTHON INSTALLER REQUIREMENTS
- DRAG-n-DROP INSTALL -- 
- INSTALLER -- I don't want to depend upon any existing python on the target machine.  This app must either install with the python interpreter built in, or it must somehow pull this interpreter down from the internet w/o any user involvement.
- PYTHON 3 -- 
- SOURCE CODE -- 
- TKINTER --
- 



I am looking for a MacOS application which I can install onto a new computer and then select this application




### --- Super Quick MacOS Prototype
#### -- RETE - Indicators used for URL and MENU data --
- HOOK -- Each hook has an "IF-THEN" structure.  
- DATA -- The input data is a tree of text files.  output is a name tree
- RETE -- A rete-like engine to track hook rule activation and deactivation based on DATA deltas

DATA
- CONTAINER -- possibly ordered, possibly labelled, possibly indexed container
- ATOMS -- Textual, File, int, string, ?object?

HOOK
- The IF part is an expression that may bind variables, 
- The THEN part executes in context of those vars.
- VARS:
	- FROM -- path to source node
	- TO -- path to destination node
	- SOURCE -- source node
	- DEST -- destination node


MATCH EXPRESSION
- AND -- Conjuction of expressions, satisfied if all parts are.
- UNDER -- True if expression is true of parent, or recursively true of their parents
- REGEX -- Regex with named parameters for both node name, and node contents

VALUE EXPRESSION
- CONTAINER TEMPLATE -- list of insertion and modification actions

ACTION
- INSERTION -- Appends/set element of a container.  Parts:
	- WHERE -- computes parent
	- ID/NAME -- computes name and id abbreviation
	- DEST -- computes 
- DELETION -- Undoes the effect of an insertion (for rete removal of an 'unfired' rule)

_
#### -- URL - targeting by filename or content hooks --
#### -- MENU - Merge menus
- NAME TREE -- Tree of alpha numeric names and IDs that map onto URLS
- MENU NODES -- Certain naming tree node


ID Entry:
- PARENT -- Parent entry indicates position within the tree
- ID -- Case insensitive abbreviation name for entry
- NAME -- Case insensitive name
- URL -- Url mapped by this id

_
### --- Super Quick Text-only Prototype ---
- Finish UF-structure, and UF-eval and Py-bridge
- Build Text file bindings
- Build listener

DSL
dotted.$A.path.text ==> other.$A.path.text
dotted.$A.path <==> other.$A.path

dotted.$A.path.text.outline(MARKDOWN) ==> other.$A.path.text



TXTPROTO := L.topics.ufdocs.treesheets.system design.2021 design.super quick*



_
### --- Data Values ---

- CELL -- All data stored is stored in a placespace of cells
- AGGREGATED PATCH TREE -- For each cellspace patch:
	- The patch expressions are evaluated in the context of the attachement point.
	- The resulting patch is aggregated in application order across the whole tree
	- ???  is this computation correct ???
		- Aim for the result to be mostly indpendent of application order
- INFO IN CELLS
	- APP		-- 	the application itself
	- DATA STATE --	Data-related state configuration (e. g.) tabs that are open
	- EXTENSIONS	-- 	Extensions added to the application
	- CONFIG		-- 	System and user configuration of the system
	- MODE		--	"Mode" configuration
	- SCREEN		--	Screen-wide configuration
	- STATE		--	Interface state configuration (e. g.) pane sizing value
	- DATA		--  The data underlying the displays
- LAZY COMPUTED CACHED DATA


ROOT
- state...	-- Current tree of configuration patches (all are applied)
- fn.xxx		-- These are defintions 
- gui.xxx	-- Defines the current interface
- data.xxx	-- Is the tree of underlying data

KINDS OF DATA VALUES
- VALUE -- computed data value
- DEF -- a formula used to compute a derived value.  If 'fn.foo' is a 
	- DATA FORMULA -- then 'xxx.foo' derives a value from 'xxx' using foo
	- TREE TEMPLATE -- then:  gui....zzz := foo(arg1=27, arg2='example') used to extend gui 
- GUI_ELE -- a gui element
	- LIST(GUI_ELE..., _=..., _size=SIZE)
	- TREE()

_
### --- Python-based editor ---

STEP1 -- A TKinter editor based solution
- EDITOR -- Adapt rich text editor for use
- INTERVALS -- Track intervals as edits are happening
- CHANGE STREAM -- Provide change stream



STEP2 - A JS-viewer ---
- DJANGO stack
- JS FRONT END
- RUNS LOCAL OR ON HEROKU
- DOM FORM -- simple DOM form encodes interface with rich text areas w. custom intervals and hover regions
- JS EDITOR -- specially adapted js editor tracks intervals and hover regions
- CHANGE PROPAGATION -- BI-DIRECTIONAL CHANGE PROPAGATION WITH CACHED COMPONENTS


_
## === USE CASES ===
### --- Project/Task Management ---

- represent info in documents, but process it as knowlege graphs
- Projects linkage graphs from:
	- file folder structure.  
	- Document heading levels and organization
	- explicit link references
	- from loose containent within sparse hierarchies
- Tasks/todos embedded with special syntaxes

### --- Super CRM ---

IDEA: CRM that serves its key functions but over info that is externally stored and is very flexible about how it is organized.
- Organizes **externally stored** info and indexes it by person, by company ...
- Live management of linked data (so if name is changed all references are edited)


KEY CAPABILITY
- BI-LINKS -- Live bi-directional entity links (so renaming maintains link)


USAGE
- TRIP REPORT -- Can organize info into a trip report document, and over time name linkages can be refine and maintained while keeping info in the trip report document.  but this info is ALSO available in all other CRM views.
- 

_
### --- Info space morphing over time ---
BACKGROUND:
- Person has a large complex body of information whose organziation morphs and clarifies over time.
- At several points in time a new-ish global organization is created
- It will build from and reuse parts from prior origanzations, but will refactor parts of it.


WHAT IS GETTING MORPHED AND REUSED?
	- Screen layouts
	- Grouping of tags and terms
	- List of like entries


KEY ORGANIZING AXES:
- TIME - global, totally-ordered, complete, 
	- creation time, modification time, chunked-mod time, version-time
- LOCATION - 
	- data storage(ownership), logical namespace location
- LINKAGE SPACE - 
- SCREEN LAYOUT


BUCKET == Single version of a project or activity; can be a progression over time


REUSE:


_
### --- trello/project criss-cross ---

Context:
- TRELLO BOARD -- User like using trello boards to track progress on activities
- TWO FUNDAMENTAL ORGANIZATIONS -- By project planning and organization; by activity/priority exec
	- DURING PLANNING -- User uses documents organized into project, sub-project
	- DURING EXECUTION -- User operates across multiple projects and activity types


For example:
- THE "NOW" TRELLO -- High priority -OR- close due date
- THE "OFFLINE" TRELLO
- THE "PHONE" TRELLO -- Executed in early afternoon

Documents:
- folder hierarchy indicates project hierarchy by default.
- Some smaller folder are merged into a 'super project' for simplicity
- Some complex projects are broken out into multiple projects according to time horizon of outputs

Data Representation:
- trello column is expressed as a status encoded via entries property (matched by regex)
- project priority is expressed by property, or by sub-header
- project item ownership is expressed by location of entry in hierarchy.


Derived list -- computed by filtering tree nodes



_
### --- 2021-10-07 --- Dusty Robotics ---

BOM -- Bill of Materials
- Multiple overlapping ones.  (bare robot, full package w/ measuring tape, components at risk)
- Different aspects with/without pricing etc.  multiple rows for different vendors of same part.
- Robot is changing over time
- Different Robot versions in parallel.


Part Numbering
- Multiple incomplete numbering schemes
- Orders/Plans using different schemes
- Mapping tables between schemes

_
# ### LOG ### 
### --- 2021-11-26 - Testing the market ---

VALUE PROPS:
- YOUR TOOL - Your data in the tool you want
- MANY VIEWS - Same data synchronized between many views
### --- 2021-10-04 - Slide deck notes ---

#### -- Spreadsheet analogy --

Treesheets are spreadsheet's friendly but more powerful big sister:


WYSIWYG -- INTUITIVENESS
==> Both are programmed by direct WYSIWYG-editing of the desired output format

STRUCTURED CELL -- UNIVERSALITY
==> Both build from a BLANK SHEET of data cells upon which any structure can be built

DATA-FLOW -- SIMPLICITY
==> Both achieve great power and flexibilty using a trivial data flow model where cell values are continously computed from other cell values

TEMPLATING -- COMPREHENSIBLE and GENERAL
==> Both achieve great generality 




			SPREADSHEET					TREESHEET
STRUCTURE	2D Grid	of cells		==>	Tree of cells
CELL DATA	Used for content data	==>	Formatting, structure, and state are expressed using cells
VIEWS		Single Congured View	==> 

