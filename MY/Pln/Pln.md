
.[[Pln]].
  :	 	[[Now]], [[Full]], [[Prime]],
  -
  OVER:		[[Friday]], [[Habit]], [[Repeat]], [[Self Evergreen]].
  LST1:		[[Quick]], [[Todo]], [[Active]], [[SCRatch]],
  LST2:		[[Big]], [[Quarterly]], [[Later]], [[Wings]], [[Dropped]], [[Archive]],
  LONG:		[[Rocks]], [[Q2]], [[WRM]],
  STRAT:	[[Productivity]], [[Daily Schedules]],
  TOPIC:		,
  SELF:		[[Self]], , [[Spirit]], [[Actualize]], [[Mission]],
  FUN:		[[Fun]], [[Social]], [[Fried]], [[Gap]], [[Hacks]], [[Fried Later]],
  FAM:		[[Family]], [[Friday Dates]], [[Home]],
  ROOT:		[[BKT]], [[Up All]], [[Up Task]],
  EXT:		[[External]], [[Other]],
  ACTIONS:	[[Chew]],  , , , [[First]], ,
  ADDS:   	[[Arc ALL]], [[Archeology]],, [[Tracking of Time]], [[EBT]], [[Ping]],
  ADDS:   , , , ,
  ADDS:   [[Bfast]], [[Lunch]], [[Read]], [[Wake]],
  ADDS:   ,
  ADDS:   ,
  ADDS:   , , , ,
  ADDS:   [[Lrn]],



  DELS: [[COMS]], [[CVP]], [[Diary]], [[Hack]], 

















  DELS: [[CVP]], [[COMS]], [[Diary]], [[Hack]], 






















































  DELS: [[CN]], 



























































































  DELS: [[Bfast]], [[Lunch]], [[Wake]], [[Read]],[[bfast]], [[lunch]], [[read]], [[wake]], 

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

