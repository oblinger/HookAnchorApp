
	If you care not for how many angels may dance up the head of a pin, you should just skip onward... this section will have little value for you.


## === OUTLINE ===

- **UNIFORM AGENDA** -- the uniform agenda strikes most as improbable, impossible or at least ill-advised.  This somewhat rambling agenda section argues why the uniform agenda is actually none of these things.

- **UNICORE** -- the first rings of the uniform spiral are called "Unicore" they begin with a ring of constructs that are more drawn from simple mathematics rather than computer science.  It culminates with a set of 16 "semantic assembly" instructions intended to serve as natural basis into which a computer languages might be "macro expended" into.

	{[a first pass on unicore is close to done.	  it as been refactored so many times the docs are quite sketchy.	  pernicious confusions/conflicts still exist around the VAR construct.	  I struggle to complete a reference implementation.]}

- **LWL** -- the "language writing language" is Lisp-ish macro expander expressed as a simplest increment build from the unicore constructs.  

	{[ The LWL is mostly in my head as a guiding star.  I know unicore constructs need to parsimoniously support existing languages via rewriting in the LWL -- the ultimate goal of uniform is a web of interoperable DSLs, and that web needs an LWL to compile those DSLs into the unicore instructions they are based upon.]}

- **UNIFORM MARKDOWN** -- Uniform markdown is to code what YAML is to data.  A language-independent, human-readable format containing multiple variations intended to make different use cases more readable.  Uniform markdown is also a spiralling of machine parsing---as a framework it is less general than traditional yacc-like parsers, but it is compential in a way that those languages are not.  e.g. it is generally possible to "union" parsing grammars to generate languages that combine their underlying parsing constructs.

	{[An earlier version of uniform markdown is implemented and documented. but a large reformulation of the entire spiral is only partially  documented and is only 1/2 implemented. ]}
## === UNIFORM AGENDA ===
### --- INTRODUCTION ---

The _uniform agenda_ is to gain strong practical benefit from expressing software and software languages as part of an ever-growing ecosystem of software constructs.  (When we say "construct" think of: integer, lexical scope, control flow, assignment semantics, hash map, data repository, GUI window, or parse tree.)

- UNIFORMITY THESIS -- The uniformity thesis is that it is possible and practical to embed the capabilities of all software langauges and frameworks into a single ever-growing ecosystem of such constructs such that most any advantage provided by any of these languages is also afforded by an appropriate combination of constructs from this single ecosystem _without editing_ any aspect of these constructs.   

- THE MYTH OF TRADEOFFS -- Central to this agenda is a particular undertanding of the occurance of and consequences of tradeoffs within software language design.  Specifically we define notions of _intrisic splits_ vs _false forks_.  We argue that adding intrinsic splits to an ecosystem increase the power of that ecosystem by "freeing" certain constructs to provide new capabilities with damaging "trading off" capabilities of other constructs.  We argue that false forks, on the other hand, cause a profound and exponentially-compounding dilution of the effectiveness of all future development built upon that false fork.  Worse yet false forks greatly increase the complexity of all downstream software development.  We argue the practical consequences of these false forke for software engineering is profound.  We argue that well more than 90% of the effort in software development is simply overcoming unforced complexity resulting from more than 70 years of compounding false forks built into our software ecosystems.

- ITERATIVE, WHOLISTIC, AND PRAGMATIC -- We believe an alternate ecosystem is possible, but the ambitious goals of this ecosystem make it exceedingly hard to construct.  The good news is that humanity only needs to construct it once, and that it appears one can make progress towards this ecosystem and get value from it by iterative reformulation.  Thus the uniform agenda is iterative, wholistic and pragmatic.
	- It is _**iterative**_ in the sense that one does not build the spiral from the  bottom up, but rather by sloppily adding/editing pieces at all rings of the spiral as needed, and iteratively increasing the simplicity/completeness/compatiblity of constructs in all rings.
	- It is **_wholistic_** in the sense that we make early push for generality in any new or evolving construct.  Any construct in uniform should be so general as to eclipse, merge-with, or be-defined-by any other even vaguely related constructs.  No false forks!  Achieving this means continuously looking for generalizations/reductions across all spiral rings.
	- It is **_pragmatic_** in the sense that we don't shrink from concepts that are hard or impossible to formalize mathematically.  Rather we always start with some needed computing task and add/edit/refine constructs to support it.  Secondarily we attempt to go back and formalize these pieces in ways the increase their power/generality/clarity without loss of their orginal pragmatic value.

- ECOSYSTEM ADVANTAGE -- We argue that progress within this agenda will yeild transformative pragmatic improvements in the way that software is built, particularly in the ways that large collorative software efforts are undertaken.

_
### --- THE MYTH TRADEOFFS --- 
#### _

**TLDR; Intrinsic tradeoffs do exist -- like dense-array verses hashmap, or dynamic verses lexical scoping.  Such "intrinsic" splits like these should have BOTH sides represented within a full software ecosystem.  By constrast most software construct like Python loop verses C++ loop represent a "false" fork.  They include needless baggage in their construction in ways that make them incompatible and redundant with other forks in the ecosystem.  Uniform strives to merge all such forks into constructs that are so simple/essential there is no room for meaningful variation exists, instead only a single construct remains.  (I know, I know, its been tried before... well they did it wrong)**


Different programming langauges/paradigms/framworks are better for different things.  They make incompatible tradeoffs, and thus cannot be combined into some kind of single best single "super language" without loosing imporant benefits in one direction or another.

Every second year student in Computer Science knows this very basic fact.

But what if this "fact" is not in-fact true?  What if it were possible to combine all software languages into a single super ecosystem could that could realize advantages of all?  What if the efforts of all programmers could be harness as part of an ever growing totality, rather than squandered, spread thinly into ever more specialized and incompatible cul-de-sacs?

Such a shift -- to put it mildly -- would be transformative....

Well here we show that incompatabilities are _NOT_ a fact, indeed we argue here that a singular ecosystem _IS_ possible, it _WOULD_ allow the efforts and advancements of all to be continuously folded back into a singular ecosystem, and yes doing this _WILL_ be transformative.

_
#### -- Intrinsic Splits Verses False Forks --

To understand how such an "impossible" ecosystem is in fact both possible and practical it is helpful to begin to by distinguishing two kinds of taxonomic splits that are often confounded:  
-- an _**intrinsic split**_, (which are generally "instituted by God"), and
-- a _**false fork**_, (which are generally "instituted by [wo]man").

An _**intrinsic**_ taxonomic split is one that is tied up with one or more important properties of a constrcut that are (1) inherent to the mathematics underlying the construct, (2) that are derived from properties of our physical world, or (3) that are tied to properties common across all von Neumann architectures.

By contrast a _**false fork**_ taxonomic split is one that is NOT driven by such intrinsic attributes of mathematics, our physical world, or the the von Neuman machine. (see  "clarifing example" at the end section if puzzled why von Nueman architecture is listed here)

One can often identify a false fork by the parallels one can draw across the split itself.  A forking split will have many parallels between the operators and programs found on each of the split while the intrinsic split will generally result in alternative operators that do not have operators map onto each other in practical and natural ways.

For example the notion of an bounded integer is _intrinsically_ distinct from that of a finite set.  They both likely belong to common parent categories (for example, both are a kind of transmittable data and might share common operators like "print" that are associated with data transmission). But at the level of integer itself, the natural operators (e.g. increment, decrement, etc.) are distinct from those for a finite set (e.g. membership, etc.), and most procedures that are naturally written using integers (like fibonnaci) don't have natural parallels for programs that compute the same things using finite sets instead.  Each of these concepts have a collection of interlocking properties and opeators that do not translate well or at all to the other.  (e.g. increment, addition, etc.)

A second example of an intrisic split is the array verses the hashmap.  These are instrinsic refinements of some common container construct.  All methods in common between these can be defined at the level of the container construct, so all code with parallels between these constructs is actually not specialized towards either.  Only when some procedure REQUIRES the properties of one or the other is it tied directly to one of these constructs.

At the other extreme is a trival example of two data formats for encoding limited precision integers on von neuman machines: big indian verses little indian formats (the first put the most integer significant digits in lower addresses while the second does the opposite.  In this case, nearly all operators that apply to one also apply to the other, further procedures written for one split will have a close parallel on the other split.  This is a classic all be it trivial example of a false fork.

These false forks are so prevelant that like a fish in water we barely see them in such clear and simple form as this however.  Most constructs in most modern software langauges/frameworks are composed from dozens upon dozens of these false forks.  Indeed they are so thick and heavy the software community has come to believe at a pragmatic level at least that these forks -- like intrinsic splits are inevitable.  We disagree.

_
#### -- Uniform Agenda Claims --

_CLAIM #1 NO FORKING_ -- Here we argue that one can adjust a collection of constructs in order to progressively remove all false forks.  The remaining constructs are so distinct a that procedure which applies to one side of a remaining fork will generally have no parallel on the other side, OR this commonalities is merged so tightly that one construct disappears or common procedures can be attached to a common ancestor that specifically encodes this commonality.

_CLAIM #2 MINIMALITY_ -- Here we argue the one can "shrink/decompose" the definition of each construct to its "essence" such that for any computing task it is either (1) not applicable for the given computing task, or (2) it is precisely what is needed.  That is, there is no "freedom" left within the defintion of the construct which might have been chosen differently.  Every aspect of its definition is essential to its functioning.  Once such freedoms are removed one never needs to first "edit" a construct in order to correctly apply in a given to a usage case.  Arriving at these "essential elements of computation" is central to the Uniform Agenda.  

_CLAIM #3 POWER SPIRALING_ -- Here we argue that one can arrange software constructs into a "power spiral" a sequence of rings where:
- NESTING -- Each ring is a set of constructs that are functionally defined in terms of the more inward rings of the spiral.
- COVERING -- Each ring is complete in the sense that the entire hiearachy would not be greatly simplified by moving some construct down into a lower ring, all can be naturally built ontop of the ring.  (For example "graph models" seem to form a complete ring, one need not add to the constructs in this ring but rather can naturally form ever more complex patterns on top of it.)
- COMBINATORIAL -- The constructs within a single spiral are designed to be combined such that their powerset of its combinations encode the highest possible density of useful combinations.  (For example uniform has a single container type with a large number of interoperable refining attributes like: mutable, functional, iterable, etc.  The resulting powerset contains a high density of useful container types.)
- RING THESES -- A good construct or good ring often seems to come with a "Ring Thesis."  These assertions seem to be akin to the chruch-turning thesis, they are never proven true, but rather are pragmatic assertions of completeness which are borne out by usage and example.  "All data can be parsimoniously encoded as a graphical model" is an example of such a statement.  We look for these touchstone claims as key organizing principles for all rings in the spiral.

_CLAIM #4 PERFECTION WRAPPED BY APPROXIMATION_ -- Here we argue that many false forks in modern software stem from very necessary approximations which do not seem to lend themselves to "choice free" intrinsic splits.  In these ugly cases we delay and minimize the impact of these false forks by delaying them as long and as far as possible, so they do not "infect" the all-important lower rings of our spiral.  An common example is limited precision integers.  All software langauges agree on what an integer is, but often they disagree on the details of their limited precision versions of integer.  So we use (unrealizable) infinite-precision integers at the center of our spiral so as much code as possible can be written fork free.  But then eventually we must somehow limit the precision in some way in order to run the code.  Sadly the ways to do this limiting are not as universal as the integer itself is.  (Even "arbitrary precision" integers are still limited by the size of the von-neumann memory space).  The uniform approach is to first define "platonic ideals" for each construct in as simple and universal way as possible EVEN AS THESE ARE NOT REALIZABLE.  Then at the latest moment we add false forking constraints pragmatically to get code to run.  The result is that more code can be shared across a much wider range of computing contexts.  The cost however is that most of the core is not realizable, and the full constraints on valid realization are generally complex and left implicit.  We push all the messiness to the end and the edge, but theer things turn very messy and very pragmatic.  If it seems to be working for the cases you care about... you are done.


_
#### -- Taking A Pragmatic Approach --

Perhaps it is _theoretically_ possible to avoid all false forks as listed above.  Perhaps it is _theoretically_ possible to wholistically and iteratively refactor the entire spiral of all constructs required for to express all software languages required for all software as applied against all relevant target tasks.  Saying ``this uniform agenda seems entirely impractical'' itself sounds like a collosal understandment of the situation!  Holy _CRAP_ this seems too stupidly complex to consider!


_80/20 OPTOMALITY_
One reason avoiding tradeoffs seems so impossible is that there are a great many dimensions that one might want to optomize for each construct.  Worse yet those dimensions often tradeoff with each other.  Thus even if one did arrive at a set of intrinsic splits it would result in a HUGE number of varients.  How can there be no forks?!?!

Fortunately most dimensions exhibit diminishing returns around improvements and a few clear regions of disasterous consequence.  Thus there are often many ways of dodging any given dimensions worst effects, followed by smaller incremental improvements once going beyond the first rounds of improvment on the dimension.  This gives us room to improve other dimensions.  Thus we consider, code readability, run time performance, construct complexity and about a dozen other dimenions simultaneously when optomize -- it is an art in deciding when to consider further improvment to be diminishing returns.  The 80/20 rule gives us the flexibility to avoid certain splits by simply choosing to admit certain non-optomalities within in the solutions constructable from the spiral relative to other possible spirals.  


_A PRAGMATIC APPROACH_ 
Note, the focus of all of the four claims above is pragmatic as opposed to theoretical.  The key aim here is a spiral of constructs that are PRAGMATICALLY, non-forking, minimal, covering, and spiraling.  As a secondary consideration we may be able formalize parts of this to prove specfic aspects of these claims in specific cases, but the aim is primarily pragmatic.  What do me mean by pragmatic?  One way to frame it is this: our aim is a sprial of constructs that do not compel language designer to refactor any of those constructs when developing a new DSL for some domain using those constructs.  So for example, integer, array, and lexical scope all feel like constrcuts that are clearly in this "no refactor" category.


After years of trying, the early evidence is that such spirals seem pragmatically possible.  What may not be obvious on first consideration of this perposterous agenda is the enormous degrees of freedom available in designing such a spiral.  With enough hammock time it seems even the most vexing knots can be cut.  The result seems to be surprisingly small sets of constructs that are, surprisingly free from forks.  The trick is to split complex constructs down until you can describe their goal using a simple sentence, then to struggle to refactor these many snipits when you find ones that are in some way parallel.


A CLARIFYING EXAMPLE:
A hash map, and an array map form an intrinsic fork.  At one level of abstration (mapping) they are the same, both are a kind of mapping-container and have the same operators.  At a lower more refined level they have distinct performance characteristics.  The cause for this split is an intrinsic consequence of the memory accessing patterns inherent in a von-neumann architectures.  HUH???  Yes, both cases we desire the most performant container-element lookup operators possible.  On a von-neuman architecture this means minimizing the number of memory cells accessed during lookup.  Given this constraint, dense-arrays and hash-maps provide distinct approaches for mapping-container implementations that are _intrisically incompatible_.  It appears to be impossible to create a single construct having all performance benefits of both on a von-neuman architecture.  Notice on an associative-memory machine it would be possible to merge these.  Thus we see this particular split is intrinsic to the von-neumann machine.

The moral of the story: if you cannot tie every aspect of the defintion of a construct to some intrinsic aspect of our world, then this work is not the work of God, but rather it is the folly of [wo]man.

But I know, dear reader, that you are still not really buying any of this.  You know all of this been tried over and over again before... So instead I just need to SHOW it to you... or at least show enough of the first sprial rings that you can make an assessement for yourself:  Is it the beginnings of a pragmatic, minimal, covering, non-forking basis for computation or is it not? 

Lets get to it!

_
## === CUT TO THE CHASE == HERE IS THE DEFINITION OF URF ITSELF ===
{[Master copy of this is at:  Uniform.Structure.URF.formalization ]}







## === OLDER GRAPH DISCUSSIONS ===
### --- REVIST THESE ---
#### DISCUSSION OF KNOWLEDGE CONTEXTUALIZATION

WTF do you mean in URF you cannot assert that 7 is prime?!  That is clearly broken!
	    <7, Prime, True>  and <7, NextInt, 8> both clearly should belong to  in the graph of defined knowledge, they are "True" after all.

We deny this, at least if 7 is to be treated as atomic then it cannot occur as the subject of a triple, it can have no "structure."  and triples with 7 as its subject according to our terminology is its structure.

PADA = PeanoAxiomDerivedAssertions

Prime(7) < PADA
#### DATA (webster definition)
- PARTS
	- Numbers, measurements, etc. 
	- used as the basis for analysis, reasoning, decision-making
	- collected-for...   in a form useful for...  
	- 
- GOOGLE Data definition is - factual information (such as measurements or statistics) used as a basis for reasoning, discussion, or calculation
- GOOGLE an assumption or premise from which inferences may be drawn.
- MERRIAM WEBSTER -- 
	- factual information (such as measurements or statistics) used as a basis for reasoning, discussion, or calculation.
	- information in digital form that can be transmitted or processed.
	- information output by a sensing device or organ that includes both useful and irrelevant or redundant information and must be processed to be meaningful
	- https://www.merriam-webster.com/dictionary/data
- CAMBRIDGE DICTIONARY
	- information collected for use (information = news, facts, numbers)
	- information, especially facts or numbers, collected to be examined and considered and used to help decision-making, or information in an electronic form that can be stored and used by a computer
	- https://dictionary.cambridge.org/us/dictionary/english/data
- OXFORD ENGLISH DICTIONARY
	- Facts and statistics collected together for reference or analysis.
	- The quantities, characters, or symbols on which operations are performed by a computer, which may be stored and transmitted in the form of electrical signals and recorded on magnetic, optical, or mechanical recording media.
	- Things known or assumed as facts, making the basis of reasoning or calculation.
	- https://en.oxforddictionaries.com/definition/data
- COLLINS
	- a series of observations, measurements, or facts; information
	- the information operated on by a computer program
	- https://www.collinsdictionary.com/dictionary/english/data




THE DENOTATIONAL CULDESAC

### Comparison of RDF and URF:
- URF has simpler, more uniform, data encoding and access.
- All URF info is explicit in the graphs (not in the nodes)
- URF encodes stop-gaps mechanisms like RDF reification, Quad Notation, Namespacing, and Triple Store with a more pure more powerful and simpler graph encoding of all.

URF is graph structure, and only graph structure all the way down!
### clip...

**TERMINOLOGY:**
	- Given a triple in the graph, the vertices in the three positions are called:
		- 1st position is called the _**unit**_ or the _**subject**_. 
		- 2nd position is called the _**key**_ or _**predicate**_. 
		- 3rd position is called the _**value**_ or _**object**_. 
	- An edge, e,  is _**defined**_, if _e_ in **G**.
	- _**structure of**_ -- the _structure of_ u is the set of triples _t_ in **G** where the first part (the unit) of _t_ is _u_.
	- **_graph of_** -- the _graph of_ u is the closure of the structure function over u -- i.e. it is smallest set containing the structure of u, and containing the structure of all vertices within the graph itself.
	- Within some contexts there will be a single implict data graph containing all info, in that case parameter G is implied.
	- 
 .... clip ...
_**Unit Form**_ is a compact notation for the triples of a unit.
	**u = head(va, ..., vn, k0: v0, ..., km: vm)**  
		is shorthand for 
	<u HEAD head>
	<u 0 va>
	...
	<u 1 vb>
	<u k0 v0>
	...
	<u k0 v0>


**Formally**: 


**G** is a _labelled directed graph_, iff 
	**G** is a set of edges (also called _triples_) where
		each _g_ in **G** is a triple of three verticies _<v1, v2, v3>_

We use **V**(**G**) to denote the set of _verticies_ of **G**.
	**V** is the smallest set where: 
		For all _u_ = _<v1,v2,v3>_ in **G**   _v1, v2, v3_ in **V**.

**LABELLED DIRECTED GRAPH**
Let **G** be a graph, a set of triples: _g_ in **G**  where _g_ = <v1,v2,v3>
Let **V** be the set of verticies, where _v_ in **V** iff _v_ in some _g_ in **G**.
Given _v_ in **V**:
- The _**structure**_ of _v_ is the set of triples with _v_ in 1st postion
- The **_keys_** of _v_ are the verticies in 2nd position its stucture
- The **_value_** of _v_ are the verticies in 3nd position its stucture
### ... formal spec ...

principles:
- Be general enough to encode everything in parsimonious ways.
- Be as mathematically simple as possible.
- Avoid doing similar things in two different ways - pick one.

_
### **DIRECTED LABELLED GRAPH** 
-- a graph with a set of nodes and set of labelled edges

A **DIRECTED LABELLED GRAPH** is defined by a set of edges (triples):
	Let **V** be some set verticies, the _verticies_ of the graph.
	Let **E** be some set of graph edges, the _triples_ of the graph.
		**E** must be a subset of the cross product  **V** x **V** x **V**. 

RDF Graph is a set of triples < subject predicate object > where
	subject is a URI_reference or blank_node
	predicate is URI_reference
	object is URI_reference, blank_node, or a plain/typed literal


URF is a restriction on RDF where
	subject is a blank_nodes
	predicate is a blank_node or literal
	object is a blank_node or literal
### UNF  (OUT OF DATE)

**UNIT NORMAL FORM** (UNF) is a restriction on directed labelled graphs which simplifies and normalizes the encoding of data.

Unit Normal Form is based on the _**unit**_.  The unit, _u_, is the set of triples in the data graph with u in their first postion.

_**Unit Form**_ is a compact notation for the triples of a unit.
	**u = head(va, ..., vn, k0: v0, ..., km: vm)**  
		is shorthand for 
	<u HEAD head>
	<u 0 va>
	...
	<u 1 vb>
	<u k0 v0>
	...
	<u k0 v0>


**Formally**: 


**G** is a _labelled directed graph_, iff 
	**G** is a set of edges (also called _triples_) where
		each _g_ in **G** is a triple of three verticies _<v1, v2, v3>_

We use **V**(**G**) to denote the set of _verticies_ of **G**.
	**V** is the smallest set where: 
		For all _u_ = _<v1,v2,v3>_ in **G**   _v1, v2, v3_ in **V**.

**LABELLED DIRECTED GRAPH**
Let **G** be a graph, a set of triples: _g_ in **G**  where _g_ = <v1,v2,v3>
Let **V** be the set of verticies, where _v_ in **V** iff _v_ in some _g_ in **G**.
Given _v_ in **V**:
- The _**structure**_ of _v_ is the set of triples with _v_ in 1st postion
- The **_keys_** of _v_ are the verticies in 2nd position its stucture
- The **_value_** of _v_ are the verticies in 3nd position its stucture

**UNIFORM NORMAL FORM**
Let **A** be a subset of **V**.  These elements are called _atoms_, 
Let **U** be **V\\A** (the remainder of **V**).  These are called _units_.
Let **B** be the set of _bounded values_.  It is the smallest set where:
	where **A** is subset of **B** and
		Forall _u_ in **E**   _u_ in **B** if keys(_u_)+value(_u_) in **B**

The graph **E,U,A** is in Unit Normal Form iff 
	**E** is a subset of  **U** x **B** x **V**.


NOTES:
- **BOUNDED VALUES** -- think of bounded values as a JSON expression or an SEXPR -- it is a structure that can be written down on paper.
- **UNIT VALUES** -- Think of units as containers that contain other units or think of them as RDF blank nodes.
- **URIS ARE GONE** -- Notice URIs are gone, there is zero structure/information in any vertex in the graph.
- **URIs ARE REPLACED BY BOUNDED VALUES** -- the second "predict" position in URF graphs must have a bounded value.  This is the way that URF graph coorespondance is performed, by matching corresponding bound values.
- **RELATIONS ARE ALWAYS REIFIED** -- The entire URF graph is effectively reified.  First positions are always blank nodes, so both binary and n-ary predicates are encoded the same way: as a unit with structure defined by multiple triples.
- **NOT MORE COMPLEX** -- Replacing URIs with _graphs_ of nodes might seem a large jump in usage difficult & complexity since graph coorespondance must be used to even match URIs.  But its not true since bounded units have a unique "print form" they can be hashed, and can also be compared using a linear element-by-element comparison.  Indeed they could even be converted to unicode!
- **ACTUALLY SIMPLER / MORE UNIFORM** -- much structure often left latent in RDF ends up expressed as triples in UNF.

**TERMINOLOGY:**
	- Given a triple in the graph, the vertices in the three positions are called:
		- 1st position is called the _**unit**_ or the _**subject**_. 
		- 2nd position is called the _**key**_ or _**predicate**_. 
		- 3rd position is called the _**value**_ or _**object**_. 
	- An edge, e,  is _**defined**_, if _e_ in **G**.
	- _**structure of**_ -- the _structure of_ u is the set of triples _t_ in **G** where the first part (the unit) of _t_ is _u_.
	- **_graph of_** -- the _graph of_ u is the closure of the structure function over u -- i.e. it is smallest set containing the structure of u, and containing the structure of all vertices within the graph itself.
	- _**rooted**_ -- A graph, G, is _rooted_ if there exists a unit, u, such that G is the graph of u.
	- Within some contexts there will be a single implict data graph containing all info, in that case parameter G is implied.
	- 
### older
		set of elements called _units_, 
		where **U** has no overlap with **A**, i.e. **U** ^ **A** = 0
	Let _u_ and _k_ denote arbitrary unit elements in **U**.
	Let **V** := **A** u **U** be the set of _values_, and
		_v_ denote an arbitrary composite unit in **V**.
	Let **TRIPLESPACE** be the cross product  **U** x **V** x **V**, and
		_t_ be an arbitrary element of **TRIPLESPACE**.
	Let **G** be some subset of **TRIPLESPACE**.

and
	Let	_a_ denotes an arbitrary element _(atomic value)_ in **A**.

	Let **V** be := **A** u **U** be the set of _values_, and
		_v_ denote an arbitrary composite unit in **V**.
	Let **A** be a subset of **U**, called the _atomic units_ of **U**, and
		_a_ denote an arbitrary atomic unit in **A**.
	Let **C** be **U** \ **A** the non-atomic or _composite units_ of **U**, and
		_c_ denote an arbitrary composite unit in **C**.
### TL;DR --- RDF simplifications

- HOMEGENIZED PATTERNS
	If a structural pattern repeately occurs over time we create a 'uniform' standard way for encoding that pattern, then strive to always use that uniform standard encoding.  (see Uniform Spiral for discussion)
	- HOMEGONIZE REPRESENTATION OF BINARY & N-ARY RELATIONS.
		We reifiy all binary relations thus all uniform relations are always associated with a single unit occuring as the subject of the triple.
	- HOMEGONIZED REPRESENTATION OF:
		Ordering,
		
_

#### OLDER - Relation to RDF
Uniform Triples are RDF Triples with two restrictions on form:
- **FUNCTIONALITY** -- If AxBxC1 and AxBxC2  then C1 is C2
- **CONTAINMENT** -- U x UP x parent must be defined for all U
 
###### Why Functionality?  Answer: Uniformity
Suppose A & B are directly relatable in some way what is THE uniform way of expressing this connection?

If the relationship between A & B is functional where each A may have at most one B, then: A x predicate x B  is pretty natural and uniform.

But what if there is some form of many-way relationship here?
Now triples provide too much flexibity to indicate a single uniform structural form.

CASE ONE
[dan, age, 53] or [dan, friend, nick]   is one way to express info.

- but if one wanted to reify the set friends of dan then one would have:   [dan, friends, FO_dan] and [FO_dan, member, nick]

	Notice we now have two structural ways of notating the same relation between Nick and Dan, based on whether or not we additionally want to be able to reference the set of all friends.

    Of course we could mandate and implicature that automatically translates between the friend and friends notation.  But there is no additional expressivity in having both.  

	Uniform opts to have a uniform structure for representing 1-to-many relations.  They are always expressed as functional relation to the set of all, then a functional relation to the members of the set.

A second case.  What if there is a defined ordering between the related elements.

For example:  The BasteBeef Step in BbqRecipe.

Now the parsimonious encoding is: 
[BbqRecipe, Steps, BR_steps] and [BR_steps, 5, BasteBeef]

Uniform again requires that this be expressed as two functional relations.


So in Uniform:
- All relationships are functional
- All enumerations are expressed as [u, i, ith_ele]
- All singular relations are expressed as a single triple
    Singular includes 1-to-1, functional, and partial
    All 1-to-many relations are expressed as a functional relation to an enumeration.
  - even UNORDERED set are expressed as an enumeration with an
	 arbitrary ordering.
#### OLDER - Why Require Context?

The idea with RDF is that its URI/URL linkage provides a universal denotation for RDF triples.

But this does not really work.  If it did we would not need triple stores, would not need quad notation, and would not need reification for triples.

All of these are crutches providing missing semantics to the RDF triples--but we call them crutches since they only partially solve the denotation problem.  They all allow the programmer to explicitly manage the unrepresented context for these triples.  Its not enough to know [Dan, Friend, Nick], one must know this triple applies at a particular point in time, and to know it represents a friend link on facebook or linked in.

The RDF party line is that one should specialized the Friend relationship to indicate which is the precise denotation.  So  Dan, FacebookFriend-On-Jan-17th-2019, Nick.  However, because of the qualification problem, we know that the specificity needed to reach the true denotation of a triple is boundless.  In practice, the majority of the actual semantic meaning of triples are opaquely expressed implictly in the code used to construct and use the triple store and quad indexing of the store.  


Uniform, on the other hand, acknowledges there is no such thing a denotation w/o interpretation.  Thus a bare triple is meaningless, and is disallowed:
- every triple has a first component (a unit), and 
- that unit must have an "UP" triple that places this unit within a context that defines both is structure (by defining its backing) and its function (by defining the range of operators that apply to this unit.)

RESULT
- Essentially each unit can be thought of as the reification of all triples that have that unit as their subject.  
- Then it is one big tree of data eminating from a single root
- Where both the strcture and the interpretational semantics of each triple is determined recursively from the units above it.


MORE UNIFORM
- If A and a 1-to-many relation to B, there is a single way assert this relation between A and B:   A x pred x C    C x natural_number x B
- If B1, ... Bn are ordered, there is one way to express that ordering.
### --- PATTERN STR --- TERMINOLOGY ---
TERMINOLOGY        Structure Patterns (Terminology)
- **DATA** 		--  A collection of units
- **PRINTABLE** 	--  Can Print   -- @can(print) -- printable units - fixed, bounded, taxonomic 
- **CODE** 		--  Can Exe		-- @can(exe) -- Printable, Executable Data
- **ACCESSIBLE** --  Can Access  -- @can(get) or @can(get, x)
- **MUTABLE**    --  Can Assign  -- @can(set) or @can(set, x)
- **FIXED**		--  Cant Assign -- @cant(set) or @cant(set, x)
					WRONG: may by able to set, but resulting object will change.
					So x != x.SET(k,v)
- **LISTY**      --  Can List    -- @can(append)  (implies accessible, and iterable)
					Listy is a special case where unit keys are all natural numbers from 0 upto and including len-1 
- **ITERABLE**   --  Can Iterate -- @can(itr) Can iterate over values
- **INVERTABLE** --  Can Invert  -- @can(inv)
- **TAXONOMIC**  --  Can Traverse-- @can(path) -- Unique placement for all
- **BACKED**     --  Can Operate -- @can(ops)

- ATOM		--  not iterable???
- ID 		--  Unique int  -- 'is'
- VALUE		--  Unit with   -- '=='.  (maybe this is from the iterator?)

- ARGS       --  the triples, the keys and values associated with a unit's current iterator
- KEY        --  := Int+Str
- PATH       --  List(Key)
- CONTAINER 	--  is iterable
#### PATTERN FORMALISM 
(See DATA Formalism for context)
#### FUNCTIONAL PATTERN

A unit, u, and key, k, are called **FUNCTIONAL** iff 
	 <u, k, x> in **K** and <u, k, y> in **K**  _implies_  x == y

A unit, u, is **FUNCTIONAL** iff u,k is functional for all k in U.

NOTE: calling a unit functional makes no claim about its metakeys
	 
#### LISTY PATTERN 

A unit, u, is **LISTY** iff <u, LEN, o> in **K**.
	    if o is a natural number, n, then <u, i, v> from [0,n]
	    if o is INF
	    if o is NAN

#### ITERABLE PATTERN

	    A unit, u, is ITERABLE iff <u, ITR, enum> is defined and
	       Forall defined <u,k,v> there exist i in Natural numbers such that
		       <enum, i, _i> <_i, 0, k> <_i, 1, v>

NOTE: the ITR key may NOT be functional.  That is, there may be many triples <u, ITR, enum_i> if so, then u is iterable, but unordred.

#### ORDERED PATTERN
	    An iterable unit, u, is ORDERED iff <u,ITR> is FUNCTIONAL.

#### 'ACCESSIBLE' access pattern

A unit and key pair, u,kk are 