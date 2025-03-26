
# [[RES War AI]], [[RES Star]], 


# TODO

- [ ] elevator pitch who you are (ask QL)


- **A manager must also debug projects.** _“Ask the manager to describe a time when she ran a project that was behind schedule, and what she did in that scenario.”_ [page 141] However, careful with what is mentioned in point (1), managers can play the words.
- **Interview for management philosophy.** This might take some people by surprise as it is not a typical question. However, _“If she doesn’t have one at all, that might be a red flag.”_ [page 141] If the interviewee is stuck, nudge them with questions such as how does she stay hands-on vs how does she delegate? How does she understand that something is important vs urgent? What does she think her role as a manager is?

TEXECUTION
Which (inter)

NEXT WEEK
- Ordered pubs.  get photos for authors.
- Reorder to get interesting ones at top.
- Get few 3-5 recommendations: Co-Founder, Theo, Philepe, Dan (Other letters of rec)  might be interested in going back to a larger co, what's it like
	- star player on their team; where is the energy today (in projects) ...  should I

- REWRITE ABOUT - ADD COLOR: More ML in my about section.
- Break it down:  ML Researcher; Entrepreneur   (See what resonates with my contacts)  Recent research, Leading bigger teams, Entrepreneur 

SOON
- ML/AI/IoT Clean Energy
- Background @ top.
- Dump photo w. suit; 
- what are your interests today.   
- Highlight tidbits.  
- update crunchbase photo / angel-list / tweet-company got acquired.
- Focuses on more hands on stuff & mentorship
- causes.


- QUESTIONS FOR ME:  What are your pain points today?


- Featured Link for Instant Design & R-zero

AEOLUS - how many lines of code?
- ARTHREX MGT FAILURE - 
- AEOLUS HW DELIVERY FAILURES
- PREMATURE SCALING (ROS REMOVAL)
- PIECEWISE HW TESTING; DIRECTLY BUILDING ON HW TEAMS OUTPUT

### notes
Interesting Story - 
The five whys, 

TDLR
PhD IC into first time managers.
Can't hire, how do solve.
Remove redundancy
Amazon Principles


### --- Favorite Manager: Chuck Morefield ---
Nick?? Chuck?? - He taught me how to manage & 
Peter WMR - how to think about my career
Chuck Morefield - He brought challenging problems to me
### --- Qualities of a good leader ---
Emotive & Incisive



### --- ABOUT US ---
- Name of our CEO
- Why is our company interesting to you
#### --- What question have I not asked you ---




### DUST BIN
#### WHY LEAVE JOB
- PASSION PROJECTS - What to be more Hands on again technically ...
- INDUSTRY HAS CHANGED - Pulling SW&AI project leadership in house
- LITTLE VALUE IN DOUBLING DOWN - 





# STORIES: TECHNICAL
## AF - Instant Design

#### Instant Design Story

Path we took:
	1. Non-ML-based Edge extraction
	2. Hand-built energy-based edge cleanup
	3. NN-based obstruction detection
	4. Combine NN & Geo edge detection


Level 1: **Edge Extraction**  -- (Based on finding flats.  Leverages accuracy from plane fitting large region)
- find flat planes
- glue together
- derive edges

Level 2:**Energy based cleanup**
- remove small planes
- remove super sharp corners
- replace line pairs that are nearly 180-degrees
- (sides of roof were not symmetric; fixed this for sales)

Level 3:**NN MODELS** - pixel level models (RGB+depth)
	- Tree detection (?yard tree?)
	- obstruction detection
	- pixel based-edge detection   (weakest model, needed help from the others)

Level 4: **NN-models + Geo**
- Perf improvements & combine with Geo edge detection
- Do batch normalization
- Use residual connection
- grab resnet-50.
- Started with plain units.
- We always trained from scratch 

- do contrast learning (never did that to do pretraining)
- 2% delta from best to worse model, so we needed to care.  (semantics segmentation accracy at the pixel level)  
	- was 97 - 98% at pixel level


SUMMARY OUTCOME: Worked pretty well - but still not enough to completely eliminate the human planner
Ultimate failures came from mis labelling one part of roof and failed from there (e.g. mis-labelled one part... and it kills )

- FUTURE IDEA:  Do object detect, instead of  (use mask CNN to do it)
	- Grabbed the RTR from facebook


OTHER IDEAS:
- RoofGan paper: [https://arxiv.org/abs/2012.09340](https://arxiv.org/abs/2012.09340)  code: [https://github.com/yi-ming-qian/roofgan](https://github.com/yi-ming-qian/roofgan)
- Object detection:  DETR: [https://github.com/facebookresearch/detr](https://github.com/facebookresearch/detr)
- RTF frome facebook

- Hand written  CV
	- derivative of DSM data, flat planes  (Grz Bartchek; MIT)
	- hand written CV geo algorithms
	- apply RGB data (Energy minimization)
		- corners should meet at non-nearly straight angles (energy constraints)
	- NN for detecting obstructions
		- then idea lets swap edges from NN into the existing pipeline for 


# STORIES:  MISC
## MISC STORIES

### TEAM GROWTH
	-  [Theo](https://www.linkedin.com/in/theodore-enns-1329633a/)  - Polished Gem - Amazon SDM who led the Astro project
	-  [Felipe](https://www.linkedin.com/in/theodore-enns-1329633a/)  - Student -> mgr of 13 person team -> Robotics Engineer @ Amazon
	-  [Sahil](https://www.linkedin.com/in/theodore-enns-1329633a/)  -  Student to Senior Roboticist @ Agility & Osario 
	- [Rogerio Fonteles](https://www.linkedin.com/in/rogerio-fonteles-074169b0/)  Student -> Manager? -> Senior Robotics Engineer --> Head of Engineering @ Moley Robotics
	- [Georgios](https://www.linkedin.com/in/georgios-floros-76a3067/) - Fragile researcher --> Effective Manager


	- [Attilla Aros](https://www.linkedin.com/in/aaros/)  - Hacker -> Strongest Tech Lead --> Crypto Founder
	- Senad - 
	- [Krzysiek Herod](https://www.linkedin.com/in/krzysiek-herod/) - Hacker -> ML 'founder'
	- [Ben Jacobs](https://www.linkedin.com/in/bencjacobs/) - Web dev -> Multi-Team Project Management -> Eng Mgr @ RZ  &  Founded his own company while @ AF


##### ALEX - "Stole" Alex from Apple @ 1/3 pay...  and he did not really value equity... he valued experience.
	- Time-stamping buffering multi-tiered architecture.
	- Git-driven Hardware unit-testing rig.
	- Worked hard & long hours on task he was motivated by.
	- Managed him via a dual backlog
##### THEO - Polished Gem   << GROWING TALENT >>
	- Asberger, Physics guy hacking robots @ night, writing test scripts
	- Grew thru MGT -> Aeolus 
	- Tone Deaf, insecure, very teachable Task Manager.  (lots of post-hoc repair)
	- Now accomplished Lab 126 - SDM - lead Amazon Astro project.

### CES DEMO - 13 Months from a team of one

### MARTIAN ARDUINO BOT - Just do it
	- CEO came w/ a broken demo
	- pulled me into a week-long hack fest to build the demo system

### LANDING TALENT - Pipeline of very best talent in the world
	- SCALED HIRING PIPE 
	- RELATION W U VIENNA / SW LAB
	- LICENCE SKELETAL TRACKING SW 

### DARPA MENTORING - XXX YYY ZZZ at DARPA

	Today we cannot XXX, but YYY is a promising approach, and if it succeeds will mean ZZZ.
	XXX a very basic thing that cannot be done today
	YYY an approach that might work for XXX
	ZZZ some transformative benefit that having XXX will achieve

	_
### --- IBM: Few shot learning ---

	_
### --- IBM: Personal Wizards Algorithm - Interleaving edits and learning ---

	_
### --- IBM: Email Mining - Overcoming privacy concerns ---

	_


## UNUSUAL QUESTIONS
	STAR: Situation, Task -> Action -> Result

### BRAINTEASERS
	1.  If you suddenly gained the ability to time travel, what’s the first thing you’d do?
	2.  If you could get rid of any U.S. state, which would you choose and why?
	3.  Which is more important: creativity or efficiency?
	4.  Is it better to be good and on time or perfect and late with your work?
	5.  How many times per day do a clock’s hands overlap?
	6.  How many stacked pennies would equal the height of the Empire State Building?

### TEACH - Teach me something fascinating I don't know.
		Patterson Commitments.  Merkel Tree.  CNN.
### PITCH - Elevator pitch for Dream client

### TOMBSTONE - What do you want on your tombstone & how does this job help
	- what are you doing now to make it happen.
# STORIES: BY YEAR

## SCHOOL YEARS 
	- Tons of personal hacking projects; its all I did
	- Second place state science fair

## COLLEGE YEARS
	- Math club president; Putnam Math Exams
	- Wes Ragland; attendant for mute quadriplegic; brother paid me to build specialized editor environment for him
	- Student of the year in CS and Math

## GRAD YEARS

## IBM RESEARCH YEARS
### Watch ME Read - 
	- INTERPERSONAL: In classroom, developing & testing solution
### Personal Wizards - Pitched, Won, and Co-Lead 12 PY Effort To Develop ...
	- ACCOMPLISHMENT:  Conceived -> Won -> Lead -> Technically Innovated -> Productized ... Shipped
### ?20? Patents - Law suit against Air BnB 

## DARPA YEARS
### Andrew Ng
 
### Machine Reading Program
	- ACCOMPLISHMENT:  Conceived, Won, Led, Largest DARPA effort in IPTO office at the time.  Outcome IBM Watson / Jeopardy
#### NIST TREC-TAC - International Competition
	- Creation of standards track and competition.  
	- OWNED IT:  Initiated it, Funded it, hired the teams to develop it, and was responsible for its visions and approved details of its implementation.
### BL & TL

## PAY BY GROUP YEARS

## JOTTR YEARS

## MARTIAN YEARS
### PROTOTYPE - Late-night Proto Building
	- Really wanted me as a co-founder
	- Inverse Kinematics by hand on an Arduino powered bath cleaning robot
### - HIRING - Hiring Martian Team
### PEOPLE: Alex, Theo, ?Brandon, 
##### ALEX - "Stole" Alex from Apple @ 1/3 pay...  and he did not really value equity... he valued experience.
	- Time-stamping buffering multi-tiered architecture.
	- Git-driven Hardware unit-testing rig.
	- Worked hard & long hours on task he was motivated by.
	- Managed him via a dual backlog
##### THEO - Polished Gem   << GROWING TALENT >>
	- Asberger, Physics guy hacking robots @ night, writing test scripts
	- Grew thru MGT -> Aeolus 
	- Tone Deaf, insecure, very teachable Task Manager.  (lots of post-hoc repair)
	- Now accomplished Lab 126 - SDM - lead Amazon Astro project.
## AEOLUS YEARS
### Hiring
#### . Hiring Pipe
#### . Vienna Research Project
#### Management
#### . Swim Lane Architecture
#### . Remote Robotics

### Management
#### . Remote Work
	- 3-shifts.  24/7 lab tech support.  Fly-in
#### . Siloed & Interoperable work streams
### FAILURES
#### . "Completed" work failures
	- Frequently we were running into failures in work that was claimed to be completed & tested.
	- Obvious approach is to agree on formal tests, but way to costly for our velocity and work piece size.
	  (is appropriate for 2 month integration tests, and maybe 3 week sprint tests, but not back log items)
	- PUBLISHED 10 LEVELS of test & validation.  Documentation of validation included: the level used, an explanation, and a video.

	--- Aeolus: 'done' things were never really ---
	- Team members said it was done; Even fantisized it was
		but they had only tested their part
		but only run in simulation
		but only ran it in their context
		but it only ran once and is not stable

	==> Needed to carefully construct test, that would really show it.	
	==> Cell phone video	
	_
#### . MIT Work - Delivered Work Failures
	- Tested vision system w/ high accuracies was non-usable & needed to go back to drawing board.
	- SOLN:  Work delivered in sprint X, tested on robot in sprint X or X+1.  (delivery owned by both teams)
#### . Integration failures
### PEOPLE - Alexey, Georgios, ?Haddick

## ANALYTICS FIRE YEARS
### BASIL HEALTH - Basil Health Non-Payment Scam
### INSTANT DESIGN - Cutting edge Deep Learning
### PROJECTS
	- OASIS - soup to nuts build of a new product line
	- ARTHREX - Robotic surgical equipment communications suite
	- I-CRACKED - B2B Marketplace
	- SUN POWER - Solar Control Systems
	- ENPHASE - Inverters
	- MOXION - Batteries
### TOPIC: PERSONALITIES - Jess, Derrick
### TOPIC: TEAM - Cohesion / Retention / Quality - Offsite, Bonuses, Friday talks




# STORIES:   WHOLISTIC
## ONE-OFF QUESTIONS


### SELF SUMMARY - Tell me about yourself           ?????       < JY
	1. Highlight skills, qualities, and attributes that match role
	2. One example where you have succeeded in previous job
	3. Explain how you understand your perf links to AMZ success
	- Tell Story/Be memorable

	{{what order?}}

	- HANDS ON NERD - Aeolus started w/ hacking inverse kinematics on an arduino.  Todo list --> Notester & OBS plug-in
	- HAIR ON FIRE:  30min w. DARPA dir. (Looking for hair on fire)      VISION?? (Pitching $150M MR ML-NLP)
		- VERBALLY FAST - Represented DARPA to Sec Def Bill Gates
		- ENTHUSIASTIC - Selected from over 50 TA to lead separate campus.  (articulate, ).   (AF low turn over??)
	- PRODUCT ORIENTED - Martian bath-bot; Aeolus Elder-Robot; AF solar designer; DARPA-jeopardizer 
	- LEADER (team 20-50) - management philosophy
		- LOYAL - Theo, JoeA, Nick, Felipe
		- . . .
#### .
#### . HAIR ON FIRE
	- Fits DARPA Efforts & Instant Design AT AF.
	- Example from AEOLUS:  Zero employees, zero HW, CES-2018 booth is already booked 13 months later.
		- Tie in the tech stack
		- Was the defacto architect 
	. Andrew NG ?
#### . Robert Gates 
#### . XXX  ENTHUSIASTIC
	- I never knew someone could be that excited about computer science
		- Selected from 50 TAs to lead/run our external OSU labs and extension classes at Bellcore
		- One of two PMs at DARPA to directly present to Director of defense Gates his visit.
		- Fast on feet, technical deep, but mostly a bit over the top enthused about the possibilities of tech.


### - WHY AMAZON / WHY XXX
	1. talk about the research you carried out
	2. Leadership principles
	3. Amz products
	4. A place I can grow and develop
	- Why do you want to work here? 

	????    

	WHY MICROSOFT
	- Like the cutulre, haerd good things, and here is why...

### - WHY WOULD YOU BE A GREAT HIRE     ????
	- Strong executor from day one.  Positive influence on team dynamics.  Learn very fast.     Been able to execute quickly ; plan
	- In 6 months I will be better than anyone you could hire today at whatever I am tasked to do.
	- Folks will want to work with/for me.
	- Grown orgs and people w/ great outcomes
		- Hold on to team during covid
		- Career growth


### - Why would you be a good leader?
	Constellation: Foment Action, People follow, Technically Strong, Cut to core of issue, 


### - DO YOU HAVE ANY FINAL QUESTIONS
	- What is the next step?


### - WHY WOULD YOU BE A GREAT FIT?
	- Contagiously enthusiastic; get great joy from owning&delivering something technical
	- Love cool tech but focused on effective motion   (balance company need & team joy)

	- I excel at what others consider boring.  I enjoy them.  My teams find me complementary


### - ANY TECHNICAL QUESTION YOU DON'T KNOW
	- I don't know, but I can venture to guess...


### - TELL ME A TIME WHEN YOU OVERCAME ...
	- Always close w/ the reactions of other people to your action


### - WORDS TO USE IN AN INTERVIEW 
	- My research indicates ...
	- BEST ANSWER - talk about your customer's customer:  what do you like to cook?  your fav thing.  I enjoy cooking & will do well


### - BIGGEST ACCOMPLISHMENTS   ?????
	What are some of your most significant accomplishments?
	What are the most successful projects that you’ve worked on (and what results did the project achieve)?
	In what ways have you had a positive impact?
	Most impressive thing you have built or achieved

#### . Aeolus OS 
	- OS for robot
	- 

	 #### . Most impressive thing built.
		 - the IBM Jeopardy Playing Computer Watson.  (didn't build it, but I conceived of and executed
		   the 150+ million DARPA program mapping unstructured text onto fixed ontologies -- which is the core 
		   tech the IBM used to beat Jennings)

#### . Coded By Me
##### -- Personal Wizards -- A learning system that induces executable scripts with loops, branches, and variables 
		based on "watching" a desktop user (by trapping & abstracting messages at Windows event layer).  
		This is now part of an IBM Rational product.
##### -- Notester -- Note taking app

##### -- Full Screen LISP Debugger - 
	- A full screen LISP debugger that allows user to both fwd and backwards in time and up and down in abstraction.


##### .
### - WHY ME - Why should we hire you?
	- ENTHUSIASTIC - adding joy and velocity to team
	- TECHNICAL - depth, breadth, vision, creativity
	- STRONG MANAGER - I like, people like me, am organized


## ONE-OFF TOPICS


### SALARY                                                         ????    < JY
	1.  [Can you discuss your salary history?](https://www.indeed.com/career-advice/pay-salary/providing-salary-history)
	2.  How much do you expect to be earning in five years? 
	    Read more:  [How To Talk About Salary in a Job Interview](https://www.indeed.com/career-advice/interviewing/how-to-talk-about-salary-in-a-job-interview)

	"My recent salaries have ranged from half a million to just over a million per year.  So my expectation for this job will really depend on my role and level."


	“I’m looking for a competitive offer that includes benefits and other kinds of compensation, but I’d like to know more about the specifics of what this job requires first."

### STRENGTHS
	1.  [What are some positive things your last boss would say about you?](https://www.indeed.com/career-advice/interviewing/how-would-your-boss-describe-you)
	2.  [What differentiates you from our other candidates?](https://www.indeed.com/career-advice/interviewing/interview-question-what-makes-you-unique)


	- ML Researcher, Eng. Mgr, Entrepreneur

#### - TECHNICAL - depth, breadth, vision, creativity

#### -  FAST - VERBALLY/INTELLECTUALLY FAST ON FEET
	- DARPA PROGRAMS - 
	- AF?

#### - MGR - Strong Manager
	- PEOPLE PHILIC: I like people & growing people;  people like me, trust my integrity & see I have their interests at heart.
	- ORGANIZED - SOLVER - I am organized; I am a problem solver
	- INSIGHTFUL -  I am quick to identify heart of a problem, and quick/creative in proposing solutions
	- CONFIDENT -  Personal confidence in my abilities makes it fun and natural to revel in the strengths of others -- not threatened by those better than me
	- PERCEPTIVE - Very good at understanding misunderstandings & perspectives of others.
		- Strong (but not superb) at anticipating / detecting problems before they are problems. (very perceptive, but too fast & too optimistic) 



### MY WEAKNESSES        ???   ++++++ 
	- Short, simple weakness w/ emphasis on improvement

#### (TOO) FAST START 
	- Fast on feet problem solver; so I will just jump right to the solution and propose & execute it on the spot.
	- Can be disconcerting to the team
	- Can lead to loss of buy in
	- Can lead to poorly planned action
	- Can lead to thrashing
	- Can lead loss of ownership for (middle skill levels)
	==> Measure is

### ??? 


### no!  DECISIVENESS - Core value of a leader is to DECIDE and MOVE
	- Long process of improvement
		1. Shoot from the hip in low consequence or easily reversed decisions.
		2. Explicitly state appropriate amount of analysis for a choice, then follow thru.
		3. Aggressively gather key info on time sensitive decisions.


### ABOUT THEM
	6.  What is the name of our CEO?
	8.  [What do you know about our company?](https://www.indeed.com/career-advice/interviewing/what-do-you-know-about-our-company)
	11.  [Why is our company interesting to you?](https://www.indeed.com/career-advice/interviewing/interview-question-why-do-you-want-to-work-here)
	13.  Who are our competitors?
	17.  What do you know about our industry?
	1.  [Why are you the right person for this job?](https://www.indeed.com/career-advice/interviewing/what-make-you-a-good-candidate-for-this-position)

	- COMPETITORS - Who are our competitors?
	- COMPANY - What do you know about our company?
	- CEO - What is the name of our CEO?
	- OVER QUALIFIED - Are you over qualified for this role?
	- CHANGE - Why are you changing careers?
	- INTERESTING - What makes our company interesting to you?
	- FIRST 90 - What do you want to accomplish in first 90 days?
		- Deliver end-to-end result
		- Demonstrate Readiness: Technical, Leadership, Creativity, Interpersonal, Organizational
## . 

## .
# QUESTIONS: BIG LIST ([from indeed](https://www.indeed.com/career-advice/interviewing/top-interview-questions-and-answers?from=careeradvice-US))


## = BASIC 

### ENDING OF INTERVIEW -


### OTHER QUESTIONS - What questions haven’t I asked you?    <<  ??? 
{{AIM towards a thing I want to highlight, which they missed}}
- EXPERIENCE - HandsOn(Bizzy,Obs); Innovation Leadership(Aeolus/AF-Team); Turnaround(AeolusOctober)
- EXCITEMENT - Indicate excitement - What will I do when I join, and imagine super performance... what would I have accomplished in my first year?
- THEM - Questions related to their group.  (my experience related to them)

"we didn't talk as much about my background in XXX, YYY, ZZZ"
- DAN PILARS: Leadership; Hands On; Researcher; Entrepreneur.   (give them one last story about me)


### LEADER QUALITIES - What qualities make a good leader?   
- MANY!
- LEADING:  Integrity, High-EQ, Dedicated, Infectiousness, Ownership-Taking
- EXECUTING:  Informed Decisiveness, Results oriented / Efficient, Organized, Context-Switcher, 


### WHY LEAVE JOB - [Why are you changing careers?](https://www.indeed.com/career-advice/interviewing/how-to-explain-your-reasons-for-leaving-a-job)
- GREAT EXIT - Former client raised $170M for Covid tech company.  
  Great opportunity for our team and a great exit for me to become more hands on again.
- HANDS ON - Wanted to be more hands on again technically


### WALK RESUME - Can you walk us through your resume?
   (Focus on hands on stuff)
- PhD ML from UIUC * IBM Research 22 patents, Faculty & Columbia, ML & PBD Research.
- DARPA - Scaled Science.  $200M R&D.  Spearhead statistical-NLP.
- Startups - Cleantech ML-IoT, Robotics


### FAV MGR - Who was your favorite manager and why?
- Nick Allen -   (Stakeholder) 
- Chuck Morefield - Believed in me & Trusted me: IPTO SAG, present to Sec Def Gates, mentored new PMs,  Helped draft office vision
- Jacob Ukelson - Reached down into org, to mentor me & aim me.  Won 12 PY award for Personal Wizards (Augmentation Based Learning)


### GREATEST ACHIEVEMENT - [What is your greatest personal achievement?](https://www.indeed.com/career-advice/interviewing/what-is-your-greatest-accomplishment)
- SPEARHEADED STATISTICAL NLP - (DARPA MR)
- AUGMENTATION BASED LEARNING - 
- ELDERCARE BOT - ????


### BEST ACHIEVEMENT - [What is your proudest achievement?](https://www.indeed.com/career-advice/interviewing/what-is-your-greatest-accomplishment)        <<<<<<<  JY
- Elder Bot Architecture
- Building two world class orgs
- Stats & NLP revolution


### FUTURE -  [Where do you see yourself in 10 years?](https://www.indeed.com/career-advice/interviewing/where-do-you-see-yourself-in-10-years)
- HANDS ON on a project w/ GLOBAL IMPACT.    AIM: 50-150 people.
#### .
- LEGACY - In 5, 10 years I will have completed 1 or 2 more "legacy" projects.
- PHILANTHROPIC - My focus will be on purely philanthropic outcomes (inside or outside this company)  E.g.
	- The Forum, EduCorp, 


#### .



## = BEHAVIORAL

### MY GOALS - What do you want to accomplish in the first 90 days of this job?   ++++   JY
1.   What do you want to accomplish in the first 30 days of this job?   (subset of items below)
#### MANAGE 
- EARLY WINS - Identify quick wins to gain credibility (w/ team, w/ peers, w/ my leadership)
- LISTEN!! - I get double credit if any actions taken are tied to their input.
	- FACE2FACE meetings with, reports, management, peers, and stake holders.
		- PEERS - Alignment with relevant peers--at DARPA as the budgets grew into double and triple digit millions the number of relevant peers grew.
	- UNDERSTAND:  Makes them tick; Burning issues; What can I do for them; What can they do for me.
	- DO SOMETHING FOR INDIVIDUALS -  
	- GET ONE or TWO REAL MENTORS IN COMPANY -  (ask HR for 2-3 who are willing to have a single meeting.)
- KICK TIRES ON KEY DELIVERABLES - Any three alarm fires?
	- FIX TACTICAL FIRES - 
- PATTERN - Establish an effective interaction pattern w/ my direct reports
#### PLAN
- ESTABLISH VISION & LEADERSHIP CREDIBILITY IN MEETINGS - 
- ROADMAP - Identify & get buy in on my first roadmap
#### TABLE STAKES
- LEARN THE CORP TOOLS - (so I can be hands on)
- DELIVER VISIBLE RESULT FOR THE ORG -  prove I am a solid 
- TECHNICAL CONTRIBUTION - Find time efficient technical contribution



#### .

### PREVIOUS BOSS - What do you really think about your previous boss?
- NICK - Trust him w/ my life.  Learned so much about leading people.  Not strong verbo-technically, or fast on feet.
- ALEX - Gifted salesman; Gap in commercialization and driving R&D org; - flash in pan abilities as a deep tech product leader
?woman boss or others 


### MOST REWARDING - What has been the most rewarding experience of your career thus far?   ++++
- FORGING AEOLUS TEAM UNDER FIRE - Growing my direct reports into leaders
- THE AUGMENTATION BASED LEARNING ALG - 
- BOOTSTRAPPED LEARNING - Forged early community thinking about non-traditional ML.


### DONE BETTER - Do you think you could have done better in your last job?    ++++ 
- AEOLUS - 
	- *strategy*:  Get away from cost-savings mentality;  Forged product roadmap (Intelligence w/ limited mobility in assisted living facility)    
		- Adjusted org towards R&D.
	- *technical*:  Ripped ROS out too early.
- AF - Built a product; Pivoted into a targeted value proposition.  (e. g.  Data cleaning specialists; SOC compliance; Open ADR enablement)
- DARPA - BL program's testing platform did not live on like MR's program did.  (Should be a major driver today, but is not.)
- AS MANAGER - 
	- Fired faster and more decisively at AF.



## = ABOUT ME

### UNCOMFORTABLE - What makes you uncomfortable? 
- BRO / SPORTS  TALK - Grew up a nerd.  Never watched sports


### BEST ENV - [What is your ideal working environment?](https://www.indeed.com/career-advice/career-development/ideal-work-environment)  {PER COMPANY}
- WIN-WIN w/ BLUNT ASSESSMENTS - The mood is win-win, not zero-sum.  Still blunt feedback appreciated, given, and received.
- INITIATIVE - Rewards initiative.  (not a keep your head down kind of place)
- ACTION & ASYNCH COMMS not POLITICS or LONG/INEFFICIENT MEETINGS - If you keep focused on outcome you get rewarded 
- FOCUS - Need focus time.  (can get it w/ headphones)
- TEAM - Common goal, common celebrations, common stress
.
"In my research of your company I found YYY, which matches my ZZZ well"   <<<<
GOOGLE:  "Those I know at Google all report that personal conviction matters a lot.  If you see a vision, you will have room to drive it.  This is perfect for me as I am really galvanized when I see a chance to make a difference through following some novel innovation."
~~
GOOGLE: "Impact the world @ scale."   Get Granular about parts of google that I would impact.


### COMMON VIEW DISAGREED - What commonly accepted view do you disagree with and why?
- **AGI** - I think AGI is far closer at hand than is commonly believed inside or outside the field.              
  (tie in disagree; know how to lead; 


### MORNING PERSON - Are you a morning person?
- YES - but this is really by choice.  I am just so much more focused/productive in the morning.


### OTHERS SEE ME - How would a good friend describe you?          <<<<<<<<<<<<      ??????
- **Gregarious** - Dean - "I've seen Dan strike up a conversation with a random dog on the street" - Last to leave a party.
- **Nerdy** - Uber nerdy - Nina (nerdy reseracher @AMZ) - likes to call me out...  "you are such a nerd"
- **Enthusiastic/Energetic** - Student evaluation I got from a non-major -- I never knew anyone could be that excited about Comp Sci
   	 -- it was a backhanded compliment.

	- Andrew Ng.   Palm.  
	- Teaching CS at OSU - I remember one student said.
	- AEOLUS - 
	- AF -  


### LEADER/FOLLOWER - Are you more of a leader or a follower?     ++++
- LEADER - Quick on my feet, articulate, quick with a plan, others tend to gravitate towards me.
- BUT HAPPY TO FOLLOW TOO - I love the feeling of an effectively run hierarchy.
	- Attentive listening from below & Decisive action from above.  Deciding w/ tempo almost always better than 100% correctness.
	- And don't like contention over leadership - if you lead it, then you own it for better and worse.
- CONSENSUS - When I am leading, I look for consensus when I can get it w/o significant loss of time or efficiency.
	- e. g.  spend a little extra effort to validate contentious choice.  Do a bit extra to also satisfy a reasonable dissenting voice.


### MISSION - [Do you have a personal mission statement?](https://www.indeed.com/career-advice/career-development/personal-mission-statement-examples) 
"To use my infectious energy, and technical strength in ways that have significant and positive impact"
"To inspire others to achieve great things, to make a real difference."  


### SELF LIKE - What do you like most about yourself?
- TECHNICAL STRENGTH/DIVERSITY/CREATIVITY - Ability to imagine innovative approaches to across a wide range of technical areas.
- INFECTIOUS POSITIVE ENERGY - Can-do optimism resonates often


### TENURE - How long do you expect to work for this company?
- 6+ years.  That has historically been my shortest tenure in one place.  
- I never go to a place I plan to leave.  Historically it has been some bolt from out of the blue offering a transformative opportunity.  


### STAYING ORGANIZED - [How do you keep yourself organized?](https://www.indeed.com/career-advice/interviewing/how-do-you-stay-organized-interview-question)
- USING PLAIN TEXT - LISTS / DOCS
	- Created 'Notester' / now 'Obsidian' environment.
	- Use 'Anchor' docs for efficient context switching over extended time
- TEAM - Written processes - 


### TRAITS - What character traits would your friends use to describe you?     ++++ ??????
- NERD!!!
- EXTROVERTED - 
- CONCERNED FOR THE LOST / KIND/SINCERE/MIDWESTERN???? - 
- TECHNO-OPTIMIST - 


### MISSING TRAITS - What are three skills or traits you wish you had?                       <<<<<<   ????

- CONCISENESS

- HI-EXPECTATION & HI-VELOCITY - but can wear self and folks down.  
	- PATIENCE & BALANCE - next level performance is to have those expectations with less stress generation.
		- TODAY - much much better, but I still know others who are EVEN better, so it is an area of focus for me

- SPORTS-LIKING - Good for health and mentality


- SOCIAL INSTIGATOR

- LESS INTENSE - 
	- Understand flow and tempo of R&D org and when to switch into emergency.
		- Do it w/o always flipping into higher intensity ... burnout.  
	- Naturally easy going, but less intense under fire.  (ideally all emotion)  I am working to internalize less
		- Multi-year projects.

  
- ENTREPRENEUR STARTUP EXECUTIVE - Work best in "fire hose" roles with where rapid response is required.  Less excited and less valueable in slower moving contents where politics is key.


### PERFECT COMPANY - [Describe your perfect company.](https://www.indeed.com/career-advice/career-development/ideal-work-environment)
- THEIR COMPANY!   (Like Big Co; )
- SCOPE & MANDATE - An org with resources, scope, and mandate to change the world.
- TEAM-ORIENTED & COLLABORATIVE with DIRECT FEEDBACK / FOCUED-ON-OUTCOME - 


### ALONE/TEAM - [Do you prefer to work alone or on a team?](https://www.indeed.com/career-advice/interviewing/do-you-prefer-working-alone-or-in-a-group)    ++++
- WORKING IN A TEAM - If I had to choose, it would be as a team.  I love leading & building big
- BUT HEADS DOWN IS FUN&FAST - But heads down cranking out a technical idea, or exposition great.  I prefer that alone.

**MY NATURE**
- *FAST START* - As soon as I hear of a problem, I want to lay out its solution, and then refine over time.   Framing & bulleted lists as thinking
- *ITERATIVE REFINEMENT* - I am very comfortable with progressive refinement and rapid switches in thinking and approach as new info is available.  
- *PROPOSE -> DEBATE -> CONSENSUS* - Love blunt debate and fast shifts.

**MY DISCIPLINE**
- GATHER - Need to push myself to pause & consider optimal approach given larger org.  
   	Then gather inputs in more methodical way.  (My natural approach has startup-like benefits, but needs methodical backbone added)
- PROGRESSIVE & LAYERED REFINEMENT - Is often theoretically optimal, but real orgs often have larger switching costs than one expects.
  Iterative refinement must be 'hidden' in very strategic ways.  
  (bring different stakeholders into discussion at the right times, and in layered fashion; consider their ability to back-flip.)
  - I am good at this, but it takes focus and discipline.
    ==>  SOW writing at AF.  2-month goal setting, sprint and API decisions at Aeolus.


### IMPROVE - How do you want to improve yourself in the upcoming year?
- ML HACKING -  (talk about specific project; get granular)
- HEART HEALTH - 
- SIDE PROJS - 

- LEADERSHIP SKILLS
	- Book learning & mentoring & doing it
- EXECUTION LEADERSHIP
	- Book learning & mentoring & doing it
- DEEP LEARNING
	- Course, doing on the side


### HEROS - Who are your heroes?
- BILL GATES - 
- WARREN BUFFETT - 


### CHILDHOOD - What is your favorite memory from childhood?
- SWIM & DIVE TEAM - 
- STATE SCIENCE FAIR - Measured light in inches
- PRANKS ON FAMILY - Bugging the house, rubber band booby trap, sink spray, Electrified heating ducts


### FAV WEBSITE - What is your favorite website?
- GOOGLE -  even my son will say  "Daddy ask the Google"   


### SATISFYING JOB - When were you most satisfied in a previous job?
- AEOLUS - Hands on building team, hands on building architecture, hands on building a solution


### BEST BOOK - What’s the last book you read?
- A management book?     First-90-days                   <<<<<<< Good ML or Biz or anything where it obviously enriching

- Ghost Boy - 
- How to not Die - 
- Ciuxin Liu's Three Body Problem    <<<< needs story about WHY it is interesting or enriching


### BEST JOB - What is the best job you ever had?
- AEOLUS - Fast paced.  Hands on.  Tangible Output.  Org & new leader growth.  Dedicated to supporting eldercare.
- DARPA - National scope.  Strongest minds on the planet.  Tremendous leverage.


### FEAR - What is your greatest fear?
- THINK ABILITY - Loosing my ability to think.


### LOTTERY - If you won a $10 million lottery, would you still work?
- YES, BUT NOT PAID WORK.  I have several non-profit startups that I would found and execute.


### LAST PROJECT - What was the last project you led and what was the outcome?
- AEOLUS Robot - Built a prototype debuted in CES 2018, and used to raise $16M in funding.
- AF INSTANT DESIGN - New Kind of sales channel - First instant, self-serve solar installation estimator that automatically constructed detailed installation plans w/ costs, energy estimates.


### HOURS WORKED - How many hours per week do you normally work?
- VARYIES WIDELY
	- At least 50 in most jobs.
	- Many many more hours (80-90) in crunch time in a startup.
	- Strive to be organized enough that my baseline is not much above 50.  (I loose effectiveness over time at that level)


### TAKE WORK HOME - Do you ever take your work home with you?
- SURE!  - If the work needs to get done, then it needs to get done.
- BUT - Something is wrong if this is a daily occurrence (unless it is a timezone thing, that is ok)


### IMPT ASPECTS - [What three things are most important to you in your job?](https://www.indeed.com/career-advice/interviewing/what-is-most-important-to-you-in-your-next-position)
- INNOVATIVE PRODUCT -  Some concreted output requiring real innovation.
- CONSTRUCTIVE / WIN-WIN & DIRECT - 
- ???   CHALLENGING?  FAST-PACED?

=====

### NEGATIVE - What is one negative thing your last boss might say about you?   ???????
- TOO MANY IDEAS - Nick
- TOO CONSERVATIVE - Alex Huang


### MISS - What will you miss about your previous job?      +++++     ?????
- EASY!  The people.   (for AF)
- The scope  (for Aeolus)


### CAREER IMPACT - Who has impacted you most in your career?
- MIKE KLEMBARA  -  Loved math for the intrinsic value of it.
- JACOB UKELSON  -  Saw an under-utilized mind; cultivated a "think bigger" mindset.


### LEAST FAVORITE ABOUT ME - What is your least favorite thing about yourself?              ??????
- TOO WORK FOCUSED - 


### BIGGEST REGRET - What is your biggest regret and why?
- NOT TRAVELING - My college summers were frittered.  Too goal oriented in early grad school.


### PET PEEVES - What are your coworker pet peeves?      ?????? 
- NICK - Leaves his phone on SILENT!
- NICK - Gets tongue tied in meetings.  
- ALEX - Asian deference hierarchy.
- MARIA - Does minimal effort on task.  (not lazy, just unimaginative) 
- NICK - could get lost ANYWHERE (I am pretty bad too).  


### CS MAJOR - Why did you choose your major?
- IT CHOSE ME!
	- At two the only gift I wanted was an extension cord.
	- At 5 I had a little work bench, and a spiral notebook with little schematics of a remote control radio, and relay made from a wire wrapped nail.
	- At 15 I stayed up all night at a friends house who had a computer.


### CORP SIZE - What is your ideal company size?  (ANSWER:  their size is perfect!)
- SIZE -  <100 or MASSIVE
	- Under 100 with me as a founder, I am charting a course on a novel idea of my making.   --or-- 
	- Massive - Companies with the depth require to push the envelope and to be at the cutting edge of humanities ability


### BOOK - What is a book that everyone needs to read and why?
- Stephen COVEY, Seven Habits of Highly Successful People - 
- HIGH OUTPUT MGT, by Andy Grove - Trove of info for any mgr beyond first line


### ALONE or TEAM - Do you prefer working alone or in a team environment?
- TEAM - I generally prefer the increased scope of impact that usually accompanies team work.
- THINKING - But like producing artifacts in a focused solo manner.


### ADAPT - Do you find it difficult to adapt to new situations?     ????             <<<<<<<<<<<
- NOT IN WORK SITUATIONS - Very wide ranging experience; very fast on my feet;  can figure enough to be effective quickly
- SOME IN "BRO" CONTEXTS - ADAPTABLE IN UNUSUAL SOCIAL SITUATIONS, but feel like I am "Faking it" in overly 'bro' contexts.


### MENTOR - Do you have a mentor?   ????      <<<<<<<<<
- NINA - Research
- LUIS - Product / Pitch
- NICK/JASON - Startup/Business & Wisdom
- MELANIE - FAANG?


### SPARE TIME - [What do you do in your spare time?](https://www.indeed.com/career-advice/interviewing/what-do-you-like-to-do)
- FAMILY - I have a young son, so many outings are centered on the family.
- HACKING - I got into computing for love of hacking.  So I have a never ending stream of hacking projects.
- LEGACY PROJECTS - 


### SKILLS - Describe your top three technical skills?
- SW Engineering & Architecture
- FORMAL MATH - THEOREM PROVING - 


### HANDS ON - Describe last time you were hands on.
- OBSIDIAN - Active pages plugin
- NOOM - Time series analysis of causal factors health changes
- Python ???; Notester
- MARTIAN ROBOTICS - Marching cubes adaptation
- AEOLUS ROBOTICS - Architectural design   ????
- AF - Instant Design


#### YOUNGER - How would you feel about reporting to a person younger than you?
- ZERO issues.  I have (Tessa Lau).  Harder is the case of reporting to someone with gaps.  (Alex)


#### CAUSES - [What causes are you passionate about?](https://www.indeed.com/career-advice/interviewing/interview-question-what-are-you-passionate-about) 
- SCALING SOCIETAL GOODS
	- Good Jobs
	- Global Equality
	- Good Information


## .


# REF
## QUESTION LISTS
- https://www.indeed.com/career-advice/interviewing/top-interview-questions-and-answers?from=careeradvice-US
- https://www.indeed.com/career-advice/interviewing/most-common-behavioral-interview-questions-and-answers
- [AMAZON Interview TIPS](https://www.youtube.com/watch?v=0jFjGkjdHl8)  

# LOG
<!--stackedit_data:
eyJoaXN0b3J5IjpbLTIwNzgyNDM2MjBdfQ==
-->