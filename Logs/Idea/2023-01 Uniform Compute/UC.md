# 2023-01-23  Uniform Compute


A trivial seed used to define shared compute 

A simplest space of hosted compute:


## CORE SEMANTICS


### UNIFORM DATA

In essence computer data is symbolic information that affords structured access.

**VALUE** -- A *value* can be any JSON value.  

**ARGS** -- An *args* is a value that is also a JSON map whose keys are non-negative integers or are strings that are valid Java identifiers.    By convention, a value associated with the special "^" key is called the ***head***, values associated with whole numbered keys are called the ***fixed args***, and the remaining values are called the ***keyword args***.  An ARGS can be written in functional form as:
	head(fixed1, fixed2, ..., key1=value1, key2=value2, ...)

**MAP** - A *map* (or namespace) is an API that implements a "get" operator that maps its first arg called the "key" onto a value.  
Optionally a map can also implement "set", "len", and "iterate" operators too.  The following shorthand may be used:
	map[key]  to get value.    map[key]=value  to set value.   len(map) for its size.   map.map(args) to iterate over all keys except the head of an args 


### UNIFORM COMPUTE

At its essence a computer language specifies what happens when you "apply" some "executable" code to some "input" data.

Uniform Lang expresses this idea in a most basic and universal form:


A language (lang) is exe that implements such a compile operator.  Implicitly each lang specifies a set of "valid" exe value, these are the values that it compiles onto non null target exe.  we use lang.source and lang.target to refer to these two evaluators.  

LANG is not quite right 

**LANG** -- A *language* (*Lang*) is an executable with apply, compile, target_lang, and valid_exe operators as defined below.  Formally:
  *lang*.apply(*exe*, *input*) --> *result*					is used to apply supplied exe to the given input, where exe is interpreted according to this given language.
  *lang*.target --> *lang*								is used to obtain the language onto which this language compiles.
  *lang*.compile(*source_exe*) --> *target_exe*		is used to compile executable in this source language into its underlying base lang (which can simply be itself)
  *lang*.valid(*exe*) --> Boolean						return true iff *lang*.compile(*exe*) is non null.  (May be faster than compiling the input exe.)


**APPLY** / **EXE** -- The *apply* operator is a mapping from a JSON value, (called an executable or "exe") and a JSON value, (called the "input") onto another value called the "result" as shown here:    
		apply( exe_value, input_value ) --> result_value
By convention, if the input value is an args we refer to the head of that args as the "operator" being applied, and further say that the executable, *exe*, "implements" the operator, *op*.  E.g. if  apply(*exe*, *op*(...) ) is well defined we say exe implement op.  We can use the chaining operator's dot notation " . " to implicitly denote apply function like this:   *exe* **.** *op*(...)

**COMPILE**  -- The *compile* operator maps a source executable value appropriate for some source apply operator onto a target_exe that performs equivalently for some target apply function or to null if the source_exe is invalid as an exe for the source apply operator.  Formally:
		given src_apply, target_apply, operators then  compile is define such that 
			for all src_exe, and target_exe where  target_exe = compile(src_exe) either 
				src_apply(src_exe, input) == target_apply(target_exe, input) for all input values    OR   target_exe == null


**BOUND EXECUTABLE** - An executable is said to be "bound" if it has an 'lang' operator that returns the language in which the exe is expected to be executed.  This lang operator is optional, and it assumes some implied root language capable of executing the 'lang' operator in order to return the language object. Formally:
	*exe*.lang --> *lang*    where the returned *lang* must itself be a bound executable.

**BASE EXECUTABLE** - An executable language is said to be a "base" executable if it is its own language.  This simply indicates that this language is somehow natively executable without reference to any other lang.  Formally a lang is a base lang iff
	*lang*.lang --> *lang*    where the returned *lang* must itself be a bound executable.

NOTES:
- Implicitly a language's compile function defines a subset of all JSON values that may be passed as the first arg to its apply operator.  These are the expressions that have value interpretations according to this language---they are the values that do not map to null when passed to the compile operator.  
- A language can have a 'lang' operator that specified the language in which this interpreter itself is executed in.  Separate from this, a lang it make also have a a target_lang, which it the language that exe form are mapped into.  As an example one might have a compiler written in C code that implements some highlevel language like python into a more primative language, perhaps like a python exctuable



### UNIFORM SPACE

**NAMESPACE** - A *namespace* is a persistent is a map whose keys are restricted to be strings sometimes referred to as names.  The namespace's set operator will return a new namespace with the updated value.

**PATH** / **PATCH** / **LOAD** - 
- A *path* indicates a recursive traversal of a namespace as a list of keys, a single key, or a string of keys with embded "." separators.
- A *patch* is an ordered sequence of paths associated with sub-patches or values.  The path-value pairs of a patch are always listed in lexical order with the "." character sorting after all other key characters.  A patch can be expressed as a namespace or as a list of alternating path value entries.
- The *load* operator on map iterates thru the path value pairs of the patch applying setting each value as indicating.

### UNIFORM TIME

**WALLET** / **IDENTITY** - A namespace containing the currencies required to pay for operations (computation, bandwidth, and storage, etc.), as well as the credentials required for those operations.  Wallets-namespaces can be composed to aggregate resources available.  Each wallet serves as an an *identity* that may correspond to a person, a group of people, an authority that can be granted, etc.

**CHAIN** - A *chain* is an immutable, extendable sequence of values that is controlled by some wallet, that is called its authority.  Formally:
	chain.owner --> wallet								# the wallet that owns (pays for the existence of this chain)
	chain.writer -->											# the wallet permitted to append to this chain
	chain.reader --> wallet								# the wallet permitted to access the chain
	chain[i] 														# reads the ith value from the chain
	chain.append( value ) 								# appends a new value to the chain.



### UNIFORM UNIVERSE

**UNIVERSE** - A *universe*, U, is a versioned namespace of shared information.  U is recursively constructed from sub universes which are each collaboratively maintained via distributed control authorities.  Each universe has a time, a place, a substrate, and an authority:
- TIME - The timestamp on a sub universe is a UTC timestamp denoting when this version came to exist.  
  	By convention this is expressed as digits after the U symbol:   U . yyyy . mm . dd . hh . mm . ss . micro . nano ...        (Any prefix of these of digits is a valid timestamp)
- PLACE - The placement of a sub universe is a naming path that is relative to some containing universe via some sequence of symbols that position it in the larger universe.
  	By convention this place is indicated after the time digits as shown:  U . time-digits . place-naming-path
- SUBSTRATE - The substrate for a universe is the chain used to store the sequence of patches used to construct the universe
  	The U.substrate operator returns this defining chain.
- AUTHORITY - Each universe is controlled by a wallet that has permission the append to the chain that defines it.  
  	This is simply the wallet:  U.substrate.owner
- SNAPSHOT - Universes are versioned.  Snapshotting merges temporal versions into a single fully realized version.
  	U.snapshot --> fully realized universe
- CACHING - Universes are recursively composed.  Caching pulls some level of recursive data directly into the same substate as the universes root.
  	U.cache(level)
  	U.cache(list-of-paths)
- JURISDICTION - Each universe exists within some jurisdictional context that controls appropriate and allowable behavior within that universe. 
  	U.jurisdiction --> Jurisdiction object 					(see Collaboration section for discussion of Jurisdiction)


Using these three simplest notions we express software as a space (Universe) where actions (Exe) are applied to data objects (Values).


**ENV** - An evaluation environment (*env*) ties a wallet, universe, evaluator, and lang into a composite evaluator derived from these parts.  Formally:
	*env*.eval(*expr*)  -->  *env*.U[ *expr*[0]].apply( map(*env*.eval, *expr*) )		If *expr* is an args
	*env*.eval(*expr*)  -->  *expr*																			otherwise



### UNIFORM SEMANTICS


**UNIVERSE SEMANTICS**
- Each universe is fully defined by its underlying chain of patch values.  
- The sequence of patch entries are applied in lexical order first based on their timestamps and then based lexically on their place paths.
- This defines a finite tree of Namespace objects that either contain values, sub-namespace, or sub-universes.
- Each universe may reference other universes, but are otherwise separated from them, they don't share jurisdictions, semantics, nor updates.

Given some universe, U, with underlying chain, U.chain

Let UPDATES be the list of update triples derived from the union of all patches contained in the universe's underlying chain.  Each entry is a triple a timestamp as a list of numbers, a path as a list of keys, and a value that was stored by that entry.  The UPDATES list is lexically ordered according to the timestamp lists.  A special "null" update entry < [], [], null > is prepended to the UPDATES list for semantic completeness.

latest_entry_at(time, place) is entry1 = < time1, place, value1 >  where time1 <= time0 and 
			there does not exist any other entry2 = <time2, place, value2 > in UPDATES where time1 <= time2 <= time0.

value_at(time, place) is 
	value1    				if  time1 >= time2 else
	value2[ key ]		if value2 is Namespace object
	null						otherwise
	.
	where place = parent_place + [ key ]     for some parent_place and key, and
				< time1, place1, value1 >   =   last_entry_at(time, place)
				< time2, place2, value2 >   =   last_entry_at(time, parent_place)

value_at(time, []) is value1 where < time1, place1, value1 > = last_entry_at(time, place)



**CHAIN SEMANTICS**
- A chain is a persistent, extendable sequence of patch entries where each entry has a
  	watermark hash, timestamped, and a patch of values.
- To be valid, each timestamp must be prior to and near the UTC time it was appended to the chain.
- The timestamp of each patch-entry cannot be smaller than the previous patch-entry's timestamp.
- The hash on each entry combines the hash of the previous entry with the hash of the timestamp and hash of the ENTIRE universe structure realized from applying this latest patch.
- This approach ensures that any agent that has a reliable copy of the lastest watermark hash will be able to validate the correctness of a universe they construct from a potentially unreliable server of universe snapshots and unreliable server of chain patches to apply after that snapshot.
- This means that potentially only a sequence of water mark hashes need be stored on a traditional blockchain, the bulk of this chain data as well as snapshots and caches can be stored on S3 or any number of other untrusted but cheaper and high availability services.











### COLLABORATION

U0 - The U operator returns the universe of objects used in some compute environment.  Uniform.org supplies a time version chain of these U starting points called U0.  But other authorities could decide to also provide their own starting universe.  In any case U0 is recursively constructed from sub-universes that are controlled by other authorities.  Including the "wild-west" authority which accepts any sub-universe willing to pay the nominal one-time cost required to add their authority.

Controlled Universes -- Other than the wild-west most sub-authorities have some vetting process for controlling what can be added to their space.  Sub-authorities are pragmatically responsible for the contents of their universes, in some jurisdictions they are also legally liable for their contents.

Controlled Compute -- Universes may additionally contain compute as well as data.  That is they contain env objects that will perform computations when instructed.  Typically these compute resources are limited either in who has permission to use them and or what Exe / data forms they will process.  Just as with the controlled universes these controlled compute resources are the responsibility of their owners.

Controlled Storage -- Universes themselves are immutable, still they may contain Exe objects that provide access to mutable-data.  Just as with all other controlled objects propere usage is the responsibility of their owners.

Recursive responsibility -- Humanity contains bad actors, so there will be bad acting wallets generated bad bad data, storage and compute.  Thus it is the recursively responsibility of containing universes to maintain some levels pragmatic control over the universes use to construct their composite.  Specific policies, validation, and controls will vary depending upon the specific context, usage, and sensitivity.


#### Jurisdiction

A jurisdiction is a region that collectively operates underneath a single set of rules, laws, customs regarding appropriate, legal, or valid behavior.  This very human, societal concept applies quite naturally in the unicore context.  Here is how jurisdictions operate within Unicore:

- Each jurisdiction is itself a bound executable value.  This means the semantics of it operators are well defined and interpretable w/o further dependencies.

- Each jurisdiction places restrictions on appropriate/valid function of the universes that are controlled by the jurisdiction.  
- These restrictions are expressed provided in a number of forms:
	1. by laws written in natural text and interpreted by those who control the universe
	2. legal contracts as interpreted by relevant governmental that bind people or corporations that explicitly or implicitly accept these terms through their interaction with the universe itself.
	3. validation code that mechanically ensures certain validity conditions are by the universe.
	4. voting constraints that are mechanically verified based on signed voting of relevant community members.
- In addition to the four different kinds of jurisdictional rules as expressed above there are three different levels that these restrictions might be expressed:
	a. At the value level - The simplest rules can be assess a the level of each individual value assigned to the universe in isolation.
	b. At the universe level - The next level of rules require assessing a universe as a whole in one point in time.
	c. At the historical level - The final level of rules require assessing validity by looking at the entire history of a universe.


Here are some examples of jurisdictional rules of various kinds:

- 1a:  The text contained in the universe should not contain profane language.  
  		This #1 rule since it is expressed in natural language it must be interpreted by people, and is not expressed as a legal requirement.
		It is level A since it can be validated one value at a time without looking at the universe as a whole.
- 2a:	The contents of the universe is the intellectual property of Acme Corp and protected by the following NDA.
  		This is a #2 type rule since is is a legally binding contractual term
- 3a:	Code that verifies that the number 777 is never written into the universe anywhere explicitly.
  		This is a #3 type rule since it specified a computable validation function to be checked before accepting any extension of the universe.
- 4a:	A rule that at least 2 of the listed senior developer contributors have viewed and accepted a specified update to a universe contained come codebase.

Here are some example of different levels of rules, all expressed as #3 computable jurisdictional rules:

- 3b:	A universe may not contain more than 20 megabytes of uncompressed JSON values.
  		This is a computable function, but in cannot be assessed one value at a time, but it must be assessed by looking at one snapshot of the whole universe.
- 3c:	A universe may never delete or change an assigned value over time, but instead must only aggregate values at the leaves.
  		This rule is also a computable function, but it cannot be assessed by looking at one snapshot of a universe.  Instead it must be assessed by looking at the whole history of the universe.

Of course all twelve combinatorial types of jurisdictional rules are possible, we only listed a few combinations here to give a flavor of these rules.

Formally a jurisdiction, j, is an executable namespace with the following fields and operators:

- j.textual_rules --> string
- j.invalid_value(patch) --> concern				Returns null if there is no issue, or a string describing the issue with a given value.
- j.invalid_universe(u) --> concern				Returns null if there is no issue, or a string describing the issue with a given universe snapshot.
- j.invalid_history(patch, state) --> concern		Returns a concern string if there is an issue, otherwise it returns the state object to pass for the next historical check.
- j.default_env --> env							Returns the default interpretation environment to be used for any unbound exe within the universe.

NOTES: 
- Historical checking is performed via a "reduce" like operator as shown above so that it can potentially be checked incrementally as the chain is extended over time.  The first historical check use null as the input state for the first invalid_history call.







sub-space of values that are collectively 



### FUNCTORS
Functors are used to transform mappings into new mappings.  Each Exe object maps input args into an output result.  Thus functors serve as Exe transformers.


**LOAD** - The *load* functor transform one Exe mapping into another of the same kind, but with some kind of enhanced functionality.
- namespace.load(new_bindings) --> new_namespace			-- this is just structure composition
- lang.load(new_operators) --> new_language					-- these are unimplemented languages
- env.load(sub_env) --> new_env								-- combines implemented languages into a composite env
- env.load(sub_lang) --> new_env								-- adding unimplemented languages into an env
- wallet.load(sub_resources) --> new_wallet
- universe.load(sub_components) --> new_universe
- jursidiction.load(sub_rules) --> new_jursidiction


MAP - The map functor wraps a base exe into an one that will apply the base exe to the elements of a map object.
- The first arg specifies which elements and what info should be mapped
- The second arg specifies the base exe to be mapped
- The optional third arg specifies the target to be mapped (which causes the functors to be immediately executed instead of being returned)
- How:  args, argpairs, keys, keypairs, fixed, fixedpairs 
- Iterator form:	container.map(how, exe)			my_list.map(argpairs, exe)
- Functor form:	map(how, exe)					f = map(keys, exe)



### UNFORM STARTING OBJECTS

- ENV0 - A least common denominator base env for the system.  It has all the operators specified here

- U - is the immutable universe of objects used to construct ENV0.  Uniform.org supplied a time version chain of these U starting points.  But other authorities could decide to also provide their own starting universe.  In any case

- NS - The empty namespace, used to construct other namespaces.

- S - Are the available mutable storage locations.  Many are URL-based, externally-supplied and manipulated values.  Base env supports:
	- http, https, s3, dropbox, gdrive, github
	- file - Local filesystem access for this base env  (optional)
	- cached - Local cache for this env (that may continue to exist)
	- tmp - local data store deleted upon environment termination

- DO - Available base compute platforms:
	- do.env0		- this compute platform
	- do.js0		- locally executed restricted from of Javascript consistent w/ many browsers and node.js

	OPTIONALLY PROVIDED IN SOME ENV
	- do.dom		- locally executed js with browser dom access
	- do.node		- locally executed node.js Exe
	- do.python	- locally executed python 3 Exe
	- do.lambda	- remote aws lamba compute platform
	- do.docker	- local

- IO - providing print / log / error message outputs. and dialog inputs.
	- io.out(message_string, level)      0=logging, 1=warning, 2=error, 3=fatal error, ...      -1 or omitted level indicates printed output
	- io.input(dialog_message)

- WALLET - 
	- usd / btc / eth / amazon credits





#### U.web.protocol.url
- U.web.http...
- U.web.https...
- U.web.git.*USERNAME*.*PROJECT*

#### U.pub
- U.pub.*ORGNAME*...

#### U.lang
- U.lang.py
- U.lang.py311
- U.lang.js

#### U.io
- U.io.in			-- Console input
- U.io.out			-- Console out
- U.io.err			-- Console error messages
- U.io.dom		-- Local browser dom

#### U.env
- U.env.py0		-- 


## VERSION 0

JS browser based system executable
	- DOM control
	- gdrive storage connector
	- YAML reader
	- AWS lambda connector

Platform provider
	- One or more AWS IAM accounts running instances

Payment processing
	- Accept USD in but not out.  only to pay server fees, and into the unicoin treasury



## API



### MAP
#### - get(k)
#### - set(k, v)
#### - len()
#### - itr(exe)
#### - met([k]  [, v])
#### - head()
#### .

### NAMESPACE
#### - PathSpec
#### - Patch
#### - load(patch)
#### .

### UNIVERSE
#### head(obj)
#### meta([key]  [, value])
#### .



### ENV
#### - Env
#### - apply(exe, args)

## FIRST USE CASES

### LOW FRICTION PAY




# JUNK

FUNCTORS



IMPLEMENTED vs UNIMPLEMENTED - 


**IMPLEMENTED** LANG / **BASE** LANG - A lang is said to be "implemented" if it has an apply operator that will directly execute its Exe forms, and a "base" lang is an implemented lang that compiles to itself.  Formally:




**Exe**  -- Exe is a JSON value that intended to be "executed" -- that is be passed as the first argument to the special APPLY directive.  The APPLY directive accepts a Exe value and an args value and derives a result value as shown:  
		APPLY[ code_value, intput_args ] --> output_value
By convention we refer to the head of the input_args as the "method invoked by the api", and say that code, C, implements implements method, M, if the APPLY directive will inputs of the form:  APPLY[ C, M(...) ]




APIs built into the null env

NULL ENV
- WEB
	- S3.get(uri) -
- CHAIN_ID:  load(utxo), 
- ID
	- BTC_ID spend(utxo, instruction)
	- AWS_ID 



DAP - API
- STORAGE - immutable, semi-permanent storage.  e.g. filecoin, aws, 
- STATE - progression of immutable states (Chain)
- BANDWIDTH - read write access to storage and state
- COMPUTE - aws-lambda

- LOOKUP - Translation from a unicode name string onto json containing URLs based on approved protocol prefixes
- COMPOSE - 



#### BASIC FUNCTIONING

ACCESS POINT 
An access point maintains a set of composable APIs (Functors) that are used to construct the environment needed for some computational step

ARGS
- A JSON map. 0 = indi1, ..., n are the fix args, 

AN API
- A JSON map, with the special 0 key specifying how it should be applied.

APPLY(api, args) -> result
- An operation over APIs 

- APIs
- System as 

