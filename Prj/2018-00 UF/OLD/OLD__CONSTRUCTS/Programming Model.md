
### >PROGramming model -- older
#### Universe Structure

seeds --  unit factories providing implementations for this universe
Each seed must implement NEW, str, parse, cmp
????? 

"Lib = a collection of named resolved packages"
defclass Lib = Patch(Path<-Pkg)
Lib(
	'qualified.package.name <- Pkg(
		content_0, 
		...,
		key1: value1,
		____init____: initialization_form 		// only executed if pkg is instantiated
	),
	'package.name2 <- Pkg(
		...
	),

	"App = a Lib containing a package called 'main' -- an app is a toplevel application"
	defclass App = is(Lib) && contains( `main<-Pkg($has(____init____)) ) 

	main <- Pkg(
		____init____: initialization_form 		// The toplevel procedural code
	)

	"Lang = a Lib with a Pkg called 'lang' which is capable of interpreting some kind of Code"
	lang <- Pkg(
		ATOM: atomic_unit_seed,					// .parse() .denote() .snapshot() .pickle() .str()
		BAG: mutable_unit_seed,
		FORM: immutable_meaning_seed,
		ENV: dynamic_extent_stack_frame_seed,
		LEX: immutable_place_seed,
		OPS: root_environment,
		bang: 
		EXE:
		GROUNDING:
	)	
#### Objects in the models
        tree/dag. bangable. Bangable w. work.  Persistent/Modifiable/Frozen/Extendable/
code -- tbp  --  A persistent/immutable tree of code denotational forms
self -- t.m  --  A node in tree of contextual assignment objects (has var assignments)
heap -- tBm  --  A tree of units that store (persist) data
auth -- tb   --  A tree of permissions (each env has set of)


ops  --	A node in extensible tree of immutable trees   [tbp]
		CTX operator in tree used to define sub nodes and tree of VARS
		VAR elements in tree have v: are @lazy set.
		lang.*  defines current language
vals --	Persistent tree of bound values (matches VAR forms in Ops tree)  [tbp]
		Some vals themselves are persistent trees of values, others are mutable.
		'ops.*' is a persistent tree underlying this 'vals' tree
root -- tbf  --  An infinite tree of places.
env  -- tbm  --  A node in tree of interpretational contexts (thread is linear fork in tree)
#### OLD - Essential Interpretation OLD
Env
	self
	_ _ OPS _ _
	alpha


Interp(form, self, ops, alpha)

env form  => env form meaning            meaning = form.OPS(form.GET(head))
#### Tiny Model API
Env
	self 						--  Inner class instance context
	_ _ OPS _ _ 				--  Current dynamic meanings
	_ _ UP _ _ 					--  Outer dynamic context

Ops
	key -> Ops

Unit
	_ _ OPS _ _ 	
#### APIs	
**=== Universe ===** 				--  Ways to kick everything off
Universe
	.bang(Ops) -> Env 			--  Instantiates and 'starts' computational universe
	.compile(Ops) -> Ops 		--  Targets ops for execution in this environment
									Contains spec for rewrite system for compiling Ops

Instances
	lib.universe.Javascript 	--  Creates Javascript-backed universe for execution
	lib.universe.Java8
	lib.universe.Python3
		
**=== Ops ===** 					--  The static form of a runable thing
Ops 							--  Immutable tree of unit values.  lang/prog/fn/code
	lang 						--  If defined then ops is a programming language
	lib							--  If defined then ops has a library
	main 						--  If defined then ops is a program (different name??)
	
Pkg
	@PKG						--  Logical name (many may have same)
	import List Path 			--  List of requirements 
	init 						--  Operator specifying init actions

**=== Lang ===** 					--  A uniform Language
lang.CODE 						--  Source forms w. denotations in lang (might cause error)
lang.FORM						--  Denotation forms that may be in Ops (for execution)
lang.ROOT						--  
lang.UNIT
lang.ENV 						--  Base Env for this universe (all load forms operate here)
lang.load(Sources)  -> Patch  	--  Derives a set an updates to ops
lang.parse(Str)		-> Code 	--  Translates string source text form into tree of code units
									(performs normalizing rewrites)
lang.expand(Code) 	-> Code     --  Translates code rewriting/expanding
lang.compile(Code, Tgt) -> Ops 	--  Translates fully expanded code into interpretable forms
lang.main() -> Unit				--  Executed as first interpretation in the new universe

__=== Env ===__
.ops 				-> Ops	 	-- 	Operators available in the interpretation environment
[...var...] 					--  Variables defined within the env
env								--  Defining parameters for this env
env.bang(Form)  	-> Env    	--  Creates derived universe
env.load(Src)   	-> Env		--  ????   DOES THIS INJECT ???
env.parse(Str)  	-> Code
env.denote(Code)	-> Form
env.seed.(heap/form/lex/env)	--  Seed units for this env  (can provide other seeds)
env.root()				 		--  Root of lexspace  ??? (maybe just env.seed.lex)
__=== Unit ===__ 					--  Standard methods
.ops 							--  The 'ops' defining the _home_ env for unit
__=== Ops ===__						--  Tree of semantic operator bindings
[bang]							--  Returns the subtree of bindings for the universe
__=== Form ===__					--  The interpretational meaning of Uniform code
Form.EXE(form0, ...) -> u		--  Each FORM has groundings .. e.g. 'eval 'compile ...
CTX.enter(Env)					--  Performs entrance actions for this ctx
CTX.exit(Env)					--  Performs exit actions for this ctx
CTX.structure()					--  ??? Maybe just inside 'ops'
CTX.ops_env()				
VAR.access()
VAR.assign


**=== Bang(ops) ===**			--  Computational Universe Instance (e.g. compiled JS)
__=== Thread ===__				--  Inherits ops from bang
#### APIs new

=== Lang ===
Lang(UNITS: Seq(Unit), HEAP: FORM:, ENV:, CODE: UnitSpec, FORMS: Seq(Link))
.forms -> Named(Link)
  .syntax -> aggregation('-1.forms.syntax)
  .syntax -> Seq(f(h:'Syntax, head:'Sym, kind: nest||comment||str, arg: 'Unit))
  .normalizations -> 
  .expansions -> 
.parse(Str) -> Code    // uses syntax
.print(Code) -> Str    // uses head to index forms
.compile(Code) -> Link // uses head to index forms (and maybe 'links' to perform global updates )
.dump(Link) -> Code

=== Form ===
.idx -> head
.syntax
.normalizations
.expansions
.parse .print .compile .dump .bang ."+" .snapshot
#### >Resolve

alg.resolve(f Form, v Vals, vars List Vars) -> Patch	
-- Accepts a form with unresolved vars and computes patch that resolves them
#### Stuff

       MARKDOWN                             INTERPRETER
Str  ---parse-->  Code  ---denote-->  Form  ---bang-->   Env        Form --compile--> Form      Env --load--> Env
      <--str---         <--express--

TYPE    COMPUTATION EXPRESSED AS:
Str  -- linearly-structured, textual data.   Str  --parse-->  Code
Code -- recursively-structured, unit data.   Code --denote--> Form
Form -- mathematically simple constructs.    Form --bang-->   Env
Env  -- an active execution.                 Env  --load-->   Env

===CODE===
Code             --  Formal spec langauge for a kind of computation
CodeFlow         --  Flow constructs that control the order of computation
CodeDeclaration  --  Declarations that affect non-control-flow aspects of computation 
CodeStatement    --  Statements which modify the behavior of declaration forms
CodeArgbody      --  Schema for arguments for some 
CodeCommand      --  Commands that define or control this computation as a whole

===FORM===
Form
PKG
FN               --  An operation 

//  ENV -- An Interpretation Environment
defclass Env extends Unit:
	$pkg  PKG
	$self Unit

//  ALPHA -- An Interpretation Universe
//  An Alpha Unit is used to create a new computational universe.
defclass Alpha extends Unit:
	$lang    PKG            //  The alpha pkg providing meanings for this universe
	$lex     Lex            //  The alpha lex the origin point of the lexspace for this universe
	$env     Env			//  The alpha env is the stack frame bounding the existence of this universe
	$threads [Env]          //  Computation threads with access to the shared globals and heap
	$heap    UnitBacking    //  Persistent backing store whose Unit exist for the duration of this universe
	$io      PKG            //  Structure containing external interfaces for this universe

	// BANG -- Creates a new Universe based on an App
	deffn bang(l Lang):
		$ap     Pkg    = l['alpha]
		$heap   Unit   = ap.heap.NEW(ap.Unit)	//  Uses 'alpha.heap' implementation for heapstore
		$a_unit Alpha  = heap.NEW(ap.Alpha)     //  Uses heap to allocate the Alpha unit based on types from 'l'
		$mains  [Code] = pkg.select(=~Pkg && has('main, =~Fn))
		
		a_unit.lang    = l                     	//  The language for a universe is simply its defining package
		a_unit.lex     = a.lex.NEW(a.Lex)      	//  Uses 'alpha.lex' impl of Lex
		a_unit.env     = a.env.NEW(l)	      	//  Uses 'alpha.env' impl of Env, with language as type
		a_unit.io      = a.io
		a_unit.threads = []
		Thread('{ for p in $mains {p.main} })	//  
		return a_unit
		

	// RUN -- creates a new thread and begins its execution
	defop run(...args):
		return Thread(...args)
		
	// LOAD(src) -- Loads new packages into current universe
	defop load(src Str||Code||Form, @into=true):
		if src=~Str: src = cmd.parse(src.as(Str))
		if src=~Code: src = cmd.denote(src.as(Code))
		if !src=~Form: error("expected a Str/Code/Form but found {src.type} insead")
		$patch Patch = pkgs.build_load_form(pkg, src.as(Form))
		$updated_pkg = lang.path_sets(patch)
		if @into: 
			self.lang = updated_pkg
			for pkg in patch.select(=~Pkg && has('main, =~Fn)) {pkg.main}
			return updated_pkg
		else:
			return bang(updated_pkg)
		
// THREAD --
// 
defclass Thread:
	$action Code
	$stack  [Env]
	 
	defop init(action Code, @start, @wait):  	// INIT -- Creates a thread
		self.action = action
	defop execution_body: 						// Internal operator executed as body of thread by underlying OS
		...
		
defclass PKG($KEY) extends Form:
	defop source -> Code
	defop lex -> Lex
	defop GET(k $KEY)->$VALUE
	// GET('main) is run on load.  GET('init) is run on instance creation
	
defclass FN extends Form:
	$prefix ARGBODY
#### Primatives

EXE -- 
PKG -- A mapping from identifier to identified meaning
GND -- An element of control flow (FN)
ENV -- An execution context
NEW -- A creator of backing

PKG --> PKG || GND
ENV --> PKG || Unit


(  Decl)   PKG/GND
(5 Flow)   BLK/BRA/REP/RET   TRY  EXE???
(7 Unit)   GET/SET/ITR/LEN/UP/IDX  NEW      
           Maybe: ITR/SIT
(5 Stream) GET/SET/FIN/POS/??? close/sync/position (FIN/???/POS)
           Maybe: open, close, read, write, position, sync

heap  UnitBacking of mutable units for computation
env   UnitBacking of stack 
root  UnitBacking for immutable units (used for Code)
io    UnitBacking for external units


--UNIT--CALCULUS--

ENV

alpha_interpret
alpha_expand
alpha_reduce

PKG/GND

Unit
#### OLD ALPHA BANG

	// BANG
	// Creates a new universe
	deffn bang(seed Alpha||Pkg):
		$pkg   Pkg    	= if seed=~Pkg { seed } else { seed.pkg }
		$alpha Alpha   	= if seed=~Alpha { seed } else { pkg.alpha.heap.NEW(Alpha) }
		alpha.heap    ||= pkg.alpha.heap.NEW(UnitBacking)
		alpha.lex     ||= pkg.alpha.lex.NEW(Lex)
		alpha.env     ||= pkg.alpha.env.NEW(pkg) 	// Global variables are assigned within this frame
		alpha.threads ||= alpha.heap.NEW([Env]) 
		alpha.io      ||= alpha.heap.NEW(Pkg)
		alpha.EXE     ||= pkg.alpha.EXE

		alpha.env['alpha] = alpha
		alpha.env['io]    = alpha.io
		alpha.threads.append(alpha.env) 		// ??? Add newly created thread to 
	
		return alpha.alpha(alpha: alpha, io: alpha.io)	// ???
		return alpha.funcall(pkg.Alpha, alpha: alpha)	// Calls the constructor
##### Execution Rules

Given   Unit alpha,
		Form form, 
		meaning = scope.path_get(form.head)
- GROUNDING REDUCTION -- 
- STD EXPANSION -- Form is replaced by meaning of form applied to the form itself
	form  ==>  meaning * form     
- DEF EXPANSION -- Form is replaced by the OP( alpha: DEF(...)) meaning of form
	form  ==>  BLK(body...) * form 		where body = meaning.GET(alpha)

GND( eval:")

##### REWRITE RULES
	meaning	= env.scope.GET(form.head)
	defbody = meaning[alpha]{0}	   IF meaning.head=='GND  AND  meaning[alpha].head=='DEF
form rest... x env x alpha	==>		meaning form rest... x env		# SEMANTIC EXPANSION 
form rest... x env x alpha	==>		defbody form rest... x env		# DEF EXPANSION -- 
form rest... x env x alpha	==>		grounding 
EXE(result, alpha, env, form0, form1, ..., form_n) ==>
EXE(r,a,e,f0,f1,...fn) ==>
EXE('eval,e,SEQ(f0, f1...)) ==>
	EXE('eval,e,f0)  EXE('eval,e,SEQ(f1...)) 


Given form, env, alpha
	let m = meaning	= env.scope.GET(form.head)
	let s = selection = m.GET(alpha)
	let g = reduce( EXE(a,e,f0,...,fn) )
stack..., EXE(a0,e0,F...), EXE(a,e,f0,...,fn) ==>
	stack..., EXE(a0,e0,F...), EXE(a,e,f0,...,fn,meaning)		if meaning.head!='GND		# SEMANTIC EXPANSION
	stack..., EXE(a0,e0,F...), EXE(a,e,f0,...,fn,selection[n])	elif m[alpha].head=='DEF	# DEF EXPANSION
	stack..., EXE(a0,e1,F...), g					where g,e1=reduce(EXE(a,e,f0...,fn)) 	# GROUNDING REDUCTION


>> ~~~~~~~~
>> replace EXE with stack frame
>> How do create sub-stack frame, how do I create var within frame?

Given form, env, alpha
	let form 		= 
	let meaning		= env.scope.GET(form.head)
	let selection 	= m.GET(alpha)
	let reduction	= reduce( EXE(a,e,f0,...,fn) )
heap0, stack_0, ..., stack_n-1, EXE(alpha), f0, ..., fn ==>
	heap0, stack_0, ..., stack_n-1, EXE(alpha), f0, ..., fn, meaning	  if meaning.head!='GND	   # SEMANTIC EXPANSION
	heap0, stack_0, ..., stack_n-1, EXE(alpha), f0, ..., fn, select[n])	  elif m[alpha].head=='DEF # DEF EXPANSION
	heap1, stack_0, ..., stack_n-1, result	where result,heap1 = reduce(heap0,alpha,f0,...,fn)	   # GROUNDING REDUCTION


SEQ()						==>  null
SEQ(c)						==>  c
SEQ(c0, c1, ..., cn)		==>  SEQ(c1, ..., cn), c0

BRA()				  		==>  null
BRA(c0->a0, rest...)		==>  BRA(->a0, rest...) c0
BRA(->a0, rest...) True		==>  a0
BRA(->a0, rest...) False	==>  BRA(->a0, rest...)




##### Alpha
form(i [,j]) --> Form	# Returns the ith form in the jth frame (from the most recent frame)
env	--> Env				# Returns the current env
#### DEF
DEF lambda_form			# Simply performs EXE on lambda w/o changing ENV (however 'forms' is updated)
##### CTX (see >CTX)

##### Scope

Scope is a place that defines mappings from symbol to meanings.

Interpretation may enter/exit a scope and units created within a scope retain its meanings over lifetime of unit.
#### Rewrite System

In Uniform a RuleEngine is a system that accepts a set of 'rule' forms which configure the behavior of the Engine.
The engine then accepts inputs and generates outputs according to its configuration rules.

Engine:  takes rules and an input and processes them to produce an output.  may take multiple inputs & multiple outputs

bar: engine.Ruleset( P($x)==>Q($x) )
Foo: engine.Rewrite( P($x)==>Q($x) )

build: engine.RewriteBeanPole(import bar)
result = x.build
result = build(x)

implicate = Foo( '[P(5)] )
e.result

x = 'P(5)
y = x.implicate