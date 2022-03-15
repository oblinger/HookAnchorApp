<%-*
const name = await tp.system.prompt("Name")
await tp.file.move("/Person/PP/" + name) 
%>
