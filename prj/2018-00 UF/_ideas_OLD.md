## EVERYTHING
### Metasyntastic -- lotta syntactic goodness
     Pure syntax -- mapping from text to data-structure is semantics free
            (parse&print do not depend knowlege of language terms -- req for homoiconic)
     Aesthetically Covering -- contains <q>complete-ish’’ set of syntactic variants such that for any specialized language, L, there exists a Uniform sublanguage L’ that is semantically equivelant to L, and having the same astetic attributes and comprable <q>beauty’’
### ESSENCE
    ====
    A formulation captures the ESSENCE of a notion when any simplification of that formulation
    ceases to capture the notion.
    ====
    A formulation captures the ESSENCE of a notion when even small simplifications of that
    formulation no longer capture the notion at all.
    ====
    Expressing the essence of a thing, is to express it so simply that it is no longer expressing the thing at all
    ====
    One has captured the essence of a thing when any simplification means it is no longer captured.
### INTRINSIC 5Ws  who/what/when/where/why/how
    who -- agent initiating computation resulting in value
    what -- multiple descriptions of the data itself
    when -- timepoint when comutation occurred (described in time DAG form)
    where -- lex location of this data and of origination data
    why -- n-level provenance explanation (written in why spec)
    how -- (I think this is duplicative of why)
### JITASTIC
    (Partially) removing the false fork of performance / simplicity-beauty-convenience-flexibility.

    Powerful hammer is auto-managed functional dependency.  Even if one cannot tell for sure forever
    If a particular optimizationn is possible, we can know *now* that it is and JIT compile that option
    with triggers to recompute if things change.

    (this is simply the notion of JITting, but it is make all the more simple and convenient when
    the compile the sources and the sources of the soruces are all within one universe that is visible
    to the JIT mechanism.)
### PERMISSIONS -- ACCESS

    In non-technical settings "ACCESS" is a means of approaching or entering a place.

    In Lex we express all notions of permissions and security as "access" to lex locations.
    Specifically each computation is operating under an access model which is a treespace selector over lexspace
    (a selectorspec formula).  Such restricted computations execute as the values of all lex not matching its
    formula were undefined.  (so this will update the size of parents which are still visible, etc.  it is as
    if those lex values where just not there.

    All permissions in lex are managed by the single 'restrict' operator:
        restrict(code, selectorspec)

    This operators executes 'code' with permissions: cat(current_restrictions, freeze(selectorspec)),
    that is it operates under the constraints on the current execution intersected with the frozen value of
    the current value of 'selectorspec'


    There are a great many ways this restrict operator can be combined with other aspects of Lex in order to achieve
    a range of permission models:

    -- Permission escalations.  Normally restrict is used to produce executions that are more restricted than the
    current computation.  But it is possible to provide a restricted execution context with access to a lex
    capable of less restricted execution.  In that case you are providing the keys "to get out of the box"
    Using these modestly trusted code could upgrade its own permissions in certain controlled sections where
    greater access was needed.  such targeted upgrade can sigificantly reduce the attack surfaces on this code
    since attack must target those upgraded regions if they hope to utilize those greater permissions.

    -- Reframing.  Lex frame allow one to view data as if only parts of that data exist.  Combining reframing and
    restrict operator allows one to "harden" these views so it becomes impossible for a computation to ever reframe
    back to the larger view of the data.

    -- screen.  By providing access the screen of a lex, but not access to the lex itself, you are in effect
    providing read-only access to that location.


    taken together a single selectorspec formula can very flexibly restrict access.
    If we imagine the universe as a collection of database tables where each lex datatype is a different table
    and each instance is a row of that data.  then we are able to programmatically restrict columns of individual tables
    or interrelated tables via arbitrary compuation.

    Rows of each table can be hetrogenously mixed and scattered aross lexspace so that their acess patterns
    match the structure of lexspace, finally access to entire tables is controlled by access to that kind of data.

    -- UNDERSTANDABILITY of the consequences of all code restrictions is declarish.

       -- Selectorspecs' themselvs are declarish, they are designed to be understood via static inspection.
       -- Lexical scoping itself is declarish, and the restrict operator's consequences are a function of the
          restriction of the calling environment.  In the common cases where constant (or at least templated)
          restriction selector specs are employed, then one can declarishly understand all restrictions on all
          computations.

      -- it becomes possible to visualize all parts of lexspace as a
         as a fuction of which parts of the code can affect them.  a very nice way to understand and assess
         and minimize attack surfaces within the code.

Manifesto Point:  All code should be DECLARISH.
    in some cases this aspiration will result in exactly the code we have today, but in many cases
    the declarish mantra would yield understandability gains w. little or not loss.
### COMPILE

    transform computation into a functionally equivelant computation where the transformed computation is
    targeted for some execution environment with the intent of speeding it execution time in that environment.

    compile(codeLex, targetLex, keys,...)-->code


    A compiler is a code rewriting system that targets specific execution environments with intent of faster execution.
    The lexlang default binding for the compile operator is a planning system that applies a rewriting system over
    code rewrite rules
### SHORT WORDS TO CONSIDER ===

to
be
been
is
that
which
and
has
have
with
at
of
same

not deep any all a include ok true false exist exists empty blank args eq eql equal
satisfy sat change


~-~~-~~-~~-~
### SHADOW VALUES ===
### STATEMENT FORM  ===


 HEAD_SYMBOL



public static final int fn(int x)


foo: fn(int x) public static final returns int extends SomeClass



======
### VIEWS:  as uniform (all forms), as tree (lexographic), as boxes
### DIV CONTROLS:

-- zoom in/out
-- drag-adjust
-- drag-n-drop
-- click / right-click / double-click
-- edit
-- hover
-- peel-back:  data(source/filter/compute) | layout

-- toggle:   view/edit
   edit -- all control points and regions become visible and controllable

-----
### XXXXXX
 uniform embedding should expose units, and a method 'source' which provides the substring source for that unit.
  -- the interval of an atom's extent (w/o leading/trailing whitespace)
  -- the interval of a brace form including braces (excluding trailing \n for ws forms, and trimmed indents for form)
  -- setting either source or value will compute the other

-----
### FACTORY
A software factory is an object that produces object instances that adhere to a particular standard
requrire by the code that accepts the factory.

A great pattern that ends up clunky in many langauges, bolted onto a system that already uses
classes as Factories for instances.  They are also clunky since in addition to the data
being operated on, one must also pass around factories for producing more data (which you mostly
want to be of the same kind that you are already operating on)

In lex we oblit this difference between Factory and class, and also oblits the distinction bewteen
handle of an instance prodced by factory and handle for the factory that produces instances.

//////
In Lex we use objects to represent their own class, and we use instances as their own factory.
So the new() opeator, operates on an INSTANCE of a class, and will create a new instance with the same implementation as the original, but empty.
NOTE: Lex is a traditional class/instance langauge it is NOT a prototyping language,
it just oblits a lot of needless fiddling and flipping, by always using a instance of a class
as a representative for the class itself.

how to get the first instance of a class

-----
### Namespace
namespace is mapping from names to values which is declaratively specified in source code

pkg;  import

------
### Timepoints store/retreive/apply
In Lex there are three semantically relevant time points
-- the time a value is stored
-- the time a value is retrieved
-- the time a value is applied.

* Each of these time points can cause dependent computation to occur across Lexspace.
* The results of retrieval/application are only dependent on the latest values in lexspace
       not in the order they were placed there.

XXXX
  BUT: temporal ordering among assignments to non-overlapping parts of Lexspace does not matter


-------
### LEX (definition)
lex -- an elegant language for elegantly creating elegant domain specific languages

lex is an elegant language for creating elegant languages, elegantly.

if you found this sentence more annoying than amusing, you might want to skip to the JSON script docs.
if you found this sentence more amusing than annoying, you might want to read on.
### List Truncation Semantics
------
==ISSUE==  if we avoid malformed lists, then assigning undefined to a list cell must erase those to the left
or we keep track of 'length' or compute it slowly each time

-------
### ISSUE map list
  -- unit map/list is really weird -- why is this at the very center???
  IDEA -- The 99 is right
  if 99% of humanity agree on what a thing means, not doing that thing is probably wrong.
  parsing of multiple values

------

  unit map/list/what ever they are are really weird... that must be wrong!!
  hmmm, good criticism.
  for me it helps to not focus on the weird case fist.
  first say:  uniform allows maps and lists.      CHECK -- solid idea, all good langauges should
              uniform allows weak typing where a variable could be a map or a list.   CHECK -- solid idea.  all good langaugues should
              uniform has a data type representing a function call.
                JUST FINE.  most languages don't but it is a well defined thing, useful if one were doing homoiconic things
                            and it doesn't introce any wirdness for having it.  (not more than extra types for calendar entries or such)
              uniform correctly include this function call type in the inheritance hierarchy of data types.
                CHECK if you have the type, you should definitely do that.
              uniform's there is no class in uniform more general than this funky hybrid class -- so it is synoymous with the uniform "top" class...
                CHECK it is a odd fact, maybe.  a result of how you defined things, but its just a fact.
              uniform replace the name for its top class with "unit"
                CHECK there is no advantage and much confustion for having two langauge constructs that represent "top"

          ER-WAIT!  This piss-ant, chimera-map/list-functioan-call-thing, which a moment ago was relegated to some
                    sub-sub-package useful only in specialized homoiconic cases is now the cornerstone of all data in your language?!

          Yup. that pretty much sums it up.
          My take is that:
            -- it *is* an odd structure.
            -- building one that is performant in implemented systems is tricky and perhaps not worth doing.
            -- but in practice in non-homoiconic contexts it will just be a list, --OR-- a map, --OR-- an atomic type.
               so there is no harm done.
            -- and my experience writing code with it, has been quite pleasant.  Specifically because
               (1) if a variable is just a list, a map, or an atom, then I just think about it that way -- no harm done
               (2) having a head on base types like list and map is really nice!
                   in other langauges one either ends up with data that as in grows in complexity is hard to understand and error
                   prone, since it doesnt' say what the map is a map of, etc.   ---OR--
                   you have to go down the JAVA path where every freakin data structure is a new class, and you spend all day
                   writing insert, delete, and all the things that already exist on base composite types.
                      I always reap the best of both, where there is reaping to be done.   this little thing is huge in practice.
                   >>> one can extend base types in some langauges, but often it takes some care to make those things
                       work **exactly** like the base types do... print like they do, etc.
                   in unit with head "FOO" and keys 0,1,2 is already as much a list as one with a head "LST"
               (3) a surprising number of extra "if" conditions drop away even in non-homiconic "fancy" contents. when that shared
                   'unit' context exists between all data types.
               (4) a great place to wedge meta/ancellary data ... a place the automatically 'does the right thing'
                   the problem with ancellary data in tranditoanl contexts is it breaks the beatuty of the raw strcutre, since it is not
                   longer pure able to be implemented as a simple strcutre.  worse yet, continuous care must be employed to keep that
                   ancellary data aligned with the core data.  ugh.  waht a messs.
                   >>> other flexible lanauge like python, java script, etc. allow flexible annotation of meta data
                       but only at the object level.  so adding meta comment to the integers in a list of int would require creating a parallel structure
                       and maintain the alignment.   this is horrible when it happens.

         so yes. a bit weird, but a weirdness that seems to be more theoretical tahn practical.
         in working with standard code, either you forget it even there, or you see it, because it
         helping out, providing some uniformity would require special casing / extra data / etc.

         Totaly work the trade IMHO, but then Lex is my baby, and all babies are beautiful in their parent eyes.
### UNIFORM PARSING
- parsing is the mapping of an interval of text onto the subunits of a unit.

- ATOM tokens parse a unit that represents the info of that single token.
- recursive tokens initiate a recursive specialized parsing of their "body" text upto their matching closing token using
  - BRACE is a recursive token which applies std parsing to its body
  - QUOTE is a recursive token which parses into a unit where subunit 0 is assigned the body string
  - COMMENT is a recursive token whose body and result token are ignored by parsing

- PUNCTUATION is a token


----------

PERMISSIVE PARSING --- dont error

GRADUATED PARSING -- error when you absolutely must, but try to limit the scope of the effect.
### BASE_POWER --- don't be a d-bag lazy language creator.  recognize
   many will suffer needlessly because of your laziness!


-----------
### 80/20-capture of "source code"
-- ESSENCE -- The following statements are true of nearly all "source code", they capture notions that define the "essence" of source code.
-- SIMPLER -- The resulting language is simpler than most source-code languages
-- COVERAGE -- All things called "source code" map onto this one "uniform" langauge
-- USABLE -- in a way the retains the feel and and usability of the original.


 The observations below are so intrinsic to the notion of source code, they almost feel tautological
 yet taken together they are the of uniform, an 80/20 capture of all the world's "source code"
### SOURCE CODE
  -- a text-based format for a recursive structured used to direct execution

TOKEN -- tokens are specially designated sequences of characters (uniform handles alphanumeric indicators, but does not call them tokens)
CONTAINMENT & TYPE  -- tokens are used to specify the type and containment structure of the entities within source code.
TOKENS BIND IN PRECEDENCE ORDER  -- each punctuation form has a precedence level, and punctuation with higher precedence group to form sub-units within.
AT EACH LEVEL PARSING IS LINEAR  -- at a single level of precedence tokens are paired with their arguments in either a right-to-left or a left-to-right fashion.
COMMENT BEHAVE AS IF THEY ARE NOT PRESENT --  begin with their initiator, end with their terminator, source is parsed as if they are not there exist.
QUOTES         --  are like comments comments, but they parse to a STR. they have an ignored escape char which allows terminator to be added to the STR
BLOCKS  -- source code can be broken up into block which are recusrively arranged -- they are visually aparent and indicate key semantic relations in source  (e.g. flow, scope, accessiblity, etc.)

Lots of common imparative structures look the way that we have come to expect them to look:
 funtion calls, method calls, chaining,  looping, branching, var access, var assignemnt, conditions, recursive expressions, structure indexing
### INSTALLER
    installer

  -- a computation that results in packaged code being useable in the install environment.

safe installer

  -- one that only DIRECTLY affects it install point
  -- one that can be perfectly undone (as if never installed)

  -- places itself in single place in env & DECLARES all exports into env.
     env itself integrates
### DECLARATIVE BINDINGS
    -- by default 'import' causes all declarative bindings to flow
    --
### PREDICATES
                and(set1,...)   or(set1,...)    not(set)
- "is" tests    obj() lst()     num() int()     str() sym()     defined()assigned()atom()bool()satisfied() composite() undefined()
- unit specs    typed(ks)       named(ks)       has(ks,us)      all_have(ks,us)
- keyspecs      fixed()         name()          meta()          non_meta()      any()
                equals(v)       same(v)

primitiveS
  converters    to_obj()        to_lst()        to_num()        to_int()        to_str()        to_sym()   true false null und
    str         format()        length()        char(i)         chars()

PATTERNS        ...()  set_...(..., value)   is...   mk_...  use_...  contents()  req...


import  in_pkg
LOCAL FLOW      block(c1,...)   while(bool){}   if/elif/else   return
  helpers       map req([v],else:)  try(...,else:){} contexts: und, fn, block
I/O             input  output   parse print pprint source                  uf, canonical, json, xml, rdf,

SPEC TYPES      AS=atomspec  CS=codespec  KS=keyspec   SS=strspec   MS=metaspec   PS=pathspec   RS=refspec   TS=typespec  XS=exitspec
                rulespec, parspec

 keys([ks])      values([ks])    items([ks])
### FACTORIES
    The Factory pattern provides awesome flexiblity and generality, but it is clunky
    since two pointers must often be passed around, and it must be baked in from the beginning

    in lex all objects use 'new' as their factory.
### FRAME
           fget fset fsize fdo fremap(km) fbase fslot fmapping faccessor(get/set/size/do/remap          key_in(k)key_reverse(k)mapkey(k)remap(km) FRAME           frame_get(i)    frame_set(i,v)  frame_expand(c) frame_ordering  frame _instances frame_shape   reframe  unframe
~ reframing creates a nway graph of views for the data.
~ a shield is a reframing that yeilds opaque expansions
### TEMPLATING
FORMAT    -- "flights {} from {} to {} are delayed by {}".format(arg1, arg2, ...)   ``flights {num} from {source} to {dest}``
             ``this is {'{}'} weird but you can do it``
OTHERSPEC -- force/continue/break/exit/err/fmt_string   {self} {at} {y.path} {fn_name??}
    x.update_in(y,'flights {at})
### PROTECT(code, chroot)
    safe_execute(code, read:treespec, write:treespec

    protexecute
lex.reframe(kswitch

    --

    def super_flat():
        result = []
        for child in self:
            result.extend( child.super_flat() )
        return result


    in_pkg lexlang.queryspec;

    def up() { map(){parent()} }

    pkg = myself
    pkgs = tree.all_under().typed("in_pkg").has(0,self.resolve(self))==pkg).map(){parent()}
    result

    tree.qs( typed("in_pkg).up() }


    def super_flat()   { self.map(){self.super_flat()}.flatten() }
    def super_flat()   { flatten(){super_flat()} }

    tree.all_in(){

lst.flatten(){ self.
### SUGAR FOR REFERENCES

    foo[low|high]
    foo[low;high]
    foo[low,high!continue] = [1,2,3]

    kwargs.get('key', !default=22)
### DEPENDENT

    A dependency graph specifies certain values are a function of other values.
### GRAPH SPEC

    specifies a potentially infinite driected graph.

    graph:  nodes, sources, sinks,   node.inputs()  node.outputs


    Graph( Node(inputs: Lst(Arc), outputs: Lst(Arc)), Arc(inputs:Lst(Node), outputs:Lst(Node)) )
### ALTSPEC  EXITSPEC  OTHERSPEC OSPEC (force,continue,break,exit,error)



    INHERITANCE TREE FOR TYPES
    under

    x.head().is.under(



    fast_sym_switch_table
### LEXOGRAPHIC ORDERING
    a... < b...  iff a<b or a==b and ... < ...   (the a part less than the b part)

    operators:  < > == <= >=   ~~
### BUILDING FROM THE BOTTOM

    -- Reference lexcore written in python:  dump/load   lex/unit   execute eight
    -- Interpret:  lex-based compiler
    -- Compile:    lex-based lexcore.  lex-based  dump/load,  lex/unit (based on frame),  parts of execute, all rest of lexlang
    -- Cross compiled lex runs on other
### Half-ass declarative or DECLARASS is
### IO
    io.textual
    io.visual
    io.clickable
        -- A Screen Region
        -- Registers mouse clicking -- as x/y coordinate relative to clickable origin
        -- Dragging registered via changing point value
        -- Specifies release location if drag-n-drop occurred.
### Copyright
        Copyright (c)  Daniel Oblinger.  All rights reserved.
