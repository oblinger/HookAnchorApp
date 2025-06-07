# === UNIFORM ELEMENTS OF COMPUTATION ===
(C)opyright Dan Oblinger.  Do not distribute.
## Introduction

**TL;DR #1 --  The Uniform Agenda is to refactor the range of modern programming paradigms into a single DAG of paradigms.  Each elemental paradigm in the DAG is as (1) independent as possible, (2) simple/general as possible, and (3) as interoperable as possible.  By iterating the entire collection we approach a possible 'best basis' ladder of paradigms that can be mixed and matched to yield the same utilty (simplicity, performance, etc.) found across the space of today's domain specific langauges (DSLs).  Unlike today's DSLs interoperation across these are more akin to the much smoother integrations that occur across the library packages within single programming language today.  The powerset combination of importable packages within a single language system like Python tends to interoperate well since these packages inherit from a common set of underlying paradigms.  Uniform aspires to extend that level of interoperability to a much wider scale.**


Different programming langauges/paradigms/frameworks are better for different computational tasks.  They make incompatible tradeoffs, and thus cannot be combined into some kind of single best "super" language ecosystem.  Every second year student in Computer Science knows this very basic fact about tradeoffs.

But what if this "fact" is not true?  What if it were possible to combine all software languages into a single 'super' ecosystem that could realize advantages of all of them, all at the same time?  What if the efforts of all programmers could be harnessed as part of an ever growing totality, rather than squandered, spread thinly into ever more specialized and incompatible cul-de-sacs?

Well that sounds totally impossible, but if it _were_ possible ...
it would be, to put it mildly, transformative....


Here we argue it is both possible and practical:
- we argue that incompatabilities are _NOT_ a fact of computation, but rather are a henious curse which we have placed upon ourselves.
- A singular ecosystem _IS_ possible
- It _WILL_ allow knowledge work to be exponentially amplified by multiplying the ways that they combine with other human efforts.
- Allowing the continuous folding of effort back into a singular ecosystem
- And yes doing this _WILL_ be transformative.

_
## The nature of splits

_**TL;DR.  There are good splits and bad splits.  Good splits are ones where one want BOTH sides of this split within a complete computational ecosystem.  Bad historic split by contrast should be expunged from the ecosystem as they exponentialy dilute efficacy w/o providing any compensating value.**_


To understand how such an "impossible" ecosystem is in fact both possible and practical it is helpful to begin to by distinguishing two kinds of taxonomic splits that are often confounded:  
-- an _**intrinsic constraint**_, ("a definition created by God"), and
-- a _**forking constraint**_, ("a definition created by human folly").

An _**intrinsic**_ (good) taxonomic constraint is one that is tied up with some important property that:
1. is either inherent in the _mathematics_ underlying computation, 
2. is derived from a properties of the _physical world_, 
3. is derived from a property inherent to all _von Neumann machines_, or
4. is a _vacuous_ constraint, one that is satisfied by all computing tasks of human interest.

By contrast a _**forking**_ (bad) taxonomic constraint is one that is NOT tied to such intrinsic attributes of mathematics, the physical world, the von Neuman machine, or common to all tasks of human interest.

One can sometimes identify a forking constraint by the parallels one can draw across the split implied by that constraint.  A forking constraint will create many parallels between the operators and algorithms found on either side of the split, while the intrinsic split will generally result in alternatives that do not map well or at all between the different sides of the split.

Based on our experience it seems there are two primary ways one can introduce a constraint into an ecosystem, while not forking the ecosystem:
- **ADDING SHARP CONSTRAINTS / INTRINSIC SPLITS** -- The notion of a DAG (directed asyclic graph) is a constraint on graph structure.  We argue this graph constraint (as opposed to many other possible graph restrictions) is an intrinsic split.  Why?  Because it serves as a mathematically sharp boundary enclosing exactly the set of graphs over which algorithms like the classic depth first tranveral (DFT) algorithms work properly.  (See the example below for a contrived alternative decimation that should be rejects as it is not a forking split.)
- **ADDING COVERING CONSTRAINTS / VACUOUS SPLITS** -- The other kind of non-forking constraint is one that adds a useful concept to build upon that happens to apply to the full range of comptational tasks relevant for humanity.  In this case the addition of the constraint is non-forking since there are no tasks / algorithms required for cases NOT satisfied by the constraint -- of the outside set is empty it is not a split!  For example, we argue the concept of RDF-expressible data, or JSON-expressible data, are both non-forking, since all data processable by von Neumann machines will satisfy both such constraints, thus adding one (or both) to the ecosystem is non-forking.



OPTIONAL: AN EXAMPLE OF AN INTRISIC VS FORKING SPILT

To make this idea of intrinsic verses forking constraint clear, we provide  DAG (directed ascylic graph) as an example of an intrinsic split, and "decimation" and "diamation" as examples of forking splits.  We claim the notion of a directed asyclic graph (DAG) is a constraint intrinsic to the nature of computation.  It splits off a new sub-category of graphs -- the set of all graphs without cycles.  We argue DAG is _intrinsic_ to the mathematics of computation since many algorithms specifically require their input graphs to be DAGs to operate properly.  For example, the simple depth first graph traversal algorithms will NOT operate properly on non-DAG graphs.  Thus the DAG boundary is intrinsically tied to the nature of that algorithm.  Consider, however, another hypothetical constraint definition:  we might define a "decimation" to be a graph that can be partitioned into ten "levels" where each level _only_ has directed links to the verticies assigned to the levels below it.  This is a restricted kind of DAG and is useful in describing a class of graphs over which certain algorithms can operate.  But we argue decimation concept itself is not instrinsic, it is a forking split, since many algorithms that operate on decimations have close parallels that ALSO operate on certain kinds of non-decimations graphs too (e.g. specifically those that are non-decimating DAGS).  Thus this definitional constraint is not an intrinsic constraint, but rather it is a forking constraint.  To see this is forking, consider a second graph constraint:  we define a "diamation" as a graph whose nodes can be mapped onto the squares of a chess board where links in the graph only go from node in one cell onto nodes in a cell to its immediate left or immediately below it.  Graphs that fit this pattern are not decimations, but yet another kind of DAG.  Now imagine an alternate world where we never considered the notion of a DAG, but instead created data structures and algorithms that are optomized to operate on decimations and diamation graphs.  In that world, computer scientists will have to choose whether to base their code on decimations or diamonds structures (or yet a third structure) to support their processing.  This will lead to enormous duplication of similar but incompatible algorithms.  For example some languages might use decimating type hierarchies, while others would opt for diomating type hierarchies.

This is a contrived case designed to make the needless and disasterous forking very aparant.  Still we argue that more subtle version of this forking are extremely common within computer science, and even for these subtle forks the consequenses are just as disasterous.  This kind of forking happens EVERYTIME one adopts some constraint, but then does not build APIs and algorithms that fully leverage the _FULL_ generality implied by the contraints that one has taken on.  Everytime this happens (and it happens all the time) someone else will be forced, later add new algorithms LEVERAGING THE SAME UNDERLYING INSTRINSIC PROPERTY to cover other parts of the space missed by your lack of generality in the first case.

_
## Ok, but why should we care so much about this forky stuff?

_**TL;DR.  Forking is really really bad; exponentially bad.  Don't do it!**_


Yeah forking seems messy, but why is it such a big deal?  Here are the collossal destructive consequences laid out in a few pithy bullets:

- **DUPLICATION** -- Each fork engenders the need for two or more incompatible solution algorithms to cover the space of relevant tasks.

- **DUPLICATION CONTAGEON** -- Worse, any algorithm builts upon any such a forked algorithm, will themselves be forked.  That is, a parallel but incompatible solution likely will be required on top of the other forks to serve similar but incompatible use cases in parallel but incompatible ways.  (For example, an OO-language-type system built upon decimations would be incompatible with a type system build upon diamonds.  Both intrinsically still depende upon non-circularity, but they are incompatible.  Notice the introduced fork of decimation is contageous, once introduced it will spread and fork everything it touches.)

- **EXPONENTIAL INTERACTIONS** -- Most destructive of all are "cluster forks".  Anytime an algorithic solutions depend upon a two or more forked subcomponents, the result is _MULTIPLICATIVELY_ forked.  So as computer science builds more complex constructs based upon multiple simpler ones the results of forking is exponentially accelerated -- an exponential number of parallel but incompatible solutions are required to cover the space of possible problems.  Catastrophy-upon-catastropy this multiplicative forking is TRANSITIVE.  That is the number of forking alteranatives required to cover a space of algorithms is exponential in the total number of forks occuring ANYWHERE within the dependency tree of sub algorithms require.  

- **MERCIFULLY HUMANITY IS LIMITED BY IT OWN RESOURCE** -- The cluster-forking that results from an even a modest number of forks generates a space of alternatives so vast that humanity would have required untold melinnia to fully develop the exponentially sprawling space of variations that we ourselves have introduced within computation in only a few short decades of forking.  Mercifully, humanity has not yet expended the time require for completion of this, so we are instead we are left with several very purpose-built "tendrils" that we call programming languages or frameworks.  These are each ever growing heaps of incompatible forks that are built towards a somewhat overlapping, somewhat distinct subrange of target tasks.  The result is that humanity currently has a set of radically incompatible programming languages and programming constructs.  This leaves humanity with the sense that the tradeoffs that do exist today between these very sparse alternatives somehow reflect something inherent about how tradeoffs operate in computation itself.  (I conjecture this is an incorrect conclusion, and the tradeoffs a computer scientist faces today is aggregation of injuries inflicted upon them, many dating back even before their birth.)

_
## The Uniform Agenda 

_**TL;DR.  The uniform agenda is to iteratively edit a DAG of computing paradigms inching ever closer to a single "uniform" non-forked computing ecosystem that covers all computing tasks of human interest using a DAG of pure elements that are combined to form the molecular combinations that are present day programming languages and frameworks.**_


Ok, so forking is really-really, really-really, REALLY-REALLY bad.  Got it.  Well then, lets just build a non-forking basis for computation instead.  That, in a nutshell, is the Uniform Agenda -- to build a collection of paradigms (constraints and algorithms) that are non-forking and that cover all human-relevant computational tasks.  

**PARADIGM** -- A _**paradigm**_ is any functionally described approach for doing a thing.  A generalized approach for organizing and addressing some collection of related problems or sub-tasks to be performed.  Each paradigm includes an algorithmic and represenational implementation, as well as informal / formal descriptions of how and when the paradigm should be used.

**ECOSYSTEM** -- An _**ecosystem**_ then is a directed ascylic graph (DAG) of paradigms which are each described and implemented using only those paradigms transitively linked within the DAG.


This single software ecosystem could, in some sense, be viewed the last "programming language" humanity would build.  In principle, it could be used to build algorithms for any software task with results that were at least as good as any existing or future programming language's results would be.  And this ecosystem acheives this aim while avoiding the exponential redundancy within the ecosystem itself.

Great, so lets get started.  I guess we just start building from clean APIs, and simple concepts?  Not surprisingly it turns out to not be this simple:

- **Problem #1: SLACK KILLS** -- Anytime a paradigmatic constraint is introduced into the ecosystem where there exists an alternate paradigm which is strictly more general and similarily performant, then we have inadvertently introduced a fork into the ecosystem.  The only way to avoid this is to fully understand the constraints underlying each paradigm and only add ones where we can provably ensure that one is fully exercising the generality supported by the defining constraint of the paradigm.  Sadly, we often don't have this kind of clarity for the paradigms we are introducing.  Instead we see them as "nice, elegant, and general."  History shows us this is likely not going to be good enough.

- **Problem #2: COMBINATORIAL COVERAGE** -- Even if one were to collect constraint-algorithm paradigms that were mathematically sharp, there would still be no guarantee that one could combine these isolated paradigms to solve problems that lie within the intersection of their constraints.  The algorithmic building blocks must be able to be _combined_ into an aggregate algorithms.  But this is not guaranteed to be the case, even if each elemental paradigm is individually non-forking.  It is possible that somehow the algorithmic approaches cannot be combined, or more likely, it is possible that while the generalized ideas underlying these paradigms could in principle be combined, the specific realizations of those paradigms we have chosen are structured in a way that requires some rewriting in order to get them into a compatible form.  Again, just as with "slack kills", assessing this requirement, requires the consideration of the powerset-grammar of all ways these paradigms might be realized and combined, in order to verify that the ecosystem is non-forking.

Neither of these problems are remotely plausible for humanity to actually solve.  Perhaps their does exist some non-forking basis for computation, but even if were to write down a part of it: we couldn't even know if we had done so!  The Greek philospher Xenophanes provides a relevant quote here:

As for certain truth no [wo-]man knows it, ...  even if by chance s/he were to utter The Final Truth, they would themselves not know it: for all is but a woven web of guesses.  
   -- Xenophanes 570-478BC

Yup, that pretty much sums up our situation nicely.


Ok, well this is not very encouraging for our Uniform Agenda.  Fortunately there is a bit of empirical evidence that one can make interesting progress towards this ambious goal through a series of glancing blows, rather than taking the challenge head on.  It seems we can make progress if we attack the problem as Engineers rather than as Mathematicians.  Here is an outline of our approach:
1. We define "paradigm" as the basic building block.  Each paradigm has an implementation, as well as both an informal and sometimes formal documentation of how to use the paradigm to solve programming tasks.
2. We define several observable signs of likely forking within a collection of such paradigms along with a strategies for editing those paradigms in order to remove these observed instances of forking.  
3. We use these observable signs and strategies to iteratively edit a collection of paradigms in order make them "less forking" DAG of paradigms over time.  

Surprisingly, we have found that as long as one is willing to edit multiple paradigms in tandem one is able to remove a large categories of forks -- and to do so in a way that progressively results in a "less forky" system overall.  Its not clear just how far this glancing blow approach for the uniform agenda might be taken.  Still after years of messing with this it seems it might be possible to take this approach quite far.


Instead of diving deep into the imprecise and incomplete heuristics for guiding this agenda, lets just give the jist of three of these heuristics to give a general sense of the actual agenda in action:

- **PARADIGM DE-DUPING** -- Two paradigms Px and Py give indication of a fork in the case that an engineer describing Px to another engineer who knows Py might say something like:  ``The XXX of Px is kind of like YYY of Py except blah, blah, blah ...''
The corrective action is to assume there is a paradigm Pz with ZZZ such that XXX and YYY become special cases of ZZZ.  (Note one may need to edit multiple other paradigms in order to make this work out)

- **DECOMPLECTING** -- A paradigm, P, is likely causing a fork if it is possible to coherently formulate any part of P without reference to the  rest of P.  Such a paradigm is forking since it force the injection of spurrious content into solutions that do not require them.
The corrective action here it to refomulate P with a sub paradigm p' that is expressed in a standalone fashion w/o reference to the rest of P.
	
- **PARADIGM THESES** -- A third heuristic is the identification of "paradigm theses" similar to the Church-Turing Thesis.  Recall the Church-Turing thesis claims that any computable function can be expressed as a Turing machine.  These paradigm theses are similarly informal, since they make a universality claim without formally defining the space covered, rather it is a claim of universality that is borne out via experience.  Any observed task which is not covered by such a thesis indicates a possible fork requiring a generalization (or other edit) of the paradigm in order to avoid the failure.



There are presently 15 dimensions of "quality" we have expressed for a collection of paradigms and implementations.  The idea is that the dimesions themselves are incomparable, but within a single dimension one can conclude all-things-being-equal improving this dimension is an improvment of the whole system.  Among these dimensions there is two umbrella groupings that are important:
- **MINIMALITY** -- The idea here is that paradigms should be so simple that they cannot be semantically meaningful and yet simpler.
- **COMPONENTIALITY** -- The idea here is the the paradigms should be designed to they parsimonously combine in all the right ways to do all expecting things those paradigms "should" do given all tasks of human interest.


There is much more that one could say about our early understanding of the Uniform Agenda how one might execute it...  But screw all that formality for now!  What the heck is comming OUT of this agenda?!?





_
## The Uniform Ecosystem

Just to bring this idea down to earth a bit, here is a very sloppy rendition of the first rungs of this Uniform ladder, just to give the jist of where all of this might be going.  More precise treatments are given later.  
Our proposal for the lowest two rungs of the ladder employ well established building blocks from Mathematics and Computer science.  The third and forth rungs employ very well established ideas in computer science, still they have not been isolated and crystalized into a independent paradigms capable of covering all existing languages.  Arranging the details of the eight paradigms in rungs 3 and 4 so they fit nicely on rungs one and two and "combine in all the right ways" to encode uniform versions of cooresponding constructs from existing programming langaguges is where most of the energy has been spent thus far on the uniform agenda.  Many data paradigms from existing programming langauges are directly expressible from as special cases of rung three.  Most functional constructs from existing programming languages would exist as a rung 5 immediately above heavily utilizing both rungs 3 and 4.  In this way we sometimes refer to the unicore of rungs 3 and 4 as a "semantic assembly language" since it defines a grammar of language constructs out of which most (all?) constructs for mainstream programming languages are naturally fashioned (e.g. Python, Java, Ruby, Lisp, C/C++.  Note: Haskell just hurts my head, I can't tell all the things you can do with that language are expressible as unicore).

Here are the uniform paradigms DAG as presently understood in its first three ladder rungs:

**ELEMENTS OF UNIFORM COMPUTATION**

**MATH** 		-- Rung 1 -- Mathematical Uniform -- Encodes pre-requisite concepts
  **GRAPH**		-- Directed graphs whose verticies may have edges that are:
			   (1) ordered, (2) labelled, (3) and/or functional
  **NUMBER**	-- Natural or Decimal numbers with expected associated operators
  **REWRITE**	-- Term Rewriting Systems defined using graph structures

**STRUCTURE**	-- Rung 2 -- Structural Uniform -- Encodes data
  **ACCESS**	-- Vertex-centric graph operations
  **SPACE**		-- Graph-based notions of space, location, path
  **TIME**		-- Model of state, change, persistence and assignment
  **FLAT**		-- Flattening of graph data into linear forms (including 'printing' as markdown)

**FUNCTION**	-- Rung 3 -- Functional Uniform -- Encodes computation
  **GROUNDING** -- Specification embodiment
  **EVAL**		-- Uniform interpretation
  **FLOW**		-- Control flow (block,branch,repeat,chain,exit)
  **PKG**		-- Source code grouping and combining
  **UNIT**		-- Unifies structural and functional paradigms into a mix and match object model


THESIS STATEMENT -- In the the paradigms listed below we sometime identify a 'thesis' statement.  Each thesis captures a claim of generality (in the spirit of the Church-Turing thesis).  Some theses theses claim that the constraint defined by the paradigm is a "covering" constraint.  This is a claim that one can adopt this paradigm and still 'cover' all use cases of interest to humanity w/o serious loss in utilty in the resulting solution space.  (See XXX for deeper discussion of this idea.)

_