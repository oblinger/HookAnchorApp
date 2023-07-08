
# PTRS
- FOLDERS: [[SV Root]] 	[svdata](https://drive.google.com/drive/folders/10VWKl_3u0--LyLZmzgu8So7i1Jgq6HSr?usp=drive_link) - 
 [[CV.]]

- [Shared Folder](https://drive.google.com/drive/u/4/folders/1GdLWPNXw6lNDp9kxLjIQsaX1zAcnszAl) - 
- [BitBucket MVP Algos](https://bitbucket.org/SVEngineering/workspace/projects/MVPAL) - 
- [JIRA](https://software-engineering-team.atlassian.net/jira/software/projects/AIT/boards/25) 
- [CONFLUENCE](https://software-engineering-team.atlassian.net/wiki/spaces/SVAI/overview) 
- [[Agenda.]] - 

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

### 2023-07-03  Flow

STATS
- Hoop Detect
- Shot Detect
- Make Miss


=== SHOTS ===

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


=== SHOT RELATED ===

ASSISTS

REBOUNDS

SHOT MAP


=== SUB SYSTEMS ===

PLAYER TRACKING
- Person tracker
- Player ID
- Belief Propagation

- PLAYER ID
	- ML Trainer
		- GALLERY



=== LOGIC ===

ACTIVE HOOPS


ACTIVE COURT


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

