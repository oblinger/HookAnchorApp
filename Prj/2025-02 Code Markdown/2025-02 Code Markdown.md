


Plan:
- Parse Syntax
- Write Tokenizer (w/o indention)
- Build basic parser for + * and ( )





TASK: 
Write a parser that reads UNIT_MARKDOWN_GRAMMAR and parses it into a list of Syntax objects.
- Each line represents a unique precedence level from highest to lowest
- The "###" marks on some lines indicate a comment to the end of line, they are not parsed
- Whitespace is used to separate each syntax specs from others on the same page
- Each syntax spec has this format:   TTTT_FLAGS_CLOSE
	- FLAGS is a comma-separated list of alphanumeric keywords with at least one keyword which is the 'type' of the syntax element.
	  Syntax types include: infix, group, and comment.  
	- If a syntax spec has no "_" then the whole spec is the token, and it is assumed to be infix.
	- CLOSE is an optional closing brace indicated by a second "_" after the flags
	- TTT, are they token characters, which can be any character, including " _ " except space
	- (NAMED) 