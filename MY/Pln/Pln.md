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
  UNUSED:	[[Prime]], [[Full]], [[Ping]]

  [[MIT]] 

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



---

# Pln

This is an auto-generated anchor file for patch 'Pln'.

Add your content here.


---

# Pln

This is an auto-generated anchor file for patch 'Pln'.

Add your content here.


---

# Pln

This is an auto-generated anchor file for patch 'Pln'.

Add your content here.
