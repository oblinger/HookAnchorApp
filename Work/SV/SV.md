
:: [[CV]]
#top 

.[[SV]].
  :
  [[Bug.]]:
  [Bug Actual List Page](spot://bugactuallistpage):
  [Bug Board Page](spot://bugboardpage):
  [[Bugs]]:
  [Fireball](spot://fireball):
  [HALOS](spot://halos):
  [[Investor Intros]]:
  [Jira](spot://jira):
  [Jira Me Page](spot://jiramepage):
  [[July Hiring]]:
  [SV Clickup Page](spot://svclickuppage):
  [SV Confluence Page](spot://svconfluencepage):
  [SV C Root](spot://svcroot):
  [[SV FIN]]:
  [SV Google Drive Page](spot://svgoogledrivepage):
  [SV G Root](spot://svgroot):
  [[SV Individuals]]:
  [[SV Letter]]:
  [SV Log Page](spot://svlogpage):
  [[SV lrn]]:
  [[SV Roadmap]]:
  [[2023 Deliverables]]:
  [Canva](spot://canva):
  [[Circle]]:
  :
  [[SV90]]:
  [[SV90.]]:
  [[INV]]:
  [[Meta SV]]:
  [[Mgt]]:
  [[QQ]]:
  [[Reset]]:
  [[ROADMAP]]:
  [[SV Apps]]:
  [[SV Archive]]:
  [[SV Basketball Info]]:
  [[SV Code]]:
  [[SV Competitor]]:
  [[SV Customer]]:
  [[SV Dashboards]]:
  :
  [SV Folder](spot://svfolder):
  [[SV Legal]]:
  [[SV Market]]:
  [[SV Ops]]:
  [[SV Partner]]:
  [[SV People]]:
  [[SV Planning]]:
  [[SV Product]]:
  [[SV Software]]:
  [[SV Strat]]:
  [[SV SW Team]]:
  [[SV System]]:
  [[SVT]]:
  [SV Tasks](spot://svtasks):
  [[SV Testing]]:
  [[SV Wings]]:
  [[WW]]:
  DELS: , ,
- [[SV Tasks]] -
- TOPS:		[[WW]],   [[QQ]],  [[Mgt]],
- [[SV Strat]]:	[[Reset]],  [[ROADMAP]],
- [[SV Ops]]:	 	[[SV Dashboards]],  [[SV Testing]], [[SV Planning]],
- [[SV People]]:	[[SV SW Team]],   ,
- [[SV Market]]:	[[SV Customer]],  [[SV Competitor]], [[SV Partner]],
- [[SV Product]]: [[SV Code]],  [[SV Apps]], [[SV System]], ,
- [[SV Software]]:
- [[INV]]:
- INFO:			[[SV Code]], [[SVT]], [[SV Legal]], [[SV Folder]], [[Meta SV]],
- ARCHIVE:		[[SV Archive]],   [[SV Wings]],
- [[SV Basketball Info]]
  ADDS:   [[Idea]]











:: [[cvt]], 

:: [[Circle]],   [[SV Software]]

< [[???]]   [[@Sports Visio]],  [[@Jason Syversen]], [[@SV]]
[[Canva]]

LOG
-  [[SV90]], [[SV90.]],  [[2023 Deliverables]],  


#### [[SV]] [[SV Folder|--]] SPORTS VISIO STUFF
```dataviewjs
let rows = dv.pages("")
  .where(p => {
    if (!p.file) return false;

	let prefix = "SV"
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







# TOPICS
## = People
# LOG
### 2023-03-19  hold  sports stats data

TIME DEPENDENCY & DATA DEPENDENCY - Once we introduce the idea of editing stats at multiple levels the interaction between these edits becomes truly mind bending to get all corner cases for the exponential number of temporal orders that these edits can be applied and later edited and removed.  This will be a source of never-ending bugs if we further cache computed data allowing the possibility that computed results will differ depending upon which part of the caching occurred before/after the subsequently applied edit.

Further the semantics for how these potentially conflicting edits override each other can easily cause the behavior of our App to become very hard to understand.  The user may have difficulty understand what other things will and wont change and consequences of their edits propagate in unexpected ways.

KINDS OF EDITS FOR CONSIDERATION
- AI ANNOTATION - Annotation computed by our AI system.
- APPROVAL - An approval / rejection of AI-annotation by a user.
- MANUAL ANNOTATION - A user-supplied game play annotation.
- IMPORTED STAT APPROVAL - Coaches might opt to accept annotations / edits players make on their version of the game into the master game record.
  (Motivated players may not want to wait for the coaches to fix their plays if they want an accurate rendering of their game they might do it themselves.)
- INTERMEDIATE STAT EDIT - Game stats sometimes are derived from other game stats.  (For example the number of points in a game is the sum of all the points made by all players in the game.)  Editing an intermediate stat could have indirect effect on derived game stats. (see intermediate stat use case for an example)
- TOP-LEVEL STAT EDIT - Top level stats provide summary for the whole game 
- CHANGES - Subsequent edits to any of these annotations / Updates of manually provided annotations / approvals etc.



PROGRESSIVE IMPLEMENTATION - This document attempts to capture the full generality of sports stats computation with multi-level intermediate user-based stats editing.   It introduces a lot of complexity that we don't need to deal with right away.  We expect we will only implement part of this full approach initially, but we consider all of this complexity up front in to avoid "painting in a corner" needing a rewrite, and also to head off a host of timing related bugs that will bite us if we don't think very carefully about our system's semantics.





USE CASE FROM HELL
This use case family jams all kinds of edits into a single game as a way of exercising the space of corner cases.  Much of the complexity stems from the fact that users might supply these many kinds of stats updates in different orders, and they might subsequently edit their edits also in differing orders.  This family of use cases attempts to layout a unified semantics for handling this mess.

In this annotation example from hell the:
- 321 shot attempt AI Annotations.
- 43 user shot annotations. The coach and two players add 43 shot annotations.
- 25 coach approvals. The coach approves 25 shot annotations that the two players added to be part of his master stats for that game.
- 1 intermediate stat edit was applied by a player 31 to increase his final number of baskets made by one to match that stats reported during the playoff game by the league, the coach approved this update.
- 1 final state update was applied to adjust the final score for the opposing team to lower the their final score to match the recorded finals score for the game w/o adjusting any underlying shot annotations.


INTERMEDIATE STATS UPDATE
- When manually increasing the shots made by player 31 from 8 to 9 should the system automatically update to total score for the game so that it is the sum of the shots from all players.
  (which could then be edited back down if desired)
- Alternatively the system might not make any other updates except the score for player 31, and leave the final score unchanged.  (so the number don't add up by default after an edit)

==> what do we want the system to do?


PROBLEMATIC ORDERING FOR CONSIDERATION
- SIMPLE EDIT - The coach adds an annotation for a missed shot attempts.    
  ==> This DOES update the intermediate game stats for that player, and final stats too.



LAYERED EDITS - DATA PROPAGATES
- The coach adds an annotation for a two point shot for player 31
  ==> The final score for the game increases 7-->79 points as a result

LAYEREDEDITS - DATA DOES NOT PROPAGATE
- The coach edits the number of shots for player 35   8-->9
  ==> This further increases game score 79-->81
- The coach adds an annotation for a two point shot for player 35
  ==> The player's score and final score are NOT adjusted since an explicit edit for player 35 is in effect.
- The 
- The player annotated this as



, coaches etc. will want both intermediate and final stats to be correct, but editing ALL stats will be tedious, thus in some cases they will prefer to directly edit intermediate game stats


### 2023-01-23  Prior Inventions

SOURCES:  [[Idea]], [[Ideas]]

VIRTUAL TOUR GUIDE - An Augmented-Reality/smartphone-based virtual tour guide service.  Proposal for a crowd-sourced market of virtual curated tours of popular attractions and city destinations using Computer Vision and Augmented Reality.  Solution supports the logistics of sightseeing (getting to the right location with the right tickets at the right time) as well as both curated and spontaneous tour-guide-provided information. 

ENGINES OF CONTROL - Proposed models for AGI (Artificial General Intelligence), along with analyses of properties that are likely required of all AGI systems.  These proposals serve as the basis for AI-safety work -- assessing the feasibility and approaches one might take to ensure safe usage of such AGI systems.

UNIFORM - A syntax, semantics, and implementation of languages optimized for code/data interchange and for meta-programming. Uniform languages and Uniform Code Markdown take a least-commitment approach in order to maximize ease of translating code and data across disparate programming contexts. The language and markdown are designed to facilitate "LISP-like" homoiconic programming style within the modern programming stack, as well as provide the basis for a marketplace of collaboratively shared and collaboratively defined computation.





UNIFORM - A syntax, semantics, and implementation of languages optimized for code/data interchange and for meta-programming. Uniform languages and Uniform Code Markdown take a least-commitment approach in order to maximize ease of translating code and data across disparate programming contexts. The language and markdown are designed to facilitate "LISP-like" homoiconic programming style within the modern programming stack, as well as provide the basis for collaboratively shared and collaboratively defined computation.




UNIFORM - A syntax, semantics, and implementation of languages optimized for code/data interchange and for meta-programming. Uniform and Uniform Code Markdown take a least-commitment approach in order to maximize ease of translating code and data across disparate programming contexts. The language and markdown are designed to facilitate "LISP-like" homoiconic programming style within the modern programming stack.

