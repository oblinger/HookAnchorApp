



.**TODO**.
 - expansive testing of the manager app
 - DATA - 100 PlayOn/HQ Games, eval humans, Origin data, 
 -  [PlayOn](hook://playon): Roster - get and use
 - [Cloud Roadmap](https://docs.google.com/document/d/1g135euS74c7FfBO6GDMRGyvi_0jXHBm4Pd4bqU9Pa94/edit?tab=t.0): plan for moving stuff to cloud - 
 - [Media convert](https://www.notion.so/sportsvisio/SV-Hybrid-Media-Convert-Prototype-27d719d6f2bd805084dfeb5b166debcb?source=copy_link):  SW Spot instances
 - Darien
 - OCT 19th - own meeting
 - [[PHQ]] 
  BACKLOG
    - AI-code brainstorm    [[2025 AI Codeing Brainstorm|AI-code]]  
	-  [AWS Costs](https://drive.google.com/drive/folders/1Mnu6bwVw9bqxGq40uM_fBZ6x_75O3c6H)  (talk w/ Matty?) 
    - Pursues beta testing in app
	- !summary single metric for BB 
 SOON [[hack]]
    Beta tester stuff
    [[@Google]] GPU  answers
    rerun softbank; send email 
    spotlight effect (fireball++)
    stability:  upload stress test
    Test automation:  (1) Mery Impersonation; (2) API testing.
    Top of mind from Jason; ask for topics before (for mgt meeting)
## PAUSE
  Baseball opportunity
  Effort Analysis for BB stats
  Halo analysis
  [AI Prod Matrix](https://docs.google.com/spreadsheets/d/1iQvOKwS09QcFTqXK1Ko0ydSgwqjXItbCRrkOMZaBj40/edit?gid=0#gid=0) possible products
  [**BUG**](https://www.notion.so/sportsvisio/SOFTWARE-STABILIZATION-1eb719d6f2bd807586afdba684e60901?pvs=4): Beta, RegTst0, Factory, [R](https://www.notion.so/sportsvisio/Regression-Testing-1f4719d6f2bd80ddaa0de76008e9f41b?pvs=4) 
  [**PATENT**](https://www.notion.so/sportsvisio/PATENTS-3434075a725f4997b57d9aa9bd0b818a?pvs=4): (do it)
  [**CAMERA**](https://www.notion.so/sportsvisio/AI-Camera-e8e5e2d211604c9cbe81ca19fa11f8cd?pvs=4): LOE Plan,  [V1](https://www.notion.so/sportsvisio/AI-CAMERA-v1-1e0719d6f2bd80a3abb8eac297561b07?pvs=4) 
- F11 [AWS](https://docs.google.com/document/d/1QS4081TT_Yyz1TS9GLZSfjQaczQ7ynzWTSu14waDcG0/edit?tab=t.0):	Estimate from Brian [[2024-08 Backend Architect|Arch]]: [r](https://docs.google.com/document/d/1gbutlP_EF-_wUdg4F3IEuw8Bhz8oET3Z/edit) 
- playcharts
 Crossover:	^james checkin
  [VB](https://www.notion.so/sportsvisio/VB-Speedup-Ideas-142719d6f2bd80889ad8fc7cb2484a91?pvs=4):	Speedup ideas
  EXP:  [HotKey](https://software-engineering-team.atlassian.net/browse/WU-1935), J^5games 
  GCP:	[Brian plan](https://docs.google.com/spreadsheets/d/1W38uHew6aaqyeolKzbO9NbaA96Q-jzyDb7gYoUst0qw/edit?gid=0#gid=0). 
  [SW cort](https://www.notion.so/sportsvisio/HOMO-SW-integration-plan-77fb63f7cac7433bb0d2e2cd4d374e4b?pvs=4): Get plan
  VidQ:		^Neg entropy gallery
  SW proc:	[db](https://www.notion.so/sportsvisio/cf65339c09c6486296f8adaa084c141a?v=f8a45e385eea4c2fb590502835f4453f&pvs=4),[doc](https://docs.google.com/document/d/1aHnoqgQ3H_0abUtlVWoMPHDA7nQhzoq0JRqZLujORc0/edit?usp=drive_web&ouid=105281685541312342727)  ^Jasu 
  Assess	> Assess plan for BB
.[[work]].
  , [[SV]],    
  , [work Obsidian](spot://workobsidian),
  DELS: ,[work Obsidian](spot://workobsidian),
  , ,  
  , [work Note](spot://worknote), 
  DELS: [work Note](spot://worknote),[work Obsidian](spot://workobsidian),[work Note](spot://worknote),[work Note](spot://worknote),
 SOON [[mgt]] [[CVP]] l
[[Journaling]] 







# __

Codes:  D2332, D2391 (2), D2392 (3)

DETECT
- State of the ball (Dribble, Held, Pass)
	- Train a model
	- Use CV algs on ball tracks
	- Correlate with player tracks to assess who is where
	- Train a still-image or action model for player actions

RISKS
- Don't have reliable start/stop times.
- Feature - Finding game start using the Timing on bottom screen
- Court corners - 
- alignment of video time, w/ annotator clock time
- Bandwidth - Optimistically available in Feb for real work on PlayHQ
- Rebounds and assists depend upon ball possession, need invest time on this.
- Speedups are still needed.  
	- Long game times (no dead time detect)
- No court corners
- Availability of GPUs
- Ball possession
- Accuracy
	- Accuracy of all the things we think are working (but are probably not)
	- ShotAttempt
	- AirBalls
- Valid Usage
	- The customer is getting charged for multiple submissions of the same game.
		
- Game Validity
	- 2/3 of games in from James are invalid
	- Assuming games are refereed and folks have jerseys etc.
- Ops Risks
	- Feedback, bug volume
	- Managing Kube Cluster: loose money, fail to execute game, mis-understand execution
	- Disaster Recover Plan 

- Roster 
- Color

V2 STRATEGY
- Provide a signal when a game is likely to be of low quality.
- 2/3 point line determination
- Ability to terminate processing mid-flight

PHQ
- Need ball possession - rebounds/assists
- Need good timing of the actual shot (with 10 sec misalign)
- Shots should be accurate




show AI running
show fireball & magic hour
table showing improvements

https://docs.google.com/presentation/d/18-EgPTzjksHM8LPWDcRwrUl2pCflegCELsh3l2eQ8OE/edit?slide=id.g39bdd97e068_0_350#slide=id.g39bdd97e068_0_350

