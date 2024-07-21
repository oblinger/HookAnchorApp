
:: [[2023-12 CV Tree]]
.[[CVP]].
  WEEKLY:		[SprintPlanning](spot://sprintplanning), [[CV Roadmap]], [Backlog Planning](spot://backlogplanning),
  QUARTER:		[OKRs](spot://okrs), [OKR Slide](spot://okrslide),
  DATA:			[CV Planning Folder](spot://cvplanningfolder),
  SW TEAM:	[Pln Factory](spot://plnfactory), [Pln Work](spot://plnwork),
  ADDS:   [JuanDaily](spot://juandaily),
  ADDS:   [Weekly Agenda](spot://weeklyagenda),
  ADDS:   [tasks](spot://tasks),
  ADDS:   [[2023-12 CV Tree]], [[GameGraph]], [[Infra]], [[Player Tracking Problem]], [[Rpt]], [[ShotTopView]],
  ADDS:   [[CV Annotations]],
  ADDS:   [Brainstorming Q3](spot://brainstormingq3),



GB: 	Finish OCR
MR:	.
Mx:	.

Holdout set.
hoop metric.
plan.


# TOP
## __
- env.yml, sarthi, 

- Error Code Analysis
- PlayerID optimization (includes combining Player-ID + OCR)
- Ball physic - for passing

- Hands up
- Team hoop assignment to shot origin
- Team hoop assignment to ball possession
- Smarter "adaptive" ball 
- Improve SC courtlines
- Region of court analysis
- Passing Detection
- Hoop Assignment




# CVP

## PL - TREE


## PL - SOON
- Team hoop assignment into shooter ID
- Four points for homography
- Gallery thumbnails
## PL - ACTIVE
- G: 	Hands up
- J: 	Homography
- X: 
- M:	Alignment
- S: 	Threading / Object permanence


	186T  .303T/s 	1.6Q total 	>>363T   >>240T
	40?T	2.7T/s	3.2Q total


## Todo
- 



- halos accuracy analysis (vball, Baller, ClearVideo)
- list of fixes
- volleyball

- G-doc: Errors, Profile
- Tighten the screws: top-priority-first, tight-handoffs
- Tighten PRs, P2P, BugFix, RunPipe, QuickTask (test)
	- martin was not doing the PR.
	- datasets, push2production, PRs, bug fixing

- showcase - homography

SPEEDUPS:	
ERRORS:		
METRICS:  	Baller10-MM-do, profile_table
CAT_BUGS: 
VBALL:		MD: Dset(playerID), ActionDetect, MR:
BASEB:		Max POC
LCM:			Docs

## Planning
- [[CV Needle Movers]] - 
- [[CV Roadmap]] -
- [CV Roadmap](https://www.notion.so/sportsvisio/CV-Roadmap-093f05a080994c07bcdb2a9b865f99aa) - Notion
- [[Dataset]]
- [[CV Annotations]]. 
- [[CV Anal]] - 

### Ideas
FROM JUAN
- Abstracted and standardize pipe.
- Add ML-flow automation
- Restructure the Repo
- Type and document the code

FROM JASON
- Probabilistic model of shooter
- Orientation of offensive players
- Shooters are never running
- Offense tend to shoot from same sides

- Train a model of a shot (w/ defenders)

## Info
## Datasets
### Events - ask GRZ
### Objects - Task James


## PlayerID

## Diagnose ShooterID Errors


## __


## PTRS
:: [[Infra]]

::   [[Player Tracking Problem]],   [[Rpt]],   [[ShotTopView]]

PLANNING
- [sprint planning](https://docs.google.com/document/d/1F2hISCp9p-uvfzVt6OTclhOGswQ9EmbVwGxKk9uqJ28/edit) - 

[sheet](https://docs.google.com/document/d/1XF1NfYpF7gqGBeLYZ_b-mBNkHxZgsRwW78p8ICIEHoQ/edit)	
- [[2023 Least Commitment Model For CV Pipe]] - 
- [[2023 CV team new agenda metrics]] - 
- [[Infra]] 

[Metrics Folder](https://drive.google.com/drive/folders/1CO5eTdcTrUS9T8BkBcMBBE9e3B6YORvg) - 
- [Agenda](https://docs.google.com/document/d/1Wrg8rrMgmPAkl-g88IEImaUjQMaAhS49Tv8nnShLGL4/edit#) - 
- [Least Commitment Model](https://docs.google.com/document/d/10jm1RRqCqvAXy3Ti1Nu-jLzATy0jjBejUY7-Dz2e85U/edit) 
- [Components and Metrics](https://docs.google.com/document/d/1KC3aFWQPWtzhA8CN2572ysNtyofN5DUKrEif6HJYCoA/edit) 
- [Game Graph](https://docs.google.com/document/d/1ZL5zMCpwDJ3Ai2groT2dmtfiw0yFLfnt3aU0u4mm7zw/edit)  - local [[GameGraph]] 
- 
- [Player Tracking Problem](https://docs.google.com/document/d/1Wrg8rrMgmPAkl-g88IEImaUjQMaAhS49Tv8nnShLGL4/edit) 
- [Dataset Construction](https://docs.google.com/document/d/1_xntQQvwLlmRNKL8isX6tQi23R22KMkdUBFAbMoUtbU/edit) 
- [The Money Metric](https://docs.google.com/document/d/1wBDwrPvgvY4CiYC26zec6c5hSwiYX1CmiVVldVBhnZw/edit) 
- [Gallery Metrics](https://docs.google.com/document/d/1l07TNwaT5wF39WxN8DWrhivPqj_57exYrKV_8u01600/edit) 


- [ ] NuGallery, Datasets, Metrics
- [ ] [CV plan](https://docs.google.com/document/d/1sxwVOEqIynj4-bkg97b4HQCeodnB743ebamrUw0BMnk/edit): Mike's list
- [[GameGraph]] - 
- [[Player Tracking Problem]] 
- [[ShotTopView]] - 
- [[2023 CV team new agenda metrics]] - 
- [[Rpt]] - 


# LOG

### 2024-06-11  
GB: RP pitcher.    OCR, 
MD: Homography, 
MR: RP, 
JB: Metrics & more
SR: Minutes

### 2024-01-23  Design for Notion

Planning lists:
- Now, Next, Soon
- Needle


### 2023-12-01  ideas from james
- multiple hoops tips
- using ball arc estimation to reject non-shots



### 2023-11-09  Still image Shot Analysis



### 2023-11-06  Weekly Management Meeting

- Version Two Err Assessment:  Visualization, Error Codes, James
	- add passing, integrate possession&origination
- Add Ball passing and integrate possession
- Integrate OCR & Player ID model
- Experimentation Speedups
	- Caching Galleries w versioning

### 2023-10-31  Big Rock Planning

BLIND
- Visualizations
- Datasets
- Analysis

VELOCITY
- Turn Key Actions

IMPROVEMENTS
- Merge Shooter & Possession
- Smarter Possession
- Better ball tracking

### 2023-10-30  kinds of visualizations


____
- Debugging Video:
	- Gallery Slide 5 examples of each player
	- All boxes. best ID & score


____
- Shot Analysis - S:couple of days
	- Real Shooter
	- Predicted Shooter (halo above)


____
Maxim Frame-by-Frame 15 min halo measurements
- Sar: 
- Frame-by-frame accuracy


### 2023-09-21  AI goals from Mgt meeting

I added clarification parentheses and other items from the notes, then ordered them.  
1.      "Halos need as soon as possible"  
2.      "95% accurate player IDs" (95% accurate shooter IDs)  
3.      "game breaks" (start, time-outs, half-time, re-starts, end)  
4.      "Possession changes" (includes steals)  
5.      "blocked shots"  
6.      "Fouls detection" (especially fouled shots)  
7.      (Assister IDs, Rebounder IDs, Fouler IDs)  
8.      "no-human-in-the-loop annotation service" (95% accurate)  
9.      "2nd sport"  
10.  "demo experiment showing faster-than-human annotations" (25% faster)  
11.  “augmented reality”  
12.  “instant replays”  
13.  "Big plays (crowd noise)"  
14.  “auto-scoreboard”

### 2023-09-05  older stuff

- [Stuff](https://docs.google.com/document/d/1sxwVOEqIynj4-bkg97b4HQCeodnB743ebamrUw0BMnk/edit)  

- Planning Volleyball - 
- Shot map 
- Assist - video add flames - 
- Pivo

- Ground Truth - 


SAM model
DaoT - classifier of pixels


