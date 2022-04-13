  [__DocHtml__](__DocHtml__.md)



DOCS:
# Docs
list    html 5       http://webdesignledger.com/tutorials/15-useful-html5-tutorials-and-cheat-sheets

Visible Examples:    http://www.quackit.com/html/html_cheat_sheet.cfm
Try it Tool:         http://www.w3schools.com/tags/tryit.asp?filename=tryhtml_label

## My Notes

### <div align="left"> A generic tag used to format large blocks of HTML, also used for stylesheets
  id='UniqueNameOnPage'
  class='LogicalClassOfDiv'


## Basics            http://www.webmonkey.com/2010/02/html_cheatsheet/  
### Basic Tags
<html></html> Creates an HTML document

<head></head> Sets off the title and other information that isn’t displayed on the web page itself

<body></body> Sets off the visible portion of the document

Body Attributes
<body bgcolor="pink"> Sets the background color, using name or hex value

<body text="black"> Sets the text color, using name or hex value

<body link="blue"> Sets the color of links, using name or hex value

<body vlink="#ff0000"> Sets the color of followed links, using name or hex value

<body alink="#00ff00"> Sets the color of links on click

<body ondragstart="return false" onselectstart="return false"> Disallows text selection with the mouse and keyboard

### Text Tags
<pre></pre> Creates preformatted text

<hl></hl> Creates the largest headline

<h6></h6> Creates the smallest headline

<b></b> Creates bold text

<i></i> Creates italic text

<tt></tt> Creates teletype, or typewriter-style text

<cite></cite> Creates a citation, usually italic

<em></em> Emphasizes a word (with italic or bold)

<strong></strong> Emphasizes a word (with italic or bold)

<font size="3"></font> Sets size of font, from 1 to 7

<font color="green"></font> Sets font color, using name or hex value

### Links
<a href="URL"></a> Creates a hyperlink

<a href="mailto:EMAIL"></a> Creates a mailto link

<a href="URL"><img src="URL"> </a> Creates an image/link

<a name="NAME"></a> Creates a target location within a document

<a href="#NAME"></a> Links to that target location from elsewhere in the document

### Formatting
<p></p> Creates a new paragraph

<p align="left"> Aligns a paragraph to the left (default), right, or center.

<br> Inserts a line break

<blockquote></blockquote> Indents text from both sides

<dl></dl> Creates a definition list

<dt> Precedes each definition term

<dd> Precedes each definition

<ol></ol> Creates a numbered list

<ul></ul> Creates a bulleted list

<li></li> Precedes each list item, and adds a number or symbol depending upon the type of list selected

<div align="left"> A generic tag used to format large blocks of HTML, also used for stylesheets

<img src="name"> Adds an image

<img src="name" align="left"> Aligns an image: left, right, center; bottom, top, middle

<img src="name" border="1"> Sets size of border around an image

<hr /> Inserts a horizontal rule

<hr size="3" /> Sets size (height) of rule

<hr width="80%" /> Sets width of rule, in percentage or absolute value

<hr noshade /> Creates a rule without a shadow

### Tables
<table></table> Creates a table

<tr></tr> Sets off each row in a table

<td></td> Sets off each cell in a row

<th></th> Sets off the table header (a normal cell with bold, centered text)

### Table Attributes
<table border="1"> Sets width of border around table cells

<table cellspacing="1"> Sets amount of space between table cells

<table cellpadding="1"> Sets amount of space between a cell’s border and its contents

<table width="500" or "80%"> Sets width of table, in pixels or as a percentage of document width

<tr align="left"> or <td align="left"> Sets alignment for cell(s) (left, center, or right)

<tr valign="top"> or <td valign="top"> Sets vertical alignment for cell(s) (top, middle, or bottom)

<td colspan="2"> Sets number of columns a cell should span (default=1)

<td rowspan="4"> Sets number of rows a cell should span (default=1)

<td nowrap> Prevents the lines within a cell from being broken to fit

### Frames
<frameset></frameset> Replaces the <body> tag in a frames document; can also be nested in other framesets

<frameset rows="value,value"> Defines the rows within a frameset, using number in pixels, or percentage of width

<frameset cols="value,value"> Defines the columns within a frameset, using number in pixels, or percentage of width

<frame> Defines a single frame — or region — within a frameset

<noframes></noframes> Defines what will appear on browsers that don’t support frames

### Frames Attributes
<frame src="URL"> Specifies which HTML document should be displayed

<frame name="name"> Names the frame, or region, so it may be targeted by other frames

<frame marginwidth="value"> Defines the left and right margins for the frame; must be equal to or greater than 1

<frame marginheight="value"> Defines the top and bottom margins for the frame; must be equal to or greater than 1

<frame scrolling="value"> Sets whether the frame has a scrollbar; value may equal “yes,” “no,” or “auto.” The default, as in ordinary documents, is auto.

<frame noresize="noresize"> Prevents the user from resizing a frame

### Forms
For functional forms, you’ll have to run a script. The HTML just creates the appearance of a form.

<form></form> Creates all forms

<select multiple name="NAME" size=?></select> Creates a scrolling menu. Size sets the number of menu items visible before you need to scroll.

<option> Sets off each menu item

<select name="NAME"></select> Creates a pulldown menu

<option> Sets off each menu item

<textarea name="NAME" cols=40 rows=8></textarea name> Creates a text box area. Columns set the width; rows set the height.

<input  type="hidden"   name="token" value="234234">
<input  type="checkbox" name="NAME"> Creates a checkbox. Text follows tag.
<input  type="radio"    name="NAME" value="x"> Creates a radio button. Text follows tag
<input  type="text"     name="NAME" size=20> Creates a one-line text area. Size sets length, in characters.
<input  type="submit"   value="NAME"> Creates a Submit button
<button type="submit">  Submit</button> Creates an actual button that is clicked
<input  type="image"    border=0 name="NAME" src="name.gif"> Creates a Submit button using an image
<input  type="reset">   Creates a Reset button


## below:            http://www.netlingo.com/tips/html-code-cheat-sheet.php

<!DOCTYPE>       NoAttr	*  NoClose
<HTML>           NoAttr	*  ReqClose
<HEAD>           NoAttr	*  ReqClose
<TITLE>          NoAttr	*  ReqClose
<META>           content, http-equiv, name, scheme	*  NoClose
<STYLE>          NoAttr	NN4, IE45  ReqClose
<BODY>           alink, background, bgcolor, bgproperties, leftmargin, link, text, topmargin, vlink	*  ReqClose

<P>              align	*  ReqClose
<BR>             clear	*  ReqClose
<HR>             align, noshade, size, width	*  NoClose
<OL>             compact, start, type	*  ReqClose
<UL>             compact, type	*  ReqClose
<LI>             type, vlaue	*  ReqClose
<DT>             NoAttr	*  NoClose
<DD>             NoAttr	   NoClose

<ADDRESS>        NoAttr	*  ReqClose
<BLOCKQUOTES>    NoAttr	*  ReqClose
<CENTER>         NoAttr	*  ReqClose
<!-- -->         NoAttr	*  ReqClose
<COMMENT>        NoAttr	IE345  ReqClose
<DIV>            align	NN234, IE345  ReqClose
<PRE>            width	*  ReqClose
<B>              NoAttr	*  ReqClose
<BASEFONT>       color, face, size	*  NoClose
<BIG>            NoAttr	NN234, IE345  ReqClose
<BLINK>          NoAttr	NN234  ReqClose
<CITE>           NoAttr	*  ReqClose
<CODE>           NoAttr	*  ReqClose
<DFN>            NoAttr	   ReqClose
<EM>             NoAttr	*  ReqClose
<FONT>           color, face, size	*  ReqClose
<I>              NoAttr	*  ReqClose
<KBD>            NoAttr	NN234, IE245  ReqClose
<S>              NoAttr	NN34, IE45  ReqClose
<STRIKE>         NoAttr	NN34, IE2345  ReqClose
<SAMP>           NoAttr	NN234, IE245  ReqClose
<SMALL>          NoAttr	NN234, IE245  ReqClose
<STRONG>         NoAttr	*  ReqClose
<SUB>            NoAttr	NN234, IE345  ReqClose
<SUP>            NoAttr	NN234, IE345  ReqClose
<TT>             NoAttr	*  ReqClose
<U>              NoAttr	*  ReqClose
<IMG>            align, alt, border, height, hspace, ismap, src, usemap, vspace, width, dynsrc, start, controls, loop, loopdelay	*  NoClose
<A>              href, name, target, title, method, rel, rev, urn	*  ReqClose
<MAP>            name	*  ReqClose
<AREA>           shape, coords, href, nohref	*	 
<FORM>           action, enctype, method	*  ReqClose
<INPUT>          type, name, value, align, checked, maxlength, size, src	*  ReqClose
<SELECT>         name, multiple, size	*  ReqClose
<OPTION>         selected, value	*  ReqClose
<TABLE>          align, background, bgcolor, border, bordercolor, bordercolorlight, bordercolordark, cellspacing, cellpadding, cols, frame, height, rules, style, width	*  ReqClose
<TR>             align, valign, backgroundcolor, bgcolor, bordercolor, bordercolorlight, bordercolordark, height, style	*  ReqClose
<TD>             align, colspan, rowspan, nowrap, background, bgcolor, brodercolor, bordercolorlight, bordercolordark, height, width, style, valign	*  ReqClose
<CAPTION>        align, valign	*  ReqClose
<FRAMESET>       cols, rows, frameborder, framespacing, bordercolor	NN234, IE345  ReqClose
<FRAME>          frameborder, framespacing, marginwidth, marginheight, name, noresize, scrolling, src, bordercolor	NN234, IE345  NoClose
<NOFRAMES>       NoAttr	NN234, IE345  ReqClose
<IFRAME>         width, height, hspace, vspace, align, frameborder, marginheight, marginwidth, noresize	IE345  ReqClose
<BGSOUND>        src, loop	IE2345  NoClose
<MARQUEE>        align, behavior, bgcolor, direction, height, hspace, loop, scrollamount, scrolldelay, vspace, width	IE2345  ReqClose
<EMBED>          src, width, height, border, hspace, vspace, align, name, units	NN234, IE345  ReqClose
<DEL>            NoAttr	IE45  ReqClose
<INS>            NoAttr	IE45  ReqClose
<Q>              cite	IE45  ReqClose
<BASE>           href, target,	*  NoClose
<LINK>           href, methods, rev, rel, title, type, urn	NN4, IE2345  NoClose
<ABBR>           title  NoClosene  ReqClose
<ACRONYM>        title  NoClosene  ReqClose
<SPAN>           NoAttr	NN4, IE45  ReqClose
<VAR>            NoAttr	*  ReqClose
<MULTICOL>       cols, gutter, width	NN234  ReqClose
<NOBR>           NoAttr	*  ReqClose
<WBR>            NoAttr	*  NoClose
<DIR>            NoAttr	*  ReqClose
<DL>             compact	*  ReqClose
<MENU>           compact, type	*  ReqClose
<APPLET>         align, alt, code, codebase, height, hspace, name, vspace, width	NN234, IE345  ReqClose
<NOEMBED>        NoAttr	NN234  ReqClose
<NOSCRIPT>       NoAttr	NN34, IE345  ReqClose
<OBJECT>         align, border, classid, codebase, data, height, hspace, name, type, uesmap, vspace, width	*  ReqClose
<PARAM>          name, value, type	NN234, IE345  ReqClose
<SCRIPT>         type, language, src	NN34, IE345  ReqClose
<SPACER>         type, size, height, width, align	NN34  NoClose
<COL>            align, char, charoff, span, valign, width	IE45  NoClose
<COLGROUP>       align, char, charoff, span, valign, width	IE45  ReqClose
<TBODY>          align, char, charoff, valign	IE45  ReqClose
<TFOOT>          align, char, charoff, valign	IE45  ReqClose
<TH>             align, colspan, rowspan, nowrap, background, bgcolor, brodercolor, bordercolorlight, bordercolordark, height, width, style, valign	*  ReqClose
<THEAD>          align, char, charoff, valign	IE45  ReqClose
<BUTTON>         name, value, type	NN4, IE45  ReqClose
<FIELDSET>       NoAttr	IE45  ReqClose
<ISINDEX>        NoAttr	*  NoClose
<LABEL>          NoAttr	IE45  ReqClose
<LEGEND>         NoAttr	IE45  ReqClose
<OPTGROUP>       NoAttr  NoClosene  ReqClose
<TEXTAREA>       cols, name, rows, wrap	*  ReqClose



  
  #####################################################################################################################
  ###############################################################   D O M A I N - S P E C I F I C   S U P P O R T   ###
  #####################################################################################################################
  
  # Looks up portal fields.
  # Valid crud_methods  :edit  :list (the 'index' method)  :show
  def fields_for_portal(some_portal, crud_method)
    U.chk_error(![:edit, :list].include?(crud_method), "Unknown CRUD method: #{crud_method.inspect}")
    val("#{some_portal}_#{crud_method}_fields".to_sym).map { |s| val_field(s) }
  end
 
# Refs
CSS Content:  [[https://www.amazon.com/Fitbit-Charge-Heart-Fitness-Wristband/product-reviews/B01K9S260E/ref=cm_cr_arp_d_viewopt_srt?ie=UTF8&reviewerType=all_reviews&showViewpoints=1&sortBy=helpful&pageNumber=1][content]]    [[https://css-tricks.com/pseudo-element-roundup/][pseudo elements]]
