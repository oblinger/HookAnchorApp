
## TODO  [[LOGS/ideas/2009-03 Team Saver/TeamSaver.pptx|Powerpoint]] 
# TOPICS
## === DECIDE ===
Should I double down on this?
-- If viral loop hooks, then yes
-- If companies bite easily, then yes


Early Evidence
-- ask knowledge workers
-- ask investors: 

MVP 
-- person id; rough gaze; voice reco
-- door
-- Web then iOS app,


TARGET USERS: Knowledge workers
-- Software Engineers, Designers, BackOffice?
## === MONEY & EQUITY
w. nick

-- default is an EIP-like arrangement.  If it starts going somewhere we spin it off of AF with significant joint ownership from each.  
-- Ownership slants towards Dan as this is one of his many-year ideas.  
-- still AF supported its developement and it is part of the AF family so is is significantly joint
-- exact percentage should shift upward if Nick is also taking on a leadership role in it execution 


-- if we get external funding then we should spin out & build a formal cap table.  
	-- with either AF or Nick and Dan getting equity for work done upto that point.
	-- further work could be combo of equity & cash from investor
	-- seperate equity should go to nick if he is also taking a leadership role too
		(this decision should remain open, so Nick can take a wait and see approach too)

_
## === VIRAL ADOPTION ===


ACTIVE USERS

CREATE NEW HALLWAYS
- User names & interacts with a group of friends or collegues by creating a "hallway"
- System prompts user with ideas for new hallways and people to add to them too.

ADD FRIENDS
- Based on recent communication patterns, system suggests folks to add to halls
- This process is 100% PRIVACY PRESERVING.  All done locally, no data to the cloud




NEW USER
- User is offered a ONE-CLICK "peek" into virtual world without any sign in.

100% PRIVATE SUGGESTION 



- they create a hallway.



GROUP IS USING
- Group of friends

NEW COMMUNITY PROMPT
- System asks users if there is any other community they care to be aware of
- Family * Friends * Work Groups
- User names group, and uses existing media channels (email, FB, text, etc.) to id some participants

NEW USER PROMPT
- System will ask if members of community are missing

NON-WORK SIGNUP
1. SEND LINK -- User sends invitation link by text/email etc. to their friend.
	- User provides their name and the community they are part of.
	- System will scan recent email to provide list of most frequent friends 
	- Message explains by clicking link they will be a guest of 'user' in 'community'
		and they will be able to see activities of others and optionally show their activity
2. CLICK -- Recipient clicks
	- Once they click they are immediately shown a first person view into the space and are in the caffeteria.
	- If others are around anywhere in the community they are shown as icons
	- A helpful robot icon explains their situation and how to control the interface.
3. GUEST MODE -- User is automatically entered in 'guest mode'
	- They are already a permanent user of the system
		System uses a browser cookie to keep track of them as distinct user.
	- They have a humorous 'guest body' icon which is distinct, but generic
	- Their name is listed as "Judy's guest Bob"
	- User can use system for as long as they like in guest mode, but all will see them as a newbie user.
	- Built-in camera & mic.  encourage leaving tablet or laptop on
4. REGULAR USER UPGRADE -- User can upgrade to be a regular user by providing:
	- Only AFTER user has decided they like the app do we make them really signup.
	- VERIFICATION -- Username/password/email -- this lets us recover their account, use system from multiple devices, and gives stronger understand of who they really are.
	- HEAD SHOTS -- using their computer's camera we take head shots from several angles and left them edit out the background.  (e.g. profile left, profile right, looking down, etc.)
	- CARTOON BODY -- Users get to add cartoon bodies for themselves walking, sitting etc.
	- NOT REALISTIC -- No attempt is made to make this realistic body might be flamboyant or generic, but either way it looks like these videos where a real head is pasted onto a cartoon body.
		(see [photo](https://dbjk47w5ep2rm.cloudfront.net/blog/wp-content/uploads/2015/06/Screen-Shot-2015-06-10-at-4.16.06-PM.png) or [video](https://videohive.net/item/christmas-faces-greeting/13556955))
5. xx
## === THE HOOK MODEL ===

TRIGGER:	User notices (sees/hears) motion on the Presence screen
ACTION:		User looks at it and decided to interact for 1 or 2 seconds or not
REWARD:		Round trip acknowledgement from teamate, maybe status info conveyed.
INVESTMENT:	User might check status of door and/or consider deeper connection
_
## === QUESTIONS


ASSUME I HAVE A "magic room" TECHNOLOGY   
   ASSUME CUSTOMERS WOULD TRY IT & USE IT.
   -- Would this change the world.  would you be able to wholesale replace domestic teams with world talent?


-- Could I get customers to try it.


-- Am I addressing the core of the issue.


-- how would I 'test' ado


IDEAS:
--Find folks that do project based work and ask them
  -- Procom -- NCI -- examples of houses that provide this.

ISSUES:
-- timezone
-- cultural gaps -- soln:  project mgr that understand both cultures
-- trust of team -- this seems solved by magic room that provides all times viewing



"I think maybe clients would give it a go"
"I don't think inability to 'leave' the magic room would be a big issue."


_
## === REF-DESIGN
### Timing Based Justification

===AWARENESS==

COMM-STATE -- Awareness of the communicational state of your communication partner(s).  E.g.
              INTERRUPT -- wants to interrupt (with question, topic change, to add info etc.)
              ATTENTIONAL LEVEL -- what fraction of their attention is on the communication itself (instead of email, or sleeping)
              COMPREHENSIONAL LEVEL -- how well is partner understanding current communication
              EMOTIONAL STATE -- what emotional reactions are they having.  Boredom, Frustration, Anger, Rejecting, Accepting, Resonating, Excited, Happiness.

ACTIVITY  --  Awareness of communication partners current activity
              ACTIVITY STATE -- Broad characterization of partners current activity. 
                NOT AT WORK, AT WORK, DOING WORK, IN MEETING, 
              ENGAGEMENT     -- How deeply are they engrossed in what they are doing
                HEADS DOWN     -- Eyes locked onto task, fingers executing.  Little awareness of surroundings.
                WORKING        -- Clearly working on task, but also showing signs of area awareness.  (looking up, talking, pausing)
                TRANSITIONING  -- Moving from a working task to something else
                MULTI-TASKING  -- Working across multiple tasks in quick succession.
                PLAYING
                RESTING
              TASK -- Knowledge of specific tasking currently being worked on

===COMM=ACTION==
   


===MEASURES=OF=DISTANCE==

TIME   --  Minimum and maximum time it typically takes to perform the specified operation in the specified context
           E.g.  <1sec  5-10sec  1hour
FREQ   --  Min/Max number of times an operation type is typically performed per time period in a specified context
           E.g.  5/hr  10-20/week
EFFORT --  Qualitative characterization of the difficulty of performing operation in the specified context
           SUB (subconsciously performed w/o intent)  AUTO (subconscious requiring intent)   
           EASY (perceived easy, but requiring focus) NORMAL (not easy, not hard)   HARD (sufficient perceived effort/annoyance to notably impact propensity to execute)

==NATURAL=MODES==     INTERRUPT
1  In Conversation    <1s  >10/min  SUB
2  In Group Conv
3  At Same Desk       
4  In Same Room       1s   5-20/hr  SUB-AUTO
5  On Same Floor      20-100s 
6  In Same Building
7  In Same Complex
8  Commutable
9  Same Time Zone
10 Anywhere

_
### Target and Design

-- fail: flow, cog, coverage

-- attentional measures of others 

_
### Target Use Cases and Value Story

KEY VALUES
-- SAMETABLE  --  provides 'working at same table' experience for tight team
-- ONSITE     --  customer get 'on site' experience w. windows & crash cart
-- TELEPERSON --  single expert can serve in remote location
_
#### Tight Remote team with single client.
--  Tight team of 4-20 knowledge workers
--  They are collaborating full time on a project for a single client
--  They are all remote from each other, and from the single client location

#### Single Remote Worker in a tight team
--   One specialist worker working remotely within a tight team
--   They have their own office, cubical, desk within the team space
     and a parallel remote setup at their home.

..   Single remote 


_
### Solution Design

--  WORKER'S OFFICE  --  Each worker's home office is equipped with:
    -- DESK          --  A physical desk.  Ideally facing into a wall or window.
    -- A TABLETOP    --  Placed on the desk (at a slight incline, or constructed into the desk)
    -- A HEADSHOT    --  Placed on the wall / window opposite the worker
    -- A COMPUTER    --  Connected to the tabletop and placed to its left.

--  CRASH CART       --  A mobile version of an office.  A TABLETOP and HEADSHOT on WHEELS w. UPS.

--  OFFICE WINDOW    --  A window (usually hung along a hallway near multiplex room)

--  MULTIPLEX ROOM   --  A workers office with a doorbell

--  DOOR BELL        --  A keypad with a labelled list of room numbers.
                         Doorbell is mounted by office window, or multiplex room.
                        (Pressing the labelled doorbell causes room or window to become that room's door or window)


--  A TABLETOP       -- Each "tabletop" is a synchronized view onto the common resources within the virtual 
                        'project' room where it is currently placed.   The table's hardware / software components:
    -- MONITOR       -- a 24-30" 2.5-4K touch screen display -- placed at 20 degrees on table in the user's physical space.
                        All tabletops in the same room always display the same shared image.
    -- KEYBOARD      -- Wireless keyboard specific for tabletop
    -- USB           -- USB port that synchronizes thumb drive w. shared room resources
    -- PRINTER       -- Printer/scanner/fax that prints/loads locally or to table
    -- COMPUTER      -- A local computer whose monoitor is tied to the table
    -- COLLAB-SAAS   -- Each project room has a dedicated folder synchronized to tables, and to shared
                        web services like: Dropbox, GoogleDrive, Slack, etc.
 

--  A HEADSHOT       -- Awareness of the room and its surroundings
    -- MONITOR       -- 16-20" LCD display
    -- MICS          -- Multiple microphones
    -- CAMERA        -- Wide angle camera.  (system using voice & vision processing to clip around 1 of 3 speakers)
    -- SPEAKERS      -- Speakers providing room sound
    -- HEADSET       -- Bluetooth headset for more accurate sound capture and presetation
    -- AWARENESS BAR -- 
    -- OPERATION
       -- ALONE         Headshot shows view 'out' of current room's door.  (closed/cracked/open)
       -- IN MEETING    Headshot shows view of current speaker(s) or the participant grid if no one is speaking


--  AWARENESS BAR   -- Provide global awareness
                    -- By default headshot shows nearby activity along its bottom.
                       Activity in shared projects/rooms.  people coming and going.  (maybe worker's room activity)



--  A PROJECT        -- a named collaboration.  Provides history, control, access.  
                        Tied to web-based collaboration tools like:  Dropbox, Google Drive, Hipchat, Slack, etc.
    -- FOLDER        -- Files shared into that project.  (we just expose collaboration tools sharing facilitates)
    -- HISTORY       -- Chat history around the topic.


--  A ROOM           -- A room is a virtual place where collaboration can happen.  Each room has:
    -- HOST          -- Each active room has a cloud-based computational host for shared resources.
    -- PROJECT       -- Each active room is associated with a named project (even if an ad-hoc one).
    -- FOLDER        -- Each project has a shared folder containing the shared resources in the active room.
   
    -- NAME/PLACE    -- Each room has a name (or number) plus a location within some virtual space
    -- DOOR          -- Each room door is used for managing access to the room
    -- DOOR KNOB     -- The door can be opened, cracked, closed, locked, or sealed
    -- DOOR PLATE    -- The door plate specifies the project currently being discussed in the room

_
### Key Questions / Markets

--  Will customers accept virtually embedded remote workers?

_
### Activities

LEVEL OF EFFORT
--  N  --  Number of times a thing typically happens per day.
--  S  --  Seconds of singular effort activity require
--  D  --  Disruption level  (complex thought==multiplying numbers in head.  Simple thought=speaking known sentence.)
           -7 -- Speed and ability of complex thought processing is not noticably impacted
           -6 -- Complex thought is diminished but not interrupted
           -5 -- Complex thought is broken
           -4 -- Simple thought is only minimally impacted
           -3 -- Simple thought is possible
           -2 -- Simple thought is broken
           -1 -- 1/10 sec.  0=1 sec.  1=10 sec.  2=100 sec.




AWARENESS
--  K_STATE     --  Know a person's state:  at-work, at-computer, in-between
--  K_TASK      --  Knowing what task a person is working on
--  K_PROGRESS  --  Knowing the current progress and remaining effort needed for task person is working on

SAY
-- WANT_TO_COMMENT

ASK
-- ASK_TO_TALK      

RESPOND


SHARE
-- SHARE_QA        -- Simple question / answer transaction
-- SHARE_VOICE     -- Talk together
-- SHARE_SCREEN




_
### Misc Features
- push button mike (push on; push off; default on; default off;  Light showing current state)
- this is one of the areas that needs to be sub-second.

- register 'INTENT' to talk.


Russia Time Zones

     PST  EST  Pol  Mosc
v    3a   6a   12   1p
vvv  6a   9a   3p   4p
     9a   12   6p   7p
--   10a  1p   7p   8p
^    12   3p   9p   10p
^^^  3p   6p   12a  1a


_
## === PATENT POTENTIAL ===
### --- Op Comms ---
- No intention; No interruptions

gap detection; alignment; 
non-interruptive presentation
_
### --- Op Comms second monitor ---

_
### --- Head Tracking Graph Navigation ---
- Remap space so certain directions can be controls

_
## === OLD INFO ===
### External docs

	Remote work article  https://source.opennews.org/en-US/learning/making-remote-work-work/

	_
### Existing Buisnesses
	  - Conf Advisors   http://www.conferencingadvisors.com/

	  -  Sqwiggle (https://www.sqwiggle.com/). Always-on videoconferencing
	    for a fully distributed team
	  -  Voxer (http://voxer.com/) is a great example of this that Julian
	    and I found which we think is awesome. Yes, you could send
	    voicemails

	- 
	http://screenhero.com


	_
### Articles on collaboration

	- The Fog of Work
	http://blog.screenhero.com/post/60401526813/connected-but-isolated-the-real-problem-with-working?



	_
# INFO
## === PTRS ===
### --- Touch screens ---
#### -- $190 - Asus 21" touch screen --
- [Asus 21"](https://www.amazon.com/ASUS-VT229H-Monitor-1080P-10-Point/dp/B07P619NXM/ref=sxin_2_ac_d_rm?ac_md=0-0-dG91Y2hzY3JlZW4gbW9uaXRvcg%3D%3D-ac_d_rm&cv_ct_cx=touchscreen+monitor&dchild=1&keywords=touchscreen+monitor&pd_rd_i=B07P619NXM&pd_rd_r=5a201a1a-2ad6-4491-bda4-40ddf0f22966&pd_rd_w=kl9Lx&pd_rd_wg=GMSCX&pf_rd_p=e3dc9e0c-9eab-4c3e-b43a-ba36f8522e14&pf_rd_r=7DXGFRCNXA6ZDTKESVRG&psc=1&qid=1598566814&sr=1-1-12d4272d-8adb-4121-8624-135149aa9081) 
_
### --- Head Tracking ---
#### -- Papers --
##### - fitz law -
[mobile device - fitz law](https://www.yorku.ca/mack/ijhcs2018.html) 
_
## === PAPERS ===
### --- Remote Presence ---


[1994 - Casual collaboration [visual interface]](https://www.semanticscholar.org/paper/Casual-collaboration-%5Bvisual-interface%5D-Donath/6f15dc720a2c3c0916e2413420c05f1abe6e25c0) 
- Showing presence using animated characters
- [Judith Donath](Judith S. Donath) jdonath@cyber.harvard.edu

[2002 - Informal Comm in orgs: Form, Function, and Tech](https://pdfs.semanticscholar.org/60c5/25c2beee3d1820c020b7e880b46ca91f5685.pdf?_ga=2.81593495.182898066.1590809752-242610115.1590809752)  
- (Cruiser)

[2002 - Descriptive theory of awareness for groupware development](https://www.semanticscholar.org/paper/Descriptive-theory-of-awareness-for-groupware-Collazos-Vela/393f8c33a69fc40e714da2794547569f7dfce15b) 
- [Carl Gutwin](https://www.cs.usask.ca/faculty/gutwin/)  gutwin @ cs.usask.ca
- [SAUL GREENBERG](http://saul.cpsc.ucalgary.ca/)  saul.greenberg@ucalgary.ca
 
[2004 - The Use of A/V in Synchronous Collaborative Writing](https://www.semanticscholar.org/paper/The-Use-of-Audio-and-Video-in-Synchronous-Writing-Nijenhuis-Eklundh/0434a02ec2bc53376961c79d5954ceba50d34282) 
- Thesis studying synchronous remote authoring
- [Nienke Nijenhuis](https://www.linkedin.com/in/ndnijenhuis/?originalSubdomain=dk) kth

[2006 - Turn it this way : remote gesturing](https://www.semanticscholar.org/paper/Turn-it-this-way-%3A-remote-gesturing-in-Kirk/11de53bfe81d8fdb5955e538024345cfac3281fe). 
- Fancy Gesturing

[2009 Virtual Teams: a Literature Review](http://citeseerx.ist.psu.edu/viewdoc/download?doi=10.1.1.174.5464&rep=rep1&type=pdf) 

[2013 Effortless Coordination : 25 Years of Awareness Research](https://www.semanticscholar.org/paper/Effortless-Coordination-%3A-25-Years-of-Awareness-Gross/e8599e517955d72899e80901e0dc9bc53017a9e3) 
  - privacy tradeoff


[Theories and methods in mediated communication](https://www.semanticscholar.org/paper/Theories-and-methods-in-mediated-communication-Whittaker/6c1416351a9da5d13a440a2e6577e7b5a14cc333) 
[16 types of meetings](https://blog.lucidmeetings.com/blog/16-types-of-business-meetings) 


[Rooms for Virtual Comms: Reciprocal Awareness](https://www.semanticscholar.org/paper/Designing-Rooms-for-Virtual%2C-Informal-Reciprocal-as-Flepp-Imhof/8a7f7040702f2afdc6886726788bd604faf8da3f)   (nice graphic)


[What Are the Challenges of Working in Virtual Teams?](https://us.experteer.com/magazine/what-are-the-challenges-of-working-in-virtual-teams/)  (pic of challenges)
[What is face-to-face conversation? Advantages and disadvantages](https://thebusinesscommunication.com/what-is-face-to-face-conversation/) 
[Silence & art of conversation](https://faithandleadership.com/diane-m-millis-silence-and-art-conversation) 
[Cogent Communicator: How to have conversation](https://insight.ieeeusa.org/articles/cogent-communicator-how-to-have-a-conversation/) 
[5 Business Conversations to Have Today](5 Business Conversations to Have Today) 

[Bibliography](https://www.semanticscholar.org/paper/Video-as-a-technology-for-informal-communication-Fish-Kraut/a4a941dd5731f4d1a60794afaf8cc44196b85e2a?citingPapersSort=relevance&citingPapersLimit=10&citingPapersOffset=20&year%5B0%5D=&year%5B1%5D=&citedPapersSort=relevance&citedPapersLimit=10&citedPapersOffset=0) 


**OTHER I/O**

**LONG TERM REMOTE PRESENCE**
[A Room of Your Own: Assessing Teams in Dedicated Project Rooms?](https://www.semanticscholar.org/paper/A-Room-of-Your-Own%3A-What-Do-We-Learn-about-Support-Covi-Olson/54aa2cfc039625c69b41d3a8fb8c0d852baa9a09) 
[The magic window: lessons from a year in the life of a co-present media space](https://www.semanticscholar.org/paper/The-magic-window%3A-lessons-from-a-year-in-the-life-a-Kim-Gutwin/3201a92cd6c7313132bd5ce454325358df82ba95) 
[MY LIFE WITH ALWAYS-ON VIDEO 1 My Life with Always-On Video](https://www.semanticscholar.org/paper/MY-LIFE-WITH-ALWAYS-ON-VIDEO-1-My-Life-with-Video-Neustaedter-Fraser/1bf5586ec703b24c4ae85b63b84597fc7cce487b) 
[Sharing Domestic Life through Long-Term Video Connections](https://www.semanticscholar.org/paper/Sharing-Domestic-Life-through-Long-Term-Video-Neustaedter-Pang/79ccd89433ac7bcc89ab35f888f3ec00eeb029e1) 
### --- HEAD TRACKING ---

[Android Tablet GPU](https://www.youtube.com/watch?v=_KV56XGxvEw) 
[TABLET - Google Pixel Slate](https://www.amazon.com/Google-Pixel-12-3-Inch-Tablet-aspect/dp/B07JWCHWRM/ref=sr_1_3?dchild=1&keywords=google+pixel+tablet&qid=1592457494&s=electronics&sr=1-3)  $455  M3 8Gig
[TABLET - Google Pixel C](https://www.amazon.com/Google-Pixel-Tablet-Silver-Aluminum/dp/B01944ZBMO)  Nvida Tegra. 
[GPU TABLET](https://www.amazon.com/1280x800-Processor-Bluetooth-10-1Inch-24CMx17CM/dp/B0865Q1BWH/ref=sr_1_3?dchild=1&keywords=cheap+tablet+with+GPU&qid=1592458007&s=electronics&sr=1-3) $80

[TrackIR 5](TrackIR 5)  (Scaled Motion)

- Use scaled motion to navigate enviornment

**Head Pose From Camera**
[Hopenet head pose from camera](https://www.youtube.com/watch?v=Bz6eF4Nl1O8) 
[AllGo System](https://www.visteon.com/newsroom/white-paper-camera-based-driver-monitoring-deep-learning/) 
[OpenCV dlib](https://www.youtube.com/watch?v=xurEs0G9ARs)  
[Feature-based head post from img - ACADEMIC PAPER](https://www.researchgate.net/publication/224401340_Feature-based_head_pose_estimation_from_images) 
## === PEOPLE ===

CRUISER
	[Robert Kraut](https://www.cmu.edu/tepper/faculty-and-research/faculty-by-area/profiles/kraut-robert.html)  Student [Darren Gergle](https://dgergle.soc.northwestern.edu/) 
	[Robert Fish](https://www.cs.princeton.edu/people/profile/rfish) Princeton Lecturer.  no new work

Comm and Info: Alt use of Internet in households  CHI 98
- [Tridas Mukhopadhyay](http://www.andrew.cmu.edu/user/tridas/vita.html) tridas@andrew.cmu.edu
- [Janusz Szczypula](https://www.heinz.cmu.edu/faculty-research/profiles/szczypula-janusz) jslm@
- [Sara Kiesler](https://www.cs.cmu.edu/~kiesler/). kiesler+  Emeritus robot HCI, anonymoty online, 
- [William Scherlis](https://www.isri.cmu.edu/people/core-faculty/scherlis-william.html)  scherlis@   <-- Ignored me, after one try

_
## === BLOGS ===
[Zoom peer to peer vs cloud](https://blog.zoom.us/cloud-based-and-peer-peer-meetings/)  
# 2021 MATERIALS
#### ...
Non Creepy
- hide present activity


JobToBeDone
- Connect team
- provide awareness of key things happening within team in timely fashion

PAINS
- Team member is not executing in best way for lack of support from specific person at specific moment.
- Lost on best way to proceed
- Not working on most impt stuff
- People cannot just "jump in" and help opportunistically.  thus opp is lost.
	most peer to peer connections are lost
- can ask light 5-second questions that mushroom into real help
GAINS
- Get support from the right person in time to matter
- contagon of momentum as others are doing stuff

COMPETITOR DIFFERENTIATION
- slack -- 


DOWNSIDES / LIMITATIONS
- No friction
- can control my reputation/image
- non-creepy
- needs to happen on my schedule

- 

_
#### -- format of hypothesis --
I believe that [persona] experiences [pain] when [job-to-be-done].

They experience [pain] because [problem].

[Pain] can be relieved by [solution]

However, [persona] would have to be okay with [downside / limitation].

Unlike [competitor / substitute], [solution] will [differentiating advantage], allowing [company] to [current goal].

_
## ACTIVITY
### === ... ===
#### notes

> >>> Setting the door when you sit down
> 	>>> Turning on navigation
> https://patents.google.com/patent/US8696458B2/en
> _
#### _

	ACTIVE
	- **RESEARCHERS** -- Chat with 2-4 researchers about the value/novelty of the idea.
	- **PITCH** -- Get pitch fleshed out.
		- PATENT POTENTIAL -- Write abstract for these ideas up.
	- **PLANNING** -- Workout level of effort for demos
	- **OWNERSHIP FRAMEWORK** -- 
	- **OP COMM DEMO** --
	- **BOWTIE DEMO** -- Mouse based; head based; cobbled to launch something; cobble to chat

	- **INVESTORS** -- Chat with 2-4 investors about the value of the idea.

	read about zoom, chat-impl

	_
#### --- Ownership framework ---

	Two paths:  The EIP-model.  The Funded-Model.

	Either this product is externally seed-funded, or it is bootstrapped

	EIP-model.
	- Costs of building product are borne by AF.  (Bulk of costs are dev labor costs)
	- Once product is off ground we would form an LLC with direct ownership by nick&dan
	- Split would be similar to percentages we discussed for EIP... depending on roles and effort each founder was putting in.
	- Maybe we set a rate based on roles but vest it over 4 years so it can shift if roles shift.


	Funded-Model.
	- Work done (before and after seed) is paid via combo of cash and equity.
	- Work done is paid on some kind of friends and family rate.
	- Thus AF is directly on the cap table.
	- Independently Nick, Dan, and investor are on the cap table



	==> My aim would be to lean in to a Nick and Dan co-adventure
	==> Also to be ensure that equity matched effort over long run
	==> Try to keep door open for extended period for nick-role upgrade if it takes off




	_
#### --- Questions for reserachers ---
	- KINDS OF CORP COMMS -- Taxonomy of important conversations within the paperless knowlege worker corporate env.
	- PRIOR WORK -- 
	- COMM TECH & MODALITIES -- 
	_
#### --- providing value - setting equity ---

	- IDEA -- Modest value.  ~10-15% of A-round value
	- FUNDS -- at whatever valuation they are provided
	- CONTRIBUTOR WORK -- paid in cash or in equity 
	- LEADERSHIP -- paid in equity.

	AF I-RAD Scenerio -- System is built primarily using AF I-RAD funds.
	- Modest equity awarded for idea.
	- Modest equity awarded for leadership.  (to nick, to dan as work is done)
	- Contributor work from AF performed in compensation for equity

	INVESTOR Scenerio -- System is build primarily using investor funds.
	- Any Early work performed at lower valuation.


	_
### === TODO ===
#### --- NOW ITEMS ---
	- MONEY			--  Get money&equity section (under topics) done
	- SLIDES			--  Get "jist" slides complete (for friends&potential cust)
	- CALLS			--  Get calls lined up & executed
	- DECIDE			--  Decide if this direction to go in
#### --- ARC ---
	- JIST DECK		--  Express Jist of idea for a friendly audience
	- PM FIT9		--  Describe and test Product/Mkt FIT (September)
	- OCTO			--  OctoNav Demo1
	- ENG CALLS		--	Engineering Cust Calls
#### --- Jist Deck ---
	- JIST			-- Express the "jist" of the idea
	- FRIENDLY		-- Expressed for friendly audience
	_
#### --- PM FIT09 ---
	- FRAME VALUE 	-- Describe value in way cust says ``yes this is big pain''
	- NORTH STAR 	-- Make remote paperless K workers as effective as in-person
	- MVP VALUE 		-- Enable Op comms and Loose comms

#### --- OctoNav Demo1 ---
	- MOUSE DRIVEN 	-- Demo of octo nav based on mouse motions
	- SCROLLING 		-- Demo of scrolling within 2D plane
	- ZOOMING 		-- Demo of bouse-based zooming in/out
	- JUMP 			-- Demo of jumping in/out

#### --- Calls ---
	FIRST:  Kartik, Freed, Tessa, Dan Shapiro, Bill Katz, Dan Adams,

 
	INVESTORS: 
		Christina Fonseca, Chris Ramming, 
	
	FRIENDS: 
		Kartik, Josh, Jason, Yashin, Theo, James Miller, Jessie Lee?
		Greg Hornby
		
	FRIENDS: 
		Dan Nuke, Susan Moray, Ethan Herdrick, Phil & Tessa, Michael Freed, 
		Acorn Pooley, Rob Farrell*, Doug Gordin, Mark Brodie, Shane, 
		Matt Chana, Adam Kempler, Pietro Michelluchi, Jacob Ukelson (IBM internship)
		Matt Nolker, Chris Klanac, 
	
	STARTUP: 
		Gaurav Rekhi, Ben Wild, Dan Haddick, Jack Porter, Nandu, Ric Szopa, Dennis DeCoste
		Mark Drummond, 
	
	500:	
		Earnest Tarango, Dick Brouwer, Michael Mayernick, Geoff Hayes, Kenny Hawk, 
		James Levine, Ruwan Relenta (CEO Evo), Jean Yang, Mark Urdhal (helped me early)
	
	AI GUYS: 
		Pat Lincoln, Steve Minton, Praven Paratosh, Oren Etzoni, Haym Hirsh, Tom Dietterich
		Prayful Krishna, George Cybenko, Carlos Guestrin, Dean Allemang, Ron Brachman
		Russ Griener, Craig Knowblock, Ronny Kohavi
		John Langford, Barney Pell, Joe Hellerstein, Michael Whitbrock, 
		pat hayes, Paul O'rorke, David Ferrucci, 
	
	DARPA:
		Tom Wagner, Bob Schrag, Ray Perrult (SRI), Ciaran O'reiley, Jim Hendler, 
	
	Engineers:  
		Alan Tritchler, Jean Yang, 

	Mech E:
		Yoon Jeong (at berkeley)

##### -- Friends
	Kartik, Josh?, 

	_
### === PLANNING ===
#### --- TODO ---
	- DECK -- Build to get feedback for: investment, idea eval
	- DEMO -- Get octo-nav demo v0 working
	- CALL -- Get list of calls lined up
	- PROFESSIONS

	- Greatest pain point -- what profession is most hurt by covid
		- Find list of professions and scan for best candidates
	_
#### --- Sub Todos ---
##### -- CALLS --

	Later
	- MEL - how will CIOs think about this software.
	- TALKDESK - 
	_
## PARTS
### === LEAN CANVAS ===
#### PROBLEM - Remote collaboration is not as good as in-person collaboration
		Provide in-person efficiencies between co-temporal remote workers
		_
#### SEGMENTS - Businesses with tightly interacting desk workers
		_
#### VALUE PROP - Make remote work as effective or more effective than in-person work
##### _

		Replacement for key in-person-like comms for the remote desk workers

		PROVIDES
		- INFORMAL COMMS -- Frequent "no-cost" informal "intention-free" comms.
		- FLUID MULTI CHANNEL COMMS -- 
		- NO-SPY AWARENESS -- Provide comms related awareness, w/o being "spying" upon
		_
##### --- progression ---

		base awareness
		- that some action is happening and who is involved

		grape vine -- overhearing
		- incomplete, secondary, non-interactive

		Interactive
		_
##### --- High Level Solution ---

		_**Provide remote-work alternatives for lost in-person interactions**_:
		- _unconscious actions_ -- actions whose completion don't require conscious thought/act.
		- _unconscious knowlege_ -- information that is known w/o any intention to collect it.


		FACE2FACE -- Speech, Viewing, Gaze, Attention, 
			=> Pointing	
		BG
			=> Inject, Glom


		Kinds of in-person interaction types
		Provide remote alternatives for key in-person interactions.  The:
		- SCHEDULED		--  
		- FACE2FACE		--  Extended dedicated interaction
		- QUICK CHECK	--  Explicitly initiated comm, w intelligent interruption
		- SHOUT OUT		--
		- OP COMM		
		- GLOM ON

		Kinds of lost actions and awarenesses:

		=== FACE TO FACE ===
		- GAZE -- What is each person looking at RIGHT NOW?
			Are they looking directly at my face?  At another's face?  At slide?  laptop?  door?
		- DISTRACTIONS -- What momentary distractions are they facing RIGHT NOW?
			Is something changing in their environments?
			Is something or someone clamoring for their attention?
			==> in person allows smooth inclusion and handling of these asymetric interruptions   	  since all parties have knowledge of sitaution & likely resolution.
		- HEAR -- Can they hear and understand you?
		- POINT -- What are they pointing at?
		- THEIR TURN -- 
		- AGREEMENT -- Gaining agreement on a known choice

		=== OPPORTUNISTIC ===
		- FREE NOW -- Person is momentarily broken from focused work
		- BOTH FREE -- 


		_
###### -- properties

		comm properties
		- who is involved
		- when is it happening
		- what is the scope & topic
		- order of questions and material
#### SOLUTION - 

	Provide remote alternatives for key in person abilities:  
	detection of:  Gaze * Availability * Attention


	Drive organizational awareness using remote alternatives for key in-person comm patterns:
	- Opportunistic comms
	- Loose comms




		- SEPARATE HW -- Does not interfere w. computing & minimizes setup

		ATTRIBUTES:
		- INTENTION FREE -- Ad hoc communication happens w/o explicit intent to communicate.
		- INTERRUPTION FREE -- Work flow & focus is not interrupted 
		- NO CONSCIOUS EFFORT -- Like "hello" while walking by no effort beyond intent needed.


		Is partner distracted by something
		Break off and chat with 2 other people

		_
#### UNFAIR ADV - Better idea

		Protecting this work:

		- INCOMPATIBLE USE CASE -- Building long running presence is completely difference UX from existing comm solutions
		- COCKTAIL COMMS -- 
			Overlapping speech via attention-driven channel formation for corporate chat
			-- hands free attention sensing
		- OP COMMS -- 
			- PERIPHERAL VISION 
		- VIRAL ADOPTION MODEL

		_

	_
### === VALUE CLAIM ===

	The Incremental In-Person Benefit over Remote Non-Physical Knowledge Work:

	**claim** The bulk of this incremental value is stems from two key communication paradigms: 	
		_op-comms_ -- opportunistic communcations that occur 'accidentally'
		_loose-comms_ -- loosely-structured communications that dont fit the turn-taking structure meeting pattern

	Neither are supported by existing comm tools and both are central to the RemP tool.

	**justification** -- 
	That's it?!  Get these two right and remote paperless knowlege work is just like in person work??
	To see this is true, let us consider the following bizarre workplace.  Its just like a tradional office for paperless knowelge workers except:

	- opportunistic comms are denied by requiring workers to travel exclusively from room to room in opaque driverless carts where they enter the target room to go from one to the other.  AND

	- loose comms are denied by requiring them to wear audio headsets when they get to their new room.  These headsets ensure that all people in the room hear precisely the same audio.

	But otherwise their working habits and work place are completely untouched. Observations:
	 #1. What a demoralizing, F-ing pain in the ASS place to work!!
	 #2. Working at this bizare corp is not much different than being a remote paperless knowlege worker today who connects via email, text, phone and video chat.

	So whatever op-comms and loose-comms are they contain massive value, since nearly all of the incremental value of in-person are contained in them!



	**Opportunistic Communications (OP-COMMS)** -- OP-COMMS are opportunities for short, small-group communications where the opportunity itself occurs without _ANY_ intent from any participant to create the opporunity or to select the participants -- the opportunity "just happens" as a natural consquence of other unrelated work activities.  (e.g. an accidental meeting on the way somewhere)

	**Loose Communications (LOOSE-COMMS)** -- LOOSE-COMMS are a generalization of traditional "turn-taking" group communications where:
		(a) the set of participants is "loose" -- membership can evolve on sec-by-sec basis, (b) participation itself is "loose" in varys from full engagement to barely aware
		(c) finally the set of separate communications channels themselves can evolve on a second by second basis without precise or explicit moments when they are created or destroyed.
	_
### === OP COMMS ===
#### _
	OPPORTUNISTIC COMMUNICATIONS -- small-group, quick communication opportunites that occur w/o intent

		1. NO INTENT -- Specific opportunity to communicate is created w/o ANY selection of partner 
		2. DESIGNED -- 
		3. NON-INTERRUPTIVE -- 
		4. EPHEMERAL -- 
		5. OFTEN ASYMETRIC -- 
		6. SUBCONSCOUSLY MANAGED -- 

	VALUE -- 

				Frequency		Effort
	CONFIG		~10x/day		Instant decision; seconds to implement
	PERCEIVE	100-1000x/day	Cannot perceive effort (unless very focused)
	ACTIVATE	~10-100x/day	Instant decision; no perceptable effort dozen at time
	UPGRADE		2-5x/day		<10 seconds; minimal effort
	
	CONFIG - Configuring how accessible you want to be for period of time

	_
#### --- OP COMMS - "be right back"

	- Coffee cup

	- manually or automatically shows the "be right back" symbol

	- worker looks at door, says "check josh"
		==> pov walks hallway to Josh's office and this sets up 
		==> optionally set urgency flag as you leave  from:  no worries, upto connect ASAP
		==> "walk back to your desk is op for others to connect w. you

	"be right back"

	- if others visit while you are away they will show on side of screen when you return

	- any that are still as their desk will show as walking by when you return this allows a hands free op comm

	_
#### --- OP COMMS - How it works ---


	- Watcher is has door open in the background
	- Watcher has not been speaking recently and is settled into an activity

	- Walker's enviornment is relatively quiet and they are not at their desk
	- Walker is not speaking and comes back to desk and sits down
		==> Walker's icon shows in watchers doorway & person specific walking sound is generated.

	- Watcher looks at doorway 
		==> The COMMS highlight appears around the walkers head for next 5 seconds

	- Watcher begins speaking
		==> Large icon of watcher sitting at desk peering out a doorway shows on walker's screen
		==> 1-way communication channel is setup so walker can hear what the watcher said

	- Walker begins speaking in order to respond 
		==> communication channel expanded to 2-way in order to chat to happen


	~~

	- MOUSE -- either party can use the mouse to directly talk with any visible person

	- VOICE -- while no COMM highlight, voice is used to highlight any other visible (or invisible person)

	- LOOK AWAY -- if both people look away from their screens the communication channel is broken
		(But is easily reconnected by )



	_
### === LOOSE COMMS ===

	- MANY VISIBLE LURKERS
		Unless special effort 
	- LOW EFFORT CONTENT AWARENESS
		Partial 
	- EFFORTLESS UP & DOWN GRADE
	- xxxxx

	_
#### --- How It Works ---

	THE SETUP

	- Lucy lurker has her door open and is listening to the chatter outside
		==> Others in the area see her icon listening in to the chatter
		==> Since she is loosely listening the frame around her icon is "lurk lit"

	- Watching william has his door open but he is not listening
		==> Others see he has his door open, but also see the big headphone on his head

	- Chatting charlie & chris have been at it for a while, finally distracting Lucy too much
		- Using the mouse she pulled their volume down to a mumble


	WHAT HAPPENED

	- Sarah Stepper just returned to her desk
		==> Watching William sees this as does Lucy Lurker
	
	- Watching William looks at Sarah in his door and begins speaking (starting an OP COMM)
		==> Both Sarah Stepper and Lucy Lurker hear what William Watcher says
		==> Sarah sees a large image of Williams door with william at desk,  
			(other lurker/watchers are shown to her as pictures on the wall behind William)
		==> William continues to see sarah out his door; with comms highlight around her icon looking at him
			(other lurker/watchers are still shown as pictures on the wall)
		==> Lucy sees the pair move down to the tables where their icons talk in profile

	- Lucy momentarily looks up at her monitor, then looks back down at her work
		==> Both William and Sarah see Lucy's icon looked up and her picture frame glowed a bit extra too
		==> Once she looked away from her monitor the icon looked down, 
			but her frame continued to be highlighted since she is still loosely listening

	- Later lucy looks up again.  Says "hey william"
		==> Others see her look up, and Lucy sees the comm highlight around both William and Sarah
		Lucy begins speaking
		==> A three way communication channel is initiated, and both see a comm highlight around Lucy

	- Lucy looks down and stops speaking for 10 seconds
		==> Her icon immediately shows that she has looked away, but is has a comm highlight
		==> After ten seconds the comm highlight goes away
		==> Since she is still listening, her frame still "lurk lit"

	- Lucy goes for a drink, and comes back after 10 minutes
		==> her audio is cut, and her icon leaves the frame, if gone long enough the frame goes away
		==> when she returns her audio listening link resumes, and again people she her lurking

	- A second pair begins chatting while Lucy is lurking
		==> Their audio is at 100% 
		==> The audio of William and Sarah is cut to 30% to put it more in the background
		==> The audio of Charlie and Chris Chatters is cut out completely
		==> After 20 seconds the audio of William and Sarah is balanced with the new conversation

	- Sarah and William decide to continue the conversation in his office and not in the hallway
		Sarah clicks the "video upgrade button"
		==>  Lucy sees william and sarah leave the table area together, and can no longer hear them
		==>  William and Sarah's screens are replace with a tradional video conference software


	_
### === NON-CREEPY ===

	Goal: System must be:
	- NON-CREEPY	--
	- NON-SPYING	--
	- NON-LEAKY	-- 
	- AFFIRMING	-- Everyone's facebook feed looks more fun than live footage of their vaction


	- AUDIO ONLY -- to start
	- SYMMETRIC	-- 
	- RANDOM		-- 
	- FIXED OUTCOMES	-- 
	- COVERS FOR YOU	-- 
	- CONTROLLABLE	-- 


	_
### === CUSTOMER SEGMENTS ===

#### --- Key properties of ideal clients ---

	- **Paperless knowlege work** -- nearly 100% of the info workers deal with is online
	- **Computer based work** -- workers spend significant portion of working day in front of a keyboard
	- Tight team collaboration -- ideally team members have tight interaction patterns
	- Creative -- work is somehow creative; and requres each item to be discussed by multiple team members
	- Integrative -- requires integration of disparate types of expertise to produce each outcome

#### --- Types of Knowledge workers ---
	- [types of k workers](https://blog.dropbox.com/topics/work-culture/types-of-knowledge-workers) 
		Communicators:  Comms, Security, Recruiting, Sales
		Coordinators: executive admins, product mgrs, chief of staff
		Creators:  Writers, +Designers, +Engineers
	_

#### --- Engineering teams ---
	Key benefits
	- Facilitate short verbal exchanges to keep team synchronized w/o cost
	- Increase side participation in exchanges


	NON-DISTRACTION -- Engineers prize focus and non-interruption.  Providing required interactions in moments when it is not focus breaking.

	ACTIVITY AWARENESS / BEST-MICRO-COLLABORATIONS -- "No cost" hour-by-hour activity awareness increases chances that best micro-collaborations are always occuring.

	INFO AMPLIFIACTION -- Amplifies the info fanout provided of each conversation.  The loose comm abililty to unilaterally "opt in" to a non-sensitive conversation means that more people opportunistically gain benefits that would have been lost had speaker needed to specifically select who would hear them (as is require in most remote settings)

##### -- To Ask:
	- QL
	- Rob Farrell
	- Kartick
	- Nina


#### --- Design teams ---
	 - Sudy -- 

#### --- Back office teams ---

	_
### === USER SCENERIOS ===

#### --- KRISTEN IS SICK ---

	Kristen is not in the office today, but I was expecting he would have this report ready


## SOLUTION
#### --- 2020.09.15 - 2nd MONITOR "Team Saver" MVP

	OP COMM FEATURES:
	- BACKGROUND -- happy to keep always on.  not too distracting.
	- REPUTATION -- does not provide info that sullys ones "hard worker" reputation.
	- NO CREEP -- 

	LOOSE COMM FEATURES:

	**ABILITIES**:
	- **OP COMMS** -- Programmable, no intention, no load, no friction team comm opportunities
	- **LOOSE COMMS** -- 

	Parts:

	Detects:
	- BODY PLACEMENT -- rough location of user's body and face
	- GAZE -- fast, rough gaze tracking (10cm accuracy)



	Form Factor:
	- USB camera above 2nd monitor; background app w. keyboard wake controls
	- Standalone monitor/camera  (wall or table mounted)


	HOW:
	- REPUTATION - systems strives for fixed num p2p per day.  ?someone looked?

##### -- How it works --

	- CONTROLS
		- DOOR OPENING
		- VOLUME & MIC/CAMERA
		- 

	- SETUP -- 
		- BUILDING STRUCTURE -- Who will you regularly see?  How often?  
		- DAY STRUCTURE -- List of activity types & interaction pattern for each
	
	- IN-BACKGROUND
		- FADE IN -- when not using monitor, and settled
		- INDICATE -- 

	_
#### --- 2020.08 - LOOSE COMMS ---

	**LOOSE COMM STATES**
	- **INACTIVE** -- Loose comms are not active, no ambient info is provided
	- **BACKGROUND** -- Loose comms interface is active, but in background mode
		==> Can be small on ipad or screen.  full audio 
	- **ZOOMED** -- Loose comms interface takes up full large screen display
		==> Gaze tracking used to control loose comms
	- **UPGRADED** -- Loose comms are upgraded into a dedicated video chat

	TRANSITIONS
	- ACTIVATION -- Inactive comms are activate when:
		- User is heads up, has idle display, and has open door 
	- ZOOMED -- A backgrounded user looks at the comms display for a few seconds it will become zoomed.
	- UPGRADE -- Explicit use voice, gesture, mouse, or keyboard action is used to upgrade a zoomed comms into a dedicated video chat with the active chat group.
	- DOWNGRADING -- Loose comm state downgrades occur when:
		- Dedicated chat downgrade -- Requires explict exit voice, gesture, mouse, key
		- Zoomed comms downgrade -- once user looks away from display for 2 seconds
		- Background comms downgrade -- 
			- when user moves mouse onto display to use it
			- when user closes door 

	Behavior
	- LOCAL COMMS -- while active the local comms are presented as avatars chatting
	- PROXIMITY -- avatar proximity shows who is chatting with who is chatting w who

	- AUDIO -- by default audio is auto down modulated to avoid in-room conflicts


	ZOOMING CONTROLS
	- HEAD TRACKING -- User head position is used to scroll loose comms region
	- GAZE -- head scrolling is used to zoom in on a single avatar during active control
	- RESIZE -- while looking at an avatar it may be scaled up or down by vertical head motion, this changes their volume and clarity.
	- AUTO -- Intelligent resizing is also performed based on viewing, and prior learned controls as well as other agent behaviors.  
	- VOLUME -- Global volume control is also available on touch screen and it affects both volume and number of loose comms presented 

	AVATAR
	- GAZE -- avatar gaze:
		- When a worker is looking at another this is shown by having their avatar in profile and looking in 2D at other avatar.
		- When worker is looking at YOU their avatar is shown face you and avatar's brightness and saturate are popup up a bit so it draws attention.
		- When a worker is not looking at any other their avatar is looking down.
	- STATUS -- Worker status is presented by their avatar
		- SPEAKING -- speech rings (like a sideways wifi symbol) are used to show avatar speaking.
		- WATCHING -- a worker looking at the zoomed display is shown by profile avatar aiming off (but not at another avatar)
		- GAZING -- a worker gazing at another is shown as described above
		- FOCUSED -- a worker that is focusing on a single activity is shown as looking down at avatar desk in front of them w only small occasional avatar shifts
		- LEAVE -- when an worker downgrades their loose comms visiblity on this floors they are shown as walking off the edge of the screen.
		- UPGRADE -- when workers upgrade to a dedicated room they are shown as leaving together.
	optional 'full info' avatar
		- MULTITASKING -- a worker that is multitasking with gaze moving from one area of their physical office to another is shown as an avatar that shifts gaze back and forth in random pattern over desk
		- ON PHONE -- synchronous voice / video chat is detected and shown as avatar holding phone to its head


	DEDICATED CONTROLS
	- DOOR -- when in dedicated meeting all participants share a common door.
		This door controls visibility and access to this meeting by others:
		- BOLTED -- room is locked and cannot even be seen from outside.
		- LOCKED -- like bolted but other can SEE participants, and can KNOCK to enter.
		- CLOSED -- like locked but others can open door to enter.
		- CRACKED -- like closed but others can hear into room for short period.
			During their listening their avatar is visible at the door
		- OPENED -- liked cracked but others can listen indefinitely 

	

 

	_
#### --- 2020.08 - PERCEPTUAL TERMS ---

##### -- Display In Use --

	- GAZE -- Head tracking is used to track roughly where a worker is looking
	- LOOK AT -- Work has looked at a monitor if they focused for at least 3 seconds
	- USED -- A monitor is _used_ each time user interacts via mouse/key with window on it
	- IN USE -- A monitor is _in use_ if used and user has not looked away for > 5 min
	- IDLE -- A monitor is said to be idle if not in use

##### -- Heads Down --
	- FIXED GAZE 
	- FIXED PATTERN
	_
	_
#### --- 2020.07.00 - COVID DESIGN ---
##### === FEATURES ===
###### --- VALUE PROPS ---
			- **PRESENCE** -- Non-creepy, pervasive, background presence with team.
			- **NO-SETUP** -- Zero setup interactions with sliding connectivity.

			- NON-CREEPY -- People using more than slack, cause it not creepy.
			- SECONDS TO TALK

			- **REACHABLE** -- Team is reachable, w/o annoyance, or lost privacy.

			- OPPORTUNISTIC -- 

			_
###### --- DIFFERENCE FROM TELECOMMUNICATIONS TODAY ---

			**NON-CREEPY**
			- CONTROL -- user have fine-grained control over what is seen
			- VISIBILITY -- cannot "spy" -- cannot gain info about a user w/o them knowing who you are, and that you looked.
			==> you _**could**_ use slack/zoom/skype for continuous interactivity, 
				but **_nobody would_** since it is way too creepy.  
			==> Many details here lower creep factor, thus gain continuous interactivity.


			**CONTINUOUS AWARENESS DURING DISCUSSIONS**

			- ATTENTION/DISTRACTION LEVEL -- 

			- **MUTE FUMBLE** -- For privacy and disruption reasons one does not want to be always broadcasting audio.  But this leads to frequent 'mute fumbles' wher
				- You think you are being heard, but you are not
				- You cannot interrupt speaker effectively/timely
				- It just takes time/effort to keep switching back and forth.
	
			- RAISED HAND --


			**=== UNDER ONE SECOND ===**

	
			- **STALE STATE** -- You are not trying to hide anything, but you are busy, so your status is never upto date.  People really need ping you everytime in order to understand what it up.

			- **CATCH YOU** -- Others can non-creepy see when you are transitioning between activites without any effort on your part and can opportunisitically "catch you".


			=== UNDER TWO SECONDS ===

			- GO SOMEWHERE
			- PEEK SOMEWHERE



			- **ASK AROUND** -- Let joe know I stopped by looking for him.
	
			- **PAGER** -- Reliable attention getting.
				- Walk from desk.


			=== UNDER ONE MIN ===

			- **WANDER AROUND** -- What is everyone doing?
				- Swipe over an area of the map, then "walk" that path to see what it up


			NON-CREEPY AWARENESS

			- solid control of info out.
			_
##### === SOLUTION DESIGN ===
###### --- KEY INTERFACE PARTS/FUNCTIONS ---

			BOTTOM ROW, ALWAYS-ON INTERFACE FUNCTIONS ON TABLET
			- **TILES** -- Per person squares to see/control those in room
			- **SLIDER** -- Visibility within place
			- **LOCATION** -- Named locations within hiearchically mapped world
			- **MIC** -- Tap-based "Mic" control
			- **A/V** -- Control of audio/video and second monitor
			- **ONAIR** -- Lightbulb indicating when A/V sending from user

			KEY PARAMETERS
			- PRESENCE -- What visibility and info outflow from this user.
				- CLAIMED STATUS, ACTUAL STATUS, HISTORY, LOCATION
				- QUALITY audio/video
			- INTERRUPTABILITY -- 

			_
###### --- TABLET PARTS ---

			TABLET - $200-400
			- TABLET -- Dedicated Android App on touch screen tablet
			- AUDIO -- Headphones/Speakers & mic
			- VIDEO -- Connection to second monitor
			- CAMERA -- Webcam mounted on second monitor

			MONITOR
			Second monitor for use w. computer or connectivity
			- CONTROLLED -- Video input source is controlled:  computer -or- REM-PRE

			COMPUTER
			- ZOOM -- Zoom under remote control. 
				- Join zoom; change screen size/location; share screen

			OPTIONAL
			- MS MIRACAST -- 
			- HEADSET -- Airpods, or wired headest with quality audio/mic 
			- BELT VIBE -- 
			- $20 		[HDMI 4K SWITCH](https://www.amazon.com/SGEYR-Splitter-Switcher-Switches-Selector/dp/B07VPD3BXC/ref=sr_1_6_sspa?dchild=1&keywords=Dell+monitor+%22remote+control%22&qid=1586455827&s=electronics&sr=1-6-spons&psc=1&spLa=ZW5jcnlwdGVkUXVhbGlmaWVyPUExVEI4SVJPTzA5MTNDJmVuY3J5cHRlZElkPUEwODIwNzAxMklLOFFUSjYzSjBOVCZlbmNyeXB0ZWRBZElkPUEwNjUxODA4NVVWT0Y5WU5TVU1WJndpZGdldE5hbWU9c3BfYnRmJmFjdGlvbj1jbGlja1JlZGlyZWN0JmRvTm90TG9nQ2xpY2s9dHJ1ZQ==) -- $20
			- $50-200	[DOCUMENT CAMERA](https://www.amazon.com/s?k=Elmo+document+camera+writing+board&i=electronics&ref=nb_sb_noss) [IPEVO $100](https://www.amazon.com/IPEVO-Definition-Document-Camera-5-880-4-01-00/dp/B079DLTG9F/ref=sr_1_1?dchild=1&keywords=Elmo+document+camera+writing+board&qid=1586461183&s=electronics&sr=1-1)  
			_
##### === FEATURES DETAIL ===
###### --- PRESENCE ---
			STATE
			Interruptable	Break, Work, Headsdown, Really Headsdown, Better be dying
 

			Away    --  
			Quiet	-- 
			Talking --  
			Busy -- In meeting/call, away from desk


	 
			_
###### --- TAP MIC CONTROL ---
			- AUDIO OUT IS AUTOMATIC -- Others can interrupt, and push audio/video
			- AUDIO IN -- User is always in control of A/V inputs from them.
			- TTT -- Tap to talk is available from tablet

			**TAP CONTROL**
			- Control and state is always visible on tablet
			- Touching control brings up swipe menu

			**TAP**
			- **OFF** -- Avoids accidental audio out
			- **MUTE/TTT** -- Tap-to-talk allows quick response w/o keeping channel open
			- **STICKY** -- Opens mic for specific time in current location.
			- **VOICED** -- Mic opens when you are speaking.
			- **ON** -- Mic is open in whatever room you are in.

			_
###### --- SLIDER ---

			As you adjust your slider, your tablet screen momentarily shows what your door will look like for others.

			As you slide, sub-status swipes appear above your selected state

			You can remove status types you don't like
			You can control how automatic you slider chan

			SLIDER STATES
			- DARK -- Zero info about state or claimed state
			- AWAY -- Claimed 'away from work' status
			- AWAY FOR THE DAY
			- AWAY FOR A WHILE
			- AWAY FOR A MINUTE
			- AWAY FOR AN ACTIVITY (Lunch, Client, )
			- AT WORK -- Claimed 'at work' status
			- 
			- IN OFFICE
			- IN BREAK	-- In office; open for interruption; and taking a break
			- IN OPEN	-- In office; open for interruption
			- IN BUSY	-- In office; "heads down" working on something
			- IN TALKING -- In office; talking with someone
			- IN MEETING -- In office; in a multi-person meeting.
			- 


			- AUTO -- Basic state info updated automatically (keyboard, sound, etc.)
			- INTERRUPTABILITY -- By default others can interrupt, and user can squelch. User can set global interruptability level, others can shout louder.

			STATUS IMAGES -- Status names is also shown in corner of image
			- DARK		-- Closed door with question mark
			- AWAY 4DAY	-- Closed door with no light coming from below the door
			- AWAY 4WHILE-- Closed door with light comming from below the door
			- AWAY 4MIN	-- Open door with empty room but lights on
			- BREAK		-- Feet up
			- OPEN		-- At desk facing door looking down at someting
			- BUSY		-- At desk facing away from door "heads down"
			- HEADS DOWN	-- Busy with big headphones on head.
			- DONT BUG	-- Busy with mean looking dog w. teeth at door
			- REALLY DONT-- Busy with two mean dogs
			_
###### --- CONNECT ---
			- THREE OPTIONS
				- GO -- Leaves current location to go to another
				- JOIN -- Stays in current location by adds person/group
				- PEEK -- Stays in current loc, but "peeks" at another

			UX FLOW
			- TAP 		Go/Join/Peek
			- DRAG		Drag select person/location
			- SEE		See status of person:  Gone / Busy / HeadsDown / Available
			- RELEASE	Release over action to execute
							Call, Text, Request talke NOW/soon/hour/afternoon/today,asac

			_
###### --- LOCATION CONTROL ---

			Map location shown in bottom center tile; 
			Touching map converts entire tablet into locacation control

			Location Control
			- XXX ROW1 is scroll bar -- right and left controls velocity of scroll
			- ROW1 is toplevel map linear maps of places 4-16 choices	(4-16 total)
			- ROW2 is more detailed level of places  4-16 choices.  		(16-256 total)
			- ROW3 is most detailed map of places.  4-16 choices.  		(64-4096 total)

			Single swipe location travel:

			- Touch location map in order to view location control (finger in top center)
			- Swipe up into first row -- 
				- second and third rows jump to "location of finger"
				- then rapidly scroll over whole map in greater magnification
			- Swiping up into second row 
				- freezes second row in place while third rows keeps moving
			- Swiping up into freezes all rows
			- Selector vertical line follows finger and is always selecting room indicated in the third row

			_
#### --- 2020.06.00 - COBBLED DESIGN -- Pragmatic Tools for current use

		-- One click
		---  Share Voice, Face, Screen

		---  Connect to existing meeting.

		- Use gVoice to call TO phone
		- Use webex to call TO phone

##### Basic approach
		- Always in persistent chat.  Use this for status.

		- Make phone call

##### WebEx


##### LogMeIn
		- Easy cold-machine screen sharing

##### Existing Tools for Collab

		http://blog.braidapp.com/working-remotely-and-the-tools-that-make-it-possible

		_
### === SOLN TOPICS ===
#### --- Solution Logic ---


	HEADS_UP ==> HUD_AWARE


	SYSTEM MODES:
	- HUD AWARE	-- HUD is providing situational awareness
	- NAV MODE 	-- In HUD or PAD NAV mode



	USER ACTIVITY MODES:
	- AWAY 		-- Not at desk area
	- ON TASK 	-- Doing a task:  Using laptop (mouse/keyboard, gazing at it), Focused on desk.
	- ON CALL 	-- In synchronous communication
	- HEADS UP 	-- At desk but not on task or call for a period of time (~5min)

	DOOR MODES
	_
#### --- Solution Components ---

	PAD -- Dedicated tablet at the center of users control of the remote presence environment.  Pad rests on user's desk at 60-degree angle on heavy base that allows touch screen manipuation.

	HUD MONITOR -- A second computer monitor resting on user's desk.  The HUD may be temporarily taken over for remote presence activities.

	CAMERA MODULE -- The camera module rests upon the HUD MONITOR is provides user's front facing camera, microphone as well as head/body pose tracker

	POSE TRACKER -- Tracks pose & postion of head & torso.




	DOOR SLIDER -- The door slider is permenently displayed at the bottom of the PAD, and that pad itself centrally shows the state of the door itself (e.g. closed or open showing others)

	_
#### --- USER MODES ---

	**USER MODE** -- A coarse-grained description of what the worker is doing in a given moment.
	_
##### -- LAPTOP MODE --

	**LAPTOP MODE** -- Worker is actively interacting with their laptop's interfaces via mouse or keyboard. 
	Or worker is steadily gazing at laptop or second monitor.

	_
##### -- CHAT MODE --
	CHAT MODE -- User is actively engaged in a synchronous conversation involving voice.

	This includes: Phone, Video Chat, In-person discussions.
	Some of these modes are a "carveout" of laptop mode.

	SENSORS:  
	- Ambient sound + user voice print recognition
	- Laptop activities -- Active video chat program.
	_
##### -- HUD AWARE MODE --

	HUD AWARE MODE -- Worker is not deeply focused on a task at hand, or a conversation and is thus more free to have awareness of the larger organization around them.

	- HUD 		-- The hud is providing situational awareness.
	- OP COMMS 	-- The hud is also providing opportunistic exposures.
	- FOCI		-- 

	EXIT
	- LAPTOP:  	Worker uses keyboard, or moves mouse off the HUD monitor
	- CHAT:		Worker begins synchronous chat via some channel
	- AWAY: 		Worker leave the desk area
	- 

##### -- OP COMMS - Opportunistic Comms --

	**OPPORTUNISTIC COMMS** -- An _opportunistic communication_ is pairwise human-to-human interaction that (1) never interrupts a user's activity (2) is created w/o user intent and (3) is completed without conscious effort.

	E.g. Asking if someone needs help on a report as you pass them on the way to get a drink.


	**KEY FEATURES**
	- **INTENTION FREE** -- Neither user has the intention of communicating prior to the communication opportunity being presented.  (like walking to the bathroom and incidentally running into someone.)
	- **UNCONSCIOUS** -- Awareness of the opportunity to communicate occurs w/o conscious effort.  Further if communication is desired, no conscious effort or action is required.  (No click, no genture, Just say "hello")
	- **QUICK** -- After the formation of intention to communicate actual communication begins less than 250ms later.
	- **FREQUENT** -- Core collaborators will have many 5-20 interaction opportunites per day
	- **PAIRWISE** -- Opportunistic comms are always between two users
	- **INTERRUPTION FREE** -- Opportunistic comms are arranged so they never interrupt either user.
	- **CONTROLLABLE** -- The ways, time, and users one might have opportunity to communicate with has defaults that are controllable by the user.
	- **CLOAKING** -- System is arranged to minimize general visibility others have about ones settings and the state of ones working.  (can "fake it" if you want!)
	- **SYMMETRIC** -- Anytime inforamation is divulged about a user they know what information was seen, and who saw it.


	IN-PERSON CONTEXT -- How opportunistics comms occur in the in-person setting.
	- OPPORTUNITY -- Offices are arranged so that entering, leaving, bathroom, eating all causes opportunites to occur where pairwise communication might happen.
	- 


	REMOTE CONTEXT -- How are opportunitisic comms accomplished in the remote setting.
	- SETUP -- Each user indicates periods how/when system should assume user is heads down and not interruptable.  They also specify which indictors should be used as signs they may want information about others, or allow interruptions from others.
	- "DOOR" SETTING -- A dedicated tablet displays and provides control of the user's door from closed to open -- this indicates how visible user is for other's to notice.
	- TRANSITION -- User's natural transitions are detected by tracking both user and devices (e.g. head/body/phone/laptop tracking)
	- RESPONSE -- User's response to passively provided information is passively tracked (using same sensors)
	- ATTENTION -- Head and torso tracking is used to indicate
	- SPEAKING -- User's voice characteristics are used to determine when speaking


	REMOTE CONTEXT STATUS SENSING
	1. GET UP -- In the seconds when user stands but has not left desk area.
	2. RETURN -- The period when user has first returned to their desk area.
	3. PAD LOOK -- User has gazed at their dedicated team pad for >5 seconds
	4. HUD LOOK -- The HUD is active and user has rested gaze on it.
	5. PHONE -- User has started/stopped using phone call or computer-chat


	COMM STEPS
	1. All users have set interruptions / visibility rules (updated over time) 
	2. Seeker's sets door to indicates desire to be aware of others
	3. Sought user sets door to indicate desire to be visible but working
	4. Sought user executes transtion (like returning to desk or gazing at pad)
	5. Seeker's PAD or HUD momentarily shows cartoon animation of photo of sought user.
	6. If seeker speaks while cartoon is animated then users comms starts
	7. If seeker touches user icon shown on side of pad for 30 second then comms starts
	8. Comms are supported thru the audio/video of the pad
	9. Comms are terminated by clicking end or if both users look away for period of time
	10. Comms are upgraded by clicking room/skype/zoom/call etc.

	_
##### -- MULTI COMMS - MULTI-CONVERSATION ROOM --

	**MULTI-CONVERATION ROOM** -- A _**multi-conversation room (MCR)**_ allows many dozen of users to fluidly move between multiple non-overlapping simultaneous conversations between different subgroups of users in the MCCR.  Each "conversation" (channel) has isolated audio allowing simultaneous speech, while actively encouraging in-channel turn-taking speech.

	KEY MCCR FEATURES
	- AWARENESS -- Users within an MCCR are generally aware of all comms happening in MCCR
	- MULTI-CHANNEL -- The MCCR is dynamically partitioned into multiple converstion communication channels.
	- SELF -- Users are aware of which other users are listening to them.
	- DYNAMIC -- The number of channels and user assignments to channels is changing continously over time.
	- UNCONSCIOUS -- Key MCCR actions occur without taking conscious action:
		- Channel creation / Channel switching / Channel termiation.
	- SMOOTH -- Transition between channels is smoothly moves thru states where user is partly in and can hear multiple channels.


	PARTS OF THE MCCR ENVIRONMENT
	- FOCUS -- the center of the HUD is region called the "focus" 
	- RECENCY ROW -- to the left and right of focus are 4 positions called the recency row
	- PERIMETER -- outside of the focus and recency row is the perimeter
	- GLYPH -- users and things in the room are each indicated by a glyph
	- CHANNEL -- users in the room are partitions into comm channels


	KEY ACTIONS
	- FOCUS ON CONVERSATION -- head-point to conversation until it highlights then moves it to the focus row.
	- FOCUS ON USER -- use head-pointing or mouse to indicate a specific user.  This creates a side channel shared with that user

	- ZOOM IN -- user may zoom in on any conversation or individual by mouse or head motion
	- ZOOM OUT -- user may zoom out by dipping head down
	- FOCUS -- continue looking as user once they have migrated to center of HUD
	- 

	FOCUS -- 
	MOUSE CLICK -- User may  mouse click on any glyph to directly focus on it.
	Alternately head tracking alone may be used to change focus:
	- HEAD TRACKING -- Head tracking is used to identify screen region being attented to.
	- DISTINGUISHABLE REGIONS -- Distinguisable include:
		Focus, Four distinct recency slots, Six non-recency regions (3 upper & 3 lower)
	- ZOOM -- Use head motion or mouse click to indicate a new focus:
		- CENTER -- If user head-points at HUD center for several seconds then that thing becomes the user's "focus"
		- RECENCY ROW -- If a user head-point to any of the four recency slots that slot will highlight and after several seconds will "zoom" to the center.
		- PERIPHERY -- If the user head-points to any of the six periphery regions that region will highlight and the user will hear "room chatter" from that area.  After several seconds it will "zoom" that region to become the center:
			- If only one channel is highligted then that channel become active and it current speaker the center and recent speakers in recent slots.
			- Otherwise all channels and users in region are divided into 5 groups and moved into the center and 4 recency slots.
			- Recency-row head-pointing can be used to progressively zoom interface.



	COMM CHANNEL
	- PARTITIONING -- Room is partitions into a dynamic collection of communication channels.
	- PRIMARY CHANNEL -- Each user is in at most one channel -- their "primary" channel.
	- SHARED AUDIO -- Audio from all users within a channel is broadcast to all in the channel.
	- FOCUS SHIFTING -- By default all users within a channel are focused on the user speaking within their primary channel.
	- RECENCY ROW -- The most recent speakers are show in the recency row
	- HEAD FOCUS -- Using head motion 
	- FOCUS
	- SECONDARY CHANNELS -- Each user may have one or more secondary channels.
	- CHANNEL JOINING -- Once a user focuses on a another user for a period of time (e.g. 12 seconds) they "join the same channel" that user is in.
	- 
	-  channel and will hear anything spoken in that ch


	ROOM CONTEXT --
	- ROOM -- Multi-channel comms are executed in a multi-comm room, or "room" for short
	- HUD -- Room is displayed on user's HUD.
	- FOCUS -- The center of the hud is called the "focus".  When a user is looking at the focus area and a user or info is displayed there that is called "focusing on" them or that.
	- CLIQUE ROW -- The area to the left and right of the focus is called the "clique row" and it contains live images of the last 4 users/things the user focused on.
	- GLYPHS -- Each user and info page within the room is indicated by a glyph.
	- CLUMP -- Non-clique-row Glyphs are organized on edges of screen near other glyphs they have recently been focusing on.
	- AUDIO -- All audio from 


	AUDIO 
	- AUTO MUTE -- By default each user is auto muted when they are not speaking (system specific listens for that users voice pattern to determine this.)
	- CLIQUE ROW -- Audio from all five participants is presented at full volume at all times that those users are broadcasting.
	- MUMBLE -- Audio from all other glyphs in the room can be heards at a user setable "mumble" level.  The idea is to provide awareness of speaking tempo of groups around the room without sufficient volume to be clearly understood or to interfere with conversation within the clique group.  
	- CHATTER -- If no one is speaking for a period within the clique group then the sound from the other speakers is increased and mixed room "chatter" is provided.
	- SILENCE -- ???


	GLYPH 
	- WIDGET -- Each user or "thing" in the room has an easily distinguished glyph.  This includes each user in the room or info glyphs (shared screens, presentations, medalian etc.)
	- LIVE -- Each glyph has a "live" form.  This is some video feed of some potentially changing image.
	- ICON -- Each glyph also has an iconic form.  This mostly-static form may shift a bit on occasion to provide state information.  But mostly these have an headshot or other image and word that distinguishes it easily from other icons.




	A user may focus on any glyph shown within the multi-comm room

	- so that where distance by recent focusing actions 

	- or user has a glyph

	- The last 5 users a user has focused. have focused on are in displayed on  clique and there images are displayed on the users CLIQUE ROW in their HUD.
	- LOCATION -- Each user engaged within a multi-channel comm will occupy a 


	ROOM MEDALIAN / ROOM MAP
	- FOCUS GRAPH -- If a user focuses on another user or glyph for a dozen seconds or more, then they are "listening to" that user.  After a dozen second of focusing elsewhere they are no longer focusing on that user.  (thus each user may be "listening to" at most one other user at any given time.)
	- MAP -- The room medalian is a logical map of the recent comm patterns within the room
	- GRAPH -- Shows vertex for each user and link to the user they most recently were focusing on.
	- POSITION -- User verticies are place separated from others, but close to the user they were recently focusing on.
	- MOVEMENT -- Vertices move over time as their focusing patterns change.
	- FULL MAP -- If a user focuses on the medalian glyph the HUD switches to the full screen room map, with each user shown as a glyph not just as a vertex.
	- ROOM CHATTER -- while focusing on the room medalian or the full map the audio channel is users audio is set to room chatter.  


	SINGLE CHANNEL COMMS
	- FULL DUPLEX -- System provides "full duplex" multi-way audio between all participants



	PERCEPTION
	- GAZE -- where is each user looking at each moment in time.
	- MOVEMENT -- logically "where" a user is within the multi-comm room


###### ,

	- FOCUS -- A user is "focusing on" a glyph when it is in the centeral focus area, and when the user has their head pointed at that center area for a couple of seconds.
	- CLIQUE ROW HIGHLIGHTING -- When a user moves their head horizontally the system tracks and immediately highlights any of the 5 images shown in the clique row as the user head moves.
	- CLIQUE ROW TRANSITION -- After a second or so the clique rows starts drifting slowly and smoothly to bring the highlighted person to the center focus area.  
	- QUADRANT -- If user points head above or below the clique row then the appropriate quadrant is highlighted and audio from the quadrant is momentarily made primary.  If user maintains head position for several seconds in that area then screen drifts to take up whole screen
	_
##### -- GAZE COMMS --
	- Use head pose and scaled motion to control what you are looking at
	- Use highlighting, animation pose to indicate when others are looking at you.

	_
##### -- THE DOOR - THE USER'S DOOR --

	USER'S DOOR --"DOOR" SETTING -- A dedicated tablet displays and provides control of the user's door.  


	DOOR SLIDER
		It can vary from 

		- GONE	-- User is not in their office
		- CLOSED	--	
		- CRACKED--	
		- HALF	--	
		- OPEN	-- 
		- COMMON	--	User sitting outside in common area

	GONE:	for the day.  for hours.  for minutes.	for seconds.
	CLOSED:	Sealed; Locked; Closed; Screened; Cracked; Half
	OPEN:	heads down, working, interruptable, seeking interaction; sitting in hallway

	_
##### -- BOWTIE NAV --

	Goal:  Visually navigate a graph


	**NAV OFF**
	- While nav is off, the pad shows the door, and HUD shows current place in full screen
	  TOUCH			Place finger on pad's main view region to activate "place navigation"
				    => Instantly place nav boarder appears

	**PLACE NAVIGATION**
	- The pad full screen view shrinks by 20% and nav boarder is shown around edge.
	- All graph locations connected to this place are shown.
	- At top it says "ZOOM IN"
	- At bottom is says "ZOOM OUT"

	  PULL TO		Drag entire pad view to pull desired boarder item to center to select it.
	  PULL BOTTOM	Draging straight down off bottom will "nav off" in current place
	  RELEASE		Simply relasing from pad will also turn nav off after 500ms
  
	  PUSH UP		Drag up to "push away" from current "place" into head scroll mode
	  PUSH TOP		Drag up to top to fully "push away" from current place "back" to prev


	**HEAD SCROLL MODE**
	- Horizontal row of items with center magnification is shown
	- Above and below the scroll row are the other target nav places

	  HEAD SCROLL	Pose head to HUD center; move head left/right small amount; pause 250ms
	  NON SCROLL	Moving head outside cone of HUD, or not pausing will cancel move	
	  PULL DOWN		Drag down on the track pad to zoom back into place navigation
##### -- NonSpying Awareness --

	- NON-SPYING -- Awareness that others have about you does not feel like spying on you.
		- SYMMETRIC -- They cannot gain info about you w/o you knowing they did so.
		- CONTROLLABLE -- 
	- AWARENESS 
		- GAZE -- Are they looking specifically at me at this moment?
		- 

	_
###### --- Max Info / Min Exposure ---

	- PAIRING -- need to maximize opportunities for informal comms when both are free


	- Typing/mousing/sound/phone
	- Explicit focus slider

	_
#### --- SENSOR TYPES ---
#### --- Head tracking math ---

	**VARIABLES** 
	fx	Horizontal position relative to focused item.  [-1, +1]
	fy	Vertical position relative to focused item.  [-1, +1]
	fm	Magnification of focused item relative to nominal magnification of 0.  [-1, 1]
		-1 means zooming out, and +1 means full screen

	bw	Bowtie width.  	1=just knot, 2=bowtie(4), 3=bowtie(5), 4=bowtie(12), 5=bowtie(13),
						6=bowtie(28), 7=bowtie(29), 8=bowtie(60), 9=bowtie(61),10=bowtie(124)

	cx	Center region size +/- this amount  (0,1)
	cy	Center region size +/- this amount  (0,1)

	px	Normalized head horizontal position [-1,+1]
	py	Normalized head vertical position [-1,+1]
	vx,vy	Velocity of head
	ax,ay	Acceleration of head




##### -- GAZE --

	- HEAD TRACKING -- Use head tracking to determine if user is looking at:
		- At Desk
		- At HUD/second monitor  (11 different positions)
		- At Pad
		- At Laptop
	_
##### -- ATTENTION --

	ATTENTION -- where is the user's focus of attention at any moment in time.
	Determined via multiple input sources.

	ATTENTION IS ON:
	- ENV (the surrounding enviornment, walls etc.)
		- DOOR
		- WINDOW
		- CEILING, HIGH UP
	- DESK  (work surface)
	- PHONE
	- VIDEO
	- COMPUTER
		- TYPING
		- MOUSING
		- PRIMARY SCREEN
		- SECONDARY SCREEN. (same as HUD)
	- HUD  (11 different positions)
	- PAD



	SENSORS:
	- GAZE
	- KEYBOARD
	- MOUSE
	- USER'S VOICE / sound in the roomoh 

	_
#### --- OPP COMM SETUP ---

	IPAD -- iPad on rigid tripod shows comm possibilities

	HEAD TRACK -- 
	- determines if one is looking at IPAD
	- used to position self within virtual-comms room


	OP COMMS EXECUTION
	1. **DOOR** -- User adjusts "door" to indicate their interruptability and how much they see
	2. **TRACKING** -- System tracks users head pose and ambient sounds to determine moment-to-moment focus.

	_
### === DETAILS ===
#### --- HEAD TRACKING ---

	IR Head Tracker -- https://www.youtube.com/watch?v=M_VN4mMA6Cw


	_
#### --- REF ---
	[Kinds of meetings](https://www.google.com/imgres?imgurl=http%3A%2F%2Fblog.lucidmeetings.com%2Fhubfs%2Fno-excuses-series%2Fmeeting-types-chart.png&imgrefurl=https%3A%2F%2Fblog.lucidmeetings.com%2Fblog%2F16-types-of-business-meetings&tbnid=ldmvB5kWLVaPnM&vet=10CDYQMygZahcKEwjIj5q70trpAhUAAAAAHQAAAAAQAw..i&docid=so2WxB5NE0AoRM&w=1871&h=1105&q=virtual%20informal%20communication&ved=0CDYQMygZahcKEwjIj5q70trpAhUAAAAAHQAAAAAQAw) 

	[Informal comm in orgs ; crusier](https://pdfs.semanticscholar.org/60c5/25c2beee3d1820c020b7e880b46ca91f5685.pdf?_ga=2.81593495.182898066.1590809752-242610115.1590809752) 
	- [cruiser 1992](https://dl.acm.org/doi/pdf/10.1145/142750.142755) 
	- [Robert Kraut](https://www.cmu.edu/tepper/faculty-and-research/faculty-by-area/profiles/kraut-robert.html) 




	_
### === OPTIONAL PARTS ===
#### --- VIRAL ADOPTION MODEL ---

	Goal:  get this system to scale virally

	DESIGN
	- Email invite, one click install, preconfigured connection
	- Operates on second monitor


	CONNECT
	- One click to add new friends to your "floor"
	- Even non users * Multi-channel invite
	- One click invite friend of friend to share floor

	INSTALL 
	- One click install
	- Preconfigured connection of inviting person


	OUT OF BOX
	- WATCH -- non-use of second monitor
	- DOORWAY -- fade doorway in, when monitor not in use
		???
	- CARTOON OP COMM
	- CHATTER COMMS
	- ZOOM UPGRADE

	PRIVACY
	- Verified connection to existing social media accounts (FB, Instagram, Linkedin) to verify identity
	- Obfuscation in connecting rates to hide freind level
	- can't tell if comm was upgraded or stopped
	- anyone looking is VISIBLE doing so


	quiet downgrade of individuals
	maybe need to actively do stuff to keep person on same floor


	Dennis Hummel, a doctoral researcher at Karlsruhe Institute of Technology in Germany whose research focuses on digital nudges, says defaults are one of the better ways to nudge people in one particular direction. One 2016 study, he says, “used defaults in emails by automatically signing up individuals for vaccine treatments, thereby doubling the rate of appointments.” 

	_
# LOG
### n2022-05-30  Compressed summary

PROBLEM:
- Remote knowledge work is sometimes less effective than in person work?  (BUT WHY?!)
- Its lack of situational awareness:  You don't know what you don't know, so you cant fix it.
- Two key forms of missing informational flows
	- OP COMMS –  *Opportunities* for synchronous communication that occur with:  
	  (1) no intent, (2) no interruptions, (3) low cognitive effort, and (4) no setup
	  ==> Phone call requires: Intent, Interruption
	  ==> Asking about the sales report when seeing Kim on the way to lunch requires none of this.
	- LOOSE COMMS -- Streams of ongoing, overlapping, partially understood background info.
	  ==> Before meeting
- OP COMMS & LOOSE COMMS aren't just difficult to achieve w/ today software they are impossible.
	- Today's remote work has 1%-10% of the OP/LOOSE comms that in person work does
	- in some cases this reduced info flow makes all the difference in outcomes.

SOLUTION:
- Use sensors & AI to detect
	- Interruptability and cognitive load
- Use algorithms to manage coordination
- Use UX and Statistical obfuscation to maintain 'at home' social contract
	- Cant help co-workers 'spy' on user
	- Cant be creepy


### 2020.08.16 - North Star

Provide perception/action alternatives that enable in-person-like effectiveness for remote knowlege-worker teams.

Key Abilities:


I was an avid Alfred user years ago.  I could not use it as I wished, so have scripted my own solution on top of spotlight.  (by creating one new .app for each command)

But it's hard to get spotlight to do exactly what I want, so I wanted to check if I can fight with Alfred to get what I want:

_
### 2020.07.12 - Saul questions

- OPPORTUNISTIC COMMS --
- GAZE BASED COMMS (COCKTAIL COMMS)
- GAZE 


CTX: paperless knowlege worker

HYP:  These four are central -- and would make a transformative difference
- Op comms is center of non-face2face comms
- cocktail comms
- Add Gaze/Attention/Pointing. 
	I am I hitting 80/20 on face to face

> 
> Other tech; key reserachers; comm frameworks; 

thightly coupled work -- 
loosely coupled work -- 

they focused on John Tom -- 1980 looked at nuanced when working over workspace
shared cursor

-- casual interactions.  
Causal interactions decay with distance. 
What do they afford.  

-- Vertegal 

-- 
From Saul Greenberg to Everyone: (1:35 PM)
Vertegaal
Roel Vertegaal (PhD on that topic)
    company became -- phidgets

nothing -> something (its a big win)   appropriation

F-formation.   http://grouplab.cpsc.ucalgary.ca/Publications/2012-GroupTogether.UIST
group can loose social control

how do we create lightweight interactions between people:
http://www.izix.com/docs/Tang-Montage-CSCW94.pdf

Bill Buckton


~


I have a script that scans my filesystem looking for particular marker files and uses these to create thousands of keyword triggered actions automatically based on the structure of the file-system itself.  In a perfect world there would be an import file format for defining many new Alfred keywords/action pairs, or a command line interface.



Still if that does not exist, I could perhaps directly generate the internal file format or some other gross hack as a much weaker alternative.  Is there any way to do what I want?



This update action should be relatively fast, and should be executable in a headless fashion (no user clicking/typing required).



thanks for the help!

--dan

_
### 2020.07.12 - Saul


saul.greenberg@ucalgary.ca
Subj:  Awareness for Groupware

I am an AI researcher and serial entreprenure that has worked on and off for several years on an approach for supporting remote office work.  It seems "obvious" this approach will work, still I am humbled by the many who have tried and their aparent lackluster success.

I have read your paper on Awareness for Groupware, and it seems you would be in good position to provide me wisdom and pointers.  Wisdom about whether I what am correctly framing the key gaps in existing tools for remote work, as well as pointers to prior work related to my proposed solution.

If you have the time and interest in a short phone call, I would love to get your thoughts.

Thanks in advance,
Dan


https://www.linkedin.com/in/danieloblinger/

_
### 2020.07.12 - Judith   not sent
jdonath@cyber.harvard.edu
Subj:  Casual Collaboration  thru  Signals, Truth, and Design

Judith:

I am an AI researcher and serial entreprenure that has worked on and off for several years on an approach for supporting remote office work.  It seems "obvious" this approach will work, still I am humbled by the many who have tried and their aparent lackluster success.

Many have published work on replicating in-person experience remotely.  I was taken however by early work of yours using cartoon surrogates and word clouds as alternatives for in-person awareness.  Also you seem much more focused on the privacy and sharing implications than many technology-focused folks.  I am convinced the key to remote collaboration is NOT to replicate in-person experience, but rather to identify and provide remote alternatives for key in-person forms of awareness and communication.

It seems you would be in good position to provide me wisdom and pointers.  Wisdom about whether I what am correctly framing the key gaps in existing tools for remote work, as well as pointers to prior work related to my proposed solution.

If you have the time and interest in a short phone call, I would love to get your thoughts.

Thanks in advance,
Dan


https://www.linkedin.com/in/danieloblinger/
_
### 2020.07.12 -  not sent - left field - found in linked in

nienke-n@nada.kth.se
Subj:  Remote collaboration and your thesis work

I am an AI researcher and serial entreprenure that has worked on and off for several years on an approach for supporting remote office work.  It seems "obvious" this approach will work, still I am humbled by the many who have tried and their aparent lackluster success.

I have read a paper of your papers and skimmed your whole thesis, and it seems you would be in good position to provide me wisdom and pointers.  Wisdom about whether I what am correctly framing the key gaps in existing tools for remote work, as well as pointers to prior work related to my proposed solution.

If you have the time and interest in a short phone call, I would love to get your thoughts.

Thanks in advance,
Dan

_
### 2020.06.30 ---

scherlis@cmu.edu  Subject:  Ancient history... your CHI98 paper.

William,
It seems we are both vetrans of DARPA, though I am a decade your junior in doing my stint there.  Those were some impactful years!

But that is not why I am writing...  It is your CHI 98 paper on Alternative communication patterns in the home using the internet.  I have struggled to find more recent work by you or others, but have not succeeded.

I am an AI researcher and serial entreprenure that has worked on and off for several years on an approach for supporting remote office work---a few hardware and configuration tricks to provide remote alternatives for traditional "in person" forms of awareness.

If you have time for a 5 or 10 minute call I could show an image or two that would provide context for giving me pointers to relevant reserachers and more recent work.

Let me know if you have time and interest,
--Dan


_
### 2020.06.14 - Vanilla Email

XXXXX:

I am an AI researcher and serial entreprenure that has worked on and off for several years on an approach for supporting remote office work.  It seems "obvious" this approach will work, still I am humbled by the many who have tried this in the 90s and their aparent lackluster success.

I have read a few of your papers on the topic, and it seems you would be in good position to provide me wisdom and pointers.  Wisdom about whether I what am correctly framing the key gaps in existing tools for remote work, as well as pointers to prior work related to my proposed solution.

If you have the time and interest in a short phone call, I would love to get your thoughts.

Thanks in advance,
Dan



_
### 20??.??.?? - Discussion with Ruwan about solution tech
Hey Dan,

Sorry this took so long to get back to. I got on the plane back to the UK soon after we spoke, and everything else got jumbled up.

My gut reaction to the remote workstation/videoconference idea is that if you can't actually share the desktop, rather than seeing a representation of it, it will be disappointing to the user. If that's the case, I would look into VNC for the desktop part. The downside to this, of course, is needing to run a VNC client on the remote user's computer, not just do everything on the appliance. There are open-source VNC clients for all the major OS's, I think. But you have the get the user to install it.

If that's not acceptable, then go back to taking the HDMI or other video feed into the appliance, which would also have a built in camera. Then, use two H.264 codecs running in software on the appliance. One would encode the camera at relatively low resolution (640x480 even) but high frame rate (30 fps). The bit rate for this would be somewhere in the neighborhood of 1 Mbit/sec, probably. And the other codec would run at a high resolution, but low frame rate, for the screen itself. The video compressor should, I think, be able to get a lot of compression out of the large regions that are all the same color. You will need to run the compressor at a high quality setting to keep crisp edges, but the screen should be pretty static, so the bandwidth can be recovered by using a very slow frame rate, even 1 fps.

You may want to do one trick, which is to animate the cursor separately. It might be annoying to have it flicker at a very different rate than the screen updates, but that's definitely a post-MVP thing.

To my mind, this gets you almost all the value of the "extracting glyphs from the video feed" approach, but with much much less work. The glyph idea would leave you still trying to match fonts, rendering buttons and pictures separately, a whole lot of crap that would just go away if you encode the screen pixels directly.

Anyway, that's my thoughts, after letting it roll around in my mind for a while. Let me know if you want to discuss some more - there's obviously stuff I'm missing about your goal. It may be if we circle around this again we get some more insight. But hopefully this gives you something to chew on.


On Thu, Jan 3, 2013 at 11:21 PM, Dan <oblinger@gmail.com> wrote:
Indeed!

This is a sign that you are more a technical guy, than a business guy.
Very dangerous as a startup guy!  we fall in love w. ideas, not the 4 years of execution that follows  :-)


Still if you are thinking the deep thoughts, here are some of the  "a some magic occurs here" parts of the technical spec:

(1) It would be really valuable if this appliance could work out of the box with a wide range of user computers  _AND_ was
      still very efficient with bandwidth usage.  The holy grail would be taking HDMI, DVI, or such as input and working directly from that.
      this is harder, and potentially not as efficient on bandwidth.  Still very very easy for the user.  The alternative would be some sort
      of driver on each computer, but that hits the second issue below:

(2) At least for V1 of this system I would want to use nearly 100% cots/open source stuff for both hardware and software.
      it is just too expensive to develop proprietary stuff before we have investments and customers.  I am assuming that 
      we build customer control code and little else to begin with.

      thus any existing software and hardware components to manage video, streams, audio, scanners, etc would be cool to know about
      my sense is that the core of this would be hardware components and then hardware/software that let me build custom dynamic video streams
      (by merging other streams etc.) would be the bulk of the components I would need.


(3) and I wonder how close could modern codecs come to transmitting ascii, by compressing the character glyphs and screen layout parts
      from a digital image???   in theory it seems like it could get pretty good, but dunno about in practice.
       
(is there some off-the shelf best in breed I could see somewhere, by just looking at a existing device, over regular DSL?)

(4) how the heck might you try to look for customers for such a thing?


anyway, was great chatting!

Cheers, 
Dan








On Jan 3, 2013, at TJan 3  1:09 PM, Ruwan Welaratna wrote:

Indeed! Let me write up some tech thoughts, and we can chat again after. This block diagram/invention phase is by far the most entertaining part of engineering for me!

<<phoned in>>

On Jan 3, 2013 12:37 PM, "Dan" <oblinger@gmail.com> wrote:
Cool, or maybe even a follow up chat sometime, your experience in build video systems is amazingly relevant, given the random reason we were talking.  
the world is like that.

Happy new year,
--dan




On Jan 3, 2013, at TJan 3  12:31 PM, Ruwan Welaratna wrote:

Thanks! I've actually got some more thoughts on your video idea, I'll email them over later on. And thanks for the team growth ideas that was really helpful. Great chatting with you, let's keep in touch.

<<phoned in>>

On Jan 3, 2013 12:29 PM, "Dan" <oblinger@gmail.com> wrote:
Hey my phone ran dry.

Thanks for the quick thinking on shared video.
If I ever move forward with that idea, I may hit you up for more reactions -- those were useful.

And feel free to ping me again as you strategy gels on how you will grow your business.  
A quick call is always easy to do.

Cheers, 
Dan







On Jan 3, 2013, at TJan 3  11:02 AM, Ruwan Welaratna wrote:

I hear you on the time zones - I have one team member in New Zealand of all places, which is 13 hours from Minsk. Great for emergency support, horrible for meetings :)

Yeah, let's chat. I'm at +1.415.254.1552 - are you available in the next couple of hours? My afternoon is not so free. Or if you're busy maybe sometime over the weekend?


On Thu, Jan 3, 2013 at 10:57 AM, Dan <oblinger@gmail.com> wrote:
Ruwan,

Great to hear from you.  We did move forward w. iTransition.  One member of our team is there --- I spend 2 hours last nite working with him.
It has spread me across all the time zones of the planet as I also am working w. a fellow in columbia this morning.

I have alot to say both postive and negative about iTransition, as well as about insourcing vs. outsourcing, and iTransition vs. others.

happy to chat....  today or anytime.

cheers, 
Dan






On Jan 3, 2013, at TJan 3  10:53 AM, Ruwan Welaratna wrote:

Hi Dan,

Happy New Year! I hope you're doing well. We are assessing our past year and planning for the future, and I was wondering if you ever moved forward with iTransition, and what your experience was. Or if you didn't use them, if you found another offshore team you liked. We had one dev on iOS who turned out to be not very good, and while overall we're still happy with them as a whole, we are sort of looking around at our options. Any feedback you have, positive or negative, would be greatly appreciated, and of course strictly between you and me.


On Fri, May 18, 2012 at 6:13 PM, Ruwan Welaratna <ruwan@evomonitors.com> wrote:
Hi Dan,

Anytime this evening at 415.254.1552 would work. Sorry I haven't been around earlier. Tomorrow daytime will be good too.

On May 18, 2012 3:27 PM, "Daniel Oblinger" <oblinger@gmail.com> wrote:



Ruwan,

I am the CTO at PayByGroup and we are considering iTransition for offshore dev work.
As I understand it, you have some experience in this area.  I would love to get your thoughts in a quick call.

Can I give you a call?   If so, what is the best number?

Thanks,
Dan





--
Ruwan Welaratna
WeMo Baby - Belkin-Evoz baby monitor with great parenting services!
Web - MyEvoz || Pinterest - Never Enough Time || FB: Never Enough Time




--
Ruwan Welaratna
WeMo Baby - Belkin-Evoz baby monitor with great parenting services!
Web - MyEvoz || Pinterest - Never Enough Time || FB: Never Enough Time






--
Ruwan Welaratna
WeMo Baby - Belkin-Evoz baby monitor with great parenting services!
Web - MyEvoz || Pinterest - Never Enough Time || FB: Never Enough Time

_

_