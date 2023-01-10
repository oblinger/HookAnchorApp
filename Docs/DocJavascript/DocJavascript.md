  [__DocJavascript__](__DocJavascript__.md)



### Class Template

var LEX;
LEX.namespace('module.path.ClassName');

LEX.module.path.ClassName = (function () {
    "use strict";
    var u               = LEX.u,
        Constructor;

    // Constructor
    Constructor = function (o) {
        // SUPER  -->> LexStorage.call(this);
    };

    // Public API
    Constructor.prototype = {
        constructor: LEX.module.path.ClassName,

        // SOME_METHOD --
        someMethod: function (arg1, arg2) {
        }
    };
    return Constructor;
}());


### Module Template

var LEX;
LEX.namespace('m.path.mName');

LEX.m.path.mName = (function () {
    "use strict";
    var u               = LEX.u,
        x               = null;

    // module methods
    return {
        version:     "1.0",

        // someMethod -- 
        someMethod: function () {
        }
    };
}());





# Docs
- Mozz Docs   https://developer.mozilla.org/en-US/docs/JavaScript/Reference
- Reference   https://developer.mozilla.org/en-US/docs/JavaScript/Reference
- Core w. Ex  http://javascript-reference.info/

TUTORIALS  
- Nice approach for defining objects etc.  https://gist.github.com/a2fd1da997f457b76efe
- Best Tutorial    https://developer.mozilla.org/en-US/docs/JavaScript/Guide
  What is the best tutorial    http://stackoverflow.com/questions/4527488/what-is-the-best-online-free-javascript-tutorial

- 1pg  Latexed Ex. http://sage.math.washington.edu/home/agc/lit/javascript/javascriptcheatsheet.pdf
- Coffeescript for Classes http://jspro.com/coffeescript/classes-in-coffeescript/#.UMuslXLy00N

FROM DANIEL POLISH
Here is quick book which describes most common issues in JS.
http://www.amazon.com/JavaScript-Good-Parts-Douglas-Crockford/dp/0596517742

And here is one of his presentations:
http://www.youtube.com/watch?v=hQVTIJBZook

For future you can find interesting this book, too:
http://www.amazon.com/JavaScript-Patterns-Stoyan-Stefanov/dp/0596806752

# Tools

- http://jslint.com/

## Development Environments
### WebStorm


# blam CONTROL
  while (x<5) {
  for (var i = 0; i < arguments.length; i++) { break; }
  for (var x in [1,2,3])

for (var key in p) {
  if (p.hasOwnProperty(key)) {
    alert(key + " -> " + p[key]);
  }
}

function Add () {
    var sum = 0;
    for (var i = 0; i < arguments.length; i++) {
        sum += arguments[i];
    }
    return sum;
}
try {
    unknownFunction ();
}
throw EXPR;
catch(e) {
    document.write ("The error message: <b>" + e.message + "</b>");
    if ('number' in e) {
        document.write ("<br />");
        document.write ("The error code: <b>" + e.number + "</b>");
    }
    if ('lineNumber' in e) {
        document.write ("<br />");
        document.write ("The error occurs at line: <b>" + e.lineNumber + "</b>");
    }
finally ...
}

# blam OPERATORS
Arithmetic operators: + , -, *, /, %, ++, --, + (unary plus), - (unary minus)
Assignment operators: =, +=, -=, *=, /=, %=, <<=, >>=, >>>=, &=, |=, ^=
Bitwise operators: &, |, ^, ~, <<, >>, >>>
Comparison/ Logic: ==, ===, !=, !==, <, <=, >, >=, ?:, &&, ||, !
Other operators: , (comma), ?:, delete, in, instanceof, new, try/catch/finally, typeof, void
# blam DATASTRUCTURES

STRING
http://www.hunlock.com/blogs/The_Complete_Javascript_Strings_Reference
http://www.w3schools.com/jsref/jsref_obj_string.asp

 x="a string"

.anchor(nameAttribute), .big, .blink, .bold, .charAt(Index) .charCodeAt(Index)
.concat(String[, String[, String...]])
.fixed, .fontcolor(color)
.fontsize(size)
.fromCharCode(code1[, code#...])
.indexOf(SearchValue[, startIndex])
.italics, .lastIndexOf(SearchValue[, startIndex])
.link(url)
.localeCompare(string)
.match(regExp)
.replace(regExp/substr, replacementValue[, flags])
.search(regExp)
.slice(startIndex[, endIndex])
.small, .split(delimiter[, limit])
.strike, .sub, .substr(index[, numChars])
.substring(index[, stopIndex])
.sup, .toLocaleLowerCase, .toLocaleUpperCase, .toLowerCase, .toSource, .toString, .toUpperCase, .valueOf, 

trim, htmlEntities, stripTags, toArray, toIntArray, 

# Creating a post form dynamically on button press
link_to "Destroy", project_path(project), :method => :delete
=>
<a href="/projects/1"
onclick="var f = document.createElement(’form’);
f.style.display = ’none’; this.parentNode.appendChild(f);
f.method = ’POST’; f.action = this.href;
var m = document.createElement(’input’);
m.setAttribute(’type’, ’hidden’);
m.setAttribute(’name’, ’_method’);
m.setAttribute(’value’, ’delete’); f.appendChild(m);f.submit();
return false;">Destroy</a>

# Underscore

## Collections

_.map([1, 2, 3], function(num){ return num*num });   ===> [1, 4, 9]
_([1, 2, 3]).map(function(num){ return num*num });   ===> [1, 4, 9]
_([1, 2, 3]).chain().map(function(num){ return num*num }).size().value();  ===> 3

## Other
### Bind
var func = function(greeting){ return greeting + ', ' + this.name };
func = _.bind(func, {name : 'Adrian'}, 'Yo');
func();
=> "Yo, Adrian"

### Templating
var list = "<% _.each(people, function(name) { %> <li><%= name %></li> <% }); %>";
_.template(list, {people : ['moe', 'curly', 'larry']});
=> " <li>moe</li> <li>curly</li> <li>larry</li> "




# jQuery
## BLAM
$("h2").("#id").(".class")

[DOM]
TRAVERSE  .parent(), .next(), .prev(), .children(), .siblings()
MANIP     .append .prepend .remove .clone

[ACTIONS] .hide()  .css()  .remove()


$ == jQuery    (the jQuery object)

       uses CSS selectors to find dom nodes 


<script src='http://code.jquery.com/jquery.js'></script>   // From CDN

$("div").addClass("special");  // "div" finds some elements, class is added to each



$(document).ready( function() {...} );   // run when doc loaded

$("div").hide().css("color","blue").remove();   // allows chaining

### Operates on element sets
####  .css({..})   // applies css styles


### components
# A complete set of themed, cross-browser, user interface components.
# Drag, Drop, Sort, Select, Resize Accordion, Datepicker, Dialog, Slider, Tabs
# New in 1.8: Button, Autocomplete


### Docs
Good Intro:   http://ejohn.org/apps/workshop/intro/#13
More info: http://jqueryui.com/
amimations sliding etc.  -- http://playground.benbarnett.net/jquery-animate-enhanced/





# CheatSheet

Prototype JavaScript Library 1.5.0 -- http://snook.ca/files/prototype_1.5.0_snookca.pdf
Utility Methods
Takes one or more element ID’s or 
elements and mixes in Element methods
$(s|el [,s|el,..])
Returns array of elements using 
CSS Selectors
$$(cssSelectors s[,s,s..])
Returns array with Array and 
Enumerable methods
$A(a)
Returns array with Hash and 
Enumerable methods
$H(o)
Splits string on spaces
$w(s)
Exits after first successful function
Try.these(f[,f,f..])
$R = new ObjectRange
$F = Form.Element.getValue()
each(f)
eachSlice(n, f)
all(f)
any(f)
collect(f)
detect(f)
findAll(f)
grep(pattern, f)
include(o)
inGroupsOf(n, fillWith)
inject(memo, f)
invoke(method s)
max(f)
min(f)
partition(f)
pluck(property s)
reject(f)
sortBy(f)
toArray()
zip(a[,a,a..][,f])
size()
inspect()
map=collect
find=detect
select=finaAll
member=include
entries=toArray
Enumerable
Handles enumeration
keys()
values()
merge(hash)
toQueryString()
inspect()
Hash
includeScrollOffsets
prepare()
realOffset(el)
cumulativeOffset(el)
positionedOffset(el)
offsetParent(element)
within(el, x, y)
winthinIncludingScrollOffsets
      (el, x, y)
overlap(’vertical’|’horizontal’, el)
page(el)
clone(src el, target el)
absolutize(el)
relativize(el)
Position
DOM node positioning
visible()
toggle()
hide()
show()
remove()
update(html s)
replace(html s)
inspect()
recursivelyCollect(property s)
ancestors()
descendants()
immediateDescendants()
previousSiblings()
nextSiblings()
match(selector)
up(expression, index)
down(expression, index)
previous(expression, index)
next(expression, index)
getElementsBySelector()
getElementsByClassName(s)
readAttribute(s)
getHeight()
classNames()
hasClassName(s)
addClassName(s)
removeClassName(s)
toggleClassName(s)
observe()
stopObserving()
cleanWhitespace()
empty()
childOf(ancestor el)
scrollTo()
getStyle(s)
setStyle(s)
getDimensions()
makePositioned()
undoPositioned()
makeClipping()
undoClipping()
Element
Extends DOM nodes via $() and $$() (if 
executed as  Singleton, element is 
first argument)
Extends FORM nodes via $() and $$() (if 
executed as Singleton, element is 
first argument)
Extends INPUT, TEXTAREA, and SELECT 
nodes via $() and $$() (if executed as 
Singleton, element is first argument)
# reset and serializeElements can only be
  called from Singleton
# focus and select can only be
  called from Singleton
initialize(el, content s)
initialize(el)
set(classname s)
add(s)
remove(s)
toString()
Element.ClassNames    
extends Enumerable
Handles element class names
initialize(expression)
findElements(scope el)
toString()
matchElements(els, expression)
findElement(els,expression, index)
findChildElements(el, expressions)
Selector
Selecting elements with CSS selectors
reset(frm)
serializeElements(els)
serialize()
getElements()
getInputs(type s, name s)
disable()
enable()
findFirstElement()
focusFirstElement()
focus()
select()
serialize()
getValue()
clear()
present()
activate()
disable()
enable()
Form
focus(el)
select(el)
serialize()
getValue()
clear()
present()
activate()
disable()
enable()
Form.Element = Field
s=string
o=object
n=number
el=element
els=elements
f=function (callback)
b=boolean
[ ]=optional
| = or
pattern=regex
Legend
initialize(el, freq n, f)
Abstract.TimedObserver
initialize(el, f)
Abstract.EventObserver
getValue()
Form.Element.Observer
Form.Observer
extends Abstract.TimedObserver
getValue()
Form.Element.EventObserver
Form.EventObserver
extends Abstract.EventObserver
Insertion.Before
Insertion.Top
Insertion.Bottom
Insertion.After 
   extend Abstract.Insertion
Handles content insertion
getTransport()
activeRequestCount
Ajax (singleton)
XMLHttpRequest Interface
responders
register(o)
unregister(o)
dispatch(s, req, transport, json)
Ajax.Responders extends 
Enumerable (singleton)
Responds to Ajax events
initialize(url s, options o)
success()
getHeader(s)
Ajax.Request extends .Base
Processes an Ajax request
initialize(el, url s, options o)
Ajax.Updater extends .Base 
and .Request
Updates a page element via Ajax
initialize(el, url s, options o)
start()
stop()
Ajax.PeriodicalUpdater 
extends .Base
Repeatedly updates element via Ajax
element(evt)
isLeftClick(evt)
pointerX(evt)
pointerY(evt)
stop(evt)
findElement(evt, tagname s)
observe(el, evtname, f, usecapture b)
stopObserving(el, evtname, f, usecapture b)
Event
Replaces default browser Event handling
initialize(start n, end n, exclusive b)
include(n)
ObjectRange ext. Enumerable
A numeric range
String template using #{var} markers
from(a)
clear()
first()
last()
compact()
flatten()
without(a)
indexOf(o)
reverse(inline b)
reduce()
uniq()
clone()
size()
inspect()
toArray = clone
Array extends Enumerable
Extends default Array object
interpret(value)
gsub(pattern, f)
sub(pattern, f, count n)
scan(pattern, f)
truncate(length, s)
strip()
stripTags()
stripScripts()
extractScripts()
evalScripts()
escapeHTML()
unescapeHTML()
toQueryParams(separator)
toArray()
succ()
camelize()
capitalize()
underscore()
dasherize()
inspect(dblquotes b)
parseQuery = toQueryParams
String
initialize(template s, pattern)
evaluate(o)
Template
Prototype
Version
BrowserFeatures.XPath
ScriptFragment
emptyFunction()
K()
Class
Enables object to use intialize method 
as constructor
create()
Object
extend(dest o, src o)
inspect(o)
keys(o)
values(o)
clone(o)
Function
bind()
bindAsEventListener(o)
Number
toColorPart()
succ()
times(f)
initialize(f, freq n)
stop()
PeriodicalExecuter
like setInterval but cooler
