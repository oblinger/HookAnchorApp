
# LOG

### 2023-09-04  Hiring Timeline

**2022 IDEA - MAYBE REPLACE BRAIN/SCOTT**

Nov/Dec During hiring Jason said probably need to replace Brian


**FEB IDEA - DEFINITELY REPLACE THEM, BUT AFTER A-ROUND**

Feb 1 When I joined 


**JUNE IDEA - REPLACE SCOTT, THEN AFTER REPLACE BRIAN**

Jun 4 Jason gives Dan the go ahead to hire backend & SW head
- but the agree upon plan was to hire the backend FIRST, then select the Head that leaned towards a PM assuming the backend person was strong.

July 12th - I created a back channel w/ Ignacio to informally begin looking for head of SW.  (BUT not being interviewing 'till we know who the new Backend person will be)

TC initially had difficulty finding experienced folks willing to leave existing positions in order to consider Sports Visio

July 24th - First informal discussion scheduled for following week.

Over the next two weeks I had discussions with about 4 potential candidates.  Each time I told them we have not yet begun interviewing, but we soon will be.


**AUG IDEA - REPLACE BOTH NOW W/ CHEAPER FOLKS**

Somewhere in August I began to feel we could not wait for the Backend Role since it was harder than expected to fill, so told Ignacio we would likely begin interviewing head of SW, but this was still not committed.

I found two more recruiting agencies for Latin Am: one required a retainer, but the other (techunters) did not, so we are now working with them now.


Aug 22 - Sapphire Call
- **Decisions**:  
	- The candidates from US are probably too expensive.
	- Sapphire is not likely to provide any good candidates.
	- Jason said we are now hiring for both roles.
	- Sam said his target for the Head of SW was $150K
	- We all agreed we did not need super senior person,
	  we should try to get cheaper, more "roll up your sleeves" recently promoted guys, rather than VP eng types.

Aug 25 - Had first "cheaper guy" Interview.

Sept 1 - Had call with TecHunters to kick off SW-head search for cheaper guy.





### 2023-06-03  AGENDA -- Whole Company Roadmap

#### **SV ROADMAP FOR FIRST INDUSTRY DOMINATION**

1. **LOVED** - A player/coach loved stats product.   (Persona: Men's league coach; gym owner; ???)
2. **ADOPTED** - A strong fly-wheel with proven ability to drive adoption at scale.   (Persona: ???)
3. **AUTOMATED** - Full-AI providing "adoptable value" at market-disruptive pricing.   (Persona:  AAU-league parent)
4. **DOMINATED** - Dominate cohesive, sticky market network effects.  (Multiple Persona Roles: AAU and/or Men's leagues)

  #2 and #3 are "industry miracles."  Both with timelines for actions, but no timelines for completion


#### **REALITY CHECKS**

#### Global Status
- **TWO** years, **XXX** millions spent.
- **ZERO** MVP systems in regular "loved" usage.

###### Part of the reason why ...
- **FOUR** full time WS devs:    TT, Achilles, Scott, Leo
- **EIGHT** supported systems:  Hoops, Cust-Web-Portal, Android Record, iOS Record, Pivo,  Annotation Portal, WebUI, Air table


#### Top Status
- Raised good money at a strong valuation
- Identified and cultivated great partnership relations
- Built a team with mediocre to solid players

KEY FAILURES
- Failure to focus on a true MVP
- Failed to narrow to a target customer.  (zero customers with regular repeated usage)
- Peanut buttering not pivoting. (we have never left a use-case or customer, instead just added more!)

PATH FORWARD - but too deep in to turn back now...
- Muddle *slowly* forward on: Cerebro, Pivo, ?high-schools?
- Anoint SINGLE "loved" customer target - ensure focus there first
- Make leadership changes for Backend Architect & SW-LEAD+TechPM+VPENG 

##### __



#### SW Team Status
- Poor architectural/design choices - just a moderate cost
- Very poor processes, getting better
- Current gaps & forward-looking consequences:
	- ARCHITECT - hard/slow to develop new features.  High overhead to ensure choices.
	- SW LEAD - hard to estimate time to completion, DanO driving process changes, low motor.  Good SW choices, weeds tracking
- Team is junior and thinly spread - doing ok/good given that - will converge, but slowly
	==> Trudge onward:  Prioritize LOVED(2android).  but cannot drop the rest, gonna be slow, but many systems kinda work.
	==> DanO doing the tiny bit of product work around game creation.  In trouble if high-schools require much new.
	==> Slowly fixing processes: Jira, Sprint, Bugs, QA, Alpha Testing, Release Cycle.
- Hiring
	- #1 ARCHITECT -  (GAP: Customer focus)
	- #2 SW LEAD / TECHNICAL PM / VP-ENG -  (GAP: trains on time, high motor)



#### CV Team Actions
- FOUR - Ideal configuration:  Four cheaper MS worker bees.  
	- Three is too few (overhead eat up ratio of research advancements).  
	- Five or more as we had with Sarthi when we have more cash
- VISHAL - He is ok, but not as valuable as Grzegorz or Maxim; 3x in cost
- SARTHI - We have made an offer to Sarthi and he has gotten an apartment in mass based on this job
	- is ok, but less experienced than others for the same cash
- TIMING - Ideally we don't gap down to three, and start looking for another if his priority applications does not come thru in June.
	- Vishal is the only one doing belief propagation now, and our SW pushes, need to transition them.

PLAN
- Baseline system now...
- 4 sprint deadline to crack player tracking.  begin searching for a super PhD guy at end of sprint 3 if needed
- Crawl-walk-run iterations to expand complete product and harden along multiple axes.
- Error-Analysis-Based,"Headroom"-driven Improvements Prioritization 


#### CV Team Status
- CORRECT FOCUS - Player Tracking Focus is smart - Its the hardest part
- SLOW PROGRESS - Whole team focus on Player Tracking since before Argentina.
	- Trackers get lost after several seconds on average, then players are anonymous

- MISSING PARTS
	- Don't currently have baseline system
	- Don't currently have good stats on end-to-end system performance or component performance

- ONLY USING **ONE** OF **FOUR** SOURCES INFO?! - Tracking (local Inf); Belief Propagation (global inf); Gallery (image); Partial Img
	- Have been only focused on tracking, 
		- But not global constraint propagation, nor pixel-level recognition, nor micro-recognition)
		- Big hopes for the first two sources
		==> Give it 3 sprints if progress, keep MS-level team, else find world leading PhD in crowd tracking

- EXTREME LEVEL OF UNCERTAINTY
	- Lots of water under the bridge - 
	- Internal stats are not entirely a lie, but they seem sloppy, unclear where we are, and DON'T imply end-to-end performance

##### No-Pony

==> Jason, that promised pony will not be under the tree in July, nor will it be there two weeks later.
	nor is it even reasonable to define a hard timelines for business-relevant end-to-end metrics.


##### SOME GOOD NEWS
- We don't need to hit some arbitrary number like 95% accurate stats to win.  We need to be:
	- (1) better than others, and (2) good enough for a viable use case (of which there are many)
- We can put hard timelines on specific actions, and we can make educated guesses about the outcomes of those actions
- Player Tracking - This is the peak of the basic stats mountain, many problems after this, but this by far is hardest
- Soon - It is plausible (>60%) that we will make significant progress on player tracking in the next 4 sprints as we
	- build baseline system w/ best in class components
	- build robust test end-to-end test
	- address player tracking from 4 out of my 5 angles
	- 4 sprints will give us two solid attempts on these problems, and let us integrate all into a single solution.
	- and if it is going to work, we should see movement on the parts along the way
- FOUR-STEP domination does not require full-AI for quite some time

##### ...
==> NOT ANGLING FOR MORE TIME!  Actions must be wise.  Actions must be urgent.  
	But "real" outcomes are unknown, and not coming soon
	Angling for more swings at bat before I am struck out!
	Angling to take smarter swings, and to swing faster!


#### PLAUSIBLE FULL-AI OUTPUTS THIS YEAR:
- Shot Chart <--

- Intelligence Enhanced Visuals -  ball- and player effects.  <<<<< 

- BallerTV - 
- Per-player highlights reel - 
- Player hight

- ?? AI-Recorder ??

Vishal


League are you in
Roster - 
Schedule - 

Meta data to unclaimed Teams <- un
Unique URLs per team


- Can I require globally unique Team names



#### __





### 2023-06-03  AGENDA -- CV Team Assessment

- [CV Issues](https://docs.google.com/document/d/1rHDeKRHHuT4w0q9V8uICIqAt9EzYiUVxdzq85DNzKt0/edit#heading=h.lvbyhldlsmo6) - 
- 

HISTORICAL -  I have not looked carefully, but from a distance it feels like a train wreck to me
- One year lost on an experimentation platform -- Chris
- year long multi-person efforts to develop novel algs that resulted in zero usable output.
- developing lots of algs that are on ice & were never tested robustly (as too many other things were still broken)
- Fired Fong Liu too, Chris, and Phillip


RECENT / CURRENT - Much better, but still some gaps.
++ Maxim and Grzegorz *REALLY* upgraded the team capacity & velocity.  (Martin too, but maybe not as much)
++ Mike's focus on Player Tracking Problem was the right move.  Some parallelization was warranted.
-- but mindless parallelism is a bit leadership-lazy.  look for cases where approaches are not likely to be fruitful and cut them.  Jam other stuff inthere too.
-- In general should be getting "in there" with each dev when their chose path seems off.
-- Platform is too fancy for now: isolated memory,  rule triggering languages, rolling memory window
-- Not testing end-to-end system --- gives false impression of state of our capability
-- Too tracker centric
	-- should not deal with complexity of temporal-tracker when performing a-temporal processing.  hitting a nail w/ screw-driver
	-- belief propagation
	-- player identification 
	-- segment&track anything



REALITY CHECK
-- No working end-to-end system 
-- Key components are known to be broken.  (e. g. shot organization)
-- System cannot id a player unless it sees a Jersey which is generally NOT visible most of the time, & often +10 seconds w/o seeing number.
-- Team has worked on PTP !!IN-PARALLEL!! since *BEFORE* Argentina and still tracker looses the player after handful of seconds on average.
   -- even SIX months w/ the NEW team.
   -- I see ways the team could be working moderately more effectively, but still putting a hard timeline on PTP is fantasy.

- modest reasons for optimism:  
	- Segment & Track Anything anecdotally tracked over long time periods, 
	- Gallery-method looks slightly promising & player-ID is not well tried so I have hopes.  (should not have dropped this!!)
	- Belief propagation intuitively seems potent to me & is totally unexplored.
- FORK IN THE ROAD - 66% We crack the PTP nut, then have team of MS level guys handling the dozens of issues in full stats, 
	 						 33% We go all out for a PhD superstar who has specifically studied person tracking in crowds 
	- Would want to try 3 or so sprints before declaring failure.  Still could start trying to hiring that person before declaring failure  
	  (but its alot of work, and they will be costly!!!)

+++ Whatever we do:  2/3 to 3/4 effort should stay on PTP until strong progress is made.


- Modest ways team is presently performing poorly.
	- Not streamlined (see below)
	- Not "leaning in" enough on the research agendas of our guys
	- modestly less parallelization (maybe?)

- Reality Check 
	- Gap to robust full-solution is large and not well understood.
	- Known gaps would currently drive actual performance far downward for probably fixable, but currently broken reasons.
	  (shot origination, hoop swap, shooter occlusions) 
	- 
	- Others have run from the problem; it will not fall easily.


### 2023-06-03 AGENDA -- CV Team Roadmap

#### Roadmap Thinking

CRAWL-WALK-RUN:  Use cases: Ideal case -> Typical case -> Edge cases
CRAWL-WALK-RUN:  Player Tracking: core-tracking -> shot-moment-tracking  ->  money metric  ->  performant  ->  production 
CRAWL-WALK-RUN:  Player Tracking -> Shots (Player, make/miss, time, ?points?) ->   Products: Baller, Still-image-stats, viral-highlights


DAY1
- Continue on PTP but in broader ways (2/3 effort).  whole system, APIs, systematic testing etc. (1/3)
- Lean in opportunistically with devs
- Do gallery-based player ID
- Build baseline measures for system, and baseline system?
- Test segment&test anything qualitatively
- Build framework for one-line testing of system (use stuff from past)
- Measure SMT to assess importance of Belief propagation
- Key visualization tools.  (e.g. tracklet graph, shot moment graph)

BE READY FOR "RESEARCH MICRO-PIVOTS" on 2-3 days tempo.



CV ROADMAP - shots, players & highlights.   (points are later???)
- Baseline metrics & system:  player-tracking(PT), shot-moment-tracking(SMT), MM. (2-3 sprints).     SMT - all close are localized, others are known away
- Ideal case:  >PT  >SMT
- usual case: >PT  >SMT
- Ideal case: >MM
- usual case: >MM
- performant system:
- production system:
- edge cases:




#### CV Roadmap

BASELINE & INTEGRATED PLAYER TRACKING. (baseline+end2end+sharedClasses; decomposed tasks swing1; swing2; integrate)
- Dataset capture
- Baseline end-to-end system
- Decomposed Player Tracking Problem
	- Baseline per-component metrics

HARDENING FOR NOMINAL CASE 
- Crowd suppression
- Pivo rapid calibration
- ????

TARGETS
==> Baller TV - Basic stats
==> Player halos
==> Still image stats



### 2023-06-03  VISIT -- Old Notes

STREAMLINING (3)
- BASELINE - 
- DECOMPOSITION APIs - 
- MODEST PARALLELIZATION - 

TOOLING (1)
- DATASET COVERAGE
- METRIC COVERAGE
- 
- VISUALIZATION 

RAPID EXPERIMENTATION
- 

PLAYER-ID / RE-ID (2+)

GLOBAL INFERENCE (2)  for initial heuristics

LOCAL INFERENCE (2)
- 

- 2-sprints - End 2 End baseline system
- 2-sprints - Player ID
- 1-sprint - Global Inference

- n-sprints - Local Inference
	- Crowd suppression
	- Distractor hoops, balls




BRIAN
++ Liked, Lowest



BIGGEST GAPS
- ARCHITECT - Able to design toward customer need.  (still must own system integrity. security, etc.)
- TRAINS-ON-TIME - 





REALITY CHECK
- Collosal damage is behind us.
	- No Jira, No Testing, No product specs, no product planning, no bug capture process, 
- Significant damage is still with us.


PRODUCT TARGETS
- STATS AS A SERVICE - 
	- HUDL is a billion dollar competitor with a decade head start that does this, and this alone.  Doing this well is a big task.
- INTELLIGENT GAME RECORDING -
	- PIXELOT 



COMPANY ROADMAP


- STATS AS A SERVICE
	- We break even doing a barely passable job at stats based on non-optomized, somewhat buggy SW.

- PENETRATE AAU
	- Tournament-based Coach Introduction
	- Viral flywheel

- FULL-AI driven adoption scaling

- DOMINATE AAU - Network effect locks AAU league for sustained differentiation.




CV-TEAM GAPS
- MYOPIC - Extreme focus on tracking
	- GLOBAL INFERENCE - Hoops swapped, not in two placed at once, don't shoot your hoop, don't pass to your opponent
	- LOCAL INFERENCE - Bystanders aren't playing, background game ball must be ignored.  Foreground ball ignored too
	- PLAYER ID - Details of ten players are key when jersey is blocked
- END2END
	- SHOT ORIGINATION - 

