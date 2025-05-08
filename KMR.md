.[[kmr]].
  , [[2024-10-29]], [[AT]], [[ATTACH]], [GG](spot://gg), [KMR Gdrive](spot://kmrgdrive), [[Log]]
  , [[MY]], [[Prj]], [[RR]], [[Set]], [[SYS]], [[T]], [[TOC]]
  , [TRASH for Obsidian Folder](spot://trashforobsidianfolder)
  , [TRASH Obsidian](spot://trashobsidian), [[WEEKLY]], [[Work]], 

TOPS: [[KMR]], [[Meta]], [[TOC]], 

.[[KMR]].
  [[Set]],
  [[Log]],
  ,
  [[MY]],
  [[Prj]],
  [[T]], [GG](spot://gg), , [[RR]],
  [TRASH Obsidian](spot://trashobsidian), [TRASH for Obsidian Folder](spot://trashforobsidianfolder)
  OTHER: ,  , ,
  , [KMR Gdrive](spot://kmrgdrive),
  , ,
  , , , ,
  , [[2024-10-29]], , [[SYS]], [[TOC]], , ,
  DELS: [[TOC]], , [[SYS]], , , [[2024-10-29]],
  , [[At]], [[Attach]], [[Weekly]], [[work]],
  , ,
  DELS: [[TOC]], [[Work]], [[SYS]], [[ATTACH]], [[WEEKLY]], [[2024-10-29]],[[JJ]], [[Misc]], [[test987]],[top](spot://top),[[AT]], [[ATTACH]], [[WEEKLY]], [[Work]], [[Work]], [[ATTACH]], [[WEEKLY]],[[AlienBiologyWhitepaper]], 





#### [[KMR]] [[Kmr Folder|--]] Toplevel roots for all pages
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



























































































  DELS: [WW](spot://ww), 












































   	
| ORG       | FOLDER                  | META           | By       | Notes                                  |
| --------- | ----------------------- | -------------- | -------- | -------------------------------------- |
| [[Roots]] | [[Kmr Folder\|KMR]]     | [[Meta Meta]]  | ALL      | Toplevel roots for all pages ([[Dir]]) |
| [[Pln]]   | [Pln](spot://plnfolder) | [[Meta Plan]]  | By TASK  | Short and long term planning           |
| [[Log]]   | [Log](spot://logfolder) | [[Meta Log]]   | By TIME  | Lists with a time-based org            |
| [[Set]]   | [[Set Folder\|Set]]     | [[Meta Set]]   | By TYPE  | Groups of like (typed) entries         |
| [[TOC]]   | [[JJ]]                  | [[Meta Group]] | By GROUP | Taxonomy of Groups                     |
|           |                         | [[Meta Flow]]  | By PROC  | My data processing paths               |
| [[SV]]    | [[SV Folder\|SV]]       | [[Meta SV]]    | WORK     | Work related [[WW]]                    |

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