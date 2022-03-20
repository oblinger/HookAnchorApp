
# EXAMPLE APPS
## IOT Example

    # Key Claims: 
    # -1-  Coding and Collaboration are greatly improved by providing an ideal ontologies and tooling environments.
    # -2-  This rarely happens since building tools and adapting ontologies are both very expensive
    # -3-  Uniform greatly simplifies both ontology adaptation and tool building, thus affording great improvement in coding and collaboration.


    # Key Simplifications At Client Level:

    #
    # - Easy to build DSL very tailored to IOT context
    # - Easy to build bespoke scripts over components since they are in common framework / namespace.
    # - Easy to express logic as data & use to handle exceptions to exceptions that come up.
    # - Easy to express desired configuration as static declarative structure w. assertions
    # - Easy to express actual  configuration as structure.
    # - Easy to express meta constraints on execution which are monitored during execution.
    # - Easy to write code based on both to manage system
    #
    # - Results can transpile to C++ to execute in embedded controllers or as part of edge processing
    #
    # Not shown:
    # - Easy to collaborate and share code.



::
    pkg com.ogrid.IOT_Cntl;

    data:
        users


    #
    # Defines an overlay





    ###########
    # SUB LANGUAGE FOR DEFINING DATA SCREENS
    #
    # screen -- defines the layout and behavior of a data screen
    # 


    # Topology Screens

    def screen crud_network_topology GraphNodeEditor



    # Here are the administration screen definitions

    def screen add_remove_users ScreenTable(rows: )


    #########
    # SUB LANGUAGE FOR DEFINING NETWORK-COMPUTED DATA TABLES
    #
    # RowSpec -- A specification of the data rows to be considered
    # ColSpec -- A specification of the column properties to derive for each row
    # TimeSpec -- A specification of the temporal axis for the data table (its range, its granularity)



    #######
    # SUB LANGUAGES FOR DEFINING COMPUTATIONAL TOPOLOGY




    #   ~-~~-~~-~~-~~-~~-~~-~~-~~-~~-~~-~~-~~-~~-~~-~~-~~-~~-~~-~~-~~-~~-~~-~~-~~-~~-~~-~~-~~-~~-~~-~~-~~-~~-~~-~~-~~-~~-~~-~~-~~-~~-~~-~~-~
    #
    # IOT - Domain Specific Types
    #

    pkg ...lang.type.iot:

        def interface User {
            $address Address;
            @cached;
            $permissions Permissions := impl.derive_permissions(address.UUID, )

        def type Permissions:
            Permission => bool

        def Permission:
            a, b, c, d



        def type DeviceGroup                    // Collection of IOT devices
        def type Taxonomy = Dag(DeviceGroup)

        def type Attribute extends Named        // IOT devices output a sequence of attribute values which vary over time
        def type Value = Num(8)                 // IOT associates attributes with double values by default 
        def type TimeInterval:
            $start Int(8); $stop Int(8) 

        def type Network


        def type Device(ID:TS, ACTIONS:TS, VARS:TS, CONTROL:TS, STATE:TS):
            // This highly templated class can is instantiated for novel categories of IOT devices.
            // Just as with C++ these can have highly optomized runtime memory layouts
            $ACTIONS    ||= []
            $VARS       ||= []
            $VALUE_TYPE ||= `Int
            $STATE      ||= { "||"($$VARS) => $VALS }                                                   // Default state lang
            $CONTROL    ||= `List( U(h:Query, "||"($$VARS)) || U(h:Set, "||"($$VARS), $VALUE_TYPE) )    // Default control lang
            $ID         ||= Hex(16)                                                                     // Unique ID for device

            // Instance parameters for a given device
            $id Device.ID
            $groups Type( List(Device.Group), ^sat: len>1)   // Groups used to organize and control (in at least one group)
            $state { Named=>Value }


        def interface Device.Controller:


        // A Statistic is some computation defined over a set of parameters, group of devices and window of time
        def type Statistic extends Attribute {
            $fn Fn(g Group, t TimeInterval, ->Value)
        }


        // Topology -- A description 





    #
### Background Types 
    #

    pkg ...lang.store:

        def type Db { Named=>Table }                // A Database is a bag of named tables
        def type Db.Entry = Int || Num || String
        def type Db.Table = List( Db.Entry )

        def type ValueStore { Path=>Unit }          // Like Redis


    pkg ...lang.compute:

        def type
## TEXT CONTROL EXAMPLE
### pkg com.dano.text_control;


    vocab_file = file.ob.proj.uf.ufdocs.1_vocabulary.html


    lines := [ header(title:$title) if line >>re"===(.*)===" else line for line in str.lines(vocab_file) ]





    #######
    # SUB LANGUAGE FOR DERIVED DATA MODELS


    with CORPUS:DocSet::    // These parameters are fixed for the data model

        def todo_items := [ it[0]  for line in CORPUS.lines if line=~re"TODO: (.*)$" ]   # Build list of all todo items


    #
    # VISUAL EQUATIONS

    # VISUAL MARKERS
    #    
    # ---<MY LIST>---
    #
### VISUAL TYPES
    #
    # VisualHook -- the regex that tells us it is there
    # Extractor  -- the code that pulls/pushes data to instances of the type.




    def textual text_list($ITEM)::
        := List($ITEM)
        anchor: re"^ *---*"


    ######
    # LINKAGES
### DATA EXTRACTS
    # 
    # Datatypes
    # - Table (selectable, persistable list)
    # - Tree  ()


    def todo_lists := TextFind()

_
### DATA TYPES  ~-~
    #   ~-~~-~~-~~-~~-~~-~-~
    #
    # TextTree    --
    # TextList    --  Text-based sequence with head and entry inticators
    # TextEntry   -- 
    # TextLines   --  := List Str
    #
    # Provenance  --  Derivation of 'how' (dynamic derivation/dependencies) and 'why' (static dependencies) value is as it is.
    # Place       --  Indication of location (used in provenance)

    def class Project extends DocSet::

    def class DocSet::
        $name
        def text(->Text)                                    // Returns the text associate with project
        def todo(header:RegEx, ->List Str)

    def type ProjectsTree := Taxonomy(Project)

_
### VISUAL ANCHORS


    == Instant Design  ===
    -  Task1
    -  task 2



    _______________

    === TITLE ===
    BODY
    ===
    _______________
    `ListAnchor
        TITLE: ".*"
        BODY
        address: text_nesting_address
        entries: ListEntry


    ______________
    INDENT- ENTRY
    ______________
    `ListEntry
        INDENT: re" *"
        ENTRY: re".*"
        indent := len(INDENT)



    ############
### DATA TYPES


    # DOC SET -- A sequence of Text intervals
    def type DocSet = Seq(Text)


    # TEXT -- 
    def type Text extends Str::
        source: Expr(=>Text)


    # TABLE -- 
    def type Table = List(Unit, @persist, @placed)






    #######
    # OTHER DATA SOURCES

    def class Table(ROWTYPE)

    def class Spreadsheet := sheet_name Str -> Sheet
    def class Sheet 
# === UNIFORM IN UNIFORM ===
### intro 

    <p> Writing Unicore in Uniform seems circular and pointless, since Unicore is used to <i>define</i> Uniform.
        One could never <i>implement</i> unicore in this way, this would be the snake eating its own tail!
        Still Uniform is claims to be the simplest the universal means of encoding all computational formalisms
        thus it should work well to encode Unicore.  Doing so gives us an example of Uniform at work, and it
        also provides a concise, precise (allbeit, recursive) semantics for the operators within Unicore.

Notes
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


_
### UNIFORM LANGUAGE DOCUMENTATION
pkg lang.uniform::

    structural::            #  Unicore STRUCTURAL constructs
    functional::            #  Unicore FUNCTIONAL constructs
        lang::              #  Unicore LANGUAGE MODEL functions
    markdown::              #  Uniform textual MARKDOWN format

800 221-
Docs(lang.uniform)'''

	///   =====================
	///   STRUCTURAL CONSTRUCTS
	///   =====================
	///   [data], [pattern], [syntax], backing, addressing, atomic_ops, container_ops
	structural -- Unicore STRUCTURAL constructs
		
		backing -- 
			GET -- 
			SET -- 
			LEN --
			ITR --
			INV -- 
			NS --
			NEW -- 
			DEL --
			
		addressing -- 
			path_top -- 
			path -- 
			path_get -- 
			path_set -- 
			path_compare --
			"+" -- 
			path_sets --
			path_gets -- 

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


	///   =====================
	///   FUNCTIONAL CONSTRUCTS
	///   =====================
	///   interpretation, placeholder, context, package, control, lang_model, universe
	functional --
		interpretation -- 
			EXE -- 
		declarational -- 
			VAR -- 
			CTX -- 
			PKG -- 
		control --
			BLK --
			BRA --
			REP --
			CHN -- 
			RET --
		language_model --
		  parse --
		  print --
		  compile --
		  dump -- 
		  load -- 
		  bang --
		  snapshot --
		  Text/Str --
		  Code --
		  Lib -- 
		  Env --


	///   =======================
	///   UNIFORM CODE CONSTRUCTS
	///   =======================
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


	///   =========================
	///   MENAGERIE DATA CONSTRUCTS
	///   =========================
	///   Lens, DDD, Stream, Docs, provenance



	///   ===========================
	///   MENAGERIE ENGINE CONSTRUCTS
	///   ===========================
	///   Transform, Pipe, RewriteSystem, TreeMatcher?, Grammar, 

	  

	///   ========================
	///   COLLABORATION CONSTRUCTS
	///   ========================
	///   Versioning, Uniform Where, Uniform Spiral



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

