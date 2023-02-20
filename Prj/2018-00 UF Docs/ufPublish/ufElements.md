
 [Elements Powerpoint](Elements.pptx) 

 
#### _

interfaces/frameworks and languages built from these nine Unicore elements are automatically interoperable with each other, and with all over all concepts covered by Unicore.

~-~~

Muddled -> Knew Place (didn’t fit) -> Below VonNeumann Machine below all computation.
   Flip it!  Define mutable-place as extension of immutable-place
 Align it! Immutible-place == graph edge



~-~~-~
**Mutation was MUDDLED!**
This “bad smell” persisted for several years; it was like pushing a bubble under the carpet, could not get all mutation elegant unified and elegant.

**"Place" was known, but didn't fit**
I knew the simple theory I wanted:  a “Place” is a thing that properly implements “Access” and “Assign”
PROBLEM:  Cannot define “Place” in the bottom rung (#1)
It’s not granular to put mutation below functional computation

**PROBLEMS**
PROBLEM: Thus Rung #1 computation cannot have mutation or places.
SOLN:  Use rewrite system for compute … nothing “changes”, just rewrites are created.

PROBLEM: Thus Rung #1 data cannot cannot be defined as mutation.
SOLN:  Flip it!  Define Place in terms of Edge, as an edge in a graph whose target vertex is the value stored.
PROBLEM:  all values are denoted by a single graph vertex (we made that so)SOLN: all vertices denoted the transitive closure of all “by-value” links from a vertex 
PROBLEM:  all Places now will have an extra source vertex as well.  (not granular)SOLN: The Unit-handles assumption requires everything has a handle.


PROBLEM: Fundamental get/set composite object operators don’t match.
SOLN:  They are not fundamental.  They are a combine movement in both space and time. E.g.   Get(source, key) = Access( Index(Source, Key) )    Set(source, key, value) == Assign( Index(Source, Key), Value)

PROBLEM: What about mutating the key for a graph edge?  That is not a “place”.
SOLN:  Keys never change! -- all units have all keys most values are just ‘void’ so keys never change, just their value

PROBLEM: What about the ordering of info for edges?  Not a place.
SOLN:  Edge ordering == assignment history.   Semantically it is equivalent to a hypothetical sequencing of place assignments 

PROBLEM: Mutation occurs in file writes and iterator updates
SOLN: All things that change over time are the history of some (possibly synthetic) place.
THUS:  A generator is a history that has not happened yet.
THUS: An iterator is the play back of some history
THUS: Computation is a generator
THUS: An ordered composite is the history of assignments to all places indexed by its source vertex.
THUS: An open file handle is just a history


Finally!  Functional programming is Not defined as change to place, but all mutation DOES now stem from the obvious Place/Access/Assign element






Mutation
(A real-world example in developing Unicore)         
Many elements had a muddled relationship with ideas of                    access, mutation, and persistence
This “bad smell” persisted for several years;it was like pushing a bubble under the carpet.

Its resolution required several simultaneous reformulations:
Defined “Place” as kind of graph edge that implements persistence.	(explicitly deriving rung 2 persistence from rung 1 graphs)
Formulated all “mutation” using the assignment a on place.	(implies rung 1 cannot have mutation)
Split original notion of “access” into the - SPACE ELEMENT “traversal” to get TO a place, and- PLACE ELEMENT “retrieval” to get value OUT of that place
Refactored streams, iterators, etc. to be expressed as a movable place alias (so they can reuse common mutate, etc.) 	(since “change” must be tied to mutation of a “place”.) 
Allowing “ethereal” data graphs    (allows change to exist even when not tied to an explicit data graph)




_
### --- UNIFORM DAG OVERVIEW ---


**UNIFORM ELEMENTS OF COMPUTATION**
Here are the first three rungs of the Uniform DAG as presently understood:

**MATH** 		-- _RUNG 1 -- Uniform Mathematics_ -- Defines some pre-requisite concepts
  **GRAPH**		-- Directed graphs whose verticies may have edges that are:
			   (1) ordered, (2) labelled, (3) and/or functional
  **NUMBER**	-- Natural or Decimal numbers with expected associated operators
  **REWRITE**	-- Term Rewriting Systems defined using graph structures

**STRUCTURE**	-- _RUNG 2 -- Uniform Structure_ -- Paradigms for encoding data
  **ACCESS**	-- Vertex-centric graph operations
  **SPACE**		-- Graph-based notions of space, location, path
  **PLACE**		-- Model of state, change, persistence and assignment
  **FLAT**		-- Flattening of graph data into a linear forms (including 'printing' of data) 
  
**FUNCTION**	-- _RUNG 3 -- Uniform Function_ -- Paradigms that formulate computation
  **GND** 		-- Paradigm for 'grounding' an executable embodiment onto its static spec
  **EVAL**		-- Uniform interpretation
  **FLOW**		-- Control flow (block,branch,repeat,chain,exit)
  **PKG**		-- Source code grouping and combining
  **UNIT**		-- Unifies structural and functional paradigms into a mix and match object model


**THESIS STATEMENT** -- In the the paradigms defined below we sometime identify a "thesis statement".  Each thesis is a claim of generality in the spirit of the Church-Turing thesis.  Each formulates the generality we hope the paradigm has or will have attained.  Some thesis statements claim that the paradigm is introducing a "covering" constraint.  This is a claim that one can adopt this paradigm and still 'cover' all use cases of interest to humanity w/o serious loss in utilty of the resulting solution space.  (See XXX for deeper discussion of this idea.)  _{[maybe this notion of thesis statement is too subtle/sloppy to explain so briefly here?]}_

# ### RUNG 1 -- DEFINITIONAL -- UNIFORM "TURING MACHINE" ###
## === Math Rung Overview ===
### ,

_**TL;DR. Uniform MATH is a small collection of "mathematical" concepts underlying Uniform.  All of these concepts have been understood and accepted by humanity for at least a century.**_

- **GRAPH** -- A _**uniform graph**_ is a countable collection of vertices, each with a countable collection of edges.
- **NUMBER** -- A _**uniform number**_ is a possibly bounded integer or decimal with: +, -, *, /  defined as expected.
- **REWRITE** -- A _**uniform rewrite**_ is a term rewriting system (TRS) where rules and terms are expressed as graphs as defined above.

_
## === MATH.GRAPH  -  A formulation of a family of graphical models ===

**GRAPH** -- A _**uniform graph**_ is a countable collection of vertices, each with a countable collection of edges.  Each edge has a source or subject vertex and a destination or child vertex.  Further there are a countable subset of the vertices are "constant verticies" -- these verticies are unambiguiously aligned across all uniform graphs that contain them.

_
##### _

**TL;DR.   _Uniform Graphical Models_ are graphs with possibly-ordered, possibly-labelled, possibly-directed edges.**


**GRAPH** -- A graph, G=<U,E>, is a collection of verticies (_units_), U, and set of edges, E, where each edge connects a vertex (called called edge _subject_) with a vertex (called the edge _object_).

**ORDERED** -- A graph vertex is _ordered_ iff there is a total ordering defined over its edges.

**LABELLED** -- A graph vertex is _labelled_ iff its edges also have a "label"---an additional vertex (called the _predicate_ for the edge).

INDEXED -- 

		

					     No Key				With Key
    Indexed			Set / OrderedSet	Map / OrderedMap			
    Un-indexed		Bag / List			MultiSet / Code


-KEY -ORD	MultiSet	Set
-KEY +ORD	List		OrderedSet
+KEY -ORD	Map			Bijection
+KEY +ORD	OrderedMap	OrderedBijection

	_
## === MATH.NUMBER  -  Integers, decimals with several expected operators ===

**NUMBER** -- A _**uniform number**_ is either a _**natural**_ number or a _**decimal**_ number, and either has either a _**fix precision**_ or _**varying precision**_.

**NUMERIC OPERATORS** -- The following expected _**numeric operators**_ are defined over all four kinds of number:
	+, -, *, /, %, <, = 


There is nothing unexpected or surprising about the Number paradigm.  Thus we have yet to provide a formal specification, but when it is done, it will be just as the reader expects.

_
## === MATH.REWRITE  -  A term rewriting system built entirely using graph data ===

**REWRITE** -- A uniform _**rewrite system**_ is a "vanilla" term rewrite system where both the terms and rules are expressed as graph structures in a fashion that allows both to be treated uniformly as all other graph data.


We have yet to document this paradigm, but there is nothing surprising here.  This is a vanilla TRS except that both rules at terms are express as graph structures.
_

# ### RUNG 2 -- STRUCTURAL -- UNIFORM REPRESENTATION ###
## === Uniform Structure Rung Overview ===

_**TL;DR. "Uniform Data" combines the four constraints below to guide the encoding-of and operating-upon all data. We argue these these constraints are so flexible and general, all existing computation can easily be fit into this frame.**_

- **DATA.ACCESS** -- provides a unifying constraint for organizing all all data related operations:
	All data-access is performed in a vertex-centric fashion -- i.e. all data operations accept a vertex as first arg.
	- DATA.HANDLES -- 
	- DATA.STRUCTURE --
	- DATA.MODIFIERS -- 
- **DATA.SPACE** -- formulates "space" as the transitive edge traversal within a graph structure.
- **DATA.TIME** -- formulates "time" as a timeline whose spatial structure represents the temporal structure of a "place" -- a simplest model of data persistence.
- **DATA.GROUNDING** -- separates the realization of data from the specification of that data.
	i.e. all graph data comes from a "factory" which itself is a data access operation executed over other graph data that exists prior to the realization (creation) of the grounded data.

_
## === STRUCTURE.DATA  -  How Uniform operates on graph data ===
### _

Graph.Vertex.OPS
Graph.Edge.OPS
Graph.Vertex.MODS

TL;DR. The ACCESS paradigm provides a uniform model for all data operations within the uniform ecosystem.  It is composed of three organizing sub-paradigms: Handles, Ops, and Mods.

**STRUCTURE.ACCESS** -- The _**access**_ paradigm is union of the _**handles**_, _**ops**_, and _**mods**_ paradigms.

ACCESS := ACCESS.HANDLES + ACCESS.OPS + ACCESS.MODS

_
### --- DATA.HANDLES - A kind of object orientation ---

_**TL;DR. The HANDLES paradigm indicates that all data operations are vertex centric -- this is a kind of object-orientation where a graph vertex is always the first argument for all data operations.**_


**STRUCTURE.HANDLES** -- The _**uniform handles**_ paradigm restricts all data operations to be graph-vertex-centric operations.  

Meaning:
- Effectively this means that all uniform graph operations accept a single graph vertex as their "first argument".
- Typically this means that all processing indicated by these operations proceed locally from this single vertex.
- Later when the range of operations are extend beyond structural operations into other semantic operations that have nothing to do with the graph structure itself, those operations are will still 'handled' in the sense that they still take a first argument which is some graph vertex.  In that case the vertex is taken to be a handle for some bundle of semantically cohesive set of operations.
- This paradigm is very roughly commiting to a kind of object-oriented style of programming, albeit only in a limited sense (e.g. we do not assume inheritance, objects state, or other parts of OO)  We are only commited to the universal existence of handles.

COVERAGE ARGUMENT:
- **SOME LANGS VIOLATE THIS** -- Some languages do not universally use handles, they have functional operations which are not tied to any particular handle.  The how can we claim the is a universal "covering" constraint?
- **HOW IS IT COVERING?** -- We argue that these languages DO have handles, they just leave the handle implicit.  Further we argue it does not have notable cost to make this handle explicit, and we argue there are benefits to adopting this discipline.
- **THE EXECUTION ENVIRONMENT AS A HANDLE** -- As a backstop the entire execution enviornment can be viewed as one big handle for such languages.  (even this degenerate case will have value in meta programming.)

_{[Really alot more needs to be said to justify this claim of coverage here.]}_



_
### --- DATA.OPS - The semantic basis for graph data ---

_**TL;DR. The STRUCTURE.OPS paradigm defines graph data semantics in terms of six graph operators.  Later paradigms may only define graph operators in terms of this six.**_

**STRUCTURE.OPS** -- The _**uniform structural operators (OPS)**_ constraint defines "graph structure" to be all information about a graph that can be accessed or manipulated via the following six operators: IDX, ITR, LEN, MET, NEW, DEL.

**SIX STRUCTURAL OPERATORS** -- The six accessors here use the graph.access paradigm to provide access to and manipulation of uniform graph data.  Semantically ALL graph access and ALL graph manipulation can be expressed in terms of these six operators:
- **IDX** -- _index_ -- obtains substructure by "index" whatever that means for this vertex
- **ITR** -- _iterate_ -- iterates over the edges of a vertex
- **LEN** -- _length_ -- returns the "length" the number of edges a vertex has
- **MET** -- _meta_ -- returns a cooresponding vertex with in separate graph of "meta" (non-structural data) associated with the vertex
- **NEW** -- _create_ -- creates a "new" vertex inheriting certain information from the current vertex
- **DEL** -- _detsroy_ -- destroys a vertex so it no longer exists as part of the graph structure


NOTE: For performance, elegance, and other reasons later Uniform rungs define a larger set of "convenience" graph accessors, but all of those accessors are semantically defined in terms of these six operators.  One effectively 'creates' new graph structure, by defining the behavior of these six operators over collections of new vertices.

NOTE: We claim this is a covering constraint, that is one can use this graph.access principle to orgnaize all computation w/o introducing forks where useful computation lies outside this framing.  (On this face of it, this seems incorrect since many paradigms allow for "search" and other operations which are non-local graph operations.)  Uniform handles these cases by defining meta graphs where these serach operations turn out to fit this local graph.access pattern within that meta graph.  A much longer discussion as to why this is an acceptable (beneficial constraint to place upon all of computing)

NOTE: Both the 'index' and 'iterate' operators return a Place.  Thus this paradigm uses the Places model of persistence to access and modify a single edge within the graph.


**GRAPH.ACCESS** -- The _**uniform graph access**_ paradigm restricts all operations over graphs to be vertex centric operations.  

**GRAPH.OPS** -- The _**uniform graph ops**_ paradigm is union of the _**graph access**_ paradigm and the _**graph structure**_ paradigm.

_
#### --- GRAPH.OPS - VERTEX CENTRIC OPERATORS ---
##### UNIT.OPS - Overview

	UNIT.OPS provides an eight-operator API for access and mainipulation of the underlying graphical model.
	The semantics for all uniform data operations are defined in terms of these eight.


	EDGE OPERATORS
	- GET(k)		-- Accesses a unit's structure (one of its edges) by key
	- SET(k, v)	-- Modifies unit structure by assigning a value to a key
	- ITR()		-- Iterates thru the unit's structure (its edges)
	- LEN()		-- Returns the number of outgoing edges a unit has, or INF if infinite

	NODE OPERATORS
	- GND()					-- Returns the grounding unit for this unit
	- MET([k], [v])			-- Returns or assigns to the unit's meta keys
	- NEW([spec], meta:...)	-- Creates a new graph unit
	- DEL()					-- Deletes a unit from the graph

	_
##### UNIT.OPS - API

	Unit:
		**extends**: Inst
		met: **slot**(**op**(-> Unit)) 
		ops: **slot**(**op**(-> Unit)) 
		new: **slot**(**op**([structure: Unit], meta..., -> Unit)) 	-- Creates a new unit
		del: **slot**(**op**(-> Void))
		#
		head: **slot**(**op**(-> Unit)) 


	Composite:
		**extends**: Unit
		init: {K, V}
		# Dunder getitem, setitem, iter, len
		get: **slot**(**op**(key, [altspec], -> Unit))			-- Accesses the substructure of a unit
		set: **slot**(**op**(key, value, -> Unit)) 				-- Changes the substructure of a unit
		itr: **slot**(**op**(-> Stream)) 
		len: **slot**(**op**(-> Int||Inf)) 

		# Operators with derived semantics
		__ iter __: **slot**(**op**(-> Stream))				// Listy values
		items: **slot**(**op**(-> List[(K, V)]))
		values: **slot**(**op**(-> List[V]))
		has: **slot**(**op**(value, -> Bool)) 	


	Gnd:
		args: {HANDLE, KEY, VALUE, *, UNIT: Unit(KEY, VALUE)}			
		gmet: **slot**(**op**(u: UNIT, -> Unit)) 
		gnew: **slot**(**op**(u: UNIT, [structure: Unit], meta..., -> Unit)) 
		gdel: **slot**(**op**(u: UNIT, -> Void)) 
		ghead: **slot**(**op**(u: UNIT, -> Unit))
		#
		unwrap_inst: **slot**(**op**(u: UNIT, -> HANDLE))
		wrap_inst: **slot**(**op**(h: HANDLE, -> UNIT))
		unwrap_gnd: **slot**(**op**(->Gnd))					//  Gnd accepting handles as first arg not Unit
		wrap_gnd: **slot**(**op**(->Gnd))					//  Gnd accepting wrapped versions of handles
		child_gnd: **slot**(**op**(->Gnd))					//  Gnd for all children (might be Unit)
		unwrap_gnd_unwrap_inst: **slot**(**op**(->Gnd))		//  Unwraped gnd returning unwrapped handles


	GndComposite:
		**extends**: Gnd
		gget: **slot**(**op**(u: UNIT, key, [Altspec], -> Unit))				
		gset: **slot**(**op**(u: UNIT, key, value, -> Unit))					
		gitr: **slot**(**op**(u: UNIT, -> Stream)) 
		glen: **slot**(**op**(u: UNIT, -> Int||Inf)) 


	_
##### _

	{[Out of place.  Maybe this goes in LAL1?]}


	META KEYS & METHODS (FUNCTION RELATED)
	- head	-- 	Location reported as the type of this unit
	- root	-- 	Location used as the root of semantics lookup
	- seed	-- 	Location of the structure factory used to build this unit
	META KEYS & METHODS (STRUCTURE RELATED)
	- lex	-- 	The location of the structural data contained within this unit
	- up		--	The structural "parent" of this unit within lexspace
	- index	--	The structural "down" key used in the parent to obtain this unit
	- id		-- 	Integer that uniquely identifies this unit at a given moment within a given universe

	_
##### -- get --

	**_get_**(key: Unit, [altspec], Unit<->Unit)

	Structural unit access.  Uses key to "get" the subunit structure functionally mapped from the unit by the key, or altspec if no such key exists.  Get performs a _**set**_ operation when it is assigned.




	_
##### -- set --

	**_set_**(_key_: Unit, _value_: Unit, Unit<->Unit)

	Updates the structure of a unit.  If the unit is persistent the original structure is unchanged, instead a copy of the unit with the updated structure is returned.  If the unit is not persistent then the original unit is directly modified, and that same unit is returned.

	CHOICE:  Where does altspec go????

	_
##### -- itr --

	**_itr_**(_key_=Place, _meta_=Bool, Unit->Stream)
	items(Unit->Stream)
	next(Stream -> (key, value))
	peek(Stream -> (key, value))

	_
###### - 
##### -- len --

	**_len_**(Unit -> Int||Void)

		- Returns VOID if a unit is an ATOM
		- Returns an Int or INF if a unit is composite
		- If unit is a @lst then 
	_
##### -- ops / met --

	**OPS** -- The _**operators operator (OPS)**_ provides structural access to an unit's functional operators.  Specifically:
		U.x = U.ops()['x']  for all operators x.


	**MET** -- The _meta operator (MET)_ provides access to a unit's meta data. 

	**META DATA** -- A unit's _meta data_ is extra (non-structural information) about a unit.  It can be expressed as a second "meta unit" -- a boundy-identy-mappy-bagy unit of meta data about the underlying unit.  This meta data is expressly unconnected to all the other other structural aspects of the underlying unit as exposed by all other VERT operators.


	API FOR THE MET OPERATOR
	- unit.MET(->Unit)				// Returns all meta keys as a unit
	- unit.MET(key, ->value: Unit)	// Returns the value of a single meta key
	- unit.MET(key, value, ->Unit)	// Assigns the value of a meta key and returns modified unit

	- The ops unit will have a 'structure' operator the renders the original unit's structure

	_
###### -- META DATA ENCODINGS --

	Most unit encodings are optomized for rendering of a unit's structure without consideration for its meta data.  There are two basic methods for adding meta data: adding meta keys to the front of a unit, or creating a class of keys that are distinguishable from standard data keys, then injecting meta key/value pairs into the unit's iterator using these distinguishable keys.

	OPTION #1

	**META HEADDED UNIT** -- A unit whose the first listy element is a unit with 'META' as its head, and whose k/v pairs contain the unit's meta data, including its head value which is conventionally listed as the first key value pair within the meta form.


	OPTION #2 -- INLINE META DATA ENCODING -- The meta keys are injected into the list of k/v pairs for a unit

	OPTION #2A -- **META KEYS** -- A _meta key_ is a unit whose head is 'META' and whose first position element is a string containing a valid identifier, and whose second element is an optional recursive encoding counter.

	OPTION #2B -- **META LIST ENTRIES** -- A meta list entry is a string beginning with a .  List form uses prefix encoding character ':' as prefix for structural k,v pairs.  In a similar way the the '^' character is used as a prefix for meta key/value pairs.  So the unit [' foo', ':a', 111, '^b', 222] as one structural key 'a' with value 111, and two meta keys, 'head'='foo' and 'b'=222.

	OPTION #2C -- **META PREFIX PARSE TOKEN** -- within a unit form a bare key beginning with '^' indicates a meta key while a string with a '^' prefix indicates a structral key beginning with ^.  So:
	foo(^one=111, '^two'=222)  has one conventional structural key '^222' and two meta keys: 'head'='foo' and 'two'=222/



	_corner case note_: In the extrodinary corner case of a meta key being use as a conventional key, this can be encoded by using a meta key with a recursive encoding counter set as 1 or a 1 + the passed meta key.  In this way we can distinguish a true meta key from a conventional k/v pair with a meta key structural value.



	, with the head meta key conventionally injected as the first k/v pair and remaining meta keys injected next for an unbounded unit, or 

	and since they


	_
##### -- new --

	_**new**_([_unit_structure_], _unit_meta_keys_..., Unit->Unit)

	The operator creates a new unit.
	- It looks up and calls the factory associated with the 'new' method on the 'self' unit
	- It passes the structural and meta keys to the factory for creation
	- Meta keys 'root' and 'head' define the semantics for the unit
	- Uses 'unit_structure' as a template for the new language typically by performing a shallow copy of its contents into the newly created unit.
	- May or may not allow for the setting a head for the unit (some types have a fixed head)
	- May or may not allow the setting of the root of the unit (some factories only produce one type of unit)

	- ??init??  May or may not allow changing or bypassing the initialization (can you subclass and )


	_
###### - new discussion -

	- high performance units will separate head from meta
	- some may not allow meta field setting
	- either head is fixed, or is an internal field
	- specialty map proxy for separated-head meta-dict can provide access to full map. UnitImpl.
	_
##### -- del --

	_
### --- DATA.MODS - Vertex-centric graph specializatins ---
#### _

_**TL;DR.  The ACCESS.MODS paradigm expresses graph constraints and properties (e.g. tree or ordered set) as an extensible vocabulary of mixable "vertex modifiers"**_

**ACCESS.MODS** -- A _**uniform graph modifier (graph.mod)**_ is a property of a graph vertex that constrains or modifies the nature of the edges and child verticies of the indicated vertex.

- As much as possible these modifiers have a componential semantics, meaning that one can apply combinations of these modifiers to each vertex in such a way that the meaning of a modifier in no way depends upon which other modifiers it is combined with.  
- COMPONENTIALITY EXCEPTIONS:  The primary exceptions restricting the full powerset of graph.mods stems from logical inconsitencies among certain combinations of modifiers would entail.  For example there cannot be a non-DAG tree vertex, since all trees ARE DAGs according to the definition of tree and DAG.
- Some modifiers are transitive meaning they can only be applied to a vertex in the case that they also apply to all child vertices of that vertex.  For example a vertex can only be a tree vertex in the case that all of its child verticies are also tree verticies.

_
#### --- LIST OF VERTEX MODIFIERS ---

Here we list the vertex modifiers that are part of unicore.  This set is intended to be open such that later spirals will include further vertex modifiers.  This is allowed without restriction as long as the set of graph mods remains mostly componential.

In OO terms these modifiers are 'traits' such that the _power set_ of combinations are mostly all instantiatable, meaningful, and useful:




**VERTEX MODIFIERS**
- **Ordy	  Ordered**		--  The edges for a vertex have a well defined stable order
- **Keyed  Labelled** 	--  Edges have a vertex (called a _predicate_ or _key_) labelling them  (KEYED)
- **Mapy	  Functional**	--  Vertex is labelled, and no two edges have the same key vertex
- **Dagy	  DAG**			--  A unit is the root of a DAG if its space is asyclic
- **Taxy	  Tree**		--  A unit is the root of a tree iff there is exactly one 
						   path to each of its descendents.
- **Fixy	  Static**		--  A unit is static if its edges can never change
- **Finy	  Finite**		--	A unit is finite if it has a bounded number of edges
- **Boundy Bounded** 	--  A unit is bounded if all children are bounded and it is finite
- **Undy	  Symmetric** 	--  A unit is symmetric if a[b]=c implies c[b]=a
- **Live	  Live**		--	A unit is live if its values are persisted via some external source outside of the unit
- **Lazy	  Lazy**		--  A unit is lazy if its structure is only derived upon access to that structure.
- **Heady  Head Settable**	--  A unit is head settable if 

		Or, La, Un, Da/Tr/Gr, Fr/Pe/Mu, Fin, Sym, Uppy, Historic, Head, 
		8 * 9 * 32

	**ADJECTIVE NAMINGS**:
		Ordy, Keyed, Mapy, Dagy, Taxy, Fixy, Finy, Boundy, Lively

	These alternate names are expressed as short adjectives allowing one to parsimoniously name elements in the powerset of combination types:  For example, one can say the "Source Code" data type is fixy-boundy-ordy-mapy data.

	**NEGATION NAMINGS**:
	Vertices may also be labelled at NOT satifying some combination of these 11 properties using:

	- Ordered	Ordy		Unordered	Bagy
	- Keyed		Keyed		Unkeyed		Unkeyed		
	- Functional	Mappy		Relational	??
	- ??			Listy		MultiSet	??
	- DAG		Dagy 		Circular	Graphy
	- Tree		Taxy								Also called Taxonomic
	- Static		Fixy		Mutable		Muty
	- Finite		Finy		Infinite	Infy
	- Bounded	Boundy		Unbounded	Unboundy
	- Symmetric	Simy		Asymmetric
	- Live		Lively		Persistent?

	_
### --- API ---

-~- lal -~-
package UC.DATA

args:									# Data Element's type parameters
	OPERATORS: List[Op]					# Vertex operators
	MODIFIERS:	List[VertexModifier]	# Vertex modifiers
	EDGE_MODIFIERS: List[EdgeModifier]	# Edge modifiers
	ATOMIC_TYPES: List[Class]			# List of scalar types

OPERATORS: package::
	keys: 
	IDX: slot(op(key Vertex, -> Edge))
	ITR: slot(op(-> Stream[Edge])			# from UC.TIME
	LEN: slot(op(-> Integer))
	GND: slot(op(-> Gnd))
	MET: slot(op(-> List[VertexModifier]))
	DEL: slot(op(-> Boolean))

MODIFIERS:
	Ordered: class(extends: VertexModifier)

EDGE_MODIFIERS:

ATOMIC_TYPES:
	Num:	class(size: slot(op(-> Nat)))		# From SEMANTICS Rung
	Int:	class(size: slot(op(-> Nat)))		# From SEMANTICS Rung
	Nat:	type(base: Int, where >=0)			# the natural numbers
	Bool:	type(where =='true || =='false)		# the boolean values	
	Str:	class()								# From UC.FLAT


Vertex: class(
	extends: OPERATORS
)
		
Edge: class(
	extends: EDGE_MODIFIERS
	unit: 	slot(op(->Vertex))
	key:	slot(op(->Optional Vertex))
	value:	slot(op(->Vertex))
)
	
VertexModifier: class(
	extends: Code
)
	
EdgeModifier: class(
	extends: Code
)

Unit: class(
	extends: Vertex
)

			

	_VertexMods: type::
		::= Ordy  || Keyed || Mapy
		::= Dagy  || Taxy  || Fixy   || Finy   || Boundy  || Undy  || Live  || Heady  || Listy
	
-~-


uf.package(".Vertex.ops")
_
## === STRUCTURE.SPACE 	-  Uniform Spatial Location ===
### --- SPACE Overview ---

_**TL;DR.  The SPACE paradigm defines familiar notions of location, space, path, and addressing entirely in terms of graph structure.**_

**LEXSPACE** -- The Lexspace graph is an infinite, static, tree-structured graph object, where
each vertex has an infinite collection of children which coorespond to each natural number and each finite string value.  Each vertex has a single parent vertex, sometimes referred to as "UP".

NOTICE:  Lexspace is infinite in all directions (even up).  Lexspace is isotropic, that is, the local structure around each vertex it isomorphic to the local structure around all other verticies within lexspace (the links back DOWN to each isotropic vertex may be different, in that sense their local address is not isotropic).


**KEY TERMS**
- **SPACE** -- A _**space**_ is the set of verticies reachable via repeated edge traversals relative to some designated origin vertex.
- ORIGIN -- 
- LOCATION -- A vertex within some 'space' graph.
- PATH -- A finite list of graph-edge indices.  Each index is used to select the next edge to follow when performing the graph traversal indicated by the path.
- CANONICAL PATH -- 

- **LEXSPACE** -- The _**Lexspace**_ graph is an infinite, static, tree-structured graph object upon which notions of _space_ and _location_ built.
- **SPACE/SPATIAL** -- In uniform, saying that a relation or collection of entities are _**"spatial"**_ means that they can somehow be systematically mapped onto the Lexspace object.  All uniform notions of space and location derive from mapping to the Lexspace object.
- **LEX/LOCATION** -- Each vertex in the lexspace graph represents a distinct "location" also refered to as a _**lex**_.
- **PATH** -- A path is a finite sequence of vertex traversals within lexspace.  It is a list of keys indicating a unique traversal one level in lexspace, or a negative integer indicating a number of upward traversal steps within the Lexspace graph.
- **CANONICAL PATH** -- Each pair of source and target lex locations defines a unique canonical _**path**_ as the shortest sequence of graph traversals from the source to target within lexspace.
- **ORIGIN** -- The origin of lexspace is a specially designated lex vertex within the space.  (This may be any vertex in the graph, and other views of the same graph may be obtained by simply selecting a different origin.)
- **ADDRESS** -- The address of each lex vertex is simply the canonical path from the origin to that target lex.  There is a one-to-one mapping between all possible canonical paths and all possible lex in lexspace.
_
### --- API ---


-~- lal -~-
package UC.SPACE


Lexspace:
	args: 	  [LEX=Lex, IDX=Class, PREFIX]
	_init_: 	**slot**(**op**(Spec))
	_pathify_:	**slot**(**fn**(Spec, ->Path))
	_str_: 		**slot**(**fn**(Spec, ->Str))
	
	_UP_:		**slot**(**fn**(->IDX))		
	_Path_: 	**type**(List[IDX||UP])
	_Spec_:		**type**(Path || LEX || Str)

Identifier: Lexspace(LEX=Lex, IDX= Str||Int, PREFIX='I.')

I: Identifier('')						  // The origin for the Identifiers space
	
Unit:
	_lex_: **slot**(**op**(<->Lex))					// CHOICE: Should really be called place or ident
	_path_: **slot**(**op**([source], ->Path))
	_follow_: **slot**(Path||Ident, ->Unit)

Lex:
	**extends**: Unit
	_lexspace_: **slot**(**op**(-> Lexspace))
	
-~-
_
### --- SEMANTICS ---

LEXSPACE INVARIENTS
- STATIC 
- TYPES -- Each is of type LEX
- CHILDREN -- and has has a child lex for every k in KEYS, and
- PARENTS -- each lex's up is another lex
- ISOTROPY --   

CREATION
- WHOLE SPACE -- Whenever a lex is created the infinite lexspace that contains it is created at the same time.
- CONTENTS -- 
- CREATION -- Creating a lex with no arguments will create a new lexspace, with an arg it will "wire" in that parent


LEX METHODS
- PATH(dest)			Lex||Path||Str	==> Path
- PATH_STR(dest)		Lex||Path||Str	==> Str
- FOLLOW(dest)		Lex||Path||Str	==> Lex		(uses lex.path() to get path of lex input)  ???


_
### --- DETAILS ---
#### -- String Path Notation --

IDENT PRINT FORM -- An Ident path is a list composed of strings, ints, and the special UP key.  These are converted into String Path Form as follows.
- The list of keys are each converted into a string, and those strings are concatenated with a period between each.
- Each key is converted into a string as follows:
	- Integers are simply converted into their textual form.
	- The special UP key is converted into the string "UP"
	- Strings composed of alphanumerics and underscores (that are not the string "UP") are not changed
	- Other strings are surrounded with single quotes (') with the (') backquoted (\) internally.

_
### --- IMPL CHOICES ---
- CHOOSE: Lexspace may have hetrogeneous types.  (need different operators) thus it cannot be their grounding
- CHOOSE: A lex may not relocate since it would break pointers.  
- CHOOSE: A lex should be weakly referenced by parent
- CHOOSE: Each lex points to lexspace?  needed when converting to string (for the prefix), 
			can be found by chaining up in the unusual case.


- CHOICE: does a Lex expose its structure?  Yes. but Ident does not.
- IDEA: Lexspace:  Lexspace(x)   .path_to(x)   .str_to(x)   .chroot(x)  

TO DO
- CHOICE: Does L.env get the current env?  (Needed inside __repr__)
- CHOICE: How do we access pathify and access origin?
- CHOICE: should Idents have a get method?  a follow method??   (maybe on the lexspace?)


~-~~
origin.path_to(x)    as_path(x),   pathify
origin.str_to(x)     as_string(x), stringify, str
origin.follow(x)
lex.str()
lex.path()


~-~~
I(_anything_) = ident    #  I.as_lex  I.follow
I.as_path(_anything_) = path
I.as_str(_anything_) = path

~-~~
 ‘str’ vs ’repr’
MOVE pathify/str into Lex.  (inherently treats current lex as origin)
need to move UP, LEX, KEY, PREFIX
Change any reference to Identspace to L.Identspace
Rename L.Identspace to L.IdentOrigin
???  Maybe we don’t need a ‘lexspace’ method.  It would mean all meta info UP, LEX, KEY, PREFIX would need to be on lex



CLASSES
- Lexspace(prefix) - Returns a lexspace instance
- Lex(x) - Returns a lex instance

_
#### ,
// OLDER 
-~- lal -~-
package uc.data.space

LexspaceTemplate:
	**extends**: Template
	_inst_: **slot**(**op**(?contents, _prefix_: Str, _lex_gnd_: Gnd, -> Lexspace))

Lexspace:
	**extends**: Composite
	_get_: **slot**(**op**(Int, -> List[Lex]))  	  // The zipper of lexspace
	_origin_: **slot**(<->Lex)
	
Lex:
	**extends**: Composite, Taxy
	_lexspace_: **slot**(-> Lexspace)
	// get/itr/len/up/up_idx				// Provides infinite lexspace as unit strcuture

Path:
	**alias**: List[Int||Str]				

Loc:
	**alias**:	Str || Ident || Path
	
Ident:
	**extends**: Lex
	_init_: **slot**(**op**(Path||Str||Ident))
	_pathify_: **slot**(**fn**(Path||Str||Ident, ->Path))
	_str_: **slot**(**fn**(Path||Str||Ident, ->Str))
	_path_: **slot**(**op**(->Path))

Unit:
	_lex_: **slot**(<->Lex)					// CHOICE: Should really be called place or ident
	_path_: **slot**([source], ->Path)
	_follow_: **slot**(Path||Ident, ->Unit)


_
## === STRUCTURE.TIME  -  Uniform Time and Persistence ===
### --- TIME and PLACE Overview ---

**_TL;DR.  The PLACE paradigm formulates the widely used notions of persistance, timeline, state and change using the very familiar place/value/access/assign operations over graph structures_**


**PLACE** -- A _**place**_ is thing that provides persistence---it accepts newly assigned values, and provides access to the states that result from those assigned values according to expected assignment semantics.


The goal of this paradigm is to provide a one-size-fits-all model of time and persistance that can serve as a uniform basis for the semantics that all other paradigms can build on top of.

This paradigm is not new, still its systematic use in all cases involving state, persistance, and change would go along way in cleaning up and homonizing many software frameworks and langauges.  There are many desirable properties that are often lost by sloppy / partial implementations of space and time:  e.g. objects only exist in one place at one time, and no two objects can be in the same place at the same time, places are unique.  mappings between structures are always well defined, all space and time is uniquely addressable, etc.)  These are properties that often not incompatible with language features, but just out of sloppiness they are not fully maintained.  Also different paradigms often use slightly incompatible versions of space and time, leading to subtle and needlessly complex mappings between them.



_{[overview here is a bit wordy]}_
In many ways the place paradigm is totally obvious since it encodes the very commonly used notion of assignment.  At the same time, parts of it can be a bit subtle and confusing since it is formalizing the slippery notion of time, as a requirement for encoding assignment semantics.  The place paradigm defines assignment semantics, but in order to do this we need to define time and change as well.  Like everything else in uniform we hope to build from the prior paradigms.  In this case we define temporal structure as a particular interpretation of spatial graph structure.  The only parts of the assignment model that are modestly surprising is that we allow for assignment operations that update state by combinging prior state with assigned value. In this case, the very common assignment semantics becomes a special-case update function that ignores prior state and uses the assigned value as the new state.  The second slightly surprising aspect is that we allow for distinct timelines in distinct place/locations.  The time in each location flows monotonically but is not directly mappable to the flow of time in other places.  The third and final generalization is admittely a bit unexpected.  We organize places into an infinite lexspace and then treat assignment of a structure onto those places as an alignment of each of the vertices in that value into the sub-places of the master place.  Thus we have a kind of automatic value-destructuring by default when doing assignment to places as the align to lexspace.  It is helpful to think of the space of places as an infinite piece of paper, and that access and assignment of values is the reading and writing of infinte sub-trees on this paper.  But that one wierdness not withstanding, keep in mind all the stuff below is just formalizing the stuff about persistence and assignement that you already know.

Clip on those jet packs... we are launching!


**PLACE** -- A _**place**_ is thing that provides persistence.  It accepts new assigned values, and provides access to the states that result from those assigned values according to expected assignment semantics.

	**ASSIGN** -- The _**assign**_ operator for a place accepts a value to be assigned.

	**ACCESS** -- The _**access**_ operator for a place returns the current state of that place.


**PLACE SPACE** -- _**Place Space**_ is a Lexspace of Place.  Each lex serves as a place where a unit value might be persisted.
	**AT** -- The at operator returns the place, p, associated with a unit, and which symmetrically is assigned back to that unit.  Formally, a unit, u, is _**in**_ place, p, iff u.at == p and p.value == u.

	**PLACEMEMENT** -- The _**placement**_ operation causes a provided unit, u, to be "at" provided place, p.  (Operation can be written as:  u.at = p)

	**VALUE** -- A value is a data graph vertex (any vertex that is NOT a place).  Each value has an implied extent which is the transitive closure of verticies that are "at" cooresponding subplaces where their parents are "at".  Formally:
	implied_extent(root) is the small set of verticies where
		root in implied_extent(root), and 
		child in implied_extent(root) if 
			parent in implied_extent(root) and parent[idx] == child and child.at.parent == parent.at
	
	**APEX** -- The _**apex**_ of place space is the lowest lexspace zipper place element such that all places with non-void values are descendents of this apex place.

	OLD -- The _**placement**_ operation is a combination of assignment and localization, it causes a given data vertex to be located at a place which in turn has that data unit as its value.


The elements of place space are related structurally such that assignment of a structure, S1, to place P1 recursively causes all decendents of P1 to be assigned corresponding structural values from P1.  Formally assignment of S1 to P1 causes every, descendent, Pd, to be assigned the structural sub element, Sd, where Sd = S1.follow(Pd.path(from=P1)) or VOID if no such Sd exists.  The structures being assigned can be finite or infinite is size.  In either case, these assignments execute in constant time, and are typically implemented via a single pointer update on many implementations.  Assigments are atomic meaning that they introduce bi-directional not-after links within the time relation connecting the timelines for all places under the root of the assignment itself.


**LOCALIZATION** -- The dual of assignment is _**localization**_ of a value vertex to a given place.  With assignment one can query a place it get the value assigned.  With localization one can query a value and the place where it is localized.

	**LOCATE** -- The _**locate**_ operation is the dual of assignment operation, it associates a specific lex as the "location" of a given data unit.


**PATCH** -- A _**patch**_ is a list of path-value pairs.  It defines a set of assignments or placements, in the case that some of these paths are prefixes of other paths, then they are assigned first, allowing the longer paths the "shadow" their assigned values in sub places.

	**PATCH** -- 

	**DELTA** -- 



**PLACE SPACE** -- _**Place Space**_ is an infinite mathematical object -- it is a lexspace where each lex is a place.  (In Uniform all data exists somewhere in place space)

	A given place space at a given moment in time is an env

	**Env** -- The environment of a place is the whole lexspace of places each at a given moment in time.
	

	**PLACE SPACE ASSIGNMENT** --

	**TIME** -- In uniform _**time**_ is a partial ordering defined over the set of place-value-assignments across all moments with the timelines of place space.  The uniform time relation only requires that this partial ordering is totally ordered when considering the subset value-place-assignments involving a single lex place.

	**UNIVERSE** -- A _**uniform universe**_ is a "timeline of env" -- it is a sequence of env where not only are the value-place assignments consitent with the time relation within each env, but is also consistent with the time relation between successive env.

		**SPLICE** -- The _**splice**_ operation edits the place fabric of the universe.  Splice is used to install different place update functions into different places within the universe.  Because these splice operations time and the assignment semantics connecting different parts of lexspace the splice operation is typically executed only during the creation of a universe, not during its execution.



**FORK** -- The uniform _**fork**_ operation forks one environment into two environments an "outter env" which is unchanged by the fork operation, and an "inner env" which has a set of assignments "patched" onto its timelines.  After the forking operation both the inner and outer environments continue maintaining their respective timelines as independent unrelated timelines.  (NOTE: Typically the inner universe will only extend out to some finite time index, while the outer universe continues onward, but this is not a requirement of the with operation.)

	**WITH** -- Environment forking is accomplished by the _**with**_ operation.  The with operation accepts two arguments, a specific place within place space (the "application root"), and a patch of values to apply.  It then:
	1. Forks the entire Lexspace of places of the Env that this application place is part of
	2. Creates a partial ordering of the paths with the path, ordering path p1 before p2 if p1 is a prefix of p2.
	3. For each path-value pair in the patch it:
		- follows the path from the appliacation root place to some place (that maybe be above or below) that place
		- then assigns the associated value to the place resulting from the follow.
		(The time relation over the assignment moments created by this patching of the new inner universe, must respect the partial ordering obtained in step 2 between the paths, e.g. shorters paths were assigned first, so longer paths could "shadow" parts the assigned values.)

	IMPLEMENTATION NOTES:  A fork is potentially an extremely heavy operation, yet there are many cases where we will want to use them within the inner loops of a computation.  Fortunately there are several common special case simifications that are available for use in particular language contexts:
	- SHALLOW BINDINGS -- In the case that nothing is going to happen with the outter env during the lifetime of the inner env, we can use shallow bindings to simply keep track of the values that need to get "put back" once the environment is close.  For this to work properly place space needs to be read-only and we need to know that any references to places within the inner env that are retained, will simply be pointing at the corresponding places in the outer environment once the inner one terminates.
	- READ ONLY STRUCTURES WITH SELECTIVE DUPLICATION -- In the case that place space is read-only, one can create forks of the space and can share parts of those spaces which have the same substructure (since neither fork can modify those structures)
	- PERSISTENT STRUCTURES -- Finally one can use persistent data structures to encode place space.  This allows us to fork those structures and have two handles that are both subsequently modifiable w/o concern of conflict becuase of the operation of the persistent structures.


**STREAM** -- A _**stream**_ is a temporal manifestation of some spatial structure.  It is a mapping between space and time.  The stream itself is a place that can be accessed and assigned values which come from and become spatial structure.

	**ITR** -- The _**iterate (itr)**_ operation can accept any "source unit" vertex within the data graph and it creates and returns an "iterator" over the child edges of that unit vertex.  

	- The returned iterator is a stream place which can be access or assiged values just as any place can be. 
	- At any given time this stream place is "at" an edge of the source unit.   
	- Accessing and assigning the stream place cooresponds to getting or setting of the key/value vertices of the cooresponding spatial edge.
	- A stream may be implicitly advanced upon access ("read") or assignment ("write"), or it may require explicit trigger for advancement.


**TIMELINE** -- A _**timeline**_ is a list of "state" values derived from the iterative application of some "update" function.  So a timeline is just a list of values -- a list of values fit this model:

	**MOMENT / POINT IN TIME** -- A _**moment (point in time)**_ is a single element of this timeline list.

	**TIME INDEX** -- The _**time index**_ for a moment is the natural number that indexes that moment within the timeline list.

	**STATE** -- A _**state**_ can be any graph value.  A moment on the timeline list may have some state value.  Ituitively this means that this value was "being persisted" at that moment in time.

	**UPDATE FUNCTION** -- A _**place update function**_ is any binary function accepting two graph values, a "prior state" value and an "assigned" value, and it returns a new state value.  The simplest update model just discards the prior state entirely and returns the assigned state as the new state for the next moment, but timeline can support more complex update functions.  
	
	**VOID** -- Void is a special value vertex that indicates "no value"  (This is kind of like "null" or "None" but most languages don't treat their null value properly to serve as void, so we use the distinct term "void")

	**PERSISTS** / **PROVIDES PERSISTENCE** -- We say a timeline with a defined update function _**provides persistence (persists)**_ a sequence of assigned values iff its state values, timeline[i], obey the recurrance relation indicated by the place update function, namely that:
		timeline[0] = VOID
		timeline[i] = place_update_function( timeline[i-1], value_to_be_assigned[i] )   if i > 0


	**TIME-AT** -- Each place, _p_, must keep track of a natural number, _n_, a time index into its timeline.  We refer to this number n by saying the _**time-at**_ _p_ is _n_.  This index is required by both access and assign in order to keep track of which value is being assigned or accessed.  (Notice:  This time index number is required to formalize the recurrance relation being persisted by the place, an actual implementation of a place need not actually store this number, it is only required to obey the place's update function.)

	**ADVANCEMENT** -- The time-at a place is _**advanced**_ by incrementing its time index number by one.  Typically this is automatically done on assignment.  (Those certain places separate time advancement and assignment)

_
### Place Modifiers

- **Watchy Watchable**	  --  A unit is watchable if one can obtain 
- **Histy  Historical** 	  --  A unit is historical if some prior versions are realizable (reconstructable)
- **Provy  Providential**  --  A unit is providence tracking (providential) if one can generate explanation 
							 structures how its current value was derived (the computation performed).
_
### Breakin down all that fancy-pants talk

The definitions above are short-sweet and abstract as heck -- but intuiting all of the consequences of these definitions is not at all obvious, even though what is being modelled here is quite intuitive and each to understand.  Here are some observations about the place paradigm that aim to make all of this a bit clearer, a bit more explicit, and a bit simpler:
- **ASSIGNMENT** causes a "call by reference", "sim-link-like" connection to be created from that place to the value stored in the place being referenced.
- But does not MOVE the object to this place, and it does not make a COPY of the original object.
- Surprisingly perhaps, the assignment operation is recursive.  It recursively it causes each sub-place to be a sub-reference to the corresponding sub-place within the source place.
- **PLACEMENT** does more performing a simple assignment.  It clears the infinite tree of places under a give location (replaces them with references to VOID), and then MOVES each unit of data over to the corresponding place based on their arrangment within the original space.
- Reference links within the original structure are preserved and link to the coorresponding positions in the new structure.
- Reference links going OUTSIDE the origininating structure are left intact still pointing to the same places they pointed before the move.
- Even though each place space is infinite, if the data contains is finite, n, units then only that n units are moved, the rest of the space are merely reflected references from these actual nodes.
- Placement "MOVES" to data to a new location -- meaning those units no longer exists in the old place -- those units are replace by VOID in that place, or by whatever would have been in that place had these units not been placed in that place.  (see layering)
- Notice that placement is different than "call by value" since with call by value the value is COPIED and exists in both places.
- Notice that placement paradigm is more like our intuitive notion of object-place from the physical world:
	e.g. objects can only be in one place at one time, and two objects can't be in the same place.
- FAST -- Even though placement and assignment sound hairy and complex, they are not.  They are both implmenented as fast operations that do not depend upon the size of the data being moved.
- this translates to fast-constant total update time for "normal" variable places, 
- but does incur overhead if there are "watched sub places" within the affected substructure.
- **SEMNANTICALLY INSTANTANEOUS** -- Semantically both assignment and placement are instataneous.
- REAL VS ECHO


- Unlike many models of place in place space all objects "really are in one spot" 
- INFINITE ECHOS OF FINITE DATA --
- Circular data causes infinite echoes within place space, but still only a finite amount of data only a finite number of 
- BUT ALSO CAN HAVE INFINITE VALUES TOO -- 
- values can be finite even if references "echos" of them to extend infinitely far down each place subspace.
- but values CAN truly be infinite containing an unbounded number of unit verticies none of which are references of any others, but are their own original copy.
- Even these values may be assigned or placed in fast constant time.
- 
- Even though each sub-placespace is infinite assigning a value with a finite number of 

TIME
- HAPPENINGS -- Everything that "happens" in unicore boils down to the creation of a new a value-place assignment in the time graph.
- TEMPORAL CONNECTIONS -- With the exception of "live" data (not yet disucssed), everything that happen in uniform happens for a reason.  e.g. it happens becuase of the program being run.  But the specific happening that occur (the specfic values that are assigned) depends upon other value place assignments (other happenings).  This gives us our temporal connections.  If v1-p1 is used in computing another assignment v2-p2 then their is a temporal link from v1-p1 to v2-p2.  what does this mean?
	
- TOTALLY ORDERED AT EACH SPOT -- this just means that a each single place in place space the assignments form a single sequence.  X=5; X=23; x=[1,2,3]; x='hello bob'
- SEPERATE PLACES HAVE SEPARATE TIME LINES
	- TOTALLY SEPARATE -- If two computations are unrelated then the assignments will be totally unrelated.  So if one program assigning x the sequential values of the fibonacci sequence while another program is assigning the sequence of prime numbers to y, there will be no connection in the global time relation between values of x and values of y.  They are just running open loop without synchronization.
	- ARROW OF TIME CONNECTED -- - TIME FLOWS FORWARD -- if x=5 is used in the calculation of y=10, then we know what ever order happen, that somehow the assignment of x=5 could never have happned AFTER the assignment of y=10 since it was used in the calculation.
	- 
	- SUB SPACE CONNECTED -- 
	- INSTANTANEOUS INFINITE PROPAGATION

ENVIRONMENT


_
### API

-~- lal -~-
package UC.TIME

Place:
	**extends**: UC.Edge
	_value_: **slot**(**op**(->Unit),				// The access operation
				**op**(<-Unit))			  	  // The assign operation
	_history_: **slot**(**op**(->Unit))				// Get this place's value history

Stream:
	**extends**: Place
	_key_: **slot**(**op**(<-> Unit))
	_seek_: **slot**(**op**(->), **op**(Int, ->))
	
LexPlace:
	**extends**: [Place, Lex]
	_fork_: **slot**(**op**(Patch, ->LexPlace)		// The fork operation
			   **op**(->LexPlace))			  // Fork pop operation	

Patch:
	**extends**: Path => Unit

Composite:
	_patch_: **slot**(**op**(Patch->Composite),	  	// Patch of tree values
				**op**(->Patch))
		
Unit(KEY, VAL):
	_lex_: **slot**(<->Lex)						// The localization access and assignment ops
	_itr_: **slot**(->Stream)					// The iterate operation
	_delta_: **slot**(op(Unit->Patch))			// Express difference between units as a patch
	



_
### Place Discussion
#### Why make recursive value assignment the default?
_
### --- PLACE.WITH ---
#### _

**TL;DR.  x**


**WITH** -- The _**with**_ statement "shadows" a defined set of values on some "outter" package(s) to create a contextualized "inner" version of those package(s) that exists temporarily, during the interpretation of the body statements of the with itself.  The with statement can occur as part of a "code" structure to be evaluated, in that case it temporarily alters the contexts of the outter package(s) during evaluation of its body.  Alternatively it can occur within the declaration structure for a set of packages.  In that case it redefines the contents of some package as seen by any packages declared within the body of the with statement.



CONTEXTULIZATION -- _**Contextualization**_ is the specification and creation of some "situation" (context) relative to some other situation.

The context / situation term is understood very broadly to mean the configuration or setup of thing, assumptions made, approaches used, parameters set etc.

UNIFORM CONTEXTUALIZATION -- In uniform the _**CTX**_ operator uses a contextualization "spec" to derive some "inner" context from a given "outer" context.  The inner and outer contexts as well as the spec are all encoded as a "patch".  A _**patch**_ is sparse mapping of particular "root" locations in lexspace onto data values "layered" onto those places in lexspace.

**CONTEXT** -- A context is another name for a patch.  We use the term patch when emphasizing the data object itself, and use "context" when emphasizing the semantic situation resulting from the application of that patch.

**CTX** -- The _**CTX**_ operator derives an inner context patch as a function of an outer context patch and a contextualization patch.


FORMALIZATION

_value(ident, context)_ 
= context[longest].follow(ident.path(from=longest))
	where longest is the ident in context where it is a prefix of ident
= **void**  if no such prefix exists.

CTX(spec, outer)   =  ident->value  where 
- ident is an ident from either spec or outer and
- value is the value associated with ident in spec, or in outer if it does not occur within the spec mapping

_
#### DISCUSSION
##### Use of 'WITH' in declaration structures

KEY IDEAS
- NO ADDED INDENT -- Dont want to have a 'with' form that "increases indent" since this is a semantic namespace.
- MULTIPLE CONTEXTS ARE NEEDED -- Having "global vars" with different values is needed.
- MONKEY PATCHING NEEDED -- Further, one needs to be able to change most anything within certain sub-contexts, not just changes to things that spatially live within that (or any) specific sub-space.
- INNER AND OUTER SIMULTANEOUSLY NEEDED -- In some cases one will have class C1 that requires the outer context, and class C2 that requires the inner class, both active at the same time.


**Solution #1** -- anything that may be changed via a 'with' must be declared as such within the outer context.  
- PROVIDING BOUNDED VARIATION -- Thus 'with' is unlike the operation of creating the base space itself, but instead is providing variation within the envelope bounded by creation.
- SAME AS CLASS INSTANCE CREATION -- this approach reuses the class instance creation mechanism for support of context variation.
- MECHANISM -- 
	- SLOTS would be placed in lexspace with default value formulas
	- value of such variables would always be accessed from 'env'
		- they could be part of the stack frame
		- part of the 'self' objects
		- part of the lexical scope of the code itself
			- can be constants within that scope, or a reference to a slot in the scope
			- ???

**Solution #1a** -- Variation within a context is only dependent upon the bindings of immediate children slots in parent tree, but not in descendents of parents.  
- Thus mod-counters can be used within the space tree to track change.
	- new instances can be shaped according to current class spec
	- changes in slots could leave old instances on old tree
	- Could 'rake' instances into new form


**Solution #2** -- contents of lexspace itself is a nested progression of 'with' statements.
- ALLOWS ANYTHING TO CHANGE -- Does not place any limits on what can be changed.
- SAME AS SPACE CREATION -- This with statement reuses space creation.  
- NEEDS TO PROVIDE WAY FOR SPACES TO SHARE SUBPARTS
- SHARING IS MESSY -- Seems hard to share anything except a read-only prehistory before the fork, since either side can change anything with would be unclear which future edits should bleed across the fork
_
## === STRUCTURE.FLAT  -  How uniform map graph structures onto linear "print-like" forms
### --- Flattening Overview ---

_**TL;DR. The flattening paradigm describes bidirectional lossless mappings from finite graph sub-structures onto strings and other linear forms (e.g. abstract pickel or print).**_



Graphical Models are quite flexible, this is a virtue when building upon it, but is a source of complexity and challenge when trying to support them in some way within another system.  The flattening (FLAT) paradigm provides several kinds of simplification operations which can be mixed and matched in order make different kinds of simplification in the data aimed at common difficulties associated with processing and transport of fully general graph data.

The flat paradigm includes "unit-form" -- a specific graph flattening combination chosen as a simple concrete flattening used as a simple print form for Uniform graph data.  The details of Unit Form have also been chosen in a way that fits smoothly into Uniform Code Markdown as a tiny subset of that source code paradigm.


**FLATTENING** -- A _**flattening**_ data transform is an operator having an inverse operator, and whose output is guaranteed satisfy some property that its input did not satisfy.  Formally a graph operation, f, is a flattening operation iff there is some f' inverse, and property P such that:
	For all finite graph values v    
		v == f'(f(x))			-- all graph values can be mapped, and all mapped values losslessly map back
		P[ f(v) ]				-- The flattened version of each value v will satisfy the simplification property


**LOSSLESS / EIGHT-BIT-CLEAN** -- A transform having an inverse is call _**lossless**_ or _**eight-bit-clean**_ if ALL possible input values "cleanly" map back to their original values without exception.  This is the first condition shown in the definition of flattening above.  The definition harkens back to early data communication protocols which often had special codes used in the transport itself which could cause certain very special data values to NOT transmit accurately (they were not 8-bit clean)


**MANGLING -- ITEM FORM REDUCTIONS** -- An _**item-form reduction**_ is a graph transform that losslessly converts edges whose keys do not satisfy some indicated "valid key" predicate into graph structures where all edge keys DO satisfy this indicated valid-key predicate.  (Many embedding contexts limit keys to be valid software identifiers or other constraints.)

**META EMBEDDING** -- A _**meta-embed**_ transform accepts two graphs: a structure graph and a meta graph, and then losslessly "embeds" the meta graph into the structure graph yielding a combination graph which can be used to recontruct the original two graphs.

**CYCLE BREAKING** -- A _**cycle breaking**_ transform converts cyclic graph structure losslessly into an acyclic version.  The cycle-breaking included in unicore breaks cycles by replacing some graph edges with terminal "reference pointers" thus turning a cyclic structure into a tree structure.

**TEXTUAL FLATTENING** -- A _**textual flattening**_ operator losslessly converts a tree-structured graph into a string -- a sequence of character symbols drawn from some fixed alphabet.

**UNIT FORM** -- The _**unit form**_ flattening is a specific lossless mapping from from graph data onto character strings.  Unit form includes a form of: (1) cycle breaking, (2) key-preserving functionalizing, (3) meta-embedding, and (4) textual flattening.

_
### --- FLAT.MANGLE - Mangling ---
### --- FLAT.EMBED - Unicore Meta Embedding ---

Unicore meta-embedding transform accepts input data which include specialty edge labels and head values used to indicate different forms of meta data, and losslessly transforms this input into a form that does not utilize these edge indicators.

INTUITION: 
- Meta-embedding simply prepends a special "meta edge" to the iterators of all nodes that have associated meta data or already begin with such a meta edge.
- Thus in all case the original graph structure can be recovered by simply pulling the first element off of any unit iterators beginning with such a meta edge.


**THE DETAILS**

Given:
	_**structure**_ -- a set of unit nodes each expressed as a sequence of <k, v> pairs where both k and v 
				 are other units within the structure or are atomic units.
	_**meta**_ -- a second graph over the same nodes providing "meta" information about those nodes.
	valid head -- a predicate assessing the validity of
	_**reserved**_ -- a set of head values strings that are "reserved" -- in unicore this set is:
                 {"M", "R", "__", "___"}
Output:
	A unified "merge" graph that losslessly encodes both input graphs and avoids the use of the reserved head values.

Steps:

1. Identify all units, u, that either have at least one k/v pair in the in the meta graph OR, have a meta key head value



 into a merged output form 

_
### --- FLAT.TREEIFY - Cycle Breaking ---


_
### --- Item Form Reductions ---


**ITEM FORM REDUCTIONS** -- An _**item-form reduction**_ is a graph transform that losslessly converts edges whose keys do not satisfy some indicated "valid key" predicate into graph structures where all edge keys DO satisfy this indicated valid-key predicate.  (Many embedding contexts limit keys to be valid software identifiers or other constraints.)




**RAW ITEM FORM** -- The initial representation of a graph edge is call raw item form, and it is:
	<k, v>		in the case that an edge is labelled.  k is the labelling key, and v is the edge's object value
	<GAP, v>	is the alternative used if the edge has no label.


These raw items can be converted into edges whose keys adhere to any specific binary predicate, valid_key(k), accepting the key's name and return true if this key name is permitted.  The valid_key predicate may be any binary predicate.  The resulting item's keys will either satisfy this predicate or will be a replaced by a natural number (these are the keys used as a backup when the initial key value is not acceptable.)


**POSITION-BASED ITEM FORM** -- The _**position-based item form**_ simply transforms every raw item into a position-index pair.  the result is verbose, but is also quite regular.  So each item <k, v> at position n is tranformed into:
    <n, __(k, v)>		// The double underscore head indicates it contains a key-value pair


**KEY-PRESERVING ITEM FORM** -- The _**key-preserving item form**_ is a more complex transformation, but it attempts to preserve key values when possible while simultaneously mirroring list-like edge indexing for unlabelled edges.

Given an item <k, v> at position n, its key-preserving item form is:
	<k, v>			if valid_key(k) and k is not already mapped to another <k, v>
	<n, __(k, v)>	otherwise

_
#### -- FLAT.ITEM_FORM_REDUCTION - Choices --

Bedrock? Constraints:
- dict(x) == dict(code(x))  for all x
- list(x) == list(code(x))  for all x
- for all python functiona calls, fn_call = fn(a0, a1, ..., k1: v1, k2: v2...)
	dict(code(fn_call)) == {0: a0, 1: a1, ..., k1: v1, k2: v2, ...}  

##### CHOICE: counting method
ISSUE: The bedrock constraints are underconstrained and there are conflicting preference beyond this.
CHOICE #1:  When the key for the nth item is invalid it is replace with 'n'
CHOICE #2:  When the key for the nth item is invalid it is replace with the first unused natual number

Pro #1: COMBO-MAP -- If other no raw keys are natural numbers then this mapping can be extended to map ALL items both by key and by position within a single map.

Pro #1: SIMPLE MAPPING -- The mapping from items to reduced keys is simple in one way.  Each item either has its original key or n as it reduced key.

Pro #1: ISOLATED ANALYSIS -- It is possible to perform a limited analysis of just the one item plus a single membership check for that key k on the whole map to determine the reduced form of a single item.

Pro #1: STATELESS -- dont need to maintain a counter for last assigned number

Pro #1: MERGABLE -- given two reduced item lists checking for and resolving conllisions between their mapping lists is sufficient their mapping dicts for collisions is 

Con #1: NON AGGREGATIVE -- Even if just one item is appended to a unit it could remap ALL existing item reductions in the worst case.  I think... yes... here:

	[(1,1), (2,2), (3,3), ... (1000, 1000)]
	then add (1000, 1000) again in the 1000th position.  this will switch every prior item in the unit.

##### CHOICE: What to do with finite composite keys?  What about infinite keys?

_
### --- Unit Form Structural Format ---

- Keys are string values or natural numbers
- Head is an ident

_
### --- Unit Form Textual Format ---

UNIT_FORM  ::=	HEAD "(" META "," BODY ")"  |  HEAD "(" META ")"  |  HEAD "(" BODY ")" |  HEAD "()"
HEAD	   ::=	IDENTIFIER 
META	   ::=  "M(" BODY ")"  |  "M()"
BODY	   ::=	PAIR "," BODY  |  PAIR
PAIR	   ::=  VALUE   |  IDENTIFIER "=" VALUE
VALUE	   ::=  STRING  |  NUMBER  |  UNIT_FORM
IDENTIFIER ::=   a valid identifier



The unit form can be used to restrict the structure of the flattened form to avoid forms that will not "fit" into different embedding contexts.  By default unit form ensures the flattened form will have valid itentifiers for the head and keys (not beginning with a digit, and componed only of lower and upper case letter, digits, and underscore).      
    Further, unit form itself uses the four heads "M" for meta, "R" for reference, "__" for a pair, and "___" to indicate a non-compliant head.  So each of these are also treated as non-compliant head.


{[Add detailed description of each transformation]}


_
### --- FLAT API ---

ISSUE: better names.
ISSUE: do all of these methods get exposed
ISSUE: how do these methods itegrate with a unicore lang?


Flat:
	parse: **slot**(str, ->Unit)
	print: **slot**(u: Unit, ->str)
	
	unit_form: **slot**(u, ->str)
	add_meta: **slot**(u, ->Unit)
	position_based_functional_simplification: **slot**(u, ->Unit)
	key_preserving_functional_simplification: **slot**(u, ->Unit)
	break_cycles: **slot**(u, ->Tree)
	tree_to_string: **slot**(t: Tree, ->str)


_

# ### RUNG 3 -- UNIFORM COMPUTATION (COMP) ###
## === Uniform Function Rung Overview ===

**TL;DR. The Uniform Function paradigm is the union of the Grounding, Evaluation, Control Flow, Source-Code Packaging, and Unit-Object paradigms.  Together these define uniform computation and are built from the Uniform Data paradigm.**

- **COMP.GND** -- The grounding paradigm uses the factory pattern to link created instances having dynamic semantics back to the static data used to specify those semantics.
- **COMP.EVAL** -- formulates the interpretation of "code" as the recursive "grounding" of code-data onto "interpreter" elements capable of transforming code-data into "interpretable data".  Interpretable data in turn is can be executed with the 'exe' operator.
- **COMP.FLOW** -- The flow paradigm expresses control flow as a recursive structure of the five control flow operators:  block, branch, repeat, chain, and return
- **COMP.PKG** -- The package paradigm encodes source-code packaging semantics -- the definging of the contents of each LEXical code location as a static (human interpretable) function of the source forms structurally related to that location.
- **COMP.UNIT** -- The unit paradigm pulls the uniform structural and functional paradigms into a uniform compuatational element, a universal datatype (called a unit).

_
## === FUNCTION.GND	 -  Uniform Grounding of semantics down to its specification ===
### --- GND Overview ---

_**TL;DR.  The GROUNDING paradigm is a use of the OO notion of a Factory---a micro language that maps between the specification of a thing onto the live manifestations of that specification.**_

_**TL;DR. A collection of GROUNDINGS define "micro software languages".  Each grounding maps some specification of a functional capability onto a "live" software artefacts that has the capablities described.  This grounding notion of gets at the heart of a concept very widely used in computation -- the notion of "regularity".  Thus the GND element is the "center of the center" -- all of unicore computation depends heavily upon it.**_

In both common langauge usage and formal usage a "grounding" of a thing is that which affords meaning or ability of that thing.  (For example one might say: A strong grounding mathematics is require for work in Phyics, or Integer addition and multiplication can be grounded in Peano's axioms.)  The GND Element formalizes this notion as a mapping from specification data onto embodied things such that the functional behavior of the embodied thing is specified via the specification data.  This basic idea is applied extremely broadly within computation.  For example, source code is the specification of a running program, and a constructor expression is the specification of the OO-object that it causes to be created.  Here we formalized Uniform Grounding, and use this as the basis for all Uniform Computation.


**GND** -- A _**grounding (Gnd)**_ is a relation between a particular kind of specification (spec) and the (functionally active) embodiment of that specification.

**SPECIFICATION** -- A _**specification (Spec)**_ is a data value -- it is the symbolic/structural description of some functional artefact.

**UNIT** -- A _**unit**_ is the functioning artefact produced or selected based on some given spec structure.

**UNIVERSE** -- A _**universe**_ is the required context for a groundings and grounded arifacts to exist---it is an abstraction of computer memory.  In is made up of:
1. the heap used to persist the constructed embodiments (or re-constructed spec units too).
2. a fininte, static space of groundings of those embodimenets, which are themselves grounded by the universe itself.
3. a special 'ground' function that looks up an appropriate embodiment for a Spec.

**HOMOICONIC** -- We say that a unit is homoiconic iff _s_ == _u_.**gnd**().**spec**(_u_) and _u_ == _u_.**gnd**().**gnd**().**ground**(_s_).**instantiate**(_u_.**spec**()).  That is, if a unit is guaranteed to be equal to the instantiation of its spec structure.  (Unexposed internal state, or essential relationships to external units can cause a unit to not be homoiconic)



KEY DESIGN IDEA #1:
We Generalize the notion of homoiconicity to this idea of "embodied data element" -- the idea that some purely syntactic source structure could (without any additional contextizing parameters) be used to derive the currently embodied element.  i.e. the element has a structural specification that completely specifies the element itself.

This is consistent with the existing notion of a homoiconic language, this definition simply extends this idea to elements that are not understood primarlily as chunk of procedural code, but it is entirely consistent with the 'chunk of code' special homoiconic case.

KEY IDEA #2:
We generalize the common notion of type erasure to the computation is a two step process where source code is transformed into loaded form which is then further processed at a subsequent run time.  This two step evaluation assumption is most evident in the Eval element.  Still it also motivates the idea of a template as a grounding of a grounding.  The first grounding often occurs during this first phase of processing, the resulting intermediate is maintained as as part of the 'executable' loaded from source.  The second grounding (or spec-ing) can then occur at runtime.  This commoness of this pattern makes the idea of Template as a grounding of a grounding central to many aspecting of computing.





#### ...


**INSTANCE** -- An _**instance (Inst)**_ is the functioning artefact produced or selected based on a given Spec structure.

**ENV** -- The _**environment (Env)**_ is a handle for the context data used during the instantiation process.

	_ _call_ _: **slot**(**op**(...args..., env: Env, -> Inst))		# Constructs new inst from spec
	
CHOICE: Call a grounding a 'Factory'   (fac)
		Notice this means that no two factories can produce the same type
CHOICE: remove self.inst_template and instead use self.spec.gnd
_
##### examples

These concepts are so abstract, a few examples might provide valuable clarification:

- grounding -- A whole programming language like Python is an example of a grounding.  
	- Python source code is its spec, and a running program is its functional artefact.
	- A single class from an OO-language is a grounding that maps arguments passed to its constructor onto the running OO-instance indicated by thouse args.
	- A single format statement like f"Hello %s, good %s!" is a grounding that maps 
		a spec list ['dan', 'morning'] onto a strings that "functions" as a salutation.
		the string instance string. A single regex expression like "(\d\d\d)-(\d\d)-(\d\d\d\d)" lists of three numbers 
	- Or arguments passed to the constrcutor of a specific OO-language class. is a spec for the instance to be created, which is the functional artefact.
- spec / inst -- Any data can serve as a spec.  The key idea is that the spec "specifies" some functional behavior, but it, itself does not POSSESS or MAIFEST that behavior, rather it describes a behavior that the grounded this should posess.  Likewise the key idea behind the inst is to be the entity with the described behavior.  This situation is quite clear when the behavior being described is executable.  The python source code for a video game is just text, you cant play it.  While the running instance can be played.  But non-functional groundings are possible too.  For example one might use the natural numbers, 1, 2, 3, ... to "ground" the set of all primes.  So prime(1)=2, prime(2)=3, prime(3)=5, prime(4)=7, ...  So here we see that the spec of the fourth prime, 4, is itself not prime, but the instance it ground, 7, does exhibit the indicated "primeness" behavior.
- TEMPLATE -- A template is a grounding, that grounds instances which in turn are grounding for a second mapping to instances.  Both the OO-class definition and regular expressions are examples of template.  If we instantiate the source code for an OO-class, we get a running class, and the running class will map class constructor arguments into instances of that class.  The regex will map the characters of a regex expression into a matcher, where the matcher will map instance character strings back into the variables which match.  
- MATCHING -- Notice the regex matcher grounding is not functional.  Given a single set of spec variable bindings, there might be many instance strings that could parse backward onto those bindings -- there could be many instances strings that match a given spec of variable bindings, though in this case the inverse mapping from string instances onto variable specs is functiona.  In similar fashion, Likewise each time you map an OO-object into a functional instance you get a different instance.  This is all ok, groundings 
	
	
_

##### ...

	Spec:


	???

	Bang:
		**template**: AlphaNum => Gnd || Bang

	~-~~-~~ confused ~-~~-~
		  extends: Tree(Inst)	 		  // ???? CHOICE -- which one?  CHOICE

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


	Stmt:
		template:
		slot:		F[ s(SLOT_SPEC)]


	Spec:
		match: **slot**(t: Template, u: Unit, ->bindings: Unit)
		fill: **slot**()

	Slot:
		fields


	_
### --- API ---

Env: class::
	**extends**: Unit
	
Gnd: class::
	**extends**: []
	args: [of: INST]
	_inst_head_: **slot**(**op**(-> Ident))							# ?? maybe this is just its path?
	_inst_of_: **slot**(**op**(spec, Env, -> INST))	
	_spec_of_: **slot**(**op**(Inst, Env, -> spec))
	_template_of_: **slot**(**op**(-> Template))				

Inst:
	_gnd_: **slot**(**op**(-> Gnd))
	_spec_: **slot**(**op**(-> Unit))

_
### --- SEMANTICS ---

UNIVERSE
- Has a lexspace of groundings
- Has a set of Unit grounded by those groundings
- Has a 'ground' function that maps spec expressions into the set of groundings

GND SPACE
- FACTORY & MATCHER -- Each grounding is also both, but each may be undefined.
- STATIC -- The whole GndSpace is atomically created based on its contents field (Just as a lexspace is)
- FACTORY -- The Gex class is A grounding for all the Gnd elements in GndSpace
- HETROGENEOUS TYPES -- But the reverse is not guaranteed, element.gnd() need not all point back to the Gnd class

HEAP
- All Units grounded by a Gnd Space are part of a commone Gnd Space heap associated with the Gn

SHARING
- Units may reference units from other universes

### --- Common Example Languages ---

>>> TEMPLATE EXAMPLES <<<

Template	Tmp Spec	-inst->		C	Env spec  -inst->	Target Inst
STR-FMT		F"Hi {n}, good {tod}" 	*	{n=dan, tod=morn}	"Hi Dan, good morn"
REGEX		/d+\.d+/				*	z=re.match(c, x)	
code 		blk(a, b, c)
fn			f(x)=x*x				*	y=2 * f(z)			y = 2 * z * z		

Regex.inst( "(.)(.*)" ).spec("this") = ["t", "his"]
Uf.inst("f(x)=x*x").

   === LANG ===		     === DESCRIPTIVE SPEC ===		    === FUNCTIONAL INSTANCES ===

STRUCTURE-TEMPLATE
String-Fmt			Formatting string & args			Strings produced by the formatter
					F"Hello {name}, good {tod}"			"Hello Dan, good morning"			{name='Dan', tod='morn'}
Structure-template	Backquoted source code				Structures produced by backquoting


TOPLEVEL LANG
Procedural Lang		Source Code: Python, Java, ...		Compiled or JITed executable form
PHP					PHP source-code						Strings can can parse as HTML


SUB LANGAUGES
fn-args				Formal arglist in a fn definition	Resulting stack frame used during fn evaluation
OO-class-def	 	Source-code definition				Created instance with fields and methods
Source code pkg		Statements defininging contents		Contents structure resulting from pkg stmts


META PROGRAMMING

_
### --- GND.TEMPLATE ---
#### _


_**TL;DR. Templates are a special kind of grounding that itself yields instances which recursively are groundings.  For example, 'format' or 'printf' both accept strings and return formatter-instances that are themselves groundings.  These formatter instances accept a spec (a list string values) and intantiate the final formated string.  Thus format and printf are templates.**_


**TEMPLATE** -- A template is the grounding of a grounding, so it is a micro language whose instances are themselves specifications that are instantiated into some final target form.

**TEMPLATE** -- A _**template (Template)**_ is a kind grounding whose functional artefacts have the "function" of being templates, "spec entities" for some second lower grounding language.

_
##### ?

TEMPLATE
**SPEC_LANG** -- The _**spec_lang**_ for a base language is a "meta language" describing the spec structures used as input by the base language.  Notice:  all languages provide some kind of structural specifications for some kind of functional instances -- in the case of the spec_lang the "function" of its instances is to serve as structural specs for the original base language.  This chain of meta langauges can extend indefinitely depending upon the sophistication of the meta programming involved.


**STRUCTURAL TEMPLATE** -- A _**structural template**_ is kind of template language where the structure of spec elements parallels the structure of the functional instance elements they describe.

**SLOT** -- A _**slot**_ is a particular kind of leaf structure within a structural template.  These slot leaves are the part of the template structure that does not parallel the artefact structure it describes.

@persist -- slot adheres to persistance semantics

**SPEC_LANG** -- The _**spec_lang**_ for a base language is a "meta language" describing the spec structures used as input by the base language.  Notice:  all languages provide some kind of structural specifications for some kind of functional instances -- in the case of the spec_lang the "function" of its instances is to serve as structural specs for the original base language.  This chain of meta langauges can extend indefinitely depending upon the sophistication of the meta programming involved.

_
#### -- GND.TEMPLATE - API --


Template:
	**extends**: Gnd
	_fill_: **slot**(**op**(template, spec, Env, -> Inst))
	_match_: **slot**(**op**(template, Inst, Env, -> spec))


StringTemplate:
	**extends**: Template

UnitTemplate:
	**extends**: Template

EnvTemplate:
	**extends**: Template


Factory[TYPE]:
	_ call _: slot(op(Code, -> TYPE))
Matcher[TYPE]:
	spec: slot(op(TYPE, -> Code))
Gnd[TYPE]:
	extends: [Factory[TYPE], Matcher[TYPE]]

StringFactory:	extends:	Gnd[Factory[Str]]	# Formatter
StringMatcher:	extends:	Gnd[Matcher[Str]]	# Parser
UnitFactory:	extends:	Gnd[Factory[Unit]]	# Structure Templater
UnitMatcher:	extends:	Gnd[Matcher[Unit]]	# Structure Parser
EnvFactory:		extends:	Gnd[Factory[Env]]	# Arg Processor

	
	extends:	Gnd[]

_
### --- CHOICES ---
- CHOOSE: Grounding must efficiently point to their universe (that is where the 'ground' and other functions are)
- CHOICE: Universe extends Lexspace
	- YES - Maybe then Universe extends Lexspace since Lex point to their lexspace -- don't want too many pointers
	- NO - We need to chroot the universe which will force its duplication
- 
#### GND BASE-LEVEL COMMITMENT:  .instantiate .spec  .type
- A GND is at least a Factory & a Matcher
- There is a datatype that is a Factory+Matcher+Lex
- Instances have a way to indicate a base level grounding (.type)
- Gnd has a 'bang' operator that creates a new heap ???
_
#### UNIVERSE == HEAP + Groundings
- REQUIRES PERSISTED ENTITIES -- The GND element requires the creation of persisted entities both embodied Unit and underlying spec
- REQUIRES GROUNDINGS -- A universe *is* the collection of entities embodied by a defined collection of groundings
_
#### UNIVERSE SUFFICIENCY
A universe is self contained.  All needed info for computation can be derived from the universe.
_
#### UNIVERSE SHARING
- IMMUTIBLE SHARING -- It is important that immutible objects can be shared between universes, since they can be derived in one and used in another.  Also certain derived structures need not be replicated, and avoiding this can make universes more light weight.
- MUTIBLE SHARING -- Mutible sharing is also valuable since IO exists, we already have entities that are extrinsically modified.  Thus it is natural to allow one universe to provide IO elements for another universe

APPROACH: 
-- Every embodied Unit points to exactly one base grounding.
-- This grounding is in all universes that share these elements.
-- But not all entities accessible in a universe are based in that universe.
_
#### ????
- HEAP TREE - 
- GND CODE - The GND element assumes a set of 
- UNIVERSE SUFFICIENCY


### --- OLD ---
 GND - Wet Ink ---
#### -- Bundling eval/load/ground/ops --

ops = env.ground(expr, 0)		# returns ops
gnd = env.ground(expr, 1)		# returns grounding of 'expr'
exe = env.ground(expr, 2)		# returns the instantiation of 'expr' using the grounding
result = env.ground(expr, 3)	# returns the result of executing the instantiated 'expr' in same env


_
#### -- Generality and Semantics Discussion --

Most general case:
	spec + env --> inst			# This mapping is usually functional, and usually depends upon env vars in spec
	gnd.place --> ident			# Specs with this head are controlled by this grounding
	gnd.head --> ident			# This is the default head used by repr of inst of this gnd
	gnd.inst_spec -> spec	



WHAT IS A GROUNDING?
- FACTORY -- A grounding is a factory for producing instances of a kind of artefact from spec of this artefact
- PLACEMENT -- A grounding should be placed such that relevant spec will ground to itself
- META -- A grounding provides meta info about the indicated collection of artefacts (e.g. Template lang)
- REVERSE -- Reverse lookup

_
## === FUNCTION.EXE  -  Uniform Interpretation ===
### --- OVERVIEW ---

**_TL;DR. Uniform evaluation is a least commitment model of interpretation built upon the unit data graphs.  The model is federated in that it assumes collection of tightly interacting interpreters operate over a common execution environment structure which they have collectively constructed._**


The Eval element is a least commitment model of computation that makes the following three assumptions:

1. **HOMOICONIC** -- The Uniform Agenda is to express computation as a web of micro languages that are built on top of others.  Thus Uniform assumes computation is homoiconic (code is data as it is with Lisp) in order to facilitate language mix and match recombination.
2. **COMPILED** -- Uniform provides explicit representation of two distinct execution environments:  The load/compile time computational environment, and the distinct runtime environment.  This allows uniform to support language features like type erasure, as well as allowing the creation of meta scripting that explicitly controls the data flows that exist within complex language dependency graphs.
3. **FEDERATED** -- Uniform's computational models is federated in that it explicitly represents a space of compilers for different micro-languages which are expected to depend upon each other.  Again this choice is key to the mix and match language creation central to the Uniform Agenda.

_
### --- VOCABULARY ---

Here we define the key terms that underlie Uniform's Execution model.


**---EVAL-VOCABULARY---**

**ENV** -- An execution environment (Env) is a mutable data structure used to maintain the state of some execution in progress.

**EXECUTABLE** -- An executable (Exe) is a static data value that used to direct a specific "execution" within an appropriate universe.

COMPILER -- A compiler 

**GROUND** -- The _**ground**_ operator accepts a "source code" expression data and "looks up" and returns its "grounding semantics" for that data.  In uniform "grounding semantics" for some data is the grounding entity that instantiates these kinds of expression data into their executable form.

**UNIVERSE** -- An _**interpretational universe (Universe)**_ is a "running" interpreter---it includes all information and capabilities required to perform a particular kind of interpretation. 



**---EVAL.FN-VOCABULARY---**

**ARGS** -- An _**Args**_ is an Env template.  It instantiates a formal args list into an "args matcher", which in turn is used to instantiate some actual arg list into an executable matcher.  This executable matcher accepts some "calling" enviornment and returns sub enviornment obtained by matching those formal args against the supplied actual args.

**FN** -- A _**function (fn)**_ is an args paired with an expression.  The args is referred to as the function's "formal arguments", and the expression is referred to as the function's "body".  

**OP** -- An _**operator (op)**_ is a function whose first parameter is the 'self' object that it is expected to be called on.


**---EVAL.BANG-VOCABULARY---**
_TL;DR. Eval.Bang is a simple model of how computational universes comes to exist._

**THREAD** -- A thread is the potentially active execution of some executable

**BINARY** -- A binary is a static data structure that grounds an interpretational universe--it defines the meaning (execution) of code within that Universe, and it is use to instantiate that universe.

**BANG** -- The action of creating an interpretational universe from a binary.

**MAIN** -- The _**main**_ thread for a universe is the thread created by bang along with the creation of the universe itself.
### --- API ---


Scope:											  // Maps identifiers to their grounding
	**extends**: LexSpace[Place]

Code:											  // Used to ground Executables
	**extends**: Bounded, Tree
	
Executable:
	_ _call_ _: **slot**(**op**(env: Env, ->Unit))			// Should 'env' be passed or just assumed?
	_returns_: **slot**(**op**(e: Env, -> Unit))			// Alias for the _ call _ 
	
	
Compiler:									   	  // Used to lookup (ground) code to its meaning (it executable)
	**extends**: Gnd[Executable]
	_ _call_ _: **slot**(**op**(c: Code, ->Executable))	  	// Should 'env' be passed or just assumed?
	
Env:
	**extends**: Composite
	_eval_: **slot**(**op**(expr: Spec, -> Unit))
	_ground_: **slot**(**op**(_expr_: Spec, -> Gnd))		  // Could simplify to 'ground()' w no args?
	_scope_: **slot**(**op**(-> Lexspace))				
	_extent_: **slot**(**op**(-> Unit))				
	
Universe:										  // Rename as a 'lang'
	**extends**: Lexspace[Construct]
	_bang_: **slot**(**op**(spec: Lang, -> Env))			// Env is tied to a new Lang and Lexspace
	_thread_: **slot**(**op**(-> Lang))
	_prime_: **slot**(**op**(-> Env))



_
### --- EVAL SEMANTICS ---

**def** ground(_env_, _code_):
	**return** _env_.origin.follow(_code_.head)

**def** eval (_env_, _code_):			# Compiles all; uses constructor
	_compiler_ = _env_.ground(_code_)
	_executable_ = _compiler_.inst_of(_code_)
	**return** _executable_.exe(_env_)

**def** fn(_formal_arg_, _body_expr_, env=):
	_args_matcher_ = Args.inst_of(_actual_args_, env=_env_)
	_compiled_body_ = Code.inst_of(_body_expr_, env=_env_)
	**def** fn_invocation(_actual_args_, env=):
		_env2_ = args_matcher(_actual_args_, env=_env_)
		**return** _compiled_body_(_env2_)
	**return** fn_invocation
	
	
**def** bang(universe, bang_spec):
    env = universe.env
    compiler = env.ground(bang_spec)
    executable = compiler(bang_spec, env=env)  # compiles source in orig

    universe_gnd = executable["Universe"]		# Use new Universe constructor
    return universe_gnd(executable)

    # executable = universe.env.ground(bang_spec)(bang_spec, env=universe.env)
    # return executable["Universe"] (executable)


_
### --- old vocabulary ---

The EVAL paradigm is a least commitment that formulates interpretation as:
1. a grounding that maps code onto the executable that behaves as the code indicates.
2. a two step process:  source code --> executable code --> execution
	providing explicit framing for type erasure
3. and an explicit framing of a homoiconic language where "source code" is data structure optomized for meta programming.


_{[note:  parts of the API here do not look very 'least commitment' here but they are.  Many of these details are just a vanilla instantiation of the 'MATCH' paradigm which is used   contained within a sub paradigm for matching which is used for arg matching... this text just needs refactoring!]}_



**INTERPRETATION** -- _**Interpretation**_ is the process of:
1. Using the current execution Env to obtaining the "grounding" semantics for some "expression" data.
2. Using this obtained semantics to instantiate this expression data into its functional (executable) form.
3. Executing this resulting executable form.
4. Returning the result of this execution as the "interpretation" of the original expression data.
In uniform the _**eval**_ operator performs this grounding-based interpretation.

**EXPRESSION** -- Within an execution enviornment, an _**expression**_ is any data that has some evaluation behavior defined over it.  In uniform an expression is any data results that results in a grounding entity when passed to the ground lookup operator.

**LANGUAGE** -- A collection of expression groundings along with a ground lookup function together define a mini-iterpretation-based _**language**_.  In uniform we use _**Lang**_ to refer to a structure that somehow specifies such a collection of groundings.

**UNIVERSE** -- A functioning _**interpretational universe (Universe)**_ is all information and capabilities required to perform a particular kind of interpretation.  In uniform this requires (1) an interpretation Env, (2) the grounding lookup function, (3) a collection of groundings, and (4) optionally may include a set of all instances of that have been instantiated from the groundings of the universe (the interpretation heap).  In uniform these groundings are placed into a Lexspace for lookup by the ground function which itself is an operator of the universe itself.

**PRIME** -- 

**EXEC / EXE** -- The result obtained from grounding an expression is called an _**executable (Exe)**_.  According to the uniform definition of interpretation such executable instances can be "executed" in order to iterpret the original expression.  Thus in uniform each Exe has an _**execute (exe)**_ operator which accepts a computational enviornment, and "executes".  The result of executing this Exe defines the interpretative behavior of the original expression.

**ENV** -- The _**environment (Env)**_ is a handle for "all information that is derivable from a single instant in time within the current computation."  

	??? **ENV** -- A _**uniform environment (env)**_ is a complete set of value-place assignments "at a single point in time".  This means it is a complete set of assignments which satisfy all partial order temporal relationships.
	
Notice:  This definition of Env is consistent with, but is an extension of the meaning ascribed to an Env by the GND (grounding) paradigm.

A simple way to think about Env is to think of it as an element of the executing stack frame, but implementationally parts of it may be embedded as constants within compbiled code, in thread-local storage, or global state as well.  It is a semantic handle potentially containing any info that could be derived and used during computation.

~-~



The interpretation behavior referred to as a "function call" is produced from (1) a base env, (2) an actual arg list, and (3) a fn instance.  The call behavior is to use the fn-args to instantiate the actual args as a sub env and then execute the function body in that sub environment and return the results of that execution as the result of the function call execution.


~-~~

**SELF-CONTAINED UNIVERSE** -- A universe is _**self-contained universe**_ if all instances of of the universe are grounded by the groundings in the universe's place space.  (This requires that even the instance entities that serve as grounding entities in the universe's place-space must recursively instantiated from other entries in that same space.  

**UNI-BANG UNIVERSE** -- A _**uni-bang universe**_ is a universe that was instantantiated from the Bang grounding of some implentation of pure unicore.  (See EVAL.BANG discussion for value and intent of uni-bang universes.)

**BANG** -- The process of creating a new functional interpretational universe from the Lang specification of that universe is called "bang".  In uniform, _**Bang**_ is a grounding entity for universes, it accepts a Lang spec and returns a functional universe---a universe capable of interpreting expressions encoded in its underlying interpretational Lang.


**THREAD** -- An interpretational universe is a static collection of units.  The "thread" is the way one gets it to _DO_ something.  The _**thread**_ operator on a universe accepts an executable from that same universe and begins a new "thread" of interpretation based on that executable.  If the 'wait' flag is set (or the universe is single threaded), then the thread operators "waits" for the interpretation to complete, so it can return the result of its execution as the result of the thread operator.  As its name suggests the interpretation of multiple thread share and can interact thru the mutible state of the universe.

_
#### ,

**APPLY** -- The _**apply**_ operator "applies" some "grounding" expression to a "source" expression in order to "interpret" that source expression.

**INTERPRETER** -- An entity having an "apply" operator as described above.

**CODE PLANE / CONTROL PLANE** -- The data structure used to persist code expressions (and all other data in Uniform) is called the _**code plane**_ and within this is a read-only subspace called the _**control plane**_.  For simplicity it is useful to have data organized into a single structure, but for both security and semantic clarity it is valuable to segregate the controlling ground information from the expressions that are being evaluated.

BANG -- A bang is a running computational universe.

**BANG** -- The _**bang**_ operator creates a new interpretation universe.  It accepts a control plane which will  become the code plane for this newly created universe -- It defines how interpretation operates in this new universe, and thus serves as the single structure that fully specifies the semantics of this newly created universe.

BANG -- Bang is the grounding for interpretation univers
_
_
#### ...

**UNIVERSE** -- A execution universe (Universe) is a static data structure that functionally defines the meaning (execution) of code within that language.

In Uniform a Universe is a Lexspace of Groundings (sometimes called the "grounding plane").  This is a static tree of mini-langauge compilers, where each grounding is a factory that maps some source code expression into the executable unit that will perform the execution actions indicated by that code when called.

THREAD -- 



**UNIVERSE** -- A _**interpretational universe (Universe)**_ is all information and capabilities required to perform a particular kind of interpretation.  In uniform this requires (1) an interpretation Env, (2) the grounding lookup function, (3) a collection of groundings, and (4) optionally may include a set of all instances of that have been instantiated from the groundings of the universe (the interpretation heap).  In uniform these groundings are placed into a Lexspace for lookup by the ground function which itself is an operator of the universe itself.


**GROUND** -- The _**ground**_ operator accepts a "source code" expression data and "looks up" and returns its "grounding semantics" for that data.  In uniform "grounding semantics" for some data is the grounding entity that instantiates these kinds of expression data into their executable form.

**EXPRESSION** -- Within an execution enviornment, an _**expression**_ is any data that has some evaluation behavior defined over it.  In uniform an expression is any data results that results in a grounding entity when passed to the ground lookup operator.

**LANGUAGE** -- A collection of expression groundings along with a ground lookup function together define a mini-iterpretation-based _**language**_.  In uniform we use _**Lang**_ to refer to a structure that somehow specifies such a collection of groundings.




Uniform evaluation semantics are define by three functions: ground, eval, and bang as described here.  In order to fully understand Unicore evaluation semantics we include the source code for each as well.


**GROUNDING**
- **EXPRESSION GROUND PROCESS** -- The grounding process maps a code expression onto the factory that gives it a functional meaning---that compile it into an executable.
- **ORIGIN** -- The ORIGIN is a vertex within the space of grounding for this computational universe (sometimes called the grounding plane).
- **GROUND** -- This function looks up appropriate the grounding (compiler) for a source code expression.  This unicore grounding function simply uses the head of the expression, but more advanced languages may utilized complex lookup functions that rely on typing and other info from the compiling context.

**def** ground(_env_, _expr_):
	**return** _env_.origin.follow(_expr_.head()))


	
**EVALUATION**
- **HOMOICONIC** -- Uniform is a homoiconic language, so its source text is first parsed into a recursive data structure (called code).
- **PARSING** -- The singleton "flat.SOURCE" is the default flattening that maps between text and code.
- **COMPILING** -- Uniform is a compiled language so expression are first turned into an executable before interpretation (execution).
- **EXECUTION** -- Unicore assumes the existence of a virtual (or real) machine for execution, but it places no constrains on what that machine might be, or what its machine code might look like.  Thus an executable is opaque, and execution is just the one argument operator shown here.
- **STEPS**:
	1. Retrieves the grounding for an expression.
	2. Uses that grounding to compile that expression into an executable.
	3. Executes the expression within the current environment.

code = flat.SOURCE.parse(source_code_string)

**def** compile(_env_, _code_):
	_compiler_ = _env_.ground(_code_)
	_executable_ = gnd(_code_)
	**return** _executable_



**BANG (Computational Universe Creation)**
- **BANG** -- The creation of a new computational universe (either a language or an application)
- **BANG_SPEC** -- A specification of the new universe which is interpreted by the CURRENT universe.
- **STEPS**:
	1. Parse the bang_spec into an expression (if it is a string)
	2. Ground that expression to get it compiler
	3. Compile the expression to get the resulting tree of grounding instances and other executable code
	4. Instantiate a new universe based on the tree of compiled artifacts.  
- **NOTES**:
	- The constructor for the new universe is pulled out of this compiled structure.  This allows new universes (langauges) to specialize their own Universe instance modifying the Universe class used within its own instantiation.
	- The constructed universe does not depend upon the compiling universe unless their are aspects of that universe that are explicitly copied into the spec for the universe, OR if during compilation some linkages are created by to the compiling universe (which is discouraged).  
	- The Universe class' init function can be specified as a kind of 'main' function if one wishes for the Universe to express a running application.  In this way that 'bang' operation will serve a the 'run' function for the universe application, as it will indirectly being execution of the init function (which need never terminate).

- recursively grounds out in this Unicore spec.
- BANG.LOADER -- The language used to interpret the bang spec in order to instantiate the grounding plane used by this lang
- BANG.FORM -- The expression that initializes the grounding plane before universe instantiation
- UNIVERSE.INIT -- The expression that executes upon creation


**def** bang(_universe_, _bang_spec_):
	_env_ = universe._prime_
	_parse_fn_ = _universe_.origin.follow(["flat", "SOURCE", "parse"])
	_bang_expr_ = _parse_fn_(_bang_spec_) **if** isinstance(_bang_spec_, str) **else** _bang_spec_
	_compiler_ = _env_.ground(_bang_expr_)
	_grounding_plane_ = _compiler_(_bang_spec_, env=_env_)  # Translates universe spec 

	_universe_factory_ = _grounding_plane_["Universe"]  # Get Universe constructor from newly compile plane
	**return** _universe_factory_(_grounding_plane_)







		
CHOICE:  maybe 'ground' is a method of Universe?


Uniform evaluation is performed by obtaining the grounding of an expression to guide its interpretation, and then obtaining the grounding of the grounding to guide in the interpretation of that interpretation.  The apply operator "applies" this control expression to the original source expression in order to perform evaluation.  This double grounding might seem a bit circuitous but it allows fine-grained intermixing of different interpretation paradigms within a single evaluation environment.

_
### --- CHOICES ---

#### -- Name Universe or Lang
#### -- BANG must create an env -- 
#### -- Can Universes share heap? --
### --- LOG ---
#### -- 2021-11-08 - rejigger global handles for universe --

Spec	Static	CanCompile	Heap	SubThread	Env

Binary			Lang							Env


LexSpace					LexSpace(contents=Tree[Lex]) 
Lang extends Lexspace:		.bang(argv)->Env  	.chroot  .to_str  .ground  .compile
Env extends Composite:		.env(lang=, extent=, ...)  	.lang	.self  	.extent  
Executable:
Lang == fully specified way of interpreting forms


NOPE
Thread:
EnvTemplateFactory:			(lang=, extent=, ...)->  EnvFactory: (Env)->Env
FullEnv:					.self  .scope

Hidden Globals:  ENV, EXTENT/GLOBALS/CLOSURE/CONTEXT

_
#### -- 2021-11-09 - Env handling --

CONSTRAINTS
- SIMPLE STACK:	Efficient single threaded stack implementation
- ARG_FILLING: 	Can be filled in by a matcher
- FORK THREAD:	Can be part of a new asychronous sub-thread
- ADJUST STUFF:	Can adjust lang, extent, and self
- MULTIPLE:		Can perform multiple actions in a sub-env w/o bundling them into an executable

ENV API

.env(self=Unit, extent=, lang=Lexspace, asynch=Executable, pop=Bool)

.spec(into=)

_
## === FUNCTION.FLOW  -  Uniform Control Flow ===
### --- FLOW Overview ---

_**TL;DR. The uniform flow paradigm provides a simple formulation of "procedural control flow" as the recursive combination of five flow operators sufficient for encoding most/all of the control flow provided by most procedural programming languages.**_


**CONTROL FLOW** -- The process of deriving a totally-ordered sequence of statement invocations given an execution environment

The repeated selection of a "next step" for execution given an enumerated set of possible steps.

**STRUCTURED CONTROL FLOW** -- Control flow driven by a code structure with statement units as leaves.  Control flow is constrained to flow among these leaves following the structure of the code tree.

**UNIFORM CONTROL FLOW** -- Structured control flow built from the five uniform flow operators

**CONTROL FLOW** -- The information required to derive a totally-ordered sequence of statement invocations given some execution environment.

**FLOW** -- The uniform flow paradigm is a grammar used to construct "control-flow" tree based on its five flow statements.  Uniform evaluation can evaluate these flow tree structures in order to generate a temporally ordered execution of the leaf statements within this struture.

**LEAF STATEMENT** -- A _**leaf statement**_ is a leaf node within the control flow tree.  During interpretation of the flow tree these leaf statements may be interpreted zero or more times in different temporal orders depending upon the control flow logic contained in the flow statement internal nodes of the flow tree.

**FLOW STATEMENTS** -- These five flow statements are recursively combined into control flow trees used to drive the control-flow interpretation:
	**BLK** -- The _**block**_ statement will execute each of its sub statements temporally in the same order that they
	 	are listed in the block statement's structure.
	**BRA** -- The _**branch**_ statement will execute at most one of its sub-statements depending upon the results 
		of the conditioning expression at the front of each of its branches.
	**CHN** -- The _**chain**_ statement is the uniform "method call" operator, it interprets each of its 
		sub-statements in temporal order just as the block statement does, but when chaining each of these
		sub-statements is interpeted within an environment that is "chained" (derived) from the semantic rooting of the result of the last sub-statements interpretation.
		(The chain statement depends upon details of the Eval paradigm to construct these sub-environments,
		and upon the template paradigm in order to fill in that sub-enviornment.)
	**REP** -- The _**repeat**_ statement will repeatedly interpret its child statement.
	**TRY/RET** -- The _**try-return**_ statement will terminate interpretation of some dynamic package of interpretation.
		(The ret flow statement depends upon the Pkg paradigm to provide the targets for the termiation flow)
	
_
### --- API ---

Flow Grounding

Stmt:
	_block_:	F[ blk(BODY, ...), BODY=Stmt ]
	_branch_:	F[ bra([SWITCH]), '->'(CONDITION, BODY, ...), ...), SWITCH=Expr, CONDITION=Expr, BODY=Stmt
	_chain_:	F[ blk(BODY, ...), BODY=Expr ]
	_repeat_:	F[ rep(CONDITION, BODY, ...), CONDITION=Expr, BODY=Stmt ]
	_return_:	F[ ret(TARGET, RESULT), TARGET=Unit, RESULT=Expr ]

	:STMT
	STMT:	||: block, branch, chain, repeat, return
	EXPR:	Expr
	BODY:	Stmt
	TARGET: Unit
	SWITCH:	Expr
	CONDITION: Expr

_
### --- Pseudo code ---


**def** chn(*exprs):
	result = None
	env = ENV
	**for** expr in exprs:
		result = eval(env, expr)
		env = new(env)
		env['self'] = result
		env['ctrl'] = result.head
		match(env, expr.head, expr)
	**return** result


**def** chn(*exprs):
	result = None
	env = ENV
	**for** expr in exprs:
		result = eval(env, expr)
		env = new(env)
		env['self'] = result
		env['ctrl'] = result.head
		match(env, expr.head, expr)
	**return** result


**class** Chain(Flow):
	**def** _ init _ (self, *exprs):
		self._exprs = map(E.inst, exprs)
		self._matchers = [E.ground(e).matcher(E.ground(e).args) for e in exprs]   # where to get the matcher fn from?
		
	**def** exe(self):
		global E
		original_env = E
		u = self._exprs[0].exe()
		for i in range(1, len(self._exprs)):
			E = E.new()
			self._matcher[i] (original_env)      # Pushes its result back into the current env ???
			u = self_exprs[i].exe()
		E = original_env
		return u


**class** Chain(Flow):
<--- statically determined arg matcher

	@staticmethod
	def executor(base, expr):
		def exe():
			global E
			orig, E = E, E.new()
			
			u = base.exe()
			
				
		
			
			
			
			
		    

_
### --- BLK - Block ---
### --- BRA - Branch ---
### --- CHN - Chaining ---



_
### --- REP - Repeat ---
### --- RET - Try-Return ---

_
## === FUNCTION.PKG  -  Uniform Source-Code Packaging ===
### _

_**TL;DR. The Pkg element formalizes the idea of statically combined collections of source code forms.  These combiantions are used to define 'scopes' --- mappings from identifiers onto meanings for those identifiers.**_


**SOURCE CODE ORGANIZATION**

The pkg element formulates the set of assumptions and behaviors that are baked into nearly all mature software developement languages in a way that is general enough to serve as a common basis underlying these languages.  Here are the assumptions about source code organization that it is based upon:
- **TREE** -- the source code for most software languages can be understood as a tree structure of information --- one can thing of this as the abstract syntax tree (AST) from parsing the language, or the output of the reader for a homoiconic language.
- **GROUPINGS** -- most software languages partition this source code tree into referenceable grouping of 'top-level' source code structures.  (We mean this word group in the most general sense, so it would include class definitions, software source files, packages, modules, sub-packages, inner classes, executable forms within a method or function definition, etc.  These are all groupings of one kind or another.)
- **STATEMENT / EXPRESSION** -- often we refer to thse top level tree substrees as the "statements" of the language which because they are all top-level cannot contain "sub statements" as their children.  Expressions by contrast are source code structures which are the children of these statement forms, and typically (with restrictions) these nodes are allowed to recursively contain sub expression nodes.
- **DIRECTIVES** -- Some of the statements within a source code grouping are directives indicating how this grouping should be combined with the source code forms from other groupings.
- **STATICALLY ANALYZABLE** -- A key aspect for most languages is the idea that the resulting membership of source forms into different groupings can easily be understood merely by quick static inspection of the source code itself.


All of these ideas underly formulated pkg shown here:

NOTE:  We use the term 'pkg' to capture all forms of source code grouping, so this covers the traditional software packages, sub-packages, and modules.  Further it covers OO-classes, sub-classes, as well as groups of executable statements within a function or method definition, a dynamic extent defined by a with clause.  Even groups of horn clauses for prolog or equations for a math solver could also be formulated as packages.  Really any source code region that is reference and operated on as a group is expected to be encoded using this package element.


**PKG** -- A _**package (Pkg)**_ is a collection of source code forms and statically expressed combination directives (e.g. import, extends, etc.) which together define the 'scope' of the package--the mapping from identifiers to source forms.

Importantly the scope of a package must be easily derived from static inspection of the source code -- i.e. a person should be able to merely look at the code and quickly answer questions about which source forms are contained within a given package.

**PACKAGE SPACE** -- _**Package space (PkgSpace)**_ is a lexspace of packages, thus each package will have a unique "address" within pkgspace which can be used to identify each particular package.

**PACKAGE STATEMENT** -- Package _**statements**_ are the loaded (embodied) version of toplevel forms (direct children) of the package code form.

When a package code form, p, is loaded into a computational environment, each of its direct children are also loaded, and these become the 'statements' of p.

**PACKAGE DIRECTIVES** -- _**Package directives**_ are a statements that specifies how package contents are to be combined in order to compute the scope for each package.

**PACKAGE CONTENTS** -- _**Package contents**_ are the non-directive statements within a package.  These statements are the "raw material" that is combined to form the scopes for the packages.

**PACKAGE SCOPE** -- Each package defines a _**scope**_ which maps identifiers onto the source form denoted by that identifier within that scope.  These scopes are encoded as a PlaceSpace containing package statements derived from the original source code.

~-~-~ 



--older version--

_**TL;DR. The Package element provides the ability to group source-code elements, as well as the expected static and dynamic manipulation operations expected over such groups (e.g. import, export, extends, with, etc.)**_


**PKG** -- A _**package (pkg)**_ is a source-code unit whose "_**contents**_" (its semantic functionality) is composably derived from the source-code elements of this and other packages by applying a sequence of package definition "statements" provided in this an other packages.
. defined from this and other   in a set theoretic manner from the functionality provided by its structural components.

_{[BUG: This is correct, but not maximally general.  this should be a statement about scope contents being functionally derived in a statically aparent way.]}_


**PKG STMTS** -- The package paradigm defines a set of _**package statements**_ which facilitate the construction of aggregated packages with aggregated functionality based upon as set of given building block packages.

**IMPORT/EXTENDS** -- The _**import**_ and _**extends**_ statements add one or many elements to a package as drawn from other packages.

**EXPORT** -- The _**export**_ statement explicitly 

**INCLUDE/EXCLUDE/BEFORE/AFTER/APPEND/EXTEND** -- The _**include/exclude/before/after/append/extend**_ statements are sequence manipulation operations that are meaningfully applied to packages whose contents are understood to be a semantic sequence.  (e.g. the UNIX path variable is a package whose semantics is defined in terms of the sequence of its contents)


**CONTENTS** -- A package's _**contents**_ is the strcructure that results from applying all of its defining statement operations.  e.g. it is the semantics/functional structure that _results_ after from applying all its import/include/exclude pkg statements.


_
### --- PKG API ---

CHOICE: Move '+' to composite
CHOICE: Do we need from / import form?  Do we need import at all?  (Just treat as list-op)

Pkg:
	**extends**: Composite
	_contents_: **slot**(-> Composite)

stmt.pkg:
	import:		import(IDENT, as: ?IDENT)
	extend:		extends(IDENT...)
	with:		with(DELTAS, BODY...)
	
_
### --- Details ---

- pkg
- with
- import IDENT as: ?IDENT
- extends IDENT...
- "+"(STMTS...)					// Appends all 
- '-'(MATCHER)					// Removes all matching elements
- '<'(MATCHER, STMTS...)			// Add stmts after
- '>'(MATCHER, STMTS...)			// Add stmts before matcher
- ':'(key, value)				// Adds a key value to package



_
## === FUNCTION.OBJ  -  Uniform Object Model ===
### _

Insert definitions of Unit and Composite here

A uniform unit the four structural and four functional paradigms into a least commitment object model.  This model can be specialized to match existing software paradigms.

One can think of this as the ultimate foreign function interface...  Ultimate in the sense that there is no 'native' objects, all are expressed using the same underlying unified paradigm.


**STRUCTURAL TEMPLATE** -- A _**structural template**_ is a kind grounding template where the structure of its spec entities parallels the structure of the instances that they specify.  Typically the structure of template specs have "slots" in them, parts of the structure that can match varying parts of the instance structure.

**SLOT** -- A _**slot**_ is a particular kind of leaf structure within a structural template spec.  These slot leaves are the part of the template structure that does not parallel the artefact structure it describes.

**SLOT TEMPLATE** -- A _**structural template**_ is kind of template language where the structure of spec elements parallels the structure of the functional instance elements they describe.


_
### --- API ---

Class:
	extends: Pkg
	_**structure_spec**_: slot(t: )
	// DERIVED
	**new**:	slot(t: `this.form)
	
Slot:
	**extends**: Gnd
	**template**: U.slot(ACCESSOR..., n:NAME, t:TYPE, d:DEFN)
	_ _init_ _ : **slot**(op(Accessor..., _k_:Ident, _n_:NAME, _t_:TYPE))
	_ _call_ _ : slot(op)
	// derived stem==self_type; parent; schema

Executable:
	_args_: **slot**(**op**(-> TemplateOfEnv))				  		//  move to OBJ?
		
_
### --- OBJ.ARGS ---

-NOT CORRECT-

ARGS -- An Args is a Gnd.Template

def compile(actual_args, formal_args):
def Fncall.inst_of(invocation, env=env):
	invocation.head
	
	_
### --- OBJ.FN ---

-NOT CORRECT-

FN -- A function (Fn) is a an (args, code) pair where args is an instantiation of a formal arglist Args and code is an instantiation of some code body.

def funcall(fn, actual_args, env):
	subenv = fn.args(actual_args, env=env)
	return fn.code(subenv)
	
 is an Args matcher thainstatiated with 
_
### --- SEMANTICS ---

**def** Universe.**bang**(_self_, _spec_, argv=_List[Str]_):
	_grounding_plane_ = _self_.load(_spec_)
	_universe_constructor_ = _grounding_plane_["Universe"]
	_new_universe_ = _universe_constructor_(_grounding_plane_)
	_main_fn_ = _new_universe_["main"]
	_expr_ = Q.main(List(++_argv_))
	_executable_calling_main_ = _new_universe_.load( _expr_ )
	
	_new_universe_.env_push([argv])
	_result_ = _executable_calling_main_( _main_fn_["body"] )
	_new_universe_.env_pop()
	**return** _result_


# ### RUNG 4 -- UNIFORM MENAGERIE (LIB) ###

# ### RUNG 5 -- UNIFORM LANGUAGES (LANG) ###
# ### LAL - Language Authoring Language ###
## === LAL.BOOT ===
### --- BOOT ---
#### _

**TL;DR.  Uniform Boot initiates execution of some target language or application recursively from its specification expression as the simpler language used to interpret this specification.  This recursion always terminates in Unicore as the base language.**


**BOOT BLOCK** -- A _**boot block**_ is a chunk of code (called the "_**boot body**_") delineated within a larger text document by a special marker (called the "boot mark").  This marker indicates the beginning of the boot body, and also specifies the language to be used for the intepretation of the code chunk.



Here are two examples of a boot block.  Both of them are using the languages called 'SOME.LANG.NAME'
to interpret their boot bodies:


-~- SOME.LANG.NAME -~-
code block contents
...
-~-


   /*  This boot block is embedded within a comment within some source file
    *
    *  -~- SOME.LANG.NAME -~-
    *    code block contents
    *    ...
    *  -~-
    *
    */


**BOOT MARK** -- A _**boot mark**_ is a specific pattern of characters that indicates the beginning of the boot body for a boot block.  The following regex is the boot mark for the Uniform family of languages.  The first capture group is called "the prefix" and the second capture group is called "the boot mark":
    /^([.*\S])\W*-~-\s+(.*\S)\s-~-/

**BOOT BODY** -- The boot body for a boot block is the sequence of characters beginning after the boot mark upto but not including the first match of this regex /^{PREFIX}\W*-~-/ which signifies the termination of the block.  Notice the prefix from the boot mark is inserted here.  If the boot mark has a prefix string then that prefix string is removed from the beginning of each line where it occurs within the boot body.

**BOOT** -- the creation of a computational environment whose properties are derived entirely from the supplied specification of that environment.  


**BOOT CHAINING**
Notice each boot block specifies some code which is to be interpreted, as well as the language that is to be used to intepret that code.  Within the Uniform ecosystem the language used for interpretation will be a unicore Universe instance that itself was defined (typically using its own boot block) using a yet more basic language.  This recursion continues until finally reaching Unicore as the base for all Uniform languages.

Within lexspace the 'uniform_languages' namespace prefix is used to organize the languages available.  The unit in this location is a Universe constructor mechanism that retrieve available languages using their path name.  Typically this constructor is also caching system since uniform languages are static entities it need not reconstruct them, but can simply reuse them.  Below is the source code that defines the boot chaining behavior for boot marks:


**def** boot_chaining(_universe_, _boot_mark_, _boot_body_):
	_lang_path_ =  Ident.pathify(_spec_parts_[0].strip())
	_boot_lang_ = _universe_.origin["uniform_languages"].follow(_lang_path_) 
	_parse_fn_ = _boot_lang_.origin.follow(["IO", "parse"])
	**return** _boot_lang_.bang(_parse_fn_(_boot_body_))

This function parses the bootmark as an identifier and uses its path to lookup the chained boot language.
It then pulls the string parsing function out of that universe to parse

#### ,


// Is this right?
**def** bang_chain(_universe_, _spec_):
	_spec_parts_ = _spec_.split(" ", 1)
	_prefix_path_ = ["lang", "langs"] + Ident.pathify(_spec_parts_[0].strip()
	_prefix_universe_spec_ = _universe_.prime.origin.follow(_prefix_path_)) 
	prefix_universe = _universe_.bang(_prefix_universe_spec)
	**if** len(spec_parts) == 1:
		**return** _prefix_universe_
	**else**:
		**return** _prefix_universe_.bang_chain(_spec_parts_[1])

**def** bang_chain(_universe_, _spec_):
	_space_idx_ = _base_spec_.strip().find(" ")
	**if** _space_idx_ == -1:
		**return** _universe_.bang( _universe_.env.origin["lang"].follow(Ident.pathify(_base_spec_.strip())) )
	**else**:
		_base_ = _universe_.bang( _universe_.env.origin["lang"].follow(Ident.pathify(_base_spec_[:_space_idx_]) )
		**return** _base_.bang_base(_base_spec_[_space_idx_:].strip())



_
#### -- Boot API / Semantics --

def boot_marks(Textual t, -> List('BootMark(Form, ?Form)))

**def** boot(lang_spec, code_block=None):
	e = EXE(lang, ns: UF.BOOT)
	**if** code_block:
		**return** e.run(code_block)
	**else**:
		**return** e

**def** _boot_(lang_spec, code_block):
	**return** run(lang_spec).run(code_block)
	
**def** _run_(code):
	**return** EXE(code, ns: uf.boot)




_
#### -- Examples --
##### - examples -


	// -~- http.com.mycorp.2019.loader -~-
	//
	// pkg examples.tst::
	//   load( ^source )
	//   foo: 1+1
	//   bar: 2+2
	//
	//   out(a, b) <-- inp(a, c)
	//    { b := python_fn_below(c) }
	//
	// -~-
##### - boot combo builder -


-~- unicore.0.1 -~-     // This block is in the "_" file
export uf.2.0
import uf.langtools

def Lang uf.2.0 := PKG1 + PKG2
pkg uf.2.0:
	import .tree(boot=., ext=py)
-~-


// Here is the ENGINES construct in ENGINES.py file
-~- . -~-
##### - sub lang builder -

CFG.py
-~- . -~-

pkg pkgs.ENGINE.CFG:
	


return ENV(lang.env.self, ns)
-~-
##### - Lang Pkgs -


-~- ufsimple -~-  	// in file ../../../TLC

pkg(lang.uf):
	live.push https.org.uniform.lang.uf.2019.10.alpha
	
-~-


-~- . -~-   // in file CFG.py

def pkg .ENGINE.RULE:

	def opt rule: 	left, right
	def type "<--": @rule=left

def pkg .ENGINE.CFG_GRAMMAR:

	def type CFG_grammar = List
	def type CFG_rule: ...

	def class CFG_Grammar < ENGINE.GRAMMAR:
	
		def op rewrite(): ...
	pkg 
#### -- **Unicore Boot Semantics** --

**def** op "+"(_e_ Env, _s_ Spec):
	**if** _s_ =~ Str:
		**return** _self_ + _self_.lang.parse(_s_)
	**elif** _s_ =~ Spec:
		**return** _self_ + _self_.lang.load(_s_)
	**elif** _s_ =~ Exec:
		**return** _self_.NEW().EXE(_s_)
	**else**:
		error "Invalid Spec"

**def** fn exe_code_mark(_mark_ Str, _body_ Str, _base_ Unit):
	**with** lang.origin = _base_:
		**return** EXE(lookup(Ident(_mark_))) + _body_

	

**def** boot(_language_path_):
	**return** load(EXE(lang.path_get(_language_path_)))

**def** run(_program_source_)
	**return** EXE(load(parse(_program_source_)))

**def** boot(_language_path_):
	_boot_expr_ = lang.path_get(_language_path_)
	lang_exec = EXE(_boot_expr_)
	lang_env = load(_lang_exec_)
	**return** _lang_env_

**def** run(program_source)
	program_spec = parse(program_source)
	executable = load(program_spec)
	**return** EXE(executable)

~-~~
e0 = Env()
e1 = e0.boot(e1_src)
e2 = 

**def** op boot(env, source):
	e = env.NEW()
	init_expr = e.load(env.parse(source))
	e.EXE(init_expr)
	**return** e
#### -- Discussion --
##### discussion



	Thus this created environment does not inherent any computational properties from its implementing substrate, all of those properties are derived from its specification.


	A function call performs an initiation of some computation -- the body of the function.  But notice many aspects of the nature of the computation derive from its calling context and upon the calling language itself, so this is not _boot_ operation.  By contrast, beginning the execution of some program given only the code itself, its starting parameters, and a specification of the CPU instruction set, _is_ an example of _booting_ that code.  Notice we would call this booting if we implemented the CPU directly using transistors, or if we used an emulator executing within some host OS.  Unlike the function call case, both of these are boot operations since both booted systems makes no reference to, nor dependancy upon any details of the hosting substrate other than their faithful implementation of this target's execution rules.


	Uniform aspires towards an essential (a most general, most practical) formulation of BOOT. It is expressed as a progression of langauges defined in terms of earlier languages with unicore as the base language.  Unicore itself strives to only utilize constructs in their most elemental form.  The hope is that booting in this way means that the boot loader itself is really just written in a paired down version of the larger languages that are being booted.  In this way, the booting process itself adds minimal additional complexity over just the specification of the target language.



	CODE

	 most general and practically useful formulation of the boot concept.   model of boot. 




	 given the specifiation of a CPU and code to run on 






	**COLD BOOT** -- Performing a boot directly from some non-uniform computational environment.
##### - Boot mark discussion -
		**LANG INITIATOR** -- The lang initiator is a textual mark used to indicate the presence of some source code from within the uniform language family.
		- ALIEN CONTEXT -- Format is designed to operate well within a wide range of foreign contexts.  This means it should not place constraints which limit either the foreign context, nor the embedded language.
		- MINIMAL SYNTAX CONFUSION -- Specific 
	
			      -_-_- 2019.uf -_-_-
			      
			      _-_ 2019.uf _-_
			      
			      -_- 2019.uf -_-
			      
			      - _ - 2019.uf - _ -
		
		
			      _-_ 2019.uf _-_
		
			      _-_         _-_
##### Details

	DETAILS ABOUT THE LANGUAGE BLOCK

	- The lang block must occur within the first kilobyte of its containing file.  (The block contents may have any length)

	- BOOT EXPRESSION  (Ident or fncall)

	- CHAINED BOOT  .boot(subexpr)
		Each computing environment is booted from some prior environment.  
	
	- LEXICAL BOOT -- Up"foo.py" -> 
		A block mark that begins with a preceding '.' 


	- A content marker is a marker that indicates content type for uniform.
	- It is a ditinctive, single-line text-based character pattern.
	- It is flexibly designed so it can be placed within a comment region of another content type.
	- Example marker:    -~- 2019.uf -~-

XXXXX

	- CONTENT BLOCK STRING
		The content block begins on the first character after the newline after the lang marker, and ends on the last character before the newline before the termination marker.

Processing
	- Search first and last kilobyte of text file for content marker
	- If found then a forward (or reverse) scan for a SECOND marker is performed in order to bound the marked content.
	- If all of the marked content (including the leading and trailing marker lines) and a consistent prefix characters before the content markers, these characters are stripped from all lines.
	- The lead (or trailing) content marker indicates the language format (fmt) of the text between the leading and trailing content markers (content).
	- Processing of the body is accomplished by:
		_.get('std.lang + fmt).bang.load(content)
	
	- The content itself often loads the body of the full file. Load analyzes the file to decide how to load it into runtime, then performs this integration (perhaps by connecting a python, JS or other interpreter.)
  

_   
##### - boot generality -

	The aim is to express "boot" in the most general terms possible.
	Here are the parts that together are the framing that yielded the boot construct:

	**SUBSTRATE** -- A thing with agency.  
	Interpretation is a process of interpreting a structure.  This involves change over time in a way that is constrained by the rules of interpretation.  The substrate might be a collection of transistors wired together, a program running on someones laptop, or a nation of people passing slips of paper to each other.  The point is, it is a thing with agency that can be made to change itself over time according to the rules of interpretation.

	**MACHINE SPEC** -- A specification of the rules for interpretation.
	In order for interpretation to occur the substrate needs to be configured in a way that it not only changes over time, but that it changes over time in accordance with the rules of intepretation.  This machine spec is a written description of this correct arrangement.  It could be the circuit diagram for the CPU of a computer, the soure or binary code for a program that performs this interpretation.  What ever it is, it is a static written document describing the construction/operation of this interpretation machine.

	**MACHINE SPEC LANGUAGE** -- The machine spec itself needs to be interpreted, by the manufacterer who is building the CPU, or the computer that is going to run the interpreter machine.  And in order to get this recursive interpretation loop to stop the machine spec langauge must be understood by whatever agency is performing the booting operation.


	Thus we see that at its essence "Booting" is a process for using some kind of execution according to one set of interpretation rules and using that to create execution according to some other set of interpretation rules.  In its most elemental application this underlying substrate might just be the laws of physics, and a human "compiling" the specification of some CPU into the wiring of a set of transistors.  Then booted environment is configured to interpret that specific instruction set.

	In uniform we reserve the term "boot" for the purest cases of creating new interpretational rules.  All important attributes of a uniform booted environment should depend ONLY upon the machine spec and not on attributes of the implementing substrate.  The idea is the spec should capture all important aspects of the interpretation enviornment such that any conforming substrate is as good as any other in implementing this spec.  In reality thinks like speed, reliability, stocasticity, and perhaps even mathematical precision could vary from one substrate to another.  To the degree that these attributes are important to the ultimate interpretation they should be included as constraints in the machine spec so that all conforming substrates are adequate embodiments of the desired interpretation.

	Achieving this level of specification, and then ensuring that an implementing substrate lives up to the specification are both quite daunting tasks.  The uniform framing of boot only takes the first step of ensuring these is a correct place to include these requirements and stated obligations to follow them.



	 which itself is defined in terms of graph models and the apply/reduce operators of unit calculus.
### --- LOG ---
#### 2021-07-14 - LANG.LAL.BOOT.LOAD -- run/boot/load steps

LOAD -- The creation of a new computational universe from a prior one along with a number of extensions to "load" into that universe.

	env.origin 	-- the grounding of the currently execution universe
	load.base	-- the base grounding_plane used for this load
	load.patch	-- the patch applied to generate the resulting plane



_
# ### WHERE TO GO FROM HERE? ###

Uniform aims to unify the range of software languages and frameworks into a single ecosystem DAG where each node is an element which formulates a single paradigmatic notion.  The idea is that each element is 'pure' in the sense that its formulation only mixes ideas from another element if it inherits the functionality explictly from an earlier rung in the DAG.  Further these elements are formulated so they parsimoniously interoperate in ways that allow construction of existing languages and frameworks to be constructed as a macro "powerset" combinations of these earlier paradigmatic elements of the Uniform DAG.

Not shown here is a complementary paradigm for "Code Markdown" which provides a code/data parser that is a semantics-free, language-independent reader very analogous to the LISP sexpr reader.  This reader maps beautifully presented source code forms onto a tree of uniform units.  Together these three rungs along with code markdown a basis sufficient for the construction of a large number of elemental langauge constructs.

Unlike existing software constructs, these constructs would be built from the ground up to build from common substructure as much a possible since each concept (by construction) may only exist within the paradigm DAG in exactly one place.


As an example of this, consider the Flow paradigm captures the ubiquitous notion 'control flow' as a static source-code structure that used to specify the temporal ordering of the set of iterpretation steps listed in the leaves of that structure.  The Flow.Branch sub-element builds upon this paradigm to formulate the idea of a one-to-n split in that control flow graph.  This maximally general/maximally simple Flow.Branch paradigm is then reused over and over again in any and all cases where interpretation choices are made regarding which piece of source code "happens next".  Thus both the Python try-catch logic, the method dispatching semantics of Java, along with parts of the logic behing the cut operator in Prolog must all be expressible as macro expansions built from Flow.Branch operator, since all cause splits in "happens next" graph as implied by the semantics of these languages.  Since the Flow.Branch operator semantics does not contain any extraneous fluff not ABSOLUTELY REQUIRED by the nature of branching it turns out to be a parsimonious way of expressing each realization of interpretation branching.

As a second example, consider the inheritance mechanism for Javascript, Python, and C++.  All are parsimously built on top of the Package paradigm in order to encode the semantics of how scoping works within those languages.  These scope contents in turn, are used in the expected way by the Eval and Grounding paradigms in order to produce each language's execution semantics.  In order to keep the Uniform Package paradigm pure, it explictly depends upon the Structure.Ops paradigm in order to reuse the the set combination operators found there.  In that way set union is only defined once across the entire uniform ecosystem.

At the same time, the definition of scope for each of the languages above are decomplected from their model of persistence, their module loading semantics, and argument binding semantics.  The result allows one to use python semantics to express invarient constraints persisted within a relational database, or combine prolog back tracking with javascript object inheritance.  These languages are no longer separate, but instead are part of a single 'mix and match' whole.

The more near-term value of the Uniform ecosystem is to enable the creation of "simplest possible" software components.  These components are 'embedded everywhere' components just as JSON is an embed everywhere data type.  Just like JSON these component have zero extra fluff beyond the requirements logically needed to perform their intended function.  

It is important to understand, this 'embed everywhere' capability is not possible when building from traditional (non-uniform) elements.  Any component within a traditional software stack bakes in dozens upon dozens of assumptions many of which are not logically REQUIRED by that component, and each one of those assumptions is yet another potential for incompatibility with some target usage.  Uniform elements are different, each was designed around a single defining constraint and each incorporates only those constraints logically required for their functioning, thus when building from a collection of them, one takes on no fluff, nothing except those constraints strictly required in order to provide the desired functionality.







# ### LOG ###