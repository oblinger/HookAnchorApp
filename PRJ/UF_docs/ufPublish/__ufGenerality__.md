
# ### ESSENCE ###
## _

We are interested in identifying 'universal' paradigms that are as "simple and as interoperable as possible".  These are paradigms for which humanity has not (as yet) derived even simpler formulations.  Typically they hinge on a commitment to a single newly defined "backbone" term that reresents the key commitment made by the paradim.

The API and implementation are then simply 'least commitment' embodiments of that backbone commitment.  The universality of a paradigm then comes down to getting the backbone definition correct, and then selecting API & implementation that avoids unintentionally taking on further commitments.



**EFFECTIVELY COVERS** -- A paradigm P _**effectively covers**_ a set of tasks T given some collection of background paradigms, BP, iff:
- There is a non-empty subset of T where P the solution to some T_i in T given BP is simplified by the additional usage of P, and 
- There are no tasks in T where engineers would generally agree that some specific refactoring of the paradigm, P, would allow it to apply to this other in a significantly better way.

**UNIFORM COVERING** -- A paradigm P is a _**uniform covering**_ given some collection of background paradigms, BP, iff P effectively covers all computational tasks of interest to humanity.


It is helpful to informally indicate the range of capability that ones believe and intends for a paradigm to cover.  Generally we do not have a formal vocabulary for specifying these ranges of usage, so instead we treat the claimed area of coverage for a paradigm as a kind of comment intended as an informal assertion of the aspects of computational tasks to which this paradigm applies, and what value it brings.

For example, consider the claim that ``the graph paradigm is a uniform covering of all data structures''

Formally this is a assertion that the paradigm has value, and there are no cases where engineers will prefer to refactor this paradigm in some way.  Importantly this is not a clain that one will always want to operate at the level of this paradigm.  That is not being claimed, the only claim being made is that having an ecosystem that build from this graph paradigm will not in some way to hindered by that paradigm.

## RUNG1 -- Math
### GRAPH

**GRAPH** -- A _**uniform graph**_ is a countable collection of vertices, each with a countable collection of edges.  Each edge at a minimum has a source or subject vertex and a destination or child vertex.  Further there are a countable subset of the vertices are "constant verticies" -- these verticies are unambiguiously aligned across all uniform graphs that contain them.


**THESIS**:  The Graph paradigm is a uniform covering of all data structures and all encoding of data.


As such the rewrite paradigm fits into the Uniform DAG as the single basis for all representation.

_
### REWRITE

THESIS:  The Rewrite paradigm is a uniform covering of all computable functions.


**THESIS**: The Church-Turing thesis that rewrite systems cover all computable functions.  i.e. the constraint that all computation must ultimately reduce to a rewrite system is a covering constraint.



As such the rewrite paradigm fits into the Uniform DAG as the single basis for all of computation.

_
## RUNG 2 -- REPRESENTATION
### ACCESS
#### _
**THESES**:  
- ACCESS.HANDLES is a uniform covering for APIs providing access to data.
- ACCESS.OPS is a uniform covering for the data's semantics.
- ACCESS.MODS provides a uniform covering for all graph structure restrictions and semantics as vertext traits.

These three access paradigms each organize an aspect of uniform data manipulation.
- ACCESS.HANDLES claims one can organize all data access as vertex-centric access without significant loss.  The second lists 
- ACCESS.OPS claims that these graph semantics can be completely expressed according to these six graph operators:  IDX (index), ITR (iterate), LEN (length), MET (metagraph), NEW (vertex creation), and DEL (vertex deletion)
- ACCESS.MODS claims that one can parsimoniously express graph structure restrictions as combinable vertex traits

SECONDARY CLAIM:
- Most (All?) the major framework container library classes map 1-to-1 onto uniform uniform verticies without composition.  (this is not true of RDF or some other graph models).  Specifically the following eight trait combinations:
		Bag, 		Sets, 	Lists	OrderedSets
		Relations,	Maps, 	Code, 	OrderedMaps
	stem from these three vertex trait modifiers: ordered, labelled, and functional




_{[following text needs to be refactored]}_

~~~~~~~~~
REIFICATION BIAS
Uniform biases towards explicit reification of all "things". If an engineer might systematically consider a specific group of values (verticies) as a single "thing" while they are programming, then we bias in favor of providing the synthetic view (graph) whose nodes represent these groupings.


UNIFORM HAS FLEXIBILITY TO define one graph in terms of another graph, this can be done in cases where some operation is performed over some grouping that is not specified by any single vertex within any 
GAIN BY MAKING CONTEXT EXPLICIT -- 
Coverage constraint

_
#### OPS

**THESIS**:  The structure ops thesis claims these six operators cover data expressible by the graph paradigm
_
#### MODS
##### -- MODIFIER DETAILS --

	**ORDERED** -- A graph vertex is _ordered_ iff there is a total ordering defined over its edges.

	**LABELLED** -- A graph vertex is _labelled_ iff its edges also have a "label"---an additional vertex (called the _predicate_ for the edge).  (AKA KEYED)


	_
##### -- UNIT.MODS - Discussions ---
	Narrative Explanation of the componentiality of UNIT.MODS:
	- GRAPH -- Set of nodes where each node is expressed as a list of key value pairs.
	- PER NODE -- Specialization of the graph is done on a per-node basis
	1. LABELLED -- A given node might have labels for all, none, or some of its edges.
		We use the special key "VOID" to denote an edge without a label.
	2. ORDERED -- A given node might or might not have a stable ordering of its edges.
		In both cases we have a sequence of key/value pairs, its just in one case the ordering of that sequence has meaning while in the other case it does not.
	3. UNIQUE -- 
	4. COMBINATIONS:  Labelled none/all/some = 'N', 'A', 'S'   Functional = 'F' or '-'  Ordered = 'O' or '-'
		
											 UN-ORDERED			ORDERED		
			ALL  Labelled, UNIQUE-keys			+Map			OrderedMap			
			ALL  Labelled,    ANY-keys 			Relation		+Code  (OrderedRelation)
			NO   Labels,   UNIQUE-values		Set				OrderedSet
			NO   Labels,      ANY-values		Bag				+List

	5. Limits on set:  FROZEN, BOUNDED, TAXONOMIC, DAG
	6. The transitive closure from a root node could be: BOUNDED or UNBOUNDED 
	7. The structure under a root node could be a: TREE, a non-tree DAG, or a non-DAG GRAPH
	8. Extension on set:  PERSISTENT, HISTORICAL, SYMMETRIC, LIVE, SYMMETRIC
	9. UPPY
	10. HEADY

	_
##### -- UNIT.MOD.LISTY --

	A single edge=<_k_, _v_> for unit, _u_, is called _**listy**_ iff _u_ is an ordered node and <_k_, _v_> is the _nth_ edge (counting from zero), and _k_ == _n_.

	ITEM FORM -- A units edges is expressed in item form as a list as a sequence of (k, v) pairs.
	LISTY FORM -- A unit's edges is expressed in listy form as a sequence of elements where an element is:
	(1) the value v if an edge is listy and v[0] != k

	for each listy pair, otherwise it is 

	- Thus a sequence of items, I, 
	_
### SPACE

**LEXSPACE** -- The Lexspace graph is an infinite, static, tree-structured graph object, where
each vertex has an infinite collection of children which coorespond to each natural number and each finite string value.  Each vertex has a single parent vertex, sometimes referred to as "UP".

NOTICE:  Lexspace is infinite in all directions (even up).  Lexspace is isotropic, that is, the local structure around each vertex it isomorphic to the local structure around all other verticies within lexspace (the links back DOWN to each isotropic vertex may be different, in that sense they are not isotropic).


THESIS: The space paradigm provides a uniform covering of all indexing and addressing.

The space paradigm builds upon the least commitment idea that what ever an address is it is lexical (can be written down) and is composible (can follow one address, then another subaddress).

It provides a uniform treatment of address, location, path, etc. such that one can mix and match, and which all adhere to expected semantics guarantees (like path invertibility).

The generality claim is that we can embed all addressing paradigms into this one paradigm without loss -- although not all aspects of the full paradigm may be exposed in all cases, the semantics themselves will remain consistent and well defined.

_

#### ,

	THE Remainder of introduction needs cleaning or moving

##### - more overview or discussion texts -

	~~~ LEX.LEXSPACE ~~~


	**LEXSPACE** -- _**Lexspace**_ is a static, infinite mathematical object -- a particular graph structure.  It represents the set of all possible "locations" structured into the graph that captures the "spatial" relationships that exist between those locations.

	**SPACE** -- A _**space**_ is a fixed, possibly infinite, mathematic object resulting from the recursive application of some "expansion" operator(s) starting from some initial "zero" element.

	**LEX** -- Each vertex of the lexspace graph is called a _**lex**_, and each one represents a particular "location".  In uniform anytime one speaks about the "location", "place", or "placement" of a thing, one is _ALWAYS_ refering to some explicit or implied vertex of lexspace.

	**SPATIAL** -- In uniform, calling something "spatial" means it can somehow be viewed as a set of entities where each entity has been mapped to particular vertex of lexspace.

	**ZIPPER** -- The "_**zipper**_" of lexspace sequence of lex elements, zip0, zip1, ...  that form a kind of "backbone" for the structure of lexspace.  See "construction of lexspace" for details about this zipper.

	~~~ 

	**PATH** -- A _**path**_ is a finite list of lexspace edge traversals.  Each edge traversal is either the edge label unit used to 'GET' a sublex location from its parent lex, or it is the special 'UP' unit used to traverse from a lex to its parent lex within lexspace

	**ORIGIN** --

	**ADDRESS** --

	**IDENTIFIER** -- An ident is a lex vertex within a single global lexspace of "identifiers".  These identifiers are "global" in the sense that (1) their origin the origin of ident space may never be changed, (2) this ident space is available and common across all instantiation of the place paradigm, and (3) for simplicity the structure operators for all ident in ident space are hiddent (so set/get/itr/len/met/new are hidden).  The result is a structured set of "handle" objects that have no visible structure themselves (except their path).  Unlike the lex within a single lexspace, these lex can serve as durable  and serve as immutable placeholders that can span across different lexspace space from different instantiations of this "location" paradigm.


	This lexspace object has many nice properties that notably simplify algorthms written over it:
	- EVERY LOCATION IS ADDRESSED -- Every location in lexspace has exactly one canonical path from the origin which serves as the "address" of that location.
	- EVERY ADDRESS IS VALID -- Every address denotes exactly one lex location within lexspace.
	- PATHS ARE INVERTABLE -- Every canonical path from a to b has one canonical inverse path from b to a.
	- THE INVERSE OPERATION IS SYMMETRIC -- , and symetrically it serves as the it is the inverse of that path.
	- Following any path 
	- LEXSPACE HAS NO EDGES -- Lexspace has no edges (not even a top) -- thus a great many algorithmic corner cases are avoided since lexspace has no corners.
	- LEXSPACE IS THE SAME EVERYWHERE -- Lexspace is isotropic -- The local structure around every node in lexspace is isomorphic with the structure around every other node in lexspace.


	   -- QUICK SIDE NOTE --
   
	These properties are all totally obvious and no big deal, right?

	**EXCEPT** each and every modern SW language contains its own **DOZEN** plus **DISTINCT** versions of this one paradigm.  These paradigms are each not consistent with corresponding paradigms from other languages, but worst yet, they are not even consistent with themselves WITHIN THE SAME LANGAUGE.  Variable scoping, file trees, lists, maps, DOM trees, plus dozens more are distinct half-assed versions this paradigm that each fail to have many of the nifty properties listed above, since the each one encodes its own idiosyncratically incomplete or incorrect version of the paradigm above.

	SERIOUSLY, THIS IS ALL FORKED UP!  It breaks compatibility across langauges, it breaks compatibilty across parts of a single language, and explodes the complexity of each language ecosystem since each one must contain a half-dozen operators for each of its many dozens over versions of this paradigm.  This explodes further when including the many translation operations needed between them.  Add to this the complexity of translating between cooresponding varients of the same conepts between different langauge ecosystems and you end up with the largest multiplicative-cluster-fork ever created by humanity.  Worse of all, these distinctions and divisions seem to afford little advantage for all the pain they introduce.  It is truly a lose-lose-lose proposition!  

	File trees, data structures, variable scopes, memory layouts, dynamic environment relationships, as well as nearly every modestly complex API has its own version of this one paradigm.

	Now imagine a different universe of languages, ones that never introduce a second version of the location paradigm, but instead keep using the same one.  This ecosystem would have exponenetially less fewer operators, indeed in some cases entire APIs simply disappear being entirely covered using standard structure and location operations.  Even better, cooresponding parts of different langauges end up naturally compatible since they are building from the same paradigm.

	_
##### --- LEXSPACE SEMANTICS ---
##### -- IDENT SEMANTICS --

	- 8-bit clean strings are losslessly mappable to Ident



	TO IDENT
	- split on non-back quoted '.'
	- remove '\' before '.'


	_
##### --- LEXICAL PLACEMENT / IMPLIED INDICIES ---

	Some units are not functional, that is, they do not define a mapping from unique keys onto their values.  Implied indexing derives indicies for each graph edge which are consistent with list indexing as map indexing, and are functional, having a unique string or int index for each child of the unit.

	IMPLIED INDICIES -- Assignment of unique natural number indicies  keys for a natural numbered indexing for 

	- SIMPLE KEYED UNITS are guaranteed to have indicies that match their keys in all cases
	- LISTY UNITS are guaranteed to have indicies that match their enumeration position
	- ALL OTHERS are guaranteed to have a functional placement mappying

	EDGE(key, value, index)


	PLACEMENT ALGORITHM
	count = 0
	map = {}
	for x in input.edges() and x.key and x.key not in map and (isinstance(x, int) or isinstance(x, str)):
			map[key] = x
		else:
			while count in map:
				count += 1
			map[count] = x
			count += 1
			
	
	
	_
### PLACE

**PLACE** -- A _**place**_ is thing that provides persistence---it accepts newly assigned values, and provides access to the states that result from those assigned values according to expected assignment semantics.

THESIS: The place paradigm provides a uniform covering of notions of persistence built upon sub-paradigms of timeline and change


The place paradigm is extensively used, and nearly always abused across the space of software languages and frameworks.  It is used anytime one things of a system as somehow maintaining state, and it is abused anytime the languages designers needlessly ignored or violated parts of the full and beautiful semantic we expect from persistence.

This is especially true in for languages that combine space and place.  For example many languages don't require that each thing is only in one place at one time, and that no two things can be in the same place at the same time.

_

## RUNG 3 - COMPUTATION
### GND

#### ,




**LANGUAGE** -- A uniform _**language**_ is the machinery required to construct and manage a particular family of functional artefacts "programs".

**INST** -- The _**instantiate (inst)**_ langauge operator accepts a spec and optional spec-parameters in order to construct the functional instance it indicates.

**SPEC** -- The _**specification (spec)**_ operator is the inverse of instantiate, it accepts a functional instance and returns the spec that "describes"--the spec that have produced this instance.

**MATCH** -- The _**match**_ operator is a dual of the instantiate operator, it accepts both a spec and tests if the form could have been instantiated from the given spec.  If so, it derives the "matching" spec-parameters, those parameters that cause the spec to instantate into the form.

**LANG** -- A _**language (lang)**_ is the connection between descriptive spec and the functional form it describes.  A uniform lang is encoded by these five components: 
1. **Inst** -- the set of possible resulting functional instances describable by this langauge, 
2. **Spec** -- the set of all possible spec entities for this langauge, 
3. **match** -- the relation connecting descriptive spec with "matching" functional instances is called the match relation.  Match is also used to refer to the operator that accept both spec and instance and returns matching paramerters if they match.,
4. **inst** -- the optional "instantiation" operator that accepts a spec and returns an inst.
5. **spec** -- the optional "specification" operator that accepts a functional inst and returns its spec.



~~~

**HOMOICONIC** -- A language spec is called _**homiconic**_ if it exists in two isomorphic forms -- one textual form and one structural form, along with a, semantics-free (language independent), lossless, bidirectional mapping between them.

**TEXT** -- We use the term language text form (Text) to refer to the textual format of the spec.

**SPEC** -- We use the term language spec form (Spec) to refer to structural format of the spec.

**CODE** -- Thus a homoiconic languages define mappings between three formats: text, spec, and form we generically refer to instances of a language as "code", where the _**code**_ of a language is the union of its text, spec, and form instantiations.



~~~



~~~

AERAS:
- template	spec 	fill/    /match		???
- lang		code	load/dump/ ???		exec,form
- data		class	inst/dump/parse

rename form. (exec/live)
rename inst. (instance is an over used word)

ISSUES

_
#### ,

{[WTF... way too abstract for a TLDR.]}
_**TL;DR. The grounding paradigm connect the specification of things with the functional artifacts that they specify.  Spec is the uniform paradigm for "specifying" things -- for having forms that can be "matched" against existing artefacts or "instantiated" to create new artefacts.**_

_**TL;DR. Spec is the uniform paradigm for "specifying" things -- for having forms that can be "matched" against existing artefacts or "instantiated" to create new artefacts.**_

_**TL;DR. Lang is a paradigm for defining "micro languages", where a "language" is any system of specifying functionality then producing artefacts the exhibit that functionality.**_

_**TL;DR. Slot is the uniform paradigm for "templating" -- For having "template" structures with embedded "slots", such that they might be instantiated into data stuctures give values for those slots, or might be matched against data strutures in order to obtain match values for its slots on that data.**_

_
### EVAL
EVAL is a most vanilla framing of interpretation, given that:
- Interpretation is based entirely upon uniform data to express: code, execuable-code, data-inputs, data-outputs, etc.
- Interpretation is split into 'load-time' interpretation and 'run-time' interpretation where:
	- At "load-time" the structure of code data is known and fixed, while only "type-like" constraints are known about the run time environment.  Then 
	- At "run-time" the "compiled outputs" computed at load time are available, as well as full specification of all relevant data for runtime execution is available and is consistent with any "datatype"-like constraints provided as load time.
	- The goal of this load-time / run-time split is to shift as much computation as possible into load time, so the von-neumann execution at runtime is as improved as possible for all specified input combinations.


FLOW expresses control-flow in a generalized way -- as some source-code structure that somehow determines an execution ordering for the execution step that are embedded within the control structure.  This very general framing is then specialized by a covering set of five control flow operators which together capture the idea of "nested control flow" -- control flow that "follows" the tree structuring of the control structure itself.  i.e. control flow without "goto" spagetti-code.  

PACKAGE captures the idea of source code groupings (e.g. modules etc.)  As with the 'FLOW' concept, the idea here is that the structure of source-code itself is used to drive 


Notice a central aspect of the formulation of both FLOW and PACKAGE is the idea of arranging the use of source-code structure itself in ways  that facilitates human comprehension of the consequences of that code over all possible executions of that code.  FLOW semantics limits possible orderings of execution in ways that allow humans to visually inspect code and make blanket assertions about ordering constraints.  PACKAGE semantics allows humans to visually inspect static code structures and easily infer which sources pieces will exist in each of the thousands of distinct "scope" packages what are implied by the code strucutre itself.  Moreover PACKAGE semantics provides flexible ways for humans to "program" these scopes to have just the right stuff available at just the right times in the right ways for computation to proceed as they intend.

The UNIT paradigm ties uniform structure (DATA) and uniform function (EXE) into a tidy bow where are of these paradigms are available in their full generality to define data "objects" that can recursively serve as the basis of higher-level data process where these "units" serve as the data layer 


_

#### -- maybe out of date --

Bang:													// Rename as a 'lang'
	**extends**: Template
	_ _call_ _: **slot**(**op**(lang, -> Universe))				  // should instantiate expr in lang into exe form?  

Universe:
	**extends**: Lexspace[Gnd]
	_inst_: **slot**(**op**(spec, -> Gnd))
	_thread_: **slot**(**op**(-> Universe))
	_prime_: **slot**(**op**(-> Env))
	
_
### --- EVAL.BANG -- Creation semantics and disucssion ---

BANG SEMANTICS
- The bang operator recursively scans the universe spec for units with non-void heads.
- For each such unit it is instantiated and placed into the forming universe at the cooresponding place.
- The instantiation of each grounding spec is performed within the same 'env' as the bang itself thus depending upon the parent universe's semantics of those grounding specs.
- The instantiation of each grounding spec may in turn recursively ground sub-parts of the spec and store those sub-parts within the forming universe.
- In this way the entire grounding plane for the universe is created.
- Once completed this entire instantiated structure is copied into an immutable place space and returned as the result of the bang operation.

- If the bang operator has its "self_contained" meta flag set, then the following steps are taken at instantiation time:
	- A grounding spec within the bang spec is "self referential" if its head has the same path as it has with in the bang spec.  In this case it is specifying that it should instantiate itself.
	- In these cases the bang operator first instantiates the grounding spec using the parent universe's grounding of this spec.  
	- The result of thise first grounding is an instance which is placed into the newly forming universe.
	- A second instantiation of this same spec form is performed, but this time is performed using the now-being-formed-universe to instantiate this grounding.
	- The result of this second instantiation is written into the new universe.
	- The gnd operator for this second instantiation is surgically altered so it reports itself as the grounding of itself.

This second self-contained creation flow does significantly reduce, but not eliminate dependency on the semantics of the parent universe, hence the value of a uni-bang universe.



**SELF-CONTAINED UNIVERSES**
The advantage of a self-contained universe is that, at least at a first level, the semantics of the universe is contained within that universe, since each of the groundings of the universe are themselves expressible within the language of that universe.  As a conseqeuence, for better and worse, self-contained universe is powerful enough to create universes as powerful as itself.  In some cases, however, there are security and performance advantages obtained by building non-self-containing micro-languages.  These more limited languages facilitate creation of code forms that are much more flexible than conventional data form, but are far more secure/performant than full code forms since one can whitelist capabilities that exist withing the universe.  Creating a self-contained universe requires a bit of a hat trick since something has to create the first groundings which in turn create the subsequence groundings.  In the case that the head for the spec for a grounding refers to the grounding itself, then that same grounding FROM THE PARENT UNIVERSE is used to instantiate the grounding to be used in the new universe.  Since this grounding might have a different spec from the parent universe, this grounding is used to create a second grounding from the original spec.  This final instance is installed into the newly forming universe and it is surgically altered to refer to itself as its own grounding.  See Bang Discussion for details of the messy, but essentail hat trick.

**UNI-BANG UNIVERSES**
The Bang hat-trick solution to the prime-mover grounding paradox involves the use of grounding instances from the PARENT universe when creating the self-contained sub-universe.  The creates an unwanted semantic dependency on the parent universe.  This unwanted dependency is somewhat mitigated by further making the universe a "uni-bang universe".  If one treats the eight elements of unicore as a fixed known semantic constant, then a self-contained uni-bang universe is has a language semantics that are fully contained within the language spec for that universe.  An exercise not yet completed is to formalize unicore itself as the Bang of a language spec form written in a language based only on a simple rewrite system and UNIT.URF.  Once completed, it would afford self-contained uni-bang langauges a groundedness not often found in practical programming languages.

From a security perspective uni-bang universes offer significant advantage over non-uni-bang universes.  Indeed, by clever manipulation of the bang operator and other grounding in the non-uni-bang universe one can inject arbitrary semantics into the created sub universe -- semantics that do not show up at all within the spec of that universe.  If one want pragmatic isolation of the sub-universe self containment is a pretty good step, but if one is interested is securing the sub universe by careful control over the spec of that universe, then requiring it to be a uni-bang universe is essential.


_
### Discussion
#### Natural Representation of 'code'

Text			-- At this level sequence of chars is just fine.
Data Structure 	-- At this level a single recursive 'code' type is just fine.
Executable 		-- At this level the 'execute' action is available (and maybe no other action).


Text	17			x			x+17
Data	17			x			'+'(x, 17)
Exec	const(17)	deref(x)	call(+, [deref(x), const(17)])


Text	Data		Construct	Exec
'17'	17			int_gnd		const(17)	
'x'		x()			ident_gnd	deref(x)
'x+17'	'+'(x, 17)	call(+, [deref(x), const(17)])


Bang Spec	lang.BANG.spec		-- Json = Tree[String||Number]
Universe	lang.BANG			-- Grounding from BangSpec to Universe inst
bang		lang.BANG.bang		-- Fn that creates a new univ. from bang key, from 
#### Creation Details
##### Bang Steps



_
##### OLDER STILL GOOD DETAILS

- FEATURES
	- New Universe does not depend in anyway on the creating universe, BUT
	- New universe does depend upon the GND and other instances its bang spec tree


BANG GIVENS
1. lang -- A running universe 
	- Universe must define a bang operator on the universe itself
2. spec -- a tree of
	- XX Must map 'lang.Universe' to a live grounding (not a spec)
	- XX All head grounding chains must terminate in groundings with null constructors

BANG PROCEDURE
- define:  a 'root' unit one with a head and having no ancestor with a head
- each root is instantiated, by first instantiating the grounding of its head
	(thus grounding chains must terminate in already instantiated groundings)
- The final_bang_form is the fully instantiated structure (including any recursively called sub-instantiations) 
- BANG -- A new lexspace is with the final_bang_form as its contents.
- PRIME -- The prime env is an empty without a self var and whose ops are defined by the scope at the root.
	The prime env is the basis for any derived data ??????
- DERIVED DATA: Derived data can be computed at any time, but it is done from the prime env of the space.

BANG PROCEDURE
	1. get Universe grounding from spec
	2. call init with given spec
	3. Universe initializer checks if it is the same type as the spec.get('lang').get('universe') if not then recurse
	4. execute bang procedure below


// Implementation

def bang(spec):
    univ_gnd = s0['lang']['Universe']
    return unit_gnd.inst(spec)

def bang(self, spec):
	def instantiate(path):
		gnd_lex, gnd_spec = self.origin().follow(head_path), spec.follow(head_path)
		if spec is None:
			return None
		elif isinstance(gnd_value, Gnd):								# Already instantiated
			return gnd_value
		elif not isinstance(gnd_spec, Gnd):								# Instantiate
			head_path = Ident.pathify(gnd_spec.head())
			gnd_gnd = instantiate(head_path)
			gnd = gnd_gnd.inst(gnd_spec)
			gnd_lex.value_set(gnd)
			return gnd
		elif path != (gnd_head_path:=Ident.pathify(gnd_spec.head())):   # duplicate Gnd
			gnd_gnd = instantiate(gnd_head_path)
			dup = gnd_gnd.inst(gnd_gnd.spec(gnd_spec))
			gnd_lex.value_set(dup)
			return gnd_spec
		else:															# Use null constructor for root grounding(s)
			dup = type(gnd_spec)()
			gnd_lex.value_set(dup)
			return gnd_spec
	def recurse(spec, path):
		instantiate(path)
		for k, v in spec.items():
			recurse(v, path + [k])
	recurse(spec, [])
	
	

Universe:
	_ground_: **slot**(_expr_: Spec, ->Gnd):
		**return** _self_.origin().follow(_expr_.head).value()		# CHOICE: are Gnd instances separate from their value

	_inst_: **slot**(_gnd_: Spec, ->Gnd):
		return 
		
	_eval_: **slot**(expr: Spec, ->Unit): 
		_gnd_ = _env_.ground(_expr_)
		**return** _gnd_.apply(_behavior_, _expr_)


_
#### Use of 'with' at load time and runtime
- PKG import, extends, etc. is used to affect scope, with is not.
- WITH performs assignments to slots that are in the current scope
- WITH can perform shallow, persistent, or forking binding of those slots.
- The semantics of the slot depends upon the unit factory used to create the env it is in
- with can affect any slot in scope, so stack variables, OO-slots, closure vars, etc.
- closure instances for whole source tree are created at load time all at once
- run time instance creation may link those instances to the closures they are part of



EXAMPLE

pkg0:
	factor: slot(t:Int d:1)
	bar: 22
	scale: slot(fn([x] '*'(x, factor)))

	pkg1: 
		with(factor <- 2)
		
	pkg2: 
		with(factor <- '+'(1, 3), pkg0.foo <- 88, bar <- 99
		y := scale(10)



Tree of 'with' stmts above a spot is used to create the 'actual args' for the 'new' for the env in each spot in the code used for the 'inst' actions during load time
		
_

## === UNIT ===


**(ASPIRATIONAL) THESIS**:  The three math, four structural, and five functional paradigms serve as the first three rungs of the Uniform DAG whose extended form covers the range of software languages and paradigms in use today.





