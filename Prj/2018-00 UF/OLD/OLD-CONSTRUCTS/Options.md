
## Option Names
### CONTAINER OPTIONS (those that apply to its collection of children)
ORDER -- ordered: true, seq, bag, false   ?seq? (ORD, KEY)
SEQ -- unit or stream has a persistent ordering.   (LIST, ORD, ORDERED)
BAG -- unit or stream does not have a persistent ordering.  (UNORDERED)
SET -- unit or stream does not contain duplicates that are EQUAL
SETEQ -- unit or stream does not contain duplicates that are UNIQUE, EQ, ID, IDENT

### MUTABILITY OPTIONS 
MUTABILITY - ability to be altered   (GIVE, FABRIC, PLIANCY, ADJUSTABILITY, DUCTILITY, 			     			VARIABILITY)
LIVE - unit may change based on something other than SET.  (DYNAMIC, VOLITILE, HOT)
FREE - unit is destructively updateable.  (MUTABLE, FREE, VARYING, VARY)
LOCK - unit values will not change once defined. (PROGRESSIVE, LOCK, LOCKING)
FIRM - units value will not change. (IMMUTABLE, FIX, FIXED, STAT, STATIC, PERSISTENT, SET
		FIRM, HARD, FINAL, STABLE, STATIC)
DURABLE - unit retains value beyond interpretational universe
ARCHIVED - DURABLE & FIRM - 
VERIFIABLE - 
VERIFIED - 
CHAINED - ageless -- unit will exist with this value forever (SAFE, CHAINED, FINAL, 
		PERMANENT, TIMELESS, FOREVER, INFINITE)


### COMPUTATIONAL OPTIONS
ELECTRIC -- (when)
lazy -- unit values are not computed until accessed
eager -- unit values are [re-]computed at earliest possible moment
cached -- unit values are memoized for subsequent use.  (MEM, MEMOIZED, CACHED)

### DECLARATIONAL OPTIONS
pkg -- unit represents a package with package semantics
ctx -- unit represents a context with context semantics
var -- unit represents a variable with variable semantics
gnd -- unit provides iterpretational semantics for a unit  (OP)

### OTHERS
FAN -- fanout: true, 
FROZEN -- 
@fn 	-- is functional.  no side effects, order temporal/structural is unstable??




#### DECLARATIONAL FORMS
**@PKG** -- Declarish src code containers with combianational algebra defined over them
		fixed args specify combinations
**@CTX** -- Declarish spec of an environmental context
		first arg specifies enter/exit actions form.structure and form.function
**@VAR** --	'variable' -- declarish spec for place whose assigned value _VARIES_ over time
**@OP**  -- 'operator' -- declarish specifier for an operator






### Thinking

#### CONTAINER

Choice: what to call the top level grouping type?  ==> Bag

Option: Collection/Container
- Most standard naming for such a thing

Option: Bag
++ Short three characters for a super central concept.
++ Single character variable name 'b' does not conflict with many other key terms.

++ Class name not need to be typed in source code too much (still will end up in op names comments etc.)
-- Bag implies no ordering.  Most generic collection will have no ordering, but 


##### more

CHOICES:  Option Names

Ordered – ord:  true, bag, false
Persistent – persist: true=immutable.   forever, progressive, indefinite, immutable, mutable, dynamic
  progressive - once the head it set, then subtree is fixed forever, 
  functional – once head it set then it is immutable

dated - dated trees have date prefix between 2000 and 2099, and have progressive persistence.  
      assignment is only valid with propert auth and must be beyond specified date.
