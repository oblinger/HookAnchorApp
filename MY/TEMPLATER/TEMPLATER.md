
:: [[AT Entry]],   [[DAY Entry]],   [[JOURNAL Entry]],   [[LOGGED sub note]],   [[MEETING Entry]],   [[NAMED sub note]],   [[NOTE entry]],   [[PRJ Entry]],   [[REF Entry]],   [[TEXT Entry]],   [[UNTYPED Entry]],   [[VC Template]],   [[WP Entry]],   [[Z Example Template]],   [[Z JOURNAL header]],   [[Z TEST TEMPLATE]],   [[indent IN]],   [[indent OUT]]



from AT template
<%-*
const name = await tp.system.prompt("Entity Name (w/o @)");
await tp.file.move("/AT/@" + name) 
-%>
