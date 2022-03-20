

# NOW CHOICES
### type tree inheritance
CHOICE: How is this encoded?

BG:
- It is a DAG therefore it seems to relate to lexspace
	==> but maybe not directly since lexspace is a tree
- Types themselves are multi-level idents to structure of ident must be different than structure of DAG

Seems this question is about the uniform way to encoding a DAG / Partial Order
### QUOTE OPERATOR
-- Should we have a QUOTE == _.foo(3, 4) constructor?
- PKG.WITH - how do variables connect with singleton global with place
# _

[[CHOICE: unit/key/value. subject/predicate/object ]]
# ### CHOICES ###
### BACKING -- delete-slice [resolved]
ISSUE: how to encode the shifting behavior of slice setting

CHOICE:  use slice and set value to None or empty seq to remove it
### STRUCT.Backing -- live|lazy units - handle sub-units?[resolved]
CHOICE: when a unit exists, it type must be already known at creation.  Thus the structure of its parent is not defined until its type is known.

ISSUE: A live unit is changing its structure.  this means any pointers to the structure can become stale anytime.
ISSUE: Lazy structure is derived on the fly.  Thus at some level there must be an entity that is referenced by the completed level above, but itself is not complete.  How does one correctly choose the type of this incomplete instance?  (maybe it is in code form?)

Soln #1 -- LEX -- all pointers are to lex units whose code value can be swapped for any object after eval.

Soln #2 -- Universal Unit Root -- All units contain content unit inside.

Soln #3 -- FWDING -- All units may point to a newer value, pointers may be updated (or not) but semantically new unit's value  is used.

Soln #4 --

Soln #5 -- TREE/INVERSE MGT -- Trees / inverse links are managed explicitly.  Stale nodes are explicitly marked and later access causes them to be rederived

Soln #7 -- NO CACHING / NO STATE -- a live node is entirely defined by its spec (file_path) all accessors directly access the filesystem.  (race condition occurs if state of single filepath obj changes while accessing different parts of it.)
### how Uniform is embedded into Python [resolved]
CHOICE:  
- Units adhere to the abc.Mapping type & interface.
- Light wrappers are used for any python objects to map interfac
### backing - LISTY [resolved]
CHOICE:
- Listable is property to be mixed into a Unit
- then list_len or lenght determines if unit has list props.

Issue: how do we distinguish if unit behaves as list.

Idea1: Lists are a subtype of Units
- 'listy' returns an Int or inf if unit is listy.
- Are a restricted subset of all units.
- The keys of its items are guaranteed to be 0, 1, 2, ...
- expanded as "[ ... ]"  else as "{ ... }"
- listy(x+y) = listy(x) + listy(y) if x and y are listy

Idea2: items number and ordered are treated as such, else not.
- no special treatment needed for get/set/items
- "+" 
### Unit Indexing Order -- index, key, value   [resolved]
Choice:  In the most general context units are arbitrarily ordered multi-sets -- an items iterator sequence with no constraints
(arbitrary keys, values, finite/infinite)

Issue:  choose how unit indexing works
Choice: it seems we need to put key before value even in cases where we want auto default for key
Idea:   allow defaulted parameters BEFORE non-defaulted parameters

Situations:
- As triples:  [Subject, Predicate, Object]   [unit, key, value]
- For loop:    for k,v in u.items()
- For loop:    for k in dict(...)
- For loop:    for v in list(...)
- For loop:    for i,v in enumerate(list)
- Declaration:	Map(Int, Str)				//  what is the key and value type?
- Decl type:		Int => Str
- Decl 			Space(key, value) 			//  key should default to Ident

constraints:
	for e in a_list:	// this needs to iterate the elements of the list

_
ISSUE: ordering
CHOICE: yes
Allow mixing of fixed and named parameters?
	- How do we update "List form" in order to present in JSON?

	["^head", 444, 333, "=foo", value]
### str.back.metakeys  [resolved]
ISSUE: We need a "clean" set of keys for data structures.  We need meta parameters.  we also need a view w. all data.

CHOICE:  
- structure is clean and does not contain any meta keys.
- at least three meta keys exist for all units:  head, ns, len
- special meta views map meta keys into items by using special 'uf.meta.xxxx' namespace.
- Skip all the other shit here (e.g. 'soft' option etc.)

 META -- How to exclude meta keys by default
 
 STREAM -- default iterator (covers only named parameters)
 
ORD -- Keeping track of which subset of keys should be used
Choice: (Many options)  choice = Soft View Option

- Default ORD option.  Each Unit type has a default filter on their ORD, but any other filter may be used.  Lists use a listy filter.  Forms use a listy+named filter, etc
	- But all filters are just FILTERS, no re-ordering is permitted.
- Soft VIEW Option.
	- Each unit has its default ORD.
	- Other ORDs may be specified instead.  These can REORDER the defualt ORD.
	- This is a SOFT view in the sense that GET and SET still show values even those excluded from one view or another.

SOFT VIEW OPTION
- allows many different views (ORD) within one code block and no need to determine which GET/SET associate with which ORD (since results of GET/SET are indepnent of containment in ORD)
HARD VIEW OPTION
- provides reliable data hiding, since fields are simply not available by GET/SET or any other method
### str.back.opt.listy [resolved]
- how does one know if a unit is listy?
- what does the default interator interate over

CHOICE:  if meta.len is defined.
### structure.backing.new [resolved]
See ufparts
- where does make_seed fit in?  
- what is precise call chain on creation?
- call chain on seed creation?
CHOICE:  see structure.new
### STRUCTURE -- Atomic subtyping [resolved]
ISSUE: Can atomic types subclass in ways that are not consistent w. underlying type system?
e.g. Int extends Num, and Sym extends Str

NOTE:   Just because Int extends Num does not mean Int(bits:32) extends Float(bits:64)
		But it does mean if x Int=7, and y Num, then y=x is allowed

CHOICE:  Yes and No.
- The seed for certain unit create unit w. head and ns and no setters.  These units have frozen consistent types.
- Others may allow casting one or both in a way that is consistent w. underlying structure and get/setters.
- Others may allow anything goes.



_
### unicore - Structure/Function redundancy [resolved]
CHOICE:  yes.  
- Datatype is field by field constraint on structure.
- This can translate to a method by method constraint on class.
- interp is simply 'GET' on env objects.


Issue: constraining structure seems related to constraining function.  can they be collapsed?
### uf.op formal args   [resolved]
Issue: How to indicate optional and keyword args

foo(...allargs)
foo(args int...)		foo(*arg: int)
foo(kwargs: int...)     foo(**kwargs: int)
foo(x, y=None)
foo(x, y=)
### ! rewrite system -- how to spec, how it works
### ! how do we can to produce forms?
### ! how is a schema root identified and initialized
### Oblit template-fill and VAR n and b keys
idea is that schema is really an instance of Template
_
### ! VAR 'k' KEY or 'b' BACK
CHOICE: what to call the specifier of the backing index
_
### ! UNIT MIXINS
### ! LOAD ENV
ISSUE:  what env is used during eval & load of a new system.

issue:  often need to use app specific functions in some parts of load
issue:  there are two origins, the base lang and the lang under construction


_
### ! UNIT KEYS
Are unit keys strings or ident?

CHOICE:  Unit keys are the elements of an Ident.nub's path
		They might be strings, or interned strings or ?symbols?
### ! NS XFORM - most general transform

1. NS node fully specifies Transform Spec. Should cover:
- rewrite rule system
- head indexed rules
- method invocation grouping
- if-then-else / gates / cases

2. Merge XFORM spec and lang spec
- boot spec should allow for rewrite rules

3. Parsimonious expression of both top and bottom.
- ability to invoke xform trivially.  (by name alone?)
- ability to reference / extend parts of xform
_
### COMPILE - different alpha types to different target architectures
_
### .ATOM - Symbols
RESOLUTION:
- IDENT PARTS must be interned strings or ints
- IDENT are hashable, immutable, (and can be interned unique)
- STRINGs may be 'interned' 
    -- result is unique, hashable, immutable.
	-- result might still be a string or not.
- NS keys must match IDENT PARTS; interned strings/ints


Issue: how to symbols relate to strings
Option: MinCommit defines sym w/o resolving issue
Option: a specific set of strings

- HASH EFFICIENCY -- Many languages want hashable things to have  precomputed hashes, or to hash based on address.
- NO SUBCLASSING STRING -- Some languages don't allow subclassing string type.

Option: MinCommit
- MAYBE OVERLAP - Symbol may or may not be subset of string.
- PREHASHED - Symbol does have a hash value precomputed.
- IMMUTABLE - Symbols cannot be modified
- UNIQUE - There is one one symbol with a given char sequence
- HEAD - Head() is an Ident1 (Sym or Int)
- NS is Tree[Ident=>Gnd].  Gnd is Meta=>(Int=>REDUCER)


IDENT
- IDENT1 - Ident1 is Sym or Int(0,)
- IDENT2 - Ident2 is Sym or Int or Meta
- IDENT  - Ident  is List[Ident1]
- HEAD	- head is an Ident


COMMITMENTS
- INDEXING -- ident[i] is valid GET arg type for NS tree.
- PATHING -- head is a valid arg to NS.path_get
- UNIQUE - Ident and Sym instances are unique and hashable
- ? ANY - Unit head can be anything, but convetion is Ident
- BREAK - head() type and GET arg type are different
_
### META
how do they work?

RESULT:  Use meta key to access non-structural data via structure of special 'meta' unit.  Meta data may also be accessible via other operators (e.g. the head op).

SEPARATE -- iterators need to skip these so the should not overlap with symbols.

meta keys separate????
_
### SOURCE BLOCK -- TEXTPUTER rule embedding -- 
- RULE PATTERNS -- How to express rule patterns?
- CODE MIX -- How to mix code-ish stuff into text files?
- BI-DIRECTIONAL -- text push/pull.  How to indicate insert pt



	    _|_  .tRules  _|_


	    _-_  .tRules  _-_


	    -_-  .tRules  -_-


	    |_|  .tRules  |_|

	    _!_  .tRules  _!_
_
### .Functional Embedding
IDEA: Any functional computation can be embedded into a structural space.

does this connect to the idea that eval is really just 'get'?

RESULT:  A 'get' on an Env is an eval.
Still need "interp" as more detailed op allowing the providing of multiple levels of form expansion 
_
### PyObject tree
- how does absolute paths work
- how does py.import vs py.file work
- how does bang combine live & static parts of namespace
_
### 'with-constructor' for immutibilty
how does this work
### 'formula'
Figure it o
### 'backing'
require the all wrapping structures use the backing key in order to clarify that the passed unit will live in result.  --OR--
Just use the existing template parameter to pass this kind of info
_
### Encoding a Partial Order
ISSUE: how does the lexical '<' operator relate to a user-defined PO tree

SUBISSUE:  can a thing be multiple things at once?
_
### Colored Paths
ISSUE: How do have parameterized types
### Dynamic type constructions
ISSUE: How to dynamically construct types
- Order among type parameters is relevant
### RDF access patterns.
compactly encode the 1-to-1 ness and the accessibility of all combinations of a triple.
### ACCESS BACKING
DECISION:  Is there a uniform way of accessing a unit's backing?
### UnitConstructor's first arg

Choice: No first arg. just use keywords
Choice: follow List and Dict model?
### TEMPLATED PKG
Issue: How to build statically typed templated pkg 

Option: Create a subtype of PKG which 

BG: 
- Both pkg and class are source code specifications.
- classes can be subclassed and in that way can be templatized
### DATA STREAM-PLACE -- how do these relate
### backings py-wrapper FNS handling
Issue: how does a py_wrapper get the FNS associated with the current universe
### functional.programming_model 
SEEDS: How are seeds provided to bang
FNS vs ENV:  What is the difference here.
### --- fn.pkg -- how to group constructs ---
   Issue:  want to create packages per construct.  don't want new namespaces all the time.

   Option:  Maybe everything can be rolled into class?   
### --- SUBSCRIPTION ADOPTION ---
Issue: getting market payments to work
- EndProviders cannot make more money by circumventing
- EndUsers cannot get all value w/o joining

Strategy:  Embrace and Extend

Strategy:  
- **BUCKETS** -- Once you have access to one (or two) item in a bucket, all other items are free.  This way you maximize value give to user, but dont loose revenue (since they would not buy more equivelant items anyway)
- **TIERS** -- Items are provided in tiers, first tier is often free, then advancing from there.
- **TIME** -- Purchase item for uses forever (perpetual fallback licence), but gaining updates requires subscription.
- **DRIVER/PROVIDER** -- The item that causes a buyer to upgrade into a new bucket, new tier, or new time range is called the "DRIVER".   The items that are subsequently used are called the PROVIDER(S).  First-layer value is split betweent the providers (70%) and driver (30%).
- WINDFALL -- Maximizing value transferred is not the same a maximizing profit.  Once payments are large enought to clearly sustain continued work 

Revenue Splitting:
INCOME
- FEES -- Fixed and variable fees off the top
- FIRST LAYER 
	- DRIVER
	- PROVIDERS
- OTHER LAYERS

Single Item Revenue
- AUTHOR - 
- RESIDUAL - Funding for inspriational sources
### Surprising parsing insight
- VERY FEW PARSE FORMS -- this does not seem to be the case, we have hundreds or tens of thousands parse format they are vary greatly.  We have many dozens of very programmable parsing paradigms and the grammars we use are quite incompatible between most any two such language.  so what do we mean?
	- FEW METAPHORS -- MOST ALL PARSER ARE build from exceedingly few methaphors.  perhaps no surprising since 
		(1) the same visio-neural circuitry are use for parsing them
		(2) they need to be quick to comprehend, thus need to be simple
		(3) only so many simple methphors which are qualitiativly distinct and usefully different
	- WHAT ARE THEY?
		- PARSE INTEVALS
		- TOKENIZATION
		- BRACKET GROUPING
		- PRECIDENCE GROUING
		- SEQUENCING
	- SURPRISINGLY CONSERVED MACRO STRUCTURE -- 
		
- CAN BE UNIFIED -- with extreme difficulty, and only in an 80/20 way, they can be unified 
	- CODE MARKDOWN -- a language / sematnics independent way to parse code-like text into AST-like parse trees
		- SUCCESS MEANS -- for most all formal languages one can 
### parser.block quote
Ensure that "\" is retained *except* when preceeding a triple quote \"""
This allows simple backquoting source code from other languages
### Path_Format  [think now]
ISSUES:  
- how to represent "up"?  is it allowed in middle of path?
- how to escape path characters

DOT notation for UP
	..above
	.-1.above    // Parsing priority on '-' breaks this
### --- ENV / INTERP ---
#### Interp impl -- what is root 'handle' that is being maintained?
Choice: which handle is being maintained as thread switch and op calls are happening?

- ENV must be updated on each fn call
- ??? FNS need to be separated from 'self' since we can   (is true)
- No matter what FNS needs to be separated in different universes since shallow bindings cannot interfere
	this means universe can be shallow bound into each FNS differently.
- FNS can be part of an ENV, or both FNS and ENV can be local vars inside the interpreter procedure 
	in each universe.
### --- PKG ---
### >>> UNIFORM <<<
### DENOTATION [unclear]
	Should have two universes

	SPEC -- Source-code "structrual form"
	TYPE -- Instance form
	VARS -- Structure of instancdes

	Form:
	    Data with attached meaning
	    Unit in lexspace

	Lexical semantics:
	    Source forms denote some executable form


	u.lex    Code == spec(sat: defines('lex))
	u.denotes
	Form.source --> Lex.value --> Code.denotes --> Form

	Bindings should map to 

	Code -- duck type 
	Form -- denotation of code
	Binding --  Key=>Binding||Form      // parsing precedence is wrong here
### FUNCTION -- Method dispatch implementation
Issue:  How to map the uniform NS model onto other languages.
		Actions: Precomputed dynamic 'with'

Usage patterns:
-- Universe creation.  	typical meta programming at the languge level
-- Class Templates.    	overrides to define properties used in defining class
-- Runtime vars. 		override dynamic thread-specific vars during exe
-- Runtime class.		change class.  (for existing instances?)

-- data views.			cast object 


#### NEW -- Using a non-standard initializer

Choice: How to construct new units?
Conventional Option: Use conventional constructor in Uniform 
Factory Combo Option: combine contructor w. factory

CONVENTIONAL OPTION:
- well it is conventional.  nothing unexpected here.

FACTORY COMBO OPTION:
- we consider to disable (or make not the usual action) the ability to have a default implementation for any class.  Allowing default implementations is the source of meta forking where future extensions require pervasive code edits to install factories at all levels in the code.
	- Making factories be the only way to create avoid this.
	- Making factories an inherent part of each object reduces code bloat in many cases since one can simply piggyback on passed data to infer the correct facory to use.
		- This is not without some cost.  in some cases one needs to pass extra data in order to have a factory available.  Also it does require thinking about which factory parameter is the best one to use for each API.
	- .... seems worth it, even if not standard.

#### PATH_COMPARE
	Should this only do a lexographic comparison between the fixed (int) structure of the unit?
#### SHARP PATH TEXT ENCODING
	contatenative
	nice back quoting properties
#### KEY TYPESPEC
	-- units allow any uniform expression as a key
	-- lexspace does not???  (what about path encoding)
#### STREAM multi-arg
	How do we provide index and/or key and value using the READ method for a stream
	-- we have commited to merging an iterator and a reader
#### ++NAMES++ FOR FOR PARTS OF UNIT


	DATATYPES
	Nat  
	Str

	===PARTS OF UNIT===
	head	^
	body 	args 	Just the list of args
	meta 	^ 		_ _ x _ _	

	===KINDS OF KEYS===
	Fixed	Fixed positional arg
	Named	Named arg
	Meta	Meta parameter (which is not part of Args)
	Arg		Fixed+Named
	Key		The domain of a unit



	Lister
#### ++NAMES++ FOR PARTS OF A UNIT
### === SOMEDAY CHOICES ===
### AS syntax
	.as(Type) is a standard syntax for casting to a targeted type.
	.as_special_case  could be a way of doing specialized casts. e.g.  .as(Stream)  .as(Structure)
	or not....
### UN-OVERLOADING ":"
	- Use '=' to denote "pair" and parse this as infix
	- Use this for key/value pair in a unit
	- Still use ":" notation in the narrow case of  {"foo": ...}  but only then
	- Use ":" as the end of a statement and beginning of WS sensitive block
	- Could get rid of "::" since 

	related to choice of allowing mixing of positional and k/v args
	and to the '@' at end of Unit decl
### === UNSORTED CHOICES ===
### Parsing Modes
	CHOICES:
	-- Use special notation for blocks in List-Form
	-- MODES:  Yamaic, Javaic (Ceeic)  {  , Pythonic  , Codaic ::
### STATIC WITH [resolved]
	Is is allowed?  what does it mean?
	?? handle dyn and lex sub-env
CHOICE:  yes it is allowed.  it is not static, but rather it is dynamic within the universe that computed this universe.
### IMMUTABLE LEXSPACE

	--  'sets' method put onto UnitListy
	--  Form is immutable
	??  some way to define memoized fn in space
	??  different levels of immutabilty all works	
### BEST INTERMEDIATE FORM QUESTION
	QUESTION:
	- In the beginning I planned to simply dynamically create fixed structure
	  javascript prototypes and let the V8 compiler optomize over those, and/or
	- hack a c-compiler into my eval loop, so that I have a JIT step of c-compiling when
	  a structure is more constant.
	- One important usage context is the browser (hence looking at V8)

>   >>>  Is there a better intermediate form? <<<
### BEST IN-MEMORY REPRESENTATION
	- in memory atoms are encoded using native integers, floats, strings
	- units with fixed structure (like some stack frames) can be encoded in more performant ways
	  frame_... methods:  expand(code)  get/set/size/items  slice/set_slice/length
	  (these can be defined for a group of maps, and then JIT-style compilation may
	   be applied to translate map operations into c-style pointer operations)

	  specifically supported:
	    translation of fixed string/ints into fixed integer frame offsets
	    fixed multi-step pointer operations supporting segmented implementation objects
	    arbitrary code for rewriting key lookups

	- idea is any maps with known schemas can get translated as c-structs & c-pointer arrays and
	  can be accessed that way.
### QUESTION: ALLOW HEAD TO BE A REF NOT JUST A SYM?
	??? Idea would be to normalize units to have a head that unabiguously points to a lex in lexspace
### EMBEDDING JS
	``x int, y int -> x*x + y*y``_   jsfn
### BODY INDICATOR SYNTAX
	SOLUTION
	-- standard body parsing
	   -- The ':' indicates a key value pair
	-- block body parsing (inside double colon form)
	   -- Either ':' or '{' indicates a statement ending block
	   -- The ';' also indicates a statement termination
	needed
	   else:   body...
	   foo:    key:value


	key_mode (default mode)


	:
	    x:4
	    lst:    1,2,3,
	            4,5,6
	    foo::               <-- means BLK "body"
	            if x<5: print 1
	            else:   print 2
	    foo {               <-- means BLK "body"
	            else:   print 2
	    }


	===========
### BODY as:  blk()   __body__   or


	obj()
	--  AMBIGUITY   some ambiguity in whether an arg is a body form
	--  DIRTY ARGS  the body becomes a fixed arg  (sometimes it is out of order in LIST form but most body forms dont have keys?)
	++  FORM-ARG    'if' can have blk as second argument so it works by default when BODY is last arg


	BLK() in last arg position
	--  standard JSON dicts wont have a BLK head
	    -- could always use '{"...":...' forms for JSON.
	++  unambiguous indicator



	__body__
	--  looks bulky in list form in JSON    ["`if", ["`<", x, 5],  "__", "__body__", ["`", ["`print", "Is Five"]]]
	                                        ["`if", ["`<", x, 5], ["`", ["`print", "Is Five"]]]
	                                        ["`if", ["`<", x, 5], "__", "__body__", ["`print", "Is Five"]]
	                                        ["`if", ["`<", x, 5], ["`print", "Is Five"]]
	++  unambiguous indicator
### INSTANTIATION
	-- Want to have different name for singleton and for Class.
	-- Want to allow for 
### QUOTES
	how do we quote a symbol (so it does not eval to a variable?
### CHAINING OF SET
	Should we allow chaining for .set?

	NO.  This will limit the places where we can easily xlate uniform into

	Unit.new(...).set('x,10).set('y,20)
### META KEY USAGE -- Where should we be using meta keys?

	YES -- as meta_keys when named keys are expressing an open-ended space of names.
	YES -- special instance variables, sine regular named are open-ended
	YES -- global values in the execution environment  (since open ended)
	YES -- Programmable overridable "backend" unit operators that other operators depend upon
	??? -- "backend" operators for unit

	NO  -- other global operators on unit  (also open ended, but programmer can avoid & meaning is fixed)
###  PATH PRINT FORM
	string for all paths
###  SYNTAX FOR JAVASCRIPT

	::MyClass

	    foo:


	::jsclass   # Idea -- every colon form defines a string region contained by that key

	    aMethod:  (x) =>


	"long string is parsed" -->

	["`::", "`jsclass", "__",
	    "aMethod",
	    "'(x) => x*x;"



	==========
### SHORT COLON FORM


	colon:
	  - inside brace it operates a JSON
	  - inside double colon it operates as a block indicator


	LET FORM    One symbol key



	LET:
	    x=1,y=2,



	let [x=1,
	     y=2]:
	    print x
	    print y


	LET([x=1,
	     y=2],
	
	    print x;
	    print y)

	TRY {
	    x=1;
	    y=2;
	    print x;
	    print y;
	    IOError:
	        print "zot"
	    finally:
	        do this
	}

	  ::
	    if x<5:  print this
	    else:    print that  
### ISSUE -- NONE-AS-VALUE-IN-UNIT   
	--  ideally  {x:null} should not be the same as {}
	--  love to use '||' to compute default values, but only works if None==use-default

	-- Having both None and null is a real mess:
	   -- confusing;
	soln:  get(x) return None if x not in map --OR-- if x-->None
	       use get(x, default) to check if x-->None
	== limitation:   Never use None, always use 'und'
	-- Must import und  (with value set to None)
	-- create special und object with method __nonzero__ returning 0
	-- in python one must import 'und' == None


	PYTHON        x.get

	JAVASCRIPT    x.key || default()                            //  Assumes key != 0 or null
	              undefined!==x.key ? x.key : default()         //
	              (__t1=x.key)!==undefined ? __t1 : default()   //  Assumes value != undefined
	              x.not_undef('key')  ||


	C++           {
### ????????????

	-- building a homo-iconic langauge optomized for:
	   -- easy embedding into other lanaguages:
	      (1) at the source level where lex code embedded does not look like shit!
	          ==>  thus must stay inside langauge constucts that 'fit' nicely
	      (2) that are performant
	          ==>  strong-weak typing where performace is primarlily expected in case taht types are given
	               and context is pinned enought that lots of macro-expansion jit-astic goodness is possible
	   -- that looks super nice

	   -- optomized for creation of specialized sub-langauges.  indeed the lex view is everying is
	      a specializedx langauge


	-- fast algorithms for determining class membership within single and multi-inheritance hieararchies.

	-- minimial lanagues for expressing code for fast jit experience


	-- existing


	--  permissions model based on access to an object's pointer
### alpha.root is a Lex or a Form   [Form]

	Pro FORM
	-- calling path_get on Lex root might need to be creating lots of lex objects, to be garabaged collected.  but forms already exists.

	Pro LEX
	-- might not need UP pointers in Form objects
### Predicates vs Matchers   [Predicates]

	DECISION:  Use self boolean fn as predicates.
	then 'spec' must be indicated by a spec(...) wrapper.

	BACKGROUND
	- Simplest form for a predicate is boolean fn w. only self as arg.
	- Template like RegEx etc. seems like it should be a constant.  in any case it is unclear if eval should mean match or fill
	- When evaluating an and/or expression one may not know if it is a matcher or boolean expr (unless we require such expressions to be labelled)
	- head("foo") is a matcher.  head()  is an accessor.
### Metasyntastic -- lotta syntactic goodness
	     Pure syntax -- mapping from text to data-structure is semantics free
	            (parse&print do not depend knowlege of language terms -- req for homoiconic)
	
	     Aesthetically Covering -- contains <q>complete-ish’’ set of syntactic variants such that for any specialized language, L, there exists a Uniform sublanguage L’ that is semantically equivelant to L, and having the same astetic attributes and comprable <q>beauty’’


	CHOICES:
	-- Use special notation for blocks in List-Form
	-- MODES:  Yamaic, Javaic (Ceeic)  {  , Pythonic  , Codaic ::


	Foo:
	    x=zippy
	    Y

	Bar::
	    If x<20:

	Bat:
	    this= is a test


	fn(x)::
	    If x<4:

	::  Pythonic -- statement mode -- “:” means sub-block;  “=” means k/v
	.   Yamiac -- Dot mode.   -- “:” and “=” mean k/v
	{   javaic -- brace mode  -- “:” and “=” mean

	fn():: let x=5, y=4: x*y





	Foo.
	    Bar:
	    Sub_bar]
	         X: 4
	    Bat
	    Next_level


	Body Mode
	    [“]...[”][:]  or “[“...”]”  means JSON
	    Symbol means enqueue qualifier prefix
	    “.” means  sub mode   ( “.” sym means headed form)
	    :: means statement mode
	    {  means javaic mode
	    :  means k/v or sub stmt in stmt mode
### SEQ(public static final int fn(x) {foo; bar;})

	cc
### PREAMBLE TEXT -- THE CHOICES DOCUMENT

	This "Choices" document was initiated late in the design of the core of
	Uniform and Lex.  So much of the initial thinking behind these choices
	are not documented.  

	Also these languages were grown as much as possible "as a single crystal"
	of thought.  This means that it is the furthest possible point from "waterfall" design and execution.

	This means there is not linearity to the thinking about the design choices.  Every choice connect
	is dozens of ways to other choices, so my creation of this document is quite haphazard.  As choices
	come to the fore, I am trying to write them somewhere reasonable within this document, but there
	is no real attempt to linearize or rationalize this process -- it is all over the map.

	apoligizes in advance to any hapless reader passing this point!
### Units
### UNDEFINED != JSON null
	-- Assuming we ensure that:
	      UND is the result of:   {}.get('b')   and
	      {'a':null} != {}
	   then UND != null   (e.g. the undefined value cannot be the same as the JSON null value)
### ASSIGNING UND TO A UNIT IS LIKE DELETE
	-- Assuming we ensure:
	    UND is the result of:   {}.get('b')
	   then {'a':1}.set('a',UND) == {}
### EXTENDS vs IMPORT
	-- right now we use 'import' as the generic, but we could
	-- use 'extends' and also call the 'add' operator 'extends' in order to unify the two notions.
	-- BUT import is generic and is the dual of export which is nice
	-- 'extends' also feels like it is implying more semantics somehow that just import.
### UniCore Implementation
### BASE METHODS ALL CAPS
	-- Use ALL CAPS for six special unit types:  STR, SYM, INT, NUM, LST, OBJ
	-- Use ALL CAPS for ten implementing ops:    EXE, BLK, BRA, ITR, INV, LET, EXT, DYN, NEW, RUN
### Merging Stack and Heap allocation

	These can be (And should be?) merged into a single ALLOC operator,
	-- but the rules for finding the declared variables will be different, and
	-- the rules about the extent of the data are different
	-- and one has the complexity of not being a simple function, but rather must include other computation DURING its life.

	if we choose a lower level ALLOC, it should probably be a linear list of typed data onto which these 'primitives'
	map their allocated variables.  Still it feels natural to NOT smash these disparate form into a common form, for
	meta programming.
### Merging thread creation with heap allocation

	In large part, thread creation is just a heap allocation, along with a magic creation of an entry into the computation
	scheduler.  One could cleave these parts and create a more primitive operator that "breathes life" into
	a heap allocated thread object which was created via a NEW operation.

	Little down side to this, except that:
	-- it is possible that not all error checking could always be done in the THREAD new operator.
	-- its a little weird
	-- it replaces one primitive with another slighty more decomplected version, but with little practical gain in
	   code writing/manipulation simplicity.  Indeed introducing potential separation of thread creation from 'life breathing'
	   would probably add complexity to meta programming over this construct.
### CHAINING is not required for a simplest encoding of imparative computation.

	The CHN operator can (does) macro expand into a combination of LET, resolve, and EXE operations.
	Further the expansion is relatively simple in its construction, making it possible to automatically
	recognize and operate on chaining structures in their expanded form.

	In the end, it was left in, simple because chained method application is directly supported is SO many
	modern imparative languages is seems more natural to overlook is minor lack of simplicity.

	Sue me.
### Multi-threaded synchronization

	Lexcore provides the potential to create multiple threads, but it is a bit of head fake, since it provides
	no ability to synchrnize those threads, and no defined semantics which computations overlap.

	Seems like such could be elegantly added, but full integration of even simple primitive here could radically increase
	complexity of mapping onto all target environments, and in some cases (like javascript) might not even be possible.

	Future work!
### EXIT -- non-local transfer
	-- As usual we want it all:
	   -- a single uniform mechanism to handle all non-local control
	   -- allows DIRECT spec of alt behavior that over-rides (or modulates) all other specs
	   -- allows LEXICAL spec of alt behavior that over-rides (or modulates) dynamic specs
	   -- allows DYNAMIC spec of alt behavior that over-rides earlier dynamic specs
	   -- that has minimal performance hit for simple cases
	      -- ZERO performance hit when alt-spec is a 'simple' case as determined by direct/lexical, or JIT-dynamic-constant spec
	         (JIT-dynamic-constant means JIT compilation has determined that the dynamic value is a simple constant for typical runs)
	      -- A case is 'simple' if the exit spec is:  force/continue/break/return
	-- needs to fit ontop of c-longjmp and other language catch-throw mechanism
	-- should not heap allocate in simple cases:
	   -- local actions:  force, continue, break, and default
	   -- also supports simple non-local actions:  return
	   -- or the case that the controlling ref specifies a simple action:  force/continue/break/default
	-- want to capture and provide rich info when a non-simple "escaping" exit is occuring.


	SOLN
	-- macros that accept an exitspec, do not execute that spec unless the 'other' case happens
	   -- they expand into a function which is passed a symbol:  force,continue,break, or exit
	   -- their expansion include an "exit_wrapper" a macro that returns the result of its first argument
	      unless it returns "exit"  in that case its second argument the exitspec is executed
	   -- EXAMPLES
	        result = chk_return(tree_update(t1, t2, return))
	        result = chk_exit(tree_update(t1, t2, exit), io_err('return val is err msg', tag, special:'info', ...))

	-- EXIT special form:  EXT(return_val, exit_tag, head:)
	   -- checks if arg1 is true, if so, arg0 is executed, and the result is returned from the most recent fn-call
	   -- Otherwise an exit form object is created by evaluating all args of the exit
	   -- executes arg0 and arg1



	EXIT EXAMPLES

	lexical bindings ultimately control behavior, so one can rebind an exit ref in order to lexically redefine behavior
	So these would override any dynamic bindings since local code would ignore them
	    exit.get = break
	    exit.set = force
	    exit.get = exit.special_access_1        # Here a different controlling lex is installed for this lexical scope

	after this dynamic bindings would control






	* misc
### Class / Instance hoo-ha

	- code is written at the class level, but applies to a single instance of the class.
	- in some cases code applies directly at the code level before any particular instance has been created.
	  this forces two ways of operating, with much extra complexity, and some aspects are not perfectly aligned
	  which causes complexity.  blech!
	- in some case one desires the semantics of an instance, yet code is only correct if there is one instance
	  (a singleton).  the needless complexity!


	SOLN:
	- The multi-verse model:
	  (1) Each execution represents: a "point of view", a context for action, a thread defining both space and time.
	  (2) Thus according to a single execution, there is exactly one universe, rooted at __lexroot__
	      and all that is ever knowable according to that point of view is under that root, and time
	      flow is marked by "free" or independent changes in the values of that lexspace.
	      (dependent/functional changes) occur instantaneously at the time that its underlying independent input change.

	  (3) Thus the entire computing environment is one big mother of all singleton instances.
	  (4) By default, in lex every sub-place within a place is part of the same ''instance'' object.
	  (5) The ALLOC operator perforates lexspace at a single <q>defining_lex</q>.
	      -- The perforation creates a new
	      -- A new FRAME is created to capture  for this object
	      -- Each VAR entry immediately below this lex is varabalized with a new instance storage place
	         created associated with the allocated <q>object</q>
	   and provides an attachment point where 'instance'
	      can be hung.

	- There is no class per se.  There is mearly a defined attachment point with a singleton still hanging there
	  over top of that singleton 'instances' can be laid down
### frame_alloc auto calls __init__

	- if yes, then can ensure invarients created by __init__ always apply
	- if no, can easily circumvent __init__ when desired.

	COULD:   .frame_alloc(lex, __run_init__:false)     meh.
### export vs. public

	x
### Allow 'def' forms

	- most languages (even static ones) have some kind of 'define' form.
	- in lex we avoid having two representations of the same thing.  we strive for one.
	  this means in memory we do not want the source code for a 'def' form to be in a different place than
	  the data generated by tht define form.

	COMROMISE: could try to create an alternate reresentation of a key/value pairing that looks like a 'def' form

	- just have class def make clear the k/v association happening.  (every language has its own class boilerplate
	  anyway.  this one seems more parsimoionus and obvious than most?
### K,V
	 Optional arguments should be AFTER required args.
	 Natural ordering is key,value
	 Natural optional arg is key
### primitiveS -- BACKEND IMPLEMENTATIONS FOR THINGS

	--  optomized for backend implementation NOT end user usabilty
	    --> similar functions provided on top and bottom need to be distinct so thier APIs can vary indepdentendly
	    --> should be in same namespace -- simplicity of the lex universe
	    --> should be obvious which is which.
	    --> NOTES:
	        there are few relatively few implementation tokens in LEX
	        the only "users" of these will be the gear heads that do this level of implementing
	        When those gear heads are doing this spcialized activity, they will KNOW they are doing it.
	    ==> Therefore there is ADVANTAGE to using distinctive (slightly akward) syntax to distinguish these interface & type of coding
	    ==> Therefore
	        __var               for special (dynamic vars) used in implementing lex
	        __key: value        for specialized keys
	        __head(...)         for implementation datatypes
	        __                  special key representing 'head'.  consistent w. idea that two '_' is special.
	                            looks ok inside raw JSON:  {'__':'head', 'key1':'value1'}
	        xxx(...)            three letter types for 6 intrinsic datatypes
	        xxx(...)            three letter codes for 7 intrinsic machine instructions
	        frame_xxx           explicit prefix for instrinsic structural operators (avoids recursion conflict with
	                            the implemntation of the structural operators of the frame object itself
	
	
	
	        - Double underscores avoid collision with single underscore (commonly used in programming today)
	        - Single underscore stays within legal identifier space for most languages allowing for
	          direct embedding of lex in other langauges
	        - considered use of '__' at beginning & end just a python does, but not so pretty when used
	          as a keyword, and I hate 4 chars of boilerplate, but when used as a variable, python approach is more
	          beautiful.  (but as dynamic var they will occur less often as compared w. head or key
### LEX-VALUE CONFUSION
	-- having a lex, or the value of a lex should operate simiarly
	   -- doing get on either should result in parallel results
	-- EXE must accept LEX
	-- EXE *may* return LEX or UNIFORM <<<<<<<<<<<<<<
### NULL/UNDEF
	-- child 'a' and 'b' must be different for JSON {'a':null}
	-- JSON {} and JSON null must be different
	-- it is weird if JSON-null is not the value returned by functions that intend to signal 'no-result'   e.g. index_of(item, list)
	-- do we map undef and null both to false?
	-- ANSWER?   all of the following mean 'not-satisfied'   undef, False, null, {}, 0, ''   (maybe not the last two)
### TO DO: support printing single line prefix of full string value   (what does this mean?)
	z
### GC ISSUE
	-- when a lex is invalidated it may not be garbage collected, thus a second lex with the same address could be created.===
### ROOT ACCESS
	-- do we need a way to 'resolve' to LS root?
### UNIFORM FUNCTIONS

	ADD x + y
	    -- concat printed forms if any value is a str or sym
	    -- add if all forms are numbers
	    -- else
	        - meta keys are ignored
	        - concat fixed keys
	        - update y values overtop x values
### Parsing modes

	::  Pythonic -- statement mode -- “:” means sub-block;  “=” means k/v
	.   Yamiac -- Dot mode.   -- “:” and “=” mean k/v
	{   javaic -- brace mode  -- “:” and “=” mean
	l
### === NON-CHOICES ===
### NAMING IDEAS
	NAMES    pos_arg_items   named_arg_items   farg karg marg  fixed_items  kw_items  meta_items
	    - keys, values
	    - fixed / positional
	    - keyword / named / parameter / kwargs / kw
	    - meta

	NAMES  children, children_fixed, children_kw, children_meta, children_all    (default iterator is 'children')
	NAMES  keys      keys_fixed,     keys_kw,     keys_meta,     keys_all
	NAMES  values,   values_fixed,   values_kw,   values_meta,   values_all
	NAMES  items,    items_fixed,    items_kw,    items_meta,    items_all

	NAMES  all( fixed|kw|meta, children|keys|values|items )
	       for k,v in fixed keys of lex
	       for i in range(low, high)
	NAMES  lex.loop_fixed_keys(fn)
	NAMES  m/a regular :{value} expression/
	       pat("a regular : expression with a colon \:", value)

	NAMES  pcall, bcall, scall   pform, bform, sform

	- Lex points to strongly-typed structure
	  - map of accessors describe access.  (sub maps for sub access)
### === CHOICES MADE ===
### ??? DATA.VALUE - Representing data eles w. a special non-structural value
Background:
It is sometimes valuable to have a specially designated value associated with data elements which is not part of the structure of that element, but is in some relation to the data element as expressed by convention.  

Goal:
Adopt a uniform paradigm for expressing such data so that syntactic notion might arise to uniformly handle the case, and so programmers would know without checking where to look for this information, since it was in the "standard" spot.

OPTION: A a meta key to encode such an implicitly associated meta content.  Use helper methods to access this content.

OPTION: Don't do this, but instead have small number of semantic accessors that get reused across classes (e.g. 'path')

CHOICE:
- Do we allow computed values for this special key
	==> yes, but only in classes that override the default 'value' lookup fn
		e.g. some places interpret the ^value meta key as constant while others do not, while others don't even use the ^value meta key

ALTERNATIVES:
- Use the 'value' meta key

EXAMPLES:
- The value of an identifier is its addressing path.
- The value of file is its addressing path.
- 

_
### >NAMES / >WORDS
### Single Letter Associations

a
b = ?Bag?
c = Child==>Dependent/Element,  
    (Collection/Container/Composite ==> aggregation/bag/bundle/group/set/list), 
d
e = ?Element?  ?Enumeration?
f = File
g
h
i = Integer
j = Another Integer
k
l
m
n = Number?
o
p = Parent
q
r
s = String, SELF
t
u = Unit
v = Value  (Other Unit)
w
x
y
z
### Potentially useful three and four letter words
RAN RUN NUB BOT GEM RAW LAW EYE FIT FOE FOR
USE  VALUE HINT CLUE CUE KEY

DENOTATION, denotes DEFS, POINT, MEAT, USE, GIST, PITH, SOUL, CRUX, CORE, CENTER, GLOSS
FORM, CAST, TYPE, 

// File.contents Pkg.contents
contents out  value 

ABLE, AMIN, AWED
BEST, BRIO
CALM, CARE, COOL, CUTE
DOPE, DUTY
EASE, EASY
FAIR, FAME, FINE, FLOW, FOOD, FREE
GIFT, GIVE, GLAD, GLOW, GOOD, GROW
HALO, HELP, HERO, HOLY, HOPE, HUGE
IDEA
JOKE, JUST
KEEN, KIND, KISS
LIFE, LIKE, LIVE, LOVE, LUCK
MANY, MILD, MORE
NEAT, NICE
OPEN
PLAY
PURE
REAL, REST, RIPE
SAFE, SAVE, SEXY, SOUL, SWAG
TACT, TEAM, TIME, TRUE
VERY
WARM, WELL, WILL, WISE
YEAH
ZANY, ZEAL, ZEST, ZING
### ??? UNICORE PRIMATIVE NAMES



best   EXEC    BLK     COND    LOOP    CALL    LET     DYN,    EXIT    NEW     NAT     RUN
3char  EXE,    BLK,    BRA,    ITR,    INV,    LET     DYN,    EXT,    NEW     NAT     RUN	  PKG
       .init()                                         TRY     ALT     .frame_new

long   EXECUTE BLOCK   SWITCH  ITERATE CHAIN   LOCALS  CATCH   EXIT    CREATE  NATIVE  START  PACKAGE
any    EXECUTE BLOCK   COND    LOOP    CHAIN   LEX     CATCH   EXIT    CREATE  NATIVE  START
4char  EXEC    BLOC,   COND,   LOOP,   CALL,   LET     GRAB,   EXIT,   MAKE    NAT     RUN
2char  BL,     CH,     BR,     DO,     CA,     LT      ST,     EX,     MK      NT,     ST
### Language model names

Surface Form (text and data)
Code Form
Link Form     (denotational form; )
Env Form.  (form in execution)
### >DOT >PATH NORMALIZATION
Context:  aaa.bbb.ccc parse to "."("aaa", "bbb", "ccc")
Decision:  "."(...)  normalizes to dot(...)

Background:
-- Most infix operators parse to "XXX"(...)   where XXX is the punctuation mark.
-- Most do not normalize to any other value.
-- yet dot(...) is a very core data form in the Uniform language.

DOT Option:
++ it is slightly uncomfortable to have a non-alpha identifier head for units when explicitly programming over the AST.

"+" OPTION:
++ having a normalization for '.' but not for the many other infix operators is a kind of inconsistency.
### LEFT-TO-RIGHT PARSING AT THE SAME LEVEL OF PRECIDENCE [needs parens]

-- python community is right to hate:  x = y += 5
   but x = (y += 5) is alot clearer.


-- I really want combine units to be a puctuation based operator.
   it is so prevalant.

Solution:  allow left right parsing at the same precident level, but only for n-ary versions
of a single operator.  mixing operators even at the same precedence level requires parens.

So
    x += y                is  x.update(y)
    x = y + z             is  assign(x, add(y, z))
    x = (y.copy += z)     is  assign(x, y.copy().update(z))



* Tiny Choices

__BINDING__ maps to a LEX or the VALUE in a lex?
   needs to map to a LEX, so you can do an assignment to it
### POSITIONAL KEY TYPE
-- are the keys of lextree ^1 or int 1  ?????

Decision: Use string & int to access
### === FOR THE RECORD ===
### =OBJECT EVAL -- must obj eval to themselves [resolved]

OPTION 1 - yes.

OPTION 2 - No.
-- String templates as a first case would be nice to allow them to auto expand in code when executed, but also allow them to be objects passed around (unevaluated) and have other operators (like 'match') apply to them as well

CHOICE:  make EXE be an operator provided by all units, this allows string templates to be non-constant while others default using an adaptor to being constant.
### =PRIMATIVES -- Including VAR, CTX, PKG in unicore
Background: These operators are definable from structural and control operators, so they could be derived.
Option 1: derive them
Options 2: Include them
++ Their semantics are essential for transpiling code between interpretational contexts.
	==> seems code expressed in them is embeddable, but derived code would not be embeddable.
### =TYPING -- Decomplect Head from Type

Choice:  How are interpretation semantics indexed?

Option 1:  Using the 'type' of a unit, making function depend upon structure


Option 2:  Using a separate 'head' indicator that may or may not align with underlying type
- More cleanly separates structure from function.
	- many language take the shortcut of making function depend upon structure since it is challenging to ensure correct interpretation of function w/o making assumptions about structure.
	- other langauges (duck typing languages) provide zero guarantees about functioning, only a best effort execution based on found structure
	- Decomplecting these allows uniform to get the best of both.  In cases where static typing is desired, then head is a function of type, thus ensuring the desired dependency.  In other cases head is freely re-assignable, thus manipulating semantic interpretation orthogonally to underlying structure.


_
# ### MATH ###
# ### DATA - UF STRUCTURE ###
### --- IDENT / SYMBOL ---
#### -- Ident Head --

CHOICE: what is the head of an ident?  itself?  the 'Ident' gnd?

PRO: head = self
- HEAD represents a semantics category.
	If head of code expression is not its container class, but instead is the semantic category of that expr, then the head of an ident should not be the Ident class but
_
#### -- Ident Datatype --
**RESOLVE**:
- Ident is a subclass of Lex contained in its own lexspace.
- Ident is losslessly 1-to-1 mappable onto full space of strings
- Standard form unit form maps all unit heads to Ident (strings, and ident)
	while extended form allows for non-Ident heads


_
#### IDENT PART

Opt: IdentPart might itself be a "simple" Ident -- an ident of length one.
Benefit:  This erases the complexity of the head of a unit being one data type and the keys for a map being another.
Con: Certain implementation substrates significantly benefit from having IdentPart be a simple string.  (e.g. Python )

_
#### DATA.IDENT.REP -- How to represent identifiers in unit data
CHOICE: Most general representation of ident in unit data?
PATH: AS UNIT HEAD -- Use empty unit with headstring
- maybe have meta key indicating this is an indent
PATH: UNIT W META KEY -- 
_
# ### UNIT ###
## === UNIT - semantics ===
### --- EQUALITY ---
RESOLVE:  x.equals(y)  iff x.hash() == y.hash()
RESOLVE:  Code(x) != Code(y)  then  x != y

CHOICE:	  Does equality test more than just structure equivelance?

OPTION YES:
- Allows objects with internal state to test and not equal

_
## === UNIT.GRAPH ===
### --- UNIT.GRAPH - LISTY related ---
#### -- Choices --

CHOICE: Does listy mean all keys are natural numbers?  values are dense [0, n] 			or nat keys behave correctly?

#### -- Listy related unit invarients

1. CODE EMBEDDING


CODE EMBEDDING INVARIENCE


#### -- RESOLVE meaning of 'listy edge'

RESOLVE: The nth edge <k, v> is _**listy**_ iff k == n


DISCUSSION

ISSUE: Which integer value indicates that a k/v pair is a listy pair?
CHOICE #1: Could enumerate listy pairs starting at 0 with interspersed k/v pairs
CHOICE #1a: As with #1 but require that other k/v pairs are not integers, if they are, then they are upgraded to a listy pair with a Pair as their value
CHOICE #1b: skip over k values that are saved as a non listy pair.
CHOICE #2: A k/v pair is listy iff it is the nth pair and k=n


RESOLVE: Use option #2.

NOTE: For along time we were using option #1 since we were using a non-8bit clean structure with meta keys embedded into the structure sequence, and having the head key offsetting positional argument indicies seems a bad choice.
Once these were cleaved, option #2 became a clear winner. (I think)



CHOICE #2 PROS:
- Can determine if a k/v pair is a listy pair in isolation w/o considering rest of list
- Can append a listy pair to an arbitrary unit w/o further knowlege except its length
- Can perform set operation  U[x] = y
CON:
- The indicies of listy pairs will not start at zero and will not be contiguous if not listy pairs are added.

_
### --- UNIT.representation - 8-bit clean; representation of keys and head

RESOLVE:  keys, values, head, and meta keys are all "8-bit clean"

see unit-form for default representation of keys and head

_
### --- UNIT.representation - Uniform data rep & element indexing & list rep


RESOLVE:
- **USE UIF** (Uniform Iterator Form) is the unifying base rep for unit data
	All unit data can be losslessly be translated in and out of UIF
- **USE PLCEMENT** -- Adopt implied indexing as placement


ISSUE: how to refer to a single element within a structure?
- key, itr_index, or neither might be the right answer
ISSUE: what can be derived when holding only a reference to a single element?

LEX-LIKE REFERENCE:  
- Ability to "climb out" of element with some kind of path to get back there
- What is derivable having only a reference to the element itself?
	Value, Key (explicit or implied), Index ()

BEDROCK:
- If an input is functional; its UIF must preserve those key-value relations
- If an input is listy then its indexing operation 

**IMPLIED INDEXING** -- Implied indexing ensure that every child of a unit has a unique key that can be used by the GET operator in order to return that key.


IMPLEMENTATION CONSIDERATIONS

CODE UNIT IMPLEMENTATION
- list of pairs
- memoized the placement mapping


LEXSPACE IMPLEMENTATION
- LEX: key, value, index, up, origin, down


_
### --- UNIT.LIST - listy_idx semantics


_
### --- UNIT.LIST - how are lists embedded into k/v pairs?
CHOICE #1: Lists are maps with dense integer indicies
CHOICE #2: Lists are unkeyed, and use iterator position to determine index

PRO #1:
- >>>  lists encode as maps naturally (since each value has a unique key)
- >>>  the 'get' operator is naturally uniform since u[x] 
- --- Merging lists gets weird since indicies shift
- know where an list item came from via its index and parent

PRO #2:
- Ordered maps and orders lists are both uniformly ordered by their iterator
- Can use common 'item' when sharing sturucture between objects
- can manipulate (e.g. slice, etc.) w/o doing math gymnastics on indicies


RESOLVE: 
- All unit data can be mapped into UUF losslessly
- Specialty structures can be more efficient in space and time
	The existence of UUF does not impact their performance



UUAF - UNIFORM UNIT ACCESS FORM --

UUF == a sequence of edges, with the first edge being the head meta key
EDGE ==  EDGE(key, value, enum)


UUTF - UNIFORM UNIT TRANSPORT FORM --


element_value
`:(key, value)
`:(key, value, enum)

_
## === UNIT.OPS ===
### --- UNIT.HAS - Does unit have a 'has' operator at lowest level???

also kind of like 'find' or search for pair with given value

_
### --- UNIT.META - Meta key meta data representation & transport

ISSUE: Handling meta data in all forms when unit data must remain 8-bit clean

RESOLVE:
- Create special class of units called meta keys to use in encoding meta data
- A _meta key_ is a unit whose head is 'META' and whose first position element is a string containing a valid identifier.
_
### >DATA - base unit's iterator
Choice:  What generality do we allow in our basic unit form.  choice=#2
Option #1:  head(fixed, var, key+meta)   this is the 'standard' form for python like fn calls
Option #2:  Allow arbitary maps:   head(entry1, entry2, ...)   where entry is a fixed arg or key arg, 
	or pair of complex structures.


Option2 generates many headaches, but solve many more headaches.
This most general form allows variables and declarations to be simplified.
Allows more general code forms to map w/o issue.
Also allows generalized maps to fit into 
### >DATA - representing unordered bag
issue: UxU->U does not naturally represent a bag w/o order.

Solution: provide an arbitrary ordering (0, 1, ...)
think of this as the ordering of the ITR as it traverses the bag.
Note this ordering need not be stable.
### >DATA / RDF
Choice: What underlying mathematical model to select for data.
Option #1: RDF triples:  U x U x U    (subject predicate object)
Option #2: FN form:      U x U -> U   (subject & predicate functionally determine predicate)

Option #1 - pros
- Standardized, extremely well thought out, extensively used.
- Allows merging of arbitrary datasets with a well defined & useful combination semantics
- Allows simple expression of open knowlege bases

Option #2 - pros
- Allows uniform model of ordering.  (only thing to be ordered is predicates for a unit)
- Biases towards tree structuring of data (passing down tree repeatedly from subject to object) where groups of triples are naturally identified and managed by nodes higher in the tree.  (note: one can do that in RDF, but RDF's symetry does not bias towards such structures.)
	- We avoid the need for (and non-uniformity of) QUAD stores since this restricted framing of data biasts that each DATA fragments is explicitly "owned"  
- FN-form embeds into triple-form and lossless, bidirectional maping to triple-form too.
     (mapping:  RDF triples with common subject & prediate are replaced with single triple
	  that maps to a special 'fn-form-set' node that uses multiple triples to link to 
      each of the original object values)
  - FN-form notation facilitates a closed world assumption on the data
    RDF notation facilitates a open world assumption on the data.
    Each form can be used to express the opposite condition, but each has is default, 
    and many applications are simplified and bugs reduced when closing by default.

NOTE:  It is to be admitted, the closing-by-default model has its drawback in that the programmer may have code which can operate correctly in an open world, but they would need to explicitly provide methods for combining closed worlds to provide that benefit to their users.  

Still most data in most modern software systems are effectively close world.  This is easier for the programmer to reason about, thus we selected it as the default uniform model.
_
### STR.PATH - CHOICE - path_get create parents
ISSUE path_get create [-1] is ambiguous since index is unknown
CHOICE generate error when creating parent
ALT could allow this to be a parameter

_
## === STR.UNIT ===
### STR.UNIT.NEW - meta keys

ISSUE:  How to pass meta keys to the new operator
CHOICE #1: KWARG: meta_keys could be passed as the kwargs of the new operator
CHOICE #2: 2ND ARG: meta_keys could be passed as the second arg to the new op
CHOICE: The same choices exist for the class constructor for each unit.

PRO #1:
- Most common case of just passing a head value is simplfied

PRO #2:
- Maybe faster to avoid potentially bundling and rebundling kwargs
- There is "room" for specifying other non-meta parameters in the future.
- Perhaps less weird than scooping up listed params to a permanent structure.


RESOLVE:
- Use the 'second arg' option for class constructors.
	The space of these constructors is open-ended and includes constructors for container classes authored outside the uniform universe.  It is just too dangerous to allow arbitrary keys passed to them based on a parameterized meta key map.
- Use the 'kwarg' option for the new operator.  This affords the simplicity within the code for the most common specifying on the head, case.  And efficiencies can still be manintained for specifically developed unit classes (which should be the most common case for uniform code, and it really the only approach for gaining many other kinds of efficiencies, like compact representation of fix meta-key structures, etc. anyways.  Thus there is no loss in only gaining this particular efficiency once using specially developed unit classes.)


_
### STR.UNIT.NEW - init
how does init work?  can you circumvent?

_
### STR.UNIT.NEW - the name of the 'type' keyword
should not have a keyword called 'type' too many languages treat as reserved word.  but what to use??
_
### >STREAM 
#### READ -- Timing of position advancement
Choice: Should 'read' advance to the next step (1) BEFORE or (2) AFTER obtaining the value returned.     Choice: AFTER.

BEFORE Option:
- Means that .index() can be used to retrieve the index associated with the LAST value read.  So x=itr.read(); y=itr.index()

AFTER Option:
- When viewing stream as an iterator, is seems essential that:  itr=u.open(); x=itr.value() should obtain the first value in the sequence.  For this to be true one must select the AFTER option.
- The after option naturally provides a "peek" operator on the stream since:  val = itr.read(); peek = itr.value(), the var 'peek' will contain the value to be read in the next read operation.
### IS UP ESSENTIAL?
yes, one can define it as GET("") but doing this conflates these and it is nice to be able to declaratively express existance or absence of back links
### IS IDX ESSENTIAL?
Yes, without it one cannot define highlighted tree within data graph
### IS ITR ESSENTIAL?
Kinda.  one needs it to order subunits of a Unit, but its not clear if it needs to be a full iterator.
### IS SET ESSENTIAL? --KINDA--
kinda.  one could express SET as ASSIGN(GET) but then the structural backing level would depend upon multiple interp forms (but maybe that is fine????)
### ARE STREAM OPS all essential at the Unicore level?  --dunno--

No.  One can express the timeless "ordering" relation, then 'Stream' is an API for translating the ordering relation in structural space into the same ordering in temporal space.
# === LEX ===
### many

CHOICE: Is the struture of all lex hidden.  e.g. does  lex.get('foo') work?
RESOLVE:  No, just the structure of Ident.  ???
CHOICE: Do Ident exist as a distinct class of lex?  
RESOLVE: yes, but they are embedded into the 'main' lexspace via zip embedding

### LEX.repr
ISSUE: what does lex.repr print since no origin is definitive?

RESOLVE: Use zip(0) as the origin for repr printing

### LEX.metaverse


ISSUE:  how does ident origin relate to env origin
RESOLVE:  Each lexspace may be contained in another since its zipper roots in the lexspace value which itself may exist in a Place within another space
# ### PLACE ###
CHOICE: There is a bit of confusion here if we are placing the root unit, or the whole tree when placing.
(Seems it should just be the root unit, and any other units already "owned" by that root, unless it is recursively placed)


CHOICE:  lex or loc?  what about place?
CHOICE:  Patch.delta(before, after)  Patch.merge(a, b, before)  Patch.summarize(tree)
_
# === STR unsorted ===
# ### MD - MARKDOWN ===
### MD - Alternative data form encodings

ISSUE: What does simplified key form allow its keys to be:
CHOICE ANY: The key can be any string
CHOICE NON-^: The key can be any string not beginning with '^'
CHOICE IDENT: The key can be any  

_
### MD.PAIR - Encoding, resolving of pair/block confusion

some_block:
	else: print
_
### MD.BLOCK - SUMMARY - Unified encoding of recursive structural data

ISSUE: Satifying all sides is quite a mental game of twister

BEDROCK REQUIREMENTS:
- 8-BIT-CLEAN -- Unit data is 8-bit-clean in the sense that a generalized unit may have any head, any keys and any values, and may map between different formats without info loss.
- LOSSLESS PRINT FORM -- Print form should losslessly print and parse all such unit data.
- A list encodes as [11, 22, 33] and maps as 0->11, 1->22, 2->33
- A map encodes as {foo: 11, bar: 22} and maps as expected shown
- A function call: my_fn(11, 22, foo: 11, bar: 22) should map its arguments as list does for its fixed args and and as a map does for its key arguments.
- GET operator should index the structures as expected for lists, maps and calls.
- Mapping result should losslessly encode (and thus be able to reconstruct) arbitrary units with arbitrary values in all fields.
- As much as possible parse structure should be mapped "naturally" into unit structure, and all **LOGICAL** structure of should be retained.
	- This does generate some ambiguity regarding what should and should not be considered "logical structure"  but for many aspects you know it when you see it.
- else-unification.  "else: print" should uniformly be encoded parallel to:
	- "if x<4: print"				-- since both are valid statements
	- "el: pr" 						-- since both are key/value assignments
	- "else: print this; print that	-- since both group zero or more values
- MAPPY.  this structure, like all structures should be mapable onto lexspace.


THE TRICK:
- split the parsing of  else: print  from   if x<4: print
	use 'by-name' indexing for the first and 'by-position' indexing for the 2nd.
	THEN use BLOCK BODY FORM as a lossless tranform of unit data when ever one wishes to treat it as a sequence of source code statements.

RESOLVE:
- Use BLOCK BODY PARSING to convert into parse tree into unit form
- Use BLOCK BODY FORM transform to generate an 'else-unified' code for from a block, any time one wishes to treat a block as code.
- Define list-indicies, and toplevel separator stucture as non-logical parse detail.  Thus it is valid to loose this structural info during parsing.

_
### MD.BLOCK - Encoding and determination of code blocks
OPTION1: Use no head, and being within a block in last pos as indicator of block
OPTION2: Use being in key 'BLK' as being a block
OPTION3: Use head='BLK' or head=':' as being a block

OPTION #1
  ++  Yamaic-style section nesting is naturally seen (and indented) a blocks
  --  Not enough info to keep intended program structure ???

YAMAIC STYLE EXAMPLE   following is a map of maps with x bound to 1
foo:
  bar = {
	  
  }
	print: = 1

CONFUSION
foo:
	bar = print(111)

else:
	print

else = {print}

_
### MD.ATOMIC STRING - Markdown string prefix encoding for atomic units

    "-1234.56"			Numeric data in string form
    "+23"				Numeric data must always begin with a '+' or '-'
    "'hello there"		String value
    "`ident.value.str"	A segmented identifier
    "*some.ref.loc"		A reference to some part of lexspace
    " head.ident"
    ":key value"		A simple key value (used in list form)
    "^meta.ident"
    -1234.56			Can also use JSON number directly

_
### MD.LIST - Markdown unit-form

Compact: 	[" head", arg0, ..., ":key0", value, ..., "^metao", value, ...]
General:	[":", key0, value0, key1, value1]

Simple form head and keys are in ident form.
Values are recursive list forms or string atom forms.

Meta Keys
	^head	The head of this unit
	^lex	The location relative to the current origin of this unit
	^len	The number of k/v pairs this unit has (not counting meta keys)


LIST FORM -- A lossless unit encoding that maps directly onto JSON lists and strings

Simple Form




	List-Form

        [“`sym”, arg1, …, “__”, key1, val1, …, body]

        [“`if”, [“`<”, “`x”, 10], “___”, “`print”, “‘hello there”]

        [“`if”, [“`<”, “`x”, 10], “__”, [“`print”, “‘hello there”]]
_
### MD.STMT - Markdown Statement FORM

ALL ARE IN STMT CONTEXT

    x: Int		-->		`:(x, Int)			--> `:(x, Int)			# in not WS
    else: do	-->		`:(else, do)		--> else({do})
    print x		--> 	_(print, x)			-->	print(x)
    if x {do}	-->		`{(if(x), do)		-->	if(x, BLK(do))
    if < 5: yo	-->		:( <(if, 5) yo)		-->	if(`<(5), BLK(yo))
    f(x){zz}	-->		`{(f(x), zz)		-->	f(x, BLK(zz))
    else: foo()	-->		`:(else, foo())		-->	else(BLK(foo()))
    for x in y: z	-->	_(for x in `:(y,z))	-->	for(x, in: y, BLK(z))

    "not" stmt	-->		_("not", stmt)		--> 	-same-

defaults		head = op.head, body = op.ags
if seq, then 	pass
if block, then	head 
if seq, then body = args
if block, then body = [op.args, ..., blk]

if block then split body is just op
if pull head then split head

if '('		return
head, body = op.head, op.args
if seq		head = op.args[0]	body = op.args[1:]
	if body[-1]
if '{' or
   ':'&ws	head = pulled_head	body = [rest_arg0..., BLK(op.args[1:])]

'{' --> head, args_w_body


_
### MD.MODE - How to manage parsing modes

maybe we can begin pythonic if we can unify yamaic and pythonic parsing


2020.10.27 RESOLVE:
- Parser operates in four modes in_ws [T/F], in_code [T/F]
- ???	begins JSONIC parsing	in_sw_False	in_code=False
- '----'	begins YAMAIC parsing	in_ws=True	in_code=False
- ???	begins JAVIAC parsing	in_ws=False	in_code=True
- '::'	begins PYTHONIC parsing	in_ws=True	in_code=True
- '{'	begins JSONIC parsing 	from all modes except JAVAIC


2020.10.27 JSON inside JAVAIC

NEEDS
    PUNCTUATION HEAD:		`+(4, 4)
    PUNCTUATION SYMBOL:		[ `+, `-, `*, `/ ] 
    TYPED STRING:			/.*a+.*/re
    TYPED BRACE:			j.[]		

	`j{"foo": 777}

	if x<10 {y = /{"foo": 777}/j}


2020.10.15 RESOLVE.  (No longer accepted)
- Parser operates in four modes in_ws [T/F], in_code [T/F]
- '{[{[' begins JSONIC parsing	in_sw_False	in_code=False
- '----'	begins YAMAIC parsing	in_ws=True	in_code=False
- '{['	begins JAVIAC parsing	in_ws=False	in_code=True
- '::'	begins PYTHONIC parsing	in_ws=True	in_code=True
- '{'	begins JSONIC parsing 	from all modes except JAVAIC


TRADEOFFS
- Parses both JSON and YAMAIC w/o prefix.
	Thus the cumbersome YAMAIC and JSONIC indicators are rarely needed.


DISCUSSION
2020.10.15
- JSON -- JSON constants perfectly match UnitForm constants in both withspace sensitive and free modes.  JSON structure must begin with '[' or '{' and either one will flip parser into non-WS parsing.  Since parser begins in non-code mode, it ensure that these structures are parsed as JASONIC code which is a super set of JSON.
- YAMAIC -- It parses yamiac w/o prefix since it begins in YAMAIC mode.  
- RARELY NEED '----' or '{[{['
	The former is only needed when embedding YAML into another format,
		but commonly YAML is used at top level within source files.
	The latter is only needed to transition from JAVAIC to JSONIC
		but little need, since JAVAIC is a superset of JSONIC

2020.10.23
- NUM ONE ISSUE:  stmt form is currently applied too promiscuously see below
- Cant use '{[' for JAVAIC, since:  {if True{do}}  conficts w its close brace
- Same for '{[{[' for JSONIC

THE STMT FORM PROBLEM
- x + y  		is a very natural expression required to be supported, and 
- print -x	is a very natural statement required to be supported.
- Cannot have both in the same context!
- Currently the rhs of ':' is a statement, when the ':' occurs within a block form.  Yet we used ':' as a k/v separator in the same context.


some:
	yamish:
		key = x + y
		another_key = 
			indented string value
		


- '::'	PYTHONIC/JAVAIC		Stmt processing for ':' and '{'
- '----'	YAMAIC/JASONIC		No stmt processing for  
- '{"'	begins JSON parsing


'=' codes for PAIR with a data value
':' codes for PAIR with stmt value

example:  |<--


flow: >>>>
	this has deeper 





_
### MD.COLON - Modal colon processing

APPROACH #1 -- MODAL ':'
- All terminated brace forms () [] {} turn off whitespace sensitive parsing
	The '::' can be used to turn it back on again
- Normally ':' parses as an infix k/v pair
	In WS mode that meaning is GONE and it serves as a stmt terminator
- COLON PROCESSING IN WS MODE
	- colon is always split so any prefix is moved to (created) SEQ above
		So   _(x, `:(foo, bar))  ==>  _(x, foo, BLK(bar))
	??? foo: bar: baz: 5
 - STMT PROCESSING
	 - BLOCKS -- Statements are children of '{' and WS ':'
	 - SEQ -- Seq statements are rewritten with first element as head
		??? (even when it is not an identifier)
	 - EXPR -- Any SEQ stmt beginning w. an expr with a leading indent
		is split so ident occurs first
- RECURSIVE REWRITING PROCEDURE:
	- Keep track of: in WS, child of BLK
	1. BLK -- Create BLK by spliting prefix and adding to (created) seq above
	2. EXPR -- Pull expr head into (created) parent seq
	3. CALL -- Rewrite seq into call form

          

call:   		foo(1, 2, a: 11)
dict:   		{a: 111, b: 222}      -->  map(a=111, b=222)
block:  		blk: do_this; x=5   -->  blk(do_this(), `=(x,5))
YAML section:	foo: bar: baz: 5	-->  foo(bar(baz: 5)))
				a,b = 1,2

DONT SUPPORT??
space declare:	foo(x List Int)


MODE RULES:
- '{' and WS ':' mean block (stmt processing is only done on its children)
- BLK head can be None, if prefix so it matches JSON (but w head matches)
- NOT SPECIAL ' { " ' is a special case which parses as JSON object
- '[' and number and string already parse as JSON

- STMT processing:
  - If first element has expression level priority, and its right leaf is a valid identifier then it is "pulled out"
  - Sequence stmt converted to fn-call form
  - Arguments after the first are treated as k/v pairs for each key that is a valid identifier

- Cannot allow  a,b = c,d  if '=' becomes an expression

REASONING:
- SEQ < (,)     To allow declarations like:     foo(x: List Int, y: Str)
					Also visually seq is "less" separating than comma

_
### Typed Sublanguage 
ISSUE:  When possible we want to embed languages as uniform markdown, but that will not always work.
	When it does not, we want a very clean way to embed with an absolutely positively minimum of boilerplate
	since this may occur in the finest grain within the language

Ideas:
- INFIX CHAR:
- WITH COMMA 		date!7/14/1965  // Using space as a separator
- WITH INDENT:		java>>>			// this one can be more cumbersome since it encloses more
- WITH MULTILINE:	foo[[[ this is a mult-line ]]]
- AS BRACKET:		re`.*[A-Z] 
- Q('date, "7/14/1965")



_
### syntax.colon   COLON

Background:
COLON/EQUAL Usages
-1- python-like statement terminator
-2- key value pair specifier
-2- code assignment statement

Impt Cases:
MUST	if x==0: print "zero"
NICE	{x:5}              foo(3,4, ok: true)
CLEAR	{x:=5, y:="foo"}   foo(3,4, ok:=true, prefix:="foo")
		{x=5, y="foo"}     foo(3,4, ok=true, prefix="foo")
MUST  	x = 5


		MAP		KEY ARG				ASSIGN	BLOCK-BEGIN
NICEST	{x:5}	foo(3, ok: true) 	x=5		if x==0: print "zero"
CLEAN   {x=5}   foo(3, ok=true) 	x:=5	if x==0: print "zero"
HETRO	{x:5}	foo(3, ok: true) 	x=5		if x==0: print "zero"  	Requires 


HETRO rules
  -- '::' puts into WS mode 			':' after one alpha or numeric token it indicates key/value else sub-block
  -- ':' or '{' puts into BLOCK mode    ':' always indicates sub block

  -- Normal Mode
	     -- stmt form is multiple tokens then ':' or '{' -- body of such form is in block mode.
	     -- otherwise ':' indicates key/value pair
  -- Block mode
	     -- ':' always indicates sub-block


CLEAN COLON PARSING
	==  x==y	QUERY		ASK IS x equaled to y
	= 	x=y 	ASSERTION	x IS EQUALED to y
	:= 	x:=y 	ACTION		MAKE x equal to y
	: 	if x: 	WS-BLOCK 	terminate statement with whitespace BLOCK
	{   if x{ 	BLOCK		terminate statement with BLOCK
### PARSING -- Parsing Ambiguity of not having reserved trigger words
#### Rules
-1- Punctuation operates as prefix/unary, infix/binary or nary operators connecting string numeric or symbolic terminals
-2- Punctuations binds hightest priority to lowest and left to right within the same priority level.
-3- Bracketing, is done using opening/closing punctuation or by indention, it groups a sequence of sub-parses under the open bracket operator, or it captures the inervening characters as a simple unparsed string.
-4- Comments are brackets that are removed from the parse stream and ....
-5- Blocks are brackets that indicate their contents are parsed as potential statements
-6- Statements are
		parsed forms whose root punctionation has priority LOWER than the statement boundary
#### Statement parsing rule.

ParseStatementBody:
  while not end_of_body:		
	next = parse_expr()    # Cannot allow SEQ upgrades here, but could allows "f(x)"
		
	if next.head.priority > statement_break.priority and first_terminal_is_symbol:  
		# handle stmt start inside expr.  e.g.  if -x<5
		# Explicitly handle seq upgrade here since not handled prior
#### Terrible Cases

if -x<0 { print "it is postive" }
if (-x<0) {print}
f(x)+3

P==>Q {print("it happened");}

{1;2;x+y;3}   ==>  1, 2, x(+(y))   # must guard with parens
{1;2;(x+y),3}
### Headless Units
How does a unit w/o a head key print?  
What is returned by .head?

Choice: und Obj Unit
### VALUE PARSING
	Qualifier_symbols  dot_prefixed_symbols
	    “.” terminator   YAMAIC sub form    (only in YAMAIC mode)
	    “::” PYTHONIC mode
	    “{“ JAVAIC mode
### ???Uniform

	MAKING GOOD CHOICES FOR THE FEW TOKENS NOT PINNED DOWN BY C++:  WHICH ARE PREFIX / INFIX / SUFFIX
	-- Follow conventions used when possible.
	-- Pin most down so they are available in generic Uniform w/o syntax exntensions.
	   (Languages writers removing existing forms from Uniform is only slightly less evil than extending the language
	    best to provide expected meaning for most, maximizing chances language writers can live within conventional
	    Uniform, thus not burdening their users w. extensions at the token level.)
	
	    -- CHOICES FOR OTHER CHARS
	        $xxx ?yyy
	        -- These are super tight prefix binders so they stick with their modifying token, but they only occur in prefix
	           form, so they dont glom-on to the left, in infix contexts.  So if $x(1) then $y(2)
	           should have '$' bind tighter than '(' but should not bind to the tokens to their left.
	        -- Think of these tokens as modifying the single token to their right as their "argument"
	
	        @xxx  `yyy  *zzz  **qqq
	        -- These are super loose prefix binders.
	        -- Think of these tokens as taking the entire parsed expression to their right as their "argument"
	        e.g.
	        @my_compiler_pragma(output_as: jvm_bytecode)
	        @fn_decorator
	        ` $some($template)
	
	
	        xxx!  yyy~
	        -- These are super tight suffix binders.
	        -- Think of these as modifying the single token to their left
	        x = merge!(y)
	
	        -- NOTICE we can only play these games w. tokens that are not C++ infix characters, or are not
	           C++ prefix characters.  Else their behavior is already pinned down.
### THE-"{"-HEAD --

	Constraints:  
	--  {"key":val, ...} must be a JSON object.
	-- '{' must also serve as indicator for a Javascript code block.
	-- '{' must also terminate statement form
	-- All data is recursive expression of units  (with well defined heads)

	==> The canonical-uniform head for the "{" form is the special  no-value-specified  value  e.g. it has no head.

	So:   {k:'v'} == obj(k:'v') == ['^','', 'k','v'] 
### JSON EMBED --
	-- Must parse JSON as JSON.  with very strong preference that no special "escape" sequence needed for this
	   -- So:  [...]  needs to be a JSON lst.  {"key":val, ...} must be a JSON object.
	           all atoms need to map to JSON atoms.  Strings, Numbers, Symbols


	TABLE_OF_CONTENTS -- Uniform's toplevel structure will often be a multi-level tree of parameters 
	whose structure is understood as a 'table of contents' where the sections are not number, but are named.
	Thus:
	-- Beautiful when serving as a configuration file (e.g. looks like "Windows.INI" file
	   -- avoid "close-brace-problem" for larger structures
	   -- avoid the "where-am-i--global-positioning" problem for large deeply recursive structures
	   -- subsetion headers look beautiful
	   -- multi-level headers look beautiful and symmetric

	==> Use '::' as short hand for subsection. 
	    level1:: level2:: level3::  ....

	-- Need way of adding a head for these 


	** Classes etc.
### LIST FORM FORMAT



	~~~~~~~~~~~~~~~~~~~~
	List-Form

		[“`sym”, arg1, …, “__”, key1, val1, …, body]

		[“`if”, [“`<”, “`x”, 10], “___”, “`print”, “‘hello there”]

		[“`if”, [“`<”, “`x”, 10], “__”, [“`print”, “‘hello there”]]
### --- markdown.colon ---
ISSUE: seems we want stmt forms in most places.  
idea:  can we use ':' for both block thingy __AND__ subsection indicator?
# ### FN - FUNCTIONAL ===
## === GND / LANG ===
### --- GND.Bang - Universe creation ---
#### Name of creation operator
CHOICE: name of the universe constructor?
OPTIONS: Universe.Gnd ; Bang ; Lang

UNIVERSE.GND:
- This is the most uniform option, but is also confusing
BANG
- I love using 'bang' and nice to highlight this action as distinct from evaluation within the created universe
LANG
- Highlights the fact that each created universe is itself a mini-langauge.
### --- GND.Universe ---
#### -- Read only --
CHOICE: Universe is read only
RESOLVE:
- The placespace of a universe is immutable, but
- Its slots indicate values that 'env' accessible slots whose values can change, and 
- The entire space itself either be persistent or can be duplicated in a += to build derivative universes
### --- GND.Env - Environment management ---
RESOLVE:
The way an implementation manages the interpretation enviornment is implementation dependent.  For semantic clarity we explicitly pass the 'env' object as an argument in all places that require it.  Still a compiler need not operate in this way it might:
- construct part of the env from global state, 
- implicitly hard-code part into the compiled code, or 
- pass it on or embedded into the stack frame.
_
### --- GND.inst - arguments ---

CHOICE: Does GND.inst require spec?  
CHOICE: Does GND.inst accept/require env?


- Maybe 'inst' should do the same thing with 'env' that 'eval' & 'exe' do.
- Reasoning for eval/exe:
	- Thread local vars seem 20x slower than stack frame vars in cpython
	- Env is accessed in most every method, thus best to just carry on stack
	- Once compiled, stack frame grounding will either be ptr on stack, or be a known constant at compile time

RESOLVE: 'inst' should operate the same as all other operator calls.
- all other code in 'inst'antiated into an Exec form at compile time.
- then at run time the env is explicitly passed to the 'exe' method
- thus 'inst' should accept 'env' in same manner as 'exe'
- indeed one could inst an inst form in order to obtain an exec of it.


CHOICE:  Env could be a keyword argument, that way it could be implicit or not their on some calls.
- Can't be 'not there' since later it will be a complex structure, so even if no explicit key args are passed there will be structure there.
- Thus if it is a key argument, it will always be present.

See 'GND.Env - Env management'

RESOLVE:  For semantic clarity we pass the env as a simple argument.
Thus a dict and list must catch and discard this env in their init.


_
### many
CONFLICT: 'match' seems to be bound to particular spec???  maybe this is a method on an Inst???
RESOLVE:  There is no match, that was just the second level language's 'spec' function
CHOICE: Is 'text(Code->Text)' part of Lang, or is lang purely homoiconic
RESOLVE:  It is part of lang in the sense that text-to-structure is another Lang
CHOICE:	Should 'bindings' be a separate type
RESOLVE:  Yes, it now merged with 'Env' which is a core type
CHOICE: What to call 'spec_lang'?  meta_lang or just spec_lang, something else?
RESOLVE:  For now we are using 'lang_' as a prefix for all lang methods, since lang gets recursively employed, it helps to be clear about which entity is the lang.
ISSUE: The inst of one lang is the spec of another.  CHOICE:  So what type is arg to spec and inst? 
RESOLVE: Spec has no defined type (Unit), it might be constrained by the prior lang_meta.
		 The type of Inst is defined (limited) by the Lang entity itself. the Lang IS its type. 
CHOICE: Do 'Inst' optionally have a 'lang' operator that indicates Lang?  how relate to 'gnd'?
RESOLVE: It IS 'gnd'
CHOICE: Maybe Spec and Inst classes dont exist, since one lang's spec is another lang's inst
RESOLVE: The Lang structure itself serves as the type for its 'inst'
CHOICE: should bindings be a separate type?   RESOLVE: It is Env and matches Eval
IMPT: env at inst is about inst, not application env around result
RESOLVE:  'env' is an arg to 'inst', and in the case of a template an 'env' like map serves as a spec entity for the lower lang, which is unrelated to this first 'env'
_
### LANG.BINDINGS - should be a separate class

ISSUE: should 'bindings' be separate?

CON:
	the 'inst' operation is the inverse of 'match' but it more naturally accepts 'code' as input since it also parallels 'spec'
### ???

CHOICE:  What to call ARTEFACT?  
- Form (but that is overloaded since we talk about text form, spec form, etc)
- Functional

_
### WHY A THREE LEVEL PROGRAMMING MODEL

Why is a THREE level model of programming language required (String, Code, Form)?  
Isn't a two level model (Text / interpretable form) simpler?

Here is any argument that three level programming model is an essentials form.  (This is not to say there could not also be two level essential forms, but those more limited forms will necessarily lose critical abilities as argued here)

 - Uniform's mission is to be a best language writing language.
 - Thus it must have a beautiful surface form since human's need to write in it.
 - It must also have a beautiful code form since as a language writing langage a key activity will be the programatic creation of derived code structures.  Thus these code structures must differ from the text form since interpreting, manipulating, and applying meta data to structures latent in textual form is not elegant.
 - Less obvious is the need to be able to separate the code-form and the interpretation-form for a langauge:
   1. Programming langauges often provide flexible organizational mechanisms that allow a programmer to decouple code groupings from semantic groupings.  (e.g. package imports that allowing disparate code locations to be merged into a common semantic context e.g. a class definition).  Directly scripting interpretable forms would remove any ability to leverage these organizing mechanisms.
   2. Emitting, reading, and manipulating code as POD (plain old data) has simplicity advantages since all data is of one data type (code) even when those code units will eventually translate into a multiplicity of unit types.
   3. POD data has the second advantage of guaranteeing side-effect, state-free data.  The meaning and state of a tree of Code units is nothing more or less than the printed form of those units, much in the same way that the meaning of the text form is nothing more than its printed form.  A recurive tree of forms by contrast can and will (producutively) utilized state.
   4. Semantic obliteration -- separating a semantic form from a code form allows the code form to be optomized for script writing, which includes non-parsimonious but utile variational form.  When these forms are translated into interpretable Forms the can be reduced into a smaller simpler (but more cumbersome) set of semantic forms.

BLEAH.  this can be said so much better!
## === EVAL ===
CONFLICT:  'inst' in lang does not rely on an env when instantiating.  But Eval.Gnd might

CHOICE:  Do we need 'env0' maybe thread just returns the env0, from which we can recover the new univ???

ISSUE: ground is just inst(x.head)   Is that ok?  Maybe we get rid of it?
CHOICE: What is the type of arg to Gnd.eval
CHOICE: Is ground 'inst' (compiler) dependent upon some eval 'env'?
		Is the env some kind of special load-time env with variables bound to types not values???
CHOICE: Maybe Exec's exe operator is just part of GND?



## === PKG?? ===
### WHY ARE PKG / APP / LANG NOT DISTINCT UNIFORM CLASSES

In order to allow the Uniform paradigm to be as general as possible we want to take special care with those few parts of  Uniform that are in escapable for all languages.  All languages need to implement 'BANG' but they need not use the full unicore instruction set. 

Indeed one might adopt the SIX Unicore structure operators, and the unicore GND operator to ground the 'bang' operator.
In that case it might launch into an alternate world where NONE OF THE REST OF UNICORE OR UNIFORM EXIST.

The primary win is avoiding the Unicore NEW operator.  There is alot of langauge specific semantics bound into this operator.  

The very minimal adoption of the Unicore environment would be the 'GET' and 'GND' operator since get would be needed to access the 'bang' operator and the grounding operator would be needed to launch it.

====

A second reason to avoid classes is the avoidance of any special forms or special types is the strong maintance Uniform as a declarish langauge.

By design the 'Pkg' form derived from source code containing 'import, export, extends' statements is intended to be simple to understand and reasonable.  By keeping all three of these types pinned to the single declaration type 'PKG' allows all three of those structures to be declaratively understood, by understanding nothing more than the operations of the Pkgs statements.