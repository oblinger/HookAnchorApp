# UNIFORM COMPUTATION
## EQUALITY / IDENTITY / HASH / CONTENTS

HASH -- A hash is a large number derived 
... such that equal ...
_
## OPTIONS

Specifies
- ARGS -- As args directly on a function
- LEX -- Lexically
- DYNAMICALLY -- In place space
- 
# UNIFORM COLLABORATION
## BLOCKCHAIN

CHOICE: how do you choose the lang that covers a BC?  RESOLVE: it is the language of the placement of the BC.
BC is just memozid content, MEMOIZED


**BLOCKCHAIN** -- A _**blockchain**_ is a sequence of blocks and states which are connected by some underlying update function.  Specifically, state[i+1] = update(state[i], block[i]) and state[0] = VOID

**HASH** -- Just as with all uniform strucutures, both blocks and states have an associated hash value, which we refer to here as block hash and state hash.  (These are unchained values and only refer to the single entity.)
 
**BLOCKS HASH / CHAIN HASH** -- The _**blocks hash**_ is a hash dervied from the sequence of all block hashes, and the chain hash is a hash derive from the sequence of block and state hashes.  Formally, blocks_hash[-1] = chain_hash[-1] = 0, and:
	blocks_hash[i] = hash(blocks_hash[i-1], block[i].hash())
	chain_hash[i] = hash(chain_hash[i-1], block[i].hash(), state[i].hash())





KEY SPECIALIZATIONS
- A _**transactional**_ chain has blocks, where each block is a unit of transactions, so
	block[i] = { transaction[i, j] }
	and update(S, B) = trans_update(B(n), ... trans_update(B(2), trans_update(B(1), S))
- Each block[i] and state[i] has a hash value 

_
### API

Blockchain:
	state: slot(t:State)
	states: slot(op(->List(State))
	blocks: slot(t:List(Block))
	blocks_hash: slot(op(->Hash))
	update: slot(t:Fn(State, Block, -> State))

State:
	^chain_index: slot(t: Hash)

	
_
## MOUNT

**MOUNT** -- A _**mount**_ is possibly live, possibly derived data from some other place embedded as the contents of sub-tree of placespace. 

some possibly live external data that is embedded as a sub-tree within a place space.
**MOUNT** -- A _**mount**_ is some possibly live external data that is embedded as a sub-tree within a place space.

### examples and thinking

mount files --source--			// results in tree of file units  (local_folder/dropbox/.../http/...)
mount text  --source--			// same but filters and expresses tree of content
_
## REPO 

**REPO** -- A _**repository (REPO)**_ is a searchable store of content -- a unit providing Place persistence and a 'select' micro language.

**SELECT** -- A micro language for deriving ordered views of selected subsets of the repo's contents.

As a default the base select operator selects based on item keys with reverse recency ordering
_
## ENGINE / RULE ENGINE

**ENGINE** -- A uniform _**engine**_ is an interpretation system that once instantiated will accept data items (called commands) and will performed the intepretation actions indicated by each of those commands, possibly generating outputs and/or a results from the command interpretation.

ACCEPT -- An engine must have an accept operator that initiates command 

COMMAND -- An engine command is any data item that can be

ORDERING -- Determines the order of interpretation of the accepted but not yet executed commands

~-~~

**RULE ENGINE** -- A _**rule engine**_ is a form of interpretation that is comprised of rules that specify some procedural action to be performed upon satisfaction of some trigger condition that typically occurs when some underlying data (the knowlege base) changes.

**KNOWLEGE BASE / ASSERTION** -- A _**knowlege base**_ is a collection of data elements (called _**assertions**_).

**RULE / TRIGGER / ACTION** -- A _**rule**_ is the combination of a _**trigger condition**_ and a _**procedural action**_.


~-~~

TREE ENGINE -- A tree engine 


_
### API

RuleEngine

### IMPL NOTES

Tree Rule Engine Implementation.


Scope -- The root place of a subtree

Trigger -- A predicate of place
TriggerHead -- Optionally a trigger is limited to units with a specific head then this is the trigger head.

Rule Antecedent -- a list of scope-trigger pairs, and a predicate accepting a list of matching places for all pairs.
Rule Action -- an operator accepting a slit of matching places for all pairs.

Update -- A place that has been changed, (which is an implicit change for every place above it)


TriggerLists -- A sparse map of places and associated list of triggers with the list.


TreeDB:
	Partition:
		type: List[Partition || TriggerHeads]  // List is in pre-order 
		root: Place
	TriggerPairs:
		type: Path => Trigger	
	TriggerPair:
		predicate: Predicate[Place]
		scope: Place

ALG:
	- Given an update, U, at some place in the DB
	- Find the lowest containing partition, CP
	- Scan for subset of Trigger pairs in scope for this place.
	- If ratio is large enough then extend the partition tree down to have a partition BEFORE root, IN this root, and AFTER this root.
	- Run all triggered scopes thru the rete-net to determine triggered rules.











## VERSION

**VERSION** -- A _**version**_ is one of an enumerated set of alternative artefacts intended for some expressed common purpose.

VERSION SPACE --

VERSION DIMENSIONS -- 
- OBJECTIVE -- Each version is intended for some announced objective.
- AUTHOR -- 
- COHESION

DIMENSIONS
- WHO	Control	-- The person or group that authored and has control the future progression of a version
- WHAT	Topic	-- Concept area covered
- WHERE	Target	-- Location of the data contents of the version
- WHEN	Version	-- Date / version of the version
- WHY	Goals	-- Purpose of system
- SIZE			-- How high in the telescoping 
- PART			-- When within a part decomposition is the pkg

- CTX			--  



_
### VERSION.PATH EMBEDDING
_
# UNIFORM MARKETS
