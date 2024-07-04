.[[Pln]].
  OVER:	[[Friday]], [[Habit]], [[Repeat]], [[Productivity]], [[Self Evergreen|Ever]]
  LST1:	[[quick]], [[todo]], [[now]], [[active]], [[SCRatch]]
  LST2: [[full]], [[prime]],  [[Big]], 
  LST3:	[[Quarterly]], [[later]], [[wings]], [[Dropped]], [[archive]]
  LONG:	[[Rocks]], [[q2]], [[Lrn]], [[Spirit]], [[Actualize]], [[Mission]], [[WRM]],
  COM:  [[CRM]], [[Ping]], [[Mentor]]
  FUN:	[[fun]], [[Social]], [[fried]], [[gap]], [[Hack Todo]], [[Fried Later]]
  FAM:	[[Family]], [[EBT]], [[Friday Dates]], 
  ROOT:	[[BKT]], [[Meta List]], [[Up All]], [[Up Task]]
  MISC: [[EBT]], [[external]],  [[other]], [[Eat]], 
  WORK:	[Pln Factory](spot://plnfactory), [Pln Work](spot://plnwork), 
  ??		[Pln Personal](spot://plnpersonal), [[pPlan]], 


But that is my point, that agreement would have Israel giving up this land in return for what? Sure you can say from a moral perspective they should give it up w/o nothing in return. And the USA should give all its territory back to native Americans too by the same argument. Perhaps so, but its not happening.

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

