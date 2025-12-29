. [[S.T.A.R.]], [[RES]], [[Star Stories]], [[Star AI]], [[RES Pyramid]] 

Anchor Stories: A time when you:  (a) created value,  (b) solved a messy problem,  (c) were asked to lead

had my fun, made my millions, but this AGI revolution calls me back to more strategic roles

# __


Deep Learning/AI Researcher/Leader.  Career includes building 5 orgs from scratch (up to 60 engineers), funding hundreds of millions in research at DARPA, exiting three companies, and bringing 8 products to market worth billions. 

Looking for a hands-on role leading generative-AI based innovation.
My sweet-spot is bridging world-leading R&D into rapid productization.






# TOP STORIES

## = HANDS-ON ML EXPERIENCE

### Developed SV OCR/Gallery System
Developed a novel self-supervised player identification system that fuses a custom-trained dual-head jersey digit recognizer with multi-frame statistical fusion using signal-to-noise analysis. Built the digit recognition model on DINO v2 architecture, training on thousands of synthetic jersey number samples per class generated via inpainting techniques from real game footage, achieving robust player tracking through automated gallery generation and temporal aggregation.

S: Neither ReID, Tracking, nor OCR was strong enough to track basketball players accurately
A: Developed proprietary OCR and self-supervising Gallery system to solve player ID
R: Advanced F1 from under .3 to over .9

**Novel Technical Contributions:**
• **Dual-head digit classifier**: Repurposed generalized transformer (DINO v2) and treated left and right jersey digit as distinct classification tasks.
• **Synthetic training via inpainting**: Generated thousands of jersey number samples per class using advanced inpainting on real game footage
• **Multi-frame statistical fusion**: Custom signal-to-noise analysis aggregating OCR readings across track duration with dual-threshold validation
• **ByteTrackWithColor**: Extended ByteTracker with jersey color-aware associations to prevent identity switches between teams
• **Self-supervised gallery generation**: Automated player-specific dataset creation through temporal aggregation and collision-aware track splitting
• **Sports-optimized OCR pipeline**: Custom PaddleOCR wrapper with batch processing and jersey-specific filtering for basketball contexts


### Wrote DAT - Persistent Self-Documenting Data Containers
Architected a lightweight persistent object framework combining filesystem storage with YAML/JSON metadata, featuring automatic path resolution, lazy loading, and inheritance through base object composition. The system provides elegant abstractions for reproducible ML experiments with minimal boilerplate - enabling seamless data versioning, caching, and method chaining across distributed development workflows.

S: Code was not easily re-run or remembered; work to integrate w/ DVC or ML-flow
A: Developed simple open source data framework DVC DAT
R: So few lines to write a DAT wrapper, most all data objects are now DATs



## = BUILDING TEAMS and Maintaining them
Built and led 5 organizations from scratch, with the largest encompassing 40+ PhDs and 60+ engineers while overseeing R&D funding budgets exceeding $200M. Demonstrated consistent ability to rapidly scale high-performing technical teams across diverse domains - from robotics and ML research to commercial software development - while maintaining low turnover and delivering complex products under aggressive timelines.

- **Aeolus**: 60+ Engineering team, 40+ PhDs, hired in 9 months
- **Analytics Fire**: 45+ Engineering team, very low turnover over 9 years
- **Sports Visio**: 20+ Engineering team, replaced entire CV team


## = PRODUCT DELIVERY

### PRODUCT DELIVERY:   >$3B at AF   (largest delivery)
Largest scale deployments
S: Sunpower wanted to create a residential business from scratch.
T:	Deliver omnibus solution
A:	Built-in QA, what is not getting delivered, quarter-by-quarter deliverables.
R:	The largest driver of new business across all business units for a decacorn.

### PRODUCT DELIVERY: Solar Planner; CV BB stats (Most Advanced Delivery)
### PRODUCT DELIVERY: (execution) SportVisio BB - BB Stats & highlights


## = LEADERSHIP & VISION


## = TURN AROUND
Proven track record of executing successful organizational and technical turnarounds under extreme pressure, from rescuing "unshippable" products and replacing entire teams to rebuilding core algorithms 70 days before major demos. Consistently transformed failing initiatives into funded successes - turning $3M in burned capital into shipping products and Series A funding, building 60+ person organizations from scratch in months, and threading the needle between fixing vs rebuilding critical systems when facing immovable deadlines.

- [[Star#SPORTS VISIO - Turn Around]] 
- [[Star#Aeolus Turn Around]] 

### Aeolus Algorithm Crisis
**S**: 70 days before CES with booth already purchased, core motion planning and object recognition algorithms failing
**T**: Navigate fix-vs-rebuild decision under extreme time pressure
**A**: Rebuilt core algorithms in 3 weeks while maintaining CES timeline
**R**: Successfully delivered functional demo that secured follow-on investment

With booth already purchased and major demo commitments made, discovered that foundational algorithms were not working as expected. Successfully threaded the needle between fixing existing systems versus starting from scratch, ultimately rebuilding core functionality within the compressed timeline to deliver a successful CES demonstration.


## = YOUR WEAKNESS 

### - WEAKNESS:  Too fond of super-general, super-elegant solutions
- UPSIDE: Gifted at elegant processes/architectures & getting to the heart of some complex problem.
- DOWNSIDES: Easily can (a) impact schedule, (b) priorities, or (c) be over-engineered. 
- PREEMPT:  Good and sensing risk.
- LEARNED SOME TOOLS:
	- DELAY -
	- PIECEMEAL - Build the general solution in stages.  Nearly every project.  SV Minutes, Court Homography, Halos, DAT.
	- LOW-HIGH - Explicitly hack parts of the solution w/ throw away parts
	- TIME BOX - Deliver "best result" / POC in a very short window

#### DAT FRAMEWORK
- Delayed -
- Time boxed -
+++	Didn't over-engineer or waste time, 
---	But did delay too much before done
==>	Should have piecemeal & time-box




## = CONFLICT

### MISC CONFLICTS
- AF Equity Split with Nick
- Joe Aug selling the company while not being invited into the next one.
- Pietro Firing
- Hack week: asked, call-to-carpet... failed to pull Maxim.  Caved in, then led by example

### SCOPING (CONFLICTS) 

S:	Unrealistic scope & firm belief that a 'real' executor could do it.  (both Alex & Jason)
T: Do it
A: ShipDateBasedOrdering, Moth-Balling, Product Matrix, 1Pager
R: Org shipped product & moved from $0 to $800K ARR

#### Over Scoping --> SHIP DATE ORDERING
- Startups need to do more with less and faster - still, CEO over-tasking is counterproductive.
	- ==> #1 - Agreed to 'ship date' ordering, then built schedules based on that evolving ordering.

#### Over scoping --> "MOTHBALL" DESCOPING
- OVER SCOPING - Showed several implicit deliverables, and the product was too high for the headcount.
- "MOTHBALLING" ==> #2 - Got multiple efforts mothballed so we could focus on our mission-critical deployments.

#### Shoot from the hip tasking --> PRODUCT MATRIX & SPEC-DRIVEN RESPONSES
==> **PRODUCT MATRIX** - Developed product matrix with 'shirt size' estimates of effort, risk, and impact


**Tennis POC** - "Give me a POC for tennis in a day or two"
- "this will not be a day or two it will be the better part of a week from most of the team"
- ==> #3 - Aggressively moved forward (did planning alone).  Then made all activity very visible on day by day basis.
- He eventually shut it down after several days and accepted a smaller deliverable.
- ==> # 3b Extended sprint a week to accommodate, then let him see quarterly deliverables slip a week.
		  he accepted it all, and I think it made him more appreciative of the full POC costs; mid-sprint changes costs.





### UNDERPERFORMING QA LEAD  (CONFLICT)
S:	Rumors of "hard to work with" QA lead (Jochim);      Slow Vishal delivery
A: Joachim QA Lead - complaints & velocity -> gave him a direct line to me -> kept arm's length via email
	  -> supported him and made delivery clear and significant -> Second try -> Fired on third.
A: Maxim slow delivery, low motivation -> dug in (wanted "professor" job) -> strong delivery dates (he hated it), slow -> let go
R: The team was happier & hired better/cheaper QA later


### UNDERPERFORMING TEAM LEAD (CONFLICT)
S: Greek manager having trouble wanting to quit; hard worker & good technical choices; could not sleep
	Hard worker and ran a well-organized team.  wanted to 'demote' to IC. Stressed out, paralyzed by decisions.
A:	Gave him another manager to 'lean' on.  We turned this on more than once, but over time, he got better.
R:	Great result; the team was well managed & other manager had little trouble joining the standup.
	He needed to feel that he was not alone in committing to actions and deliverables.




## = MENTORING
Mentoring: Theo Enns and Filepe @ Aeolus.

S:	Junior Engs w/ little or no experience as TL
T:	Keep eng, drive excellence
A:	Aeolus - Choose many interim tech leads on a week-by-week basis, merge/split teams by need.
A:	Greek Mgr
R:	Delivered into CES, raised $16M

## LEADERSHIP ???

### SV Roadmap - Pivot AI & team




# === PEOPLE MANAGEMENT ===
## = MANAGEMENT STYLE/PHILOSOPHY
### MGT STYLE -  [What is your management style?](https://www.indeed.com/career-advice/career-development/what-is-my-management-style) 
- Problem Solver
	- UNDERLYING TECHNICAL PROBLEM - 
	- HUMAN PUZZLE PIECES PROBLEM;  Georgios-stressed, Adams-Proj-Autonomy;   DEMO LOYALTY
		- Adams ran tangents if not corralled, but chaffed (gave his own proj) - 
		- Georgios-stressed leader, but awesome leader (tandem leadeship ownership)

- Human puzzle pieces
- How “hands-on” of a manager are you?  depends.  See delegation
- How do you support an underperforming team member?  see delegation
	-> fast/precise decision to fire  (AF-test-tasks)
	-> progress towards ownership (see situational leadership)
	-> peering/mentoring arrangements
- Can you provide examples of when you have had to deliver negative feedback?  PIETRO, ADAMS, 
	- THEO(Autistic, ), 
- What strategies do you use to motivate your team? (see human puzzle pieces)
	-> Ownership.    -> Benefit.   -> Trust/Honesty.  -> Greater Good.  
- DELEGATION


## = WORKING STYLE
1.  [Describe your work style.](https://www.indeed.com/career-advice/interviewing/what-is-your-work-style)     <<<<<
- How would you describe your top three strengths related to work? What are your three biggest areas that have room for improvement?
- How do you deal with stressful situations?

#### . Organized Person - Do you describe yourself as an organized person?
	- Builder of systems.  Fast Start, Fast Edit, No Committees (just do it, or assign it)
	- Out of my head and into lists kind of organization person

	Use document, spreadsheet, or asana as step one

	_
### - MISSED DEADLINES - When is the last time you missed a deadline? Why did this happen?
	Aeolus: 2-month demos.  would pull capabilities back

	_
### - STAYING CALM - STRESSFUL SITUATIONS
	- Am a talker


	_
### --- Aeolus: 3M to go and core functions are unexpectedly failing; poor algs ---
	- WHY: Moving too fast on the basic stuff; claims were false
	- 3 weeks to build new algs

	_   <<<<<
## = PERSONNEL MANAGEMENT    !!!!!
### - TEAM MANAGEMENT
#### . Personal Wizards Team (3)
#### . DARPA in house () BL () TL () MR ()
#### . AF 40+ 

### - HIRING
#### . Hiring Grasping Experts (Aeolus, Hiring)
	- Had team identify tier1 journals; & frequently publishing teams; picked top 10 world wide
	- Wrote hand crafted emails to lead prof at each
	- Phone call wooing; Flew to Vienna; funding multi-year grant;  Hired two leading graduates
#### . Money Ball Hiring (AF)

### - PERSONALITY MANAGEMENT
#### . Manager Of Snowflakes (Aeolus, AF)
	- ALEXI - Polyphasic sleeper (what to do but, never how)  lesser programmers; great helper, does anything asked.
	- PIOTR, ???, SENAD - Company Owners:  Piotr, S Am, Roboticist professor
	- Single Guy:  Thai-beaches, Crypto conferences
#### . People as Jigsaw Puzzles (Aeolus, AF)
	- Consider employee's:  Depth, Width, Velocity, Padding, Creativity, Care, etc.
	- Mix and Match strengths to cover any gaps before they are large problems.
	- AF EXAMPLES:  Bryan, Matt, Ben, Ruby, Jess (universal soldier)
	- AE EXAMPLES:  Georgios+, Alexey 
#### . Lone Wolf Alexey (Aeolus)
	- Super strong & focused C++ developer.  Wanted to know more than all others & be better
	- Willing to do anything in C++.  Willing to help & teach, debug, code anything
	- Asshole who knew he knew more.  wanted autonomy and isolation
#### . Fragile Georgios (Aeolus)
	- Did not mind managing details, nor working hard/long
	- Feared being 'in charge' & 'responsible w/o partner;  Could not handle pushing others or ANY confrontation.
	- Feared for job security  
#### . Vacation Policy - Hiring Europeans (Aeolus, AF)
	- Taiwanese CEO; 5-days in my contract; sleep @ desks; 5-week expectation 
	- Early employee 'red-flag' rejected offer; placeholder-excuse  ->
		emailed CEO second ask  ->  then emailed reject w/ strong policy recommend for no explicit policy
#### . Jess.  The Universal Soldier
	- Supported her music career, worked flexibly with her
#### . Derrick Depression -
	- Worked at a distance.  Dinners when in town.
### - POLITICS - MANAGING UP
	- Vacation Policy (Aeolus)
	- Product vision (Aeolus)
#### . Dual Backlog Maintenance
#### . Multi-team Synergies





## = STAKEHOLDER MANAGEMENT
### - CONTRACTING (customer-forced delivery failures)
#### . In-then-Out Planning; Delivery Timeframes Tied To All Inputs In; Aggressive Tracking & Notify (AF)
	- Joe at Acme will provide X by DATE1  (with contract having DATE2>DATE1 in planning)
	- Aggressive steak holder notification of delivery risks.
#### . In House Testing Upon Delivery (AF)
	- Ultimately do it for free; but try to charge on next cycle.
#### . Poor Counter-Party Performance Planning (AF)
	==> Easy to get apriori-agreement for harsh actions on non-compliance; better to get appropriate actions 
	==> Example:  Payment and Work-stoppage Timelines
	- Massively screwed by an unscrupulous founder - almost sunk the company.
#### . Dual BackLog Management
#### . Basil Health Contracting Failure

	- State with controller @ sun power
	- Arthrex agree on timeline; but then did not deliver inputs; we had to scramble including unpaid work to keep timeline
	- Enphase 
### - HIRING by direct testing  (Blow their doors off & see what happens.)
#### . BP Sox
	- CEO's buddy runs a medical device company & whole outsourced dev team for one product ghosted.
	- half-ass spec, and email history indirectly describing setup.
	- faked-inbox of panicked, lazy, vacationing team members.
	- bullet point dossier of team members.
	- What do you do in first 3 hours, first 3 days, three 3months.
	- Outline of an improvement roadmap pitch to the client (drawing from complaints seen in emails)
	- One presentable slide from the pitch to the client.
	- If you need some missing info, then email the CEO, and invent a reasonable response email and go with it.
	  (e.g.  do you have offsite backups of the source code?)

	- Do they raise a flag at the email FYI that the a key guy is taking a PTO.
	- How do they respond
	- Can they play nice with the one dev from the team the ghosted them in order to get server keys.
	- Do they act to preserve data.
	- Do they act to keep servers online (bills had not been paid).
#### . Vision algorithms do flood fill






## = ORGANIZATION MANAGEMENT
### - ORG CHANGING
	- Broken org, org in flight, dramatic org shift (to )
### - MARTIAN->AEOLUS->LANE-LEADERS
### - AF
	- Hiring
	- Pod Teaming Matrix Structure (PM, Dev, QA, Admin)   Cross Cutting Org Roles
	- Quarterly Planning Discipline, Digital Escalation Tools.
	  ==> Days matter, need to know early when things are wrong
### - DARPA
	- BL-testing team
	- MR-
## = EXAMPLES: TEAM MOTIVATION & RETENTION
	- What strategies do you use to motivate your team? (see human puzzle pieces)
		->Ownership.    ->Benefit.    ->Trust/Honesty.    -> Greater Good.  

### - OWNERSHIP -


### - VISION - Motivation from Coherent Company Vision
	        AF: Clean Tech.  Aeolus: Eldercare.  DARPA: Driving AI Field

### - TRUST - How to maintain trust among co-workers
	Is is scalable, is it personal

#### . AF - BONUS STRUCTURE - 
#### . AF - Personnel Evaluation
	- Input from MGR but final determinations two levels up
	- Explicit 360 Positive Attribute assessment from others


### TOPIC: TEAM - Cohesion / Retention / Quality
#### . AF - Annual Offsite
	- Spent 2% of top line revenue on this.
	- Huge team involvement!  
	Nominate, Analyze, Trim outliers, Let team vote (sometimes with location qualifiers)
	- Hackathon. Philippine winners.
#### . AF - Bonus structuring 
	- team bonuses.  Job title, one-time bonuses
#### . AF - Friday "lunch" talks, Offsite hackathon, talks





## = EXAMPLES: HUMAN PUZZLE PIECES
	- Clearly demand what is required; with argument about why; then let them propose 
		Early failure feedback
	- Notice where people gravitate; build from that

### - AMBITIOUS
#### . Attilla
#### . Krzsiek

### - STRONG DIFFICULT
#### . Alexy
	- Let him operate on his own
	- Let him define & own and run his own stuff
		- He defines, we review, raise inconsistencies
		- He delivers, tests and supports integration
	- Required to provide short term coaching on specific activities

	- Issues:
		- didn't respect others (but he was stronger dev)
			but was polite as long as other was more junior & not too much of his time
		- Poly-phasic sleeper
		- didn't want others defining his tasks/apis, but his work was good
			--> just needed to validate his approach was compatible (& would easily update)
#### . in the Italian log cabin

### --- AF: Snowflakes Krzsiek/Attila side gigs,

### - GAP FILLING
#### . Matt 
	Stressed by top down politics
	Stressed by ambiguity
	- Great people skills, client skills, technical skills, 
	- High velocity, bandwidth

## = EXAMPLES: DELEGATION
	What’s your approach to delegating work to employees? 
### --- Ensuring Results:  SMART
	How do you ensure that tasks are completed?

	--> Let THEM tell YOU when and how they will do it.  AF-admin-team.  AF-leads.  AF-alexey. 
> > Get precise especially about when.  Add time for other things.
> > Allow them to slip for good reason, but be explicit and target new time

	- SPECIFIC (simple, sensible, significant).
	- MEASURABLE (meaningful, motivating).
	- AGREED  Achievable (agreed, attainable).
	- REALISTIC   Relevant (reasonable, realistic and resourced, results-based).
	- TIME bounded (time-based, time limited, time/cost limited, timely, time-sensitive).


### --- R&R Doc + One belly button ---
	- AEOLUS Role & Responsibilities docs at.  (needed to tread softly, tentatively,  and swiftly, would often ask person to assume role for a month to 'help out')
	- AF: Responsibilities Chart -- Hundreds of responsibilities
### --- By Written Input & Output ---
	- Written output
	- Video outputs -- At Aeolus.

	_
### --- By Pairing ---
	- AF: Hiring/Research - James with Maria
	- AF: Piotr and Monica
	- AEOLUS: Georgios (afraid of responsibilty), and Filepe
	_
### --- Theory: Situational Leadership:   Directing --> Coaching --> Supporting --> Delegating
	[Web](http://www.md2md.uk/delegation-using-situational-leadership/) [Situational Leadership](Kenneth Blanchard and Paul Hersey’s situational leadership model ) Blanchard&Hersey

						EMPLOYEE SKILL
					High		Low
	SUPPORT High	Supporting	Coaching
	SUPPORT Low		Delegating	Directing

	- Precise output
	- Small / Quick -- 
	- From Them (in quick iterations)
	- Written -- goal&output (even few sentences)
	- SLOPPY! -- I grade on sloppiness of result
	- VIDEO --
	- Actually Delivered
	- Process

	DIRECTING: 
	COACHING:  DARPA slides.  Aeolus: Theo.  AF Hiring
	SUPPORT:  Aeolus: Georgios
	DELEGATING: Aeolus Lane leaders:  Filepe&Severin, 

	_
### --- Clarity beats Speed
	- Dot your i-s and cross your t-s when delegating.  Extra overhead seems a waste at the time, but it's NOT!  Something blows up so often when you don't do it!

	- Not able to make a quick 'firing' decision because tasking was not clear.  So failure was plausibly our fault.
	- Ownership of testing and validation was not agreed upon @ Aeolus and it resulting in team skill gap.
	- PH team data gathering.  Many hours of wasted effort, until we forced "single row" acknowledgment.  (cannot just say 'I got it'.  Instead say 'like this, right?')
	  Highlighted mis-understandings, missing knowledge/capabilities etc.
### --- Simplicity is key in goal/business case
	- Everyone needs to be able to articulate their goal easily and quickly
	- 
### --- Communicate Obsessively
	- Have regular 1-on-1 meetings w/ reports and stakeholders

### --- Create a Flywheel
	- Creatively clump things together in order for you and others to gain efficiency in throughput.
	- How things are delegated.  What things you&others are responsible for.  
	- Which areas each person must 'keep track of'


### --- OKRs 
	- Objectives and Key Results - Very powerful in cases where they can be made to make sense
	- Many Contracting Objectives - Around completeness and timeliness for contracts and agile stories
### --- EXAMPLES ---

### --- AEOLUS:  Two level org structure
	- Built around skill areas & architectural design
### --- DARPA: Decks.  DIRECTING -> COACHING ---

### --- AF: PDD.  Supporting --> Delegating ---

### --- AF: Hiring.  Directing --> Delegating ---

## = EXAMPLES: TROUBLE
#### ?
	1.  Describe a time when your boss was wrong. How did you handle the situation?
	2.  [Tell me about the last mistake you made.](https://www.indeed.com/career-advice/interviewing/tell-me-about-a-time-you-made-a-mistake)
	3.  Describe a time you got angry at work.
	4.  Describe a time when you disagreed with your boss.
	5.  [Describe a time when you had to give a person difficult feedback.](https://www.indeed.com/career-advice/career-development/how-to-give-negative-feedback)
	6.  [Tell me about how you dealt with a difficult challenge in the workplace.](https://www.indeed.com/career-advice/interviewing/what-is-the-biggest-challenge-you've-faced-in-work)
	7.  [How would you deal with an angry or irate customer?](https://www.indeed.com/career-advice/career-development/how-to-deal-with-angry-customers)
	8.  Describe a time you chose to not help a teammate.
	9.  [Describe a time when your work was criticized?](https://www.indeed.com/career-advice/career-development/steps-to-handle-criticism-at-work)
	10.  [How would you fire someone?](https://www.indeed.com/career-advice/career-development/terminate-an-employee)
	11.  [What was your greatest failure, and what did you learn from it?](https://www.indeed.com/career-advice/interviewing/interview-question-what-is-your-biggest-failure)
	12.  What’s the biggest lesson you’ve learned from a mistake you’ve made?

		- When have you failed and learned from the failure?  Basil Health.  
		- When did you fix something that went wrong or broke down?  AEOLUS-FAILURES
		- When did you improve something? 

### MISTAKE - Tell me about the last mistake you made.

### CONFLICT - When have you had a conflict with a coworker and worked through it?

#### JASON - prioritization, focus & consistency --> delivery date ordering --> jack PM
- Total ordering for delivery dates - agreed in principle, but documenting for commitment was failing
- Worked indirectly to have an untrained 22y/o manage the back log w/ Jason & head of sales 
- Trained him and eventually got in formally listed at the PM

#### ALEX - RefusedToAcceptTradeoff; effiency & prototype

#### ...
		- TESSA, PIETRO/ADAMS(PhD-helper,FeetDragging,GoodDelegation,OwnedSBIRs), AZAT, 
		- 
### - BOSS WAS WRONG?
#### . Alex 

### - FIRING SOMEONE - How would you fire someone?
#### . Pietro
#### . Ken @ AF


### - ANGRY - Describe a time you got angry at work.
#### . Alex CEO -
#### . Axle - Working on parts he wanted
	- Dual backlog, working weekends
#### . AZAT - Reimplemented core as means of control



### - FAILURE&FIXING EXAMPLES
	- CONTRACTING:  Whole section Basil Health, Dual Backlog, ...  
	- TESTING: Robot integration

## = EXAMPLES: GOOD OUTCOMES
	2.  Describe a time you went out of your way to help somebody.

#### ABOVE & BEYOND - Describe a time you went above and beyond at work.


## = EXAMPLES: INTEGRITY
	- Dedication to team - Theo
### INTEGRITY
#### INTEGRITY STORIES
##### .. IBM motto -> DARPA reality
	honesty always
		“IBMers value:
		Dedication to every client’s success
		Innovation that matters – for our company and for the world
		Trust and personal responsibility in all relationships”
##### .. China Stealing Secrets
	- Double locked doors, with blanket kept overtop robot


#### LIE - [Would you ever lie for a company?](https://www.indeed.com/career-advice/interviewing/integrity-interview-questions)   [Integrity Story]
	YES:  Under legal obligation.
	- NDA occasionally requires white lies in order to keep secrecy.  (90% of the time an lie of omission will suffice)
		- Why are you in town?  (they can infer it is because of work with a competitor...)
	- SELLING NDA:
		- Needed to create an entire fake activity stream to cover the large gaps in founder availability.
	- SELLING MARTIAN ROBOTICS:
		- Tricky investors, midnight call, don't tell the CEO, etc.
		- Told them my Lawyer told me I cannot talk to them until I resign from Martian



## = EASILY ANSWERED or IRRELEVANT

### TRAVEL - [Are you willing to travel?](https://www.indeed.com/career-advice/interviewing/how-to-answer-interview-questions-about-travel)
	- ABSOLUTELY YES - You do what you need to do to get the job done.  I would even enjoy a certain amount of it.
	- BUT - I am not looking for a job where frequent travel is a core part of the job itself.


	NIGHT AND WEEKENDS -[Would you be willing to work nights and weekends?](https://www.indeed.com/career-advice/interviewing/how-to-answer-interview-questions-about-shift-work)
	- ABSOLUTELY YES - You do what you need to do to get the job done.  I would even enjoy a certain amount of it.
	- BURSTY - Indeed I am a 'bursty' performer, with desire and ability to drive hard when it matters
	- BUT - A job that frequently requires lots of nights and weekend work is not sustainable or efficient/effective.  It is a sign of a broken org; I would not be happy with that & would try to change it


### YOUNGER MANAGER - How would you feel about reporting to a person younger than you?
	- I HAVE - Nick @ AF.  Tessa @ IBM.  
	- ZERO ISSUE - Ideally I work for someone I respect.  And my respect has ABSOLUTELY ZERO to do with age.  They might also have less experience too.  is fine too, if good instincts.


### MANY JOBS -  Explain why you’ve had so many jobs?
	- N/A - I have not had so many jobs


### GAPS - [Can you explain these gaps in your resume?](https://www.indeed.com/career-advice/interviewing/how-to-explain-employment-gaps)
	- N/A - I don't have gaps on my resume


### OVERQUALIFIED - [Are you overqualified for this role?](https://www.indeed.com/career-advice/finding-a-job/overqualified-for-job)
	- N/A - I am not over qualified for this job


### MOVIE - What is your favorite movie of all time and why?

## .

# ===  PROJECT MANAGEMENT ===
## = PROJECT MANAGEMENT
### - REMOTE WORK
#### . ROBOTIC VIDEOING - Video surveillance system w. Tripods and canned setups.

### - DELEGATION & MONITORING
#### . Developed Ten Stage Deck (DARPA)
#### . Monitoring and Testing Standards (DARPA)
	- Worked with NIST/LDC on TREC competition and on test set creation & APIs
	- Worked closely with XXX at Cycorp to develop BL curricula


## = DEVELOPMENT MANAGEMENT
### - TESTING
#### . Universal Video Validation  (Aeolus, Mgt, Fail, Testing)
	   REQ:  Explain in text how you tested it & why that shows it is done.  Include phone video w/ audio.
	==> Super super EASY.  No one can argue it is too hard.  avoids generality/stability need for repeatable tests.
	==> Ensure testing really did happen.  (just compiled it!  couldn't test easily so said it 'worked')
	==> Upper management results porn; Team alignment (hey! that is not done, or not right!)

	  people were not lying exactly, but they were being sloppy and generous in saying it was tested.
	  backlog items had a star if they needed a video (often sub items did not need a video)
  
#### . Robot Integration - Not done till upstream system ran (Aeolus, Mgt, Fail, Testing)
#### . Customer Bug-Frustration
	- Customers don't like paying to fix newly discovered bugs in already delivered code.
	- Built out cost effective testing org, and use it even when customer isn't paying for it.


## = VISION / LEADERSHIP
### - Pitched winning proposals
#### . 12PY adventurous research grant (IBM)
#### . BL program, MR Program (DARPA)
#### . Tapped for IPTO SAG; Gates presentation (DARPA)


## = INITIATIVE
	- When did you create a new process or initiative?  

#### . AEOLUS - 2-level mgt; R&R; 3-week sprints, 2-month prototypes, remote work, testing regime

#### . AF&AEOLUS HIRING PROCESS, 
#### . TREC-TAC, 
#### . BL-PROGRAM, 


### HACKED - Hacked something to my advantage

		- Convinced my linear algebra teacher that I should not be scored on my homeworks but only on my
		  test results.

		- Later used his open-ended test taking policy to save my GPA, by staying over an entire night and re-deriving 
		  two required lemmas that I would have known had I actually done the homework.

		  (ok, so my second hack was really more of a painful recovery from the consequences of my first hack.
		   still in the end, I got my way, and saved my grade.  Wise?  no.  but hacked, yes.)



	_

## = OUTCOMES & ACCOMPLISHMENTS



## = GONE ABOVE & BEYOND 


## .







## BY COMPANY
## SPORTS VISIO - Turn Around
**S**: $3M and 2 years spent, never shipped, never met deadline within factor of 2
**T**: Right the ship, improve quality & velocity
**A**: Layoffs, Descoping, Hires, Revamped all processes
**R**: Shipped stable products, bugs dropped immeasurably, raised $5M+ from Tier-1 VCs

Hired to fix quality, velocity, delivery, and personnel issues across Software and AI teams. Replaced the entire AI team, hired new QA team, developed and implemented formalized SW life cycle, and strategically reduced product scope to match dev resources while still delivering features required to drive sales. Went from an "unshippable" product to deployments that routinely had zero high severity bugs, raised $5M+ from Tier-1 VCs including Sapphire Ventures, and grew sales from thousands to hundreds of thousands.

Hired to fix quality, velocity, delivery, and personnel issues across Software and AI teams.
- Replaced the entire AI team; Hired a new QA team
- Developed and implemented formalization SW life cycle
- Worked to strategically reduce product scope to match dev resources while still delivering features required to drive sales.
==> Went from an "unshipable" product to deployments that routinely had zero high severity bugs.
==> Raised $5M+ from Tier-1 VCs including Sapphire Ventures
==> Grew sales from thousands to hundreds of thousands 

### __
#### ...
As a result we: raised over $4M in new funding, Grew sales from thousands to hundreds of thousands, formalized bi-monthly shipping of four products typically with zero high priority bugs in production across all
==> Transitioned to bi-monthly shipping of 4 products with >10x reduction in serious bugs

==> Raised $3M seed from Sapphire Ventures
==> Grew sales from x to y ARR
==> Transitioned to bi-monthly shipping of 4 products with >10x reduction in serious bugs


#### star
SPORTS VISIO Turn Around
- S: $3M and 2years spent.  never shipped, never met deadline within factor of 2.
- T: Right the ship, improve quality & velocity.
- A: Layoffs, Descoping, Hires, Revamped all processes
- R: Shipped.  Stable.  Bugs dropped immeasurably > ?3? quarterly.  

#### SITUATION - Late joining "founder" (>2 years) w/ lg equity & turn round mandate
- Burnt $3M - no AI product, SW product was an unusable brick. 
- Great CEO vision and investor relations but dumpster Fires EVERYWHERE in SW team.

#### TASK: Fix Org and Personnel.  ACTIONS: ReOrg. Layoffs/Hiring. HireFlow
- **Re-Org** --> Initially SW and CV leads still reported to CEO.  -->  whole SW org report to me.
- **Two Level Mgt** & **Split VP Eng** into (Dir Eng, PM, AI head)
- Many **layoffs**.  Hired QA team; 
- **Hiring pipe** "interviewed" recruiting firms & hired 2.  built key paid tests & tracking/triage flows.
#### TASK: Ship Product.  ACTIONS: Descope. Freeze/Test. Plan. Track.
- SCOPE: **Excessive Scope** --> 2apps.   (started as 8 separate interfaces being worked on by 4 full time devs and 4 part time devs.)
- FREEZE: **App Stability** Issues --> Feature freeze + Fellowship of the bug + External Testing team to drive bug capture & fixing to 100%
- PLAN:
	- **Low level tracking** in a spreadsheet  --> Jira tracking & sprint planning
	- **Mid-level planning** --> Fully ordered backlog of features complete at two sprints out.
	- No written **High level targets** --> Top roadmap, A-round targets, OKR methodology
	- **OKRs** --> Instituted pervasive use of **OKRs** per department. (with head of sales)
	- **CEO Vision** --> One Page Vision & A-round Deliverables.
- EXECUTE:
	- **Bug tracking** in slack --> Jira + Bug board.   (Fellow ship of the bug)
	- **Poor testing** --> Added field testing + regression testing + fellowship of the bug + Weekly tracking of six KPIs
	- No **Regression testing** --> Tests on major apps; Diversity tests; 
	- Added **Test Automation** --> Circle CI; Unit testing frameworks; key functional tests & slowly started adding coverage
- FORMALIZE:
	- Unified company roadmap
	- Quarterly OKRs, tracked monthly
	- Ad hoc **Dev Ops** --> Push2Prod flow + Cloud Formation

##### detailed actions

- STRATEGIC
	- **Key Strategic "Ingest" Gap** --> Owned solution analysis matrix & tasked ICs under my leads.
	- Multi-sport shift
- CV TEAM
	- **Missed every deliverable** date and every metric target ever set.
	- **Replaced 100% of CV team**:  Vishal, Chris, Sarthi, Michael, (Fong)  with  Greg-Martin-Maxim-Juan-James.
	- Built ML infrastructure -- ML-flow,
	- Created all execution processes:  Jira, CI-testing, Regression, Collaborative API planning 
	- Systematic 

###### No Jira (Low Planning)
Teams had not been doing any ticket tracking and were using spreadsheets
- They had just hired an junior PM as I joined.
- We had her start attending sprint planning meetings, initially she just documented activity using JIRA
- Over time she started prompting for Jira tickets during "planning". but planning was way too in the weeds.
- --> Instituted 1-page sprint over view broken out by person.  This greatly helped one-sprint at a time planning abstraction.


## ANALYTICS FIRE

Founded and grew 45+ person clean energy company.  Our 140+ projects included:
==> Developed, shipped, and managed firmware through cloud infrastructure controlling over $3B in various clean energy assets.  Projects include:

- A computer-vision-based solar-roof-installation design tool for our fortune 1000 customer.
  ==> Became their “largest single driver of new revenue growth” as they transitioned to residential solar.

- Over 3 years and 22 FTE we built majority of Sunpower's tools managing entire residential solar life product lifecycle.
  ==> Transitioned from XXXX in residential solar in 20xx up to YYYY in 20yy, ultimately enabling a complete commercial divestiture and transition in 20ZZ.

~~
Lead and advised 40+ member boutique workforce across **130+ software and data science projects,** including:


- Scaled IoT & Infrastructure > Commercial Clean Energy: Shepherding the company’s greenfielding, build, launch, and operation of complex **software ecosystem** controlling **many millions in clean energy assets worth $3B+**, comprising firmware, dynamically throttled multi-layer networking.
- Commercial Medical Device > Connected Robotic Surgery: Built software and infrastructure enabling video connectivity for a medical device company’s **telerobotically operated surgical unit.**
~~

### __
#### Instant Design

Co-designed and lead automated solar-panel installation designer using deep learning and geometric reasoning over Google satellite image data. 
=> First of a kind sales channel: instant online solar installation quotes

- S: Fortune 1000 client tasked us to build a computer-vision-based solar roof installation designer tool.  Customer gave us limited rope after internal team failed at this. 
- T:	Initial task was to demonstrate "instant design" was possible
- A: Light 1-month R&D cycle + 2-month dirty prototyping
  Won long term contract, built and operated capability until we sold the company
- R: Our solution became “the largest single driver of new revenue growth” according to our client — enabling them to penetrate and scale in a new high-margin market segment.


- ML / Vision-based Solar Installation Designer > Automated Sales Channel: Co-designed technical solution and advised on 6-month research, creation, and **productization of custom geometric reasoning-deep learning-based solution.**
First-of-its-kind sales tool used satellite data to construct consumer roofs and **enabled client to scale its** [**one-touch sales-and-design tool**](https://us.sunpower.com/blog/2019/04/09/home-solar-design-made-easier-google-cloud)—enabling 30-second processing of physical measurements, pricing, and consumer savings, and eliminating human-in-the-loop solar installation design.

#### Sunpower Residential Solar

Directed a 20+ person-year omnibus solution for residential solar (included sales tools, installation marketplaces and workflows, analytics, customer-facing control, etc.)
=> This was “the largest single driver of new revenue growth in 2017 Q3” — enabling a Fortune 1000 client to penetrate and scale in a new high-margin industry.


- S: Sunpower desired a strategic shift from commercial to residential solar
- T: Build suite of infrastructure required for penetration into the consumer solar market.
- A: Over 3 years, and 22 FTE we build majority of their suite of tools managing entire residential solar life cycle, including:
- R: Sunpower transitioned from XXXX residential in 20XX upto YYYY in 20YY, and ultimately divested commercial solar in ZZZZ.


Directed 20+ person-year greenfield-to-operations of a infrastructure omnibus system (sales and installation marketplaces and workflows, analytics, customer-facing interfaces, etc.)

- Omnibus Solution for a New Market > Residential Solar: Directed 20+ person year project for **greenfield-to-operations of a computational infrastructure** (omnibus system), including market-specific workflows, analytics, and customer-facing interfaces.

 _Å as “the largest single driver of new revenue growth in 2017 Q3”—enabling client to penetrate and scale in a high-margin industry._

## AEOLUS

### Summary
Tapped to deliver a two armed eldercare assistive robot.
- Built a 60+ engineer org from scratch with over 40 PhDs across 8 robotics-related disciplines.
- Led 13-month custom HW/SW build for a dexterous robot from scratch.
=> debuted in CES, and raised $16M in follow on investment
=> deployed thru an exclusive relationship with a ¥2,950B Japanese conglomerate
- Identified, negotiated, and awarded a $2M collaboration with University of Vienna.
=> hired two world class PhDs in robotic grasping & collaborated top 10 lab for robotic grasping.

### Aeolus Turn Around
**S**: Cash rich, Short timeline, Unrealistic investors
**T**: Build team, Create Aeolus Robot OS, Deliver CES Prototype
**A**: Build 2-tiered mgt structure, AROS framework, and CES prototype
**R**: Hired 40+ PhDs & 20+ Eng, Raised $16M, Debuted autonomous two-armed robot at CES

Tapped as employee #1 following Martian Robotics acquisition to reimagine the IP as an eldercare robot. Led 13-month "Hail Mary" build of team, robot, and technology to demo prototype at CES 2017, helping secure $16M in follow-on funding. Built 60-member, 8-discipline follow-the-sun HW/SW R&D organization (40+ PhDs, 20+ engineers) remotely distributed across 6 continents.

Bang Start
- S: Cash rich, Short timeline, Unrealistic investors
- T: Build team, Create Aeolus Robot OS, Deliver CES Prototype
- A: Build 2 tiered mgt structure, AROS framework, and CES prototype
- A: Processed over 2,000 applications in 8 months & 
- ==> Hired 40+ PhDs & 20+ Eng
- ==> Raised over $16M ???
- ==> Debuted autonomous two armed robot performing open-ended household tasks at CES

### Notes
JARED
Tapped as employee #1—following Martian Robotics acquisition—to reimagine the IP as an eldercare robot, ultimately deployed through an exclusive relationship with a ¥2,950B Japanese conglomerate with 1,920 rooms.

- [Human-like Elder-care Robot:](https://www.washingtonpost.com/news/innovations/wp/2018/01/11/this-robotic-maid-takes-us-one-step-closer-to-the-jetsons/?noredirect=on) Directed evolution of Martian Robotics IP, creating a dexterous robot prototype capable of manipulating human tools.

- **Led 13-month** **“Hail Mary” build** of team, robot, and technology to [**demo** **prototype at CES 2017**](https://www.youtube.com/watch?v=IzHb6G97tLw)—helping secure **$16M in follow-on funding.**

- **Built 60-member,** 8-discipline follow-the-sun HW / SW **R&D organization** (40+ PhDs, 20+ engineers) remotely distributed across 6 continents.

- **Identified, co-negotiated,** and **awarded a $2M collaboration** with **University of Vienna**—a Top 10 institution in automated robotic grasping in unconstrained (household) environments.

- **Built 3-person recruiting team** and a **scaled 13-step hiring process** to handle 2,000+ applicants, as well as targeted recruiting of university partners (e.g., University of Vienna).
## MARTIAN

### MARTIAN BATH BOT
- S: Started from no robot and no team
- T: Build team and robot for bathroom cleaning
- A: Hired 13 person team, delivered toilet cleaning robot.
- R: Sold company to Aeolus Robotics and became their CTO

## DARPA

#### __
Sold and led over $200M+ in advanced AI and ML programs.
- Machine Reading Program: Aimed $125M at developing fused ML and NLP in mapping unstructured information into structured data
- Bootstrapped Learning Program: Set the stage for ongoing transformation from statistical ML to human-like, multi-paradigm ML—closing the gap between ML and human learning.
- Transfer Learning: Extended ML to incorporate learning from related but distinct tasks. For example, transferring learning to recognize and object into learning to manipulate it.
- Plus 11 more early-stage AI / ML / Cyber R&D projects ($500K to $5M).
=> fueled a field-changing revolution in NLP.
=> IBM Watson system that beat world Jeopardy champion, Ken Jennings, using underlying tech.
=> 1000+ papers and patents.

- Appointed by office director to serve the 9-member Senior Advisory Board
=> Formulating I2O’s cognitive / computational priorities, along with budgetary justification delivered biannually to the U.S. Congress.
=> Hand-picked the director of DARPA as one of two PMs briefing Secretary of Defense, Robert Gates.

#### $200M Portfolio
S: New PM
T: Sell director on new programs; Advance USA in Machine Learning and AI
A: Sold and led over $200M+ in advanced R&D programs in Machine Learning and AI
- Machine Reading Program: Aimed $125M at developing fused ML and NLP in mapping unstructured information into structured data
- Bootstrapped Learning Program: Set the stage for ongoing transformation from statistical ML to human-like, multi-paradigm ML—closing the gap between ML and human learning.
- Transfer Learning: Extended ML to incorporate learning from related but distinct tasks. For example, transferring learning to recognize and object into learning to manipulate it.
- Plus 11 more early-stage AI / ML / Cyber R&D projects ($500K to $5M).
=> fueled a field-changing revolution in NLP.
=> IBM Watson system that beat world Jeopardy champion, Ken Jennings, using underlying tech.
=> 1000+ peer-reviewed papers from more than 500 researchers in AI & ML in world-lead universities & labs.



#### DARPA PM
- S: New PM
- T: Sell director on new programs
- A: Sold and executed 3 DARPA programs
- ==> Funded over $200M in 
- 1000+ peer reviewed papers
- drove big-data enabled text reading
- collaborated on beginnings of generative AI revolution

#### My old one
**DARPA, Program Manager**.  Action: Won and lead $200M+ technical research portfolio.

Outcome:   1000+ peer-reviewed papers from more than 500 researchers in AI & ML in world-lead universities & labs.
Outcome:  Drove the technology eventually spun out as the IBM Watson NLP software suite, and  
            beat top human players in the game of Jeopardy.
### JARED 
**DARPA** (Defense Advanced Research Projects Agency), Washington, D.C………………… Aug 2005 to Oct 2011

_One of the world’s innovation engines. R&D budget: $6B. Funded scientists: ~20,000._

**Program Manager**

**Recruited to imagine, build,** and **evolve pioneering AI and ML at the forefront of innovation**—serving as one

of 130 PMs collectively adjudicating research awards and managing ~250 R&D programs.

• Research Portfolio Vision, Strategy & Prioritization: **Appointed by** **IPTO****’s director** to serve on its 9-

member Senior Advisory Board—formulating IPTO’s **cognitive / computational priorities,** along with

budgetary justification delivered biannually to the U.S. Congress.

_Hand-picked the director of DARPA as one of two PMs briefing Secretary of Defense, Robert Gates._

• Global Advancement in AI & ML: **Owned a $200M+ technical research portfolio** to advance the

machine learning field worldwide—funding and overseeing 100+ scientists. Examples:

**Three large AI / ML innovation programs** _($50M to $125M projects):_

_-_ Machine Reading Program (MRP): Aimed $125M at developing fused ML and NLP in mapping

unstructured information into structured data—**fueling a field-changing revolution in NLP.** _MRP_

_tech is core to the_ **_IBM Watson_** _system that beat world Jeopardy champion, Ken Jennings._

_-_ Bootstrapped Learning Program: Set the stage for **ongoing transformation from statistical ML to**

**human-like, multi-paradigm ML**—closing the gap between ML and human learning.

_-_ Transfer Learning Program: **Extended ML to incorporate learning from related but distinct**

**tasks.** For example, transferring learning to recognize and object into learning to manipulate it.

**Eleven early-stage AI / ML / Cyber programs** _($500K to $5M projects),_ including **Attacking Time** (cyber

program), and **Never-ending Learning** (statistical NLP program).


