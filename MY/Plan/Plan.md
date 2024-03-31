
**[[PLAN]]:**	
 OVER:	[[Friday]], [[Habit]], [[Repeat]], [[Productivity]], [[Self Evergreen|Ever]] 
 LST1:	[[Quick]], [[Todo]], [[Now]], [[Active]], [[Scratch]] 
 LST2: [[Full]], [[Prime]] 
 LST3:	[[Quarterly]], [[Later]], [[Wings]], [[Dropped]], [[Archive]] 
 LONG:	[[Rocks]], [[Q2]], [[Lrn]], [[Spirit]], [[Actualize]], [[Mission]]
 COM:  [[CRM]], [[Ping]], [[Mentor]] 
 FUN:	[[Fun]], [[Social]], [[Fried]], [[Gap]], [[Hack]], [[Fried Later]] 
 FAM:	[[Family]], [[EBT]] 
 ROOT:	[[BKT Tree]], [[BKT]], [[Meta List]], [[Up All]], [[Up Task]] 
 MISC: [[EBT]], [[external]],  [[other]] 


#### [[Plan]] [[Plan Folder|--]] Short and long term planning 
```dataviewjs
let rows = dv.pages("")
  .where(p => {
    if (!p.file) return false;

	let prefix = "MY/Plan"
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

