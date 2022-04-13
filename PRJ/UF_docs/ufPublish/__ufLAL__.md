# ### LAL - the Langauge Authoring Language
## === LAL.PKG ===

#### notes
#### -- 2021-07-13

- Pkg instructions are executed in order
- pkg instructions 'follow' specified place to determine where to operate
- pkg instructions perform operation based on underlying data type at target
- instantiation of pkg template is performed at load time, and will generate new pkg cmds that need to be executed also at load time


idea is that 

ACTIONS			scalar		set			list		map
set		=		
define	:=
append	+=		math		union		append		update k:v
del		-=		math		intersect	del	_[i]	del _[k]


place := form			# defines place (error on redefine)
place -= key			# removes matching edge
place += form			# appends place (place could be set, list, or map)
place += (key, form)	??
place += key < form		# insert before
place += key > form		# insert after




### older notes
	U._"+"_(Spec)						// Includes an element if not in package
	U._"-"_(Matcher)					// Removes any elements that match spec
	U._"<"_(Spec, Matcher)				// Adds an ele before another  (CONFLICT!)
	U._">"_(Spec, Matcher)				// Adds an element after another
	U._":"_(Ident, Spec)				// Adds a key/value pair to package
	I._"..."_							// Indicates contatenation not append


|  value 					// appends value
|  "+" value	  			// appends value if it is not already in pkg contents
|  "-" value 				// removes entry by value
|  "=" pos 					// adds a special value used as scaffolding but not included in final pkg contents
|  pos ">" value 			// adds value immediately after pos in collection
|  pos "<" value            // adds value immediately before pos in collection



|  key "=" value			// appends key/value, or overwrite existing k/v
|  key "-" 					// ??? removes entry by key
