# Python.Rexpr --

    compile(pattern[, flags])  
    Compile a regular expression pattern into a regular expression object, which can be used for matching using its match() and search() methods, described below.  
     
    re.compile("a").match("ba", 1)           # succeeds 
    re.compile("^a").search("ba", 1)         # fails; 'a' not at start 
    re.compile("^a").search("\na", 1)        # fails; 'a' not at start 
    re.compile("^a", re.M).search("\na", 1)  # succeeds 
    re.compile("^a", re.M).search("ba", 1)   # fails; no preceding \n 
     
     
    search(string[, pos[, endpos]])  
    Scan through string looking for a location where this regular expression produces a match, and return a corresponding MatchObject instance. Return None if no position in the string matches the pattern; note that this is different from finding a zero-length match at some point in the string.  
    The optional pos and endpos parameters have the same meaning as for the match() method.  
     
     
    match(string[, pos[, endpos]])  
    If zero or more characters at the beginning of string match this regular expression, return a corresponding MatchObject instance. Return None if the string does not match the pattern; note that this is different from a zero-length match.  
    Note: If you want to locate a match anywhere in string, use search() instead.  
     
    The optional second parameter pos gives an index in the string where the search is to start; it defaults to 0. This is not completely equivalent to slicing the string; the '^' pattern character matches at the real beginning of the string and at positions just after a newline, but not necessarily at the index where the search is to start.  
     
    The optional parameter endpos limits how far the string will be searched; it will be as if the string is endpos characters long, so only the characters from pos to endpos will be searched for a match.  
     
     
    split(string[, maxsplit = 0])  
    Identical to the split() function, using the compiled pattern.  
     
    findall(string)  
    Identical to the findall() function, using the compiled pattern.  
     
    finditer(string)  
    Identical to the finditer() function, using the compiled pattern.  
     
    sub(repl, string[, count = 0])  
    Identical to the sub() function, using the compiled pattern.  
     
    subn(repl, string[, count = 0])  
    Identical to the subn() function, using the compiled pattern.  
     
    flags  
    The flags argument used when the RE object was compiled, or 0 if no flags were provided.  
     
    groupindex  
    A dictionary mapping any symbolic group names defined by (?P) to group numbers. The dictionary is empty if no symbolic groups were used in the pattern.  
     
    pattern  
    The pattern string from which the RE object was compiled.  
     
     
    The special characters are:  
     
     
    "."  
    (Dot.) In the default mode, this matches any character except a newline. If the DOTALL flag has been specified, this matches any character including a newline.  
     
    "^"  
    (Caret.) Matches the start of the string, and in MULTILINE mode also matches immediately after each newline.  
     
    "$"  
    Matches the end of the string or just before the newline at the end of the string, and in MULTILINE mode also matches before a newline. foo matches both 'foo' and 'foobar', while the regular expression foo$ matches only 'foo'. More interestingly, searching for foo.$ in 'foo1\nfoo2\n' matches 'foo2' normally, but 'foo1' in MULTILINE mode.  
     
    "*"  
    Causes the resulting RE to match 0 or more repetitions of the preceding RE, as many repetitions as are possible. ab* will match 'a', 'ab', or 'a' followed by any number of 'b's.  
     
    "+"  
    Causes the resulting RE to match 1 or more repetitions of the preceding RE. ab+ will match 'a' followed by any non-zero number of 'b's; it will not match just 'a'.  
     
    "?"  
    Causes the resulting RE to match 0 or 1 repetitions of the preceding RE. ab? will match either 'a' or 'ab'.  
     
    *?, +?, ??  
    The "*", "+", and "?" qualifiers are all greedy; they match as much text as possible. Sometimes this behaviour isn't desired; if the RE <.*> is matched against 'title<<<>>>', it will match the entire string, and not just ''. Adding "?" after the qualifier makes it perform the match in non-greedy or minimal fashion; as few characters as possible will be matched. Using .*? in the previous expression will match only ''.  
     
    <<>>  
    Specifies that exactly m copies of the previous RE should be matched; fewer matches cause the entire RE not to match. For example, a<<>> will match exactly six "a" characters, but not five.  
     
    <<>>  
    Causes the resulting RE to match from m to n repetitions of the preceding RE, attempting to match as many repetitions as possible. For example, a<<>> will match from 3 to 5 "a" characters. Omitting n specifies an infinite upper bound; you can't omit m. As an example, a<<>>b will match aaaab, a thousand "a" characters followed by a b, but not aaab. The comma may not be omitted or the modifier would be confused with the previously described form.  
     
    <<>>?  
    Causes the resulting RE to match from m to n repetitions of the preceding RE, attempting to match as few repetitions as possible. This is the non-greedy version of the previous qualifier. For example, on the 6-character string 'aaaaaa', a<<>> will match 5 "a" characters, while a<<>>? will only match 3 characters.  
     
    "\"  
    Either escapes special characters (permitting you to match characters like "*", "?", and so forth), or signals a special sequence; special sequences are discussed below.  
    If you're not using a raw string to express the pattern, remember that Python also uses the backslash as an escape sequence in string literals; if the escape sequence isn't recognized by Python's parser, the backslash and subsequent character are included in the resulting string. However, if Python would recognize the resulting sequence, the backslash should be repeated twice. This is complicated and hard to understand, so it's highly recommended that you use raw strings for all but the simplest expressions.  
     
     
    []  
    Used to indicate a set of characters. Characters can be listed individually, or a range of characters can be indicated by giving two characters and separating them by a "-". Special characters are not active inside sets. For example, [akm$] will match any of the characters "a", "k", "m", or "$"; [a-z] will match any lowercase letter, and [a-zA-Z0-9] matches any letter or digit. Character classes such as \w or \S (defined below) are also acceptable inside a range. If you want to include a "]" or a "-" inside a set, precede it with a backslash, or place it as the first character. The pattern []] will match ']', for example.  
    You can match the characters not within a range by complementing the set. This is indicated by including a "^" as the first character of the set; "^" elsewhere will simply match the "^" character. For example, [^5] will match any character except "5", and [^^] will match any character except "^".  
     
     
    "|"  
    A|B, where A and B can be arbitrary REs, creates a regular expression that will match either A or B. An arbitrary number of REs can be separated by the "|" in this way. This can be used inside groups (see below) as well. REs separated by "|" are tried from left to right, and the first one that allows the complete pattern to match is considered the accepted branch. This means that if A matches, B will never be tested, even if it would produce a longer overall match. In other words, the "|" operator is never greedy. To match a literal "|", use \|, or enclose it inside a character class, as in [|].  
     
    (...)  
    Matches whatever regular expression is inside the parentheses, and indicates the start and end of a group; the contents of a group can be retrieved after a match has been performed, and can be matched later in the string with the \number special sequence, described below. To match the literals "(" or ")", use \( or \), or enclose them inside a character class: [(] [)].  
     
    (?...)  
    This is an extension notation (a "?" following a "(" is not meaningful otherwise). The first character after the "?" determines what the meaning and further syntax of the construct is. Extensions usually do not create a new group; (?P...) is the only exception to this rule. Following are the currently supported extensions.  
     
    (?iLmsux)  
    (One or more letters from the set "i", "L", "m", "s", "u", "x".) The group matches the empty string; the letters set the corresponding flags (re.I, re.L, re.M, re.S, re.U, re.X) for the entire regular expression. This is useful if you wish to include the flags as part of the regular expression, instead of passing a flag argument to the compile() function.  
    Note that the (?x) flag changes how the expression is parsed. It should be used first in the expression string, or after one or more whitespace characters. If there are non-whitespace characters before the flag, the results are undefined.  
     
     
    (?:...)  
    A non-grouping version of regular parentheses. Matches whatever regular expression is inside the parentheses, but the substring matched by the group cannot be retrieved after performing a match or referenced later in the pattern.  
     
    (?P...)  
    Similar to regular parentheses, but the substring matched by the group is accessible via the symbolic group name name. Group names must be valid Python identifiers, and each group name must be defined only once within a regular expression. A symbolic group is also a numbered group, just as if the group were not named. So the group named 'id' in the example above can also be referenced as the numbered group 1.  
    For example, if the pattern is (?P[a-zA-Z_]\w*), the group can be referenced by its name in arguments to methods of match objects, such as m.group('id') or m.end('id'), and also by name in pattern text (for example, (?P=id)) and replacement text (such as \g).  
     
     
    (?P=name)  
    Matches whatever text was matched by the earlier group named name.  
     
    (?#...)  
    A comment; the contents of the parentheses are simply ignored.  
     
    (?=...)  
    Matches if ... matches next, but doesn't consume any of the string. This is called a lookahead assertion. For example, Isaac (?=Asimov) will match 'Isaac ' only if it's followed by 'Asimov'.  
     
    (?!...)  
    Matches if ... doesn't match next. This is a negative lookahead assertion. For example, Isaac (?!Asimov) will match 'Isaac ' only if it's not followed by 'Asimov'.  
     
    (?<=...)  
    Matches if the current position in the string is preceded by a match for ... that ends at the current position. This is called a positive lookbehind assertion. (?<=abc)def will find a match in "abcdef", since the lookbehind will back up 3 characters and check if the contained pattern matches. The contained pattern must only match strings of some fixed length, meaning that abc or a|b are allowed, but a* and a<<>> are not. Note that patterns which start with positive lookbehind assertions will never match at the beginning of the string being searched; you will most likely want to use the search() function rather than the match() function:  
     
    >>> import re 
    >>> m = re.search('(?<=abc)def', 'abcdef') 
    >>> m.group(0) 
    'def' 
     
    This example looks for a word following a hyphen:  
     
     
    >>> m = re.search('(?<=-)\w+', 'spam-egg') 
    >>> m.group(0) 
    'egg' 
     
     
    (?.) Inside the "[" and "]" of a character class, all numeric escapes are treated as characters.  
     
    \A  
    Matches only at the start of the string.  
     
    \b  
    Matches the empty string, but only at the beginning or end of a word. A word is defined as a sequence of alphanumeric characters, so the end of a word is indicated by whitespace or a non-alphanumeric character. Inside a character range, \b represents the backspace character, for compatibility with Python's string literals.  
     
    \B  
    Matches the empty string, but only when it is not at the beginning or end of a word.  
     
    \d  
    Matches any decimal digit; this is equivalent to the set [0-9].  
     
    \D  
    Matches any non-digit character; this is equivalent to the set [^0-9].  
     
    \s  
    Matches any whitespace character; this is equivalent to the set [ \t\n\r\f\v].  
     
    \S  
    Matches any non-whitespace character; this is equivalent to the set [^ \t\n\r\f\v].  
     
    \w  
    When the LOCALE and UNICODE flags are not specified, matches any alphanumeric character; this is equivalent to the set [a-zA-Z0-9_]. With LOCALE, it will match the set [0-9_] plus whatever characters are defined as letters for the current locale. If UNICODE is set, this will match the characters [0-9_] plus whatever is classified as alphanumeric in the Unicode character properties database.  
     
    \W  
    When the LOCALE and UNICODE flags are not specified, matches any non-alphanumeric character; this is equivalent to the set [^a-zA-Z0-9_]. With LOCALE, it will match any character not in the set [0-9_], and not defined as a letter for the current locale. If UNICODE is set, this will match anything other than [0-9_] and characters marked at alphanumeric in the Unicode character properties database.  
     
    \Z  
    Matches only at the end of the string.  
     
    Most of the standard escapes supported by Python string literals are also accepted by the regular expression parser:  
     
     
    \a      \b      \f      \n 
    \r      \t      \v      \x 
    \\ 
     
    Note that octal escapes are not included. While the parser can attempt to determine whether a character is being specified by it's ordinal value expressed in octal, doing so yields an expression which is relatively difficult to maintain, as the same syntax is used to refer to numbered groups.  
     
     
