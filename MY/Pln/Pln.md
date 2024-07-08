
:: [[EBT]],   [[Lrn]],   [[Meta List]],   [[Ping]]
.[[Pln]].
  CV:	[[CVP]], [SprintPlanning](spot://sprintplanning), [OKR Q2](spot://okrq2), [OKRs](spot://okrs), [OKR Slide](spot://okrslide), [[CV Roadmap]],
  SW:	[Pln Factory](spot://plnfactory), [Pln Folder](spot://plnfolder), [Pln Work](spot://plnwork),
  WRK2:	[[Full]], [[Prime]],
  WRK3:	[[RD]],
  -
  OVER:	[[Friday]], [[Habit]], [[Repeat]], [[Self Evergreen|Ever]].
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
  TYPE:	[[Chew]], [[Eat]],
  ADDS:   [Pln Personal](spot://plnpersonal),
  DELS:



















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

