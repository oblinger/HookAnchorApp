- [[MGR]], [Wk Agenda](https://docs.google.com/document/d/1E5a-4Hvj2LX9KkdSPbcblWSBs3ZwdTgBvY0FA6NKytU/edit?usp=sharing), 
- [[ROADMAP]], 
- [CV Mgt Meeting Folder](https://drive.google.com/drive/folders/1M89W2MQgTDPL8t90l_z2mo2km7-ozvuo) 

# Log

### 2024-12-16  W

- Created the TopViewMapper module to produce positions for every frames, 
- Creating softbank's stats in every AI minutes job, 
- environment update in preparation for SAM2, 
- VB halos produced for James, 
- finished object detection dataset

BACK


### 2024-12-13  Fixing AI & SW products

THE ISSUES
- SW&AI having different ideas about how things work.
- Next task not happening immediately
- Issues not fully thought through early on.
- Specs are simply not written. 
	- Jira tix are TASKS not specs
	- Versioned w/ change log
	- check boxes
	- explicit email of change


- - Not being on the same page.
	- Formal spec first.  


THE SPEC
- Identify when we need one at least 2-3 months in advance


### 2024-12-09  W

- Coord: Jack, James, Brian
- Getting SAM2 in system.
- Shot charts validation; soft bank; VB defections and actions
- Datasets:  Shot charts, Object Detection, Minutes, 
- Metrics: 
- VB ann speedups


[STORY: Complete Norm1 [Juan]](https://www.notion.so/STORY-Complete-Norm1-Juan-154719d6f2bd80afb761f62fa22a3936?pvs=21)
[STORY: VAL24 - Validation set [Juan]](https://www.notion.so/STORY-VAL24-Validation-set-Juan-105719d6f2bd809d9170c84a0c8d762d?pvs=21)
[STORY: Obj Detect [Briones]](https://www.notion.so/STORY-Obj-Detect-Briones-154719d6f2bd80448ce4df88b73009b5?pvs=21)
[STORY: Make/Miss dataset](https://www.notion.so/STORY-Make-Miss-dataset-154719d6f2bd800b9179e2c89d67ac52?pvs=21)
[TASK: VB games for Crossover [Juan]](https://www.notion.so/TASK-VB-games-for-Crossover-Juan-479e40d2950847bf8b642d2c072667ba?pvs=21)
[STORY: VB shot annotions](https://www.notion.so/STORY-VB-shot-annotions-154719d6f2bd80e08d13dbe741bde751?pvs=21)
- [[[AIT-1163](https://software-engineering-team.atlassian.net/browse/AIT-1163)]]. Gather AI-min data


### 2024-12-02  W

- FANCY ALGS: SAM2 implementation for hard shot origin cases (but not integrated to the pipe yet), 
- SHOIT CHARTS: shot charts end-to-end using data from the annotator, 
- DATASETS: make/miss dataset collector, finished shot chart evaluation script and dataset for NORM.

- Q4 Analysis deliverable
	- BB improvements
- Softbank BB stats deliverable
- Dataset & Training

- Analysis this week.. VB speedups.

- AWS:
- Oct 8229  
- Nov 8851

### 2024-11-27  Monthly Meeting

- Product Velocity: alpha-testing, matias time, QA time.
- Martin owning the relationship.
- Docs & handling

- Good progress on infrastructure.
	- might need some annotator time.

- Good progress on BB shooterID & BB stats
	- only one person.  Focusing on core shooter stat.   (airballs, game breaks, ...)

### 2024-11-18  W

- Object detection dataset ready, 
- VB crossover exp

- Shot origin improvements (I don't think we have numbers yet, but Greg reported something like 19 cases fixed), 
- minutes runtime breakdown, 
- play/shot charts integration in the pipeline and beginning of play/shot charts integration with software
### 2024-11-11  W

- Play charts
- VB crossover experiment
- Scripts for ball track data
- Shooter ID
- Shot charts
- AI-min normalization bugs
	- specs, code, fixes
- fireball fixes 20% offscreen

### 2024-10-28  weekly

- Fixes, memory
- ShotCharts
- Softbank
- Fast Response


~ older stuff ~

ask: sean, Jason, jack -- target for BB speedup
- S3
- RP play format
- Minute data format?
- fusing
- IO.save(path, uri)

```
### 2024-10-14  weekly (never presented)

- Progress on data collect and metrics.
- Homography on master dev
- progress on new color model
- Standardized data pulling/pushing by name? e.g. gameset/bb/NORM/


### 2024-09-30  weekly

- [ ] 
- Softbank b2b2c
- GCP availability
- New Color model
- Fixed Fireball bugs
- Homo helps minutes
- End2End homography PR this week
- Games for BB Testing, ground truth along many dimensions(homo, shots, trackID, bounding box).  Metrics of intermediate and final outputs
- LLM Analysis

### 2024-09-16  Weekly
SHOW
- IMAGE of edge case:  
- matrix of results for min
- Homography:  Games 10, 2, 7 (edge jersey color contrast), maybe 8, 9

- Min results
- Homography progress:  v1 UX, v1 algs, end2end
- Annealing alg improvements
- Fixing colors - Generating galleries 
- New Fireballs version in.  Pushing for v2.1


- QA minutes 
- QA colors


### 2024-09-09 Weekly
- Fireball - 2 ways
- Assessment
- Track stealing issue 75%
- Homography ++
	- Load ann ct template
- Annealing game logic ++

- Ops tasks cost/benefit
- Camera choice
- OpenCV.ai
  

### 2024-01-18  FEB Meeting
- [todos](https://docs.google.com/document/d/1wASzo79-0WP6c84z7g2xbqAhXtiOx3Ucm6BY2mDBIeg/edit) 

- [sean](https://www.notion.so/sportsvisio/0d3cb69afa1a459996539e3ea97ea12a?v=e23b2f5802a941b99dd83b1b92957b3c&pvs=4)     [Sean's slides](https://docs.google.com/presentation/d/1ukAw-WWxuqCf3c_vdU79AHPuhjWVQUUdVUWAPBSw37g/edit#slide=id.p) 
- [BoD](https://docs.google.com/presentation/d/1mEps17ZJfOlh1biiBLWp2eKhu3Ti79pY/edit#slide=id.p1) 

AI STATUS SLIDE

Status: Improvements 

### 2024-01-11  JAN meeting

Topics
- Pivo
- 2024 focus
- 

2024 Focus: 50%+ effort on player ID
- First months really looking at all baller10
- Need to make this fast, easy, robust 
- Need stronger usage of james codes;  shots ground truth
- Expect more "needle movers" but quite possibly need better video, more camera, different sport etc. for 95% accuracy.
     (still don't know what we don't know, )


Seeing Better / Executing Better
- James Codes
- Shot truth
- Sarthi / Martin visualization

Executing Better
- Juan/Dan Encapsulation Game & Run
- Show: 
	- Script to assess gallery
	- Sarthi vis of image space rerun.io


Marketing Analysis
- Product Ideas
	- Volleyball Highlights (bump set spike per-team)

- Precison / Recall log scale
- 98 95 90  85 80 70  60 50 30


Possible Products
- Volleyball
	- VbTh - Highlights Per Team
	- VbPh - Highlights Per Player
	- VbPs - Bump-Set-Spike-Score Stats Per player
- Basketball
	- BbTh - Highlights Per Team
	- BbPh - Highlights Per Player
	- BbPs - Bump-Set-Spike-Score Stats Per player
	- BbMin - AI-Minutes
	- BbChart - AI-ShotCharts





### 2023-12-07  DEC MGT Meeting

Inputs x annotations x James x internal signals

x scripted logic for experiments/exploration
x aggregation over many input/run/params
x visualization of output


STATUS
- Team quality is quite good, as a CV Engineering team
	- two A players, three B players.  Intelligent leadership.  
	- Excellent given the price.  Post A we may hire a CV PhDs, for now it would be wrong

- Processes need improvement
	- Planning
	- Documentation
	- Testing - 

- Not tracking players as well as we need.
	- Three sprints to crack this before branching out.
	- Good news, this is central for nearly all 


NEEDLE MOVERS

- Fast Homography
- Belief Prop
- Threading
- Gallery Updates
- Seeing
- Doing

### 2023-12-14  Jason Talk

- March Delivery @ risk
	- Can happen, add several improvements in pipe
	- Beyond those, we need to get deeper into prob areas
		- one at a time - can't delegate this
	- Meanwhile need to upgrade Engineering quality
	  its hurting our velocity

- Remedy
	- Volleyball - 
	- Pivo - 
	- Multi-sport Showcase - 
	- Be later - 



--- Highlights product @ risk for 80% by March
     -- might stackup enough improvements
--- Pivo product

- +++ Better video
- +++ Fast Homography - off court filtering; lineup; tracking
- ??  Belief propagation
- ??  Threading tracklets into the scrum
- Seeing: James/filtering
- Speed: Datasets/caching/sharing/restarts/multiplex
- Gallery fixes? - semantic digit reco
- Pose
### 2023-11-13  NOV MGT Meeting

[Nov Canva](https://www.canva.com/design/DAFztB__Tz8/sTXhEG4p51SIZO2N7d61vw/edit) 


### 2023-10-11  OCT MGT MEETING
[Canva](https://www.canva.com/design/DAFxLGPioaI/z7FJd3p9tKKtlgPD25zNvw/edit) 
- Full AI
- Pre-A Ingest solutions:  Air-table --> API
- AI Camera
- A-la Carte
	- Customer Success Spreadsheet
	- Password management

- high level planning - A-round - Jack's task breakdown
  SW TEAM
	- Robust & Stable: Hoops, Ann Portal, Admin Port, Air Table
	- Mobile League Page  (includes refactoring programs, etc.)
	- Web Stats Page
	- Virality experiments / features
	- Tweaks:  Ops team improvements.  Easy features. Easy UX.
  AI TEAM
	- AI-Shots 
	- AI-Halos 
	- AI-Minutes
	- Reduce Risk on Full AI sport
	- Productized Full-AI
	- Prototype Second Sport
    

### 2023-09-25 Weekly meeting. 


- 8-apps --> 1-app


ACTIONS
- approach 100% time on hoops bug fix 
- - hiring QA <---
- formal of kind of bugs <--- assessment of the team
- hiring backend guy <--- programming test
- hiring new SW lead.
	- top to bottom assessment of team
	- brian decision
- automated testing


- volleyball <--
	- Miguel
	- James notes


- Formal Assessment of bugs
- QA hiring
- 


### 2023-09-15  SEPT MGT MEETING

Cust
$400K baseball
$600K channel partner (Cerebro)
$1M (50 med cust)
- 30-40 Men's leagues
- 40-50 AAU 
- ...    HS. (no football).  300 HS



- [canva](https://www.canva.com/design/DAFuuMz7DoQ/lPVWLYkeyECDR5L1dNbWyw/edit?utm_content=DAFuuMz7DoQ&utm_campaign=designshare&utm_medium=link2&utm_source=sharebutton) - 


- Ingest Slides
	- Roadmap ordering
	- Segments Slide
	- Approaches Slide
	- rPI Rig

- A La-carte Topics
	- Patents
	- Baseball
	- Stabilize Hoops & SW
		- PAST:  Jack/James
		- FUTURE:  2 full time QA, Field Testing --> Automation

### 2023-08-09  AUG MGT MEETING

- Backend: Use Juan
- Ground Truth: Use PH
### 2023-07-12  Deliverables

- [Canva](https://www.canva.com/design/DAFrEaB1aNU/eVOTF31pSlykZx6ifJMWAQ/edit) 

TASKS:  [Spreadsheet](https://docs.google.com/spreadsheets/d/1X9YNns5sMzTPdFIFhtqqVR47SV4Y7jgbbYfDN55k1GM/edit?usp=sharing) 
- Work with Jack to deliver core SV Hoops PRODUCT & AI Team Roadmap
- Present Q3 and Q4 deliverable plan at next monthly meeting
- Baseline Pivo cold storage status
- For Record app also (Find clean break point)
- Baseline for BallerTV for Player Identification
- Look into/consider preparing patents for our new ML approach to player tracking, see if we can file a patent, also will likely need 2 patents filed for Series A.


#### SLIDES

- Testing
- Pivo
- Record App

#### Explanation

- Full Regression Hoops/Ann/WebUI.  
- Exploratory [Testing](https://docs.google.com/document/d/1iKJLualzSJbBKR8K3qjpuOhf-y6cmDFgsPIW-8nXXAg/edit#heading=h.xv6bvzuutwj0): HOOPS:  Maximo, TT, Achilles, Jasu, Nasa, Leo, Juan
- Exploratory Record: similar James + Columbian Team
- Diversity Test:  

### 2023-02-15  Comments with the Team


- ALPHA REGRESSIONS - Recording process regressions.  Screen length rollup

- TESTING APP UNIFORMITY - 
	- Data copying
	- Trivial App login change.

- PRIORITIZATION / PRECISION in goal - Pivo prioritization

- FORK IN GROUPINGS - of users, players, games - Backend mgt



BUG TRACKING
- Minimize handoff lags, 
- Ensure uniform/complete processing of each bug



PRODUCT RELEASES
- Eye on the prize
- Effortless/continuous status


R1 - March Release (robust usable human annotated stats)
R2 - Rolling Release 
R3 - AI-Driven Annotations Release
R4 - Social 


QA PROCESS 
- Fast handoffs, Fast scaling.



Cerebro - tournament - 


Demo ideas
- Heatmap of shot map
- Mancoverage 


BluePrint stats - 4-5 guys in the PH. 

Annotation Efficiency.

#to-do
- .What percentage of tournaments are using Cerebro?
- .Find admin support for, annotation of games, of managing customers.
- .People who can be our alpha testing.
- .Get docs ready for offsite:

#todo
- [ ] one Item here