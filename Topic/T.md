.[[T]].
  , [[Career]], [[COM]], [[Courses]], [[Doc]], [[EduCorp]], [[FIN]], [[Food]], [[Legal]], [[Life]], [[Word]], 
  , [[MED]], [[MGR]], [[Misc]], [[Mus]], [[Tag]], [[Trash]], ,
  , [[MOV]],
  , ,
  , [[Date Rollup]], [[T Roll]],
  , ,
  DELS: ,[[T Roll]],
  , ,
  DELS: ,[[T Roll]],,
  , [T Obsidian](spot://tobsidian),
  DELS: ,[[T Roll]],,,[T Obsidian](spot://tobsidian),
  , ,
  DELS: ,[[T Roll]],,,[T Obsidian](spot://tobsidian),, ,
  , [T Note](spot://tnote),
  , [T Roll Obsidian](spot://trollobsidian), 
  DELS: [[2025-06 web]],[[T Roll]],[T Note](spot://tnote),[[2025-06 web]],[T Obsidian](spot://tobsidian),[T Note](spot://tnote), [T Note](spot://tnote),[[2025-06 web]], [[2025-06 web]],[T Note](spot://tnote), [T Note](spot://tnote), [T Note](spot://tnote), [T Note](spot://tnote),

[[MOV]] 

- [[Career]], [[Mus]], 

:: [[COM]],   [[MGR Log]],   [[Comp]],   [[EduCorp]],   [[FIN]],   [[Food]],   [[KM]],   [[Legal]],   [[Life]],   [[MED]],   [[SYS-orig]],   [[TAG]],   [[Trash]],   [[2025-06 web]],   [[Webshare]]
#top
n:: Major topic roots

- [[Proj Directories]], [[j J]], 




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



