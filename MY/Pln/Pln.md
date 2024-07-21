
.[[Pln]].
  [[CVP]]:	[[Full]], [[Prime]], ,
  -
  OVER:	[[Friday]], [[Habit]], [[Repeat]], [[Self Evergreen]].
  LST1: [[Quick]], [[Todo]], [[Now]], [[Active]], [[SCRatch]],
  LST2:	[[Big]],
  LST3:	[[Quarterly]], [[Later]], [[Wings]], [[Dropped]], [[Archive]],
  LONG:	[[Rocks]], [[Q2]], [[Spirit]], [[Actualize]], [[Mission]], [[WRM]],
  STRAT:[[Productivity]], [[Daily Schedules]],
  TOPIC:	[[COMS]], [[Diary]],
  FUN:	[[Fun]], [[Social]], [[Fried]], [[Gap]], [[Hacks]], [[Fried Later]],
  FAM:	[[Family]], [[Friday Dates]], [[Home]],
  ROOT:	[[BKT]], [[Up All]], [[Up Task]],
  EXT:	[[External]], [[Other]],
  TYPE:	[[Chew]], ,
  ADDS: [Pln Personal](spot://plnpersonal), [[Self]], , [Pln Folder](spot://plnfolder),
  ADDS:   ,
  ADDS:   [[Arc ALL]], [[Archeology]], [[bfast]], [[lunch]], , [[Tracking of Time]], [[wake]],
  ADDS:   [[First]], [[read]],
  ADDS:   [[EBT]], [[Ping]],


:: [[Work/SV/CV/Data/Data]],    [[SVCV Re-ID]]

















  DELS: [Day](spot://day), 































  DELS: [[Lrn]], 

after
:: [[EBT]], [[Ping]]





























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

