.[[Pln]].
  OVER:	[[Friday]], [[Habit]], [[Repeat]], [[Productivity]], [[Self Evergreen|Ever]]. 
  LST1:	
  z
  ADDS:   [[Active]], [[Actualize]], [[Archive]], [[Big]], [[Chew]], [[COMS]], [[Daily]], [[Daily Schedules]], [[Dropped]], [[Eat]], [[External]], [[Family]], [[Friday Dates]], [[Fried]]
  [[Fried Later]], [[Full]], [[Fun]], [[Gap]], [[Hacks]], [[Home]], [[Mission]], [[Now]], [[Other]], [[Ping]], [Pln Factory](spot://plnfactory), [Pln Folder](spot://plnfolder)
  [Pln Personal](spot://plnpersonal), [Pln Work](spot://plnwork), [[pPlan]], [[Prime]], [[Q2]], [[Quarterly]], [[Quick]], [[RD]], [[Rocks]], [[SCRatch]], [[Social]]
  [[Spirit]], [[Todo]], [[Up All]], [[Up Task]], [[Wings]], [[WRM]], 


:: [[Friday Dates]]

**[[Pln]]:**	
 OVER:	[[Friday]], [[Habit]], [[Repeat]], [[Productivity]], [[Self Evergreen|Ever]] 
 LST1:	[[quick]], [[todo]], [[now]], [[active]], [[SCRatch]] 
 LST2: [[full]], [[prime]] 
 LST3:	[[Quarterly]], [[later]], [[wings]], [[Dropped]], [[archive]] 
 LONG:	[[Rocks]], [[q2]], [[Lrn]], [[Spirit]], [[Actualize]], [[Mission]], [[WRM]], 
 COM:  [[CRM]], [[Ping]], [[Mentor]] 
 FUN:	[[fun]], [[Social]], [[fried]], [[gap]], [[hack]], [[Fried Later]] 
 FAM:	[[Family]], [[EBT]] 
 ROOT:	[[BKT]], [[Meta List]], [[Up All]], [[Up Task]] 
 MISC: [[EBT]], [[external]],  [[other]] 


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

