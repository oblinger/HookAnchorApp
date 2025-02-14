

## === URF - Uniform Relational Form ===
### _

**TL;DR.  URF is an RDF alternative that more parimoniously encodes the range of data-structures commonly employed within software langauges and frameworks.**

_
### --- VOCABULARY ---

**UNIFORM RELATIONAL FORM** -- _Uniform Relational Form (URF)_ encodes a data graph as a set of 'unit' mapping functions.  

**UNIT** -- A _unit_ is a vertex within a URF graph.

**COLL** -- A _collection_ is a unit vertex that has "structure".  The term _structure_ refers to a sequence key-value pairs associated with the vertex).

**LITERAL** -- An _literal_ is predefined vertex that can be used to align graphs with each other.

**BOUNDED** -- A unit is _bounded_ (also called _printable_) iff it is a literal or a finite collection of bounded units.

**PLACE** -- A _place_ is an element within the structure of a vertex.  
- Intuitively it represents a single spot within the graph.  
- A place is composed of three units called: _self_, _key_, and _value_.  
- Each place can be thought of as an arc within the URF graph.
- The "Self" unit is the composite vertex itself. "Key" is the arc's labelling vertex, and the "value" is its destination vertex.
- NOTE:  The name "self" is chosen because of its eventual connection to OO-method invocation, the term "place" is chosen becuse of its eventual connection to assignment semantics, and "key"/"value" are chosen because of their eventual connection to the MAP data structure.

**TRIPLE** -- A place is also sometimes called a _triple_, and its three units are called _subject_, _predicate_, _object_ respectively.
- NOTE: We generally avoid using RDF terminology in most contexts for URF places since URF semantics are not identical to RDF semantics.  In particular, in RDF if two triples have the same subject, predicate, and object, they are in fact the same triple.  This guarantee does not holds for certain parts of a URF graph (i.e. for edges eminating from a "relational" instead of "functional" verticies.)

_
### --- DEFINING URF GRAPH STRUCTURE ---

Here we formally define URF:
	
Let **L** be a set of verticies called _literals_.
Let **C** be a set of verticies call _composites_, 
	where **C** and **L** are disjoint sets
Let **U** be the union of **L** and **C**.  (collectively called _units_.)
Let **P** be a subset of **U** called _printable units_. (defined below)

Each _u_ in **C** defines an ordered mapping relation **items(_u_)**.	
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
	The URF answer: "controlled URF avoids the question by disallowing forests to exist w/o humans... after all what would be their purpose be?"

	Said in another way:  
	- URF only concerns itself with that which might be computed.
	- Data cannot be used if it cannot be accessed.
	- URF is explict, even data used to access data should be in graph.
	- Thus at a minimum, URF only operates on controlled graphs of data.


_
### --- UNIT FORM ---
- sequence of k/v pairs and bare values too
- head is the "predicate" for the nary relation
- need 'literal' notation
- need blank node notation?  (or just)
- how to notate semtantics?
	- how to notate order/relational/etc
- Grounding links on all units
### --- GRAPH MERGING ---
#### _

A key URF claim is that it expresses all information as 100% pure graph data.  At a practical level this means replacing opaque URIs with explicitly structured unit-form literals instead.  But at a theoretical level this is not fully satifying, since it seems the literals of URF still have latent information in them -- it seems they still have some kind of textual atomic information in their nodes, just like URIs do.  

Did we just push the bubble around under the carpet?  
Here we show we did not, we show we have smashed that bubble GONE!

1. WE DEFINE THE UNIFORM GLU GRAPH    (Graph of Literal Units)
~~~
	We specify the construction of the Graph of Literal Units.Uniform Atom Graph -- this is an RDF graph that contains only blank nodes.
2. DESCRIBE ATOM REPLACMENT
	We describe how one may take a "pure RDF" graph as described in the conversion section and merge it with a subset of the Atom Graph and then replace each Atom node in the pure RDF graph with an appropriate vertex of the Atom Graph.  The resulting structure is "graph only" it has no URIs at all!! It is just a wired up mass of blank nodes.
3. DESCRIBE BLANK-NODE-GRAPH-UNIFICATION
4. PROVE EQUIVELANCE
	Finally we prove given two pure RDF graphs R1 and R2, the blank-node-unification of their converted forms will be the same as the converted form of the union of the rdf triples R1 union R2.


Given two URF graphs G1 and G2 one can unabiguously relate their structures by relating their unit forms (just as RDF graphs relate parts using URIs).  


GLU - GRAPH OF LITERAL UNITS --- 

The GLU graph is an infinite, static mathematical object; it is the graph containing the set of uniform literals used for combining URF graphs.  This GLU graph is implicitly part of every URF graph.  Thus linking to these verticies serves as a kind of "glue" indicating how corresponding vertices within separated graphs should be connected.

Secondarily GLU literals have an explicit static internal graph structure defined over them.  Thus when a graph intends a linkage vertex to have a specific internal structure an appropriate GLU vertex may be chosen.  For example if one URI is within the subdirectory of another URI one might (but often does not) choose to explicitly encode this as a relationship explicitly within the RDF graph.  Of course this may also be done in URF.  Additionally however one might choose URF literals:  path("foo", "bar") and path("foo", "bar", "baz").  In this case they will automatically be related by the expected internal structure as expressed by structure within the GLU graph.


~~~~~



_
#### -- Functional Graph Alignment --

The "Functional graph alignment" of two graphs is a trivial "zippering" of two graphs togther, given some initial points of alignment between those graphs using cooresponding functional triples in each group to grow the alignment.

The "min zip" of a graph is the subgraph containging enough structure to reconstruct a given set of targeted set of vertices to be aligned.


An set A is an _**alignment**_ of two graphs G and H iff 
	it is a set of pairs {<g_i, h_i>, ...} where g_i in G, h_i in H.


An alignment A is _**conflicting**_ iff there exists   dup, x, y   where x!=y and
	<dup, x>, <dup, y> in A   or   <x, dup>, <y, dup>


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

Here we describe the construction of the _Graph of Literal Units (GLU)_---an infinite, static mathematical object that provides the structure that underlies URF literals.  The idea is that every URF graph implicitly contains a subset of the GLU and that subset is used to "glue" the URF graph to all other URFs graphs.

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
#### -- Attaching GLU to a URF graph --

Every URF graph is really short hand for an "all blank node" graph that implicitly includes an appropriate tree shake of the GLU as part of its structure.  

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

 PAT, I was confused for a long time about the relationship between URF and RDF, so I wrote alot about this to help me clarify it to myself... Thus things got a bit wordy in here feel free to skim...

_
### -- THE RDF DENOTATION FANTASY --

RDF's "Denotational emperor" has no clothes.

We argue that in practice RDF is used as a data structure.  Indeed sometimes with violent disregard to any supposed denotational semantics.  We argue this denotational semantics is mostly a fiction in RDF.  

URF, on the other hand, is unapologetically a DATA STRUCTURE and nothing but a data structure.  That said, the larger Uniform ecosystem built on top of URF does provide a sematantics for URF.  There are some key differences:

- NO ONE SEMANTICS -- In URF we officially sanction what happen in RDF.  Programmers will treat URF as a data structure and then inventing their own different semantics for its usage.  Totally cool with that.  There is no one over-arching denotational semantics one is doing violent to.  
==> Any programmer (or commmunity) can create and use their own semantics.
	
- EXPLICITY GROUNDING -- No one correct semantics does mean we need to somehow notate which semantics should be applied to each piece of data.  We need to somehow "ground" the data to a semantics.  This is done ***EXPLICITYLY*** in URF with grounding links.
- In the shallow end of the pool these grounding links are just opaque identifiers that programs and people use to understand how to interpret data.
- In the DEEEEEEEEP end of the pool the larger Uniform ecosystem pulls itself up by its bootstraps.  It provides the tools needed to begin from scratch (from an Abstract Rewrite System) and define thes semantics of the graph networks themselves ENTIRELY WITHIN IN THE GRAPH NETWORK.


~~~~~


SOCIETAL CURATATION OF URIS IS GOOD BUT IS NOT A DENOTATION FOR TRIPLES 

A key idea underlying RDF seems to be the idea that if society carefully curates and agrees upon a list of URIs it will enable effective data management since the meaning of triples, groups of triples and merged grouping of triples all have well defined meaning.  The promise of RDF:  conflict free data merging!

We argue this is fantasy, even if URIs have a denotation (which they mostly don't) the triples DEFINITELY do not have one based on this.

Still there is value in social curation of predicate and object URIs to be sure.  But the idea that this is curation is somehow either a primary semantics or sufficient semantics are both false.

Its not enough to match predicates and objects up correctly, one needs to apply the correct data for the correct processing to arrive at the correct result.  (If you doubt this, just merge all the triple data encoded in your data lake into a big triplestore and rerun your queries on that!  Denotational semantics suggest this should work.  BUT IT WONT!)

Alas the RDF triple alone does not define its meaning.  One needs to manage the triples allowed into a particular triple stores for a particular operations in order to achieve a desired result.  Unfortunately all of this crucial meta knowlege is _implicit_ in the procedural code used to build the triple store... It lives outside the universe of encodings of the bare triples themselves.

YUCK!

The solution is to first admit this reality.  Becuase of the qualification problem even when we have some semantics (denotational or otherwise) we still need to group and manage data so it gets applied to the correct problems in the correct ways.

URF acknowleges this and thus
1. "bakes in" mechanisms for grounding all assertions track of their semantics
2. provides "handles" to effectively control their application.

The RDF community has developed three mechanisms to deal with this gap: triple reification, triple's "quad" notation, and triple stores.  Uniform merges all three into a single uniform treatment:  composites as knowlege buckets.

_
### -- HANDLES DISCUSSION --
#### Tool Intro

TWO BACKGROUND IDEAS KEY FOR THIS INTRO.  THE IDEA OF
- DATA -- Information with a purpose, and 
- SPIRAL -- Encode ideas as simply/non-redundantly as possible.


A GOOD TOOL NEEDS A GOOD HANDLE

The quality of a tool is measured in how well its usage outcomes match intent the of its user, and its the tool's handle that allows the user to guide the tool toward those outcomes.

In a similar way, the quality of data is measured in how well its data processing outcomes match the intent of its user.  And in a similar fashion it is important that data has a good handles by which outcomes may be guided.  

The RDF triple is a beautiful encoding of data, perhaps it is the most profound most beautiful encoding humans have yet devised.  Yet perhaps this perfection is also it greatest flaw: they were forged without ANY handle.  One can assert a triple < seven prime true > but there is nothing to hold onto it is just a triple out there somewhere among the infinite graph of triples all possible triples.

To salvage this situation the RDF community bolting a couple of work-around handles onto the purity of the triple in order to be able to wield it:  The triple store, quad notation, triple-reification and URIs.  Each in there own way help provide a handles---methods for wielding triples in service of some data processing task.

The are each UGLY when you consider that each has a hacky non-graph way to encode data.  How terrible is that.  We invented the most beautiful, most-general, most-all-purpose data form.  And the first thing we needed to do was to create a bunch of ways to wield that form all of which encode data THAT DOES NOT FIT INTO the "universal" form.  Super UGLY.
	
We propose that all data be expressed in a uniform RDF Form, one that has handles baked in from the start, and one that unifies all of these bolt-on handle types into a single uniform control that is both more powerful and simpler that these RDF bolt-on work arounds it replaces.

_
#### CREATING HANDLES TO GROUP DATA
	
Before we replace triple stores, quads and reification, lets understand how each serves as handles for data processing.

TRIPLE STORES -- Focuses the Attention of Data Processing

If we want to use the power of and beauty the triple itself to serve as the handle for controlling the usage of triples we must first distinguish the handle from that which is handled.

==> Split RDF graph nodes into two categories:
- HANDLE CONTAINER NODES -- Container nodes used to reference and process the triple data they contain.
- ENTITY NODES -- Nodes cannot serve as containers but rather somehow relate the continer to some aspect of the world or outcome.

HANDLE CONTAINER NODES -- These RDF nodes have extra requirements.  In order to serve as an effec

==> Uniform RDF triples have the extra requirement that requires their subject nodes are handle container nodes.  And the meaning is that all triples that have this handle container as their subject are "handled" by referencing that handle node.

So what can be a handle Node?  Is < DanOblinger LivesIn SanFrancisco > a good triple?  DanOblinger seems like an ok handle for all info about that person right?  Wrong.  There is no effective decision procedure for determining if a triple should be included in such a bucket of info.  And given such an ill-defined bucket of info it is not possible to effectively wield it for an information processing task.

But Facebook-friends-of-Dan-Oblinger-on-July-14-2009 *is* a resonable bucket.  There is social / computational method of deriving the contents of this bucket, and it is specified well enough that one can effectively wield it in data processing tasks.

This bucketing requirement is not so onerous. RDF triple reification can be used to transform any bare triple like < DanOblinger  LivesIn  SanFranciscoCA > into 
< _N7093 Predicate LivesIn >
< _N7093 Person DanOblinger >
< _N7093 Location SanFranciscoCA >
#### No naked triples!!! 

**PROBLEM**:  A disembodied triple has no value as data -- we don't know what to do with it.  (see RDF DENOTATION FANTASY)

**UNIFORM SOLUTION**:  
	1. Don't allow disembodied triples -- always place them in a bucket with some known disposition (when/how to use it)
	2. Unify notions of: triple reification, n-ary predicates, triple stores, and quad notation into a single paradigm (uniform units) 

**MAPPING DENORMALIZED RDF INTO UNIFORM TRIPLES**

1. **SELECT / CREATE A BUCKET** 
	All uniform triple exist in exactly one bucket.  So we must select or create a bucket for these denormalized triples.  Maybe "ProjectX's triple store as of 9/9/2019 at 8:20pm" or
	"triples accessible via http using RDFa encoding on 9/9/2019"
	
2. REIFY 

3. CONTEXTUALIZE


- **Its buckets all the way down** -- Bare triples are always reified.  This serves as a smallest data bucket with three elements.
- **Bucket containment** -- Buckets may be referenced by many other buckets, but they are only contained within a single bucket.

Result:
- Pushes the issue of agreement on predicate usage/meaning to become the issue of bucket containment.
- Removes the need for triple stores -- and more importantly remove need for implictly encoded knowlege tying triples to world.
- generalizes triple stores, quad stores, nary-predicates, and triple-reification into a uniform treatment for all of these kinds of bucketing.
### -- DIFFERENT USAGE CONTEXT --

The RDF community seems most focused on curation of an inference over the "semantic web" of data.  URF by contrast is aiming to be a parismonous substrate for all of computation.  Both representations are sufficiently flexible to allow serving either goal fairly well.  Still the difference results in very practical differences in the realization of these two paradims.  

RDF's triple store can best be thought of as a data base of assertions which one can process.  

The URF's grounding of the unit can best be thought of as an API from which the very structure of units grounding by that API come to exist.  As a result URF often deals with infinite graphs which are defined in terms of other graphs via explict functional transformation.

One can think of URF as providing functional (and imparative) programming with a graph model substrate from which it is built.  With URF we aim for a cleaner, better grounded model semantics for computation.  This is really just a different aim, than most within the semantic web community.

That said, our ULTIMATE aim in providing a graph model semantics for computation is to come full circle and provide a solution to the semantic web task of community sharing of knowlege.  This full circle solution provides framework honoring the more honest approach where all denotation is functionally defined -- but in this larger context is EXPLICITLY DEFINED and EXPLICITLY LINK to the data it serves.


### -- FUNCTIONALLY DEFINED --
Uniform Triples have two pragmatic differences in usage:
- **INTERPRETATIONALLY DEFINED** -- No inherent denotion.
- **EPHEMERAL** -- Often RDF triples are operated on as realized  in DB entities or in memory within a triple store, while Units only exist ephemerally within an interpretational context.  Outside of an interpretational context there is only structure w/o 
### -- ISSUES WITH RDF IMPROVED BY URF --
Problems with RDF that are improved using the URF encoding.
1. LATENT INFORMATION IS MADE EXPLICIT IN GRAPH STRUCTURE
2. STOP-GAP HACKS REMOVED: URIs, Literal Typing, Reification, Quad Notation, and Triple Store
3. VERTEX-CENTRIC DATA-ACCESS over common data structures are made uniform.
_
#### Motivation

As a structuring of data, the uniformity of RDF is extremely compelling
_
#### [ONE] URIs contain crucial semantic data while URF nodes don't

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
#### [TWO] RDF forgot to put "handles" on the data.
RDF forgot to put handles on the data, hap hazardly bolted several partial fixes

- URF provides an explict handle, the unit.
It is more powerful than all RDF handles, and more uniform/simpler.
	
- Uniform RDF Form replaces all of these half-ass bolt on "handle" appendages for operating on RDF structures using _only_ the RDF triple itself.  The result is both _SIMPLER_ and more _POWERFUL_ than Non-Uniform RDF. 
#### [THREE] RDF data encodings of many common SW data structures are not uniform

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
### -- GOAL OF URF --
	
URF was designed as a simplest most uniform starting point for the data structures commonly used in software.  

Specifically URF was designed to parsimoniously capture the graphs encoding of the 12 graph patterns listed in: UNIFORM . MATH . GRAPH TYPES

For example, if G1 represents a tree with ordered edges, while G2 represents a tree with unordered edges, we would like the same algorithm to traverse both.  (of course the latter will be traversed in arbitrary order.)

_
### XXXXXX  A "bare" triple has no meaning.

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

URF & RDF have 2 large differences, many smaller differences follow from these:

1. In URF 100% of all info is contained IN the graph, thus many "bolt on" aspects of RDF are translated into pure graph structure.

2. RDF generalizes the information stored within a single node, the aim is to increase the number of different graphical models that uniformly map onto URF without translation.

3. URF graphs are closed while RDF graphs are open.


MORE SPECIFIC DIFFERENCES

- **RELATIONS ARE ALWAYS REIFIED** -- The entire URF graph is effectively reified.  First positions are always blank nodes, so both binary and n-ary predicates are encoded the same way: as a single unit with their structure defined by multiple items.  (See ontological discuss on why we think this is this ok.)

- **URIS ARE GONE** -- Notice URIs are gone, there is zero latent information in any vertex in a URF graph except the REL connections to other verticies.  (RDF graphs generally have important information latent in the structure of the URI character sequence.  This structure is leveraged by turtle and many other RDF encoding schemes.)

- **URIs ARE REPLACED BY PRINTABLE UNITS** -- Notice the "predicate" position with URF graphs are units drawn from **P** the set of printable units.  In an RDF graph the predicate position is constrained to be a URI.  This allows one to match cooresponding parts of RDF graphs.  Matching cooresponding parts of URF graphs requires matching printable unit (entire sub-graphs).  This is the way that URF graph coorespondances are determined, by matching printable units.

- **NOT MORE COMPLEX** -- Replacing URIs with _graphs_ of nodes might seem a large jump in usage difficult & complexity since graph coorespondance must be used to even match single links within the URF graphs.  But this is not the case.  Each printable unit has a unique print form (called unit-form).  One can treat that print form as a unicode URI -- it is just a URI with a structure that is explicit within the URF notation.

- **ACTUALLY SIMPLER / MORE UNIFORM** -- much structure often left latent in RDF ends up expressed as URF structure.

- **UNIT BACKING** is an interface for accessing & manipulating URF data


#### -- OLDER INFO --
Problems with RDF that are improved using the URF encoding.
1. LATENT INFORMATION IS MADE EXPLICIT IN GRAPH STRUCTURE
2. STOP-GAP HACKS REMOVED: Literal Typing, Reification, Quad Notation, and Triple Store
3. VERTEX CENTRIC DATA ACCESS IS MADE UNIFORM

- **PRINTABLE** -- think of a bounded or printable unit as JSON expression or LISP sexpr -- They are structures that can be written down on paper.
- **COMPOSITE** -- Think of units as containers that contain other units or think of them as RDF blank nodes.  Don't think of them as URIs since they cannot directly coorespond to any unit within any other graph.
_
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

1. REIFY -- Reify all triples within a triple store or dataset.
2. QUAD -- Create place units for each unique quad value as connect each reified triple, to its quad.
3. STORE -- Create unit denoting the triple store and connect to all quad units and any reified triples not in a quad
4. VALUE -- Each URI literal is converted to a unit with a type and value triple.

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
#### ... URF Results ...

**URIs, GONE!** -- All nodes in graph are "blank nodes" or literals (which are just short-hand for a subgraph).  The latent structure that used to reside in the URI now exist in the recursive container structure of a subgraph that explicitly encode that latent structure.  (e.g. there might be a bucket node for all data controlled by a single HTTP host is on the internet.)

**TYPE LITERALS, GONE!** -- The typing information is now expressed as a single triple in the graph.  Better yet, the pattern used to indicate typing for atomic units is now the same pattern used to indicate typing of composite units.

**REIFICATION, GONE!** -- Both binary and n-ary triples are encoded same way.  Further, all predicates can have meta-data added to their container without changing how they are mapped.

**QUADS, GONE!** -- Any need for grouping knowlege is better more flexibly handled by graph structure than could have been done with QUAD notions.  (e.g. one reified triple might simultaneously associate with eight different indexing terms effectively, thus allowing the same predicate to be indexed in eight different ways simultaneously... it is not at all easy with quad notation, but is efforless here!)

**TRIPLE STORES, GONE!** -- Of course there are still decision procedures for gathering and grouping info for data processing, one might even implement a container handle using a triple store.  But just as often not, instead using a relational DB, an in-memory data structure, or even an infinite (but decidable) mathematical structure are all equally good implementations for a container. 


Uniform RDF Form replaces all of these half-ass bolt on "handle" appendages for operating on RDF structures using _only_ the RDF triple itself.  The result is both _SIMPLER_ and more _POWERFUL_ than Non-Uniform RDF.  

Ah the glory and beauty of it!

~~~~

PRETENSE OF DENOTATION, GONE! -- ``Data is what data does.''  With traditional RDF and traditional URIs there is a notion that triples intrinsically "denote" things.  With URF the meaning of triple data is tied ONLY to how it is obtained for data processing and how it is used in data processing.     The new functional meaning of data is both better defined and more usable.  (See )




