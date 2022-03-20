

## === OVERVIEW ===
### --- BIRDS EYE THINKING ---


LANG	EVAL	PKG		FLOW
UNIT	SPACE	PLACE	FLAT


LANG<UNIT  <<  LEX<PLACE  << FLAT; PKG; EVAL<FLOW 


UNICORE STRUCTURE

UNIT
- VERT 	- STRUCTURED DATA - Possibly ordered, labelled, directed graph with 4 access and 2 manip ops
- MODS 	- VERTEX MODIFIERS - Componential set of vertex modifiers
- UCM	- UNIFORM CODE MARKDOWN - List/Map/Number/String, Head, Stmt, 

PLACE
- LEXSPACE
- PATH
- ADDRESS
- PLACE / SPACE
- ASSIGNMENT



UNICORE FUNCTION

CTX  - CONTEXT  = INSIDE/OUTSIDE, PATCH
EVAL - EXECUTE	= Env, Exe, Op
PKG  - PACKAGE  = IMPORT/EXPORT, ???
SLOT - VARIABLE = 
FLOW - CONTROL	= 
LANG - LANGUAGE = 


#### -- Dependencies --

CTX - PLACE
_
### --- THESIS STATEMENTS ---
#### UNIT Thesis

Vertex-centric modifiers and graph-operators provides an elemental/componential framing of structured data.



**GRAPH-RESTRICTION THESIS** -- These 11 vertex attributes form a power-spiral ring underlying all computation.  (e.g. The powerset of these combinations are useful enough that engineers can elegantly build from these semantic combinations without extension or refactoring them.)

NON-CONFLICTING COVERING SET OF MODIFIERS

_
#### LEX

The Lex paradigm is a uniform framing of the interrelated notions of location, space, address, and path.

_
#### PLACE

The Place paradigm is a uniform.  That is, its place-value-access-assignment model of persistence is both elemental and compenential.

_
#### FLAT

Meh, six-of-one half-dozen of the other -- there does not appear to be a particular flattening approach that is more elemental/componential than others.  Thus we do not make a strong flattening thesis claim here.  Still we do claim the four flattening operations contained in this paradigm are essential in the sense that it seems any alternative flattening technique that does not provide a suitable replacement for each of these four transformations will be less capable of dealing with certain important types of graph-structure complexity as listed here:

Flattening Claims:
- **META INJECTION IMPORTANCE** -- Having a separated "meta data" channel seems to be essential in combining "8-bit-clean" structural data with non-structural information about that data.  Further we show the FLAT paradigm is 8-bit clean (can losslessly reproduce any finite graph structure) w/o confusion introduced by presence of meta data.
- **FUNCTIONAL SIMPLIFICATION** -- The functional-simplification ensures that the mapping from key to value is functional (well defined) for each unit within the structure.  Further it can ensure that the key values within the resulting structure satisfy some limitation require for embedding within other representation systems (like lexspace) that place restrictions on the complexity of the indexing structures.
- **CYCLE BREAKING** -- The cycle breaking operation translates graph structures into a form where "simple minded" graph travesals algoorithms (like the one used for character flattening) are guaranteed to terminate.
- TEXTUAL FLATTENING -- 

_
#### GND

??? GND is a uniform framing of 

The slot 

good enough to support all of the following common forms of templating w/o adding any constraint or significant complexity:
for function-call argument parsing, string formatting, back-quote like structure creation

General thesis.  Slot is a uniform paradigm for SW templating.

_
#### EVAL

Uniform evaluation is a least-commitment model of computation which can serve an "outer shell" for any other interpretation paradigm without adding any commitment beyond the commitment to the federation of interpretation paradigms and a commitment to the use of unit data.

e.g. framing all forms of interpretation as uniform interpretation will always "works" and in each case adds only the modest complexity of interpretation-federation as indicated above.

_
#### FLOW is a uniform framing of "procedural control flow"

Flow is an elemental and componential 

_
#### PKG THESIS


_
### --- APIS ---


**pkg** _lang_:

	_UNICORE_: = UC
	
	_UC_:
		= UC.STRUCTURE + UC.FUNCTION
		_STRUCTURE_ = UNIT + PLACE + UCM 
		_FUNCTION_ = LANG + CTX + EVAL + PKG + SLOT + FLOW



**pkg** lang.UC._UNIT_:
	Unit:
		args: $K, $V
		_GET_ = **slot**(io:  (_self_: Unit, _key_: Unit, -> Unit)) 
		_SET_ = **slot**(out: (_self_: Unit, _key_: Unit, -> Unit))
		_ITR_ = **slot**(out: (_self_: Unit, _key_: Place[$K], -> Stream[$V]) 
		_LEN_ = **slot**(io:  -> Int
		_MET_ = **slot**(out: (_self_: Unit, -> Unit)
		_NEW_ = **slot**(out: (_self_: Unit, _template_: Unit=None, *, _head_: Unit, -> Unit) 

	Atom:					// An atom has structure, it is just always empty
		**extends** = Unit
		
	Coll:

	Stream:
		args: $V=Unit	
		_next_	= **slot**(out: -> $V)
		_has_next_ = **slot**(out: ->Bool)




**pkg** lang.UC._PLACE_:
	Unit:
		_lex_    = **slot**(io: ->Lex)
		_path_ 	 = **slot**(out: (_u_: Unit, *, _from_: Path, -> Unit))
		_follow_ = **slot**(out: (_u_: Unit, _p_: Path, -> Unit))
		_up_ 	 = **slot**(io: -> (_parent_: Unit, _idx_: Unit))

	Ident:
		**extends**	= Unit
		_path_	  = **slot**(out: List[Int||Str])

	Lex:
		**extends** = Unit
		_value_   = **slot**(io: -> UnitOrVoid)

	Env: 
		_origin_  = **slot**(io: ->Lex, @persist)

	Ident = ALIAS(Lex)



	UnitOrVoid:
		// get/len/meta/itr all return void. set generates an error
	
	Unit:
		extends = UnitOrVoid
		
	VOID	// the singleton constant used to denote no value... the endless void



**pkg** lang.UC._LANG_:
	_Code_ = **class**(Text || Spec || Form)
	_Text_ = **class**(**extends** = Str)						// Code's textual form
	_Spec_ = **class**: **extends** = [Heady,Boundy,Tree]		// Code's homoiconic form
	
	_Form_: **class**: 
		**extends** = Tree(Form) 				  		  // Code's executable form
		_text_ = **slot**(a:[Spec], t:Text)	
		_form_ = **slot**(a:[Spec, ], t:Form)
		_spec_ = **slot**(a:[self, spec], t:Spec)

	Env: 
		_text_ = **slot**(Code, -> Text)
		_spec_ = **slot**(Code, -> Spec)
		_form_ = **slot**(Code, -> Form)



**pkg** lang.UC._CTX_:
	**import** lang.UC.PLACE
	
	_Patch_ = class(Ident=>Unit)

	Env:
		_lookup_(_p_: Path, body: Block=None, ->Unit||Env)
		_with_(p: Patch, body: Block=None, ->Unit||Env)

	spec.code:
		Patch($_KEY_, $_VALUE_) = U.'->'($_KEY_, $_VALUE_)



**pkg** lang.UC._EVAL_:
	Form = **class**(**extends** = [Bounded, Immutable, Taxonmic])
	
	_Interpreter_: **class**:
		**extends** = Form
		_apply_ = **slot**(_operator_fn_, _expr_: Form, *, env: Env, -> Unit)
		
	Env:
		_eval_ = **slot**(_env_: Env, _expr_: Form, -> Unit)
		_ground_ = **slot**(_env_: Env, _expr_: Form, -> Form)

	// Implementation
	**def op** _eval_(_env_: Env, _expr_: Form, -> Unit)
		_operator_fn_ = _env_.ground(_expr_)
		_interpreter_ = _env_.ground(_operator_fn_)
		**return** _interpreter_.apply(_operator_fn_, _expr_, env=_env_)




lang.UC._SLOT_ := lang.UC.SLOT.VAR + lang.UC.SLOT.TYPE + lang.UC.SLOT.TEMPLATE

**pkg** lang.UC.SLOT._VAR_:
	_Slot_:
		args: 	[$T]					  // Slot parameters
		_name_:	**slot**(t: Ident, <Fixed)		// Namespace
		_type_:	**slot**(t: $T)					// Field / Return type
		_expr_:	**slot**(t: Spec)
		_stem_:	**slot**(t: Class)				// Self type 
		_key_: 	**slot**(t: IdentKey)
		// DERIVED
		_parent_:	**slot**(t: Pkg)
		_schema_:	**slot**(t: Class)

**pkg** lang.UC.SLOT._TYPE_:
	Form:
		**extends**: Pkg
		_backing_: **slot**(t: )
		// DERIVED


		// AS A SET OF INSTANCES
		subclass = **slot**(out: fn(_other_ Form, -> Bool))

	Unit:
		ISA = **slot**(Unit->Bool)
		NEW = **slot**(t: `this.form)
		'=~' = ALIAS(ISA)

**pkg** lang.UC.SLOT._TEMPLATE_:
	Form:
		fill = slot(out)
		match = slot()



**pkg** lang.UC._PKG_:

	Form:
		'+' =

	PkgSpec:
		::= **import** IDENT
		::= import IDENT as IDENT
		::= imports 


**pkg** lang.UC._SPEC_:

	::=	$FLOW || $PKG || $CTX || $SLOT || $EVAL || $LANG ||

	// UNICORE FUNCTION

	$FLOW	::= $BLK || $BRA || $REP || $INV || $ RET
	$BLK	::= **blk**([$EXPR], $FLOW...)
	$BRA	::= **bra**([$EXPR], '->'($EXPR, $FLOW...) ... )
	$REP	::= **rep**($STMT...)
	$INV
	$RET

	$PKG	::= **pkg**($PKGSTMT..., ...:$FORM)
	$PKGSTMT::= **import**($IDENT, **as**:$IDENT||$VOID)
	$PKGSTMT::= **export**($IDENT, **as**:$IDENT||$VOID)

	$CTX


	$PKG	::= _pkg_($IDENT): $STMT...

	$SLOT	::= slot()
	


	IDENT 	::= U."."('I', KEY...)
	KEY		::=	INT(...) || STR(...)
	

_
### --- TEMPLATE ---
#### === ELEMENT ===
##### _

_**TL;DR. xxx**_

###### Thesis
###### Overview

_
##### --- API ---

_
##### --- VOCABULARY ---


_

_
_
_
# ### FROM ELEMENTS ###
### TLDR top

**TL;DR #2 -- Intrinsic tradeoffs in software constructs exist (For example, dense arrays verses sparse maps represents a "real" split).  Any such "real" splits should have BOTH sides of the split included within a complete programming ecosystem.  By contrast, historic or convention-based splits could be avoided; we contend this would allow all software to be merged into a single programming paradigm without loss of performance or capability.**



The uniform agenda is so wildly ambitious in it aim, it is not possible to write down any particular paradigm and have confidence that it belongs in the Uniform ladder of paradigms.  Instead we generate and then iteratively edit the entirely of this DAG progressively approaching the criteria we set forth above.


two ideas:  single ecosystem; expressing effort in maximally interoperable form
### splits

- **ADDING SHARP CONSTRAINTS / INTRINSIC SPLITS** -- The notion of a DAG (directed asyclic graph) is a constraint on graph structure.  We argue this graph constraint (as opposed to many other possible graph restrictions) is an intrinsic split.  Why?  Because it serves as a mathematically sharp boundary enclosing exactly the set of graphs over which algorithms like the classic depth first tranveral (DFT) algorithms work properly.  (See the example below for a contrived alternative decimation that should be rejects as it is not a forking split.) concept which could also be usedIt is the precise boundary separating the set of graphs where DFTs are guaranteed to terminate from those where termination is not guaranteed.  The decimation boundary also guarantees termination, but it is not a sharp boundary; there are guaranteed termination cases that lie on the other side of this boundary -- it is overly conservative -- thus any DFT algorithms build upon decimations is a fork.
### rung list

RUNG 1 -- "MATH" 			  -- Paradigms:  Graph, Number (Natural&Decimal)
RUNG 2 -- "COMPUTE"			  -- Paradigms:  Abstract Rewrite Systems, 
											 Term Rewrite Systems
RUNG 3 -- "UNICORE STRUCTURE" -- Paradigms:  Unit, Space, Time, Flat
RUNG 4 -- "UNICORE FUNCTION"  -- Paradigms:  Ground, Interpret, 
											 Control Flow, Package
RUNG 5 -- "LANG AUHTORING LANG"  Paradigms:	 Composite, 
											 Parse, Code Markdown, Lang Mark
											 Mount, 
_   
### old thesis statement
	{[This thesis statment is unclear, and unsupported]}
THESIS:  Requiring that future data structure modifiers remain compatible with this starter set is not a damaging constraint.
_
# REPRESENTATION RUNG

## PLACE

### --- PLACE OLD Overview ---

_**TL;DR.  The Place paradigm is mostly just what one would expect, while adding the dual notion of location bringing all the intuitive placement properties that exist in the physical world but missing in most programming languages.  Also the place paradigm generalizes the notion of place to include a powerful form of  entity destructuring that is only partially covered in a few langauges.**_

The unusual bits of the place paradigm are summarized here at the top, and the full boring spec of the place paradigm follows this:


KEY ASPECTS OF PLACE
- An object can only be in one place at one time.
- A place can only contain on thing at one time.
- A uniform universe IS
- All things are somewhere -- they are in a place.
- In uniform all "things" are data and all data are a graph of unit.
- All "Things" in uniform are units, and everything thing exists
Placement properties follow 

formal spe
These unusual bits are summarize at the 


missing from many programming languages along with all the nice placement guarantees



_

# FUNCTIONAL RUNG
## Intro / discussion

### --- Uniformity Claim ---

UNICORE -- Unicore is defined as the combination of the seven paradigms uniform structure and function into the organizing "Unit" paradigm.  (Seven paradigms include: Data space, time, access, and grounding, plus compuation evaluation, control flow, and source-code packaging.)


UNICORE CLAIM OR THESES:

- UNICORE can be thought of as a "most generalized possible foreign function interface" used to express the relationship of and move data between other paradigms.
- UNICORE can serve as a kind of "semantic assembly language", meaning these nine paradigms can be mixed and matched to create "uniform" versions of langauges and frameworks we see today in software system.  
- But the result is different and much more powerful since the resulting paradigms will interoperate in ways the languages built today cannot interoperate.
- WHY?  Because these paradigms will share relevant computational substrate in ways that allow them to inform and iteract with each other.


Examples of this:
- Unicore supports an extremely very general formulation of the notion of "select" such that both SQL-like-select, JQuery-like-select, and RETE-match-like-select could be defined and implemented as special cases of this "select" notion at the "unit" level.
- Not only could those be defined, but the very high performance algorithms use to optomize those paradigms could ALSO be expressed in an equally genearl way.  
- This means that over any data type or implementation where "select" would be meaningful one could bring to bear all of that power and capability "for free" since it is already implemented in a way that is supported by those cases.  So a nifty rule-base, or parsimonious jQuery like selector might be used to indicate the current active elements in a choice box on a GUI, or to indicate which files to operate on, which shading transforms to apply in the inner loop in some GPU code, which elements to add up

- This formulation of object was specifically chosen with deep consideration of common optomizations for implementations on von Neuman machines, so that one is not precluded or limited in employing these techniques by the constraints adopted by unicore.  in particular:
	- The framing of slots, the two layer load-time/run-time, paradigm and the bang fixing of data groundings are all arragned to support type-erasure, generics, and C++ like templating for high performance.
	- The expression of "place" as an indicator of graph edge within synthetically generated graph structures is both flexible, but also admits to implementation flexibilty where one can arrange for graph places to align predictably with von Neuman memory layouts in ways that can dramtically improve execution performance.

- The notion of compute as a two-level data transformation allows on to express a very genearlized notion of "template" where code-macroexpand-like-templating, XSLT-like-structure-rewriting-templating, and 

- One might define classes of proxy objects that 
- Language reflection classes tend to be 



_

# ########## MARKDOWN #########
### ,
INTERVAL TREE -- A listy tree
=== SPIRAL 1 == UNIFORM MARKDOWN == THE FRAMING CONSTRUCT ===

TREE OF STRING INTERVALS  (this single construct provides the framework for all of Uniform Parsing)
- INPUT/OUTPUT --  Uniform parsing accepts a sequence of characters as input and returns this tree-of-intervals as its output.
- INTERVAL     --  Each node within the parse tree indicates an interval of the input text corresponding to this parse node.
- HEAD         --  Each node has a parse node type indicator drawn from a finite set of parse node types.
- CHILDREN     --  Each node has a list of child nodes.
- PARTITIONING --  Each node's interval is the concatenation of the intervals of its immediate child nodes.


# ###########################
# OLDER STUFF
# >>> THE STRUCTURAL ELEMENTS OF UNIFORM COMPUTATION <<<
## === UNIT  -  Uniform Structured Data ===
### _

**TL;DR. The UNIT paradigm models all data as a possibly directed graph with possibly ordered, possibly labelled edges.** 
	**UNIT.OPS provides an eight-operator API for mainipulating graphs of this graph data.**
	**UNIT.MODS provides an 12-attribute combinatorial grammar for constructing composite-semantic data units.**
	**UNIT.GRAPH provides a straightforward mathematical formulation of the graph underlying the Unit Paradigm.**

(See UNIT.GRAPH for the vocabulary used by the unit paradigm.)

_
### --- UNIT.GRAPH - UNIFORM GRAPH MODELS ---

**TL;DR.  UNIT.GRAPH provides a formal mathematical basis for the Unit data paradigm.**

_
#### -- UNIT.GRAPH - Introduction --

The key first key insight underlying the unit graph paradigm is that a great range of variation in different graph types can be reduced to three vertex-centric questions:
- Are the edges from a vertex ordered?
- Are the edges from a vertex labelled?
- Are the edges from a vertex uniquely indexed using their contents?
The combinatoral answers to these three questions leads to eight archetypal vertex-centric graph patterns:
	Bag, List, Set, Ordered Set, Relation, "Code", Map, and Ordered Map


The second insight is that the information content underlying these eight vertex patterns can be in encoded into a single data accessor, an edge iterator:
	ITR(vertex) ->  edge1, edge2, edge3, ...
The ordering of the edges may or may not have meaning, the edges may only be an object value vertex, or a pair of key-predicate vertex with a object value vertex, and the edges may or may not have unique labels or unique values.  But in all cases this single accessor captures all graph data from all eight varient graph types within a single operator call.


The third insight is the great range of specializations interpretations and usages of graph data can all fit on top of these eight vertex archetypes.  One class of specialization are simply restrictions on existing graph structure:  Tree, DAGS, undirected graphs, queues etc. are supported as simple restrictions on eight basic graph patterns.  The second class of specializations modify how the graph data functions.  Mutability, Versioning/History, Live embedding of external data, etc. are all supported as specially functioning verticies, but again all expressed within the structures supported by these eight graph structuring patterns.


~-~~-~~-~


Uniform graph defines EDGE-LIST as a single unform way to operate on these eight architypes.  
This uniform access model is what underlies the UNIT.OPS operators.  And these 8 archtypal graph types serve as a sufficient basis set for the 12-modifier grammar provided in UNIT.MODS.

The result is combinatorial factory for producing several thousand different graph types along with a single access pattern that allows code to be generalized without introduction of spurrious branching.

For example a pre-order tree traversal that sums the values in a of a tree that might be directed/undirected, labelled/unlabelled, ordered/unordered, indexable/non-indexable is:

def sum(x):
	s = 0
	for _, v in x.items():				# Wont unify if you treat maps and lists differently
		s += sum(v)
	return s							# Wont unify if you treat atoms and composites differently


As a second example using a relation to connect countries and languages spoken:
	[ <country1, lang1>, <country2, lang2> ]

def can_speak(lang_data, country, language):
	for c, lang in lang_data.items():
		if country==c and language==lang:
			return True
		else:
			return False

~-~-~

The many definitions provided here are mostly as one would expect given conventional usage of graphical models.  
There are a few subtle fiddly bits regarding how 'get' interacts with expected data structures, and how notions of place fit into the graph model, otherwise these are pretty standard defintions.

_
#### -- UNIT.GRAPH - Vocabulary --


**GRAPH / UNIT / EDGE / SUBJECT / OBJECT / VALUE** -- A _**graph**_, G=<U,E>, is a collection of verticies, U, (called _**units**_), and set of directed graph edges, E, where each edge connects a vertex (called the edge's _**subject**_) to a second vertex (called the edge's _**object**_ or _**value**_).

**VERTEX-CENTRIC / EDGES-OF** -- A _**unit-centric**_ organization of a graph is obtained by partitioning edges of the graph by their source unit, and then associating each edge partition with its unit vertex.  The set of edges of a partition is called the _**edges of**_ the unit vertex which is the source of each of the edges.

**ORDERED** -- A graph unit vertex is _**ordered**_ iff there is a total ordering defined over the edges of this unit vertex.

**LABELLED / PREDICATE / KEY** -- A graph edge is called _**labelled**_ iff it is associated with a "labelled with" an additional unit vertex.  This additional unit vertex is called the _**predicate**_ or _**key**_ of the edge.  A unit is called _**labelled**_ if ALL of its edges are labelled.

**EDGE / EDGE-LIST / PAIR** -- Since we have partitioned the graph by unit vertex, each _**edge**_ may be represented simply as just a value if it is unlabelled, or as a _**pair**_, a <key, value> combination if labelled.  All data regarding a single unit vertex in the graph may then be encoded as a _**edge list**_, a list of edges whose ordering is only meaningful in the case that its unit vertex is ordered.  

**ITEM / ITEM-LIST / GAP** -- An alternate edge notion is the _**item**_ for an edge.  It uses a special unit vertex GAP as the key for unlabelled edges, thus is denotes all edges as a <key, value> pair, and it encodes info regarding a unit vertex as an _**item list**_, a list of such items.

**INDEXABLE** -- A unit vertex is _**indexable**_ iff all of its edges are unique 
using just their key verticies or just their value verticies it can be distinguish it from all other edges having the same subject unit vertex.  A unit vertex is _**indexable**_ if all of its edges are indexed by their key or all of its edges are indexed by their value.

**INDEXABLE** -- A graph edge is called _**indexable**_ iff using just its key vertex or just its value vertex it can be distinguish it from all other edges having the same subject unit vertex.  A unit vertex is _**indexable**_ if all of its edges are indexed by their key or all of its edges are indexed by their value.

**VERTEX-CENTRIC GRAPH PATTERN** -- The cross product of: ordered x labelled x unique defines eight possible vertex-centric graph patterns: 
	-O-L-I = _**Bag**_,  -O-L+I = _**Set**_,         -O+L-I = _**Relation**_, -O+L+I = _**Map**_,
	+O-L-I = _**List**_, +O-L+I = _**Ordered Set**_, +O+L-I = _**Code**_,     +O+L+I = _**Ordered Map**_ 

**INDEX / BY-NAME / BY-POSITION** -- The _**index**_ for an edge is a vertex that may be used to select the edge from among edges sharing the same subject unit.
1. In the case that the edge <k, v> has a key, k, that is unique across edges of its subject and k != GAP then k is the index for this edge.  This is called _**indexing by name**_.
2. Else if the subject of the unit is ordered, then the natural number indicating the zero-indexed position of the edge in the item list is used as the index.  This is called _**indexing by position**_.
3. Otherwise the edge is not indexed.

**PLACE** -- A single edge within a graph is also called a _**place**_ within and graph.  The place operator accepts a unit, _u_, and index, _i_, and returns the edge indicated by this unit and index:  _p_ = _u_.**place**(_i_)

**GET** -- The 'get' operator accepts a unit, _u_, and index, _i_, and returns the "resulting" unit vertex, r, which is the object of the selected edge.  _r_ = _u_.**get**(_i_).  Notice: this get operator behaves as expected for lists, for maps and for  function-call expressions with both positional and keyword arguments.  Further it is well defined over chimera hybrids of these basic forms as well.

**VALUE** -- Each unit vertex, v, implicitly defines subgraph (call the _**value subgraph**_ or just _**value**_) as the transitive closure of the 'get' operator as applied to this starting vertex, v.

**UP / UP_IDX / UP LINKED / PLACED AT** -- A unit vertex may also define 'up' and 'up_idx' operators which are an kind of optional inverse for the 'get' operator.  In the case that u.up() = p is defined we say u is _**up linked**_.  In the case that: _u_.up() = _p_, _u_.up_idx() = _k_, and _p_.get(_k_) = _u_ we say that _u_ is _**placed at**_ _k_ within _p_.

**PATH / FOLLOW / PATH-FROM** -- A _**path**_ is a sequence of unit verticies, including the special UP indicator vertex.  The idea is that one may "follow" such a path by repeated applying the get or up operators as indicated by the path.
_**Path-From**_ is the inverse of follow.  It returns a path (typically the shortest) from one vertex to another.  Formally:
	_u_.follow(_p_) == _u_  if  _p_ == []   								// the empty path
	_u_.follow(_u_, [**UP**, _p_...]) == _u_.up().follow([_p_...])				  // path begins with 'UP'
	_u_.follow(_u_, [_k_, _p_...]) == _u_.get(_k_).follow([_p_...]) and _k_!=UP		// path begins with non 'UP' unit vertex

**CHILD / DESCENDENT-OF / ANCESTOR-OF / CONNECTED-TO / STRUCTURE-OF / VALUE-OF / DAG / TREE** --
Here we formally define a number of conventional graph terms.
Given a subject unit, _s_, a target unit, _t_, a key _k_, and subgraphs _G_ and _V_ then:
  t is a _**child of**_ s   	iff  there is a key, k where t==s.GET(k)
  t is a **_descendant of_** s	iff  t is a child of s or child of a descendant of s
  t is a _**ancestor of**_ s	iff  s is a descendant of t.
  t is **_connected to_** s		iff  t==s, t is an ancestor/descendant of s, or 
						   t is connected to an ancestor or descendant of s
  G is the _**structure of**_ s	iff  G is the graph of units, u, connected to s.
  V is the _**value of**_ s     iff  V is the transitive closure of 'get' applied to s
  V is the _**DAG**_ of s		iff  V is the value of s and V does not contain a cycle, that is:
	  There is no unit u in the value of s, and path p where:  u == u.follow(p)
  V is the _**tree**_ of s 	    iff  V is the value of s and each unit in V has a unique path, that is:
	  For all v in V, there does not paths p1 != p2 where s.follow(p1) == s.follow(p2)

_
### --- UNIT DISCUSSION ---
#### ...
**TL;DR. There are many many formats for structured data, it seems URF is the most Uniform among all of these.**

That TLDR sound big, beefy, and and a bit cheeky since engieneers have been representing structured data for a long time now, and now this URF-thing we have never heard of comes along is is some how "best" in some way.



has been around for a

_
#### -- Telescoping --

TELESCOPING -- A set of encoding recipes, R, are _telescoping_ if anytime R1 in R is more restrictive than R2 in R then instances of R1 are also instances of R2.

For example encoding both sets and lists as an enumeration of elements surrounded by square brackets forms a telescoping pair of encodings:  "[" ELE1 "," ELE2 "," ... "]".  How is that?  Well the set of all sets of some elements is a subset of the set of all lists of those elements since sets do not allow duplicates.  And we see according to our encoding every encoding of a set is also the precise encoding of some list.

We can contast this case of using RDF to encode a map from keys to values, verses an ORDERED map from keys to values


Ok, but why does this matter?

A key aspect of uniform is the framing of options componetially so that each option operates consistently across the powerset of combinations of all option combinations.  But becomes a collossal problem as soon as we start introducting non-telescoping restrictions in our recipes.  Code for ALL OTHER options which applies to the against the more general encoding must now contain conditional checks for each kind of representation encodings these both 

Concretely if one wanted to perform most any operation over a map of key value pairs inside an RDF system with both ordered and unordered maps, one would need to condition one code to check which encoding recipe was  being used.  Damingly this is true, even in the case that the operation in question does not know about or care about the ordering information.

At first glance this seems a small loss.  But it is NOT.  Even adding just one non-telescoping pair to our set of encodings DOUBLES the number of cases that our code must account for.  Worst still these doublings interact with each other, so if one add two unrelated non-telecoping pairs within a single encoding unifverse it QUADRUPLES the cases to be handled.  The explosion of cases is EXPONENTIAL in non-telescoping pairs.  (See multiplicative cluster fork for discussion of the pain that awaits this path.)

Notice it is possible to make RDF recipes for unordered and ordered maps that are telescoping.  One just needs to reify all triples in both maps regardless of whether one is ordering the maps or not.  This works, but this is not the most natural or simple encoding of map data in RDF, and it is not the encoding that most use when using RDF.


In URF the most natural encodings for _all_ core data types used in SW engineering are telescoping.  Specifically:  Sets, Multisets, Lists, Unorderd Maps, Ordered Maps, Non-functional binary relations, DAGs, Trees, Labelled Graphs, Unlabelled Graphs, Directed graphs, Undirected graphs.

As well as labelled/unlabelled, directed/undirected version of DAGs, trees, etc.
And for all of these one has both finite and infinite or generated versions of each.

ALL OF THESE ARE TELESCOPING !!

That is, one can write code that operates over any one of the more than 100 combinatorial options without adding a single conditional for handling alterantive cases AND ones structures are nearly optimal for each given case.

The primary non-optimality is the existence of a superflous 'key' field in cases when a structure's elements are unlabelled.  Of course one can implement more space efficient list-like structures for these special cases, then one is left paying the relatively minor cost of unpacking each element into a pair where the key is not utlized.

From a generality perspective it seems more complex structured used in SW engineering are already composed of composites of this list of structures, rather than adding some other more complex lowest level building block.  (But I welcome counter examples to this claim.)  The only counter example I have found is multi-edge graphs.  These are generalizations of binary edge graphs which allow for nary edges.  These nary edge graphs are a clear genearlization of binary graphs, but these seems there is no way to express these multi-edge graphs in URF in a way that is telescoping.  One could generalize URF itself two be a sequence of nary edges, but this increases the complexity of nearly all representations in order to componentially include what seems to be a relatively obscure graphical model type.  Its just not worth it.





_
# >>> THE FUNCTIONAL ELEMENTS OF UNIFORM COMPUTATION <<<
### --- API ---

Gnd:
	extends: Exec, Lang  // Place



Lang:
	load: slot(| L Lang, p Patch, ->Lang):
		for path, form in p:
			with origin=path:
				meaning = L.lookup(form)
				loaded = meaning.load(form)
				origin.value = loaded
		return L

_
### --- IMPL DETAILS ---

_
#### Grounding

- The Root of the runtime lexspace is a grounding that is its own backing
- It children have it as their backing until the 'Py' grounding is added, it becomes the grounding for certain sub-trees in lexspace.
- 
_ 
# >>> LAL1  -  Uniform Language Authoring Language
### LAL1.TREES

file...
http...
https...

std.
_
### LAL1.BANG


ISSUE: BANG ARGS -- We args to specify how to combine to create universe, but this should have "handle" 4 it
Create Eval.Bang ???  or LAL.BANG


Universe:
	bang(spec, using=)

Bang:
	chain(bang_spec, ->Universe)
	block(Textual)


BangSpec
- parse
- split at semicolons
- eval each section into a spec to bang
- subsequent

BangChain
- If string 		then parse it
- If head==';' 	then turn into list
- For each part
	- If ident then 	eval in context of prior and bang result

BangBlock
- BangChain the header
- Extract, parse, and bang the body


-~- lang.uf ~-~

_
## minimal impl
- Code emdedder
- Lang launch
	- bang lang univ
	- bang script from other 

_
## PKG.OPS -- package operators
### _

Mappy Package Manipulation Ops
- extend --
- import -- 
- export --

Listy Package Manipulation Ops
- append
- extend
- include
- exclude
- before
- after

_
## LANG.SCHEMA -- Structure Templating

_
## LANG.EMBEDDING -- Language embedding
## LANG.NORMALIZATION -- Normalization
## FLAT.UCM -- Uniform Code Markdown (UCM) ===
### _

_**TL;DR. In Uniform everying is encoded as graph data.**_

#### Thesis -- The UNIT uniformly frames the notion of "Structured Data"

As part of Unicore the Unit paradigm serves as both an elemental and componential basis for Structure Data.

 in the context of Unicore 

Paradigms intended to encode 
## FLAT.SYNTAX -- UCM Syntactic Mappings
- match	=~
- get	x [ y ]
- set	x [ y ] = z
- place	& x
- value	* x

_
# MORE OLD STUFF

### ,
EDITS NEEDED:
	_[[fix definition of "structure of" sometime it refers to immediate structure, the transitive closure downward, or in all directions.]]_
	ADD THESE:  
		value == transitive closure from vertex.
		Note: we use the term 'value' interchangably to refer to a specific vertex or to subgraph of vertices and edges reachable from that vertex.
		graph data == refers collectively to the nodes and edges of graph models
  G is _**rooted at**_ 		iff  For all o, t, k
	  o.GET(k)=t implies t.UP==o and t.IDX==k

? G is _**rooted**_ 			iff  G is a back linked DAG
  o is _**the root of**_ G   	iff  G is rooted and o is the ancestor of every, t in G except itself.




NOTE: We intentionally avoid using the RDF triple terms subject, predicate, object for two reasons:  Unit/Key/Value will map onto other expected notation later, and because certain kinds of graphical models would require multiple RDF triples to encode a single graph model edge, this would introduce confusion.  We use the subject/predicate/object terms only when refering specifically to RDF triples.


**XXX FUNCTIONAL** -- A graph edge is called _functional_ iff its key is unique among all edges of its source unit.  A unit is called functional if all of its edges are functional.


_
### --- Unit Node Structuring Patterns ---

Uniform allows unit nodes with a mixture of labelled and unlabeled edges and edges that are indexed by position and those indexed by the key label for each edge.

These alternative graphical models are unified as follows:

1. SET OF NODES -- A graph, G, is a set of nodes, N0, N1, ...
2. COUNTABLE -- The graph is countable, but may be finite or countably infinite.
3. EDGE SET -- Each node has a set of directed edges originating there and terminating at some other node in the graph.
4. POSSIBLY ORDERED -- The edges originating from a node may or may not be ordered.
5. SOME EDGES LABELLED -- Some edges may be labelled with a second "key" graph while other nodes are not.
6. SOME EDGES ARE INDEXABLE -- Some edge may be uniquely specified given a source node and some 'key' info.

Desired special cases and unifying properties:
	UNIFORM DATA FORM:
	Any of these graph alternatives should be losslessly encodable in a single uniform data structure.
	Here is the uniform graph structure:
		Graph G = set( sequence_i(pair(k0, v0), pair(k1, v1), ...), sequence_i+1(...), ...)
	INDEXING KEYS
	In the case that 

Single unified data form:
	[(k0, v0), (k1, v1), ... ]

Desired properties:
 
_
### 
**ITEM LIST / GAP** -- The item list for a unit vertex is a list of key-value pairs encoding the graph information contained in the edges of this unit vertex.  
- In the case that the edges of a unit vertex are not ordered, then the specific ordering of this list has no meaning.  
- In the case that an edge is unlabelled the special key '**GAP**' is used to indicate this absence of a label.
- Both indexed and non-indexed edges are encoded in the same manner.


**KEY-PRESERVING KEY REDUCTION** -- A _**key-preserving functionalizing**_ transform will convert any graph edge <k, v> into <n, pair(k, v)> in the case that the edge's key, k, is not unique across all edges of the unit u, or the head of the value, v, is the special head string (a pair of two underscore characters '__')

## === -OLD  -  Uniform Code ===
### _

TL;DR. The code paradigm  provides a least commitment framing of a homoiconic ("Lispy") langauge.  

That is software language that builds its executable code from user-visible code-as-data structures which were "read" from a textual source file using a language independent "reader" parser.

_
### Thesis


_
### --- Code Overview ---

_
### --- API ---

Form:
	**extends**: Unit
	_text_: **slot**(Code, -> Text)
	_spec_: **slot**(Code, -> Spec)
	_exec_: **slot**(Code, -> Exec)

_Code_: matches(Text || Spec || Expr)	// ??? how to express type matching?
_Text_: **extends**: Str					// Code's textual form
_Spec_: **extends**: [Heady,Boundy,Tree]	// Code's homoiconic form
_Expr_: extends: Unit					// Code's executable form
	  extends: Tree(Expr)	 		  // ???? ISSUE -- which one?  CHOICE

Env:
	_parse_: **slot**(t: Text, ->Spec)
	_print_: **slot**(c: Spec||Exec, ->Text)
	_load_: **slot**(c: Code, [at: path])
	_dump_: **slot**(e: Expr, ->Spec)



// Implementation

Env:
	
	_parse_: **slot**(t: Text, ->Spec): 
		for f in forms: 
			if s:=f.spec(t): 
				return s
				
	_print_: **slot**(c: Spec||Exec, ->Text): 
		f = c.head
		return f.text(f.spec(c))
		
	_load_: **slot**(c: Code, [at: path]): 
		spec = self.parse(c) **if** t =~ Text **else** c.head.spec(c)
		loc = Ident('origin').follow(at or c.lex)
		return self.assign(loc, spec.head.exec(spec))

	_dump_: **slot**(e: Expr, ->Spec):
		f = e.form
		return f.text(f.spec(e))




ISSUE:  How to represent a persistent update of internal structure
ISSUE:  'assign' is not currently exposed, but is needed for 'load'
ISSUE:  from a security perspective it is better to have data (paramters) separate from code (control)
		seems this reality would apply recursively 
		==> Maybe the data is a distinct overlay on the control layer

Rolling it all up.  a unit has:  .head .form .back .lex .origin

HEAD - is constant and exists across and between execution environments
FORM - defines its behavior within a single enviornment
BACK - provides persistence within a single enviornment
LEX - is the location of the 


_
### _


**TLDR;  "Lang" is the template used to define mini-languages that are combined into ever bigger languages forming an ever more complete ecosystem of interoperable software frameworks and langauges.**


**LANG** is an _**essential framing**_ of "interpretable software langauge".
**LANG** _encodes the essence_ of "interpretable software langauge".

The _**purpose**_ of LANG is to provide a decomposable encoding of iterpretation that operates on statements optomized for: (1) human manipulation, (2) machine manipulation, and (3) execution.


**LANG** -- A _Uniform Language (LANG)_ is recursively formed from collections of "constructs", where each construct formalizes the interpretation of some defined type of "code" units.

**CONSTRUCT** -- A _construct_ defines three interchangable code formats:
	**TEXT** -- Text is code expressed as a String.
	**SPEC** -- Spec is code expressed as a tree of Unit. 
	**FORM** -- Form is code expressed in executable (reducable) form.

**EXE**		-- Inteprets a code form based on some interpretation environment.
**ENV**		-- Env is the local state (environment) of an Interpretation.
**BANG**	-- Creates a new interpretation "environment" 
**REDUCE** -- Advances an Env state by interpreting some code form.
**"+="**	 -- Combines forms with code formats.
**LANG0** -- A simplest language instance used to bootstrap more complex languages.


(see uc.lang.generality for discussion of the generality of this framing)
_

GUARANTEE:  Forms never disown's its own code.  Specfically, for all spec s
c.head.exec(c).form = c.head{[