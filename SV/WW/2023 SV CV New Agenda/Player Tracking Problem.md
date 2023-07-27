
-[[Agenda]] 
Out of Date



Questions For The Team
- Can we conver



# The Player Tracking Problem 

Here we consider a framing, a possible breakdown, and a family of approaches to the player tracking problem.

**Player Tracking Problem (PTP)** - The problem of capturing the movements of each player over the course of a game as a sequence of bounding boxes covering the player and correctly identifying the team color and jersey number for each in as many cases as possible with special emphasis on those frames where the player is taking critical actions that will affect the stats reported for the game.


## Player Tracking Framework

The ***Player Tracking Framework (PTF)*** aims to be a least commitment framing of the player tracking problem that allows several sub-aspects of the PTP to be quasi-independently evaluated and optimized.  The PTF defines a very high level architecture for solving the PTP along with metrics for each of the key components within that architecture.  Our idea is to measure and optimize each of these components separately as well as measure the money metric for mix and match versions of the system as a whole. To accomplish this the PTF formally defines a set of components, metrics, and APIs facilitating these goals. The idea is for the PTF to be a 'live' document that is updated to reflect our evolving understanding of the PTP and how to best solve it.



### Components

- **DETECTOR** - the detector accepts an image and returns a set of detections found within the image.
  When used repeatedly over a sequence of images the detector constructs the game graph.
- **TRACKER** - the tracker condenses the game graph into a condensed game graph.
- **GLOBAL INFERENCER** - the inferencer performs belief propagation more global condensed graph structures to update the graph to better satisfy sports- and physics-based constraints that we know to be respected by the underlying data.
- **TWEAKER** - a tweaker updates the nodes of the condensed graph according to the heuristic underlying this particular tweaker.  Three common types of tweakers include:
	1. PLAYER IDENTIFICATION - Player identification can be built into the initial detector, or it can be opportunistically applied as a tweaker in cases as indicated.
	2. GLOBAL INFERENCE - Global inference can be applied as a single pass optimization step, or it can be applied opportunistically as a tweaker.
	3. FINE GRAINED ANALYZER - 
- **STATSER** - a statser derives annotations, stats, and single-game metrics from the CGI. 
  ...
- **ORCHISTRATOR** - Each instantiation of the framework is build upon an orchestration layer.  Broadly all orchestration implementations performs detection, tracking, tweaking & global inference, then finally statsing.  But the devil is often in the details, so we assume multiple custom orchestration layers are developed each interleaving these operations 

- **PLAYER IDENTIFICATION** - Accepts an image and returns a pdf over playe



## APIs

**class** Node
**def** `detect(source:Image) -> Set[Node]` 

### Game Graph / Condensed Game Graph
(See the game graph doc for these details)


### Detector API

# METRICS 

- Money Metric - Measures "exact" match against gold standard annotations 
- Label Accuracy - Accuracy of correctly labelling a given detection. (also label precision, recall, and F1)
-  
- Delta Money Metric - 
- "Delta" metrics - Tweakers can be measured by comparing the value of many different metrics with and without the application of the indicated tweak. (as a single tweak or as a delta between no usage of the tweak and "standard" usage of the tweak in all appropriate contexts)

## Assessing Detectors
## Assessing Player Identification 
## Assessing Tweakers

# --- OLDER ---

### Importantly Separable Sub-problems 

Within the literature there are several sub-problems which are generic enough that families of mature algorithms exist for attacking them.  Obviously we want to build from these basins of excellence, and then define and tackle the parts "between" these pieces using Sports Visio proprietary methods.  Here are several well established "Parts" for us to build from:

#### Tracking
...

#### Image Detection / Person Detection
Given a single image and optionally a set of models constructed from expected object detections return  a set of "detections" where each detection has a bounding box, along with other optional meta data, like the object's mask, an object identifier, and other properties. 

Person detection is a special case of image detection where detections are limited to images that might correspond to a person.


#### ReID
Image 


#### Skeletal Tracking - 



#### Belief Propagation - 



#### Action Recognition - 



#### Hierarchical Image Detection 





### One Slicing of the Pie

The existence of these to established areas suggested the following decomposition of PTP into the following partitioning into four interdependent sub-problems.  The devil is in the details, so we cannot know that this decomposition is correct, but it does seem to map nicely onto the components the field is providing to us so at the least is is worthy of consideration.

**Image Identification** - Given a single image identify which of a pre-enumerated set of objects generated that image.  (when use as part of a tracker this is called ReId but here we are separating and optimizing this classifier in isolation)

**Tracker Based Processing** - Much of all of the work done to date by the SV team falls into this category.  It is classical tracking -- the is pulling information which at times exists outside the ST-MEDIUM grain size into determinations about the progression of bounding boxes over successive frames for given objects in the real world.  The output of this tracker based processing is a set of tracklets consisting of a sequence of bounding boxes along with meta data about each frame and about the tracklet itself.  We allow for the possibility that these trackets only extend for tens or hundreds of frame and in cases fail to include crucial information (like player identity).

**Global Inference** - This sub problem deals with the propagation (mostly of meta data information) at the ST-LARGE grain size between assertions generated at by Tracking 

**Hyper Fine Inference** - This is a part of the problem we have yet to address in great detail.  It attempts to draw conclusions at the ST-SMALL level to distinguish stats outputs that remain ambiguous.



four somewhat independently addressable sub-problems.  The fact that these sub-problems map somewhat nicely into these prior problme sp






Space time itself is


==> Identifying and tracking players in sports video

## Decomposing problem into somewhat orthogonal sub-problems
- Full solution will leverage knowlege from each to drive the others.
- Still seems we can investigate and optimize solutions for each independently.

### (1) SINGLE FRAME INFO - Re-Indentification
### (1) Tracking
- Given a "degree of match" function derived from 


# Slide Two




# Notes

- PROBLEM'S IMPORTANCE
Tracking identified players is a (the?) fundamental capability underlying AI-based sport stats.
It does not come "out of the box" from any avaialble tools as appears to be blocking us and others from presently delivering good stats.
Thus a tour-de-force approach is indicated here




## Problem Framing and Decomposition

**Space Time Granularity** - This is a fancy sounding term for the idea that separately looking at constraints and information that is occurring in smaller or larger windows of time, and smaller and larger windows of space.  In this document we define three levels of space-time granularity and use these three levels to talk about different ways we might slice up the larger PTP problem into independently attackable sub-problems.  But of course there could be many ways to define these:

**ST-SMALL** - The "Space Time Small" grain size deals with action happening spatially at a size notably smaller than a full player's bounding box.  and temporally over just a couple of frames.  So right now hoop detect is paying attention to features at the ST-SMALL grain size.  Later I think as we get more sophisticated in resolving player action we will be building skeletal and other models of each player's hands as a way of fully discriminating tough cases where the players are in a scrum and a shot is made.  All of this will be at the ST-SMALL grain size.

**ST-MEDIUM** - The "Space Time Medium" grain size deals with action happen spatially and temporally near a single bounding box.  So our trackers are currently mostly operating at the ST-MEDIUM level.  Computing the velocity of a player, or de-conflicting two crossing tracks is currently being addressed using ST-MEDIUM constraints.

**ST-LARGE** - The "Space Time Large" grain size deal temporally with the entirely of the game, and spatially with the entire court.  So the constraint that each team has 5 players on the court at a given time is an ST-LARGE constraint as it can only be expressed by looking at the entire court.  And player minutes is an ST-LARGE computation as it deals temporally with the whole game.  But any constraint or processing that is temporally larger than a couple seconds or spatially larger than one bounding box away from a player is already ST-LARGE, it need not cover the whole game or whole court.

