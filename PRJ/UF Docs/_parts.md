# _
~-~~
tree-sitter (incremental parser)
https://github.com/tree-sitter/tree-sitter-c
~-~~

GUI
- WIDGET: tree
- WIDGET: layout of widgets
- WIDGET: text/choice
- WIDGET: buttong
- WIDGET: draggable
- FLOW

~-~ 
COMPONENTIAL:  Semantically Complete

~-~
REMANING MERGES:  
- seems declaring a slot on an object is a bit like creating a grounding that links two worlds... the world of instances of that object, and the world of return values for that operator on those instances
# !!!
- LANG OBJECT -- Is lang an pkg?  or an object?
- LANG ACCESS -- Correct way to access current lang from given env or unit?
	Could use thread var.  NS (inside env/unit).  or class of env/unit
_

## the rest 

~-~~
**IDENTIFIER CASE RULES**
Each code identifier refers to some grounding, its case is a function of the the grounding it is associated with.  If its gnd is XXXX then its case is YYYY: 
- VARIABLE ==> snake_case	gnd is a slot and slot is not fixed
- CONSTANT ==> ALL_CAPS		gnd is a slot and slot is fixed
- TYPE     ==> CamelCase 	gnd is a pkg and used as unit template
- PACKAGE  ==> snake_case	gnd is a pkg and is not used as a unit template


Confusing cases of case
- what about constructs?

~-~~

The false-forking of RDF resulted from its committing to more than was essential, and also committing to less than was needed:

THE ESSENSE OF A UIV (Unambiguously Identifiable Vertex):
	**UIV** -- A graph vertex that may unambiguously identified within any graph.
	==> The essential purpose of a UIV is to provide alignment/merging 		
		semantics over graphs.  
	==> The definition above is the essense needed to fill this purpose.
	
	==> RDF URIs are over specializations.  
		They commit us to a specific (hacky) internal structure for URIs that is woefully, needlessly under powered -- There is no justification for limiting the structure of these verticies except to limit it to be finite such that alignment is decidable.


THE ESSENSE OF STATEMENT:
	STATEMENT -- A statement is a referenceable, combinable object.
	==> The essential purpose of a statement is to be a structure over which
		processing operators are defined.
	==> Defining these operators requires that the statements are _referencable_.

	==> RDF Triples are not referencable.  As a consequence they cannot serve
		their purpose of defining processing operators without extension.
	==> In practice Reification, Quads, Triple stores, and variously defined
		groupings of triples all effectively serve as combinable "statements"---each with its own bolted-on referencing handles.  This is quite hacky given that the triple was billed as a uniform representational structure, yet it immediately requires ontological information that doesn't fit within its own framing!  yuck, yuck, yuck!


The uniform agenda is attempt to avoid these kinds of sins---to express computation as a spiral of constructs which each capture an "essense"
	==> the barest commitment required for its singular purpose.


~ ~ ~
The uniform agenda is understood in terms of this definition of applicability.



**The Uniform Agenda is pragmatic**:   It aims to provide its non-forking community benefit by building a spiral that avoids "mismatched" applicabilty across the tasks which occur IN PRACTICE.  Often it is hard and ill-posed to ask if a construct is essential---after all one can consider a nearly infinite number of "purposes" for which it might be essential for.  Instead we iteratively edit the entire spiral, and ask, in practice do we need to edit it's constructs in order to support existing SW solutions.  If not, then in practice we it is non-forking, and if so, then we have a foil, a specific case we can use to edit the spiral to improve it.


PRAGMATIST'S DEFINTION FOR A UNIFORM SPIRAL

**CODE** -- _Code_ is any iterpretable structure.

**TASK** -- A _task_ is a specification of some computational objective which can be understood by an engineer and which can be solved by some program (code).

**SOLVES** -- We say code C _solves_ task T iff Engineers agree it does.

**CONSTRUCT** -- A _construct_ is some potentially reusable code that an engineer might use to simplify their prodcution of a solution for some software development task.  (examples:  The python programming language is a construct, so is the source code for heap sort.)

**NATURAL** -- A solution, S, is a natural solution for some task T iff engineers given the time, resources, and inclination would not find value it refactoring S into S' which they view as a "better" solution to T.  "Better" the assessment that an Engineer makes continouously as they write code, typically it includes many aspects typically including:  more maintainable, more understandable, simpler, less likely to contain bugs, easier to modify/extend, etc.

**CONSTRUCT-APPLICABILITY** -- Construct-Applicability partitions the space of all construct/task combinations into one of these three cases:
For construct, C, and task, T, we say
- C is _**perfect for**_ T
		iff engineers deem C to be useful in producing a solution for T, and even given time and resources to do so engineers dont feel need to refactor C in use for T.
- C is _**mismatched for**_ T
		iff engineers deem C to useful for T, but desire refactoring C given time and resources in order best solve for T.
- C is _**irrelevant for**_ T
		iff engineers deem that nothing like C is valuable when solving for T.


{[not quite right, since we have not pinned down what counts as a refactor]}


**SPIRAL** -- A _spiral_ is a set of constructs with a DAG of code dependencies express as a _requires_ relation over the constructs.  

**UNIFORM SPIRAL FOR** -- A spiral, S, is a _uniform spiral for_ tasks T={T_i} iff there does not exist C in S and T_i in T such that C is mismatched for T.

**SPIRAL VALUE** -- The _spiral value_ of S for task set T given some background development context, B, is a fractional value between 0 and 1 indicating the average reduction in solution development time for task T_i for the case when an unmodifiable version of sprial S is available and already understood by an engineer verses the same task given only the backgorund context B.

**SPIRAL COVERAGE** -- A spiral coverage of S for task set T is the subset T' that obtains at least delta, d, improvement in spiral value given S.



				  **EXPRESSING THE UNIFORM AGENDA AS AN ALGORITHM**

Given some set of targets software tasks T
Let S = {}  // begin from an empty spiral
repeat
	choose zero or more construct OLD_i in S
	choose one or more constructs NEW_i not in S
	where  S' = S\OLD_i + NEW_i  has greater spiral value than S
	set S = S'


NOTES:
- Because the constructs with the spiral heavily depend upon earlier constructs, often additions/edits of constructs require additions/edits of others lower both lower an above within the spiral.  Thus the process is difficult because it is holistic.
- In practice this process proceeds by finding some UPDATE OBJECTIVE:  A grouping of tasks (or construct form an existing software language) that is not well served by the spiral, either because a there is a construct that is mismatched or some repeatedly needed construct is missing.  
- Then creativity is applied to imagine edited spirals that improve spiral value for the UPDATE OBJECTIVE _while avoiding significant reduction in value for all other tasks_.

There is an assumption in computer science that one almost never can improve a spiral for one class of tasks without significantly reducing it value for other tasks.  The uniform agenda challenges this assumption, indeed the uniform agenda is predicated on the refutation of this assumption.

Instead of debating the veracity of this widely held assumption, we aim to simple refute it by example.  Below are a the first rings of a spiral designed to provide value across all of modern software engineering.
- We are not arguing whether there is or is not a better spiral than this.
- We are not overly focused on gaps in the coverage of this spiral, other than to show that the coverage is extremely broad.
- We are not demonstrating its value directly on software tasks.... there are too many of them.  Instead we take it as a given that engineers find value in existing software languages, and we show that these constructs underly (all) of these langauges and provide constructs that unify and simplify the implementation of the corresponding parts of those languages.

What is demonstrated is that one can unify a great many concept central most software langauges like, assignment semantics, control flow, scoping, packaging.  We show by example these "semantic building blocks" can be variously combined to achieve languages very near and just a valuable as existing languages.

If this is really established, it suggests a different model of software development where all code is referenced relative to a single spiral, whose constructs are refactored when they mismatch some task so they are either irrelevant for that task, or perfect for that task.



Mini-Roadmap for understanding the functional ring below:

Uniform Functional Ring unifies procedural languages by expressing them in terms of a "semantic assembly language".  Key to this language is first decomposing concerns so they dont interact, then expressing each in essential form.  All procedural langauges fully defined by explaining their:
	(1) state -- how data persists at each step during execution.  (See VAR)
	(2) flow -- the order in which steps are executed.  (See FLOW)
	(3) scope -- how identifiers map onto meaning in each "place"
		- in each STATIC place in the code. (See PKG)
		- in each DYNAMIC place in the execution timeline. (see CTX)

The key for merging these languages into a single spiral is providing building blocks isolate these concerns so all possible recominations are possible w/o constraint.  Existing muddle state/flow/scope with primatives that 

_
CHOICE:  VAR.slot how does gnd/type work?
CHOICE:  How do text, spec, form work?
_
# ### TOP ###
## === tosort ===

~-~~
rename ddd -> recipe

RECIPE -- An executable description of how a thing is constructed.

the recipe presupposes some language of construction operators and language of logical connectors for those operators.



~-~
embed transpilation in python file

    foo.bar		->	foo_bar
    foo_bar		->  foo__bar
    foo__bar	->	foo___bar
    _foo		->  __foo
    __foo__		->	___foo___
    foo._bar	->  fooDOT__bar
    foo_.bar	->	foo__DOTbar	(not sure)
    
    ..._0		Null environment only uses the '_0' suffix when name ends in '_nnn'
    ..._1		First env uses '_1' if needs distinguishing or ends in '_nnn'

~-~
oblit -- class variables and instance variables and class functions and instance methods -- oblit into slot.
sets - soa
703 738 6906

~-~
gnd spec back

need to say that a construct is a kind of package where 'load' is defined in a certain way, etc.
~-~
compact schemaless list notation
~-~



### _
	everything in all ways ; the mostest at the lowest ; cost with lowest loss, ; more for less 
	~-~-~
	exactly how does "+" merge work for listy and non-listy
unitary:  one handle,  no fork,   essential(k delta). interop(pwr spiral),
Unitary  Reified  
noshadowing (prop of pkg)
should a lang be a type (the type of its spec)
predicate uses self?
	~-~

	@gnd("lang.uf.Pkg.stmt.pkg")
	def lang_uf_

	~-~
	-- -ER nominializing suffix.  An instance providing the specified operator behavior.
	-- -SPEC nominializing suffix.
	-- add agenda ufAgenda
	-- resist choosing -- Struggle mightily and without end to find the best ways to have both.  There can be no proof that you cannot have both, for since redefining the meanings of any and all parts of the conflict is often the path forward.  But finding the redefinitions that still connect in needed ways to a newly repaired ecosystem... that is the trick.
	-- unicore is the cueball of programming paradigms -- 
		a unity that is all case with no corners to case upon
	~-~
	MIN DELTA INFO
	ALREADY KNOW -- Arrange so user already knows the answer to their own question                                                                                                                                   
	~-~
	file 
	.contents .nub .parts .fill .guts .wad .kernel .bulk .pith 
	~-~
	Right spiral
	- significant cluster of programming should not be done at the wrong spiral of abstraction when different one would serve better.
	- both too low or too high are wrong


	Never dynamically catch errors that could be caught statically

	~-~
	Lexical doc extension.

	it is valuable to allow the lexical context for a uf file to extend to text that is outside a single doc within a filesystem.

	idea is somehow to indicate that parent folder have special __init__  like files that are lexically part of the context for the existing file.

	- CANT BE TOO AGGRESIVE -- can be confusing and not performant to have a complex lexical scope.  need to limit this feature.


	~-~
	Essences never fork
	The essence of a never fork
	~-~
	INCREMENTAL COMPLEXITY -- A measure of the time-effort required understand and use some element, given full understanding and proficiency with all other elements.

	measured in tasks/human_second

	~-~
	OBJ -- a unit with its specialized semantics attached

	~-~
	ALWAYS REUSE and CHOOSE SIMPLEST --
	When possible express a new thing as the smallest delta from some exist construct.  All things equal choose the simplest construct to build from.

	~-~~
	LESS IS MORE -- Never use more when less will suffice.

	Note Structure is simpler than function.

	~-~~ 
	Nominalization -- the art of naming things.
	RETRO-REIFY
	Breakthings down and name them.
	- Break them down so the essence of each is expressible in a sentence.
	- Avoid resuing the same term for different things.
	- But of course DO oblitmerge whenever possible.
	- AssocioNominalize -- All things equal, important technical meanings are better named by an invented word rather than a multi-word phrase.  The correct understanding of both the multi-word phrase and the invent-o-word can only be dervied by  looking up the meaning.  But in the case of a multi-word phrase in some cases there might be confusion that the correct precise understanding of the phrase in derived from combining the meanings of the individual words.
	- Combinophilic -- the aim is that constructs are created and defined in ways that all reasonable (ideally all possible) combinations of the terms are meaningful, useful and mean what they say.  (e.g. ideally if there is a concept C which is kind of like X + Y + Z, then we can define X,Y,Z,C such that C=X+Y+Z precisely)
	- Main word should connote reification and fine layering
		phyllo-  filo-  Nominial
	~-~~
	UNIFORM UNDERSTANDING --
	A uniform understanding combines a framing sentence expressing the essence of a thing with a formalization minimally encoding the thing.


	A construct is not uniformly undertood until it essence is expressed in a single sentences, and formally expressed:
	Named an

	~-~~
	FILE -- A Pathy, Archival, String, Place.
	~-~~
	PAVING -- Providing an interface as it would exist if it were not constrained by any of the classes that it is constructed.
	~-~
	REDRY -- Radically Enhanced Don't Repeat Yourself


	UDRY -- Uniformly Dont Repeat Yourself
	Radically Executed. Reused Essence

	We treat all uniform code as if it were a single large program and refactor repeated portions in order to simplify its entirety.

	Why this is (slightly) less crazy than it sound:

	There are innumerable ways of adding complexity to a thing, but far fewer ways to simplify it.  As one approaches the essence of the thing alternatives shrink, often leaving but one choice at the very center.  Write that one down, and add it to the book.

	See NeverFork, Oblituaries, 

	~-~

	no API; prior knowlege -- you know how to operate on structures
	you already know the logical structure of a file system; a web page; a database; a document; a shopping cart; 
#### POWER SET DECOMPOSITION

	Power set decomposition is generally the most preferred form of decomposition, better than inheritance, taxonomies, or part decompositions.

	POWERSET DECOMPOSTION -- A powerset decomposition expresses a collection of alternatives as combinations of some underlying basis set

	?should this really be part-compositions of powerset combinations?## Places for programming languages grad student hire
		- Cophenhagen (main)
		- Scotland (stirling)
		- Scotland (St andrews)
		- England ()
		- Japanese (Aki)
		- Russia (Keldysh - Andre Klemov)
### EIP TRACKING OBJECTIVE
		- Automate / simplify / robustify eletronic capture of project state at the right level of detail.
			- Need explicit status update handshake
			- Diversity of input methods to get sufficient info in

		two devs
		one pm
		all features validated
		all releases 
		ticket flow

		ticket flow ; git flow ; deployment flow

		are people frustrated idle
		reqs are 

		FAILS
		- not broken into tickets
		- missing / incomplete reqs
### old API.md
	CHOICE: maybe the ops in the current env should not be a meta key?
#### STRUCTURE

	**BACKING**
	unit.GET(key) → value
	unit.SET(key, value) → unit2
	unit.NEW(args…) → unit2
	unit.DEL() → und
	unit.ITR() → iteration
	unit.LEN() → size
	unit.IDX() → key
	unit.UP → parent	  		// CHOICE: remove?  
	unit.OPS([child]) -> Ops

##### Special units
	_ _ len _ _
	_ _ itr _ _ 
	_ _ idx _ _
	_ _ up _ _
	_ _ ops _ _

	inf 
	und
	true
	false
	???null???  tuple???? blank???


	**OPTIONS**
	Ops.GET('_ _ opt _ _) -> Options
	order:		seq, bag
	set:		true, false
	mutability: live, free, lock, firm, durable, archived, verifiable, verified, chained
	tempo:		eager, lazy
	cached:		true, false
				pkg, ctx, var, gnd


	**PATH**
	u.path_top → root			// Structure’s root
	u.path([from_unit]) → path	// “Dot” path to u
	u.path_get(path) → Unit		// Follows path
	u.path_set(path, value)		// Follow & set
	u1.path_compare(u2) → 		// Lexical sort (from operators)   CHOICE: Remove?
	“+”(path1, …) → combined_path	// Concat paths (from operators)
	// Patch Operators
	root1.path_sets(patch) → root2	// update tree
	root_new.path_gets(root_original) → patch	// create patch


	**ATOMIC OPS**
	u.print
	u.parse
	u.equals

	spec.matches(u)
#### FUNCTION

##### Interpret

	**Env**
		_ _ alpha _ _ 	-> _current_interpretation_type_
		_ _ ops _ _ 	-> _current_functional_bindings_
		self 			-> _current_structural_bindings_
		_local_var0_ -> . . .
		_local_var1_ -> . . .

	**LANG**
	lang.interpret(form0, ..., env: Env, self: Unit, fns: Fns, -> result Unit)
	lang.REDUCE(env, form0, ..., grounding_form)

	**ALPHA**
		eval
		expand
		lex???
		compile
		denote???
##### Interpretation Primatives
	**EXE** 
	**DEF** 
	**GND** 
##### Declaration Primatives

	**VAR**
		n: t: e: 
		access
		assign
		enter
		exit

	**CTX** 
		VAR(n: t: e:)
		"<-"(place Path, expr Code)
		"->"(tag Unit, block Code...)
		"->"(true, finally_block Code...)
		Ctx(c Code) -> Ctx
		.structure(-> Patch(Code))		//  vars accessible from an instance of this context
		.function(-> Fns)				//  functions accessible an instance of this context
		.exe_within(e Env, code Code..., -> result Unit)	// creates sub-env and executes in it.

	**PKG**
##### Control Flow Primatives
	BLK
	BRA
	CHN
	REP
	RET






	
	

	xxxx
#### LANGUAGE 
##### PLACE
	n: t:TYPE e:CODE v:
	.access(->TYPE)     *x
	.assign(TYPE)        x = y
	.lex(->Lex)
##### STREAM
	.step()
#### === MENAGERIE ===
##### --- ENGINES ---
###### TRS - Term Rewriting System

	_CONSEQUENT_ <== _ANTECEDENT_  { _BODY_ }

	trs.rewrite(form)
### === TODO ===
#### --- QUICKIES ---
#### --- BIGGIES ---
##### -- ASM --
	- DOT -- test it.
	- PKG -- 
	- CTX 
	- REP 
##### -- Boot --
	- boot("uc") -> Env
	- boot("uc", "444") -> 444
	- boot("uc", "uc") -> Env
	- boot("uc", "uc") == boot("uc")
	- EXE("file.this.or.that") -> file
	- ufl -> heady tuple uf.collection
	- file -> bootmark ufl
	- load(file)           {[with "uc" 444]}

	- boot_modules -> boot_module {[finds mark&bootsit]}
	- load pkg -> Pkg ;
	- Env + NS -> Env'
	- Env + Pkg -> Env'
	- loader functions (like macros) recursively load
		- so load think w mark will load mark
		- loading Pkg will side effect add self to E.lang.env.ns ??
		- ^^^^ is this side effect correct????

	- decide how E.lang.env.ns gets updated?
		A: by adding
	- decide what is the exact steps of boot:
		E.base.parse -> E.base.load 
	- BootMark
	- lang_load: repeated eval till 
> >>>  ; DOT ; Env parse/print ; one arg boot
> >>> one arg boot (parse mark, then EXE, then Env   which does ns 'load')
> ??? NS-hacking (so it does imports)
> ??? load needed for EXE?  for boot?

	Run textputer parser tree and dump all from boot mark
###### - load construct file -

	-~- uc -~-
:: 
		pkg uc.LAZY;
			example.pkg_name.foo1: GND(eval3: "pyfn.foo1")
			def fn foo2: eval3:
	-~-
##### -- Unicore --
	pkg
	later:  rep, ctx/ret
##### -- Textputer -- 
#### --- EVER DO ---
	- CODE CLEAN
## === EXPOSITION ===



- Elemental Intro
- Ecosystem slides ; Agenda slides
- Periodic Table  (completeness conjectures)
- Structure overview (Unit Form; Unit API; addressing; URF)
- Function overview (UCOMP; Lang; Slot; Flow; Pkg; Ctx)
- LWL

UNICORE
UNIT:		GET,SET, ITR,LEN, NEW,DEL
FLOW:		BLK, BRA, REP, DOT, RET
Lang:		parse,print,  load,dump,  bang
Declare:	SLOT, PKG, CTX, 

LWL
- DDD
_
## === PERIODIC TABLE ===

-~- lang.ddd -~-
        
pkg lang.uf.constructs

// **The periodic table of computational elements**
ECOSYSTEM	:= UNICORE + UNIFORM + MENAGERIE

UNICORE		:= MATH    + UNIT    + FUNC    + ASM  		
  MATH 		:= GRAPH + TYPE  + ADDR  + SPACE + ATOM	
  NODE    	:= URF   + UNIT  + OPT   + UFORM	
  FUNC    	:= LEX   + UCOMP + EXE   + LANG	
  ASM 		:= FLOW  + VAR   + PKG   + CTX 	


UNIFORM	:= LWL     + IO      + DATA    + FORM    + ENGINE
LWL 	:= SPECS + ALPHA + CTRL  + MISC  + PLUG
IO		:= MKDWN + 
DATA	:= VIEW  + LIVE  + STRM  + VER?
ENGINE	:=
FORM	:=
   	

MENAGERIE	:= AGENDA  + LIVE    + COLAB   + SOCIETY + 
MKDOWN  := PARSE + TOKEN + NEST  + XFORM + UFORM
SPECS 	:= ALT   + ARG   + BOOT  + CLASS + IDENT + INTERFACE + 
		   META  + LANG  + PKG   + STR   + TYPE 
	
SOCIETY	:=
COLLAB	:= VER
LIVE	:= 
AGENDA	:= 
			

UNIFORM ASSEMBLY:  UASM := UNIT + FUNC + ASM

DECLARE	  **PKG** (package) **VAR** (variable)**CTX** (context)	**RET** (return)
CONTROL	  **BLK** (block)   **BRA** (branch)  **DOT** (compose) **REP** (repeat)
ACCESS	  **IDX** (index)   **ITR** (iterate) **LEN** (length)
UNIT	  **NEW** (create)  **DEL** (delete)  **INV** (invert)  **MET** (meta)

META ACCESSORS -- SPECIAL SYMBOLS
^gnd, ^head, ^ns, ^body (nope last arg is block), 




base	space	compute	organize
GND		LEX		EVAL	FLAT
UNIT	PLACE	FLOW	PKG		

UNIT	LEX		PLACE	FLAT	Structure
GND		EVAL	PKG		FLOW	Function

GND < UNIT < LEX < PLACE < EVAL < FLOW	
		   < FLAT		 < PKG

_
## === UNIFORM TREE ===
LANG	.UC	.UF								- LANGUAGES
UC		.STR .FUN							- UNICORE
STR 	.UNIT  .PLACE .MDc					- STRUCTURAL
STR.MD	
FUN 	.LANG .EVAL .PKG .CTX .SLOT .FLOW 	- FUNCTION ELEMENTS
MD - MARKDOWN ELEMENTS
LWL - LANGUAGE WRITING LANGUAGE ELEMENTS
## === ESSENCE STATEMENTS ===

GRAPH 


**LANG** is the essence of _**"homoiconic software language"**_
- It commits to three losslessly convertable forms:  Text, Spec, Form

**LEX** is the essensce of _**"lexical semantics"**_
- It commits to a tree structured space of identifiers as 'semantic locations' which may be associated with 'groundings' -- finite structures which somehow are operationalizable to provide a functionally defined semantics.

EXE0 is the essence of **_"interpretation"_** or "computation" or "execution"
- It commits to the existence of state-containing univereses for execution
- It commits to some new universe creation operator:  -> Env  
- It commits to some 'execution' operator:   Spec, Env -> Unit
- It commies to some 'reduction' operator:   Form, Env -> Unit
	(EXE0 Uses names:  'Env', 'bang', 'exe', and 'reduce' for these operations)

**EXE** specializes EXE0 capturing the essence of _**"object oriented interpretation"**_
- It commits to a special unit it calls 'self' within some interpretations Env
- It commits to a kind of reduction it calls 'invoke' that depends upon 'self'

**PKG** is the essence of **_"source code grouping"_**     (static context)
- It commits to semantics collections (Forms) with elements that map 1-to-1 from Spec "pkg" containers.
- It commits to a "declarish" combination algebra over these packages.    (this means packages combine in a ways where the resulting set memberships are understandable by inspection)

**CTX** is the essense of _**"dynamic extent"**_  (dynamic context)
- It commits to a nesting structure for intervals of interpretation time
- It commits to a "shadowing" semantic for interpretation environment changes within these intervals

**VAR** is the essence of _**"object-based persistance"**_
- It commits to place-value-access-assignment semantics
- It commits to 'places' being grouped by 'object' handles "where" they "exist
- It commits to 'places' being access by these object handles
- It commits to an execution semantic (operators available during interpretation) which are also indexed by object handle.




UNICORE is the essense of "procedural computation"
- It commits to expressing all procedural langauges as macro combinations of
	- DATA ops:	ref, len, itr, inv, new, del, meta, and Place
	- DECL ops:	Lang, lex, exe, pkg, ctx, var
	- FLOW ops:	blk, bra, chn, rep, ret


_
## === THESIS STATEMENTS ===

**TL;DR.  Each thesis is a church-turning style claim of some kind of completeness  at some spiral.**

   >>> These are not ready for review at all! <<<
	(see agenda > principles > Spiral0 > UniformityThesis )

### +GRAPH THESIS 
All software can be naturally expressed using graphical models
   (no, some data is more naturally expressed as an image)
All software can be naturally expressed as directed graphs
All data can be expressed as triple data
All data can be expressed as URF graphs

>> All data may be encoded as a graphical model.
>> All data is naturally encoded as a graphical model.  (not true)

_
### +GRAPH TYPES THESIS
Selecting these 12 graph patters as a basis set, forms a natural first rung for the space of all modern data structures.

>> The 12 uniform graph patterns are a first spiral for a power spiraling covering all modern data-structures.
_
### MATH THESIS


Is this true for non-procedural formalisms?  (I think so)

>> The MATH construct provides a natural first rung for all software.

(If true, this would be empirically determined only after much usage.)

_
### +UNIT CENTRIC THESIS

>> All data access and data manipulation are naturally expressed using the eight unicore UNIT operators.

_
### FUNCTIONAL SEMANTICS THESIS
NATURAL -- SW langauges and frameworks can be naturally framed as a collection of LANG constructs embeded within a LEXSPACE overwhich some EXEecutable model is constructed.

Further, procedural langauges are naturally partitioned into dynamic control structures, and static declarational structures.

Unicore 80/20 unifies procedural languages, naturally expressing them as macro combination of its GET/SET/ITR/LEN/MET/INV/NEW/DEL object primatives, its BLK/BRA/CHN/REP/RET FLOW primatives, and its PKG/VAR/CTX declarational primatives.

_
### LEXSPACE THESIS

### UCALC THESIS

### INTERP THESIS
All computataion 

### +FLOW THESIS

### ASSEMBLY THESIS
All SW systems are naturally expressed as abstractions of the 20 semantics assembly instructions.

>> 

### LANG THESIS


### UNICORE THESIS

>> All SW is naturally expressed as some combination of uniform structural and uniform functional elements. 
_
## === PARADIGM COMMITMENTS ===

UNIT -- Vertex centric graph access/manipulation is sufficient and natural

LEX -- 
When vertex edges are uniquly identifiable then traversals become well defined.
Commitment:  All indexing operations, notions of paths, and addressing build from the lex paradigm.

PLACE 


GND --

EVAL -- 
Commit:  Interpretation is 
Commit:  interpretation paradigm defined by spec


_
## === MEASURING HUMAN FACTORS ===
### WHAT is measured?  indicate: participants, training, task, target
Several language factors (Embeddable, Natural, Homoiconic) are human factors – they are defined in terms of the behavior of a particular contextualize audience as defined by these:  
- Participants -- drawn from drawn from a defined class, 
- Training – Having defined prior training, 
- Task – Performing an indicated task in context.
- Target -- Task and context are representative drawn from a target problem domain relevant to some community.
_
### WHO is measured?  Sofware Engineers
Unless indicated otherwise, audience is a "Software Engineer" ass defined here:

SOFTWARE ENGINEER:
PEOPLE == Moderately experienced software engineers.  ~4 years experience.
TRAINING == Their software experience, knowledge of Uniform Structure & Uniform Markdown, and minimal (< 10min) training related to the specific construct being tested.
TASK == Looking at source code they are currently operating on in some way.Having (~10seconds) to make the required determination.
TARGET == Developing a SW solution for some defined targeted problem, or implementing a language construct for some targeted class of problems.

GENERAL USER:
PEOPLE == Moderately experience users of technology.  Facile navigation of smart-phone applications, including setup menus. 
TRAINING == Prior knowledge of the intent for an interface or application, and knowledge mastery of the domain to which interface/application applies.Ten minutes of experience playing with the application/interface without any instructor or instruction materials provided.
SITUATION == Participant is asked to perform some task, or to derive some value while using that application.  They have ten seconds to execute the task or provide the answer. 

POWER USER:  (Like a general user except:)
PEOPLE == Non-programmer, configuration king.  Learns/uses complex interfaces. 


FINE PRINT
- QUALITATIVE & COMPARATIVE – In principle most tests could yield quantitative scores.  Generally all that is required is an ability achieves some context dependent "sufficiency" level, or a comparative score indicating and A scores better than B.
- SIZE -- TASK/DATA/CODE -- Most tasks can be made impossible by simple scaling the volume of data being processed.  This is not an interesting dimension of complexity, thus each of these tests are performed uses artefacts with a level of complexity expected  “in the wild" for the target context.
- PERFORMANCE LEVEL == Performance level is measured over a range having most utility and generally expected values for that skill.  
_
### FINE PRINT
- QUALITATIVE & COMPARATIVE – In principle most tests could yield quantitative scores.  Generally all that is required is an ability achieves some context dependent "sufficiency" level, or a comparative score indicating and A scores better than B.
- SIZE -- TASK/DATA/CODE -- Most tasks can be made impossible by simple scaling the volume of data being processed.  This is not an interesting dimension of complexity, thus each of these tests are performed uses artefacts with a level of complexity expected  “in the wild" for the target context.
- PERFORMANCE LEVEL == Performance level is measured over a range having most utility and generally expected values for that skill.  
_
### KEY HUMAN MEASURMENTS 

**PERCEPTUALLY PARALLEL** – Two classes are perceptually parallel if people can reliably/effortlessly map instances from one onto the other.
- TRAINING:  Participants are taught membership criteria for both classes.
- They spend 10 min. viewing pairs of instances that map onto each other.
- TEST:  When presented with a new pair they can determine if they are parallel, and can indicate the parallel sub-parts given 10 seconds time.
		Example usage: homoiconicity requires user to ``see’’ textual source-code as parallel to its parsed data form.
		
**NATURAL / PRACTICAL** – A solution is deemed _natural_ according to some population of software engineers if the are not "often" inclined to refactor the solution into another form (or language) given the resources and the mandate needed for refactoring.  
	(Here the percent threshold for "often" is set at one or two standard deviations above the "base propensity" to refactor.  This _base propensity_ the distribution of refactored solutions are themselves "re-refactored" when presented to other members of this same population of engineers.  So a 50% refactoring rate is not counted as "often" in the case that the refactored solutions, are themselves refactored 50% of the time.  While a 30% refactoring rate is counted as 'often' if the refactored code is itself almost never refactored.)

**UNDERSTOOD** – A software artefact is _understood_ by a participant if:
1. It can be correctly implemented by them given sufficient time. 
2. They can correctly predict behavior/outputs given specific interpretation context (inputs) 

**DECLARISH** – A _declarish_ property is a property of source-code that is defined precisely functionally (through by some specific computation) while also being independently approximately derivable by a person looking briefly only at the source code itself.
- TRAINING:  Participants are taught the procedural interpretation of the declarish property.  They then spend a couple of minutes viewing examples of source code with associated properties’ value.
- TESTING:  When presented with novel source code can participants reliably compute and report the declarish property reliably within a ten seconds after seeing the novel code for first time.
		E.g.  Package aggregation must be declarish – that is, a participant must be able to rapidly determine resulting package contents looking only at package source code and thinking about the associated import statements/shadowing etc.

**NEARLY AS GOOD AS** – One set of constructs is _nearly as good as_ another set if it does not score meaningfully worse on ANY virtue:
- This assessment is performed within a given problem domain.
- The threshold for "meaningfully worse" is when engineers would choose to refactor into the alternate form during a rewrite of the code-base because of its poor performance on the indicated virtue.
_
## === CONSTRUCT NOTIONS ===

We believe each construct named below is a best formulation of listed SW notion.

==MATH==
GRAPH 		-- Graphical Models
TYPE 		-- Set theoretic modelling of possible graph values
ADDRESSING 	-- formulation of "indexing data via structured address values"
SPACETIME 	-- formulation of "space, place, time, assignment, spacetime" 
ATOM 		-- formulation of 
	
STRUCTURE --
URF 		-- formulation of "vertex-centric graphical model" of data
NODE 		-- formulation of "vertex-centric data access model"
	
FUNCTION -- 
REWRITE 	-- formulation of "rewrite system"
INTERP 		-- formulation of "interprtation"
FLOW 		-- formulation of "execution flow control"
VAR 		-- formulation of "persistence"
PKG 		-- formulation of "source-code grouping"
CTX 		-- formulation of "sub context"
LANG 		-- formulation of "software language"

LWL --

DATA -- 
	

Progression
ARS		-- Abstract Rewrite System	--  A most abstracted model of execution
TRS		-- Term Rewrite System		--  Rewriting of structured objects
UCALC	-- A Lambda Calculus 		--  Locus of ctrl; stack expansion; 
										load vs run; type erasure
EXE		-- Von Neumann like machine --  Semantic interpreter--model of grounding
ASM		-- Semantic Assembly		--	

_
## === other names / discussion ===
NODE == VERT
OPT == TYPE   (in the unit level of tree)

TYPE:  MATH(type/category/kind)  STRUCTURE(type/head)  LWL(class)
# ### UDM  - UNIFORM TURING MACHINE ###
## === GRAPH ===
((SECTION MOVED TO ELEMENTS))
#### === GRAPH - GRAPH MODELS ===
##### --- VOCABULARY --- 

		Standard Terms: child, ancestor, descendant, connected, structure, DAG, back linked, rooted, and root.

		Uniform uses these conventional terms for discussing its graph data.
		Formally, given an origin unit, _o_, target unit, _t_, key _k_, and subgraph _G_:
		  t is a _**child of**_ o   	iff  there is a key, k where t==o.GET(k)
		  t is a **_descendant of_** o	iff  t is a child of o or child of a descendant of o
		  t is a _**ancestor of**_ o	iff  o is a descendant of t.
		  t is **_connected to_** o		iff  t==o, t is an ancestor/descendant of o, or 
								   t is connected to an ancestor or descendant of o
		  G is _**the structure of**_ o	iff  G is the graph of units u, connected to o.
		  G is _**a DAG**_    			iff  it does not contain a cycle, that is:
		  There is no a sequence ui in S and keys ki such that
			  ui+1==ui.GET(ki) and u0==un.GET(kn) where n is the length of o.
		  G is _**back linked**_ 		iff  For all o, t, k
			  o.GET(k)=t implies t.UP==o and t.IDX==k
		? G is _**rooted**_ 			iff  G is a back linked DAG
		  o is _**the root of**_ G   	iff  G is rooted and o is the ancestor of every, t in G except itself.


		EDITS NEEDED:
			_{[fix definition of "structure of" sometime it refers to immediate structure, the transitive closure downward, or in all directions.]}_
			ADD THESE:  
				value == transitive closure from vertex.
				Note: we use the term 'value' interchangably to refer to a specific vertex or to subgraph of vertices and edges reachable from that vertex.
				graph data == refers collectively to the nodes and edges of graph models


		**GRAPH VOCABULARY**:
		**VERTEX** -- A node within a graph.   {[kind of circular!]}
		**EDGE** -- A directed edge connecting unit verticies in a graph.
		**UNIT** -- An edge's orginating unit vertex.
		**KEY** -- An edge's optional labelling unit vertex.
		**VALUE** -- An edge's destination unit vertex.

		NOTE: We intentionally avoid using the RDF triple terms subject, predicate, object for two reasons:  Unit/Key/Value will map onto other expected notation later, and because certain kinds of graphical models would require multiple RDF triples to encode a single graph model edge, this would introduce confusion.  We use the subject/predicate/object terms only when refering specifically to RDF triples.



		{[CHOICE: unit/key/value. subject/predicate/object ]}
		_
###### COLL - Collection
		**BAG** -- A whole which is composed of elements
		**IN**	-- Relation that holds between a part is the whole
		**ADD** -- Cause element to be "in" whole
		**DEL** -- Cause element to not be "in" whole
		**ITR** -- An enumeration of the elements within a whole

		HAS(c, e)
		ADD(c, e)
		DEL(c, e)
		ITR(c) -> Iterator

		Iterator() -> (key, value)
		_
###### --- ORDER ---

		should this be higher level language stuff?

		**INDEX -- NTH** -- Returns the nth place within 
		**LESS** -- Returns true if first ele before second
		**SLICE** -- A sub-interval of elements in the ordering
		SLICE_SET -- 
		APPEND, FIRST, REST, LAST, BUTLAST 

		NTH(coll, i) -> place   ??
		LESS(item1, item2)     ??
		SLICE(start, end)
		SLICE_SET(start, end, values)

		ITEM(value: nth:)


		_
##### --- Graph Operators ---

		{[eh, don't worry about these now]}

		**HAS**(u1, u2)			-- True if edge from u1 to u2
		**EDGES**(u)			-- List of edges with u as subject
		**LEN**(u) 				-- The cardinality of the edges list
		**ADD_EDGE**(e)			-- Adds verticies referenced by edge
		**DEL_EDGE**(e)			-- 	
		**ADD_VERTEX**(v)		-- 
		**DEL_VERTEX**(v)  		-- Removes vertex and all edges that ref vertex

		_
## === FN    - FUNCTIONS ===

**FUNCTION** - a _function_ maps a vector of "input" verticies in the data graph onto a result graph
_
## === TYPES - TYPING ===
### _

**TL;DR.  Any computable restriction on graph structure is a _type_.  Each type separates graph verticies into two categories:  those graph vertices that match the type and those that don't.  Uniform defines 11 type constraints here which may be mixed and matched.**

**TYPE** -- A _type_ is any computable restriction on graph structure.  A type can be viewed as a predicate over graph values, returning true iff the structure surrounding a given vertex within a given graph matches the type.

**GROUNDING** -- The _grounding_ for a type is a structure that serves to declaratively and functionally define the sematics for that type.  It specifies the set of functions defined over the type and declaratively indicates properties shared by elements of the type.  A graph is said to be _grounded_ iff each vertex can be mapped to a grounding structure of a matching type.

_{[does grounding belong in the "math" ring?... this is really functional behavior]}_






**EXPECTED VOCABULARY**
Here we formally define vocabulary that is frequently used to discuss graphical structures:

**CHILD/ELEMENT** -- C is a _child_ of U iff there exists edge, E, with U as its unit and C as its value. C is also sometimes called an _element_ of U.
**CHILDREN/ELEMENTS** -- The _children_ of U refers to the collection of all child elements of U.  The children of U are also sometimes called the _elements_ of U.  (NOTE: we refer to it as a collection instead of a set since it is possible that membership and/or enumeration are not available.)
**DESCENDENT/DESCENDENTS/SUBSPACE OF** -- D is a _descendent_ of U iff D is a child of U or D is a child of a descendent of U.  The collection of verticies that are a desendent of U are called the _descendents_ of U.  The subspace of U refers to: the vertex, U, all descendents of U, as well as all edges with these vertices as their subject.
**PARENT/ANCESTOR** -- _parent_ and _ancestor_ are the inverses of child and descendent respectfully.
**CONTAINER/CONTAINS/IN** -- In the case that a graph denotes a containment hiearchy we may use these synonyms:  D is in U iff D is a descendent of U.  U contains D iff D in U.  Finally a vertex maybe be called a _container_ to emphsize the hiearchy below it.
**OF** -- The "of" notation is used to specify the type of a container as well as the type of the elements within the container.  For example:  U is a tree **_of_** Bounded Lists.
**STRUCTURE/STRUCTURAL** -- _Structure_ is a synonym for graph, and the adjective _structural_ refers to properties that are derived from structure.
**EQUALITY** -- Vertex, V1, is _equal_ to a vertex, V2 iff there exists a one-to-one mapping from the subspace of V1 onto the subspace of V2 that preserves all edge-vertex relations, and edge orderings.
Specifically for each corresponding edges, E1 and E2 in V1 and V2, their unit, key and value verticies are either cooresponding vertices in the value graphs or are the SAME vertex within the graph external to each of these value graphs.


_
### other terminology DATA STRUCTURING PATTERNS
- **structure of** -- The _structure of_ u is the term used to refer to the items(u), meta(u), keys(u) and values(u).
- **graph defined by** -- The _graph defined by_ u is the structure of u and the structure of all units d contained in u.
- _**rooted**_ -- A graph, G, is _rooted_ if there exists a unit, u, (called the _root_) such that G is the graph defined by u.

TREE STRUCTURES
- path -- sequence of keys
- acyclic(u, keys) -- no paths loops
- taxonomic(u, keys) -- only one path to node
### -- MAPPY -- Functional / Mapy --

**FUNCTIONAL** -- A vertex is functional iff it guarantees that for any given key there is at most one of its place triples cointaining this key.

_
### -- LISTY --
#### _

**LISTY** -- A vertex is _listy_ iff its keys include all natural beginning at 0 and possibly ending at some integer n.

**LISTY** -- A vertex is _listy_ iff it is mappy, ordered, and all triples having natural numbered keys occur within its ordering before all other triples with larger numeric keys.


CHOICE:  There are good reasons for both definitions above.  See choices below for mess of conflicting options.


_
#### -- CHOICES --
##### - Len -
ISSUE: How to know the elements that should be treated as list elements.
CHOICE: LISTY - LENGTH

**OPT LISTY vs HYBRID** - all options can apply only to listy units, or all R hybrid
**OPT IMPLICIT** - All naturally numbered pairs form an implicit list
**OPT CONTIGUOUS** - Numbers 0..n-1 for list, and list[n]==Null is the first null
**OPT LENGTH** - Special method is defined that specifies end of listy elements.
	(can be bound to LEN for units that only have listy children)
**OPT PURITY** - Listy units may not have non-nat keys.
	==> trouble for fn-call like structures that need both

**REQUIREMENTS**
- **ITEMS** - for k,v in u.items() - works for all units.  list may not be first
- **BARE** - for item in lst - must work for listy units.
- ???EXPLICIT - for item in units.zzz(u) - must ???? not sure
### -- DAGY / Tree --

**DAG** -- A _Directed Acylic Graph (DAG)_ is a graph with no cycles in the graph.  Formally, it is a graph where there does not exist a unit, _u_, and non-empty path, _p_, such that path_get(_u, p_) = _u_.

**PARTIAL ORDER** -- A DAG is also called a partial order.  Such a DAG implicitly defines an ordering where:
  LESS(x, y) iff there exists non-empty p where y = path_get(x,p)

**TREE** -- A _tree_ is a singly connected DAG

Several important algorithms are defined over DAGS.
These include:

- Preorder/inorder/postorder traversals.
- subsumption check
- closure enumeration (traversals)
- least common ancestor
- topological sorting.

Motivating Tasks:
(1) find out if there is a path from one node to another
(2) retrieve all notes which a path exists to
(3) find the least common ancestor for a set of nodes


_
## === SPACE - LOCATION, ADDRESSING ===
kinda junky stuff here ... some gems
### used to be PATH - ADDRESSING _

**TL;DR.  A path is a sequence of keys denoting a graph traversal; all uniform addressing is built from traversals executed according to this structure.**

**PATH** -- A sequence of edge keys or negative integers

**ADDRESSING** -- The act of sequentially navigating from one vertex to another within a graph based on a path's edge keys.

**>>This construct is well understood and quite beautiful. I just have not written it up here.  See "ADDRESSING SLIDE"**

ORIGIN? -- 

IDENT? --

- Addressing the process of mapping an address into a sub-part within some larger whole.
- A Path Address is a sequence of keys used to iteratively traverse to the indicated data.

_
### PLACEMENT DETAILS  (out of place)


PLACE -- A referencable, and possibly modifiable portion of the data graph.
ACCESS -- 
ASSIGN --

- PLACE - a smallest referenceable possibly modifiable part of a  data graph.  (the value of one pair in one units items)
- Each unit is in exactly one place at any one time.

~-~~-~ TRY #2
PLACE - A unit, key pair where the key is a functional on u.
ACCESS - 
ASSIGN -

- All quering or modifications of any URF graph can be expressed as a set of place accesses or assignments


PLACEMENT CANONICALIZATION ALGORITHM
1. Select or create a unit to serve as the root place tree, and add it as the first element in the sequence of placed units.
2. Repeatedly select an unplaced unit, u, to add to the sequence.
- This unit, u, must have at least one parent unit that is already placed and has a key, k, where p[k]==u.
- If there are multiple potential parents, one is designated as THE parent of the unit, u, and u is added to the sequence of placed units.
- In some cases one many need to create and add new 'place' nodes to the data graph in order to place all nodes.

PROPERTIES
- COUNTABLY INFINITE -- This process allows for a countably infinite sequence of placed units.
- FINITE PARANTAGE -- Each placed unit will have a finite chain of parent nodes eventually leading to the root placed unit.
- UNIQUE PATH -- Each placed unit has a unique path designator	
### from uf.html

path(key0, key1, ...)
A Path specifies a sequence of keys that can be used to traverse from a source unit to a target unit within a structure. Beginning at some origin unit, each keyi specifies how unit, ui+1 is derived from ui. The final un is the target unit in the path. If a keyi is a negative integer, then it indicates a number of iterations of calling the UP operator to get the next ui+1 from ui. Otherwise if keyi is not a negative integer, then ui+1 = ui.GET(keyi.

The Uniform declaration for the Path unit type is:
  defclass Path( Key || !path(...type(Key||`UP) )

NOTE: Path units are constants which means they evaluate to themselves and do not evaluate their arguments. A path constant may be directly entered into Unicore code, so x = path(-2, foo, 33) is the path traversing up two unit then down the foo and 3 key indicies. Further x == eval(x) will be true since the path is a constant value.)

destination_unit.path_get([source_unit]) ⟶ Path
For all pairs of units, this returns the guaranteed unique shortest path from source_unit to destination_unit. path_get returns the address (the path from root) if no source_unit is specified..
address
The address of a unit within a structure is simply the result of calling path_get() on the unit – it is path to that unit from the root of the structure. In this way paths can serve a both an absolute or relative path.
source_unit.path_follow(...path Path) ⟶ destination Unit
path_follow is the inverse of path_get it will follow the provided path from the source_unit to return the destination_unit. Formally, for all possible unit_src and unit_dest:
unit_dest == unit_src.path_follow( unit_dest.path_get(unit_src) )
In the case that path_follow has multiple arguments they are simply chained together following each path_follow starting from the result of the prior path_follow.
"+"(...paths Path) ⟶ cat_path Path
The cat operator (see container ops) applies to path Units by concatenating their elements as expected for any Unit, so path(-2, foo) + path(bar, 3) + path(more) == path(-2, foo, bar, 3, more). Further, Unicore's addressing logic simultaneously ensures that following cat-ed paths is always the same as sequentially following the individual paths. Specifically for all Units, u, all Paths, pathi, and Path combo where
  combo == '+'( path0, path1, ... ) then we have
    u.at(combo_path) == u.at(path0).at(path1)... == u.
Path Operators
Repeately executing the GET and UP operators allows for the traversal of any connected structure. In Uniform such a traversal is called a Path, and is encoded as a sequence of string or integer keys or the special "UP" symbol. This dot is defined below along with operators the create and follow such paths:

Path ≡ dot(key0, key1, ...)
Path ≡ key0 . key1 . ...
A Path is a sequence of structural accessors which may be sequentially followed in order to traverse from one unit in a structure to another.
Path(key0, key1, ...) ⟶ p Path
A constructor macro that returns a dot path list. The Path operator does not evaluate it arguments, in order to make it easy to create constant path objects in Uniform.
u.path_get(p Path, exitspec) ⟶ dest Unit
Repeatedly calls GET or UP based on the elements of p, returns the final unit in the traversal. Otherwise it executes and returns exitspec.
u.path_top(p Path, exitspec) ⟶ root Unit
Repeatedly calls UP and returns the unit whose parent is und. Returns exitspec if the up operator is not defined for some type of unit.
u.path(+root Unit) ⟶ traversal Path
This operator is the inverse of path_get it returns the Path that goes from the root unit to the u unit.So for all Units,u1, u2, in a connected structure with 'UP' links:
    u2 == u1.path_follow( u2.path(u1) )
If root is not specified then the path_top of the entire structure is used.
u.path_set(p Path, new_value Unit, exitspec)
The destructive inverse of path_get – it traverses a structure and sets the final key, in a way that a subsequent path_get will return new_value.
u.path_sets(sets [Path->Unit], exit ExitSpec) ⟶ new_root Unit
Performs repeated path_set operations. In the case that the structure is immutable, then the new immutable root after assignment is returns. If the structure is mutable, then null is returned. In the case that the structure is frozen, or other error, then exitspec is processed and returned.
u.path_patch(orig) ⟶ p Patch
Greedily computes an approximately minimal patch which when applied (path_sets) to the orig unit will produce a unit equal to u.
u.path_compare(v) ⟶ order spec( '-1 || '0 || '1 )
Performs a lexographic comparison of the fix (integer) keys for two units. Each position is compared against it counterpart using the default '<' operator or the ^less_fn passed. und is considered to be less than all other values. Returns -1 if u is smaller, zero if they are equal, otherwise returns 1.

_
### old--- Path ---

- Fixy, Listy Unit of KEY
- ABSOLUTE PATH -- A path whose first element is a PathRoot 
Indicates that path begins from the specified path root.
- LOOKUP -- Returns the unit reference by this absolute path
- SECURITY ISSUE -- Need simple ways to limit scope of the reference for a path variable

INVARIENTS
- Path1 + Path2 == Path( List(Path1) + List(Path2) )
- dest == src.path_get( y.path(source=src) )     if src =~ Pathy
- u.path_get(p1+p2) == u.path_get(p1).path_get(p2)
## === SPACE - SPACETIME ===
### _

**TL;DR.  The spacetime construct uses graph structure to formalize intuitive notions of: space, time, place, value, change, assignment, and spacetime.**
_
### -- SPACE --

**SPACE** -- A grouping of units and edges are a _closed space (space)_ iff all edge traversals remain within the collection.  Formally:
S = <U, E> is a _space_ iff 
	forall e in E, e.subject, e.predicate, and e.object are in U
	forall u in U, u=e.subject or u=e.predicate or u=e.object implies e in E


**VOCABULARY:**

**ROOTED** -- A Space is _rooted_ iff it is the transitive closure of a single vertex, called the _root_ of the space.

**SUBSPACE** -- In the case that the units and edges of a space are a subset of the the units and edges of another space.

**G-SPACE / SUBPACE OF G** -- The _subspace of g_, or _g-space_, is the subspace rooted at vertex g.  Since g uniquely specifies the subspace, and the rooted subspace uniquely specifies g, we sometimes refer to g and g-space interchangably.

**SPATIAL** -- Refers to properties derived from some space, and often these properties additionally rely on the element ordering between the edges within the space.


_
### -- VALUE --

**CONSTANT** -- a bounded, rooted subspace of static elements. 

**ATOM** -- a type of vertex that cannot have structure (children). 

**VALUE** -- the _value_ associated with some graph vertex, g, is the subset of the subspace of g that in the same location as g itself.

{[definition depends upon "location" which is in flux]}
	
**VOCABULARY**
- **DATA** -- _Data_ is one or many bounded values intended for use as part of some information processing task.
- **DATA GRAPH** -- The _data graph_ is the collection of all data.  Each subgraph in the data graph is disjoint from all others.

_
### -- TIME --

**TIMELINE** -- an ordered collection of _access_ and _assign_ elements that satisfy _assignment semantics_:
- Formally a timeline is an ordered collection of AA-tuples where each AA-tuple is a list whose first element is the atom "ACCESS" or "ASSIGN" and whose second element is any vertex.
- The **ACCESS** tuple denotes an access that returned to specified vertex.
- The **ASSIGN** tuple denotes an assignment which associated the newly supplied vertex to this place.
- **ASSIGNMENT SEMANTICS** stipulates that the verticies associated with each access element in the timeline must be the same vertex associated with the closest assign element lower than this access element within the timeline ordering.


**VOCABULARY:**

**TEMPORAL** -- Refers to properties derived from some timeline graph structure.

**TIME POINT / POINT IN TIME** -- A specfic vertex within some timeline.

**PERSIST / PERSISTENCE** -- In order to achieve assignment semantics, the values assigned must somehow be maintained to they are available for subsequent access -- this maintance of assigned values is called _**persistence**_.  And if some particular structure is used to maintain that state we say it _**persists**_, or is _**persisting**_ that state.

For example if the the third position (the value vertex) of a specific edge within a graph is used to track the most recently assign value, then we say that edge is providing persistence for this assignment semantics.



**TIMELINE INTUITION**:
- Timeline formalizes TEMPORAL assignment semantics as SPATIAL constraints between a collection of tuple elements.  Each access element denotes the "reading" and returning of some vertex value, while assign elements denotes the "writing" of a vertex value for subsequent reading.
- The semantics of assignment itself requires a total ordering of these accesses and assignment for a given timeline, but it places no requirement of synchronization between different timelines.  Thus our most general model of time encodes history as a set of unconnected timelines.  (To be a useful programming paradigm most languages impose further constraint onto this most general model of time in order to ensure well defined derivation of assignments in each place.)
- ??? A timeline denotes the history of all edges for a single vertex within the 




_
### -- PLACE --

_{[WET INK.  This section is no longer consistent with URF place and LEX location.  Further its notion of placement might now be an incompatible notion. ]}_

**LOCATION** -- A recursive partitioning used to organize graph data.  

**PLACEMENT** -- A mapping of the verticies of a graph into this location partitioning.

**PLACE** -- A _place_ is a location vertex that also implements assignment semantics as defined above.

**PLACE SPACE** -- _Place space_ is a fixed mathematical object -- an idealized piece of paper upon which all graph data may be "written"


Formally:
- _**location**_ is itself a graph of location vertices organzied into a recursive partitioning tree.
- A _**placement**_, P, for a graph, G, is a mapping of each vertex, g in G, onto some location, l, within the location tree. 
- Each _**place**_ has a structure (children and parent) indicating its location within the recursive containment tree, and a timeline indicating the value assigned this place over time.
- _**Place space**_ is a static, ordered, mapy graph of location place verticies.  Without loss of generality we arrange that all location trees exist within this single place space structure.  

VOCABULARY:
In that case the place vertex, p is called the _location_, _place_, or _placement_ of the mapped vertex g, and g is _in the location_ or _in the place_ p.


<<<<<<<<< EDGE PLACEMENT

<<<<<<<<< PLACEMENT OF NON-MAPY, UNORDERED EDGES

~-~-~

**PLACE SPACE** -- is a static, ordered, mapy structure of places.  
- **SINGULAR** -- There is only one Place Space and with out loss of generality we can arrage that all location trees occur somewhere within this single static structure.
- **INFINITE** -- Place space is infinite:
- Each place has a parent place, e.g. there is no root to this tree.
- Each place has an, ordered, countably-infinite number of child places, one for each possible vertex. 
		<<<<<<<<<<<<<<<<< ????
- **ORIGIN** -- For addressing purposes, one vertex is designated as the _origin_ vertex.  This may be any vertex (even a vertex far outside the populated portion of the tree).  This origin is used to provide unique addresses for each place in Place Space.

_
### -- SPACETIME --

**SPACETIME** - A _spacetime_ is a coherent collection of PLACE TIMELINES.

**COHERENCE** -- A spacetime, ST = {T0, ...Tm}, is _coherent_ iff:
There exists a total ordering, A = {A1, ..., An}, and
progression of coherence graphs G = {G1, ..., Gn} such that:
1.  _The total ordering is the union of all timelines_:
	A = union of timelines {T0, ... Tm} in ST
2.  _The total ordering preserves all orderings_:
	For all i, j, k, l, m   where
		Ti[j] = A[l] and Ti[k] = A[m]
			j<k iff l<m
3.  _The coherence graphs are consistent with the assignments_ 
	For all T_i in ST, a_j in T_i, there exists k where 
		A_k = a_j  AND
			G_i = G_i-1		if Ak is an ACCESS
			G_i = PATH_SET(G_i-1, PATH(T_i), VERTEX(A_k)) 
							if Ak is an ASSIGNMENT
4.  _All accessed verticies are consistent with the cooresponding part of the cooresponding coherence graph_
	For all T_i in ST, a_j in T_i, there exists k where 
		A_k = a_j, A_k is an ACCESS, AND
			VALUE(VERTEX(A_k)) = 
			VALUE(PATH_GET(G_i, PATH(T_i)))


**SPACETIME INTUITION**:
- Spacetime is an aggregating structure with no edits or delete.
- It merges all data values input, processed, or output by some computation into one large structure whose changes over time are encoded on to the relevant timelines.
- Each timeline encodes assignement changes that have occurred at a specific spot within the coherence graph, while the access must return a vertex whose value in the coherence graph is equal to the value 


<<<<<< is reading a value more than reading one vertex?
<<<<<< do we need functional spanners in history?
<<<<<< are we saying there exists, then w/o race cond for all?

<<<<<< core idea - this is most general history.  most languages relate these access/assignments to each other, and add constraints to the ordering between timelines in order to ensure that the outcome is predictable (no race conditions).


#### spacetime intuition

Intuitively spacetime is a graph of "places" that contain values which change over time.

The assignments to each individual place are fully ordered.  This ensures that the value returned by an access operation is well defined since the total ordering ensures there is a specific assignment operation which is later than all other assignments that are earlier than this access.  The value returned is that most recent assigned value.

Still spacetime does not assume a single "global clock" -- accesses and assignments happening in one place are not totally ordered with respect to those happening in other places.

Instead they must only be _coherent_.  This coherency requires that there exist at least one total ordering of all accesses and assignments which is consistent with the accesses and assignments to the unified value graph.


At the same time we don't assume that assignments to separate places are ordered with respect to each other.


BUT the assignment of a value to a place actually creates 
### -- VOCABULARY --

**STATIC** -- A thing is said to be static if no property or attribute derivable from it alone can be different at different points in time.

**DYNAMIC** -- A thing that is not static.

**AGGREGATING** -- A value that is dynamic in that it changes over time, but it changes at most once, and only from undefined into some defined value.  

==> Need to add _UNDEFINED_ vertex into graph???

#### -- vocabulary discussion --

- EXACT WORDING MATTERS
The wording of definition for static draws attention to the fact that the boundary that one puts around a thing might affect whether it is static or not.
For example an identifier is static it will always be the same sequence of integers and strings.  But a set of identifier bindings are not static since their associations might be different at different times.

- SCOPE MATTERS
Most static things become dynamic once the scope is expanded wide enough.  ???? or are they just different things ????
Are only mathematical tautologies are truely static


**AGGREGATION**
An aggregating structure is possibly unbounded static structure with a defined order defined over its places.  Specifically 
- Each place within this structure must have its value assigned prior to any access to that place.
- Once a place has a value assigned it is never reassigned.
- The derivation of each place-value must only depende upon place-values that have already been assigned at the time they are assigned.

ISSUE: Is it valid to search an aggregating list?
- Searching could be ok, as long as the item is found.  But falling off the end of the list is not ok, since then one has accessed an undefined location that later might be defined.
CHOICE:
- Any access to an undefined place terminates a computation, that way the aggregation property is not lost, that particular computation simply fails (with no error catching allowed, essentially it is as if that computation had never begun)
## === ATOM  - ATOMIC OPS ===
### _

**TL;DR.  Uniform has 3 atomic types (integers, strings, and floating point numbers) along with expected operators (+, -, *, /, %, ==, <).**


**ATOM** -- An _atom_ is a unit that cannot have structure.  

At a minimum Uniform has at least one 32-bit precision integer, floating point atomic type, and one 32-bit length string atom.

At a minimum these atomic types define +, -, *, /, %, ==, < operators as expected for integers and numbers, and + for strings.

_
### --- PEANO'S LITERALS GRAPH ---

**PEANO'S LITERALS GRAPH** -- _Peano's Atomic Graph_ is a infinite, static structure that  defines the space of Unicore Atoms along with their expected operators.

The key property of the atomic graph is that all properties and operators defined over the Uniform Atoms may be derived from graph structure matching over this graph.


_
## === PLUS  - COMBINE ===

{[WET INK!!  I think it might be possible to define a single group theory operator at this level and use it in as an important (or ?only?) strucutural operation needed at this level in the power spiral???  not sure...]}

**PLUS** -- To combine is to derive a single output from multiple inputs.

In the case of graph structures there are a number of commonly used combination operators which we lists here:

- CONCATENATE -- A very common 
- **OVERWRITE** -- A very common way to combine functional verticies is to "overwrite" links with the same key.
- LISTY -- 
- relative order of numeric labels is maintained, 
- order between inputs is maintained
- canonical output is 0..n-1
- MERGE -- 
- STRUCTURED -- 
- RECURSIVE -- 
- DECLARATIONAL -- 

**As an option on place**: overwrite, concat, pkg
- pkg -- stmts to update pkg.

**PKG PLACE CMDS**:		(Place may have ordered, labelled structure)
    + V		Cat value V to place
    + K:V 	Cat V to K-place
    + K1.K2.K3:V	Cat V to K-subplace
    - V		Removes value V from place 
    - K:V	Removes value V from k-place
    




_
# ### DATA - STRUCTURE - UNIFORM DATA STRUCTURES ###
## === URF - UNIFORM RELATIONAL FORM ===
(see _Urf.md)
_
### --- old pat intro ---



Start by considering all data structure classes you might find in the standard libraries for existing langauges:  

URH is a intended to be a simplest universal data-structure.
- Perhaps the simplest structure is the container.  it has a handle and it has elements handled by that handle.
- We want a data structure that captures notion of container in a general/uniform way.  Specifically the test that X contains Y should be uniform no matter the kind of container that X is.  e.g. if X is unbounded/bounded, set/multiset, functional mapping/relation, order/unordered, etc.  it should not matter... the structure relating X and Y should be uniform over all these cases.
	(There IS a uniform RDF encoding of such containers, you use 5 triples to encode a single membership relation between X and Y)
- then the check required to test 
 builder -- as such its simplicity is measured by how well 


I think my goals for URF are different from my understanding of goals of RDF.
My aim with URF is for it to be a (1) simplest, (2) universal, (3) natural, (4) uniform representational system as a single layer within a much larger convering all of computing.
The layer below URH is the bitvector, and the layer above URH is a library of data structures you might find in modern programming langauges:  (list, map, multiset, ordered-dict, tuple, queue, tree, etc.)

We the URH abstraction layer to be:
1. SIMPLEST -- its description is short.  
2. UNIVERSAL -- it can encode any part of any computational system.
		(triple are also beautifully simple and universal)
3. NATURAL -- not only does it encode all parts of all computational systems, but it does so "naturally".  This means that the entities and operations that one would intuitively use to talk about these systems are each captured as entities and operations with in URF encoding, and there are no spurrious entities or operations which exist only because of artifacts or limitations of the representational system instead of that which it encodes.
4. UNIFORM -- not only is it natural, but it is uniform.  This means that the way you represent and operate at different levels of abstraction is UNIFORM.  Both the representations and the operations are "turtles all the way down".  (in uniform it is units all the way down.)


So what is a simplest **UNIVERSAL UNIFORM FORM** which can express the inference over statements (and all other kinds of computational processing)?
- _BITVECTOR?_ -- Well ontologically this is (1) simple, (2) universal, and (4) is a uniform a basis in which to embed all computational system.  But it is NOT natural, the parts of a bit vector do not correspond well to most higher level computational systems.
- _RDF?_ -- It is higher level than a bitvector, and considerably more natural in many cases.  Still, ontologically it is at the same level as the bit vector--one can EMBED the representations and operations of an inference system in RDF, but it does not have first class entities which correspond to statement nor other parts of an inference system.  Further if you map complex computational frameworks onto RDF you end up with blank node and other entities which are not natural analogs of those systems -- it is not natural.
- _URF?_ -- This is the simplest framing I could think of which UNIFORMLY and UNIVERSALLY encodes the parts of an the inference framework, as well as other computational systems I could think of.

URF is "uniform" in that it is "turtles all the way down" -- everything is a unit, and units are composed of units, all the way down...

Uniform, (a language not described here), is built on top of URF.  It defines "functional semantics" and "operations" in a way that aim to be uniform.

So I think the goals might be a bit different:
(1) I am not satisfied with ontological sufficiency for embedding... a bitvector already gives me that.
(2) I AM centrally concerned with the naturalness and uniformity of an encoding... both are slipery ideas, that I only partially understand, but I am convinced they are super important.
(3) And I dis-beleive in the idea of a one-size-fits-all denotational semantics, I think this is a fantasy which detracts more than it adds.  (I might have a thought for an alternative to the traditional denotational approach.)


What is confusing is that each URF node can be thought of as a collection of RDF nodes, so it might seem that RDF has the same ontological status as URF.  I don't believe this is true, any more than I believe a bitvector has the same ontological status as RDF, even though we all agree they both are sufficient for encoding graphs.

CLAIM ONE:  URF is a simplest representational system where one can treat all representation and all processing uniformly on top of a single representation ... "turtles all the way down"

	URF is intended to be used as a single abstraction mechanims to be used at multiple levels, by defining one level funtionally in terms of lower levels.  you can do this with RDF, but URF is designed to give precise control over the entities and operators which exists at each level.  RDF would require giving ontological status to many "phantom" blank nodes when constructing these layers.

CLAIM TWO:  Computation requires sets with computable "closed" membership.  One can cobble such closed-world structures together using RDF, but URF "bakes in" this definiteness, directly into its formulation.  RDF, by contrast, is open and indefinite by default.

e.g. For example, what is the set of all triples that contain   <http://cnn.com>  as their subject?  This is not really a well posed question, one needs to first indicate some kind of universe or container object in order to provide scope for this question to make sense.
 

This is a subtle and as yet a muddled concept in my mind.
Still I think it is important in fully expressing computation.

~-~-~

A FEW MESSY DETAILS ABOUT URF.  
URF graphs are much more permissive than RDF graphs.
- There can be 5 triples with the same subject, predicate, and object yet they are distinct triples.
- The edges extending from a node may have an ordering imposed over them INDEPENDENT of any ordering that might be implied by their predicates.
- Further we frequently operate on structures that are (countably) infinite in various ways.
- And a few more wierdnesses too.

These things defintely DO make things messier to deal with URF.  I have kept this messy generality in URF because it I seem to need it for the larger UNIFORM universe I am hoping to build.  The issue is, if I remove things like ordering of the edges, then at higher levels of abstraction when I am forced to build structures to accomodate such info, I end up creating NON-UNIFORMITY at higher levels.  So it seems the more elegant path is to accept the complexity at this lowest level in lieu of greater sin commited above.
I am not positive of this.  It would be an interesting point to debate later with you sometime if we ever talk about the larger Uniform agenda.

Still all of the advantages discussed in this text do NOT depend upon these generalizations.  One can read this entire document as if we are talking about a simpler URF with unique triples, unordered edges, etc.  The entire document is consistent with this simpler picture.

_
### --- OLD STUFF -- URF GRAPH UNIFICATION ---

The basic idea behind URF graph merging is the straightforward linking of graphs based on the literals within those graphs in a fashion similar to URI linking of RDF.  There are a couple of differences:
1. The structure latent within URI structures are made explicit by the GLU graph of uniform literals (see next section for GLU literals).
2. These GLU literals do not "denote" anything beyond themselves.  They are like the integer 7.  It just denotes 7.


#### -- Graph Correspondance --
Here we formalized the notion of growing a correspondance mapping between two URF graphs inductively by matching triples in both graphs that have the corresponding subject and predicate vertices.

A _**partial correspondance**_, C, between two graphs G and H is expressed as some set of pairs C={<g0,h0>, <g1,h1>, ...} where g_i in G, h_i in H, and
    i != j   implies   g_i != g_j and h_i != h_j 

Given graphs G, H, and a partial correspondance, C, we say
C' is an **_extension_** of C    iff   C' = C union { <g_k, h_k> }  where
	there exists i, j, k where
		<g_i g_j g_k> in G   <h_i h_j h_k> in H  and  g_i, h_i are functional

D is the _**maximal coorespondance**_ of graphs G and H given partial corr. C iff
	there exists C_0, C_1, ... C_n where C_0 = C, C_n = D and 
		for each 0<=i<n   C_i+1 extends C_i given G and H

A correspondance, C, is said to be _**conflicted**_ iff  
	there exists i != j,  C[i]=<g_i, h_i>,  C[j]=<g_j, h_j>,  and 
		either g_i = g_j  or  h_i = h_j 


#### -- Graph Merging --
Graph Merging is accomplished unioning the units of the input graphs and then swapping each pair of corresponding units with a new combintation verticies.

items(c) is an _**order preserving merge**_ of items(a) and items(b) iff forevery i,j
	<g_i, h_i> occurs before <g_j, h_j> within items(a) or items(b)  IMPLIES
		<g_i, h_i> occurs before <g_j, h_j> within items(c), and
	items(c) is the smallest sequence satifying this.

G' is _result of **swapping**_ vertex S for A and B within graph G iff
	G's items sequences are the same as G except that each occurance of A or B is replaced by S

M is the _**merge**_ of G and H given correspondance C iff
	C is not conflicted,   M_0 = G union H,    M_n = M,  and
	for all i, 0<=i<n   M_i+1 is the result of swapping S for A and B
		where S is an order preserving merge of items(A) and items(B) and
		C[i] = <A, B>   (it is the ith corresponding pair of vertices)


#### -- Key Graph Merging Results --

**URF GRAPH MERGING**
URF graphs are merged by stitching their corresponding GLU literals together.  
Formally:
	Let LL be the GLU Literals identity coorspondance =
		union[ <L, L> ]  for all L in GLU
		
	M is the _**URF merge**_ of G and H iff M is the merge of G and H given LL


**URF LITERAL ERASURE MERGING**
URF "literal erasure" merging is a purist's form of graph merging.  

It first replaces all GLU literals in each starting graph with the corresponding vertices within the GLU graph corresponding to each GLU literal.  These resulting graphs 

uses the GLU literals SOLELY for purposes of "gluing" the graphs together, but otherwise discards all information  

determine the blank node alignments appropriate between the graphs to be merged, but once this merging is complete a 

 merging is simply merging 


RAPH MERGING

		


_
### old2
#### pat note

Mapping Notes:
- Each vertex expressed above has it 


	**{[PAT NOTE]}**
	There is some real wet ink here.  URF was created to be part of UNIFORM.  Uniform provides a model of computation, and that model of computation is used to ground URF semantics EXPLICITY.

	Thus data has explicit "grounding" links in the data which specifies how it combines with other graph data.  Graph linking by identity match is just a simplest case.  but here it would be nice to express URF w/o referece to the other complexity.

	that data would be "loaded" ino
	, and uniform makes even the semantic grounding of data explicit in the graph via "grounding" graph links.  Whether or not a node is a URF Literal or not is simply an annotation on its grounding semantics.

	But here I am trying to pull URF away from all of that machinery.  It seems one light-weight way to do this is to simply annotates each vertex with a boolean indicating whether or not it is a URF literal.  I don't really love this solution since it elevates this one bit to have huge status.  

	In practice in URF as used in Uniform whether or not to treat a vertex as a globally referencable constant is just one aspect of its semantics.  So predicates Like URL, Datetime, and Social Security number would all be treated as global literals while an assertion like Age(ssn123451234, Int(54)) would be treated as an assertion since the semantics of the age predicate indicates it is not a literal.

	I am not positive of the most elegant way to indicate Literal if one divorces URF from Uniform.
#### -- .BNF For Unit Form --

	EXPR  ::=	UNIT | NUM | STR | IDENT 

	UNIT  ::=	IDENT **"()"** 						# Zero Arg Form
	UNIT  ::=	IDENT **"("** BODY {**", "** BODY} **")"** 	# Fn Form
	UNIT  ::=	**"_("** EXPR {**", "** BODY} **")"** 		# Lispy Form
	HEAD  ::=   IDENT { **"."** IDENT } 			# 
	BODY  ::=	EXPR  |  KEY **": "** EXPR			# Key/val pair or arg
	KEY   ::= 	IDENT | STR	| META				# Path form for interp

    NUM   ::=	[+-]?([0-9]*[.])?[0-9]+ 		# REGEX for decimal
    STR	  ::=	\"(\\.|[^\"])*\"   				# REGEX for string
    IDENT ::=	[_a-zA-Z]{_a-zA-Z0-9}			# REGEX for a "bare string"	


Literals with 
int num str sym head
	
_
#### (maybe delete this CREATING HANDLES TO GROUP DATA
	
Before we replace triple stores, quads and reification, lets understand how each serves as handles for data processing.

TRIPLE STORES -- Focuses the Attention of Data Processing

If we want to use the power of and beauty the triple itself to serve as the handle for controlling the usage of triples we must first distinguish the handle from that which is handled.

==> Split RDF graph nodes into two categories:
- HANDLE CONTAINER NODES -- Container nodes used to reference and process the triple data they contain.
- ENTITY NODES -- Nodes cannot serve as containers but rather somehow relate the continer to some aspect of the world or outcome.

HANDLE CONTAINER NODES -- These RDF nodes have extra requirements.  In order to serve as an effec

==> Uniform RDF triples have the extra requirement that requires their subject nodes are handle container nodes.  And the meaning is that all triples that have this handle container as their subject are "handled" by referencing that handle node.

So what can be a handle Node?  Is < DanOblinger LivesIn SanFrancisco > a good triple?  DanOblinger seems like an ok handle for all info about that person right?  Wrong.  There is no effective decision procedure for determining if a triple should be included in such a bucket of info.  And given such an ill-defined bucket of info it is not possible to effectively wield it for an information processing task.

But Facebook-friends-of-Dan-Oblinger-on-July-14-2009 *is* a resonable bucket.  There is social / computational method of deriving the contents of this bucket, and it is specified well enough that one can effectively wield it in subsequent data processing tasks.

This bucketing requirement is not so onerous. RDF triple reification can be used to transform any bare triple like < DanOblinger  LivesIn  SanFranciscoCA > into 
< _N7093 Predicate LivesIn >
< _N7093 Person DanOblinger >
< _N7093 Location SanFranciscoCA >

_
#### XXXXXX  A "bare" triple has no meaning.
### old


__ 
We say a URF graph is _**taxonomized**_ if it contains a DAG of "bucket" units as defined below, and we say it is _**untaxonomized**_ if it is not known to contain such a DAG.  If no indication is made we assume a URF graph is taxonomized by default.  Intuitively this means closed means there is a nested DAG of "bucket" units that contain all nodes.  (see semantics discussions explaining this preference for closure)

We say the set of vertices, S, is the **_closure_** of vertex, V given graph G iff
	S is the smallest set which contains V and 
	Forall g, h in G, and Forall s in S
		h in S  if  <s g h> in G
We say a graph G, w. "root" vertex R, and bucket predicates {B_i} is _**closed**_ iff
	G is a URF graph containing R and all predicates B_i,
	the transitive closure of B_i starting a R forms a DAG D_j in G, and
	forall g in G there exists exactly one B_i where g is in the closure of B_i
	

We use the notation  <A B C>  to indicate that unit A with in a graph has an items(A) sequence that contains the pair (B,C).  This is similar to the idea of triple in RDF.)

#### --- x UNIT FORM ---

	**TL;DR -- UNIT FORM is mostly just JSON, but looks like a Python function call.  It has the obvious ordering "<" and equality "==" operators defined over it.**

	Let THEORY be the data graph defining the JSON data semantics,
	this entire section used THEORY as its implied data graph. 

	_Unit Form_ is a compact notation for making an asserting about the set of units in the data graph that begin with some unit u:
	**u = head(va, ..., vn, k0: v0, ..., km: vm)**  
		is shorthand for 
	<u HEAD head>
	<u 0 va>
	...
	<u 1 vb>
	<u k0 v0>
	...
	<u k0 v0>

	NOTE:
	- Unit Form is only defined for units with finite structure.
	- Unit Form is bounded; is both an assertion about elements that are in G, and an assertion that no other triples beginning with u are in G.
	- URF theories themselves need not be bounded in the sense that there may be units whose full structure is infinite or is not known; those units simply have no defined unit form.
	- Note 

	We use "fn(...)" as shorthand for u in THEORY where u=fn(...)
	We use "x < y" as shorthand for LESS(x, y) in JSON
	We use "x = y" as shorthand for EQUAL(x, y) in JSON
	We use "z = x + y" as shorthand for APPEND(z, x, y) in JSON


	Let JSON be the set of "atoms" of the JSON_THEORY

	A set S is unique iff
	For all x, y in S 
		EQUAL(x, x) and
		EQUAL(x, y) implies x=y


	Let NAT_ZERO, STR_ZERO, LIST_ZERO,
	HEAD, EQUAL, LESS, 
	NAT, INT, NUM, and STR 
	are all elements in JSON_THEORY


	Let Nat be the set of _natual numbers_ where 
	NAT_ZERO in Nat and
	For all x in Nat there exists y in Nat where
		SUCC(x, y) in JSON_THEORY, and
	For all x, y, z in Nat if SUCC(x,y), SUCC(x,z) then y=z
	For all x, y, z in Nat
		if SUCC(x, y) then LESS(x, y), and
		if SUCC(x, y) and LESS(y, z) then LESS(x, z)

	Let Str 	be the appendings of Nat,	STR_ZERO
	Let Set		be the appendings of Expr,	SET_ZERO, LESS
	Let List 	be the appendings of Expr, 	LIST_ZERO
	Let Items 	be the appendings of Pair,	MAP_ZERO 
	Let Map 	be the appendings of Pair,	ITEM_ZERO, LESS 

	A set, _S_, is the _**appendings of**_ some basis set _B_, zero element, _z_, 			and optional limiter predicate, L, iff
	z in S, and
	for all b in B, and s1 in S
		there exists a unique s2 in S such that
			either APPEND(s2, s1, b) or L(s1, b) in THEORY

	Extending LESS, EQUAL, and TYPE predicates
	for all s1, s2 in S, and b in B
		APPEND(s2, b, s1) implies LESS(s1, s2)
	for all s, s1, s2 in S, and b1, b2 in B
		where APPEND(s1, s, b1) and APPEND(s2, s,  b2)
			LESS(b1, b2) implies LESS(s1, s2) and
			EQUAL(b1, b2) implies EQUAL(b1, b2)  

	Let Negatives be the set of 
	Let Int be the set containing Nat and N
	Let Expr 


	
	

	Let Str(k) be a subset of JSON such that
	EMPTY_STR in Str, and
	For all s1,s2 in Str and n < k in Nat 
		there exists exactly one u in JSON_THEORY where
			APPEND(s2, n, s1)
		



	(See Slide for details)
	-- Object Centric Graph Patterns



	- <u, k, v> -- denotes a triple, a combination of 3 units from U
	- **K** -- denotes the set of all **DEFINED** triples, a subset of all possible triples of **U**.  K < (**U** x **U** x **U**).
	- s -- denotes a unit from U\A used as a 1nd triple arg.
	- k -- denotes a key, a unit from U used as a 2nd triple arg.
	- v -- denotes a value, a unit from U used as a 3nd triple arg.
	- **M** -- denotes the set of special "meta" key nodes.
	- bk -- denotes a base key, a key drawn from, **BK** == U\M
	- . May not be unit in triple in K
	- **NN** -- the natural numbers 0, 1, 2, 3, ...  (usually in **A**)
	- **DEFINED** -- a triple, T, is defined iff T in **UM** 


	UNIT-FORM


	TL;DR -- UNIT FORM is mostly just JSON, but it looks like a python function call.  

	_Unit Form_ is a compact notation for encoding the relational information associated with some unit u:
	**u = head(va, ..., vn, k0: v0, ..., km: vm)**  
		is shorthand for 
#### x older UNIT-FORM

	Let **M** be a set of atoms disjoint from **A** and **U** (called _meta keys_).
	Let **head** be specially designated meta key in **M**.
	Let meta(u) be the items(u) for the relations in this meta graph.
		meta is unordered, finite, and functional.

	**BACKING TERMINOLOGY**
	- **meta(_u_, _k_)** is notation used for u[k] within the meta graph.
	- **head(u)** is meta(u, head)
	- **REL(u)** == meta(u) + items(u)
#### x junk
	INT	  ::=	[ **""** | **"-"**] DIGIT { DIGIT }
	NUM	  ::=	INT **"."** { DIGIT } 
	PATH  	::=	[STR|IDENT] {**"."** [INT|STR|IDENT]}	# Simple Path
	INDEX ::=	INT | STR | IDENT
	PATH	::= **'"'** PATHSTR **'"'** 
	PATHSTR ::=	NODOT {**"."** NODOT}  |  **""**		# Path inside string
		    /"([^"\\]*(\\.[^"\\]*)*)"/
#### x --- LIST FORM ---
	LIST FORM -- A lossless mapping from Expr onto JSON lists, numbers and strings

	LEXPR  	::=	 LUNIT | LIDENT | INT | NUM | LSTR 
	LUNIT  	::=	 **"["** LHEAD {**", "** LBODY} **"]"**
	LBODY  	::=	 EXPR  |  LKEY **", "** LBODY

	LHEAD	::=	 **(" )**  PATHSTR  **(")**		# SPACE prefix
	LKEY	::=	 **(":)**  PATHSTR  **(")**		# COLON prefix
	LSTR	::=	 **("')**  PATHSTR  **(")** 	# QUOTE prefix
	LIDENT	::=	 **(")**    IDENT   **(")** 	# NO prefix

##### List Form Examples

	f(x)						[" f", "x"]
	foo.bar(x)					[" foo.bar", "x"]
	f(a.b: 3)					[" f", ":a.b", 3]
	f(1, "two", pi: 3.14)		[" f", "'two", ":pi", 3.14]
	f(g(x: true))				[" f", [" g", ":x", "true"]}
	"*"(3, 4, 5)				[" *", 3, 4, 5]
	woha("one", x:y, "two")		[" woah", "one", ":x", "y", "two"]
	_(head(1), body)			[ [" head", 1], "body"]	
#### x Unit-form

	head(arg, ...) {body, ...}

	OPT1: head(TUP(arg, ...), body, ...)
	OPT2: head(arg, ..., BLK(body, ...))

	OPT1:
	- format of some unicore stmt like BRA might be simplier. e.g.  (BRA "<-"(x<5, print "tiny"))
	but not quite, since this still does not have a TUP for x<5
	OPT2:
	- Args are found in the same place both with and w/o a body
	- Different handling of body args vs. head args is natural here since it is part of a BLK

	_

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
_

#### -- OLDER INFO --
Problems with RDF that are improved using the URF encoding.
1. LATENT INFORMATION IS MADE EXPLICIT IN GRAPH STRUCTURE
2. STOP-GAP HACKS REMOVED: Literal Typing, Reification, Quad Notation, and Triple Store
3. VERTEX CENTRIC DATA ACCESS IS MADE UNIFORM

- **PRINTABLE** -- think of a bounded or printable unit as JSON expression or LISP sexpr -- They are structures that can be written down on paper.
- **COMPOSITE** -- Think of units as containers that contain other units or think of them as RDF blank nodes.  Don't think of them as URIs since they cannot directly coorespond to any unit within any other graph.
_
#### -- 

~-~~

PRETENSE OF DENOTATION, GONE! -- ``Data is what data does.''  With traditional RDF and traditional URIs there is a notion that triples intrinsically "denote" things.  With URF the meaning of triple data is tied ONLY to how it is obtained for data processing and how it is used in data processing.     The new functional meaning of data is both better defined and more usable.  (See )
## === NODE - NODE CENTRIC GRAPH ACCESS ===
### _

NODE := GET + SET + ITR + LEN + GND + INV + NEW + DEL
NODE := IDX + ITR + LEN + MET + INV + NEW + DEL


**NODE** -- _NODE_ is a vertex-centric, cleavably-typed interface for graph data.
	
**VERTEX-CENTRIC** -- The interface is _vertex-centric_.  This means all operators accept a single vertex as a common parameter, and generally provide information regarding that one vertex and the edges directly impinging upon that vertex.

**-TYPED** -- The interface is _typed_.  This means each graph vertex is associated with one of an enumerated set of types, and this type provides implementations for all graph access operators.

**CLEAVABLY-** -- The interface is _cleavable_.  This means that a graph vertex indicator (pointer) may be "cleved" into an indicator of its type structure, and a separate handle which indicates the vertex, but only in conjunction with the typing structure.  (see cleavable-typing discussion for its relationship with type erasure)

_
#### -- TYPE DRIVE AND TYPE ERASURE --

An important simplification made in most programming languages is to perform some compile-time cleaving of verticies into vertex type and vertex handle where code associated with each vertex type is simplified since in must only operate on handles of that type, and whose handles are simplified since they must only encode for a given vertex type.


 specific behaviors into 'types' of verticies with related 
_
#### -- TYPE ERASURE AND VERTEX CLEAVING --

First two cleved-type encodings:

The C language cleaves the type information about the contents of a struct from the runtime pointer to that struct.  Only by knowing both the runtime pointer and the compile time type infomation could one reconstruct the graph data indicated by that struct.

The rows within a single database table could be mapped onto the integer ids for those rows.  In this way one could create a cleved type for those rows. 
The handle for each vertex would simply be an int, and the grounding for the type would be the accessors of for the parts of each row as indicated by an id lookup in the database.  Thus according to this cleving a simple int would be the stand in for the subdata graph encoded by the database row.

In both cases this cleaving is tied with the notion of type erasure within programming languages.  The commitment here is that access to nodes within the data graph is always managed by code which is associated to some grounding structure.  This means that the way that the data is processed must be computable from some code common to all elements of the graph vertex type.
_
#### -- GENERALITY DISCUSSION --

The Node interface elevates the imporance of a concept like type erasure incredibly by pushing it so incredibly far down the spiral.  Why?  At the same time vertex-centic graph operations are given primacy while many other valuable graph query patterns are relegated to much higher levels within the spiral.  What is going on here?



_
### -- API --

**def class** UnitSynthetic(T):
	**def op** UnitSynthetic(base, ns: NS(extends: T))

**def class** UnitGrounding(K, V):
	**def op** _get_(base Unit, key Unit) -> Unit
	**def op** _set_(base Unit, key Unit, value Unit) -> Unit
	
	**def op** _idx_(base Unit, key K) -> Place(V)
	**def op** _len_(base Unit) -> Int
	**def op** _itr_(base Unit) -> Stream(K, V)
	**def op** _inv_(base Unit) -> Unit
	**def op** _met_(base Unit) -> Unit
	**def op** _new_(base Unit, spec Spec, ...args) -> Unit
	**def op** _new_(base Unit, ns: NS) -> Unit(K => V)
	**def op** _del_(base Unit)
	
	**def op** _base_(base Unit) -> base
	**def op** _ns_(base Unit) -> NS
	


**def class** _Klass_:  // really a Construct is a kind of unit Klass
	**def op** _head_(u Unit) -> Ident
	**def op** matches(u Unit, template Unit) -> Unit
	

**def class** _Place_: 
	**extends**: Executable 
	**def op** _assign_(env Env, _value_ Unit) 
	
	
_
### --- METHODS ---
#### --- IDX ---

	**IDX** -- The index operator returns a Place given a 'self' unit and a 'key' vertex.  

	**def op** _idx_(self Unit, key Unit, ->Place)

	**def class** Place(BASE, VALUE):
		def var base: $BASE
		def var key: IdentKey
		def var value: $VALUE

	_
#### --- NEW ---
##### _

	**NEW** -- The _new_ operator creates and returns a newly allocated unit instance based using the calling unit as its seed -- its controlling factory.

	**def op** _new_(gnd, args..., keys:...) -> Unit

	Env.exe(SEED)
	CODE()   # 2020-02-01 -- this works, but it is not correct???

	Details:
	- Factory itself controls the persistence of the created unit.
	- The 'gnd' parameter connects unit to the unit providing its semantic grounding -- all structural and functional semantics 
		(other than the semantics of it persistence properties)
	- The 'args' parameter provide listy elements for the unit.
		throws alt.new.not_listy if unit is not listy.
	- The 'keys' parameter provide non-listy elements.
	_
##### - API -

factory.new(type Gnd, args Unit )

_
##### -- Semantics --

	def fn new(gnd Gnd, )

	_
##### -- Choices --
###### RESOLVED: QUESTION How to invoke 'new' on a backing heap store?

	RESOLVED: A seed / factor is a function that returns new instances.
	RESOLVED: The root heap/store object accepts a single arg, the unit's grounding.
###### CHOICE: No Fixed Args For New
	ISSUE: different stages in construction seem to need different args.

	**OPT NO_FIXED_NEW** -- Just don't have a fixed API for new
	- A Backing Store always accepts one arg, the unit's ground.
	- BackingStore.new is recursively called by the class constructor
_
###### CHOICE: SEED_ARGS
		OPT first arg is unit template
		OPT include_vararg_slots:	Include varargs number of arg values
		OPT exclude_vararg_slots:	Use items to encode any arg values

		CONSIDERATIONS:
		- INLINE CONSTRUCTORS (++include_args) -- 
###### CHOICE: SEED_ARGS - FirstArgTemplate
		Choice Arg1: Use the 1st arg of seed factories for unit template
		Choice Key:  Use the 'unit' key of factories for unit template

		Arg1
		- Natural -- very natural to call a type passing a template
		- Pythonic -- very pythonic way to instantiate a collection
		- Faster -- in case that zero kwargs passed, then 
###### CHOICE: SEED_LEN
		OPT include_len: 	Include 'len' as a keyword to seed.
		OPT derive_len: 	Derive 'len' from the number of 'args'

		ISSUES:
		- LARGE ARRAYS (++include) -- Using a seed constructor would be the natural way to copy large units.  But this would fail for on derive_len for large arrays since it would require too many arguments.
		- ONE MORE ARG (++derive) -- Adds one more arg to precious call
		- SPARSE LIST (++include) -- Nice spec for spare lists
		- SIMPLICITY (++derive) -- Need to handle case where len is more or less than number of args.
###### CHOICE: Goals
		- RANGE -- applies to:  collections, classinst, atoms
		- STD CONSTRUCTOR -- Able to create objects in normal py way
		- LOAD FROM CODE -- Able to directly create load form from another load form code.  (should be max efficiency method)
		- LOAD from STRUCT -- Able to create from unit structure

		- CODE FORM -- Able to create code form from code too.

		- FACTORY BACKINGS -- Allow classes to override their standard backing and/or have a seed mechanism that allows binding a factory to any var
		- IDE.VARS -- Hint the IDE so it knows class attrs are defined
		- BOILDER PLATE -- Avoid boilerplate on var declarations
		- CONSTRUCTOR FORMS:
			- inst = address("101 Bache", zip="94110")
						unit.build(head="Address", args=[], kwargs={})
			- inst = Person.code()
			- inst = seed()
####### Factory
		- All uniform creation is factory creation
		- Allow std class creation too???  adjust metaclass, no UserList
		- Factory two parts:  backing, ns-interface
		- CALL FORM -- Constructor data that will create current state

		META_FACTORY(backing_seed, ns_interface) --> factory
		factory2 = factory.set(seed=new_seed)
		factory(constructor_form) --> instance

		UF LOAD(code_form)
		- factory = env.ns.path_get(code_form.head)
		- unit = factory(call=code_form)
			- seed = self.seed
			- arg_spec = self.klass
			- unit = seed()
			- units.structure_set(unit, 
						template=code_form, arg_spec=arg_spec)
			- unit._ init _(IGNORE_ARGS)
			- return unit
##### -- OLD --
###### v9
	The Ident is used to lookup the unit's FrameSchema which provides limits on values the new structure might hold, as well as the namespace to associate, this namespace provides semantic bindings for any execution that is controlled by this new unit.  (e.g. interpretation of a method's body code).


	_
###### v8

	FORM
	(a) 	ClassInitializer( load_form... )
	(b)		ClassInitializer( structure_template, qualifiers... ) 
	(c)		u.NEW -> seed
	(d)		seed( load_form... )  

	SEMANTICS
	- LOAD_FORM -- The code used to create the denoted_form
	- #A INLINE -- Form used in source code to create new denoted
	- #B STRUCTURE -- Form use in source to create new strucuture
	- #C SEED -- Form used get seed underlying an instance
	- #D SEED BASED NEW -- Form used to create from seed

	STEPS:
	- Structure implementer used to create new structure
	- Structure passed to _ init _ to initialize denoted


	Semantics.
	- Typically a seed will produce a unit of a single fixed type.
	- Template is a mappy structure used as basis for initial structure of the created unit.  (never a part of the unit.)

	- lang_load(x) -- performs load, default is to call _ init _(x)
	Non-structural Units
	- Units that accept 

	p = Person("dano", Address("101 bache", zip=94110))
	lst = list([1,2,3])
###### v7

	u.NEW -> seed

	seed(*slots, ns, head, len, unit)

	**ARGS**
	- **slots** -- fields providing the 'raw' data for unit.
	- **ns** -- namespace for unit, or ns of 'unit' is used.
	- **head** -- head for unit, or head of 'unit' is used.
	- **len** -- len for unit, or len of 'unit' is used.
	- **unit** -- The template unit used to fill in the items for this new unit, and potentially its ns, head, and len as well.

	**seed semantics**
	- **creation** -- The seed function creates a new seed with a specified backing, of a specified type, and specified head.  
	- **ignore** -- Takes what it needs, ignores what it does not.
		It may or may not attends to the: head, len, ns keys

	semantics
	- seed -- may use or ignore: head, len, ns
	- items -- if specified, then its values are used

	behaviors
	- If seed has 'head_set' method it is as if it were called to set the head.  It defaults to the type name if not specified.
###### v6
		NEW -> factory -> 
		factory(unit, items, args, kwargs, meta, head, len, fns)

		repr(i=[])

		load(code) -> lookup(code.head).seed(args=, kwargs=)
			u = unit_initialize(args, kwargs)
			u._ init _(* args, ** kwargs)
###### v5

		create(back:, seed:, fns:)
			-- if 'back' then wrap it
			-- if 'seed' then NEW it
###### NEW v1,v2,v3,v4
		NEW(items:, head:, len:, ns:)
		- items:  sequence of key/value pairs for unit.
		items -- sequence of items (which are sequences of at least two elements, a key and value).  If supplied

		ISSUE: Like most OO-langs we need way to separate creation from initialization (so init can be chained)
		ISSUE: Need way to have all instances created from factoryies like Java spring.  
		ISSUE: Need effective way to create immutable forms

		NEW Call Chain
		- Class constructor is called:   f=Foo(a, b)   with ns active
		- x = ns.lang.seed.heap.NEW(ns[`Foo])
		- f = x.INIT(a, b)
		- Unpickling of instance:   f=

		IDEA FOR NEW HEAD / TYPE 
		-- NEW always accepts the ns for the unit being created as its first arg.
		-- Some NS define head as an access to the special meta key '__'
		-- Special initializer used to instantiate Code forms. Code(head, ....)

		V3
		code.NEW = u(content ?Unit, fn=Fns)

		_
		NEW v4

		NEW(initializer, __ head __:, __ fns __:)
#### --- api ---
			// UNIT -- 
			doc Unit:
				GET = Gets the value associated with a unit's key
				SET = Updates the value associated with a unit's key
				LEN = Returns 'listy' len of unit.  
					  Int length or 'inf' if unit is listy.
				ITR = Returns a Stream of key/value pairs for unit
				INV = Returns u containing inverse links into unit
				FNS = Fns that define the semantics associated of unit
				NEW = Unit factory
				DEL = Unit destructor

			def UnitBacking(KEY=Sym, VALUE=Unit):
				GET = op(k KEY) -> VALUE									
				SET = op(k KEY, v VALUE) -> self_class
				LEN = op() -> Int || Inf
				ITR = op() -> Stream(UNIT=self_class)
				INV = op() -> Unit
				FNS = op(u ) -> Fns
				NEW = op(initializer Unit=null, fns=Fns) -> Unit()
				DEL = op() -> Und


			// STREAM --
			doc Stream:
				close     = Flushes changes;
							terminate connection to underlying unit.
				flush     = Updates the underlying unit with any pending modifications, and 
							optionally waits for completion.
				peek      = Returns the currently indexed value in the unit
				read      = Returns the currently indexed value in the unit and advances the index
				write 	  = Writes the currently indexed value 
				next      = 
				listeners = 
				history   =
				^eval     = Returns true if the stream is not at its end

			def Stream(UNIT=Unit(KEY=KEY, VALUE=VALUE)):
				close     = op()
				flush     = op(wait=bool)
				read      = op() -> VALUE
				write     = op(v VALUE)
				next      = 
				listeners = 


#### SEMANTICS DISCUSSION (mappy & listy)

	SEMANTICS
	- GET - only defined when unit's key is functional
	- SET - returns unit with value updated (might be mutable)
	- APPEND - adds a new value to the unit

	MERGING LIST AND MAP
	    __len__		Is the same for both list and map
	    GET/SET 	Of single index term works the same for both list and map
	    __bool__ 	Always true, even when __len__ == 0
	    DEL 		Is mapped to SET UND
	    ITEMS 		Is defined for list as enumerate(list)
	    VALUES		Is defined for list as self. (to be consistent with map)
	    KEYS		Is defined for list as range(len(list)) 
	    APPEND		Is defined for maps as map[unique]=v where
	                'unique' is some chosen key that isn't in the map
	    SLICE		Is defined for maps as a slice of the items list.
				    (will error if map is not ordered)
	    __iter__	Is set to 'values()' but should be avoided if an object 
	                might be a Python map since its iter is actually 'keys()' 

	CHIMERA SEMANTICS
	- expr_args	scans 
### --- CONTAINER OPS --- 
#### update 
Iterates over 
#### copy
- **deep** - specifies that copying should be done recursively
- **source** - Uses 'self' as source, or '**source**' if specified.
- **target** - If '**target**' is specified, data is copied there.
	Otherwise new unit is created as target.
- **seed** - used as alternate factory for all composite units
- **atom** - used as an alternate factory for all atomic units
### --- OLDER ---
#### -- Notes from UF class --

"""

=== METHODS 2020-01-24 ===
- Unit  Always(DEL,MET)  Factory(NEW), Iterable(ITR,LEN), Mappy(GET), Mutable(SET), Invertible(INV)
  - UnitCollection

MISSING METHODS:  List (head,items)   Map(head,iter)

=== IMPLEMENTATIONS ===
- Unit
  - UnitStructureAdapter
    - UnitListAdapter
    - UnitInstanceAdapter
      - UnitElectricInstance
    - UnitMapWrap
      - UnitElectric (Listener, Historical, Lazy, Memoized)
      - Expr
      - Tree
   - UnitAtomWrap
     - Symbol

=== MIX-INS ===
    @CAN
 -  append  --  LISTY  --  Defines slicing and appending
 -  path_up --  PATHY  --  Defines path_up and all path operators
 -  set     --  MUTABLE --


=== HELPERS ===
 -  declare(klass, head, args, kwargs)  --  Declares a pyclass as a unit type
 -  declare_head_classes(head1=klass1, ...) --  Declares mappings from head to class

=== UNIT BACKING ===   (see units.py)

=== UNIT INTERFACE ===
 -  __init__(args, kwargs, **meta)      --  Creates and initializes a new unit.
 -  EXE(expr)                           --  Evaluates 'expr' in the context of self=unit
 -  NEW(template)                       --  Creates a new unit
 -  head()                              --  Returns head symbol for this unit
 -  code()                              --  Returns the form that evaluates into this unit  (where does structure go?)



CREATION:
load( u(head, a1, k1=v1) ) --> u=get_klass(u.head()).__new__(); self.data,self._kwargs=...;
                self.__init__(*self.data, **self.kwargs)
Klass(

CLASS HIERARCHY:
UnitBacking         --  The defining methods that underlie the structure of a Unit.
  Unit
    UnitAdapter     --  Abstract adaptor class that adds structure operators to any base class
      PyUnit

Unit
  UnitAtomAdapter
    UnitPyAtom
    UnitContainerAdapter
      UnitPyMap
      UnitWrapper


"""
## === FNS ===
### >> PLACE <<
### -- access --
### -- assign --
### -- type --
### -- path / ident -- 
### -- def?? --
### -- uf.how --
### -- uf.why --
### >> UNIT OPS <<
### -- head --
HEAD -- Ident associated with this unit instance.
### -- id --
ID -- An int value unique to this object
### -- nub --
### -- ops --
OPS -- Tree of sematic associations underlying interpretation over this object.
### -- type --
TYPE -- 
_
### -- ISA --
### -- base --
this is the unit 'under' this unit

### >> COLLECTION OPS <<
### -- place --

**PLACE** -- returns the sub-place given a dynamic place.  

does this also work on a static places?

_
### -- items --
**items** -- List of 
### -- index.html / zero key / "_" --
## === UFORM - UNIT FORM ===
### --- UNIFORM NOTATIONS ---
#### ...

**ITEM LIST FORM** -- list of k/v pairs with special 'GAP' key when needed.

**PLACED ITEM FORM** -- 

given an item <k, v> its placed item form is:
	<k, v>			if k is unique identifer (and 'GAP' nor an Int)
	<i, v>			if k is 'GAP'
	<i, __(k, v)>	if k not unique or an Int value

**UIF - UNIT ITERATOR FORM** -- Encodes each unit as an iterator over its edges.
- The iterator returns a key and a value on each iteration.
- This included edges (key) from a separated meta-key graph.
- The first iteration always yields the meta-key "^head" along with a value.



**UEF - UNIT EDGE-SET FORM** -- Encodes unit data as a set of edges, including edges from a seperate graph of meta data associated with each vertex.  Each edge has a value, it MAY have a labelling key, and it MAY have an enumeration nu

	UEF = Set of edges, where
	EDGE ==  EDGE(key, value, enum)


**UF - UNIT FORM**

HEAD([KEY=]VALUE, ...)

- KEY= is omitted on unlabelled edges.
- KEY= may be KEY^ if key is a meta key.      ???????
- If head is not a string then the surrogate head '_' maybe used, and head becomes first argument in the unit form body

choice
confusion of meta key as value of 'key' in pair

??? copy enum into key for lists ???


BBF - BLOCK BODY FORM



DISCUSSION 
During parsing blocks 
	


**UPF - UNIT PARSE FORM** --

element_value
`:(key, value)
`:(key, value)


**ULF - UNIFORM UNIT LIST FORM** -- A varient of UPF that encodes all data for a single unit as a single flat list:

	[HEAD, EDGE, ...]    where 'EDGE' is one of:

	VALUE					-- Recursively specified unit data value
	":KEY", VALUE			-- 
	[':', KEY, VALUE]

#### -- EDGE REPRESENTATIONAL FORMS --

KEY CONSIDERATIONS

INDEX -- Edges may be indexed, if they are then the 'get' operator will return the value associated with the edge.

INDEX-VALUE-PAIR -- So one representation for an edge is info passed to 'get' and info returned from 'get'

INDEX-FORK -- meaning of 'index' is different sometimes it is the position within the iterator, and sometimes it is the key

UNIFIED TRANSPORT FORM -- Single lossless form covering all graph node patterns

TRANSPORT INVARIANCE -- Any pattern should map into transport and back w/o loss

CODE INVARIENCE -- One node pattern 'code' should losslessly encode all others


CHOICE: when unit is packaged as a 'code' element can we index by position?
ISSUE:  We really want to, since paths and placement needs 'get' to have good values for lists.


### BNF
UNIT -- A data form with these parts:  head, ns, arglen, items
UNIT FORM -- A simple lossless mapping from Expr units onto Text

EXPR  ::=	UNIT | NUM | STR | IDPATH 

UNIT  ::=	PATH **"()"** 						# Zero Arg Form
UNIT  ::=	PATH **"("** BODY {**", "** BODY} **")"** 	# Fn Form
UNIT  ::=	**"_("** EXPR {**", "** BODY} **")"** 		# Lispy Form
BODY  ::=	EXPR  |  PATH **": "** EXPR
PATH  ::= 	IDENT | STR						# Path form for interp

IDPATH::=	IDENT {**"."** [IDENT|INT|STR]} **")"**

	    NUM   ::=	[+-]?([0-9]*[.])?[0-9]+ 	# REGEX for float
	    STR	  ::=	\"(\\.|[^\"])*\"   			# REGEX for string
	    IDENT ::=	[_a-zA-Z]{_a-zA-Z0-9}		# REGEX for an ident
### --- Unit Form Examples ---

"f.g"(x: v.3.2(1))
_		
### --- UNIT FORM ---

**TL;DR -- UNIT FORM is mostly just JSON, but looks like a Python function call.  It has the obvious ordering "<" and equality "==" operators defined over it.**

Let THEORY be the data graph defining the JSON data semantics,
this entire section used THEORY as its implied data graph. 

_Unit Form_ is a compact notation for making an asserting about the set of units in the data graph that begin with some unit u:
**u = head(va, ..., vn, k0: v0, ..., km: vm)**  
	is shorthand for 
<u HEAD head>
<u 0 va>
...
<u 1 vb>
<u k0 v0>
...
<u k0 v0>

NOTE:
- Unit Form is only defined for units with finite structure.
- Unit Form is bounded; is both an assertion about elements that are in G, and an assertion that no other triples beginning with u are in G.
- URF theories themselves need not be bounded in the sense that there may be units whose full structure is infinite or is not known; those units simply have no defined unit form.
- Note 

We use "fn(...)" as shorthand for u in THEORY where u=fn(...)
We use "x < y" as shorthand for LESS(x, y) in JSON
We use "x = y" as shorthand for EQUAL(x, y) in JSON
We use "z = x + y" as shorthand for APPEND(z, x, y) in JSON


Let JSON be the set of "atoms" of the JSON_THEORY

A set S is unique iff
For all x, y in S 
	EQUAL(x, x) and
	EQUAL(x, y) implies x=y


Let NAT_ZERO, STR_ZERO, LIST_ZERO,
HEAD, EQUAL, LESS, 
NAT, INT, NUM, and STR 
are all elements in JSON_THEORY


Let Nat be the set of _natual numbers_ where 
NAT_ZERO in Nat and
For all x in Nat there exists y in Nat where
	SUCC(x, y) in JSON_THEORY, and
For all x, y, z in Nat if SUCC(x,y), SUCC(x,z) then y=z
For all x, y, z in Nat
	if SUCC(x, y) then LESS(x, y), and
	if SUCC(x, y) and LESS(y, z) then LESS(x, z)

Let Str 	be the appendings of Nat,	STR_ZERO
Let Set		be the appendings of Expr,	SET_ZERO, LESS
Let List 	be the appendings of Expr, 	LIST_ZERO
Let Items 	be the appendings of Pair,	MAP_ZERO 
Let Map 	be the appendings of Pair,	ITEM_ZERO, LESS 

A set, _S_, is the _**appendings of**_ some basis set _B_, zero element, _z_, 			and optional limiter predicate, L, iff
z in S, and
for all b in B, and s1 in S
	there exists a unique s2 in S such that
		either APPEND(s2, s1, b) or L(s1, b) in THEORY

Extending LESS, EQUAL, and TYPE predicates
for all s1, s2 in S, and b in B
	APPEND(s2, b, s1) implies LESS(s1, s2)
for all s, s1, s2 in S, and b1, b2 in B
	where APPEND(s1, s, b1) and APPEND(s2, s,  b2)
		LESS(b1, b2) implies LESS(s1, s2) and
		EQUAL(b1, b2) implies EQUAL(b1, b2)  

Let Negatives be the set of 
Let Int be the set containing Nat and N
Let Expr 


	
	

Let Str(k) be a subset of JSON such that
EMPTY_STR in Str, and
For all s1,s2 in Str and n < k in Nat 
	there exists exactly one u in JSON_THEORY where
		APPEND(s2, n, s1)
		



(See Slide for details)
-- Object Centric Graph Patterns



- <u, k, v> -- denotes a triple, a combination of 3 units from U
- **K** -- denotes the set of all **DEFINED** triples, a subset of all possible triples of **U**.  K < (**U** x **U** x **U**).
- s -- denotes a unit from U\A used as a 1nd triple arg.
- k -- denotes a key, a unit from U used as a 2nd triple arg.
- v -- denotes a value, a unit from U used as a 3nd triple arg.
- **M** -- denotes the set of special "meta" key nodes.
- bk -- denotes a base key, a key drawn from, **BK** == U\M
- . May not be unit in triple in K
- **NN** -- the natural numbers 0, 1, 2, 3, ...  (usually in **A**)
- **DEFINED** -- a triple, T, is defined iff T in **UM** 


UNIT-FORM


TL;DR -- UNIT FORM is mostly just JSON, but it looks like a python function call.  

_Unit Form_ is a compact notation for encoding the relational information associated with some unit u:
**u = head(va, ..., vn, k0: v0, ..., km: vm)**  
	is shorthand for 
### older UNIT-FORM

Let **M** be a set of atoms disjoint from **A** and **U** (called _meta keys_).
Let **head** be specially designated meta key in **M**.
Let meta(u) be the items(u) for the relations in this meta graph.
	meta is unordered, finite, and functional.

**BACKING TERMINOLOGY**
- **meta(_u_, _k_)** is notation used for u[k] within the meta graph.
- **head(u)** is meta(u, head)
- **REL(u)** == meta(u) + items(u)
### junk
INT	  ::=	[ **""** | **"-"**] DIGIT { DIGIT }
NUM	  ::=	INT **"."** { DIGIT } 
PATH  	::=	[STR|IDENT] {**"."** [INT|STR|IDENT]}	# Simple Path
INDEX ::=	INT | STR | IDENT
PATH	::= **'"'** PATHSTR **'"'** 
PATHSTR ::=	NODOT {**"."** NODOT}  |  **""**		# Path inside string
	    /"([^"\\]*(\\.[^"\\]*)*)"/
### --- LIST FORM ---
LIST FORM -- A lossless mapping from Expr onto JSON lists, numbers and strings

LEXPR  	::=	 LUNIT | LIDENT | INT | NUM | LSTR 
LUNIT  	::=	 **"["** LHEAD {**", "** LBODY} **"]"**
LBODY  	::=	 EXPR  |  LKEY **", "** LBODY

LHEAD	::=	 **(" )**  PATHSTR  **(")**		# SPACE prefix
LKEY	::=	 **(":)**  PATHSTR  **(")**		# COLON prefix
LSTR	::=	 **("')**  PATHSTR  **(")** 	# QUOTE prefix
LIDENT	::=	 **(")**    IDENT   **(")** 	# NO prefix

#### List Form Examples

f(x)						[" f", "x"]
foo.bar(x)					[" foo.bar", "x"]
f(a.b: 3)					[" f", ":a.b", 3]
f(1, "two", pi: 3.14)		[" f", "'two", ":pi", 3.14]
f(g(x: true))				[" f", [" g", ":x", "true"]}
"*"(3, 4, 5)				[" *", 3, 4, 5]
woha("one", x:y, "two")		[" woah", "one", ":x", "y", "two"]
_(head(1), body)			[ [" head", 1], "body"]	
### SEE IDENT for IDENTSPEC
### Unit-form

head(arg, ...) {body, ...}

OPT1: head(TUP(arg, ...), body, ...)
OPT2: head(arg, ..., BLK(body, ...))

OPT1:
- format of some unicore stmt like BRA might be simplier. e.g.  (BRA "<-"(x<5, print "tiny"))
but not quite, since this still does not have a TUP for x<5
OPT2:
- Args are found in the same place both with and w/o a body
- Different handling of body args vs. head args is natural here since it is part of a BLK

_
### Choice block form
# ### FUNCTIONAL ###
## _
### choices
#### LANG AT BOTTOM
	- LANG AT BOTTOM -- 
		- Why?  Creates idea of "construct" which others instantiate.
		- Why?  Does not depend upon lexical semantics or any specific semantics.
	- LEX AT BOTTOM -- 
		- Why?  Does not depend on any other functional thing
	- EXE AT TOP
		- EXE combines it all.  
			- implements lang 'exe/reduce'
	- USP -- Is Lang instance, depends upon, Lex, Unit, Ucalc

	- Waht are languages constructs?  (super class of pkg)
	- What does load accept?  (construct thus pkg.  but really it is just +=)
	- Are constructs code?  (yes, they R pkg?)

	- PKG
		- the superclass of Construct since it can be added to lang
		- is analogous to a Lex (but no infinite parents)
		- not a kind of GND since it has dedicated structure


	_
## === UCOMP ===
### _

**TL;DR.  UCOMP (Uniform Computation) provides a formulation of computation as a kind of predicte-calculus (Unit Calculus) built as transformations of unit structure instead of the (messier?) transformations defined over mathematical equations.**

**TL;DR.  Unit Calculus is a simple rewrite system constructed from unit structures; it is used to provide a formal semantics for Uniform.**


**UCOMP** -- The UCOMP construct defines computation as Unit Calculus (UCALC) a predicate-calculus like formulation defined as a specific term rewriting system (TRS) defined over unit-structured terms.   TRS in turn is expressed as an instance of ARS.

The **_purpose_** of UCOMP is to provide a least-commit model of computation.


~-~
**REWRITE** -- A _rewrite system (REWRITE)_ is a formalism where rewriting rules are used to iteratively transform ("rewrite") objects into other objects.

**TRS** -- A _term rewriting system (TRS)_ is a rewriting system whose objects are terms (expressions with nested sub-expressions) are rewritten by a kind of template match and replace rules.

**UCALC** -- _Unit Calculus (UCALC)_ is a term rewrite system designed as a basis upon which all computation may be built, in close analogy to the way that predicate calculus provides the same basis.  The difference is that UCALC's rules and terms are not mathematical equations, instead UCALC builds these from Unit structure.

Unless you have a penchant for counting exactly how many angels might dance upon the head of a pin, you might decide to skip this section.  None of the executable code in this section is actually run under normal circumstances within a Uniform language environment.  Rather this section provide formal semantics for the lowest two functional spirals for the Uniform ecosystem.

Creation of this section was helpful in the creation of Uniform; working out these details did force attention onto areas of conflict or ambiguity within the spec.  These very lowest levels are also used as jumping off points for other less procedural branches of the Uniform ecosystem, still as written here it is really the prequel to EXE, the Unicore computation construct.

OUTLINE:
1.	The first ARS section provides a framing of computation as sequence of rewriting transfomations.  
2.	The TERM REWRITING SYSTEM section specializes this framing using the structural "unit" machinery developed in Uniform.  
3.	The UNIT CALCULUS section provides a version of predicate calculus derived from this TRS and uniform's structural machinery.

_
### --- ARS - ABSTRACT REDUCTION SYSTEM ---

**ARS** -- An _abstract reduction system (ARS)_ is the most abstracted model of computation within the Uniform ecosystem.  An ARS, (A, -->), is comprised of a set of elements, A, and a rewrite relation, -->, defined over those elements.

Intuitively an ARS captures a most abstract notion of "computing" as a process that takes something as a starting point, and then repeatedly transforms (reduces) that thing towards the end of the computation.

_
#### -- Vocabulary --

**OBJECT** -- Elements of the ARS set A are commonly called _objects_.

**REWRITE RELATION** -- The _rewrite relation_, commonly written as "-->", is a binary relation over a pair of object elements in A.  

**IRREDUCABLE / NORMALIZED / NORMAL FORM** -- Given a rewrite relation, -->, an object, _O_, is said to be _irreducible, normalized_ or in _normal form_ iff there is no rewrite of _O_, that is, iff there does not exist _O2_ such that _O_ --> _O2_.

**REDUCE** -- Given a rewrite relation, -->, an object, _O_, is said to reduce to O2 iff there exists _O1, O2, ..., On_ in A where _Oi_ --> _Oi+1_ for all _i<n_ and _O1==O_ and _On==O'_.

**NORMALIZES-TO** -- Given an ARS = (A, -->), we say, O _normalizes to_ O' iff O' is the only element of A where O reduces to O'.

**RULE** -- The rewrite relation is sometimes expressed as an indexed union:  --> == union -->1, -->2, ..., -->n  Each of these indexed parts of the rewrite relation is called a _rule_.

_
### --- TRS - TERM REWRITING SYSTEM ---
#### _

**TRS** -- A term rewriting system (TRS) is a kind of ARS where the objects are recursively-structured units (called terms), and the rewrite relation is expresed as a set of unit structures (called rules).  The resulting rewrite relation is expanded from these rules by systematically replacing each variable with all possible term values.

The TRS construct defined here is a straightforward formulation of a "vanilla" term rewriting system, but built using the structures and operators provided by the Uniform MATH and STRUCTURE spirals.  These structures are easier and perhaps cleaner substrate to build unit calculus from instead of the building from teh textual mathematical equation notation employed by traditional predicate calculus.  It is helpful in this section to use both the notation and terminology of uniform structure as well as the traditional terminology of TRS systems.  The vocabulary shown here maps closely onto that used in the wikipedia article on Term Rewrite Systems:

_
#### -- Vocabulary --

**TERM/SPEC** -- A _Term_ or _Spec_ unit is a bounded tree of unit that serve as the "expression" objects over which rewriting occurs.

**VARIABLE/IDENT** -- A variable or identifier unit (Ident) are a kind of Term units that serve as placeholders within term expressions.

**BINDING** -- A _binding_ is a mapping of Ident "variables" onto Spec terms to which they are "bound".

**SUBSTITUTE/FILL** -- A Binding, b, may be _substituted_ or _filled_ into a term, T, by replacing all occurance of Idents, i, in T that also occur within b, with the terms bound to those corresponding Idents in b.

**MATCH** -- A "pattern" term, p, is said to _match_ a "target" term, t, iff there exists a binding, b, such that substituting b into p yields the term, t.

**RULE** -- A _term rewrite rule (Rule)_ is simply a pair of terms.  Rules are commonly written as " _left_ --> _right_ " to indicate that the left-hand side term can be replaced by the right hand side term.

**APPLY** -- One may _apply_ a rule to a term if some sub part of that term matches the rules left hand side.  The result is a new term where the subterm has been replaced.

**=== FROM ARS ===**

**REDUCE / DIRECT REWRITE / REWRITE ONCE** -- A _direct rewrite_ of some term, T, given a set of rules, R, is a term T' obtained by applying some r in R to sub part of T.  T is said to be rewritten to T' iff there is some sequence of direct rewrites that yield's T'.

**IRREDUCABLE / NORMAL FORM** -- Given a set of rules, R, a term, t, is _irreducible_ or in _normal form_ iff there is no rule r in R applies to any subpart s in T.

**NORMALIZES-AS / REDUCES-TO** -- Given a set of rules R, one may _normalize_ T _as_ T' or _reduce_ T _to_ T' iff T' is the only rewrite of T that is in normal form.  Further a set of rules are _normalizing_ if each term has some normalization T'.

_
#### -- API --

**def type** Term  :=  Bounded Unit
**def type** _Bindings_  :=  Map(Ident, Term)
**def fn** _match_(_template_ Term, _form_ Term, _bindings_: Bindings, ->_Bindings_)
**def fn** _fill_(_form_ Term, _bindings_ Bindings, ->Term)
**def fn** _apply_(_form_ Term, _rule_ Rule, _position_ Path, ->Term)
**def fn** _rewrite_(form Term, ruleset List(Rule), ->Term)
**def fn** _normalize_(form Spec, ruleset List(Rule), ->Spec)
_
#### -- Semantics --

**RULE EXPANSION** 
Each rewrite rule is shorthand for an infinite number of rewrite pairs within the rewrite relation (-->).  These pairs are generated by substituting all possible combinations of term values consistently for each of ident variable within the ancedent and consequence of a rule.  Then systematically subtituting the resulting antecedent/consequent into all possible leaf positions within all possible terms.

Intuitively 
This expansions defines an ARS that rewrites any term where some part of that term "matches" the antecedent of a rule.  

**RULE MATCHING** 
Thus, a rule is said to _match_ an expression if there exists a set terms that can be substituted for the variables in that rule's antecedent form such that the result of the substitution equals the expr.

**RULE APPLICATION**
Once a rule is matched, it is applied by replacing variables in its consequent using the bindings derived during matching, and then substituting the entire result into the place within the term that matched the rules antecedent. 


**PROCEDURAL** -- A ruleset is called _procedural_ if for each term there is at most one rewrite of that term that exists, or is considered at each step in the rewriting process.

**DETERMINISTIC** -- A ruleset is called _deterministic_ if for each term, the final rewrite of any term is consistent across all possible rewrites of that term.

Surprisingly all four combinations of procedural and determinstic are possible.

Non-deterministic, procedural languages include some bit of randomness into their expansion.  A simple example would be a rewrite formulation of a non-deterministic finite automata.  Such systems need not backtrack over any rewrite choice made, but since multiple rewrites exist leading to different expansions it is non-deterministic.  Practical procedural languages like C, Python, etc. often include non-determinsitic pieces in their semantics, usually for performance reasons.  E.g. the order of an iterator in Python, or the order of argument evaluation in C function calls.

Non-procedural, determinstic languages will consider multiple alternate rewrites (even as they later, retro actively winnow down to a single choice.)  Prolog, for example, can be formulated as rewrite systems that entertains multiple choices, but (though backtracking) eventually arrives at a single specific sequence of rewrites.  Prolog is deterministic since given a single start is has a single sequence of rewrites ending in a single output, but its simplest encoding as a rewrite system is non-procedural since at any given moment there will be multiple rewrites for a give term that must be considered.  Only arbitrarily far into the future, do we learn with of these paths will be the final 'determined' one.  Confusingly since prolog is deterministic, one could encode it in as a procedural ruleset.  Instead of using its ...
XXXX

The UnicoreBase language, by contrast is both procedural and determistic by construction.  Its rewrite rules define exactly one rewrite fully define
makes this single rewrite commitment and is thus determinstic.
IS TRUE????




**def fn** _fill_(_form_ Spec, _bindings_ Bindings, ->Spec):
	if form in bindings:
		return bindings[form]
	elif form isa Atom:
		return form
	else:
		result = Spec(^head: form.head)
		for k, v in form.items():
			result.set(k, fill(v, binding))
		return result
		
**def fn** _match_(_template_ Spec, _form_ Spec, _bindings_:Bindings, ->_Bindings_):

**def fn** _apply_(_form_ Spec, _rule_ Rule, _position_ Path, ->Spec):
	
**def fn** _reduce_(form Spec, ruleset List(Rule), ->Spec)
**def fn** _normalize_(form_ Spec, ruleset List(Rule), ->Spec)

_
### --- UCALC - UNIT CALCULUS ---
#### REWRITING CONTROL
##### _

**UCALC** -- Uniform UNIT CALCULUS described here is on a bit of shaky ground.  Before this section the Rewrite and TRS systems are  well understood and accepted formalisms, beyond this section the control flow and memory usage of Von Neumann machines are also well accepted formalisms.  Unit calculus is our attempt at a uniform (essential) formalism that connects these two.  UCALC itself does not have this same pedigree as those before or after, it borrows loosely from lambda calculus, but its Raison D'être, is to be the simplest most essential bridge between rewriting and von neuman control and data.


**What must be bridged by Unit Calculus?**
Rewrite systems afford great flexibility in specifying the computation to be performed, but in their simplest form they provide little control over the ordering and the choices inherent in that execution.  This lack of rewrite system control presents two areas of incompatibiliy that must be managed in order to implement these systems on a von Neumann architecture:
- **SINGLE LOCUS** -- Von Neumann Machines have a single locus of control, so rule expansion must be fully ordered in time on a von neumann machine.  One must have mechanisms to specify/control expansion order in order to implement on a von Nuemann architecture.  
- **BIG TIME -- MORE TIME THAN SPACE** -- Von Neumann Machines typically have a runtime that is much greater than their memory size.  This means the size of the expansions of a rewrite systems possible on these machines are much larger than the memory sizes of those machines. 
- **BIG CONFIG -- MORE CONFIGURATIONS THAN SPACE** --  

Here we introduce several widely employed paradigms for managing Term Rewriting flexibility:
- **EXPANSION POLICIES** -- 
- **EXECUTION MARKERS** -- 
- **COMPILATION / TYPE ERASURE** -- 

_
##### EXECUTION MARKERS

Thus again we must control the order of expansion so that the expanded-but-not-yet-collapsed portions of the rewrite are maintained to remain smaller than all available memory through out the entire rewriting progression.


**exe(term)**
We systematically replace the lefthand side term, _LEFT_, of each rule with a wrapped term: "**exe(**_LEFT_**)**".  These new rules wrapped rules now _only_ match specially wrapped sub expression terms, T, as "**exe(**_T_**)**".

Rewrite expansion order may now be controlled by replacing certain sub terms, S, with the "**exe(**_S_**)**" term.  At any point in time the only rewrite rule applications that might be considered must key off of these "**exe(**_S_**)**" terms.

_
###### TERM-DEPENDENT RULESETS -- MANAGING RULE APPLICABILITY

**exe(term, rules: rules)**
A second, very powerful way to control the rewriting process is to allow the specification of a different subset of rules to apply at each 'exe' rewriting location.


RULE APPLICATION PROPERTIES
- deterministic
- non-backtracking


_
##### EXPANSION POLICIES

**exe(term, policy: policy_identifier)**
Execution markers are quite flexible since individual terms can be programmatically "flipped" on and off allowing for a very tightly controlled expansion order.  Fully specifying each individual expansion step is, however, a great increase the complexity of a rewrite system, and is often not needed.  Instead we can define a generalized expansion order policies, use them to control rewrite expansion.  And only use explicit manipulation of execution markers in cases where no generalized expansion policy has the desired effect.  This combination can be parsimonious -- relying on generalized policies to control execution, and fully controllable execution markers generalized policies will not yield desired results.

STACK BASED EXPANSION --

POST ORDER POLICY -- 

**ANY LEAF POLICY** -- A leaf-first expansion policy like post order expansion will only consider expansions of a term after all of its children are full expanded.  But unlike post order, leaf first does not specify the order of expansion for peer leaves.

FIRST EXPANSION ONLY POLICY -- 

CUT POINT POLICY -- 

is some form of post order expansion traversal.  It constrains 
_
##### EXECUTION CONTROL DEPARTURES

THEOREM PROVERS / SEARCH ALGORITHMS

PROLOG -- 

HASKELL -- 

PROCEDURAL LANGUAGES -- 

STOCASTIC EXECUTION ORDER -- 
_
#### COMPILATION

Above we noted that the size of rewrite expansions are much larger than system memory, thus they are also much larger than the program being executed.  This implies that the average instruction is being executed many times during execution.  The average number of executions per instruction is multiplied even higher in the case that this same program is to be run executed multiple times with different inputs.  Therefore whenever possible we should arrange to
any rewrites that can be performed

_
#### TYPE ERASURE
##### _

The Von Neuman architecture separates execution into program and data.  The idea is that a fix program executes the same steps in the same ways over different data to achieve the aims of computation.  

In order for this to work the specifics of the data must be constrained so that the fixed program performs sensible operations over the different data.  

Data typing is an imporant way to achieve this end.  A Data Type defines a range of permissible data values, and a semantic meaning for those values.  Fixed programs can now be constructed with provable computational properties based on these types.  E.g. a fixed Von Neumann program can be written which interprets a range of memory cells as a list of integers and sorts those integers.  The correctness of this program is proven in terms of the list and int types, proving that ANY unsorted list will result in an output which is the a sorted permutation of the original list.

Notice here, the structure of the program depends upon the types, but at run time indicators of those types are not needed, since their effects are "baked into" the structure of the program itself.

The divison of labor is enforced by type erasure semantics




TYPE ERASURE is an approach for organizing computation in a way that requires type-dependent execution behavior to be determined at load/compile time, so that run-time execution can be executed without type information.
##### SEGREGATED BINDINGS -- DECOMPLECTING THE MATCHING AND BINDING STEPS

According to TRS semantics, the second half of rule application is the application of the obtain variable bindings from matching the left hand side to the rule's right hand side.  A second way we can control execution order is by allowing the match portion of rule application to be separated from the binding application portion, and to be performed in multiple segregated steps.  

**apply(lambda(var, term), value))**
Taking a cue from lambda calculus we can use the expression above to indicate the intent to replace each occurence of 'var' in 'term' with 'value'.  By nesting multiple of these expressions one can indicate the binding of value1, value2, ..., valuen for variables var1, var2, ..., varn.

**exe(term, bindings)**
We use the expression above as shorthand to denote the interpretation of 'term' given 'bindings' where bindings={var1: value1, var2: value2, ..., varn: valuen}.

Once a rule's left hand side is matched, using variable bindings, b, it may be rewritten to:  exe(RIGHT, b) where RIGHT is the rules right hand side term.

_
##### - model of type erasure -
A generalized formulation of type erasure is naturally expressed as controlled execution.  Here are the parts:
- PROGRAM -- the code 'term' to be executed.
- TYPE_BINDINGS -- the binding of each program variable to a type that constrains the range of values that may be bound to that variable.
- INIT_BINDINGS -- the arguments provided at the start of execution.
- LOAD_TIME_RULES -- rules that 'compile' type-specific choices into the load-time expansion of the program.  This expansion guides runtime execution implictly using type information since those expansions are base the indicated types.
- RUN_TIME_RULES -- rules that guide the 'runtime' choices by during program execution.  These rules do not have access to the types.

RUNTIME_PROGRAM = exe(SOURCE_PROGRAM, TYPE_BINDINGS, LOAD_TIME_RULES) 
FINAL_BINDING = exe(RUNTIME_PROGRAM, INIT_BINDINGS, RUN_TIME_RULES)

_
#### -- Execution Model --

def fn exe(term, bindings:, rules:, policies:):
	rule = lookup(rules, term)
	if rule:
		return invoke(rule, term, bindings, rules, policies)
	else:
		return {k:exe(v, bindings, rules, policies) for k,v in term.items()}


def fn invoke(rule, term, bindings, rules, policies):
	bindings2 = match(rule.antecedent, term, bindings)
	return reduce(rule.consequent, bindings2, rules, policies)

def fn reduce(expr, bindingscoo)

class Env:
	bindings
	rules
	polices

def fn exe(term, env, rules:):
	rules = rules || env.rules
	rule = lookup(rules, term)
	if rule:
		invoke(rule, envterm, bindings, rules, )
	bindings2 = match(rule, term, bindings)

exe(term, bindings:, rules:, policy:)

Env == DATA FOR EXE EXPANSION == bindings + rules + policies

rule = lookup(env, term) 
env2 = match(rule, term, env)
term2 = fill()

env2 = NEW(gnd)
match(env2, schema, env)

previous, term
gnd  = ns.path_get(term.head)
env2 = env.NEW(gnd)
env2[`self] = previous
gnd[`schema].match(env2, term, env)
return gnd[`reducer].reduce(env2)

form.invoke(self, call_spec, env) --> result
-- self is limited to certain type
-- really types in env are all limited to something


_
#### -- SEMANTICS ASSEMBLY AS REWRITE RULES --
##### _
##### - Structural Operators -
AT/ITR/LEN NEW/DEL INV/MET
##### - Flow Operators -
BLK, BRA, CHN, ITR, RET
##### departures


BLK EXECUTION SEMANTICS

exe(blk(), env:e)  ==>  None

exe(blk(t1), env:e)  ==>  exe(t1, env:e)

exe(blk(t1, t2, t3...), env:e)  ==>  
	exe(blk(t2, t3...), env: exe(t1, env: e))
_
#### older junk

- implicitly g information to guide how subsequent 

setting for this type erasure concept 

Such separation is achieved by splitting bindings into type 



 specific 

- map each grounding onto an Ident, and use these idents to index rule application.
- rule application is expressed solely as a function of variable types and matching structure contained explicitly in the exe's term (not in its binding)

ident => Gnd
Gnd.schema
Gnd.action

NS == Map(Ident, Gnd)



_
##### older

The first Unit Calculus extension is the use of execution markers to control rewriting.  Here we wrap a term, _T_,  to be expanded in a special "**exe(**_T_**)**" term. 


 that specifically expand these exe 
This indicates that  use a special exe head 

.  Unit Calculus is an extension of Uniform TRS which affords such control.  Unit Calculus is also designed as a kind of predicate calculus that uses unit structures instead of mathematical equations as the data structure over which it operates.

**exe(term)**
(1) Order of execution can be managed by rewriting wrapping sub terms in with an 'exe(*)' wrapper and then having all rewrite rules focus specifically on exe forms.

_
_
#### API

**def type** _Ident_: extends: Atom

**def type** _Term_: extends: Unit

**def type** Env:
	**extends**: Map(Ident, Term)
	self: Unit		// Cannot be Null

**def type** _Exe_:
	**def var** term: Term
	**def var** bindings: Map(Ident, Term)
	
**def type** _Rule_:
	**def var** _schema_: t: Term, s:0
	**def var** _action_: t: Term, s:1
	
**def type** _NS_ := Map(Ident, Gnd)

**def type** _Gnd_:
	extends: Rule
	def var ns Ns

**def fn** _lookup_(exe Exe, ->Gnd):
	gnd = RULESETS[exe.self.gnd.ident]
	return gnd.ns[exe.term.head]
	return exe.self.gnd.ns[exe.term.head]

**def fn** _eval_(exe Exe):
	gnd = lookup(exe)
	bindings = match(gnd.schema, exe.term, bindings: exe.bindings)
	result = Q.exe($gnd.action, $bindings, $exe.self)

_
#### OLDER

The UCALC construct first defines the a term rewrite system U_Calc, and then uses it to define Uniform alpha interpretation, and to define the semantics of the XXX Uniform assembly instructions.  U_Calc is pattered off of lambda calculus only it uses unit structures in lieu of mathematical formulas as the basis for its rewriting.
##### Gnd / Exe

Ideas:
- Semantics derived from single Ns semantics tree.
- All execution is expressed as reducing EXE forms, thus all rules are keyed to that one form.
- In general rule application is non-determinstic this is formulated as a backtracking rewrite system.  
- Here we consider simplification #1:
		deterministic decision procedure for rule to apply to each subform
- Simplification #2 -- DP is a constant structural variable-base dereferencing  based on form and self groundings (type erasure).8
- deterministic DP for subform to reduce next.
	==> all rules index off first arg in EXE being a resolved binding,
	    and all transform rules ensure that only one exe is "clear" at a time
	(control)

Gnd.pattern
Gnd.action

exe(bindings, form, self) |- 
	rule := lookup(self.head, form.head)
	bindings2 := match(rule.pattern, form, bindings: bindings)
	bindings3 = bindings + bindings2    // needed??
	return exe(bindings3, rule.action, self)


exe(env, blk(a, b, c), self) |- exe(exe(env, a, self), blk	
##### older


The UCALC construct includes 

TRS -- A term rewriting system 

TRS - Term Rewriting System
term, bindings, DOT

term = bounded, fixy unit
var = specific substructure
binding = mapping from vars to values
DOT = term * binding
match(x, y) -> z   finds z, bindings for vars in x in order for it to match y
reduce = remove dot by applying bindings to a variablized term

_
#### 
## === LANG ===
SMALLER / NEWER VERSION IN ELEMENTS
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
### --- API ---

_lang.uc_:
	_**Code**_: Class(Text || Spec || Form)
	_**Text**_: Class(extends: Str)					// Code's textual form
	_**Spec**_: Class: extends: [Heady,Boundy,Tree]	// Code's homoiconic form
	_**Form**_: Class: 								// Code's executable form
		extends: Tree(Form)		
		text:	slot(a:[Spec], t:Text)	
		form:	slot(a:[Spec, ], t:Form)
		spec: 	slot(a:[self, spec], t:Spec)

	_**Lang**_: Class:
		extends: Unit
		_**text**_: Op(Code, -> Text)
		_**spec**_: Op(Code, -> Spec)
		_**form**_: Op(Code, -> Form)




//  #####################
**def type** _Code_ := Text || Spec || Form
**def type** _Text_ extends Str						// Code's textual form
**def type** _Spec_ extends Heady Boundy Tree		// Code's homoiconic form
**def type** _Form_ extends Tree(Form):				// Code's executable form
	
**def class** _Lang_: 
	**extends**: Construct
	**def op** _"+="_(Construct)					// Loads construct into lang

**def class** _Construct_:							// Interpretable thing
	**extends**: Tree(Construct) 				  //  
	**def type** _MyText_ **extends** Text
	**def type** _MySpec_ **extends** Spec
	**def op** _text_(Code) -> Text					// Converts Code to Text
	**def op** _spec_(Code) -> Spec					// Converts Code to Spec
	**def op** _form_(Code) -> Form					// Converts Code to Form
	
	**def op** _form_(_spec_: Spec, _meta_: Gnd, _accepts_ Type, _returns_ Type, 
					_env_: Env=None) -> Form:

**def class** _Klass_:  // really a Construct is a kind of unit Klass
	**def op** new()
	**def op** matches(u Unit) -> Unit

**def var** _this.lang_ -> Lang						// Current lang instance
	
_
#### -- Details --

Here we see the bit of Uniform that formally defines a Lang as a kind of interpretation environment with relevant types and operators defined:



**pkg** unicore.lang;

**def type** _LangSpec_:
	**extends**: PkgSpec


**pkg** unicore.lang;

**def type** _LangSpec_:
	**extends**: PkgSpec

**def pkg** _Construct_ **extends** Unit:

**def pkg** _Lang_ **extends** Unit:
	// These parts are from parent lang
	**def type** Text extends Str
	**def type** Spec extends Heady Boundy Unit  // Fixy Provy
	**def type** Form extends Unit
	**def type** Code := Text || Spec || Form
	**def op** _text_(Code, ->Text)
	**def op** _spec_(Code, ->Spec)
	**def op** _form_(Code, ->Form)
	**def op** _load_(Code)					// Performs += on root
	**def op** _exe_(Code, ->Unit)			// Might freeze universe
	**def op** _bang_(List(Code))			// Creates sub-universe using lang

	**def var** _base_ Env					// Env&self both from parent universe
	**def var** _sources_ List(Code)		// Sources loaded into universe
	**def var** _root_ Form					// Structure resulting from loaded sources

// OLD parse/print methods
**def class** _Lang_ **extends** Unit:
	**def var** _base_ Fixed Lang
	**def var** _constructs_ List(base.Code)
	**def op** _bang_(List(Code))		// Bang op creates new Lang
	**def op** _exe_(Code, ->Unit)
	**def op** _parse_(Text, ->Spec)
	**def op** _print_(Spec, ->Text)
	**def op** _load_(Spec, ->Form)
	**def op** _dump_(Form, ->Spec)
	**def op** _"+"_(Code, ->Lang)		// Really on Env??
	**def op** _"+="_(Code)

**def class** _REPL_:
	**def var** base Lang
	**def var** lang Lang
	**def op** _eval_(Code, ->Unit)   // compile +=
	**def op** _load_(Spec, ->Null)   // compile +=

**def class** _Construct_:
	**extends**: Pkg
	**def var** _self_ Env
	**def op** _run_(->Unit)
	**def op** _parse_(Text, ->Spec)
	**def op** _print_(Spec, ->Text)
	**def op** _load_(Spec, ->Exec)
	**def op** _dump_(Exec, ->Spec)

**CONSTRUCT** -- A construct is an interoperable mini-software language potentially having textual, spec, and exec forms.


_
### --- SEMANTICS ---

**class** Lang(Gnd, Form):
	@static
	**def** form(cls, spec):
		gnd = self.lookup(spec)
	
_
### --- FORM/LOAD ---

**LOAD** -- Translates language spec into language forms, and incorporates those forms into Incorporates a language form into the "EMERGING CHUNK"


**def op** _load_(lang Lang, spec Spec, parent Form):
**def fn** _load_(spec Spec, parent Form, ctx Map):

**def op** _form_(self Form, parent Form, spec Spec):

**def switch** _load_(self Form, spec Spec, ^key: `head):


Stuff needed during loading
- **spec** the spec to be loaded into a form
- **return type** the return type if form is reduced or invoked
- **load_env** (all of these things are indexed from meta)
	- **env** is used for load time execution
	- **origin** of ground space into which form is being loaded (from env)
	- **lang** of entire universe
	
	- **local types** -- types for all local variables
	- **self type** -- type for self 
	- **parent** form parent-chain and form-tree
		- parent pkg -- closest form-parent that is a pkg
		- parent env pkg(s) -- place where local vars attach
		- parent self pkg(s) -- place where self indexed
- **READONLY == progressive**
	- once value is set, it is never changed (maps, constants, objects)
### --- BA NG ---

BA NG -- 






_
#### -- Discussion --

Steps
- Create null lang instance

1. LANG0 -- bang unicore env
2. load sources for new lang
3. new form built for new lang. 

JUST BOOT BASE ENV
- load base forms
- exe lang.main

SIMPLE APP LOAD
- boot base
- load app_form
- exe app.main  

	
_
### --- Semantics ---

- AGGREGATIVE -- Plus operator generates a new Lang 
- STATIONARY -- Lang operators use the 'base' Lang to derive their effects.
				Thus load has no effect, and "+" returns new Lang
- REPL -- 


_
### --- Construct ---


**CONSTRUCT** -- A _construct_ provides an environment-specific implementation of some cohesive computational notion.

A construct usually involves types and operators which perform in concert in order to provide some indicated functionality.  For example: TREE, JSON, and TERM_REWRITE_SYSTEM might all be examples of constructs, each come with expected data types, expected operators, etc. A construct is specified as a namespace with the provided parts embedded within it.


**CONTRUCT TEMPLATE**
- **loader** -- a mapping from Spec head symbols onto the operators that translate Spec forms with that type into their cooresponding Exec form.
- **dumper** -- a mapping from Exec type symbols onto the operators that translate Exec forms with that type into their cooresponding Spec form.
- **printer** -- a mapping from Spec head symbols onto the operators that translate Spec forms with that head into their cooresponding Text form.
- **printer** -- a mapping from Parse tree head symbols onto the operators that translate parse tree nodes with that head into their cooresponding Spec form.

- **normalizer** -- 
- **expander** -- 

~-~

**CONTRUCT** -- A _Construct_ is a group of structural data types and functional operators that work in concert to implement some cohesive software notion.  (The tree data structure, a rewrite grammar system, and the looping control flow primative are all examples of such software constructs.)

In Uniform constructs are specified as an NS (namespace) containing the relevant types and operators.  The Unicore base lang defines its four parse/print/load/dump operators so their functionality is extensible by adding operators and transforms to the relevant parts of its lang namespace.  In this way each construct can simply define parsing rules, data types etc such that merely adding their namespaces together will generate combo-languages containing interoperable collections of the indicated constructs.

Often mini-langauges are defined in this simple way.  Indeed Unicore Lang itself is defined as the composition of four constructs, which are themselves composed from smaller constructs:

**def** lang UNICORE := Lang(MATH + UNIT + EXEC + ASM)
**def** construct ASM := FLOW + VAR + PKG + CTX
**def** op lang.Lang.Lang(c Construct, ->Lang)

NOTE: The third line above defines the constructor for Lang as an operator that accepts a Construct and returns a Lang, as used in the first line.


_
#### -- UNICORE CONSTRUCT NAMESPACE TEMPLATE --

**pkg** unicore.lang;
	
**def** var _parsers_:  Ident => List Op(form Text, -> Spec)
**def** var _printers_: Ident => List Op(form Spec, -> Text)
**def** var _loaders_:  Ident => List Op(form Spec, -> Exec)
**def** var _dumpers_:  Ident => List Op(form Exec, -> Spec)


Each of these sub lang namespaces map their identifier symbols onto operators that will translate a form with the specified head into the target format:  Parsers operate on parse Interval forms, printers and loaders operate on Spec forms, and dumpers operate on Exec forms.



_
### --- parse/print/load/dump ---
#### - parse -
#### - print -
#### - load -


**LOAD** -- Translates a Spec code form into the Exec (executable) form for some specified interpretation environment (Env).

- load -- load fn on a construct is used to instantiate an Exec
- **execute** -- Uses the given Env to execute (EXE) the spec form.
- **repeat** -- Repeats this process until the object returned is the same as the one passed -- e.g. until a constant value is returned.
- xxxx

##### pseudo code

def pkg_load(source):
b = E.lang.base
pkg = b.load(b.parse(source))
E.lang.env.ns += pkg.contents

// but what runs this pkg_load op?
// -1- maybe EXE of pkg will add self to E.lang.env.ns
// -2- maybe Lang + ns  returns new lang


// Progressive load idea:
// load_source(src)
// - Textual is anything with a Str contents
// - has_bootmark?  then return load bootmark
// - Textual?	then parse to Spec
// - Spec

##### decisions
CHOICE:  are operations like load able to accept text? A: Yes
CHOICE:  can load of non-pkg yield non-Pkg Exec forms? A: Yes
CHOICE:  does  env + ns ==> updated_env?    A: Yes

ISSUE: then how does E.lang.env.ns get updated on pkg load?

ISSUE: from REPL command line how does 'load' work
A:	REPL uses E.lang.base; load does: E.lang.env.ns += load_it

RESULT: yes can load non-pkg to non-Pkg Exec
RESULT: yes Env + NS -> Env'

_
#### - dump -

- **spec** -- calls the Exec forms 'spec' operator in order to return its Spec.

_
### --- Discussion ---
#### -- 2020-04-04 -- Memory Structures

IDENT TREE -- Each just has pointer to parent upto null.

INTERNED IDENT -- Stored in a weak map, guarnateed unique

LEX -- 

NS(T) -- Tree of mappings to T

PKG -- kind of Form the dervies an NS

DECL -- tree of declaration forms 

GND -- A semantics grounding structure
	.form -- the form that generated this grounding
	.lex -- the semantic location of this grounding
	.slots -- the structure indicated by this grounding
	.ns -- the namespace indicated by this grounding



_
#### -- 2020-02-15 -- Memory Structures

a_unit:
	gnd:		GND_A_UNIT
	inst_var:	"a value"

GND_A_UNIT:
	gnd:		GND_A_UNIT_CLASS
	schema:		list(var(n:"inst_var", t:Str))		// Schema for 'a_unit'
	class_var:	"class wide value"

GND_A_UNIT_CLASS:
	gnd:		STD_META_CLASS
	schema:		list(var(n:"class_var", t:Str))		// Schema for class vars

STD_META_CLASS
	gnd:		GND_STD_META_CLASS
	schema:		list(var(n:"schema", t:Schema))		// Schema for classes
	lang:		
	

_
#### -- 2020-02-09 -- Memory Structures & APIs --

Pkg:
	0:		GND_ROOT
	lang:	PKG_LANG
	
GND_ROOT:

GND_LANG:
	0:	origin
#### -- load data flow

lang.load() -- unit case

lang = bang()

lang.load(spec)

load(spec, parent, ctx)
#### -- 2020-02-06 -- Gordian Knot Cutting --

Gnd Spec:   Gnd([...], k1: Gnd(...), k2: Gnd(...), ...)
Patch form: GndPatch(Ident1->Gnd(...), ...)

Gnd: NamedTuple, SlottedBareClass
Gnd: Parent/Index/Children, Source, Ns, Exe, Schema, 

Pkg

KINDS OF GROUNDINGS
Pkg:	ns, schema 		| key1: SubGnd(...), ...
Fn:		body			| args: Schema(...)
Schema:					| 0..n: Var(...)
Var:	t: n: b:	 	| access: assign:


Var:	new(Gnd)	Var.sub_env(env, schema, form, self)
#### -- Iterpretation of Multi-level-langs --
IDEA:
- Each 'bang' creates a new meta-class instance with its own 'exe' slot.
- 'NEW' classes built from the meta-class share common 'exe'
- 'NEW' Env build from this meta-class inherit flow operators etc. from there
- 'DOT' operator bridges from one functional universe to another 
	(only structure is shared across this bridge)
#### -- Implementation --
#### -- Creation & Datastructures --
##### - Creation Steps - 

**BANG**:
- base.bang(sources) -> lang
- During constructions 'lang.load' exetends upper non-functional lang.


BASE -- Base Lang First the base lang must exist and be operational
	(it is used to construct all the structures needed)
	
**lang.BANG(sources)** -- Uses base to create new Lang, and .load(sources)
- LANG -- the 'self' lang used for all to load all sources.
- METACLASS -- Create GndGndGnd and GndGnd instances and wire them together
- VAR -- Create Var classes: Place, PlaceSchema, BackingSchema
- BASE.LOAD -- Sources are sequentially loaded onto this new lang instance
- GND -- Thus all gnd and exec structures are controlled by base lang, base heap etc.
- NEW -- 

_
##### - MEM STRUCTURES SUMMARY - 
###### - OLDER MEM STRUCTURES SUMMARY - 
V2
			 SomeClass		 SomeClass.Pkg
a_unit	.gnd=unit_type	.gnd=pkg_type	.gnd=Gnd .gnd=GndGnd .gnd=Gnd
		.extends=Y		.extends=Y.gnd		


// Notice pkg_types have parallel hiearchy
V1

a_unit  .gnd=unit_type  .gnd=meta_type == built_lang  .base= base_lang

bang operator ==>  built_lang

_
###### - DATA STRUCTURES IN MEMORY -

GLOBALS
- lang instance
- 

Unit>Form>Pkg>Gnd
Backing != Object

SOMEUNIT
- gnd: 			SOMEUNIT_GND
- field1, ...
SOMEUNIT_GND 		(this is a pkg_instance which extends ChainedMap)
- gnd:			PKG_CLASS_GND
- {children}		Map(Ident, Exec||Unit)	// The derived namespace
- path_parent: 	SOMEUNIT_SUPERCLASS_GND
- path_idx: 		"Foo"  					// name for this type
- action: 		CLASS_CONSTRUCTOR
- schema: 		BACKING_SCHEMA
- match:			MATCH_OPERATOR
GND'S GND
- lang instance


PKG_CLASS_GND


LANG INSTANCE
- origin
LANG GND
- run
- parse
- print
- load
- dump


ENV
- self
- ...args, locals
- ...ptrs to larger lexical scopes

ENV_GND -- Thread
- thread vars
- glob vars

LEX 
- value
- parent
- children (the active children)

EXEC - Can be anything, but must be reducable

GND
- run
- ident
- schema
- ns


EXE
- body


PkgType			PkgTypesType


MetaType

lang_pkg_singleton   LangPkgType
	base_env:
	parse
	print
	load
	dump


###### out of date
TYPE INFO PARTS
- r_inline:		Reducer
- r_special: 	Reducer
- r_standard:	Reducer	
- r_decl: 		Reducer
- r_meta_decl:	Reducer
- value:			Unit
- children: 		Map(IdentPart, Gnd)   alt: List(Ident)
- schema_unit:	List(Var)
- schema_class:	List(Var)
- virtual_0:		Unit
- ...

VAR
- name			Ident  // relative to schema root
- type			Ident
- expr			Exec

EXEC
- gnd			Gnd
- ...			args

IDENT
- arg1			Int || Str
- ...




INFERRED/DROPPED
// 7	lex:				Lex
// 8	parents:			List(Gnd)  only one parent					

_
##### - Execution Structures In Memory -

// Exec_instance.reduce(env) -- updates 'env' and returns result

// A slot with an expression in it
expr: blk(one(), two())

EXPR
gnd:	EXEC_GND
action:	EXPR_ACTION

EXPR_ACTION
gnd:	BLK_GND
0:		one()
1:		two()

BLK_GND
	.reduce(self Blk, env Env)  	// Moves exe forward over 'self' block

EXEC_GND
	.reduce(self Exec, env Env):	// Just reduces its own action
		return self.action.reduce(env)

~-~~-~-~
	**2 * _x_**

DOUBLING_X_EXPR
gnd:	TIMES_GND
0:		2
1:		var(n:I.x, t:Int)


~-~~-~-~
	y = double(x)

	def fn double(_x_):
		2 * _x_

	def macro fn(arglist, body):
		schema := lang.exec(arglist)
		return `dot($schema.lang.exec(, )
		result := lambda(code): arglist.lang.exec(code).
		return result





SOME_PKG
gnd:	
double:	DOUBLE_FN

DOUBLE_FN
gnd:	FN_GND
body:	DOUBLING_EXPR
schema:	DOUBLE_ARGS

DOUBLE_ARGS
gnd:	ARGLIST
0:		var(n: I.x, t: Int)

FN_GND
gnd:	DEF_GND
action:	dot(self, exec(), reduce(DOUBLING_EXPR))
match:	FN_MATCH_OP

FN_MATCH_OP
gnd:




DEF_GND
	.reduce(self Def, env Env):
		return self.body.reduce(env)




~-~~-~-~


_
##### - Execution Structures In Memory - Example Class -


**def class** Mult:
	**def var** _m_ Int
	**def op** Mult(_self_, __m_):
		**return** _self.m_ = __m_
	**def op** scale(_self_, _x_):
		**return** _self.m_ * _x_

MULT_PKG
Class(
  ^schema: Schema(Var(n: I.m, t: Int)),
  ^init: Fn(^schema: Schema(Var(n: I.m, t: Int)), action: `=(I.self.m, I.m))
  scale: Op(^schema: Schema(Var(n: I.x, t: Int)), action: `*(I.self.m, I.x))

Class ==>
	Pkg()
	
  gnd:	UF_CLASS_PKG
  schema:	

**MULT_PKG**
gnd:	UF_CLASS_PKG
schema:	MULT_CLASS_SCHEMA
action: MULT_CONSTRUCTOR_FN	
scale:	SCALE_OP

**MULT_CLASS_SCHEMA**
gnd:	UF_SCHEMA_PKG
	0:	var(n: I.m, t: Int)

**MULT_CONSTRUCTOR_FN**	
gnd:	UF_FN_PKG
schema:	CONSTRUCTOR_ARGLIST
action:	CONSTRUCTOR_EXPR

**CONSTRUCTOR_ARGLIST**
gnd:	UF_ARGLIST_PKG
	0:	var(n: I.m, t: Int)
	
**CONSTRUCTOR_EXPR**
gnd:	UF_ASSIGN
	0:	I.self.m
	1:	I.m


// META STUFF

UF_CLASS_PKG
gnd:	UF_CLASS_PKG
schema:	



DOUBLE_METHOD
action:	

action: dot(I.foo, exec(, )
schema: DOUBLE_ARGS


DOUBLE_ARGS
gnd:	ARGLIST
0:		VAR(n:"x", t:Int)


ARGLIST
gnd: 	LANG_GND
exec:	ARGFILL

ARGFILL
gnd:	






_
##### - Choices -
##### ISSUE - federated typeinfo 

Background:
tree naturally uses unit children for its structure.
but typeinfo/exec cannot both use children for its facets and its lex-structure
##### - implementation -

bang(lang_gnd) -> lang_env
Lang.load(x) -> Gnd
Gnd."+"(g) -> Gnd
Lang.bang(g) -> Env

// AFTER BANG
unit.gnd -> gnd_backing.gnd -> |IN-BASE-HEAP| -> gnd_gnd_backing.gnd -> {toself}
env.gnd -> |IN-BASE-HEAP| -> env_gnd_backing

// "Lang grounding" is a grounding that defines:
// 		lang. print/parse/load/dump/bang Gnd/Env  with
//		Gnd."+"     Env.EXE
//		lang.base


**===FIXY PROVY BACKING HEAP===**
-- intern(value, deep:)
-- derive()

**===DATA=STRUCTURES===**

    LANG:	backing_lang_obj:		Nul, base(Lang),  layers(List Gnd), 
    									 root(Gnd),  env(Env)
    UNIV: 	backing_glob_vars: 		^up, lang.sources(List Gnd), lang.root(Gnd), 
    									 lang.base(Lang), lang.env(Env)
    THREAD: backing_thread_vars:	^up, lang.threads(List Env)
    ENV:	backing_local_vars:		^up, self(Unit)
    Gnd:	List Unit				[*] [value] [R1-5] [LexIdent] [children]
								    [ClassVars C1...]
								    
    Gnd		[*] [Children] [Lex] 
    TypeInf		...	[Schema] [load] [dump] ... 
    Op			... [Schema] 
								    
    [StackFrameT]	up  self
    [GndOpT]							    
    
Backings:
    layers:		List Gnd	[base_gnd] [layer1] ...
    root :=		``+( $...sources )
    prime := 	lang.heap.NEW(Unit, head^`prime, gnd^ root)
    env := 		lang.heap.NEW(Env, self: prime)
    Schema		.SUBNEW

===OP CALL==

op1 = some operator
code1 = some code invoking op1

stack_frame = E.new(op1)  // copies self?
op1.load(code1, stack_frame)
stack_frame.reduce(op1)


**===EXECUTION===**
BANG -- Creates new base universe. Creates base, Thread, Env, sets U0 = base
LOAD -- Loads new sources to update U0 to U1
- RR(x) == lang.base.lang.load(x, once: True)
- RR(Source) -> Text
- RR(Text) -> TextBuffer			// Parse Buffer
- RR(Text) -> Ivl
- RR(Text) -> Spec
- RR(Text) -> Gnd
- + -> Lang
- PARSE_BUFFER -- Creates a new one
- TEXT 
- PARSE -- Creates Interval tree,  and spec trees
- root.LOAD(Text) -> Gnd
- LOAD(Gnd) -> Gnd (updated root)

===PROVENANCE==
- ^how(x)	returns the expression evaluted in lang.base that resulted in x
- ^why(x)	returns the static set of 
- ^who(x)

===CASE=RULES===

def r1_export(env, )
##### - Semantics -

How it is computed:

lang.sources
lang.texts := [TextBuffer(s) for x in lang.sources]
lang.specs := [lang.base.parse(t) for t in lang.texts]

backing_schemas := ident=>BackingSchema
vars (with data and backing filled in)  (returned by compile)

pkgs (with)
, vars := Scan_for_vars
##### -- THE BASE AND RESULTING INTERPRETATION ENVIRONMENTS --

	**pkg** unicore.lang;
	
	**def** var lang.env: Lang			// The newly defined language
	**def** var lang.base: Lang			// The base lang used to define it


Lang is a way of combining Constructs into new language combinations.
But we need some languge to interpret the logic of the constructs themselves, this is generally not the same language as the one being defined.  

Within the lang package, "base" refers to the Env instance used to interpret the parsers etc. of the constructs within this lang package.  The result of all of this interpreting is a newly constructed language, which also implemented as a Lang intepretation Env.



NOTE:  UNIFORM DISALLOWS CIRCULAR SEMANTICS -- Uniform's two-level non-circular Env defining behavior is quite intentional.  This makes Uniform trivally compatible with languages that require an explicit heavy compilation step.  In this case, compilation is performed under the covers after the moment when a new Lang Env is defined and before the moment it is first executed.  By contract dynamic languages (like Lisp and Ruby) generally fuddle up their semantics by circularly defining those semantics using the program executing this continuously evolving semantics as the program is loaded and run.  In uniform this is not allowed.  Still it is trivial to define a progression of evolving languages which represent successive stages of some evolving semantic, if such an open ended semantic is required.  (For example most REPLs effectively represent an evolving language.)  In uniform we just require that this looping semantic dependency is explicitly represented as a progression of dependencies.

_
#### -- Env Inovcation Forms --

env.exe(form)  |-  form.exe(env)
env.exe( expr )  |-  gnd.exe( env + gnd.match(expr, env) )
	expr := Q.foo(x,y)
	gnd := env.self.ns.path_get(expr.head.nub)

env.exe( expr1.expr2 )  |-  
	result1 := expr1.exe(env)
    expr2.exe( env, self: expr1.exe(env) )

Python Impl

	form.exe(env) --> result
	form.exe(env, Pkg.form(env, pkg_spec))
#### -- Old --
pkg uf;

def Text = type(Str)
def Spec = type(@Fixed, @Ordered, @Mapy)
def Exec = type(Unit)

def fn lang() -> Lang

def var lang.base: Env
def var lang.prime: Env   // maybe called lang.env
def fn lang.parse(source Text, ->Spec)
def fn lang.print(spec Spec, ->Str)
def fn lang.load(source Spec, ->Exec)
def fn lang.dump(exec Exec, ->Spec)

def type Construct:
	$parsers: Sym=>List(Fn(source Text, ->Spec))
	$printers: 	
	Fn(spec Spec, ->Str)
	load:	Fn(source Spec, -> Exec)
	dump:	Fn(form Form, -> Spec)
##### -- Semantics --



	- env.base is provided outside of this universe
	- declaration forms are EXE in lang.base Env
	- declaration forms affect the lang.env Env
	- boot is complete after lang.env.EXE( lang.prime ) returns.
##### Table

	Textual Form	Structural Form		Functional Form
	Str				Code				Form

	**TEXTUAL FORM (as a string)**		
	- **Type:**				Str
	- **Optomized for:**		Human presentation/manipulation
	- **Key Properties:** 	Static, Finite, Sequential
	- **Examples:** 			"[1,2,3]"  "Int(0,)"  
						   u(Int=>Int, 
							 i==>1 if i<2 else self(i-1)+self(i-2))

	**SPEC FORM (as a structure)**		
	- **TYPE**: 	type(u(Ident=>Code)||Int||Num||Str, @finite, @tree)
	- **Optomized for:**		Machine manipulation AS DATA
	- **KEY PROPERTIES:** 	x == load(x.code()) -- true when possible
							Static, Finite, Tree-structured
							Code: 
	- **Examples:**  		Same as above but expressed as a code tree
	- **FORMAT:**  Code form can be any format.  Common formats:
		- **CALL**:  path(ele, ..., key: val, ..., _ meta _: val, ...)
				   direct form means code struct matches form
		- **ITEM**:  path(":"(key, val), ..., _ meta _: val, ...)

	**SEMANTIC FORM (as a form)**	[semantic form]
	- Datatype:			NsForm (contained in a Ns)
	- Optomized for:		Semantic simplicity   (xx load efficiency)
	- Key Properties:	Static, Infinite, Functional (computed)
	- Sequence Example:  [1, 2, 3]  [0, 1, 2, ...]  
						   [..., 1, 1, 2, 3, 5, 8, 13, ...]

	**EXECUTABLE FORM (in an interpretation universe)**  [live form]
	- Datatype:			Live (contained in an Env) 
	- Optomized for:		Interpretation
	- Key Properties:	Dynamic, Infinite, 
	- Sequence Example:  [1, 2, 3]  [0, 1, 2, ...]  
						   -states-of-execution-of-fib-for-all-inputs-
##### Invarients
code == lang.parse( code.print() )		# print/parse invarient
form == lang.load( form.dump() )		# load/dump invarient
form == form.seed( code=form.dump() ) 
	fd = form.dump()
form == type(form)(*fd.args(), _ head _=fd.head(), **fd.kwargs())
	fn = type(form)._ new _(type(form))
form == fn.load(form.dump())
###### CHOICE - Using multiple alpha-interpeters

Issue: two diff ways to have funnel of calls to handle parse/load/EXE of text
(See Lib.)



###### DISCUSS - Generality - Why is LANG model simplest?
Why is this programming model a simplest possible model for a language-writing-language?

Here we provide a sketch of the argument for its key components
- TEXT - Moderns SW expresses programs as text -- we need it.
- CODE - What is CODE and why is it separte from Text?
	- CODE is data structure representation of a program -- it is an AST (Abstract Syntax Tree)
	- LWL need to operate directly on code trees this is their primary data.  
	- Manipulating text is buggy and at wrong semantic level for the ops being performed.  An AST should be used. 
	- Collaborating on lang-writing code requires that the AST is standardized.
	- Humans must think at the text-level since end users code there.  Language writers must think at the AST level since they code there yet are designing for the text-level since that is what their users use.  
		==> Therefore the mapping from Text to AST should be as simple as possible while satisfying needs of both Text and AST users.  
	- Ideally a language writer should be able to look at any text and reliably produce its AST, and an AST and produce the matching text, since they must "program" these ASTs
- FORM - What is FORM and why is it separate from CODE?
	- FORM is the expression of the meaning of a program in its mathematically simplest form -- it is its DENOTATION 
	- CODE is optomized for simplicity of structural manipulation and for simplicity of mapping to text.
		This is often incompatible with having a mathematically simplest expression of meaning.
			EXAMPLE:  SEQUENCE
			For mathematical simplicity we might have a unified semantic notion of a sequence -- one that unifies all sequences: finite, infinite, explicitly enumerated, computationally derived, etc.
			There is no issue unifying these at the sematnic level, we can have infinite semantic structures with ease.  For example the sequence [red, green, blue], the range of integers from 10 to 10000000, the sequence of all natural numbers, or the fibonnaci sequence. 
			The CODE form for these sequences however cannot be unified: [red, green, blue]  range(10,10000000)  range(1,)
			def fib(x) = fib(x-1)+fib(x-2) if x>0 else 1

			At the semantic level these can be uniformly expressed using a common notion of sequence, but at the code level they cannot.
		- Anytime the CODE level can map easily onto a beautiful Textual form AND a beautiful semantic form, then we can allow the CODE level and the FORM to collapse into the same object
	- ENV - What is an Env and why is it separate from Form
		- An ENV is a dynamic artifact that performs the interpretation specified by a FORM
		- So a FORM is an immutible specification of interpretation (and possible a language for interpretation) while
			an ENV is a dynamic, mutible "executing" version of some form.
##### notes
	**Text**  <-parse/print->  **Code**  compile->  **Fns**  <-bang/load/snapshot->  **Env**




	// An interpretational component
	def class Form:

	FORM OPERATORS
		- BANG -- Creats a new computational universe (a Thread), optionally sharing aspects of an existing thread.

		- Loads 'Fns' into 'Env' producing 
	

	Forms:
	**Text** -- string with code in it
	- parse
		- text_to_tree
		- normalize
			(with provenance: output units will map to unit-form which maps to text intervals)
	**Data** -- parsed string (perhaps w. provenance back to str; perhaps conforming to some lang)
	- load
	**Code** -- executable forms
	- init -- instantiate Code as a live running instance

	Text --parse--> 
	- compile
		- expand
	- init load/bang
		==> bang and load both perform load-time macro expansion and execution in the null env


	INTERP -- Interpret a given form according to a given semantics
	- use interp 'receipe'   e.g. macro expand, then eval

	Levels of Interpretation
	- Level 0 -- Immediate
	- Level 1 -- Special Form
	- Level 2 -- Fn / Macro     double; fn; def
	- Level 3 -- Declarational  parse-call; engine-instance; engine-spec. pipe-spec
### --- CHOICES ---
#### -- Choice: 'form' or constructor

CHOICE: Use static 'form' method or class constructor for lang.form(x)

CHOOSE: USE_FORM

- Allows distinct constructor method that does not fit lang.form signature
- Does not depend upon language constructor idiosyncracies
- NEGATIVE:  Does need static method support from lang  (this is assumed)
- NEGATIVE:  Ends up needing duplicate code to also provide a constructor.
				Maybe create default 'form' method which call constructor?


#### -- Choice: Construct text/spec/form goals methods could be verb methods --
Choice:  parse/print/load/dump.   text/spec/form source = _.text
#### -- Choice: Env has methods --
really want an 'exe' method on Env
-- follows the 'one-handle' mantra

seems nice if 'gnd' 'obj' and 'env' can avoid methods.
-- allows Unicore to fit in foreign contexts ????  (not sure)

_
#### -- Choice: GND_IDENT FUSION --
Issue: 
- Want x.head to return an ident
- Want x.head[SLOT] to avoid extra indirection
- Want to be able to 'chroot' and change idents??
- Want Ident to be simple datatype w/o hair
- Want Ident to be unique ??  (or at least interned Idents)
### --- OLD ---

**FORM** -- A _Form_ is a data element that is "written in" some langauge -- i.e. data that conforms to the constraints imposed by the functioning of that language.

with E += foo:
	with 

def fn load
	
**def** op run(_e_ Env, _code_ Str||Spec||Exec):
	**if** _code_ =~ Str:
		_code_ = _e_.lang.parse(_code_)
	**if** _s_ =~ Spec:
		_code_ = _e_.lang.load(_code_)
	**if** ! _s_ =~ Exec:
		error "Invalid Spec"
	**return** _e_.EXE(_code_)

_
## === EVAL ===
### --- BANG ---
#### _

Bang 
1. Creates the grounding instance for the universe.
2. Sets that grounding as the value of some lex within lexspace.
3. Creates an instances map for grounding lookup for that universe.
4. Sets the grounding itself as the mapping of the null ident Ident("") in that mapping.

_
#### -- Initialization --
Here is the structure created by the bang operator

PrimeGrounding

_
#### -- LEXSPACE INVARIENTS --
Lexspace provides a specfic structure as the semantic substrate for computation

Invarient Properties:
- UNIT -- Each unit is a vertex within the data graph.  
- UNIT REFERENT -- Each unit must have at least one referent in order for a lexspace to provide semantics for the unit.
- ONE GROUNDING -- Each unit must have exactly one grounding structure that defines its semantics.
- PARTITIONING -- Each grounding is the the value associated with exactly one lex within exactly one lexspace.  In this way all units are partitioned into a single lexspace.
- GROUNDING CHAIN -- Each unit's grounding is derived by looking it up based on a different grounding.  This forms a chain of groundings that must terminate at the origin grounding.

_
#### -- Generality Argument --

DOING -- Computation needs to do something -- to start with some data at a moment in time, and arrive at some other data at a future time.  The doing must occur according to some kind of constrained process.

Break them into operators.  (is that fully general)

OPERATOR -- In order to apply some operation to a unit of data there must be some computable representation of this operation.  Computable in the sense that it can be used in the right context to cause the operation to occur.

GROUNDING -- We can refer to the set of all operators that could be applied to some unit of data as the grounding of that data.  Somehow this grounding must be accessed in order to computation to occur.

UNIT REFERENT -- It seems that any digital or symbol processor is going to need to have some symbol, some digital data, that indicates a unit, if it is to operate on that unit.

BASE_GROUNDING -- There may be many different approaches for encoding and using the referents underlying the unit data.  Thus each approach needs some way to map the referents onto the groundings with operators that provide meaning to 

GROUNDING CHAIN -- There must be a chain of groundings, since each grounding itself must be grounded.

META GROUNDING -- This chain must be finite, else the time to realize the first grounding is unbounded.  Thus there must be a grounding which is its own grounding.  There could be multiple such groundings, but WLOG we can assume there is only one.  If there were multiple, then one could be constructed that yielded all of the others assuming each could be associated with some data 



its referents onto grounding for those referents 

must be some 'base grounding', some way to 

- e root grounding is used 

-  referent Within the lexspace providing semantics (meaning and operators) over that has some 
#### - 2021-07-09 - latest thinking


EXECUTABLE -- A value that 'executes' when composed with an env

LANG -- A factory that produces an executable when composed with code

BANG -- The creation of a new universe based upon an existing universe & a spec

UC -- The Unicore singleton serves as the basis for all computation.  All Uniform languages derive in some way from this core.


UC





def eval (env, code):
gnd = env.ground(code)
env2 = gnd.args(env)
return gnd(env2)


def eval (env, code):			# This allows special forms
gnd = env.ground(code)
return gnd.eval(code)


def eval (env, code):			# Compiles all; uses constructor
    gnd = env.ground(code)
    executable = gnd(code)
    return executable(env)

    # result = env.ground(code)(env)(code)







_
### --- LEX ---
#### _

	LEX is an _**essential framing**_ of "lexically grounded functional semantics".

	The _**purpose**_ of LEX is to provide a structural grounding for functional semantics.


	**LEX** -- The _LEX construct_ formalizes the "lexical grounding", the intuitive idea of writing code onto paper and then deriving functional semantics based on the structural relationships latent within that written form.  

	LEX -- The LEX construct provides a structural basis for the grounding functional semantics.  It is based on the 


	???Key Aspects:
	- GROUNDING -- Grounding is the process of associating functionally define semantic means to structureally specified data.
	- LEXICAL -- A thing is Lexical if it ties to places within the code and assuming it is derived from its placement within the code structure.
	- IDENTIFIER -- thing used to orgnaize bridge between strcuturealy specified source code, and the semantically ????
	- OBJECT -- An object is the fusing of structural data with functional semantics.



	_{[version #1]}_    Specifically LEX formalizes:
	- "piece of paper" as an infinite data structure called Lexspace.
	- semantically relevant spatial relationships as lexspace structure.
		(e.g. sub-scope as child-of in lexspace)
	- The "Writing" of code as a mapping of its parts onto Lexspace.
	- Lexspace is also used to organize the semantics of interpretation itself, thus its vertices (lex) become locations where "semantic" resides.
	- The functional semantics of interpretation itself is "grounded" by associating specially formatted grounding objects to these lex.

	_{[version #2]}_  Specifically LEX formalizes the: (1) nesting, (2) linearization, (3) addressibilty, and (4) transportability, of source code in terms of the structure of its "written" rendering.  This structure is subsequently used to ground the sematantics of interpretation itself.



	_LEXSPACE_ -- Lexspace is a static, indefinitely extending, tree datastructure.  Intuitively one can think of _lexspace_ an an infinite sheet of paper.  Each lex vertex is a "location" on that page.  Each vertex has an infinite number of children, one for each possible labelling number or string.  Recursively each of these child vertices are themselves lex vertices with the same infinite children.  Thus each location is has sub structure that is isometric to all other vertices.  (But of course each vertex has a unique path of keys leading from above toward it.)

	_TWO USES FOR LEXSPACE_ -- This structure can be used in two important ways:  
	- _IT MAY BE "WRITTEN" UPON_ -- 
		First, any structured data may be "written" onto lexspace.  This is done by mapping the root vertex of the data structure onto some lex vertex in lexspace, each child in the data structure is then recurively mapped onto the corresponding sub-vertex of lexspace using the same access key as used in the data structure.  
	- _IT MAY SERVE AS AN ABSTRACTION OF "SEMANTIC LOCATION"_ -- 
		Second, the vertices can serve as a space of unique identifiers used to organize, reference, and decompose semantics meanings.
		- "Semantics location" is key for type erasure based languages as type erased interpretative behaviors are organized into specific places in this space.

	NIFTY PROPERTIES OF LEXSPACE
	- The structure under each vertex is the same as the stucture under every other vertex.
	- All pathtastic properties apply to lexspace.
	- By picking an arbitrary origin vertex, we can map each vertex into a the unique canonical path from the origin to that vertex.  This space of paths becomes an infinte number of names (Identifiers) that can be used as unique references.

	_
##### -- INTRO --

	The Lex setting is quite general, it aims to be the first rung under any kind of computational substrate supporting any kind of representational/performance tricks.  Thus in its full generality it kinda make your head hurt.  Still one need not utilize this flexibility.  e.g. units might serve as their own referent, and bang might only be called once yeilding a simple universe.  So it can be simple or complex as the use case dictates.  Here we do describe it in it most general form.

	Roadmap:
	- A grounding is a structure 
	- Lexspace is a constant tree of all possible identifers
	- Computation is "grounded" (defined) by associating grounding data with locations in lexspace.


	Uniform Interpretation 
	The "meaning" or semantics of a structure is the result of the interpretation of that structure.  The interpretation of a structure is defined by meaning-giving structures called "groundings".  The interpretation of these grounding structures is in-turn recursively defined in terms of other grounding structures.

	Identifier
	An identifier 

	_
#### --- VOCABULARY ---

	**LEXSPACE** --  An infinite, static tree of graph verticies.  The structure of this tree embodies an idealized notion of "location" within a data space.

	**LEX** -- A _lex_ is a vertex of lexspace.

	**LABEL** -- A _label_ is a unit key indexing one lex from another.  The default lexspace uses natural numbers and alphanumeric strings as labels.

	**IDENT** -- An _Ident_ is "Placeholder" unit used to organize computation -- Each ident maps onto a lex vertex within lexspace, and each lex vertex maps onto an Ident.

	**GROUNDING** -- A _grounding_ is a data association with a lex identifier.  Groundings provide the functional semantics underlying interpretation itself -- it defines the behavior of execution over "code" (it defines the interpretion unit data).

	**NAMESPACE** --  A _Namespace (NS)_ is a mappings from Ident to Groundings.

	**ORIGIN** -- The _origin_ of a space is an arbitrarily chosen vertex used to provide unique paths and naming strings for all vertices of that space.  The origin of Lex space is a lex that provides this for lexspace.

	**THIS** -- A package of variables indicating the particular instance of some defining component being operated on in _"this"_ moment of the interpretation.

	**LOOKUP** -- The _lookup_ function uses the origin to map identifiers onto the grounding data they indicate.

	**OBJECT** -- A Uniform Object fuses handled data with a grounding that provides functional semantics for that data.

	**DATA** -- Uniform data is some collection of verticies and their associated edges.

	**HANDLED DATA** -- Uniform data is said to be _handled_ iff its entire collection of vertices can be recovered from a single "handle" vertex via some fixed traversal procedure.

	_
#### --- API ---

	_lang.uc_:

		_**Lexspace**_:								// Semantic space for universe 
			_**origin**_:	Slot(t: Gnd)   // should this be Lex or NS???

		_**Gnd**_: 
			extends: Unit
			_**ident**_: Slot(t: Ident)
	
		_**NS**_:									_// Space of semantic groundings_
			extends: [Place(T), Tree(LexKey), Pathy(LexKey, NS(T))]

		_**Ident**_:								_// A "location" in meaning space_	
			extends: [Pathy(LexKey), Tree(LexKey)]
	
		_**LexKey**_: 								_// Link of lexspace_
			Type(Int || re"[a-zA-Z_][a-zA-Z_0-9]*")
	
		_**Lex**_:									_// Tree of assigned values_
			**extends** Place					 


	choose:  merge Gnd, NS, Lex somehow


	// #####################
	¸ 

	**import** uc.lang

	**def class** _Ident_:						// A "location" in meaning space
		**extends** Pathy(LexKey), Tree(LexKey)

	**def type** _LexKey_: 						// Link of lexspace
		Int || re"[a-zA-Z_][a-zA-Z_0-9]*"

	**def type** _Lex_:							// Tree of assigned values
		**extends** Place					  // Lex =

	**def type** _NS(T)_: 						// Space of semantic groundings 
		**extends**: Place(T)
		**extends**: Tree(LexKey), Pathy(LexKey, NS(T))	
	
	**def class** _Gnd_:						// A meaning containing structure
		**extends** Unit
		**def op** _ident_ -> Ident				// The location of this grounding
		**def op** _construct_ -> Construct		// The Constuct at this location

	**def pkg** this:
		**def var** lexspace(->NS(Gnd))		  // The origin of lexspace

	// replace with 'this.lexspace'
	**def type** _Lexspace_:					// Tree of assigned values
		**extends** Unit				  	  // Lex = 
		**def op** _origin_ -> NS(Gnd)			// The origin of lexspace
	

	_
##### -- older API stuff --

		**def op** _lookup_(_unitspec_, ->Gnd)


	**def class** _Lex_:
		**extends**:   Pathy(Ident1, Lex), Tree(LexKey, Lex)   #, Place
	
	**def class** _Universe_ **extends** Gnd:
		**def var** _origin_(t: Gnd)

	**def class** _Gnd_ **extends** Unit:
		**def var** _instances_(t: Coll)
		**def op** _lookup_(_unitspec_, ->Unit)
		**def op** _lex_(->Lex)
		**def op** _schema_(->List Var)   ??? maybe not needed.  
		**def op** _ns_(->NS)

	**def var** _ORIGIN_: Gnd

	**def pkg** _Gnds_:
		**def fn** _gnd_(_base_grounding_ Gnd, _unit_indicator_, ->Gnd)
		**def fn** _origin_(_grounding_ Gnd, ->Gnd)
		**def fn** _lookup_(_grounding_ Gnd, _[pathspec]_, ->Gnd)
		**def fn** _lexspace_(_grounding_ Gnd, ->Lexspace)
	
		**def fn** _ident_(_grounding_ Gnd, ->Pkg)
		**def fn** _pkg_(_grounding_ Gnd, ->Pkg)
		**def fn** _schema_(_grounding_ Gnd, ->List Var)
		**def fn** _ns_(_grounding_ Gnd, ->NS)

	_
#### --- The implementing structures of lexspace ---

	**THE BASIC SETUP FOR A GROUNDING (FOR A GIVEN UNIT TYPE)**
		Grounding:  universe, lex, ns? (instances), other_info...
		Lex:		parent/index/children, value
		Unit:		unit_referent
		Access:		Gnds.at(grounding, unit_referent)

	- **UNIVERSE** -- _Each grounding must occur within some universe_
		- Each grounding occurs within exactly one universe.
		- Each unit has exactly one grounding as its grounding, thus units are also contained within exactly one universe.
		- NOTE:  Some units have values that are tied to cooresponding units within other universes, indeed sometimes they are even units with identical mutable contents that are precisely synchronized with the other universe.  Even in this extreme case they are still technically distinct units and have distinct groundings that are distinct as well.
		- NOTE:  While each unit has exactly one grounding instance that controls its semantics, it is possible that other more general groundings will also list it as an instance of that grounding.  In this case the unit must simultaenously be operable by the semantics provided by either grounding.

	- **LEX** -- Groundings map onto lexspace.
		A grounding must be the value of exactly one lex within lexspace.

	- **INSTANCES** -- _Each grounding may have a referent-to-unit mappings_
		Each grounding must have a way of deriving the unit structure for any provided unit referent.  
		- The instances unit performs this mapping.
		- May be the identity mapping if units are their own referent
		- Must only map to units that are interpretable using this grounding.
		- Those units might be grounded by a different more specialized grounding.
		- The member unit may or may not be ordered and/or iterable.


	**THE BASIC SETUP FOR A UNIVERSE**
		Universe:	a-kind-of-grounding,       origin_lex, other_info...
		Universe:	universe, lex, instances,  other_info...
	
	**UNIVERSE** -- A universe is all stuff defined as a result of the bang operator
	- **GROUNDING** -- A Universe is a kind of grounding.
	- **LEXSPACE** -- Like all groundings it is "at" some lex within lexspace.
	- **SUBSPACE** -- The subspace of the lex under this lex is the extent of this universe.  All groundings bound to sublex within this subspace are grounding of this universe.
	- **INSTANCES** -- The universe grounding has all groundings within the universe as its instances.  
	- **NON-OVERLAPPING** -- Since all grounding under a universe are only part of that universe, it follows that one universe may not be within the subspace of another universe.
	- **ORIGIN** -- Each universe keeps track of a single "origin" lex used to index all lex and groundings within this universe.
		- NOTE:  The _default_ origin for a universe is the lex associated with the universe grounding.  Still it need not be that lex, it could be any lex within lexspace, even a lex which is outside this universe!
		- NOTE:  At any moment in time the origin for a universe must have a well defined value. Still it is possible for that origin to change over time
	- **IDENT** -- The 'instances' unit of universe grounding maps ident onto groundings.  It uses the origin lex as the base for this translation.

	_TWISTY LITTLE KNOT_
	1. All groundings are in some universe, the universe grounding is inside itself
	2. All units (including groundings) are grounded.  
	3. A universe grounding is the only unit that grounds itself.
	4. By default the origin_lex for a universe is the lex for that universe's grounding. ???? maybe not. ????
	5. Ident are mapped onto universe grounding relative to the universe origin which is the top universe lex by default.

	_??? RESOLUTION ???_
	- root lexspace grounding that reference self as null 

	_
#### --- FORMALIZATION OF LEXSPACE ---


	**def op** _lookup_(_ident_ Ident):
		**return** ORIGIN.path_get(_ident_.nub)


	**LEX KEY** -- _LexKey_ is the ordered set upon which LEXSPACE is defined.  

	Unless explicitly defined otherwise LexKey is assumed to be the ordered set beginning with the natural numbers followed by the lexograpically ordered string labels.  A _label_ is a string starting with an alphabetic or underscore character, and followed by zero or more alpha, underscore, or numeric chars.

	**LEXSPACE** -- _Lexspace_ is a mathematical object -- the smallest graph satisfying:
	1. Lexspace must contain at least one element.
	2. For any L in Lexspace there exists P in Lexspace, and K in LexKey such that
			L = get(P, K)
	3. For any _L_ in Lexspace and any _K_ in LexKey
			 get(_L, k_) in Lexspace
	4. For all L and P in Lexspace, and K in LexKey where  L = get(P, K)
			There does not exist P' in Lexspace and K' in LexKey where
				( P' != P or K' != K )  and  L = get(P', K')

	NOTE: A bit surprisingly, #2 above indicates an infinite tree of parents with no "top" vertex.  This is required to ensure a bidirectional mapping from paths to vertices.  By convention we situate ourselves within lexspace under some quasi-top where all parents above are indexed by the same "0" LexKey, so it is turtles all the way up from there.



	**IDENT** -- _Identifiers (Ident)_ provide unique syntactic naming units as a means of organizing computation. 

		**IDENT SYMBOL** -- An _IdentSymbol_ is a string starting with alpha character or underscore, followed by alpha, underscore, or numeric characters.

		**IDENT KEY** -- An _IdentKey_ is an IdentLabel or an Integer.  (This is the LexKey from above but also including negative integers.)

		**IDENT PATH** -- An _IdentPath_ is a list of IdentPart.

		**CANONICAL IDENT PATH** -- A _Canonical IdentPath_ is one that has at most one negative integer possibly in its first position.

		Each Ident is created from an IdentPath that it is associated with.  The 'nub' operator will return the IdentPath contained within each Ident.



	**GND** -- A _Grounding (Gnd)_ units provide the meaning that underlies interpretation -- they contain specific kinds of structured information this is used guide the interpretation process.

	**NS** -- A _Namespace (NS)_ is a mapping from Ident units onto Grounding units.



	**ORIGIN** -- The _origin_ of lexspace is an arbitrarily chosen lex vertex within the space.  NOTE:  This origin might be anywhere, not necessarily above all relevant lex.

	_
#### --- IMPORTANT MATHEMATICAL PROPERTIES OF LEXSPACE ---

	RE-ORIGINATION -- Any data mapped onto lexspace may be "re-origined" -- it may be expressed relative to any other vertext as origin.  All paths can be translated by prepending an offset path.  All relative relationships are preserved.  All ver 

	**PATH-PLACE DUALITY** -- The two properties below form the path-place duality:
	- **UNIQUE PATH** -- using:  _lex_.path_from(ORIGIN) one can map any _lex_ vertex in lexspace onto a unique Canonical IdentPath.
	- **UNIQUE IDENT** -- using:  ORIGIN.path_get(_ident_.nub) one can map any ident onto a unique Lex in Lexspace.


	**ONE-TO-ONE** -- given a defined ORIGIN lex, these two mapping provide a one-to-one mapping between Canonical Idents and Lex verticies.  So:
		For all lex   lex == ORIGIN.path_get( Ident(lex.path_from(ORIGIN)).nub )
		For all ident ident == ORIGIN.path_get( ORIGIN.path_get(ident) )

	**PATH EQUIVLANCY** -- two paths p and q are _equivelant_ iff for all vertices A and B
		A.path_get(p) == B   iff   A.path_get(q) == B

	**CANONICALITY** -- given any path, P, there exists exactly one canonical path P' where P and P' are equivelant.

	**INVERTABILITY** -- given any canonical path, P, there exists an inverse canonical path P', where for all verticies A and B:
		B = A.path_get(P)  iff  A = B.path_get(P')

	**PATH COMPOSABILITY** -- Path composiblity asserts that concatenating the structure of two paths is equivelant to composing the 'path_get' transitions implied by those paths.  Formally it asserts:  
		For all paths, A, B, and vertex V:
			V.path_get( A + B ) == V.path_get(A).path_get(B) 
	

	_
#### --- CHOICES ---
##### -- CHOICE - A node may have a 
##### -- CHOICE - Lexspace key complexity --

	OPTION 1 -- Key == Int || Str
	Option 2 -- Key == any indexible value
	Option 3 -- includes iterable places

	Issue #1 - Non indexible subunits
	==> Use list of pairs notation ???

	_
##### -- CHOICE - Mutability --
	CHOICE:  Mutibility
	- Lexspace is immutable in the sense that each lex always has the same parent and always represents the same "place" 
	- Each lex is a place and the value stored (the place contents) can be mutated (of course) so the places within lexspace are mutable.
	- Iteration and dereferencing of a lex is driven by iteration and dereferencing on its contents, thus the active children for a lex change as its contents change.
	- But inactive lex still exist and can be accessed, but their value will be undefined.
	- Printing a Lex requires a Lang which defines an origin for lexspace which is used to give each lex a print path.

	_
#### --- GND/TYP ---
##### _

	**GND** -- The _grounding (GND)_ for a unit provides semantics for a unit as well as other information about the class of units to which it belongs.

	**TYP** -- The _type (TYP)_ of a unit is an indicator about the collection to which the unit belongs.

	**TYP** -- The _type info (TYP)_ for a unit is collection of info associated with the grouping (collection) to which the unit belongs.

	_
##### -- API --

	class Unit:
		def gnd(referent Unit=None, ->Gnd)

	// operator needs no argument when unit has

	_
##### -- Grounding Discussion --
###### - PROPERTIES - Expected properties of the grounding operator -

	- **UNIVERSAL** -- All units must have a grounding.
	- **CONTROLLING** -- The grounding for a unit is used to access/control all properties, and information associated with a unit.
	- **DERIVABLE** -- The grounding for a unit must be an efficiently computed function of  the unit's referent and its base grounding.  
	- **REFERENT** -- The referent for a unit is any information that can unambiguously indicate a specific unit.  (think of a referent as a pointer to a unit)
	- BASE GROUNDING -- (Var type) 
	- **EFFICIENT** -- (Derivation must be very efficientGiven a unit, and appropriate context, it must be possible to derive its grounding unit efficiently.
	- EXAMPLE GROUNDING METHODS
		- DYNAMIC UNITS -- directly contain a pointer to their grounding.
		- FLY WEIGHT UNITS -- fly-weight units have a container object that can compute the grounding for each of its contained units.
		- SIMPLY TYPED "STATIC" VARIABLE -- a simply typed variable may only contain units all having the same grounding.  Derivation is trivial, it is just a constant.
		- COMPLEX TYPING -- an appropriate grounding can be derived from known differences in the structure w/o using an explicit grounding pointer.

	- DOUBLED -- some groundings are "doubled" the grounding itself is a singleton, and ITS grounding is called the "class grounding" which is simply the grounding describing the singleton.
	- META -- A meta grounding is its own grounding.  Following the 
	_
##### -- Lex Bang min structure --

	Minimum Structure:
	- Origin_gnd _lex -- must be an origin grounding & lex
	- Origin_class_gnd _class_lex
	- Univ_gnd & Univ_lex

	Why:
	_
##### -- Lex Up --
	Tradeoff: 
	- Lexspace paths operations require an 'up' operator.
	- Vanilla data types dont have an 'up' operator

	Choose:
	- Gnd structures CAN be vanilla lists, but Lex verticies CANNOT be vanilla,
		they must mixin Pathy.
	_
##### -- Lex top --
	Choice:  What is at the 'top' of lexspace
	Options: does it have bean pole on top?  root at top?  or infinite progression?

	Choose:  There is no top (required)
	Why: To have full path duality (e.g. all paths map to a lex)


	Choose:  Bean pole of zeros on top (by convention)
	Why:  It is simplest case of having no top

	So by convention lexspace will eventaully become an infinite progression of Lex whose index is '0' leading up to another with the same index.

	(NOTE:  It could be all sevens all the way up, or the sequence of digits of PI.  choosing '0' is arbitrary.)
	_
##### -- 1GND2 -- One GND two structures --

	Issue:  Each GND must associate with two different structures.
	- A gnd must have structure (slots) with different categories of information.
	- A gnd must associate with its children gnd

	Choose: 
	- Use Gnd with slots for kinds of info.  
	- Use Lex with structure connecting gnd to their children.
	- Provide bidirectional links between groundings and lex

	Consequence:
	- Lex are actual objects in memory
	- Lex need to be pathy
	- Lex need not maintain an ident, it can be computed, or be virtual
	_
##### -- Lex vertex type constraint --
##### -- Lexspace Design --
###### - Min Requirements For Lexspace -

	LEXSPACE -- a single, static, infinite structure

	**FIND** -- Given an 'env' there needs to be a way to find an grounding based on an ident taken from a unit's head.
	- HOW? Could be found thru 'NS', 'self', or the stack-frame's type.
		Choose: NS go up thru the 'gnd' link

	**MULTI-KIND** -- While there is one namespace often there are different kinds of info associated with this single tree.  e.g. var name, function name, etc.

	**MULTI-TREE** -- There might be multiple Lexspace within one memory space (allowed?)
	So one must be able to determine which is relevant.

	**WORK TOGETHER** -- Maybe they could/should be able to work in concert?  e.g. objects from one somehow use those from another.


	- HANDLE -- There needs to be a handle for lexspace 

	_
###### - implementation --

	Considerations:
	- IMMUTIBLE -- lexspace is immutible, it would be nice if implementations of the lex in lexspace had static properties:
		- OBJECT PERMANENCE -- 
		- SINGLE IMPLEMENTATIONS

	Choose:
	- All lex are the same type, as defined at lexspace creation.
#### OLDER
###### notes

	Tree Addressing -> Path -> Origin |-> Ident -> LexKey+Lexspace ->
	ADDR: 	Traversal, Path, Origin, Fancy properties
	LEX:	LexKey, Lexspace, Ident, NS

	NS: KEY => (NS || Place(T))     path_get(ident)->T
	GND: NS, Pkg, Matcher

	LEX(K,T): K=>LEX(K,T), Place(T)
	NS(K,T)

	GND: KEY=>GND, 
###### to re-add


	**GND** -- A Grounding (GND) unit is a structure that (somehow) provides interpretational meaning.

	**NAMESPACE** -- A _Namespace (NS)_ is a mapping from Ident onto some specified unit type.  (Typically including Grounding units)

	each lex in lexspace (given that some "origin" reference lex is chosen).


> >>> ADD SEMANTICS FOR ASSIGNMENT PASSING INTO SUBLEX
	- origin -> path -> Ident -> Patch
	- 


	The _LEXSPACE_ graph is iductively defined as the smallest graph where:

	**LEX** -- A _lex_ is a vertex of lexspace; each lex has a single Lex parent and an infinte number of lex children, on child for every possible unit key value.
###### older

	**NS** -- A _namespace (Ns)_ is a package of groundings.

	  is a referencable handle for the grounding units.  These handles provide a bridge between the grounding semantics and the structural expression that they grounds.

	??? tree that binds (maps) identifiers onto their grounding units.

	//  ????
	**def type** _LexSpec_: 
		Pathy Tree(IdentPart, Spec)
	
	_

	- 
	**def var** _LEX_ROOT_ Lex

	**def var** _LEX_TOP_ Lex
### --- TEMPLATE ---
#### -- API --

_lang.uc_:
	_**Template**_:
		_**form**_:		slot(a:[Spec], t:Template)
		_**match**_:	slot(a:[self, Unit, *, Env], t:NS)
		_**fill**_:		slot(a:[self, Env], t:Unit)
		_**reduce**_:	slot(a:[self, Env], t:Unit)

#### -- SEMANTICS --

REDUCE -- 
### --- EXE/USP ---
#### _

	**TL;DR.  EXE provides the interpretation framework that underlies all uniform compuation.   It implements uniform interpretation as defined by UNIT CALC.**

	The _**purpose**_ of EXE is to serve as a parsimonious, pragmatic basis for computation that is _performant_ when executed on a von-Nemann architecture.

	**EXE** -- The _EXE construct_ defines Lang interpretation as UCALC rewriting over a LEX grounded semantics.


##### ,
		... with support for Object-Oriented-style computation.
	WTF is OO doing down this low in the spiral?!?  (see OO justification)
	**TL;DR.  The Uniform Structure Processor (USP) implements uniform interpretation as defined by UNIT CALC; its evaluation operator (EXE) implements execution for all Uniform languages.**
	_
#### --- VOCABULARY ---

	**USP** -- The _uniform structure processor (USP)_ implements a least commitments interpreter architecture as a LANG instance top of UNIT structure and unit calculus.  The USP is designed to serve as a parsimonious basis layer underlying all computation.


	These first three terms map pretty directly onto the original von Neuman architecture:

	**MEMORY** -- USP _memory_ is encoded as a bounded structure that changes over time but whose total always remains below some predefined size threshold.

	**PROGRAM** -- A USP _program_ is bounded memory structure (Code) used to control the execution of the USP.

	**DATA** -- USP _data_ bounded memory structures that are the input and/or output of the USP program's processing.


	These next five terms provide the basis for uniform intepretation:

	**NS** -- a USP _namespace (NS)_ is a mapping from identifiers onto grounding structures.  These structures are used by the processor to control the reduction of each exe structure onto to the result of their indicated computation.  Each program is composed of a tree of these namespace mappings, and each point in this tree represents a different "scope" within that program -- a different namespace mapping controlling interpretation performed within that scope.

	**ENV** -- A USP execution _environment (Env)_ contains the local data required to drive the intepretation process.

	**SELF** -- Each environment may have a '_self_' unit.  If so, this unit is used to determine a default namespace (NS) for control of execution using that ENV environment.

	**EXPR** -- A USP _expr_ is a unit that is to be interpreted.

	**EXE** -- A USP _execution_ unit is a structure that points at both program and data stuctures in order to indicate a particular point within an execution that is to be performed.  Each exe will point to a particular NS (namespace) providing interpretation control program, an ENV (environment) providing data, and an EXPR pointer to the code to be executed.
	exe(form, bindings, semantics)


	**FN** -- A _**FN**_ implements expression execution recursively as the execution of some 
	definition expression within a constructed sub environment.

	**ARG_MATCH** -- _**ARG_MATCH**_ is specific application of uniform template matching used in both function (FN) and operator (OP) application.

	_
##### The Original Von Neumann Architecture

	- **PROCESSOR** -- A collection of components that perform logical and arithematic operations based on a supervisory signal (an instruction) indicating which operations to perform.
	- **MEMORY** -- A sequence of memory cells that each contain a fixed width sequence of  bits.  These cells are used to store the instructions used to control the processor as well as the data that is input or output by the execution of these instructions.
	- **INPUT** -- Data provided at the beginning of a computation.
	- **OUTPUT** -- Data derived from the computational process.
	- **PROGRAM COUNTER** -- A part of the processor that tracks the current location in memory being used to control the processor.

	_
#### --- API ---

	import lang.uc.*

	_lang.uc_:
	
		_**Interpreter**_:
			extends:  Lex
			_**exe**_: 		slot(a:[Code, Env, *, unit: Unit, ns: NS], p: exe.reduce)
			_**lookup**_: 	slot(args:[Spec, *, ns: NS], t: Gnd)

		_**Env**_:
			extends: Unit
			_**self**_: slot(t: Unit, o: [Fixed])

		_**Reducible**_:
			extends: Form
			_**reduce**_: slot(args(Env), t: Unit)
		

		_**Fn**_:
			extends: Slot	
			_**form**_: Slot(args:[Expr, Inv...])

		_**ArgList**_:
			Type()
	  
		_**Template**_:
			extends: Form
			_// Bindings from matching to spec_
			_**match**_: slot(args:[_spec_: Spec, *, _env_: Env=None], t: Env)
			_// Fills template with bindings yielding a structure_
			_**fill**_: slot(args:[_env_ Env], t: Spec)
	

	// #######################################
	**import** uc.lang
	
	**def type** _Env_:
		**extends**: Unit	
		**def var** _self_: Fixed Unit

	**def class** _Reducible_:
		**extends**: _Form_:
		**def op** _reduce_(_env_ Env) -> Unit			  // Exe form; updates Env
			   
	**def class** _Lang_:								// State of an interp environment
		**def op** _bang_() -> Env						// Universe for exe of lang
		**def op** _lookup_(s Spec, _ns_:NS=None) -> Gnd  // Maps code to its gnd semantics

	**def class** _Template_:
		**extends**: _Form_:
		**def op** _match_(_spec_ Spec, _env_ Env=None)->Env// Bindings from matching to spec
		**def op** _fill_(_env_ Env) -> Spec			  // OO-method invocation

##### ,
	**def class** _Construct_:
		**def op** text_lang 
		**def op** _spec_lang_ -> Form					// Predicate 4 matching spec
		**def op** _gnd_ -> Gnd							// Grounding defined by this
		**def op** _gnd_(self Gnd, u Unit) -> Gnd		// Executable semantics for unit
	
	// SIMPLE NON-OO INTERPRETER MODEL
	**def type** _Env_ **extends** Unit;

	**def class** _Form_:
		**def op** _reduce_(_env_ Env, ->Unit)	

	// For flyweight & external units
	**def op** _gnd_(self Unit, ->Ident)
	_
	**def op** _ _init_ _ (_self_, env, *, ns, ->Fixed Unit) 
	// Like Unit this is subtyped by adding slots

	// Interprets 'form' using and updating the context of 'env'
	**def op** _exe_(env Env, form Form, self: Unit, ns: NS,  ->Unit)


	// Unit type can be subtyped by adding slots (different than structure)
	**def type** _Unit_:	


	**def class** _Gnd_(T):
		**def op** _ident_(->Ident)				// ??? where does this op inherit from?
		**def op** _ns_ Fixed Key=>Unit			// Mapping defined by package
		**def op** _pkg_ Fixed Pkg				// Package
		**def op** _schema_ Fixed Schema		// Schema for instances w this grounding
	
	**def type** _Gnd_:
		**extends**_:_ Form
		**def var** _pkg_:			Fixed Pkg
		**def var** _unit_schema_:	Fixed BackingSchema
		**def var** _gnd_schema_:	Fixed BackingSchema
		**def var** _action_:		Fixed Spec
		**def var** _gnd_idx_:		Fixed Int
		**def var** _ident_:		Fixed Ident
		// Type is specialized by adding class slots

	**def class** GndGnd:
		extends: Gnd
		**def fn** _exe_(env Env, expr Exec, ->Unit)
	


	_
	_
#### --- SEMANTICS ---
##### -- EXE SEMANTICS AS CODE --

	**import** uc.LEX, uc.LANG

	**def var** _this.env_ -> Env
	**def var** _this.form_ -> Spec

	**def op** _Lang.exe_(_self_ Lang, _code_ Spec, _ns_: NS=None, _env_: Env=None):
		_env_ = _env_ || this.env
		_ns_ = _ns_ || _env_.ns
		_gnd_ = _self_.lookup(_code_, ns: _ns_)
		**if** gnd.has(`match):
			_env2_ = _gnd_.match(_code_, env: _env_)
			**return** _gnd_.form.reduce(_env2_)
		**else**:
			Lang.form = _code_
			**return** _gnd_.form.reduce(_env_)

	**def op** _Lang.run_(form Form, env Env):
		if form.isa(`Template):
			gnd = env.lang.lookup(form)
			env2 = env.NEW(gnd)
			form.match(form.spec, env: env, into: env2)
			env2 = form.matcher(env)
			return form.reduce(env2)
		else:
			return form.reduce(env)

	**def class** _Fn_:
		**extends**: Reducible, Template, Spec
		**def op** _reduce_(env Env):
			**return** this.lang.exe(_self_, env: _env_)


	_
##### -- Compiling semantics --

	stack of env frames
	stack of values

	block(
		call(args, body))

	SUMMARY
	- **MATCHER** -- the 'form' created by a matcher instance foo(7) is a reducer that creates a new stack frame, given only the current stack frame.
	- 
	CHN()
	_
##### -- Functional Breakdown --

	What is going on in the default EXE semantics as defined below:

	- EXE -- The intepretation (exe) of an expression (expr) is controlled by two pieces of information, the execution state (Env) and semantics (NS).
		These are each fully expressed as:
		- ENV -- the data accesible from the 'environement' subgraph.
		- NS -- the groundings accessible from the 'namespace' index into the lexspace graph of semantics-grounding objects.

	- LOOKUP -- The default semantics-lookup operator uses a unit head as an index into lexspace at ns in order to obtain the grounding that controls its interpretation.







	// The uniform least-commitments model of OO-style unit interpretation

	def fn _**exe**_(_expr_ Unit, _env_ Env, _ns_ NS, ->Unit):
		_ns_ = _ns_ || env.self.gnd					
		_grounding_ = lookup(_ns_, _expr_)				
		**if** atomic(_grounding_):
			**return** spec_ground(_grounding_, _env_)
		**else**:
			**return** spec_invoke(_grounding_, _expr_, _ns_)

	def fn _**lookup**_(_ns_, _expr_):
		**return** _ns_.path_get(_expr_.head.nub)

	def fn _**spec_ground**_(_grounding_, _calling_expr_ Unit, _env_ Env, _ns_ NS):

	def fn _**spec_invoke**_(_grounding_, _calling_expr_ Unit, _env_ Env, _ns_ NS):
		_env2_ = match(_grounding_.args, _calling_expr_, _env_)
		**return** exe(_env2_, _grounding_.body, _ns_)

	def fn _**match**_(_template_ list, _callform_, _env_ Env):
		env2 = copy(env)
		for i, arg in enumerate(template):
			env2[arg] = exe(_callform_[i], env, ns)
		return env2

	_
##### -- Functional Breakdown with self --

	// A generalized model of unit interpretation
	def fn _**exe**_(_expr_ Unit, _env_ Env, _self_unit_ Unit, _ns_ NS, ->Unit):
		_grounding_ = lookup(_ns_, _expr_)
		**if** atomic(_grounding_):
			**return** reduce(_grounding_, _env_)
		**else**:
			**return** invoke(_grounding_, _expr_, _env_, _self_unit_, _ns_)

	def fn _**lookup**_(_ns_, _expr_):
		**return** _ns_.path_get(_expr_.head.nub)

	def fn _**invoke**_(_grounding_, _expr_ Unit, _env_ Env, _self_u_ Unit, _ns_ NS):
		_env2_ = match(_grounding_.args, _args_, _env_)
		_env2_["self"] = _self_u_  
		**return** exe(_env2_, _grounding_.body, _ns_)

	def fn _**match**_(_template_ list, _callform_, _env_ Env):
		env2 = copy(env)
		for i, arg in enumerate(template):
			env2[arg] = exe(_callform_[i], env, ns)
		return env2

	_
##### -- OLD Functional Breakdown with SELF --

	// ADD SELF
	def fn _**exe**_(_expr_ Unit, _self_unit_ Unit, _env_ Env, _ns_ NS, ->Unit):
		_ns_ = _ns_ || _self_unit_.ns       # **<-- Semantics follow 'self'**
		_grounding_ = lookup(_ns_, _expr_)
		**if** atomic(_grounding_):
			**return** reduce(_grounding_, env)
		**else**:
			_env2_ = match(_gnd_.schema, _expr_, _env_)
			env2["self"] = _self_unit_  # **<-- field data follows 'self'**
			**return** exe(_env2_, _grounding_.body, _ns_)
		
	_
##### -- Executable Class Breakdown --

	class _**Reducable**_:
		def op _**reduce**_(_env_ Env, ->Unit):
			pass   # native reduction operation goes here

	class _**Grounding**_ extends Reducable:
		def var schema(t: Unit)  # calling template

	class _**ReducableCallForm**_ extends Reducable:
		def fn _**init**_(_callform_ Unit, *, _env_ Env, _ns_ NS)
			ns = ns || _env_.ns
			_self_.callform = _callform_
			_self_.grounding = lookup(_ns_, _callform_)
		
		def op _**invoke**_(_env_ Env, _self_unit_ Unit):     # Method operator call
			env2 = _self_.match(_self_.callform, _env_)
			env2["self"] = _self_unit_
			**return** _self_.grounding.reduce(env2)
		
		def op _**reduce**_(_env_ Env):						# Function call
			env2 = _self_.match(_self_.callform, _env_)
			**return** _self_.grounding.reduce(env2)
		
		def op _**match**_(_self_, _env_ Env):
			_env2_ = copy(_env_)
			for _i_, _arg_ in enumerate(_self_.grounding.schema):
				_env2_[_arg_] = _self_.callform[_i_].reduce(_env_)
			

	_
##### --- Semantics as code ---

	**EXE -- UNIFORM INTERPRETATION SEMANTICS**

	**def fn** _exe_(_env_ Env, _expr_ Executable, _self_ Unit, _ns_ NS, ->Unit):
		_self_ = _self_ || _env_.self
		_ns_ = _ns_ || _self_.gnd.ns
		_gnd_ = lookup(ns, expr)
		_env2_ = _gnd_.schema.match(_expr_, env: _env_)
		**return** exe(_env2_, _gnd_.action)

	_ns_[_expr_.head]

	**SELF** -- By default the "self" unit is taken from the interpretation environment, but it may also be specified explicitly (see line 1 above).

	**NAMESPACE** -- By default, ns, the namespace used for intepretation is taken from the grounding of its self unit, though it may also be specified explicitly (see line 2 above).

	**GROUNDING** -- An expression structure is interpreted according to the "grounding" (gnd) associated with the head of that expression (see line 3 above).  

	**SCHEMA** -- An expression is interpreted by first matching its structure onto a schema for its structure.  The result of this mathing is an new interpretational enviornment with parts of the matched structure bound to variables within that env (see line 4 above).

	**ACTION** -- Expr interpretation is completed by intepreting the rule's action expression within the context of the variables bound by matching its schema (see line 5 above).

	_
#### --- API IDEA ---

	class Env extends list
		__slots__ = ("self", "ns")

	class Gnds:
		def compile(expr)   # ->Reducable)

	class Exe:
		def reduce(env)		# ->Unit


	class ExeChn:
		def __init__(spec)   # spec
		def reduce(env)

	class ExeBra:
		def reduce(env):
			env2 = env if base is None else Gnds.sub(env, base)
			for i, cond in enumerate(conditions):
				if Gnds.sat(cond.reduce(env2)):
					for action in actions[i]:
						action.reduce(env)

	???? load(expr, self_type)
		
	

	_
#### --- FN ---

	_
#### --- Discussion ---
##### -- The building of an exe universe --
	Lang extends GndGnd
##### -- Componential Semantics Idea --

	E = Env = all local, global, definitional data needed for interp
	F = Form = an executable form

	EXE(F, E{self,args,...}, )

	F E |- result												F.eval(E)
	F >> self |- result   ==   F E{self=self} |- result       	F.call(x, E)
	F(args...) |- result  ==   F E{0,1,...=args} |- result    	F.call(..., E)


	fn  fn(x){x+x}  double(d)    			{d:5}
		if   		if v {x=1} else {x=2} 	{v:True}


	LOAD BEHAVIOR
	form1 >> form1.head |- form2

	double: fn(x){x+x} |- 

	spec.match(U($arg1), E.NEW('double^schema))

	qq = double(rr)

	stack_frame = E.NEW(double)
	double.match(`double(rr), stack_frame)	// fill frame
	double.eval(stack_frame)

	qq = EXE(double, env: double.match(`double(rr), E.NEW(double)))
	qq = double.EXE(double.MCH(`double(rr), E.NEW(double))




	_
#### --- CHOICES ---
##### -- Lang composed of constructs --
	DECIDE: Lang composed of constructs.
	- Would be simpler if constructs are just forms
	- Forms are not always templates with text/spec/form formats
	- Constructs are not always interpretable (but valuable if they can)
##### -- Constructs are kind of like a 'class' - but distinctly named --
	DECIDE: Constructs are mini-language templates for form instances
	SAME AS CLASS 
	- Like a class in begin a template for instances

	DIFFERENT
	- Instances are specialized to be CODE instances that can be reduced.
		NOPE: some constructs need not be executable, & some class constructs could
	- May serve multiple types (classes) of code types.
		STANCE: Treat that grouping as a synthetic class.
	- Separate from the grounding object; which is also part of a class
	- Nice to use a distinct word 'construct' since class already used
	_
##### -- Constructs are may or may not be forms --
	DECIDE: Construct extends Unit, but specific ones extend Form
	- Sometimes nice that construct-class instances are executable
		e.g. HeadCase
	- But it is not needed for all to be executable, this can be chosen on a case by case basis.  
	- Means that 
#### --- OLD ---


	def fn _**invoke**_(_grounding_, _expr_ Unit, _env_ Env, _ns_ NS, ->Unit):
		_args_ = exebody(_expr_, _env_, _ns_)
		_env2_ = match(_grounding_.args, _args_, _env_)
		**return** exe(_env2_, _grounding_.body, _ns_)

	def fn _**exebody**_(env Env, body Unit, _ns_ NS, ->Unit):
		**return** [exe(env, arg, ns) for arg in body]


	~-~-~
	**def fn** _invoke_(gnd Grounding, expr Unit, env Env, _ns_ NS, ->Unit):
		env2 = copy(env)
		for i, arg in enumerate(template):
			env2[arg] = exe(env, expr[i], ns)
		**return** exe(_env2_, _grounding_.body, _ns_)


	**ENV** -- a USP execution _environment (Env)_ contains all information required to perform expression interpretation.  


	The contents of memory are split into two categories: Program and Data.

	CODE -- 


	**ENV** -- An _interpretation environment (Env)_ is a structure containing all information required to perform uniform intepretion over any well formed expression.

	**EXE** -- The primary Env operator is _exe_ -- it performs uniform interpretation over the provided unit structure.

	**Form** -- A _form_ is structure that is to be intepreted.

##### older

	**EXE** -- The primary Env operator is _execute (EXE)_.  It performs uniform interpretation over the expression provided to it.

	**GND** -- A _grounding (Gnd)_ unit provides semantic information -- information that defines the behavior of the EXE operator. 



	Formally, an Env is a frame instance providing persistence for the variables defined within that enviornment.  At a minimum each of these environments must define a **self** unit providing structure for the interpretation, and **^ns** providing the function for the interpretation.  (Usually this Env also defines local variables, operator arguments as well -- think of an Env as a stack frame)

	Formally, 
	- An IdentPart is a String or Int which is one part of an identifier.  
	- An Ident is a unit whose contents field contains a list of IdentPart.  
	- These Ident provide addresses to unique nodes within a namespace.
	- A namespace is a tree of IdentPart onto Gnd groundings.
	- A Gnd unit contains universe-specific information used guide execution.
		(could be bytecodes, or references to external libraries)

	_
##### older

	**def type** _NS_: Tree(Exec)

	xxx
		**def fn**	_new_(_self_ Unit, ->Env)
		**def fn**  _lookup_(Ident||Path, ->Exec)

		$_ns_:	  NS(Exec) getter   // Needed?
		**def fn** 	_EXE_(exprs Exec..., unit: Unit, ns: NS, ->Unit)
	_
##### _

		$_gnd_:		 Gnd getter				# dynamically created
		$_children_: List(Ident) getter  	# XXXX this is just 'self'
		**def fn** 	   _GET_(IdentPart, ->Ident)

    
	**def class** _Gnd_:
		extends   List(Unit)				  # reducers & type vars
		$_ident_:	Ident getter
		$_sub_gnd_:	List(Gnd)
		$_schema_:	BackingSchema getter
		**def fn**	  _Gnd_(Ident)
##### -- GND --

	**GROUNDING PARTS**
		1:		Reducer for level 1, inline GND forms
		2:		Reducer for level 2, special GND forms
		3:		Reducer for level 3, declared GND forms
		4:		Reducer for level 4, meta-delarative GND forms
	
		frame:	The FrameSchemaSpec bound here
		vtable:	The VirtualTable instance bound here


	METHODS
	- accessors: frame, vtable, 
	
	_
##### -- Simple Semantics --



	**def** op _EXE_(env: Env, forms Code..., alpha: ='eval):
		**while** not forms[-1] =~ Gnd:
			expansion = env.lookup(forms[-1], alpha)
			forms.append(expansion)
		**return** expansion(env, forms)

	**def** op _lookup_(env: Env, form, env, level=2):
		grounding = env.ns.path_get(form.head())
		if grounding.head() != 'Meaning:
			error("not defined")
		result = meaning[alpha]{level]



	def EXPAND(*forms, ns, alpha):
		last = forms.last
		while last.head =~ 'Gnd:
			last = ns[last.head] or ns['GAP]
			alpha_form = last[alpha]
			forms.append(alpha_form)
		return forms

	class Env(Unit):
		def GET(form):
			return REDUCE(EXPAND(form, ns=self.ns, alpha=self.alpha))
		def SET(form, value):
			return REDUCE(EXPAND(value,form,ns=self.ns,alpha='ASSIGN))

	def REDUCE(exprs, env):
		path = exprs[-1]	

	def fn bang(ns):
		nsp 	= ns.lang.prime
		seeds	= ns.lang.seeds
		globals	= ns.lang.seeds.heap.NEW(ns)
		env		= seeds.env.NEW
		run(nsp.expr)
	

	_
###### - older Semantics & Impl Sketch -
	Env:
		unit	--  defining 'self' Unit
		ns 		-- 	defining Namespace
		EXE		--  Interpret code within env context
		lookup	--

	InterpTypes:
		place	--  Get place
	

	def class Env:
		ns: NS
		self: Unit
		def op EXE(forms: ...Code, alpha: Unit, self: Unit) -> Unit
		def op lookup(i Ident, ->Exec)
		def op boot (form, ->Env)

	def interface Reducer:
		def __ call __(env: Env, exprs: List(Code), -> Unit )
	def type Ident1 = re"^([a-zA-Z_ $[a-zA-Z\d_$]*)$
	def type Ident = List(Ident1||Nat||Meta)
	def type Ns = Tree(Ident1||Nat||Meta, Reducer)


	**def** op "+"(_e_ Env, _ns_ NS):
		**return** _e_.NEW(_e_, _e_.ns + _ns_)

	- Gnd
		- Gnd.structure .fns .pkg .doc 	--  The structural definition grounded here
	- Env.EXE - 
		- grounding = fns[head]
		- Env.apply(grounding, forms, ...)
	- Env.apply(grounding, forms, ...)
		- reducer = grounding.GET()
		- structure = meaning.
		- subenv = __ env __.NEW()
##### -- Discussion --
###### - generalized interpretation -
####### Type Case Examples 
	x = this.is.a.virtual.function( form )
	this.is.a.virtual.function: TypeCase::
		Class1Name: op(...)
		Class1Name: op(...)
	
	uf.TypeCase: GND:: 2: "..."

	    -~- uc -~-
	    ::
		    pkg this.is.my.pkg.path;
		    foo.bar: pyfn              # without args it uses 'bar'
	
	    -~-

	x = this.special.language( some_form )
	this.special.language.GND.2: 
##### -- Older --
###### EXE
		EXE(form0, form1, ..., ctx:) ==
		??? should we relegate 'ctx' to the ctx command somehow???  yes if we can
		==>      with meta {...}
		==> 	CTX(ctx_name) {...}


		EXE(form1, ..., alpha: alpha, self: self, ns: ns) ==
###### Implementation

		Lookup(path, alpha) = 

		//  nsptr=   headpath=  idx=   topform=   forms=
		ns = 
		ptr = 
		forms = args_in_reverse_order
		head  = forms.last.head

		if nsptr.head!='OP:	
			nsptr = nsptr[headpath[idx]} || 
			idx+=1

		if nsptr.head=='OP:
			if idx<len(headpath):  
				self = invoke OP without top form  // continue eval with no forms and new self
			else:
				return invoke OP with topform
		
####### Impl try 2

		High level

		Invoke
		- frame = env.NEW()
		- impl.fill_frame(frame, fn.args)
		- fn.ctx.enter(frame)
		- return impl.call(frame, fn.body)



		EXE FRAME
		-- i1 i2 i3 ...
		-- headpath

		headpath idx 
###### Example of multi-level interpretation

		Level 0 -- Immediate Form:	GND
		Level 1 -- Grounded Form: 	BRA --> GND
		Level 2 -- Special Form: 	if --> DEF --> GND
		Level 3 -- Standard Form:	double --> fn --> DEF --> GND
		Level 4 -- Declarational Form:	nputation --> macro --> DEF --> GND


		Spiral 0 -- GND
		Spiral 1 -- DEF, BRA, BLK, ...
		Spiral 2 -- fn, macro, class
		Spiral 3 -- nputation ???
###### Simplest Presentation

		This is a specification of Alpha-interpretation a simplest model of interpretation assuming
		uniform structure and intended for coverage of modern programming (specifically aimed at 
		capturing notion of multiple ways of interpreting a single form, and aimed as languages
		where data object may carry semantics, e.g. "objects")

		**Assumed Background:**
	
			NS{[ unit ]} -> Namespace
				Framework assumes there is some mechanims for binding a semantic namespace to each unit of data.
			GROUND{[ interp_form ]} -> result
				The grounding function reduces a fully expanded form into its interpretation


		//  INTERPRETATION CONTEXT
		//  f0,...,fj	Sequence of forms to be semantically expanded then reduced
		//  env: 		The interpretation environment used to launch this interpretation. (default is current env)
		//  self:		The actor performing this interpretation. (default is self in env)
		//  ns:         The namespace providing semantic terms underlying interpretation step.  (default from self)
		//  alpha: 		The kind of intepretation begin performed. (default is eval)
		interp( f0, f1, ... fj, env:, self:, ns:, alpha: )


		//  SEMANTIC INDEXING -- Namespace reduction via semantic lookup
		//  when f0 isa Symbol then ns1 = ns[f0] or --somehow-get-default--
			interp( f1, ..., ns: ns1 )


		//  SEMANTIC EXPANSION -- Add operator expansion in front for subsequent interpretation
		//  when ns=OP(...) then expansion=OP[alpha],  h=path.pathify(expansion.head()) 
			interp( h[0], h[1], ..., h[i], expansion, f0, ... )
	

		//  SEMANTIC GROUNDING
		//  when ns=GND(...) then result=GROUND( ...all interp args... ),  i=ns[`level]
			interp( fi, ..., fj, env:, alpha:, self: result, ns: NS[result] )
###### Spec (Discussion)

		- Uses the current or supplied 

		<dd>Creates a mew execution Env using the supplied <v>env</v> or active env as a parent.
> 	            Adds passed <v>self</v> or uses the curren one, and adds <v>bindings</v> as passed or from the self unit.
		            Finally extends the sequence <v>form<sub>i</sub></v> by expanding the last element until it is grounded,
		            then reduces the resulting Env.  Returning the result of interpretation.
		        </dd>


		_
###### - ISSUE: Alpha-interp == segregated or unified tree -

		Choice #1:  Unified alpha tree w. many alpha types
		Choice #2:  Seperated interp trees

		Unified
		- >>> Easy to group many aspects of construct into one pkg

		Seperated
		- >>> More elegant interp model
		- >>> Decomplects interp-types so they have their own tree

		Alpha-tipped - first level of tree is always alpha
		- >>> Chroot operation is allocating, needs to construct new root

		See LIB.FORM.VTABLE discussion
###### Thread API
		- impl	
			- CHN()
			- CTX()
			- RET()
			- NEW(ns, ...)		-- Creates & returns new unit frame on top of thread stack
			- pop(unit)			-- Pops down to and removes specified frame

			- stack: List[Unit]	-- Thread's list of stack frames
			- env: Env			-- Current env frame (not always top of stack)
			- ns: NS				-- Current namespace
			- base: Env			-- 
			- alpha: Code		-- 
			- forms: List[Code]	-- 
###### IMPL-PY-Interp-Style
		- 
###### IMPL-C-Style
		- Implementation Pointers
			- 'env' points to active stack frame byte 0.
			- 'tos' points to last part of stack in use (assuming downward growth stack)
		- Env format
			- env[0] is pointer to namespace for this frame
			- ptr[1] is pointer to prior active frame
			- ptr[2] ... are the fixed locals for this frame
		- Returns Values
			- Immediate Return Values are always stored in tos[0] & optionally tos[1]
			- Multi-values always have offset stored at tos[2] & len at tos[3]
		- Operators
			- push(frame)  -- adds frame to tos and moves tos to account for new data
			- settos(frame)   -- causes tos to point to this frame (making it top)
			- activating a different frame is just a matter of using it,
				tos is set so data will not be overwritten.
###### IMPL in GC language
		- Implementation Pointers
			- Native stack is implicitly used for env frames by ensuring they are dynamically scoped within the 'calling' frame
			- Native stack is also used for try block by storing patch and bra form as locals in that try block frame
			- Thus an env is really just a 
###### FUNCTION define   structure / function discussion
		the structure of a thing is the graph of relationships intrinsic to that thing indepent of any specific semantics, interpretation or processing you might apply to it.
		the function of a thing is the semantics of the thing -- the interpretations or processing you might apply to the thing indpendent of the specific structure intrinsic to that thing.

		so the structure of a chess board would be encoded as the relationship between the squares and the peices.  The function would be encoded as a set of legal moves, game objectives, strategies, etc.

		Notice the structure of chess board might serve as a basis for checkers, or to represent a 64x64 bitmap image.  Conversely one might map the function of chess game semantics onto a string as the underlying structure.  The string having a sequence of three character codes: a lower case letter is used for white piece, uppercase for black, followed by two digits indicating piece position.
 
		list of pieces and coordinates for each piece encoded as a two digit number.
###### FUNCTIONAL the thorny knot

		CORE PIECES
			INV(form)		NEW; FillFrame; EXE body; RemoveFrame; Return Result
			TRY(patch, from, catch, finally)
			Env.BANG(self,ns,form)	New thread, w. self, ns, and form (defaulting to Env)
			Env.LOAD(patch,|seed,form)	Creates new env, using seed or env & adds patch
				if form then runs form and removes patch.
			NEW(structure, frame_var: expr_value, ...)  
				-- on env this does not change active frame
		NS
			NS[0] is the current
	
		ENV OPS
			Env = pointer to the active frame within a stack of frames
				ptr = frame ptr points to first byte AFTER negative fixed offsets locals
				I ptr[-1] = frame pointer for prior frame
			return frame.  if multi-value, then ptr[0] is offset and ptr[1] is len
				if room exists it is placed BEFORE current frame, if not, then after.
			push(structure)->instance    returns instance added (does not change active)
			pop() -- pops top frame (which should not be active)
	
	
	

		Env.with_frame()

		Env.invoke(frame_structure, frame_fill, frame_body)
		- Adds frame to stack. 
		- EXE frame_fill while in ORIGINAL frame, and with 'it' pointing to NEW frame.
		- Sets new frame as active stack frame.
		- EXE frame_body keeping result.
		- Sets original frame as active stack frame.
		- Returns result.

		Env.enter(ctx_patch, ctx_body, ctx_ctx_final)
		- Adds frame to stack with room for new patch value
##### -- garbage --

	provide a performant basis


	XXXXXXXX

	The EXE construct formalizes a von-Neumann-style architecture within the Uniform ecosystem.  It builds upon UCALC and provides the basis for UNICORE as semantic assembly langauge providing a higher-level substrate for specification and interoperation of modern software langauges.

	the EXE construc

	-- We refer to EXE as a semantic interpreter.
	-- It is similar in complexity to a turing machine, and 
	-- leaves specific details of processor execution open-ended as the von Neumann architecture does.
	-- we call it a semantic interpreter 
		-- formalizes a number of key ideas underlying most SW languages 
		--	provide context for broadest interoperability by sharing these concepts 	in most genearlized context.
	-- key underlying concepts include:
		--  types
			--  loadtime/runtime -- type erasure
			--  object orientation; (type-caseing)
		--  program scope
		--	explicit mapping of structure to meaning
		--  stack based execution

	_
### --- OBJ ---
#### _

	**OBJ** -- The _**OBJ construct**_ defines new unit spaces and object-oriented method definitions recursively in terms of uniform interpretation.

	**CLASS** -- A _**class**_ defines the structure and function of a new category of units.

	**SLOT** -- A _**slot**_ defines an element of the structure or function within a class of units.

	**CHN** -- The _**chaining operator (CHN)**_ defines it execution recursively in terms of execution within a sub environment defined by the class of the "self" instance.


##### ,

	**OP** -- An _**OP**_ is a function that 

	**TEMPLATE** -- A _**template**_ is a structure that can be _matched_ against another to generate a collection of bindings, or can be _filled_ in by a collection of bindings to produce a resulting structure.  Arg processing for Fn and Op are both done as template matching.

	**OBJ** -- The OBJ construct supports recursive implementation of uniform execution (EXE).
#### --- API --- 

	lang.uc:
	
		_**Class**_:
			extends: Pkg:
			_**args**_: ArgList				# Template args for this class
			_**init**_:	Slot(t: None)		# Initializes new instances of class
			_**form**_:	Slot(t: Unit)		# Creates or looksup instances from spec
			_**NEW**_:	Slot(t: ???)		# Seed
			_**inst_path**_: Slot(t: ???)	# Path from Env for access to instances


		_**Slot**_:
			extends: Pathy(SlotSchema):
			_**args**_: ArgList
			_**name**_:	Slot(t: Ident, extends: Fixed)
			_**type**_:	Slot(t: Ident, extends: Fixed)
			_**expr**_: Slot(t: Expr, extends: Fixed)
			_**key**_:		IdentKey	getter	# key to access (maybe 'path')
			_**spec**_: 	VarSpec 	getter	
			_access_(Env, ->T)
			_assign_(Env, T)
			def op** _accessor_(Env.meta, base_accessor Form) -> Form
			def op** _assigner_(Env.meta, base_accessor Form, value Form) -> Form
	
			**def op** _form_(Env.meta, call_form Spec) -> Invokable(T)
		
		_**CHN**_: 
			_**form**_: Slot(args:[Expr, Inv...])
	

	_	  
#### --- SEMANTICS ---


	**def fn** _chn.form_(_spec_: Spec, _parent_: Form, _accepts_ Type, _returns_ Type, 
						_env_: Env=None) -> Reducible:

	**class** Chn(Gnd, Form):
		@static
		**def** form(cls, ctx):
			with ctx.spec = ctx.spec[0]:
				self.first = ctx.lang.form(ctx)
			prev_ret_type = self.first.returns
			self.rest = []
			for s in ctx.spec[1:]}:
				with ctx.spec = s, ctx.self_type = prev_ret_type:
					 form = ctx.lang.form(ctx)
					 prev_ret_type = form.returns
					 self.rest.append(form)
						
		**def** reduce(env: Env):
			u = self.first.reduce(env)
			for inv in self.rest:
				u = inv.invoke(u, env)
			**return** u


	**class** Slot(Gnd, Form):
		@static
		**def** form(cls, ctx):
			spec = ctx.spec
			gnd = ctx.lang.lookup(spec)

		**def** invoke(unit, env):


	x.foo ==> dot(x, foo) ==> CHN(x, foo)
		lookup( foo ) => gnd1==PyObjSlot( n:foo, k:3 )
		gnd.form( foo ) => invoker_foo == 
			key => 3
			{ return base[key] }
		
	x.foo(7) ==> dot(x, foo(bar)) ==> CHN(x, foo(bar))
		lookup( foo(bar) ) => gnd1==PyObjSlot( n:foo, fn: py_fn1 args: [a1] )
		gnd1.form( foo(bar) ) => invoke_foo
			py_arg_builder(gnd1.args) => arg_builder1
			lookup(bar) => gnd2  gnd2.form(bar) => reducer_bar
			invoker_foo


	def py_invoke(slot, spec)
		args = slot.args
		method = getattr(slot.klass, slot.key)
		if len(args)==1:
			return py_invoke1(method, spec[0])
		
	def py_invoke1(method, arg)
		def inv(base, env):
			a1 = arg.reduce(env)
			return method(base, a1)	

	_				
#### --- OP ---
##### _

	**OP** -- An Op The _**OBJ construct**_ extends uniform execution (EXE) to include


	**OBJ** -- The _**OBJ construct**_ provides an implementation of the Uniform Unit using Uniform's Execution model (EXE) to implement the unit's structural and functional operators.  

	According to the OBJ construct each Unit must be an element of some unit class.
	Each unit is a combination of "backing" information that varys from one element to the next within that class, and "grounding" information that is common across all elements of the class.

	**GND** -- The _**grounding**_ (gnd) of an object is that part that is common across all instances of the class.

	**BACKING** -- The _**backing**_ of an object is this instance's varying part.  The In general the backing of an object will be of a different type (have a different grounding), but 


	WELD -- 

	MATCH -- 

	INVOKE -- 

	_
##### -- API --

	**def op** _Lang.exe_(_self_ Lang, _code_ Spec, 
				_unit_: Unit=None, _ns_: NS=None, _env_: Env=None):
		_env_ = _env_ || this.env
		_ns_ = _ns_ || _env_.ns
		_gnd_ = _self_.lookup(_code_, ns: _ns_)
		**if** _gnd_.isa(`Native):
			**with** _this.form_ = _code_:
				**return** _gnd_.form.reduce(_env_)
		**elif** _gnd_.isa(`Op):
			env2 = gnd.match(code, env: env)
			 _unit_.gnd[]	
			_env2_.self = _unit_					// Added this line for OO exe
			**return** _gnd_.form.reduce(_env2_)
		**elif** _gnd_.isa(`Fn):
			_env2_ = _gnd_.match(_code_, env: _env_)
			**return** _gnd_.form.reduce(_env2_)
		**else**:
			error(fmt"{_gnd_} is not executable")




	~-~-~
	**def type** _Class_:
		**extends**: Gnd

	**def class** _Invocable_:
		**extends**: _Form_:
		**def op** _invoke_(_self_ Invocable, _u_ Unit, 
					_env_: Env = None) -> Unit	  // OO-method invocation
		**def op** _returns_ -> Gnd	


	**def op** _invoker_(env_gnd Gnd, self_gnd Gnd) -> 
				Fn(_env_ Env, _self_ Unit,->Unit)	// Compiles OO-invocation

	_
##### -- Semantics --
###### - As Code -

	**def op** _Lang.exe_(_self_ Lang, _code_ Spec, _ns_: NS=None, _env_: Env=None):
		_env_ = _env_ || current_env
		_ns_ = _ns_ || _env_.ns
		_gnd_ = _self_.lookup(_code_, ns: _ns_)
		**if** gnd.has(`match):
			_env2_ = _gnd_.match(_code_, env: _env_)
			**return** _gnd_.form.reduce(_env2_)
		**else**:
			Lang.form = _code_
			**return** _gnd_.form.reduce(_env_)

	// OO-method invocation
	**def class** _Op_:
		**extends**: _Invocable, Template, Form_:
		**def op** _invoke_(_self_ Op, _u_ Unit, _env_: Env = None):
			env = env || current_env
			env2 = self.match(self)
		
		**def op** _returns_ -> Gnd	

	_
##### -- Generality Discussion --
	**THE UNIFORM OBJECT MODEL**
	- GRAPH MODELS -- 
	- VON NEUMANN MACHINE -- 

	- BIG CONFIG -- The size of the space of possible configurations for a typical program written for a von Neumann machine is typically much much larger that the size of memory for these machines and thus larger than the programs run on these machines.

	- ONE CODE MANY CONFIGS -- This means that some fixed code must be used operate over many instances of some common underlying pattern of data.  (e.g. multiply)

	- GROUNDING -- we call the code used to operate as the grounding.

	- BASE -- we call an instance of the underlying pattern the base.

	- GROUNDING SPACE -- the space of all possible underlying base patterns is called the grounding space.

	- BASE-GROUNDING-OPERATIONAL-SPLIT -- We can label memory as 'base' if it is an instance of this underlying pattern, or 'grounding' if it the common code that operates over these instance(s). 

	- UNIVERSALITY OF BASE/GROUND/OPERATOR split.
		- often can decompose full system into recursively smaller 
			base/ground/operator splits
		- might degenerate into ground is whole memory, which becomes data as it rewrites itself... even crazy degenerate case fits model

	- ALL DATA IS GRAPH

	- NO INCONSISTENCY HERE -- can represent von Neuman memory as graphical model at a base level it can be a dense array of fix range values.  But same memory can be interpreted as graphical data in many other ways too.

	- HANDLED DATA -- Any time we want to operate over some collection of data we may find or create a single vertex to 'handle' that data.
		- may be found with existing graph structure or may add.
		- may only exist in a different space
		- may be explicitly encoded in von Neumann machine or 
			implicitly encoded within space of derived data.
	
	- HANDLED-BASE, HANDLED-GROUNDING -- both kinds of data are expressed in von Neumann memory, and both can be handled, thus there will be a handle in some explict or implict space for each of these data values.

	- GROUNDING CONTAINS OPERATORS -- code needed to implement operators is handled by the grounding.  Grounding operators must be directly interpretable/executable.

	- NODE OPS IN GROUNDING -- Needed node-centric operators are provided by the grounding.  Indeed it is by putting apporpriate grounding code on the von Neumann machine that we enable existing memory/data to be re-interpreted in more abstracted graphical models.

	- UNIT -- We define a "unit" to be the pairing of some base vertex with some cooresponding grounding vertex.  
		- UNIT -- The unit pairing itself may be handled meaning there is a vertex that denotes the unit pairing itself.   Here we use u to denote a unit.
		- BASE -- we use the notion  u.base to denote the base vertex of a unit u
		- GND  -- we use the notion  u.gnd  to denote the grounding vertex for u
	
	- TURTLES ALL THE WAY DOWN -- All units have a base and gnd, and each of these are also units.  So they will recursively have a base and unit.  This process terminates when unit is its own base, which implies its grounding is the same as its base's grounding.  The needed structural operators within the grounding of these lowest level units must somehow be directly interpretable by the von Neumann machine itself.

	- WELD -- Groundings may provide a 'weld' operator as a means of obtaining units with a specifieid base.  Each grounding may only operate on units within some specially represented space on the von Neumann machine, thus when welding a base onto the grounding may require the base to be copied into a cooresponding base within that space's representation.  Thus the resulting unit's base is may not be identical to the original, Further the resulting unit may or may not be seperately represented from its base, thus we see that two units welded from the same base will be equal but may or may not be the same unit.  These choices depend upon the semantics and implementing of this particular grounding space. unit = grounding.weld(base)
		unit2= grounding.weld(base)
		- SAME GROUNDING --  unit.gnd EQ grounding
		- EQUAL BASE --		unit.base EQUAL base
		- EQUAL UNITS --		unit EQUAL unit2
	

		- that is EQUAL to the supplied base structure, and whose grounding IS the 


	~-~~
	- DATA IS MEMORY -- The von Neumann Architecture uses a dense array of memory locations to store with fixed integer

	_
##### -- implementation cases --

	==KINDS==
	- Py UnitContainer
	- Py UnitWrapper of Py object
	- NS-backed unit
	- NS-backed unit wrapper
	- Derived python class (eventually with compiled methods)

	PyClassGnd
	DbPerson	int		list	DbPersonSqlOrm		//  Flyweight DB interface
	DbPersonSql	spec	list	SqlOrm				//  
	DbPerson	list	
	Num			double	double	PyDouble			// Wrapping Python objects	
	PyDouble	DoubleSOP		SopContainer		//

	Scan Module/Class and build NS


	// Wrapped Py ATOM
	Example	3.14
	pytype	PyNum extends UnitAtomAdapter
	base	double	
	gnd		PyClassGnd(PyNum)	
	c arg	number

	// Grounding of an adaptor class
	Example	PyClassGnd(PyNum)	
	pytype	--generated--
	base	PyNum
	gnd		PyClassGnd(PyClassGnd)
	c arg	PyClassGnd


	// Generic Unit
	Example person(first: "dan")
	pytype  UcSpec extends Spec
	base	UcSpec
	gnd		PyClassGnd(UcSpec)
	c arg	{"^":"person", "first":"dan"}

	// Simple Object
	Example	`person(first:"dan")
	pytype	Person extends Form     // Class is generated
	base	["dan"]
	gnd		PyClassGnd(Person)
	c arg	

	_
##### -- API discussion --

	NS 	 = Tree(Label, Executable && )
	base = unit.base -> Unit
	ns	 = unit.gnd -> NS

	GROUNDING RING
	x	 = gnd || type || ns || static_op
	ns	 = x.ns									#  NS is tree of Executable 
	gnd  = Gnd(x) = Gnd.weld(ns)				#  GND is raw shared data
	type = Type(x) = Type.weld(gnd)
	st	 = StaticOp(ns) = StaticOp.weld(ns)

	 TYPE  BASE  GND    GROUND TYPES
	 NS				  
	 Gnd   list  Gnd
	 Type  Gnd   Type
	 SOP

	 UnitSOP	Unit	SOP		Used to convert SOP objects into fused units
	 UnitNS		Unit	NS
 


	SEMANTICS
	- u == Gnd.weld(u.gnd).weld(u.base)
	- u == Type.weld(u.type).weld(u.base)


	_
##### -- Cleavable Types Discussion --

	OBJ -- A _**uniform object**_ (OBJ) is an element of some semantic class -- it is the combination of the _structural_ and _functional_ information for that instance.

	- **unit <-> base + ctrl**	-- Able to split _unit_ into _base & ctrl_.
	- **ctrl <-> gnd + Ctrl**	-- Able to split ctrl into gnd & Gnds.
	- **gnd <-> form + Gnd**		-- ???
	- equiv 					--  g.fuse(x) == g.fuse(y)





	unit.base, unit.gnd, u=opmap.weld(base)

	Gnd.meta
	origin.path_get(f.head)[_.meta].new()

	ns 	=  (Ident=>NS) && Place
	g 	=  Gnd(ns) 
	u	=  g(base)



	Let env  = some execution environment
		spec = some Spec expression
		self = some 'self' unit for method invocation
	
		base = self.base
		gnd  = self.gnd
		ctrl = Gnds.control(self.gnd)
		ns	 = self.ns
		op 	 = lookup(f, ns)
		env2 = op.match(env, f)
	
		form = lang.form(expr)					// Loaded form
		ivkr = form.invoker(env.gnd, self.gnd)	// Compiled form


		r1 = exe(expr, self: self, env: env)
		r2 = op.invoke(env2, self)
	
		r3 = form.invoke(env, self)
		r4 = ivkr(env, self.base)




	u = unit
	g = u.GND
	b = u.base
	u = g.NEW(h)  g.bind(h)   
	// meta: head, base, 

	g = Gnd(path:, ns:, schema:, form:)


	// Unit formats:
	// 	FUSED:			Unit == UnitFused(base, ns)
	//  DECOMPOSED:		base, ns
	//  CONTROLLED:		base, 



	u = units.fuse(base, ns);	u.op1(*args)
	g = units.gnd(ns);  		g.op1(base, *args)
	u = g.bind(base);			b.op1(*args)
	base = u.base 
	gnd = u.gnd							
								exe(_.op1(*args), self: base, ns: ns)
								apply(lookup(_.op1, ns), *args, self: base, ns: ns)
#### --- CHN ---
	
	_
#### --- NEW ---
	_
## === SLOT ===
### _
#### _


**TL;DR.  Var is the grounding of Place -- it supports the implementation of type-organized assigment and persistence.**

**VAR** -- The _**VAR construct**_ implements the persistence requried for assignment semantics within Uniform. 


**VAR** is an **_essential framing_** of "persistance" and "assignment semantics"

The **_purpose_** of VAR is to provide flexibly-arranged structures with lexically-named, persistently-assignable slots.


_
#### --- **TERMS DEFINED BY THE VAR CONSTRUCT** ---

We consider how one might implement the following code snippet on a Von Neumann Machine (a computer) in order to help illuminate the VAR construct:

**def** _add_(_x_: Int, _y_: Float):
_z_: Float = _x_ + _y_
**return** _z_

_w_ = add(5, 10)


- _variable_ -- There are four variables (x, y, z, and w) with different names and types in the code shown here.  
- _var spec_ -- Varspec shown here is the format used to "specify" information about these vars.  (For example, "x: Int" is Varspec used to indicate a var named "x" with type Int.)
- _place_ -- Each call to the _add_ function will create a new stack frame on the stack having three "places" in the frame.  Each variable will have its "place" where its value could persist -- be assigned and later accessed.
- _backing_ -- This stack frame is called the "backing unit" for those three places since it is within the structure of this unit that each of these places _persist_ their assigned values.
- _place_schema_ and _backing_schema_ -- Each time the add function is called, the same stack frame with the same three places are created in the same way, with the same structure shape (order and types) on the stack.  The format of each place is called a "place schema" and the format of the backing is called the "backing schema".
- _place root_ -- According to the widely-used lexical semantics, the way the variables x, y, z, ended up being grouped together as part of the stack frame is because they are lexically contained within the add function definition in the source code,  while the w variable is outside of that scope.  This is formalized by the notion of "place roots."  These are verticies within the source code spec tree that collect variables under them into a single backing schema.  In the code above the def of add would be a backing root which collects x, y, and z.


**VARSPEC** -- A _VarSpec_ is the source-code specification for a Variable -- a Place Schema that may specify the name, type, definition, and options for the variable.

**SLOT - PLACE** -- A _place_ is the of implementor of assignment semantics -- it can be _access_-ed or _assign_-ed a specific type of value.

**PLACE** -- A _place_ is a unit that implements assignment semantics  (it provides persistence).  REMEMBER:  According to the MATH construct assignment semantics requires that one may repeated _assign_ a value, such that a subsequent _access_ will return a value equal to the most recently assigned value.

**BACKING** -- A _backing_ is a unit that provides persistence for a collection of places that are created, accessed, manipulated, and destroyed as a group.

**SLOT - PLACE SCHEMA** -- A _PlaceSchema_ is the schema used over and over to create a the places which persist a specific variable (VarSpec) within the code.

**BACKING SCHEMA** -- A _BackingSchema_ is a list of PlaceSchema -- it is the type for the backings of the places persisting those PlaceSchema variables.  Each backing schema defines the type for its associated backing units, since it specifies the order and types of all key value pairs within its structure.

**BACKING - PLACE ROOT** -- Certain vertices (units) in the source code are designated as a "place roots".  VarSpec that occur in the source code under these verticies are mapped onto PlaceSchema that are gathered into a single BackingSchema associated with this root place.  (The new operator accepts a place root and uses this to define the structure, and constraints on the structure of the unit being created.)


SLOT
n	name
t	type
e	expr
k	key

_
#### --- SEMANTIC VOCABULARY --- 

ABBREV:  [a]args  [n]ame  [t]ype  [e]xpr. [k]ey. 
			kind schema ns(from type) extent

**SLOT SEMANTICS**
- **NAME** -- A variable's _name_ is the Ident used to "path_get" that Var from its containing namespace.
	- Normally names are single-part Ident, but they may also be multi-part.
	- 'None' name means var is not referencable by namespace.
- **KEY** -- A variable's _key_ is the IdentPart used to access its place with its backing unit.  By default key will be the lowest ununsed natural number.
- **VAR KIND** -- Each var has a _kind_.  This is not the type of the var, but rather an indicator of where the varible is backed, and thus it defines the scope, extent, and behavior of the var.
- **SCHEMA** -- loading a Var within declarational will it to be appended to a relevant schema depending on the kind of variables it is (see below), the backing key associated with the var is either the varspec's 'b' key or its 'n' key if 'b' is not specified.  It is an error for two vars to have the same backing key within a single schema.
- **NS** -- within declarational contexts var forms will add a NS entry to the current lexical context based on the 'n' key of the varspec.
- **TYPE** -- the type of the var's place is specified by the 't' varspec key.
- **EXTENT** -- the dynamic extent for each var is defined by the extent of its matching schema.
- ACCESS -- within functional contexts var forms evaluate to an access of the relevant slot, or an assginment of that slot.
- SCHMA ROOTS -- Packages my declare themselves to be 

_
### --- API ---

accessor:
	self.field, self.gnd.field

out of date

lang.uc:
_**Slot**_:
	args: 	[T]						  // Slot parameters
	_**name**_:	slot(t: Ident, <Fixed)		// Namespace
	_**type**_:	slot(t: T)					// Field / Return type
	_**expr**_:	slot(t: Spec)
	_**stem**_:	slot(t: Class)				// Self type 
	_**key**_: 	slot(t: IdentKey)
	// DERIVED
	_**parent**_:	slot(t: Pkg)
	_**schema**_:	slot(t: Class)

_**Class**_:
	extends: Pkg
	_**backing**_: slot(t: )
	// DERIVED
	NEW:	slot(t: `this.form)
		



//	

**def class** _Slot_(T) **extends** Pathy(SlotSchema):
	**def var** _name_:		Ident 		getter	# Relative to schema grounding
	**def var** _type_:		T	 		getter
	**def var** _expr_: 	Expr 		getter	# ??definition
	**def var** _schema_:	SlotSchema 	getter	# (or use Pathy)
	**def var** _key_:		IdentKey	getter	# key to access (maybe 'path')
	**def var** _spec_: 	VarSpec 	getter	
	**def op** _access_(Env, ->T)
	**def op** _assign_(Env, T)
	**def op** _accessor_(Env.meta, base_accessor Form) -> Form
	**def op** _assigner_(Env.meta, base_accessor Form, value Form) -> Form

	**def op** _form_(Env.meta, call_form Spec) -> Invokable(T)
	

**def class** _SlotSchema_:
	**extends**: List(Slot)
	**def op** accessor(Env.meta) -> Form	# result will reduce to slot base
	
**def type** _VarSpec_ **extends** Spec:
	**def var** _n_ IdentName getter	// name
	**def var** _t_ TypeSpec getter		// type
	**def var** _e_ Spec getter			// expr
	**def var** _k_ IdentKey getter		// key
	**def var** ^opt getter

**def class** _Place_(_T_) **extends** Declaration:
	**def op** Place(backing Unit, v VarSpec):
		T := v.type
	**def var** _var_ Var getter
	**def op** _access_(->_T_)
	**def op** _assign_(_T_)



**def class** _SlotRef_(T) **extends** Unit:

slot_base
SlotGroup
SlotDefinition






**def class** _BackingSchema_:
	**extends**: Fixed List(PlaceSchema)
	**def op** _dereference_(Env, ->Backing)

**def class** _Backing_:
	**extends**: List(Unit)
	**def var** _gnd_ Gnd getter	  
	**def op** _place_(Ident _name_, ->_Place_)
	**def macro** _place_(Ident _name_, ->_Place_(T)):
		var := self.gnd.instance_schema[_name_]
		T := var.type
		return `Place($self, $var.offset)
		


_
### --- VAR TYPES ---

VAR TYPES
- Slot 	--  Slots within instances defined by the package structure
- Pkg 	--  Indicates that assignments to slot apply pkg semantics to value
- Op 	--  Methods tied to instances defined by pkg structure
- Fn 	--  Class scoped functions (w/o reference to self) 
- Static	--  Slot tied to class singleton not to instance units
- Closure--  All closure vars stored in 
- Thread	--  All thread vars stored in the single thread tied structs
- Global	--  All global vars stored in the single globals struct
- Const	--  Macro is same as Fixed Static -- Constant value tied to scope
_
### --- formally --- 

Formally a **_VarSpec_** is a declaration for that optionally specifies a: name, type, defining expression, and options.  The var spec uses the keys: n, t, d, and ^opt respectively to indicate these parameters.  All var spec have the @var option to indicate that they are a var spec.

Formally a **frame instance** is just a unit.  It must be functional collection since its structure must be indexible in order to access and assign its persisted vars.  Often frame instanced are mutible tuples with each position cooresponding to a different var's type.  One can think of the ordering of the tuple as the in-memory layout of this instance data.

Formally a **_FrameSchemaSpec_** is a functional collection of VarSpec (often just a list of VarSpec).  Each schema describes the structure of the frame instance units that are used to persist its variables.  (During interpretation, often there will be many frame instances which share a common schema.)

Formally a _**frame root**_ is any code spec with the @frame_root option.

The default unicore load behavior is to perform a pre-order traversal of the code spec, creating a new frame schema for each frame root found in the loaded spec, and to append each VarSpec found to the most recently created frame schema that is an ancestor of that var.

The framing linkage declared here is used to track this connection:
Linkage(type1: FrameSchemaSpec, key1: contents,
		type2: VarSpec, key2: frame, order: 'one2many)

Each FrameSchemaSpec has a _**dereference**_ operator that accepts an Env instance and returns the cooresponding frame instance.  Thus accessing a var can be done by accessing its backing, then its place, then its value, as shown here
	
v^frame.dereference(E)[v^frame_key].access

_v_ 				=  some var
_E_ 				=  the current execution environment
_frame_instance_ 	=  _v_^frame.dereference(_E_)
_var_place_ 		=  _frame_instance_[ _v_^frame_key ]
_current_value_		=  _var_place_.access

_
#### - discussion -

Many langauges implement things like functional closures, multiple inheritance, inner classes, etc, by creating frame schemas "under the covers" to house vars that are persisted differently depending upon the specific language semantics for those vars.  

Unicore is quite general in it approach here, as long as there is defined procedure for getting to these schema frames and a procedure for getting to the elements of each frame, then it can be encoded within this paradigm.

Often the " v^frame.dereference(E) " operator actually expand into a sequence of frame dereference lookups.  This all fits nicely into the Unicore framework.  the only commitment being made here is that whatever is providing the persistence can be expressed as a unit.  (And of course the entirely of the Unicore's structure layer is dedicated to allowing for very very general and very performant implementation of unit structure.  

It seems to the author that the backing structures of all existing software languages can naturally be expressed using this VAR-FRAME paradimg.  (If anyone knows of counter examples for this conjecture, please let DanO know!))

_
##### choice - "backing"
ISSUE: the word 'backing' is superfluous, the word persistence is already defined.

CHOICE:  Use both the back and persist terms.

Reasoning:  What is happening here is pretty simple, and easily understood by anyone who understands how computer memory is used to persist data.  Still talking about an action "persist" and a noun "backing" using the same word is a bit confusing.  we can use the term back when we want to consider the data artifact providing persistance, and the terms persist when we want to emphasis the behavior provided.  Using this approach requires the introduction of the second term. 
### --- IMPL ---

**def class** _BackingInstance_:
	**def var** place_backing_schema Schema getter
	**def var** _gnd_ Gnd getter	  
	**def op** _place_(Ident _name_, ->_Place_)
	**def macro** _place_(Ident _name_, ->_Place_(T)):
		var := self.gnd.instance_schema[_name_]
		T := var.type
		return `Place($self, $var.offset)

**def class** _Place_(_T_) **extends** Declaration:
	**def op** Place(Unit backing, Var v):
		T := v.type
		self.backing = backing
		self.offset = v.offset
	**def var** _var_ Var getter
	**def macro** _access_(->_T_):
		return `GET($self.backing, $self.offset)
	**def op** _assign_(_T_)

_
### --- SEMANTICS ---

- PREORDER -- Perform a preorder traversal of spec tree
- BACKING ROOT -- Keep track of most recent backing root seen
- VAR -- Add each var to the backing schema for the most recent backing root
- FLESH -- Fleshout name of var


spec 

_
### --- VAR Types and Options ---

VAR(n:I.x, 		t:Int, 	b:StackVar)
VAR(n:I.a_name, t:Str,	b:InstanceVar)
VAR(n:I.clsvar,	t:Int, 	b:TypeInfo)  # Class var


StackVar: 		BackingSchema(path: I)
InstanceVar:	BackingSchema(path: I.self)
ClassVar: 		BackingSchema(path: I.self^TYP)
### -- Generality --

**TLDR; The VAR construct is a best implementation of persistence across all contexts.**

At first glance this collection of classes seems dangerously specific.  The goal of unicore is to make no commitment that is at odds with any software framework.  How can all of this detail safely avoid that peril?

It is because this detail follows from a couple of organizing principles built deeply into the Von Neumann architecture itself -- an architecture that underlies all of our computing devices today, and by extension underlies all computer languages of today.

Specifically:
- **MEMORY WITH FIXED DENSE LINEAR STRUCTURE** -- Von Neumann architectures provide persistence in the form of computer "memory": 
- a sequence of cells indexed by natural numbers, where
- each cell may contain a natural number from 0 upto some max cell contents value.
- the sequnce is fixed (one cannot insert, remove, rearrange memory cells)
	(Note: The implementations of virtual memory on modern processors do blur this clarity a bit, and indeed some VM "tricks" fall outside the generality of the model shown here.)
- Notice this basic linear address architecture holds both for primary "RAM" memory as well as secondary "Disk" memory.
**FIXED OFFSET ADDRESSING** -- Fixed offset addressing is an efficient building block to base other more complex structuring of data.
- Modern memory caching architectures work to ensure that dense access patterns spanning narrow memory address ranges are handled efficiently by cacheline block loading, prefectching, etc.
- This means fixed offsets that are stored as part near or part of an assembly instruction performing the memory access is general faster that calculations requiring data that is not close to the current program counter.
- This also means these architectures are optomized for storing values that are often accessed together in narrow address ranges as well, so that the memory caching mechansims have greater chance of providing speed.
- **UNMARKED, HETRO-INTERPRETABLE, DATATYPES** -- Most modern processors provide a range of built in data types: memory pointers, signed integers, floating point, etc.  Generally the data for these types is unmarked, that is, there is simple a range of bytes, and it is up to the program to ensure that range is processed correctly. 

These three properties of VonNeumann machines as implemented by modern computer architectures put a premium on the usage of: (1) schema-based persistence, implemented as (2) dense, (3) fixed offset, (4) limited-type elements.  We explain these contraints in turn:
- **Schema-based** -- implementing persistence using structures with repeated form means that fixed simple instruction sequences can be used to operate on in parallel ways on all instances of those memory schema.  (e.g. using the same schema for all calls to a specific function allows a single simple sequence of instructions to process those stack frames in the "same" way -- whereas a varying frame structure would increase the complexity of the instructions needed to process that frame.)
- **dense** -- implementing persistence using dense memory structures is generally preferred to sparse structures, as they can be organized more easily within memory, and have better cache performance since they generally fit within fewer cache lines.
- **fixed-offset** -- building persistence structures that use fixed offsets at their  lowest level of organization is usually preferred, as this lowest level organization is most frequently access and should be most efficient when possible.
- **limited-types** -- having single typed data at these specific offest is generally preferred, since no checking is needed when performing operations on those addresses.  Even if mono-typing cannot be achieve, the greater the limitations on possible types for a given persistent element, the faster its processing will be.

The first argument that the VAR construct is a best formulation persistence comes from the universality of these four properties.  The preference for these four properties stems directly from the Von Neumann architecture, which in turn underlies most all programming languages.  The VAR APIs are intended to be a simplest formulation of these four properties.


The second argument that the VAR construct is a best formulation of persistence is more of a practical argument.  Below we consider a great many memory structures used "under the covers" across a great range of language implementations.  We investigate if one were forced to build all of these structures within the confines of the VAR construt if they would work, if they would be parsimonious, if they would be as performant, etc.  This is quite a high bar, we hope VAR is a non-forking construct.  We hope no language is forced to choose between living within the confines of the VAR construct, or obtaining some desirable language property that lies outside what is possible using this contract.

The early evidence is reassuring.  It seems that even quite complex structures like those used: in performant, multi-parent inheritance hiearchies, in multi-level functional closures, in multi-threaded shallow binding, in "inner class" implementations with partially overlapping state, in progressively reified persistence structures, in dynamic unbounded structures all fit nicely as instantiations of the VAR persistence model.

CASE FOR CONSIDERATION

STACK FRAME

HEAP OBJECT

STACK ALLOCATED DATA

CLOSURE DATA

HETROGENOUSLY BACKED COLLECTIONS 

MULTI-LEVEL CLOSURES

INNER CLASSES ET AL.

MULTIPLE INHERITANCE



GLOBAL VARIABLES

MODULES VARIABLE

SHALOW BOUND VARIABLES

THREAD VARIABLES


INFINI-STRUCTURES

INFINITE DATA STRUCTURES

PROGRESSIVELY REIFIED STRUCTURES  (a-la haskell matchers)

IMMUTIBLE STRUCTURES  (a-la closure)


DATA TRANSMISSION PROTOCOLS
-- some of these do not fit well into the var paradigm (of course one can also argue those protocols are not aiming to implement the random access and assignment that is central to the notion of persistence)

but many protocols do still fit (e.g. protobuf, ROS messages)





**THE VAR THESIS**
Requiring VAR construct usage for all implementation of persistence does no harm

Any persistence structure P built on a Von Neumann machine, can be expressed as a comperable persistence structure P' expressed using the VAR construct such that no acknowledge measure of quality is significantly diminished.
### Examples

$x:Int=5

@path=env.self.type, $print:fn()->Str
def Virtual
### -- Schema --

**SCHEMA** -- A _schema_ is a fixy list of VAR places.  Each schema is a template indicating the relative position of VAR places within some backing structure.


NOTES:
- STATIC -- Schemas

Some vars within the schema map directly to 



#### - Example pkgs and schemas -


NOTES:
- Each pkg has its place (type ident)
- Each vars is in a pkg, and these get placed into pkg NS
- Each var also has schema it is part of
- Each schema as access_paths describing how to ptr to instance w. var
- Each Env NS has a schema whose access_path is simply "E" (current env)
- EXE of an ident does Env.ns lookup ->
	then backtracking search to find schema path from var to Env NS schema
		follow path from E to var's place



// FN stack frame pkg
pkg.schema -> StackFrameSchema

StackFrameSchema
$ SF_Schema		:SchemaType
$ Self			:SelfType
$ Arg1			:Int
$ Arg2			:Str
$ KeyArg		:List(Int)
$ RestArgs		: =>Int
$ ClosureVars	: ClosureSchemaType
$ ThreadVars	: ThreadSchemaType

StackFrameSchema_TypeSchema
0:	TypeSchema_Type
1:	ClassMethodBinding1
--	ClassMethodBinding2
	...
--	InstanceMethodBinding1
--	InstanceMethodBinding2
	...
--	TypeCase1

ThreadSchemaType
### -- Note about placeholder vars --
Vars that implement assignent semantics require some form of backing to persist that data.  But some vars are only used as a place holder that are for example reference by name, but do not provide full assignment semantics.  (A mustache-style html template has variables that can be filled in, but they are not persisted.)  This is an allowable usage of the VAR construct, it is just the case that there is no backing instances created for these vars.
### -- Discussion --

{[this is the right idea, but is super hacky]}

- ANCHOR -- Vars are always occur within a container that defines an _anchor expression_.  This expression derives a run-time pointer the container.
- REF PATH -- Vars also have a ref path which derives a pointer to the var's "home"
- OFFSET -- Each Var will have a unique offset id within its home.

kinds of VARS
- SLOT VARS - instance slots are offset from the self object which is pointed from the env
- LOCAL VARS - are instance slots directly from the env
- CLOSURE VARS - are instance slots off containing env for this env
- TYPE VARS - are instance slots off the type of the self of the env
### junk
's place can be done by 

 environment 


 that persists 


~-~~
A 'variable' is a possibly named, typed place whose 'assigned' value may _vary_ over time according to assignment semantics.
~-~~
~-~~
Highlight -- A hook is a mechanism for registering actions to be executed when some underlying action has occurred.
~-~-~

VAR
n:Str		name -- 
t:Type		type -- 
d:Expr		definition -- 
s:Schema	schema -- indicates the containing schema 
^opt		options -- 
## === FLOW ===
### _

**FLOW** -- A _Control Flow (FLOW)_ operator specifies the temporal ordering of the execution of its elements.

The FLOW spiral-thesis is that all control flow is naturally expressed as a combination of these five control operators:
- _**sequencing**_ — the execution sub-elements of a form in temporal order based on the form's structural order.
- _**branching**_ — the selecting among a finite number of different sub-parts of the code to execute next.
- _**composing**_ — the execution of a sequence of forms where self is bound to result of each prior execution.
- _**repeating**_ — the repeated execution of some fixed forms.
- _**terminating**_ — the stopping the execution of a form before its completion and the non-local transfer of control to the point in the execution that would have followed the completed execution of that form.

_
### --- BLK ---

BLK(form0, form1, ...) ⟶ value   [sequencing]
Executes the (BL)oc(K) of forms, form0, ... Each subunit is executed in sequential order. Returns the value returned by the last form.

_
#### -- TRS RULES --

exe(blk($x, $y...), $b) ==> exe(blk($y...), exe($x, $b))
exe(blk($x), $b, $s) ==> exe($x, $b)
exe(blk(), $b, $s) ==> exe(None, $b)

exe(Q.$head($...body), env(self: $s)) ==> exe
	rule := s.gnd.ns[head]

_
### --- BRA ---

BRA( [expr, ] IF(condi, formi,0, formi,1, ...), ... 
	[, "IF"(true, formtrue,0, ...)] ) ⟶ value   [branching]

Executes each case condi until one is satisfied. Then executes the forms in this matching case. Returns the the value returned by the last form in this matching case. Returns und if no case matches. Optionally accepts expr as a first argument; this is assigned as the self unit during execution of each condi, but not during the execution of the forms of the matching case.

SPECIAL CASES: The last IF might have true as a condition, in that case its forms are the catch all forms which are executed when no other case executed. The form BRA(type, IF(...), ...) is called a typecase since it branches based on the type of self; some implementations handle these efficiently. By ironic analogy this means the BRA(head, IF(...), ...) form is called a headcase since it branches based on the head; these may also be handled efficiently by some implementations.


BRA([base], c1->a1, c2->a2, ...) ==
CTX($_b=$(base||`self)) {
SAT(EXE(c1, self:$_b, a1, SAT(EXE(c2,self:$ _b), a2, ...))
_
### --- CHN ---
#### -- Semantics --

**FORM CONVERSION** -- form conversion steps:
- Convert arg forms, then obtain the return value type for each
- Use return type to lookup grounding whose ns is used to lookup slot
- Use slot to get the accessor for the operator invocation

return_type = form1.returns
slot = lang.lookup(return_type.ns, form1)
slot.accessor()

_
#### -- API --

**def class** CHN 
	**extends**: Form
	**def op** _reduce_(Env)
_
#### -- Original Intro --

**CHN** -- The _chain_ operator composes a seqeuence of interpretable forms by indexing the interpretation of each form by the result obtained from the interpretation of the prior form.

**CHN**(form1, ...) ⟶ value   [composing]

Chains the execution of each formi by setting self to be the result of the execution of the prior formi, finally returning the result of the last execution, and leaving self as it was prior to the chaining operator.

_
#### -- Semantics --

CHN(_form_1_, _form_2_, ..., _form_n_) ==
	EXE(_form_n_, self: EXE(_form_n-1_, self: ... self: EXE(_form1_)))

**TL;DR.**  Chaining dereference the NS until a ground element is found, then the result of executing that grounding become the new 'self' unit for the remaining chaining.  Returns the final 'self' unit as its result.


**def fn** _dot_reduce_(_dot_form_ Exec, _env_ Env):
	_u_ Unit = _env_.self
	**for** _form_ in _dot_form_.values():
		_u_ = _env_.exe(_form_, self:_u_)
	**return** _s_

**def op** Env._exe_(_env_ Env, _expr_ Exec, _unit_: Unit, _ns_: Ns):
	_grounding_ = _unit_.gnd.ns[_expr_.head]
	_stack_frame_ = _env_.new(_grounding_, unit: _unit_)
	_grounding_.match(_form_, _env_, into: _stack_frame_)
	**return** _stack_frame_.exe(_meaning_)

**def op** Env._exe_(_env_ Env, _expr_ Spec, _unit_: Unit, _ns_: Ns):
	_grounding_ = _unit_.gnd.ns[_expr_.head]
	_stack_frame_ = _grounding_.exec(_expr_, _env_)
	**return** _stack_frame_.exe(_meaning_)


// Direct impl
**def op** _dot_reduce_(_env_ Env, forms Spec...):
	_s_ Unit = _env_.self
	**for** _form_ in _forms_:
		_meaning_ = _s_.gnd.ns[_form_.head]
		_stack_frame_ = env.new(_meaning_, self: _s_)
		_meaning_.match(_form_, _env_, into: _stack_frame_)
		_s_ = stack_frame.exe(_meaning_)
	**return** _s_

**def class** _Dot_:
	extends: Exec
	**def op** _exe_(_self_ Dot, _env_ Env):
		_s_ Unit = _env_.self
		**for** _form_ in _self_.values():
			_meaning_ = _s_.gnd.ns[_form_.head]
			_stack_frame_ = _env_.new(_meaning_, self: _s_)
			_meaning_.match(_form_, _env_, into: _stack_frame_)
			_s_ = _stack_frame_.exe(_meaning_)
	**return** _s_

_
#### -- Dot Semantic Contract --

imagine:  DOT(... foo(...)  )

env = env
code = foo(...)
meaning = self.ns[I.foo]

stack_frame = Env() = Env.seed.new(Env, self: self)
meaning.match(code, env, into: stack_frame)
result = stack_frame.run(meaning)

_
#### -- OLD CALL --

**CALL** -- The _call_ operator creates a sub-environment (Env) and recursively performs an interpretation of some body form in order to obtain its interpretation result.

Call provides the basis for object-orientation by optionally indexing both the available environment (local data slots) and the semantic namespace (local methods) based on a specially designated 'self' unit.

INVOKE -- The invoke operator


**def class** Form:
	**def op** _call_(self Form, self_unit Unit, env:, ns:) 

_
### --- REP ---

REP(condition, form1, ...) ⟶ value   [iterating]
Repeatedly executes condition, if not satisfied the operator terminates returning null. Otherwise each formi is executed in order and the entire process repeats indefinitely until the condition is not satisfied.

_
#### discussion
REP(itr, body1, ...) ==
SAT(itr, REP(itr, body1, ...))


EXAMPLE:
for k,v in dooby.items(): print(k+v)
CTX(k,v,v(itr)=dooby.items().step(k, v), BLK:[ 
REP(v(itr), print("+"(k,v))) ])


WHY THIS EXPANSION?
- index vars must be local vars since they may be accessed multiple times in body.
- CTX should be the only method for creating local vars.
- Iterating list with one index should interate values.
- 
_
### --- RET ---

RET(try_tag, return_value)   [terminating]
Terminates interpretation of some active operator. The try_tag specifies which active operator is to be terminated. A negative integer indicates the number of active REP operators to terminate (within the current interpreted form). A null tag indicates termination of the most recent active functional (DEF or GND) form. Finally a SYM value indicates termination of the most recent TRY with a tag that matches the try_tag. In any case, interpretation continues after the terminated operator with return_value used as its result. If no matching TRY form is found then the UnmatchedRetTag error is thrown.

_
### --- PURITY VS MINIMALITY ---

From a purity and decomposibility perspective it is ideal if all control flow is expressed as a combination of the five operators above, and if each operator is "pure" in the sense that it does not redundantly express some part of the semantics of the other four operators.  This is achievable and it is called the "_pure_" control form, but achieving this requires us to replace single operators as defined above with two or more "pure" operators.  In this sense the result is not minimal.  Thus the alternate form is called "_minimal_" since it minimizes the number of operators used.  Happily one can convert losslessly between the pure and minimal forms of control.  Becuase of this, we don't make a big deal about which form is being used.  Often we use the minimal form in text since it is more compact, but we always remember it really is the same as the equivelant pure form.

**FIRST CONVERSATION -- BLK EXTRACTION**

We must introduce extra BLK operators in to CLT, REP, and BRA forms in order to remove the sequencing implied in the defintions above.  These transformations are trivial, they are usually applied only when BODY has two or more elements:

CTL($BODY...) ==
	CTL(BLK($BODY...))

BRA($PREFIX... IF($COND, $BODY...), $SUFFIX) ==
	BRA($PREFIX... IF($COND, BLK($BODY...)), $SUFFIX)

REP($COND, $BODY...) ==
	REP($COND, $BODY...) BRA($PREFIX... IF($COND, BLK($BODY...)), $SUFFIX)

Adding these extra BLK operators ensures that BLK is the only sequencing that occurs in the pure control flow form.


**SECOND CONVERSATION -- BRA EXTRACTON**

Likewise we can remove the implicit branch from the REP instruction.  The pure form of REP has has one argument and it repeatedly executes that operator forever.  One can convert this pure form back to the more complex "while" loop behavior as:
REP($_CONDITION_, $_BODY_...) ==
	CTX(REP(BRA(IF($_CONDITION_, BLK($_BODY_...), IF(True, RET($_TAG_)))), $_TAG_->None)
If a REP has two or more args it is converted into a tagged block with a pure REP operator that executes a branch that either continues repeating or breaks out of the REP to the tagged CTX block.



**THIRD CONVERSION -- CHN EXTRACTION**

Finally each bare operator invocations can be rewritten as a chaining operator with only a single chaining.  In this way all operator invocations are performed via CHN:

$HEAD(...$ARGS) ==
	CHN($HEAD(...$ARGS))
## === PKG ===
### _

The **_purpose_** of PKG is to provide a natural basis for source-code grouping mechanisms.

**PKG** -- A _package (PKG)_ is a named collection of source-code forms that are derived directly from the source code of the package itself or from other packages explicitly, recursively referenced by this package.


_
### --- SEMANTICS ---

recursive pkg inside vars

**PKG SEMANTICS**
	- **PATH** -- All pkg (including the root pkg) must have a pkg path.
		- Either the first argument to a pkg form or its pkg stmt provide the path for a pkg.  It is an error to supply both.  
		- If no pkg path is provided, then the pkg must be the value of key for some parent pkg and its path is derived from its parent by appending that key.
		- May not be changed, rather a pkg's path indicates it location.
	- **KEYS** -- Each unique pkg key may be defined at most once via:
		1. A 'def' stmt for may define a value for a named key.
		2. The key of a pkg spec may be explicitly defined.
		3. A key may be included by defining a child pkg elsewhere in the code.
		4. MUTABLE??? 
	- **EXTENDS** -- 
	- **NS** -- Each pkg defines a namespace by combining its keys with keys from 	other pkgs.  By default it is the chaining of:
		1. Its own keys 
		2. Recursively the immediate keys of each lexical ancestor. 
		3. The keys from each pkg extended by this pkg.
		4. IMMUTIBLE

_
#### older

**PKG** -- A _package_ (pkg) is a collection of source form derived by importing and extending other packages.

**NS** -- A _name space_ (NS) is a tree of semantic bindings.

**GND** -- A _grounding_ is a collection semantic bindings -- information that drives the interpretation of code Spec forms.

**PKG** -- A _package_ (pkg) is a named collection of bindings of identifiers onto the semantic "grounding" form they denote.
- **DECLARISH** -- Packages are declarish meaning the derivation of their contents is specified precisely with a functionally defined semantics, and informally with a semantic that can be in the head of the viewer with ease.
- **TREE** -- Packages are arranged into a tree indexed by IdentSymbol, thus providing unique path and Ident for each package.
- **GND** -- A Grounding form is the umbrella term used to for things used to provide meaning for the interpretational process.  These include type info for describing the structure of an object, or reducers that execute the action associated with an iterpretational step., etc.

_
##### older

	
**def type** _NS_(T: Exec):
	**extends**: Fixy Tree(Ident, T)

 -- a declarish mapping from identifiers onto their semantics grounding
, which declarishly provides the semantic bindings underlying some interpretational process.

Often packages contain identifiers as names for the semantic forms.

Often packages are themselves contained within larger packages, thus providing global naming for the packages, as well as the largest scale super structure for the source code of a software language.

Packages are declarish this means their effects are formally defined functionally, but are secondarily partially understood via a static, cognatively transparent derivation.  

The unicore pkg operator comes with several cognatively transparent derivational operators (import, extends, and export) used to derive the contents of the package based on its immediate contents as well as the exported contents of the other referenced packages.  Cognitive transparency simply means that a programmer can easily infer package content by static inspection of its contents and its derivation operators.  (See "declarish" for details)



**PKG** -- A _package_ (pkg) is a source code form that declaratively provides semantic bindings for a collection of identifers.


**PKG** -- A _package_ (pkg) is a declarish, mapping tree of semantic elements.

_
### --- Package Terms ---

The structure of a Pkg (its contents) is derived by load operator.  The Unicore Pkg load operator combines the Pkg base with the Pkg imports to compute its contents  (See definition below).  

Unicore packages are _@memo_ and _@live_; their content derivation is dynamically computed to reflect changes that occur within imported packages. 

**PKG CONTENTS** -- The structure of the executable Pkg unit.
**PKG BASE** -- 
**PKG IMPORTS** --
**PKG EXPORT** -- 
**PKG LOAD** -- 

-- A Package 
-- CONTENTS (children stripped of stmts) form the base container
-- IMPORTS form a patch to base
-- EXTENDS are each added using the "+" operator on the 'exports' of the reference packages
x.add(y, @force(chk_for_aggregation))
-- EXPORT by default is the same as the contents of a package, but an explicit export exclusion statement removes items, while an explicit export inclusion statement defines the specific subset exported by enumeration.

-- @aggregation or @agg=true or @agg=auto     false, true, auto, recursive
Uses aggregational semantics to derive specified sub-pkgs 
_
#### - rule

add_args := [,? self.path_up, self ,++ self.form_stmts.select(`extends)]
content_expr := "+"(,++ add_args.map(`export), @live)
contents := EXE(content_expr)
export := 

_
### --- API ---

// NOTE: The Unicore's Lang only accepts constructs that are also Pkg

import uc.lang, uc.lex

**def class** _Declaration_: 
	**extends**: Form				  	  // Might be constructor, headcase, etc.
	**extends**: Pathy, Tree(LexKey)  	  // Maps onto lexspace  (just Decl?)
	

// subclass of Construct (can add to lang)  , NS(well at least lex)
**def class** _Pkg_: 
	**extends**: Declaration		  	  // Might be constructor, headcase, etc.
	**def op** _ns_ NS						// Mapping defined by this pkg
	
	**def op** _"+"_(other Pkg, ->Pkg)		// Combines with other
	**def fn** _load_(env, spec, ->Pkg)		// Loads spec relative to self
	**def op** _exports_ Fixed Ident=>T 	// The mapping used
	**?def op** __base_ Fixed Ident=>T		// 
	**?def op** __extends_ Fixed DAG(Pkg)	// Non-circular list of parents


**def type** _Pkg.Spec_(PkgStmtSpec..., ...:Spec, extends: Spec, head: isa(`PKG))


**def type** _Pkg.Spec.Stmt_ := `||:
	U._pkg_(Ident)
	U._import_(Ident, as: IdentPart)
	U._extends_(Ident...)
	U._run_action_(Spec)
	I._"---"_							// Separates header stmts from contents	

	U._"+"_(Spec)						// Includes an element if not in package
	U._"-"_(Matcher)					// Removes any elements that match spec
	U._"<"_(Spec, Matcher)				// Adds an ele before another  (CONFLICT!)
	U._">"_(Spec, Matcher)				// Adds an element after another
	U._":"_(Ident, Spec)				// Adds a key/value pair to package
	I._"..."_							// Indicates contatenation not append



Format:
pkg [path] {stmts..., sub1: val1, ...}
PKG( [path,] stmts..., sub1: val1, ...)

#### older

**def type** _NS_(T) := Listy Map(Ident, T)

_
### --- SEMANTICS ---
#### _

	**def op** ^_extends_(->DAG(Pkg))	// Contents derived from all
				:= self^stmts.select(head=='extends) 
	**def var** ^_contents_ Chimera(Ident, $T) 
				:= eval( `+($Chimera(self), $_extends_...) )
**def op** Pkg.loader

_
#### -- Semantics Discussion --

- PKG as CODE TEMPLATE 
	- PKG template -- pkg whose vars constrain 'extending' packages. (any pkg) 
	- TYPE template -- pkg whose vars constrain instantiating new units (@Gnd)
	- INTERFACE template -- pkg whose vars constrain 'op' in gnd of new units.
	- CLASS template -- type, interface pkg

- FUNCTIONAL -- The Pkg/Var/Gnd objects are immutable and defined via spec.

- PKG -- parts: ^path, 

**CASES**
- **EXTENDS** -- Adds arguments to DAG.parents
- PKG -- Specifies path and aggregation semantics
- DEF -- 

// pseudo code for 'load'
for spec in source.flatten():
	switch head:
		case I.


_
#### -- PKG.form semantics --

lang.root.path_get()
#### -- Agg semantics --
|  value 					// appends value
|  "+" value	  			// appends value if it is not already in pkg contents
|  "-" value 				// removes entry by value
|  "=" pos 					// adds a special value used as scaffolding but not included in final pkg contents
|  pos ">" value 			// adds value immediately after pos in collection
|  pos "<" value            // adds value immediately before pos in collection



|  key "=" value			// appends key/value, or overwrite existing k/v
|  key "-" 					// ??? removes entry by key

								TODO: allow adding k/v pairs too????
_
#### -- Discussion --

- LISTY -- somehow we know if pkg is listy
- AGG -- means pkg values do not overwrite, but instead they aggregate
#### Semantic Examples
pkg foo::
@aggregation=auto
x: pkg

< foo bar
> foo bar


pkg example(extends ex1 ex2, @aggregation=auto)::
foo: pkg(){a=4, b=222, d=555, e=777}

pkg ex1::
foo: [c=333, -777, b<555, -b, 555>last_one]

example = example(a=4, 555=555, last_one=last_one, d=555, e=777)




_
#### API
provide full.logical.path(date=2015-04-12.06.20, version=v.9.2)
require full.logical.path(date>2018.12, )
when full.logical.path {...}
### --- EXAMPLES ---

pkg foo.bar::
	extends: foo.other
	import:  subtree as: st
	stuff: +another_one, +this_too
	parts: + ... $computes_more() 		// computes more and includes them
	parts2:  ... $computes_more()		// concatenates and adds duplicates
	
	_
### --- DISCUSSION ---
#### -- Properties --
#### COHESIVE
 -- a package is referential (cohesive) if it is 100% specified via a single URI (note it might have explicit parameters, but otherwise is fully specified)
Notice this means that even the language, langauge category, langauge version, future referentiabilty, etc. are ALL specified.
In practice this means the language indicator must be flexible enough to exist in the context of other language categories langauge indicators, AND must provide enough info that forever into the future it is possible to reliably differentiate the expectations of this version's loader.

Uniform uses -* - ... - *-    (should be different since this is already used???)
#### CONTAINED
 -- a package is contained if it may exists within some ???context??? along with other versions of itself and other contained packages w/o conflict.  (conflict is unintended / unwanted interaction between packages where an interaction is any change in behavior of the packages)
#### TRANSPORTABLE
 -- maintains its intended behavior across all valid installation contexts
#### INSTALL/UNINSTALL/USE -- must provide these behaviors
#### -- Implementation Notes --

def type Slot = Int || Sym
#### Syntax
Pkg x = [1, two:2, 3]

y := [extends x, two<two.five :]
## === CTX ===
### _

**CTX** -- A _context (Ctx)_ defines an "inside" interpretation Env as a function of some given "outside" Env.  The namespace (NS) of this inner Env is some static derivation based on the static NS of the outer Env.  The place assignments of the outer Env are dynamically updated to create and destroy the inner Env upon the entry and exit of this context.

old **CTX** -- A _context_ (Ctx) defines an "inner context" value as a computable function of some "outer context" value.

_
### --- SEMANTICS ---

**CTX SEMANTICS**

- **DEFINGING ENVS** -- Every Env has a structure that backs the vars defined by the namespace of some Ctx form.
- **SHADOWING** -- The namespace (NS) of a Ctx is statically derived by _shadowing_ the variables of its outer Env with all Vars declared within this Ctx. 
	All Ctx variables will be added to the NS of the Ctx.  Any variables from the outer Env whose name is replaced it 
- Shadowing means adding variables with novel names are to the inner Env, and overwriting mappings to varaiable this same names.  (All of this is done non-destructive, w/o affecting the outer namespace.)
- **SHALLOW BINDING** -- 


Key Properties:
- Since contexts are recurively defined in terms of other contexts they naturally form a tree of ever-more-inner subcontexts.
- A context maybe be invoked _statically_ to derive a static inner value from some static outer value.
- A context maybe invoked _dynamically_ during an interval of computation in which case it demarks a sub-interval of computation where the inner context value shadows the outer context value.
- The form's expressions are executed in the outer defining context, while their results define a patch which is applied to derive the inner context.
_
### --- API ---


**def type** _Patch_ := Ident=>Unit

**def fn** _CTX_(outer_ctx: Ctx, patch: Patch) -> Ctx
	
**def type** _PatchSpec_(KEY: Ident, VALUE: Spec) :=
	U.patch( U."->"($K, $V)... )
	

_
### FORMAT
CTX (prefix...) body ...
CTX (VAR1, ...) body ...
VAR(n: t: v: @fn|@mut) 	# def a var as functional|mutable 	????
IN(n:_PATH_, v:_VALUE_)	
OUT(n:_PATH_, v:_VALUE_)
		

PREFIX FORMS
local_var = expr	# Simple contextualized binding.  Interpreted at creation time
					  (visible within scope; only active within scope)
local_var := expr   # defines a constant value within the local context
<- form 			# initializer (interpreted on 'enter')			  
global_var <- form 	# initializer whose result is assigned to var in the outter context    -> form 	  		   # finally form (unconditionally executed upon exit)
global_foo -> form  # finally form that is assigned to outer_var on exit.
extends ctx, ...    # ??? context defined as extension of another

label ???

#### Normalized Form
CTX()
- PATH        -- 
- STRUCTURE   -- [ VAR(), ...]  # name, initializer, type       ( =, := )
- FNS_PATCH   -- [ VAR(), ...]  # path, value, type             ( <- )
- CONNECTIONS -- [ VAR(), ...]  # relative_addr, accessor_form  ( == )
### OPERATORS
#### . fix

Let "==" be an equivelance relation for some languge L
Let "length" be a complexity measure for elements in L that map them onto some real valued number.

Define: canon(L) 
canon(L) is the subset l in L where there does not exist l2 in L where length(l2)<length(l) and l2==l

Define: A embeds into B
Given complexity and equivalance relations for both A and B we say _A embeds into B_ iff 

there exists f:A->B such that 

canon(A) maps to canon(B)

~-~-~ 
Options:
- inherit_closure_vars false, direct, true
#### examples

with globals:
with special_vars:
	Class Outer:
		class Inner:
			class APartOf:
				def op op1():
					with region1:


globals			env w. GL1 pointing to globals struct
special_vars	env w. GL1 pointing to shallow binding copy
Outer			env w. self pointing to Outer struct (plus parent connector)
Inner			env w. self 
#### Semantics


- PARENT -- 
- Each CTX (except the root) has a parent context that is used to perform the instance creation for this Ctx.
- ENV -- 
- Each Ctx defines a sub interpretation environment (context) based on its parent.
- PATCH -- 
- Each Ctx may explicitly update the Fns tree by supplying patches to that tree.
- Each patch is an absolute path in the Fns tree, and a code form evaluated to obtain new value.
- STRUCTURE --
- A Ctx may define a structure -- a list of variables available within instances of this Ctx
- CONNECTIONS -- 
- Each connection is an expression that can be interpreted with any instance of this context in order to obtain the relative structure instance defined within some parent or child context.
- Connections are used to provide access to variables defined relative to those contexts within this context.  (so it would de-reference multi-level closure variables, inner class variables, etc.)


OPERATIONS
- MAKE
- subenv = env.NEW( fns=ctx.fns )

#### SUMMARY/API 
    -_- 2019.lang.uf1 -_-
Patch == @immutable 
Gnd == h('GND) && @immutable
Lib == Patch(Path, Gnd)) && @immutable
Fns == Tree(Key, Gnd) && @immutable

form.Env:
EXE			= op(forms Code..., alpha Sym, -> Unit)						# Interprets forms in Env

form.Pkg:

form.Var:


#### structure
- @defines(+unicore.structure)
- code.structure maps  returns a list of possibly dotted Vars:
VAR(n:this.that, t:Int, v:a_value)
- List is built from with the recursive structure of the CTX form, excluding any sub CTX forms.
- Initializer forms are not evaluated.
- 
#### patch
- defines patch of overwrites explicitly defined (using the '<-' syntax) at this unit. 
- 
#### function
- @defines(+unicore.function)
- code.function(Code,->Lib) 
- Maps code onto a Patch of Form
- uses Structure and structure of parent
- uses explicit '<-' assignments
- Semantics
- defines if/how immediate structure is accessed
- Connectors
- A list of reference chains that provide access to parent/child contexts from this context
#### var.__init__ .assign 
- 
- Execute the init code associated with a var


#### as a transform
- converts self=Env into child env with context applied
- converts self=Space onto child space with context applied
- tree2 = tree1 >> ctx       tree2 = EXE(ctx, self: tree1)  
#### enter
ctx.enter(space)
env_parent .enter(ctx) --> env_of_ctx
- call's NEW passing the struture of ctx
- performs '=' and '<-' operations in arbitrary order
#### exit
env_of_ctx .exit()
- performs '->' operations in arbitrary order
- call's DEL on structure 
#### path
ctx .path() --> Path

As with all forms a ctx has a unique path.  This address is used to name the CTX for the RET command.
#### getdefs
Returns definitions that assume execute given instance unit of the CTX spec
#### getref (somehow return instance of CTX)
### OPTIONS
@fan      1-to-many          (inferred by non-referencable name with positonal last element)
@static   UP is undefined.   (inferred by lack of references in closure vars)
@choice   parent choose one
@toggle	  on/off in parent   (allow entering/exiting while also in peer context.  ?toggle)

- When a context has a child context that is @choice or @toggle then they are all instantiated together and the UNION of all choice and mixin values are bound as electric values that change depending on which choice/mixin are active at any moment.
- When a space enters/exits a @choice or @toggle context it is updates as if this choice/toggle were entered at the moment the parent space were entered (e.g. it "rewrites history" as if the choice or toggles were chosen differently at that time.)
### >with >let >bind
A dynamic version of with....   I think it oblits with 'with'

with [env.seed.HEAP <- env.seed.HEAP.0] ...
with [env.seed.HEAP <- bind(`env.seed.HEAP.0)] ...
with [env.seed.HEAP <- bind] ...


// somehow use 'match' to perform binding operations
// note: might need to perform backtracking or matching since some bindings 
// may not be compible with others ???

CTX ( @ord=backtracking)

// ???  really the notion of backtracking constraint solving should be orthogonal 
// see 'solve'


#### Usages
##### As a LET statement
- Perform '=' stmts and call .enter when computation reaches the 'let'
##### As a WITH statement
- Performs '->' on enter; when computations reaches 'with'
- Performs '<-' on exit; when computation reaches end of 'with', or on non-local exit 
##### As a CATCH-THROW / TRY-EXCEPT
- xx
##### As an UNWIND-PROTECT / FINALLY
##### As a STACK FRAME stack frame for a function call or operator invocation
- Performs '=' stmts in caller context

????Uses 'label' in order to reference ctx where into which this context will assign

##### As a CLASS statement
- A CTX's structure defines the member fields for the instances of a class.
- The '=' and '=:' declarations are used for initialization of the class instance vars
##### As a INNER CLASS statement
- Defines scoped variables for instances of the class to include local which shadow outter vars
- Allows the UP operator to gain access to the outter vars from instantiations of 

##### As Inner, Anonymous, Static, CLASS statements

or inner-upper-over-outer-anonymou-static classes (Java you know I am trash talking you)
All of these messy variations seem natually expresible as variations of CTX
### Tree of contexts
TREE -- base (std eval)
ADDON:
- meta / full -- meta key access, or full combined meta and non-meta key forms?
- permissions -- 
ALTS
- STEPS: normalization, expansion, 
- compilation


- PERMISSIONS -- permission/authority info for unit.
- ALPHA ALTERNATIVES -- Different types of alpha interpretation:  
normalization, compilation, 
- ALPHA ADD ONS --
assign
- MACRO/EVAL --  
### DISCUSSION
#### PowerSet Idea
- Every Ops has an and/or tree of add ons (must verify that all combinations are safe).
- Hints determine which combinations have been used in the past to be cached.

Given a set of contexts that share a common parent

- POWERSET CONTEXTS 
Given a set of 

~-~~
IMPLEMENTATION
#### The Null Environment
- EnvNull
- This is the environment which is provided externally to the tree of contexts.  
- This is the base interpretatation environment from which others are derived.
- This is the environment used for executing macros load etc.
#### SEMANTICS
- A context defines a space of values in terms of some base "containing" contextual space via the 'enter' function.  The 'enter' function is executed within the null environment.
	c2 = ctx.enter( c1 )
- A tree (space) of context declarations can be interpreted statically to derive a dependency tree of static contexts.  Each is "contained" by the one above, and is derived by entering from the one above.
- A dynamic space of values may by managed by dynamically entering/exiting contexts. Rules:
- A context can only be entered from a space that has already entered its parent.
- A context cannot be entered by a space that has currently entered a sibling context.
- A toggle context is an exception, it may be entered even as its peer is entered.
### OLD

 that serve as the basis for the process of interpretation.

to which defines an interpretation environment 

@class -- has many to one relationship with parent CTX
CTX [relpath] {bindings}*

^opt
fan				# Fanout, split, 

relpath
path for the name of this context within the parent ctx


var types:  infn, outfn, let/field
-- all (int&str) args of CTX define fields within the first 'fan' parent CTX

CTX
VAR
## === SAT ===
SAT(expr [, iftrue [, iffalse]}) ==

False = False || Und || Null
## === LWL1 ===
NEWEST STUFF IN ELEMENTS UNICORE

all of this stuff is becoming part of unicore
### --- summary ---

SUMMARY
- CODE --   
	- PKG TREE -- Code is expressed as a tree of pkg definitions
	- ORIGIN -- Pkgs are named relative to some common origin
	- LINKED -- Pkgs are linked with web of import/extends
	- NAMESPACE -- Each pkg defines a namespace tree
- SLOT -- 
	- VAR(name: type: backing_path:)
	- slot(match: args(x), get: expr, set: exprgs )
	- FN()
_
### _

LWL1 -- Combines Unicore Constructs into a 'bootloader' language used to define LWL and other languages.


- text/spec/form -- 

_
### --- API ---

**def class** _UcGnd_:						// A meaning containing structure
	**extends** Gnd
	**def op** _ident_(->Ident)				// The location of this grounding
	**def op** _form_(->Form)				// The form defining this location
	**def op** _ns_(->ns)					// The ns defined by this location
	**def op** _schema_(->schema)			// schema of location (really form.spec?)
	**def op** matcher(->)

// Alternative:
// NS: Ident=>Gnd   Class<Form  Form<Gnd  Gnd<PathyTree(Gnd)  
// Form: ns->NS, reduce(Env)->Unit, invoke(Unit,Env)->Unit


**def class** _UcLang_:								// State of an interp environment
	**def op** _bang_() -> Env						// Universe for exe of lang
	**def op** _lookup_(s Spec, _ns_:NS=None) -> Gnd: // Maps code to its gnd semantics
		_ns_ = _ns_ || _self_.gnd.ns
		**return** _ns_.path_get(_s_.head.path)
	**def op** _exe_(_self_ Lang, _code_ Code, _env_ Env, // Performs interpretation
			   _ns_: NS=None) -> Unit:
		_gnd_ = _self_.lookup(_code_)			
		_form_ = gnd.form.form(code)
		**return** _form_.reduce(_env_)
		
~-~~
	
**def class** _Gnd_:
	**def op** _ns_(self, Gnd, ->NS)				// 

**def class** _Unit_:
	**def op** _head_(_self_ Unit) -> Ident			// Semantics for a spec unit
### --- OPTS ---
Options defined at the unicore level

GND OPTS
	@is_slot		--  Grounding intended to define a slot
	@is_class		--	Grounding intended to define kind of unit
	@class_inst_path--  Path from Env used to access instances

SLOT OPTS
	@is_fixed
	
_
# ### LIB - MENAGERIE ###
## === IO ===
### --- SOURCE BLOCK ---

**SOURCE BLOCK** -- A delimited block of code contained within some textual source where the block delimter itself specifies the language for the block itself.


Desirable Properties. The source block is:
- CONTEXT AGNOSTIC -- it should naturally fit into a great variety of textual contexts, that is, it delimiter should make as few assumptions as possible about the syntactic constraints in its containing textual context.
- LANGUAGE AGNOSTIC -- it should support the widest possible range of software languages, with care taken to cover all existing langauges.
- NATURAL LOOKING -- it should look good and natural as the header of a section within the text
- should be easily parsed, ideally with a regular expression

Key Properties:
- LANGUAGE AGNOSTIC -- 
- DECLARISH -- One can statically inspect a web of DDDs and easily determine the dependency structure.
- UNORDERED -- 
- STATELESS -- just computes a value
- DYNAMIC -- value of DDD depends upon the source itself, so as that changes the DDD changes.
- EFFICIENT -- 
	- SCAN EFFICIENCY -- 
	- CACHE EFFICIENCY -- 


#### -- Semantics --

-=- lang.uf -=-

**pkg** lang.lwl;


lang.ddd := ddd.Lang()

**pkg** .ddd:
	
	**def** run(Textual t):
		mark, contents = source_block(t)	// Extract Source Block
		lang = Lang.bang(mark)				// Get Lang universe
		code = lang.load(contents)			// Parses and loads contents
		return lang.EXE(code)				// Execute code

	**class** Lang extends lwl.Lang:
		**def** load(Lang self, Textual t):
			code = lang.parse(t)
			pkg = Pkg.load(code)
			**if** pkg has `result:
				root[pkg.path] = pkg.result()
			**else**:			
				root[pkg.path] = pkg
			**return** root[pkg.path]

	**def** extract(Textual t) -> (Identifier, Str)

	def type DDDForm:
		:= U(...STMT, ...:DATA)
		STMT := LANG || PKG || IMPORT || COLON_EQUALS
		EXPR := CALL || PLUS



1. EXTRACT 	-- Extract body string and language indicator from text
2. BANG		-- Obtain the indicated language universe
3. PARSE	-- Parse the body string into source code
4. PKG		-- Process package statements in source code (e.g. import, ...)
5. RESULT	

_
### --- SPIRAL DOCS ---
Spiral Types (extended, sprouted, transformed)
_
### --- Data and Interface Embeddings ---
#### intro
A central design aspiration of Uniform is uniformity.  This is particularly critical for its embeddings.  
An <d>embedding</d> is a mapping of data or interfaces which exist outside of Uniform into the Uniform universe.

Uniform's design goals makes the creation of ideal embeddings quite tricky.  One wants have <q>have it all</q>, an interface that is a simple as possible, has as many <q>nice</q> properties as possible, and relates to the external world in the most natural way possible.  This is a tall order that inherently involves tradeoffs.
#### Properties of an ideal embedding
##### Telescoping
##### concatenative
##### Prefix grammared
#### Meta Details of an embedding
Embedding NAME
spec: SPEC
detectable:  false | true | Num(0,1)  // indicates a qualitative level of detectability
#### Path Embeddings
##### Date Indicator
date.
20YY.MM.DD.hh.mm.ss.micro.pico


TIMESTAMP:  2YYY.MM.DD.HH.MM.SS.MICRO.PICO
In uniform a "structural timestamp" is any dot form beginning with 2000 to 2099
(representing the year) which has a prefix matching the form above where
0<MM<12  0<DD<31. 0<HH<24. 0<MM<60  0<MICRO<999,999. 0<PICO<999,999
##### Version Number
version.
v.
3.2.5

major.minor.patch  .push_numbering

https://guides.rubyonrails.org/maintenance_policy.html
https://semver.org/


--- Versioned Paths ---

-- format:  xxx.yyy...zzz.2YYY.MM.DD.HH.MM.SS.micro.pico
##### persist -- File embeddings
##### persist -- Filesystem path
##### persist -- Hostname
##### persist -- URI
### --- TRANSPORT FORMATS ---
#### String form
Used to encode several types into unicode strings.  Primarily using prefix char to distinguish.

BACKSLASH (\)	Used to treat char after the backslash as a plain unicode char w/o any extra meaning.
				" ... \? ..."   here '?' is treated as simple unicode char.  
				("~" is also treated as a backslash character too)
QUOTE (')		Used as first char in order to indicate a string
BACKQUOTE (`)	Used as first char to indicate a symbol or path.  
				As in: "`this.is.a.path"
NUM (-)(0-9) 	A number begins with either a '-' or a digit, and has one (.) characters
INT				A number with zero (.) characters
_
#### -- JSON List form
"'STR VALUE"
["`head_path", ARG1, ":pkg.key1", VAL1, ]

_
#### -- Flat Schema Form --


IDS
						//  Small Integers
	0					//  NULL
						//  Schema Reference
						//  Circular Unit Reference
						//  Schema Definition
						//  Circular Unit Definition
	NUM_CHAR..			//  Regular Unit ids
	0..NUM_CHAR-1		//  Byte string values

UNIT	::=	 SCHEMA_ID  [CIRCULAR_UNIT_DEF]  VAL1  VAL2  ...
SCHEMA	::=	 SCHEMA_DEF  	LEN  VAR1  VAR2  ...
VAR		::=  VAR_SCHEMA_ID	NAME_IDENT  TYPE  

CHAR ::=	"'" + chars			// String value
CHAR ::=	"$" + chars			// Ident value
CHAR ::=	"#" + chars			// Int binary (big indian??) form
CHAR ::=	"." + chars			// Decimal value
CHAR ::=

VAL	::=	ID		< -65536
VAL	::=	int32
VAL ::= 

UNIT 	::=	SCHEMA  VAL1 ...
SCHEMA	::=	SCHEMA	KEY1 ...
KEY		::=	IDENT_PTR || VAR_PTR 
VAR		::= VAR_SCHEMA_ID 

schema, val1, val2, 

_
## === DATA ===
### --- IDENT ---
SYMBOL / IDENT DETAILS

- IDENT - An ident is a path indexing namespaces.
- IMMUTABLE - Idents are immutible.
- ATOMIC - Idents are atomic, but have structured contents.
- CONTENTS - Ident contents R keys for NS nodes, thus hashable.
	- INT - content elements may be ints.
	- SYM - content elements may be strings or hashable symbols
				depending on if strings are hashable/immutable
- HEAD - Is an Ident.
### --- GRAPHY ---

**LINKAGE** -- A _linkage_ is a named connection between instances of unit types. 

**UNDIRECTED LINKAGE** -- 

**COLORED GRAPH** -- A _colored graph_ is the transitive closure over a collection

**GRAPH** -- A _mono colored graph_ (or just _graph_ for short) is a colored graph with only one linkage type. 

**DAG** -- A _directed acylic graph (DAG)_ is a directed graph that has no cycles.

**TREE** -- A _tree_ is a singly connect graph.

{[picture]}


 


#### -- API --

DAG($KEY, $PARENT)


**LINKAGE**($T1: Class, $K1: Ident, $A1: $T2 || Stream($T2),
		$T2: Class, $K2: Ident, $A2: $T1 || Stream($T1))

Each accessor either returns a element of the other type, or return an iterator of the other type (if single elem)


#### -- Examples --


Linkage(I.one2many, type1: VarFrame, type2: Var, key2: I.frame)

LinkToMany(key_expr, type)

_
### --- MIXINS ---
#### -- PROV - Provy - Provenance --
##### _
**@provy** -- A unit whose provenance is tracked.

**PROVENANCE** -- Provenance is the origin and/or history of an object.

**HOW** -- The dynamic expression used to derive a value
**WHY** -- The static elements underlying this derivation
**WHO** -- The agent that initiated interpretation when the derivation happened

**PROVENANCE MODEL** -- The Languages for these three kinds of explanations

_
##### - API -

// ???
def pkg prov < ProvenanceModel
	def class HowLang Lang
	def var why_lang Type
	def var who_lang Type

def op how(Unit->lang.prov.how_lang)



_
#### -- DAG --
A space of units of a common type linked as a space w/o cycles

DAG(link_type, node_type, link_name)

_
#### -- TREE --
A singly connected DAG of units of a common type
### === BACKINGS ===
#### --- table ---

	-type-		-code-							-structure-
	PyClass		Address("1 Maple", zip="94110")	xxxx
	PyMap		Range(10,20)					[10, 11, ..., 19]
				Memo(1 if i<=1 else self(i-1)+self(i-2))
												[1, 1, 2, 3, 5, 8,...]
	Expr		Address("1 Maple", zip="94110")	[(0,"1 Maple"), 
												 (zip="94110")]
#### --- KINDS ---
	- Code = Immutable, Bounded, Tree (GET)
	- Heap = Mutable, Eq (has identity equality)
#### -- UnitExportList - 
	Designed 4 visual simplicity in other langs
	@can(LEN) 
	Format:  []
#### -- UnitCodeSimple --
	- In python this is backed by an OrderedDict
	- Is immutable, 
#### -- UnitAtomAdapter --
#### -- UnitStructAdapter --
#### -- UnitListAdapter --
#### -- UnitMapAdapter --
### --- WRAPPER ---

- WrapAdapter -- 
- __ facade __	== Returns wrapped version of object
- __ repr __		== Returns the form that derives wrapped object
- Wrap(Type) {GET: op(){...}, }
#### Pkg example

def type adder(T): 
	extends WrapAdapter,
	"+"(self, arg T...) ->





#### Examples
x.wrap(v1: und, v2:=1+v2, v3, ->) { print self; }
### --- LENS / VIEW ---
#### intro
LENS -- A live transformation of the data-graph and interpretation environment.
- "live" in the sense that changes on either side are see on other.
- Lens may be applied to a single instance to produce a transformed one  (see wrapper)
- Lens may be applied to an Env to produce one that sees xformed instances
#### semantics
### --- DDD ---

**DDD** -- A Data Dependency DAG (DDD) defines some structured data recusively in terms of simpler data forms.

Key Properties:
- LANGUAGE AGNOSTIC -- 
- DECLARISH -- One can statically inspect a web of DDDs and easily determine the dependency structure.
- UNORDERED -- 
- STATELESS -- just computes a value
- DYNAMIC -- value of DDD depends upon the source itself, so as that changes the DDD changes.
- EFFICIENT -- 

#### -- Semantics --



#### Usage

ddd_instance = github.oblinger.a_proj(tag: alpha2)

#### Detailed Execution
- lookup( x ) = _ ns _.path_get( x )
- INPUT: path_form (dotted list plus)
- ddd = lookup( path_form )
- lang = ddd.get(`lang)
- env = memo(lang) { bang(lookup(lang)) }
- result = env.EXE( path_form )

#### ??? DEPENDENT DATA FORM
- Instantion Form:       std.path.to.comp.ver.2.3("prime7", 44, cal: true)
- Instantiation Env:		std.lang.python.ver.3.(>=7)
- Instantiation proc:	__ init __ 
- Dependent fn:			$foo := ...
- Resulting Unit:		comp(...)     (abbr head when unabiguous in env)


_
### --- REPO ---
REPO (Repository) = an indefinite, versioned, persistant store of data.
- A sequence (or partial ordering???) of patches
- A history of values
- A queryable store
- A chain of Merkel trees
- An auth decision procedure for accepting new entries

- SPACETIME ADDRESS = STA( path, time, version )
- PATH ADDRESS:   path1.path2....20xx.time2...ver.3
- RETRIEVAL SEMANTICS -- (access) Select the lastest patch that:
	1. contain the specified path, and
	2. has a date prefix no later than specified prefix, and
	3. has a version matching the specified version prefix
		==> relevant sub-unit in the retrieved patch is returned.
- RETENTION SEMANTICS -- (persistence) Logic specifying (1) assurances about conditions where values are guarantees to be persisted, and (2) expectations of when value typically will be perged

#### -- Semantics --

RETENTION POLICY



_
#### -- Examples --
example patch
2019-05-18.003.lang.python3.7..types.builtin ->
pkg lang.types.builtin::


Example address within the repo:

std.lang.python3 	# the latest value stored for this address
					# maybe 3.7.13.
std.lang.python2.7 	# the latest value stored for this sub-version


EXAMPLE ADDR TESTS

std.uf.2017			#  	The most recent uf lang within 2017

eth.uf.2019.10		#  	Uses the most recent (eth.2017) to map the
					#	eth chain, and retrieves the last october
					#	version of uf (uf.2019-10-23.9426)

btc.uf.date(2019.05)


#### REPO Query Language

this.is.a.path		# Matches any entry with this prefix

a.path..2019.10		# Latest entry matching date prefix 2019.10 & path prefix
2019.10..a.path
a.path..ver.3.7 	# 

_
#### --- DOT LANG ---

DOT EXPR -- A _**dot expr**_ is a unit whose head is "."


FOO(arg1, ...)		-- open paren causes fn call
FOO.arg1			-- the ENTIRE suffix is treated as first arg, and passed.

foo(arg1.sub_arg1)

B2021.udk(com.ibm.2021.11.zuppy_pkg).




_
### --- ELECTRIC REPO ---

**ELECTRIC REPO** -- An _**electric repo**_ is a repo is a repo that retrieves and instantiates it contents.

**ELECTRIC PATH** -- An electric path is a language for recursively selecting, instantiating, and operating on structural and functional units.


OR: Live Repo, Semantic Repo

_
#### -- API & Semantics --

_repo_.**get**(_query_) -> _structure_only_unit_
_repo_.**exe**(_query_) -> _unit_handle_to_live_entity_

	Calling 'get' on a repo returns a matching unit structure
	Calling 'exe' on a repo will return an instantiated "live" unit entity


"PATH" REPO DEFAULT "GET" SEMANTICS
- All queries are mapped into a path query:  part1.part2...part_n.
- At most one prefix of pathquery will match a repo bucket's name.
	That bucket is used and the rest of the query is used to select within the repo bucket.


"PATH" REPO DEFAULT "EXE" SEMANTICS
- Query is broken by the '*' or '(' operator into a _prefix_ and _suffix_ expr
- A repo-get is performed on the _prefix_ to obtain the _base_ unit
- A repo instantiation time a base_env was supplied this is used here
- using the _base_env_ the _base_ unit is instantiated given the _suffix_ args
- The resulting unit instance is returned.


IDEA
- base is instantiated, and 'apply' operation is performed on result w suffix
	This allows each instance to control the semantics of its initializer keys
	

#### -- Live Path Part Types --

- REPO -- Allows part to specify a derived repo
- BANG -- Allows part to specify a derifed sublanguage
- DOCK -- Allows part to specify a docker container
- WEB -- Allows part to specify an https url for CRUD access

_
#### -- Examples --

import mkt_juris.shaXXXXX




juris10.2020.08 * 

    J10*web.com.af.repo**py3.8*app1


_
### --- VERSIONING ---
#### -- Semantics --

Version Number Semantics
- ORDERING -- temporal order generally follows the lexical ordering.
- VERSION RELEASE -- The publishing of a version to the larger community of users.  
- VERSION ACID -- Version Publishing mechanisms should be:
	- Atomic -- 
	- Consistent -- Once one user sees a version as having been published, all who care to look also see it as published.
	- Durable -- Version -> content mapping does not change

- VERSION LOCKING -- A condition where the mapping from version number to version contents is known to be forever fixed.
- VERSION NUMBER LENGTH -- Some version numbering systems will use a fixed length for their version numbers, this is called the version number length.
- VERSION RETENTION -- 


MATCHING SEMANTICS
- PREFIX MATCHING -- A template matches a target version number if it is a prefix of the target.
- 



LOCKING SEMANTICS
- PEER LOCKING -- when 
the default locking semantics for each version number is that it is locked at the time of release of the next version number at the same level as the number itself.  so Python
### --- STD ---
BOOT?
STD -- The "standard repo" is @chained @locked and exists in every uniform universe.  It provides the initial code used to bootstrap the universe itself.


- BOOTSTRAP -- Language used to 'boot' other languages
	std.lang.bootstrap	-- The unicore bootstrap language
	std.lang.bootstrap.root --  
### --- MIGRATION ---
MIGRATION -- A functional specification of a transformation which could be applied to some collection of elements.

REVERSABLE -- 

ADD KEY -- Add a key that is derived from the others
DEL KEY -- 
RENAME KEY -- A reversable add/remove
RELAXING/RESTRICTING -- of key or of whole element
_
### --- PLACE ---
issue: STREAM-PLACE -- how do these related?

PLACE  -- an association to a value that may change over time.
STREAM -- a sequence of values to be processed in temporal order 	on demand (pull semantics)
MESSAGE QUEUE  -- a sequence

--- spiral 1 ---
PLACEHOLDER 	 -- Temporally organized data
PLACE            -- A temporally organized sequence of values
ASSIGNABLE PLACE -- Processes 'assign' according to assignment sem
ACCESSIBLE PLACE -- Processes 'access' according to assignment sem
ADDRESSIBLE PLACE-- A place that can obtained by systemtaic 						processing of an 'address' structure.
LOGICAL PLACE	 -- A place that can be obtained thru its 							association with 

--- spiral 2 ---
STREAMING, MESSAGING, ARCHIVING PLACE
STREAM           -- Temporally organized data whose value only 						changes and it processed when read.  
					(pull semantics)
MESSAGE QUEUE	 -- Temporally organized data whose values are 
					processed when written. (push semantics)

Iterator, Stream, Queue (Pipe Queue, Messaging Queue), Archive

--- spiral 3 ---
HISTORY / CHANGELOG  (Unit Tree Graph)
PLACE QUEUE  -- A place 
STREAM ITERATOR -- 

#### Options

@can 	read(accessible), write(assignable), position(archiving)
		path(addressable), lookup(lookupable), listen(messaging)
		backing(backed)
### --- STREAM / HOOK ---

#### semantics
- maps the spatial structure of a unit onto 
	the temporal structure of a Stream.
- **listen** -- calls a binary fn on write to stream, ?append unit?
- **history** -- returns unit w. an item for every set
- **write** -- allows two args??  is the optional key
	
#### API
### --- TEMPLATE / SCHEMA ---
(maybe this is a dup?)

TEMPLATE -- 

MATCH -- 

FILL -- 

SCHEMA -- Alternate name for 'template'
_
### --- EXO-DATA ---
data that is presented as part of a thing in one context and NOT in another
### --- I/O ---
#### orig
-- Data services:  console  dom  local    http https  gdrive  dropbox

PATH
!!io.http.com.ibm.ww.uf(u:"oblinger",p:"******",x:"ext").dir.subdir.subsub.fname._.structure.in.file
!!io.http.com.ibm.ww."@oblinger".dir.subdir.subsub."fname.ext"._.structure.in.file
!!io.http.com.ibm.ww._.dir.subdir.subsub.name.ext.^uf.structure.within.file

ISSUES
-- allow forms in path (requires upgrading key type on GET)
--  SEAMLESS MAPPING:
[ www.ibm.com oblinger pass***  dir.subdir.subsub.filename.ext include_filename? file_cvt_type ]
^meta  (meta data about the mapping is stored in a DB associated with the 'http' and one associated with the host
or maybe just all with the db-per-host)
-- to avoid the '^uf' wedge one could have a side db that specifies the splices to make
or one could alias the active subtree down as an explicit command?
-- the 'http' nodes would be

INDICATORS
    / -- beginning of filesystem based structure 
    ~ -- beginning of filesystem based structure (in homedir of accessing account)
    & -- beginning of parsed file based structure
    #xxx  -- beginning of parsed file structure with 'xxx' processing
    @user -- specifies the user account used for filesystem access
    :port -- specifies the TCP port number
    ' -- begins string with quoted content

GOALS


-- simple ability to be 8bit clean with minimal mangling
-- dont treat extension as layer in tree!
#### MAPPING
-- scheme is simply mapped: e.g. http
-- scheme determines if authority exists
-- authority is mapped as subtree splitting on '.'
-- authority may begin with "@user" "@user:pass" 
-- authority may end with ":port"
-- PATH mapped as subtree splitting on '/'
-- SLUG (final part of path)
-- slug '#' and '?' split off
#### PRINCIPLES
-- PERSISTENT -- persistent paths to persistent info
-- PARALLEL -- Strucutre of remote data captured in parallel lexspace structure with minimal loss & translational complexity
-- DISTINGUISHING -- no collapse of two different info chunks onto the same path.
-- CONCATENATIVE -- Concatenative paths
-- TELESCOPING -- Path embedding DSL is indefinitely extending
-- NON-DUPLICATIVE -- (but two paths to one info is not ideal, but is ok)
-- PREFIX GRAMMAR -- specialized indexing parameters included in path using a Segment specific prefix grammar
-- COPY INVARIENCE -- lexspace is copy invarient, but target spaces in the worst case cannot be since the target itself is not copy invarient.  Still Uniform's embedding should not INTRODUCE any invarience.
#### RULE
-- map any p[*] prefixed with "'" as simple string sub structure w/o any specialized meanings below
-- map p[1] to scheme
-- map p[2] if prefixed with '@' as user/pass
-- map p[2] or p[3] if prefix with ':' as port
-- map 



EXAMPLES
io.http.com.ibm.ww._.folder.tree."file.ext".internal.structure
#### Embeddings
##### STD -- known values
-- a "standard" value is a value that is commonly available & consistent across interpretation env
-- these values are versioned (by storing a YYYY value under them)
-- these values are arrived at by community consensus
-- a given value (with a timestamp year marker) are consistent across all env

std.uf           // a current or recent value in this tree (a 'max' sub value)
std.uf.2018      // set at the beginning of the year
std.uf.2018.08   // set at the beginning of the month

lang std.uf;     	// a recent version of uniform
lang std.uf.2018; 	// a precise verion of uniform which was consensus on Jan,1 2018
## === FORM (FUNCTION) RELATED ===
### --- PYTHON ---
PYTHON -- Python related bindings


#### -- Examples --


def gnd_xxx(...):		# Reducer xxx in current package

@gnd("xxx.yyy") 		# Reducer yyy in xxx sub package
def foo(...):

def any_fn(x, y):		#  Invokes function w. std eval
### --- COMPONENTIAL SEMANTICS ---
COMPONENTIAL SEMATNICS -- Semantics that are derived by combining elements.
A componential system is defined by:
- COMPONENTS -- A class defining the semantic elements that can be composed
- ADD -- The composition operator "+" used to perform semantic composition on components 
- BASIS -- The framework into which the combination is unified


// COMPONENT SEMANTICS -- A class that defines the basis for combination and is the class of instances which 
// maybe combined.  Thus subclasses of this class are themselves 
def interface ComponentSemantics extends Class:
	@associative
	def fn "+" (comp ComponentSemantics) -> ComponentSemantics





// Example
Language lang = Lang + 
	uf.component.AtomicOps + 
	uf.component.CollectionOps +
	uf.component.
### --- INDEXED CASE ---

**INDEXED CASE** -- An indexed case uses an indexing function to (map) objects onto an enumerated set of 'cases' where each case may execute some arbitrary block of code.

**DETAILS**
- _CALLING FUNCTION_ - An indexd case is accessed by invoking its 'calling function'
- _CLASS FN_ - This calling function is accessed as a static function defined within some class/interface/pkg scope.
- _INDEXING ARG_ - Case indexing is derived from the first argument to this calling function.
- _INDEXING OPERATOR_ - The indexing operator deterministically maps each object onto a specific instance of some targeted "slot class".  (by default the indexing fn is the 'head' operator)
- _SLOT CLASS_ - Each indexed case implicitly declares an operator slot on the slot class. (by default the slot class is the unit's singleton class)
- _SLOT OPERATOR_ - The implicitly defined slot operator has the same signature as the calling function with its first argument removed.  That first arg is only used to index the slot instance to use.
- _SLOT KEY_ - Slot Key is the key used in declaring the operator within the slot class.  (By default the slot key has the same name as the calling function.)

API
	
	**def case** _case_fn_(_SrcClass_, ..._args_, 
				to: _SlotClass_ op: _op_, ->_ResultClass_):
		ident1: action1
		ident2: action2
		
	**def case** _case_fn_name_(onto: SlotClass index: `head)


#### -- Examples --

pkg _DeclaringContext_:
	**def case** _some_case_fn_(dup, sep: ='', ^onto: _MySlotClass_, 
			^index: _`head_, ->Str)

pkg _MySlotClass_:
	**def op** some_case_fn(dup, sep:, ->Str):
		return "this the default case value"
		
pkg _MySubSlotClass_:
	extends: MySlotClass
	**def op** some_case_fn(dup, sep: sep, -> Str):
		return sep.join([print(self)] * dup)


#### -- Choices --

- INDEX OP TYPE OUT OF RANGE -- what do to when instance returned by the indexing fn is not of the declared type.
	(maybe this should not be allowed?)

- SLOT OPERATOR vs SLOT FN -- slot operator is more natural, but requires triggering a specified op with specified self, where op was not looked up from self.



#### -- Unicore Usage --

pkg lang.unicore.lang.Exec:

    def case exec(spec Spec, onto:Spec.Meta, op:`head.access, ->Exec):
		blk: Blk,


y = x.exec(deep: True)
    y = x.head.access[`exec](deep: True)

def fn lang.exec(c Code, ->Exec):
	return c.head.exec(c)

pkg lang.Exec.Class:
def fn exec.access(back lang.Exec.Class, c Code, ->Exec):
    	return back[`exec](c)
		
		
_
### --- STATIC CASING / HEAD CASE / VIRTUAL TABLES ---
#### -- Head Case --

**HEAD CASE** -- A HeadCase is a software form that cases (branches) on a unit's head in order to decide how to process it.

Details:
- root -- A head case is rooted at a particular type, and only applies to objects of that type.

head_case(...args) { type1: expr1, ... }


#### -- Discussion --

discussion continuing in VAR section

**STATIC CASING** -- perform static case derivation and instance structuring so run time casing can be performed by simple constant offset pointer dereferencing.

Generality:
- Single inheritance
- Aggregative

Idea:
- PKG -- Pkg are aggregated into Lexspace over time.  
	(all defined values within sub tree are frozen, but new subpkg / value may be added.)
- CASE VARS -- All case vars that apply over the pkg must be defined with pkg itself.  And each is given a unique int id (starting after last inherited int id).
- 
- BOOT -- At universe creation all static cases are known (declared in code). Each is scoped within an existing pkg, and given an int id unique for that pkg.
- TYPES -- New pkgs may be added but each is immutable
- INTERN -- Interning an identifier returns a new globally unique identifier 

Points
- On PKG.load most impl create a class type vtable for pkg needing it.
- Virtual fields or functions use indirection thru each instance's vtable-ptr in order to access those fields/functions.  
- This roughly doubles the time complexity relative to a non-virtual access.

(most discussion in ENV)
### --- HOOK ---

**HOOK** -- A _**hook**_ is a composite that executes its children when it is executed.  Its list of children are modified in order to add or remove computation at the "hook point" -- a specifically defined point within some larger computation.


API

Hook: Class::
	extends: [Composite, Executable]



## === ENGINES ===
### _

**ENGINE** -- An engine is a form whose spec fully specifies the interpretation of call forms that reference this engine.  

Each engine type defines a mini-language for specifying it execution behavior

_
### --- SWITCH ---

**SWITCH OPERATOR** -- A case operator is a callable (reducible) form that selects between an extensible set of reducible "cases" when the form is executed.


**TYPE CASE / HEAD CASE** -- headcase and typecase are built in kinds of switch operators.  A typecase indexes off 

SWITCH OP, SPLIT OP, MULTI OP, DISPATCHER

_
#### -- SEMANTICS --


- The 'form' macro xlates a CALL expr into a reducible form
- Each switch case is labelled by an identifier
- Each case is a reducible form that executes it case
- The 'key' fn is the dispatcher that maps a case form to a case identifier

_
#### -- API ---

path.to.case_op:  headcase:
	case1: reducible1, 
	...

_
### --- LANG SPEC ---

Maybe merges w. Function.LangModel
### --- FORMULA ---
**FORMULA** -- An application specific parametric expression (1) that adheres to some expression spec langauge, and (2) is used for deriving some app specific value. 

- FORMULA LANG
### --- TRANSFORM ---
what is a shorter name for this
Decision Procedure (actually in collab or society?)
### --- REWRITE ---
#### Rule Matching Engine

Match Tree
~ A DAG matchers organized by containment relations, and having unique paths for each node
~ A Pure tree only references a single relation
~ A Simple tree only uses the GET relation 

Rule
~ A set of antecedent matchers
~ A consequent template

Node -- 
~ a collection of match threads

Node Weight
~ 1 + the average weight of its splits if has splits
~ w * the number of unsplit rules.  (w=1/2 by default)

Node Split
~ choose the splitting dimension with greatest weight reduction
~ split a node if its weight drops significantly

Context
~ assignment for some 
### --- NPUTATION ---
NPUTATION -- An n-way computation -- a collection of functions that allow value updates to flow in different directions across a related set of input/output variables.

#### semantics
- **NPUTATION FORM** -- An nputation is encoded as
	1. A sequence of named places.
	2. Functions associated with some of these named places,
		whose arguments are a subset of these named places.
- **NPUTATION EAGER UPDATE SEMANTICS**:
	Given a map of one or more place updates
	1. Constrcut a map of the original values for these places.
	2. Scan all UNCHANGED named places in order
	3. If place has an associated nputation function
	4. Call it with originals map and all place values
	5. Assign result to associated place

before or after update?   after is cleaner, but keep original?
format to send listener in?  use keys
abilty to send: key, old, timestamp -- if args not none
### OLD ENGINE STUFF
#### Impl idea

-- When context switch auth count is increased (this will invalidate all auth entries)
-- All DAG nodes are labelled with: 
	(1) An order number, a min & max entries for all order numbers within its tree.
	(2) a possibly empty list of active min/max ranges, or unknown 
	(3) a lambda
-- if auth count is behind, then node is recomputed.  
_
#### Old Notes

permit(auth_ctx, requested_op, targeted_unit)

assertion:

	allow





where
  ctx 	is a location within semantic space
  


    <h3>PERMISSIONS</h3>

    NOT READY FOR PRIME TIME....

    In large software systems there is great value is having strongly controlled permissions.  If any part
    of the computational system can affect any other part of the system, then reasoning about these systems
    becomes impossible.  At the same time, overly onerous permission system, or systems that do not
    closely align to the task at hand, can also increase accidental complexity.

    There can be no one-size-fits all solution here, but a laissez-faire model where anything goes really breaks
    down quickly.

    Fortunately Lex affords an extremely elegant model for permission management: 100% of all access and all
    action is controlled by the bindings available at during execution &mdash; if you cannot see it, you cannot
    access it, you cannot modify it, you cannot do it.

    <br><br><code><i>permissions_model</i>()</code><br>


    <br><br><code><i>ACTIVE_PERMISSIONS_OBJECT</i>.<b>is_permitted</b>(<i>code</i>, <i>env</i>, <i>path</i>)</code><br>

    This permissions object is indexed off of the current execution thread, and it applied by the executor
    based on code and env.
# ### LAL - LANGUAGE AUTHORING LANGUAGE ###
## _

LAL1 -- "Language Authoring Language" -- a collection of uniform constructs aimed at writing the parsers, macro-expanders, and transpilers required for creating new Uniform DSLs.



LWL := SPEC + ALPHA + CTRL + MISC + PLUG
LWL := TEXT + SPEC + FLOW + MISC + PLUG

LWL -- The _LWL (language writing language)_ is a DSL specialized for the specification of other DSLs.  


_
## === TEXT ===
TEXT := MARKDOWN + FORMATS +
### --- COLON ---

section		::
value		:
code		:

#### choice - can code and section body be the same
- well section body should/could allow statements (like a def form)
- corner case.   else: xxx
	- Encode this as ":"(else, xxx)  this can work for both sub-section AND code???
### MARKDOWN
### EXPORT FORMATS (list export form)
[":foo", 1, 2, ":three", 3]
[":", ":", key, value, ...]
how to show a graph 
#### List Form

"'a string", 555, "$a symbol", ":a_keyword"

[":person", "'do", :first", "'dan", 12, "^meta", 4]
[":", ":key", "'value"]  // no head
## === CLASS/OPT === 
### _
OPT = UnitOpt + PlaceOpt + 

**OPTION** -- an property associated with the type of a unit which indicates something about the behavior of the unit.

Thus options are themselves a kind of type, an indicator for the collection of units that have the cooresponding behavior.



_
#### -- PLACE OPTIONS --
All place options may apply to a unit or a type.  In these cases the option applies to all places in the unit, or across all places within all instances of the type.

##### - @Provy -
##### RETENTION 
RETENTION -- The period of time a place is guaranteed to exist and maintain its assigned value.

LIFETIME
#### -- UNIT OPTIONS --
#### -- ENV OPTIONS --


#### -- 


#### --- UNIT ABILITY MIXINS ---

	**--ABILITY--** 		**--MEANING--**
	Headable		--  Able to head_set
	Listable		--  Able to append (appendable)
	Functional(x)	--  Mapping of key to value is 'onto'
	Accessible(x)	--  Can 'get' values by key
	Versionable(x)	--  Can 'set' value to version existing unit
	Mutable(x)		--  Can 'set' value to mutate existing unit

	Iterable		--  Can 'iterate' values
	Ordered			--  Iterator is stable
	Finite			--  Iterator is guaranteed to terminate

	Taxonomic		--  If x!=y then path_get(x)!=path_get(y)
	Pathable		--	Is taxonomic and 'path' defined everywhere
	Bounded			--  Traversal is guaranteed to terminate

	Electric		--  Actions occur as result of writes
	Historical		--  Maintains seq of versions (unit or tree)
	Watchable		--  Has listener queue of edit messages
	Missing			--  Action on missing key:  null, UND, ERR

	PERSISTENCE		--  How unit data can change over time
						live, mutable, lazy, fixed, frozen
	LIFETIME		--  How long does the unit data exist
						stack,heap,universe,stored,chained
#### UNIT
##### @persist = @live || @free || @lazy || @frozen / @firm 
	@volitility / @stability = 
	Option describes the level of persistence provided by a unit.  
	- @live - volitile - Under control exogenous to current thread of interpretation.
	- @free - mutable - assignment semantics.
	- @lazy - locked - unchanging once defined.
	- @firm - fixed - unchanging value
	- @froze - frozen - error on set
##### @lifetime = @stack | @heap | @universe | @indefinite | @chained
	- @stack / @dynamic - 
	- @heap / @static - 
	- @universe - 
	- @indefinite / @stored
	- @durable - contents stored
	- @archived - 
	- @validated - archived and hash validated on chain.
	- @chained - validated and contents on chain.
##### @order = @ordered | @bag
##### @values = @multi | @fn / @onto 
##### @can = @get @set @append @path @inv 
		@get = Structure can be accessed via get
		@set = Structure can be updated via set
		@append = 
		@path = Structure can be traversed as a tree (up&get work)
##### OTHERS
	@closed / @bounded - iterator is bounded 
	@finite - transitive closure over the iterator is bounded
#### --- VAR ---
	- @electric  @tempo
	@list	@listy @lst 
#### --- LANG OPTIONS ---
	@static -- the language's bang.env is @muty 
	@dynamic -- The langauge's bang.env is @fixy
### --- KINDS OF TYPING ---
#### _

**VOCABULARY**

**STRUCTURAL (INSTRINSIC) TYPE --**

**CHECKED TYPE --**

**CLAIMED TYPE --**

**OBSERVED (EMPIRACAL) TYPE --** 

**MANIFEST (HEAD) TYPE --** 

_
## === SPECS ===
### _

**SPEC** -- An _**X**-spec_ is DSL for specifying instances of an indicated class **_X_**.

For example, Regular Expressions are a kind of "String Spec" -- they are a mini-language for specifying instance of type String.  Here are the spec languages that make up uniform's language writing language:

- **STRING SPEC** -- Format strings, regular expressions, and string templates are three kinds of string specification mini-DSLs.

- **IDENT SPEC** -- _Identspec_ is a small DSL for specifying path structures within source code.

- **ARG SPEC** -- Argspec is a DSL for the actual arguments for a function/operator call into the map of parameters to be injected into the operators stack frame

- **TYPE SPEC** -- _Typespec_ is a DSL for specifying types -- constraint predicates indicating matching graph structures.

- **INTERFACE SPEC** -- _Interface spec_ is a DSL for specifying functional contraints -- for specifying matching functional signatures.

- **CLASS SPEC** -- _Class spec_ combines type and interface into an OO-like specification of both structure and function for the instances of the class.

- **PKG SPEC** -- _Pkgspec_ is used to specify the contents of a source code package.

- **UNIT SPEC** -- _Unitspec_ is a DSL for specifying unit predicates -- predicates that return true/false depending if some unit is in the set.  Structure templates are an example of unitspecs since they match certain structures but not others.

- **META SPEC** -- _Meta spec_ is the spec for new kinds of specification languages.  So there is a Metaspec for regular expressions, and for all mini-DSLs that make up the uniform langauge writing language.

- **LANG SPEC** -- The uniform langage writing language is also called "_Langspec_" since it itself is a DSL for specifying DSL language instances.  Langspec refers to the collection of uniform constructs aimed at writing the needed parsers, macros, and transpiler rules.
### --- ALT SPEC ---
### --- ARG SPEC ---

ARG SPEC -- 
### --- CLASS SPEC ---

**CLASS** -- A _class_ is a **type** coupled with an **interface**, where a _type_ a specifier of some constraint on the **structure** of a unit, while an _interface_ is the specifier of some constraint on the **function** of a unit.

CLASS SPEC 
### --- IDENT SPEC ---

**IDENT** -- A unit whose nub is a @Fixed @List of Str or Int. 

    I(-2, "I_am", "an", "$identifier", 777, 44) 

All Units have an Ident as their head.  This head Ident provides all meanings for the unit.  The semantic groundings for the unit are obtained by looking up this head within the Namespace of the current interpretation environment.  (See Env, Env.lookup, and NS for details.)





**IDENTSPEC** -- Various code forms used to specify an Ident.

#### -- Examples --

Idents are a frequently used Uniform workhorse.  The serve as atomic symbols within data forms, as indicies within looping contexts, as logial naming identifiers within semantic (functional) forms, and as addressing paths inside structural forms.  Thus Uniform include a range of alternate ident specifiering sugared forma covering a wide range of different coding contexts. 
Here we enumerate these alternatives by example:


**STR FORM**	**UNIT FORM**
foo			I("foo")			# Bare Identifier
foo.bar		I("foo", "bar")		# Simple dotted ident
foo.3.bar	I("foo", 3, "bar")	# Allows numbers
foo.'3.bar	I("foo", "3", "bar")
foo'.bar	I("foo.bar")		# Quote removal
foo\$bar	I("foo\$bar")		# Leave slashes alone
..up one 	I(-1, "up one")
...up		I(-2, "up")
......up	I(-5, "up")
-5.up		I(-5, "up")
.up			I("", "up")		--OR--   "."(GAP, "up")
two word	I("two word")
'foo		"foo"

**UNIT FORM**		**PATH FORM**
f(x)		f(x) 		# No conversion
"'3"		"3"			# Converts to a reduced string
"'"			"" 			# Converts to a reduced string
"" 			path()
"."			path("")	
".."		path(-1)
"3"   		path(3)
"3'"		path("3")


=== PATH ===
  a.b                                         //  a simple path
  "a".b."c".3                                 //  numbers & strings allowed
  $a . ?b .^c                                 //  so are prefix tokens
  '...a.b                                     //
  a.b[c].d(e).f                               //
 foo.bar.bat = 23                             //
 foo  .bar. bat = 23                          //
  '.a                                         //
  '..a                                        //
  '...a                                       //
  '....a                                      //
  '.....a                                     //
  '......a                                    //
  '.......a                                   //
  '........a                                  //
  '.........a                                 //
 [here.there ,  _.this.that, foo.bar(x) ]     //
_                                             //  evaluates to current Env
\'"."("a", "b")                               //  absolute path relative to root
'a.b                                          //  same same
'_.a                                          //  path relative to current 'place'
'_1.a                                         //  up one level
'_2.a                                         //  up two levels
#### -- Identifiers as indicies of meaning --

**TL;DR.**  The head of an identifer is equal to itself; Symbols are even more restrictive, they guarantee that their head object is the symbol object itself:
	ident == ident.head
    EQ(symbol, symbol.head)


**SYMBOL** -- A Symbol is an Ident that is it own head.

WTF is going on here?!  Unlike nearly all langauges, Uniform cleanly decomplects structure from function.  This decomposition allows us to pin down the structure of an identifier without pinning down its functional bindings.  Uh... ok, but why do this?  OO-languages typically provide powerful method-dispatching mechanisms which branch execution based on instance 'type'.  But they fail to generalize this dispatching ability to dis-embodies 'types', e.g. to symbol like objects.  Some languages like C++ and Clojure do decomplect method dispatch (as virtual functions and ).  Uniform does this as well, but it merges this disembodied type-dispatching with conventional class method dispatching 

- decomplect mehtod-dispatching from the type hiearchy, really one can apply method to any set with a partial order defined over it.

- And the path lists within identifiers defines such a partial order structure even though they are just one structural data type.

- So in uniform we decompletec method dispatch entirely from typeing, and generalize it to apply to any DAG structure.  Then we unify that DAG dispatching with operator dispatch sematics into a unified semantics for interpretation:

	meaning = env.lookup( form.head )
    while form != I.GND:  form = env.lookup( form.head )

- this allows us to use an identifer as a disembodied type for dispatch, but also allows us to use identifiers as message protocol symbols, as Exception tags, etc. and leverage extensible method dispatching over these as well.

- The token identifiers used by the parser provides a good example of this extended usage.  At a base level tokens are symbols just as with all parsers.  They are grouped by kind of token so one can perform certain parsing actions using operator dispatch over that hiearchy.  One can also define specialized grounding for a single token, this would afford that one token its own parse behavior.

- Essentially Uniform behaves as if each token value cooresponds to an implicitly defined OO-class and the token itself is the singlton instances of that class.  But we get all of that behavior "for free" since we have decomplected the dispatching machinery from the class machinery and from the type hiearchy.

- n example use





Unlike most programming languages, there is no single Class for Ident these objects.  There *IS* a single data type, as we said an Ident is an object whose nub is a list of Str + Int


#### ... IDENT SPEC ...

IDENT -- serves as name

**IDENT SPEC** -- The address of one unit A simple lossless mapping from Path units onto Text

def type Ident: List(Int || Str)
def spec IdentSpec: Str => Ident

**IDENT ELEMENTS**
- INTEGERS
	- NAT -- Natural numbers indicate positional sub-part
	- NEGATIVE INT -- Indicates number of "UP" transitions
- STRINGS
	- ALPHANUMBERIC -- Alphanumeric identifiers containing $ or _ and not beginning with a digit indicate named sub parts
	- META -- Alphanumeric strings beginning with "^" are meta keys
	- ROOT -- A bare "^" indicates the root of the indexed structure
	- RELATIVE -- The empty string explicitly indicates a relative path
- OTHER -- Other strings or non-int non-string values are legal elements of an ident's nub

	
**IDENT SPEC**
- **DOT** -- Path parts are separated by the "." character
- **SLASH** -- Single slash indicates special treatment of next char
- **PREFIX DOT** -- Indicates a GAP at front (current dir)
- **DOUBLE DOT** -- Indicates up one level
- **MANY DOTS** -- Indicates multiple levels up (one minus num dots)
#### PATH FORMAT TYPES
- SIMPLE PATH FORM -- parts are integers or simple strings.
	Strings of printable characters without (.) (') and do not parse to some integer.
- PATH STRING SPEC FORM -- A string denoting a Path form.
- P-FORM -- A string with a 'p' prefix indicating that it denotes a Path form and should be parsed accordingly.
- PATH FORM -- A unit whose head is 'path' and whose args are integers or strings.
- GENERALIZED PATH FORM -- a PATH form whose args are any Expr
#### PATH SPEC to PATH FORM loading
1.	**DIRECT** -- If spec is of 'Path' type then it maps to path form
2.	**NON-STRING EXCEPTION** -- If not a string then maps to itself.
3.	**QUOTE EXCEPTION** -- If is a string beginning with single quote ('), then maps to string with only the remaining chars.
4.	**DOT SPLITTING** -- The string is split into PARTS at each dot (.) that is NOT preceded by a (')
5.	**PART INTEGER CONVERSION** -- PARTS that represent integers are converted to those integers.
6.	**STRIP QUOTE** -- Reduce any sequence of one or more quote (') characters in a part string by one quote character.

PATH CANONICALIZATION
7.	**EMPTY STRING CONVERSION** -- Replace subsequences of n empty strings "" into the integer 1-n.  A single empty string is replaced with nothing, except for the first postion which is left as an empty string.  e.g.  path("", "", "") -> path(-2)
8.	**UP MERGING** -- Any adjacent pairs of negative integers can be merged by adding them together:
	e.g. path(-1, -1)  becomes  path(-2)
9.  **SHORT CUTTING** -- any string or non-negative integer followed by a negative integer can be removed by adding 1 to the negative integer.  e.g.   path("foo", -1, "bar")  becomes  path("bar")  
#### PATH FORM to PATH SPEC dumping
- **PRE-QUOTE CHARS** -- Add an extra (') character before any 
	(.) or (') characters in any part strings. 
- **POST-QUOTE INTEGERS** -- Convert any part integer to a string and add a single (') at the beginning of the part string. Unless this is the first part, then the (') is added at the end of the integer string
- **DOT MERGE** -- Join the part strings together adding a (.) character between each part.

Dumps to a string that will load back into the same path form.

#### CHOICES

##### CHOICE PATH_NULLS 
	CHOICE: Allow nulls in path?  

	Do we need them?  Could use the "" string for this?
##### CHOICE: PATH_RELATIVE
	CHOICE: Use "" or -1 for relative path prefix.

	CONSIDERATIONS:
	- Canonical paths have a slightly simpler type signature 
		(first arg is always an int, or could be nothing)
##### CHOICE PATH_HEAD
	Use ".", sym.path, or None as head for path

	DECISION:  Use 'path'

	CONSIDERATION:
	- SIMPLE -- The simplest datastructure might be a simple array (no head) for path.  (++None)
	- IDENTIFIED -- Nice if path objects are somehow indicated when they are passed around in the code (not as the head of a unit).  (--None)
	- DOT CONFLATION -- Using the "." notation will conflate the path usage of dot with the chain usage of dot. (--dot)
##### CHOICE PATH_GAP
	Use sym.GAP, "", or -1 to denote "relative path"

	DECISION:  Use ""

	CONSIDERATION:
	- SIMPLE TYPE -- Path types become simpler since they now allow all strings and all ints, and only strings and only ints.
### --- INTERFACE SPEC ---
### --- META SPEC ---

META SPEC -- 

Each of the spec types 
### --- LANG SPEC ---

**LANG SPEC** -- This is the name of the Uniform layer that completes the LWL -- the DSL optomized to create specialized languages.

Formally uniform's LWL is a combination of the first five layers of the uniform stack:

LWL = uf.Math +uf.Structure +uf.Interp + uf.Assembly + uf.LangSpec
### --- PKG SPEC ---
### --- STRING SPEC ---
### --- TYPE SPEC ---

**TYPE** -- A constraint on graph structure.

**TYPE SPEC** -- A DSL for specifying types.


Here is the rewrite grammar for typespec:

	TS ::= `&&(...TS)
	TS ::= `||(...TS)
	TS ::= `!!(TS)
	TS ::= `U(args UBODY..., h:STRSPEC)

	UBODY ::= TS
	UBODY ::= `@(OPTSPEC)				# Option
	UBODY ::= `=>([CLASS], [CLASS])		# Domain & Range
	UBODY ::= `->(GAP, CLASS)			# Return value


#### -- Types built into Uniform --

**INT** -- _Int_ is the class of all Integer atoms.   The colon operator may be used to specify a sub range of all integers:  Int(5:10), Int(100:)

**NAT** -- _Nat_ is the class of natural numbers.  E.g. Int(0:)

**NUM** -- _Num_ is the class of all decimal numbers of finite length.  Like integers one may specify a range, additionally the first integer parameter specifies the number of bits used to store the float.  So Num(0:1, 32) specifies a 32-bit floating point number between 0 and one.

**STR** -- _Str_ is the class of all String atoms.

**IDENT0** -- An _Ident0_ is one segment of a dotted identifier.  It is either an integer or matches the regex below for a valid code identier (an underscore or char optionally followed by more chars underscores or digits)
      [a-zA-Z_][a-zA-Z_\d]*

**IDENT** -- An _Ident_ is a Uniform Identifier -- an atom whose nub is a List of Ident0.

#### -- Examples --


List( 'Foo(Env, ->Unit) )
'Foo(Env,->)...
'...Foo(Env,->...Int)

#### ??? MATCHER

##### Syntax
	- **STRUCTURE**	mapping(d, r)	domain => range
	- **RETURN**		returns(r)		-> return_value
	- **OPERATOR**	returns(op,r)	method -> return_val
	- **PART**		slot(k,p)		key: part_type
	- **SAT**		sat(e)			sat: sat_expr
	- **HEAD**		head(h)			h: str_matcher
	- **TEMPLATE**	template(t)		`some_template($with, $$variables)
	- **EXTENDS**	extends(s)		< super_type
	- **AND**		and(t1, t2)		t1 && t2
	- **OR**			type(t1, t2)	t1 || t2


	formats:
	Int<Num Sym<Str None<Any
	List(x) == [x]    Map(x,y) ==  x->y   Op(x,y)  ==  x=>y
	u(a Int, b List Int=[], c Int..., foo: Int, ...: Str)
	u(h:headspec, t:typespec, sat: expr, Domain -> Range, "foo": type, SelfType => ReturnType)


	Match(form, template)

###### Syntax Examples

	def type foo:
	



##### Arrows
	In JS:  (a,b) => a+b
##### ??? TYPE +
	* EMBEDDED TYPING -- Just as 'backing' allows strucutre to be embedded, one should be allowed to embed into the type tree.  so when one gets to the 'file' type, the type tree should reach down into types of files.  but somehow these sub-typings cannot get in the way of different types of files (e.g. locally backed, lockable, etc.)...  Should be a uniform typing form.
	* TYPE - A partial order over matchers (unary predicates).
	* TYPE backing -- 
##### ... Typespec Examples ...


	def type HTML:
		HTML($HTML_HEAD, $HTML_BODY)

	def type HTML_BODY:
		::= U(h:re"[h|H]\d")
## === ALPHA ===
### --- COMPILE ---
### --- DUMP ---
### --- EVAL ---
### --- EXPAND ---
### --- HOW ---
Dynamic provenance
### --- LOAD ---

**LOAD** -- Interpret a code form (Spec) as an executable form (Exec).
### --- NORMALIZE ---
### --- PLACE ---
### --- PARSE ---
### --- PRINT ---
### --- WHY ---
Static provenance
## === FLOW ===
### _
**FLOW** -- The LWL control flow primatives for interating, branching, and non-local jumping.
_
### --- IF ---
### --- CASE ---
#### _

**CASE** -- An n-way branch whose execution complexity is sublinear (typically constant) in n.

?INHERIT? CASE -- IDENT CASE -- ORDERED CASE -- PO CASE 

 **HEADCASE / TYPECASE** -- 

_
### --- FOR ---
### --- 
## === PLUG ===

**PLUG** -- The plug or plugboard is an extensible taxonomy of entities with configurable semantics.  These entities provide standardized naming and control for an extensible collection of many specialized aspects of computation.

The plug is used to avoid language forking by providing a place where communities can express aspects of computation in most general shared contexts.  (See LWL.NAMES for root plugboard, see AGENDA.NONFORKING for strategy.)

#### notes
To achieve non-forking one must always express specializtion in its most general, most essential forms.  Thus one cannot tie specializations to the packages that first require that specialization, since it must evolve to equally apply for to all relevant future packages as well.

The only way in practice to aim in this direction is to take each proposed specialization entity and boil that entity down to a simplest expression of that concept, and to encode that concept within the plug.

For example, a object has the notion of lifetime (e.g. stack, heap), this is mapped onto our graph model as a property of certain places.

Later we see that a data repository also has a someone related notion of data retention, but with a much more general vocabulary about times where persistence is guaranteed.

So we generalize this core notion of guarantees about when assignement semantics will hold so that it covers both cases and is expressed in as univesal way we can conceive.
## === LANG === 
### PARSE +
- CONTEXT TERMINATOR -- :_group_
### PRINT
- Pretty printer
### LOAD +
#### thinking
Loader embed form.
- Need way to specify how file types are loaded.  (used extended typing)
- Need prefix that indicates uniform within such file. 
- Need way to exclude line prefixing over region

##### INTELLIGENT load action
- Load needs to use file type, and parameters on load to decide how to process an input in order to load it.  (so loading a python file should result in loading it into an new or available attached python runtime)
### --- LAYERS ---
OBJECT MIXIN LAYERS -- 

DATA ANNOTATIONS--  Refinement info about base data graph
					Ordering, InverseLinks
DATA GRAPH 		--  Mathematic structure underlying unit data
## === NAMING CONVENTIONS ===
### --- OPERATORS --

UNIT
- head -- 
- type --
- text --

GROUP
- new -- 
- del -- 
- itr -- 
- 
- index -- unit has pair w. specified value (at given position)
- has -- unit has pair w. specified key
- in -- map 'has' or list 'index'

EXEC
- spec -- 
GND
- id --

#### -- discussion/ideas --

if x isa Klass
if x =~ Klass
if KlassX < KlassY

live { x := y + z }



### --- COMMON NAMESPACE TREE ---

A
E. ...			-- Env		The active stack frame
I. ...			-- Ident	== Ident("...")
L. ...			-- Local	== local...			// Heap (local to Univ)
W. ...			-- World	== io...

io				-- Pkg		Connections to world.  (URL?)
io.dom...		-- Tree		Browser dom
io.home...		-- Tree		User homedir (usually an alias into the filesystem)
io.file...		-- Tree		Local filesystem
io.http...		-- Tree		HTTP protocol ports
io.https...		-- Tree		HTTPS protocol ports

lang			-- Pkg		Constructs for this world
lang.alpha		-- Alpha	The interpreter for universe
lang.base		-- Env  	Base Env for universe
lang.base.ns	-- NS		Base semantics
lang.env

py
py.import...	-- Tree.	Tree of python imports
py

seed			-- Pkg.		Constructors.  "new" function returning new units.
					Maybe these are just the class constructors:  Env, Spec, ...
seed.ENV
seed.TEXT
seed.SPEC
seed.EXEC

#### -- Naming Examples --

L.uf.Point.44		# use outer most abbreviation as path ???
L.thread.0.23		# 23rd stack frame of thread 0

e = ENV.get_args
### --- NAMING RULES ---
#### _
LAL 
- ALL_CAPS 	-- constant	-- A value that is constant over lifetime of universe
- CamelCase 	-- class	-- A group of related instances
- snake_case -- var/pkg 	-- A slot or package

ALL_CAPS (constant)
- a constant value; a singleton; the spec of a pkg
- a special variable???
CamelCase (class)
- a grounding (Gnd); a class specifier ()
- a class, a type, an interface, a template
snake_case
- a slot as local variable (stack variable)
- a slot as a operator name 
- a package (Pkg) name

THINGS
- UNIT W/O SEMANTICS -- use snake_case  (spec instances) 
		person("Dan")	if(true, print)
- UNIT W SEMATNICS -- use CamelCase.   (exec instances)
		Person("Dan")	If(True, )

CHOICE: FACTORY_SEED_CASE
- Are factories constants in all caps?  Use new operator?
_
#### -- Example Namings --
	**pkg**    	// The head of a structural spec for a package (a Spec instance)
	**PkgSpec**	// The type of the structural spec
	**Pkg**		// The head of an executable package (an Exec instance)
	**Pkg**		// The type of the executable package instance
	**PKG**		// The name of the package construct implementation pkg
	**@pkg**	// The option indicating that a Spec or an Exec is a package
_
#### -- Suffix Rules --
##### ...Y
##### ...ER
##### ...SPEC
##### ...TASTIC
#### -- Identifier Case Rules --
	- Identifiers for placeholders with **_varying_** values use snake_case.
	- Identifiers that have a _**constant**_ value over their entire scope are written in ALL_CAPS.
	- Identifiers that reference a _**single**_ ??? cate

	THE FINE PRINT
	- DIFFERENTIATING CONSTANTS -- Often ALL_CAP variables will have different values within different instantiations of the Env in which they are defined.  This is still not a varying value, since it remains constant across the Env in which it is defined.  By contrast a snake_case variable may have different values at different times within A SINGLE Env instance.
#### -- Package naming uniqueness --

shortname -- naming phrase for the package
alias -- abbreviation for the package
mountname -- the pkg shortname or an extension of it unique for this universe
##### - discussion -

Desirable properties for packages names:
- SHORT - Pkg names should be short, since they are used as prefix for many variables.
- UNIQUE - Pkg names should be unique since they are prefixes used within common spaces.
- DESCRIPTIVE - Pkg names should be descriptive since they are expected to be used in contexts far removed from their origin and far removed from other usage contexts.

- HIERARCHICAL & DIRECT NAMING - A great many languages encourage multi.level.descriptively.organzied.pkg.hiearchies with full words package names.  This hierarchical organization coupled with descriptive naming provides solid information even across disparate contexts.  The final identifier within a package path name need not be unique.  

- UNIQUE SHORTNAMES - By convention developers often aim to have unique "shortnames" for important identifiers.  The shortname for an identifier is the last element in its fullpath name.
-  work to that that name by itself (without reference to the hirerarhcy above) be (a) unique and (b) globally meaningful.  This is quite helpful since it is cogatively easier to associate a single coehsive naming phrase, than it is to associate the full path of a given package to the meaning of that package. 

- AD HOC ABBREVIATING - A great many languages encourage an adhoc abbreviation naming to achieve the short/unique/descriptive all in one.  They achieve the shortness requirement by mapping these full path names onto some abbreviation for usage within a single file using it.  Thus each usage file will reference that full package name and define an ad-hoc abbreviation specifically for use in this one file.

- AMBIGUITY DOWNSIDE -- Within the code these abbreviations have little apriori meaning for the developers.  Each file defines its own, and they are so short they do not indicate their referent well.  Developers must lookup the mapping (which is typically included at the top of the source file)

- ABBREVIATION REUSE - Knowing this, developers often will develop informal abbreviations for commonly used packages.  Nothing enforces this behavior, and there is no consensus mechanism, so only on the most uses packages does there tend to be an informally agreed upon abbreviation.  (e.g. dt for the datetime package in python)

**ADDITIONAL UNIFORM COMMITMENTS**

- >> CONSENSUS ABBREVIATION - Uniform formalizes this informal abbreviation reuse.

- >> CONSENSUS NAMING UNIQUENESS - Uniform additionally formalizes this unique shortnaming as described here:  By convention uniform packages are coordinated into large groupings of interoperable packages with group-wide unique short names.  The root package that ensures uniqueness of shortnames for its contained packages will include a "wiring block" as part of the package.  This wiring block recursively defines each contained package (including the root) as a package that directly import each of its contained packages.  BEST OF ALL WORLDS:
		- IMPLICIT -- This approach allows one to 'not worry' about the location of a semantic entity.  Since it has a unique shortname one can just use that name directly and unambiguously.
		- EXPLICIT -- One can also be explicit an directly import component as they are used, reinforcing direct locaiton
		- AMBIGUITY TOLERANT -- The implicit only model is not tolerate of any ambiguity in shortnames.  This is often nice, but too harsh to firmly require.  In some cases a very common word (like "Node" is useful to have across incompatible ontologies.)  In those cases explicit prefixing can be used to distinguish cases.  Mixing two groups with unique names can result in naming conflicts, in that case prefixing can be used.
		- SHORTEST DISAMIGUATION --

- RESULT: IMPLICIT IMPORTING -- Following these conventions ensures that important can be performed implicitly once an explicit import of the group itself has been done. 

- out critical naming conflicts.

- RESULT: GRACEFUL DEGRADATION TO RAW TEXT - import statements are entirely derivable from the source code thus they can be automatically added / removed (or presented as if they were there) as desired

- >> ALL CAP ABBR - By convention in Uniform abbr are in ALL_CAPS.  Why?
	In cases of "single usage" there is little reason to use the abbreviation, it is better to refer to a package by its full path, or by its naming identifier since both are descriptive identifiers for the package.  
	In the case that one needs to repeatedly explicitly prefix identifiers with some indicator to indicate its containing package it is imparative that this indicator is short and unambiguous.

	  cannot, or do not want to, leave containing package implicit for repeated useage
#### -- CHOICES --
##### - choice uncore names -
ISSUE unicore semantics MAY differ from upper langauge semantics & 
names will conflict with upper language operators (e.g. 'get')
## === LOG ===
### --- 2021-07-14 - Loading --- 
# ### MENAGERIE ###
## === MARKDOWN ===
See [markdown](./_markdown.md)
## === COLLABORATION ===
### --- CHAIN ---
#### _

**CHAIN** -- A _chain_ is a sequence of _transactions_ whose _finality_ are ensured by some _consensus protocol_.

**TRANSACTION** -- A _transaction_ is bounded unit of data.  

**CONSENSUS PROTOCOL** -- A _consensus protocol_ is a distributed procedure for determining the specific consensus chains -- sequences of transactions that make up the chain at any point in time.

**FINALITY** -- _Finality_ is a claim that some specific sequence of tranactions will always be a prefix of all future consensus chains.  The finality claim might be an economic, pragmatic, or theoretic guarantee.

**CHAIN STATE** -- _Chain state_ is the result of sequentially applying all transactions within some given consensus chain.  Typically transactions are associated with some chain state account, and the value of that account is derived by applying all associated transactions in order.  Thus the chain state is the set of those resulting chain state accounts.
### --- AGGREGATED LIST ---
### --- UNIFORM WHERE ---
## === SOCIETY ===
### --- intro ===
#### --- Manifesto ---
	Mission: Enable human florishing by maximizing value transferred between humans.
	- DRIVER/BLOCKER -- purpose of prefix "enable human florishing" intended as spiritual grounding for subsequent policies.  
		- 
		- blocker -- no good if one maximizes value at net negative in human florishing.
		- SCOPE Note: Statements about benefits (florishing etc.) primarily apply to participants in the society introduce by the mechanism.  That said, consequences for non-participants should not be ignored, and of course genuine opportunity to join society should serves to blunt unfairness .
		- note: no blanket statement that guarantees net-positive happens for all, no blanket statement that measurement of net-utility is simply maximized w/o conseration for the loosers.  Need to balance somehow.
		- note: blocker is not focused only on human finacial consequences, but covers all aspects of "flourshing" as understood by the participants.
	- SUSTAINABLE -- Policies must be sustainable -- short or long term stability of system should be realatively assured by policies 
	- MAX VALUE TRANSFER --

	TIER TWO
	- DIVERSITY & EXPERIMENTATION -- 
	- INTERNALIZING -- Internalizging uncompensated exterrnalities
	- STABALIZING & EMPOWERING -- Balances stability over time against genuine ability for future participants to fashion society as they understand and desire it.
	- 80/20 -- We objectives at any level conflict society strives for 80/20 tradeoffs where some constrained losses against one objective are tolerated for benefit of large but not usually maximal benefit is accrued against another objective.

	TIER THREE
		- A diversity of alternate Consenting con
### --- COMPENSATION ---
#### --- Intents ---
	- BONUS EARLY ADOPTION -- Give incentives to early contributors.
	- SPECULATORS -- Disincentivise speculator market.  Allow contributors to speculate by holding pay in order for it to appreciate in value, but don't allow the transfers which would create a secondary market enabling private equity to scoop up future value.
	- SMOOTH -- smooth out fluctuations against the dollar so holder and workers can have some mild but not assured expectation of short term stability.
	- FIXED CASH BACKING -- Fed aims to maintain targeted cash backing for outstanding coin.  (e.g. climbs to 50% backing, the idea is that )

#### --- THE FED ---
	- AUTOMATED - Centralized automated monetary policy execution engine
	- SMOOTH PRICING - Attempts to "smooth out" short-term exchange rate relative to existing external currency.
		- does this by providing a BUY/SELL spread which it honors as it can, while adjusting both rates as conditions change.
	- REWARD EARLY BUILDERS - Controls long term appreciation (and possibly depreciation) in a way that maximizes long term adoption -- maximizes value transfer.
	- NO SKIMMING -- Assets acquired by the fed in executing its role are only to be used in the execution of its role, and not for any other purpose (no matter how agree upon or beneficial it might be.  See treasury.)

	Fed formula Aims:
	- Fed maintains 1% buy-sell spread.
	- A mostly upward fluctuating min redemption ratio.
	- Fed tries to maintain at least enough USD-liquid assets to buy all outstanding currency at min buy ratio.
	- 
#### --- THE TREASURY --- 
	- MISSION -- To support society's primary mission of enabling participant flourishing by maximizing value transferred.  To support other missions as indicated by consensus.
	- CONSENSUS DRIVEN -- The treasury has 
# ### APPS ###
## === AMAZON PRODUCT LAUNCH ===

- **Heading:** Name the product in a way the reader (i.e., your target customers) will understand.

- **Subheading:** Describe who the market for the product is and what benefit they get. One sentence only underneath the title.

- **Summary:** Give a summary of the product and the benefit. Assume the reader will not read anything else so make this paragraph good.

- **Problem:** Describe the problem your product solves.

- **Solution:** Describe how your product elegantly solves the problem.

- **Quote from You:** A quote from a spokesperson in your company.

- **How to Get Started:** Describe how easy it is to get started.

- **Customer Quote:** Provide a quote from a hypothetical customer that describes how they experienced the benefit.

- Closing and Call to Action: Wrap it up and give pointers where the reader should go next.
## === INFO MANAGER ===
(See [app/TextPuter](./Apps/__TextPuter__.md))
### --- Intro ---
Goal - Present Info Optimally For Task At Hand
What is the purpose of an info management system
AIM: To present the right information in the ideal form for the 	 task at hand with a minimum of preparation overhead.
### --- KEY IDEAS ---

#### -- INFO IS ORGANIZED --
- NAME IT -- Names are key
- PLACE IT -- Everything in its place (one place!)
	- Forces hard choices ; but we can have many links
	- Even places have a place (thus a tree)
- STAGE IT -- hard choices done as user's convenience not tool designers convenience.  
	- lazy is good, Done when required for task, save work on info that will become outdated before use
- VIEW IT -- Many view of singular data
- PROCESS IT -- The reason we organize info is so we can use it!
	means we should develop tools around usage patterns.
	=> find it, read it, scan it, process its items, 


### --- Task At Hand ---
#### SCAN AND PROCESS
- Route to correct place
- Perform action
#### TRANSFER TEXT
#### GATHER
- FIND AND MOD -- Found text / found prior view
#### THINK
### --- Many Live Views Of The Same Data ---
- CONTEXT FILTERING -- 
- DETAIL LEVEL -- 
- MERGE DATA -- Add Lists
- TABLE - FORM - ITEM MODIFIED FORM
- REFLOW DATA - Edits from one push to other view
#### Different Data Views
Flat List 
Tree
Table
Markdown Text
Sparse Text markers
Tools: Ev
## === CHAINED CODE ===
Aim:  Chained Code -- Provide a persistent, interpretable, future-proof substrate for code & data. 
- PERSISTENT -- Availability, constancy, accuracy assured indefinitely
- FUTURE PROOF -- Data/Code expressed in least commitment way so it can be run forever even as languages, frameworks, environments change completely.
- CONSENSUS -- Ensures consensus on agreed version
- BOOTSTRAPPED -- Entire universe is bootstrapped from primal structure and function (RDF triples, and a term-rewriting interpretation model).  100% closed in that all components are built from others within the universe, all the way down to RDF and term-rewriting.
## === SECURE EXECUTION ===
Aim: Provide execution that performs as intended.
SECURITY == certainty we are not tricked.  make it max hard to attach on all fronts.
- Humans can see/analyze/understand code all the way down.
- Provenance is tracked in harsh way.  (every commit of every line)
- Simple -- hard to hide tricks when code is pure and simple
- Existing Infrastructure -- must be consistent with existing tool chains/lang/etc.
	- Build INSIDE those structures.
	- Build compatible hardened "Players"
## === SETUP WIZARD ===
### --- Core Classes ---
#### IO -- A Unit whose value changes under exogenous control
- The GET/SET/sync operators on this unit are tied to external querying and control
- May be push or pull, may cache locally, require sync to get current value.
#### STREAM / HOOK -- Unit whose value is tracked and acted on over time
#### Widget
INTERFACE -- The parameters / operators available
- TYPE -- The kind of widget, and kind of properties/actions it has (list)
- SPEC -- The parameters that define the nature of the widget
- SKIN -- The spec parameters that are inherited from this skinning context 
			(These predefined params have consistent meaning across all widgets)
			VIEW STATE -- Can be shown using several standard views as well as custom views
			Standard views:  Icon, Look, Edit, Author
- CONTENT -- This zero or more parameters represent the "data" exposed 
			by the widget, usually an IO.  (the model part of the MVC) 
- SELECTION -- This zero or more parameters represents the view state or selection
			state of the widget, usually an IO.
			
WIDGET TYPES -- Types that exist in 
- TEXT     	Field / Box
- Button		Push / CheckBox / Toggle / Radio
- Scroll   	Bar / Slider 
- Tree 		Actions: Twisty/Drag/Add/Del/...
			
##### Combo Box Widget example 

def class ComboBox(extends: Widget && Skinnable):		# Type ComboBox 
	choice:  slot IO && Hook && 						# Content -- current choice
				Enumeration(maybe($CHOICES))
	text: 	 slot IO && Hook && Str						# Content -- current text input
	CHOICES: slot List(Sym)								# Spec -- list of legal choices
	default: slot Maybe(Str)							# Spec -- the initial contents of the box (might be a choice)
	focus: 	 slot Bool									# Selection -- is in focus

	render:  Op(->Code)									# Action -- derives the dom structure needed for this widget
	header:	 Op(->Code)									# Action -- derives elements require in header of page used
	
#### Universal Widget Specializations
- HOVER -- Hover show & action
- CLICK / DRAG
- 
### --- Wizard Interface Parts ---
#### PORTAL
Shows textual or graphical view of unit data
- As Uniform markdown
- As Tree of canvas regions for each object
#### PATH BAR
Location within a tree expressed > as > a > path
#### PROGRESS BAR
Flat alternative to path bar for expressing location within a flat list
LIST NAME:  . . . . _ _ /current step\ _ _ . . . 
#### TWISTY TREE
Tree with collapsable subtree
- x
- Path bar (where are you in some space)
- Progress path bar (when are you?  position as place or step in flat list)
- Place/Progress Tree (on left pane)
- Canvas Tree
## === WIZARD II ===

### --- Brass Tacs, Quick Analysis ---

#### -- Parts Needed --
- INSTALLER -- Windows/Mac/Linux+X
- REPO -- 
- UF+PY TEXT LOADER --
- CMD LINE EXEC TOOLS -- 
- DEP TRACKING

- PLACE -- 
- SCHEMA CHECKING --

#### -- GUI Parts --
- TOP -- Main View
- LEFT PLAN -- Tree & Steps
- RIGHT COMPONENTS -- List of available components (Repos; local sys)
- BOTTOM -- OBJ DETAILS PANE -- Bottom


- HINT-ER
- SEQUENCE-FOLLOWER
- CMD-LINE RUNNER -- View Steps; Step; RunAll; EditOne
- TEXT-TREE -- Text/Folding tree viewer.  (select; open/close hilighted)

- STATE SENSOR --
- STATE ACTOR --
- ACTION -- 

#### -- screen regions --
TOP-MIDDLE  -- EMPTY CANVAS	-- 
LEFT-TOP 	-- PLANNING / STATE
RIGHT	 	-- AVAILIABLE LIBS -- Repos, Palettes, Wizards, Components, Local
BOTTOM-LEFT	-- DETAIL BOX
BOTTOM-RIGHT-- CMD LINE


#### -- sys

Layout -- Set of "placed" widgets
Placement -- Left-scr

Display state:
- SYSTEM -- Set of desktops
- DESKTOP -- Set of panes + tree of wizards state
- PANE -- Root of view, many view setting parameters, buttonset, tree of data
- TREE OF DATA -- Tree of rows
- ROW -- Row data, row activation
- ROW ACTIVATION -- highlighted Y/N; open Y/N


ROW info
- row address
- row instance data
- pane view parameters
- row type root   (inheritance; layers; etc.)
_
## === COLLABORATIVE DSL ===
- DATA
	- SCHEMA -- Schema Definition (including fine grained validatation rules)
	- RENDER -- Graphical Renering
	- TEXTUAL -- DSL Specific Pretty printing, parsing, markdown.
	- GUIDES -- Tab completion, Hover docs, Toolbox menu
	- PROVENANCE -- How and why data is as it is.
		- WHY -- Derivation of static inputs explaining why a thing is
		- HOW -- Derivation of dynamic inpust explaining how a thing is

- VERSION -- 
## === MARKET PLACE ===

### --- Grounding ---

Key Properties of Value Transfer
- Knowldge-based Value Transfer usually is win-win (even w/o any payments!)
- Represents a basic human needs -- not just the fruits of the transfer, but the act of transferance itself.
- The transfer takes significant effort from both sides, the author must create, the consumer must understand and integrate.
	- Creates opportunites for gaming on both sides.
- The measure of value a particular tranfer 

Key objectives regarding value transfer
- Both sides of a transfer should be voluntary
- Opportunity to transfer should be maximized in terms of:
	- the total volume that is possible 
	- the total volume that is occuring
	- the number of individuals having significant participation access
	- the total volume occuring with the marketplace
		==> drives marketplace's power to achieve all objectives


Since participation within the marketplace is voluntary, selecting the jurisdiction underwhich one participates is by extension also voluntary.



Market Place 

#### Tier I Jurisdiction



Prefer actions and choices that maximize [benefit obtained from] value transfer unless strongly counter indicated by some tier one consideration listed here.

- Limit gaming of the system.  By changing rules, participants, groups, expectations

- As means to increase benefit, support diversity: of participants, of ways of participating, of value transferred

- As means of increasing value, drive cohesion: in protocols

- Drive benefit primarily but not exclusively for market participants as measured primarily but not exclusively by those individuals.
## === CONSENSUS ===
### --- Intro ---
Aim: 
CONSENSUS -- A version of things arrived at by a consensus decision procedure.

FRACTAL CONSENSUS -- A specific version of things expressed most parsimoniously in terms of other fractal consensus.


_
### --- CONSENSUS TERMS ---
WORK PRODUCT -- Data produced by people, possibly derive from and depending upon other work product.
DERIVED FROM -- 
DEPENDING UPON -- 
SUMMARY -- An organization of some body of work whose author strove towards some stated objective.
BODY OF WORK -- A scope 
OBJECTIVE -- Statement of 
FACTOR -- part of an objective

ALTERNATIVE -- An assertion of intent by the author that A is to be considered an alternative for B

SUPPORT -- An assertion of support by some constituent for some thing.

ANTI-SUPPORT -- 

USAGE -- A measure of how much an thing is being used by constituents


_
### --- CONCENSUS MECHANISM CATEGORIES / DP TYPES ---
#### Hierarchical
One DP relies on sub-DPs to produce its consensus.
-- Ideally with a little transformation as possible
#### Evolutionary
A DP that relies on maintaince of a sequence of population of alternatives each derived from the prior population of alternatives, and a fitness function to determine the emphasis each alternative has in the makeup of the subsequent population. 
-- May select a lead prototype, or representative set

#### Compressive
A kind of evolutionary proceess that pressures klans to gravitate towards common substructure

#### Expansive
A kind of evolutionary process that does not constrain its populations to cohere in strong ways.
### --- CONSENSUS CODE ---
- NODE -- A versioned, controlled document
	- ALT -- An aternative for some other node.
	- MASTER -- The ALT/NODE with the largest vote
	- PLUS -- Support for a node
	- MINUS -- Anti-support for a node
	- CHILD -- 
- PROJ -- A project -- the source code for some construct.  Projects are typically versioned, embedded in a dependency web showing the derivation of the source.
- LIVE PROJ -- a project that is being actively managed by some specified DP according to some stated policy.
- FEATURE PROJ -- a project that ADDS some construct capability onto other project.
- TRUNK PROJ -- a project that has feature projects associated with it.
	- TYPICALLY COMPOSITE -- Defined functionally in terms of the content of some set of optionally features.
	- NOTE: Ideally features may be included a-la-carte, but sometimes constraint limit possible combinations.
	- NOTE: A proj may serve as a trunk for features and itself also be a feature of a larger trunk project.
- TWEAK -- A patch applied to a project in order to allow it to support some feature.
	- BREAKING TWEAK -- A breaking tweak requires changes to multiple other features in order to operate.
	- NON-BREAKING TWEAK -- A non-breaking patch requiring changes to other features in order for this feature to operate.
	- CLEAN TWEAK -- A patch that is simple in that its affects on the semantics of the original code are easy to understand.  (E.g. adding a field to a structure)
- ???ATOMIC PROJ -- a project that only depends upon prior versions of itself.
- ???CONTANER PROJ -- projects tha
- TRUNK PROJ -- commits only depend upon prior head of this branch.

IDEA:
- GROUND UP -- anyone can add any feature to any project.
- FOCUSING -- The trunk project owner need not (but may) attend to these features as they are created.
- GUIDANCE -- The trunk owner may provide guidance for the feature writers.  Feature writers need not heed this guidance.
- WEIGHT -- Each feature gains weight based on its downstream usage.
- TRUNK SPLIT -- when a set of breaking tweaks are not resolved as the code base is cleaved into to seperate incompatible versions.
- SPANING FEATURE -- a feature that has been adapted to operate on more than one branch of a split trunk
- TRUNK SCORING -- 
	- A trunk's score is maximized when all feature weight is on the non-tweaked master.
	- ??? When all deltas are pushed into the features (not left as tweaks of its features)
	- If a trunk is split spanning feature's weight counts against each project if the spanning feature require tweaks on eaither side.
NOTICE:
- system allows bottom up feature work, and only requires trunk holders to notice feature workers as they gain weight.
- system pushes all to support tweak free combinations
## === BROWSER APP ===

### --- Browser Parts ---
- STATE -- The full state of the browsing system as tree
- WIDGET -- Resizable, Scrollable, Screen Region
- ELETEXT -- Electric Text Window
- ELETREE -- An Electric Tree Window
- ELECANVAS -- An Electric Canvas Window
- LAYOUT -- Recursive rows and columns of widgets
- SKIN -- A declarish layer deriving parameters for everything


### --- Browser Semantics ---

ARGS -- Full browser state expressed as a single tree of values.
- CONFIG -- Args defined before browser begins runnings.
- VIEW -- Args under direct control of user during browsings.
- EXPR -- Expr used to derive arg delta
	- STEP EXPR -- used 

BROWSER -- An interactive interface used to visually present and interactively manipulate an instance of data model according to the semantics and skinning associated with that model.

WIZARD -- A guide providing a directed browsing experience, aimed at a specific information capture/exposition goal.
- STEPS -- Wizard is expressed as a tree of steps with logic determine step transitions, and browser configuration at each step.





- CURSOR -- the "place" where the browser is currently pointing
	- LEX PATH -- the path "place" in lexspace where cursor is.
	- SORUCE -- the file/character "place" that is the source.
	- GUI SPOT -- the GUI element "place" where the cursor is.
	
- WIZARD CURSOR -- the "place" within the recursive wizard tutorial executions currently controlling the browser.

- MODES
	- USER -- Just under total control of the user. 
		(a null 'skin-only' wizard is undercontrol.)
	- WIZARD -- Interface controlled by current step in wizard
	- AUTHORING -- A wizard used to author other wizards.
## === DO WHAT YOU SAY YOU WILL ===

FUNCTIONAL TRANSPARENCY -- Ability to see the code being executed behind some function computed function.


STEPS
- Create account
- Fund account


HONEST LAUNCHER
- Posts its pub-key on chain advertising its service.
- Others post thunk to run along w. encrypted payment.
- Honest launcher launches lambda service with URL tied to hash of thunk.



_
## OLD BROWSER TEXT

CURSOR              The selected widget

IO
HOOK                A data anchor (specified relative to some parent hook by its hookspec)
                    HOOKSPEC -- A spec for one unit relative to its parent (same as refspec)
ORIGIN              x,y  dx,dy  top,left,bottom,right

CONTROLLABLE
--  Draggable
--  Droppable
--  Clickable
--  Typable


TEXTUAL


GRAPHICAL           A VISUAL ELEMENT  (WIDGET)
-- hook             Place in 'data' that serve as root for this port
-- anchor           Place on the display where element is "anchored"
-- extent           bounding box for visual element
                    both anchor & extent are expressed relative to the 'anchor/extent' for its parent element
                    (all coord derived from data under this element are relative to this anchor


SKIN   (specific configuration of its rendering bi-putation)
--  constellation of parameters for use across all IOs
--  plus specific skinning choices chosen for each instance from
    discrete parameterized types associated with each type


ACTIONS
-- BUST             Applies to the refspec of the inner most widget with an active ref
   BUST OUT         Key, Mouse?, Click in top left corner
   BUST IN          Double click on widget
   BUST DRAGGING    Drag
-- SELECT           Click cursor to a widget
-- PEEL             Key, Click bottom left corner, Show data "behind" a widget
NAVIGATION
-- Zoom in/out      Move view port





#########################
###   COMBO-WIDGETS   ###
#########################

OUTLINE VIEW        Vertical rendering of a tree w. indention to the right representing tree depth
                    Each unit is a single line, w. graphical and textual elements shown there
BOX VIEW            Each unit is a rectangle w. subunits as contained rectangles
                    Properties of unit are displayed as IOs in within the box
VIEWPORT            Scrollable canvas with the region of the unit




###############################
###   C O M P O N E N T S   ###
###############################


GUI_ITEM
- name   --  Identifier for item
- Label  --  <80 char description
-


CONTAINER --

UNIT CANVAS -- Fixed sized rectangle with subunits in specified locations

TREE --
 * Drag n drop re-ordering




#######################
###   V I E W E R   ###
#######################

Roadmap         Tree of steps each with:  layout, preq, next/prev


- Layout        Structure composed of displayed items

CURSOR
- Breadcrumbs:  RootName subfolder names within the hierarchy
- Tree:         Foldable Tree view of the space
- Item View:    View of single item
# ### OLD ###
### >>> OLDER TEXT -- NODE-CENTRIC ACCESS PATTERNS

- **bounded(u)** -- u _is bounded_ iff _items(u)_ is finite.
- **ordered(u)** -- u is ordered iff _items(u)_ has well defined ordering
- **functional(u,k)** -- _functional(u, k)_ iff there is at most one pair (x, y) in items(u) where x == k.
- **functional(u)** - u _is functional_ iff forall k in P functional(u,k)
- **infinite(u)** -- Indicates that a unit is not finite.
- **unordered(u)** -- Indicates that a unit is not ordered.
- **relational(u)** -- Indicates that unit is not functional.

- **key** and **value** terms used to refer to the 1st and 2nd parts of each (ki, vi) pair within the _items(u)_.
- **keys(u)** is the sequence of keys in the items(u) pairs.
- **values(u)** is the sequence of values in the items(u) pairs.
- **child** -- we say c is a child of u iff c in values(u)
- **contains(u,d)** -- we say u _contains_ d iff d is a child of u or
	there exists c where c is a child of u and c contains d.
- **get(u,k)** -- we say _u get k is v_ iff _u_ is functional and _k,v_ in items(_u_).  This is also written as **u[k] == v**


XXX Each graph vertex can be seen as a handle or container for the set of verticies linked from its out going edges.  These vocabulary terms build from this perspective.

u.is.Tree
place.is.Tree

Indexed -- A vertex is indexed if each 
### -- OLDER JUNK --
old place space stuff
		
p contains some q that is the placement of 

Each vertex and edge in the original graph can then be mapped to exactly one location vertex in this location graph tree.  Since these locations form a containment hiearchy such edges and verticies are thus contained in all parent locations of most specific containing location.

Assuming the edges in a graph might be mutable, we can treat any edge within a graph as a place where the edge's value is the store of the most recently assigned vertex.

**PLACE SPACE** -- A single static ordered, functional tree of all places.
- Each place space vertex has a parent vertex, and a countably infinite set of child verticies connected by edges that are labelled with each possible vertex.

mapping of each vertex in the graph onto exactly one place vertex in place space.

**XXX CONTAINS** -- The containement vocabulary for place trees is extended to include these placements.  So a place, p, contains a graph vertex, g, iff p is the placement of g, or there exists some descendent of p is the placement of g.


~-~-~
		
		origin.path_get
1. There exists a total ordering, T={A1, ..., An}, of all access and assignment calls in all of its place timelines such that the relative ordering of those access and assignment calls are consistent with each of the individual place timelines.
2. There exists a sequence of coherance graphs, C0, ..., Cn that are inductively constructed by applying the Ai operation to the Ci-1 graph to obtain the Ci graph.
3. And the value obtained by each access operation is equal 




Value entirely within one place



 having a one-to-one mapping with the edges of an underlying coherence graph.
#### notes

**ASSIGNMENT SEMANTICS** -- The idea of assigning a value to a place, such that later accesses to that place yield a value equal to the most recently assigned value for that place.

**ATOM** -- A vertex with no structure that is only equal to itself



ACCESS(place) -> value
ASSIGN(place, value) -> place      # might become new place
EQUAL(value1, value2) -> boolean   

**SHORT HAND NOTATIONS**
	place				# obtain value from place	
	place = value		# assign value to place
	value1 == value2	# test the equality of two values


>> See Unicore Assignment Semantics section for more info
#### older
PERSISTENCE -- 
**ASSIGNMENT SEMANTICS** -- A construct providing persistance: data values, and placeholders that may be assigned a value, and accessed to obtain the most recently assigned value.

PLACE -- A single edge within a graphical model.
ACCESS -- Obtaining the node value associated with this place.
ASSIGN -- Changing the value node for this place's edge.

PARENT -- SUBJECT   -- The 1st position for this place's triple.
KEY	   -- PREDICATE -- The 2nd position for this place's triple.
VALUE  -- OBJECT    -- The 3rd position for this place's triple.

~-~~-~
Placement -- Creating structures of place ???

place -- Returns the place where this unit is.

Each placed unit is in exactly one place at any given time.

PLACESPACE -- A 





_
### === older ===
#### older
	A graph, G, is composed of a set of nodes, where each unit, u, has a set of edges that connect it to other nodes, v1, v2, ... that are called the 'values' of the unit.

	The 'space' of a unit is the units and edges included in its transitive closure.


	ITEM(value: key: nth: self:)

	_
#### --- types 

	-KEY -ORD	MultiSet	Set
	-KEY +ORD	List		OrderedSet
	+KEY -ORD	Map			Bijection
	+KEY +ORD	OrderedMap	OrderedBijection

	C
	DAG/Tree	Are cross cutting constraints on the global structure
	Fixed/Finite	Are cross cutt
#### --- notes ---
	RDF
	UNF Uniform Node Form
	Uniform R Form
	Node Centric Form


	Ordered		
	Labelled
	Bagy		is linked; no order or labels

	Restrictions
	sety		unique targets
	functional	unique keys
	DAGy		non-circular
	Taxy		partitioning (tree)
	graphy		no restriction
	



	Mappy/- (generalize key)   Listy/- (ordered vals)  Unique/- 

	--	Bag		Set			    <u _ r> <r UP u> <r 0 _> <r 1 v>	
	-L	List	OrderedSet		<u i r> <r UP u> <r 0 i> <r 1 v>
	M-	Map		Bijection		<u _ r> <r UP u> <r 0 k> <r 1 v>
	ML	OrdMap	OrdBijection	<u i r> <r UP u> <r 0 k> <r 1 v>



	Map/Ord/Pinned  Rel/Fn(getable)  Dups/Uniques  

	MFD	Map			<u k v>			// many-to-one
	MFU	Bijection	<u k v> 		// one-to-one
	MRD Relation	<u k vi>		// many-to-many  
	MRU	OneToMany	<u k vi> 		// one-to-many  (inverted map)
	OFD	List		<u i v>			// (also ordered dict)
	OFU	OrderedSet	<u i v>
	ORD MultiSet	<u i vi>		// xxx  MultiSet
	ORU	Set			<u i v>			// xxx  Set
	PFU Singleton   <u _ v>			// xxx
	PFD Singleton	<u _ v>			// xxx
	PRD MultiSet    <u _ vi>
	PRU Set			<u _ vi>

	Container Promises
	- UNIQUE 	- is set	- No two value elements are equal
	- ORDERED 	- can slice - Ele have specifiable stable order
	- FUNCTIONAL - can Get 	- Keys associate to at most one value

	URF Container promises
	* FUNCTIONAL
	* ORDERED