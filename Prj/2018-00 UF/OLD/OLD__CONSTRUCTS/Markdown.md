
### Parsing Code

parse_expr (parses down to ':' and statement forms as well as ',' forms)

parse_region (adds SEQ upgrades to parse_expr)
- used internally for upgrading delimited values

__ parse __
	- __ place __ -- index of start of punctuation & reference to text unit
	- computed beginning of form (start of left most descendent), or beginning of atom if leaf
	- computed end of form (end of right most descendent), or end of atom if leaf


### Markdown Rules

MODE		INIT	COLON		COLON PARSING						PAIR-PARSING
yamaic		;;		key-val		\n multi-word ':' indented-expr		foo-: bar    ":"(foo, bar)   
pythonic	::		sub-blk		stmt-prefix ':' sub-block			foo-: bar
javaic		{		key-val		expr ':' expr


foo := bar


VALUE -- this is the default parsing mode
- Parses value as an expression.
- Colon is parsed as a pair as part of the expression.
- Multiple values w/o any separator are upgraded to a SEQ 
- Top Comma or Semicolon head is removed (if existing) and body is unit body

DATA --
- Each line is parsed upto the colon and stripped.  This is the key.
- The entire indented text is the value, and is parsed according to data-body form:
	- line breaks and ',' implicitly separate values.  
	- if more than one value then values are parsed into a SEQ.

STMT -- 
- Before the colon is the prefix which is converted into a list.
- After the colon is processed as a sub-block with data-body parsing.

BLOCK -- 
- Before the brace is the prefix which is converted into a list.
- The colon is parsed as a pair

#### Parsing syntax modes
- infix -- prefix or infix punctuation by precedence
- region -- delimited multi-line textual region -- parses until end marker (which can be indention)
- line -- kind of region, but may only be on a single line
- unit -- kind of region, but parses region as unit(s) -- with seq upgrade

##### seq/fn upgrade

- ident = string or symbol atom
- indent region is rewritten as head(body) INSIDE SEQ BODY
- indent rest is rewritten as head(rest) at top level

foo(x, y)
foo"string body"






##### rewrite rules

- comment removal -- somehow comments are stripped from form
- 


### Examples


// YAMAIC FORM
foo;;
	= first value
	= second value
	a simple key: "and value"
	another key:
		= with 
		= list of 
		= values

		
// PYTHONIC FORM
foo::
	def op f(x Int, ** y Int=0, ->Str):
		if x<y:
			return "less"
		else:
			return "more"



// List form

["`head", "'x", "'y", 17, ":foo", ["simple", "key"], ":", ["complex", "key"], "value"]

String Prefix
  `		means named symbol
  ^ 	means meta symbol ????
  ' 	means string value
  : 	means head or keyword symbol
  : 	by itself means complex key value


