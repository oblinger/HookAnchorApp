.[[Pln]].
  WORK1:	, , , [[SCRatch]],
  OVER:		[[Friday]], [[Habit]], [[Repeat]],
  PER1:		,
  PER2:		[[Quarterly]], , [[Wings]], [[Dropped]],
  LONG:		[[Rocks]], , [[WRM]], ,
  SELF:		, [[Spirit]], [[Actualize]], [[Mission]], [[Productivity]],
  FUN:		, [[Social]], , , , [[Fried Later]],
  FAM:		[[Friday Dates]],
  ROOT:		[[BKT]], [[Up All]], [[Up Task]],
  EXT:		[[External]], ,
  UNUSED:	[[Prime]], [[Full]], [[Ping]],
  , , [[Archive List]], , , , , , ,
  , , ,
  DELS: ,, ,, , ,, , , , , , [[Archive List]], , , , ,
  , ,
  DELS: ,, ,, , ,, , , , , , [[Archive List]], , , , ,, , , , , , , , ,
  , [[active]], , [[fun]], [[gap]], [[later]], [[now]], [[other]], [[q2]], [[quick]]
  , [[todo]],
  , [[Fried]],
  DELS: ,, ,, , ,, , , , , , [[Archive List]], , [[Fried]], , ,, , , , , , , , ,, [[Fried]], , , , , , , , , , , , , , , , [[Fried]], , ,, , , , , , , , , , , , , , ,,
  DELS: [[Lrn]],[[MY/Pln/HACK/HACK]], [[HACK.]],[[Hack]], [[Hacks]], [[Lrn]],[[Quick]], [[Todo]], [[Now]], [[Active]], [[Later]], [[Q2]], [[Archive List]], [[Fun]], [[Fried]], [[Gap]], [[Other]],[[EBT]], [[Hacks]], [[Lrn]], [[Lrn]], [[MY/Pln/HACK/HACK]], [[HACK.]], [[Hack]], [[Hacks]], [[Lrn]],[[Active]], [[Fried]], [[Fun]], [[Gap]], [[Later]], [[Now]], [[Other]], [[Q2]], [[Quick]], [[Todo]], [[Quick]], [[Todo]], [[Now]], [[Active]], [[Later]], [[Q2]], [[Fun]], [[Fried]], [[Gap]], [[Other]],[[Lrn]], [[MY/Pln/HACK/HACK]], [[HACK.]], [[Hack]], [[Hacks]], [[Lrn]], [[EBT]], [[Hacks]], [[Lrn]], [[Lrn]], [[MY/Pln/HACK/HACK]], [[HACK.]], [[Hack]], [[Hacks]], [[Lrn]],[[fried]],[[Self]],[[Lrn]], [[MY/Pln/HACK/HACK]], [[HACK.]], [[Hack]], [[Hacks]], [[Lrn]], [[Quick]], [[Todo]], [[Now]], [[Active]], [[Later]], [[Q2]], [[Fun]], [[Gap]], [[Other]], [[EBT]], [[Hacks]], [[Lrn]], [[Lrn]], [[MY/Pln/HACK/HACK]], [[HACK.]], [[Hack]], [[Hacks]], [[Lrn]], [[Active]], [[Fun]], [[Gap]], [[Later]], [[Now]], [[Other]], [[Q2]], [[Quick]], [[Todo]], [[Quick]], [[Todo]], [[Now]], [[Active]], [[Later]], [[Q2]], [[Fun]], [[Gap]], [[Other]], [[Lrn]], [[MY/Pln/HACK/HACK]], [[HACK.]], [[Hack]], [[Hacks]], [[Lrn]], [[EBT]], [[Hacks]], [[Lrn]], [[Lrn]], [[MY/Pln/HACK/HACK]], [[HACK.]], [[Hack]], [[Hacks]], [[Lrn]], [[fried]], 




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

