
## Adam
-- lowlevel.  RTOS, zero alloc system
-- 10K front end on M3 cortex
-- 80K lines of frontend (windows CE app)
-- He Says:  New Features.  Strong debuggers multi-threading.  hatesDriverLayer.  LittleUnitTestExperience
-- PhD -- Stocastic modeling of 3D surfaces.  
   does not get to do algo stuff now.  regrets it.  in the middle-lower for algo-stuff.
-- always the best CODER in the room.   and I don't make bugs  (making fun... oh Adam I found a bug of yours!)
-- lots of code of on MCU.  
   but mostly higher level on MCU.  but did use DMA, HW interrupts, but wants to stay higher


SKILL
-- PRIME -- The project you personally contributed most lines of C++ code
-- STRENGTHS -- 
-- AREAS -- Algo, Robotics, Visual, AI

-- fast search module

14K Zwt/mo  a little above by not alot above current salary
      there has been a large jump in polish salaries so I want to go up.
16K he will accept at this rate.



## MANY PEOPLE

## Laslow

-- Designed the system
-- He was the Technical Manager.  

CONFLICT:  
 part of project was about creating signage for stores.
 -- gave example of an external company w. conflict of interest.


 differen conflict example
 -- win 2000 system to port to XP.
 -- could not get employer to agree 

---
WebRTC working under windows.  <-- example of downloading a library.

--- 
fast in the long term.
slow in the short term.
I like to think hard about a problem, then do it right.
   <-- 

====
-- Math background from his past.
-- 


## Severin

did not describe other approaches
test was completely missing



Overall -- Lukewarm
check: can he 
check: what better distance funtion might you have used instead.  
       (note he did not need sqrt)
professional coding: innappropriate use of recursion


Some first impressions:

I would be interested in why he chose to flood fill recursively rather then with a simple loop. 
He had the experience to check the stack space for this approach, but he could have just kept a point set or a mask image using a loop and thus not risk the stack.
His color distance function looks odd. He repeatedly avoids using abs() in favor of if:else statements. He could also save substantial cpu time by not square rooting the color distance and instead comparing the distance squared to a threshold squared.
He states in his read me that he would improve this function by not using a simple vector distance of [R,G,B]. I think we should ask him what he would change it to. I would expect him to discuss converting RGB -> YUV (or some similar format) so as to compare only the chroma characteristic and factor out luminescence/brightness, but regardless of what he has in mind, how he talks about that upgrade will tell us a lot about his thought processes.
The csv file choice is curious. I don't have a problem with it but it would give some insight into his thought processes to know why he chose it.

Lines 499 through 502 of 'ImageProcessor.cpp' accomplish nothing. That code is likely vestigial.
I like his smoothing approach as it functions to remove noise for both the perimeter the flood region. However, that kind of windowed approach annihilates regions of sizes below the half window size. If you want to smooth all possible flooded regions, that could be a nasty edge case.
I would follow up with some questions about his approach here: Ask why he chose this method, what other methods he considered and what pros/cons he weighed. I would be interested to know what if any questions he asked prior to starting this smoother code regarding the prompt.
Your prompt stated " remain generally close to the original boundary for this operation. One could argue this type of smoothing fails that requirement.
In your prompt, you stated "potentially be expressed via lower degree parametric representation". I think we both read this to mean that you want a vectorization of the perimeter into arcs/lines and implicitly the perimeter segments to be an ordered list for each perimeter. In retrospec, we should have been more clear on this point. I would hold that against him much, unless he failed to ask any follow-up questions on this part of the prompt.
His '-example' argument does not exhibit much in the way of test code. I was really hoping he would show more self-verification than that. Perhaps he does not have much prior developer experience (he is a recent grad right?)
His csv file does not order the perimeter points. That is unfortunate since I wanted to judge his handling of edge cases surrounding that.

## Walmart

INTRO
-- Wallmart is a data-driven company 
-- it spends an enormous amounts to drive its critical decisions most effectively.
-- But nearly all of that analysis is:
    MEASURING THE EFFECTS  not  THE ROOT CAUSES OF THOSE EFFECTS
    MEASURING THE PAST not ANTICIPATING THE FUTURE

====

-- All to often these gaps have devistating consequences for the heath and effectivenss of our corporation

     time X -- cause happen, time Y -- effect first seen, tie Z -- action taken, Time ZZ -- crisis over
   * In Sept of 2006 walmart appliances suffered a 15% slow down in deliveries
     ==> Corrective action was finally taken in Oct 
     --> And the crisis was over by the end of November
     --> but not before an estimated XXXX million dollar loss for the company.
     --> this affected XXXX million dollars worth of product for the company.

   * In Feb of 2102 ....

#1 point one --- LAGGING INDICATOR
- And our pretty graphs and charts are COMPLTELY failing us these and hundreds of other cases.
- They fail us, because they only measure SYMPTOMS, so the are a LAGGING measure of the root problem.
  Managememt keeps its eyes firmly on these charts, but weeks or months of damange has occurred while no change of measure is aparent.
Millinos of dollars spill under the bridge before senior management even knows to start looking for the issue at hand.

#2 indication of waht but not why -- NOT CAUSAL
Second these pretty graphs fail us because they don't provide any clue as to WHY these symptoms are occuring.
We simply see a slowdown, but no measurement of WHY the slowdown is occuring.  Precious weeks are lost as management struggles
to understand from direct reporting what the root causes are, or worse yet.  
it is NEVER understood, and it is NEVER resolved, the company simply leaves those profits on the table, unclaimed.

To see how pervasive this approach is at wallmart, I encourage you to consider for each of the next dozen 
report you are shown the following question:  ``Is this report approaching the root causes in what it measures, 
or is it simply measuring the the tail end of a very long and complex pipe of interacting effects?''
Each time it is the latter, know that we are really operating blindly with regards to those root causes --- we cant affort this.

=====
So what can we do about this issue:
==>  In the longer term we....  (instrument and measure more, we impute models of earlier vars, ....)
==>  In the short term we can take practical steps in this direction by

    QL brilliant slides go here.


~~~~~~~~~
The second big gap in current WM analsis is the pervasive measuring of the past, even when all of our profits (or losses) lie in the future.

   Time X -- siganiture of badness to come happens      Time Y -- badness comes   Time Z -- react Time ZZ -- all done
   Time X' -- much earlier siganiture of badness       Time Y' -- badness happen .....
   * In June of 2013 .....


     really bad things happened.  which we did not predict.  
     but there WAS historical precident that could have been used to help us predict.


AGAIN I ask you to consider the next dozen reports you see, and again to consider:
  ``do these reports document the past, or do they anticipate the future''


So what can we do about this issue?
==>  In the longer term we....   (use ML everywhere... blah blah)
==>  In the short term we

    QL brillant slides go here.



=======
CONCLUSION
-- walmart must continue to be a data-driven company,
-- but to get the best value from the money it spends on obtainting that data it must:
      SPEND LESS TIME DOCUMENTING EFFECTS, AND MORE DIVINGING THE CASUSE, and

        [picture of cartoon guy gripping steering wheel with worry cloud above head looking as the speedometer in his car 
          vs.   picture of cartoon guy looking under the hood in his car]

      SPEND LESS TIME LOOKING AT THE PAST, AND MORE LOOKING INTO THE FUTURE

        [picture of guy looking into dumpster for printouts   vs guy looking in telescope into the distance]



  [picture of dude 




## Guro

- 7pm --- 9am
- Largest commercial proj; lg proj; rails?

-- Most work as a front end dev.
-- Rails -- None.
-- Python -- 5-Django applications
   -- Mozilla browser -- 1,500K lines
   -- Convert Latex (Jango plugin)
   -- lots of open source contrributions

   paid projects = None
   self project = http://goldpoisk.ru/   (nine person team)
     https://element.yandex.ru/
   not a very polished interface -- tried search -- no popup when no data
   -- so fucking hard to understand

-- Javascript -- Worked for Yandex -- most dev work in 
   Yandex -- bi-Syncronization for mozilla browsers; for multiple browser
    for Yandex which is extension for Chromium br
    -- In firefox you enable Yandex synchronization feature -- then it provides synchronization of bookmarks, etc. across many browswers.
    -- he was primary author for synchronization layer.
       team lead also worked on it a little to.
       some working w. 3 others in code base.
       -- 15-20 developers at Yandex interacted on this feature.
   

## Igor

Strongest languages:  Java & C++  (GWT)
- linux admin stuff


- Quit -- getty noble bank -- Getin Noble Bank 
  - did not see a future w. the company.  
    bad quality code 
  - debt collecting software
  - project ended -- time to quit
  - Java -> GWT


   ''there were changes I wanted to make to the stack''
   -- cencha widgets --- application got slow

   -- design pattern
      -- visitor pattern was helpful for writing filter
         build a query dynamically, rather than using 

_______
- Siemens & Nokia
C++ embedded base station

-- internship -- makefile

-- quick;y became expert in linusx build / makefile stuff

-- his team:  radio links -- control towers you connect with
   -- part of the code was already written -- build in finland
   -- part built in poland was much higher quality 
   -- managed to keep unit test coverage above 90%

   -- re-writing SDL code (kinda like UML) into C++
   -- hardware components.  communicting by c-struct
      device driver for this base-station radio.

favorite part of his job:
-- manage to connect gnu profiler gprof
   -- had to figure out this magic absraction layer (built in finland) 



========
His CV does not reflect his strength 
-- maybe work on RUST project -- to gain recognition

leaning towards doing my own thing;  consultant

  $2-$3k/mo at 
   aobut 1/2 

do something very well  ---  C++ / low-level





## Tarik


Pick a time.  will get you setup on gh; 


- Largest commercial proj; lg proj; rails?



-- Baby products.  Small amounts of work 

-- finishing Phd.
   wants to turn this into a commercial product.

-- Money is not the issue.  love the research issue
   goal is to resolve real problems.   then good things will follow.





we?
consulting?  
- don't have stable project.   kinda want it.  but must be innovative.

tarik.benmerar@acigna.com
- tarik.benmerar@acigna.com






## ==THILAND-HIRING==
## Vitaly Dimitri
was confused about speed of easiest problem.   N**2
don't like front end.  loves backend.  devops kinda guy.
fast code -- 
currently working as CTO -- 2   & 2 iOS devs
-- perl / console / ruby 

redis was used w. Rabit MQ for applicatant DB -- 
  candidates upload 

SUMMARY:
  - was not so impressed.
  - he did not understand the runtime of his own code.
  - seemed like a small team hacker -- not as much a software engieer.
    his tasks never really required scaling, good testing, etc.
    he described his skills a 'linux console'
       this *is* a good skill -- but I think it also shows how he has been operating:
       slam this data from here to there, or this form to that form.  
       less of a architect clean system kind of work.
  - I did not have the time, but it seemed he was not exactly clear on why his worker threads
    were designed as they were on the project where he was CTO!! 
    -- not postive about this, but was not getting warm fuzzies

## Zlatko
-- programming team in highschool
-- 7 years remote...  
-- cannot talk problems over w. team.


Build User Interface -- he was main dev -- 6guys
  -- connecting to korea, japan, etc.  
     automatic trading stragegy -- interface 
  -- performance issues in displaying in java swing market data.


=> he found the hardest problem's solution on google.

SUMMARY:
  - he is less familiar w. rails, but can probably do ok anyway.
  - is ok dev. 

## Senad Uka
he really understood test very well. only did easiest problems.

now -- Social explorer -- sensor data onto maps.

biggest -- software for car checking stations
  -- mostly backend guy.  ruby for backend.  he learned rails from 1.0 onwards
  -- 20 people. he was first.

-- for year worked exclusively as an architect.
   really likes this.
-- reputation from competitions & university -- also won Microsoft compitition.
-- was drawn to AF because of the


______
TRICK == closed door at home 
- enjoys the most tricky coding problems more

NICK ASKED: highest score on codility (too high? could he have cheated?)
     - no way, he was in the weeds with me, and it was all coming to him fast.
     - he solved those problems himself.

SUMMARY:
  - My favorite by far.
  - he built&led production system from scratch. 20 devs worked it later.
    he was upgraded to 'architect' in that job.
  - lots of work as remote team member -- seams seasoned.
  - really understood the programming tasks & the details of the tasks very well.
  - he may get bored w. our tasks -- he like complex tricky coding the most.

## Darko
-- he solved two easy problems, and co-linear triplets -- but that one in slow/easy way.

-- programming competitions in highschool -- project oiler

-- biggest project #1 -- in University -- neural networks
   one year hacking -- then read video stream -- 
   extraction of text from images using neural networks
   -- perspective transformtion 
   -- did some hand writing recognition using NNets

-- biggest project #2 -- 
   fashion product search
   terrabyte of images.  used AWS.  open CV.  removing image background
   -- one friend was coding CSS for him.

-- Working 6months for FinTech -- fulltime job.
   -- he decided the job did not feel 'right'
   -- did not like a rocket internet company -- 
   -- wants to work w. US -- wants more creative jobs
   -- on a personal basis he liked the job, but was bored by the job.
   -- was more about politics, but not building a product.


he likes our AI stuff that was why he responded to our ad.

SUMMARY:
  - he is a hacker that likes the stuff... his suduko project was just
    one big string and glue hackfest for the hell of it.
  - seemed like a bright guy, likes AI, but
  - he has much less experience in team development.  (and I think no-remote team dev work)










-----




## ==JOTTER=HIRING==
## Kalina



## Ahern

- UX lead.  PM lead.



- Poker platform:  spawn games, do tournaments, process payments
  - Came w. poker platform -- white label


- built his own poker buisness
  - Poker Host -- brand
  - Dobrosoft -- gaming platform -- 2years.  
    - get it going.  
      pump as much into teh marketing engine as possible
      - forums, content writers, good SEO rankings.
        drive players to poker host to register.
        expat network -- pay per article.  $10
        ensure keywords aligned, etc.  google ad work campaigns, etc.
    - built website:  
    - PhP


- Wager works -- lame poker product.
  - managing the buisness.  enough poker tables, etc.
  - Optomize funnel.  getting players in


strong:  practical buisness experience w. real buisnesses.
Interest people in poker


- Wagerware -- platform where players registers and depoit money.
  - A/B testing -- multiple flows -- one-page, two-page, three-page wizard
  - fidelity account form level of complexity

  - segmenting DB on 
    registered during timeframe 
    segmentation:  done by feedback partners, 

  - backend platform -- others owned the brand.

  - laid off in 2009 (w. 35% of company) -- moved into strategy role -- emerging 

- Offer wall -- way to drive virtual currency into online gaming
  - Signup w. netflix
  - in six months, moved them into regular payments (
  - facebook credits killed them

- Cyber Arts -- 
  - Robust poker platform
  - no hooks for partners to get at data
  - created API for partners to access poker platform -- he white labelled it.
    - defined the syntax of the JSON -- 
    - defined absent customer need
    - accounting platform, complete depoist, get id, bonusing, verification, 


- B-to-C -- Bwin party gaming -- 
  - merge was a clusterfuck
  - running the platform merger -- wallet, registration, affiliate team, bonus team, 
  - UX portion:  
    - web trends -- watch per session.
    - terra data w. cognos -- BI analysts -- supported by did not report in.  (two assigned to them)
  - Managed 12 product managers -- magager of product managers

- Ultimate gaming (used to be cyber arts)
  - run product.   regulatory environemnt.  
    execute against feature backlog.   (5 PMs under him, they built the 
  - launched in nevada in 
  - key idea:  system that automatically works into regultory environment (set of options)
  - again white labelling it.

============
- managing exectations, building process, framework to sale
  managing portfolio of resources use againt buisesses goals


best at: big picture, breaking it down, making sure the trains run on time
         Good a problem solving.  in meetings w. the devs to get goals done.

          - getting consensus amoung devs.






## UX Guy

- Saba

- Diamond  --- first graphics stuff

- Atheer --- more abstract UX


- Tablet command  ---  ipad app for firefighters.
  most notable UX

- Jack --- my vision one --- tony robins

- Vidaro --- conceptual display of info


v. opinionated

trouble pushing him to complex compoennts designed

clean designs



- is user story accurate?  (ask agent)
- explain it in words first  (avoid minutia)
- ask what there experiences are.    at end of day give product and ask how valuable

- design w. small team.  pick visual langauge.  borrow from the best.





## Tom Mag  Dev

is he backend or ios for us?

- iOS

xcd

## Matt

Loitery
ListPop
 i.pl?
Flint -- short term contract.
 -- ccard processor -- fin backend
 -- javascrpt, mysql/posgres,    (most work in Java)
    mostly
 
 -- interconnectivity layer.
  

worked with
- IMVirtual
- Flint

==> very bright.  not good fit a flint:  officially told him, you are expensive, 
    mismatch -- iOS guy did not like me.  unimpressed when talked?   maybe tought did not know his stuff.
    first time working in open office in long time.   (not a huge fan of open office zoo feel.)

    middle towards the quick and dirty side. 
    good at the Zen flow of 
    split up work into small chunks.   thinking --- 
    rather working 4-5 problems at a time.

    pointed button 


iOS -- 2010
Mac paid since '94

His love is PV   NOT iOS

- Script 

- 


## ==PBG=HIRING==
## Camilo Guy

## Jacek @ Net Guru

## [to interview]  Gavin

Dan,

Here's a recap on another candidates with questions on how to proceed.

Gavin Schulz - not an immediate candidate, graduates in May
- Github: https://github.com/gsmaverick
- LinkedIn: http://www.linkedin.com/in/gavinschulz
- Really strong JS
- Strong front-end
- Not a lot of back-end, has done little bit of Rails and Python
- He would be someone we could do little front-end contract work with over the holidays (12/19-1/7) to gauge things and then would need enough of a team in place by June to support his lack of back-end skills.
- He's looking to make a decision for work post-graduation by sometime in February
Do you want to interview him to better gauge his front-end skills and ability to match up with where our team might be in June and then decide whether to have him do any contract work over the holidays?


## [prob-no] Chris Bolton
- https://github.com/intermaggio/Enrollex/tree/master/app/controllers

wants:  better team, more backend, 
???     production code that matter, 

## Princeton guy

## Dev Ops Activites
- Vagrant setup for each dev to use for 
- Continuous integration testing
- load balancers

- DEPLOY SCRIPTS:  Chef scripts for deployment
- PATCHING:        Keeping server instances up to date
- BACKUPS:
- MONITORING
- TOOLING:         NewRelic
- LOAD TESTING


## Wiktor
- Thru Nick.  He knows this rails shop. 

- CI(circle ci) direct to staging.  Cdeploy()
monit--

passenger, user directory on each, cap deploy (delayed job also in cap script)
every project would have its own 

mweez.com


## JT giri 
- Recommented by maranick.  $100/hr
- Hard core chef guys.  Based in SanJose (one in china)
- IDEA 40 hrs:  setup up vagrant,  New Relic management,  Backup scripts


Giri -- thr  5102894440
skype jtgiri




http://www.linkedin.com/pub/jatinder-giri/33/b0b/46b

## Chris Rogers

## Santiago Achury
PROJECTS    see his git hub -- Achury
- Two companies: in maimi.  
- UNIV dept -- Reaserch company 
  web pages

- ARTICA -- investigation of TV and new tech
  JS & webpages.  
- Interactive TV.

- RAILS PROJ
  - w. Andy.   merge to 
  - w. Andy.   web chat

- CSS/JQUERY/  less AJAX

- 1month proj
  - little device must send req to server.  was too slow.
  - js caching


AVAILABILITY -- 
- lots of free time
- next week 
-  classes T(12-6), R(12-2), F(5-9), S(morning)  






# A-Player Refernce Email

Dear (past colleague),

I am considering hiring (candidate) for the role of (job function). If you’re like me, the last thing you have time for is a reference call. Therefore, unless you found (candidate’s) work to be EXCEPTIONAL, please just disregard this email.

However, if you found (candidate) to be an exceptional employee, in the top 10% of the people you’ve worked with, I would certainly appreciate hearing from you.

Again, if you found (candidate’s) work to be less than exceptional, go ahead and disregard this message and have a great day.

By the way, as a smart professional, you should subscribe to this wonderful blogger named Nir at NirAndFar.com. He’s swell!

Sincerely,

Dan 


# Misc People
erikas@goingon.com  met at philz surfer rails dude.


# QUESTIONS

