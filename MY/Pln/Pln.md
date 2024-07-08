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
  [[q2]]:
  [[quick]]:
  [[todo]]:
  [[wings]]:
  ADDS: [[Actualize]], [[Big]], [[Dropped]], [[Eat]], [[Family]], [[Mission]], [[Ping]], [Pln Factory](spot://plnfactory), [Pln Personal](spot://plnpersonal), [Pln Work](spot://plnwork), [[pPlan]], [[Productivity]], [[Quarterly]], [Repeat](spot://repeat), [[Rocks]], [[SCRatch]], [[Social]], [[Spirit]], [[Up All]], [[Up Task]], [[WRM]]

  OVER:	[[Friday]], [[Habit]], [[Repeat]], [[Productivity]], [[Self Evergreen|Ever]]
  LST1:	[[Quick]], [[Todo]], [[Now]], [[Active]], [[SCRatch]]
  LST2: [[Full]], [[Prime]],  [[Big]],
  LST3:	[[Quarterly]], [[Later]], [[Wings]], [[Dropped]], [[Archive]]
  LONG:	[[Rocks]], [[Q2]], [[Lrn]], [[Spirit]], [[Actualize]], [[Mission]], [[WRM]],
  COM:  [[CRM]], [[Ping]], [[Mentor]]
  FUN:	[[Fun]], [[Social]], [[Fried]], [[Gap]], [[Hacks]], [[Fried Later]]
  FAM:	[[Family]], [[EBT]], [[Friday Dates]],
  ROOT:	[[BKT]], [[Meta List]], [[Up All]], [[Up Task]]
  MISC: [[EBT]], [[External]],  [[Other]], [[Eat]],
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

