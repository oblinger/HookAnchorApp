.[[T]].
  ,
  :
  [[Eli]]:
  [[MY]]:
  [T Folder](spot://tfolder):
  [[Food]]:
  [[Legal]]:
  [[Life]]:
  [[Music]]:
  :
  :
  :
  [[Web]]:
  :
  :
  :
  [[MED]]:
  ADDS:   [[Politics]]
  ADDS:   [[SF]],
  ADDS:   ,
  ADDS:   [[Tag]],
  ADDS:   ,
  ADDS:   [[Courses]],





































































































































































































- [[Career]], [[Music]], 

:: [[COMS]],   [[MGR Log]],   [[Comp]],   [[EduCorp]],   [[FIN]],   [[Food]],   [[KM]],   [[Legal]],   [[Life]],   [[MED]],   [[SYS]],   [[TAG]],   [[Trash]],   [[Web]],   [[Webshare]]
#top
n:: Major topic roots

- [[JJ]], [[j J]], 




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



