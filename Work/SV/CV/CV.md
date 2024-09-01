
.[[CV]].
  ROOTS:[CV Root](spot://cvroot), [CV Confluence](spot://cvconfluence), [CV Jira Page](spot://cvjirapage),  [CV Notion](spot://cvnotion), [Data](spot://data), ,
  DIRS:	[CV Google Page](spot://cvgooglepage), ,  ,
  ORGS:	[CV Docs](spot://cvdocs), [DOCs](spot://docs), [CV Info](spot://cvinfo), [CV Log](spot://cvlog),
  ADDS:	, [CV Ana](spot://cvana), ,
  OLD:	[CVT](spot://cvt), [DSET](spot://dset), ,
  ADDS:   [svdata](spot://svdata), ,
  ADDS:   [Standup](spot://standup),
  ADDS:   [APIs](spot://apis),
  ADDS:   [CAPS](spot://caps),
  ADDS:   [[SVCV Re-ID]],
  ADDS:   ,
  ADDS:   [CV Gdrive](spot://cvgdrive),
  ADDS:   [[CV Hack Notes]],





  DELS: [Info](spot://info), 







































































































































after

:: [[Data]]
## Foo

- Find interaction rough.  We seem to be agreeing in the end, but somehow it seems I am shoving him the whole way.
  Need to achieve tempo in actions, and crispness in intent and outcome but often don't.
- Clearly need group discussion and PhD oversight to guide each dev, but question velocity of trying to have all devs understand all code
  (not sure).  PR process is taking a loooong time.
- Who should own master dev & push to prod?   Martin
- Who should own testing?  Maxim
- Who should own end-to-end halos?  Sarthi
- Halos:  

- [[cvt]],  [[CVP]], [[CV.]], 
- [LCM Input](https://docs.google.com/document/d/1kxGMrmPOr9S6whRnV0LI0UpWGeyBzTn8WGDq7ITM4Sc/edit),  [LCM root](https://docs.google.com/document/d/10jm1RRqCqvAXy3Ti1Nu-jLzATy0jjBejUY7-Dz2e85U/edit), 

## Todo

- [ ] Visualizers:  2D.  DataCube. matrix
- [ ] DATASETS

 [[CV]]: Evt Datasets: format, video, annotations
- [ ] [[CV]]: Ground truth datasets
- [ ] [[CV]]: Demo (shotOrig, end2end, classes)
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



[[rrCV]] 

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

## >>> Stuff <<<
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






## People
[Planning Doc](https://docs.google.com/document/d/1F2hISCp9p-uvfzVt6OTclhOGswQ9EmbVwGxKk9uqJ28/edit) 

- GRZEGORZ - PlayerID & Gallery; Common Classes
	- [Speedups](https://docs.google.com/document/d/15hc6cdpVLLoDguF75QcoFxEzzkkHtU056-3a7bePT2E/edit)  30hrs --> 7hrs (4 speedup transforms, float16, batching, smart training)
	- Propagating Player IDs across tracks; PR waiting
	- Common Input Classes
- MAXIM - Build Dataset; Metrics; 2-3-point (homography, )
	- GENERATIVE MODEL
	- CAMERA CALIBRATION - 
	- HOMOGRAPHY - 
	- POINTS - 
	- CVAT - waiting on CF - 
- MARTIN - Dev-QA-promotion process
	- EVENTS - Json file agree.  10 files to James.
	- MATRICS - Money-metric - Money, Accuracy
	- Keeping track of players in invisible
	- Making longer tracks
- SARTHI - 
	- Fixing make/miss to have 3rd class
	- Halos
- VISHAL -
	- Memory Profiler
	- Speedups

# OTHER LOG.

[[build]] 

### 2023-11-17 Alg2 Build #build

- [[DocDocker#^2023-11-17]], 

docker build -t ob-buntu .
docker run -v /Users:/Users -it ob-buntu

// In the running docker

conda config --add channels conda-forge
conda config --add channels defaults
conda update conda

### 2023-10-23  Experimentation tool

Chris - GUI
- AI clips column / Manual clips column
- Was connect AWS buckets so could check existing games

Fong - 
- Py script - run on results - eval tool
- tool old 


# LOG
=> Michael Patent

=> AI-tips
=> Swiss Army knife
=> Team stats product

- Sarthi - Survey Volleyball on BallerTV
- Martin: give me script
- Text summary to the team

- obj-detect, ocr, playerID, shot-attempt, edge-cases (tip in), bad setup (lighting, extra balls, weird lines, other players)
- video blur. 

B10 not done / hacky / consensus / production
- vis
- run

### 2024-01-22  Juan

- porting Fong code

- working on Inst; trouble loading in mem.

Pano:  adapting to use standard court in pipeline
- found bugs 



### 2024-01-22  Martin
==>? use size of back board to decide if shot attempt
==> Getting the codes fixed from James

==> retrain ball detector. (trying to get the data)


W: 
fixing active hoop bugs
retraining




### 2024-01-22  +Sarthi
- pushing to 

Getting Graph into Pipe:


### 2024-01-12  +GRZ
==> ++ Get gt for OCR training
==> Gallery metrics

==> Better Gallery
-- Refactor shot id
- hands up integration

==> Ball Possession Ideas

W: 
Roster results: perfect results would boost by 5%
new dataset: wants to see how to best use it.
R:
Understanding OCR pj

### 2024-01-15  +Maxim
==> Get CVAT data into insts
==> Getting color from LLMs into the pipe
==> GPT based-text game summary
- Testing:  


trying to get LLM apis working
W:
50-frame w different team colors: validating colors




### 2024-01-12  Martin
- Hall Shots
- Shot detector is turned off for 30 frames
- Checking not-a-shot, hall-shot,








### 2024-01-12  Sarthi
- Volleyball games

=> Get v10

### 2024-01-12  Juan
- Finishing Homography
- Doing Inst


### 2024-01-16  BAD GAMES
- #6  Bad Lighting
- #2  White numbers on yellow Jersey
- #7  Trim
- #9  Extra short video

### 2023-12-14 Points [[@Maxim Dorogov]]
- MaximEquations, RBP_Shooting_scorer (which needs update for dynamic view labels)

- RBP_Shooting_scorer
	- <-- SC_courtlines - send messages from Cam Calibration
		- <-- CameraCalibration class - 

### 2023-12-01  ideas from james
- multiple hoops tips
- using ball arc estimation to reject non-shots


### 2023-12-06  


### 2023-08-21  Grzegorz
==> Fixing OCR model
==> Speedups
==> LCM inputs


- Memory Profiling - 
- EVENTS - 
- Baseball - Kinda works
- Remove Non-Players, by adding generic group 

### 2023-08-21  Sarthi
==> New Color Alg
--> Halos.
--> FrameByFrame, ShotEval, 

- Improving halos - adding GRZ code
- Improving Color model

### 2023-08-21  Martin
==> Building new model w/o GPL. 
  (Especially for active hoops)
==> Perf detects, hoop

- Analyze metrics

really close to 60% player ID

- Ball possession.
- Player 1 and 11.  <--  
- Distraction players.  (homography)

### 2023-07-31  Maxim
==> Baseball POC

--> dataset speed
--> how do position / points when player jumps <--
==> Camera Calibration (metrics and closed loop)
==> Adding 2/3 points
??  Ball detector

- Ground Truth - Columbia team 
- Update Mike's camera/court parameterization 
- Build GENERATIVE MODEL to model court.

### 2023-10-16  ========

- Vanishing points are 

- WROTE doc shared w/ Mike about 


- CAMERA Calibration - using lines on the court.
	- USED for 3d points into 2d


### 2023-07-31  Sarthi
COLOR MODEL UPDATE - 
LEARN TO DEPLOY TO AWS - 
HALOS ??

- NOT A SHOT - Retrain using multi-class class.  (collected 120)

### 2023-07-31  Martin
EVENTS DATASET - 
- 10 games - 
- Meet Leo - bug

DEMO -
Greg PR is merged.

Getting a JSON

### 2023-07-31  Vishal
CIRCLE CI - Friday
END2END TESTING OF HALO - 


SPEEDUP DEBUGGING - 

### 2023-08-02  Grzegorz
EVENTS:  First round trip with James

COMMON CLASSES


- Built 4 speedups
- Running end to end system

### 2023-08-03  DEV OPs
- Terraform
- Ansible
- CV-image:  conda+main

### 2023-07-31  Color ALG

So @Grzegorz Biziel if you think just throwing some additional training at your current color model is going to mostly make it work, then I am in favor of just doing that, and not trying some totally new approach.  Still I must say, it seems we are going needlessly going the long away around the barn when we could be just walk thru the door.  If you think this approach is not going to go the distance and instead is going to hit a wall below 95% accurate coverage.  Then we should at least talk about a totally different path.  (beginning from the colors observed in the video itself.)

Not saying we go and do this.  It would take a sprint to code it, we I would have to think about how to do it efficiently.
But here is a thought about a totally different path:

**Compute Color PDF** -- Maps the torso region of each detect onto a PDF over 20-key colors (plus the none-of-the-above color).
(1) sample a million pixels randomly from the torso regions of detects from the frames from 5 min of game play
(2) run k-means (K = 20) in RGB space on those pixels to get top 20 most dominant colors.
(3) This defines a histogram-like PDF for each detect frame.  A 20-tuple of the number of pixels in the detect which are close in RGB-space to each of our 20 dominant colors, or an 'other' category for pixels not close to any of the 20 colors.  
(4) This defines a function mapping each detect onto a 20-tuple pdf over all pixels in the torso of that detect -- what percent are near each of the key colors.

What is the idea here?
We are looking for most common colors in torso area.  My thinking is the one to three jersey colors for each team (plus the floor, hair, skin colors, referee colors, etc) will all be among the top 20 colors.



**Compute Color Z-scores** -- Extends PDF function to map each detect onto 20 Z-scores representing surprise for that color
- Each element of the color PDF tuple naturally has a gaussian distribution for the levels of that color over ALL detects over both teams and coaches.  
(1) From this we can compute a Z-score for the level of "surprise" in seeing an abnormally high amount of this key color relative to all detect frames.
(2) We pick the simple threshold on this Z-score for how much surprise is needed to be "surprised"
(3) Using this fixed threshold we generate a 20-bit bit-vector of the surprise at the amount of each color seen.


What is the idea here?
The Z-score represents the amount of "surprise" in seeing a give level of color in a single frame.  Skin tone might exist in many frames, so seeing it will not be too surprising.  Perhaps the floor is rarely seen so in the torso, thus seeing alot of floor from a misplaced torso might generate surprise is seeing a lot of it.  We choose the fixed threshold so that most of the time the number of pixels of actual jersey color is above the threshold while things like skin color are not.  This should not be a sensitive parameter since jersey color in large amounts should basically NEVER occur for other players, and should be notably large

the amount of jersey color even with very little jersey is seen has a 

to just below the level of "surprised" we expect for a team color that will occur in more than 1/10 of all detects (gives us room for non-player detects, for the other team, and our own team that had little jersey showing).  

Either way this is detecting notable deviations from expectations.


Most Conserved Surprise -- 

at the amount of this color in this detect as compared to all detects.
(5) Now using the same gallery code we select track that are not close to other tracks an look for max surprise counts for each key color over all tracklets.  The highest scoring key colors are the jersey(s) colors.
(6) choose the top 2-4 scoring colors

  Still seeing the floor will be random, it will not be conserved as a surprising color over the tracklet's length.  But the jersey color will rise above a properly chosen threshold (for surprise among two teams with a few extras) AND jersey color when it is high will be CONSISTENTLY high over that tracklet.  Indeed it should only be those colors that are unusual and yet are conserved across a track that would be identified.
- 
- This does not give us color names



The highest max ranking colors are those colors

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

