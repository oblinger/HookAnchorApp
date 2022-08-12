
## QUESTIONS


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





## REF-DESIGN

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




### Target and Design

-- fail: flow, cog, coverage

-- attentional measures of others 




### Target Use Cases and Value Story

KEY VALUES
-- SAMETABLE  --  provides 'working at same table' experience for tight team
-- ONSITE     --  customer get 'on site' experience w. windows & crash cart
-- TELEPERSON --  single expert can serve in remote location

#### Tight Remote team with single client.
--  Tight team of 4-20 knowledge workers
--  They are collaborating full time on a project for a single client
--  They are all remote from each other, and from the single client location

#### Single Remote Worker in a tight team
--   One specialist worker working remotely within a tight team
--   They have their own office, cubical, desk within the team space
     and a parallel remote setup at their home.

..   Single remote 





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




### Key Questions / Markets

--  Will customers accept virtually embedded remote workers?


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



## COBBLED DESIGN -- Pragmatic Tools for current use

-- One click
---  Share Voice, Face, Screen

---  Connect to existing meeting.



- Use gVoice to call TO phone
- Use webex to call TO phone

### Basic appraoch
- Always in persistent chat.  Use this for status.

- Make phone call

### WebEx


### LogMeIn
- Easy cold-machine screen sharing

### Existing Tools for Collab

http://blog.braidapp.com/working-remotely-and-the-tools-that-make-it-possible



## External docs

Remote work article  https://source.opennews.org/en-US/learning/making-remote-work-work/


## Existing Buisnesses
  - Conf Advisors   http://www.conferencingadvisors.com/

  -  Sqwiggle (https://www.sqwiggle.com/). Always-on videoconferencing
    for a fully distributed team
  -  Voxer (http://voxer.com/) is a great example of this that Julian
    and I found which we think is awesome. Yes, you could send
    voicemails

- 
http://screenhero.com



## Articles on collaboration

- The Fog of Work
http://blog.screenhero.com/post/60401526813/connected-but-isolated-the-real-problem-with-working?




# LOG
## Discussion with Ruwan about solution tech
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
