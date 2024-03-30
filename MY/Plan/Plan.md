
:: [[Actualize]],   [[BKT Tree]],   [[BKT]],   [[Dropped]],   [[EBT]],   [[Friday]],   [[Fried Later]],   [[Habit]],   [[Lrn]],   [[Meta List]],   [[Mission]],   [[Per]],   [[Ping]],   [[Productivity]],   [[Prof]],   [[Quarterly]],   [[Repeat]],   [[Rest]],   [[Rocks]],   [[Scratch]],   [[Social]],   [[Spirit]],   [[Up All]],   [[Up Task]],   [[active]],   [[archive]],   [[external]],   [[fried]],   [[full]],   [[fun]],   [[gap]],   [[hack]],   [[later]],   [[now]],   [[other]],   [[prime]],   [[q2]],   [[quick]],   [[todo]],   [[wings]]


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

