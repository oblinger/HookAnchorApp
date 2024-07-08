#tag 
.[[Top]].
  [[My Content]], [[Pln]], [[RR]], [[T]], [[Work]], [[Work Jump]]
  ADDS:   [[Prj]]









#### [[Meta]]		TOP LEVEL INFO ORGANIZATIONS
| ORG       | FOLDER                   | META           | By       | Notes                          |
| --------- | ------------------------ | -------------- | -------- | ------------------------------ |
| [[Roots]] | [[Kmr Folder\|KMR]]      | [[Meta Meta]]  | ALL      | Toplevel roots for all pages   |
| [[Pln]]   | [Pln](spot://pln~folder) | [[Meta Plan]]  | By TASK  | Short and long term planning   |
| [[Log]]   | [Log](spot://logfolder)  | [[Meta Log]]   | By TIME  | Lists with a time-based org    |
| [[Set]]   | [[Set Folder\|Set]]      | [[Meta Set]]   | By TYPE  | Groups of like (typed) entries |
| [[T]]     | [[T Folder\|T]]          | [[Meta Topic]] | By TOPIC | Toplevel Taxonomic Topics      |
|           |                          | [[Meta Flow]]  | By PROC  | My data processing paths       |
| [[SV]]    | [[SV Folder\|SV]]        | [[Meta SV]]    | WORK     | Work related [[WW]]            |

#### [[Roots]] [[Kmr Folder|--]] Top level roots for all pages
```dataviewjs
let rows = dv.pages("")
  .where(p => {
    if (!p.file) return false;

    let filePathParts = String(p.file.path).split("/");
    let fileNameWithoutExt = filePathParts[filePathParts.length - 1].replace('.md', '');
    let parentFolderName = filePathParts[filePathParts.length - 2];

    return String(p.file.path).startsWith("") 
      && filePathParts.length == 2
      && fileNameWithoutExt === parentFolderName;
  })
  .map(p => [p.file.link, p.n]);

dv.table(["File", "Description"], rows);
```

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


#### [[Logs]] [[Logs Folder|--]] DATED, TYPED NOTES (DATED entries)
```dataview
TABLE n AS Description
FROM #log
SORT file.name
```



#### [[Sets]] [[Set Folder|--]] GROUPS OF LIKE ENTRIES -- (UN-dated, Typed entries)
```dataview
TABLE n AS Description
FROM #set
SORT file.name
```

#### [[T]]OPICS [[T Folder|--]] MAJOR TAXONOMIC TOPIC ROOTS. 
```dataviewjs
let rows = dv.pages("")
  .where(p => {
    if (!p.file) return false;

    let filePathParts = String(p.file.path).split("/");
    let fileNameWithoutExt = filePathParts[filePathParts.length - 1].replace('.md', '');
    let parentFolderName = filePathParts[filePathParts.length - 2];

    return String(p.file.path).startsWith("T/") 
      && filePathParts.length == 3
      && fileNameWithoutExt === parentFolderName;
  })
  .map(p => [p.file.link, p.n]);

dv.table(["File", "Description"], rows);
```

#### [[SV]] [[SV Folder|--]] SPORTS VISIO STUFF