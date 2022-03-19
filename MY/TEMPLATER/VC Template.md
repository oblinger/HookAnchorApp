<%-*
const name = await tp.system.prompt("VC Name");
await tp.file.move("/ORG/VC/@" + name) 
-%>
