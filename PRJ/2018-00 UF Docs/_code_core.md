
# ### CODE CORE ###
## _
	    _-_-_ 2019.uf1 _-_-_ 
    //
    //	structural::            #  Unicore STRUCTURAL constructs
    //	functional::            #  Unicore FUNCTIONAL constructs
    //    lang::              #  Unicore LANGUAGE MODEL functions
    //	markdown::              #  Uniform textual MARKDOWN format


<>
	pkg 2019.std.lang.unicore

// 456789 123456789 123456789 123456789 123456789 123456789 123456789 123456789 123456789 123456789 123456789 123456789
## COMMON	
### --- Common Types ---
	common:
		Unit 		= type()
		Atomic		= type:: Unit( @atomic )
		Num			= type( Atomic )
		Int 		= type( Atomic )
		Str			= type( Atomic )
		Sym			= type( Atomic )
		Collection 	= type:: Unit( @not(atomic) ) 
		Bag			= type(ELE TypeSpec=Unit):: Collection( @not(listy), =>ELE )		// Non-listy collection
		List		= type(ELE TypeSpec=Unit):: Collection( @listy, =>ELE )				// Listy collection

		Ident		= type:: Int || Sym
		Path		= type(ELE TypeSpec=Ident):: ( List(ELE) || ELE ) && @immutable
		Tree		= type(KEY TypeSpec=Ident, VALUE TypeSpec=Unit):: KEY => (VALUE||Tree(KEY,VALUE)) && @immutable 
		Patch		= type(KEY TypeSpec=Ident, VALUE TypeSpec=@immutable)::  List( u(Path(KEY), VALUE) )
		Fns			= type: Tree(Ident, FnsGrounding) && @immutable
		FnsGrounding= type:: head('GND)
## === STRUCTURAL CONSTRUCTS ===
### _
	//  [data], [pattern], [syntax], backing, addressing, atomic_ops, container_ops

	pkg 2019.std.lang.unicore.structural;
### --- BACKING --- 

	def interface Backing(++KEY TypeSpec, VALUE TypeSpec):
		GET = op(k KEY) -> VALUE 
		SET = op(k KEY, v VALUE) -> Unit(KEY, VALUE) 
		LEN = op() -> Int
		ITR = op() -> Stream(KEY, VALUE)
		INV = op() -> Unit
		FNS = op() -> Fns
		NEW = op(initializer, fns=Fns) ->
				 Unit(key: KEY, value: VALUE)
		DEL = op() -> Null
### --- ADDRESSING ---
	structural.addressing:

		pkg ...unicore.ContainerOps... :
			def op path_top() -> Unit 
			def path() -> Path 
			path_get()
			path_set -- 
			path_compare --
			"+" -- 
			path_sets --
			path_gets -- 
### --- ATOMIC OPS ---
		atomic_ops --
			"+" --
			"-" --
			"*" --
			"/" --
			"%" --
			"<" -- 
			"<=" -- 
			"==" -- 
			"!=" --
			">=" -- 
			">" -- 
			"=" -- 
			"?=" --  
			matches --
			str -- 
			parse --
### --- CONTAINER OPS ---
		container_ops --
			info_ops --
				size -- (len?)
				max --
				min -- 
				keys -- 
				values -- 
				find -- 
			destructive --
				clear --
				delete --
				delete_by_key --
				update -- 
			set_ops --
				copy --
				map --
				union --
				intersect --
				diff --
				filter --
				select --
			list_ops --
				first --
				rest --
				last -- 
				but_last --
				index --
				slice --
				set_slice --
				append --
				extend --
				reverse --
				pop --
				cat --
				sort --

	//
## === FUNCTIONAL CONSTRUCTS ===
### _

<>
	pkg 2019.std.lang.unicore.functional;
	// interpretation, placeholder, context, package, control, lang_model, universe
### --- PACKAGE --- 
	package:
		
		def class Package:
			@immutable
			def var imports Path=>Package
			def var extends Package...
			def var contents_local Code...
			def op  contents Code...
			
	//
### --- ALPHA INTERPRETATION ---
	interpretation:

		def class Env:
			def var self:: Unit
			def op NEW(repr=null, ++fns) -> Unit
			def op EXE(exprs Code..., + alpha: Unit, 
						env: Env, self: Unit, fns: Fns) -> Unit
			def op apply(fn, arg_form) -> Unit

//
### --- PROGRAMMING MODEL ---
	programming_model:

		def type Text:: Str && @immutable
		def type Code:: @immutable && ...
		def type Form:: @immutable && ...
		// class Env

		def class Form::

	def var forms List(Form)

	def pkg Construct:
		@immutable
		// nope:  def var head Sym
		def var syntax List(Token)...
		def var headmap (Path=>Form)...


	def class Form:
		@immutable
		def op  parse(text Str) -> Code
		def op  print(src _Source) -> Str
		def op  compile --
		def op  dump -- 
		def op  load -- 
		def op  bang --
		def op  snapshot --
### --- VARIABLE ---

		def class Var:
### --- CONTEXT ---
	
	def class Context extends Form:
		def var structure List(Variable)
		def var patch	  Patch(, FnsGrounding)
		def op  fns() -> Fns			// Derives sub-context Fns tree
		def op  env() -> Env			// Derives sub-context Env (from calling env)
		
	


//
### --- CONTROL FLOW ---
		
	control_flow:
		BLK --
		BRA --
		REP --
		CHN -- 
		RET --
## === UNIFORM ===
	### _
	///   UNIFORM CODE CONSTRUCTS
	///   sugar, options, variables, assignment, markdown, looping, branching, 



	///   =========================
	///   UNIFORM OBJECT CONSTRUCTS
	///   =========================
	///   type, spec, template, interface, class
	///   [persistence], [packaging], auth, view



	///   ===================================
	///   UNIFORM PROGRAMMING MODEL CONSTRUCT
	///   ===================================
	///   interpretation_types
	programming_model --
		interpretation_types --
			eval --
			normalize -- 
			denormalize --
			expand -- 
			compress --
			place --
			compile --
			denote -- 
			what -- get delaration of a thing
			how -- explain the dynamic provenance of a thing
			why -- expalin the static provenance of a thing
		LangConstruct -- 
## === MENAGERIE ===
### === MENAGERIE DATA ===
	///   =========================
	///   MENAGERIE DATA CONSTRUCTS
	///   =========================
	///   Lens, DDD, Stream, Docs, provenance
### --- LENS ---

	menagerie.lens:
### --- CONSTRUCT ---
#### Example

// UF_CONSTRUCT -- This package forms the basis for adding mix and match unicore constructs 
// in order to create a specific language combination.

pkg 2019.lang.uf_construct:



2019.lang.unicore = sources.uf_construct + sources.unicore.structural + sources.unicore.functional

2019.lang.unicore.structural = sources.unicore + 
	sources.unicore.structural.backing
	
#### Idea
A construct is a kind of source code grouping (Package) that provides a bit of functionality which may be mixed and matched with other constructs using the "+" operator.

- Each construct as a 
- Really a construct is a kind of sub-class which can be combined with other sub-classes
- APPROACH
	- In source code, several sub-classes are ADDED to their super class
	- This merges their vars and methods into an instance of the super containing the subs too
### === MENAGERIE ENGINE ===
	///   ===========================
	///   MENAGERIE ENGINE CONSTRUCTS
	///   ===========================
	///   Transform, Pipe, RewriteSystem, TreeMatcher?, Grammar, 

	//
## === COLLABORATION ===
	///   ========================
	///   COLLABORATION CONSTRUCTS
	///   ========================
	///   Versioning, Uniform Where, Uniform Spiral

//
## === SOCIETY ===
	///   ========================
	///   SOCIETY CONSTRUCTS
	///   ========================
	///   Societal Parts,
	society
		societal_parts --
			agent --
			person --
			objective -- 
			factor --
		voting --
		chain_function --
		
			

//
# OTHER STUFF
	  
  ADAPTER
	  []
	  []=
	  get
	  set
	  req
	  len/size
	  len_set

	  head
	  head_set
	  append
	  invert

	  open
	  flush
	  
	  body_append

Stream
	  read
	  write
	  close

interpretation --
			EXE
_
### ROOT TYPES
    ///
    def type Unit::                                             // The root type.  It has no constraint, so it matches all data
        $key Typespec
        $value Typespec
        $^opt   Optspec
    def type Composite:: extends Unit
    def type Atom:: extends Unit  


    //  Built in (JSON-like) data types                         // Root atomic types are supplied by underlying execution environment
    def type Num:: extends Atomic, ^opt:: bits: Int(64,)||Inf   // Float with at least 64 bits, or arbitrary precision rational
    def type Int:: extends Num,    ^opt:: bits: Int(32,)||Inf   // Integer with at least 32 bits, or arbitrary precsion integer
    def type Str:: extends Atomic
    def type Sym:: extends Str
    def type List(| $VALUE):: extends Composite, domain:Int(0,)||Inf
    def type Map(| $KEY, $VALUE)::  extends Composite, 


    // UNICORE STRUCTURAL TYPES                                 // CHOICE: do we order key&value in these types?  which order?  Consider list and dict
    def type Identifier = Int || Str
    def type Item($KEY, $VALUE) = u($KEY, $VALUE)			// Any unit with [0] and [1] having correct types
    def type Path($KEY=Identifier) = List($KEY)
    def type Patch($KEY=Identifier, $VALUE=Unit) = 
               List( Item(Path($KEY), $VALUE) )
                
    // Space Operators
    def interface Space(|$VALUE Type=Unit, $KEY Type=Key)::
        u( extends:Addressable, $KEY => $VALUE||Space($VALUE, $KEY) )
        def op path_gets(old_space) -> Patch                //  space_delta
        def op path_sets(patch)     -> Space                //  space_apply
        def op view_enter(Patch)    -> Space
        def op view_exit(Patch)     -> Space

    def interface Namespace(|$VALUE Type, extends: `Space(VALUE:$VALUE, KEY:Ident))::


    def interface Container::    @can(contain)
    def interface Accessible::   @can(get)
    def interface Mutable::      @can(set)
    def interface Listy::        // children are indexed as 0,1,2,...
    def interface Iterable::     @can(itr)
    def interface Invertable::   @can(inv)    // Might only have first (might not be total)
    def interface Addressable::  @can(path)   // Taxonomic
    def interface Backed::       @can(ops)



_
### UNICORE PROGRAMMING MODEL
///
pkg lang.unicore.lang::

    def var CONSTRUCTS List Construct                       // Defines this universe's lang (usually a '+' list)
    def var base Env                                        // Used to create universe, load time macros, etc.
        

    def op parse    (text Str) -> Code
    def op print    (form Code) -> Str
    def op compile  (src Source) -> Lib
    def op dump     (env Env||Lib) -> Code
    def op load     (src Source, env Env=) -> Env            // updates in place by default
    def op snapshot () -> Lib


    def type Code = Immutable
    def type Construct = Immutable
    def type Source:: Str || Code || Lib

    def interface Text:: extends.Str                        // needed?  probably better not?  (need to indicate denotation and lang)
    def interface Code:: @can(get, itr, path) @cant(SET)    // 

    def op parse(Str -> Code)


    def interface Lib extends=Namespace(Construct):: 
        def op compile(src Source) -> Form
        def op dump -> Text
        def op load() -> Lib
        def
        @can(load, bang)


    def interface Env::                                      //  ENV -- An interpretation environment
        def op ns -> Namespace                               //  Returns the namespace providing semantics for this env
        def op snapshot -> Lib                               //  
        def op EXE(form... Code, | alpha)                    //  Interprets code form in current environment
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           

    
    def interface Construct::                                // UNICORE PROGRAMMING MODEL
        def var tokens List markdown.Token                   // 


_
### UNICORE IMPLEMENTATION
///

pkg lang.impl.structural::




pkg lang.impl.functionalj::


    def type PKG::
        sat: grounding.has(opt:has(`PKG))





_
### Copyright
    <footer>
        Copyright (c)  Daniel Oblinger.  All rights reserved.
    </footer>

