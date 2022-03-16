

# ### AGENDA ###
## === PLANNING ===
_
# ### TOPICS ###
## FRAMING
### MATH COMPARISON INTRO

#### Multi-Set example

This trivial example demonstrates the difference between mathematical composition and computational composition:

Consider this definition of setsum and this partition lemma:

setsum(S) = 
	0 if S = {}
	x + setsum(S2) if S = {x} union S2


Sum splitting lemma:

setsum(S) = setsum(S1) + setsum(S2) if
	S = S1 union S2 and S1 disjoint from S2


This splitting lemma is overly specific, it could be generalized to all associative binary operators, and arbitrary partitions, but lets keep it simple.  If proven by a mathematician 


setsum:
  fn((s, split),
	branch(
	  if(len(s)==0,	0),
	  if(len(s)==1, s.contents)
	  if(true,
		 (s1, s2) = split(s)
		 setsum(s1) + setsum(s2))	
		)





_
### Slide Intro



-- Needless incompatibility is introduced by the software elements we use as well as how they are combined

Uniformity Claim -- Such needless incompatibilties are avoided if one builds from software elements that adhere to the three principles of uniformity.

The result is a transformative reduction in the complexity of software and software engineering as the majority of software engineering is devoted to the management of these avoidable complexities.

BUT 
-1- the principles required for 'appropriate' elements requires they are recursively constructed from appropriate sub elements.  It cannot be "bolted on" it must be built ground up.
-2- simultaenously satisfying all three principles is exceedingly difficult, and there is no guarantee it is even possible in all cases.  Still early progress is encouraging, and humanity only need succeed at this task once for all time.


~~~~~
Software systems inherit the needless complexity in each of their components as well as any introduced in their combination.  As systems grow this compounding becomes overwhelming.

~~~~~


_
### (w. tiny intro) _

The Uniform Agenda aims at a particular way to break-down and build-up software.  We argue systematic use of this approach would yield transformative improvements in software development efficacy.  We start describing a small slice of this agenda, and use that that slice to explain why we believe JSON gained so much ground against XML even at a time when the XML tool chain was already well established, the standard and technology were very mature, and nearly universally supported across most languages and frameworks.  Even today, when people compare these alternatives they often focus on their surface form, parsing efficiency, etc. as the driving difference.  Here we describe what we believe is the difference that drove adoption, and it is also one of the key ideas underlying the Uniform Agenda.  Here we define, language, generality, embedability, essence of, and use these to understand JSON vs XML in a new way:

_
### LANGUAGE FRAMING

Here we define a few familiar terms that underlie our framework:

**BEHAVIOR** -- A _**behavior**_ specifies the input-output pattern obtained from a computational system.  Such behavior might represent:
is specification of the input/output behavior of an interpretation system.  Such a behavior might represent:
**BEHAVIOR** -- A _**behavior**_ is specification of the input/output behavior of an interpretation system.  Such a behavior might represent:
- the computation of specific function
- a constant, structured data value that is persisted and is quieryable
- a class or module with multiple entry points and state that implements a desired functionality


EXECUTABLE -- An executable is a data value that, when interpreted yields a behavior.
ENV -- An environment (ENV) is a mutable data value that provides the context for interpretation.
EXE -- The execute (EXE) function accepts an executable and an env and returns the interpretation result

**EXECUTABLE** -- An _**executable**_ is data value intended for excution on the interpreter, along with a specification of the correct interpeter to use.

Such a executable might represent:

**CAPABILITY** -- A _**capability**_ is a (typically infinite) set of executables that collectively represent a nameable, describable grouping of related behaviors.

For example, "arithmetic expression evaluation" or "reactive GUI scripting" might be two such capabilities.  Perhaps the first would be a set of all possible arithematic expressions, along with a natural language description of the variety in that set (e.g. all integer expressions recursively built from binary '+', '-', '*', and '/' operations).



**LANG** -- A _**software language (LANG)**_ is a function that maps _**source code (CODE)**_ into executables for a given interpreter, along with a natural language description of the performance that results from the code of this langauge.

**LANG DOMAIN** -- The lang _**domain**_ is the set of all source code expressions that are valid inputs for the lang function.

**LANG RANGE** -- The lang _**range**_ is the set of behaviors it are output for each valid code inputs.  Formally:

LANG INTERPRETER --
	
	_executable_ = _lang_(_code_)   **given**   _code_ in DOMAIN(_lang_)
	
	_executable_ in RANGE(_lang_) **iff** 
		**there exists** _code_ in DOMAIN(lang) **where**
			_executable_ = _lang_(_code_)



TEMPLATE -- A template is a lang whose output (range) is the input (domain) for another language--that is the interpreter for the first lang is the second lang



	
_
### FRAMING

In software there are problem that must be address in some way.  In computation these might be functions that need to be computed.  In representation they might be data structures that need to be encoded.  We can even talk about white space indention as addressing the task of unambiguously specifying the tree structure that relates a set of textual entities.  In each case we call the thing that addresses these tasks a micro language or a lang.  Just to provide a few examples to make this idea clear:

LANG(s)								PROBLEM TO BE ADDRESSED		
regex								regular expression string matching	
Python, C++, Java					general purpose computing	
indention-based-structure			specifying the tree structuring of a set of textual elements
brace-based-structure


In the specific case of computing functions we can say a problem is a function to be computed, and some "program" instance of a computing language "addresses" this problem if the language's execute function yields the correct functional output for all inputs given the programs (the language instance) which "addresses" this function.  But we can generalize this notion, and say that:

PROBLEM -- Some aspect of computing solutions which repeatedly recur across a range of computations.

LANG -- A language (lang) specifies a set of "program" instances that it "addresses", along with the mechanisms that "executes" the program in order to address the problem.

ADDRESS -- a relationship between an instance of a LANG, and an instance of PROBLEM.  The relationship indicates that the specified instance of LANG addresses the indicated problem.


So this formalism applies in an expected formal way to computable functions being addressed by programs that will compute those functions.  But the formalism also covers the case where brace-based-indention "addresses" the derivation of an appropriate tree structuring for a set of textual elements.  In a completely analogous way brace-based-indention addresses the problem of specifying an appropriate tree structuring of a set of textual elements.  The case also matches our problem/lang/address paradigm as well.

_
## COMPLEXITY

One aspect of the uniformity of a semantic element is its "complexity".  Here we discuss how that is defined.

**COMPLEXITY AS A KIND OF "DELTA" MEASURE**
The Uniform notions of simplicity and complexity are reminicinet of information theory's definition of conditional information:  An element's simplicity is expressed as a kind of 'delta' relative to some assumed background information.  The idea is that the semantic complexity of explaining the arthematic division operator depend upon whether or not we can assume the recipient already understands multiplication.  If they do, then division is semantically simpler for them.  But all specifications of semantics assumes some background concepts upon which it is built.  We make this explicit, by defining our notion of complexity as a function of both the thing to be explained, as well as the background material one may assume with providing those explanations.

**ASPECTS OF COMPLEXITY**

.  Since any semantic description s on is always building from some assumed base level understanding

all of the paradigms it depends upon.  One measure of paradigm complexity is number of natural language words required to explain the paradigm well enough that it may be correctly implemented by a competent software engineer who is assumed to already understand all of the dependant paradigms.  It is the additional information required for this paradigm relative to all others.
A related measure of complexity is the number of seconds the understanding require.  Two other measures deal with the complexity of USING the paradigm:  the expected number of seconds (or lines of code) required to apply the paradigm to some appropriate target task.

As an example consider our description of the place paradigm:
``A PLACE is a graph edge, with an ACCESS function returning that edge's destination vertex, and an ASSIGN function accepted a vertex used to update the vertex's destination.''

A programmer that is already familiar with the graph paradigm should be able to use these 28 words to produce a correct implementation of the place paradigm.  This is an exceedingly simple paradigm!  It is simple of course because it pushes any complexity into the particular structure of the graph below, or into specialized paradigm semantics above it.  This is exactly what all good simple paradigms should do, push most complexity outside of themselves and only capture a single thought.  This ensures any usage of the paradigm will never be saddled with any extraneous fluff that is not required for the task at hand.  Notice:  either you don't need place semantics (persistance) at all, or this place semantics will fit perfectly for you and you will need all of it, it is hard to even think of cases of persistance that somehow dont fit!

**SIMPLICITY**
Simplicity is the opposite of complexity.  In particular it is a relative measure comparing one or more aspects of complexity between two possible elements.  
## ESSENTIAL CAPTURE
### GENERALITY

**GENERALITY** -- We say that one language L1 is _**more general**_ than L2 iff RANGE(L1) is a superset of RANGE(L2)

All other goodness measures being equal one language being more general than another makes it "better" than the other language.  

_
### NATURAL

At one level we can evaluate code purely based upon functionally what behavior results from that code.  That is an important consideration, but it is not the only consideration when choosing between different languages that might be used to produce some common behaviors.  After all both python and Turing machines are are Turing complete, yet when we look on GitHub we see few solutions developed on top of Turing machines.  Part of that is a performance issue, but another huge consideration is how "natural" those solutions feel for the software engineers who must deal with them.  Here we define this notion of naturalness:

One approach is to use the same one the supreme court used in defining pornography:  We dont need to define it, one just knows it when one sees it.  Yes, at least is the extreme cases engineers generally can agree on what is an is not a natural solution for generating some particular behavior.  Still we offer the following thought experiment as a slightly more concrete specification of this notion of naturalness:  In a nutshell some code, c, that is part of some larger software system is "natural" if software engineers are not inclined to refactor it when given the opportunity to do so.  We frame it this way, in order to avoid looking for 'perfect' solutions to problems, and rather we call a thing "natural" in any case that it is not so painful to deal with that engineers would take the step of refactoring it.  This defaulting towards calling things natural unless painful to deal with is the bias that we are aiming for in our definition.


**NATURAL** -- We say some code, c, is natural if software engineers are NOT inclined to refactor c:
- given ownership of the software system, S, containing c
- given the time and money resources to perform the refactoring, and 
- given a mandate to maintain and extend, S, over a variety of future behaviors.


Notice in this framing, c, is explicitly part of some larger system. Thus refactoring might include throwing out the language that c is written in and choosing a different language for that part of S.

All other things being equal one (obviously) prefers "natural" code over code that is not natural.

_
### EMBEDABILITY 

Lang A embeds into Lang B iff
	There exists a deterministic mapping function, M, such that:
		Forall a in VALID(A), There exists b in VALID(B) where b = M(a) and b "corresponds" to a

Different interpretations of this notion of "corresponds" results in different levels of embedability:
- **FUNCTIONAL EMBEDDING** -- the weakest form of embedding merely considers if there is any element b that provides the same functionality as the element a.  I.e.  BEHAVIOR(a, A) = BEHAVIOR(b, B).  This weakest form of embedding indicates that functionally anything one can do with the language A one can in principle do with language B.
- **NATURAL EMBEDDING** -- the next level strengthens correspondence to not only require functional equivalency, but to further requires that NATURAL(a) implies NATURAL(b).  That is, problem solutions which are natural in the first language map to natural solutions when mapped to the second language.
- **DETERMINISTIC EMBEDDING** -- the highest level of embedding is one where not only is the result functional and natural, but it is also _deterministic_ in the sense that multiple engineers given the spec for A and B, along with an element a in A, will themselves agree without coordination on a single specific element of b that corresponds to a.  Formally:  a "corresponds" to b iff b=M(a)


These three levels of embedding indicate, in progressively more restrictive ways, that one language can "fit into" another language.



_---PERHAPS THIS TEXT CAN BE REMOVED NOW THAT WE HAVE A DEFINITION OF NATURAL ABOVE???---_
In some ways, we are going to treat this notion of "natural encoding" a bit like the US supreme court decided to treat the concept of pornography.  We are not going to try to precisely define the term, but instead argue we don't need to, since we all know it when we see it.  In the same way, we are not defining the notion of "natural encoding" as we are arguing that software developers all "know it when the see it."

But as a rough way to think about the idea, we can use the following thought experiment as a rough definition for this notion of natural encoding.  Consider a team of software engineers that are given (1) ownership of a particular software system, (2) the requirement to then considerable future work extending/using the system, and (3) unlimited resources to refactor the system.  We say an aspect of this software system is a "natural encoding" of the problem it solves for that software system, if those engineers are NOT inclined to refactor that part of the system.  (perhaps by inventing a new language to better represent that part, or such.)  

Notice "natural encoding" is not trying to say the particular choice made was in some sense the absolute best in the universe choice or is in some way perfect.  Only that it fits its task so well, there is no pragmatic reason to fix it FOR ANY REASON.  So a system that used XML to encode some configuration parameters would be a natural encoding of that information, and so would a JSON encoding.  You know, tomAAAto / tomAHto, one engineer might have their favorite encoding, but at a practical level both are getting the job done, and neither is so much better than the other that it one would justify a refactor (unless some other reason was driving the refactoring).

But encoding configuration parameters as a vector of bytes, really might get refactored.  Each time you changed the parameters you would need to mess around with the opaque block, debugging errors when the wrong version of the data was being read in, would be a mess, etc.  So all of these functionally encode the problem at hand, but only the first two would be resistant to refactoring, so they are the only ones that are natural encodings.


_
### ESSENCE

In a way generality is about how "big" a language is, while, and embedability is about how "small" it is.  Using these together we can define a kind of "goldilocks" idea of being big enough in all the right ways, but not too big in order to exactly capture "the essence" a class of problems.

**ESSENCE** -- a lang, L, captures the _**essence**_ of a problem class, PC, iff 
	RANGE(L) contains PC,
	there does not exist L2 where 
		RANGE(L2) contains PC,
		L naturally embeds L2, and
		L2 does not naturally embed L

The idea is that a language captures the "essence" of a problem class of if it is _a_ smallest (most embeddable) language that is big enough to fully cover that class of behaviors.  Notice there could be several languages that each naturally embeds the others, and in this case all of those languages are said to capture the essence of the problem class.  This definition gets at this idea of being big enough and small enough in all the "right" ways, but it is very hard to reason with this definition since it is quantifying over all possible languages, and it might include "weird" languages whose range covers the problem class, but that are not at all useful for software engineers to use.

Therefore, for this reason, we extend this notion of essence to ALSO require natural embedability into those languages the software engineers already use for this class of behaviors:

**ESSENCE-FOR** -- a lang, _L_, _**captures the essence**_ of a problem class, _PC_, _**for**_ targeted languages, _T1, ..., Tn_, iff 	
	_L_ captures the essence of _PC_, and 
	For all  _Ti_ in {_T1, ..., Tn_}  _Ti_ naturally embeds _L_


The idea here is that not only is _L_ the goldilocks right size and shape for this problem class, but solutions encoded using _L_ can systematically be translated into ALL these other existing 'targeted' languages _Ti_ in a natural way.  Thus a programmer who understands both _L_ and some _Ti_ can easily utilize _L_ as an alternate form, such that solutions in _L_ will always naturally map back into their favored language.


_
### THE EXAMPLE OF XML AND JSON

With all of this machinery in place we now turn back to the XML-JSON example, both to make these abstract ideas more concrete and to use this machinery to provide one explanation for JSONs strong adoption over time.  We start by first noticing a number of reasons why one might have expected JSON to NOT have had large adoption when it was introduced:
1. WELL ESTABLISHED.  At that time, XML was already extremely well established.  
2. MORE EFFICIENT.  XML paring was specifically designed to be very efficient to parse, so XML parsers can be faster than JSON parsers.
3. MORE POWERFUL.  JSON is not more powerful in some way.  There is nothing that JSON does not do anything that XML cannot not also do.  Additionally, XML handles many pragmatically important cases like binary data, typed data objects, schema validation, and it can handle textual markup in ways that JSON cannot touch.
4. HIGH APP DOMINANCE.  XML was already used across a huge number of applications, and served as the basis for many interface standards that are themselves recursively defined in term of the XML standard.  
5. HIGH NETWORK EFFECTS.  In the world of data interchange languages, network effects are _**king**_.  A later _better_ standard often cannot penetrate into use cases where infrastructure is already built on an inferior earlier standard.  From that perspective, XML was a fortress!

Established XML toolchains, interfaces/languages built on it, massive adoption, and JSON is less powerful, and less efficient to parse.  _This is a slam dunk!_  No way for JSON to make real headway.  So why did it?!?


To understand what happed here we introduce a well understood but not generally named class of behaviors:  communication of tree-structured-object-data-values.  Many modern software systems are built on some kind of OO-style objects.  The data in these objects are typically referenced to other objects, or are simple numeric or string values.  Getting that information from one memory space (say from server-to-client in web programming) is an extremely common software 'chore' to be done.  There are fancy object serialization packages one can use, but often web programmers resist the complexity of these as they introduce tight linkages and versioning issues between different client-server software stacks which in many cases are not even using the same base languages.  Instead they opt to transmit (and sometime maintain) data that will be shared as recursive data containers that terminate in numbers and strings.  This data form is independent of language, and is often more independent of specific versioning of the endpoint software as well.  In simple cases, programmers often opt to just keep certain transmission-oriented data, in this data-value form (as lists and maps of strings and numbers) and avoid the use of the OO paradigm for that kind of data.  Thus communication of tree-structured-data-values-transmission is the problem of encoding such data to it can be transmitted: 

For precision lets define this tree-structure-data-values problem class:
TREE STRUCTURED DATA VALUES PROBLEM CLASS -- The tree structure data values (TSDV) problem class is the class of all losslessly encoded finite, recursive structures of strings and numbers where each sub container is indexed from its parent container "by-position" (using a natural number, like a list index), or "by-name" (using a string value, like an OO-field identifier).  Notice this definition is very close to what JSON can express, but of course XML can express such data quite nicely as well.  

For our analysis of XML and JSON here, we want to pick a specific list of languages to "target" as well.  Here we define "modern dynamic langs" as the ten dynamic languages having the largest number of active repositories currently listed on GitHub:
  MODERN_DYNAMIC_LANGS = {JavaScript, Python, Ruby, TypeScript, C#, Objective , R, Swift, Clojure, Lua}

Now we can assert our key claim:
	JSON captures the essence of the TSDV problem class for MODERN_DYNAMIC_LANGS, while
	XML does not capture the essence of the TSDV problem class for MODERN_DYNAMIC_LANGS

First let just convince ourselves these assertions are correct given our definitions above.  The TSDV problem class (encoding strings and numbers into containers index by position or name) is easily handled by both JSON and XML.  It is quite straight forward to encode such data in both formats.  Second, we see that JSON embeds naturally into XML.  We can define a simple mapping, M(), from all JSON expressions onto XML expressions such that for all target problems, p, if we had a natural encoding json_p for that problem, it would yield an XML instance xml_p = M(json_p) that would also be a natural solution for the original p problem.  (The typical mapping is to use XML ordering to encode lists, and use XML subtags for by-name sub fields in JSON that may contain recursive containers, and to reserve the XML key/value notation for by-name sub-fields that only contain simple string-like data values).  Anything JSON can do, XML can do too, and do it in a very parallel way.

But the reverse is not true.  XML has specific features to encode textual data, for example, which JSON does not share.  Consider encoding the following rich text using XML markup:
	This _simple_ example is a 3^rd level title right here.

In XML this is:  <h3>This <i>simple</i> 3<sup>rd</sup> level title right here.</h3>

What might this look like in JSON?  Perhaps:
	{type='header', level=3, contents= [
		"this", {type='italic', contents=[
			"simple"]},
		"3", {type='superscript', contents=["rd"]} 
		"level", "title", "right", "here."]}

JSON does _functionally_ encode this rich text information, but it is no surprise that it does not _naturally_ encode this data.  An engineer that was required to maintain and update 100K of markup text expressed using such JSON would refactor that garbage immediately!  It is just not a natural representation for such markup.  This is no surprise, JSON was not designed to handle such markup, while, XML on the other hand, is as a simplification of SGML which centrally _was_ focused on such text-related problem types.  Thus we see that XML does not embed into JSON since natural solutions in XML may not be natural solutions in JSON.  According to our definitions above JSON might capture the essence of the TSDV problem while XML could not capture its essence, since JSON is closer to a perfect match.

The second part of the assertion above is a claim that if an encoding of some data in natural in JSON, then it will continue to be natural when systematically translated into each of the MODERN_DYNAMIC_LANGS listed above.  We see this is true since each of these languages have some kind of "container" datatype that provides natural ways to encode recursive by-position and by-name containers of strings and numbers (as lists and maps).  The mapping of JSON onto these container types provides almost a direct one-for-one mapping.  (There is some complexity since these languages may provide overlapping numeric form, thus some flag must be set, but for each problem subclass there generally are natural choices to be made for this decision.)  Thus together we see that JSON seems to capture the essence of the TSDV problem for modern languages while XML does not.  (We say "seems to" since there is always a possibility that some even more embeddable language could exist, but this seems unlikely, as we seem to be fully utilizing all aspects of JSON in order to encode these data values.  Still it is usually difficult to prove a negative, so assertions of essential capture tend to be tentative assertions.)


Ok, so XML is good for rich text data and JSON is not, thus it is does not capture the essence according to our definitions above.  Fine.  Who cares?  Why does this matter?  To answer, consider the following two Python code snipits:
	
	native_collections_tree = json.loads( json_encoded_string )
	json_encoded_string = json.dumps( native_collections_tree )

Similar API call pairs exist for all of these modern dynamic languages.  These two calls ALONE are often all one needs when dealing with JSON data!  This API is beautiful, elegant, and covers all functionality that is often needed.  This API is so simple there is generally little need to even read documentation!  Its usage is intuitive even when arbitrarily complex recursive structures of lists and maps are involved.

Surely programmers would want this same simplicity for XML-parsing into these dynamic languages too.  All XML parsing API must also give such simple access as well right?  WRONG!  Generally there are DOM or SAX style parsers that parse XML into multiple speciality XML class objects with hundreds of specialized XML access methods.  These parsing libraries dozens or hundreds of pages of documentation.  And one cannot learn to uses these APIs without reference to these docs.  Wow!  Quite a difference.  Now of course those XML libraries are intended to do more than JSON libraries are intended to do.  So those APIs must be larger.  They can often perform stream processing, data validation, automatic object marshaling, etc.  So it's not a fair comparison.  

Still for the programmer that just wants to map simple recursive data-values structure from one place to another why not _also_ provide the single-call-simplified-API for XML as well?  Surely the programmers sometimes want that too.  Yes they DO want it!  Indeed we argue that is what has driven the adoption of JSON even as XML was already well supported.  So why not just add those two more call to the XML APIs, XML would have it all!

The problem is, while XML _functionally_ embeds into these container types in these languages, it does not _naturally_ embed into them.  Consider how this should be mapped:  '<list> one two three </list>' Maybe as a list?  That seems right, but then what about: '<h1 id="702"> one two three </h1>'  That also seems like a list too, but then where does the "h1" type info go?  and what about the "702" id?  Maybe one can argue that:
	{type: 'h1', id: '702', contents=["one", "two", "three"]}
is a "natural" encoding, given the complexity of the data that is being encoded.  Ok, but then our original example becomes:
	{type: 'list', contents=["one", "two", "three"]}
No Python programmer would want to operate over recursive versions of a list of list of these things when they could just operate on a lists of lists instead!  Construction, navigation and everything will just be messy.  The problem here is we are pushing the bubble under the carpet, but we cannot get rid of it.  There is **NO** natural XML to Python mapping that is definable since XML does not "fit" into Python lists and maps.  Thus the Python-XML libraries cannot provide those to beautiful load/dump methods... they are just not well defined.

JSON, on the other hand, is a goldilocks representation if one wishes to serialize TSDV between client and server, just as is very often needed in web programming.  And notice that goldilocks status is not a small difference, it is the difference between dozens or hundreds of pages of documentation with many specialized classes and hundreds of accessor methods on one hand, and TWO METHOD CALLS API on the other!  _**This**_ is why JSON has achieved such large penetration, even as the incumbent had so many other advantages.

Essential capture can really matter!


QED


_
## ORTHOGONALITY
### _

**Uniform -- the "_ultra dry_" software ecosystem**

The "Dont Repeat Yourself" (DRY) principle is accepted software engineering wisdom.  Code that repeats itself can introduce bugs when those repetitions are slightly different, code may be harder to refactor if the repeated part needs to be changes/extended, and generality is lost if capability is added to one but not the other, among other ills.  In our Uniform Agenda we take this mantra to a surprising extreme:

We express software langauges, libraries, and frameworks as a large collection of cohesively described pieces that are placed into a graph structure which indicates that the semantics (and often implementation) of one piece is described in terms of specific other pieces.

We interpret the DRY principle as a constraint that each "software notion" must be in the semantics of at most one of these pieces, while all other references to this notion must directly or indirectly link to this definition in its defining pieces.  Said in this way, it sounds like the pretty standard expectation of parsimony between compoents.  But we mean this in a very strong sense:  Anytime it is possible to somehow completely rewrite a set of software pieces, in order to refactor out a redundant sub-part that they all can later derive from, it is the uniform agenda to consider this refactoring.

We have not found a precise definition for this idea of "redundancy".  Instead we adopt the same stance the supreme court of the United States took with pornograph, when they argued they did not need to define it since people generally knew it when they saw it.  Still, we do offer the following thought experiment as an effective means to home in on parts that have redundancy between them:

Imagine Alice, an engineer, has full knowledge of sofware piece X and piece Y, while Bob, a second engineer, only has knowlege of piece X.  Alice wishes to explain piece Y to Bob, knowing that Bob is already familiar with piece X.  If at any point in her explanation it would be reasonable for Alice to utilize Bob's knowledge of piece X in order to clarify some aspect of piece Y, then there is likely a redundancy between X and Y.  This is likely an opportunity to reformulate those pieces in order to remove a redundancy between them.  The tell-tail phrase Alice might say to Bob tends to be something like this:
	
	``Bob, xxx of piece X is kind of like yyy of piece Y except ....''

We have observed most _anytime_ it is remotely possible to frame some aspect of one piece as "kind of like" some aspect of another piece, that it did not explictly depend upon, it has almost always turned out that there was a redundancy could be productively be removed in service of the uniform agenda.  Interestingly this sign of redundacy often preceeds any ideas about _how_ one might perform refactoring to remove it.  Initially, identifying that such a phrase exists between different pieces is a kind of "bad smell" indicator that  some orthogonality refactoring is needed.



Of course not all redundancy is bad.  In the case that two pieces are dependent on a common ancestor, then they may be redundant on any aspects that are derived from that common ancestor.  We are only interested in precluding a commonality that does not stem from a common ancestor.  We use the following terms to formalize this ortogonality requirement:

**PIECE** -- A "_**piece**_" of a software system is one, cohesively described, element of a partitioning of the semantic specification of that software system.  Each piece must have a natural language specification of its semantics, and may have one or more functional implementations, or formal specification as well.

One can think of a 'piece' as either a sub-routine that is called by other pieces, or as a chapter or paragraph of natural text that can be referenced within other natural text specification of semantics.  In uniform we are mostly focused on redundancies within micro languages, constructs, representations, formats, libraries, etc.  Still this notion of redundancy extends naturally to end user applications as well.

**REDUNDANT WITH** -- We use the notation "P1 _redundant with_ P2" to indicate indicate this observed "bad smell" of redundancy.

**DEFINED IN TERMS OF** -- We say one piece, P1, is _define in terms of_ another piece, P2, in the case that the definition of P1 makes explicit usage of piece P2 in it definition.

And 'depends upon' relation is the transitive closure of the defined-in-terms-of relation.  Formally:

**DEPENDS UPON** -- We say one piece P1 is _**depends upon**_ itself, and it _**depends upon**_ another piece P2 if P2 is defined in terms of P3 and piece P1 is depends upon on P3.

Now we can formalize the idea of a collection of pieces being orthogonal.  The idea here is that a set of pieces are said to be orthogonal if the only redundancies are the result of explicit dependencies between the pieces, never because of some "yet-to-be-refactored" redundancy.  Formally:

**ORTHOGONAL** -- A set of semantics pieces, **P** = {P1, ..., Pn} are said to be _orthogonal_ iff
	For all Pi, Pj in **P**  
		Pi is not redundant with Pj	 or		# Either they are NOT redundant or
		Pi depends upon Pj           or     # the first explictly depends upon the second, or 
		Pj depends upon Pi           or		# the second depends upon the first, or 
		((Pi depend upon Pj) and 			# they are circular

The idea here is that redundancies are only allowed via explicit import from a dependency chain.  A correlary to this is that each concept (anything that might be "kind of like" another thing) must occur at most once within the entire hiearchy of specifications langauges, libraries, and frameworks.  Then all other occurances of this "kind of thing" must be 'imported and reused' in all other places where it might be relevant.



_
### ORTHOGONALITY EXAMPLE

Here we present an example of refactoring a redundancy in order to clarify this idea of orthogonality, and also show the power of improving orthogonality.

For several years, our lowest level API for graph structure had _get(vertex, key)_ / _set(vertex, key, value)_ operators in it.  This seemed quite natural since it is intended to closely parallel the API for a map and object data structure that typically also has getter/setter operations.  At a higher level of abstraction we have the cohesive notion of persistence, with the typical 'access()' and 'assign(value)' operations.  


This definitely did have a "bad smell" since "access/assign are kind of like get/set except there is no key required."  But early attempts to align these ideas just didn't seem to work right---the method signatures dont fully align, and the idea access/assign persistence seems to exist seperate from.  Worse still if we consider stream objects (like read/write file handles, or iterators that allow modification we have yet another kind of persistence that does not seem naturally be defined in terms of a single cohesive basis upon which all the other pieces depend.  YUCK!!

For several years we had this bad smell of an aparent redundancy, yet without really being able to articulate what precisely was redundant in a way that we could refactor to make everything orthogonal.  Early on we had the idea that a place was kind of like an edge in a graph so maybe that was the solution, but the APIs were not cooperating!  It took several simultanous "mind flips" to square this circle.  Here they are:

1. The map API is not the 'bottom' rather it is the composite "molecule" that is constucted from two orthogonal elements, "access" and "navigation"
2. Access and navigation are coordinated by their explicit dependency on a third element "graph"
3. In defining the semantics of a molecule one may refer to graph structures that are "made up" at the semantic level, and dont even exist at the implementation level.  (The implementation of an element may systematically depend upon a different graph structure than its semantics refer to.)
4. A place ALWAYS is an edge within a graph -- they are the same concept.

Ok, so how does all of this play out?  We already had the idea of paths and navigation over graph structure.
- But in our thinking this was derived from the more basic notion of graph which in our minds was the graph access api.  
	==> Instead we now think of navigation as the more basic capability but it does not include any notions of 	mutation or persistence.
- The persistence API remains as it was, but now a place is defined to be an edge within a graph---it's possible, however, that this is a semantically defined graph, but not one that actually exists in memory anywhere.
	==> In particular anytime you have a set of places in memory, and are tempted to think about those places as the vertices in a graph, remember that always defines a duel graph where those places are the edges of the dual graph, and the vertices are phantoms that don't actaully exist as a referenceable thing by any API.
- Navigation is now define in terms of graph (which may or may not be referenceable by API or exist in memory) and the same is true for persistence, but they now have a well defined interaction since both are defined in terms of a common graph notion.
- Streams and structure iterators can both now be thought of edge-place iterators with well defined mutation semantics.  This is true even in cases where the graph they are operating on is a phantom graph that is not expressed in memory nor even by API, it is simply a sematnically well defined way of operating over the data.

In concrete terms we end up with a Vertex API with an 'index' and 'iterator' operators which returns places associated with a vertex, and the Place API has the traditional 'access' and 'assign' operations as before.  The 'get' and 'set' operators for maps still exist, but they are now macro operations that are semantically defined as part navigation and part place access.

_ 
### SO WHAT?

With significant effort we managed to replace the get/set operator operator pair with a single index operator, and updated the iterator.  This is not that much of a simplification, so why much such a fuss?  With this change we were able to get 100% of all data access/mutation in the entire Uniform Ecosystem to be semantically define directly or indirectly in terms of the access/assign interface methods.  Likewise 100% of all navigation operations now stem directly or indirectly from the index and iterate operators--there is no other way to traverse information other than deriving from this interface.  And these two APIs are guaranteed to be compatible since both are precisely define in terms of a single common underlying graph formulation.

Thus, if for example, we were to provide support for say web DOM operations with a jQuery-like indexing language, or a data selection SQL-like dataset selection language, both would be expressed in terms of this base level index/iterate formulation of navigation.  Then if we were to provide file system interfaces, or excel file parsing interfaces, both would be naturally be formulated using our access/assign model of persistance.  Doing this would naturally ensure that all varient naviagation-like languages would seemlessly combine with all other navigation langauges, and also with all persistence-like interfaces.  The resulting system would yield an exponentially expanded number of mix-and-match macro language capabilities that would all be interoperable since all derive from a commong graph ontology.  Languages contructed by combining these would be guaranteed to be semantically well defined, as each has been tied back to these two core notions of navigation and persistence.  Further, with clever code generation (which is quite possible with homoiconic language expansion that also propagates strong typing information) one can often automatically create combinations that are nearly as performant as natively constructed language combinations.

Even in cases where one must build a special case implementation of one important combination or another because semantically defined combinations do not automatically yield any implementation or any performant implementation, one is still greatly aided by the fact that these formulations were specifically constructed in ways that naturally just fit together.  No gluing or adaptation needed.

This is very different than language ecosystems that exist today.  Suppose one wrote a pre-order traversal algorithms in Java, C++, Python, or Clojure for example.  Would that same code work for a list of list, a map of maps, a file tree, for fields within objects on the heap, for in frames on the stack.  No, no, no, and no.  Not only do we have non-ortogonality across these different languages, but we have non-orthogonality WITHIN these languages too.  It is a mess, and a mess of our own creation, it didn't need to be this way, we were just too lazy to do it right!  Of course many languge writer will claim, but that is just silly.  Beyond pre-order traversal, what meaningful operation can one even define that has meaning and value over stack frame, maps, and heap objects?  Place reference.  Only c++ allows one to have a reference to an integer value without telling you if that value is on the heap or a local value on the stack, and it can only do that for integer values that are realized in memory, not an integer within a constructed view (as you could do in Uniform).  Or history, many languages provide advanced libraries to managing versioning of mutating data, but none of them provide generalized libraries that afford this support over anything beyond a single kind of data.  e.g. object fields, OR versioned data structures, OR versioned files, etc.  Provenance.  few languages even have libraries to track the provenance of how values are computed.  Or 'listeners'.  Many languages provide libraries that give 'react-like' triggering on data updates, but they either provide this only over their own specialized structures, or over a single kind of structure like object-fields or map structures, but never over all of them, not becuse it would not be useful, but just because those languages simply don't unify these things.  When we place ontop of this the burdens of handling the non-orthogonality pains which occur when considering parallel formulations that must be bridged when combinging different language ecosystems the complexity and code bloat just mushrooms.

The reader may respond, ``ok, perhaps theorectically one could go back in time, and re-invent all languages from this mythical common core, but in the real world, this is irrelevant, as we cannot go back in time, and these languages are here to stay.''  It is true, to get all possible benefits from the Uniform Agenda one really would just wipe away all existing languages and replace them with a single uniform reformulation.  Fortunately, one can gain significant benefit by embedding the uniform ecosystem into existing languages and only using it part of the time... specifically those parts that cause the most interoperabilty troubles when trying to operate inside traditional languages.  This next section will describe a notion of embedability that supports this partial use case.


{[Note:  I couldn't resist saying "one uniform ecosystem" in the last paragraph.  It is what I beleive.  Still we are taught so strongly that one ecosystem is impossible.  perhaps I reduce my credibility by slipping it in there w/o really have a very length defense of the idea.  (and I don't think the reader will have patience for a lengthy defense either here, or probably anywhere is the exposition of the idea.)  Thus maybe it should just be removed?]}



 which occur 




and the variables   The Python object model for example is quite elegant and quite general


After years of looking 

 in valuable

, but uninten, and all would be guaranteed to be sematnically well defined 

interoperable capabilities 

navigation with 

any and all navigation micro languges that are built indirectly upon 

- as verion of an iterator, and both of those things are simply a temporal method of performing navigation over graph-edge-places.  



-  which themselves depend upon a third more basic idea.

There were some cases where it was convenient to treat a 'place' object as an edge within a graph and then both access and assign end up cooresponding to gets and sets with the appropriate keys.  This worked well for indexable verticies, but it all fell apart when a vertex was a multi-set or relation and there was no uniquely indexing key.  So this didn't seem like a viable refactoring.

Then somehow the idea of


This piece did not seem to align directly with the graph API since one does not have a key but the other does.

be directly derived from graph API access since it seemed persisence in the abstract didn't really



 non-orthogonality 
This notion of orthogonality is 





~~~~
The notion of essential capture from the previous section does seem to tell us something interesting about the problem class being addressed, but it does not seem to constrain the pieces one might want to use, since there can be as many "essential captures" as there are problem classes.  But it seem there can be an infinite number of yet more and more problem classes, so this, in the end, provides no real constraint.

Enter the orthogonality constraint.

_
##### ...

...

REDUNDANCY -- An aspect A1 of piece P1 is said to be redundant with aspect A2 of language L2 if it is possible to reformulate this common subpiece into its own sub language, L3 such that L1 = L1' + L3,  L2 = L2' + L3, and L1' and L2' are no longer redundant.

...
_
### Inductive atomicity

define uniform_agenda:
	theory = set()
repeat
	choose t1 and t2 where t1, t2 in theory and t1!=t2 and t1 is not orgogonal to t2:
		theory = refactor(t1, t2, theory)
	choose problem, or problem class, P where
		there does not exist 
	

Suppose we begin with a language C which can be expressed
### ORTHOGONALITY

It is fashionable to talk about drying your code out (DRY=dont repeat yourself).  Uniform takes this notion to a whole new level.  In Uniform we strive to have each idea express in exactly one paradigm within the whole ecosystem.  The idea is to refactor and simplify each paradigm until they are "pure" and do not contain aspects of another paradigm, unless of course they explicitly depend upon and use those aspects explicitly.  The easiest way to identify this kind of non orthogonality, is to look for ANY cases where a programmer might be tempted to explain one aspect of one paradigm by comparing it to another aspect of another paradimg.  So the sentence:  ``A in P1 is kind of like B in P2" is a dead give away that your paradigms are not orthogonal.

Here is an example.  For several years the 'simplest' set of uniform operators for operating on graph data included as 'set' and 'get' operator.  This was very natural since we often think of a graph vertex as a map associating label to destination vertex, and the canonical map API has a get and set operator.  But we also had a 'place' paradigm with 'access' and 'assign'.  Over time it became apparent some refactoring needed to happen to make these orthogonal, but the problem was, neither one naturally was expressed in terms of the ohter one in all cases.  (what if your place is not part of a map?  maybe it is a local variable)

The answer was to explictly think of all places as an edge in a graph (even if it is a virtual graph that never exists explicitly as a graph in memory.)  When we think of a place as a graph edge, then it maps very beautifully onto the graph paradigm, but it also unifies with the get/set semantics of the graph API.  Now SET(x, k, v) == ASSIGN(IDX(x, k), v), and GET is similar.  Moreover the ITR operator can also return a sequence of edge places so it allows iterators to be modifiable, which causes them to be unified with the STREAM paradim in a way that fits with writable streams like a file stream.

This simple refactor now causing ****ALL**** data access and data update to flow thru this one place API.  this is extremely powerful since now we can add provenance mechanisms, history mechanisms, live data triggers, as well as backing any kind of upper paradigm on any kind of backing store.  Each time uniform achieve its perfect orthogonality aim, it creates yet one more "hook" upon which its entire universe can be pivoted.

Simplicity and orthogonality taken together is why we refer to uniform paradigms as the "elements of computation" each one cointains only one thought, and each thought is contained in only one paradigm.  The features of today's languages are then macro molecules composed from these atomic elements.

_

DEPENDS -- We say one language depends (DEP) on another language if 

_
## SPLICING CLAIM

DEF: A uniform element is any extension of the uniform hierarchy that remains granular, orthogonal, and covering

Claim: Any granular, orthogonal extension of the uniform hierarchy is _universally reusable_.

Universally reusable code can be spliced into 

~~~~

CLAIM:  A code written using nothing more than some given set of uniform elements can be natively reused in any language context that is orthogonal to those elements.

DEF: a language context is orthogonal to as set of elements if there is no orthogonality conflict between an element and some aspect of the langauge context.  This means it is possible to define the language constructs using the elements (or they are unrelated).  This does not require the language constructs themselves to be orthogonal to themselves, often they are not.

DEF: Saying the some code is natively reusable means that one can extend the langauge context without introducing any non-orthogonality into the lanaguage constructs in order to allow the provided code to run directly on the composite (spliced) collection of constructs.





_
### --- PRINCIPLE OF UNIFORMITY - orthogonality ---

In a nutshell the _principle of uniformity_ states that the more systems share common dependencies, the more interoperable they become, and the less effort is required when operating on them.  This fairly accepted idea is the basis of for the uniform ageda described here.

We introduce a bit of formalism to be able to express this idea a bit more specifically and then use the formalization to describe the agenda.  First what are these bits of commonality to be shared?  We want to encompass sharing in a very general sense and these does not seem to be a good existing word for this, so rather than mis-use an existing words we are making up two new words:

WODAT -- Way of doing a thing -- A wodat is any approach to any aspect of computing which one person might describe to another.

RATS -- Repeated Aspects of a Task -- A rats is any namable, repeatedly occuring aspect of a computing solutions for which humanity has developed approaches (wodats) to address them. 

Since both ideas are new and pretty abstract it is helpful to just enumerate a number of examples that indicate the intended range of usage for thsese terms:

WODAT(s)					RAT
Python; Java; C++
if-elif-else  case			1-of-n selection in a structured control flow graph
whitespace-based-nesting	containment specification within textually presented info
brace-based-nesting


So rats are things that programmers explictly or implictly need to deal with when building solutions, and wodats are things that programmers can use to deal with these rats.  In some cases a wodat comes with a data schema and or software implementation that is part of the wodat.  So in some but not all cases one can imagine having libraries containing implementations for various wodats.

This brings us to our third invented word:

R
SUN -- Specified by Using as Needed -- a sun is a directed link between wodats it indicates that the specification (and potentially implementation) of one wodat utilized another wodat in all places that it naturally made sense to use that wodat.

So the python wodat SU-links to the Whitespace-based-nesting wodat, while the java wodat SU-links the brace-based-nesting.  In this fashion we can express all software, langauges, etc. a one large graph of SU-links of WODAT entities.




a rough sense one can imagine having libraries contain
_
### --- INTRODUCTION ---

**TL;DR #1 --  The Uniform Agenda is to refactor the range of modern programming language and frameworks into a single DAG of paradigms.  Each 'element' of this paradigm DAG is to be as (1) simple, (2) general, (3) independent, and (4) interoperable as possible.  Using a multi-dimensional definition of 'best' we iteratively edit the entire collection to approach a single 'best' ladder of paradigms which can be mixed and matched to yield the same utilty (simplicity, performance, etc.) found across the space of today's domain specific langauges (DSLs).  Unlike today's DSLs, however, these languages where developed from common components at ALL levels of their construction.  The result is a reduction in complexity that is exponential in the number of interacting components required for the average solutions.**

(C)opyright Dan Oblinger.  Do not distribute.



Different programming langauges/paradigms/frameworks are better for different computational tasks.  They make incompatible tradeoffs, and thus cannot be combined into some kind of single best "super" language ecosystem.  Every second year student in Computer Science knows this very basic fact about tradeoffs.

But what if this "fact" is not true?  What if it were possible to combine all software languages into a single 'super' ecosystem that could realize advantages of all of them, all at the same time?  What if the efforts of all programmers could be harnessed as part of an ever growing totality, rather than squandered, spread thinly into ever more specialized and incompatible cul-de-sacs?

Well that sounds totally impossible.  But if it _were_ possible it would be... transformative.



The key insight idea driving the Uniform Agenda is the idea of decomposing software "paradigms" down to their semantically most elemental form.  This results in a recursive structure of dependencies that terminates at "elemental paradigms" where one cannot find anything valuable to assert or implement in connection with smaller sub-parts.  We refer to these paradigms as "elements" of computation.

Of course there are many differnt versions of any given element that one might consider.  Software engineers have several measures of "goodness" for a software component, so a univeral 'best' is not even well defined---things like performance, simplicity, and generality among others desired attributes.  

**BEST/DOMINATING** -- We say a paradigm, P, is "_**a best**_" or "_**dominating**_" paradigm given a set of such goodness measures, M, iff there does not exist a "better" paradigm Q, where:
- Q applies to the tasks that P applies to,
- Q is better than P on some goodness measure M1 in M, and
- There is no goodness measure M2 in M where Q is significantly worse than P.

These "best" paradigms represent a kind of frontier in the space of possible paradigms and they "dominate" all paradigms in the sense that all non-best paradigms have at least one better paradigm that is at least as good in all ways.  Of course the space of all possible paradigms is not enumerable in any practical way, thus we generally will not know that a paradigm is a best one, we will only know that it was NOT best one when we have identified a new better paradigm that dominates it.  This observation is central to our Uniform Agenda: rather than trying to prove any paradigm paradigm is best, we instead just continuously replace paradigms as better ones are identified.

To make these ideas more concrete, lets consider several paradigms:  The first paradigm is organized around the definition of a _**directed graph**_ as a collection of verticies and collection of edges, where each edge indicate a "source" vertex, and a "destination" vertex.  The second paradigm is organized around _**Directed Asyclic Graph (DAG)**_, defined as a graph without any cycles in the graph.  Notice several things about these paradigms:
1. _DEPENDENCY_ -- One cannot even express the definition for a DAG except though the use of the graph paradigm as its basis.  Thus the collection of paradigms naturally forms a dependency graph based on their requirements.
2. _BASIS_ -- The depth-first traversal (DFT) algorithm is naturally organized under the DAG definition since this term represents is the exact boundary indicating those graphs where the algorithm is guaranteed to terminate.  (Further, the relationship from DAG to DFT is one way this particular definition of DAG is better than other possible definitions) in characterizing the domain of applicability of the DFT algorithm.
3. _VARIATIONS_ -- There are always alternate organizing defintions one might select.  For example, One could choose: undirected graphs, directed graphs, hyper-graphs, ordered graphs, labelled graphs, etc.  To avoid an explosing of such paradigms one must carefully choose paradigms that can easily implement other paradigms, and one must adopt a sufficiently coarse measure of "better" in order to allow this one central paradigm to be "at least as good as others."  By taking great care in this regard it seem possible to avoid an explosion of incomparable dominating paradigms.
4. _CONTAGEON_ -- Generally when a paradigm is not a best (dominating) paradigm, then any dependent paradigm built from it will "inherit" this flaw, and also not be a dominating paradigm.  Thus it is critical that these "best paradigms are constructed bottom up.  Where one strives for each paradigm to be a best paradigm from bottom to top.


**FRAMING THE UNIFORM AGENDA**

With this framing we can define Uniform Paradigm, Uniform Ecosystem, and Uniform Agenda:


**PARADIGM** -- A _**paradigm**_ is any functionally described approach for "doing a thing".  It is a generalized approach for organizing and addressing some aspect common to a collection of related software tasks or problems to be solved.  

One can think of a paradigm as a module, but the notion of a paradigm is more general than this.  For example, Python, C++, and Prolog are very complex paradigms (all for general purpose software developement).  Below these are medium level paradigms like Regex (for matching strings), printf (for string template expansion), Jquery (for selecting tree items), SQL-ops (for tabular data manipulation), along with language paradigms like throw-catch, try-except-raise (for non-local exit), extends/import (for controlling method scope), etc.  Below these paradigms are "programmer visible" paradigms for data and compute, for example String, List, Map, Function, Method, Class, etc.  Below this are paradigms the unify the semantics of data and compute, for example, Directed Graph (for representing data), Term Rewrite System / Turing Machine / Von Neumann Machine (for representing compute).

So what is it that is in common across all of these "paradigms"?  All of them have a purpose, an approach and commitments (expressed in natural language).  And at least one implementation.  This notion of paradigm is more general than the notion of a module, since modules assume there is one common 'loader' that understand and operate it.  Here we have different lower level paradigms creating the spaces that higher level paradigms can fit it.

When we view the full paradigm stack that humanity has thus far created, there is a colossal amount of redundancy.  Hundreds of slightly incompatible versions of OO-inheritance, of regex, of tree select, of data persistence, etc.  Some alternatives like the determinism of Python, the laziness of Haskell, or the backtracking of Prolog should forever remain as distinct paradigms.  But the VAST majority of paradigm incompatibilities provide little benefit, but do great cost in bridging between variant paradigms, and exponential redundancy as we build the same stuff over and over in slightly distinct (and incompatible contexts)

The alternative is to identify the 'essence' of J-query and use that one paradigm as part of any language the requires “an expression grammar for performing selection from within tree structures”.  Or to identify the 'essence' of function call anytime a language requires one of those.

My two big idea is that when you drive toward the essense of a thing naturally gain compatibility.  JSON is a winner because it is a best capturing of the essense of "string and numeric data structured by postion and by name".  And that JSON paradigm is complete enough to capture interesting data structures across most all languages, and the same time it is striped to it essense in it avoids making any commitments which are at odds with most any langauge.

JSON (or something similar) belongs in my web of paradigms while XML does not.  Why not?  Because XML makes a host of assumptions that are not compatible with many languages.

Here is a good thought experiment for testing is a paradigms is good.  Pick two programmers each familiar with two very different programming environments.  Then have one write a program to map some real-world data or capability implemented in the first one, into the target paradigm.  Then independently have the second programmer write the code to extract the same well understood real world data or capability out of that paradigm... but don't let them talk to each other.  How did it go?  If that mapping mostly worked out well then you have a good paradigm on your hands.  If each one used the underlying paradigm in radically incompatible ways, then your paradigm did not capture the essence of the data or capability being considered.

I think one can build a web of paradigms that approximately succeed in the thought experiement above.  And you can see, if one CAN succeed in this, then one has dramtically reduced the complexity of gluing components.













a particular approach for providing 'data persistence' could be paradigm, a Turing machine, Regex matching, Jquery.  Indeed both 'data' and 'compute' are themselves examples of paradigms.  Because of their generality, each paradigm must not only describe its own functioning, but must also specify how it is to be combined with other paradigms.

A second important aspect of a paradigm is our attempt to make them "so simple they could not be simpler".  In a practical sense we have found this often means building each elemental paradigm around a single core defined term... the rest of the paradigm then centrally depends upon this single definition. E.g. tying a depth-first traversal algorithm to a paradigm that ONLY upon the definition of DAG maximizes its generality, since there are no non-DAGs where such an algorithms is appropriate.  (And of course one must also have maximally general notions of 'compute' and 'graph' among others to actually specify the DFT algorithm in its maximally general form.

Thus each paradigm is built around:
- **AN ORGANIZING DEFINITION** -- A single defined term around which the paradigm is organized
- **ESSENCE** -- A single sentence describing the capability afforded by the paradigm
- **DESCRIPTION** -- Natural language desription of the semantics and usage of the paradigm
- **INTERFACE** -- A formal model for relevant datastructures and interfaces for the paradigm
- **DEPENDENCIES** -- The list of paradigms this paradigm depends upon
- **IMPLEMENTATION** -- One or more implementations of the paradigm
- **THESIS** -- A claim of generality that we tentatively believe this paradigm achieves
- **SEMANTICS** -- An optional formalization of the semantics for a paradigm can be provided



**UNIFORM ECOSYSTEM** -- The _**uniform ecosystem**_ is a collections of "paradigms" that are organized into a directed ascylic graph (DAG) according to the dependencies between those paradigms.

For convenience we may create paradigms that are simply the union of several other paradigms.  Using this we can organize the uniform ecosystem DAG recursively into "ladders" that are each a linear sequences of "rungs" where each rung is a collection of paradigms (some of which may themselves be a smaller ladder).  

==> KEY IDEA:  One can obtain a maximially general paradigm if one can express it by 'importing' 
This structure allows each paradigm to be maximally general, as each is built from paradigms that are maximally general.  As we saw above, any limitation of the definition of graph will result in a similar limitation in the definition of DAG.  


**UNIFORM AGENDA** -- The _**Uniform Agenda**_ is to repeatedly edit the Uniform DAG of Paradigms in order to increase the range of computational tasks it covers and to progressively make the entire DAG "better" according to a multi-dimensional measure of goodness.

The dimensions of software goodness are incomparable so one cannot define a globally best DAG, still each dimension is comparable so it is possible to make an edit that improves one dimension without damage to any others, in this way one can iteratively seek 'dominating' solutions.


In other documentation we elaborate the meaning of each of these dimension of goodness, along with strategies for editing the Uniform paradigm DAG based on identified deficiencies according to each measure.  Here simply list the measures themselves along with an informal phrase describing each:

Measures of Software goodness:
- **COVERAGE** -- The fraction of all relevant software components that are naturally derived from this paradigm.
- **SIMPLICITY** -- The amount of text and interface code require to describe a paradigm
- **COHESION** -- The degree to which all aspects of a paradigm relate to a single idea
- **INDEPENDENCE** -- A paradigms is independent of another if each can be described and implemented without reference to the other.  (more is better)
- **INTEROPERABILITY** -- A set of paradigms are interoperable if they naturally combine to implement all expected SW constructs.  (e.g. their powerset does all the right things)
- **ORTHOGONALITY** -- Paradigms are orthogonal when no aspect of one paradigm can be described as being 'kind of like' as aspect of another paradigm except aspects that are explicitly inherited by DAG dependency.  (e.g. each idea has exactly one 'home' within the DAG)
- **PERFORMANCE** -- The execution speed and memory requirements of the best implementations
- **SUGARED** -- A formulation is "sugared" if its applications are syntactictically the most beautiful and cognitively-easiest forms for developers to operate with. 
- **NATURAL** -- A formulation is "natural" if is uses notation most expected by most devs.
- **DECLARISHNESS** -- The fraction of its semantics that are aparent thru visual inspection of its source forms.  (e.g. java package inheritance is more declarish than Lisp module loading)
- **SCRIPTABILITY** -- A ease with which paradigm is programmatically utilized in another.
- **HOMOICONICITY** -- An aspect of scriptability is homoiconicity -- the simplicity of the mapping from textual source form onto their parsed equivelants used when scripting


Optomization of ALL of these measures simultaneously for ALL paradigms within a DAG is, to put it mildly, wildly intractable.  Fortunately two key ideas come to our rescue:  We note that the number of ways to make a paradigm more complex is intractable large.  By contrast, the number of ways to make a simple paradigm even simpler dwindles until often there are only a few alternatives to be considered.  Second, each time we identify that our paradigm DAG underperforms according to some dimension realtive to some existing language as applied to a some specfic task of interest, it often suggests (after long consideration) an edit to the DAG that allows the DAG to be used to derive that language's approach without aparent damage to other derivations from the DAG.  It is thru a long sequence of these 'glancing blows' that we iteratively arrived at the first three rungs of the Uniform DAG as they are presently understood.  

NOTE: In the texts below we sometimes talk make claims about the "coverage" of a paradigm.  Roughly this translates to a claim that as a software engineer or language designer one can exclusively rely on the indicated paradigm's approach without significant loss across all tasks of interest -- it is akin to the Church-Turing claim of the universality for the Turing Machine.

_
_
# ### LOG ###
### --- 2021.08.04 - A Stay-at-home-dad plan

**UNIFORM PARTS** (timings @ 50 hr/wk)
- 2 weeks	-- Unicore Slides
- 2 months 	-- Unicore Paper
- 1-2 months	-- Unicore Reference Implementation
- 2 months	-- LAL python3 core
- 1 month	-- Uniform Markdown Parser


**CORP KM**
- Who is the customer?


SW MARKETPLACE
- Who is the customer?  
	Alex.  Wants to build B2B brand, but not own the code
- PARTS
	- Legal - An LLC that turns into a 503(c)
	- Chain - Python ledger chain that will migrate to a real chain
	- Tokenomics - Simple algorithms
	- Finances - Multi-sig wallet; Publish-wait-execute-if-clear

_
### --- 2021.07.27 - Marcel, getting my terms right

- REQUIREMENTS -- A list of assertions about some applicative context -- the context where a software artefact is employed -- that are require for the proper functioning of some element within a software system.

- COMPATIBLE WITH -- A set of requirements are compatible with some application of a software element if those requriements are met

- CAPABILITY -- A specification of the input/ouput behavior of an element within a software system that is desired.

- ESSENSE -- The essense of a capability is an implementation of that capability with minimal requirements


OUT TAKES


 of some capability is the full set of assertions about its interpretation environment that must hold true in order to guarantee this implementation will correctly produce the intended capability.

**COMPATIBILITY** -- An interpretation environment, E1, is _**compatible with**_ another environment E2 if and only if there is a functional (injective) mapping of elements of E1 onto E2 such that the set of requirements that hold for enviornment E1 are a subset of the mapped requirements that hold for E2.

~~~~

Just as a trivial example consider the max function:

def max(list):
	result = None
	for element in list:
		if result and element > result:
			result = element
	return result


~~~~~
say that essence is fragile in the sense that, it is easy to "break" an implementation by introducing a superfolous requirement, and once this is done, it often cannot be recovered.  Any implementations that built using this implementation will inherit the superflous requirement and thus also not be an essential capture of the capability that it implements.  We can see this effect at all levels in the software stack, consider an the "integer add" operation.  Most implemetations of this function will depend upon the byte-level representation of an integer (perhaps it is 'big-indian' or 'little-indian').  Whatever requirement is made by the add operator, that requirement will end up being inherited by any code that enbeds this add operation within some larger computation.  This inheritance of superfolous requreiments applies at all levels within the software stack, and its disasterous effects on code generality is CUMULATIVE.  The primary way to implement an essential capture of some capability is to construct it from some more primative essential capture implementation, and to do this recursively all the way down.

Such a function depends upon it invocation environment in dozens of ways that are not essential for the max computation.  It depends upon the way that functions are invoked, they details of the loop iterator, the short circuit nature of the 'and' function, etc.

and implementation that is broken once a superfolous requrirement is adopted by some implementation, it can n, all code built using that implementation will inherit this 'sticky' superfolous requirement.  For example, if one links against a particular math library, then that library becomes an assumption of that implementation---it is now "baked in" in the sense any larger software built using this library will inherit this requirement, even when many other math libraries might have been used it its stead.  Thus because of stickiness, one must build the essential capture of some capability only using subcomponents which are themselves essential captures of the capabilities we are gaining from them.  This is a harsh requirement, it means the essential capture implementation must be built exclusively from sub component implementations that are essential captures of the needed sub-capability.  This requirement applies recursively all the way down the component tree to its roots.

~~~~~~

directly build use the existing stack 



The modern sofware stack of libraries, frameworks, and languages is constructed from composites that each carry the baggage of dozens or thousands of requirements, that will be entirely unrelated to the functioning of any desired capability, but we will end up baking those unneeded requirements into any implementation we provide.  Thus the modern stack is quite ill-equipped to support essential capture implementations.



???
Of course her aim is to streamline her library so she can shrink the list of requirements as low as possible, but there is only so far that she can take this streamlining.  The build environment cannot build from fewer assertions than are required by the regression algorithm itself--at some point in the refactoring, she reaches a minimal set of requirements.  

~~~~

We say it is pragmatically unobtainable since the modern software stack is structured in a way that does not allow one to implement an essence of some capability.

express the essense of a capability.

in practice we cannot express the essence of most capabilities using the modern software stack.  The libraries, frameworks, and programming languages we might use to implement any algorithm we write down, ends up inheriting many thousands of assumptions made by those libraries and languages that have nothing to do with the algorithm we are writing.  The result is needlessly specialized software that requires special bridging code for each application in order to be used in contexts that ARE compible with the underlying capability, but are NOT compatible with the random assumptions that were forced upon its implementation.

~~~~~


 that assumption gets "baked into" our code in a way that 

Uniform takes a different tact on the problem, by providing building blocks that are themselves essential algorithms, thus taking on no assumptions beyond those required for their functionality.  Recursively building from these blocks allow us to define other blocks that are also essential.  Notice this approach must be taken starting at the bottom.  If one accepts any building block whose implementation is expressed in a non-essential way, then all blocks built from it will forever carry these needless requirements.


There are two imporant properties of 
Two important things to notice about essential capture.  


_
### --- 2021.07.21 - 

Marcin,

Sorry for the longer gap in my responding.  I did not want to just shoot back a rebuttal to your last comments.  The points you were making were quite accurate, yet also had nothing to do with this idea of embedding that I am striving to get to.  Thus I went back to the drawing board, to see if I could take a completely different tact on the problem.  One that just gets away from these irrelevant details.  Here way my ressult:


No question, plists and JSON are not self describing in ways that XML is.  And this property of XML is very important, very useful, and it makes XML superior to JSON as an interface definition format language---better at least in this one important regard.  But all of this is besides the point that my notion of embedding is trying to get to.

I think I need to take a very different tact here.  Somehow I have taken a relative simple idea and stired it into a mess of irrelevant details.  I also wonder if the intuition that comes with the phrase "embed into" is the wrong intution to build from....  Below is a completely different framing (of the same underlying idea); here I express the idea by the phrase "compatible with".  

Lets see if this is closer to the mark:
 

~~~~~~

**ESSENCE**

Imagine a linux library developer that has produced a "stats" library that calculates dataset average, standard deviation, etc.  She hopes to streamline it so that it builds cleanly onto nearly any linux environment.  To do this, she has compiled a list of requirements (assertions) that the library is making about its target environment. 

To test this she has constructed a representative build environment that supports exactly this minimal list of requirements and uses it to test her library.  Ensuring her regression library runs correctly on this represenative build is, by proxy, ensuring that it will also build on all environments that satify any superset of the requirements built into this representative environment.  Here we provide a formal definitions to support this "compatibility" notion:

**REQUIREMENTS** -- The _**requirements**_ for a valid application of a given implementation are a set of assertions that must hold about its applicative context---the interpretation environment where it is used---in order to guarantee intended behavior from that implementation.

**COMPATIBILITY** -- An implementation with requirements, R, is _**compatible with**_ an application enviornment, E, for that implementation if R is satisfied by E.

**ELEMENT** -- A software _**element**_ is any cohesive part of the software stack for which one can express capabilities, requirements, and provide an implementation.  (A software library is an element, but many deeper parts of the software stack can be elements too.)

Of course all things being equal, our library author prefers depending on a build environment having a "greatest compatibility"---that is, depending upon a smallest set of requirements as this will allow it to be applied into the greatest number of target enviornments.  Given some targeted "capability" (as defined by the input/output behavior of the library) she could refactor her libraries implementation over and over until arriving at some set of requirement assertions which cannot be further reduced given the nature of the capability being provided.  We refer to this final set of requirements and final implementation as an "essense" of the targeted capability.  We use the term essence since it is the nature of the capability itself that limits further shrinking of the requirements.  And we say _AN_ essence instead of _THE_ essence since it is possible there could be two or more implementations that have incomparable sets of requrirements where neither is a subset of the other.  In that case there are multiple essenses for this capabilitity.  Formally:

**CAPABILITY/PROBLEM** -- A _**capability**_ or _**problem**_ is a specification of the input/output behavior of a subsystem within some software system.

**ESSENCE/ESSENTIAL CAPTURE** -- An _**essence**_ of a capability is an implemetation of that capability that has a smallest possible requirement set.  (We sometimes say an implementation with a smallest requirement set is an _**essential capture**_ of the associated capability.)

There are three important observations we make regarding this notion of essential capture: its fragility, its impracticality, and its universality.  

**FRAGILITY**.  We notice this notion of essential capture is quite "fragile" in the sense that it is easily broken, and once broken it generally cannot be recovered.  Any implementation that depends upon another implementation that provides more capability than is strictly required will no longer be elegable as the essential capture for the provided capability.  For example, in order to provide the n-ary sum of a series of data values it is required that binary addition is defined on those elements, but it is not require that multiplication is defined.  Any implementation of series sum that depends on a math library providing multiplication will be in a small way overly restrictive, and in that sense not be an essential capture of the notion of series sum.  Worse yet, this superfolous dependence upon multiplication is forever carried by any implementations that recursively depend upon any composite implementation built from it.  This fragility effect is cumulative poisoning any code it touches.  The only solution to the fragility problem is to recursively build essential implementations on top of other essential implementations all the way down the software stack.  One can never bolt simplicity ontop of needless complexity, it must be built from the bottom up!


**IMPRACTICALITY**.  Which brings us to our second observation.  We say our aim here is pragmatic, yet the definition of essential capture above looks very very impractical.  How could any implementation built the modern stack of libraries, frameworks, and languages ever hope to be the essential capture of anything.  It seems impossible to even write a single line of code w/o depending on dozens or thousands of requirements that have nothing to do with the capability one is trying to produce.  That is correct, one cannot develop software in the traditional manner and even hope to capture the essense of some capability.  Still we show below, there is a way forward, a way practical means for capturing the essense of important software capabilities.


**UNIVERSALITY**.  The final observation about essensce is its universalitity.  Any essential implementation is either irrelevant for some use case, or it is a perfect fit for that use case.  As we have defined things above it is impossible for some capability to be needed in some context, yet somehow an essential implemenation of that capability is not applicable to that context.  The only requriements that an essential implementation has are those that are logically required by the capability itself.  If one fails to meet those requirements then one does not actually have an opportunity for use of the particular capability.  Consider the silly case where one thinks one would like to compute the sum of a series of elements where binary addition is not defined.  This just makes no sense, it is not somehow the application enviornment is not compatible with this particular implementation of series sub, not it is much more profound, this environment is incompatible with the CAPABILITY of series sum; it does not apply.  So essential implementations have a very nice property, either they are irrelevant, or they are implemented exactly as needed.


Given the framing above, it might seem we are aiming for some proof theoretic nirvana.  We could prove theorems that connect requirements, and capabilities, to their application environments, then write tiny 5 line functions that are provably connected to other tiny functions.  That might be a valuable endevor, but it is not the Uniform Agenda.  For us this is a PRAGMANTIC not THEORETIC framework for understanding software systems.  Thus, we assume both requirements and capabilities are specified using _natural language_ not some formal system.  For our purposes, it is sufficient that trained engineers generally agree when requirements have been met, and whether a given implementation does or does not provide some capability as described in natural text.  Further, our interests in essential implementations is pragmatic as well.  We don't care if some implementation is dependent upon a requirement that is not provably required for the provided capability.  We only care if IN PRACTICE, that implentation's extra requirements are causing it IN PRACTICE to to needlessly not apply in contexts where an engineer actually wanted to use it.  In that case we care.  So we seek software implementations whose requirements are pragmatically minimal, and that are pragmatically applicable in contexts where they are needed.  This is still a tall order, but as we will see, it is achievable even in cases where we have no formally understood essences.


{[ I have a section that connects these ideas back to JSON... but for the moment I omitted it. I want to see if I can at least nail this idea of compatible with and essence.  (even if you have no idea why this idea might be important).  let me know how I did.]}

~~~~~~

EXAMPLE OF AN ESSENTIAL IMPLEMENTATION

Here we provide an example to show in practice what an essential implementation might look like, and what practical value it might have.  For this we use the example, we frame JSON as an essential form, and use that framing to understanding the dramatic adoption that JSON has enjoyed.

Before lauching into that it is useful to first outline why one might expect that JSON would NOT make significant inroads against very established alternatives like XML....

So why did JSON perform so well?  We believe it is becasue it captures the essence of an important capability.  How so?  Well consider in the web-dev context a key ability that is required is the textual transmission of data-structures from one language (on the server side) to a different language (on the client side).  In framing this capability in a broad way it is important that one does not depend upon details of data structures in one language that do not map to other languages.  Further is it important to only depend upon details that easily map onto some textual representation, as that is part of the specfication of the problem.  For concreteness let me summarize the capability that I believe JSON captures:

**JSON Capability** -- The ability to losslessly transmit tree-structured containers of numbers and strings as a unicode text message, where the values within each container re indexed by position (using a natural number as with a list) or name (using a string key as with a map).

{[is the following  paragraph too abstract and terse... it is why I think the details of the JSON standard are a good match... but it might be too complex of a point to make in such a brief way.]}
We think this was a sweet spot in the space in capability space, since it dodges complexity in managing graph structures and maps that have complex keys which might not have a constant hash value.  At the same time this level of complexity allows one to cleanly map the OO-data that is common within many programming languages today... they only need constant string valued keys for their object fields.  

In assessing the JSON format many folks attend to the details of it parsing, its visual appeal, etc.  But from a requirements perspective, what JSON requires is pretty narrow.  It requires that one can:

**JSON Requirements** -- The ability to 
- map string values and numeric values bidirectionally into and out of unicode.
- the ability to iterate container structures as a sequence of key (number or string) and value (recursive form)
- the ability to create a new container give such a sequence of keys and values (all at once, or incrementally)

From the web-dev perspective, what are we to make of these capabilities and requirements?  In the case that the source and target languages are different, it makes alot of sense to try to transmit strutured data, rather than OO-objects since their internal structure and associated methods are not likely to match.  Thus maps with string fields  seems a close match.  One could make an argument that the JSON format would be improved by allowing graph structures rather than only tree structures.  Perhaps so, but it does add complexity when translating into a flattened form, and in any case it is often not a large constraint to satisfy.  Other than this one possible gap the JSON capabilities statement, it seems to accurately capture the web-dev data-transmission problem quite well.

On the requirements side JSON is just a beauty.  Nearly all programming languages allow these very basic methods of accessing atoms and containers.  Thus the JSON requirements add no superfolous constraints that in practice will preclude the creation of a JSON parser for languages that support the basic data transmitted by the format.  Thus in practice JSON is an essential capture of the textually-transmit-recursive-containers-of-numbers-and-strings capability in a way that other formats are not.  Notice, within XML one could define a sub-language that was equivelant to JSON, and thus would be another essntial capture of this same capability.  But in order for that sub-language to catch on, it would need to be a defined thing, and many parsers defined specfically for this sublanguage.  Saying ``here is a general purpose XML parser that maps all XML into a Dom structure that one can further process'' does not cut it!  One needs to explicitly define







 

Here we provide an example where essential capture played a pivotal role in the adoption of a now ubiquotous technology.  Before we explain why we believe the asendance of JSON as a transmit format is due to the essence it capture, we first consider why one might expect that JSON would acheve very small adoption at the time it was introduced:


So why was it adopted so strongly?

To answer this, lets consider a capability that is quite central for web dev:  data propagation.  Web developement involves multiple distinct procedural languages on the backend, and 

-- many data transfer formats that were more powerful, mature, and capable than JSON so why did it grow

-- First articulate an important capability: bi-directional map between tree structured program data and a lossless textual representation of that same data.

-- Requirements: programs native types support recursive composition of strings, numbers, lists and maps with string keys. 




~~~~~~

There are a some cases in practical programming, where the essense of an important capability has been captured and used in practice.  In those cases we observe surprisingly strong adoption, we believe it is BECAUSE it is an essential capture of an important capability.

Within web-development the ability to transmit data as messages between different systems running on a range of programming languages is an important capability---lets call this the data-structure-transmit (DST) problem.  It is important that any solution to the DSTP be compatible with a range of modern languages, this means the semantics of the transmitted data needs to map 1-for-1 into each targeted environment.  If one introduces semantics in the transmit format that don't exist in a given target, its not that one cannot encode the result in this turing complete enviornment, it is simply that that resulting encoding will not be as parsimonous as its source was.  For example, if ones trasit language assumed ordered maps, while many targeted languages had unordered maps, this would result in significant bloat at the target.  One could not be sure if the transmitted data was making use of the presented ordering, thus one would need to somehow encode the data on the target with a combination of structures that captured both the mapping and the ordering.

So selecting a transmit representation that make FEWER assumptions that are made by ALL target environments is key to having a transmit representation that is "compatable with" all of those targets.  JSON is the first popular transmit representation that captures the essence of the DST problem.  Nearly all software langauges support trees of lists-maps-numbers and strings.  So the assumptions made by JSON are a SUBSET of the assumptions made by each of these languages.  Capturing the essence is critical to the simplicity of JSON since the following code is only possible BECAUSE of that essntial capture:

    message = json.encode( data_structure1 )    # Code in the source language
    data_structure2 = json.decode( message )    # Code in the target language

It convenience of having simple encode/decode functions is ONLY possible for solutions to the DST problem that are compatable with the source and target languages.




 allowed ordered maps in trasit, but 


Marcin,

Given the semantics of plutil it appears







~~~~~~


Lets consider a range of different XML schemas that one might select if one wanted to provide guidance to an ontologist that is going to encode 1000 different domains.

It turns out we happen to know that these 1000 domains are going to be processed as POD data in Python.  Which XML language should we select?

~~~~~~

_
### --- 2021.07.21 - Marcel response using the 'plutil' converter

Marcel,

The use of 'plutil' is glorious here!  It really allows me to be very concrete about my claims.  I did not know this existed already on my mac.  Super convenient.

HERE ARE TWO KEY POINTS FOR THIS WHOLE RESPONSE:  
-- I agree that XML and JSON are representationally equivelant in the sense that one can losslessly map in either direction.  Indeed they are equivelant to a bitvector too.  So any discussion about JSON semantics has nothing to do with representational power, as they all are equivelant.
-- Just because  A does not embed into B does not imply that A' a sub-langauge represented in A also does not embed into B, A' very well might embed into B, even when A does not.  I think you are assuming this implication holds, but it does not.

So while I claim that XML does not "embed" into python lists and dicts, I definitely do conceed that one can define a sub-language within XML, that has additional constraints and semantics, and this sub language WILL embed into to python.  In general, when a language A will not embed into langauge B, it is because language A is "too big".  Thus it is often possible to define a sub-language that is "small enough" to fit.

The plutil utilility is not mapping into generic XML, but rather it is mapping into a sub-languages defined within XML.  This sub-language has special semantics for tags like 'array', 'dict', etc.  But of course it is very possible to create sub-languages of XML what will embed, even as generic XML does not embed.  Here I am checking to see if we what happens if we use this dumper, but we just use generic XML semantics.  (e.g. I am editing all those tags so the data dumper is using generic XML semantics)


<zarray>
	<zdict>
		<zkey>first</zkey>
		<zstring>dan</zstring>
		<zkey>last</zkey>
		<zstring>oblinger</zstring>
	</zdict>
	<zdict>
		<zkey>first</zkey>
		<zstring>marcel</zstring>
		<zkey>last</zkey>
		<zstring>weiher</zstring>
	</zdict>
</zarray>


This is the conversion of the names.json into XML, but w/o any semantics implied on the XML keys.
Now lets convert that to JSON....


% plutil -convert json names.xmlplist -o names.json
names.xmlplist: Property List error: Encountered unknown tag zarray on line 4 / JSON error: JSON text did not start with array or object and option to allow fragments not set.

Bummer! I was hoping this was a universal XML to JSON converter.  It does not seem to be one :-( 
But as you correctly point out.  Such universal converters do exist.... Here I found one xq... oh it depends on jq.... oh my brew is boinked...  there, now it is working.  If you want to follow along:

$ pip install yq    # this will install xq as a side effect
$ brew install jq   # this is needed by xq

$ cat names.xmlplist | xq .

{
  "zarray": {
    "zdict": [
      {
        "zkey": [
          "first",
          "last"
        ],
        "zstring": [
          "dan",
          "oblinger"
        ]
      },
      {
        "zkey": [
          "first",
          "last"
        ],
        "zstring": [
          "marcel",
          "weiher"
        ]
      }
    ]
  }
}

So this dumper did map the generic XML into JSON (and we could use this as an initializer for Python too).
But according to XML semantics the only thing it knew was that the zdict node contains two zkey nodes, but it did not understand the implicit connection between the zkey node and the following zstring node.  


Indeed, in my example I chose the exact structure of my first two XML examples in order to highlight the underlying problem with XML dumping.  {[sadly my prose was not as good as my example!!]}

<capitals>   <Ohio>   Cincinnati </Ohio>    <Iowa>      Demoines   </Iowa> </capitals>
<directions> <forward> 20 </forward> <clockwise> 90 </clockwise> </directions>


An engineer that wants to indicate the _functional_ mapping from states to cities could use XML as shown above.  Likewise if they wish to indicate the _ordering_ between some direction commands to a robot they might use the XML shown below that.  We argue these are very natural ways to indicate these respective semantic relations using XML; please look at the examples and see if you agree.  Also notice the structure of the two XML examples above happens to be IDENTICAL.  Thus any determinstic XML-to-Python converter must map them to the SAME resulting structures.  But, given the order-dependent semantics of robot-directions most engineers would prefer a top level list structure for that, and a top level dict structure for capturing the functional relationship between states and their capitals.  Examples are shown here:

capitals = dict(Ohio="Cincinnati", Iowa="Demoines")
directions = list(dict(command="forward", arg="20"), dict(command="clockwise", arg="90"))

These outputs are not parallel, so no matter what an XML dumper does, it will do the wrong thing on at least one of these two cases.  Notice, the same is not true for JSON.  An engineer who understands the mapping from states to capitals, and the ordering of robot directions would generally encode this data using a JSON Object for the first case and JSON list as we show here for the second case:


	{"Ohio": "Cincinnati", "Iowa": "Demoines"}
	[{"command": "forward", "arg": "20"}, {"command": "clockwise", "arg": 90}]


And of course these are trivially mapped into the desired Python structures.  So why is it easy to write a JSON-to-Python dumper that does not require refactoring on its outputs, while it is mathematically impossible to write an XML-to-Python dumper that does not require refactoring on its outputs?

It is because the set semantic assertions one can make (and build upon) when using the JSON language is mostly a SUBSET of the semantic assertions one can make about Python lists, dicts, Numbers, and str.  Notice the word "semantics" here does not refer to the expressive power of the representations, XML, JSON, and bit vectors, all map losslessly into the other representations.  Instead the semantics of JSON includes facts like:  ``The text-file ordering of the key/value pairs of a JSON object have no meaning, and the mapping from its keys to its values form an injective (functional) mapping.''  These are the semantic assertions about JSON representation.  It turns out it is possible to define a 1-for-1 mapping from JSON representational parts (like "{") onto Python python representational parts  (like dict) such that, except in a few edge cases, the FULL SEMANTICS of JSON is preserved when mapping into python.

This is just not true for XML. Of course *is* possible to preserve the FULL semantics of XML when mapping onto Python-lists-and-dicts, but one would need to be very verbose in order to do that.  Since order matters in XML, one would ALWAYS need to use lists to encode nested nodes, since we cannot be sure if their ordering matters for any given case.  And these nodes will also have a tag name, and may have attributes too, thus it should have a dict along with each list in order to contain that data.  Because of this verbosity, many POD-data-XML dumpers to  do not attempt to fully capture the XML semantics since the result is just so so cumbersome.  Instead these dumpers opt to make heuristic guesses about those semantics, but these are guesses that are just plain wrong some of the time.  Many XML parsers dodge the whole mess by creating Python class objects that perfect match XML semantics, thus providing both parsimony and precise rendition of XML semantics.  That work well, but it does not support POD-style programming.

At a very pragmatic level here is the situation: if one set of engineers are going to be encoding data in some transmit language, while another set of engineers are going to be processing that data as Python lists and dicts, then JSON as a transmit language simply crushes XML as a transmit language.  Complex XML formats will require notable effort to re-architect them into a structure of Python lists and dicts that the engineer would like to perform processing over.  In contrast, if all of that data had been encoded into parsimonous JSON formats, there would be essentially no work to be done at the Python level.  Required representational decisions would have already been correctly made when encoding the data in JSON form in the first place.

Notice JSON simply crushes XML for this use case.  There is no advantage to mapping a domain knowlege onto XML semantics, only to turn around and refactor it into JSON/Python semantics.  If you need Python data semantics, it is far better to simply encode the data directly into Python data semantics!  And this use case is the dominant use case for web programming when POD data manipulation is being used.

JSON crushes XML for dumping into Python POD data precisely because JSON embeds into Python POD data, while XML does not.  At a practical level embedability really matters.




~~~~~~
INSIGHT ALERT -- Beep beep beep.

If you notice, in my original rendition of embeddability, I defined embed as a "you know it if you see it" kind of definition, and then gave my thought experiment to clarify.  Even then I was using the phrasing to say one languge "fits into" another, but I was not really clear in what sense does it 'fit it'

I think I can formulate embed in a more precise way by saying:
	A embeds B iff there exists a there exists a 1-for-1 coorespondance where all semantics assertions about language B are preserved in A (they are a subset in assertion space)

this is still very hazy in my mind, I am still not saying it in a mathematically precise way, but at least I see the space where JSON is a subset of python.  It is the set of assertions about JSON semantics that are a subset!

And this is the key.  It is the semtantic assertions that an ontologist is using when they encode data, and if those assertions continue to be true in the target, then it means that mapped data will continue to be valid.  But the 1-for-1 constraint is key here... it is no good if one can generate a super verbose mapping in the target in order to capture all of the source semantics.  In that case the engineer would NOT select the auto-mapping, but instead would opt for some refactored representation.

Now I see a glimer of how I might be able to say 'embedded' in a mathematically precise way.


This is an idea that I did not have prior to our discussions!






_
### --- 2021.07.14 - Marcel response.  Embedding Rewrite



The way these grand visions have gone for me is about as follows:

1.	Feeling of unease, something is wrong (this is a good instinct that I also look for when hiring engineers)
2.	Intuitive leap / grand but fuzzy vision that cleans up all the “small small” that others have wasted their time on
3.	Clarification of grand vision with prior art and the real world (and explaining to others)
4.	Depression, because my grand vision was (a) just BS or (b) just something invented decades ago
5.	Continued depression
6.	Realisation that my thing is actually different from previous in some small way
7.	More depression, because the mountain laboured, and what it produced was a mouse
8.	Realisation that this seemingly small difference is actually significant
…

I am currently at the steps following step 8.


Hehe, yes, and I think I am at #3, and I am waiting for the shoe to drop.... for you (or another) to say,
``well dan, this effort is very much like what professor Bluzaldorf in Romania did with 20 of his students ove a decade, and this is what happend.''  or to realize:  Actually my thinking about embedding (or other things) is just hopelessly fuddled, and really it does NOT say anything like what I think it says.

Yeah, just waiting for that!  Really I should never have allowed myself to think so long by myself!  It is just most of the time I was just struggling with a feeling that I was only 1/2 saying the thing just at the edge of my grasp.  only a bit more thinking and I will HAVE IT!  (and right now, I still think it is close!)  But at least I am forcing myself to try to see if at least one smart, very informed, very on my side, guy can get parts of it.  It I cannot succeed in this most modest test, I really have to question what I really have.



Here is responses to your comments, and a rewrite of the embedding idea.  This time I just stuck only to the notion of embedding, and tried to tie it to concrete XML / JSON structures:


~~~~~~~~~~ 



DAN:  I can see I need to get even more narrow, before I can truly land my first claim.  (which I agree I still have not done.)  Let me try to get to the "bones" of this argument and see where we diverge.

MARCEL: Yes.  It is still very unclear what exactly it is you are claiming, and even more unclear if and how JSON/XML/plists supports that claim.

MARCEL: As far as I can see, the only real difference is this:
- FROM DAN:  but to have ever had a shot at supplanting XML, one would have needed to have libraries with encode/decode written in web languages.  (Did we have those?). 


EINSTEIN:  “If you can't explain it to a six year old, you don't understand it yourself.”  — Albert Einstein


I very much agree.  Therefore I dont yet understand this yet!  But maybe I am close?  I continue to believe there is a well-formed and important notion of embedding in here.  It is just one piece of Uniform, but I have thus far failed to articulate it!  I am going to try to at least show the difference between XML and JSON using a most narrow formulation of embedding that only speaks about representational embedding:


~~~~~~~~~

**Representation R1 _embeds_ into another representation R2 if there is an "obvious, simple, lossless (injective) mapping" from R1 onto R2.**

Here is a thought experiment to determine if there exists an "obvious, simple, lossless (injective) mapping":

Lets call R1 the "transmit representation" and R2 the "program representation".

In our thought experiment, Alice is an ontologist that specializes is capturing domain knowledge and related domain processing to be performed over that data.  She captures her sample domain data using a text editor and directly encods that data directly in the transmit representation R1; she also captures the relvant processing operations and expresses those as free form natural text in English.

For an unrelated project, years ago, Bob wrote a simple, lossless data dumper that accepts arbitrary data expressed in the transmit format R1 and then outputs data in R2 (the native data structures in his favorite programming language, Python).  Bob is a big fan of Plain Old Data (POD) scripting, so his dumper just maps onto the native Python data types.

Now Bob it tasked with implementing hundreds of stand alone mini-applications that each coorespond one of hundreds of the domains that Alice has encoded over the past years.  To start with for each new application, but first runs his Python decoder on Alice's sample data, and look at the resulting data structure.  This is just some test case data, so Bob does not want the details of this output format to overly influence his architecture for each application.  He wants to choose a data architecture that will be ideal for that domain, given his programming language Python, and the data processing as outlined by Alice in her English overview of the data processing required for each application.

In some cases the format that happens to be generated by his dumper is just fine.  He sees no reason to refactor that that test data in any way, and he just builds the app around those data structures.  Lets call this the "direct mapping" case.  The "refactor" case is alternative when he saw good reason to select a different data representation given the required processing.

Now we can compute an "embedding ratio" as follows: 
	Emdedding_ratio = num_direct_mapping_cases / ( num_direct_mapping_cases + num_refactor_cases )

This is a ratio between 0 and 1 which indicates how often this blindly created dumper function just happens to yield a great representation for Bob's POD scripting.

We claim that "R1 embeds into R2" if and only if the R1-to-R2 embedding ratio approaches 1.0.  Why?  The only way the ratio can approach 1.0 is if somehow Alice was implicitly honoring the constraints that Bob will later be operating under when he designs his data representation, and that can only happen in the case that fitting into R1 CAUSES one to automatically be fitting into R2.

In our thought experiment imagine that Alice used XML as her transmit language for half of the cases and JSON for the other half, while Bob is using Python as his programming language throughout.  Here are three examples data forms to consider:  A map of the capital cities for each state, a list of people's  first and last names, and a weeks worth of temperature data for several locations.  All expressed just as Alice did in XML and then then for comparison, Alice went back years later and independently expressed them in JSON:


<capitals> 	<capital state='Ohio', city='Cincinnati'/> 
			<capital state='Iowa', city='Demoines'/> </capitals>
<names>		<person first='dan', last='oblinger'/> 
			<person first='marcel', last='weiher'/> </names>
<temps> 	<location name='Cincinnati'> 73 64 71 69 72 78 74 </location> 
			<location name='Demoines'> 78 69 81 79 78 74 81 </location> </temps>
			
And for contrast here we consider what Alice might have written if she were using JSON for this same data:
	
	{"Ohio": "Cincinnati", "Iowa": "Demoines"}
	[{"first": "dan", "last": "oblinger"}, {"first": "marcel", "last": "weiher"}]
	{"Cincinnati": [73 64 71 69 72 78 74], "Demoines":[78 69 81 79 78 74 81]}


Notice that the first two examples the XML is structurally identical.  Both are a list of elements that each have a pair of attributes.  But in JSON they are not the same, one is a JSON object, reflecting the injective nature of the mapping between states and their capitals, while the other reflects the container-of-elements nature of a list of names.  One could have kept a parallel structure by having a single XML node with 50 attributes, one for each state, but that does not feel like very natural for XML.  In any case, when we consider the third example, it destroys that possibility---the list of temperatures could not be encoded as a list since XML attributes can only be a string.  Thus Alice was required to encode this a children of the XML node.

Now lets try to imagine how Bob's universal JSON to Python and XML to Python dumpers must work.  If we think about the semantics of Bob's data dumper for JSON, it is pretty straight forward.  every JSON "{" object becomes a dict, and every "[" square bracket form become a Python list.  But what about in the XML case?  If the operations that Alice described in English for cities and states involve state capital lookups, then overwhelmingly Bob would want to express this data as a dict.  And likely Alice would have use the '{ ...}' notation as well, since she is explicitly talking about the injective nature of the state-capital relationship.  But it is hard to see how Bob's dumper would get this right for both the names list and the capitals list, since they have idential XML structure, but different best Python structures.  It seems there is no good way for Bob to create an XML dumper that yields parsimonious results.  But whatever he did do, it would frequently generate verbose dict/list structures in Python that he would then need to refactor.

So XML does not have an embed ratio that approaches 1.0, while JSON does have a ratio near 1.0 as long as Alice uses JSON in the best way given the operations she was writing down in English at the time she encoded the JSON data.  Why is this?  It is because JSON "fits into" Python lists and dicts in a way that XML does not.  XML is expressive in ways that are valuable for many kinds of data, but that don't "obviously, simply, and injectively map" onto Python lists and dicts.

So what are the practical consequence of this mismatch?  Well in many cases Bob will need to write scripts that "cleanup" the output from his XML to Python data dumper in order to build his tests cases; while he needs to do almost no work for JSON-dumper in this regard.  And this is a difference that makes a difference.  Data marshalling, data reformatting code is just tedious to write, and it can easily represent a significant fraction of ALL of the code needed, in the Web programming case where one is just parsing some data from the server, performing a modest amount of processing on that data, and then presenting the data on the client side.  In that case, JSON can significantly reduce total work required relative to XML when maping into client side POD data.  We think this parsing-POD-data-processing-presentation pattern is quite prevelant in web programming.  And we think the significant advantage that JSON has for these POD data uses cases is one key reason why JSON made such strong headway against its solidly entrenched XML rival.  Embedding matters!



~~~~~~~

 














If all three cases 
	


Lets just look at the last case.  Alice and Bob both understand states only have one capital, and if the processing will require us to frequently look up info about a state, then using a map of states is a smart thing.  One could have encoded this in XML as:

	<states>
		<capital state='Ohio'> <state_info> 
			<pair key='city', value='Cincinnati'/> <pair key='pop', value='600K'> </state_info>
		<capital state='Iowa'> <state_info> 
			<pair key='city', value='Demoines'/> <pair key='pop', value='200K'> </state_info>
	</states>

One COULD have encoded the XML that way, and maybe someone WOULD encode the XML that way, but we argue the only person that would choose that particular twisted form, would be someone who was explicitly thinking about the target programmer data form representation, and trying to "land on it" in a consistent way.

Notice by contrast such mismatches are very hard to even constrcut with JSON since JSON's representational constraints "have the same shape" as the representational constraints that python dicts and lists have.  What works well for one will work well for the other.


And this is the point.  In practice there many data sets are expressed in 'transmit data form' no constraint is placed on the program form.  But if there exists a decode function that maps into POD data that the programmer is happy to not refactor, but instead to just use directly, then we have totally automated a chore that will still exist in a language like XML where the resulting data will NOT be correct.  (indeed the power of XML is so mis-matched to the representational power of python dicts and lists, most parsing libraries do not output dicts and lists, but instead the output special DOM Node instances that better match the semantics of XML.)  Json by comparison only uses not POD based parser when performance indicates it, because JSON embeds into Python while XML does not.


Highly embedable languages like JSON are quite powerful, they are analogous to the idea of a "universal donar" in blood types.  Type O-negative blood is the single thing you can pack into your med kit, knowing that it will be ok no matter where you later need to inject it.  While XML is more like AB-postive blood, it can make a real mess if you just go injecting it willy-nilly into any body you happen to find.  (Of coruse XML is far from the worst, perhaps we should reserver the AB-positive analog for yet poorer choice for a universal doner...  maybe SGML or such.)



{[
	I STILL don't understand this well.  If I did, I could say all of this stuff with 1/3 the text!!
	But I am humbled by our earlier iterations.  I want to first just see if this verbose explanation 
	let you 'see' what embedding is suppose to mean.

]}






imaginging the data as the targeted data structures 

there are very natural representations of data in XML


a stricter set of constraints

the constraints





those hundreds processes as described by 

  Bob and Chris are two engineers in completely separate organizations that recieve the texts from Alice and use them as the basis for encoding data from these domains.  But Bob is encoding into the transmit representation, while Chris is encoding into the program data representation (along with a script for data entry)




independently capture domain data from these domains, Bob uses the transmit representation, and 

which she typically describes in English)  After the fact, sometimes Bob gets called into to develop applications that operate of the same data using the processes that Alice has documented in English.

For simplicity, Bob has written some code that maps the trasmit data into program data, it maps from R1 to R2.  Alice doesn't know that Bob exists (say they both work for Department of Defense and it is classified :-) and Alice does not know the programming language that Bob uses.






, Bob, and Chris are three engineers all familar 





_
### --- 2021.07.01 - WODAT
WODAT = Way of Doing a Thing
BU = Built Using
DEP = Depends upon
ATTBD = A Thing To Be Done
TTBD = Thing To Be Done
CAPABILITY = 


BEHAVIOR / PROBLEM / SOLUTION =
- a specific function
- infinite spec of inputs/outputs
- a specfic structured data value
CAPABILITY = cohesive set of behaviors
- Natural langauge description
- a spec langauge that can be mechanically translated into an executable exhibiting behaviors
- an implementation of that compiler
_


_
### --- 2021.07.14 - Marcel response on Essence

> I also don’t see the fact that there are single methods to do the conversion as demonstrating that plists/json captured some “essence", but simply as a convenience.  Convenience is an important factor in adoption, but again, very distinct from anything I can identify as an essence.

Marcel,

I can see I need to get even more narrow, before I can truly land my first claim.  (which I agree I still have not done.)  Let me try to get to the "bones" of this argument and see where we diverge.



[1] One can imagine the programming tasks exist in some very large space of all possible things a programmer might want to do.  Then we can define a "capabilitity" as a cohesive set of these tasks.  So "sorting fixed lists of elements given a "<" operator, would be an example capability -- an example subset of that space"

[2] When I say capturing the essense of a capability, I mean approximating the shape of the boundary of that set -- not too big, and also not too small.

[3] I would claim that "data representation used for transmitting tree structured data" is a capability of some importance in the web-programming world, and that JSON more closely matches the shape of that capability than XML.

Now why does it matter that JSON is a close match?

[4] BIG ENOUGH -- Given any pair of languages, say ruby on the back end and javascript on the front end, it is important that the data that is expressible by the intersection of these languages is also expressible by our prospective transmission language.  Otherwise our transmission langauge is unnecessarily restricting the solution space that might be used.  All things equal we would not want to accept such a restriction if we didn't need to.

[5] SMALL ENOUGH -- The only way for their to be invertable encode/decode operators that map directly into native data types is for the encoded data representation to be a subset of the native types.  Specifically:
	transmit_data1 == encode(decode(transmit_data1))
Can only be well defined over all transmittable data iff one can define a proper mapping into the native data.

[6] CONVENIENCE MATTER -- Having a two operator API, and being able to directly utilize native data structures without mapping them is valuable to the programmer.  All things equal they would much prefer to use a transmission representation that affords these advantages.

[7] CONCLUSION -- Thus for the programmer that requires the "transmit structured data" capability, JSON offers the advantages of big enough and small enough, while XML only has advantages offered by the big enough side, but looses the API encode/decode benefits of the small enough side.  In the web-dev context this simplicity of just squirting the data in and indexing into it using 2-3 lines of code is quite powerful, and likely did contribute to the acendence of JSON even after XML was well established.


{[
- I do agree the difference is a matter of convenience.  I am only arguing that this convenience is causally connected to the 'small enough' aspect of 'right shape'
- I do agree in the case that one hopes to do object marshalling that json's advantage diminishes, indeed I think many fancy graph structure object marshalling services are still done using XML.  JSON's advantage is in this simpler case.  I would just notice that this simple case seems quite BIG in usage terms, and that is why JSON has grown so much.
- I wonder, even if you "buy" the argument above, you might still feel that this idea of big-enough/small-enough shaping matching is distinct enough that calling it 'capturing the essence' is still to wishy-washy to add rather than detract value from the claim being made?  In the end, the key claim I want to make is a claim about needing to create micro-languages that "right size" relative to the capability they hope to address.  If the word 'essense' get in the way of that notion, I should just dump it!
	(it is the way I think about it, but that does not mean it is the right way to express it to others.)
]}



PLISTS
As far as plists go.  They seem to be a nice transmit language.  
- but to have ever had a shot at supplanting XML, one would have needed to have libraries with encode/decode written in web languages.  (Did we have those?). 
- Also, as the never-been-updated 1-page web spec at http:://json.org the spec is truly simple.  Does an equivelantly precise yet simple spec exist for plists?
- Finally I wonder about numbers.  Having an integer encoded as a string (without typing info include) along side strings also without typing is inconvenient.  Now I need to add extra info to indicate numbers and strings, then do conversions of the appropriate strings.  Ugh.  In that way plists look inferorior.

Still I am not trying to claim that JSON is perfect, or that many other alterntaives could serve the same role. I only want to claim that getting the "shape" kinda right, is key when providing a capability.

Does that narrow claim feel well supported?  (even if it does, I now need to figure out how to say it clearly on first pass!!)


~~~~~~~~~

You wrote alot about message based APIs vs.  representation based APIs.

There is ALOT to unpack in here.  (pun not intended, but also not deleted once observed. ;-) )

It actually connect to a part of the Uniform Ecosystem that I call 'live' data -- and this is probably the part of the ecosystem that could have earliest actual relevance for real software systems.  Still I resist going too deep, since I have yet to establish even the most basic claims of the whole darn endevor!

> Anyway, that’s probably more than you wanted to hear on the subject of JSON and XML :-)

No I bet we will talk about it yet again.... but let me say just a little bit right now:


Uniform aims to get the semantics right first.  At the semantic level there is no difference between JSON decoded as a static structure, and JSON decoded into a temporal stream of messages.  But once you have selected a given representational class, THEN various methods of processing that information become relevant.


The parts of Uniform that deal with 'live' data allow us to decomplect how we will interact with some data from the representational class and actual data values persisted.  Essentially every unit can be treated as a persisted structure, and as an iterator over that structure at the convenience of the programmer... and a combinations of eager, lazy, and caching are used under the covers to operate optimally given how the programmer is using the live object.

I get a big warm-fuzzy idea when I hear 'tyrany of call-return'.  I think it connects in a deep way with this idea of separating the decision about what that data is, from the decision about how to best access that data.  But as I said, I think it is a deep deep hole, and I don't understand it well. (and I am even having great difficulty expressing ideas that I though I DID understand well).  (again we have a post-hoc pun :-)


~~~~~~~~

I started out with the idea 'essense' but in retrospect I think the idea of 'orthogonality' might be the better starting point for what is different about the Uniform agenda.  I have been struggling to express this idea in a pity and clear way...  I will send you what I end up with once I am not embarrassed by it.

Cheers!
--Dan




















