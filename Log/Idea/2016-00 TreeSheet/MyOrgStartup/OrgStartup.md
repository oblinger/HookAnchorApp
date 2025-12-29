EverTask -- written in 2010
MyStartupOrg -- written in 2014

# Old features

- Multiple Live Views
- Context Tag Selection
- Programmable view:  tags, modTime, createTime, 

- Word / Tree -- synonyimzer -- 
- Editing template == editing note

- Cockpit View constructable -- Savable; LiveData; EmbeddableInOther; Parameterzied

- Supervariables -- Allowed anywhere in spec, Controllable w. slider/inputs/box/etc., 
                    inverted captured. -- computed using referential langauge.

- Actions on lists. embed workflow, move/copy/del from other lists


TEMPLATE DUALITY -- Editing a tempalate for a view/note/etc is the same as editing a view/note/etc.


UNIVERSALITY -- SPREADSHEET PROGRAMMING -- view is 'blank' but relation between 'cells' are systematically configured so that appropriate relations are maintained.
Idea is that today's tools can be 'programmed' into behavior of this generic system
   |
What happens When
- edit/create-child/delete/move/copy/checkoff/change param

LIVE DATA -- Entire view defined declaratively so all data updates live in all directions.  
SIMULTANEOUS VIEWS -- Data appears to simultaneously organized in multiple ways -- all changes refect as they should.


OBJECTIVE -- Display information 

COCKPIT CONSTRUCTION -- Key objective is creating a presentation of information in a way that supports a task.

REIFIED VIEW -- View itself is namable, embeddable, 

SCRIPTING BY DOING -- Operations used to configure view are 'programming that view' so it can be applied in analogous way in future.


MUTI-GUI FORMAT  list 
# Killer Design Features

## DRIVING IDEALS
--- EMERGENT UTILITY  -- User primarily performs actions that acieve immediate ends, yet emergent asymptotic result achieves all driving ideals here.
--- INHERENT INDEXING -- Using only info already in their brain, user robustly places and retrieves items from an unboundedly increasing universe of things


## PLACEMENT
   ONE "PLACE" FOR EACH THING 
   - UNIQUES   --  All 'things' should have exactly one 'place' where they can be found.
   - RETRIEVAL --  Robust retrieval acheived by user 'following the obvious path, using indexing already in their head'
   - CLOSURE   --  Coverage of all relevant aspects of a thing can be accomplished by traversing DAG below that thing
-  SOLUTION -- GLOBAL/LOCAL naming: 
                Knowledge organized into a forest of named trees, with cross linking.
                Roots are glabally named, Internal nodes referenced by local name & path from root.
                Many alises for root names, and cross linking ensure robust retrieval even with ambiguous indexing, while reinforcing canonical locating structure.
- ADDITIONAL PROPERTIES
  - Emergent organization.  Naming and structure updated incrementally/locally and still asymptotic structure retains PLACEMENT PROPERTIES


## RE-VIEWABLE
   A 'RE-VIEW' is a re-structuring of previously entered items and new items into an new structure.
   - Primary organizing activity done to:  (1) create place for new item, or (2) to arrange info in best way for an immediate activity (what to do this week)
   - PERSISTENCE -- Key cognitive activity in RE-VEIW is to determine which items / item-groups are important for the current intent and how to 
     best structure items for current 
   SOLUTION:
   - A re-view is an item named by topic and intent.
   - Under it is a tree of embeded item trees with persistent view settings
     + Embed single item initially in closed state
     + Embed range of items from the body of another item
       - Header line is hover discoverable but takes up little vertical space
       - Subranges of items from embeded body are elidable.  '. . .' takes up little vertical space, and is visually unobtrusive, and persistent.
       - Each item has an infinite number of empty entries at end of its body
       - OPERATIONS:  Dragging '...' will elide more or less either above or below, from underlying list.
         '-' will elide current item.  '=/+' will un-elide next item and move to that item.
             From embed header: '-' this will cycle thru elide none/all.  '+/=' will unelide first item and move to it 
         'delete' will delete current from origin and here.
         'SCROLL' -- each toplevel embed can be scrolled (length is constant but beginning of embed region is shifted)


## ENTRIES
### Entries may have
#### An optional (within a user's namespace)
#### A Titleline
#### A body with text ad  links to files/urls
#### A tool tip text???
#### A list of child entries
#### A filter

## ENTRY VIEW STATE
View state info is indexed by the display path back to the layout entry.
Or default layout for is used.
### Each entry has a display state.
It may be in one of the following states: closed, partial, open, expanded
### Children may be sorted by (alpha)
### Descendents may be shown or now




## ENTRY TYPES
### TAG -- Entry that 'tags' entries it LINKS to.  (can be multiple)
### CATEGORY -- Like a tag, but there can only be one.
### TYPE -- Built-in Category with special semsntics (e.g. entries in this list here)
### PROPERTY -- Child entry specifying additional info (hidden)
### TEMPLATE -- Entry used for new entry creation
### TRAGET -- Entry where other entries can the thrown.
(Throw entries become children of the target)
### FILTER???
### NAMESPACE -- Entry that indexes all node under it.
(By default associated with a single user)

### SPECIALTY ENTRIES
#### LAYOUT
A 'layout' is a view set with presentation data.
- It specifies entry placement on a canvas
- Some spcialized entries (like selectors, and linked view ports)
#### VIEW SET
A 'view' of the data is a perspective, a "place" within the data.
- It is constrained by/specified by a 'Top Node'
- It maintains display states for nodes
#### SIMPLE LAYOUT
- A combo box across top, and a list of columns, w. top of col entry


#### SELECTORS
A COMBOBOX selector is a special entry that allows the user to "select" a top entry for its layout
- A 'combobox' accepts text input to select an new top entry by name
- It also presents a tree of all named entries under the selector
  (entries with only a single child are merged into their parent)
- The presentation tree may also be 'PEGGED' partially opened
  (this state can be locked when in authoring mode)
- When FILTER entries are selected they are listed on right hand side 
  of Combobox, clicking on them will remove them.

#### PRESENATION PANE
A presentation pane is a special entry that displays its layout's TOP NODE.





### SPECIALTY PROPERTIES / TAGS
#### LOOSE -- Entry does not hold onto children after move



_
# Killer Features

## TAG-LISTS -- A Key organizational concept.

CONCEPTUAL IPLEMENTATION
- a tag list (TL) is a named sequence of entries
- the top of the TL are 'promoted' entries that are presented in 'aggregated' views of the TL
- A TL entry can be a 'closed' or 'open' pointer to a tag list
  when closed it will display as a single summary line, when open all promoted entries are shown.
- A TL has a fullname (with periods), and a shortname (the part after the last period)  

GOALS:
- Provide a way to momentarily organize information that:
  - Leverages previous organizations
  - Provides strong ability to structure / hide / show data for current purposes

## Full features list
### Buttons
- "+" create button
### Keys
- At beginning of title line:  <backspace> / <space>  change indent
- <enter> new entry at with same parent

### Click
- Cycle thru states:  closed, partial, opened.
- On DOT.  Changes state, (e.g. done)
- At end of list.  Creates new entry

### Drag
- Dragging an entries title dot onto another entry causes that entry to 
  be LINKED under that entry within the parents ordering.
- If destination is in same tree, then it is UNLINKED from its original 
  location in the tree
- Dragging special entries: 
  TAG/CATEGORY adds label
  TEMPLATE creates new instance
  TARGET causes entry to be moved to this 
  

### Scrolling
#### Hover expand when using a mouse
#### Gap expander when using tablet
  (Maintains a user slected gap in middle of screen.
   entries are dialated above, below, and within gap.)
#### Glide expander
- User can push or pull any entry up or down.
    Dialation is done above/below entry in order to maintain entry under
    user's finger.
- Right/left drags dialate/contract entry under finger (or cursor?)
- Each entry uses decreasing space to present body of each subsequent entry
- Has cursor fixed at 1/3 point on screen.
- Right/Left moves faint triangle left right over cursor.
  (More to the left causes larger/deeper expansion


- Dialiation view auto opens children in order to maintain
  a fixed number of presentation lines.
-- It prefers to open breadth first, and first entries in lists first




## Single List of entries, and also structured lists

## Single Metaphor -- List Canvas
### Named  (in a user namespace)
### Nested list of entries

## Drag-n-drop entries
## Live&Synced embedding of Contacts(face/linked/gmail)


_
# SEMANTICS

## CORE GRAPH STRUCTURE
### NameSpace -- A set of entries.
- All tokens in namespace are indexed in a dictionary (sorted by prefix)
- Free text index also provided.
### Entry -- A thing that contains text (or the print form of a token)
### Edge -- A pair of entries (called up and down)
### Lists -- Each entry has an ordered list of up and down entries.
System maintains symmetry that if B is in A.up, then A is in B.down


## DERIVED PARTS OF AN ENTRY
### NAME -- the last up entry is to a TOKEN entry whose title contains the word
### PARENT -- the first up entry
(usually the entry that it has been the child of for the longest time.)
### TAG -- an up entry of type "TAG"
### TYPE -- a TAG of type "TYPE"  (a subtype of TAG)
### CATEGORY -- a TAG of type "CATEGORY"
### PROPERTY -- a down entry of type "PROPERTY"
name = value

## ENTRIES THAT DEFINE NEW ENTRY PARTS
### CATEGORY DEFINITION -- has ancestor "CATEGORY"
### TAG DEFINITION -- has ancestor "TAG" (which is a sub-category of category)
### TYPE DEFINITION -- has ancestor "TYPE" (a sub category of TAG)

### PROPERTY DEFINITION -- has ancestor "PROPERTY" (a sub category of TAG)

## DERIVED TERMS
### ANCESTOR -- A parent or ancestor's parent.
### UNDER -- An entry is 'under' another if it is recursively connected by the PARENT relation.
### NAMESPACE -- A namespace entry, n, proivde an index of all named entries UNDER n.
Thus all named entries must have a unique name within a namespace

### REACHABLE -- The set of entries recursively reachable by traversing he LINKED links fron an entry.

## FORMAT:
   <stars> [ [ <head> [","] ] { <tag> [","] } [ <name> ] " -- "] <title> "\n"
   <BodyLines>

   HEAD :==  <type> | <category> | <tag> | <target> | <name>



_
# USAGE SNIPITS

## Projects DAG. (both as a top node and as a filter selector

## Waterfall lists

## Context filter terms


## USAGE EXAMPLES

- TEMPLATE projects 

## EXAMLE LAYOUTS

- PEG BOARD.  Left hand strip has peg board of common entries
- PROJECTS LIST


_
# IMPLEMENTATION
## DATASTRUCTURES
### Entry -- the core 'thing' of this system
Entries can go in the bi-graph, and tokens can go in the dict or bi-graph
- STRUCT FIELDS: upIdx, downIdx, body (first line is title line)
### Edge -- A pair.  An up entry and a down entry.
- STRUCT FIELDS: upIdx, downIdx (could also include an upEntry for speed)
  (can be implemented as very large int array)
### Bi-Graph
Set of Obj, Obj pairs that are ordered for each Obj
(So ADD operator is ADD(graph, obj1, obj2, place1, place2)
 Here 'place' is either the obj, or a pair to add after)
### Dictionary
Ordered.  Indexed by token prefix.

## DEV ENVIRONMENTS

var myConf = {
    ide : 'Aptana',
    browser : 'Firefox',
    plugins : [ 
        'Webdeveloper toolbar', 
        'Firebug',
        'Firepallete',
        'FireRainbow',
        'Yslow'
        ],
    support : [ 'IE7-8', 'Safari', 'Opera', 'Chrome']
 }


