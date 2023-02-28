=[[FAANG]]    [[@Amazon]]



# LOG

### m2023-02-27  Theo 2023 chat1

GETTING HIRED AT AMAZON

- DanO best as Software Manager Role
	- Could be an AS, but higher levels endup more like a SDE (software developer)
	- Could be a SDE, but probably add more value as mgr, and probably like it more too.  (but lots of meetings)
	- Expected to understand deep learning algs, but not expected to be able to code tensor flow models.  (should get coursera training anyway)

- Leveling Comparison.
	- Theo is presently an L5; could become an L6 in right context
	- Grzegorz would probably come in as an L6 SDE

- L7 Expectations:  $500-700 total comp; 25-50people in report to; e.g. 2 regular managers + 3-4 direct reports; Scope of one or multiple products

- Getting Hired
	- Leadership principles - able to answer questions based on them
	- Role Guidelines - each role has its guidelines (they are internal, but likely on the web somewhere)


### m2022-10-30  Prep for Reference call for Jason

- Ability to create and execute processes for production quality outputs.
	- Lead Payments Platform were all transactions had to complete error free & interface needed to look perfect.
	- I was leading a team of 60+ so I am not doing it all directly myself.  Still it is important to emphasize as many cases as you can where I am in the weeds on quality.  And of course I learned by being part of it too.
	- Because of our velocity & timeline it was chaos, but we needed great many fixes to support quality for CES
	  Then in the of our time @ Aeolus we were beginning to ship robots to Japan where they were in spaces with elder patients


	- CENTRAL IN CREATING MUCH OF OUR PROCESSES & DOCS - then we collectively evolved these over time
	- SAFETY REQUIREMENTS - 
	- CELL PHONE DEMOS - We instituted Cell Phone Demos, 
	- RE-RE-STRUCTURING TEAMS TO MINIMIZE DEPENDENCIES 
	- REQ UPSTREAM INTEGRATION TESTS - Required upstream integration & test as part of ticket completion
	  (authored the spec levels of testing for each ticket.  (e.g. smoke test, etc.))
	- SCRUM OF SCRUMS - Weekly scrum of scrums
	- 24/7 TEST&DEV CENTER - Remote dev center.  initial setup of the resource tracking docs.
	- HIRING TESTS - Authored our hiring tests, and got into the coding weeds with each finalist candidate before hiring.
	- HANDLING ESCALATIONS - Handling escalations of problems.
	- SEPTEMBER CODE REWRITE
	- Monitoring quality on sprints
	- MIT QUALITY - Systematic testing in operational conditions:
		- TURN TABLE - Developed turn table systematic data collect; Live robot data collect;
		- CONTINUOUS TEST FLOOR DATA COLLECTING
	

### t2022-06-29  Request for recommendation


Theo,
Any forward progress on figuring out your next project?  Any ML ideas?  From my side I am spending some time getting current on ML algorithms.  (Everything has changed since I was last here!)  Not sure how deep I want to go, but my thought is it might land me more interesting gigs.  (I have not yet gotten serious about interviewing, but I am ramping up to it.)

And for that I have a favor to ask.  Could you write letter of reference for me?  I think you have seen me in action as a manager, as mentor, and as a technology driver, so you are in a great position to write one.  

let me know,
--Dan


If it is valuable here are a few ideas that occurred to me... feel free to use or disregard them:

- MENTOR:  When I started working with Dan, I had never managed, was not confident nor knowledgeable about how to do this.  Over time at Martian then Aeolus robotics I went from managing my own efforts to effectively serving as the VP of eng over 60+ engineers.  Over these years Dan provided nuanced feedback about how to best manage the interpersonal, and organizational aspects as I progressively transitioned into these larger roles. 
- TECHNICAL: Dan is very deep and very broad technically.  His Robot OS architecture was the blueprint for the Martian software system and the starting point for our Aeolus system as well.  As our organization grew his job push him progressively higher level.  Still as problems arose he could and would dive very deep into all aspects of AI, Firmware, Robotic Control, Vision etc. in order to understand and collaborate in plotting technical paths forward.  He is an ideal advanced technologist leader in that sense.  In the early days he and I spent many afternoons at the white board discussing algorithmic innovations that would control the complex 3D wiping required by our bathroom cleaning bot.  Later he architected and mentored the team building our asynchronous, incremental, hierarchical planning system.
- INTERPERSONAL:  Dan is a hard driving leader, during our time at Aeolus especially we were under incredible time pressure, the CES deadline could not slip, not even by a day.  He slides from delegate&monitor, to mentoring&proposing, all the way down to tactical firefighting depending on severity and urgency---he is an 'all in' leader.  At the same time, the team likes and respects him.  Many of his direct reports (like me) never managed anyone before, so we all looked to him to varying degrees in how to best execute.  
- I would say he also promotes a low politics, low drama, mission oriented organization.  There were stresses, but those were the stresses from driving hard for an ambitious deadline that cannot slip, not of politics.
- Dan, was watchful for issues of burnout.  He and I often discussed how individual members of the whole organization were doing, and in cases we would temporarily assign one manager to help another, or tactically move specific tasks with pressing deadlines to avoid emotional overwhelm anywhere in the team.  
- His approach promoted a "we are with you", rather than "we own you" vibe.  With Dan's help I also became much better at driving an aggressive schedule w/o burning or angering the team.

This is an interesting story, but it might be too much info for a letter of recommend:
- TURN AROUND STORY:  With only three months before CES we realized several of our key vision and motion algorithms were performing more poorly than we thought.  It was a big risk to rip these out, but they too poor to build from even in the medium term.  He organized a two week sprint to see if we could get completely new algorithms to a basic level of functionality.  Once we did that, then the big swap decision was made.  Dan's approach minimized the risks of total failure while still allow the team to move over much better algorithms.



### m2022-03-08 Astro. FAANG. ML new ideas.
existing product; science early prod validation; new product

[[FAANG]].   




### m2022-03-08 Ideas About Amazon Levels
#### Dan's Role:   
- AS (Associate Scientist)
	- Pubs do matter, but so does doing research in applied setting.
	- AS science role (code but not production; )
	- strategies around stuff algo the read
- SDM (Software Development Manager)
- Staff scientists (publishing)

#### LEVELS AT AMAZON
The "essence" of a level is the amount of scope afforded company.  
- L5 own a feature, not strategy
- L6 influence across multiple groups; front line interacting w other groups
		    - strategy with their group; mentoring; hiring; growth members and team
		    - scope: influence multiple groups within a product
- L7 influence across multiple products.  over a whole product and some others
		- 40-60 under them.
		- principle engineer  (hard part of code, or sys architect)
			- large strategic impact; referees in org
		- need to align w/ corp culture from day one


#### THEO:
L6 IC->SDM (Software development manager)
   very productive team. low headcount. 

INTERVIEWING PREP
- What have you accomplished lately
- Break down problem; both in the weeds & strategic
- Amazon corp values

### m2022-03-03 - AEOLUS research summary from Theo's perspective

AEOLUS RESEARCH
- Markus Vincze lab @ TU WIEN (U of Vienna)
  V4R (Vision for robotics)
- David Fischinger (Grasping Research)
  [Height Accumulated Features (HAF) grasping](https://www.acin.tuwien.ac.at/fileadmin/acin/v4r/v4r/IROS2012_Draft.pdf) 
  Parametric Object Grasping
- George Todoran. (Base Motion Planning)
  Optimal Local Path-Planning  (using an ODE solver)
  Moving Horizon Trajectory Planner (MHTP) with an update rate of 10Hz
  Randomly-exploring random tree search  (RTT*).  
  A* over heuristically generated way-points graph


FROM THEO
- wiping research - severin, Brandon
	- Triangular mesh.  
		- Marching squares
		- Flat panes
- What algos did we use?  
	- SIFT/SURF - tracking
	- skeletal tracking
	- base motion - 
	- grasping  HAF. (prof name & )

Triangule -- Marching squares.  
- flat region (largest planes, fill with lesser planes)
	- greedy grid growth (continuity -gap -angle; curvature; )
		- first / second order discontinuities
	- flood fill (based on normal vector similarity, and physical separation and gaps)
		- -> 3D least squares plane fitting

Academic Algs impractical / 70% cheats

Entirety of use-cases.  least rescources to solve the problem.


- haptic edge wiping algorithms
