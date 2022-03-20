
# _
Extensibility -- is a continuum.  No aspect of a system can be extended, to every aspect of a system can be extended.  Many put bright lines around extensibility zones and call that extensibility, but as a LWL this is not sufficient.
Extensibility -- the ability to take a given capability and "extend" it so one leverages the original capability in a new context containing new abilities.
From a LWL perspective most languages are NOT very extensible.  because there are many different choices for many different aspects of a language, and when building a new langauge one often wants a different choice than the undrlyting lanaguge provided.  These extensions are often not possible.  One must instead build an interpreter for the new lanaguge "from scratch" loosing many abilities froomt he original langauge and needing to rebuild them whith the interpreer.
FOR EXAMPLE: native types within many langauge are often quite fixed.  if they do not behave exactly as you need in your extended context, then one cannot extend the original language, but must instead use underlying lang to re-implement from ground your desired language.  e.g. the casting-behavior, the existing operators, print behavior, etc. are often fixed.
# Unsorted
### unsorted misc
- - flesh out example of live unit as parse and non-parsed and other parse
	- example of a spreadsheet and emails and code commits, and 
syntax is non-forking.  only other non-forking syntax basically present the raw AST to the user (e.g. lisp)


- organization -- group of agents.  structure with purpose.
- hash -- merkle 
- INFO DELTA -- Measured in seconds.  (parsomony, verbosity, cognitive complexity)
* DECLARISH -- visually apparent in an approximate way directly from static source code, while precisely defined through functional interpretation of the same source code.
* CONTEXT -- 
* LEXICAL -- declarish properties associated with the containment strucure of code.
* SCOPE -- a place in space of meaning mappings.  a mapping from identifier to meanings.



??? when does NEW set the UP pointer... what is it set to

??? does Bang exist separate from Thread?
??? do Bang methods get passed to Thread/env is env accessible/needed
??? it thread accessible outside ???

### >STD >VERSION

std.2018

// Structure is progresive but assignment is not accepted until 
dated.2018.
### Shimless
A carpenter uses a shim to make small adjustments in order that two things fit tightly.  A good carpenter never allows these shims to be visible, and when hiding is impossible they will use a 'shimless' design where pieces are made so precisely no shim are needed.
 
In a similar way, bits of code are often used to make adjustments so one thing can fit into another.  A good programmer also keeps these hidden.  But in a transparent hiearchy of components built from other compoents, there is no place to hide.  By analogy we describe a langauge design as 'shimless' when compoents are combined without shim code.

Specifically this means if C is the combination of A into B,
then understand and operating C is as clean as if A and B did not exist, and one simply developed an interface for C from scratch.

Ideally all construct combinations are shimless.
# === PLAN ===
## --- TACTICAL TARGETS NOW ---
- Find languages person to talk w.
- Ready presentation:  Markdown #1,2,3,4 examples.
## --- USABLE PY SYSTEM ---
- Embedded Interp & toplevel loader for .py with embeds
- Unit Embeddings:  Filesystem.  Text/Json/Yaml/Html data.  Exe.  Gdrive.
## --- HIGH LEVEL PLANS ---
### DOC PARTS IN ORDER
- SLIDES - Structure & Function complete and precise
- SLIDES - Exposition of Key Uniform Lang:  View, Electric Unit
- SLIDES - Core parts of Language and Menagerie captured
- SLIDES - Target1 - Comprehensible Software: complete providence, source-code&delta simplicity
    full dependency transparency
- SLIDES - Target2 - Live Text Management
- SLIDES - Target3 - 
- 
### CODE PARTS IN ORDER
- PARSER -- python parser.
- PRIMATIVES -- bang+14 primatives
- TRS -- Functioning TRS
- LANG PARTS -- Build macros for key/nifty language parts (show easy to do & cool results)
- COMPILER -- SIMPLE COMPILER TO JS||Python
- Structure & Function: Slides.  API summary (bullet semantics).  Python Implemementation.
- TRS implementation.
## --- AGENDA TARGETS ---
### HACKER COMMUNITY
- Idea of a so-small-it-cant-be-smaller lang is compelling.  get ~10 super devs excited about agenda.
### **CODE MARKDOWN** -- Get a strong code-markdown implementation out and in use.
	(but by itself will it be widely used?)
### Others	
- WWW DOCKER CHEF -- 
- BUSINESS AUTOMATION --Develop small community building interoperable
- TEXT KM -- 
- ML TOOL ECOSYSTEM -- 
- HACKER COMMUNITY -- Foster tiny hacker community who "get" the vision.
### TEXT KM
- regex view
- connectors to file/docker/gdrive
- connectors to CSV/xslt/gsheet
- parsers for XML/HTML/JSON/YAML
- 
### BUSINESS AUTOMATION
- >>> KEY INDICATOR: EXTREME VARIABILITY <<<
- Identify small slice of business automation that has extreme variability
	- ML data applications / visualization / reporting
	- Textual K management
	- Server meta-management
- Build toolkit to support key use cases
- Expand nacent support community -- allow customers to pull request back in for residuals
## --- QUESTIONS ---
### EVALUATE CODE-MARKDOWN
- good manually generated examples
### EVALUATE SIMPLICITY AGENDA
- Define multi-level completeness
### SUPER EMBEDDABLE
x
### SHOW THE POWER
- get def type specified
# === THINK ===

## --- NOW ---
ISSUE:  what is the elegant way to create a new unit?  how to specify its head?, its type?
ISSUE:  UnitCode allows graph keys?  uses identity hashing?
wtf is a SYMBOL?
stream semantics for writing, return value of key&value, unit.append(u, i)
how to encode graphs in uniform
can we use ':' for generalized maps?
load compile new-path-map
### PYTHON OPERATIONS ON UNITS
- py operators  head ... 
- For loop -- 
### BROKEN
- Issue:  the idea is that parsing is semantics free, but how do we handle format string?  Its internal structure is PARSED!  but in a way determined by the SEMANTICS of the operator
- Solution:  Parser is re-invoked on sub parts, during normalization? 
	This means parsing, can be re-invoked, but only during the normalization
	phase, not later, so it remains separated from CODE form.  sufficient.
### ITR but not GET
ISSUE: Should be able to express collection that is iterable(ITR) but not accessible(GET)
### How are ENV lenses implemented?
### How is uniform type embedding done?
### SINGLY ROOTED CODE SPEC THINGY
- what the hell to call it?
- what specifies its API
- what is its predicate/transform/matcher form
### Map semantics
- we allow list of key/val pairs as most general version of a map
- but we also require that a single key has only one value
- what is the right "most general" answer.
### COLLECTION -- how to represent an unordered collection?
### COMPILE -- what is the general form
target lang
target data type
eval / assign / 
are these args to 'alpha' or to EXE?
- is this now just part of load
### Parsing???
### NEW cannot initialize meta keywords
### PATH TEMPLATES
a templated thing whose only parameter is the path the roots it.
### DENOTE
should be a short 'denote' function which maps a source form into the thing it "denotes"

denoted object is only defined for a unit that is in lexspace.
lex is also only defined for unit in lexspace
### LOAD (needs to append to global closure)
### HOOK
### HOISTED STMTS
set of commands associated by key
indicated by head or by stmt or by (meta) key
### OUT-RIGGER
A LAYER is data collected into a named sheet and layered onto some structure w/o affecting structure or other layers
A META is internally embedded layering info
A RIG is externally associated layering info 
## SOMETIME
### TEMPLATE TYPES OF UNIFORM.  KEY, VALUE, etc

Unit(KEY,VALUE)      Unit(DOMAIN, RANGE)

KEY     --  Nat||Str -- The standard 
VALUE   --  Unit

UNIT	--  The standard mutable object type  (UNIT / MUT / CONAINER / SEED)
ATOM 	--  The implementation type for atomic value 		
FORM 	--  The implementation type for immutable sematic units   (CODE)
ENV 	--  The implementation type for a stack frame
### OBLIT static & dynamic
Every 'static' thing will be a dynamic thing within some containing ???temporal???  ???scope???

So @static should indiate the static-ifying scope boundary.
### DECLARE OP EVAL
Foo: SomeType(...)
f = Foo()

y = x.f   // how to control its behavior as an operator

x.f = y   // or as a place
### FORM(->type)
type system needs to type forms (not just functions) based on what they return
### TEMPLATE VARS
#### how to reference source code template var values at compile time.   e.g. UNIT.KEY
### CHOICE syntax colon
### SPEC groups
It is natural to define a number of spec and have a spec that merges them.

uf.Statement::
### WRAPPING
-- need class wrapping & dynamic wrapping
-- need compile time actions and runtime actions
-- need conversion x.as(Int)   but also creation. x.make(lazy)
### How to do bitvector operators  & | !
### "Relative Places"
A "relative place" is a form that appends onto other place forms
 ( creats a dereferecable assignment expression)
x <~~ y.select( foo )

add/removing e from x should set  e.foo = true/false so it is selected or not as expected.
### Lang and friends
bindings -- map meaning to data
executable -- bindings with standalone interpretation that depend only upon their interp lang
executable -- a standalone set of bindings expressed within some langauge.
language -- bindings that provide an interpretation context depend upon nothing

language -- a framework for interpretation.  (interpreting forms data with meanings attached)
executable -- a stand alone, interpretable form.
### Special Unit Backings
-- Generator -- a unit that:  cant GET,SET,LEN,UP,IDX only ITR
-- Innumerable -- a unit that cant ITR/LEN
### Lazy structure
two ways
-- specific fields are lazy
-- entire structure is generated
### Option flags
-- flag is a boolean option
-- options may be boolean, nary, or power set.  (or other)
-- when nary or power set.  they should/could also define boolean options
-- how to we efficiently
-- options STMT form is all fixed args, but then processed options are all keys
### DDD -- Declarative Dependency DAG
build chains
provenance declarative/static
data flow

-- write the Dddspec

-- highlight version number -- a structured identifier with a total ordering defined over it which is used to indicate a version of a thing.

-- Lang == Bindings   highlight (langauge == a collection of meanings)
   obliterate distinion between lang/application/DSL/module
-- DDD  language  version_source location form_with_functionally_bound_deterministic_vars output

-- less & equal defined over structured.  (then use 'can' to control ???)
-- define ordered things like TIME ??
-- store -- spec that says it is persistent beyond any execution env.  (e.g. object permanence)

-- op_prop -- deterministic
### BANG and friends
prefer the have declarative relationship between bindings.  
So nicer if BANG creates env from a current execution env and a bindings object (nothing more).

would like to define *unversioned* functional dependencies between URI named entites. 
then when specifying a version for one of these, it defines versioned versions of dependent entites.  
nice if naming auto-carries this versioning info.  (maybe using dates, global version numbering to ensure merging w/o having explosive namings...  versions would require versioning 'source' to coordinate dependies. )

declarative dependency graphs.

#### Simple Bang
-- should bang *only* accept one arg and return a new env?
i know that is an option, but should that be the only option, then all else is just editing the bindings?
### THNK CONTRUCTOR BINDING
 -- Constructors should somehow unify w. backend implementations to pick correct at compile/run time
### Object Invarients for created objects
-- NEW -- issue how to create an instance while ensuring the object invarients are satified?
### FN TYPE

-- Fn -- what is the type for a thing which can be called?  (e.g. Type 'init'  or any operator)
### OPTIONAL GC
Is it ok to GC data that was expecting explicit free operation
# === TASK ===
## --- ASAP TASK ---
### !!! REMOVE ALPHA SELECT
Fix alpha-reduce fix GND to do select 
### Add VAR and PKG as ^opts
### UNIT-CALC
-- Change rewrite systems to only have expand / reduce
-- edit code primatives and unicore sections
-- what about primative that defines new ops.
### Flagset Int is atomic?
what is atomic?
how to handle the structure within these atoms?
### Add "as" operator
.as(Int)   // no need for quoting
// but then how do you do an 'as' to a dynamic target?
// what is the simpest extensible syntax.
### Adding option syntax inside unit parens
foo(x:4, @atomic)
Maybe options are extracted from ANY position in a form?

Can macro have a 'Eliminate' value?
Can macro collect values that exist outside the macro?
### Misc stuff
* ADD META PROVENANCE (static impute thru operators?u)
* THINK   try{...}   how to do it?  with  quiet_null quiet_und   set ops so it is null not err
* VIEW how it works
* CLASS TEMPLATES VARS how?

* How do options variables names relate to the operator names that they option?
* figure out if unit spec responds to 'matches'
* body forms for pkg etc.  (use special code to indicate body beginning)
## --- EASY TASKS ---
### Add headcase and typecase to menagerie
NOTE: It was a moment of pure joy when the author realized that, without stretching any naming to get there, Uniform
was going to have a form called "head case."  Perhaps recursively this has implication for the author himself.  
No matter!  There it is.  
### Figure out tab.  
use it to replace tables.
### Areas of the Docs

    <!------------------------------------------------------------------------------------------------------------->
    <!-- ACTIVE PORTIONS OF THE DOCS:

    lex_core        --  Defines core semantic notions underlying all of Lex.
    uniform_script  --  Reference list of all lex operators, grouped by function.
    uniform_menagerie   Reference list of uniform libraries

    lex_sentences   --  Attempt to enumerate essential elements of imparative computation
    lex_manifesto   --  Discussion of the ideas and goals underlying the design of Lex
    lex_oblits      --  A gleeful, aspirational look at the dragons Lex aims to vanquish


    OUT OF DATE
    lex_glossary    --
    z_lex.html        --  An overview (maybe never to be rebuilt)
    lex_goals       --  goals of Lex (should be rolled into the manifesto)
    lex_execution


    -->
## --- TASK SOON ---
### UF-PPTX ready
-- core argument in place
-- Main lists filled in

### UF-CHEATSHEET Ready
-- Most rows points to section with
   API section
   dl list of ops
   examples
-- Use '+' and '*' to denote done-ness
### AREAS
#### Provenance
#### Build
#### Access/Auth
### Write SOW
### PPTX MARKETS
### UF who to ask


### ADD SECTIONS
AltSpec Options Syntax
### Add flush
 -- or maybe flush is more primative...  applying to any non-locally persisted data</dt>
### Define 'declarative' in 'package' section.   (a thing visually aparent from structure of the source code)
### MERGE alpha select&gnd
TODO merge alpha select into alpha gnd.  be sure that opts fit with groundings (how to declare opts and ensure they don't conflict w.applied opts.)
## --- DECIDED NOT DONE --- 
* LOWER CASE -- use lowercase 'var' since these data structures will be used in many places.  but maybe needs seperate from 'var' operator
* THINK   delete keymap???  list itr   (YES)
# === DONE ===
### PLACE on structure not lex
* really need to have a "place" operator on structure not on lexspace (it will apply there too)  u.place(k)->p    p.set(v)  p.get()
### ONE LINERS
TODO add path_set and path_sets (SET should also return 'self')
OPTS -- use list or map for opts?  
# === UNSORTED ===
## Words
### UR-way

-- when morass of related mess is found
-- pick two parts that seem different.... try to precisely state common & boundary
-- if cannot then state two most stark alternatives
-- place one on left of table and other on right
-- tightly grip table and repeated slam forehead
-- after minutes, weeks or years one might begin to see outlines of a 3rd form hovering above table
-- staring long its the outlines of true-forks 

-- or smash together
## To place
cam, cem, cim, com, cum
cake eating maneuver:  all objects have unique addresses, but we take no performance hit in space or speed when it is not needed.

Oblit:  C++ style templating for free
Oblit:  VAR  C++ template var, XSLT structural template var, prolog var, PHP template var,
## to scan
TODO template.eval == fill(alpha.env_current)
THNK what about resolve...  can be simplified?
!!!! how do opts injections with and @ fit together?
!!!! TEMPLATE MACROS --


-- Write Mantras & examples of oblit, decomplect, simplify
-- detail implementation

THINK   op( options: eval: GND()  macro: DEF() )




THINK  lexspace/bindspace is immutable ref->form or string->form   (what about circular)
THINK  LOAD -- load, bind/resolve
       Update 'bang' so __alpha__ is not part of the structure of lexspace by default, but is in the bindings.



TODO PKG ALIAS -- Update Unicore pkg to have alias and to carefully summarize the logic of importing

TODO  add 'opt' and 'err' symbols to UNIFORM (not unicore)
TODO  the NEW operator needs to macro expand its target before doing the NEW op.
TODO  BODY FORMS -- update text describing the parsing rules for '::' etc.
TODO  UNICORE DECLARATION FORM -- update pkg so that it uses unit body, and includes location destination for data as first arg.        semantics is that location ref is resolved and that is the location. or it is added to current location.

TODO  KEYMAP --  named, args, ?keys?, meta,
TODO  LOWERCASE INST HEAD -- Unit creation generates head that is all lowercase


!!!   We want simplest AST, yet pkg use body key, so it introduces a second layer in representation.  fix??
!!!   How to add '+' constant head in template?  avoid it standard meaning

!!!   SYNTAX DOT     foo.bar(x) == DOT(foo, bar(x))   or   SEQ(DOT(foo, bar), TUP(x))   or   {'^':DOT(foo,bar), 0:x}   or   DOT(foo,bar)(x)
!!!   PRECIDENCE     zoo | -foo | boo
!!!   Lexical semantics should be defined as function of Lexical form, not modules in lexspace once unified

?!?   PLACE SYNTAX --  *foo  and &foo  (maybe??)
?!?   FACTORIES -- where to original unit

???   AUTH & FACTORIES -- exactly how does Uniform use Auth to create new instances
??? CURRENT ENV -- How to reference the current env object
????  MOUNTS -- how do we efficiently call in code.  we need method that does not require construction of unit with pairs in it
????  MAP CONST -- how is a generalized unit presented.  (where keys are not just symbols)
????  CARROT -- Use ("^") instead of "__foo__"
????
????  Backend/Frontend   maybe .foo() is frontend and .^foo:5 is backend
????  OPT DEP RECOMPUTE


??? -- contains, in, set_or_append, imports
??? should symbols really be units with that same head?  advantage is that
            any time a sym is used as an indicator, it could be 'upgraded' to have additional info.
            but this removes idea that symbols are unique (since two indicators with the same symbol name)
            could now be different
???  is it ok that all instances have an address in lexspace?  it means that one can always "see them" even if they should not be accessible because of encapsulation.
???  <b>lex()</b> should each instance be in lexspace (maybe only when queries?)  what about each ENV.  again maybe only when lex() is called?
???  What is the value of ALT, alt, null, PKG?  symbol is its own value?  or not?
???  KEYMAPS do we still use keymaps?  how to manage generality there?  relates to permissions?
???  REF!=PATH   should the ref type be different than the path type
???  Should the 'io' namespace be separate, or just part of Uniform?


REALLY DO
DO Change parser to ',' is same as ';'

MAYBE DO
DO allow TRY to push onto the stack and stay on stack requiring an explict RET to exit
DO change INT to Int  ???
DO maybe the arg order for resolve, get_ref, path_get and path_follow should have reversed args
DO move all doc todos groups to this file...
DO - add oblit complections
DO DECLARE MAP -- how does a unit declare its intent to be viewed as a map?
DO str, IDX and UP -- should these also optionally be settable fields


LATER DO ===
DO SCRUB syntax doc -- there is out of date info
THNK DESTRUCTORS



TA SK DO   ===
DO   OPTIONS for UNICORE -- add options section specifically for Unicore
DO   Motivation Slides
--


NEVER DO ===
NO  consider lowercase for GET SET ITR LEN type
NO  merge PKG and NEW



=== HOW TO SHOW ===


=======
=== NAMES ===


???  pkg -> bag
???  Bindings ->  Defs



????  WORDS TO USE:  bag/bin/lib/vat/can/pot  pin/pen   sec


USING FIXED NOT NAME ARGS FOR primitive COMMANDS
-- would be more elegant in case of 'let' and 'with' but would require reliance on map ordering.
  up side is that it in memory == source map directly, but downside is that semantics depends on that ordering



==CRITICAL==  operators like '==' will map as FUNCTIONS !!  oh we can use a macro?\


==ISSUE==  cannot parse python  stmt ending in a colon, since we currently use colon for k/v

==ISSUE==  null    unit()=no-head     lst()=JSON_list, obj()=JSON_object    null=unit() or =null()
           now blank can be null()


==ISSUE==  LEX-VALUE CONFUSION
           -- have 'get' & other operators on place map to data operators.
           -- have 'child/parent' operate at the lex level.
==ISSUE==  way of indicating live vs copy vs destructive
==ISSUE==  NUM-STR AMBIGUITY.  when shown in 'dot' form, an int key, and a string w. int value look the same
==ISSUE==  default map iterator is not ordered, but list iterators do expect ordered keys
==ISSUE==  NULL-UNDEF.  Is JSON null the same value returned what value is returned for "no value" from a statement?
           .child_or_none returns child for null, and None for undef.    but child.value() returns None or undef
           for Uniform   undef also returned (not None)
==ISSUE==  KEY-ORDERING.   In fncall ideally args are evaluated in order.
==ISSUE==  LIST LENGTH.  Maybe we say the 'len' of a list is 1+ the largest defined numeric index??  Issue is undefined / missing values

==task==   refactor le.run  to be Lex.start or such
==task==   need to add support for pushing and pulling distant state into / out of backing

==task==   clarify.  Lexspace may be computationally HEAVY or it might be nothing more than adding a
           parent pointer and a child binding which refers back to GET.


UNIFORM
-- '::'    nesting should create obj(...) forms.
-- '::'    with no-whitespace suffix should add


~-~-~
ISSUES: <br>

-- fixed_children, kw_children, meta_children
-- value / set_value   should accept pathspec

-- LEX DAG???   -- stack frames need to have pointers.  so do objects on the heap! <br>
-- PATH???      -- how does this parse out?  and refs in the code?   dot(-1, foo, bar)   ref(one_level)
-- Watch        -- fn(^watch: list(ref1, ref2)    watch(ref,
-- watch forms need to be set <br>
-- ^lex.bindings needs to be easily edited
<br><br><br>

- a watch should allow a lex value to depend on other lex values
- should allow many values to depend upon many
- should not do any spooky action at a distance

- a fn or value derivable from currnet bindings and should seem like a constant,
  then later if bindings change the constant should change
- activate/deactivate

watch( ^body:foo,  ^update:update-code, ^cached_watch_values: bindings(foo:22), foo:ref(-1,foo) )
- a 'watch' is special head symbol...
- when a lex is 'activated' all watches under it are scanned and indexed and replaced with a current value
# === LOG ===
### Formal Semantics

(1)   xxx  means replace xxx with semantics expansion of xxx
(2)   xxx(....)   means  apply( resolve(xxx), stack(....) )     means funcall( resolve(xxx), .... )
(3)   aaa . bbb . ccc   means   ccc( self: bbb( self: aaa ) )
      funcall( resolve(ccc), self:funcall( resolve(bbb), self:funcall(aaa) ) )

square: fn(x):: return x*x

square(3)

apply[ fn(x)::return x*x, 3 ]

apply[ apply[ fn_def, fn(x)::return x*x], 3 ]


COMPOSE( fn_def, fn(x)::return x*x, square(3) )
### Wrap command idea
*** 

wrap(raw_type, [ref_prefix='wrap_'], get:, set:, ... )

--  Resolves operator definitions relative to raw_type first adding the 'ref_prefix'

--  
### Misc 2
Array Form
  ['^UNIFORM', uniform-forms-here, ...]

  [":def", "class", "Something",
      ':extends', ['Super1', 'Super2'], 

:{in-string-colon replacement}



C++ CODE EXPANSIONS
   x.get('foo', y())  ==>
   x.GET(foo, y())    ==> 
   (__t1=x.foo)!='__und__' ? __t1 : y()


JS CODE EXPANSIONS
   x.get('foo', y())  ==>
   x.foo === undefined



ARRAY PROPERTIES
-- a.length  (1+ largest assigned int)
-- a[i]      std when in bounds

-- C++       needs to do bounds checking, so might as well handle -1 etc.
### Misc 1

   EXAMPLE CODE   ===


::
    pkg lexlang.type;

    Lex = Typespec;;
        self: Lex
        parent: f
        n(__ret:Lex): get('__parent', break)




    bindings = fn():
        is_pkg_spec = fn(): get(0).head('pkg') && self[0][0].head(".")
        lex_roots   = fn(): [self].all{ get(0).head('pkg') } + declared_imports + lex_imports(parent)
        lex_import  = fn(): lex_roots
        decl_imports= fn(): self.head('pkg').all_subunits{ fixed, head('import'), add_all(self) }
        return [].tree_update( *lex_imports, x:continue )




  #
  # The bindings for a lex, l, are defined by:
  #
The pkg(l), for a lex, l, is defined as:



def pkg fn(self:Lex, returns:Ref):
    get(0).get(0)  if get(0).head('pkg')
    parent.pkg.copy.tee(append(key, continue))


 # your imports are:  yourself if you are a package, and every named import (it is an error if they are not a pkg),
def imports fn(self:Lex, returns: [Refs, ...]):
    [self].all{pkg} + all{fixed, head('import'), **self.all(fixed)} + **parent.imports


def bindings fn(self.Lex, returns: bindings(k(str):Lex









some = fn():
    arg Unit x ==



=====================
### Misc ignore

 Unsorted

* To do
*** Docs 1.0
     Unicore:  primitives
     Spec -- lots of work


*** BUILD 1

- 20  UnitBasicAdaptor        clear, delete, ...
- 10  TrivialInstances   Pkg, Path, Ref
-     Lex
-     Bindings

-     UnitFactory        .new(lex, ....)    creates new 'unit' instance
                         ==> somehow used to interp_native ????
-     unit               u.foo(...) invokes chained op.  

-     Impl

-     Js                
-     Units 



Bindings
 .get(
 .resolve(ref)

UnitFullAdaptor
 .__bindings__             

Unit
 .__bindings__().resolve(op)
 .__chain__(op)
 .__call__(op, arg1,..., KEYS, k1,v1,..., BODY, b1,...)
 .op(arg1,..., KEYS, k1,v1,..., BODY, b1,...)

Val
 


*** Tiny stuff
CORE OPS -- what is needed to port to a new platform

*** Syntax Updates
***** Need single quote (') as reader quote form
***** Need  $foo  $(foo+bar)  ${maybe} 
           '$'(foo)
***** Need  foo(x, y, $$insert)  $+foo  $-foo
***** Need fn(x,y):: ``Is {x} times {y}``str
      expands to qq_str(x, y): "Is {} times {}"
*** Ric Release
***** Example/Sample.uf -- scan thru Uniform docs to generate example doc
***** REF -- Finish ref doc (exe codes)
***** LIVE -- provide basics of 'live'
***** SPEC -- provide meta spec spec
***** TYPE -- provide type spec

*** Minimal JS Compiler Approach

-- 


APPROACH
-- Treat three types specially:  JS scalars, List, Map
-- compile( expr, 
### === TYPE TEMPLATE ===


  [VALUE_TYPE, ...]  shorthand for {fixed:VALUE_TYPE, ...}
  HEAD(TYPE_FOR_FIXED, ..., name:TYPE_FOR_NAMED)

  {VALUE_TYPE, ...}




  a_fn(str, str, int, ..., a_key:sym, name:int, ...)
  a_fn(str, str, rest(fixed, int), rest(name, int), a_key:sym)
###  Green Blatt's quote -- the unifom agenda

     <!-- THE UNIFORM AGENDA

     green blatt's :   every program has a bad version of lisp.

     Dan Oblinger:  Ur right.  So lets build a ``Lisp'' once and for all specifically designed for this purpose.


     The Uniform Agenda:

     Strip away any and all details that separate software languages from each other.
     Re-imagine these details as meta programming on top of some ``UR'' substrate of computation.
     A substrate that explicitly contains the essence of the important ideas of software systems
     without any of the surrounding details.

     The uniform ``ur'' language, can then become the conceptual interchange format...
     Any software boilded down into uniform-form, can be translated without loss, and naturally into
     other software frameworks, since uniform only contains notions central across most or all software models.

     -->
# === OLD ===
### >ORD
ORD()			#  The default ordering
ORD(all)		#  An ordering that including all keys
ORD(list)		#  "List" ordering: the range(0,LEN())
ORD(meta) 		#  Alpha-numeric-underscore keys beginning and ending with "__"
ORD(named) 		#  Alpha-numeric-underscore keys begining w. alpha and not meta keye 

The @ord option on a unit specifies if its ordering is stable.
(@fn is a shortcut for @ord=false)