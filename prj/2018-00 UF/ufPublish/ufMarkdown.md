
(See 2025 ufMarkdown.md)

[HTML docs](../../markdown/markdown.html) 

# ### INTRODUCTION ATTEMPTS ###
### --- CODE MARKDOWN UNIFORM AGENDA INTRO ---

The Uniform Agenda is to iteratively edit a ladder of paradigms to "have it all" and over time to "see where you land".  Here is the current formulation of "Uniform Code Markdown" after many years of iteration:

Our aim with Uniform Code Markdown (UCM) is to simultaneously achieve the following:

Aspires towards a collection of elements to good they cannot be made better for their intended purpose.  In the case of UCM, my aim was to write a best language writing language syntax.  On that:

1. HOMOICONIC -- Supports the meta programming that is central to the language writing require in building this web of paradigms.  Ideally it is homoiconic, have a simple, LISP-reader-like semantics-free mapping from text onto AST -- so the meta programmer can effortlessly program that AST.
2. BEAUTIFUL -- Is as apt, and looks as beautiful as any other syntax developed for any  DSLs developed for any targeted software activity.
3. COMPONENTIAL -- Facilitates seemlessly interspersing logically meaningful fragments of independently developed languages into new synthesis languages without changing the original languages or writing a new parser for the new language.

~-~~
As a motivating example imagine: one person developed a jQuery-like micro language (paradigm) for sub-tree selection, independently a second another person developed a PHP-like template-structure-filling paradigm, and a third

.  Later it should be possible for a third person to easily combine these paradigms to produce a DSL for specifying dynamic structures  might easily construct 

a parallel flow language designed to efficiently transpile onto Google's Tensor Processor it would be easy for a forth person to the source-code syntax of each of these langauges 
~-~~

To even attempt such a crazy thing, one needs to identify key elements of software which can be repeatedly recombined in order to admit a close proxy for all existing (and future) software languages.  So for SW language parsing what are these conserved elements?

- PARSE TREE --
- TOKENIZATION -- 
- PRECEDENCE PARSING --
- BRACKET PARSING -- 
- COMPOSITION PARSING -- 
- HUMANITY'S TOKEN LIST -- 

- SCAFFOLDING -- 
	- PAREN SCAFFOLDING
	- SEPARATOR SCAFFOLDING

- STATEMENTS and BLOCKS -- Code expressions are naturally recurive structure, the statement-block paradigm places a specific restriction on this rescurive structure:  Code blocks are containers of code statements.  And while code statements may recursively contain other code expressions they may not directly contain other statements, though they may contain code blocks (which themselves can contain code statements).


Ok, so these all sound pretty generic.  Not all languages use all these ideas, but those that do, seem to fit the terms as defined pretty well.  Now how do we get a language from this?!?





_
### ...
should have python3.8 in path
and also the oblinger local 3.8 in path too


python3 -m markdown -x codehilite test.md > body.html
 pygmentize -S default -f html > codehilite.css
 https://sebastianraschka.com/Articles/2014_markdown_syntax_color.html


_
### ,

Uniform Syntax is to code what YAML is for lists and maps -- a simple semantics-free format that supports a couple of alternatives allowing human-beautiful presentation of a variety of data.  

This "code markdown" is a source code format that has no reserved words thus no syntax specialized for any particular language, instead it includes _ALL_ the major structuring techniques used in modern languages woven into a single whole.  Just list LISP is lists all the way down, Uniform uses the ``uni''-form all the way down.  But Uniform allows:

_
### --- Intro ---

CODE MARKDOWN

The simplicity and schema-free nature of both Markdown and YAML have driven their widespread adoption:  Markdown for natural text, and YAML for structured data.  So why couldn't there be a markdown for source code?  What would that look like?  And what would it be good for?

There can be.  Here we present Uniform Syntax, a markdown for code.  Just as with YAML or Text Markdown it:
- It is a simple format having only a few parsing rules that are use for all langauges.
- Is is schema-free, the same parsing rules work the same way for all languages.
- It has a couple different textual alternatives designed to capture the major different ways that people tend to render certain types of information.  (e.g. like the different ways to denote a header title in markdown, or the different ways to show a string in YAML, or indention-based or brace-based nesting in source code)


Ok, but what is the for??  It allows us to treat source code as easily manipulated data:
- Software configuration and business logic should both allow a full programming language, but they are both treated as data whose internal structure needs to be access and manipulated.
- A very powerful approach is to develop a DSL specific to the application, but can be heavy


~-~
 
Eric Horowitz said that all the cool ideas from LISP have made it into modern languages except its macro system.  He is right.  The problem is that the macro system relies on the fact that LISP uses a semantics-free "reader" that maps from text onto the list-of-lists which is used as the AST for the program structure.  This is amazingly powerful since you can invent any new behavior for your language using macros, and then parser (which is sematantics-free will work perfectly).  BUT visually LISP sucks eggs.  Modern languages (except clojure) have not gone down the parenteses path, even knowing that they were going to loose all that macro goodness.

But does it need to be this way?  Could there be a "code markdown" analgous to "text markdown" -- a reader for source-code that is designed to look as good as modern languges do while also being semantics-free language-independent?  _Surprisingly it turns out this is possible._  It has taken nearly a decade of screwing around to dodge the conflicts that traditional languages dodge by using reserved words and special syntax for to solve, but it is possible.  The "Uniform" code markdown shown here allows for a semantics-free reader like LISP:
- It maps all text onto a recursive structure built from a single "_uni_" form (called a _**unit**_)
- It has no reserved words in its parsing, it uses the same uniform parsing all the way down
- It combines _**ALL**_ of the major parsing paradigms into a single parser:
		prcedence parsing, brace parsing, indention parsing, composition, scaffolding, and tokenization
- It just looks darn beautiful since it lets the code's author choose the parsing paradigm most attractive for each part of the code


So what are the parsing paradigms that are woven into code markdown:
	
	1 + 2 * 3			_precedence_ 	parsing so '*' comes before '+' 
									  just as 5th grader would expect
									  
	if x<10 {print x}	_brace_ 		parsing so this is structured just as expected
	if x<10:			_indention_ 	parsing so indention 'braces' operate as expected
		print x
		
	def f(x: list int)	_composition_	parsing so 'list' and 'int' are compose here
	(1+2)*3    2,-3		_scaffold_		parsing to reorder the parse tree w/o adding to the parse tree
											(e.g.  "(...)" for grouping and " , " for splitting)
	x <-- ?y << z		_tokenization_  parsing text into: tokens, whitespace and terminals


And what is this uni- unit form?  It looks like a function call with a mixture of po

	head "(" arg "," key )


Ok enough theory... SHOW ME SOME CODE!









~-~
POST PROCESSING
Ok so we did not do away with language semantics; admittedly we just pushed that bubble around under the carpet.  But we pushed it to a place that allows language-independent 

In order to give a langauge its semantics one must apply a few rewrite rules to this unit-tree so that its structure reflects the semantics of ones language, but this is ok because this is done AFTER parsing during the macro processing step.  This is the key trick in uniform, by pushing semantics outside of the parser one is able to really treat CODE as DATA.
- pass code around, combine it, and manipulate it as DATA without knowing which language it is part of.  This is critical since complex systems have configuration and business logic composed multiple different extensible sub-languages.  


_
### --- INTRODUCTION ---

Uniform Parsing is broken into three spirals of increasing specificity:
- Rung One provides a simple framework covering all parsing (interval trees).
- Rung Two provides mix and match set of parsing methods.
- Rung Three uses rung two to define Uniform Markdown, a semantics free code form designed to cover a very wide range of domain specific languages with a single parse form.

OVERVIEW
- Spiral one frames all parsing as creation of interval trees.
- Spiral two expresses parsing as a combination of: Token, Precedence, Bracket, and Composition Parsing, along with normalizing rewrite rules and scaffolding.
- Spiral three defines Uniform Markdown 


Uniform is a simple, semantics-free, universal, markdown for code-like languages:
(1) Their parsing rules are simple and fixed.  
(2) Their parsing is semantics-free the parse rules do not vary depend upon the meaning or intent of the data.
(3) They are universal in the sense that they are intended to apply to all forms of data.
(4) They are markdown, meaning they are intended to be easily understood by people, and support a low ratio of punctuation
    verses content characters.

Because of their advantages these markdown data formats have are supplanting non-markdown file data forms.
In a parallel way, could one have a code form that was a simple, semantics-free, universal, markdown format for code?
What might that look like, and what advantages might it convey?
- In meta-programming, code-sharing, and many configuration languages one needs to express code as data, and have some 
  format for operating on code in this way.  These are the applications for which Uniform Markdown is intended.
  

UNIFORM MARKDOWN PARSING

In uniform we strive for constructs that accommodate the widest range of diversity through simplification,
generalization, and parameterization.  Uniform markdown strives to do this for text parsing of code-like forms.  
Here are its constructs:
# ### UNIFORM MARKDOWN - RUNG 1 - PARSING ###
## === PARSE TREE ===

This single construct provides the structure for all uniform parsing.

**TEXT PARSER** -- A _**text parser**_ accepts a _source text string_ and computes an interval tree as the resulting output of "parsing" the source text string.

**INTERVAL TREE** -- A an _**interval tree**_ is a listy tree of unit intervals, where:  
1.	**STRING INTERVAL** -- Each unit specifies a starting (inclusive) and ending (exclusive) indicies into the source text string.
2.	**CONTENT** -- The content of a parse tree unit is the subsequence of characters indicated by its starting and ending indicies.
3.	**CHILDREN** -- The children of each unit interval are the subintervals that perfectly partition the parent's content into a set of substrings.
4.	**ORDERING** -- The ordering of the children preserve the ordering of the intervals in the parent, so forall nodes: parent.contents = child_1.contents + ... + child_n.contents


_
# ### UNIFORM MARKDOWN RUNG 2 - PARSING PARADIGMS ###
## intro


Uniform Markdown Rung Two (Parsing Paradigms) formulates four extremely commong parsing paradigms, as well as a 'combo-parsing' paradigm for combining them:

- **TOKENIZATION PARSING** -- Parses an interval into a flat sequence of subintervals of three kinds: token, terminal, whitespace

- **PRECIDENCE PARSING** -- Token based parsing that optionally merges intervals before and after token into a prefix, infix, or suffix parse.  Interval merging repeats in an order controlled by a partial ordering defined over the tokens themselves.

- **BRACKET PARSING** -- Token based parsing that brackets interval being parsed between an initating and terminating indicators. 

- **COMPOSITIONAL PARSING** -- Parses interval by repeatedly calling some underlying parser on the interval until it is fully consumed, resulting in a flat list of sub intervals.

- **COMBO PARSING** -- Combines the four rung two parsing paradigms into a single parsing technique controlled by a partial order defined over a set of typed tokens.

_
## --- TOKEN PARSING ---

**TOKEN PARSING** -- a _**token parser**_ is a text parser that returns a flat interval tree where each sub-interval is either a: token interval, terminal interval, or whitespace interval.  Tokens are taken from a fixed list of token strings which are typically matched left to right with longer token strings matching before shorter strings are matched.


### ,

Tokenization partitions a text string into a flat list of leaf intervals.
- INTERVAL TYPES     --  There are three types of leaf intervals:
 1. WHITESPACE     --  an interval composed entirely of whitespace characters.
 2. TOKEN          --  an interval whose characters match on of the predefined non-whitespace token strings.
 3. TERMINAL       --  an interval of characters that don't include whitespace characters, nor token substrings.
- INCREMENTAL ALG    --  Tokens can be extracted via an incremental algorithm:
  - Sort tokens from shortest to longest, and maintain an index of the first unparsed char in string.
  - Find first token which matches the prefix of the unparsed text
  - If not token matches, then scan unparsed text until matching prefix is found, return scanned text as a terminal 	   interval, and update the index of first unparsed char
  - If token match is found, then return that token and update the index of first unparsed char
  - If end of parse region is reached, then return the special EOF token.
## --- BRACKET PARSING ---

**BRACKET PARSER** -- a _**bracket parser**_ is a text parser that uses a specific collection of open-bracket tokens to identify a text interval what will be recursively parsed.  Bracket parsers always use an enumerated set of bracket initiation token, but then may use a variety of means for identifying the end of the bracketed region.

Metadata associated with the bracket initiation token is used to control bracket parsing.  This meta data indicates:
- INITIATION TOKEN -- 
- TERMINATION INFO -- The kind of terminator this bracket form has (Unform Markdown defines four kinds.)
- BODY PARSING -- 

**TERMINATOR TYPES**    	--  Uniform markdown defines four methods for determining the end of a bracked interval:
1. **SAME TOKEN** -- If terminator is the same as the initiator, then body it treated as raw characters with only escape processing.
2. **MATCHING TOKEN** --  If terminator differs, recursive parsing is performed, and bracket form is terminated by the specified "terminator" token.
 3. **END_OF_LINE**     --  The bracket is terminated by the end of the text line character of the text line that contains the initiator token.
  (4) END_OF_INDENT   --  The bracket is terminated by the end of the text region that has indention deeper than the 
                          indention of the line containing the initiator token.
  (5) SEPARATOR       --  The bracket is terminated by a SEPARATOR token.  Certain tokens are designated as separator
                          tokens, these serve this termination role.  Unlike other bracket terminators these separator 
                          tokens are not "consumed" -- that is, they are not included as part of the parsed bracket interval.
                            
**BODY PARSE TYPE**     --  Body parse type determines how the body of a bracketed for is to be recursively processed.  Three types:
  (1) PARSED          --  The body interval is recursively parsed as if it were an independent top-level string using the same parser.exclude used for the overall text.
  (2) RAW             --  The body interval is not parsed at all, but rather included into the parse tree as a single leaf node 
                          containing the unparsed chars in the underlying text string upto the terminator index.
  (3) EXCISED         --  The body is parsed just as with RAW parsing, but additionally the entire bracketed form is NOT 
                          included into the parse tree, it is completely excised, and instead merely serves as a comment included in
                          the original text as a form of documentation of that text.




Bracketing use file signals to indicate the beginning and ending of a parse interval.  By recursively parsing the 
body of a bracketed interval one can build a nested tree of parse intervals.
- KINDS OF BRACKETING --  There are 12 bracket variants:  all combinations of four TERMINATOR-TYPES and three BODY-PARSE-TYPES.  
- BRACKET STRUCTURE   --  Brackets always have an initiator interval, a body interval (broken into zero or more child intervals), and possibly a terminator interval.
- BRACKET BODY        --  The bracket BODY is the interval of characters beginning after the initiator and upto but not including any termination characters. 
- TOKENIZATION USAGE  --  Bracketing builds upon a combination of the token parsing of the underlying text as well as by directly accessing the RAW unparsed intervals of that text. 
- INITIATOR TOKEN     --  Bracketing uses an initiator token to signal the existence of and start of a "bracket" form.

_
## --- PRECEDENCE PARSING ---

**PRECIDENCE PARSER** -- A _**precidence parser**_ is a text parser that uses a total ordering over a set of token in order to compute an infix-style parse using this ordered set of tokens.

EXTENSIONS:
- allow left, right default binding
- allow tokens to preclude left, right, or ambiguous parsing


Precedence based parsing uses a partial order defined over all tokens to determine how intervals delineated by those 
tokens will combine.  
- Background:
  - A PRECEDENCE partial ordering is defined over all tokens with terminal intervals having the highest precedence.
- Basic Algorithm:
  - TOKENIZATION -- The interval to be parsed is expressed as sequence of token and terminal intervals with all whitespace intervals removed.
  - MERGING -- Intervals are repeatedly merged into composite intervals starting with those with highest precedence first.
   - HIGHEST FIRST -- An interval is merged with the one before and after it when they are both have higher precedence, 
     or the one after it has the same precedence. (left to right binding).
   - OPERATOR TREE -- The merged interval has the precedence of its merging interval, and in the parse tree it is 
     represented as a parse tree node with its merging interval as its head.
  - SPECIAL CASES
   - NARY MERGING -- Any operator interval whose head matches the head of its subsequent interval is merged with that 
     interval to create an nary operator interval.
   - GAP TOKENS -- If an interval can only bind with one of its neighbors but not the other, then the special GAP 
     token is added to the parse tree to indicate this fact.
   - SEQ COMPOSITION -- If a terminal is merged with a subsequent token the special SEQ token is used as the head of 
     the implicit composition operator that merged these two intervals together.
   - BRACKETED INTERVALS -- Bracketed intervals have the precedence of their initiator token and the entirety of 
     their body characters are treated as if they are part of that initiator token for purposes of precedence parsing.
     
_
## --- COMPOSITION PARSED INTERVALS ---
Composition is a form of parsing so simple it almost does not count as parsing.  It is called composition as it
originates with a form of interpretation where each subsequent form is applied (composed) with the previous form.
- DEPENDENCE --  Composition relies on some other more basic form of parsing or tokenization.
- ALGORITHM  --  Composition simply appends the result of repeatedly invoking this sub-parser.exclude to parse a text string as a flat SEQUENCE of independently parsed sub-intervals.
- RESULT     --  The result is a 'SEQ' (sequence) parse node with children for each sub-parse
## --- COMBO PARSING ---
COMBO PARSING -- Combo-parsing integrates all four spiral-two parsing constructs into a single framework. 
The four are defined and implementable independently from each other, but they are also are framed in a way 
specifically designed to allows them to cleanly interoperate.  
- COMMON TOKENS -- Combo-parsing uses common set of token definitions to underlie all four methods, and associates metadata with these tokens in order arbitrate between the four parsing modes.
- TOKENIZATION BASED DISPATCHING -- Tokenization applies directly to any unparsed interval of text, is implemented as an incremental algorithm so one can parse and index off "the next token" as a way of selecting among the parsing modes.
- BRACKET EXTRACTION --  A Bracketed form always begins with an initiator token.  Thus the tokenized token itself
  indicates the subsequent text is to be controlled by bracket parsing upto its terminator index.  
- RECURSIVE COMBO PARSING -- Depending on metadata associated with the initiator token, combo parsing may be 
  recursively applied to the bracket's body.
- MERGING PRECEDENT PARSING
  - PRECEDENCE PARSING -- At each level within the tree of nested bracket parses one can perform precedence parsing.
  - PRECEDENCE PARSING OF BRACKET INTERVALS --   Each bracket interval can be replaced by an interval only containing 
   its initiator.  This allows precedence parsing organize sub-bracket forms in the same way it organizes 
   sub-precedent-parsed intervals.
  - MERGED OUTPUT -- Merged precedence+bracket parsing returns a unit whose head is the initiator token, whose first 
   argument is the left parse tree from precedence parsing, and whose remaining arguments are the body elements from 
   the bracket body.  (The terminator interval is dropped)
- COMPOSITIONAL PARSING -- Merged precedent+bracket parsing is used as the term parser.exclude for compositional parsing, and
  this parsing is used at the top-level and at the top of each bracketed interval.
- TOKEN STRUCTURE SPEC
  ...
## --- NORMALIZATION ---

**NORMALIZATION** -- Idempotent rewrite rules that transformed interval trees into semantically natural AST forms.

Combo parsing is necessarily semantics free.  It maps an interval into a tree without ....
## --- SCAFFOLDING ---
  
SCAFFOLDING -- Scaffolding is a technique where parse structure is included in the source text which is not intended 
to be part of the final parsed form, instead it is included as a way to shape the structure of the parse tree which 
will remain after the scaffolding is removed.  
Thus scaffolding can be implemented as a rewrite rules which transforms one parse tree into another parse tree
where the scaffolding is removed.  
Four common scaffolding forms:

### ... PAREN SCAFFOLDING ...

**PAREN SCAFFOLDING** -- In the case that one wants to include a low precedence interval as a sub-interval within a higher precedence interval, PAREN SCAFFOLDING may be used.  Paren scaffolding is a bracket form with a very high precedence.  The high precedence ensures that the paren form itself will embedded within any other interval according to precedence parsing rules.  At the same time, the body of all bracket forms are parsed assuming the lowest precedence context thus a low precedence form may be included in their body.  After parsing, the paren bracketing scaffold parse node is remove from the parse tree and it replaced by its first (and only) child body form.  The result is a parse tree with a low precedence interval as a sub interval of the high precedence interval, and the paren bracketing node and initiator and terminator tokens are removed.
  Traditionally:  '(' and ')' are used for paren scaffolding.

### ... SEPARATOR SCAFFOLDING ...

**SEPARATOR SCAFFOLDING** -- By default, precedence parsing will bind each interval to intervals on the left or right with greater (or equal) precedence.  In some cases one may want a prefix or post-fix token that does NOT bind to its left or right, even in cases where a higher precedence interval exists there which otherwise get bound.  

Separator tokens can be used to prevent these associations.  (e.g. putting "5" next to "-2" would result in "5-2", but adding a separator "5, -2" prevents that operator binding.)  Separators tokens themselves also have a precedence, this allows them to separate some forms while being included as a single n-ary interval within other forms. (e.g. some languages allow "," within a statement, while ";" ends statements.) And just as with all scaffolding these separator nodes are removed from the resulting parse tree.  The children of an separator form are merged in as children of the parent of the separator node in the tree.  So:
  BLOCK( SEMICOLON_SEPARATOR( stmt1  stmt2  stmt3 ) ) 
      is simplified to be:
  BLOCK( stmt1  stmt2  stmt3 )

Traditionally: ',' and ';' are used as separator tokens.
## === SPIRAL 3 PARSING == UNIFORM MARKDOWN ===
### --- intro ---

Spiral three defines a consensus language structures based on the spiral two framework.
These consensus tokens, structures and scaffolding are widely employed across a wide range of formal languages.
### --- CONSENSUS TOKEN STRUCTURE ---
  
CONSENSUS TOKEN STRUCTURE -- Through hundreds of years of mathematics and dozens of years of software engineering
humanity has settled on a surprisingly well conserved notational conventions and token structure.  

The following categories of token exist in many formal languages, and when they are present they generally have the same parsing semantics and same relative precedence.

  == NAME ==        == EXAMPLES ==      == EXPLANATION ==
- BRACKETING        " ' ` ( { [         These bind tightly on their outsides and loosely on their insides
- PREFIX / SUFFIX   ! ?                 These bind tightly to the token they adjoin
- MATH OPERATORS    * + - /             Many of these have the longest history and are most conserved
- COMPARATOR OPS    < > <= >= == !=
- LOGICAL OPS       and or not && ||    Logical operators
- STATEMENT FORMS   = --> ::=           Toplevel forms.  These bind loosely and cannot be contained in any other form except a block

The many specialized formal languages humanity has developed over time, have had wildly different semantics and usage.
Despite this, and quite surprisingly, the basic precedence and bracketing properties of the actual text tokens items
themselves are fairly conserved.  For example, "+" generally binds looser that "* ", even in cases when "*" might mean 
memory dereference not multiplication!  The "[" token is generally a bracket matching the "]" token, and has higher precedence than "+" even as the MEANING of the "[" is changed between language.  

It seems the human eye has grown accustomed to expecting certain bracketing and precedence structure to be associated with specific tokens, thus language creators very rarely violate these expectations.  
### --- CONSENSUS LANGUAGE STRUCTURES ---

CONSENSUS LANGUAGE STRUCTURE -- In addition to having a conserved token structure, the global or macro-structure of 
many formal languages is also fairly conserved as well.  Many formal languages have parts which map into the 
categories listed below.  Expression token will have higher precedence than statements which have higher precedence than block tokens:
- EXPRESSION  --  Expressions are contained within statements and within other expressions.
- STATEMENT   --  Statements specialized operators that only occur as first level children of statements.
- BLOCK       --  Blocks nest lexically within other blocks, and are used to define the largest language structures.
### --- TRAILING SEPARATOR SCAFFOLDING ---

**TRAILING SEPARATOR SCAFFOLDING** -- There are two reasons why a separator might be included at the end of a parsed region.  First the author may think of these separators as terminators for the interval to their left.  

In this case they might include it a the end of the last interval since they are thinking of it as a terminator in addition to being a separator.  Second the same bracketing token might be used as both paren scaffolding, and well as a bracket form which remains as part of the parse.  These cases can be distinguished since paren form must have exactly one child parse form, while brackets may have any number of children.  But this distinction is ambiguous in the case of a bracketed for with exactly one sub-interval.  To distinguish this case one can add a trailing separator token to the bracket form.  Now it is known to not be paren scaffolding, and by removing trailing separator scaffolding one is able to cover this final case -- to have a bracket form with exactly one child.  So:   
    3*(4+5) uses paren scaffolding to embed + inside *,  While 
    3*(4+5,) creates a one-element tuple as the second argument to the * operator.
### --- STATEMENT FORM SCAFFOLDING ---

**SEQUENCE SCAFFOLDING** -- It is often desirable to omit "boiler plate" tokens from a source text in cases where doing so will not introduce parsing ambiguity.

By default combo parsing will compose a sequence of forms without punctuation into a SEQ interval.  SEQUENCE SCAFFOLDING rewrites specific cases of these SEQ forms into the correctly structured parse trees without the SEQ node.


STATEMENT FORM is an example of this, it drops bracketing and separators tokens from an interval in cases where one can recover the correct parse structure without ambiguity.
- The Statement Form rewrite rule, will rewrite any SEQ that occurs at the top level of a block, as long as the first token in the original form is an alphanumeric terminal token.
- This first token is taken to be the type or head of the rewritten parse node.
- Then the sequence is effectively re-parsed with this first token removed in order to obtain the children of this parse node.  For example:  "if !done process_next" is parsed as a SEQ, then removing the 'if' and reparsing we get the following tree:  if( '!'(done), process_next ).  
- NOTICE:  We must remove the 'if' before parsing the remaining text, otherwise the first expression will be '!'(if, done) where the 'if' is buried within this first expression.  This is a challenge since this rewrite is performed over the completed parse tree.  Still it turns out one can reliably find and remove the first pre-order terminal within the first parse tree, and this will yield the parse tree that would have been generated had one parsed without the first element in place.
### --- BLOCK TERMINATOR SCAFFOLDING ---

**BLOCK TERMINATOR SCAFFOLDING** -- Block terminator scaffolding is another token omission scaffolding.  This one merges statement termination and sub-block specification for statements.  Many statements accept a block as an argument in which sub-statements can be placed.  block terminator uses this sub-block as the signal for termination of the statement form itself.

This is a semantics-free way to avoid the use of sub-block form punctuation AND statement termination punctuation. For example, "if x<5 {print x} print 'done'" is parsed as: SEQ( if( '<'(x,5), BLK(print(x))), print('done')).

Notice this semantics free rewrite rule only allows ONE sub-block per statement.  Thus many languages have statements which will be parsed as two or more sequential statements.  For example:
    "if x<5 {print 'low'} else {print 'high'}" will parse as 
    SEQ( if( '<'(x,5) BLK( print('low') ), else( BLK( print('high') ) ))
### --- SEMANTIC STATEMENT NORMALIZATION ---
It seems there is no way to accommodate the multi-block statements in a semantics free manner.  However, most languages have relatively few of these multi-block statement types, and those types are even somewhat conserved across formal languages.  Thus a simple post-parsing semantic rewrite can be applied for all languages, and possibly including special rewrites that are language specific.  Notice as long as the languages to be parsed do not disagree on the which terms are to be considered reserved they can all be parsed simultaneously with the same semantic statement rewriting rules in place.  Here is an example of the Statement Rule Rewriting Spec:
  [["if", "elif", "else"],
   ["try", "except"]]
   
   
pkg 2018.lang.uniform.markdown.rewrites:

    # Fn call rewrite
    def rule fn_call    "("( $fn, $args... ) --> $fn( $body... )
        if: $fn != 'GAP
        if $args[0]

    # Removes paren scaffolding and replaces it with its first arg
    def rule paren_scaffolding   "("( GAP, $only_arg ) --> $only_arg ;
    
    # Rewrite non paren bracket into its prefix form   
    def rule non_paren_prefix_bracket  
            $b( GAP, $only_arg ) --> $head( $only_arg )
        {
            when: $b =~ Bracket and $b != "("
            $head = bracket_heads.$b.prefix
        }
        
    # Removes separator scaffolding, trailing separator, and replaces bracket bracket head with its prefix or infix form
    def rule separator_scaffolding  $b( $left, $s( $args... ) ) --> $head( $body... ) ::
        when: $b =~ Bracket and $b != "(" and $s =~ Separator
        
        if $left == 'GAP:   
            $head = bracket_heads.$b.infix
            $prefix = [$left]
        else:
            $head = bracket_heads.$b.prefix
            $prefix = []
        
        if $args[-1] == 'GAP:
            $body = $prefix + $args[:-1]
        else:
            $body = $prefix + $args
            
    bracket_heads:
        "(": { prefix:TUP }  # Infix is fn call
        
    trailing_separator:     $b( $s( $args... ) ) --> $b( $args[:-1]... )  # removes trailing separator
                                { sat: $b =~ 'group and $s =~ 'separator and $args[-1] != 'GAP }
                                

_
## ... discussion ...
                       
=== DISCUSSION ===

This whole said semantics-free code markdown thing is a cheat!  In the end you semantics are jammed into the last
last normalization step.  True, its a cheat, but it is an 80/20 cheat that affords nearly all of the advantages 
that would come with true semantics-free parsing AND nearly all of the advantages that would come from the worlds 
most beautiful file forms.  We almost fully get BOTH, because we were willing to give up a little on EACH.

(This by the way is the guiding principle of Uniform in action.  80/20 optimal and you (almost) have it all, all at once.)

What advantages does semantics-free parsing give, and what do we get:

COMPOSABILITY --

CODE AS DATA -- 


BUT WHAT WAS LOST
INCOMPATIBLE NORMALIZATIONS -- 
SOME SIMPLICITY -- I tiny bit of semantics info must be shared.








                                
                                
                                
                                
                                
   
>>> pkg 2018.lang.uniform.markdown.spec:

    tokens: 
        (var)
        "_string,nowrap_"       ###  High precedence container of raw unparsed characters
        (_group_)   [_group_]   ###  High precedence containers that can be part of expression forms
        *    /   
        +    -
        <=   <    >    >=       ###  These six comparators are conserved over hundreds of years
        ==   !=                 ###    their ascii renditions were adopted (and widely conserved within computer languages)
        (expr)
        (seq)  
        ,_infix,separator       ###  Tracking its usage in English, comma is a separator of parts within a sentence (statement)
        =   <--  ::=  :-
        (stmt)
        ;_infix,separator       ###  Also tracking English, semicolon is a separator that separates complete sentence parts (complete statements)
        {_group_}               ###  Consensus usage treats these as matching bracket token.  Computer languages use 
                                ###    these as low precedence containers that do not fit into expressions
        (eol) 
    


  



  
  
- Spiral 3 markdown is intended to be used as a universal, semantics-free transformation of text into a parse tree.
- The idea is that most specialized Uniform DSLs will not need to make changes to these tokenization or parsing rules; 
  instead they can rely on DSL-specific, post-error checking, and the occasional post-parsing tree transformation rule.
- In order to facilitate meta-programming over this collection of DSLs, it is critical that the end software developer
  can effortlessly, mentally translate the source text forms seen into the parse tree structures that they will 
  meta-program over.  Having a single semantics-free parsing layer conserved over all DSLs facilitates this.

- Just as important.  


Note:  Uniform markdown parsing is an extremely permissive parsing form since it aims to cover a huge range of languages.
This means that a subsequent phase of translation and error reporting must be added in order to have a complete language parser.exclude.
Spiral two of uniform parsing is quite opinionated, it does NOT cover all forms of parsing, as spiral one does.
Nonetheless there are universality advantages to the Uniform spiral two approach two parsing:
(1) COMPOSABILITY --  Two different languages based on the same underlying tokens can be generally be composed seamlessly.
    This is in stark contrast to most other parsing paradigms where one CANNOT combine rules from different languages
    and expect to get a parsable language.
(2) EASE OF AUTHORING --  Writing parse grammars for text is notoriously difficult.  Writing the error checking and 
    parse tree transformations needed to complete a uniform language, by contrast, is much simpler.
(3) UNDERSTANDABILITY --  Uniform parsing relies on only THREE constructs which are already very widely understood
    and expected by software language users -- without even reading a manual.  This means that even as authors create 
    hundreds interacting and overlapping specialized DSLs each with their own syntax, their users will be able to 
    understand the parsing that results since all of it stems from three big well understood ideas:
    TOKENIZATION, BRACKETING, and PRECEDENCE.
    

    
        
    
- Bracketing
- Prefix operators
- Mathematical operators
- Logical operators
- Comparative operators
- --expression-boundary--
- Statement like operators
- Separators, terminators, statement modifier, and statement bracketing operators

    
And it turns out, with delicate balancing it seems very little beauty or expressivity are lost by restricting ourselves 
to these three constructs.


=== SPIRAL 3 == EXPECTED NOTATION ===
Centuries of math, and decades of programming have left us with a basin of consensus regarding a number of expected forms.

These are captured here:

MATH FORMS
- CORE INFIX -- Here are the infix expression operators with their associated precedence indicated by line number:
  * /
  + -
  <= < == != > >=
- FUNCTION CALL -- f(x,y)


PROGRAMMING OPERATORS
 



UNIFORM SUGARED NOTATION
# === MARKDOWN ===
### --- Block form printing

(k,v)   ==>   k=v
(k,v)   ==>   k: v1, v2, ...		if v.head == None
(k,v)   ==>   
(k,v)   ==>   
(k,v)   ==>   


in_ws value_isa_block   pib

javaic block.    

_
### --- ALTERNATE UNIT FORMATS / REPRESENTATIONS ---
#### 

Unit data is used to encode all forms of structured data.  All of these forms have some notion of grouping or organizing data into "blocks", but the exact details of how these blocks operate are not full consistent.  Thus we define several block encoding formats that one may losslessly convert between.  Each of these forms is expressed as a recursive tree of units, it is just the details of how those units structure that are different.  Here are the different forms, and how to convert between them:

- _**GUF -- Generalized Unit Form**_   -- The base format: unit head/keys/values are all arbitrary graphs
- **NBF -- Nested Block Form** 		-- Views strucuture as nested code blocks with controlling logic
- **SUF -- Simplified Unit Form** 	-- Rewrites complex heads & keys simplify so unit data embeds many places
- **LUF -- List Unit Form** -- 

_
#### -- GUF - GENERALIZED UNIT FORM --

**GENERALIZED UNIT FORM** -- _Generalized Unit Form (GUF)_ allows for arbitrary data values in all positions, that is arbitrary structure within the head, the keys and the values of the unit.  A generalized unit is expressed as a sequence of k/v pairs where the first pair is ('^', meta)  where meta is a map of meta keys including a mapping for 'head':

     ('^', meta)  (k1, v1)  (k2, v2) ...


_
#### -- NBF - NESTED BLOCK FORM --

**NESTED BLOCK FORM** -- _Nested Block Form (NBF)_ is a unit format designed organize code and data into recursive lists of statement+block combination elements.  Each element may have a naming/controlling statement logic and may have an associated nested sub-block.  

Generalized unit form is losslessly converted into CBF by converting each **"k, v"** pair in GUF into a statement plus nested body form in CBF:  **"stmt ':' body1, body2, ..."**  There are four conversion cases depending upon:
- whether or not the k,v pair is listy (whether k is list indexing natural number or not), and 
- whether or not the value has head or is itself just a block of statements.  

Here are the cases:
1. A listy k,v pair with no-head value v is treated as a bare block:			None ':' body1, body2, ... 
		where v = { value1, value2, ... }
2. A listy k,v pair 
		with headed value v is treated as simple stmt:							 stmt
		where stmt = v
3. A non-listy k,v pair with no-head value v is treated as: 					stmt ':' body1, body2, ...
		where k = stmt, and v = { body1, body2, ... }
4. A non-listy k,v pair with a headed value v is treated as:					stmt 
		where stmt = (k ':' v) is an assignment statement.

Notice: The Code-Block Transform above converts a generalized unit with keys and values into into a numbered _LIST_ of statements where the original keys are now embedded into the statements and values are sometimes part of statements and other times are a nested block of statements.  The mapping described above is bidirectional and lossless, thus GUF and CBF are just different ways of looking at the same underlying data.  GUF is optomized to view data as a tree of subunits that are indexed by naming key or by numbering position, while CBF is optomized for consumption of the data as recursive semantic code blocks connected by controlling logic.  During interpretation these two views of the data may be interleaved as finely as needed.


_
##### - example

foo:
  if this:
	print that
	print the_other
  else:
	or other
	
  summary:
  x = 25
_
##### - Conversion Code -

DURING PARSING of '__'(left, right1, right2, ...)	

    -->		< left, BLK(right1, right2, ...) >			If len > 2
    -->		< left, right >								If len == 2
    -->		< left, None >								If len == 1

DURING PRINTING TO PAIR-BLOCK FORM

    <i, BLK(...)>		-->			<i, '__'(BLK(...))>
    <i, stmt>			-->			<i,	'__'(stmt)>
    <i, '__'(k, v)>		-->		    <i, '__'(k, v)>	
    
    <stmt, BLK(...)>	-->			<i, '__'(stmt, BLK(...))>
    <else, print> 		-->			<i,	'__'(else, print)


**to_body_block_form**:
	for k,v in unit.items():
		if k,v is not a listy pair:
			map to listy pair  < n, ':'(k, v) >
		elif v.head == ':':
			map to listy pair  < n, v >
		else:
			map to listy pair  < n, v >

**from_body_block_form**:
	for k,v in unit.items():
		if k,v is not a listy pair or v != ':' then
			map to k,v
		elif v[1].head is None:
			map to ':'(k, v[0], v[1], ...)
		else:
			map to ':'(k, v)

_
#### -- SUF - SIMPLIFIED UNIT FORM --
##### _

**SIMPLIFIED UNIT FORM** -- _Simplified Unit Form (SUF)_ is another lossless, bidirectional transformation of the generalized unit form.  SUF potentially transforms both the head and the keys of a generalized unit in order to allow that generalized unit to "fit" (embed) into more restrictive formats.  Often SW langauges and transport format has valid identifier restrictions and/or structure restrictions (e.g. keys cannot themselves be structured, etc.)  SUF encodes that same data, but reorganizes it in order to satify (nearly) arbitrary constraints on valid values of unit head and keys.  SUF is also required in order to embed generalized unit data into lexspace whose key structure is similar to the lexical constraints of most programming languages.

Given a head_predicate, H, and key_predicate, K, and unit, U:
- If the head of U does not satisfy H, or itself is three underscores '_ _ _' then it is replaced with '_ _ _'  (three underscores), and the original head is prepended as a positional subunit at the front of U.
- For any pair (Ki, Vi) in U where Ki does not satisfy the key predicate, K, or its value Vi has head '_' then the pair is replaced with a listy pair (N, ' _ '(Ki, Vi)) where N is the appropriate listy index value.

Notice: the resulting form pushes the original complexity in the head or keys into values (where almost all languages expect to have recursive complexity).  The ability to losslessly move back and forth between GUF and SUF allows uniform to simultaneously have great flexibility in using complex head/key structures to naturally encode complex code forms, while simultaneously allowing it to be embedded into most structured data formats where such flexibility is not allowed.  Moreover the details of this mapping are chosen so that in cases where simple keys/heads are used, then recursive k tranversal addresses both formats equivelantly and as expected given the sequence of addressing keys.



**UNIT FORM** -- _Unit Form_ is specific instance of SUF form where head values must be strings, but can be artitrary strings, while keys must be unique natural numbers or unique valid identifier strings (alpha-numeric with leading alpha).  This format is chosen as a "simplest" presentation form for unit data, since it embeds directly into lexspace, as well as code structured supported by most programming langauges.


unit keys is functional

recursive pointers

Null heads


NOTE:  Allowing arbitrary strings as unit form heads was needed to provide the flexibility needed to encode punctuation marks in the head as is needed when encoding parsed code structure, e.g.:   '+'(1, '*'(2, 3))

_
##### - SUF examples -

    foo("one", 2, three: 4)
    
    ___('+'('complex', 'head'), is_true: yes)
    
    foo(_('+'('complex', 'head'), example_value))
    
    BLK(map: 'without', a: 'head')
_
#### -- LIST FORM --
**LIST FORM** -- _List Form_ is a varient of Simplfied Unit Form that encodes each unit as a flat JSON list of string and int.  First each unit is converted into SUF where both head and keys are expressed as strings.

Given a unit U, it is conversion into list L = [ META, PAIR1, PAIR2, ... ].
- First U is converted into U1 a simplified unit form with both head and keys expressed as strings.
- If the only meta key is head, then META above is simply the head string with a leading space.  
	So "foo" becomes " foo".  Otherwise the meta list begins with " " a string with a space, followed pairs of list entries the first is a string meta key, and the second is a recursive unit value encoded as a lists.
- Next each k,v pair in the unit are added to the list:
	- If it is a listy pair, then the key k is discarded and just the value is recursively encoded in the list.
	- If it is not a listy pair, then a ':' is prepended to the string key value, and the recursive value is appended to the list next.

string conversion
recursive 

As an example this unit-form:
    postal([123, 'main'], state: 'CA', zip: 94110, ^cached: true)
    
    [[" ", "'postal", ":cached", "`true"], "["(123, "'main"), ":state", "'CA", ":zip", 94110]]
    eq( '='( '+'( '*'(a, a), '*'(b, b)), '*'(c, c)),  )

Whould translate to this list-form:
	[" pythagorean", [" =", [" +", [" *", a, a]]]]



  __('^'={head='foo', len=5}, one, two, three=33)



-  SUF with string head and keys.
- If the only meta key is the head string, then it is prep


unit form where so both keys and values
  

**LISTY PAIR** -- A subunit is a _listy pair_ if its key is the natual number, _n_, where all prior natural numbers [0, _n_-1] have served as keys in some prior element.  So in the unit:  
unit(a='one', 3='two', 0='three', 1='four', red='five', 2='six', 4='seven')  the pairs: 0='three', 1='four', 2='six', 3='two', 4='seven' are the listy pairs.  Intuitively this subset of elements forms a list-like sequence of values.

_
#### -- CONVERSION PROCEDURES --
**=== CONVERSIONS ===**

**to_simplified_key_form(head_predicate, key_predicate)**:
	if head does not satify 'head_predicate' then 
		replaced with '_' as head, and the original head becomes the value of the first k,v pair
	For k,v in unit.items():
	- if v.head != ':'
		- if k is a string satifying 'key_predicate' then it is left unchanged as k,v
		- if k,v is a listy pair then it is left unchanged as k,v
		- if k is VOID the it is mapped to n,v as the next listy pair
	- otherwise < k,v > is mapped to < n,':'(k, v) > as the next listy pair.

**from_simplified_key_form**:
	if head is '_' then replace it by the value of the first k/v pair
	for k,v in unit.items():
	- if k matches the kth item and v == ':'(a,b) then the pair is replace by a,b


PARSING
- apply 'from_code_form'

PRINTING
- apply to_code_form
- if printing in unit form then apply to_simplified_key_form with unit defaults
	



_
## -- Misplaced Notes --

CONVERSIONS
- PARSING REWRITES -- During parsing code structure is converted into SKF.
- PRINTING REWRITES -- During printing code structure is convered into BLOCK BODY FORM.
- MACRO EXPANSION -- During macro processing of code blocks these units may also be converted into BBF for simplicity of the resoning / processing
- STANDARD DATA PROCESSING -- The standard form for unit data is SKF however, since this format best maps onto  generalizaed data structures




DATA BLOCK -- A block is a unit that contains a sequence of statements in "data" form
- The block itself is typeless, that is, it has no head.
- A statement may be:
	1. A simple-sub-value encoded as a key/value with a key that is a string.
	2. A position-indexed value without any key or a list-indexed natural number key.
		NOTE:  A complex K/V pair is simply treated as a position-indexed k/v with ':'(k, v) as the value



CONVERSION INTO BLOCK BODY FORM


NOTES:

**INTENTIONALLY LOSSY** -- Block form is intentionally lossy regarding several specific kinds of structure.
1. SEPARATOR STRUCTURE.  User may use indention, linefeed, implicit sequencing, or various separators to break a block into statements.  It does not matter what combination are used, all of that detail is lost and only a list of statements remains.
2. LIST INDICIES.  Some unit containers require mappy contents, that each value is associated with some unique key.  In these cases the natural numbers, 0, 1, 2, ... are used.  Thus the presence of these list indicies is ignored in the case that they precisely match the implied indices for those values.



_
## stmt form
foo (2, x:3) {11, y:"b"}
foo(TUP(2, x:3), 11, y:"b")
## === EXAMPLES ===
	**pkg** uf.construct;
	**def** construct _MARKDOWN_ := PARSING + TOKENIZER + 
			BRACE_PARSER + PRECEDENT_PARSER + COMPOSITION_PARSER +
			UNIFORM_MARKDOWN

	**pkg** uf.markdown.PARSING;
	**def** type Parse extends Str:
	**def** interface _Parser_:
		def op parse(Buffer, ->Interval)
	**def** class _Buffer_:
		$contents Str
		$tree Interval
		$tokens Token
		$precedence List Ident
	**def** type _Token_:
		$name Str	
# ### MARKDOWN BY EXAMPLE ###
### --- UNIT FORM ---

// Example Unit.  EVERYTHING in uniform parses down into this one form:
		
		i_am_a_unit( arg1, arg2, key1: value1, key2: value2 )


// JSON ATOMS -- All parse to units with special head and one key argument:
	111         ==  INT("111")
	-1.1        ==  NUM("-1.1")
	"a string"  ==  STR("a string")
	true        ==  true()
	foo			==  foo()

// UNIFORM LISTS -- Parse to unit with a 'LST' head
	[11, "two"] ==  LST( INT("11"), STR("two") )

// UNIFORM MAPS 
	{k: value}  ==  OBJ( k: SYM("value") )									# Maps have special 'OBJ' head
	{0: 'arg1', 1: 'arg2', key1: 111} == OBJ('arg1', 'arg2', key1: 111)     # Positional args are just numeric keys
	{"Krazy $key": 777}  ==  OBJ("Krazy $key": 777)							# Crazy keys are quoted

// UNIT HEAD -- is really use a special meta key "^"
	{ "^":"foo", "one":1 } == OBJ( "^": SYM("foo"), one: INT("1") ) == foo(one:1)

// The special "OBJ" head is used whenever there is no head "^" key specified:
	{ "a": 11 } == OBJ(a: 11)

// The FIXED arguments for a unit are really just numeric keys starting at 0:
	foo(11, 22)   ==  {"^": "foo", 0: 11, 1: 22}

// BLOCK FORM -- Units can have a BLOCK as a suffix, this is stored in the "^body" meta key:
	some_unit(111, k:"three") { aaa; bbb; }
		== {"^": "some_unit", 0: INT("111"), k: STR("three"), "^body": OBJ(SYM("aaa"), SYM("bbb")) }

_
### --- OUTPUT FORMATS ---

// ORDERED-MAP INTERFACE   
// Every Unit is really just an ordered map from of key, value pairs.
// Programmer APIs allow one to access unit data using either
// native types (like float) or the Unit Form map interface


// JSON LIST-FORM
// For manipulation and transport, Unit-form can be expressed as recursive lists of strings:
//     -11 --> "-11"    		111.1 --> "+111.1"   
//	   "hello" --> "'hello" 	hello --> "`hello"   
//

{[pick better example that is not a statement form!]}
	some_unit(111, k:"three") { aaa; bbb }
        == ["^some_unit", "+111", ":k", "`three", ":aaa", "`bbb"]



// UNIT FORM
// Unit form represents data recursively using a single data form, the Unit

    some('unit', 'value', key: true)
    x
    

_
### --- UNIFORM VARIATIONAL FORMS ---

//  Vive la différence !
//
//  Unit-Form is quite a "uni"-form; it is just a map-of-maps all the way down.
//  This is optomized for meta-programmatic manipulation.
//
//  At the same time Uniform Syntax looks so darn good because it provides
//  multiple ways to encode the same unit data.  This allows an author to select
//  a surface varient that is ideal for each circumstance and kind of data.  
//  These variational surface forms make uniform ideal for human editing / viewing.
//
//  Having your cake and eating it too!  ---  so very French.



//  ________________
//  UNIFORM COMMENTS

// We have already seen "end of line" comments like this one

    [1, 2,  /* and if you need an inline comment, uniform has those too. */  3]

    <!-- and this HTML style comment type is ideal for commenting <i>around</i> other /*comments*/  -->

 #  Uniform is an equal opportunity comment notation...  Its not picky...
 #  If you like Pythonic/Shell comments, you can this way too.



//  ___________________
//  SYMBOLS and STRINGS

    of_course_i_am_a_symbol   ==  SYM("of_course_i_am_a_symbol")

    "me+too"(simple_key: value, "garbage%!#$key": 111) == {"`": "me+too", "garbage%!#key": INT("111") )



// {[not working]}
//  _________________
//  MULTI-LINE STRING
//
//  The first \n and all whitespace columns prior to the first non-whitespace character are trimmed from each line.

key: """
  Multi-line str
  ings can be important.
	  --> Whitespace columns are trimmed from each line. <-- """  ==

"Multi-line strings can be important.\n    --> Whitespace columns are trimmed from each line."



//  String that begins with a \n or with whitespace are handled as shown:

"""   x  """  ==  "   x  "      // whitespace not trimmed before first \n

"\n"     == """

"""                             // An extra \n must be added if string begins with one.




//  __________
//  JSON FORMS
//
//   Uniform includes all JSON forms without any escaping required.
//   Uniform atoms are the same as JSON atoms, and Uniform uses [ ... ]  and { ... } for lists and objects.



//  _______________
//  YAMAIC FORMS
//
//  JSON is great and "human readable".  Well at least until one tries to understand or
//  edit a ten page JSON structure.  By contrast, viewing/editing a 100 page YAML structure
//  is a snap, since its indention-based parsing ensures the viewer always knows "where" they are,
//  and missing or misaligned closing braces are impossible!
//
//  NOTE: Its that "::" at the front that starts YAMAIC parsing

::

	key1: value1

	key2:
		sub_key1: value2
		sub_key2: [list, of, symbols]


// ... is equivelant to this map form
== { key1: value1, 
	 key2: { sub_key1: value2, sub_key2: [list, of, symbols] }


// And don't forget, its really all just Unit-Forms all the way down
== OBJ(key1: SYM("value1"), 
	   key2: OBJ( sub_key1: value2, sub_key2: LST(SYM("list"), SYM("of"), SYM("symbols")))

_
### -not-working-
// {[not working right yet!]}
// WORD WRAPPING -- Just like YAML, Uniform also has whitespace preserving multi-line
// strings with and without auto word wrapping.

:
	first_key_value :- This is a word wrapping paragraph of text.
		It is used in cases where one wants string without
		anny line breaks.

	line_break_preserving_string: """
		Here this line
			with weird indention.
		But it still works...   """



//  The YAMAIC parse form is Unit-Form complete, <i>ANY</i> unit form *can* be YAMAIC parse/printing:

a_unit_form( 111, key: value, key2: sub_unit(with:"sub values") ) { print("hello") }  ==

:a_unit_tree
	111
	key: value
	key2: sub_unit
		with: "sub values"
	".": BLK
		print:
			"hello"

NOTE FROM INFIX SECTION
<!--
//  Procedural languages often don't agree on their operators, or their meanings.
//  Interestingly enough, they generally <d>do</d> agree on the precedence orderings and usage structure
//  of the syntactic characters themselves!  So exactly what '+' does, and what types it applies to may vary
//  but it can always be used as an infix operator, and it always binds less tightly than '*' does.
//  The is little cost to allowing all the operators common to many lan-->

_
### CODE-LIKE FORMS
//  The follow Unit Form variants are intended to encode procedure-like data in Unit Form
//  This allows Uniform Syntax to be used as the basis for JSON-based homo-iconic meta programming languages.


//  _______________________________
//  INFIX, PREFIX, AND SUFFIX FORMS

//  Uniform allows the expected prefix, infix, and suffix forms.  All parse to Unit-Form so:
    x = y + z    ==    `=( x, `+(y, z) )

//  And these operators have the expected structure and precedence:
    x = y*y - z >= 5 || done    ==    `=( `||( `>=( `-( `*(y, y), z), 5 ), done )

//  Operators can be infix, prefix, or suffix just as expected:
//  (Notice:  "++_" uses an underscore to differentiate it from "++"(z) as a prefix operator.)
    x  /=  -y + z++   ==
    `/=( `+( `-(y), `_++"(z) ))


_
### --- INDENTION BASED PARSING ---

{[not working; should not conflate statement form and indention based parsing???]}
//  ______________________________________
//  PYTHONIC (INDENTION-BASED) BLOCK FORMS
//
//  Uniform, like Python, uses the ":" character to indicate the end of a statement,
//  and the beginning of an indention based sub-block:

	while x<10:
		print x

	== while( "<"(x,10), ":": BLK(print(x)) )

	== while x<10 { print x }

_
### --- STATEMENT FORM ---

//  ______________
//  STATEMENT FORM
//
//  Many procedural languages have the notion formulas like those listed above
//  fitting into larger syntactic forms often called statements.
//  In Uniform a statement is simply a Unit-Form without the '(' and ')' marks, and with out
//  key/value pairs.  They are initiated when a symbol is followed by another atom without
//  any other punctuation like this:

    print "this is a statement"
    ==  print("this is a statement)


//  A statement may be terminated by a ';' character, or it may be terminated
//  by a block form '{ ... }', in that case the block become the value of ":" key:
//
	while ++x < 10 ;               ==  while( "<"("++"(x), 10) )
	while ++x < 10 { print x; }    ==  while( "<"("++"(x), 10) ":": BLK(print(x)) )






//  _______________
//  STATEMENT BLOCK
//
//  Normally ":" indicates a key-value pair, but at the end of a statement or
//  within a statement block the ":" indicates a sub-block.
//  Statement blocks are indicated by '{ }', by pythonic sub-blocks, and by "::"


if x<1 {
	try:
		print x
}

==  if x<1:
		try:
			print x

==  ::if
		x<1
		try:
			print x

==  if( "<"(x,1), ":": BLK( try(":": BLK( print x ) ) )



_
### --- MIXING PARSE MODES ---

//  ________________________________
//  SUMMARY OF UNIFORM PARSING MODES

//  By default Uniform performs Unit-Form parsing with standard infix, prefix, suffix operators.
//  The initiators below are used to embed different parsing modes into each other:

	sym  atom           // begins a statement form
	:    :head          // begins a YAMAIC    whitespace    sensitive, nested key/value parsing
	::   ::head         // begins a PYTHONIC  whitespace    sensitive, statement block parsing
	{ "...              // begins a JSON      whitespace IN-sensitive, JSON parsing
	{ ...               // begins a JAVAIC    whitespace IN-sensitive, statement block parsing


// EXAMPLE OF MIXING MODES
	:top_head
		key1: [v1, v2] + [3]

		key2:
			fn(int x)::                 // "int x" is a stmt
				try:                    // "try" is NOT a key, since inside "::"
					while x>0 {x--;}    // mixed JAVAIC into that PYTHONIC form... all good!
				except IOerr:
					pass



//  PYTHONIC is better?  Yes!  It makes larger structures visually clearer.  no brace errors.
//  JAVAIC   is better?  Yes!  It makes smaller structures more compact and visually cohesive.

fn(int x)::
	if x<0:
		-1
	else:                   // else is a second statement!
		+1                  // is ok, Uniform normalization fixes it.

== fn(int x) { if x<0 {-1} else {+1} }

== fn( SEQ(int x), ":": BLK(            // Its all just unit form ... all the way down
		if( "<"(x,0), ":":BLK(-1)),
            else( ":":BLK(1) ))

_
# Example code snipits
### PRACTICAL EXAMPLES OF UNIFORM SYNTAX

    TYPES:Lex:Typespec
        self: Lex
        UP: fn( ->Lex ):: get('__UP', ||break)



    // This file contains a range of sample for visual and other assessment
    EXAMPLES::
        aa: fn():: 1
        # your imports are:  yourself if you are a package, and every named import (it is an error if they are not a pkg),
        bb: fn(self:Lex, returns: [Refs, ...]):
            [self].all{pkg}



        bindings: fn(self.Lex, returns: bindings(k(str):Lex):
            is_pkg_spec = fn(): get(0).head('pkg') && self[0][0].head(".")
            lex_roots   = fn(): [self].all{ get(0).head('pkg') } + declared_imports + lex_imports(UP)
            lex_import  = fn(): lex_roots
            decl_imports= fn(): self.head('pkg').all_subunits{ fixed, head('import'), add_all(self) }
            return [].tree_update( *lex_imports, x:continue )

        pkg: fn(self:Lex, returns:Ref):
            get(0).get(0)  if get(0).head('pkg')
            parent.pkg.copy.tee(append(key, continue))

        # your imports are:  yourself if you are a package, and every named import (it is an error if they are not a pkg),
        imports: fn(self:Lex, returns: [Refs, ...]):
            [self].all{pkg} + all{fixed, head('import'), **self.all(fixed)} + **parent.imports}


    xxx:
      [VALUE_TYPE, ...]  shorthand for {fixed:VALUE_TYPE, ...}
      HEAD(TYPE_FOR_FIXED, ..., name:TYPE_FOR_NAMED)

      {VALUE_TYPE, ...}

      fn(str, str, int, ..., a_key:sym, name:int, ...)
      fn(str, str, rest(fixed, int), rest(name, int), a_key:sym)
        my_class:type::
            frame: py_dict          // specifies simple python dict backend
            slots: [int xo; int yy]
            init: fn(int x; int y) { self.xx=x; self.yy=y; }


    LOOPS:
        for u in coll:
            print u
        for u in all(coll):
            print u
        for k,v in all(coll):
            print k,v
        l = [[k,v] for k,v in coll]

        # Comprehension.  Self is the resulting object during body execute
        l = [self.set(k,v*v) for k,v in coll]
        l = [k=v*v for k,v in coll]
        l = [append(v*v) for v in coll]

        # Same as:
        l = new_unit().for(k,v,coll) { append(v*v) }



    STANDARD_CONTROL_FLOW:
        for (int i=0; i<10; i++) { print "looks like a C loop."}

        if x+y&lt;z:
            print "pseudo code like"
        elif x<0:
            print "kinda python-ish"
        else:
            print "even infix percent %s" % x

        try {
            y = 0/0;
        } except:
            print "wait, is this python or Java?"




    yyy:
        PageCtrl.prototype.loadData = function (done) {
            var self = this;
            // attach page controller to a template
            var userCall = $.get('/user');
            var groupCall = $.get('/group');
            $.when(userCall, groupCall).done(function (a1, a2) {
                // Each argument is an array with the following structure: [ data, statusText, jqXHR ]
                var groups = a2[0].groups;
                var users = a1[0].users;
                self.pageModel.groupSpecList = groups;
                self.pageModel.groupUserList = users.concat(groups);
                done(null);
            });
        };




    ==============
::
    pkg pytop.test;

    # looks like Java,
    public static final foo(int x) {     # but with python comments
        while x<10 {
            for (i=0; i<10; i++) {
                print "this looks like Javascript";
            }
        }
    },

    maybe: {'this':['is', 'just', 'straight'], 'up':'JSON' },

    while(y<10):      /* or Python with c-comments */
        for k,v in something.iteritems():
            print "or maybe it is Python?";  nah('not exactly, but what the fuck it is?', answer:dunno.what.it.is)


    YAML_ISH::    // YAML-ish with java script comments, and python&java values
        kinda like yaml too :- with whole paragraphs
            of text embedded with out quotes
        or maybe with embedded :-
         linefeed characters and also
         internal indention, line breaks
            and tabular alignments
         retained!
        wtf:  x*y + z + if(x<10) {
            template = `` with "s and embedded {fn(call+3)} code forms``
            print "I can't even tell which format are in NOW?!";
        }



    =================
pytop: obj(test: pkg(pkg('.'(pytop, test)),
                    public(
                        static,
                        final,
                        foo(x),
                        '{'(while(
                                '<'(x, 10),
                                '{'(for('='(i, 0), '<'(i, 10), '_++'(i), '{'(print('this looks like Javascript'))))))),
                    while(
                        '<'(y, 10),
                        for(k,
                            v,
                            in,
                            '.'(something, iteritems()),
                            blk(print('or maybe it is Python?'),
                                nah('not exactly, but what the fuck it is?', answer: '.'(dunno, what, it, is))))),
                    YAML_ISH: YAML_ISH(
                            'kinda like yaml too': 'with whole paragraphs of text embedded with out quotes',
                            'or maybe with embedded':
                                'linefeed characters and also
internal indention, line breaks
   and tabular alignments
retained!',
                            wtf: '+'('*'(x, y),
                                    z,
                                    if( '<'(x, 10),
                                        '{'('='(template, ' with "s and embedded {fn(call+3)} code forms'),
                                            print('I can\'t even tell which format are in NOW?!'))))),
                    maybe: '{'(this: '['('is', 'just', 'straight'), up: 'JSON'),
</smcode></pre>
### Iteration


<pre><smcode>pkg examples.uf.control.iteration::
    // Every composite unit may have sub-units, thus it may be the subject of a for loop
    range(3) ==> [0, 1, 2]                      # A range evaluates to a unit with the specified values
    range(10,)  ==> [10, 11, 12, ... ]          # An unbounded range evaluates to a unit with an infinite number of sub-units
    Colors(red, green, blue)                    # A sequence of three symbols with naming head `Colors

    // Example loops
    for x in [0,1,2]: print x                   # Loop over three elements
    for x in range(3):  print x                 # Same same
    range(3).for( x ) { print x; }              # Same same
    range(3).for { print self; }                # same same
    for x in range(3) out [] { append(x) }      # list of numbers [0, 1, 2]  (rebuilds list)

    for x in range(5) out 0 { += x }            # returns 10. Begins with 0 and adds 1,2,3,4 to it
    for x in range(5) op "+" { x }              # Same same.  applies op 'append' to result of each body eval (as a list reducer)
    for(range(5), op: "+")                      # Same same with implicit identity body
    range(5).for(op: "+")                       # Same same expressed as a <q>mapping</q> transform accepting subject as self unit
    // Note for operators with an reducing-interpretation these iterators are automatically performed via
    // incremental, parallelizable updates.

    for u,i in range(100,) { if i>10 { break; } else { print u; } }   # prints 10 elements the breaks
        // return break and continue all operate as expected with respect to loops and fn bodies.
        for i in range(100,) { if i<110 {continue;} elif i>120 {break;} else { print u; } }   # prints 110, ... 120

    // But these also control implicit loops.  For example concatenating named subunits is implicitly a loop.
    x = add(y, z, ||force)          // forces overwrite with z value (the default)
    x = add(y, z, ||break)          // causes merge operation to stop, returns the partially merged forms
    x = add(y, z, ||continue)       // skips the overwriting iteration step thus keeping the original y value
    x = add(y, z, ||return [])      // cause merge to stop and return [] instead
    x = add(y, z, ||throw Err("!" ) // exits the whole situation
    x = add(y, z, ||"conflict!")    // ignores both x and y and instead uses constant value
    x = add(x, y, ||revert)         // Specifies operation is atomic, and reverts the merge on conflict

_
## Uniform Code Samples
### _

<pre><smcode><!-- code uniform -->
// NOTE: <b>ALL</b> data here maps onto a single recursive unit-expression as shown above.
pkg examples.sample_code::
  Person::
    name: "Dan Oblinger"        # Indention can be used to indicate the end of an embedded object
    funky string:- like YAML, string do not need back quoting.  even
       multi-line strings with "quotes" and 'quotes' and back quotes \ are just fine.
    home:
      Address::
        "3403 Cesar Chavez street"
        Apt  :- C
        City :- San Francisco
        State:- CA
        zip  :- 94110
        email:- dan@oblinger.us
    friends: ["Qingling Ni", "Jack Porter", "Josh Yelon",
        Person::  name: "Inline-defined Person"                         # Fourth friend is an in-line defined object
          home: Address::
            "1234 inline street address"
            City :- Some City
            State:- CA
            Zip  :- 12345
        ]
    good_old_JSON_still_works_here:
        {"equation": ["equals", "e", ["times" "m" ["squared", "c"]]]}   # Uniform is a superset of JSON

    theorems:
        a**2 + b**2 == c**2     # This is not a string, it is a recursive uniform code expression with inline operators
                                # Modern procedural language differ on which infix operators they include, but all language agree
                                # on the PRECEDENCE between these operators...  so we have just included them ALL in Uniform

    // Notice you can mix-in many commenting styles
    some_uniform_code_that_looks_like_Javascript:: {
        function factorial(x) {
            if (x<=1) {
                return 1;
            } else {
                return x * factorial(x-1);
        }
    }

    ### Here is the same unit expression expressed Pythonically
    equal_expression::
        def factorial(x):
            if x<=1:
                return 1
            else:
                return x * factorial(x-1)

    /* Of course the Java-ish expression will not be exactly equal units, since it must also supply type indicators */
    java-ish:: {
        public static final factorial(int i) {
            if (x<=1) { return 1; }
            else { return x * factorial(x-1); } }

    # Similarly C, C++, and Bash script also fit into uniform.

    // But so do other languages that are tuned for commonly specialized form that often look like
    // garbage when embedded in existing languages:

    Simple_Templating_Formats::
        ' $head( comment: "A template form", $var: "with embedded",  template: $$values )

    Destructuring_Templates::
        ' $this($a, sub(b:$b)) <== $that($aa, $bb) && { $this = some_computation_to_expand($that) }

    String_Templates::
        [`this {num*1000} words.`str,       # 'Format' style string template
         `this [0-9]* words.`re ]           # Regular expressions... if you must.

    # Finally if one wants to include another language as a large string; generally we do NOT need to add backquoting
    java_8_source_code_as_string:: ```   /*  Notice here we are embedding the source code of another language */
        import java.lang;
        package com.my.package;
        public static final EmbeddedMethodDefinition(String arg1, int arg2) {
            system.out.println("Just a bit of Java here");
        }
    ```java8

</smcode></pre>


<pre><smcode>

// Provides examples of Uniform markdown
pkg uf.examples::


    Unit::
        import uniform.unicore.Unit;
        import UnitDefOps;

    UnitDefOps::
        fn: macro()
        macro:: GND::
            spec:  form(import(fn_argspec)) 
            eval:  GND(null, "com.xcorp.uniform.macro")
            asJS:  


    fn_argbody:
        ...form()

        
            

lexlang.import_types:
    or(pkg, fn, macro, block)     // maybe 'block' should not be here

lexlang.import_list:
    fn():
        for ancestor in self.ancestors().typed(import_types()).shape(), out {}:
            result += ancestor.import

lexlang.bindings:
    fn():
        for import in lex_imports(), output bindings = {}:
            bindings += import


========

lexlang.shape:
    fn():
        memoization( parts: copy() ):


lexlang.import_list:
    fn():
        memoization( imports: self.ancestors().typed(import_types()).shape() ):
            for ancestor in imports:
                result += ancestor.import


lexlang.bindings:
    fn(imports:mem(lex_imports())):
        for import in lex_imports(), out bindings = {}:
            bindings += import



_
###  Default Semantics  ###

    Form::
        bindings:= memoize


    // BINDING_SEMANTICS --
    DefaultSemantics::

        // COLLECT_SEMANTICS_FORMS -- Scans pkg and combines all import statements together
        collect_semantic_forms:= fn(semantic_indicator)::
            for stmt in self.fixed out=[]:
                if resolve(stmt.head):
                    out.append(stmt)

        // COMPUTE_BINDINGS --
        compute_bindings:= fn()::
            List(Ref) inheritance_list = reverse(merge_stmts(`extends))
            for ref in inheritance_list {}->out:
                out += resolve(ref).exports


        // SCOPE -- Returns the bindings
        scope: fn(f Form):
_
### PKG
    <p> Unicore provides default Binding semantics based on the <code>PKG</code> Form above, and used to derive
        <code>Bindings</code> and <code>Structure</code> from its statement forms.  Of course these may be used in
        turn to define other declaration forms with other semantics.  We describe these default semantics in English
        followed by a more precise specification as a Uniform Script.</p>
    <ul>
        <li>The <code>PKG</code> statement accepts a single argument that specifies the structural path
            to follow from <code>lexroot</code> to find the Lex whose Bindings should be controlled by this Pkg.</li>
        <li>The <code>import</code> statement uses the Bindings controlling the <code>pkg</code> code form to
            <code>resolve</code> all of its passed references into either their Binding or Lex meanings.  Those
            meanings are then combined in import statement order in order to derive the pkg's Bindings.
            Keyword named references <v>key<sub>a</sub></v> will result in a Bindings that map the specified keyword
            onto the Bindings <code>exported</code> from the reference Lex place.
            fixed references <code><v>ref<sub>0</sub></v>, ...</code> on the other hand will merge all
            exported Bindings directly into the resulting Bindings Unit.</li>
        <li>The <code>export</code> statement specifies which bindings should be imported into <i>other</i> pkgs that
            import this package.  Each <v>ref</v> is treated as a <i>PATH</i> and is <code>path_follow</code>ed from
            the Lex containing the pkg source form to find the Lex indicated by this <v>ref</v>, or it is
            <code>resolve</code>d from the imported Bindings if it does not lexically (structurally) found.
            References that are named by <v>key<sub>i</sub></v> are added to the export list under the specified key
            name, rather than their local name in Bindings.</li>
        <li>The Structure derived from a pkg form is simply the concatenation of all <code>var</code> statements that
            are immediate sub-units of the pkg form itself.</li>
        <li>The symbol <code>type</code> is bound to the <code>Lex</code> containing the PKG source Form.
            So in Uniform a Unit type <i>IS</i> a place &mdash; the place where its Bindings came from.</li>
    </ul>
### Unicore Tree
    pkg uniform.unicore;

    Unit: PKG::
        import UnitBackingOps;
        import UnitDefiningOps;
        import UnitUniversalOps;
        import UnitSemanticOps;
        import UnitInterpretationOps;


    UnitBackingOps: PKG::
        GET:    fn(Key k, ->Unit)
        SET:    fn(Key k, Unit v)
        ITR:    macro(u Unit, k Key, Keymap km, ->Iterator)  //  Returns an iterator that edits vars as a side effect
        LEN:    fn(->Int||null||inf)
        UP:     fn(->Unit)
        IDX:    fn(->key)

    UnitDefiningOps: PKG::
        head:   fn( ->SYM, <-SYM)               //  Head is gettable / settable symbol that may serve as a type designator
        body:   fn(<->list(Unit))               //  Note the short hand for specifying the get/set type
        type:   fn( ->Lex)                      //  ??? should this return the Lex where the pkg is?
        lex:    fn( ->Lex)
        __:
        ___:

    UnitUniversalOps: PKG::
        str:    fn( ->print_form str)
        eq:     fn(other Unit, ->bool)
        equals: fn(other Unit, ->bool)
        less:
        get_path:
        follow_path:
        __vars__:


    // UNICORE CONSTANT TYPES
    Int:    spec(form(str print_form, ...none, ...:none, h:SYM("INT"))   //  Int has form:  INT("xxx")
    Num:    spec(form(str print_form, ...none, ...:none, h:SYM("NUM"))   //  Num has form:  NUM("xxx")
    Str:    spec(form(str print_form, ...none, ...:none, h:SYM("STR"))   //  Str has form:  STR("xxx")
    Sym:    spec(form(...none, ...:none, ^sym:`true))

    Key:    spec(Int||Sym)                    //  in some contexts a negative integer key is not allowed while others allow it.
    Bool:   spec(`true||`false)
    Path:   spec( form(...key, ...:none) )
    Ref:    spec( form(...key, ...:none) )

    // SYMBOLS WITH SPECIALIZED MEANING
    symbols::
        true:
        false:
        null:
        und:
        nan:  // Nan
        alt:
        inf:
        minus_inf:

    //  UNIFORM TYPES
    types::
        Pairs: spec( list(form(Unit, Unit, None...)) )      // A list of two element





    Bindings: PKG::
        GET: fn(key k, ->Unit)
        resolve: fn(ref r, ->Lex)


    Form: PKG::
        bindings: fn(->Bindings)

    FormControlOps: PKG::
        BLK: form(...Form, ...:none)
        BRA: form(input Form, ...Spec(condition Form, ...actions Form, h:"->"))
        DOT: form(ref Form, ...form Form)
        REP: form(condition Form, ...body Form)
        RET: form(tag SYM, ret_val Form)

    FormDeclarationOps: PKG::
        GND: form(...)
        DEF:
        PKG:
        TRY:
        VAR:
    FormGroundingOps: PKG::
        EXE:
        SAT:
        ALT:



    Env: PKG::
        bang: fn()




    # Grounding Uniform code forms for direct interpretation
    #
    # NOTE: Here is a safe form of monkey patching.  Static code analysis can easily determine the relation between source code and lexspace.
    package(FormOps)::

        BLK.__eval__: fn(__args__:args list(Forms))::
            for a in args: EXE(a)

        BRA.__eval__:



    # Here we provide the Javascript compiler bindings
    package(FormOps)::

        BLK.__asJS__: fn()





    # Caches the Structure Unit derived which only depends upon the structure of the pkg form itself
    structure: <b>cache</b>( pkg_form:self )::
        <b>return</b> Structure( pkg_form.select{ head(var) } )

    bindings: fn()::


    //insert pkg operator definitions here

_
### pkg std.lang.unicore

    # defines an interpretation universe
    defclass Alpha($FORM, $UNIT):
        var alpha Unit
        var forms list $FORM
        var heap  str->Store        #
        var reduce                  #  the
        var root Lex                #  the root of all 'places'

        op bang(ops)                #  Creates a new computational universe instance
        op EXE($FORM)               #  Interprets some form in the base environment defined by Alpha

    # defines a single (stack) frame of reference within an interpretaton
    defclass Env:
        var ^UP  Env
        var ^ops Ops
        var self Unit
        # other local variables

    defclass Lex:
        var ^UP Lex
        in  Unit        # Lex may take any unit as a key in the most general case (????)
        out Lex         # All lex point to other Lex as children


    ### EXE(form0, ..., env:, alpha:, )
    deffn interp(forms $FORM..., alpha:Unit):
        # copy forms into alpha
        h = alpha.forms[-1].head
        while h!='GND and h!='DEF:
            if h=='OP:
                next = h[alpha]
            else:






    ### NEW(class_path, args...)




::
    pkg std.lang.unicore.declarations


::
    pkg std.lang.unicore.programming_model

    // DECLARATIONS
    def var lang.syntax Bag(Token) 
    def var lang.normalization Ctx


    // IMPLEMENTATION
    def parse(txt):
        $form Data = lang.impl.text2data(lang.syntax)
        return eval(form, ctx:lang.normalization)

    def compile(form):
        $expansion 



::
    pkg std.lang.unicore

    def macro eval(form):: `interp($form.as(Code), alpha:`eval)
    def macro compile(...form)

    def class Interpreter::
        def interp(...forms Code, | alpha Unit, env Env, continue Bool)
        def alpha_set_base
        def 
_
### pkg lib.2019.01.unit_from_markdown::
    lang uf;

    defOp header_level(line):
        prefix = line =~ re"^(#*)/s/S"
        return len( () [0] )


    
    defOp markdown2unit(source):
        str = source.as(Str)