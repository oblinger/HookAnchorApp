

## 2025-10-15  Simple Share Model

JSON -- A JSON data value.

VALUE -- A possibly dynamic, possibly infinite JSON data value where object values may be typed using the " _ " key.

CONTEXT -- A Context Is simply a mapping from dotted names onto values.  It may be expressed as a value like this:
  {"_":"Context", "dotted.key.one": { sub tree here}, "dotted.key.two": "a constant value", ... }

ENVIRONMENT -- A thing that can take actions and perform computation.  An environment fully specifies all information required for execution, including:
- The language for execution, the current state of execution, and a permissions model controlling allowable actions.

NULL ENVIRONMENT -- The execution environment from which all other environments are constructed.
- It has the required values and operators needed to build other context objects.


**NULL TYPES** -- The list of types supported in the null environment
JSON TYPES:  'int', 'str', 'bool', 'list', 'map'
CODE TYPES: 'Operator', 'Node', 'Context', 'Environment'

**NULL ENVIRONMENT**
state.*	--  The current execution environment
lang.* 	--  The vocabulary of operators and artifacts available within this context
u.* 		--  The universe of contexts, artifacts, and data that is available in this environment
io.*		--  These I/O objects facilitate communication between this execution environment and elsewhere


**NULL LANGUAGE OPERATORS**
lang.context	--  Operators for context objects.
.	merge			--  Recursively combines contexts
lang.node		--  Operators for values (works on VALUES, list, map, and scalar):  type, len, keys, get, set.
lang.op			--  Operators for operator type objects
lang.env			--  Operators on environment objects.  execute, 
.	execute		--  Executes some specified code object using the specified lang, env, and permit models.
lang.validate	-- 
lang.permit		--  Permissions / Legislation model
lang.version	--  The dotted name and version of this execution language.  By convention, it should begin "lang. ..."


EXPECTED STATE STRUCTURE
state.code		--  The default code to execute in this environment if none is specified.

