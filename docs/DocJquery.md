  [__DocJquery__](__DocJquery__.md)




Good Intro:   http://ejohn.org/apps/workshop/intro/#13

$ == jQuery    (the jQuery object)

 #xxx   select XML elements whose 'id' tag matches xxx
 .xxx   select XML elements whose 'class' tag matches xxx
 xxx    select XML elements whose 'head' matches xxx

.parent() .next() .prev() .children() .siblings() ...


$("div").addClass("special");  // "div" finds some elements, class is added to each



$(document).ready( function() {...} );   // run when doc loaded

$("div").hide().css("color","blue").remove();   // allows chaining

## Operates on element sets
###  .css({..})   // applies css styles


## components
# A complete set of themed, cross-browser, user interface components.
# Drag, Drop, Sort, Select, Resize Accordion, Datepicker, Dialog, Slider, Tabs
# New in 1.8: Button, Autocomplete
More info: http://jqueryui.com/

