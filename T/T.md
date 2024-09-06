.[[T]].
  ,
  :
  :
  :
  :
  :
  :
  :
  [[Music]]:
  :
  :
  :
  [[Web]]:
  :
  :
  :
  :
  ADDS:
  ADDS:   ,
  ADDS:   ,
  ADDS:   ,
  ADDS:   ,
  ADDS:   ,
  ADDS:   , ,
  ADDS:   , , , , , , , , , , [T Folder](spot://tfolder),
  ADDS:   , , , , , , , , ,
  ADDS:   ,
  ADDS:   , , , , , [KM Folder](spot://kmfolder), [Legal Folder](spot://legalfolder), , ,
  ,   [[Career]], [[COMS]], [[Courses]], [[EduCorp]], [[FIN]], [[Food]], [[KM]], [[Legal]], [[Life]], [[MED]], [[MGR]], [[Tag]], [[Trash]], 
  DELS: [[Courses]], [[Eli]], [[Food]], [[Legal]], [[Life]], [[MED]], [[MY]], [[Tag]], [[Career]], [[COMS]], [[EduCorp]], [[FIN]], [[KM]], [[MGR]], [[Trash]],[[Politics]], 









  DELS: [[Career]], [[COMS]], [[EduCorp]], [[FIN]], [[KM]], [KM Folder](spot://kmfolder), [Legal Folder](spot://legalfolder), [[MGR]], [[Trash]],[[SF]], 























  DELS: [[SF]], 














  DELS: [KM Folder](spot://kmfolder), [Legal Folder](spot://legalfolder), 


  DELS: [[Eli]], [[MY]], [T Folder](spot://tfolder), [[Food]], [[Legal]], [[Life]], [[MED]], [[Politics]], [[SF]], [[Tag]], [[Courses]], 





































































































































































































































































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



