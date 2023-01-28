I THINK THESE ARE NOTES FROM DISCUSSIONS PRIOR TO JOINING BIZZY


-- doesn't work -- little control -- driving towards ML -- ability to travel -- daily details -- 


10M    Joe
10M    Ra
05M    Dan O
2.75M  Paul
2.5    Linda
2.5    Alex
---
32.5



# Ref

## TODO
-- Produce Early Rungs

## Rungs

--  Arming -- Cheap, Fast, Arm Outcomes




--  Location
    1.  Static Mapped Env Location
    2a. Location with immobile distractors
    2b. Location with indistinct objects
    2c. Location with invisible blocking objects

--  Locomotion
    1.  Motion w. known location


      

## Parts



-- big box retailer -- Tom Wagner
   segment -- sharper image 



=== SWIM LANES ===

--- Infrastructure
--- Hardware:         Arm, Gripper, Locomotion/Sensing, Body
--- SLAM:
--- Local Mapping:
--- Objectifier:      (categorize, id, parameterized shapes)
--- Complaint Acts:   
    -- grasp

--- Simulations:      Gripper / Arm / Locomotion / Environment / (emulation of embedded w. stubbed APIs)

--- Planning
    -- grasp
    -- arm motion
    -- task tracking
    -- 


--- Micro behaviors
    -- Lift/drop toilet lid
    -- 

--- Behaviors
    -- wipe surface
    -- 


=== LEVEL 1 == INFRASTRUCTURE ===

--- OPS                --  
    --- GOLD ROOM      --  Continuous, Any gold-standard rung, w. any HW build, w. any SW build.
    --- QA???          --
    --- BUILD 
    --- MAINT          --  keeping all our hardware working


--- INFRASTRUCTURE --  
    --- DEPLOY     --  1-click 100% reconfiguring deploy, from single JSON spec.
    --- TESTING    --  Continuous Integration Testing, Unit tests,  
    --- DEV SEAT   --  Turn-key: dev, debugging, testing, VM boxes etc.



=== LEVEL 2 == PLATFORM ===

--- HARDWARE  (design, mfg-mgr, eletrical, machanical)
    --- SYSTEM         --  Power, computation, 
    --- SHELL
    --- ARM / gripper
    --- BALANCE OF SYS --  Locomotion, sensor suite, actuators
  

--- PERCEPTION
    --- GLOBAL MAPPING --  Build 2d static map, with dynamic obstacle modelling (e.g. people, pet)
    --- LOCAL MAPPING  --  Building fine grained gripper centric map 
    --- OBJECTIFY      --  Seeing and parametrically characterizing small distinct objects
    --- IDENTIFY       --  Identify specific objects(e.g. dad's gym shoe)
    --- CATEGORIZE     --  (e.g. bottle), 


--- VERBAL
    --- SPEAKING
    --- N-WORD 
    --- DICTATION



--- P&A                --  PERCEPTION & ACTION
    --- SLAM           --  Simultaneous localization and mapping
    --- Navigation     --  Path Planning
    --- "PID loops"    --  Low level implementation of many compliant action executions



=== LEVEL 3 == APIs ===

--- COMPLIANT ARM API  --  Classes of arm actions w. compliant conditions.  
    --- PRESS          --  Move gripper thru trajectory while maintaining some pressure vector
    --- GLIDE          --  Move gripper thru trajectory while mantaining directly measured distance
    --- HOVER          --  Move gripper thru trajectory while mantaining model-inferred distance


--- GRIPPER LANG API   --  Constraint based language for programming gripper action
    --- ORIENTATION    --  Maintain gripper orientation
    --- PRESSURE

--- COMBO API
    --- DISTANCE
    --- VELOCITY/ACC   --  Min/Max/Constant 



--- HOMEOSTATICS       --  
    --- ANTI-TIP       --  Center of mass projection planning
    --- ANTI-TUMBLE    --  Terrain testing
    --- LEGAL-SPACE    --  Avoid no-go zones
    --- ANTI-INTERACT  --  Avoid hitting other agents, or being in place where it will be hit
                           (dont 
    --- ENVELOPE MON   --  Operate within:  temp, current, battery
    --- ANTI-EFFECTS   --  Scan for and limit unexpected action consequenses
    --- ANTI-CONFUSION --  Maintain acceptable perception expectation levels

    ===> plus "reset" strategies for each of the failures.


--- LEARNING           --  PARAMETRIC TRAINING


=== LEVEL 4 ==  PARAMETRIC ACTIONS ===
    --- DUSTING WIPE   -- 
    --- SCRUB WIPE     --
    --- SCOOP WIPE
    --- STABLE PUSH

    --- ARM POS    --  
    --- GRASPING   -- 
        - rigid Centered balanced rigid
        - flex  
    --- STABALIZING GRASP



=== LEVEL 5 == APPLICATIONS ===

--- Clean Toilet
    --- Perception
        --- Toilet Type Determination.       (round bowl, enlongated bowl, continuous lid, jacked lid, ...)
        --- Toilet base type determination.  (smooth convex, exposed piping, exposed mounts, ridged lip)
        --- 
    --- Toilet Realative Positioning
        --- Front Centerline Relative
        --- Left/Right corner relative
        --- Scrubber
    --- Lid Control
        --- Lid state sensing (covered/uncovered/colored/uncolored.  Water colored/not.)
        --- Scraping Lid Lift
        --- Pulling Lid Drop

    --- Scrubber control
        --- Position Base For pickup.  Pickup.  PutDown.
	--- Parametric Location Indentification.  Brush Status

    --- Wiping Patches
        --- Rim Wiping Flat:    Quadrant I, II, III, IV
        --- Rim Wiping Inner:   Quadrant I, II, III, IV
        --- Seat Wipe (circular, oval, broken)  (flat-portion, outter Quad I, II, III, IV, Broken Tip left/right)

        --- Vertical Resivorior Wipe:  Left, Center, Right
        --- Resivorior Corner Wipes:   Left, Right
        --- Oval Resivoirior Wipes:    left pull/push, center left/right, right pull/push
    --- Extension-based Wipes
        --- Bowl radial vertical scrubbing
        --- Bowl center brush placement
        --- Bowl radial upstroke swipe
    
    




=== LEVEL 6 == CONTROL & COORDINATION ===  

--- AGENCY         --  Managing global status, states, planning, prioritization, and control
    --- PERSONALITY    


--- EXEC           --  Coordinated action execution





# Log

## Breakdown of tasks


+  5  Lab Techs, Gold Room


+  5-?   HARDWARE
         ?  Chassy & Structure
+        END EFFECTORS: 1-2 engineers per end effector. 
         (They are trying out many different hardware designs to understand required pressures and movement tolerances.
          Initially they are testing, w/o any arm, then using off shelf arm, then iterating on our evolving arm.)

         -- Flat surface wiper (electro-adhesion, or other)
         -- Rotating brushes
         -- Dusting "feather" approach
         -- Bowl Scrubber
         -- Floor Mop

+  1-2   ARM DESIGNS:  (Haptic control may work much better with non-rigid components, since it relies 
                        so much less on precise location, and so much more on a smoothly varying force feedback)

   ?  Industrial designer


+  3  Electronics/System  -- processor, motor drivers, sensors, etc.

   3  Locomotion Team: 
      2-3  Slam, Path Planning, Movement Execution and monitoring.  
+     1-2  Task specific base placement procedures


+  3  Vision Team:  
      2   2D world mapped Objects.  Arm Centric Map.  
      1   Object Recognition & Registration


+  4  Arm Team:
      1 - data integration, execution
      3 - The BIZZY haptic control DSL

   ?  WHOLE TASK TEAMS
      2 - clean toilet
      2 - counter top wiping
      2 - fixture cleaning
      2 - floor moping (plus contour cleaning)     


   3  Agency
      Top level control, prioritization, personality, user interaction and control/configuration
   

+  3  Device Software Stack
      OS, Build Tools, Testing Tools, Libraries, Dev environment, deployment scripts


   3  Cloud Stack  (front end (full stack) dev.  designer.  backend dev.) 
         Cloud Collected Data
         Website.  User Accounts, Purchasing, Refills, Configuration
         (Additional Devs if we are going to control the robot using mobile devices)
      

+  2  TESTING
      Device simulations.  CI testing.  Unit tests,  End-to-end simulation test, 
      Physical tests:  "unit" or isolated compoent test.  integration, or end to end system tests.
      Variational tests (designed to capture and test full spectrum of customer usage conditions)





MODLULE TESTS

-- INFRASTRUCTURE -- Functional 'Gold-Standard Execution Room'.  From
formal testing spec file, configuration of 100% of all
software/firmware, hardware, and test context are configured.  Then
manual and automated testing is performed based on specification, and
results available for dev.

-- GRIPPER -- Full range of targeted cleaning and manipulation tasks
are demonstrated by tele-operating the gripper.  (we could specify a
list of targeted properties including weight and other properties, or
we could just state that the targeted cleaning and manipulation tasks
are sufficient to drive purchasing at the $1000 to $2000 price point.)

-- TWO LEVEL PERCEPTION -- Build 3D arm-centric world model of space
around gripper, sufficient for determining freespace requirements
needed for arm movements.  Build Object-level representation of
objects within arm area, sufficient for object manipulation.  Use
arm-centric world model, object-level representation, to plan and
begin executing object manipulation action.  Manipulation action is
completed using visual servoing primarily based on gripper level
object sensing coupled with proprioceptic sensing, and arm-centric sensing.

-- TOUCH PERCEPTION -- 

-- SINGLE OBJECT GRASPING --

-- 


 controlled by proprioception  and gripper centeric sensing to execute 
object manipulation


## My Stance

-- prototype is most important.
-- as long as the physics
   all engineer
   -- use you own arm is important
   -- use formula to prove.
   touch under. 

-- second criteria -- second most important
   -- idea is already endorced by the marketplace is a very powerful milestone.

-- third Go-to-market.  how much differentiation 


he see nothgin but an important idea.   no results.

-- what kind of demonstration would have reduced the risk for him 



Joe,
I am happy to meet Linda at a convenient opportunity.  I will be in SF on Friday,
are you guys also going to be in SF, if so that would be a great time to meet.

I have given hours of thought to the Bizzy effort, and my next steps.
I would love to have a one on one, call or meeting to talk about this too.

Is there a good time tomorrow for this?


## Stance II

here are some notes I made during this thinking session.
The first section is my reactions to the summary of the Lidon response to us.


======  Point by point response from me:

1. Robotics is definitely a must go business. We will be preparing
   for the manufacturing of this kind of business.
   I AGREE.

2. Bizzy robotics’ idea is very good for its key differentiation:
   software driven and for un-routine jobs.
   I AGREE (and would add notion of using touch to overcoming the  the good-fast-powerful-cheap  bottleneck) 

3. Bizzy Robotics is in the seed fund stage. 
   I AGREE

   Although founder is very experience in Neato
   I AGREE

   Bizzy is very DIFFERENT from Neato, in terms of technology, safety as well as market development. 
    -- technology:           I AGREE
    -- safety:               I AGREE
    -- market development:   I DONT AGREE, OR ONLY PARTIALLY AGREE.  (Scuba proves the market, but at a different price point) 


   Bizzy is very DIFFICULT compared to Neato, in terms of technology, and safety
   I AGREE.


   It might take years to reach mass production with volumes after taking away all risk factors.
   I AGREE.


4. ... incubate business with high uncertainty (of time to volume)
   I AGREE.



========

Dan's Primary Concerns moving forward:

-- Efficient Team Composition -- each founder ideally is central to the founding itself. 
   People that can help later should be hired later.

-- Efficient use of personal time -- At Analytics Fire my time is multipled by 5 or so, 
   as I am directing multiple devs, thus multiplying my value per day.  Situating self for effective
   sustained output is important.  

-- Effective execution -- in a winner take all startup world, one must be executing at the top of 
   ones abilities to rise above the others.  This means it is important to dedicate ones effort in
   very focused and sustained ways in order to get to a place of relative effectiveness.
   Quick hacks are not sufficient, my efforts must be a solid stream of quick hacks in some focused direction.
   So this means I need to either go big or go home on efforts like Bizzy.

-- I am concerned that we have realistic estimates of:
   -- of Task difficulty
   -- of various Idea's innovativeness. 
   (Both must be assessed by 'experts' in the most relevant field & also by empirical evidence 
    E.g. impressing HW guys w. software, or software guys w. HW does *NOT* count.)


   But I feel like our backgrounds collectively put us in rarefied company in
   trying to secure funding.  This is shown by the level of the talks we are having.
   And I am a fan of some of the ideas that we are trying to build upon:  (touch control,
   gripper design) are very innovative, and very sophisticated "top shelf" kinds of ideas.




======

MY CONCLUSIONS:

-- I am not in a "giving up" mood, but I am going to follow Nick's wisdom in this area, and very
   consciously curtail time/effort that I am putting in here w/o acceptance signal.  This is a go-big or go-home
   situation.  There is no issue waiting for conditions to be right, and then acting, BUT
   there is a real cost to having long discussions and dreams about what might be, in the 
   absence of concrete action and concrete outcomes.


-- Further action should address the issues above:  time efficiency, team alignement, recognition of 
   task difficulty, and current state of our effort (as a seed effort).  still all of that is possible.


-- For example in a company reset kind of scenerio, I could imagine being at the center of a $1 million 
   'fast ramp up' prototyping aimed at removing the HW/SW "whole task" risk surrounding the:  
   sensing-computing-acting loops, needed for the arm-effector assembly.  It could demonstrate the full range of:
   - flexibility needed to deal with environment diversity
   - abilities needed for trageted tasks.
   - performance characteristics (speed, power, reach, etc.)

-- I could also imagine being in the middle of those investor meetings needed secure such seed funding.

## next x


   My assessment in this area is really mixed:

   On the one hand I feel we display a amature's level of sophistication in some areas of our execution, and 
   world class excellence in others:

   --  Cavilearly assuming we can perform better than current state of art in multiple areas (like vision),
       without really even understanding current results and limitations those area.

   --  Treating our arm-chair pontification and intuitions about existing endevor areas as ground truth, and
       treating the deep expertise and decades of effort by smart folks in those areas as almost irrelevant is a
       sign of amature execution.

   --  I am a believer that an outsider to an area, CAN and often DOES bring transformative ideas to an area.
       But that does not allow me to ignore expertise of the area.  it is all too easy to just assume
       'I am smart and they are all just dumb'   In those cases, where I suspect this maybe true,
       I should be able to convince an existing expert that my idea is plaussible, or at least possible,
       and that it has not be actively DISPROVEN by past work.  Ingoring those assessments is more hacker/hobbiest 
       like and less professional.  (The bizzy team may well have more direct expert assessements that I am not
       aware of on this point.)

   --  We seem all too eger to take on risks/complexity beyond what is strictly necessary for our first 
       rev in getting to market to solve our first task.  This bravado stems from our own smarts, but
       it reduces our chances of success.

   -- Our demos (even some of our proposed demos) don't *actually* prove anything... not to a sophisticated
      audience, and we should not be spending our efforts towards demos that are aimed at unsophisticated
      audiences hoping that we can get $$$ from them, thru their ignorance.  
      (Any demo we build should ACTUALLY be strongly proving something critical to a sophisticated 
       audience/decision maker.)

   -- There is a reason that most startups are founded by those in their late 20s and early 30s.
      Hunger.  They are in a life position to throw huge hours into a high-risk thing.  They
      don't have family situations that require their attentions, they are not used to the cushioning
      that success brings, so they are prepped for a hard life.  We need to be like those 20s & 30s types
      if we want to play their game.  Dreams that our wisdom and soften those blows....  are just that...
      dreams...



## Decline Email

Dear Joe/Gaurav,
 
I finally got in touch with Mr. Wang and also discussed with directors from Liteon. Below is the conclusion for this deal.
 
1.     Robotics is definitely a must go business. We will be preparing for the manufacturing of this kind of business.
2.     Bizzy robotics’ idea is very good for its key differentiation: software driven and for un-routine jobs.
3.     Bizzy Robotics is in the seed fund stage. Although founder is very experience in Neato, Bizzy is very different and difficult from Neato, in terms of technology, safety as well as market development. It might take years to reach mass production with volumes after taking away all risk factors.
4.     Liteon as a manufacturer, as opposed to the Venture Capital, should only invest into companies that leads to immediately businesses. It is inappropriate to invest company funds to incubate business with high uncertainty (of time to volume)
 
The conclusion is very frustrated for me. However, as one of the investment committee members, I need to accept group decision. I am very sad that I couldn’t convince the rest of people to buy in my idea. Although I am still very wishful we could cooperate in the future, I may need to call for a stop at this stage.
 
Let’s talk more detail this afternoon.
 
Best regards,
Alex


## Nicks Notes


the questions I would want answers to going into it are:
is this R&D or skipping straight to building a product that is to be sold?
if it's R&D
what is the roadmap? what is the goal & how do we measure success?
is $3m enough for even 1 iteration of R&D in this space?
where is the next round of $$ coming from to take it to market?
what is the timeline, & will we miss the window?
if it's skipping straight to building a product
exactly what problem do they envision the robot will solve? & is there evidence that
it is really useful to people in practice?
that people would pay for it?
that robots can do it?
that this team can build the robot to do it?
is $3m enough to build a sellable product & take it to market?
who on their team is in charge of strategy? & what is their take on the following:
do the economics actually make sense?
how big is the market, really?
what is the sales model?
cost of robot + customer acquisition cost < selling price?
is it defensible?
if they are going to outsource the manufacturing, & the are coming to us about the SW, are these really the guys to win this?
who are the competitors & how are the positioned? what is their strategy?

my main concern is that this will all be unravelled within 2 years because of one of the following reasons:
not enough $$ to survive design iterations / pivots
not enough $$ to take to market
turns out no one wants to buy the robot because
it is too expensive, or
it does not work right, or
there are other, more robust solutions available
the vision was off
