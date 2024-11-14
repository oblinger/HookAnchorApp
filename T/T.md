.[[T]].
  ,
  :
  :
  :
  :
  :
  :
  :
  :
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
  ADDS:   , , , , , , , , , , ,
  ADDS:   , , , , , , , , ,
  ADDS:   ,
  ADDS:   , , , , , , , , ,
  ,   [[Career]], [[COMS]], [[Courses]], [[EduCorp]], [[FIN]], [[Food]], , [[Legal]], [[Life]], [[MED]], [[MGR]], [[Tag]], [[Trash]],
  , , ,
  , ,
  DELS: [[Courses]], , [[Food]], [[Legal]], [[Life]], [[MED]], , [[Tag]], [[Career]], [[COMS]], [[EduCorp]], [[FIN]], , [[MGR]], [[Trash]],,,, ,,
  , ,
  DELS: [[Courses]], , [[Food]], [[Legal]], [[Life]], [[MED]], , [[Tag]], [[Career]], [[COMS]], [[EduCorp]], [[FIN]], , [[MGR]], [[Trash]],,,, ,,,
  , , , ,
  DELS: [[Courses]], , [[Food]], [[Legal]], [[Life]], [[MED]], , [[Tag]], [[Career]], [[COMS]], [[EduCorp]], [[FIN]], , [[MGR]], [[Trash]],,,, ,,,, , , , , , , , ,
  , [[Doc]], [[Misc]], [[Mus]], 
  DELS: [[Courses]], , [[Food]], [[Legal]], [[Life]], [[MED]], , [[Tag]], [[Career]], [[COMS]], [[EduCorp]], [[FIN]], , [[MGR]], [[Trash]],,,, ,,[[Mus]],, , , , , , , , ,[[Doc]], [[Misc]], [[Mus]], [[Mus]],
  DELS: [[Courses]], [[Eli]], [[Food]], [[Legal]], [[Life]], [[MED]], [[MY]], [[Tag]], [[Career]], [[COMS]], [[EduCorp]], [[FIN]], [[KM]], [[MGR]], [[Trash]],[[Politics]],[T Folder](spot://tfolder),[KM Folder](spot://kmfolder), [Legal Folder](spot://legalfolder),[[KM]],[[Mus]],[[Music]], [[Eli]], [[MY]], [[KM]], [[Politics]], [T Folder](spot://tfolder), [KM Folder](spot://kmfolder), [Legal Folder](spot://legalfolder), [[KM]],[[Doc]], [[Misc]], [[Mus]], [[Mus]],[[Comp]], [[KM]], [[Music]], [[Eli]], [[MY]], [[KM]], [[Politics]], [T Folder](spot://tfolder), [KM Folder](spot://kmfolder), [Legal Folder](spot://legalfolder), [[KM]], [[Music]], [[Eli]], [[MY]], [[KM]], [[Politics]], [T Folder](spot://tfolder), [KM Folder](spot://kmfolder), [Legal Folder](spot://legalfolder), [[KM]], 









  DELS: [[Career]], [[COMS]], [[EduCorp]], [[FIN]], [[KM]], [KM Folder](spot://kmfolder), [Legal Folder](spot://legalfolder), [[MGR]], [[Trash]],[[SF]], 























  DELS: [[SF]], 














  DELS: [KM Folder](spot://kmfolder), [Legal Folder](spot://legalfolder), 


  DELS: [[Eli]], [[MY]], [T Folder](spot://tfolder), [[Food]], [[Legal]], [[Life]], [[MED]], [[Politics]], [[SF]], [[Tag]], [[Courses]], 





































































































































































































































































- [[Career]], [[Mus]], 

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



