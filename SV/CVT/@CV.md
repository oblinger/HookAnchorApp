- Find interaction rough.  We seem to be agreeing in the end, but somehow it seems I am shoving him the whole way.
  Need to achieve tempo in actions, and crispness in intent and outcome but often don't.
- Clearly need group discussion and PhD oversight to guide each dev, but question velocity of trying to have all devs understand all code
  (not sure).  PR process is taking a loooong time.
- Who should own master dev & push to prod?   Martin
- Who should own testing?  Maxim
- Who should own end-to-end halos?  Sarthi
- Halos:  

- [[CV Log]],  [[CVT]],  [[CV Planning]], [[CV.]], 
- [LCM Input](https://docs.google.com/document/d/1kxGMrmPOr9S6whRnV0LI0UpWGeyBzTn8WGDq7ITM4Sc/edit),  [LCM root](https://docs.google.com/document/d/10jm1RRqCqvAXy3Ti1Nu-jLzATy0jjBejUY7-Dz2e85U/edit), 

## People
- GRZEGORZ - PlayerID & Gallery; Common Classes
	- Recognizing 
	- Propagating Player IDs across tracks
- MAXIM - Build Dataset; Metrics; 2-3-point (homography, )
	- HOMOGRAPHY - 
	- POINTS - 
	- CVAT - waiting on CF
- MARTIN - Dev-QA-promotion process
	- Making longer tracks
	- EVENTS (waiting on what?) - 
	- Money-metric - 
	- Keeping track of players in invisible
- SARTHI - 
	- Halos
- VISHAL -
Propagate playerID. 


## Todo

- [ ] Visualizers:  2D.  DataCube. matrix
- [ ] DATASETS

 [[@CV]]: Evt Datasets: format, video, annotations
- [ ] [[@CV]]: Ground truth datasets
- [ ] [[@CV]]: Demo (shotOrig, end2end, classes)
- [ ] Sidney (trained, docs, g-sheet)


THINGS WE WANT
- [ ] Experimentation Pipelines
	- [ ] Command Line 'System' translator.
	- [ ] DVC setup
- [ ] Datasets
	- [ ] Box truths
- [ ]  Taxonomy of errors
- [ ] Heavy Execution
	- [ ] Fong Machine
	- [ ] Lambda 



[[cv]] 

Taxonomy of errors
- [ ] Recall error - Just didn't see the shot
- [ ] Prediction error - Wasn't a shot attempt
- [ ] PredictMake/Miss error - wa
- [ ] Shot Pixel Error
- [ ] Gallery Errors
- [ ] Player ID error


Execution
- [ ] Build setup scripts
- [ ] signup for lambda
- [ ] build fong machine
- [ ] starting from console


- [Baller Demo](https://docs.google.com/document/d/1U0JQ4TQ9Ojblw6OXPuYyPId7X5tq42-8gwmLJNfbswQ/edit)  

[https://bitbucket.org/SVEngineering/algorithms2/src/master-dev/](https://bitbucket.org/SVEngineering/algorithms2/src/master-dev/)





## Tasks
- Halo pipe

# PTRS
- FOLDERS: [[SV Root]] 	[svdata](https://drive.google.com/drive/folders/10VWKl_3u0--LyLZmzgu8So7i1Jgq6HSr?usp=drive_link) - 
 [[CV.]]

- [Shared Folder](https://drive.google.com/drive/u/4/folders/1GdLWPNXw6lNDp9kxLjIQsaX1zAcnszAl) - 
- [BitBucket MVP Algos](https://bitbucket.org/SVEngineering/workspace/projects/MVPAL) - 
- [JIRA](https://software-engineering-team.atlassian.net/jira/software/projects/AIT/boards/25) 
- [CONFLUENCE](https://software-engineering-team.atlassian.net/wiki/spaces/SVAI/overview) 
- [[Agenda]] - 

- [[CV Planning]] - 


# Stuff
## = ALGORITHMS
DeAOT (left) and byte tracker (right)

#### Segment and Track Anything
https://github.com/z-x-yang/Segment-and-Track-Anything

## = IDEAS

#### Bayesian Track-let Identity Mapping

#### Deep Learned Identity Model

## = MISC INFO

### METRICS 
#### J-Check
#### P-Check


#### Tracker Info
- Velocity
- IOU

94%
05




# LOG
### 2023-07-13  Progression of halo apps

- Player ID 
- Gallery Fleshing - 
- Player ID tracks (not just individual frames)
- 

### 2023-07-03  CV team components

STATS
- Hoop Detect
- Shot Detect
- Make Miss


=== SHOT STATS - SUB SYSTEM ===

SHOTS

- MAKE/MISS

- POINT VALUE
	- Court Lines

- SHOOTER ID
	- Shot Origination Pixel
	- Pixel Person Association

- GAME PLAY STATUS
	- Jump Ball
	- AUDIO: Whistles, Calls
	- Idle Players
	- Free Throw


=== PLAYER TRACKING - SUB SYSTEM ===

PLAYER TRACKING
- Person tracker
- Player ID
- Belief Propagation

- PLAYER ID
	- ML Trainer
		- GALLERY



=== SHOT RELATED ===

ASSISTS

REBOUNDS

SHOT MAP



=== LOGIC ===


ACTIVE HOOPS


ACTIVE COURT


ACTIVE PLAYER


=== BASE COMPONENTS ===



TRACKERS
- Ball Tracker
- Hoop Tracker
- Person Tracker

DETECTORS
- Ball Detector
- Hoop Detector
- Person Detector


HOMOGRAPHY
- Camera Tracking


CAMERA TRACKING
- Location & Orientation Relative To court
- Vanishing points



=== CROSS CUTTING ===

PERFORMANCE


=== METRICS ===

EVENTS


OBJECTS



=== PIC ===
- Accuracy Overview
- Performance Overview
- Ceiling



### 2023-06-27
- MARTIN - Active-Hoop -> Hoop-Assign
- MAXIM - Build Dataset; Metrics; 2-3-point (homography, ); DONE Shot Origination;
- SARTHI - Make/Miss - 
- GRZEGORZ - PR Gallery
- VISHAL - 


- jason slide; achilles; datasets; grz-leave

- BODY-BLOCK - 
- MOVE TO FAST - 
- FALSE ALARM DRAGGING OFF BALL - 


.4 = 9.3 - 7.9 


### 2023-06-20  GRZ how many frams

100K frames per game
20 detects per 

2M 

### 2023-06-06  daily

### 2023-06-06  ??? 

- Docker; GG-Agenda; Gallery; Dataset; 
- GRZ=ReiD, Martin=Dataset, Maxim=Global, Vishal=Stats+Reports


Where the player made the shot.

- Game Tester: shot detection. (game harvesting)
- shot orgigination;  ball location;  ransack (to find parabola)


- shot detect   (recall is strong; precision is weak;   make/miss/other)


- Good-track-turned-bad- (could have been a track swap)


-- least commitment piece
-- bring in other info
-- multiple hypothesis tracker ()
   -- x. Track the crowd
   -- use a horizon and collapse multi-hypothesis
   -- MVP player tracker
   -- track before detect


Pcheck - martin's version - truth file is CSV + pickel - 
- look in the confluence doc; SVAI -> PT player tracking > results > CXY-id 

### 2023-05-31  Daily

- Martin -  problem getting bytetrack + DoT did not work well
- Maxim -  measuring code

a

### 2023-05-30  Daily

- Vishal


- Maxim working on metrics

### 2023-05-29  Daily

- Grz got gallery; trying to short track re-id


- CSV output 


- integration dot 

### 2023-05-24  Daily

Maxim
- working on metrics.

Martin
- ??

Vishal
- Not getting results

Jersey detect
95% P. 85% R. 



### 2023-05-23  

ToDo:
- Money Metric
- publish Decomposing the Stats problem

Maxim
- modified the metrics
- 

### 2023-05-22  Weekly 

Standup. 
- One week left integrating color tracking (&velocity into) tracker
- three weeks will be done w/ first sprint doing Re-ID


TOPICS

PLAYER-ID

METRICS
- MM annotations
- Need good internal metrics for Player ID
- Want to build stats-oriented metrics for Player Tracking
- Using MM folks to generated testing data


Baller Video - 
- 1-2-3 point detect.   95% accurate
- player number.  80% accurate
- rough time stamp

TODO
- get video from baller; get contact person?
- get PIVO testing person



### 2023-05-22  daily

- May-26th - pcheck; player tracked;
- June-9th - re-id


Kalman filter-based tracker

Martin:
- Better results and pcheck. (now is a little better that byte-track tracker)
- See if extend TTL

Maxim
- outperform bytetracker
- add connected componets.  (fragments because of large bounding box; use connected components)


Grzegorz
- Trying to implement re-id 


MEASURE
- Internal 


IDEAS
- Annotate more frames
- Use bounding boxes & IOU
- TRACKING RATIO - 


### 2023-05-22  TAKING MEASUREMENTS

Measuring Player-ID / Re-ID

Pivo

- Ball Tracking	
- Player ID
- Hoop Detect	
- Make/Miss
- Player Position
- 2/3-point			


player - pcheck
player - bcheck 







### 2023-05-17  [[SVCV Re-ID]] 
### 2023-04-20  Big Ideas

- End to End stats estimation
- 

### 2023-04-20  What are people doing?


MARTIN
METRIC ANAYSIS - 
Maxim - Analyzing metrics 
	- Soci paper
	- 

Association Paper


MAXIM
Maxim - Visualization tool.


GRZ - 
- PCHECK - 

