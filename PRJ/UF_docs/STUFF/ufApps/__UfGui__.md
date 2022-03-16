
# ### COMPONENTS ###
## === DATA FORMS ===
### --- Special .md files in filesystem ---
### --- Google doc/sheet - with tree / table transparent organization ---
### --- HTML - output clickable-links files ---
### --- Github repo ---
### --- Zipfile ---
## === DOM - Widget - Generalized GUI elements ===
### --- DIV/ELEMENT ---

DIV -- A Browser DOM Div (Div) is a single independently controllable graphical element within the browser.

- DivType -- 
- DivId --

- GROUNDING -- The instantiatiator for the widget or widget template
- PARAMETERS
	- CURSOR PARAMETERS -- these parameters indicate what data is to be displayed by the widget
	- CONFIG PARAMETERS -- portion of widget parameters indended to be relatively fixed, these are specified at instantiation time
	- STATE PARAMETERS -- set of parameters that indicate the current "view state" of the widget
- SCROLL BARS --
- HOVER TEXT -- 
- DRAG ACTION -- Drag action of entire widget or sub-part onto another widget
	- Drag form fn -- function returning glyph to use when hovering over different drag targets
	- (Text and tree will both indicate sub-place within their structure)
- CSS MARKUP --


SCROLL BARS -- An element may have horizontal and/or vertical scrollbars that are fixed or automatic based on size.
HOVER TEXT -- An element may have Hover Text
CSS MARKUP -- An element may have CSS markup tied to is div type or div id


DIV TYPE -- Each screen has its own hiearchy of div types matching the CSS document for that screen.


_ 
### --- GRID/FLOW ---

**LAYOUT GRID** -- A _**layout grid**_ 12x12 bootstrap-like grid that lays out its sub-elements onto a 12x12 grid with optionally draggable resizable borders.

**LAYOUT FLOW** -- A _**layout flow**_ is a placement rule for a variable length list of elements.  Can be horizonal, vertical, or wrapping.
- DIRECTION -- flow direction is horizontal, vertical, or wrapping.
- GAP -- special 'gap' elements indicate spacing should be equally introduced to fill region
	(optional numeric parameter indicates more >1.0 or less <1.0 spacing at a spot)
- STRETCH -- special 'stretch' elements indicate the following element can be stretched along with gap spacing.
	(optional numeric paraemter indicates more or less stretch)


>>> Maybe GRID/FLOW is all we need we can ignore these two <<<
ROW / COL -- A row or column (Row/Col) is a Div that lays out n sub elements in a row or column.  

- The Row/Col has n biputative positive floating point numbers that always add to 1.0.  
- These numbers indicate the relative size of each partition 
- May have user draggable borders

_
### --- TABBED PANE ---

TABBED PANE -- A tabbed pane 

uses Code unit with items being tab name and contents

biputative code list
- uses code2dom div-list mapping.  (key for each div is stored on div and div content is the div body)

_
### --- TEXTBOX ---

**TEXTBOX** -- A textbox widget whose contents are a single pane text box.

- May be a single line high, or a multi-line pane
- May have horizonal and/or vertical scroll bars
- Clicking within text can indicate 'place' within parse tree that was clicked
- subrange -- might only render a subrange of text list underlying box

_
### --- CHOICE ---

- May have state specific decoration (e.g. button pressed / unpressed)
- May maintain toggle state between n states (often just two states)
- May open a "choice box" style list of choices
- May have scrollbar on choices if many
- May support "type in" choice where all choices are narrow based on prefix matching the typed in text
- May support simple "type in" choice
- May just display a text or glyph content (as a label would do)


DAG Selector
- First level of tree is shown at top
- Double break, then most common leaf selections listed next
- If mouse is used, the top section is a menu-submenu input
- If keyboard is used, the prefix string generated will perform prefix selection on BOTH lists
- As soon as the prefix indicates a single sub menu, that submenu is listed at the path at top, 
	and the contents of its submenu is listed in the top menu section
- At all times the top most common leaf node is highlighted, and is the one that will be selected by space or return
	EXCEPT when the submenu narrows to a single selection, then that item would be highlighted


Proj > Proposal Work > Bayer Proposal
- bacground materials
- related work
~~~~~~
CNN Report
Bayer Proposal

_
### --- OTHERS TO ADD ---
- Table
- Multi-selector
- Graph (node and arc) view
- Color Picker
- Font Picker

multi-selector ???? do we need this?

graph view ???? do we need a link and node graph view ???
color picker
font picker
_
## === DOM - Tree Related Widgets ===
### --- TREE OUTLINE ---

Parameters:
- Root Place
- Leaf? -- uses the 'prune' tree operator
- Subrange -- might reactively compute display of only a vertical portion of tree
- Row -- Each row can be a head icon and simple text or widget

Operations
- INSERT BEFORE/AFTER -- 
- DELETE -- 
- DRAG -- The drag operation links a source place to a dest place (typically resulting in move or copy)
- INDENT/DEINDENT -- 
- FOLDING -- Moves viewing of node thru n 'openess' states

_
### --- TREE CURSOR ---

TREE CURSOR -- A Tree Cursor indicates a single location within a tree

Shown as a path list where each element is a choice that may be clicked inorder to select a different child at that level.

_
## === DOM - Special purpose controls ===

For the most part these controls are trival instantiations of DOM general elements.
They are specifically named and instantiated, since the semantics of the authoring tool is built around these elements

### --- LOCATION ---

A tree cursor

### --- LEVEL OF DETAIL ---
Choice box specifying one of n levels of presentation detail

### --- STEP ---
Shows path of current sub-step
# ### AUTHORING INTERFACE ###
## --- Interface ---
Thin instantiation of widgets to generate interface


- LOCATION -- Tree cursor showing 'where' view in currently pointed
- STEP -- Tree cursor showing what 'step' is currently being executed
- STATUS -- Status/message bar at very bottom of interface
- LEFT / RIGHT / BOTTOM / CENTER -- 


- 
- PALETTE -- Tabbed pane of flows, lists and trees of components to be inserted

_
## === USAGE EXAMPLES ===
### --- GUI meta authoring ---

WYSIWYG authoring of interfaces.

EXAMPLE LAYOUT

INTERFACE:
- MAIN PANE -- Shows a live application of the interface to some starter data. (left hand side)
- PALETTE -- Drag and Drop components to be added by dropping onto existing main-pane widget elements
- PROPERTIES PANE -- Possibly tabbed pane of properties that are all indexed off of the place indicated by a single cursor

design of a GUI configuration is done as a recursive wizard instantiation

CONTROLS:
- Clicking the 'edit' button while using an interface will flip into interface editing mode
- Both the palette and properties pane appear; the drag and drop becomes a reconfiguration operation where:
	- lists are reordered, elements pulled away are deleted, widgets can be moved
		(if they are from a constant template list)
- List of Wizards can also perform more limited configurations

_
### --- Recurive Wizard Instantiator ---

INTERFACE
- INST -- The instantiation tree.  (steps shown on each row)
- MAIN -- The main inspection/view pane

- Generic config-state driven walk thur of instantiation process
- Seperate "cursor" is maintained at each level within the config state
- Input paramters are highlighted on interface if the need to be filled, or already filled
- Inputs may or may not have computed defaults.
- Input

- Recursive walk thru of configuration

- CONFIG STATE 
	- recursive tree of parameters
	- each level is first a binding of a dependency with a matching filler component
	- filler component may recurively have dependcy parameters and/or scalar parameter values

- STANDARD WORK FLOW -- unfolds as config state is instantiated
	- COMPONENT FLOW -- each component has its own workflow of parts to instantiate, steps taken, and info provided.
_
#### --- Single parameter field info
- purpose of the field
- effects of the field (automatically determined & textually listed)
- 
- FILL STRATEGIES LIST
	- strategy for filling it (explanation of this alternate choice)
	- computed default value for this approach


_
### --- Info Manager ---

- LOG -- Single sequence of notes
	- MEETING:  next steps, todos, followups, participants, company
- PROJ -- Tree of projects / subprojects, ...
- TAGS -- 
- TODO -- Priority ordered list of activities
- CALENDAR -- 
- WEEK PRIORITIES

- WORKFLOWS
	- Monthly/Quarterly/LongerRange planning
	- Weekly
	- Daily
	- 


### --- Info Manager Common Workflows ---
#### -- Info Ingestion --

- Modeless, ready anytime.  One key open
- Top of log with fresh entry indicating time of entry
- Maybe pulls in calendar entry info if connected
- No requirement to indicate category, tags or anything... just text
- Palette off to right of special markup that could be used

_
#### -- Triage To Zero -- 

- TRACK -- System tracks which notes have not had triage organization performed on them
- PROMPT -- System prompts user accrording to a configured workflow to do a quick check on these notes in order to catch any followup actions or such that should follow from them
- ACTIONS
	- BUCKET -- put note into general category bucket
	- FOLLOWUPS -- encode all followup actions that are needed for this.
	- LINK -- add any relevancy links in order to be reminded of this in other projects

_
#### -- Category Morph --

- Over time active categories will shift
- RENAME -- Category can just be renamed and all connected meta data will transform
	(sometimes after the fact, if that external data is not being modified, timestamps are used to track this)

_
#### -- Gather Info --

When beginning a new info processing task one must find and organize relevant materials:

- Build heap
- Organize info


_
#### -- IORG - Organize Info --

Given an info heap IORG is the process of organizing it.

- PARTIAL -- process may not complete, and contents need to be meaningfully organized, even in these partials states
- HEAP 2 TREE -- process is non-linear transform of heap into a tree.




_
# ### LOG ###
### --- 2021.03.16 - TREE GUI ---
#### -- Capabilities --
GUI ELEMENTS:	Row, Col, TextTree, Text
_
#### -- Text Tree --

- NODE -- sequence of named and fixed entries
- DRAG -- performs binary operation, may prompt user for additional parameters≤
