<%-*
const name = await tp.system.prompt("Entity Name (w/o @)");
await tp.file.move("/AT/@" + name) 
-%>
