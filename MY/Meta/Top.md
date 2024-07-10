.[[Top]].
  ADDS:   [[Work]]
  ADDS:   [[Pln]], [[T]]
#tag
  ADDS:   [[Work Jump]]
#### 		TOP LEVEL INFO ORGANIZATIONS
| ORG       | FOLDER                  | META           | By       | Notes                                |
| --------- | ----------------------- | -------------- | -------- | ------------------------------------ |
|  |      |   | ALL      | Toplevel roots for all pages         |
| [[Pln]]   |    |   | By TASK  | Short and long term planning         |
|    |  |    | By TIME  | Archival lists with a time-based org |
|    |      |    | By TYPE  | Groups of like (typed) entries       |
| [[T]]     |          |  | By TOPIC | Toplevel Taxonomic Topics            |
|           |                         |   | By PROC  | My data processing paths             |
|     |        |     | WORK     | Work related                   |
  ADDS:   [[My Content]]
  DELS: , , , , , , , , , , , , , , , , , , ,
```dataviewjs
let rows = dv.pages("")
  .where(p => {
    if (!p.file) return false;
  ADDS:   [[RR]]
  DELS: , , , , , , , , , , , , , , , , , , ,
    let fileNameWithoutExt = filePathParts[filePathParts.length - 1].replace('.md', '');
    let parentFolderName = filePathParts[filePathParts.length - 2];
  DELS: , , , , , , , , , , , , , , , , , , ,
    return String(p.file.path).startsWith("")
      && filePathParts.length == 2
      && fileNameWithoutExt === parentFolderName;
  })
  .map(p => [p.file.link, p.n]);
  DELS: , , , , , , , , , , , , , , , , , , ,
dv.table(["File", "Description"], rows);
```
  ADDS:   [[Prj]],
  ADDS:   [[Zap]],
  ADDS:   [[Set]],















































  DELS: [[Meta]], [[Roots]], [[Kmr Folder\|KMR]], [[Meta Meta]], [[Plan Folder\|Plan]], [[Meta Plan]], [[Log]], [Log](spot://logfolder), [[Meta Log]], [[Set]], [[Set Folder\|Set]], [[Meta Set]], [[T Folder\|T]], [[Meta Topic]], [[Meta Flow]], [[SV]], [[SV Folder\|SV]], [[Meta SV]], [[WW]], 

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


#### [[Log]] [[Logs Folder|--]] DATED, TYPED NOTES (DATED entries)
```dataview
TABLE n AS Description
FROM #log
SORT file.name
```



#### [[Set/Set]] [[Set Folder|--]] GROUPS OF LIKE ENTRIES -- (UN-dated, Typed entries)
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