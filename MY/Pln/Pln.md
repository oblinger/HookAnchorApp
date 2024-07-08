
:: [[BKT]],   [[EBT]],   [[Hacks]],   [[Later]],   [[Lrn]],   [[Meta List]]
.[[PLN]].
  ADDS:   
  DELS: [[COMS]], 
.[[Pln]].
  [[active]]:
  [[archive]]:
  [[external]]:
  [[full]]:
  [[now]]:
  [[other]]:
  [[prime]]:
  :
  :
  [[todo]]:
  [[wings]]:
  ADDS: [[Actualize]], [[Big]], [[Dropped]], [[Eat]], [[Family]], [[Mission]], [[Ping]], [Pln Factory](spot://plnfactory), [Pln Personal](spot://plnpersonal), [Pln Work](spot://plnwork), [[pPlan]], [[Productivity]], [[Quarterly]], [Repeat](spot://repeat), [[Rocks]], [[SCRatch]], [[Social]], [[Spirit]], [[Up All]], [[Up Task]], [[WRM]]
  ADDS:   [[chew]], [[COMS]], [[Daily]], [[Daily Schedules]], [[Friday]], [[Friday Dates]], [[fried]], [[Fried Later]], [[fun]], [[gap]], [[Habit]], [[home]]
  OVER:	[[Friday]], [[Habit]], [[Repeat]], [[Productivity]],
  LST1:	, , , , [[SCRatch]]
  LST2: , ,  [[Big]],
  LST3:	[[Quarterly]], , , [[Dropped]],
  LONG:	[[Rocks]], , , [[Spirit]], [[Actualize]], [[Mission]], [[WRM]],
  COM:  , [[Ping]],
  FUN:	, [[Social]], , , , [[Fried Later]]
  FAM:	[[Family]], , [[Friday Dates]],
  ROOT:	, , [[Up All]], [[Up Task]]
  MISC: , ,  , [[Eat]],
  WORK:	[Pln Factory](spot://plnfactory), [Pln Work](spot://plnwork),
  ??		[Pln Personal](spot://plnpersonal), [[pPlan]],
  ADDS:   [[RD]]
  ADDS:   [[Q2]], [[Quick]]
  DELS: [[q2]], [[quick]], 

In the case of Israel the natural "give back" would be assurances they they would be safe from attack from within this new nation. This would I think it is very reasonable for Israeli's to doubt it would be safe


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

