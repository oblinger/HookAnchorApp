.[[Pln]].
  WORK1:	[[Quick]], [[Todo]], [[Now]], [[SCRatch]],
  OVER:		[[Friday]], [[Habit]], [[Repeat]],
  PER1:		[[Active]],
  PER2:		[[Quarterly]], [[Later]], [[Wings]], [[Dropped]],
  LONG:		[[Rocks]], [[Q2]], [[WRM]], [[Archive List]],
  SELF:		, [[Spirit]], [[Actualize]], [[Mission]], [[Productivity]],
  FUN:		[[Fun]], [[Social]], [[Fried]], [[Gap]], , [[Fried Later]],
  FAM:		[[Friday Dates]],
  ROOT:		[[BKT]], [[Up All]], [[Up Task]],
  EXT:		[[External]], [[Other]],
  UNUSED:	[[Prime]], [[Full]], [[Ping]],
  , ,
  , , ,
  DELS: ,
  DELS: [[Lrn]],[[MY/Pln/HACK/HACK]], [[HACK.]],[[Hack]], [[Hacks]], [[Lrn]], 




#### [[Pln]] [[Plan Folder|--]] Short and long term planning 
```dataviewjs
let rows = dv.pages("")
  .where(p => {
    if (!p.file) return false;

	let prefix = "MY/Pln"
	let prefix_len = prefix.split("/").length
    let filePathParts = String(p.file.path).split("/");
    let fileNameWithoutExt = filePathParts[filePathParts.length - 1].replace('.md', '');
    let parentFolderName = filePathParts[filePathParts.length - 2];

    return String(p.file.path).startsWith(prefix) 
      && (filePathParts.length == prefix_len + 1 || 
	      filePathParts.length == prefix_len + 2 && fileNameWithoutExt === parentFolderName);
  })
  .sort((a, b) => a.file && b.file && a.file.name.localeCompare(b.file.name))
  .map(p => [p.file.link, p.n]);

dv.table(["File", "Description"], rows);
```

