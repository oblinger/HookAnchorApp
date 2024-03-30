
:: [[Up All]],   [[Up Task]]
LONG:	[[Rocks]], [[Q2]],  [[Spirit]],  [[Actualize]],  [[Mission]],  
ROOT:	[[BKT Tree]],  [[BKT]],  [[Meta List]], 
LATR:	[[Dropped]], 
OVER:	[[Friday]],  [[Habit]],  [[Repeat]],  [[Productivity]], [[Self Evergreen]],   
FUN:	[[Fun]], [[Social]],  [[Fried]],  [[Gap]],  [[Hack]],  [[Fried Later]],
FAM:	[[Family]], [[EBT]], 
NJ:	[[Lrn]],  
LST1:	[[Quick]],  [[Todo]],   [[Now]],   [[Active]],  [[Scratch]], 
LST2: [[Full]],   [[Prime]],
LST3:	[[Quarterly]],   [[Later]],   [[Wings]],  [[Archive]],
COM:  [[CRM]],   [[Ping]], [[Mentor]], 
MISC: [[EBT]],  [[external]],  [[other]], 

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

