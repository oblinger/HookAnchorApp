
~~~~~~
THE GLUE GRAPH, GLU, HAS THESE PROPERTIES
- Begins from one specially designated vertex called ZERO.
- GLU is the smallest graph that guarantees that:
	(1) Forall A, B in GLU there exists exactly one triple  <A, B, C> in GLU
	(2) Forall C in GLU except ZERO there exists exactly one triple <?, ?, C>


- ONE -- Given the zero vertex there must be one triple: <ZERO, ZERO, X> 
	Since the graph must have more than one vertex X cannot be ZERO, lets call that one ONE.
- NATURAL NUMBERS -- Likewise there must be a distinct Y <ZERO, ONE, Y>.
	We can call it two, and thus the natural numbers are in glu.

- PATH NAMING -- 3 5 4
- RECURSING --  3 5 (4 2)
- VERTEX SEMANTICS

2 = enum-of
3 = natural numbers
4 = unicode
5 = strings
6 = lists

2 4 65 == enum-of  unicode  65th-element  == Unicode A
2 4 66 == Unicode B
...

2 5 == enum-of strings == ""   the empty string

2 5 (2 4 65) == enum-of strings (unicode A) == "A"  string containing an A
2 5 (2 4 66) == String containing a "B"
2 5 (2 4 66) (2 4 65) == the string containing "BA"




"b" = set_of characters 1

"a



Char	= An agreed upon semantics and name for one of the O### verticies


URF, a pure-blank-node representation, that unifies: quad-notation, triple-stores, URIs, triple-reification, type literals, and many other computing notions into the "unit", and uses this single structure "all the way down"

The atomic element of RDF is the triple, URI is just a fragment.  In the same way the container is the atomic element in URH, a single linkage is just a fragment.  In your mind you may think of a "Unit" as the set of triples that share the same subject, but I want you to flip your perspective.  Try to think of the Unit itself as the atomic element within the URF system...

## Pat Intro

Pat, 
It turns out that HUGE amounts of RDF was still in my head, and not on paper!  But you really put a bug in my bonnet.  So after 7 days of non-stop writing I have a good chunk of the skeleton written.  If I later decided this was worth going somewhere with, then I can go back to actually write this up properly.  still I think (or hope) it is mostly understandable in is present form.

URF is really just one piece of larger thing I call UNIFORM.  So my focus has never been to try to get this one URF piece correct.  Still it is pretty well understood, and perhaps it is a nice little nuggest all by itself?

~~~~

A bit of context:  I have no beef with RDF as a DATASTRUCTURE---it is quite beautiful.  But RDF attempts to treat the triple as something more, it tries to treat it as the UNIVERSAL ASSERTION---as a smallest isolatable statement with an isolatable independent semantics over which one may perform processing and inference.

This is where I think the train jumped the tracks:  A key requirement for an inferential framework is that its statements are first class objects which can be referenced themselves --- otherwise one cannot fully specify the inference system.  Even if we define a container object in RDF, there is no way to put the triples INTO that container since we cannot reference them!  So triples beautiful as they are, do not serve well as assertion form since they have no referenceable handle.

~~~~

I think my goals for URF are different from my understanding of goals of RDF.
My aim is a uniform encoding of computation.  

This is too complex and too illunderstood for me to express succinctly.  Instead let me offer this framing:  take standard libraries for all popular programming languages as a proxy for "the data structures underlying modern computation". URF encodes all of these structures using a single "container" type.  So list, map, set, ordered-map, multiset, tree, DAG, unbounded-series, graph, queue, stack, etc. should all be encoded using the same "unit" structure 
"IN-A-UNIFORM-WAY"

Roughly this mean in a way that unifies membership tests.  So imagine writing the code to test is X is a member of Y.  Only later do I meantion that Y was an ordered collection of X, or Y is a multiset, or Y encodes a key/value mapping onto its element values, etc.  We want that membership check to be uniform over ALL of these variations.  Which means the structure used must be uniform over these different options.  You can actually do this using RDF -- but it requires a very stylized form of RDF that uses 4 triples for each linkage.  URH bakes this in, by making the container-object the atomic unit.

Both RDF and URF can encode any graphical model, but URF encodes "datastructure-like" models more uniformly.



Several key ideas in URF:

1. _URIs_.  Treating URIs as internally-structured, semantics-free placeholders whose **_ONLY_** purpose is graph alignment.  

2. _Universal handle notation_.  I argue that a good tool needs a good handle to guide its use.  Thus a language of logical assertions NEEDS to have first class handles for those assertiona (not bolted on!). I think RDF has multiple such bolt-on notions: (1) Reification, (2) quad notation, (3) triple stores, and (4) URIs all come from this lack of proper handles.

3. _URF stance on denotation_.  "There is no denotation without interpretation." Thus I very intentionally leave out any denotational meaning for URF graphs.  I believe "data is what data does".  I don't disupute the value of having a denotational semantics in some context, but I think we falsely pretend RDF has a single universal denotational semantics which applies to all triples.  In reality we often use RDF as a data-structure where most all of its semantics are actually implicit within the code that is used to operate over those structures.  that code is its REAL semantics.

	URF instead acknowleges this reality.  In URF one settles for making semantic annotations EXPLICIT within the URF graph.  The denotational meaning of those tags are usually implicit.  (one could even have a RDF tag that indicates that some units should be interpreted using RDF triple sematnics, among others). But strongly grounded graph will reference a full functional semantics for their units, again explicitly within the graph.  (This full functional grounding requires all of the machinery from Uniform... so I don't include it in this doc.)


with out further ado:

_
## === URF - Uniform Relational Form ===
### _

**TL;DR.  URF is an RDF alternative that more parimoniously encodes the range of data-structures commonly employed within software langauges and frameworks.**

_
### --- VOCABULARY ---

**UNIFORM RELATIONAL FORM** -- _Uniform Relational Form (URF)_ encodes a data graph as a set of 'unit' mapping functions.  

**UNIT** -- A _unit_ is a vertex within a URF graph.

**COLL** -- A _collection_ is a unit vertex that has "structure".  The term _structure_ refers to a sequence key-value pairs associated with a vertex).

**LITERAL** -- An _literal_ is predefined vertex that can be used to align graphs with each other.

**BOUNDED** -- A unit is _bounded_ (also called _printable_) iff it is a literal or a finite collection of bounded units.

**PLACE** -- A _place_ is an element within the structure of a vertex.  
- Intuitively it represents a single "spot" (triple) within the graph.  
- A place is composed of three unit parts called: _self_, _key_, and _value_.  
- Each place can be thought of as an arc within the URF graph.
- The "Self" unit is the collection vertex itself. "Key" is the arc's labelling vertex, and the "value" is its destination vertex.
- NOTE:  The name "self" is chosen because of its eventual connection to OO-method invocation, the term "place" is chosen becuse of its eventual connection to assignment semantics, and "key"/"value" are chosen because of their eventual connection to the MAP data structure.

**TRIPLE** -- A place is also sometimes called a _triple_, and its three unit parts are called _subject_, _predicate_, _object_ respectively.  A URF vertex is a DATASTRUCTURE that does not have a single universal semantic.  So we generally use the place/self/key/value terminology in von-Neumann-like processing contexts, and triple/subject/predicat/object terminology in assertion-inference contexts.

**GROUNDING** -- A unit is _grounded_ by a triple using a special "grounding" or "head" predicate to links a unit to a "grounding structure" that meta-data and/or functional semantics for the unit.

_
### --- DEFINING URF GRAPH STRUCTURE ---

Formally a URF graph G = **<** **U**, **_items_** **>** is defined as:
	
Let **L** be a set of verticies called _literals_.
Let **D** be a set of verticies called _data_, 
	where **D** and **L** are disjoint sets
Let **U** be the union of **L** and **D**.  (collectively called _units_.)
Let **P** be a subset of **U** called _printable units_. (defined below)

Each _u_ in **U** defines an ordered mapping relation **items(_u_)**.	
	**items(_u_)** is a possibly unbounded, possibly unordered sequence of pairs:
		(K1, V1) (K2, V2) ...   where _K_i_ in **P** and _V_i_ in **U**
	
A unit, _u_, **_is printable_** iff _u_ is in **L** or 
	there exists a natural number, _n_, where
		items(_u_) == (_K1,V1_) ... (_Kn_,_Vn_) and 
			for all natural numbers, _i_, where  _i_ < _n_   
				_K_i_ is printable and _V_i_ is printable	

__
We say a graph, G, is _**controlled**_ iff it contains a single root, R, that itself is not the object of any triple in the graph, and whose closure contains the entire graph, G.  Otherwise we say it is _**uncontrolled**_ if it is not known to contain such a root.  If no indication is made, we assume a URF graph is controlled by default.  Formally:

We say the set of vertices, S, is the **_closure_** of vertex, V given graph G iff
	S is the smallest set which contains V and 
	Forall g, h in G, and Forall s in S
		h in S  if  <s g h> in G



COMMENT:
	The master asks: "A tree falls in a forest w/o humans does it make a sound?"
	The URF answer: "we avoid the question by disallowing forests to exist w/o humans... after all what would be their purpose be?"

	Said in another way:  
	1. URF only concerns itself with that which might be computed.
	2. Data cannot be computed over if it cannot be accessed.
	3. URF is explict, even the meta data used for access should be in graph.
	==> Therefore at a minimum, URF only operates on controlled graphs of data,
		since it would be impossible to express the usage of some data without being able reference that data



[[	WET INK
	PAT, this notion of control might not be quite strong enough yet.  
	I am trying to capture the minimal assumption required for data that can be "handled" see the handle discussion towards end.  It feels more natural to have some kind of DAG of owndership containders, but each encoding I consider seems overly restrictive.  The notion of "control" is certainly necessary for well "handled" data, but it is not sufficient, and perhaps there is some other more restrictive definition that is both necessary and sufficient.  the whole idea is a bit fuzzy for me.]]


{[NOTES FOR DAN (feel free to ignore)
- right now we limit unit keys to be bounded unit, this it to ensure decidable key matching, but perhaps this is overly restrictive? ]} 


_
### --- Semantic Grounding ---

[[WET INK:  Pat, URF is really the data-structure part of Uniform which
	constructs it own semantics directly on top of URF, so:
	URF
	TRS (term rewriting with rules and atoms expressed in URH)
	UCALC (Unit Calculus -- a predicate calculus clone expressed in TRS)
	ASM (A set of 9 primatives expressed in UCALC)
     ==> the idea is that URF functional semantics would be expressed 
         as an assertion in this ASM language.



**SIMPLE SEMANTIC NOTATION**



URF units are semantically grounded via explicit links in the graph.  When a unit is treated as an assertion its 'head' link is used to link a unit to its grounding structure.  In uniform this become a fully computable, recusively defined, semantics.

In isolated URF, we may annotate with the following core type annotations.  Each of the literal verticies listed below are conserved across all graphs.  (See section on GLU graph for literal structure conserved across graphs.)

head = lang.uf.head
ident 
Int = lang.uf.int              # these are the four atom types in the GLU
Num = lang.uf.num
Str = lang.uf.str


_
### --- UFORM - UNIT FORM ---
#### _

[[Pat, this is an optional section... it is just about having a print format and a transport format for URH]]


There are data formats defined for URH.  The first expresses bounded, tree-structured URH as nested text structures, and the second expresses the all-blank-node structure of a URH graph as sequence of integers.

**UNIT-FORM** -- _Unit form_ is a simple lossless textual representation of bounded tree-structured unit data

**UNIT DATA BLOCK FORM** -- The _unit data block form_ is a URF graph expressed as a sequence of integers.   


Unit Form Examples

	age(dan, 54)
	person(dan, age:54, address: address("101 bache", zip))


[[Pat, there is a full "code" markdown, which maps expected source code structures (like blocks, and '.' notation onto URH) but I am going to keep that out of this URH document... for for here just know there is a nice way to map pretty natural looking code structures onto URH in a semantics free way (like the LISP reader is sematantics free).  But this markdown looks much nicer than lisp, basically it looks like python.]]


_
#### -- Data Block Form --

**Data Block Form** -- _Data block form_ is a URF graph expressed as a sequence of integers.  
- This sequence is divided into subsequences by the 0 (zero) integer
- Each subsequence is a sequence of key/value pair integers.  
- All integers except zero are a reference to the vertex encoded by the nth subseuence

##### - Conventions that simplify human presentation block form data -

Data block form is just a list of integers that code for a graph which can be merged with other graphs.  For human presentation it is helpful to provide a few hints that make this data block a bit more understandable even in its data block format:

- **LIST** -- For human presentation square brackets "[ ... ]" can be used if the remaining keys for a unit are 0, 1, 2, ... .  Then only the values are shown, the keys are omitted.

- **IDENT** -- Human presentation is often aided by replacing integer values with "blank node identifiers--traditionally an identifier prefixed by an underscore ("_") char.  When using ident labels, we must being each each key/value pair sequence with a single label which indicates the identifer short hand for the integer cooresponding to this row.

- **ATOM** -- For human presentation, GLU Literals are expressed as an identifier that is NOT prefixed by an underscore "_".  These implicitly reference the cooresponding vertex within the GLU graph.  Again each of these literals denote the integer cooresponding to their GLU literal.

- **GLU** -- By convention negative integers are used to indicate nodes in the common GLU graph (since this common graph is typically not stored as part of most data block encodings, it is nice to treat them as negative integers so they do not change numbering of the rest of the graph.


For example:




_origin	_origin _origin 0			# the GLU graph origin

15		_head _int		0 15		# an implicit entry in GLU that defines 15
foo		_head _ident    [ f o o ]	# implicit identifier entry in the GLU
f		_head _char		0 102		# this GLU was built on unicode numberings

head	0
_001	^head



TEXT		IS SHORT HAND FOR
1234		_n1234	_head	_int	0 "1234" .
12.34		_n12_34	_head	_int	0 "12.34" .
"foo"		_sfoo	_head	_str	0 "foo" .
foo			_ifoo	_head	_ident	0 "foo" !

_head	_head _ident	0 "lang"	1 "uf"		2 "head"	!
_ident	_head _ident	0 "lang"	1 "uf"		2 "identifier"	!
_num	_head _ident	0 "lang"	1 "uf"		2 "number"	!
_str	_head _ident	0 "lang"	1 "uf"		2 "string"	!
### --- GRAPH MERGING ---
#### _

A key URF claim is that it expresses all information as 100% pure graph data.  At a practical level this means replacing opaque URIs with explicitly structured unit-form literals instead.  But at a theoretical level this is not fully satifying, since it seems the literals of URF graph still have latent information in them -- it seems they still have some kind of textual atomic information in their nodes, just like URIs do.  

Did we just push the bubble around under the carpet?  
Here we show we did not, we show we have smashed that bubble GONE!

- LITERAL BASED ALIGNMENT -- 
	URF graphs support unambiguous alignment of separate URF graphs since they use the SAME literals in all URF graphs.  
	
- GLU GRAPH  -- 
	This is accomplished by defining a fixed GLU graph (a Graph of Literal Units).  This infinite, static graph constant is implicitly contained within all URF graphs, that way multiple URF graphs may reliably linked by aligning the GLU graph vertices that implicitly exist in all graphs.
	
- GLU GRAPH CONSTRUCTION -- 
	Here we specify the construction of a GLU graph as a specific RDF graph.  The precise structure of the GLU is not critical, only that:
	- It contains ONLY blank nodes.
	- It contains sufficent diversity of literals to support alignment.
	
- LITERAL REPLACMENT -- 
	We describe how one may take URF graph and replace its literals with vertices within the GLU in order to generate a _pure URF graph_ -- a pure blank node RDF graph.  The resulting structure is "graph only" it has no URIs at all!! It is just a wired up mass of blank nodes.

- WE DESCRIBE URF GRAPH MERGING AND BLANK-NODE-GRAPH-UNIFICATION --
	URF Graph merging matches literals in order to merge graphs.
	Blank node graph merging describes a parameterless process of aligning two pure graphs.
	
- "LIFTING LEMMAS"
	Here we prove given two RDF graphs R1 and R2, their litteral based alignment, R3, is equivelant to the blank-node-unification of their pure (lifted) URF forms.

_
#### -- GLU GRAPH CONSTRUCTION --

The GLU graph (Graph of Literal Units) is an infinite, static mathematical object; it is the graph containing the set of all uniform literals used for merging URF graphs.  This GLU graph is implicitly part of every URF graph.  Thus linking to verticies in this graph serves as a kind of "glue" indicating how corresponding vertices within separated URF graphs should be connected.

Secondarily, GLU literals are like any other (bounded) graph nodes within the data.  Thus they may have an explicit static internal graph structure defined over them.  Thus when a graph intends a linkage vertex which has specific internal structure an appropriate GLU vertex may be chosen.  For example, if a URI is contained within the subdirectory of another URI, one might choose to encode this relationship explicitly within the RDF graph--often it is not done, because it is a big messy.  In URF this can be done trivially by choosing URF literals with correct structure.  So:  path("foo", "bar") and path("foo", "bar", "baz").  In this case they will automatically be related by the expected internal structure as expressed by structure within the GLU graph itself.

_
#### -- Functional Graph Alignment --

The "Functional graph alignment" of two graphs is a trivial "zippering" of two graphs togther, given some initial point(s) of alignment between those graphs using cooresponding functional triples in each group to grow the alignment.

The "min zip" of a graph is the subgraph containging enough structure to reconstruct a given set of targeted vertices to be aligned.  (The "min zip" is a kind of "tree shaking" algorithm capturing the substructure needed for unambiguous alignment.)


An set A is an _**alignment**_ of two graphs G and H iff 
	it is a set of pairs {<g_i, h_i>, ...} where g_i in G, h_i in H.


An alignment A is _**conflicting**_ iff there exists   dup, x, y   where x!=y and
	<dup, x>, <dup, y> in A   or   <x, dup>, <y, dup> in A


B is a _**functional graph alignment**_ of graphs G, H, given alignment A iff

	B is the smallest alignment that contains A, 
	B is not conflicting, and
	for all X and Y in A, if <X,Y,g1> in G and <X,Y,h1> in H then <g1,h1> in B


Graph G is the _**gluing**_ of graphs G, H at alignment A iff
	A is a non-conflicting alignment of G and H
	Z is the smallest graph containing G and H' where 
		H' is H with all h_i replaced with g_i for <g_i,h_i> in A


Intuition:  The '_tree shake_' of a graph is a smallest subgraph that reliably aligns the targeted verticies given the starting vertices.

The graph M is a **_tree shake_** of graph G, for vertices T, and S iff
	T is a set of "target" vertices in G,
	M is a subgraph of G containing all  t_i  in T,
	S is a set of "starting" verticies in G,
	A is the alignment of M and G given S
	Forall t_i in T   the pair  <t_i, t_i> is in A.
	There is no tree shake M' of G where M contains M' 

_
#### -- Constructing the GLU graph --

Here we describe the construction of a _Graph of Literal Units (GLU)_---an infinite, static mathematical object that provides the structure that underlies URF literals.  The idea is that every URF graph implicitly contains a subset of the GLU and that subset is used to "glue" the URF graph to all other URFs graphs.

- The GLU is an infinte graph structure built entirely from blank nodes.
- A subset of this graph is implicitly included into each URF graph.
- Each link to a literal within a URF graph is essentially a shorthand notation for a link to the corresponding vertex within the GLU.
- Thus each URF graph built with literals is really short hand for a pure URF graph that is constructed entirely of blank nodes (having no literals).
- The URF merge equivelancy theorems prove that "shortcut" literal merging is always equivelant to pure blank node graph merging.
- The details of the GLU graph below are not so important, what IS important is that there is only one way for the GLU graph to align onto itself, and the GLU graph somehow or another constructs exactly the literals we hope to use for graph alignment.
- The GLU only has unordered unit literals, but one could create a different GLU that had ordered literals.


NOTE #1:  Before we begin this theoretical excursion it should be noted in practice this graph is never actually constructed.  Instead any graph vertices that indicated to be literals by their grounding, are printed into a string, and these strings are matches against other graphs exactly as on might match URIs.  the only difference is that URIs have some hacky form restrictions and no good internal structure, while URF literals may have any structural internals the suit your domain.

NOTE #2:  The specifics of my GLU graph are not so critical here.  Think of the glue graph a just a bunch of blank nodes which are conserved between URF graphs, so you can use them for aligning nodes in separated graphs.  

NOTE #3:  Big theoretical difference.  Unlike URIs these GLU literals don't have any semantics... they are more like the number 7, they just represent themselves.  There only purpose in is One could agree on a different structure of glue vertices and the meaning of the graphs would not change.


Lets get started constucting the GLU.  Unless specifically indicated otherwise each constructed blank node is unique and does not correspond to any other constructed blank node.

**ORIGIN** -- The GLU graph has exactly one origin vertex.  It is the only vertex that has a triple in the O:<O,O> where the origin, O, is the subject, predicate, and object of the triple.  The origin is unique, no other vertex in any URF map may link back to itself in this way.

**SUCC** -- The successor node is a different unique, special blank node within the GLU.  The succ of a GLU graph is S != O, where O is the origin of the graph and O:<O,S>.  The successor vertex is the only vertex linked to from the origin.

**SEQ** -- the GLU has an infinite number of nodes which correspond to the natural numbers: 0, 1, 2, ...  The ORIGIN node corresponds to the natural number 0.  The SUCC node corresponds to 1.  For each natural number i there exists a single triple N_i:<SUCC,N_i+1> in the GLU which indicates the "next" at N_i+1 is the "next" blank node cooresponding to the next natural number.

**TABLE OF GLU DEFINING VERTICES**
The follow table lists the speciality "meta predicate" verticies within the GLU. Each one has a nickname associated with it, which indicates its usage within the GLU, and each one is associated with a distinct position within the SEQ sequence

	0	**origin**		< **O O O** >			Specifies the beginning of the nats
	1	**succ**		< seq **succ** seq >	Specifies the nat progression
	2	**gnd**			< unit **gnd** g >		Specifies that 'g' ground 'unit'
	3	**head**		< u **head** type >		Specifies a unit's type
	4	**zero**							Indicates Nat-0 and Frac-0
	5	**bit_add**		< seq **bit_add** ba_i>	Predicate that add 2^b to nat
					< nat1 ba1 nat2 >	Specifies other int vertices
	5	**frac_add**	< seq **frac_add** fa_i> Predicate that add 2^-b to frac
					< frac1 fa1 frac2 >	Specifies other int vertices
	6	**negate**		< seq **bit-** sub-er >	Predicate that subtracts 2^b from int
	7	**num_left**	< int **num_left** fl > Maps each int to left part of a float
	8  	**num_right**	< nat **num_left** fr > Maps nat to decimal appender predicate
					< fl fr num >		Each float num has left and right
	9   **empty_str**   					Vertex indicates the empty string
	10	**ch_app**		< nat **ch_app** a1 > 	Predicate that appends char to string
					< str1 a1 str2 >	Each appender will add a particular char
	11	**empty_unit**						Denotes an empty unit
	12	**add_key**		< unit add_key k1 >	Selected unit to be used as key
	13	**add_val**		< unit add_val v1 >	Selected unit to be used as value
	14	**add_pair**	< k1 v1 p1 >		P1 will add key/value pair to unit
					< unit1 p1 unit2 >	Units are built one pair at a time.
	15  **literal**		< nat **literal** lit >	Indicates all literals in the GLU.




CONSTRUCTION STEPS:

CREATE ORIGIN
	Create "origin" blank node, O, and add triple  <O O O> to the GLU.

CREATE SUCC
	Create the "succ" blank node, S, and add <O O S> to the UAG.

CREATE ALL SEQ VERTICIES
	Create blank nodes N2, N3, N4 ... then add these triples to GLU:
		<O S S>  <S S N2>  <N2 S N3> <N3 S N4> ...
	At this point we have created the 'SEQ' sequence, and the first 16 elements of that sequence are the meta predicates listed in the table above.
	
CREATE BA_i (POSITIVE POWER OF TWO ADDER) VERTICES
	For each 'SEQ' element we create a predicate that adds 2^SEQ to a NAT

CREATE FA_i (NEGATIVE POWER OF TWO ADDER) VERTICES
	For each 'SEQ' element we create a predicate that adds 2^-SEQ to a FRAC

CREATE POSITIVE POWERS OF TWO AS NAT VERTICES
	For each ba_i vertex we add the triple < zero ba_i nat >
	These are the powers of two expressed as a natural number (NAT)

CREATE NEGATIVE POWERS OF TWO AS FRAC VERTICIES
	For each fa_i vertex add triple < zero fa_i frac >
	These are the negative powers of two expressed as a fractional number

CREATE REST OF NATURAL NUMBERS (NAT)
	Forall _nat_, Forall _ba_i_ add  <_nat_, _ba_i_, _nat2_> to GLU  if
		_ba_i_ is smaller than any _bit_ used to construct _nat_

CREATE REST OF FRACTIONAL NUMBERS (FRAC)
	Forall _frac_, Forall _fa_i_ add  <_frac_, _fa_i_, _frac2_> to GLU  if
		_fa_i_ is smaller than any _fa_i_ used to construct _frac_

CREATE NEGATIVE NUMBERS
	Forall _nat_ except zero add triple to GLU:  < _nat_, neg, _neg_int_ >

GROUP INTEGERS (INT)
	The set of INT verticies is the union of NAT and NEG_INT vertices.

CREATE DECIMAL NUMBERED VERTICIES (NUM)
	For all int_i in INT  we add triple  
		< int_i, num_left, nl_i >  into the GLU and into NUM_LEFT
	For all frac_j in FRAC we add triple  
		< frac_j, num_right, nr_j >  into the GLU and into NUM_RIGHT
	for all nl_1 in NUM_LEFT, for all nr_j in NUM_RIGHT  we add triple
		<nl_i, nr_j, num_ij>  into the GLU and into NUM

CHARACTERS
	Forall nat_i   add  <nat_i, ch_app, char_appender_i> to the GLU
	These predicates indicate their value string is the same as their subject string with one specific character added.

STRINGS
	Forall str_i, and Forall char_app_j  add  <str_i, char_app_j, str_k>
		where str_k is the str_i with char indicated by char_append_j added

UNIT
	UNIT is recurively defined to be the set of vertices in the INT, STR, NUM, and COMPOSITE_UNITS set.

PAIRS
	For each unit, u, we recursively add these triples:
		< u_i add_key ak_i >
		< u_j add_val av_j >
	
	For each ak_i, av_j  we add  <ak_i, av_j, pair_ij> to the GLU

COMPOSITE_UNITS
	The EMPTY_UNIT is added to this set of verticies.
	For each composite, C_i, and pair, P_j, we add
		<C_i P_j C_ij> to the GLU
	(Each of these new vertices C_ij represent a new recursive unit expression)

LITERAL
	For each unit, u, we select some nat_i in NAT and add the triple:
		<nat_i, literal, u>
	These triples give us an enumeration of all of the literals with the GLU.

ATOM
	The set of vertices designated as atoms:
	ATOMS := INT + NUM + STRING

**UNIT FORM (PRINT FORM)**
Each atom has a _**print form**_:

- NAT -- Each NAT has a print form encoded as a sequences of digits 0-9 that correspond to the number obtained by adding the specified powers of 2.
- NEG -- Each NEG has the print form for its corresponding NAT with the "-" charater prepended to it.
- NUM -- Each NUM has the print form of its INT part appened to the "." char followed by digits 0-9 that coorespond to the fractional number specified by adding the negative powers of 2.
- CHAR -- The print form for a CHAR, c, is some character encoding for the nth character.  the print form for the (") char is (\").
- STR -- Each STR has a print form of a '"' chars '"'   where chars is the print form of each character within the string.

_
#### -- Attaching the GLU to a URF graph --

Every URF graph is really short hand for an "all blank node" graph that implicitly includes an appropriate tree shake of the infinite static GLU graph as part of its structure.  

- URF allows any bounded structure in a URF graph to serve as a "Literal".
- Each of these literals _is shorthand for_ a specific GLU vertex.
- The subgraph structure for each literal is parallel to the subgraph structure of the GLU vertex it is shorthand for.
- Each URF graph G, is really shorthand for G' the blank node conversion of G.
- All URF graphs can be unabiguously "glued" by alignment of their GLU vertices.

The URF graph alignment theorem states that URF graph gluing is equivelant to URF graph alignment by matching the unit-form print strings of their URF literals.

So in practice URF is just like RDF with URIs except URF literals 
- may utilize the full representational power of URF to describe the internals structure of its literal "URIs"
- URF graph matching is provably equivalant to pure blank node graph matching.
	(which is really another way of saying that URF graphs are matched based upon the INTERNAL structure of their URI literals as expressed as a graph.)


Formally:

A URF literal L _**is shorthand for**_ a GLU vertex V iff
	V is an ATOM and the unit-form of L matches the print for of V, or
	forall i, there exists val_i, lit_i where
		<key_i, val_i> == items(V)[i]   and   <key_i, lit_i> == items(V)[i]  and
			val_i  matches  lit_i

The alignment A is the **_GLU alignment_** for URF graph G iff
	A is the smallest alignment where
		For every literal L_i in G, there exists V_i in GLU such that
			<L_i, V_i> in A   and   L_i is short hand for V_i

_**sticky**_(G) is the **_GLU expansion_** of G iff
	L is the set of literals in G, 
	S is the tree shake of GLU for L starting from ORIGIN
	A is the GLU alignment for G,
	_**sticky**_(G) is the gluing of G and A at alignment A

**_strip_**(E) is the _**GLU stripping**_ of G iff
	E is is sticky(G) for graph G

INTUITION:  
- "GLU expansion" removes each URF literal from a graph and replaces it with the corresponding vertex in the GLU.  The substructure of the original literal is preserved in the resultin glued graph, but now the graph ONLY contains blank nodes.
- We use "tree shake" to select a finite subset of the GLU graph so the resulting graph is "proportional" to the original graph in size (see below)


_
#### -- URF Graph Gluing --

As we saw above, URF graphs are actually short hand notation for their sticky expansions.  Here we see that URF graph merging is accomplished by gluing their all-blank-node sticky forms together.  Formally:

O is _**the origin of**_ sticky(G) iff
	<O O O> in G, and
	there does not exist X, Y such that <X, Y, O> in G

M is the _**URF graph merge**_ of graphs G and H iff
	A is { (g0, h0) } where g0 & h0 are _the origin of_ sticky(G) & sticky(H),
	A2 is the _functional graph alignment_ of sticky(G) and sticky(H) given A, and
	sticky(M) is _the gluing of_ sticky(G) and sticky(H) at A2

_
#### -- Properties of the GLU graph and graph alignment --

[[If it ever became worth doing I think can prove all of this stuff]]

Here are some important properties of the GLU graph above:

1. ORIGIN IS UNIQUE -- The GLU expanion of any closed URF graph G, will have exactly one ORIGIN vertex.  e.g. exactly one vertex that 
	- is not the object of any triple in the expansion, and 
	- the only one with the triple <O O O>

2. GLU IS FUNCTIONAL -- Once you remove the <O O O> origin triple from the GLU the remainder of the GLU graph is functional.  That is, there is at most one node, Z, for any X, Y where <X, Y, Z> in GLU.

3. ORIGIN ALIGNMENT OF STICKY FORMS WILL ALWAYS ALIGN THEIR LITERALS
	Given #1 and #2 above, the structure of the GLU graph, and the nature of tree shaking, any functional graph alignment of any two GLU expansions given a starting alignment of their origins will aligns all of their literal verticies.
	
4. SKICKY FORMS NEVER ALIGN MORE THAN THEIR LITERALS.
	The GLU is specific designed so that functional alignment will "follow" the "pathways" laid down by the steps of its creation, but since URF graphs never allow a literal to exist as the subject of a triple there is no way for the alignment to continue into the joing URF graph, it will stop at the literals.
	
5. RESULT OF GLUING TWO STICKY FORMS WILL ALWAYS BE A STICKY FORM
	It will be a little messy to prove, but the graph gluing surgery will always yield a new graph that is a sticky form since it has the GLU attached in "all the right places"
	
6. THE SIZE OF STICKY(G) IS UNDER A LINEAR FACTOR OF BITS TO ENCODE G.
	We took care in the GLU to use a power of 2 encoding not the traditional peano's-axiom-like encoding of the numbers and characters.  This ensures that the size of the graph needed to encode a number or string atom is proportional to the number of bits needed to encode that same atom in non-graphical form.

_
## === ONTOLOGICAL ISSUES ===
### _

 [[PAT, I was confused for a long time about the relationship between URF and RDF, so I wrote alot about this to help me clarify it to myself... Thus things got a bit wordy in here. feel free to skim...

Also re-reading these, I see they are said more arrogantly and more conviction than I have when re-reading them.  Still they do indicate the different kind of thinking that caused me to end up pushing away from RDF... (I tried for along time to just use RDF, and these sections to capture some of the failures I was finding.) ]]

_
### -- THE RDF DENOTATION FANTASY --

RDF's "Denotational emperor" has no clothes.

We argue that, in practice, RDF is used as a data structure.  Indeed sometimes with violent disregard to any supposed denotational semantics those triples might have.  We argue this denotational semantics is mostly a fiction in RDF -- really it is a data structure, and it real semantics lies implicitly within the algorithms that are designed to operate over that structure.    

URF, on the other hand, is unapologetically just a DATA STRUCTURE and nothing but a data structure.  That said, the larger Uniform ecosystem built on top of URF does provide a semantics for URF.  But there are some key differences:

- NO ONE SEMANTICS -- In URF we officially sanction what happen in RDF.  Programmers will treat URF as a data structure and then will invent their own different semantics to govern usage of different kinds of URF.  We are totally cool with that.  There is no one over-arching denotational semantics one is doing violence to.  But one can explicitly notate if some portion of URF should use RDF triple semantics, FOPC, or any other.
==> Any programmer (or commmunity) can create and use their own semantics.
	
- EXPLICIT GROUNDING -- Because there is no one correct semantics, it does mean we need to somehow notate which semantics should be applied to each piece of data.  We need to somehow "ground" the data to a specified semantics.  This is done ***EXPLICITYLY*** in URF with grounding links.
- In the shallow end of the pool these grounding links are just opaque identifiers that programs and programmers use to understand and keep track of how to interpret each piece of data.
- In the DEEEEEEEEP end of the pool the larger Uniform ecosystem pulls itself up by its bootstraps.  It provides the tools needed to begin from scratch (from an Abstract Rewrite System) and define multiple-levels of langauges needed to encode the semantics of the graph networks themselves directly ENTIRELY WITHIN IN THE GRAPH NETWORK.
- In the deep end of the pool we require the specification of a large static structure (like the GLU) which encodes the multiple levels of languages needed in order to specify the semantics themselves.  This tour-de-force activity is mostly academic, but it does serve to ensure common understanding of the semantics, and it forces the semantics to be a simple as possible, in order to keep its dependencies from exploding the size of the required structure.


~~~~~


SOCIETAL CURATATION OF URIS IS GOOD BUT IS NOT A DENOTATION FOR TRIPLES 

A key idea underlying RDF seems to be the idea that if society carefully curates and agrees upon a list of URIs it will enable effective data management since the meaning of triples, groups of triples and merged grouping of triples all have well defined meaning.  

The promise of RDF:  conflict free data merging!

We argue the conflict free merging is real, but the actual sematics is mostly a fantasy, even if URIs have a denotation (which they mostly don't) the triples DEFINITELY do not have one based on this.  (see known issues with Qualification Problem.)

Still there IS value in social curation of predicate and object URIs to be sure.  But we claim the idea that this is curation and associated default sematnics is somehow a universally applicable semantics or sufficient semantics are both false.

Its not enough to match predicates and objects up correctly, one needs to apply the correct data for the correct processing to arrive at the correct result.  (If you doubt this, just merge all the triples data encoded in your data lake into one big triplestore and then rerun your queries on that!  Denotational semantics suggest this should work.  BUT IT WONT!)

Alas, contrary to RDF semantics, the RDF triple alone does not define its own meaning.  One needs to manage the triples allowed into a particular triple stores for a particular operations in order to achieve a desired result.  Unfortunately all of this crucial meta knowlege is _implicit_ in the procedural code used to build and use the triple store... It lives outside the universe of encodings of the bare triples themselves.

YUCK!

Our solution is to first admit this reality -- the emperor has no clothes.  Becuase of the qualification problem even when we have some semantics (denotational or otherwise) we _still_ need to group and manage data so it gets applied to the correct problems in the correct ways.

URF acknowleges this and thus
1. "bakes in" mechanisms for grounding all assertions, to track their semantics
2. provides "handles" to effectively control their application.

The RDF community has developed three mechanisms to deal with this gap: triple reification, triple's "quad" notation, and triple stores.  Uniform merges all three into a single uniform treatment:  composites as knowlege buckets.

_
### -- HANDLES DISCUSSION --
#### Tool Intro
##### _

TWO BACKGROUND IDEAS KEY FOR THIS INTRO.  THE IDEA OF
- DATA -- Information with a purpose, and 
- SPIRAL -- Encode ideas as simply/non-redundantly as possible.


A GOOD TOOL NEEDS A GOOD HANDLE

The quality of a tool is measured in how well its usage outcomes match intent the of its user, and its the tool's handle that allows the user to guide the tool toward those outcomes.

In a similar way, the quality of data is measured in how well its data processing outcomes match the intent of its user.  And in a similar fashion it is important that data has a good handles by which outcomes using it may be guided.  

The RDF triple is a beautiful encoding of data, maybe it is the most profoundly beautiful encoding humans have yet devised.  And maybe this perfection is also it greatest flaw: they were forged without ANY HANDLE to hold on to.  One can assert a triple < seven prime true > but there is nothing to hold onto.  It is just a triple out there somewhere among the infinite graph of all possible triples.

To salvage this situation the RDF community bolted a couple of work-around handles onto the purity of the triple in order to be able to wield it:  The triple store, quad notation, triple-reification and URIs.  Each, in there own way, help provide a handles---methods for wielding triples in service of some data processing task.

The are each quite UGLY and quite a failure for RDF when you consider them.  We invented this most beautiful, most-general, most-all-purpose data form.  And the first thing we needed to do was to create a bunch of ways of wielding that form all of which DO NOT FIT INTO the "universal" form.  Super UGLY.  Total FAIL.
	
We propose that all data be expressed in a uniform RDF Form, one that has handles baked in from the start, and one that unifies all of these bolt-on handle types into a single uniform data control that is both more powerful and simpler that these RDF bolt-on work arounds it replaces.

(In URF the DAG of control verticies form a uniform way of referring to a collection of data.  Each control vertex denotes the subgraph indicated by the DAG, and anytime one wishes to refer to a collection of data it is always done via a control vertex.)

_
### -- DIFFERENT USAGE CONTEXT --

The RDF community seems most focused on curation of inference over a "semantic web" of data.  URF by contrast is aiming to be a parismonous substrate for all of computation, so it aims to be a good representation of the memory state of a running progam for example.  Both representations are sufficiently flexible to allow serving either goal fairly well.  Still the difference results in very practical differences in the realization of these two paradims.  

It seems the RDF's triple store can best be thought of as a database of assertions which one can process.  

The URF's grounding of the unit can best be thought of as an API from which the very structure of the units grounding by that API come to exist.  As a result URF often deals with infinite graphs which are defined in terms of other graphs via explict functional transformation.  (in the way that functional programming languages define their semantics.)

One can think of URF as providing functional (and imparative) programming with a graph model substrate from which it is built.  With URF we aim for a cleaner, better grounded model of the semantics for computation.  It seems this is just a different aim than most within the semantic web community have.

That said, our ULTIMATE aim in providing a graph model semantics for all of computation is to come full circle and provide a solution to the semantic web task of community sharing of knowlege too.  This full circle solution provides framework honoring what we see as a more honest approach where all denotation is functionally defined -- but in this larger context it is EXPLICITLY DEFINED and EXPLICITLY LINKED to the data it serves.  There is a practical upshot to all of this explicitness, it becomes easy and natural to define structures with specialized denotational meanings that map onto the actual semantics of the world.  For example one can build into the URI structure for a filesystem the rules of filename matching so that denotational equivelance matches the filesystem name equivelances that actually exist.  

_
### --- A LA CARTE ---
#### -- FUNCTIONALLY DEFINED --
	Uniform Triples have two pragmatic differences in usage:
	- **FUNCTIONALLY DEFINED** -- No inherent denotion.
	- **EPHEMERAL** -- Often RDF triples are operated on as realized DB entities or in memory within a triple store, while Units only exist ephemerally within an interpretational context.  Outside of an interpretational context there is only structure w/o function or meaning.
#### -- ISSUES WITH RDF IMPROVED BY URF --
	Problems with RDF that are improved using the URF encoding.
	1. LATENT INFORMATION IS MADE EXPLICIT IN GRAPH STRUCTURE
	2. STOP-GAP HACKS REMOVED: URIs, Literal Typing, Reification, Quad Notation, and Triple Store
	3. VERTEX-CENTRIC DATA-ACCESS over common data structures are made uniform.
	_
##### Motivation

	As a structuring of data, the uniformity of RDF is extremely compelling
	_
##### [ONE] URIs contain crucial semantic data while URF nodes don't

	**The** central idea of a graph-based data representation is that the data is encoded in the graph.  (DUH, right?)  If one took the bytes encoding an excel spreadsheet and encoded them as one long unicode string then created a single RDF node with that URI-string, it would be valid RDF graph, but its an ABUSE to have important semantic information encoded inside the URI string!

	This is precisely what most practical RDF encodings do with URIs.  


	Lets make this formal:

	An RDF graph is PERMUTATION INVARIENT iff any systematic permutation of the URI associations of the graph nodes leaves the meaning of the graph unchanged.

	RDF is not permutation invarient, while URF IS.


	A user of RDF might decry that this measure is not fair, since this permutation "broke" all relations between graphs, but those were just coorespondances had no semantics beyond the coorespondance itself.  But this is generally a lie:

	- URI prefix structure is widely used to form latent encodings of groups the RDF triples and systematically relate them to other groups.  
	- Which server internet host are these triples stored on?
		Latent in the URI?
	- Who may have access to these?  Latent in URI
	- The whole tree structure of a file system.  Latent!
	- Even N3 sanctioned by W3 itself uses URI namespacing to systematically operated on groups based on latent URI structure.

	- URF is different:  There _are_ no URIs associated with URF graph node.  Instead graph coorespondances are determined by matching graph structures themselves, not vertex values.  (And surprisingly, it turns out this can be done as efficiently and simply as matching URI strings!)

	_
##### [TWO] RDF forgot to put "handles" on the data.
	RDF forgot to put handles on the data, hap hazardly bolted several partial fixes

	- URF provides an explict handle, the unit.
	It is more powerful than all RDF handles, and more uniform/simpler.
	
	- Uniform RDF Form replaces all of these half-ass bolt on "handle" appendages for operating on RDF structures using _only_ the RDF triple itself.  The result is both _SIMPLER_ and more _POWERFUL_ than Non-Uniform RDF. 
##### [THREE] RDF data encodings of many common SW data structures are not uniform

	- REDUNDANCY IS NOT UNIFORMITY
	First let's agree, if there are two different representations of some information where one can be derived from the other.  Then unioning those two together to allow both kinds of access is NOT uniformity, it is redundancy.
	
	- PRIMAL
	We say a representation is primal if no part of that representation could be removed, and then reliably be rederived from the remaining information.  Our aim is to represent information using primal patterns.

	- UNIFORM
	We say a particular class of information contained within a representation is _uniform_ in the case that that storage and access of that kind of information is parallel across the entire representation.  (e.g. there are no 'if' statements in the accessor functions.) 

	- RDF is either non-primal or non-uniform the in its representation of many core informational concepts.

	- URF and Uniform removes important classes of these non-primal / non-uniformities by constructing a spiral of "right ways" for encoding common patterns.  The result is primal at each level, and is uniform across the range of access patterns that exist at each level of patterns.

	EXAMPLES

	**CONTAINMENT and ORDERING** -- Wow, these are pretty basic, but even at this level RDF starts to fall apart.  The standard RDF encodings of these are either non-primal or non-uniform.  

	EXAMPLES
	- CONTAINMENT
	

	- REIFICATION
	Information can be expressed as a triple, or as the reification of that triple.  But encoding it in both ways is redundant since the triple itself is entirely derivable from its reification.  But having some triples represented by their reification instead of the bare triple means that they way triples themselves are encoded are very non-uniform.  One must always check the triple store for both reified and non-reified triples (in practice RDF is almost always stored redundantly).

	 of information that can be derived from 
#### -- GOAL OF URF --
	
	URF was designed as a simplest most uniform starting point for the data structures commonly used in software.  

	Specifically URF was designed to parsimoniously capture the graphs encoding of the 12 graph patterns listed in: UNIFORM . MATH . GRAPH TYPES

	For example, if G1 represents a tree with ordered edges, while G2 represents a tree with unordered edges, we would like the same algorithm to traverse both.  (of course the latter will be traversed in arbitrary order.)

	_

	- DATA SHOULD BE CONTEXTUALIZED
		Require a few minimal contextualizations of all triples, and encourage many more standardized meta information about strcutre and access patterns for the data.
		- INVERSE LINKS --
		
		- NO DENOTION W/O INTERPRETATION
			Replace RDF's implicit dentational model w. an explicit interpretational model of meaning.

		- CURATION -- CONTAINEMENT TREE HOMEGONIZATION
			At a bare minimum, all triples occur in exactly one bucket in tree of all triples.
## === TRANSLATING RDF TO URF ===
### _

Most semantic graphs are in RDF form, so it is important to understand how these relate to URF, and how to translate between RDF and URF.
### --- SUMMARIZING THE OF DIFFERENCES BETWEEN RDF AND URF ---

URF & RDF have 3 large differences, many smaller differences follow from these:

1. In URF 100% of all info is contained IN the graph, thus many "bolt on" aspects of RDF are translated into pure graph structure.

2. RDF generalizes the information stored within a single node, the aim is to increase the number of different graphical model types that uniformly map onto URF without translation.

3. URF graphs are controlled while RDF graphs are uncontrolled.

	[[PAT, I thought I understood this notion, but I don't.  "Controlled" means I have a handle to it, a way to unambiguously referencing the needed portions of the data graph for each processing task.  with more work I think I can formalize this notion... but the one in my head was broken!]]


MORE SPECIFIC DIFFERENCES TO RDF

- **RELATIONS ARE ALWAYS REIFIED** -- The entire URF graph is effectively reified.  First positions are always blank nodes, so both binary and n-ary predicates are encoded the same way: as a single unit with their structure defined by multiple items.  (See ontological discussion on why we think this is this ok.)

- **URIS ARE GONE** -- Notice URIs are gone, there is zero latent information in any vertex in a URF graph except the REL connections to other verticies.  (RDF graphs generally have important information latent in the structure of the URI character sequence.  This structure is leveraged by turtle and many other RDF encoding schemes.)

- **URIs ARE REPLACED BY PRINTABLE UNITS** -- Notice the "predicate" position with URF graphs are units drawn from **P** the set of printable units.  In an RDF graph the predicate position is constrained to be a URI.  This allows one to match cooresponding parts of RDF graphs.  Matching cooresponding parts of URF graphs requires matching printable unit (entire sub-graphs).  This is the way that URF graph coorespondances are determined, by matching printable units.

- **NOT MORE COMPLEX** -- Replacing URIs with _graphs_ of nodes might seem a large jump in usage difficult & complexity since graph coorespondance must be used to even match single links within the URF graphs.  But this is not the case.  Each printable unit has a unique print form (called unit-form).  One can treat that print form as a unicode URI -- it is just a URI with a structure that is explicit within the URF notation.

- **ACTUALLY SIMPLER / MORE UNIFORM** -- much structure often left latent in RDF ends up expressed as URF structure.

- **UNIT BACKING** is an interface for accessing & manipulating URF data.   This is outside the scope of this doc, but the structure of a URF graph is actually defined by its grounding, so URF graphs a often not static structures, but rather programs that generate such structures.
### --- RDF TO URF CONVERSION OVERVIEW ---

[[Pat, this conversion process is both out of date, and it is incorrect in several places.  I left it in, since it does still indicate spiritually how RDF and URF relate.]]

We think if URF as the more parsimonious/uniform notation not as a conversion of  underlying RDF.  Still since the whole semanatic web world is built upon RDF it is helpful to describe it as a step by step transformation of RFD into what we call "pure RDF" which has a direct correspondance with URF. 

STEPS OVERVIEW:

1. ADD HANDLES -- Create handle nodes for for all referencable things 
	- GIVE ALL REFERENCEABLE THINGS HANDLES &
		UNIFY BINARY and NARY PREDICATES
	- REPLACE TYPED LITERALS WITH PURE RDF
	- REPLACE QUADS WITH PURE RDF
	- REPLACE TRIPLE STORES WITH PURE RDF
	- REPLACE URIS WITH PURE RDF OVER ATOMS
	- REPLACE ATOMS WITH PURE RDF
2. REIFY -- Reify all triples within a triple store or dataset.
3. QUAD -- Create place units for each unique quad value as connect each reified triple, to its quad.
4. STORE -- Create unit denoting the triple store and connect to all quad units and any reified triples not in a quad
5. VALUE -- Each URI literal is converted to a unit with a type and value triple.

Move units down the tree 

_
#### CONVERSION STEPS

KEY IDEA -- We designate a special category of nodes as "buckets", and then construct a tree of these buckets in order to manage data that is encoded as triples.

First we start with an ecosystem of semantic web datasets, triple stores, etc.

CONVERSION RULES

1. **CREATE BUCKET DAG**  
	-  Designate a special non-overlapping category of blank nodes called _buckets_.  
	- Select a set of unique RDF nodes that does not occur within any existing RDF data to serve a '_bucket_containment_' predicates.
	- Select as second unique RDF node to be used as the 'grounding' predicate.

2. **CONVERT BLANKS** --  Blank nodes within any existing RDF graph are designated as a bucket node in the case that the RDF node is not the object of any triple within the repository.  At the discretion of the conversion process other nodes that _are_ the object of triples whose subject is also a bucket node may or may not be designated as a bucket nodes.  (See "blank node discretion" discussion for details.)

3. **REIFY TRIPLES**  --  Remove and replace any triple whose subject is not a bucket node with its reification.  (Notice the resulting three triples will all have the same freshly generated blank node as their subject, and thus according to rule #2 that node will be categorized as a bucket node.)

==> Notice: all triples now have a composite node as their subject
Thus we may use these nodes as a "handle" to refer to the set of triples sharing this subject composite.

4. **ADD GROUPINGS**  --  Create a blank node units to represent each distinct grouping that implicily exists within the RDF ecosystem:
	1. **QUAD NOTATION** -- Each distinct quad value is typically used to represent a unique grouping of triple data within a single triple store.  Thus a composite blank node unit is created for each unique quad value within a single triple store.  Then a triple is created linking this new composite to the handle nodes for this recently reified triple data using the 'contains' predicate from above.
	2. **TRIPLE STORE** -- Each triple store represents a grouping of triples data all of which now have bucket handles.  Thus just as with quad coversion we create a new bucket to represent the triple store itself and then link it to each bucket handle node stored within the triple store.
	
[typing --> grounding]
5. **ADD GROUNDINGS** -- Each unit node must have a triple indicating the type of the unit it grounding. This triple is indicated by the specially designated 'gnd' predicate.  Its subject is the unit itself, and its object is any node indicating a unit type.  Any bounded value may serve as the type of a unit.
	- ADD TYPING TRIPLES --  All units that do not have a typing triple have one added indicating what the triple denote (like 'triple store' or 'quad group') or something semtanics indicating what the unit represents.  
	- REPLACE TYPED LITERALS -- Typed literals within the original RDF graph are converted into two triples one indiating the type, and the second indicating the unicode string that maps to this atomic value.
	- REPLACE URIS -- URIs are replaced with a triple indicating the URI type, and a second triple indicating the unicode string value of the URI.
	
- GROUP NAMING -- Groups are named by adding a triple with a special predicate (e.g. QUADNAME or TRIPLESTORE) that links the unit node with its naming URI.
	

6. **TAXONOMIZE** -- URI structures are typically densely packed with latent semantic information.  Authority, locality, containment, persistence, and many other attributes are implicit within the characters of many URIs.  The typing conversion in step 5 did nothing to make that structure explicit.  A more faithful graph encoding of that information is obtained by mapping those structures onto URF path patterns.  Paths are sequences of symbols, numbers and more complex multi-attributed structures as well.  This flexibility may be used to encode the structure latent in a URI.
	e.g.  https://foo.bar.com:973/this/that/file.txt#here?q=7
	might translate as:
	URL(proto:"https", host:["com", "bar", "foo"], port:973,)



TAXONOMIZE DATA INTO DENOTATIONAL DOMAINS -- URF encodes parts or all of denotation itself into the graph structure.  Both the indexical naming data and the indexed target data are expressed using only graph structure, and the mapping between them is also expressed in graph structure.  
- REF -- 


You are done!  Now what do we have?  

_
#### -- URF Results --

**URIs, GONE!** -- All nodes in graph are "blank nodes" or literals (which are just short-hand for a subgraph).  The latent structure that used to reside in the URI now exist in the recursive container structure of a subgraph that explicitly encode that latent structure.  (e.g. there might be a bucket node for all data controlled by a single HTTP host is on the internet.)

**TYPE LITERALS, GONE!** -- The typing information is now expressed as a single triple in the graph.  Better yet, the pattern used to indicate typing for atomic units is now the same pattern used to indicate typing of composite units.

**REIFICATION, GONE!** -- Both binary and n-ary triples are encoded same way.  Further, all predicates can have meta-data added to their container without changing how they are mapped.

**QUADS, GONE!** -- Any need for grouping knowlege is better more flexibly handled by graph structure than could have been done with QUAD notions.  (e.g. one reified triple might simultaneously associate with eight different indexing terms effectively, thus allowing the same predicate to be indexed in eight different ways simultaneously... it is not at all easy with quad notation, but is efforless here!)

**TRIPLE STORES, GONE!** -- Of course there are still decision procedures for gathering and grouping info for data processing, one might even implement a container handle using a triple store.  But just as often not, instead using a relational DB, an in-memory data structure, or even an infinite (but decidable) mathematical structure are all equally good implementations for a container. 


Uniform RDF Form replaces all of these half-ass bolt on "handle" appendages for operating on RDF structures using _only_ the RDF triple itself.  The result is both _SIMPLER_ and more _POWERFUL_ than Non-Uniform RDF.  

Ah the glory and beauty of it!
