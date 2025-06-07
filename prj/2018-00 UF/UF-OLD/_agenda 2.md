### PUT A HANDLE ON IT
Generally any time a programmer thinks about a collection of concerns as a group, there should be a SINGLE HANDLE to refer to that group.

For example "the current state of execution" is a kind of collection -- Uniform should provide a single handle for this.

"the configuration of the program"
"the change in state after executing x"
"the history of values for variable x while executing y"
"that data accessible by function z in context y"

_
## to place

~~~~
EMBEDDABLE -- parametric restrictions

JSON is not embeddable in any language since it has no limits at all on the size of its integers.  This is generally not tennable for concrete languages.

but appropriate parametric restriction will allow it to embed.

~~~~
CROWD EXTENSIBLE -- A framework is crowd extensible if multiple parties can work extend the framework with orthogonal capabilities without any coordination, then merge results into a new framework with all capabilities.
==> Merge step should proceed with no/minimal refactoring except in cases where the frameworks themselves are adding capabilities which themselves conflict (e.g. adding operators/tokens with conflicting meaning)

~~~~
ESSENTIAL COVERING -- A spiralled rung that:
- where each part is necessary.
- where the rung is sufficient, 
- well spaced -- no part defined above is better placed in rung; no part of rung better placed above

~~~~
PKG -- Declarish source-code componentiation
	NS -- Scoping -- Static Scoping
VAR -- Object-slot assignment semantics
	Templated slot access
CTX -- Environment contextualization -- definining inside as function of outside

~~~~ 
SINGLE ENCLOSING FRAMING --

when possible one should always define a single enclosing frame for incompatible  formalisms.  

Even when it seems the perponderance of utility is to be found within the distinct sub areas.  

Why?   Building a single framing for each alternative provides a smooth incompatibility-free generalization trajectory for each of these framed constructs.

Further, even if most value is in each area, still getting extra compatible value w/o cost is worth it.  and not running into generalizaion walls it good.

closure vars.  dynamic shallow binding vars.  ==> dyn closure vars
list groups. map groups.  ==> map/list groups

~~~~
FLOATING -- An framework is floating if it is defined in relative isolation.

That is it is defined in terms of some low-level background vocabulary but without reference to any of the higher-level vocabulary used to implement it.  Except for it reliance on the low-level vocabulary its definition is completely unconstrained by other high-level vocabulary, even those used to implement it.

==> NO BOILERPLATE 

FRAMEWORK is a cohesive vocabulary appropriate for some given task area.
## to place2
Reasons:  Cake.  White space.  Upstream  CTx  pay. Form  micro. Tyranny of authorship.  Roll own plus done for u
~~~~
What - Who - Why
~~~~
- Sharing happens when it does not need to be translated.
- The Densest Form of information for human comprehension is textual.
- The kernel of value creation work relative to the social boilerplate is typically quite small.
- Removing the boilerplate results in large multiplier improvments.
- Chooseing the right representation make all the difference.  Even small simplification differences at the bottom make large differences in ability to understand and process complext systems.
- building in mid-air make all the difference, you can build on the exact required
- foundation, no more, no less.
- pro. on time. 
## === PLAN ===
### --- VISION ---
Spending 15-25hrs/wk, I will be able to show uniform bfore the baby is born.
I will grow this into a community developing Uniform and eventually into some revenue stream.
#### Short Term Goal
	I want to be able to show Uniform before baby is born, spending 15-25hr/wk
Be able to show Uniform:
-- (1)Agenda+ (2)Markdown+ (3)Unicore+ (4)Uniform (5)Collab (6)Mkt (7)Society.
	Slides, Video, API
-- Markdown
?? Unicore ref
?? Text xlator
# ### OVERVIEWS ###
## === TALKS / INTROS / SUMMARIES ===
### --- DRY LANG ---

DRY INTRO -- Dont Repeat Yourself 
- Good SW engineers know to refactor their code so that common parts are only defined once and repeatedly instantiated.
- Good SW engineers get this, but language researchers didn't get the memo!
- Modern SW is requries combining multiple DSLs
	- This is ideal, knowlege work within a specific domain should be done in a language adapted to that domain.
	- But spanning between these DSLs introduces great complexity since each DSL is often a universe unto itself; a great many basic SW concepts are incompatibly framed, introducing enourmous complexity in conversions, breakages, etc.
	- DSLs are of enoumous benefit in simplifiying code, complexity etc.
		But they are a double-edged sword since each new DSL potentially adds an N^2 complexity to a SW project as it interacts w. 
- WHAT IS GOING ON HERE?
	SW engineers know to dry their code, but lang researchers never got the memo


We all know: no module writer however good, can salvage a system, if required to build within a poor system architecture, 
- DSL and framework and library writers are doomed before they begin
	either they must create significant interoperabilty failures or
	they must use infrastructure poorly constructed for thier needs
 - and they are doomed to be 
_
### --- CAN/SHOULD BUILD COMBINABLE/EMBEDABLE FRAMEWORKS ---
**SO WHAT IS THE POINT?**

- Knowlege work is cognitively easiest and most effective when performed within DSLs specifically designed for that work.

- Independently developed DSLs typically contain incompatible forks in the way they formalize aspects of representation and computation. Thus combining them becomes so prohibitive that complex sofware generally tries to minimize the number of DSLs used because of the N^2 compatibility issues.

- but according to the claims above all of this can be avoided.  One can have an ecosystem of constrcuts that form a nesting, covering, power spiral.


- This would make all the difference.  Forks are deadly, each single fork has the potential to divide all future human-work built upon its branches by N.  Worst still forks can interaction to exponentially dilute the the value of subsequent work on addressing the space of all computing tasks.

But one cannot "Bolt" our desired simplicity onto an existing complex ediface.  One must build a non-forking, essential, power-spiral from the ground up.

... and building it is very hard.  One needs to consider refactorings of the entire spiral and its exponential ways that constructs can be composed.  Still it can be done incrementally, and humanity only needs to do it once.





#### FORK IS A FOUR LETTER WORD

The practical consequences of these two types of splits could not be more stark.  A forking split exponentially diminishes the value of subsequent programming efforts while intrinsic splits are non-diminishing.  This means extra programming effort required to cover some class of targeted tasks grows exponentially in the number of orthogonal forking splits, but does not grow at all given pure intrinsic splits.

~~~
A purely intrinsic split provides a specific additional constraint and then leverages this additional constraint in order to provide some additional capability.  The beauty of this split is that any additional capabilities (splits or programs) built on top of this split must specifically depend upon this additional capabilty, otherwise these operators would be defined at the level above since they do not need the constraint provided.  So 

_
#### PRACTICAL CONSEQUENCES

When we consider the splits between practical frameworks and programming languages it often is the case their splits are a mixture of the two of these types of splits.  Unfortunately the practical consequences of these mixed splits  is more similar to forking splits than they are to intrinsic splits -- they are still exponentially diminishing.

The practical solution for these complex split is to decompose them finely enough that one can separate out a set of purely intrinsic splits.  These splits can then be reformed into a single dependency tree of intrinsic splits.  This edifice provides the basis for a single language with compatible alternatives that are each built upon the smallest possible set of dependent concepts.  The result is a framework where non-diminishing programming can be done, and a singular language with that retains the full range of power posessed by all starting languages.  
_
##### use (1) essentialization, (2) incrementally minimal , and (3) power spiraling as a guide

- incrementally minimal also limits the translational complexity (require when translating across split boundary)

Knowning that decomposing 

 which depend upon splits 






NOTE:  Technically  it is possible to believe a split it intrisic, but later discover that one can unify both sides w/o loss in capability.  As a matter of experience we don't expect to often find as yet undiscovered unifications in the well trodden areas of 

data words occuring in lower addresses first) verses 



 







and forking.
_
#### WHERE DO INCOMATIBILITIES COME FROM?

To understand this approach it is helpful to first understand the nature of incompatibility itself, since this is the sin we hope to avoid.

Q:  So where do incompatibilites come from from?
A:  They occur when we make a commitment that precludes valuable possibilities that are inconsistent with those commitments.

Q:  But don't ALL commitments preclude something?!?
A:  No, _definitional_ and _intrinsic_ constraints both provide useful structure w/o precluding alternatives.


So **definitional** commitments provide scaffolding that may be (a) appropriate and useful, or (b) irrelevant but also not constraining.  For example, we might define a _tree_ as a singly connect graph.  This is useful for certain frameworks, but if it is not useful, one can safely ignore this defined term.

A second case that cannot cause incompatiblity is building upon **instrinsic** commitments.  These are commitments that stem from the mathematical nature of computation itself, or that stem from the properties of the physical universe that we inhabit.  These commitments apply equally to all software languages, thus building from these commitments also cannot cause incompatibilities.  For example, tradeoff between access and update times for linked-lists vs hash tables are a tradeoff that exists in all programming languages that have these structures.  The space and time constraints of these structures stem from constraints inherent to all von-neumann architectures, which are in turn inherent constraints drawn from our physical universe.  Languages that commit to these constraints will not be in conflict since they are only accepting constraints forced upon all languages.


So then what kinds of constraints do cause these incompatibilities?  We call them **_unforced errors_** harkening to the baseball term where points are lost "for no good reason" but rather instead simply through bad play.

In our context an unforced error occurs anytime one builds in a way that unnecessarily constrains the users of the component.  For example, 

~~~~~


, and we might define the relation connected_to by recursive traversal of that tree

DAG as an acyclic graph, then we might define iterative-deepening-traversal using this DAG definition.
These definitions do not create a tradeoff.  In the case that we need an iteratively deepening traversal we requre some acylic structure to perform it over.  If we don't need such a traversal, then both definitions may be safely ignored without creating any tradeoff.

But notice if instead, one also wanted pre-order traversals, so one defined a DAG as having an ordering over its edges, we would have 

one implemented a tree using directed edges (or undirected edges) one would have overstepped!  Either implementation would be unacceptably constraining, since there would be valid use cases not covered by either choice.


DEFINE forced vs unforced tradeoff


**So we see the crack in the door that we hope to slip through:**

We want to create a set of defining terms that are so general, so tautological, that they either apply to a use case without conflict or they are not relevant to that case.  Any capability provided must express its requirements in their weakest form, there can be no cases where such capabilities might apply, but are disallowed by overly restrictive requirements.

Notice:  It is ok, if a definition is irrelevant, e.g. the existence of "tree" in your language does not conflict with using string.  It only conflicts if you need a INCOMPATIBLE definition of tree.

_
#### EXAMPLES

Completeness Thesis Statements

GRAPH ENCODING SUFFICIENCY -- All symbolic data may be naturally expressed as directed graphs with possibly ordered, possibly labelled edges.

REWRITE SYSTEM SUFFICIENCY -- 

NESTED CONTROL -- WLOG we may restrict all control flow a nesting paradigm comprised of the following five flow primatives: sequencing, branching, chaining, looping, and returning.

SEMANTICS ASSEMBLY SUFFICIENCY -- Any paradigm, P, developed on a von-neumann architecture can be naturally mimmicked by a system, P', built from Unicore's semantics assembly along with appropriate atomic types.
_
#### AVOIDING TRADEOFFS - A PRACTICAL MEASURE

Perhaps it might theoretically possible to avoid all false forks as listed above.  Perhaps it is theoretically possible to wholistically and iteratively refactor the entire spiral of all constructs required for all software and all software languages as applied to all relevant target tasks.

But saying this agenda sounds enirely impractical still sounds like a understandment of the situation.  Holy F********** this seems too stupidly complex to consider!

to be just a tad impractical.

unforced tradeoffs making progress in this way seems entirely 

Perhaps that would be true if we did not have several powerful techniques at our disposal to drive this progress.


Further, the space of ways that definitions and algorithmic capacities combine to form a framework's web is incomprehensibly large.


LESS DETAIL / LESS TO TRIP OVER -- 


DECOMPLECTING --


SPIRALING --


POWERSET COMBINATORICS -- 


PRACTICAL vs. THEORETICAL -- 


OUR FLASHLIGHT -- We have a simple flashlight 


_
#### OLD
##### OLD V1 INTRO
What if there were a programming approach that yielded all advantages of all other approaches, and avoided all limitations except those imposed by the mathematical nature of computation itself?

Well I guess that would be transformatively great, but since it is totally impossible, why talk about it?  Turns out this is not impossible, but it does require thinking very differently about how software is constructed.

_how 
### --- HACKERS TALK ---

Greenspun's Tenth Law -- Every sufficiently complex program has a Lisp interpreter written in it... Badly.

So let provide the components to support interoperable, beautiful DSLs for for all forms of interpretational data.



I AM A	-- Closet Lisper, SO

I THINK -- Everything should be meta-programmed, BUT

I ADMIT -- A screen full of parens sucks, SO

I BUILT	-- "Code Markdown"

- Looks like python / java / YAML  (and C++, prolog, PHP, ...)
- Parses to JSON  (my LISP's "S-EXPR")
- Semantics-free reader

I DESINGED -- "Semantic Assembly"

- 8 code instructions & 10 data instructions 
- An elegent set to meta-program the rest



Developed "Code Markdown"
Lang Independe  Text ==> JSON  reader

Developed "Semantic Assembly"


- LISPER -- As a closet LISPER I am convinced that we should all programming environments should be meta-extensions of some UR-language underneight it all.

- HUMAN BEING -- I have to admit the the parenthesis hell that results from any LISP extension is less than ideal.

- CODE MARKDOWN -- So I have developed a "code-markdown"
It is sort of pyhonic, Javaic, c-ish, Yamalic stuff
It is semantics free like the LISP reader -- e.g. its parsing rules work the same for all languages, there are not 

- SEMATNIC ASSEMBLY -- trying to list the computational primatives are common to all procedural languages.


MD Examples:  HTML template; DB migration; Kubernettes scaling 
rewrite rule


- LISPER who thinks SW should be written in collection of domain specialized languages where both syntax and semantics optomized for each domain.

- And these specialized languages are meta programmed from a common computational primatives so the languages interoperate seemlessly.

- Developed a "code markdown" that looks like Python, Java, or C++, but parses semantics free into JSON.

- Developing a set of computational primatives 


- everybody should be programming in hyper-specialized meta-programmed languages that are build ground-up to be deeply interoperable.

#### Examples

// Cloud topology rule

where latency($MemCacheNode, $ControllerNode) < .015





// 

// Algebra Solver
constraint_set:
	    x * y = z + 3
	    x > 0, y > 0, z > 0
	    x ** 2 + y ** 2 = z ** 2


//  Rewrite rules with both template vars and rewrite variable
rules:
TRANSITIVE(Friend);  REFLEXIVE(Friend)
$TERM(?x, ?z) :- $TERM(?x, ?y) && $TERM(?y, ?z) &&     
	    TRANSITIVE($TERM)
$TERM(?y, ?x) :- $TERM(?x, ?y) && REFLEXIVE($TERM)

Friend(Dan, Dave)     // implies Friend(Dave, Dan)


//  Context Free Grammar
CFG:
SENTENCE ::= NP VP
NP ::= [the|a|an|]  NOUN  [PP]...
VP ::= VERB

	 




#### Programming is hard, but for dumb avoidable reasons.
Hour by hour most time, effort, and complexity in programming stems from learning-about, dealing-with, and overcomming complexity that is needlessly introduced by pointless variation.

#### I imagine an alternative ... a JSON for computation.
_JSON -so simple it can't be simpler -maps everywhere w/o conflict_
- The JSON data format is easy because it is so simple, it couldn't be made simpler.  Thus it contains few details which might conflict when mapped.

What is the JSON of computation?  
Building Blocks so simple they could not be simpler.

What would it look like?
Here are a few proposed building blocks.

(1)  TRANSLATABLE -- Straightforward mapping to most lang w. str
(2)  RIGHT LEVEL -- Result is natural
(3)  COMPLETE -- If you have some structured data, there likely 			will be a reasonable JSON encoding.

#### Examples
- RDF -- Graphical data models
- Control Flow -- BLK, BRA, CHN, REP, RET
- Addressing -- 

#### Where I am at
- Periodic table of computational elements
- Layered model of computation
- A reference implementation of some elements


**SPIRAL-1 -- GRAPHICAL DATA MODEL**
**SPIRAL-2 -- NODE-CENTRIC ACCESSORS**
**SPIRAL-3 -- FUNDAMENTAL DATA STRUCTURING PATTERNS**
**SPIRAL-4 -- INTERPRETATION**
**SPIRAL-5 -- SEMANTIC ASSEMBLY LANGUAGE INSTRUCTIONS**
**SPIRAL-6 -- META-PROGRAMMING -- LANGUAGE-WRITING-LANGUAGE**
**SPIRAL-7 -- MENAGERIE OF SHARED CONSTRUCTS** 

**SPIRAL-8 -- ALL SW LANGAUGES AND FRAMEWORKS**
### --- JSON INTRO ---
#### A JSON-LIKE FORMAT FOR CODE?

JUST AS JSON IS A KIND OF LCD FOR DATA
WHAT MIGHT BE A LCD FOR COMUPTATION?

- Any block of bytes is sufficent for encoding digital data.  But a block of bytes is not a natural form for expressing the structure inherent in data -- its at the wrong level.

- JSON, on the other hand, is a both a minimal AND practical form for encoding structured data.

- Over the decades we have developed a mind-numbing zoo of alternative encoding each with its own little bell or whistle, but it you just needed a one-size-fits-all pragmatic way to share structured data -- JSON aint a bad choice.

- So what about computation?  Suppose I have some computation I want to share?  Suppose I am tired of the endless variations of bells and whistles, and I want to just encode in the one size-fits-all simplest pragmatic encoding.  What do I use?

- A turing-machine?  That's kinda like a block of bytes.  Sufficient, yet not practical, not capturing the essence, and not at the right level for most of what is impt to share.

#### WHAT MIGHT A PRACTICAL/MINIMAL MODEL OF COMPUTATION LOOK LIKE?

It is too big to be (1) elegant (2) complete & (3) at right level

Instead imagine a SET of CONSTRUCTS that are:


(1) ESSENTIAL -- Each construct itself is so simple it could not be made any simpler.

(2) INTEROPERABLE -- The range of more complex computational constructs are naturally/practically constructable from mix and match combinations drawn from the powerset of construct combinations.

(3) SPIRALED -- Instead of trying to choose the one right level of  encoding computation, we encode it as a spiral on constructs.  Each spiral is complete in that it is sufficient for constructing all spirals above, and is parsimonious -- having a small number of components, relying instead on the powerset of combinations for generativity.

#### IF YOU COULD DO THIS CRAZY THING WHO WOULD CARE?

If it is like JSON, each construct would be simpler than existing languages, yet still at the right level to get work done.

It would be a great collaboration language, great for code-like interchange.  

Once data is encoded in JSON it can easily be mapped into 100 other languages and frameworks.

Once code is encoded in Uniform it could easily be mapped into 100 other languges too.

Likewise coding in uniform would be coding in many languages all at once.
### --- THE "WHAT IF" INTRO ---
	as you hear these impossibilities you will move from incredulity to deflation.
	Each time feeling that the solution is a "cheat"
	In a sense you are right, each solution is a cheat, but a cheat that works!
	For each one ask yourself:  does this cheat at a practical level provide much of the advantages of all of the "contradictory" sides?
	Do most languages today already do all these things all at once?

	If you agree with both, then you agree these are "cheats" worth doing!

	What if you could could combine the advantages of all typing systems into a single typing system?  you can.

	What if you could have all advantages of most all syntaxes all at once?  you can.

#### --- both big and small; both simple and complex; at the same time 
	What if you could have the completeness of a langauge that contains all combinations available in the most complex and most complete languages without loosing the lightweight b

	-- nesting extending definitions
	-- blind dependencies
	-- power set decompositions

#### Is your language so extensible that you can add something as profound as lazy evaluation as an importable module?  Uniform is.


#### Is your memory management so flexible that one could add rust-like, compile time, reference tracking to its built in equal operator?  Uniform can.


#### And if you did that w. your language has your language abstracted varialbe in a way that you could correctly pass rust references as arguments to a standard language method.

#### Is your language's reflection detailed enough to allow you to capture the history for a variable.  thats a one liner in Uniform.

#### Can you instruct your language to keep track of the derivation of a given value within a running program?  

#### Could your write a simple script to compare two executions of your code with and use the delta to detail the find the causal path from one changed value outward to a differen



#### Is your language so close to the metal that it can compile "malloc free" code for execution in high performance embedded contexts.  Uniform can.


#### Does your language decomplect persistence from structure in a way that allows 



#### Does your langauge decomlect X from Y so that extensions to X can be combined with extensions to Y.  Uniform does this for many X and Y.


X=persistence.  Y=access.

#### Is your language's parser so flexible that you can extend it with 64 punctuation combinations to inject the full set of APL operators into your existing infix operator notion scheme.  Uniform parsing is.


#### does you language allow you to operate on all places in all ways.


#### does your language allow you to use the same code to operate both finite and infinite structures.  Uniform does


#### could you trivially pair back all looping constructs in your language so it could be transpiled onto the bitcoin non-looping contract language.  Trivially done in Uniform.

#### 


#### All advantages of all languages in a single language.
	What if you could have all the advantages of all langs in one lang?  you can...

	work from the outside in.  

	the computer scientist has the mantra of beginning with the important aspects of problem class you hope to addres, and use those as the central architeure for what is being built.

	In uniform our approach is in almost direct opposition to this.  We work from the outside inward, constructing our language as a mathematican might construct a proof by contradiction.  At each stage we create structure, but in a way that strives to not exclude any use case, any usage context, for any goal, from any starting point.

	Just as with the mathmatician we might contruct a point or term which does not have relvance for one case or another... that is ok.  but we strive to only make assumptions that are vaccouly true, e.g. they are not really assumptions or constraints at all.

	The mathematician uses the term WLOG (without loss of generality) as a shorthand for saying I am adding this thing here, but in adding this thing, I have not inadverdently restricted the universe being covered here.

	The mathematician needs to preserve this generality so that when a contradiction is found she can be sure the original negated conclusion is what must be inconsistent, the contradiction is not coming from some reduction in generality.  In our case we preserve this generality so that all downstream extensions of our work will continue to apply in all cases they could apply to.  

~~~~~~

	never make an assumption which is 

	Only allow the barest of formulations; accept only the coarsest simplest statements whose refutation quickly leads to absurdity as the basis for ones edifice.  

	Surprisingly with diligence one can make progress with these weak beginnings, the result is a powerful cumulative effect.  One slowly accrues capabilities and advantages as one covers the ground covered by this feature or that, but by refusing to build from devisive or exclusionary decisions one accrues limitations at a much slower rate.

	This sounds like a cheat how can one gain the advantages of constraint w/o the constratin of constraint?  


	try working from the barest of assumptions... assumptions who negation seem to preclude the very nature of the thing you strive to capture.
### --- THE "Uniformity Thesis"


POWER SPIRAL -- We argue it is possible to arrange these constructs within a _power spiral_---a progression of "rings" where each ring is a set of constructs.

 (like integer, lexical scope, control flow, data repository)
 where the constructs of each ring

.  Further these individual advantages may be "mixed and matched" in most any combination that to the degree permitted by the inherent nature of the underlying concepts.

 _
## === AGENDA ===
### --- AGENDA OVERVIEW ---
### --- SPIRALS ---

SPIRALING THESIS -- 
- Each spiral can be encoded using nothing more than the interface provided by the previous spirals.  


**SPIRAL-1 -- GRAPHICAL DATA MODEL**
		RDF:  triple/URI
		URF:  atoms/composites/items/unit-form
				
Q:  What is an elegant, sufficient data form for all SW?
A:	Graphical Data Models
				
**SPIRAL-2 -- NODE-CENTRIC ACCESSORS**
		get/set/iterate/invert/create/delete
		bounded/unbounded  ordered/unordered functional/relational	mutable/immutable

Q:	What is a simplest, sufficient first-level access model?
A:	A node-centric access model
		
**SPIRAL-3 -- FUNDAMENTAL DATA STRUCTURING PATTERNS**
		contains/strcuture_of/graphs_of
		list-ish:  slice/append/+
		tree-ish:  contain/path/acylic/taxonomic/+
		graph-ish: from spiral 2

Q:  What is a pragmatic starter set of data structures?
A:  list-ish, tree-ish, graph-ish
				
**SPIRAL-4 -- INTERPRETATION**
		Term rewriting system / alpha interpretation

Q:  What is a simplest model of computation?
A:  Term Rewriting Systems

**SPIRAL-5 -- SEMANTIC ASSEMBLY LANGUAGE**
		Unicore:   EXE/GND/DEF  BLK/BRA/REP/RET  VAR/PKG/CTX
				
Q:What is a simplest instruction set capturing procedural code
A:  Unicore
				
**SPIRAL-6 -- META-PROGRAMMING LANGUAGE-WRITING-LANGUAGE**
		A Statically-typed/Pythonic/Lispy lang-writing-lang

Q:  What is a simplest way to use these parts to build 	
	everything else?
A:	A meta-programming, language writing

	 language
	(meta-programming language) built upon these constructs.
A:  Uniform?  Class/Object model

**SPIRAL-7 -- MENAGERIE OF SHARED CONSTRUCTS** 

e.g. a Data Dependency Graph; a Block Chain
	

**SPIRAL-8 -- ALL SW LANGAUGES AND FRAMEWORKS**
- SPIRAL-8 -- MODERN SOFTWARE LANGAUGES AND FRAMEWORKS
### --- ECOSYSTEM AGENDA ---
Aim:  To provide an optimized basis for collaborative knowledge work

**ECOSYSTEM AGENDA -- UNIFORM ECOSYSTEM-WIDE AGENDA**
Aim:  To progress iteratively towards a "best" spiral of constructs covering some universe of targeted tasks.
Aim:  To provide an optomized basis for collaborative knowledge work
Define: Societal Benefit = the amount K-work related value being transferred.


**UNIFORM LANGUAGE -- best language writing language**
Aim:  To be a best language writing language -- 
a best language for developing an interrelated web of domain specfic languages.
DSLs that maximizes 
community benefit. --OR--
transfer of K-work related value.

**UNICORE -- starting seeds**
Aim: Provide the starting seed for the uniform agenda -- 
the seed constructs from which the rest of the uniform universe is derived.

**UNIFORM COLLABORATION -- tools supporting value transfer**
Aim: Provide context and tools that support society-wide maximization of value transferred.
which support collective progress towards collective goals. 
Aim: A best framework for ma

**UNIFORM MARKETPLACE -- incentive structures used to maximize value transfer**
Aim: Provide incentive structures that drive maximization of value transfer.

**UNIFORM SOCIETY -- community aimed at maximizing benefit for its members**
Aim: Support human flourishing by maximizing value transferred

UNIFORM MARKDOWN -- 
Claim:  Single parser facilitates development of interoperable DSLs 
Claim:  Format allows range of modern sofware languages to be covered with a single markdown parser.
### --- UNIFORM AGENDA ---

Idea:
- Express computation as a sharp, spiraled, factoring of constructs that support societal knowlege targets.

Agenda is accomplished by expressing 

This is done by expressing computation as a spiral
Express computation a


Agenda:
Iteratively develop a sharp, spiraled, factoring of some targeted universe of computational tasks.


Agenda:  Best LWL

Key Idea: sharp spiraled factoring

Agenda: iteratively develop a "best" set of computational constructs that can be used to cover (implement solutions for) some universe of targeted tasks. 

~~~~
Key Idea:  Express the "best" LWL as a 
Agenda: Iteratively develop "best"     set of constructs that form a sharp, spiraled, factoring of some universe of targeted tasks.

Key Idea:  Express the "best" LWL as a set of constructs that form a sharp, spiraled, factoring of the universe of targeted tasks.
### --- UNICORE AGENDA ---
#### bean-pole-ratchet

COVER -- Use existing constructs to solve an outstanding task.

IMPROVE -- Transform web of constructs and task solution to improve in some way
- DECOMPLECT -- Split contruct into two or more.
- SHRINK -- Simplify construct and show how to reimplement dependent solutions using aspects of other constucts.
- ABSTRCT -- Simplify multiple constructs, show how to reimplement solns built on all using some new 'ur' construct.

JUSTIFY -- Explain how how each aspect of each construct is logically/provably reqired by nature of construct.
- FAILURE DRIVEN IMPROVEMENT -- Struggling to clarify the pivotal constraints often leads to recognition that construct is not in fact sharp, and further generalization is possible.

TRANSEND -- 

#### Understanding a thing
So you think you understand a thing...   great... 
- name it -- name it in a way that the name evokes key parts of the idea in the mind of others.
- define it -- express it as a single sentence -- a sentence that causes understanding in the mind of others.
- formalize it -- express it functionally in terms of other formalized things.
- use it -- apply thing as defined in all places and cases where it should apply.
- refute it -- find cases where definitions and usages are wrong.
- improve it -- change understanding to keep usages and avoid refutations.
- repeat this -- continue these steps until you can see no way avenues for further progress.
- hold it -- hold partial progress in your mind for months / years, new avenues will appear
- pass it -- having exhausted your cognative capacity for progress, pass it to others for progress.
### --- SOCIETAL AGENDA ---
#### Parenthetical
Espeically embarassed by this section.  
- ARROGANT -- A truly humble person would be embarrassed by the aims and claims of every Uniform level.  I could probably use a bigger dose of that.  Still even for me, this section does embarrass me.  Who the F does this guy think he is?  God?  Entein, Arestotle, the first guy to think these poorly articulated half thoughts?  Yeah those are the challenges that come to my mind as I look at it.  Valid challenges.  And hence the embarassement of putting this last level out there.  
- POORLY UNDERSTOOD -- In some levels in Uniform I am arrogant enough to think I have contributed some bit of clarity or understanding here.  But at this level I don't believe this.  So far this is a mish-mash of poorly articulated, poorly understood ideas groping for cohesion.  This is not humble brag -- this is my real assement of things.
- DANGEROUS -- In so much as these ideas do cohere they become potentially quite dangerous.  These ideas, like all societal ideas, are themselves are easier to understand than their consquences if put into practical practice.  

So these are some really good reasons to NOT present this material now or perhaps ever.  So why is it here?
Academics and evagilists both tend to focus downward and inward in order to make greatest progress.  This is not wrong as far as it goes, but if this focusing is done exclusively it easily results in running fast, but fast in irrelevant, distracting, damaging, or catestrophic directions.

Such works must be connected back to the larger picture if they are to improve rather than damage that larger picture.

Thus this section is included as a hedge against such damage.  
That I or others might recognize folly and limit its consquence.

At some future point perhaps this section would be well enough articulated that it might indicate capacities that could improve society, that could improve the human condition in some way.



NO GAINS WITHOUT PAIN -- 
#### Claim / Aim
Aim -- 
# ### PRINCIPLES ###

MINIMAL << ESSENTIAL, SVELT/NOBIOLERPLATE -- 

NOBAGGAGE -- should not matter how API came to be, should be best API it could be, at if invented in a vaccum.
	really only defined relative to some fixed backdrop of terms

GENERALITY THROUGH NON-DEPENDENCY -- 

USE not REUSE -- Modern code reuse typically implies use of code in a context somehow different from the one it was created for.  Contextless usage is an alternate approach for resue


## >>> OUTLINE <<<
UNITARY		<<	ESSENTIAL?, NOFORK, HANDLED
	NOFORK		<<	Shimless, Power Spiral, Caking Eating
KNOWLEGEWORK<<  P2P, 

_
## >>> UNSORTED IDEAS <<<
### --- Graceful Degredation ---
Graceful Degredation -- with greater assumptions and greater base abilities should afford greater provided abilities.  As base assumptions and abilities are removed provides abiilties should gracefull degrade; at all levels the provided abilities should be as great as possible, and transition to less shoudl be a smooth as possible.

Further the existence of higher capacities should never cause lesser environments to fall below the level they might if they were built directly for the lower capacity enviornments.

FOR EXAMPLE:  most tooling environments add capacity beyond raw text.  In doing so, they often break capacities that exist for raw text.  this is an unforced error that we should take pains to avoid.
### --- Ground Up Simplicity ---
Building Simplicity 
Simplicity can only be built from the bottom up -- You can't bolt simplicity onto complexity 
### --- Huff coded source ---
### --- IDE-ify ---
IDE-ify -- Build lang specific IDE for each novel data form

Best possible tool env is always an IDE (of course best possible language also gracefully degrades to text too.)

IDEification of a language -- Removes boiler plate characters that map unabiguously 1-to-1 with remaining source characters.

Such information can be safely removed from the source code an placed easily in hover over text, out-of-band "info box", or info lookup function
_
### --- REDRY ---
ReDRY -- Radically executed Don't Repeat Yourself 

Uniform takes the DRY "don't repeat yourself" software mantra to an extreme rarely found elsewhere.

The result is an exponential reduction in ecosystem complexity, along with an exponential increase in expressive power in that ecosystem.

The aim with DRY is to avoid having instances of a common code pattern to exist in multiple times within a single code base, if it is possible to express the patterns as applications of a single underlying abstraction.

ReDRY repeated applies this mantra while expanding the aperature of refactoring one might consider driving towards a most elemental form where no further reductions are possible.

In one sense DRY and ReDRY are one and the same.  But in practice DRYing ones code does not involve reshaping the scope of what is computed, but rather just the structure of the code which computes it.  ReDRY applies to an ecosystem where the ultimate scope of all programs that might be written within that ecosystem is not fixed. Quite often ReDRYing an ecosystem results in a new ecosystem with an expanded scope of capabilities for similar complexity.

A second different is the scope of change that might be considered is not limited in size.  ReDRYing considers the entire ecosystem and all languages and all programs as a large crystal, where some refactorings might effectively rewrite the code of the entire ecosystem.  DRYing a single application within a fixed ecosystem is by its nature a more limited action.

The third difference is the aim for unity.  Uniform ecosystem is built from a strong preference for singular instantiations.  Great leverage is achieved in cases where one can reduce a broad category of behavior onto a single artifact. 
e.g.  all change occurs because of a specific call to the SET operator.  
_
### --- LOOP TO TOP ---
Loop back to the top.
as much as it is sensible, generalize each concept so far that is applies broadly...  ideally it applies universally.  
in practice this means it is a method that applies to all units.
## _
factoring
powerset of factor combinations all work automatically
"+" operator can combine factors and factor sets as expected 
resulting combinations are as 80/20 good as a purpose built combination.
### >>> SUMMARY <<<
UNITARY := 
### >>> CLAIMS & AIMS <<<
#### Belief -- See arguments > how society works
#### Ecosystem claim -- >100x improvement
Claim:  There is >100x improvement available if humanity collaborated differently around the knowlege work it does.  Once this alternative is clear and the ball is rolling this alternative will become an unstoppable force transforming all of those collaborations.

##### --- The Fish In The Sewer ---

So if there is this magic improvement close at hand, why can't we see it?  Why aren't we already doing it?  Come on, how can this be?

Imagine a fish born in the sewer, for her everywhere she has ever travelled was clogged with filth and sludge.  Its not that she cannot perceive the sludge that slows every flip of her fin, she _can_ feel it, indeed it is a constant mysery.  Its just that when another fish tells her about the widest of wide sewers, one so wide there is no upsream or downstream it just makes no sense at all.  When told of a place of no filth she cannot imagine what else could hold her up -- it just makes no sense.

Humanity is exactly that fish, swimming in the excrement (unecessary complexity) generated by those who before us.  Its not that the artifacts that has been generated before do not have value -- they have tremendious value -- they are the very stuff through which we swim.  Its not that we cannot perceive the collossal pain of understanding and operating on those things artifacts, we feel that pain accutely every day.  Its just that we believe this pain is part-and-parcel of swimming itself, we can't imagine there could be any way to follow the path of those before us without trudging through the excrement left by them.

Its not that we never ever seen clean water -- seen artifacts without extraneous complexity -- we have in tiny patches of clear water here and there.  But these patches were too tiny and too far apart to give us any sense of what it would be like to actually _swim_ in an endless patch of only clean water.  Just as this seems like a fairytale our fish might tell her children to buffer them from the harsh reality of their filthy world, we hear this notion of connecting patches of clear water together to swim effortlessly through artifacts created before us without ever hitting the filth of extraneous complexity, and it seems nothing more than a fantasy -- one that is reminicent of related fairtails we have heard before.

We don't see it as plausible, how could it ever be practical to seperate the artifacts we build from unnecesary complexity around them over large spans?  This has never been done before except in the tinyest of patches, how could it be done that way now?   And even if one could somehow do this undoable thing, we have no belief that literally >99% of our efforts now are wasted efforts -- efforts required not by the need for motion, but rather extra efforts required to trudge through crap which did not have to even be there.  I get it, such a thing (just like this story) is so abstract it is nearly incomprehensible for us to really imagine what that knowlege work swimming free from incidental complexity might be like.  

Still I am one fishy telling you there is a different way to swim, one that moves us forward interatively removing the incidental complexity that so greatly hampers those who follow.  Since we don't clearly see the collossal damage our careless actions take, we don't perform the needed iterating and upkeep require to allow all to swim in oceans of blue.  I know other fishies have told simlar tails before, and I know we are still swimming in the same stuff we were before.  Why should you believe it?

But we can have the progress without the great pain of crap carelessly left, we just have to need to clearly separate the inherent complexity that is that act of swimming, from the far greater pile of incidental complexity, which can be excised.  We need to very clearly account for and compensate against the collosal costs paid when we dont excise it.  

We **can** swim in oceans blue.  Let's be new fish in a new ocean!


--- justification for the uniform marketplace -- 

Just like pollution in the physical work, complexity in the knowledge world is an externality.  It is a cost, but one that is not paid by the polluter, but rather by the citizens, and just as with physical world pollution, one will never have a clean environment 
until we have rules and incentives, polluters will just pollute us to death... a spiral right into the sewer that we live in today.

##### --- Contrasting examples that show ecosystem benefit ---

Dude, swimming fish? Seriously, are we even talking about software?

Ok let me try contrasting two cases, one that humanity got right, and one we got wrong, then imagine the benefits if we had gotten the second one right.

The first case:  Ascii/Unicode.  Most every software langauge uses Ascii/Unicode to encode its source code.  Notice this is a arbirary choice, really any symbol system would have worked pretty well.  Even hieroglyphics could work... might even result in more parsimonious code.  But it would be a disaster.  We would need different editors for each software language.  We would need different file systems.  Different message transmission protocols if one wanted to share code.  Really the ONE choice to cenralize on a single symbol system is responsible for a collosal improvement in efficiency as compared to some other universe where each nation of the world had adopted tooling, operating systems etc. based on incompatible symbol systems.

One could argue that each symbol system could be converted into any other, so it does not matter which we choose.  This is true, but the extra effort to have conversion layers inside every text box of every interface would be painful, dealing with these in every editor, in every compiler, etc. would be very costly, and a needless cost -- just use unicode!

Now lets consider a different case.  How we encode the notion of containment.  one thing is inside another.  It is the same formal concept, we used it everywhere, but we don't reuse any common infrastructure.  So any visualization of containment in an editor or interface has be repeatedly done hundreds of thousands of times over.
Any reasoning about containment latent in the structure of a data table must be implemented countless times over, becuase the way it is encoded is needlessly specific to that application and that table.  They way it happens in a filesystem, in a memory structure, on a visual display, are a million times over unique little snow flakes.  But we get little value, from this needless diversity, we hardly notice the pain in causes, becuase it is so ubiqutous, but it could have been different.  We could have agreed on the way this very basic notion was used and encoded in all of these contexts, then effortlessly we could operate on containment just as effortlessly as we operate on unicode.  we don't even think about it as translation, its just the same containment data anywhere we have it -- just as unicode is the same text information anywhere we have it.

Imagine all of the complexity in all applications which exists to support the encoding of, the visualization of, manipulation of, inference over, translation of containment information in all ways in all places for all uses.  If that were handled in a uniform way the savings would be enormous.  Even more, containment notions interact 
#### Ecosystem aim -- transformatively improve human collaboration
#### Unicore claim -- Unicore suffices
Claim: much of human knowlege work can be constructed
#### Unicore aim -- provide basis for human knowledge work.
Aim: provide a first spiral of constructs that cover much of human knowlege work.
#### Uniform Structure -- sufficies for encoding structure
#### Uniform Language -- 
### >>> UNDERLYING BELIEFS <<<
#### TODAY SW IS VERY WRONG, BUT IT CAN BE MADE RIGHT
- SW is wrong in the sense that a >10x improvement is effectiveness is unrealized
- Requires different tooling, different basis, and different incentive structures.
#### COST OF STATUS QUO IS COLLOSSALLY LARGE
- >10x in results per hour is available if we operate better 
- Like a fish in water, we are so used to swimming in filth of broken languages we dont even see or correctly count the costs of its many burdens

#### INSIGHTS ABOUT LANGUGE ARE AT LEVEL OF ABSTRACTION
## === SPIRAL ZERO ===
### preamble

Here we take a stab at outlining the key ideas and principles which underly the Uniform agenda.  The outline is expressed in successively deeper levels of detail.  This is done to save the reader time, and also because many of the ideas will be super abstract and also obvious ``duh, of course'' kinds of ideas, but they will lead to controversial conclusions.  This structure allows the reader to dive in at places where they are surprised in order to inspect the reasoning at play.

The beginning sections are very abstract ideas that many may feel they agree with, so they may feel pointless.  As motivation, to stick with it, here are a few spiral-two principles that are likely far from obvious:
- **never fork** -- always find a single way that covers all cases.
	e.g. one parser for all languages, one structure for all data
- **composable** -- express language formulation as composable
	constructs.  e.g. lazy eval should be addable after the fact using the "+" operator.  So if, S = some language, and  L = lazy_evaluation, then combining the soruce code for S&L will yeild S2 = S+L is a new language like S, but having lazy evaluation.
- **retroformulatable** -- given two independently developed modules A and B, it should be possible to create C from A and B such that C has all capabilities expected from a combination of A and B, but none of the baggage from either of A or B.  
	e.g. the interface exposed by C (and all of its performance characteristics) should be nearly as good as if C where developed from scratch without any constraints eminating from the structure of A or B.

Many would see these goals as nonsensical.  But we don't, indeed we argue they are 80/20 acheiveable, and this "getting close" has great value.  so stay tuned....
### UNITARY
UNITARY -- Strong preference for providing exactly one option.

UNITARY ==> HANDLES, 
_
### KNOWLEDGE WORK: WE ARE IN THIS TOGETHER
- **SPECIAL** -- Knowlege work is a special kind of work, the nature of knowlege itself requires certain patterns of interaction, processing, and interoperation in order to be effective.
- SPECIAL REPRESENTATION
- SPECIAL INCENTIVES
### --- P2P -- POWER TO THE PEOPLE ---
Knowlege work should be organized to maximize "downstream" benefits for users of knowlege products at the expense of those creating the products.

1. Knowledge work is a many layered pyramid with large fanout;
	this fact justifies "infinite" effort in optomizing the top.
2. Today we expend little effective effort in optomizing the top; so we are extremely far from what can be achieved.
3. The flip side of this is the opportunity to disruptively improve all knowledge work by changing how it is done.

**UNDERLYING FACTS** 
1. **FANOUT** -- The fanout is collossal & consequences are large
2. **SHAME** -- We are shamefully bad at optomizing today
3. **OPPORTUNITY** -- Transformative improvement is possible

#### FANOUT -- THE FANOUT IS COLLOSSAL & CONSEQUENCES ARE LARGE
Fanout is much larger than commonly thought: >100 or 10,000 to 1
1. Fanout is in both space and time.
2. Effects of fanout span layers with exponential effect.
3. This Fanout spans ecosystems/frameworks/languages.

.
#### SHAME -- SHAMEFULLY WE ARE NOT REALLY TRYING TO OPTIMIZE TODAY
.
#### OPPORTUNITY -- THINGS WILL BE DISTRUPTIVELY BETTER WHEN WE TRY

.
### ROADMAP -- THE GAPS ARE MANY BUT THE PATH IS CLEAR
1. We know a great deal about what makes knowlege work effective
2. We need to change our aims in knowldge work to improve
3. We need to change how we collaborate in order to improve

PARTS
1. GAPS
2. CHANGE

#### CHANGE
1. PROCESSES -- The way we do knowlege work needs to change
2. AIMS -- 
2. INCENTIVES -- 
### CONSEQUENCES -- THIS PATH HAS MULTI-LEVEL CONSEQUENCES
1. ONTOLOGICAL CONSEQUENCES --
2. COLLABORATION CONSEQUENCES -- 
3. MARKET PLACE CONSEQUENCES -- 
4. SOCIETAL CONSEQUENCES -- 
## === SPIRAL ONE ===
### >>>   U N I F O R M   C O L L A B O R A T I O N   <<<
CLAIM: We are not collaborating in the right way; if we did it would be a 10x or 100x boost.
_
### --- ECOSYSTEM BEGETS OUTCOME ---

**THE ECOSYSTEM (INCENTIVES, TOOLS & CONTENT) DRIVES ACTION** -- The ecosystem, the available tools, content and incentives, drive each action taken by each individual within their localized context.
**ACTIONS OVER TIME DRIVES THE ECOSYSTEM** -- The ecosystem iself is an amalgam of a sequence of local actions and decisons taken by its participating individuals.

CLAIM:  Current knowlege work is 10x or 100x less effective than it could be.  
Said another way:   Current software practice wastes 90-99% of human effort involved.  lets fix this.

**KEY CONSIDERATIONS:**

- **LEAST RESISTANCE -- PARTICIPANTS FOLLOW LOCAL PATH OF LEAST RESISTANCE** -- Individuals tend towards paths and actions that solve their immediate concerns with least effort (the least learning, the least planning, the least mental effort, and the least time required to acheive their immediate goal).
	**==>** Thus _localizing_ (aligning local incentives with global ecosystem value) is strongly indicated.
	
- **INCENTIVES STEEPLY DISCOUNTED -- PARTICIPANTS DISCOUNT INCENTIVES AS COSTS OR DELAY TIMES GROW** -- Participants will generally spend 5 seconds doing so immediately saves them 10 seconds of time (duh).  But the often will not spend 5 seconds of time even if they expect it will save them 5 minutes of time over the next month.  -----    But It is not hard to incentivise small deviations from the path of absolute least resistance (e.g. getting developers to add a small commit message).  But incentives must be dramatically (super linearly) increased in order to have larger deviations from path of least resistance.  Further the time discounting of delayed benefit is very strong, so an incentive paid distantly in the future must be much much larger than an immediate payoff needs to be in order to incentivise specific actions.
	**==>** Thus _**granularizing**_ (enabling micro-contributions with net positive value) is strongly indicated.
	
- **80/20 RULE -- ECOSYSTEM TRADEOFFS FOLLOW THE 80/20 RULE** -- Often the ecosystem can acheve 80% of maximal possible value on some tradeoff while only incurring less than a 20% cost against each of a great many other tradeoffs.  
	**==>** Thus _**middle-pathing**_ (charting an 'middle path' combining 80% beneifts) is strongly indicated.
	
- **FAT FANOUT** -- Mature components of a software ecosystem typically have a massive fanout. One unit of time is spent authoring a mature content for every 100 or 10,000 units of time spent in downstream appliction of that content.  
	**==>** Thus _**upstreaming**_ (exchanging upstream effort in liu of downstream effort) is strongly indicated.
	**==>** Thus _**enviro-optomizing**_ (having participants operate in environments optomized for the knowlege work they are performing) is strong indicated.
#### (1.1) PERFECT WORLD
Because ecosystem_begets_outcome and fat_fanout it is worth most any effort for knowlege workers to operate in a perfect_world as much as possible.  

So what is our target, what *is* an ideal environment?

- This world has no_baggage, that is the tools availble, the languages information is represented in, both syntax, semantics/operators are all ideal for this ONE are 

- This world interoperates_in_all_the_right_ways
### --- COMPLEXITY KILLS --- 
The cost of dealing with complexity dominates all other costs in knowlege work, thus reductions in local complexity are often worth nearly any cost.  

- **COMPLEXITY COMPOUNDS** -- The costs of dealing with many forms of complexity often compond multiplictvely, this is one key reason why complexity costs often dominate knowlege work which is often occurs in environments where many complexities come to bear.  As a simple example suppose there are five kinds of easter basket designs with 3 kinds of materials.  One must consider 3x5=15 combinations if you want to be sure that all baskets 
	==> Thus _decomposing_ (converting interacting complexity into decomposed independent complexities) is strongly indicated.

- **COMPLEXITY GROWS** -- One cannot can generally not extend a complex system in order to make them simple.
	==> Thus _starting simple_ (building systems by starting from simplest roots) is strongly indicated. 

- **NEGATIVE VALUE** -- Many contributions with localized value have dramatically negative ecosystem value.
	==> Thus _healty growth_ (building from contributions with net positive value) is strongly indicated.
	
#### COMPLEXITY DETAILS
- MEASURING IT -- seconds of participant time needed to understand and operate over it.

##### POWER TO THE PEOPLE (P2P)
- Costs associated with downstream complexity dominates all.  Anything one can do to simplify is better.
- Corollary -- Optimal Env -- In mature SW -- K worker should be operating in the optimal environment. 
- Never reduce flexibility from your user any more than is intrisically required in order to provide a value to them
- _IMPLIES_: Ideal Environment, Upstreaming

##### Software Is A Pyramid
A software ecosystem is an pyramid of layers flowing down from the top most starting point.
Quido noted that code will be read many more times than it is written.
**TRUE** -- As one develops code it is read over and over again OVER TIME as it is developed by a single author
**EVEN MORE** -- When multiple authors even more, so the code is read by many even when a given line is only written by one.
**EVEN MORE** -- When multiple users use a package its docs and code are read many times though written once.
**EVEN MORE** -- When down stream software is developed upon
##### SIMPLICITY MUST BE BUILT FROM THE GROUND UP
- One cannot can generally not extend a complex system in order to make them simple.
- One can cover over complexity with a layered API or such, but the only way to truly have a simple system, is to build it as a simple system from the ground up.
	- Covered over complexity tends to bite you -- and when it does they are often the most pernicious bites.
### --- UNIFORMIZATION DRIVES ECOSYSTEM VALUE ---
Uniformization is the process of expressing the solution space of for range of tasks as a sharp, spiraled set of independent, interoperable, essential constructs.
### --- DRY -- DONT REPEAT YOURSELF ---
- Meant more broadly than typically understood:  
	we mean it across all _mature_ code... everywhere
- ==> even mature software today is very very wet!
- ==> it can be practical to dry out world's code
### --- COMPOSIBILITY IS KING ---

POWERSET DECOMPOSITION
## === SPIRAL TWO ===
### --- IDEAL ENVIRONMENT ---
Specialized knowlege work is best performed in an environment specialized for that type of knowledge work.



 Provide tools to create it
Provide tools for creating ideal environment for knowledge work
- Means access to ideal tooling, ideal language, ideal docs
- But means more -- it means ideal access at all time scales.
	- at .1s -- what is visually present and visually aligned
	- at 1s -- what hover/click actions
	- at 10s -- what searches & transforms
	- at 1m -- complexity of expression
	- at 10m 
- But it means more
	- ideal organazation
- _IMPLIES:_ Web Of DSLs
#### LANGUAGE WRITING LANGUAGE -- 
### --- RETROFORMULATIVE ---
RETROFORMULATIVE -- Construct built w/o compromise to be "best" for target tasks AND
	maximizes reuse / minimizes introduced complexity.

Need: Ideal Environment -- K work is performed at a particular level of abstraction with a particular set of tools, operators, frameworks, etc.  The effectiveness of that work is dictated by the degree that the environment matches the K task.  Ideally all of these components are optomized specifically for this class of task.

not said right....
Need:  Reuse of related components -- Still this task is related in many ways to constituent components relevant to other task classes.  A single component cannot be specifically optomized for all possible relevant cases ... some of those optomizations will be incompatible with each other.  But building duplicate related by not the same would be an unacceptable fork.


TASK SPECIFIC CONSTRUCT -- Designed from the ground up for this one and only task class.  It draws exactly those distinctions required for this task, and no others, and each is presented as a spiral of abstractions that is ideal for this one task class.

RETROFORMULATION -- 

_
### --- UPSTREAMING --- 
### --- POLR -- PATH OF LEAST RESISTANCE ---
As a tendency each collaborator will opt for an approach that not much more difficult than the absolute easiest/shortest path available for them to achieve their individual, most-immediate, personal objectives.  e.g. they might form the habit of generating well documented git commit messages -- which is not strictly needed for thier immediate coding objectives -- but they will only maintain this habit if that cost is small relative to the over all cost of of achievimg their immediate objectives.

Given this, then virtuous collaboration systems adopt policies that achieve the range of their other objective given the PPLR constraint.
### --- NON-DIMINISHING ---
### --- NON-FORKING ---
It is almost always possible to avoid forking, and better if one does.

Forks seem natural and often inevitable when first approaching an issue.
Often it first feels contorted to try to NOT fork, but using a aresenal of techniques it is stunningly possible to avoid the majority of these seemingly unavoidable forks, and the result when looking back at it, is often quite beautiful and not contorted at all.

The solution is often reframing the context, or the desired results in order to align the sides of the fork.  Or it is parameterizing the fork in a way that both paths become the same path, but with different parameterization.

Sometimes the solutions seem a bit of sophistry, since buried within the "single" solution
at some level of represenation is the same fork but in a different guise.

Still this is often good progress.  A fork dressed in this way can often be treated by subsequent layers as a unity when possible, and just one of the relevant forks when not.  This is still success since, at a practical level, one can develop ontop of this construct as if it is not forked.

Fork Avoidance Techniques:
-- Fork parameterization
-- "Or what?" framing
-- "Ur" spiraling -- Developing a third "Ur" construct which can expand into the others
### --- NO CRACKED ABSTRACTIONS ---
### --- NO BOILERPLATE --- 

semantic scaffolding is ok

boiler plate that exists in order to fit DSL semantics into more general language is not ok
## === SPIRAL THREE ===
### --- HANDLES ---

HANDLE -- A singular thing used to control some logically cohesive group of structure and function

HAVE HANDLES -- Important cohesive groups should always have a handle.

If a thing is always operated on within a given enviornment, then there is no need for the handle to reference any part of that environment since it is all implicit in the usage context.  Otherwise such contextual values must be part of the handle so that it can be wielded from all needed contexts w/o requiring additioanl parameters

ok to have parameters as long as there is no inherent or practical connections implicit


when handling a thing within a given context 
# ### TERMS ###
### --- COGNITIVELY TRANSPARENT ---
### --- DECLARATIVE FORM ---
### --- SEMANTICS FORM --- 
### --- POWER SPIRAL ---

**SPIRAL** -- A _spiral_ is an organizing principle for information or action, where multiple discrete levels of complexity are accomodated as a nested progression of spiralled "levels" or "rungs".

**POWER SET** -- A _power set_ is the set of all possible combinations of some underlying basis set.

**RUNG THESIS** -- A _rung thesis_ or _spiralling thesis_ is an assertion of completeness for a given level within a spiral.

**POWER SPIRAL** -- A _power spiral_ or _power spiral decomposition_ is an arrangement of information into a spiral where each rung is encoded as the power set of some rung basis elements whose completeness is asserted its rung thesis.

SPIRAL RUNG

SPIRAL LEVEL 


A power spiral decomposition is an approach for organizing information with a number of useful properties:
- RUNG NESTING -- For any given task one can select an appropriate spiral level that best trades complexity adopted vs power provided.
- ELEMENT ISOLATION -- Both spiraling building upon the previous level and power setting help to enable element isolation.
- ELEMENT MINIMALITY -- 
- COMPREHENSIBILITY -- 


#### -- 

#### -- SNAIL ANALOGY --
{{show image of three different sized snails}}

EVER SIZED -- At each stage a snails last spiral is just the right size for the snail.

MINIMAL BAGGAGE -- Becuase the shell grows geometrically in size the weight of all prior spirals is small relative to the weight of just the last spiral.

SAME FOR SW -- Software spirals can be the same.  Different levels of completeness and of abstraction are often naturally dramatically different in size.  And which ever level one is using no later levels need to be considered, and like the shell, all earlier levels are tiny by comparison.  

UNIVERSAL -- Thus a spiraled API can make the ideal tradeoff between complexity and completeness for all applications all in one spiral.


POETRY -- The total volume of the earliest spirals are minescule, but the effect they have on the overall structure of all spirals is enormous, every microscopic detail of their shape becomes the over-girding scaffolding for every part of the larger edifice.
### --- BLIND DEPENDENCY ---
A blind dependency is a weak form of dependency where one adheres to the datatypes and conventions of some item, but one never leverages functionality defined by the item.

In cases where we desire ultimate interoperability, yet we want to avoid requiring a huge dependent compoent (since we are using only the barest surface of that component in order to ensure interoperability) we use a blind dependency.

This blind dependency does require us to keep these the two interfaces in sync with each other, and in some cases may even require us to duplicate functionality across these components, it keeps us from having an explicit code dependency.  

Blind dependencies are mostly used in the lowest spirals within an ecosystem in order to keep the core compact, yet also interoperable with the more complete system.
### --- CAKING EATING ---

**CAKING EATING** -- Cake Eating refers to simultaneous 
_
# ### TOPICS / LISTS ###
## === LISTS ===
### --- LEGAL CONSIDERATIONS ---
#### ... AIMS ...

	- **OPEN** -- Disposes folks to contribute.
	- **CONTROL** -- Provides enough control to support a marketplace.
	- **EXTEND** -- Can extend and control the extensions.

#### ... Approach ...



	- [info](https://clojure.org/community/contributing_site) [FAQ](https://www.oracle.com/technetwork/oca-faq-405384.pdf) [RHCA](https://secure.na1.echosign.com/public/hostedForm?formid=95YMDL576B336E) -- Use a Rich Hickey style agreement
	- Maintain my own copyright over the key portions of 

	- Any contribution we make under any license will also be made available under a suitable FSF or OSI approved license.


	- We are using a Rich Hickey style joint copyright model, but 
	- we are NOT licencing as open source.
	- for the moment it is simply jointly copyrighted
	- We will be building a governance and payment model designed to maximize development and adoption.
	- the aim is to give early contributors the same compensation rules that later ones receive, but since all of this is yet to be developed, current contributors should be happy to contribute w/o specific expectations of compensation.

	The
	Need ability to block unauthorized 



#### ... Key Valuable / Protectable parts ...

	TEXT
	- Textputer tool
	- 

	PATENT
	- Nputation - 
	- Live - 

	LOW LEVEL
	- Semantic assembly.  
	- Uniform Markdown.
	- 
### --- QUOTES ---
#### --- CUEBALL --- 
	in a world of corner cases lex is a cueball -- all case and no corners-----


		        The modern software ecosystem is a roiling hell of corner cases, then Lexcore is the cueball at it center --
		        it is the single case with few corners.  Anything expressed in Lex core operates seemlessly with anything
		        else expressed in lex core since there are so few corners possible to get caught upon.
### --- BEST ---
#### --- SPIRALED ---
	(replaces complete)
	Each level is complete -- able to implement all targeted tasks.

	Detail: Some spirals do require the level below in order implement targeted tasks, 
	in this case they are higher level and dramatically simplify many targeted tasks.
	??? but they should still be complete in the sense that no other constructs could be added (nope.)

	SPIRALING CLAIM -- A spiral is composed of a progression of spiral loops.  The key spiraling claim is a completeness claim on specific loops.  A spiraling claims is a claim that a give loop is complete -- its capabilities are sufficient implementing all loops above it as well as all desired applications.
### --- APIS ---
#### === SPIRAL OF CONSTRUCTS ===
##### --- S1 - URF    - GRAPHICAL DATA MODEL
		URI/triple   atoms/units/items
##### --- S3 - PAT    - FUNDAMENTAL DATA STRUCTURING PATTERNS
		List/Slice/Append   DAG/Path  
##### --- S2 - UNIT   - NODE-CENTRIC ACCESSORS
		get/set/len/itr/inv/meta/new/del
##### --- S4 - INTERP - INTERPRETATION
		ENV/EXE/GND Interp/Namespace    lang(print/parse/load/dump)
##### --- S5 - ASM    - SEMANTIC ASSEMBLY LANGUAGE INSTRUCTIONS
		   DEF/VAR/CTX/PKG   BLK/BRA/REP/CHN/RET
##### --- S6 - LWL    - META-PROGRAMMING (LANGUAGE-WRITING-LANGUAGE)
		Class,Type,Interface  
##### --- S7 - LIB    - MENAGERIE OF SHARED CONSTRUCTS 
##### --- LEVELS OF DETAIL ---
		**DETAIL-0**  
		Uniform

		**DETAIL-1**
		Uniform:
			Structure -> Function -> Unicore -> LWL -> Menagerie ->
				Collaboration -> Society -> Areas

		**DETAIL-2**
		Structure(URF -> Patterns -> Backing -> Addressing + UnitForm)
		Function (Interp -> !Unicode -> LangModel )
		Unicore  (Option, Place, ?Lexspace, Typing )
		LWL		 (DataModel, CodeModel, )
		Menagerie(Code(Text, SrcAgg), Data(Versioning), Engine)

		**DETAIL-3**
		Structure.Patterns: Composite(Ordered, Bounded, Functional), 
							Space(=PartialOrder)->Taxonomy)
		Structure.Backing(GET, SET, LEN, ITR, META, NEW, DEL) 
		Structure.Addressing(Path, Pathy)
		Function.Interp(ENV, NS, EXE )
		Function.UnitCode( VAR,CTX,PKG, Control(BLK,BRA,REP,CHN,RET) )
		Function.LangModel(parse,print,compile, ....)
		Unicore.Typing:
			Type + Head -> Spec,  Spec -> Template, 
			Spec + Interface -> Class,

		LWL.DataModel(Mixin, View, Auth, DDD, Stream,)
> > >>	LWL.DataModel.Mixin(Backy, Heady, Immuty, Lazy, Listy, Memoy, 				Pathy, SetErrory, Typey, Watchy, Versiony)
> > LWL.CodeModel(Normalization, SrcAgg, ?UF_Lang ))
				
	
		Menagerie.Text(Parsing->Markdown, SpiralDocs)
> > >> .Structure(Stream)
> > Menagerie.Engine(Transform -> (Pipe,TRS), Grammar )
> > Menagerie.Meta(Provenance, Where)


		Collaboration(Chain)
		Chain(ChainFunction, 

		Society(Pressure, DP(DecisionProcedure)->Voting, Fleshing)

		Areas(PIM)



		Principles(Sprial->Construct, )
		Agenda( NoFork, Upstream, )

		Principle.Spiral( Covering, Depends )



		Out Of Place: "+", atom & container ops
		UF: Sugar, core-operators
##### DEFINITIONS

		STRUCTURE -- Spiral 1 -- 
		A formulation of the concept of structuring of things

		FUNCTION -- Spiral 2 --
		A formulation of the concept of interpretation.

		UNICORE -- Spiral 3 --
		A covering set of constructs conversved across a wide range of modern SW langauages.  

		SPIRAL 0 -- Structure -- Expression of the idea of structure
		SPIRAL 1 -- Function  -- Expression of idea of compuation
		SPIRAL 2 -- Unicore   -- Constructs needed to express SW langs
		SPIRAL 3 -- LWL       -- 
		SPIRAL 4 -- Menagerie -- 
		_
#### === MAPPING TO PYTHON ===
##### --- STRUCTURE OPS ---
###### GET(k)
			- dict, list		u.__ getitem __(k)
			- object			u.__ dict __.get(k)
###### SET(k,v)
			- dict, list		u.__ setitem __(k,v)
			- object			u.__ dict __[k] = v
###### LEN
###### ITR
			- dict	items() . __ iter __() 
			- unit	items() . __ iter __() 
			- all	__ dict __ . items() . __ iter __()
###### INV
###### NEW
			- unit	factory(expr)
			- all?	u = klass.__ new __() ; u. __ dict __ . update( form )
###### DEL
###### --- FUNCTION OPS ---
###### EXE
			- Env	-- is an execution env
			- Fns	-- is the mapping of symbols onto their interpretation
##### OLD
###### Uniform declaration code

			def UnitBacking(KEY=Sym, VALUE=Unit):
				GET = op(k KEY) -> VALUE									
				SET = op(k KEY, v VALUE) -> self_class
				LEN = op() -> Int || Inf
				ITR = op() -> Stream(UNIT=self_class)
				INV = op() -> Unit
				FNS = op(u ) -> Fns
				NEW = op(initializer Unit=null, fns=Fns) -> Unit()
				DEL = op() -> Und

			GET __getitem__  SET __setitem__ __delitem__ LEN ??  ITR items
			INV          FNS    NEW __new__   DEL xxx 
###### _
			(See Uniform Code too)
			- **Unit**:: $KEY=>$VALUE -- A unit is inherently 
			- **Space**:: $KEY => $VALUE || u( $KEY=>Space ), @can(path)
			- **Ident**:: Str || Int
			- **ITR:: ->List($KEY,$VALUE)** -- Defines the possibly unbounded 
			- **len:: ->Int||Inf** -- Defined iff @can(ITR).  Returns the number of items in the iterator.  (may have UND values)
###### --- FUNCTION ---
			See env.py var.py ctx.py pkg.py
###### ...
			def type Structure = List(Var)

			def class Ctx::
			def Op ^init(form: Code(head(`CTX))): 
			def structure := impl.structure_get(self)
			def Op enter()
			def Op exit()
			def Op do???()

			def class Pkg::

			def class Construct::
			$syntax Bag(Tokens)
			$denote Pkg(denotation)::
###### older stuff
			def pkg Constructs::
			$syntax Aggregation()
			- BASE: Num, Int, Str, Sym
			- lang. Text, Code, Lib, Env     (maybe Text is just Str)
			- lang. normalization, expansion
			- space([TYPE [,KEY]])   -- tree of units with given type as values and type of keys
			- transform:   y = x >> xform      y = EXE(xform, self:x)   y = x.EXE(xform)
			- def type Lang = Lib{lang: { parse: print: ... }}
			APIS
			- lang.    parse/print  compile/dump   bang/load/snapshot
			- lang.bang(lang Lang)
			- eval(form0, ..., ctx: self:)

			EXAMPLE USAGE

			def parse(Str txt):
			$form = lang.impl.text2tree(txt)
			with lang.normalization { return EXE(form) }
###### out of place
			(See unicore.html for docs)

			PYTHON: id, items, len
			PYTHON: call, getitem, setitem, delitem, new, del, hash, cmp/eq/ne/lt/gt/le/ge, 
			PYTHON: str/repr, doc, name/qualname
			PYTHON: module, defaults, code 
			PY DATA MODEL:  new, init, del, repr, str, bytes, format, lt/le/eq/ne/gt/ge, hash, bool
			PY ATTR ACCESS: getattr, getattribute, setattr, delattr, dir

			new, del, get, set, len, itr, inv, ns

			META FIELDS
			^head = 
			^id = integer indicating identity
			^hash = hash value of object
			^opt = options
			SPECIAL OPS
			str = convert to string
			cmp = cmp(other, [eq-lt-gt])  0=false, 1=eq, 2=lt, 3=le, 4=gt, 5=ge, 6=ne, 7=true
			rpr = convert to unit repr-form
			itm = return items
			inv = return inverse items (for a tree this is only has one pair, and zero for root)
			ns  = return namespace used by unit
## ===1=== DIMENSIONS OF "BEST" LANG DESIGN ===
### Intro
What do we mean by "BEST" language writing language?
A collection of language constructs that simultaneously maximize ALL properties below
### Sharp
SHARP -- A constrcut is sharp if it is not possible to make it "better" (more general, simpler, more performant, etc.) without having to give up some other desirable attribute.  
That is, this design choice represents a "true fork" whose boundary stems from a tradeoff inherent in computation itself.
### --- SIMPLEST ---
### Independent
INDEPENDENT -- Two constructs are independent if each can be understood w/o reference to the other.
### Interoperable
INTEROPERABLE -- Two constructs are interoperable if they naturally combine to implement all expected SW constructs.
### Elemental
**ELEMENTAL** – Uniform formulations are so simple, any further simplification causes them to cease BEING formulations of the notions they encode.
==> Uniform formulations often expressable as a single English sentence.

ELEMENTAL - A formulation of some software concept is call elemental when any simplification of the formulation causes it to no longer BE an formulation of the software concept.  

Simplest Possible -- each part of Uniform was expressed in simplest (complete) way possible
Simplest – Uniform is a collection of “simplest possible” formulations 
Simplest -- each notion's formulation is minimal -- any reduction and it would cease being a formulation of that notion.
Simplest – Uniform formulations are so simple, if further simplified they cease to BE formulations of the computational notion they hope to capture.
### Uniform/Singular
UNIFORM/SINGULAR – Uniform is very "uniform".  Similar things are reformulated as manifestations of a SINGLE thing in Uniform.
==> e.g. all change grounded in a single SET operator.

-- Fewest distinctions --
Singular – Far more than other languages, Uniform merges related so they are the SAME thing.
Singular -- if two aspects of Uniform share a theme, they must derive from a <i>single</i> Ur formulation of that theme.
### Decompleted
DECOMPLECTED – Constructs are decomposed until constituents are uniform w/o variation.
==> Allows endless reformulations unconstrained by underlying language choices.
Decomplected – Common computing patterns are exploded apart, allowing reformulations unconstrained by Uniform language choices.
==> even function call in Uniform is constructed from parts in Unicore
### --- EMBEDDABLE ---
### Embeddable
EMBEDDABLE – Like JSON is for data, Uniform for code is so generic that it “fits into” most languages without adding complexity.
Embeddable – Like JSON is for data, Unicore for code is so generic that it maps naturally into other languages precisely without mismatch or added complexity.
==> Uniform script expands naturally to java, python, c#, etc.

Embeddable -- Unicore code-constructs can be mapped directly one-for-one into parallel language constructs in all modern languages without loss.  (For example, JSON is embeddable into Java and Python while XML is not.)

-- Each makes so little assumptions they can fit w/o compromise
   e.g.  unit's notion of structure can fit any langauges notion of structure
   
#### DEFINITION
	Language A is **_embeddable_** in language B iff for all forms a that natively encode in A
		there exists form b that natively encodes in B where 
			a is parallel to b  
 

	Two forms, a and b from two different languages A and B are **_parallel_** if:
	-- they have similar meanings according to the sematnics of their respective language, 
	-- the human relevant structure of their forms are judged parallel, 
		(engineers would think of them and operate on them in similar ways.)
		
	A form, f, is _**natively encoded**_ in language L, if there does not exist some f' in L that is functioanlly equivelant and engineers would agree is a more "native encoding" for f.

	**An example will help here:**
	It would be possible in Java to encode integers as strings that containing Roman numerals.  One could define all arithmetic operations over such "integers".

	These strings would all be parallel to some hypothetical "crap-lang" that natively encoded its numbers internally in this form.  But we would never want to say that crap-lang is embeddable into Java ... it is not embeddable, since even though one could encode integers in Java in a form parallel to crap-lang, they would not be **_natively encoded_** integers in the java context.


~~~~~ OLDER FORMS ~~~~~
	Language A is _embeddable_ in language B iff for all forms a in A there exists b in B where 
		a is parallel to b, and 
		a being natively encoded in A implies b is natively encoded in B.  
### Sprouty
SPROUTY -- Large diversity of capability may be implemented ("sprouts") from a small set of underlying constructs.

    <p> <q>SPROUTY</q> &mdash; Unicore as a whole aims to support the <i>widest</i> possible range of formalisms while 
        relying on a <i>narrowest</i> possible assumptions regarding the computational substrate upon which it is 
        built.  Thus attention is taken both on the minimality/generality of assumptions made regarding computation 
        and storage required to embed unicore with in existing computational framework, and to the flexibility of 
        formalisms that can be build <i>on top</i> of Unicore.  Happily Unicore seems to achieve very high
        generality/flexibility on both the bottom and the top!

-- Sprouting -- Extreme conceptual and implementational reuse allow entire ediface to be planted in it entirety.
                JSON Data, 10 primitives, single data type
### --- POWERFUL ---
### Sugared
SUGARED -- A formulation is "sugared" if it includes most beautiful and cognitively-easiest syntactic forms. 
### Natural
NATURAL -- A formulation is "natural" if is uses expected notation.

Beauty -- Out of the box, it provides good tool box for expressing computer-ie stuff in ways that humans expect and love.
   <li><d>Natural</d> &mdash; Unicore formulations must be natural. (e.g. a turing machine is complete yet its formulations are unnatural.)</li>
### Beautiful syntax&AST

        <li><d>Beautiful syntax <i>and</i> beautiful AST</d> &mdash; languages like LISP provide a beautiful
            AST optomized for programmatic manipulation, while langauges like Python provide beautilful source
            code.  Uniform is the first langauge which simultaneouly provides <i>BOTH</i>, at the same time
            over the same structures.</li>
### --- SCRIPTABLE ---
### Homoiconic
Homoiconic – Uniform Source-code encoded as easily machine-edited data
### Declarish
### Meta-scriptable / retro-programmable
META-SCRIPTABLE -- Supports scripts that process other scripts.
### Complete
 COMPLETE – All important constructs for modern software are expressed.

 <li><d>Complete</d> &mdash; all important aspects of modern software must be constructable from Unicore.</li>
   <li><d>Complete</d> &mdash; all important aspects of modern software must be constructable from Unicore.</li>
### Practical
### Performant
<li><d>Performant</d> &mdash; the generality and simplicity of Unicore must not come at a dramatic performance cost.</li>
### LIVE
    LIVE -- Facilitates declarish relations in lieu of procedural specifications.
    
    LIVE -- Parts are interconnected in a way that a user need not think about how things connect too much
   they just change that part they want to be different, and the rest of the system takes its cue from that change
   and adjusts accordingly.
### UnForked
A langauge is **_Unforked_** if the forking of its constrcuts has been limited to the greatest degree by logically necessity based on the semantics of the language itself.
### .
.
.
.
.
.
.
.
.
### --- OTHER ---
#### Old Summary
BEST LWL --
-- BEST SYNTAX & AST --
    -- normalization trick
    -- NATURAL --
-- DECOMPLECTED -- decomplects its parts so you can used them as needed in each context w/o retrofitting & w/o unintended other stuff
-- RETRO-PROGRAMMABLE --
-- EMBEDDABLE
    -- FITS --
    -- SIMPLEST --
    -- SINGULAR --
    -- SPROUTY --

KEY DIFFERENTIATORS -- K
-- OBLITERATIONS --
-- SINGULAR --
-- CAKE EATING --
#### JSON
JSON – Uniform’s data is JSON, and its computation is “embeddable” just as JSON.
#### Making all a POD
## ===1=== DIMENSIONS OF "BEST" COLLABORATION ENV ===
### UPSTREAMING
 WHY? "UPSTREAMING" -- Produces 100x or 100,000x improvements when possible
### WORK DONE IN BEST CONTEXT
-- All K work done in best context--in designer interface.
### RIGHT K RIGHT TIME
### Micro contributions
-- Super contextualization. (precise, automatic)
   allows tiny contribution since the mandatory overhead of situating that contribution is minimal
-- nominialization -- naming everyting so that you can easily reference 
-- re-grouping
-- upstreaming
### RECOMBINABLE
-- Embeddable/simple means works well with others
-- Pairwise interoperability focused on.
-- Decomplected so parts can be recombined to fit assumptions of any target language
### INTEROPERABLE
shared common substructure makes DSLs much more interoperables.  best of DSL w/o fracturing usually caused.
### LOWER FRICTION
-- Effortlessly Share All Knowledge Work.    
-- Effortlessly collaborate on all knowledge work
## ===2=== Approach ===
## ===X=== Stories ===
### Live
### Collaborative
#### Micro contributions
#### Non diminishing
## ===X=== Key Ideas ===
### Seperation of Structure and Function -- and ascribing meaning to data
- many modern langs move towards unbundling of methods from datastructure
	(like closure, go)
- but dont go far enough or do it cleanly enough.
- uniform allows total separation of structure and FN
	- a implementation level interp is done in context of an implied str self
		and implied fn context ops.  they can be tied together, but can also be unbundled and rebundled at will.
- This means one can refer to x1 as a class c1, then x2 the structure in x1, and x3 the instance of c2 with the same data as x1 but with new meanings
- We leverage RDF, but reject the URI binding semantics assumed as providing meaning.  We accept this as one of many ways meaning might be ascribed to RDF triples, but not an annointed holy one.  
- by themselves triples have no intrinsic meaning, but meaning can be associated (and disassociated with them)  
- ABILITY -- one part of the code associated a 'precedence' with symbols making them tokens, but another part of the code does not 'see' these same methods even on the same in-memory objects.
	- Bare native strings are made into symbols by overlaying the 'head' method.
	- 
## ===3=== BENEFITS / VALUE PROPS ===
### ---KEY VAL PROPS---
#### One Click
- The ability to (1) GIVE knowlege products, (2) INTO a user's context, (3) AT any grain size big or small
#### SUMMARY LIST
- SHIMLESS -- 
#### BROAD CLAIM
CLAIM: SW collaboration will be distruptively improved when executes as a web of DSLs.
CLAIM: WEB DSLs -- Benefits are Disruptive when software world is web of DSLs.


WORLD WILL BE DISRUPTIVELY DIFFERENT
-- when the nature of the languages we use are different.
-- when the way that we interact around shared knowledge work is different.

KNOWLEDGE WORK WILL BE TRANSFORMATIVELY DIFFERENT WHEN BUILT FROM A UNIFORM, SHARED WEB OF DSLs.

BETTER WORLD -- World will be transformed when
-- All K work done in best context--in designer interface.  max upstreaming.  All can be a language writer.  Why?
-- Open source can get paid for its value 

SHARING ACROSS A UNIFORM WEB OF DSLS
-- fixing the second haplf otbhe problem – fixing thw ways we interact around shared K work
### -BENEFITS-FOR-WEB-OF-DSLs-
### SHARP SYNTAX
Sharp combo of almost declarative simplicity for new syntax -AND- use of full range of syntactic methaphors 
-- markdown w. programmable extensions covering full range of syntactic metaphors (easily coded)
-- more easily coded than anywhere else
#### BEAUTIFUL SYNTAX
-- Universal syntax look has an <q>effective</q> choice for beautiful structures across most all languages
### -MARKET-PLACE-
### Financial logistics (payment, assessement, piracy protection)
### SHELLING
WHY? "SHELLING" -- Benefits of sharing w/o limitations imposed by tyranny of authorship
    --  Micro Slicing
    --  CONTEXT
    --  RIGHT K RIGHT TIME
    --  PAY mech
    --  Distributed consensus
### -A-BEST-BASIS-FOR- ...
#### EXPRESSING DECLARATIVE INTENT
-- Lex is a practical langague for expressing declarative and imparitive intent.
   the smallest details were focused on, in thinking thru the confusions and complexities of scripting in order
   to reduce those burdens well below other langauges
#### ESSENCE CAPTURE
-- Best basis for capture essences from other langauges (recombinable, no hair added)
#### LANGUAGE BRIDGING
-- Best language for bridging
-- Lex is a better bridging langauge -- it decomplects its parts so you can used them as needed in each context
   w/o retrofitting & w/o unintended other stuff
### .
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.ma
## ===5=== WHAT TO DO WITH IT ===
### SELF WYSIWYG RE-PROGRAMMABLE DSL EDITOR
### LIVE TEXT
GIT/EDITOR BASED, Grammar driven, live text processor
-- Tie Txt/Html/Markdown/Source Code/File Tree
-- Jira Ticket/... 
#### What could you do?
-- edit markdown summaries which are bi-jected to HTML
-- hyper programmable HTML doc output
### KM Processor Tool
### SW config
#### Deep net graph generator
#### Lightweight GPU context programming
### --TO DO SOCIALLY--
### --TO LIVE--
## === TRICKS ===

--  LEXSPACE -- lexspace addressing avoids corner cases because lexspace has no corners...  infinte and immutible in
    all directions.  So all structural operations are defined, and all paths exist and are unique.


-- CODE SNIPITS -- make smallest wff be a "disembodied code snipits & implicit reference to self" dramatically cut down on boilerplate,
    (1) removes code trimmings, (2) var refs can be 30% smaller, (3) many functions collapse onto each other MCF avoided, (4) get function algebras for free saving many boring methods in many DSL contexts
    get_failed_person().block()

-- SEMANTICS FREE PARSING -- mapping from text to data-structure is semantics free
            (parse&print do not depend knowlege of language terms -- req for homoiconic)

-- ASTETICALLY COVERING -- contains <q>complete-ish’’ set of syntactic variants such that for any specialized language, L, there exists a Uniform sublanguage L’ that is semantically equivelant to L, and having the same astetic attributes and comprable <q>beauty’’
## === AHA MOMENTS ===
### --- EXEC REDUCTIONS 2020/01 ---
Pinning down PKG, EXE, and VAR was driving me nuts.
Finally started working on all at same time, groping and not feeling good about it.  Days of waking in the morning thinking about one piece or another.

During this time:

#### --- Removed ALPHA from interp ---
Actually before the beginning of this period when I was focusing on VAR, I realized that 'Place' API would remove the need for a separate ASSIGN and ACCESS eval types.

This coupled with ideas around virtual functions being needed to implement VARs, multi-inheritance and other.

A HAH -- we don't need ALPHA, these are really just virtual functions over the space of all units.

#### --- Removed recursive expansion in definition of INTERP ---

A HAH -- seemed to come about when thinking thru what structure exists after loading.  Realized that if I allowed 'load' operation to do part of the semantic expansion, then all interp would be single level expansion reduction.

#### --- EXEC as THUNK ---

Days of confusion about what exactly is in memory post load for things that had effect (e.g. operator definitions)

Began to dispair that the two-level spec/exec was just introducing lots of redundancies, seemed that  blk would just map to Blk() and not provide much value.

Was very confused about what would exist and how it would 'run' for things like a function. 

Very confused about how these things would appear inside a conventional env (like python).  The problem is that python has a standardized evaluation of structure followed by a __call__ step that did not seem to map well on to the expand /reduce model used by uniform.

In digging to what would exist at the bottom I just kept chasing my tail.
but was not even clear what specific problem I was trying to fix... it just felt very ugly and ad-hoc.

A HAH #1 -- Realized that a natural break is to have the a Exec thing execute the BODY only of fn/operator.  This pushed stack frame marshelling to a separate operation.  

In writing out the code for this, remembered it connect to matchers.
Thus started thinking of these Exec as 'matchers' too.

A HAH #2 -- relized that 'reduce' is an operation of two arguments the Exec and the Env.  realized this seemed to have connection to functional notations (like pred calc and compositional models.)  

A HAH #2.5 -- all execution can be the result of argless invocation of 'reduce' operations.

It seemed a little cleaner to just have one .reduce operator on the Env object rather than having a different .reduce operator defined on all Exec forms.
Then realized this could help the separation of structure and function of Exec.

A HAH #3 -- All execution boilds down to the 'reduce' operation applied to various Exec forms.




_
## === EXAMPLES: AWESOMENESS ===
### SYNTACTIC -- BEST SURFACE+AST
-- No lang I know of has combo of machine awesome AST and human awesome Text form.
-- No lang affords eye-pleasing variety in surface forms while preserving uniform AST form
### SYNTACTIC -- EASY, DSL-zy
-- Super quick to create DSL optomized surface forms.  (e.g. directly embedded rule forms, regex-ish, etc.)
-- Created DSLs auto-magically interoperate since (1)all based on same semantics-free reader, (2)all parse to unit form
### UR-FORM -- Information compressed into UR-form 
-- easier to understand and operate on info expressed in mathematically simplest form
## === EXAMPLES: DECOMPLECTED ===
so separated one can use any part w/o introducing any extraneous semantics
    --  OPTIONS -- The variation in specification of options, from the constructs being optioned
    --  GROUPING -- The variation in
    --  BACKING -- The storage implementation of data from the structure of the data
    --  ASSIGNMENT -- Assignment semantics ".value" is decomplected from the backing below it and all update constructs above
    --  LANG -- Changes to underlying language vs. code written in the langauge
    --
## === EXAMPLES: >OBLIT / SINGULAR / UNFORKINGS ===
### List
#### Data
- GET -- the only way to access any data of any kind
- SET -- the only way to modify any data of any kind
- UP/IDX -- the only way to express containment 
- ORD -- the only way to express notions of ordering

- Stream -- the only mapping between time and space  
		(generators, listeners, loops, i/o-streams, iterators, loggers) not unified w/ each other or with arrays
- Lex -- the only notion of "place" for use in assignment semantics
#### Primatives
- BRA -- the only way to branch
- BLK -- the only sequencing operator
- CHN -- the only invoking / chaining operator
- REP -- the only looping operator
- RET -- the only non-local control flow

- PKG -- the only way to group (package) source code
- CTX -- the only declare 
- OP
- VAR -- 
##### VAR
fn arg var, local var, 
Generators, Iterators, Streams, Variable Histories, Listeners/Hooks
### GALAXIAL OBLITUARIES
A oblit is galaxial if its adoption has a pervasive effect on the Uniform langauge

#### LEXSPACE DATA EMBEDDING
All data things are "written" into lexspace.

The pervasive consequences of this cannot be capture easily.  Dozens of simplification and uniformizations occur since all data can be operated on in a simple consistent way.  Many oblits here are consequence of this choice.
#### WRITTEN SEMANTICS
Tying meaning of data to its mapping onto Lexspace drives use of 
    Bindings, underlies Assignment, and structural addressing.
#### UNIT TYPE
Attaching most operators directly to the root type Unit dramatically shapes language since most kinds of actions can potentially apply to make kinds of things.

Single choice obliterates hundreds of datatypes, but also often obliterates containment layers within most every kind of complex object.  A file object does not contain its contents.  it **IS** its contents.  in exactly the same way that a list IS its contents.
#### UNIT BACKING
All things are mapped into a very general unit frame (with many optional/alternate properties) allows commonalities to naturally be shared since they show up the same in the language, but also allows for alteratives to be fully expressed as well.  (e.g. ordered vs unordered; finite vs infinite; positional vs named)
### OBLITUARIES


--  Fused Forms (No cracked abstractions)
--  LIVE -- Lex is *live* all parts are interconnected in a way that a user need not think about how things connect
    too much they just change that part they want to be different, and the rest of the system takes its cue from that
    change and adjusts accordingly
--  VAR -- single placeholder class def; template slot; ...

    AST is not just parse tree.  but it is the simplest conceivable parse tree.  e.g.  tree of lexical scopes; of folders for remote data spanning different data types

--  is it a list? a map?  false fork!  its a unit w. subunits.

-- Sprouting -- Extreme conceptual and implementational reuse allow entire ediface to be planted in it entirety.
                JSON Data, 10 primitives, single data type
#### Static / Dynamic
Every 'static' thing will be dynamic within some larger scope.
Every 'dyanmic' thing may made static within some defined scope.
#### Singular Control Flow
All control macro expands into SEQ/BRA/REP/RET (CHN)
    Sequence, Branch, Repeat, and Exit
#### Generalized Stream/Unit forms
asUnit and asStream allow one to write code that unifies these dual representations for both generators and instantiated data forms.  (e.g. all four possibilities are unified:  instantiated-unit. instantiated-stream. generated-unit.  generated-stream.)
#### STREAM/Structure duality
Iterator, Generator, FileAccessor, 
--  Generator vs. instantiated
## === EXAMPLES: INTEROPERATION ===
Destructuring = Address + Assignment
## === EXAMPLES: FALSE FORKS ===
    <q>When You Come to a Fork in the Road, Take It!</q> -- Yogi Berra
    There is never a "right" answer to software choice in cases where adherants exist for multiple path.
    Any software choice with adherants multile

--  Beautiful AST & Beautiful surface form
--  Declarative & Reprogrammable build
--  Strongly-weak-declarative-duck-typing.
--  whitespace-sensitively-free parsing
--  homoiconic & heautiful syntax
--  impartively-declarative "declarish" programming
--  hi-low-retro-expansive APIs
    powerful -- simple
## === EXAMPLES: CAKE EATING ===
### Clean Levels
-- Understandable surfax syntax VS super re-programmable syntax
-- Dynamic linking, recompile, etc.  VS  Statically analyzable code
Monkey patching

 anything anywhere / understandable surface form /declarish syntax / reliable 
### python AND JS.
### !!! POD and Objects
### !!! beautiful syntax AND beautiful ast.
### Whitespace [IN-]sensitive parsing
### Pre-rolled, roll your own
## === EXAMPLES: CAKE EATING MANEUVERS ===
### SPROUT IT
If two constructs are similar then invent a third construct which can be used to precisely derive the other two constucts, then only include the newly invented construct in Uniform, and allow the others to "sprout" from this UR-form.
### SPIRAL IT
If there are multiple levels of langauge complexity that are appropriate for different contexts then invent a telescoping sequence of languages where each language embeds into all languages later than it in the sequence.
### FORK-PLECT IT
Here we actually introduce a fork in the language, but compress that fork into the smallest corner so we "maximally" minimize the damage caused by the fork.

Given two usage context A and B that appear to require INCOMPABLE language constructs, then:

(1) ARTICULATE CHOICE -- Identify and articulate the SINGLE choice which logically must be made differently between these cases.

(2) ESTABLISH IT IS A TRUE FORK-- Try really really hard to make this fork go away!  Consider other ways to think about the choice, to think about each of the incompatible cases, in order for this fork to NOT FORK.

Lets assume you failed -- this is true fork -- and you can clearly explain why there is no way around it:  We need case A and case B, and all formulations of A & B clearly result in incompatible paths taken on choice C.  We have a true fork.

(4) ARTICULATE ALTERNATIVES -- formulate a for A and b for B as values for choice C.  Based on these choices for C then each context can be formulated based on the resulting langauge.

(3) DECOMPLECT / SHRINK -- Decomplect choice, C, as much as possible form ALL other aspect of the uniform language.  Try hard to formulate remainder of use contexts A and B so they are fully parallel except A uses 'a' for choice C and B uses 'b'.

**Make the fork as SMALL as possible.**  
As much as is logically possible, one should be able to express as much as possible WITHOUT commit to which alternative was chosen for C.  This fork does not cause dimunitional damage in all cases where a programmer can express code that does not depend upon choice made for C.

Their code remains "unforked" on C.

Needlessly connecting **ANY** aspect of any part of Uniform to any specfic choice on C which is not **REQUIRED** by logical necessity steming directly from the semantics of those constructs is an unforced error -- typically collossal in magnitude.  

For it forces all programmers for all time to needlessly fork all code that depends in any way on these needlessly connected aspects.

(4) FORMULATE usage contexts using minimzed versions of choice C, and verify that it indeed does support all and minimally fork Uniform code, as well as has all the other nice properties we hope for in a langauge.

If not, this fork-plect failed, flush it, and try again or try something else.


#### An example might help here

Lists are structured forms and maps are structured forms.  
But they are incompatible in multiple way ... so nearly all programming languages unthinkingly fork these into two incompatible datatypes.

In one unthinking step they forever **DOUBLE** the effort that all programmers must expend creating and maintaining code that could otherwise operate on 

...
### PAIN-FIRST-SEARCH
Pain-First-Search is an approach where one 
-- Identifies and narrow defines a source of greatest conflict
-- Consider reformulations that break the center of the gordian knot
-- Then for each reformulation consider the closure of conseuqneces
-- Finally assess pros / cons of each
-- Tentatively decide approach    
   (In uniform as in science, nothing is ever decided with finality, only with ever greater conviction )

COMMON APPLICATION OF PFS
-- Write most natural one sentence meaning for a notion.
-- Write shortest most beautiful source code usage for top few most impt/common usages of notion.
-- Consider few most impt/common interoperations of notion with other notions.

**SIMULTANEOUSLY** achieve ideal 
## === EXAMPLES: MEASURES OF GOODNESS ===
### Perplexity
temporal perplexity -- num seconds to execute and action (for a specific person with specific training)
spatial perplexity -- num unit to express an idea (given a specific background information)
## === EXAMPLES: UNFORCED ERRORS ===

…….. But first lets enumerate our unforced errors
-- FALSE FORKS       (Oblituraries)
-- COMPLETIONS OF CONSEQUENCE
      -- UNCONTEXTUALIZED EFFORT
-- FAILED UPSTREAMING
-- BOILERPLATE FRICTIONS
## === EXAMPLES: EVILS TO AVOID ===
### Tyranny of authorship
## === UNSORTED ===
### Topic -- your users _will_ create languages for their users
- does your syntax help or hinder this important work.
- afterall it is your user's user's that spend vast time coding.

js-react, DB connectors, DOM connectors, 
libraries, math libs, 

boost container libs, 
## === Python Inconsistencies ===
x instanceof y	==>		x =~ y			reader has no reserved words
and or not		==>		&& || !! 		reader has no reserved words
x if c else y	==>  	if(c, x, y) 	reader has no reserved words
[x:y] 			==>  	[x|y]  			colon already has meaning 		slice notation
@  ???
for k,v in l	==>		for v,k in l 	wierd if optional keywords occur BEFORE mandatory args
## === Disucssions ===
### --- Guido / Jupiter Notebook ---
russel Keith pycon gil Mcgee 
- Interested in specialized DSLs for collaborative K work
- Convinced there is a gap -- getting first contributors
# ### FRAMING ###
## === PROBLEMS/VALUE PROPS ===
### HOMOICONIC / Meta-scriptable
### EMBEDDABLE / DOM scriptable / Live Data
### >>> COLLABORATION PROBLEM AREAS <<<
Pain-points when collaborating over larger systems
### RTRT - The Right Thing at the Right Time
Having exactly and only the right information at ones finger tips at the exact moment it is needed.
#### Application Areas
- Stareing at a bug/unexplained behavior
- First using a new API/Framework
- Understanding one small part of one big system
### FLEET - Managing a fleet of instances
Managing commonalities across fleet of instances 
### TRANSFER - Applying value expressed in one context to another
## === SOLUTION TRICKS ===
#### SHIMLESS SHIMMING

In the case that both A and B are realizations and each A requires exactly one B, then the shimless shimming pattern has A be a B.

(Is this just sub-classing?)
#### CONCEPTUAL ROOTING
If a constellation of configuration and behavior can be thought of as a "component", c, which has two or more possible instantiations, a and b, then there should be a single root in the code (a path) for the specification of each instantiation.  

Then using this it is possible to fully switch between instantiations by simply changing the binding 
c <- path.to.a  or  c <- path.to.b

This should be true at all levels in the code where such distinct concepualization exists, even in cases where currently only one instantiation of the conceptualization exists.
## === VIRTUES ===
### --- Language Virtues ---
GOAL: Be the best Language writing Language
### --- Scripting Virtues ---
collaboration of one -- super embeddability
GOAL: 
#### Greatest flexibility and ability
#### Least cost measured by training time, and execution time
#### Maintaince minimized
#### Understandability maximized in LPOLRD way
#### Scripting Principles
### --- Collaboration Virtues ---
#### intro
GOAL: Obtaining greatest collective utility for least collective cost.

Key Tension: the specific and the universal
- coding is most understandable and best guided when specifically applied
- code is most valuable when it is applied most broadly
#### >>> SPECIFIC <<<
#### Instance Driven
It is often beneficial to develop/use SW in an "instance driven" way.  Supporting collab for this is a virtue.
Dynamic instance driven --  using/configuring a single executable to support your goals over time.
Static instance driven --  developing apps for single business context.
Either way its desirable to connect and leverage implicit relations between this others for in/out-flowing value.
##### Instance Unification
Ability to express two related instances as a single instance and with two specializations in a post-hoc way.
Importantly because of LPOLRD this should not depend upon little or no apriori prep work
##### Instance Lifting
Given an instance and a class the instance mostly fits into
Re-express the instance as an application of the class possibly with patching in a post-hoc way.
##### Instance Driven Scripting
In many cases the LPOLR is to make a small edit to an existing running app constellation, or existing code-base at any level, breaking any encapsulation boundaries designed by authors in order achieve the short term LPOLR objective, and any "clean" refactoring of code or components will NOT be small relative to the cost of LPOLR so it wont happen.

Static or dynamic patching will happen -- instance driven scripting embraces this reality.
#### Decentralized
Decentralized Decision Making
- The larger the group required for consensus the harder the consensus.
- The larger the group to optomize over, the harder the optomization.
##### LPOLRD - Local Path of Least Resistance Driven
#### Power To The People
##### upstreamable
#### >>> FIXABLE <<<
#### Forward Factorable
 -- Retro-Factorable -- unfuckable
 A features is forward factorable if it minimizes or removes an author's abilty to lock downstream author's into choices.  Forward factorabilty ensures that choices are indefinitely revisable.
#### Retro-Factorable
#### Transparent
 -- forward factorings -- lateral translations -- pullup xlations
#### Unfork Extensible
Opposite:  A 'future forker' is any constuct choice that causes using authors to fork their downsteam authors.
for example, final atomic data types are big future forkers.  They encourage authors to construct APIs using them.
These APIs are ripe for forking, since one cannot annotate or extend these parameters, one must fork the API itself.
#### >>> PARTS <<<
#### Shimless
Shimless constructs can be combined without require "shim" code to make them fit together.

Levels
* no shim code required.
* shim code is completely hidden
* shim code is exposed
* shim code must be written by the one combining components


#### Spiraled
Expressed as a set of nested interfaces with ordred complexity

Types
* sprouted
* additive
* incompatible
#### Declarishly Instance Delta Encoding
Expressing variations across a set of variants as a prototype with a delta for each variant.  Aim is to produce minimal encoding of entire set.
- .
- mini expressions often at different interp levels -- min requires finding and using smallsest pmep
- min delta expracton
#### Convergent
 - linear
Assume large web of mostly independent efforts with zero attempts at redundancy removal, consistency, 
given desire to improve towards these desired properties there are linear paths of improvemment,
and limits of final result are as good as nature of computation allows (in terms of transparency, simplicity, back compatibility)
#### Contextualized
having all the best info and abilities for given task given line of code, given person
#### Guiding
systems causes each person to do the best thing for them and for all
#### Biggest boom
each hour has greatest impact for self and for community as it could have
#### Middle Out Programming
Specifying with minimal assumptions regarding its domain of application.
Many claim this goal, but don't have languages that support it.
Can you write code that is GC-aware and GC-free?  that needs maps but not arrays, works with in both lazy and eager evaluation contexts, using unsigned integers less than 64K with + and * defined but no division ?
- if you could it would be effortlessly reusable in haskell, python, in ASIC programming, 

==> MiddleOut Programming only require those features they use, thus can be imported into any context that has those features.
==> Traditional "Bottom Up" programming by contrast starts by selecting a single version of a single programming langauge and by virtue of that, has already made thousands of assumptions that implictly make it impractical to resue elsewhere
### --- Market Place Virtues ---
GOAL: A Market Mechanism that maximizes collaborative virtues
- based on decentralized small group decision making
- based on LPOLR

#### Post contribution pricing
### --- Societal Virtues ---
GOAL:
## === APPLICATION AREAS ===
### KM -- Manage systems / data pipes / data visualizations
### DECLARATIVE CONFIG DSLs
- Configuration languages are complex
- Powerful when declarative
- BEAUTIFUL -- Beautiful Syntax & AST -- valuable
- DECLARISH -- Declarish scripting is better than declarative or procedural scripting
- GUIDES -- Strong guides 
- COLLABORATOIVELY
# ### MOTIVATION/APPS ###
## === ARGUMENTS ===
### CONTEXTUALIZED KNOWLEGE WORK
- K work is more effective when done within an optomized tooling environment -- and the difference is very large
- But different kinds of processing requires different languages/enviornment -- can create large speed bumps
- Ideal compromise is seperate enviornments that share as much substructure as possible.
- Thus build environments from library of independent, interoperable, elemental components.
### CAN DO MUCH BETTER
CAN DO MUCH BETTER -- Many software languages are filled with tradeoffs that are not inherent consequences of the nature of computation, but rather are ``unforced errors’’ of language design.

-- Many software languages and frameworks are built from design choices which are not inherent consequences of the nature of computation, but rather are idiosyncratic or historic ``unforced errors’’ of language design.
-- Worse still, the negative consequences of these unforced errors tend to compound so the costs multiply when they interact.
#### > UNFORCED ERROR
UNFORCED ERROR -- An ``unforced error'' is any language design choice that could be significantly improved without introducing any significant offsetting drawback.

An ``unforced error'' is accepting a significant limitation without an offsetting benefit 
#### XXX CLEAN SLATE
-- Fixing complexity design errors often requires cleanly rebuilding from the bottom up -- one cannot “add on” simplicity to fix complexity.

(Fixing this requires re-building cleanly from bottom up – one cannot “add on” simplicity to fix complexity.)
Starting over it is possible to mostly ``have it all’’ – an language writing language w. many advantages and few disadvantages – whose remaining tradeoffs are true tradeoffs inherent in the nature of computation – with fewer or no unforced errors of language design.
#### XXX HELL ON EARTH
-- What is SW hell?  We are in hell.(consider the list API)  
-- There is a way out. Split/merge/simplify  (consider list)
--  Arrive at a best best basis for computation –  a best  lang writing lang – what would that look like?
--  what benefits would such a language confer?  

-- list of essences
### AGENDA SENTENCE
UNIFORM AGENDA -- Create a "best" language writing language as a collection of independent, interoperable, essences, and use it as a basis for transforming the way collaborative SW is developed.

-- Uniform Agenda -- Create a langauge writing language as a collection of sharp, independent, interoperable, constructs, that is a "best" basis for driving collaborative software development.
-- Uniform Agenda -- Create a "best" basis for computation as a langauge writing language built from a collection of sharp, independent, interoperable, constructs. 

_
#### >Four Questions
FOUR QUESTIONS:
    1.  What do we mean by "best"?
    2.  What process are we proposing?
    4.  What are the transformative benefits?
    3.  What is the resulting language? 
#### Clarity
>>> to move into properties
Each construct has strongest advantages with minimal disadvatage _simultaneouly_ across relevant language design dimensions.  
Where each construct is _"maximal"_ -- the nature of computation itself requires offsetting disadvantage for any further improvement along any relevant language design dimensions.
Each construct is "sharp" - it simultaneously achieve maximal advantage across the range of relevant language design dimensions, such that any further improvement can only occur by introducing some offsetting distadvantage.
#### >THEN WHAT
-- Full realization of the Uniform agenda is impossible.
-- Still much progress can be made since each construct can be attacked in isolation, and each must be as simple as possible so the design space must remain small.
-- In any case, one cannot make effective progress towards any goal without first clearly defining the objective.  That is what we do here. 


~~~~~~~~
Relevant Dimensions
(We try to acrue advantage with little or no disadvantage along other dimensions)
A strong bias towards simplicity strongly constrains this process -- its 
### GILBERT EMAIL 

ASSERTIONS

-1-  RIGHT TOOL FOR THE JOB — Knowledge work should be done in an environment specifically designed for its specialized type of knowledge.  This includes the DSL, IDE, Debuggers, Docs, KM tools etc.

-2-  GLUE DOMINATES — Cases where knowledge, tools, DSL and paradigms come from different worlds the "glue-code" between those worlds becomes an increasing burden within the whole system.

-3-  MINIMIZE INCIDENTAL DIFFERENCES — These dominating gaps between paradigm/DSLs come in two varieties:  Intrinsic gaps which are the result of fundamental choices made by each language, while incidental differences arise for historical or other reasons and result in glue-work which is not inherent to the gap being bridged, but exists simply because of avoidable differences between paradigms.  (e.g.  CR/LF  verses  CR, or Fahrenheit vs Celsius).  Incidental differences add to the dominating glue cost, and add not intrinsic problem solving advantage so they should be minimized.  (While allowing intrinsic differences to flourish)

-4- SHARED CONSTRUCTS — a way to ensure that distinct DSLs do not needlessly diverge is to build them from common components shared by both languages in cases where those components are orthogonal to the specializing aspect of each language.  (e.g. perhaps both languages utilize a common 
‘Assignment’ paradigm, or a common encapsulating or scoping paradigm. 

——

Consequence

CONSTRUCTS LIBRARY — Suggests a library of a-la-carte constructs which might be mix and matched in building appropriate DSLs for each specialized domain.



~~~~~~~

My take, if I am channeling my internal ``Gilbert Model’’ is that you would agree in principle with the agenda, but then in practice be skeptical that these constructs would be meaningfully interoperable, and whether they would provide much support in building the DSLs.  In the degenerate case they only cover 10% of the DSL, but 90% is idiosyncratic semantics that does not map well into any other worlds.

Both challenges seem like they could be correct (and killing blows to the idea).

But I also wonder if this is not a difference in domain of application.  That is, if you goal is building high performance, domain specific, processor specific DSLs, then overlaps might get thin, while I have focused on Divergent domains of domain knowledge as the key separator.  In that second case, performance  is less central, so maybe common paradigms might flourish more?


Anyway great food for thought!

I will write a second note, to follow up on the pointers you gave me.


Thanks for the time,
—dan
### HOW SOCIETY WORKS
#### 4 realities
Four Realities of the Present system
- **MARKETS are NON-LINEAR** -- Markets are inherently non-linear.  All things equal, a buyer will select A over B if its price is .01% cheaper, or has one tiny feature better.
- **KNOWLEGE WORK only NEEDS ONE** -- Knowlege work, unlike physical work only requires the creation of a single good of each kind.  Lesser versions have zero, or negative value.
- **COLLABORATION has N-SQUARED COST** -- Collaboration requires communication and consensus.  The cost of each of these is N-squared with the size of the group.
- **NETWORKS provide N-SQUARED VALUE** -- The relative value of two competing networks, all things equal, is something like the ratio of the square of their sizes.

The Consequence:
A winner-take-all world comprised of a select few that contribute enormous value as measured by the system, and the unwashed masses whose value is NEGATIVE as measured by the system.



_
#### === Long Parenthetical Related to your proposed solution ===

Thank god your do-gooder views are safely tucked away at college where hopefully their effect on the minds of future leaders will be minimal.  For if you managed to actually sway belief in future leaders that the moral course of action was to be inclusive and to be sure to hire their fair share of lesser talent into the SAME ROLES as the greater talent, the result would be catastrophic within the present system.  The result would be two kinds of companies:

Immoral companies driven by immoral leaders -- given the winner take all nature of markets of collaborative, networks of knowlege work, these companies would be at a collosal advantage, they would ballon in size absorbing the bulk of available work, and thus also the bulk of workers.

Moral companies driven by moral leaders -- would be lucky to get the scraps left over.  Most would actually just fail.  They would spiral downward (as we see that even modestly badly run companies today do) having less work, and thus less network, less products better than all others of the same time.  DEATH.


~~~
I grew a team of 60 at Aeolus... in the beginning I was able to keep a firm hand on the hiring, and I got the very best I could find across two continents.  Later in I delegated hiring off, and also allowed some growth when the candidates were good, but not the best of the best.  The results were profound.  Even that small shift attention to excellece had a huge effect -- the teams just could not deliver.  And even then we were NOT hiring people just to "do our part"  oh my god, if we did that......   it would be just DEATH.

~~~
Right now we have a team of about 70 at my consulting company.  We have grown 30% year over year.  This is because we are freaking amazing (honest).  We just walk on water relative to our peers in contract work.  That said, there is a very real chance that markets/customers shift and we die.  It is a winner take all world, and we are still small enough that one good wave could pull us under.  The the plateau of explosive growth/riches is tiny and surrounded by vast canyons of mediocraty its edges are curvy and very steep.  Those on top are running fast, but doing everything humanly possible to not fall off.



... every company should do its fair share of hiring lesser talent ...

:-)   yeah our paths have diverged... that is such a foreign thought... it is just shocking how wrong it is.

_
#### Affixing Blame

Humans tend to fix blame on the NATURE of other humans.
- **Liberals blame the powerful** -- oh if they would just be less greedy, then the whole system would work just fine.
- **Conservatives blame the weak** -- oh if they would just get their freaking act together they would be rich too, and not be a drain on society, all would be just fine.


But human nature hasn't changed, as a group their behavior is a consequence of the rules of the game they are part of:
- **Labor waxes** -- from 1800 to 1970 workers became ever more skilled in PHYSICAL WORK, contributing ever more value relative to the value of capital.  The plight of the median worker improves.
- **Labor wanes** -- from 1970 to present the value of physical work and capital both plummet relative to knowlege work.  The plight of the median worker regresses.


We can wring our hands and decry the corrupt nature of wall street; we can wring our hands and decry the corrupt nature of the poor.  Or we can look reality in the face:

Recognize that differeneces in the circumstance of different groups of humans are largely not a product of differences in the nature of the humans in these groups, but rather differences in the nature of the circumstances of the group itself.

(This might sound like I just sided with the liberals, but I did not.  Since, in my estimation, one of the largest systematic differences between groups of different power, are the cultures those groups have.  And conservatives love to blame bad culture in the weak groups.)


But even if the concervatives are right to blame the culture of the weak, we must ask.  Why do the weak have that culture, if we believe human nature is not changing, then what is?

     The nature of the system they are part of.


So it seems the only way forward is to change the nature of the system itself.  
#### Systems and their failings

ENTITLEMENTS -- created with good intentions but have catastrophic unintended effects too.  Good intentions:
- LIFTING - we want a society in which (somehow) citizens in worst situations trend upwards out of them.
- BACK STOP - we want a society in which (somehow) the most preventable catastrophies tend not to happen. 
Unintended Effects:
- DISEMPOWERING - 
- ENTITLED - 
- INCONSEQUENTIAL - 
- MEANING - 

Society is beginning from the wrong spot -- the way we collaborate is fucked up
- Entitlements --
- Libertarianism --    free-market -- Self Sufficiency -- 
- Captialism -- Sustainable Differentiation


capitalism idea -- money flows to place of greatest impact.

What is working
- Decentralized
- Ownership, Meaning, Consequences -- 
- Support 
- Democratize value creation

Needs Fixing
- Winner take all
- Negative greater than positive value
- 

https://www.nytimes.com/2019/04/26/upshot/women-long-hours-greedy-professions.html

_
### --older--
#### Agenda -- older stuff
Uniform Agenda – Start from bottom – build best language writing language.
SW is transformatively better when expressed as web of interoperable DSLs
 – Ultimate language writing language is most simple, decomplected, interoperable language.
Agenda – Create simplest most uniform 
**Ultimate Langauge Writing Language**
UF ultimate-lang-writing-lang.  Will accelerate trend (lower friction).
-- Lex is a language for writing other langauges -- homoiconic -- flexible syntax
#### Agenda -- Try #1 notes
THE UNIFORM LANGUAGE AGENDA
* fixing the first half of the problem – fixing the languages we use
Maximal Uniformity -- where does this agenda lead?
What would a Maximally Uniform Language look like?
-- an open-ended collection of interoperable, independently-formalized essences.
Each essence is encoded in a form so spartan that every part is essential this cannot be removed or altered w/o being a different idea.
Each essensec is encoded ina form as interoperable as possible with all others.  Intersecting language and ideas need to mesh in a way that all meaningful combinations flow naturally w. little friction.
Each essence is encoded in a form as independently as possible from others.   Dependance on universals is ok.  Dependance on choices which could legitimately be otehrsiwse is costly causing multiplicative forking.
Occasionally these forking dependencesiar are inherent in the nature of computation itself… these must be accepted, but most are not…they are actually false forks and there is a way to be independent and interoperable.
#### Resulting UR langauge
Independent Pieces – Two a shocking degree each of Uniform’s concepts are formulated in isolated ways.  So ``control flow’’ is formulated as 5 primitive flows.  These formulation is completely divorced from the rest of the language being ``flowed.’’ 
 
An example trajectory:-- The specification of optional parameters on constructs are decompleted from the constructs-- Distinctions between different kinds of options and ways of specifying are obliterated.    Leaving a few pure alternatives (e.g. dynamic param spec verses lexical param spec)-- Each remaining formulation is purified to encode only the essence.-- Options for all constructs are now expressed as combinations of these pure option forms.
#### CHANGES EVERYTHING
Doing better will result in transformative improvments in collaborative SW development
## === STUFF ===
### Pithy Statement
#### Poem
Trudging a purgatory of endless ranges of fetid complexity 
in  landscape of ossified twists and turns born from the weaving and dodging of trees long forgotten.
Like a breathing feeding coral the present generation entrudes its deposits in fracile complexity born of the weaving of the doging of the weaving endlessly back to a long forgotten shell that happend to lay on a spot for a moment

cresting the next canyon our protaganis surveys the purgatory of fetid complexity in the valley below feeling the pain of endless effort in her arms and legs, knowing the collosal effort requied from meger progress.



#### Uniform Motto
To have it all, all at once, to greatest degree allowed by the nature of computation and the limits of human cognition
## === MANIFESTO ===
### IDEAS
#### Ideas driving Conceptual Simplification
- the name of the game is maximizing the knowlege and ability gain per minute the learning time for those that follow
##### Recursive TL;DR -- SPIRALING
##### Reformulative Reificating
- Naming and framing have a profound effect on subsequent thinking, learning and acting.
- Naming and framing ultimately forms a complex deeply interacting recurively defined web of concepts.
- Best recusrive web is far from obvious -- homing in upon it is one of the greatest quests humanity has undertaken.

==> this suggests an iterative and fluid approach to reification itself.  For chomprehensibilty and velocity one hopes to express each potentially valuable reification as succinctly as possible and as generally as possible.

then the interacting web of these ideas should evolve in a chaotic and interacting way, but one that strives over time for convergence on utile reification gems among the infinite space of possible namings and framings.

Proposed Properties:
- easy to manage varent reifications
- pressure to converge
- pressure to decompose
- meta-refication when possible -- 
	e.g. finding patterns across reifications when possible

#### "live" constrain dont set
#### Cant is a four letter word

Shouldn't is not four letters.  indeed can be valuable to have constraint, but not forever for all times for all cases constraints.   convention not coercion (not constraint)
#### Fork is a four letter word
- A single fork result in collosal loss of effectiveness
- **FALSE FORK** -- didn't need to be
- **TRUE FORK** -- ties directly to some intrinsic aspect of computation itself
- **MULTIPLICATIVE CLUSTER FORK** -- a false fork that is cross cutting of many other forks
- **META FORK** -- a meta fork is a language construct indirectly causes forks by limiting downstream language writers
#### An Author's Apology
- I was dramatically opposed to creating a new language
    XKCD comic on language creation:   https://xkcd.com/927/
- But one cannot achieve simplicity by extending from complexity
    Ya gotta start from the very bottom up.
#### HAIR
structure or function beyond what is essential for some purpose
## === APP: Info Manager ===
### --- PRINCIPLES - INFO MGR ---
#### ALL KNOWLEGDE IN ALL THE RIGHT WAYS
#### DENOTATIONAL TREE
Map all info back to conceptual tree
### --- Value Props ---
- Task-driven -- value derived from KM is indirect, based on value is provides in executing a given task
	- task specific actions
		- traversal
		- divid and conquer
		- xform/view -- create systematically transformed version-slice of info
- Costs/values -- total value derived is much greater than totals costs of usage
	- Post-hoc -- best if costs can be delayed to point of usage, shapes work done best, avoids unused work
	- Multi-stage -- best if costs can be paid incrementally as value accrued
	- Min-delta -- best if costs are min-delta costs for next output leveraging all costs paid up to present
	- View-costs -- we think of costs of knowlege entry, but also count and reuse costs of task-specific processing.
		- e.g. reuse traversal
Things an info manager provides:
- **Organization** -- 
	- Placement/Structure -- a place to put info which maintains some cohesiveness to the whole
	- Retrieval -- ability to find info later
	- View -- ability to create and view a task-specific 
- Fused multi-views with independently mutable conditionally propagated updates
## === EXAMPLES ===
### --- 
# ### STUFF ###
## --TO PLACE--
### BOILER PLATE 
existence of is canary in coal mine -- the sure sign of language mismatch
what about rails boiler plate ???
## IDEAS

### TRUE FORK -- DSL
     -- When done correctly, specialized lanaguages represent a true fork, that is,
    a difference that makes enough difference in practice that the costs of forking a new language

    Because TREE-TOP-(supremcy) the primary tradeoff is one of user-comprehension-loss/gains & performance
    very secondarily one considers complexity of implementation etc.
## Problems

Bi-Level-i-tis – a simple but often damagingly restrive way to share is bi-level sharing solution.  K-work is split into exactly two parts  

