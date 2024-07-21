

- [[2023-07 ML CV ops Job Post.docx]] - Job Ad
- - [[2023-07 ML-CV Ops Job Post|CV]]: [mike](https://docs.google.com/spreadsheets/d/1WTGhzbMGHH9jDb4ls5lsdY00TlNfmJ3p/edit#gid=612073226),  [old-sheet](https://docs.google.com/spreadsheets/d/1BGS75wy9EIPLjwY2IrcUFKbbAF_t9TiecYdTLi0XOHo/edit#gid=0),

## BBI


BBI PROMPTS
- Complex or Challenging Examples:
	- ML TRAINING:
	- CV ALG:
	- NOVEL ALG: kind of coder?
	- ML OPS:
	- INTERPERSONAL:  Gain consensus, Politics
CONTEXT
- Why Leave?  Looking For?  Plan over next years?



## Job Ad

Full-time Deep Learning / Computer Vision / ML Ops Engineer  
Respond to Michael@SportsVisio.com

Who we are:

SportsVisio is a new venture going where no one has gone before: making artificial intelligence (AI) apps for amateur sports players, their parents, and coaches in order to improve their performance and have more fun. Although we’re already underway, you’ll still have all the privileges and opportunities of being an early member of an entrepreneurial development team. We accommodate talent wherever it’s based and employ an extensive suite of tools to facilitate remote teamwork. (We cannot help with VISA applications.)

What you’ll do:

·      Amaze your family and friends with never-seen-before AI demonstrations that you helped create.

·      Design, develop, prototype, and evaluate new deep learning-based computer vision algorithms.

·      Collaborate with our other staff in cloud processing, app development, and software engineering.

·      Convert your prototype code into production code.

·      Jump into whatever is needed to get the job done: we’re all pulling together!

What you’ll need:

·      BS degree in Computer Science, Data Science, Computer Engineering, Artificial Intelligence

·      Hands-on experience in training and using deep learning techniques, in particular MLPs and CNNs

·      Proficiency in Python, NumPy, and a framework such as Pytorch, Tensorflow, Caffe, etc.

·      Knowledge of Agile development, testing, and documentation processes

·      Adeptness applying with OpenCV to image processing

·      Familiarity with tools for software configuration, task tracking, and data management

·      Strong ability to work collaboratively, and remotely in verbal and written English

Ideally you also have

·      Experience with ML ops tooling:  Pipelines, Regression tools, Experimentation platforms, Data versioning tools etc.

Even better:

·      MS or PhD in a scientific field such as Computer Science, Statistics, Mathematics, or Physics

·      Hands-on experience on developing perception algorithms, including deep learning approaches for detection, classification, identification, segmentation, tracking, and characterization

·      Track record of developing, implementing, and evaluating learning-based detection and analytic algorithms against real-world data-sets

·      Significant experience developing and deploying real-time video processing algorithms

# Interviews

### 2023-10-04  Juan


BBI PROMPTS
- Complex or Challenging Examples:
	- ML TRAINING:
	- CV ALG:
	- NOVEL ALG: kind of coder?
	- ML OPS:
	- INTERPERSONAL:  Gain consensus, Politics
CONTEXT
- Why Leave?  Looking For?  Plan over next years?


=LG-SYS=

ANSER - 
- Image processing to assess the area of a wall.
- perspective corrected 
- detect the walls
- not used commercial.
- hard part:
	- how to filter out noise from lighting. (inconsistent & dark)
- Trained model with many samples.
- Segmentation network.  MASK-R CNN
	- assigning dense pixel assignment.
	- uses semantic info about context:  color, patterns-related to other pattersn in higher in network
	- he explained RCNN well 
- Labelling: manual. 
	- speedup: fancy manual edit of polygons.



=ML-TRAINING=

TECH MONTERRAY - project
- TRAINING LOOP FOR facebook for transformer (early in transformers 2021)
- Object deter model from Facebook
- Wrote core alg in PyTorch - based on pseudo code in paper.
- validated it by doing key point detection on faces
- Used model structure based on paper (plus weights)


TERNIUM - (collaboration while at univ)
- used GOOGLE's Big Transfer (model and training frmaework)
- Used different loss function. (used a focal loss from the) 
- it only worked for some epochs then training became unstable.


=CV-ALGS=
- majoiryt is off shelf

ANSER - find vanishing points
- Used ransack to find vanishing point on the walls to only keep the lines that work well
- Before that he tried to
- post process / add noise, remove noise, tried to supress lines from textured regions 
	- filtering, found open contours then and find area to decide if it was a real wall edge.
	- also used convex hull


=NOVEL-ALG=

ANSER - ReID cars - very hard
- Fuzzy system - combine:
	- time between detections
	- distance between dectections
	- vehicle class / color (top N confidences if car has multiple colors)
- easier to debug fuzzy
- timing of video offsets are noisy.

ANSER - teamates
- 3 CV - rafael and santeago 
- Rafael - background in research - very creative - buggy code - very fast at coding - does not ask for help
	- super talkative one.  talking to a fault - what he thinks he says
- Sante - slower, more by the book - lot of time to do it.  needs to be told
	- the quiet one, work on something that you missed.
- HE - likes to understand things - like to solve by self - attntion to detail - 
	- iteration speed is pretty good.  
	- I don't talk alot.  more careful socially.  perceived quiet person/introvert.
	- most ideas that he proposes are high level.  Rafaels ideas are more tactical.


=LEADERSHIP= 
- Current leading team of 5 CV and 10 devs



=CONTEXT=
- Why leave?  I am pretty happy with Algotive
- not seeing way to grow.
- ANSER lost funding. 












### 2023-09-29  Mariano

- LG:  ALG, Training Models,
	- Download/Experiment/Tweak
BBI PROMPTS
- Complex or Challenging Examples:
	- ML TRAINING:
	- NOVEL ALG: kind of coder?
	- CV ALG:
	- ML OPS:
	- INTERPERSONAL:  Gain consensus, Politics
CONTEXT
- Why Leave?  Looking For?  Plan over next years?

== ML TRAINING ==
SCANTECH - 
- 2years automated supermarket
- Use pre-trained re-id model
- who were the people over 2 years
	- 6mo ignacio & Leonardo & him -- little experience
	- 1.5yr enrique PhD - tech lead - 
	- 6mo he became tech lead
		- ignacio & him & 3 more junior devs
	- he ended w/ similar K about training to Enrique
- Tracking of people (object detection models)
	- Yolo V8
	- first dataset 
	- winter coats started 
	- Froze most model and retrained top layers
	- Camera is looking from top down so different COCO.
	- ==> fixed mistakes in prediction to speed training dataset construction
- ==> Technically succeeded, but biz side is not profitable


== NOVEL ALG == 
SCANTECH
- Detect where product is in the image.
- Use realsense camera (RBG & depth)
- goal: find kindle screen in image (can turn on/off)
- variance of the image
- ALG
	- use delta:  on-off-on
	- erosion to clean
	- connected components (on/off same time)
	- clusters 
	- fit a line to find the shelf
	- use 3d location of each shelf and depth data on hand to decide where they grabbed & other alg to decide WHO grabbed.
	- (other alg use pose estimation)

- who did what?
	- Enrique was over all stuff.
	- candidate did a bit more work than others in developing 
	- We brainstorm alot as a team.  (alot with him and enrique in this kind of thinking)
	- Daily meetings could be long, would share alot
	- Enrique & He & Franco - thought in same way so conversation would go quick.  design things quick
		- they understood each other quickly
		- how are they different
			- Enrique - lots K
				- -Lesser implementing
			- Frano 
				- very organized, thinking, and coding
			- He - fast @ understanding others
				- not as good at express self
	- Other collegues "marry their first idea"
		- if it doesn't take too much time, let them fail


== SW ENG ==
- He is more of a research scientist than a SW eng
	- The real good one:  Salvador 
	- he is learning and getting better
	  (now I am the one that documents the most)


== ML OPS ==
- Tried to use DVC - was too heavy
- should have fixed versioning of models


== INTERPERSONAL == 

SCANTECH - R&D org - not harsh deadlines
- Cases where team member was slow (Andres)
	- He was stressed w/ university
	- He was burned out ... working slower
	- talked about the
	- Gave him tasks that were easier
	- He naturally game out of it in a month or mostly
	- Next semester was lighter


== CONEXT == 
- Scantech going in less interesting direction
- Want to keep growing


### 2023-09-11  Matias
- Py, RealTimeOpt, CV, ML

- SCHOOL:		Electronic Eng (5years)
- ENGLISH: 	English is just ok.
- CV/ML:   	Seven Years professional experience, 
- POWER:		He kept loosing power.  (only once or twice a year; has UPS)
- CONTEXT:	Wants to work for American company that is more stable


EXPERIENCE
- Largest 2-3 CV/ML systems & largest two teams lead
BBI PROMPTS
- Complex or Challenging Examples:
	- ML TRAINING:
	- CV ALG:
	- NOVEL ALG: kind of coder?
	- ML OPS:
	- INTERPERSONAL:  Gain consensus, Politics
CONTEXT
- Why Leave?  Looking For?  Plan over next years?

=TOPICS=

KIND OF CODER
- Guy to figure out which algs to use
- Figuring arch of whole system
- Hacker Rank challenges - (he looks expert problems; think several days)
- Competitive coding - 
	- His team ranked 13 across latam.  more than 100 team.  4 person team.  (in 2018)
	- he is the guy that loves the maths problems.  the guy talking.


=LG=SYSTEM=

IROBOT - Maintainer of Roboscope - Internal tool - Debug, Test, 
- 3 years

ARGO LABS
- Safety security system, based on CCT TV 
- He was only dev at
- Detect
	- Detect hair cut
	- Wearing goggles
- Yolo v4 -> v8 - person detection; then crop images
- Head detector Yolo v4 (trained their own)
- Goggles, hair, vest, clothes color (upper/lower), covid masks, detector + 

ARGO LABS 
- Automate inventory tracking in warehouse using drone
- 4 devs, he is first devs.
- sequence of photos of warehouse
- use rack banner to detect position of rack and then the products
- perspective transform based on tag corners
- then use OCR


ARGO LABS
- Pig counter in a pig farm
- Used Yolo for object detect.
	- Deep Sort algorithms - pigs look the same
	- Sort works well just tracks linear bboxes
	- Centroid Trackers (does not predict speed)


CONTEST - 2021 - MarcoLibre challenge (he did not work there)
- FIRST PLACE
- Demand forcasting problem.  predict day by day sales in next month.
- Optomized the exact loss function 
- he optomized the KL-divergence directly - 



### 2023-09-08  Marvin


EXPERIENCE
- Largest 2-3 CV/ML systems & largest two teams lead
BBI PROMPTS
- Complex or Challenging Examples:
	- ML TRAINING:
	- CV ALG:
	- NOVEL ALG: kind of coder?
	- ML OPS:
	- INTERPERSONAL:  Gain consensus, Politics
CONTEXT
- Why Leave?  Looking For?  Plan over next years?


=TOPICS=

- Background:  Mechatronics, robotics, kid hacker

=LARGEST=PROJECTS=


ROBOCUP - project 
- realtime detection of hazards signs
- first tutorial I found - 

ACCENTURE - Client -
{{no real novel ML here}}
- task using computer vision.
- verify product packages complete & no one stole stuff.
- differentiate 2L coke bottle from 1.5L coke bottle
- Used clustering with sales data.
- Speedup
	- Running on Jetson Nano
	- Taking too long - yolo V3 tiny
	- maximize recall


ACCENTURE - internal project - 
- 3 months 
- protoype: use digital twin, AWS
- autonomous driving in toy world - detecting traffic signs & people
- traditional computers vision 
	- open CV for lane following
	- yolo V3 tiny - detecting signs
	- put traffic signs with numbers to triangulate where they were


- no third ML project


ALG DEVELOPMENT
- Little alg development work
- OPEN CV for lane following
	- color space filtering 
	- find shapes using blob detector to
	- calculate the center of the lane and the curvature of the road
- PID controller for on micro controller
- kinematic configurtion of the robots current stats and where the ball was to build a reactive planner.
	- hand built reactive planner (quasi-trig-kinematic-based)


CONTEXT
- wants to get into CV
- beginning his MS degree 




Automated data uploading using AWS 
EXPERIENCE
- Largest 2-3 CV/ML systems & largest two teams lead
BBI PROMPTS
- Complex or Challenging Examples:
	- ML TRAINING:
	- CV ALG:
	- NOVEL ALG: kind of coder?
	- ML OPS:
	- INTERPERSONAL:  Gain consensus, Politics
CONTEXT
- Why Leave?  Looking For?  Plan over next years?




### 2023-09-08  Sebastian

- Mostly non-ML novel alg development
- Not alot of download alg and test
- No DL optimization
- Seemed smart & capable

EXPERIENCE
- Largest 2-3 CV/ML systems & largest two teams lead
BBI PROMPTS
- Complex or Challenging Examples:
	- ML TRAINING:
	- CV ALG:
	- NOVEL ALG: kind of coder?
	- ML OPS:
	- INTERPERSONAL:  Gain consensus, Politics
CONTEXT
- Why Leave?  Looking For?  Plan over next years?

=TOPICS=

PhD in CV - action recognition - Description was very muddled.
- PhD in CV - 
	- Estimate pose 
	- Distance between joint angles between different poses
	- Did PCA on top of this
	- computed distances in 4D set based-distances


KIND OF CODER
- fan of testing
- tidy coder - as compared to non CS
- a bit above the middle - feel 



=LG=PROJECT=

Zubale - "Uber eats" - 
- 3 Data Science guys 
- Group orders "traveling salesman problem"
- Used OR-tools (from google) - not an ML problems

Zubale - 
- ML how long to pick an item
- ML how long to take to deliver an item
	- hand coded features of velocity of the work
	- Gradient boosting.  


2018 - Side Consulting work - CountIt - 
- 6 months - with one other guy 
- Use videos to count vehicles
- Yolo object detection
	- Fine tuning: providing better training data.
- Built their own tracker - needed to run on small computer
	- Hungarian algorithm / like simplex / use object permanence
	- Believe recall would good - 
	- would know how to discard boxes that don't make sense
- Also tried many single object trackers
	- decided that was computations too expensive


ALG TESTING - Grandata - more researchy - try many algs
- Social Graph insights
	- tried multiple algorithms


ML-OPS
- Air-Flow - Zubale - pipeline for features for the ETA models
- Luigi - pipelining tool - (lighter than airflow)
- played with ML flow - did know much - 