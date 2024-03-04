
:: [[BKT Tree]],   [[Prof]]

:: [[EBT]],   [[Ping]],   [[Productivity]],   [[Spirit]],   [[other]]
#top[[Top|#]] 
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


#### Short Term Tactical Lists
- [[active]]		-  [[wings]], 
- [[Current]]		-  My 'current' weekly plan
- Daily			- 
- [[fried]]			-  Easy/Fun/Taking-Break tasks.   [[Fried Later]], 
- [[quick]]			- 
- [[todo]]			- 
- [[now]]			-  
- [[work]]			- 


#### Longer Term 
#### Specialty List
- [[Actualize]]	-  
- [[BKT]]		-  
- [[Dropped]]		-  
- [[full]]			-  
- [[fun]]			-  
- [[Friday]]		-  Planning for date nights
- [[hack]]			-  Hacking/Coding activities
- [[Habit]]			-  Current and past habit goals
- [[gap]]			-  
- [[Journal Old]]	-  
- [[MY/Plan/L]]			-  
- [[Per]]			-  
- [[Quarterly]]	-  
- [[Repeat]]		-  
- [[SCRatch]]		-  
- [[Social]]		-  
- [[q2]]				- Second 
- [[Mission]]		- 

#### Offload Lists
- [[Archive]]		-  Place to retire planning items.   [[Arc ALL]].
- [[later]]			-  Tasks that have been pushed to 'later' 
- [[Quarterly]]	-  
- [[external]]		- ????? 


#### Comms Pages
- [[COMS]]			-  
- [[COM Now]]	-  
- [[COM Next]]	-  
- [[COM Waiting]]-  
- [[MY/Plan/Ping]]			-  	


#### Unsorted Planning Pages
- [[Journal Old]]	- 
- [[MY/Plan/L]] 		-
- [[MY/Plan/Ping]] 			- 


PREFIX NAMED:   [[active]]   B   [[COMS]]   DAILY   [[fried]]   E   F   G   [[hack]]   I   J   K   L   M   N   O   [[prime]]   [[quick]]  [[Repeat]]   [[MY/Plan/self]]   [[todo]]   U   V   [[external]]   X   Y   Z
CHANGE: [[wings]], [[Current]], 



- XXX
  [COMS](COMS.md)
- 

  [quick](quick.md)
  [Quarterly](Quarterly.md)  [Archive](Archive.md)  [Work](Work/Work.md)  [full](full.md)  [self](MY/Plan/self.md)  [later](later.md)  [Repeat](Repeat.md)  [Ping](MY/Plan/Ping.md)  [q2](q2.md)  [todo](todo.md)  [Habit](Habit.md)  [Social](MY/Plan/Social.md)  [SCR Note](SCR%20Note.md)  [Watching](Watching.md)  [Current](Current.md)  [L](MY/Plan/L.md)  [hack](hack.md)
  [Archive](__Archive__.md)
  [Work](__Work__.md)
  [Todo](__Todo__.md)  [Repeat](__Repeat__.md)  [Calls](AT/__Calls__.md)  [Later](__Later__.md)  [Watching](__Watching__.md)  [Habits](__Habits__.md)  [Current](__Current__.md)  [Links](__Links__.md)  [Scratch](Scratch/__Scratch__.md)  [Full](__Full__.md)  [Quarterly](__Quarterly__.md)  [Self](__Self__.md)  [Q2](__Q2__.md)  [Hack](__Hack__.md)  [Ping](__Ping__.md)  [Social](__Social__.md)  [fried](fried.md)


- 
- 
  [wings](wings.md)
[[wings]]  [Threads](Threads.md)
  [Coursra](Coursra.md)
  [external](external.md)

  [fun](fun.md)
 [[Quarterly]],  [Quarterly](Quarterly.md),   [Planning](Planning.md),   [Archive](Archive.md),   [Work](Work/Work.md),   [full](full.md),   [self](MY/Plan/self.md),   [later](later.md),   [Repeat](Repeat.md),   [Calls](Calls.md),   [Ping](MY/Plan/Ping.md),   [q2](q2.md),   [fried](fried.md),   [todo](todo.md),   [Habit](Habit.md),   [Social](MY/Plan/Social.md),   [SCR Note](SCR%20Note.md),   [COMS](COMS.md),   [prime](prime.md),   [Watching](Watching.md),   [Current](Current.md),   [L](MY/Plan/L.md),   [quick](quick.md),   [hack](hack.md),   [active](active.md)
