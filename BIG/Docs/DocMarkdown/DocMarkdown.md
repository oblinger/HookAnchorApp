
A [reference to example link][id]. This would be followed later by:   [id]: http://example.com/  "Title"

Inline (titles are optional):
![an inline image](/path/img.jpg "Title")
![a referenced image][id] would be followed later by:
[id]: /url/to/img.jpg "Title"

# Header 1 #    Header 1        
                ========
## Header 2 ##  Header 2         (closing #'s are optional)
                --------
### H3 ###      {#link-id-goes-here}
<h3 id="link-id-goes-here">Header 3</h3>
LINK   [Link back to H3](#link-id-goes-here)     
       An inline link [link text](http://url.com/ "hover")   -- Links/images include the [alt-]text in preceding brackets:

FONT   *italic*   **bold**   _italic_   __bold__




> Blockquotes are like quoted text in email replies
>> And, they can be nested

[BULLETS-LISTS]
1.  First ordered list item
2.  Second item


---- ON GIT HUB -----

| Tables        | Are           | Cool  |
| ------------- |:-------------:| -----:|
| col 3 is      | right-aligned | $1600 |
| col 2 is      | centered      |   $12 |
| zebra stripes | are neat      |    $1 |



# Docs
  http://daringfireball.net/projects/markdown/syntax

  https://raw.github.com/gist/976172/70f1e0db278340bd8167c98fb880979b4571e847/gistfile1.md
http://support.mashery.com/docs/customizing_your_portal/Markdown_Cheat_Sheet/
Extra formats?   http://warpedvisions.org/projects/markdown-cheat-sheet/
Sandbox          http://daringfireball.net/projects/markdown/dingus




# Summary
  https://raw.github.com/gist/976172/70f1e0db278340bd8167c98fb880979b4571e847/gistfile1.md

# Header 1 #
## Header 2 ##
### Header 3 ###             (Hashes on right are optional)
#### Header 4 ####
##### Header 5 #####

## Markdown plus h2 with a custom ID ##        
[Link back to H2](#id-goes-here)

This is a paragraph, which is text surrounded by whitespace. Paragraphs can be on one 
line (or many), and can drone on for hours.  

Here is a Markdown link to [Warped](http://warpedvisions.org), and a literal . 
Now some SimpleLinks, like one to [google] (automagically links to are-you-
feeling-lucky), a [wiki: test] link to a Wikipedia page, and a link to 
[foldoc: CPU]s at foldoc.  

Now some inline markup like _italics_,  **bold**, and `code()`. Note that underscores in 
words are ignored in Markdown Extra.

![picture alt](/images/photo.jpeg "Title is optional")     

> Blockquotes are like quoted text in email replies
>> And, they can be nested

# Bullet lists are easy too
- Another one
+ Another one

1. A numbered list
2. Which is numbered
3. With periods and a space

And now some code:

    // Code is just text indented a bit
    which(is_easy) to_remember();

~~~

// Markdown extra adds un-indented code blocks too

if (this_is_more_code == true && !indented) {
    // tild wrapped code blocks, also not indented
}

~~~

Text with  
two trailing spaces  
(on the right)  
can be used  
for things like poems  

### Horizontal rules

  * * * *
  ****
  --------------------------


<div class="custom-class" markdown="1">
This is a div wrapping some Markdown plus.  Without the DIV attribute, it ignores the 
block. 
</div>

## Markdown plus tables ##

| Header | Header | Right  |
| ------ | ------ | -----: |
|  Cell  |  Cell  |   $10  |
|  Cell  |  Cell  |   $20  |

# Outer pipes on tables are optional
# Colon used for alignment (right versus left)

## Markdown plus definition lists ##

Bottled water
: $ 1.25
: $ 1.55 (Large)

Milk
Pop
: $ 1.75

# Multiple definitions and terms are possible
# Definitions can include multiple paragraphs too

#[ABBR]: Markdown plus abbreviations (produces an <abbr> tag)


#   An unordered list item.

    With multiple paragraphs.

#   Another unordered list item
    1. With a nested item below it

> Email-style angle brackets
> are used for blockquotes.

> > And, they can be nested.

> #### Headers in blockquotes
> 
> * You can quote a list.
> * Etc.

`<code>` spans are delimited by backticks.  You can include literal
backticks like `` `this` ``.

    This is a preformatted code block. It must be indented
    either by at least four spaces or a tab


This is a normal <p>paragraph</p>.


Three or more dashes or asterisks make a horizontal rule:
---
    * * *
    - - - -

[BLOCKQUOTES & CODESPANS]
> Email-style angle brackets
> are used for blockquotes.

`<code>` spans are delimited by backticks.
You can include literal backticks like `` `this` ``.

This is a normal paragraph.
    This is a preformatted code block.  
    Indent every line of a code block 
    by at least 4 spaces or 1 tab.

[HORIZONTAL RULE]
---
# * *
- - - - 

[TABLES]
Name    |   Age
--------|------
Fred    |   29
Jim     |   47
Harry   |   32

[NOTES]
- End a line with two or more  
  spaces to do a manual line break.


=======================================================

[BLOCKQUOTES]
> Email-style angle brackets
> are used for blockquotes.

> > And, they can be nested.

> #### Headers in blockquotes
> 
> * You can quote a list.
> * Etc.
