 [[@Sports Visio]],  [[@Jason Syversen]], 
 - [[SV90]],  

# TODO
- [ ] Get Coach
- [ ] Build Roadmap
- [ ] Build Roadmap
- [ ] Yolo-x
- [ ] Negotiate & Sign


# TOPICS
## = People
# LOG

### t2022-10-27 Response:  So are you a VP of ENG

Jason,

In answer to your question about my management of production level code I captured a bunch of details in an attached document that you can skim as you like.  Here is a thumbnail sketch:

- PayByGroup had higher-stake, higher-tempo code deploys as compared with Sports Visio, since it was a live payments platform.   We could never afford even one critical execution bug to get past us in our bi-weekly push to production because of the large cash amounts that were at play.  (Its true PBG had a small exit, but that was market positioning and lack of differentiation, not its underlying tech—that had to be rock solid in order to even keep executing payments for over 8 years.)

- Aeolus had QA requirements that are more complex than Sports Visio has, simply because we were fusing outputs of 130 engineers into a soup-to-nuts fully autonomous Robot that is being deploy in front of live audiences/cameras at CES, and into physical spaces that are shared with Eldercare patients in Japan.  Ensuring release quality on our 3-week sprints and 2-month release cycles, given the integration complexities with both non-trivial hardware and non-trivial software is much harsher than what I will likely face at SV.  I did a stream-of-consciousness dump of many innovations that I created to manage release quality.  Many of these innovations have relevance for SV since SV also builds from very related computer vision algorithms.  (And by the way,  Aeolus is still very much alive with almost 100 employees and having doubled their funding to $30M since I left.  We will see if one day they achieve product market fit, but either way, their output quality and engineering team is solid.)

I notice when we first started talking, you were saying, "yeah I need to hire some more computer vision guys, and I know you do algorithm hacking so you would be able to help out with the algorithms hacking yourself."  Then weeks later you were saying yeah, "we need stronger CV guys, and you can help me hiring really strong guys."  And now you realize your most pressing problem is the nitty-gritty management of getting production quality code out of a PhD-holding-CV team.   No doubt a month from now there will be a DIFFERENT technical slice that will be most urgently needed.  And this is EXACTLY how it goes with hard tech startups!  The truth is, you need a technical co-founder that is strong across ALL of these areas, plus another dozen or two more that we have not even been in focus yet.  What I like about my background in the context of SV is that I have solid experience that spans from very resaearchy contexts all the way up to code being pushed for a payment processing platform on a bi-weekly cadence.  Not many others cover that kind of breadth in the scrappy fast-moving startup context.

As far as talking with someone about my execution in heat of battle, I think Theo Enns might be a good choice.   He had never even been a team lead when I first hired him then next went to Amazon as an Engineering manager after Aeolus.  He had a front row seat to my org/team building and execution.  Let me see catch up with Theo this weekend and see if he can chat with you.
free
Cheers,

Dan

  

Production quality is about how one manages execution details.  To really assess that one needs to get granular about those details.  Here a did a verbose dump of many of those details from memory.  Feel free to skim, taken as a whole I think it gives a sense of the breadth and depth of my experience.

Here I break my “code-centric” experience up into three levels: 

- **LEVEL-0** (hands on coding). 

- **LEVEL-1** (intimate single-level, direct team management.  e. g. scrum master, system architect, source-code review etc.).  And

- **LEVEL-2** (multi-level team construction and leadership;  construction of multi-team process flows, creation of QA testing protocols; scrum of scrum execution, etc.)

I have done a ton of personal coding, right up to the present day (with the creation of a plugin for Obsidian.)  But for this summary I will only focus on professional contexts:

**ML ALG DEVELOPMENT**:  Machine Learning algorithm development (at IBM Research)

- **LEVEL-0**:  Developed a novel programming by demonstration algorithms that automatically learned GUI action procedures by observation (and induced the loops in branches implied in the user’s actions).  This was ultimately transitioned into IBM's Rational Tool Suite.  We got bunch of patents and papers out of this, including a runner up best paper award from the world leading conference for programming by demonstration research.  (I invented the core algorithm and coded it along with 4 other researchers committing code to a common repository, but this was very resaearchy code).

**PRODUCTION CODE FOR PAYMENTS PLATFORM**:  (at PayByGroup)

- It's true the PayByGroup was ultimately only a small acquisition.  Still PBG had over 100K users all doing online FINANCIAL payments.  Our tech was rock solid—it had to be, it was the lack of differentiation, and lack of big player market connections, that resulted in a small exit.

- **LEVEL-0 coding**:  In our first months, the CEO fired the other developer, and we decided to scrap our entire code base.  With only $50K left in the bank I built our first end to end payments processing front end and back end from scratch over three months.  We needed to have a polished system executing live transactions in order to do our seed round raise – I built and shipped the whole system

- **LEVEL-1 team lead**:  The demands for production-level-code were far more stringent at PBG than what we likely will have at Sports Visio.  PBG was a multi-stage payments execution pipeline--the group payments that is facilitates happen in multiple stages since not everyone would be online at once.  This means with thousands of active users, one needs very reliable code, since we could never afford to execute incorrect transactions.  Even worse, even as we upgraded the code, and altered the underlying data tables, we needed to ensure that partially executed payment transactions would never be corrupted even when different parts of the transaction are actually executed by different VERSIONS of our system!  I architected and directly oversaw our execution of this in production, for over a year and a half with live transactions the whole time. 

- _SCRUM MASTER_ - I did all code reviews initially, and was scrum master for the first year, until I hired my replacement.

- _MANUAL TESTING PROCESSES_ - I developed detailed QA procedures that were expanded over time to be more that 30 pages of detailed instructions that were manually executed by our team on each code release to verify correct functioning of the end-to-end app, this was based on an ongoing code coverage analysis, to ensure we were at least executing each part of the branching code once during the QA process.  (over time parts of these flows also were automated into selenium tests as well, but nothing beats kicking the tires on the actual app with each code push too)

- _AUTOMATED TESTING PROCESSES_ - I oversaw (and helped author) the automated testing that went with this, including the automation scripting that ensured tests where systematically run across all code branches during development.

- _CHOREOGRAPHED “PUSH TO PRODUCTION”_ - By the time I left, I had developed a very choreographed "push to production" rhythm that had specific deadlines for each player's part of the process which spanned the globe.  e.g. by xxx GMT all pull requests should have a 'green light' from our handbrake testing suite, then at yyy GMT all devs involved with a PR needed to be online for hot fixing in the case that we had integration conflicts.  and a different time slot for rapid fix or code reversion for bugs uncovered in the hours devoted to executing the QA script.

- _ERROR RECOVERY LOGGING_ - I developed a robust logging system, so in rare cases where a bug would wedge some transaction in a 1/2 completed state, we had all info needed to reconstruct and manually (or by script) fix the broken transactions, w/o anyone loosing or gaining funds.

**_This was not PhD level code, but it was large, complex and it needed to be absolutely rock solid and high tempo in execution._**

**MARTIAN ROBOTICS**:  (Acquired by Aeolus)

**LEVEL-0**:  Personally coded inverse kinematics and robot motion planning on an embedded microprocessor in a mix of C-code and microprocessor libraries. 

**LEVEL-1**:   Ran a daily scrum, architecture of an autonomous robotic system with complex perception, planning and execution.  (Very researchy, and algo-development focused)

**AEOLUS ROBOTICS**:  Autonomous, dexterous, eldercare robots [CRUNCHBASE](https://www.crunchbase.com/organization/aeolus-robotics) & [WEB](https://aeolusbot.com/).

- **LEVEL-2 training and system & team architecting**:  I was employee #1 for Aeolus's software team and lead that team directly when it was below 10.  After that I organized, and continually re-organized that team into 5-8 "swim lanes".  These were called swim lanes because they were optimized to minimize their 3-week sprints from dependencies on activity in adjacent lanes in the same sprint.  Only ONE of my swim lane leaders had prior experience managing teams.  So I was the epicenter of planning & execution for this group of 60+ engineers.  We were moving very fast, and each team had numerous gaps, so it was a constant triage to form and reform these lanes to cover these gaps.

- **LEVEL-2 management**:  I kicked off each new swim lane by attending their individual daily scrums, until I saw they were on good footing.  Then I maintained a weekly scrum of scrum meeting with all swim lane leaders in order to surface conflicts or delays to our three week sprints.  Every two months we had a new set of end-to-end robotic capabilities that we were rolling out.  So my lead PM maintained the mother of all dependency charts to ensure nothing was dropped.  He was not technical, so I worked very closely with him and with the lane leaders to ensure that we were covering all key capabilities in our planning.

- **LEVEL-2 QA**:  The QA that had worked well enough at Martian robotics was soon failing at Aeolus.  Our sprint plans included testing, and a minimum of formal test coverage too.  But we were constantly bit, by items marked as completed and tested in JIRA that were just not usable in the next sprint by another team.  I developed many of innovations that dramatically reduced these problems while not costing us too much in velocity.  From memory here are some:

-       _MANUAL TESTING LIBRARY_:  We had pretty cheap labor available in Taiwan, so we simply hired a couple of test engineers and developed a library of written robotic testing procedures.  I developed a format where one could specify a full testing protocol right in JIRA tickets by referencing these testing documents.  These ticket-by-ticket tests  allowed us to surfaced bugs much faster and fix them before integration testing even began.

-       _REMOTE DEV AND TEST CENTER_: We built out a 24/7 dev and test center in Taiwan.  My counter part peer in Taiwan hired the techs, but I designed simple shared resource allocation spreadsheets, and arbitrated the contention within the SW team for those resources.  These techs executed QA, and allowed remote PhDs to run experiments on robots that were 10,000 miles away by ssh-ing into the machines, with dedicated surveillance video and voice comms to the techs.

- **LEVEL-2 documentation and processes**:  Nearly all processes and documentation used to drive this workforce were initially developed by me, then ultimately owned and refined by the team.  As issues arose, I was nearly always part of the conversation about how to redesign required documentation and processes.  For a 100% remote hardware + software company with over 130 engineers on very aggressive timelines, these written processes and flows needed to be world class.  I brought much of the learnings we had done at Analytics Fire over as templates for these.  **_We had documented templates, rules, and timelines for EVERYTHING_**.  What is a valid JIRA ticket? A valid backlog item? A valid story-item? A valid pull request? A valid hot fix? A valid git-branch? Avalid dataset collection request?  The valid problem escalation procedure and maximum allowed timelines?  I was either the author of these protocols or worked very closely with the author in all cases.  Managing this product, on those time lines, at that scale was complex, and required very well considered formal processes.

            - CONSTRAINT PLANNING:  In the early days (<30 SW engineers) we used sticky notes on a giant chunk of our wall space to manage a giant (6 foot x 30 feet) GANT style chart that was continuously edited by myself, my right hand man Theo, and my head PM.  Things were just moving too fast to capture all dependencies in a more formal tool (though the individual JIRA tickets were religiously kept as well as a detailed back log for each swim).  Later this evolved into various dependency reporting outputs from JIRA once maintenance of the wall became too complex for a sticky notes.  (but honestly we never managed to fully replicate the level of total understanding that was possible from that massive sticky note wall)

            - UPSTREAM INTEGRATION TESTING:  To avoid the she-claimed-it-works-but-later-when-used-it-does-not problem.  I instituted a defined set of seven levels of integration testing.  Then in our sprint planning we would decide on a ticket-by-ticket level the testing that was required for each major ticket.  Extensive testing was heavy hammer, but an expensive one too, so one needed to be judicious about choosing high levels of integration testing between each successive ticket only when crucial.

            - _CELL PHONE VIDEO TESTING_:  A great innovation was to leave the testing undefined on smaller tickets, but then require the engineer to create a quick video recording of whatever "testing" they did for the smaller items.  We found that we did not even need to define what specific test they needed to do.  Just mandate that they turn on their cell phone video recorder and verbally explain what was happening on their console, with the robot, to "proved" the feature was "working".  Just having this requirement was a forcing function, and it removed a common source of miscommunication between the team.   "Oh, I thought you were going to integrate the XXX with the YYY, but you have only tested it on the ZZZ."  These videos dramatically reduced the miscommunication between swim lane leaders, since each swim lane leader watched all videos for their evolving dependencies, so they could see if a feature was not "done enough" for their needs in the upcoming sprint.  What was great about this form of testing is that it added almost ZERO overhead, since you just turned on your video recorder when you ran whatever test you were going to do anyway.

            - DATASET MAINTENANCE and TEST MAINTENANCE:  Early on we had failures with our vision algorithms where we achieve very high accuracies on our test data, but then very low accuracies in the wild.  Subtle discrepancies in context were having outsized effects on outcome.  In concert with the team we developed two innovations to address this:  one, we streamlined the process of making a single algo change on a git-branch and then directly test that one small changed directly on a robot in Taiwan using a predefined test protocol.  This allowed an engineer to immediately gut check their work w/o waiting for the next two month integration tests.  And we also streamlined the data capture during routine testing.  Often the collected datasets were evolving because the whole robot was evolving, so the way the head moved changed so the data collected via cameras was different enough that the algo results from prior recorded data, and live robot results would diverge.  By continuously feeding data from routine testing back into our stored datasets we could give CV engineers more upto data for their automated regression tests.

I feel like there is a lot more LEVEL-2 stuff that I did while at Aeolus... just too much to write.  I think much of it was pretty world class for a scrappy, 100%-remote, startup under tremendous time pressure.  Much of it, is relevant to the SV situation right now, and it will cover SV as it grows an engineering team to support many millions in revenue.  The complexity of code quality problems at Aeolus were high because of the nature of algorithms, of hardware, of team size, and development velocity.  It was high stakes because the code bugs could easily damage the robot, an elder person near the robot, or our reputation at CES.







### t2022-10-27 Chew on this - pitch to join

  
  
Jason:  
  
A few thoughts as you mull over our discussion:  
  
- I am now convinced regarding the A-round funding, and for the offer on the table, I am ready to be all-in.  
  
  
  
DAN’S NEAR TERM VALUE:  
  
- In the period before the A-round.  DanO would be very focused on fire-in-the-belly driving of execution at all levels in all places.  
  
- There will be other tactical choices, like the reduction in scope that we just did for the MVP product.  Having you plus me in the trenches together is a proven benefit.  
  
- At the A-round raise, I expect to be very helpful in providing articulate compelling technical pitch support in early stages, and also in the final stages when the investor kicks the tires on our tech using some PhD guys to grill us.  I will ensure that we shine.  
  
  
  
DAN’S LONGER TERM VALUE:  
  
- I think in many ways in which I will help build actual differentiation against other CV startups which will inevitably be in the spaces we want to conquer.  
  
- We have barely scratched the surface of all the different ways this tech can be used to deliver unique value in each market segment and sport, and you need a partner that can think and operate at that level with you.  
  
- In many areas, tweaking off the shelf tech is going to be the smart path.  But in a few magic areas we can build true differentiation, but this will require identifying exactly what is the targeted R&D that will carry the day, and then identify the academics somewhere in the world already near those problems, and hiring that talent.  And or build our own hyper targeted R&D org around those narrow problems.  I have successfully done both of those things more than once, and would love to do it for Sports Visio.  
  
  
FRAMING THIS TO YOUR TEAM, INVESTORS, and SELF:  
- Dan gets a chunk of Jason's equity, but he is then legally obligated to give more than 75% of it away to charity as SV grows in value, which is exactly what I (Jason) want to do with part of my equity anyway.  
  
  
  
SUMMING IT UP  
This is an actual high-tech startup play, and as such it makes NO sense trying to hit $100+M exit w/o having a strong tech co-founder/CTO.  
True, with a strong A-round behind you and good progress, you can find another solid CTO,  
But especially when considering my willingness to accept a cash cap, I think you wont find a better one, or a better match.  
  
  
Talk again after all your family stuff is taken care of.  Enjoy!  
  
—Dan


### m2022-10-25  Am I Joining, or WHAT?!

Talent Cross.

-  [[@Brian Yormak]]:  My take on the A-round issue.  Some investors will balk at the TAM.  (60x multiple and multi-B exits are generally expected.)
    Still others probably wont balk.

- TWO PATHS:  multiple sports down market, --or-- a path to win up market

- I agree with you, this narrower MVP is much lower risk and does not require replacing the CV team before the A-round.

- BUT NOTICE:  That strategy came from the combination of the two of us.  You could have been executing that strategy 6 months ago.
- You don't have a technical co-founder and it dramatically shows.
- And SV is not getting to $350M w/o winning multiple sports, multiple markets, and multiple geos, 
  To do that you need strong differentiation in market access and strong differentiation in tech.
-  Both seem plausible to build.  But you saying things like:  
	- I can be the CEO and CTO too is not the way to do that.
	- Saying we can build this tech differentiation using download and tweak strategy does not seem right to me.
	- Saying, in long term, we don't need a few world class CV people is also does not give the differentiation for multi-geos and sports


- I am now convinced this will get funded at the A -- I'm excited to join and take on multiple sports and multiple geos.
	- With a cash cap, my effective equity is under 3.5% equity stake once we hit $350M exit.  
	- This is a TINY number since I think I notably increase the chances that we win multiple sports and multiple markets, multiple geos.
- You could get a different CTO after the A-round but are they going to cap their upside into a non-profit with you?
- And if you run into speed bumps along the way, I think it will be hard to get a strong CTO.


- We have already demonstrated that you and I together can guide SV in a way that neither of us could do in isolation.
- That is worth more than 3.5%


BACKOFF: 
- CAP WORKS?  It seems like the $12M cap really drops my equity down to a very acceptable level.
- ISSUE?  Burn-rate?  Equity?

- CUT CASH BEFORE THE A - Longer term I would need to negotiate for a stronger salary, but before the A-round I could drop down to extend our runway




- You should not TRY being the CTO and the CEO all in one, you need a real partner - either me, or someone else.
	- SV velocity and strategy up to this point really demonstrates this
	- You really CAN get to an A-round with this MVP strategy and your current CV team.  But not a $350M company.
- To be clear:  given the cash cap idea...  and knowing what I know about our chances for an A-round, I am all in.
- But I am not sure you are.  Either way, we should go ahead and decide what we are doing.
- 


### m2022-10-18  JASON TAM discussion

- How are things going from your side
- Talk w/ [[@Brian Yormak]]    (Confidence in tech; confidence in market;  What do you think SV needs to show for a strong "A")
- Mandate:  The 'soft power' of being a late joining 'co-founder' can help, but we may need to think hard about the report-to structure.



### r2022-10-14 Minimums for VC fundability

Dan,

Enjoyed the brief discussion (Good to know we can have good BRIEF discussions too!)

Spent some time trying to research what people say about expected returns. Included my findings below:

1) They say 10x return at Series A: [https://kruzeconsulting.com/blog/what-vcs-return-expectations/](https://kruzeconsulting.com/blog/what-vcs-return-expectations/)

2) GREAT analysis in [this one](https://www.industryventures.com/insight/the-venture-capital-risk-and-return-matrix/). My favorite section is when they pull data across actual VCs (and not one guys opinion) and come up with this: "Using this data together with an expected loss rate equal to that of individual early-stage investments, we calculated the performance of a hypothetical $100 million fund with a 2% annual management fee (in the first 10 years) and 20% carried interest. We assumed the fund would be invested in 20 companies, where 65% returned 0.5x and the balance returned 10x. We concluded that our hypothetical fund would likely yield a gross expected return multiple of 3.8x and generate a net multiple of 3.1x, or an IRR of 20%." 20% is the average IRR early stage return profile for 30 years according to Cambridge Associates.   

3) Simple math (which is [mentioned here](https://www.bfp.vc/vc-performance-metrics-for-early-stage-funds-beyond-common-multiples/)) would say that if someone wants us to "return the fund" and the typical VC fund does 20x investments... that we need to shoot for a 20x return from a Series A investment (unlikely to see that later/growth stages.) If we raise at $20M for the A that's a $400M valuation in 7-8 years, and implies a $40-50M revenue run rate (conservatively). A $25M valuation would imply $500M. Again, both are in range. Now there's stuff like dilution in there potentially, but my hope is we don't need to raise again after a Series A (which again is why the math changes by investment thesis/fund type/stage/details... if you're Uber and have to raise $100M every few months you gotta have a plan to be a $100B company for any of it to make sense! But if you can be cash flow positive/growth after $3M... it's a different situation from a dilution perspective. You can then explore debt financing, or very friendly growth stage investments down the road for M&A or whatever.





QUICK RESPONSES:

(1) I don't know what they are smoking, but NO ONE says series A-investors are shooting for a 10x return as the TARGET for their investments.
    (They may understand that in practice a 10x return is a good return to get.  But they companies should be AIMING higher than this!)
    I am very mystified by these guys.  Maybe they are talking about some markets that are not high tech markets, so they have more predictable outcomes.
    .
    I think the math does not work for 10x.  No idea what these guys are talking about, but this just seems wrong.



(2)  Industry Ventures
- Mark Suster of Upfront Ventures expects a 1/3 - 1/3 - 1/3 split of returns with the last 1/3 returning most profits.
  (that is a bit bigger than I have seen others assume, but Upfront Ventures does a mix of series-A and series-B)
  
- EARLY STAGE FUNDS:  hypothetical VC firm numbers
	- Expecting ONE THRID of your series-A funds to return 10x ROI is VERY AGGRESSIVE, and not industry norm.



(3) SIMPLE MATH.  I like your simple math, but the only way that a fund is going to keep its series-A pro-rata rights in order to maintain it fraction of the company, one is going to need to allocate funds for follow on rounds.  I think you simple math was no accounting for that hold back:

Simple Math:

ASSUMPTIONS:
- GOAL:  Fund aims to return all investments with a single win, each investment should be aiming at such a win.
- INVESTMENT SPLIT:  40% of the fund is allocated for initial investments, and 60% for follow on investments.  (some say 1/3 - 2/3 split)
- SERIES A-OUTCOMES:  60% of series-A companies fail.  30% earn modest return.  10% earn 10x or more as a return.
- BAT SWINGS:  Fund wants 20 investments to maximize chances of a couple big wins.

$1B		 	Fund size
$400M 	Initial investment pool   ($1000M * 40%)      60% is reserved for follow investments.
$20M		Typical investment size    ($400M / 20)
50x			Required multiple to return the whole fund.  ($1000M / $20M)

$60M		Assuming we are forced to give up very a HIGH percentage of the company with a $60M series-A post valuation.

$3B 		Required target valuation in order for their $20M investment to yield a 50x return.  ($60M * 50x)
				(and they only get that valuation if they have the funds to maintain their pro-rata percentage thru ALL rounds!)




#### Reference:
- [TAM importance](https://techcrunch.com/2017/09/05/why-tam-doesnt-matter-to-me/) - 
- [size](https://www.goingvc.com/post/venture-capital-due-diligence-the-market-test) - 
    A good benchmark is a minimum of $500M-$1B in overall market size and a minimum of $100M in addressable market size.

- [TAM size](https://www.failory.com/blog/series-a-funding) - 
 -  1.  The Total Available Market (TAM) - It should be over $1B, which shows the market is large enough to sustain $100M in revenue.
 - [Big Vision](https://www.nfx.com/post/fundraising-checklist-13-proof-points-series-a)  - 
  - To raise a top series A, be able to show a path to $100M and then potentially $1BN in revenue.
- [Quora Stan Hanks](https://www.quora.com/What-type-of-exit-multiples-are-VCs-looking-for-when-they-make-an-investment) -
  If the investor is coming in at seed or Series A, they need to see a greater than 100x return to be happy, but they’re willing to wait 7–10 years.

### m2022-10-14 -

- Proof that custs will pay
	- Mid 100K upto $1M in Revenue
- Evidence that other sports


John see.  $10-$15 on a $70

step 1 - amateur sports for basketball
step 2 - another sport (e.g. soccer)
step 3 - B2C play - streaming of games from phones 
- sell ads.
- HUDL killer raised $250M on a $2B

step 1 - 50M-70M (sale w/o A)

- compared with a consumer business - SV - has value 
- lots of optionality.


~~
- The proxy is Hudl - They raised huge money and huge valuation.  Winner in sports tech in stats.


### m2022-10-14 Jason Response Discussion

Ask: Ben++, Kian++, Nick+?, Joseph++, Tessa--, Avidan--


Need:  Many segments, Many geos, Many sports


- Told John @hyperplane about $360M target?  
- What we will need to show to get an A?  Other sports?  Other segments?  Cust growth rates?
- Committed sales?  How many asked?  At what price point?


- Thread the needle.
- Intl grad





### m2022-10-03  Counter Offer Discussion  (where I proposed the $12M cap)


Jason,

As far as triangulating the reality of these online numbers I have included a bit of analysis in a separate google doc below.  Short answer: Levels.FYI has by far the most comprehensive data on the FAANGS and that data is pretty darn accurate.  It shows a breakout of the different forms of compensation by types of compensation too, so you can see there is no fluff in those numbers.  

I also have spoken with folks working at Google about roles and salaries I might expect as well in some cases generalities about what they make.  I am pretty uncomfortable asking them to divulge their salary info to a third person...  They might do it, but this is a pretty big favor to ask from a people that I want to have owing ME favors.  As a concrete reference point, I do know ex-DARPA folks that are at both Level 7 and Level 8.

Finally, I also know FAANG recruiters that can vouch for these numbers, and for the kinds of compensation I could expect as well.  I suspect they would be willing to have a short call where you could just ask them this yourself.  But I think the summary that we said before is quite accurate:  getting a $700K compensation (or significantly more) is definitely doable for me given the right need, and at the same time it is by no means a simple "snap your fingers" kind of thing. 


.,.,.

Well I am either good or lucky when it comes to negotiating.  We started the conversation by me being skeptical about his justifications for a $350M exit for the company.  (Such an exit is certainly possible if we notably expand the products offered over time, but I remain unconvinced about such lofty numbers for the more narrow basketball stats USA rec-league plus AAU markets.)  Still this debate serve my purposes well.  It got him strongly counter arguing well OF COURSE we have great chances to hit $350M or more!  via paths A or B or C or D!!!

After that 30min lead in, THEN I started talking about my idea:

I noticed how a $12M cap would actually push my equity stake down from 15% to about 3.5% once we got to $350M.  This is BELOW the level that other employees are at.  So in many ways this cap idea is easier to sell to his existing team, than johnny-come-lately getting more than triple their equity.  (Which is what his counter offer was going to give me.)

Once he got it, he was pretty sold (he is still going to "look at" at the written proposal, but he says "we are close" with this.)
As I expected he was also accepting of the CTO title.

I want to put stuff in writing here, and I want to frame my value at a high level away from Dan is going to come in as a CV expert on day one, into a more broad role which fits my skills and value better.  Also I want to wedge any other important things into the response email...



SO THAT IS MY REQUEST:  Would you propose any edits to the email below?  And are there aspects of the role that I should include here in this very high level proposal?


Thanks!
--Dan



.,.,.


Jason,

Here are the terms we outlined yesterday.  Falling asleep I did think...  "you know Dan, you just gave away a huge, HUGE amount of potential personal upside.  I hope you are not going to be kicking yourself HARD one day!"    but then I slept quite well, so I maybe my subconscious is getting ok with this choice   :-)

Dan will join Sports Visio as a full time salaried employee with a $200K annual salary and 2M shares of equity that vests over four years.  (This is equivalent to $500K of equity over four years, and is 14.54% of the post seed equity.)  Additionally Dan's equity is subject to a $12M cap.  Dan keeps 100% of the proceeds from the sale of his first $12M worth of shares, after that point 100% of all remaining shares must be put into a 503(c) non-profit from which Dan is forever barred from obtaining funds or income of any kind.

Dan will be a partner with Jason at Sports Visio, having a co-founder style mandate to get into the knickers at all levels of the execution of the company in order to achieve A+ execution while initially not having any formal report-to authority beyond team-wide acknowledgement of this mandate.  In this partnership DanO is more focused toward the technical side of SV while Jason is more focused towards the business side.  

I am jazzed that my role, on day one, would begin with a formalized philanthropic aspect to it.  This idea has been my abstract goal since leaving DARPA almost 15 year ago, but this will be the first time I am actually "putting my money where my mouth is"

I am also jazzed about having a mission the hinges on our ability to push the very boundaries of what humans can do with CV algorithms.  It is awesome (and rare) to find spots where cutting-edge science actually drives business value in a strong way.

Onward,
Dan


.,.,.
NOTES FOR THE CALL

that the success of this company will be intimately tied with development of novel CV algorithms.  This is _HARD_ and it will take me time to personally become expert, and to find/hire world class experts, but it is a mission that is 




**Performance-based Stock Bonus**
- Dan will receive his performance based stock bonus when sports visio is able to use AI to provide:  Real time basketball stats, a dramatic reduction in manual annoation time, or some other mutually agreed upon product or engineering pivot goal.


USE OF AI TO PRODUCE REAL TIME BASKET BALL STATS
- Usable during the game.  set of stats that are meaningful to view during game with lag which meaning.
- people want to use

DRAMATIC REDUCTION IN MANUAL ANNOTATION TIME
- AI enables cuts the time required for hand annotation by at least 1/3 time.
- Use AI in the support of manual annotation of basketball stats in a way that reduces the time required to produce those stats by at least 1/3.  (the resulting time is less than 66% of the original annotation time.)


.,.,.

- your partner & in for the long haul.  will spend years becoming a super expert in CV...  so this is a huge vectoring event.
- 

(1) Jason, it seems you are aiming for a co-founder style partner, rather than a super IC contributor?
     But from an equity perspective that is not what we are framing here.  But I have an idea.  If we are really talking about an 8 or 9 figure exit here.  Let me give some of that equity BACK to you!  Let's keep the $400K (15.3%) equity stake, but let's add a $12M cap to my deal.  This way, just after hitting that 8 figure mark I hit my cap, and I must transfer ALL additional shares to a 503c non-profit, never to go into my pocket.  If we hit the 9 figure point my equity drops to just a percent or two.

(2) Ok Jason.  Lets increase the performance bonus to $150K and transfer $50K salary into equity ($250K salary + $250K equity)


.,.,.

#### _
I know I am signing up for quite a bit of intensity here, AND I will really need to hump to quickly become a competent CV alg developer.
But I do worry that he will forever have the moral high ground to demand crazy performance from me.


I think my value is ZZZZ. 



(2) Ok Jason.  I accept.  This is a solid offer and I am very excited to ramp up in CV and to work with you.  Still this is NOT a co-founder style offer. 
     Gaining co-founder level single-minded total-commitment requires a co-founder level of equity.


-- X to Y --
(3) Ok Jason.  I accept.  But this is not co-founder level equity offer to me, and ultimately I think I will evolve to effectively BE your co-founder.
	I think it is plausible I will be central in driving the whole team towards our deliverables.
	I think it is plausible that I will be central both thru hiring and direct contribution in moving the company away from tweaking off the shelf algorithms into a space sustainable differentiation coming from novel basket-ball specific CV algorithms.
	Once it becomes clear that I AM doing these things, then I am going to be pushing for a real chunk of YOUR slice of the company!
	These are the things that move this company from one of the many towards a unicorn...  If I am central to those things then I will be demanding an equity stake that really matches it.




I accept your $600K total comp.  but lets push it to 250K+100K+50K equity and $250K cash.  this get me (barely) into double digit equity (assuming I maky my performance goal)


(3) Ok Jason.  I accept.  But this is not co-founder level equity.  Do you want me as your co-founder, or just a CTO with a more narrow.
	Co-founder fund-raising, 
	Looking for minority co-founder.


We've already talked about the things you could help with, but my thinking is 
- help on the CV side
- help on engineering help
- hiring on both sides from your network. And giving me 
- a strong technical partner and friend.
- help us build this thing up into a killer company using AI/deeptech for sports.

### n2022-10-05  Jason offer email
Dan,

  

Did some research and looks like $700k is definitely high/aggressive but also a real thing and probably something that you could swing if Google was hiring and wanted you badly enough. (Looks like that's the mid point for VPs and directors and on the upper end of possible for some lower bands)

It's tricky as our team is all closer to $300-400k too, and we're a startup. Obviously part of the startup appeal (besides working from home, cool people/projects, etc.) is also the fact your stock has a lot more upside than a big firm that might move up or down 5-10%. 

When we spoke earlier you proposed something more like $500-600k which sounded more doable. Our ENTIRE AI team of 5 people for 18 months has cost about $1M to date... I'm not sure spending 70% of that in a year on one person would produce more results (although I do think you're pretty awesome and want to get you on board!) 

How about $300k cash, $200k stock and a $100k stock bonus when we hit some mutually agreed upon big AI product milestone? (Some real time capability, or get human annotations down to a low amount, etc.) 

I think we can make that work on our end. And it's not an unchanging, never negotiable point... all of us will be revisiting our situation over time as things evolve. (I'm currently making nothing!) Given we are only burning $140k a month it's a big jump in our cash burn rate (20% jump and a full two months of runway!) but I think if you bring your A game it will be more than worth it. (Which will then also make the stock go up and make that a great deal for you too!) 

We've already talked about the things you could help with, but my thinking is help on the CV side as well as some engineering help and hiring on both sides from your network. And giving me a strong technical partner and friend that can help us build this thing up into a killer company using AI/deeptech for sports. What do you say, are we finally going to do this, or do I have to build another 8 or 9 digit company without you again!? :-) 


- help on the CV side
- as well as some engineering help
- help and hiring on both sides from your network. And giving me a 
- strong technical partner and friend that can help us build this thing up into a killer company using AI/deeptech for sports.

  

Jason
### n2022-10-04 Thinking about the Job choice

DOING THIS
- STRENGTHEN MY:  Deep Learning Alg development:  Learn how much I like; Learn how good I am at it.  Gain experience & reputation in it.
- 



After a Year
- I like the job & I am adding acknowledged value.
- The product is shipping


- In 3 months if I am central, then I push for more equity?


DOING THIS



### m2022-09-22  Joining SV as consultant
#### _

| Amount | Equity @ 13M | 4 year equity | 60% @ 100M | 60% @ 130M | 60% @ 350M |
| ------ | ------------ | ------------- | ---------- | ---------- | ---------- |
| 150K   | 1.15%        | 4.6%          | $2.7M      |            | $10M       |
| 180K   | 1.38%        | 5.5%          | $3.32M     |            | $12M       |
| 500K   | 3.84%        | 15.3%         | $9.23M     | $12M       |            |
|        |              |               |            |            |            |


- QL:  Prefers I simply get a stable income that covers us both while raising Eli  
  (money now rather than maybe more money later).  
- But she knows I am way more excited about Visio than Google.   
  So if SV exits, we should aim for at least an ok bay area income for the family.  e.g.  $360K/yr
- No half measures:   Dan, go big or go home!
  If you do this you will get totally absorbed just like DARPA, Martian Robotics, and Aeolus.  so make it worth it.
- -->    $360K ~ $8M net ~ $12M gross.        $12M * 67.7% * 4.5% = $365K

#### _
- Whole company is a risk for the fact that you don't have a fire-in-the-belly technical co-founder.
	- VELOCITY IS LOW - After 1.5 years you are running tweaked off shelf code vision code IN THE LAB.
	- IP IS OFF SHELF - Replication barrier is currently low, but most believe it wont work so are not trying.
		- FOLLOWER RISK - Market penetration is always years long, fast followers will eat us
	- RECESSION - Fed is not going to stop raising until inflation comes down & winter in Europe will be brutal.
	  -->   4 more months is not good.   8 is disastrous.  
- Can be the CTO:  Get velocity, focus, differentiated-IP.     Drive valuations w investors.

##### _
- $700K total comp and $500K worth of equity is a conservative measure of my value.  
  And hits my $360K/yr  @ 130M.      Not at 350M!
  --> Costs you an extra 10% of your 60% equity
  --> You are giving up 1/6 more of your equity to notably decrease risks with a technical co-founder.


#### _
- Here is my idea:
	- Let's keep the cash cap idea from before.  
	- I don't keep a penny more than $12M, so if we really do hit $350M its as if I only got the $150K equity.


- SALES IS SLOW / PARTNERSHIPS REIGN SUPREME - Sales driven by word of mouth.


#### _ sheets


| Amount | Equity @ 13M | 4 year equity | 60% @ 100M | 60% @ 130M | 60% @ 350M |
| ------ | ------------ | ------------- | ---------- | ---------- | ---------- |
| 150K   | 1.15%        | 4.6%          | $2.7M      |            | $10M       |
| 180K   | 1.38%        | 5.5%          | $3.32M     |            | $12M       |
| 300K   | 2.3%         | 9.2%          | $5.5M      |            |            |
| 400K   | 3.07%        | 12.3%         | $7.38M     |            |            |
| 500K   | 3.84%        | 15.3%         | $9.23M     | $12M       |            |
| 600K   | 4.61%        | 18.4%         | $11.08M    |            |            |
|        |              |               |            |            |            |



| Amount | Equity @ 13M | 4 year equity | 60% @ 100M | 60% @ 130M | 60% @ 350M |
| ------ | ------------ | ------------- | ---------- | ---------- | ---------- |
| 150K   | 1.15%        | 4.6%          | $2.7M      | $433M      | $10M       |
| 200K   | 1.5%         | 6%            | $3.69M     | $325M      | $13M       |
| 300K   | 2.3%         | 9.2%          | $5.5M      | $217M      |            |
| 400K   | 3.07%        | 12.3%         | $7.38M     | $162M      |            |
| 500K   | 3.84%        | 15.3%         | $9.23M     | $130M      |            |
| 600K   | 4.61%        | 18.4%         | $11.08M    | $108M      |            |
|        |              |               |            |            |            |

12

#### _

COMPS

| Level      | L7 Comp | L8 Comp |
| ---------- | ------- | ------- |
| Levels.FYI | $695K   | $1,077K |
| Levels.FYI | $709K   | $1,043K |
|            |         |         |





_
### j2022-09-22  Proposed SV Priorities

ASSUMPTIONS
- Premium Tier >=99%   $40-50/game
- Cheap tier cannot be more than 1/2 cost and must be at least >=85% accuracy.   $20/game



- less than 60% chance we will achieve fully automated cheap tier within next year.
- most customers will be in the top tier.
- Growth will be limited by sales for most of this year.  (both funding and future sales will mostly depend upon past sales)
  ==> THUS:  sole priority should be AI-assisted MANUAL game scoring
  ==> Metric.  Number of weeks until fully satisfies all outstanding sales & now stressing sales team to increase them.





PRIORITIES
- Demote goal of 100% automation.  
- Promote goal of Paid, Scaled, AI-assisted human scoring.
- Divert from second sport, and from auto panning, to hitting the AAU market & highlights reel generation.



1. 100% MANUAL GAME SCORING - Scale the fat finger workflow to the point of 
	1. Study manual entry.  Prioritize highest ROI AI & UX actions
	2. Being able to support PAYING customers.
		- Can we start with a high enough price point to fully pay for and new customer support staff? (in order allow scaling w/o taxing present team)
	3. Stressing the sales force's ability to onboard more beta/real customers.  (be careful about the operational costs of managing a large beta/real customer base.)
	4. Getting beta customers in the AAU market - Will they pay for weekly highlight reel?
	   
2. OPTOMIZE ML WORKFLOW 
         METRIC:  Mean total time for an experimental run.  (calendar time, and run time)
	- Testing velocity optimization - Reduce the total minutes required to test an alg change.    100K/yr = $2K/wk  = $400/day = $50/hr
	- Dataset versioning, management, annotation flows, annotation sets
	- Code versioning, one-click deploy
	- Cloud job execution harness
	- Job splitting; result collation; result visualization
	- System configuration optimization.  (single YAML file?)
	- Building a 20 core harness???   no.  
	- New system configuration & running.  Devops, docker.  debugger
	- Hey Joe, "look at this."   Run code, see outputs locally, see log/datafiles locally
	  
3. AI-BASED ANNOTATIONS
   - SHOT DETECTION - 30% to 50% hand annotation cost reduction by limiting watched video.
   - JERSEY NUMBER TRACKING - Further reductions by providing jersey numbers.
   - VALIDATION WORK FLOW - additional 2x cost reductions
      - IMAGE SET - 9 targeted images immediately surrounding a stats event.  Shot, Rebound, Assist.   (hover expand)
      - ONE BUTTON STATS ADD - System suggests one or more fully specified event annotation that reviewer can accept w/ just single click.
      - ONE BUTTON PRE-ROLL - 
      - FAST - 150ms response time preload on all user actions (with double buffering)
        

### m2022-09-13  - [[@Michael Seibert]] discussion




TEAM - 

- [[@Sam Corbitt]] - 
- 
- [[@Michael Seibert]] - 
- [[@Chris Stauffer]] - Automated testing - Tracking the ball backwards - 
- [[@Fong Liu]] - Jersey number - Camera calibration - trying to get down to one camera
- [[@Vishall Batvia]] - Yolo-X faster - daily :30 min, weekly with eng team.


- GUY GONE - Re-id problem:  see someone do something interesting.   who can sync their teeth.

- Beta test on set of apps
	- iphone & android:
	
	- CAPTURE - Game capture app - 
	- HOOPS APP - Post game player statistics app -  
	- WEB HOOPS APP - 



- AI TEAM - 
- ENG TEAM - 
- MARKETING & INVESTING TEAM - 



- Handful of game videos that are hand annotated - by visio
- Beta program being annotated in Nepal - Hand annotations.  AI annotations 
	- Dozen games, and growing


- Can compare bottom line statistics.
- Jersey Number - 
	- end to end - still have errors are happening
	- running in a reasonable time frame - 20x cloud time; P3Large (GPU++ & Memory++)   $20
	- Accuracy - Close to 100% precision.   Recall level 10%.    Human are near 100%
		- byte track - 



- Two different ball trackers - 
	- 
	- Frame differencing motion detector - (ball tracker #2)


- Detect the shooter
	- Vishall 3D Action Recognition - Look at player patches - sequence of patch recognition.    3D action model.
		- 8-9 months 
	- Vishall - Pose extractor - player with hands closest to the ball when shot happened.  for shot. rebound and for assist (pass within 2 seconds)
		


- 50% error - precision errors - just the wrong person.

- One camera panning back and forth - 

- Shot events accurately - really recall on shot.   (in the 90% on recall)


COSTS
About $3/hr on demand or $1/hr reserved.
- 24 * 22 = 480 + 48 = 528 = $600/mo ~ $6K/mo for a rack




4. ZZZ
### j2022-09-09  Value of SV based on Basketball


$13.6M post.    raised $3.6M      Jason 58%.     5% pool.  

    80K teams  =  200 leagues * 40 teams/league * 10x for total    - Assume total gyms is 10x of Lifetime fitness

	2K   gyms        For profit gyms where mens rec leagues play (USA for profit gyms)
	30   teams/gym              
	60K  teams       = 2000 gyms                                 (USA Men's adult teams)
	
	3M   games/year  = 60K teams * 50 games per year             (Games per year) (15 per season * 3 seasons)     
	300K games/year  = 3M game/year * 10% penetration            (Monetized games)  
	$3M  usd/year    = 300K games/year * $10 profit per game     (Annual Revenue) 
	$24M usd         = 3M usd/year * 8x multiple                 (Sale price) 
	.
	.
	3K   leagues     = 60K teams / 20 teams/league
	300  leagues     = 3K * 10% penetration
	$3M  usd/year    = 300 * $10K/league/year profit
  


Jason's bottom up stats:

      250 NH multiplier 243 = 329.5 usa pop / 1.355 nh pop;   266 = $120M / $.45M

    AAU
                                         $17K/year        = CapCityAAU (10teams * $1100/team/year + 35teams * $200/team/year)
    $136K/year       = AAU 8x $17K    (8+ other AAU programs in the state)   <--- other programs are probably smaller
    $160K/year       = MensBA 8x * (50teams * 3sessons/team/year * $150/session/year) * .9 after commission
    $63K/year        = High School Teams (90teams * 2seasons/year * $350/season)
    $45K/year        = Middle School Teams (90teams * 2seasons/year * $350/season)
    $60K/year        = Town and Rec (100groups * 6teams/group * $100/team/year)
    $464K/year       = Total in NH
    $120M/year       = USA total revenue per year = $116M = $464K * 250x
    $12M/year        = SV income = $120M * 10% penetration
    $96M             = Sale price for USA business = $12M * 8x revenue multiple
    .
    $360M/year       = World wide basketball stats
    .                  OTHER SPORTS: Ice Hockey, Baseball/Softball, Soccer, Football, Field hockey


Dan's refactoring per game only looking at men's basketball

    100K mens teams  = USA wide number of men's teams = 50team/gym * 8x gym/nh * 250x nh/usa
	4.5M games/year  = 100K teams * 45 games per year             (Games per year) (15games/session * 3x seasons/year)     
	450K games/year  = 4.5M games/year * 10% penetration          (Monetized games)  
	$4.5M usd/year   = 450K games/year * $10 profit per game      (Annual Revenue) 
	$36M usd         = $4.5M usd/year * 8x multiple               (Sale price based only on Men's Basket Ball) 


### j2021-06-29 - Jason Redux


Terms:
-- IP owned by Dan except sports-vision related
-- 22.5% Equity for 3/4 time
-- AF HOURS.  4% equity for 1000hours of AF time.

-- SALARY -- No pay for year 1 and year 2, ramping up in year three depending on health of business.  Year four should be at full pay assuming business is strong.




~-~~
$10M max after tax into Dan's pocket, additional monies must go to a charitable trust or in some way go to charity.  
-- This should be a legal agreement between Dan and Jason, and not tied to Dan's equity, so Dan can exercise the equity as needed before a full exit.
(I don't expect this to be large money, but rather a stipend if I am trying to do something after Visio, but before visio exits.)
-- Still Dan should give Jason's charity first right of refusal on any monies he pulls in this way.


$10M max into Dan's pocket.
The aim here is to ensure that if Visio hits big, that the lion's share of Dan's slice goes into some charity work that Dan feels best about.  But many things can happen so legally encumbering Dan's equity is not the best idea.  Instead Dan and Jason sign a contract limiting cash that goes unencombered into Dan's pocket.  In addition to this the expectation is that Dan in good faith is going to try to maximize the output of all of his equity.  So, for example if he sold future rights to the equity for lesser present value, he would offer an opportunity to Jason and his trust to buy those shares to maximize value from them.  (Presently, the main ways this seems it would happen is cases where Dan wanted to ramp down Visio time and persue other things using Visio monies, PRIOR to a Visio exist.  In these cases it seems now, that he would NOT want to sacrifice pennies on the dollar in early cashout, and would only want to take what was needed to live.  but who knows what the future brings...)


_

### j2021-05-05 - DECISION - NO TO JASON

### j2021-03-00 - >>>JASON BASKETBALL<<<
#### 2021-04-29 - Thumbnail Math
- 500 	gyms
- 30 	= 30 team/gym * 10 players/team * 10% penetration
- $150	USD/person/year
- $5K	/gym/year
- $2.5M	/yr
- $7.5M	Sale price
- $1.2M	= 7.5 * 15% Dan's take (for the market; double that for the tech too)
#### 2021-03-04 - The Nick and Dan LLC Model

	- WILDERNESS -- We venture forth into the wilderness of OPP (other people's problems)
		- IDEA FUNNEL -- Each sees many a wild and weird thing, most are of no use
		- CONJURES SOLUTIONS -- Many imagined solutions are spoken of to see if anyone will listen.  If excitement is found then he digs further
		- VET -- Still in the wilderness working only with quill and papayrus each vets and discards many dozens of these half ideas seeking a gem.
			A gem that will pay for the efforts of the expedition a 1000 times over
		- GEM == An idea w/ strong market signal & strong vetting (biz model etc.)
		- SUPPLIES -- The expedition will travel light a bit of admin time, and maybe some coach time... can pull them from his money or not worry about it.
	
	- BRINGING IT HOME -- What happens next depends upon the gem found:
		- AF CENTRIC BOOTSTRAP --
			- AF PROTOTYPING -- performed for equity at full freight rate with reasonable market cap (e.g. 3M or such)
			- AF EQUITY SPLIT -- AF equity for prototyping is split 50/50 and given directly to founders. (need to work out legal details on this;
				probably done @ low rate as loan w. low interest.)
			- BOTH FOUNDERS -- aim is to have both be founders, but each would eventaully need to have substantitive roles (see hold door)
		- AF CENTRIC + FUND RAISE -- 
			- Same picture as above, except AF fraction likely to shrink.  
				Still additional equity expected based on each person's role too.
		- NOT AF CENTRIC -- Then company is built and filled it with founders and such, the aim is for both to be there, but both have to have roles.
	
	- HOLD THE DOOR OPEN -- In general whatever happens the advantage to being a team is that each can lean forward on the exploration of an idea, still allowing the other to jump in later if an idea really starts taking off.  Thus for a limited time the idea is that the 2nd founder can still "jump in"



	_
#### 2021-04-07 - Jason #5

	2,000	for profit gyms in the USA
	500		gyms @ 25% penetration
	30		30 team * 10players/team * 10% penetration
	15K		Subscribers
	1.5M	$100/yr/subscriber. ($3 game)


	2,000	gyms.    For profit gyms in the USA
	300		players.  30 team * 10players/team
	60K    teams  * 50 games per year   (25 per season)
	3M     games per year   * 10% penetration
	300K   monitized games  * $20 profit per game
	6M     Total annual revenue from Men's rec leagues  * 8x multiple
	48M    Sale price   


	Jason,
	Quite a ride.  Being co-founders is quite a sweet picture for me, so I just decided to take this very seriously and do the early analysis as if this were already my business.



	EXECUTIVE SUMMARY

	1.  I agree we you and I are slicing the rec market differently, and my slicing gives a 4% penentration.  Below I argue why I think you should split your analysis into direct to team vs direct to gym, and I think we both agree direct to gym is 4%.

	2.  I am NOT bullish on the "Smart-phone-app direct to team" market.  Non-stickiness, thin margins, cost of cust acquisitions, app-pricing expectations all kill this path (see my analysis in #2 below)

	3.  I am bullish on the value of developing the underlying AI-based sports tracking technology, though it does carry a notable risk of being eclipsed before one can sell it.

	I split the AAU market into "up market" (4) and "down market" (5)

	4. I am NOT bullish on UP market AAU leagues/tournaments.
		This is not fully explored so this could change, but synergy and hudl are both in this market.  They have so much money one cannot win on price.  Manual works fine.
	
	5. I am less clear about the down market AAU teams.  These will be very rec oriented, or younger.  Either way they wont care about stats, BUT maybe some vanity/show-daddy-my-game works.

	6. The "Show-Daddy / Instagram-posting" market
		Many I spoke with see the value of a solution that intelligently highlights a single player within the game.  
		- This is a big market since the down market is large.
			It applies to #5 but others as well.
		- Still it has all the problems of #2 above (which can kill it)
		- I believe there is significant room for UX innovation here in order to make this product distinct and valuable.
		- But UX innovation can and will be copied.
		- In the end I am a bit negative on this market, but perhaps it is still possible to eek out a thin-margin, big-business with small-mote. 
	
	7. All the other markets.  You gestured towards international markets and all these other sport markets, and added up all the numbers and said wow this is a big TAM.  I am overall not very bullish on using a single TAM for this and treating it as a single addressable market.  But still I do see some hopes in this space see analysis below.

	8. I am most bullish on the direct to gym market.  For me this is the clearest gem in the whole space.  
		- The market is largely green field (no existing products)
		- Customers are hungry for a differentiation for their gym
			(and nearly all see tech as a sexy answer)
		- Solution needs custom HW (maybe we can build from existing)
		- Needs a second direct relation to teams/players
			(gyms were ok w. this as long we gave players something for free)
		- Most excitingly it seems plausible we might get a network effect with even a few as 5-10% of players have Visio accounts, and thus prefering Visio-enabled gyms.
			(could you own gyms as HUDL owns High School?)


	ROLLUP 

	For Dan I the gym market gem and value of selling the underlying tech are not large enough to generate the kind of exit required in order to make his resulting 15% stake worth giving up Analytics Fire and dedicating the next many years exclusively to Sports Visio.  So unless new info comes to light I am going to wind down my focus on Visio… 

	But notice my analysis for Jason is quite different.  I AM bullish on this picture if I were sitting where you are.  Your upfront risks are significantly reduced by your use of equity-driven early labor (smart move!).  Visio does require an upfront focus from you, but then as the work of developing a solution gets into full swing this can easily become a "one-of-many" activity for you.  Given its low financial costs to you, you could really do SEVERAL such startups in parallel if you can generate a few more good ideas!  Then you can spread risks over multiple paths.


	--Dan





	DETAILED ANALSIS FOR EACH SECTION ABOVE


	=== ANALYSIS FOR #1 - THE PROPER WAY TO SLICE THE MARKETS ===

	JASON WROTE:
	You're still slicing it wrong I think. You're taking 20% of gyms, and THEN only taking 20% of THOSE teams. That's 4% market penetration, not 20! That's a little crazy for a market with almost nothing available today. It's why your numbers are so anemic. I plan to sign up entire gyms like MBA at once (100%), and in cases when they DON'T do that, THEN go directly to the teams (or some hybrid model.) 


	I agree that my analysis amounts to a 4% player market penetration, however, I think this is an accurate analysis of the gym-to-team-captain path that I favor.

	In any case the:
		a. gym-driven-market, 
		b. league-driven-market, and 
		c. team-driven-market 
	are all very different markets, and the way one would try to attack each of them is very different.  Which results in a very distinct penetration and business model for each.  So even in this earliest stage I think one must consider and model these specifically and individually if one is to have any kind of accuracy in the result.

	NOTICE:
	If our goal is to show a huge TAM for our investors... sure just lump it all together and hope they don't notice.  
	But when we want to make the best model we can in order to drive best business choices we should not use that kind of analysis.




	=== ANALYSIS FOR #2 - THE "SMART-PHONE DIRECT-TO-TEAM" MARKET ===

	I agree one can market directly to rec teams to get them to sign up from some website or the app store, but I think this pathway is quite fraught for a number of reasons:

	— This is 100,000 teams you are trying to market to.  
	— This means you will be unable to have any kind of relationship-driven marketing at that scale.  You must rely on websites, and generic advertising.  BUT
	    — These team captains are not part of any kind of trade-show circuit or professional orgs.  They just love and play basketball.
	    — Thus you will have to advertise in very generalize channels in order to get to them.  This will be expensive.
	    — I think the cost of reaching team captains can easily approach or overwhelm the lifetime value of those relationships.
	— Also the team captains are not the ones gaining the majority of the value from having this service.  Rather it is the players on the team that are willing to pay the extra $30 to get the video clips etc. The are the ones gaining the big value in rec leagues.
	   (NOTE: I chose $30 per player since it represents a 10% increase in costs for adult leagues... I expect children leagues have much lower fees, but parents make be willing to pay a larger percentage increases too.)
	— But the team captain is the one who will have to manage the complexity of recording the games, etc.   I dont think he will want to, not for the bulk of the down-market informal teams.

	— The price expectation for an iPhone app is at a very different price point.  We can see the assistance apps that exist for basketball and other sports as examples.  I don't see such apps with large cash outlays per year per team.
	    Having a fixed camera in a gym puts us in a very different market.

	— There is no stickiness to an iPhone app 
	  (but a great stickiness for a physical device in a gym)
	— I am confident (as are you) that we will have competitors in auto game analysis (autostats is one that is already there)
	    Thus any iPhone app will be locked into a "race to the bottom" style of competition, even if not in the beginning, in the end any profit margin will erode away.  


	So for all of these reasons, the go-directly-to-the-rec-team-captain route looks like a non-starter to me.

	I do believe in the reg-league market, but I believe in the more narrow gym-install market as I have outlined above.




	=== ANALYSIS FOR #3 - Selling the underlying AI-tech ===

	I am bullish on the underlying tech.  I think it can be done (indeed it IS being done) and there are companies like Stats Perform that are already doing it (see below), and orgs like NCAA messing around with algorithms to do it too.

	Getting to SELL this technology is going to be a bit tricker.
	I think this can still work, but it requires building a specialized solution for the particular market segment you are aiming at, then selling that technology to a bigger player that wants that market.  (e.g. a gym install really requires its own signup flow, etc. all that is distinct to that situation.  good for us!)

	The Auto Stats algorithm from Stats Perform is probably good enough right now to address the gym market.  But the UX for their solution is all wrong for that market, and that is not their focus.  Thus moving quick and producing a ready made solution for the gym market could yield a mid seven figure "build verses buy" kind of opportunity for someone who wants that market.  (If Visio has ALSO already taken parts of that market, then this figure would go up.  See Analysis #8)

	So this is a nice little nugget.  but it really does carry time risk, since multiple players would just need to build the right UX, AND there will only be one or two buyers in this market... it is not big enough for multiple big corps to be duking it out over the gym market.


	PAPER SHOWING BBALL TRACKING
	https://global-uploads.webflow.com/5f1af76ed86d6771ad48324b/5f6a64711514cb41b91b37d5_Johnson_extracting_player_tracking_data-compressed.pdf

	Not sure if you saw this paper, but we should try to connect with this Neil Johnson fellow.  He basically attacked the full BBall tracking problem with several smart algorithmic hacks.  He achieved 92% accuracy (but without enough data to have confidence in that number.)

	NOTE: His paper really shows us the challenge of getting good labelled data.  And it also shows that one smart guy can spend less than a year to make very real progress on this problem.  So we cannot treat this as a super strong technical mote.

	STATS PERFORM AI-BASED TRACKING
	https://www.statsperform.com/team-performance/basketball/advanced-player-data/

	These guys are not presently attacking the markets of interest to Visio.  but when considering the selling of the underlying AI-tech (which is the topic of Analysis #6)




	=== ANALYSIS FOR #4 and #5 - THE AAU MARKET ===

	I am not very bullish for either the up market or down market AAU opportunties.  This is an area that merits further investigation, I am not fully confident in my opinions here.  Still this is what I found:

	UP MARKET
	- I am not bullish on the up market AAU opportunity.  As one person commented, ``they have infinite money so you cannot compete on price.''  I think this is right.  This leaves us competing on features.
	- But we already know that Synergy is in that market, and HUDL is messing with it.  
	- Because I think network effect across the AAU leages will be strong, AND I see that we have two established players already serving this market, I think the chances that there is an angle in on this market is not high.
	- What features are we going to be offering this market that they current players cannot offer at any price?
			Realtime stats seems to be the only answer.
	

	DOWN MARKET
	- League fees for children leagues seems to be notably lower than the fees adults pay at rec gyms.  This is something that I only have indirect evidence for, but I have multiple data points making that same claim, and it also makes sense to me logically.
	- Also it seems pretty clear that the down market teams are not going to care at all about stats, so at the team level there is little pressure to adopt a Visio, it provides little benefit for them at the team level (it's all about show-daddy).
	- Because of opportunity #6 (show daddy/instagram) there could be a play where the children's parents for an AAU team coordinate the usage of some Visio solution, and then collectively pay for it.
		- This will be a marketing hat trick to pull off that kind of market adoption model, but maybe it can be done?

	So if there is an opportunity in this space, it is down market and connected with #6 below.In the end, I only spoke to three AAU leagues and none were positive hits for this idea, and found a Synergy user who could show me their penetration into the AAU markets.  ugh.


	Still for both up market and down market, I did not do enough analysis for me to feel confidence in my negative outlook.  So I will just end with a magic 8-ball answer:
		Don't count on it

	Or at least, if I were you I would assume it will NOT work, until you have found compelling evidence for some sub-part of that market that is telling you a very different story!




	=== ANALYSIS FOR #6 - THE INSTAGRAM AND SHOW DADDY OPPORTUNITY ===

	I like this use case.  It applies to multiple different customer segments above.  Further, multiple people I spoke with saw an opportunity for some kind of intelligent per-player video-focused summary service.

	More serious players are already paying very real money to gather videos and cut "player reels".  As much as $300 per GAME! in order to have a person attend the game and record their play!

	Less serious players nonetheless love the game, and love that their kids are playing the game.

	This second case in particular, feels ripe for innovation to me.
	I think there is an opportunity to generate some polished "montague" style outputs, that are designed to put into an electronic picture frame, or on social media.  So in addition to the technical innovation of player tracking, the UX innovation of getting those polished outputs into ideal form, and connected in all the right ways to all of the right media channels will be have notable customer value.

	Moreover this is potentially a very large market, since the down market parts are much larger than the up market parts in all of the markets we are considering.

	This biggest negative I see for this area is the dearth of ANY kind of a mote.  It will actually be quite HARD for you to iterate and innovate to get this UX right.  But then it will be super EASY for any competitor to just copy what you did, once you figure it out.  No mote.

	Any mote will come from some kind of lock on some marketplace.  Sadly it will not come from UX innovation... it will come from some other place.

	Thus as a business model for Visio, I don't love this path.  But I do like it as an ADD ON feature for some other market penetration model with its own market stickiness mechanism.  In that case I do see these features as a significant value multiplier.

	(But we know that multiplying anything by ZERO still gives you ZERO.  So this feature cannot be the backbone of any strategy.)




	=== ANALYSIS FOR #7 - ALL THE OTHER MARKETS ===

	JASON WROTE
	And by the way we want to add other sports in, like soccer which is 7x more popular than hoops globally! I got an interesting data point that UCLA is paying $1400/month to a kid who just sits there and types in stats for volleyball games and they'd LOVE to have an app that does that for them. I can't imagine volleyball stats would be more difficult than hoops, seems way easier given the smaller size and lack of horizontal movement between players (I guess ball tracking might be a minor issue?) Market has to be WAY smaller (IMO) but if there isn't an option at the D1 college level like UCLA is, then there is a chance to enter higher up in the market and maybe make our investment back quickly. 54 men's teams, 330 women's... at over 4 months long and $500/month, that's $2k a team... assume something optimistic like 50% penetration/200 teams (some other D3 or whatever) and you'd have $400k a year. Not exciting. There are 500,000 players in HS though. There are 11 on a typical varsity team, so that's about 50,000 HS teams. At $250/team/season (pretty conservative, they play like 25 matches so that's only $10/match for 60-90 minutes, that's $2.5M if 20% use it. Gets more interesting. Add in some pro leagues and you could probably get to $3M+ a year in the US, maybe $9-10M globally if the same ratio applies as hoops. Not as exciting by a long shot IMHO, but interesting exercise for a niche sport I suppose. 


	~-~~

	Yeah, there are alot of other sports out there.  If you go for the gym market, two different gyms mentioned flag football as their runner up sport they would love to have covered, and volleyball was mentioned as well. 

	Still I am NOT bullish on computing a TAM for these others and treating them as part of the market to be addressed.  Indeed I don't think this market will be organized by by sports at all.  Instead, just as with HUDL and Synergy models, these solutions will break down by customer type, and then provide a single multi-sport solution to those addressed customers.

	So in calculating the additional value one obtains from these additional sports one needs to compute the incremental value one obtains from ones EXISTING market segments, but then selling new sport solutions to them.

	This will be a much smaller number, but I think this is how these markets are working.  Getting penetration into gyms for basketball, for example, will provide real lift in getting volleyball sales into those same gyms, but it will provide nearly no benefit in attacking HUDL's lock on volleyball in highschool, or Synergy's lock on certain collegiate teams.

	I agree there likely are other greenfield opportunities in non-basketball markets, but one cannot just compute a TAM over the whole space and have a meaningful number.  These greenfield gems will be few and far between.

	And I expect that those other greenfield opportunities will be also down market in the cracks not covered by the customer-centric horizontal plays that dominate the upper markets.

	Further, even in these down market areas I expect they will get eventually get filled by customer-centric multi-sport plays just as HUDL is doing to highschools... it is up to you go find and grow your own horizonal play within some yet unknown down market greenfield starting spot.




	=== ANALYSIS FOR #8 - THE GYM MARKETS ===

	I like the gym market.
	- It is greenfield in the sense that few gyms have a solution today in their gym.
	- Its customers WANT the product today.  Especially on the heels of Covid, multiple gyms I spoke with are all looking for ways to differentiate themselves from other gyms, and each one that I spoke with liked using a technology-based differentiation.
	- It is very sticky since you are installing actual hardware in their gym.  It will be hard for others to displace you once you are in that gym.
	- It might have a network effect.  If you penetrate a couple of gyms within an area, then you are in a great position to establish a relationship directly with players in those gyms. This will in turn put pressure on other gyms to also have the same technology, otherwise they will loose all players that have a Visio account.

	**RUN AWAY NETWORK EFFECT**
	- In my most optomistic scenerio, Visio could actually tip the whole gym market within a geography
	- In the case that most of the costs for the Visio HW are offloaded onto the players served by a gym, you might get the costs for installing HW in a gym down to near zero, for the owner.
	- At the same time, as players within a geography tend to bounce around between gyms, and will put pressure on non-install gyms to install the system.
	- If the market is still greenfield in that moment, AND Visio costs are near zero for the gym, you could have a wild-fire mass adoption within the geographies with player adoption.
	- Perhaps even a small penetration (5-10%) in the player community might be enough to drive mass adoption for the gyms within the geography!!

	THIS IS AN EXCITING IDEA!

	I dont know what the chances are for this to happen, but each of the individal parts seems very plausible... it is just a case of counting chickens well before they are hatched.  

	One should not count on such a massive win in this early stage.  But still it is plausible, and if that kind of wild fire adoption did happen, it would boost my earlier email analysis by a factor of 3 or 4.  The wild fire effect would not drive up the adoption per player, just the adoption per gym.  There will still be those players that care and those that don't.  But it could drive the 20% gym adoption to 70% gym adoption, and it would lead to a nicely LOCKED UP MARKET too as well.

	_
##### xxx
	OLDER STUFF

	It would move the gym market from a $1-2M annually to $3-6M annually.

		I am NOT bullish on a tech-forward attach on these other markets.  I do think the tech is relevant to those markets, but just as with the highschool market, it will turn out that the way existing tech is laid out will dominate the discussion of the markets viability.  
		I am bullish on using the MARKET penetration that one achieve with Visio to take on other sports.  So for example if one sells gyms on cameras for basketball THEN one can upsell volleyball and flag football.
	~~
	Still in the end I think these sports are going to be covered by HUDL or others if at the High School or Collegiate levels.  So I think you are still looking at scooping these other sports out of the gym market.   This will definitely help, since you just need a few more cameras and you have covered the whole gym, but I bet this is not more than a factor of 2x boost since I think a lot of the energy is going to come from basketball.
	~~
	All the other markets.  You gestured towards international markets and all these other sport markets, and added up all the numbers and said wow this is big.   
	~~
	You asked me what I thought the next steps were.  Well I do want to do a more digging on the AAU market, but so far I am not seeing even a mid eight figure exit as possible, forget a nine figure exit.  But still I think this remains an interesting proposition for Jason Syversen.  You have got a number of guys coming in at 1/2 salary, and the rest at low salary.  Then I bet you will succeed in your seed investing (I am not so sure about the VC investing).  But that is enough.  You can just start installing into gyms and making money.  It will take some years of effort but it will be 50x ROI for you money in.  Not too shabby!  Especially since I think you can do this as one of multiple things you are doing.  

	For me the equations may not be as rosy.  I need to give up analytics fire and other activities, but at a $20M exit I net a $3M after some years explaining to the wife why I am not earning anything yet.



	_
#### 2021-04-01 - Jason #5

	Jason,

	My plan is to wind down my investigation soon.  Since I saw options outside the highschool market I decided not let my negative results there, stop me from digging further...  

	And just as you found in your discussions, when I got into the league and gym owner market, I started getting much more positive signals.  Without exception gym owners are bullish in principle on the idea.  They are in a competitive race to get players and teams to want to come to their gym.  Especially on the heels of Corona each one of them is focused on what they need to do to out compete other gyms in getting the players/team to choose them.

	It is going to be tough to roll out fast enough, but this will be an awesome wave to ride if Visio can make it in time.  Gym owners talked about wanting to bundle this offering (for free) to their members, then one could try to upsell individual 








	_
#### 2021-03-30 - Jason Summary

	-- 60% founder situation is good
	-- 60% tech works
	-- 60% market works
	   ~-~
		22% success.
		35%	= 70% founder * 70% tech works * 70% market
		34%	= 70% founder * 80% tech works * 60%

	

	Hill I want to die upon: 
	-- Configurator(Info,Mkt,Wizard),  
	-- Trainable AI


	15% * 3x sale price = ~ dan gets 50% annual revenue


	_
#### 2021-03-27 - Jason analysis #4

	Jason, 
	The analysis continues!  Getting people on the phone is quite a SLOG, but I have had some luck.  Including two managers of a local YMCA.  Both were generally positive on the idea, one even said he wanted to find a solution by this fall! (Details at bottom)

	Because the league market is services BY the gyms, it occurs to me that one way to understand both of these markets is to do an estimation based on these gyms.  Here are some slices thru the data.  I would like to get your (and Sam's opinion on these calculations):


	$300	Annual cost per player.  ($3000team/year / 10plr/team)
	$30		Visio revenue per player per year.  (10% total spent)

	2,000	For Profit Gyms in USA. (8 * 328M / 1.36M = 1929)
	20% 	penetration into market
	400		Visio Gym installs

	15		teams/gym
	10		players/team
	150		Addressable Players within one gym
	20%		penetration into players at gyms
	30		paying players at one gym
	$900	Visio revenue per gym.  ($30 * 30)

	360K	Visio annual revenue from players @ for profit gyms
	~1M		Visio revenue including intl market  (using your x3)


	Of course there will be other markets than just the for profit gym market, but the thing about this market, is I think we could land it and keep it.  Other markets have less $$$ per player, and the iPhone app market I think is not sticky, is lower price expectation, and will get overrun with competitors in the long run generating a race to the bottom.

	The one other big market this does not cover is the AAU market, particularly selling to tournament directors seems very interesting since it would be sticky in their gyms, and they have CASH!

	--dan




	$150	Assumed revenue per team per season
	$50		Assumed revenue per player (I made this up)
	2,700	YMCA in USA
	8		For Profit Gyms in New Hamshire
	2,000	For Profit Gyms in USA. (8 * 328M / 1.36M = 1929)
	5,000	Total Gyms in the USA

	1,000	Total Gym Installs (assuming 20% penetration)


	2		Seasons per year
	20		Teams per gym per season
	10		Players per team
	400 	Total rec league players per gym per year (2 * 20 * 10)
	8		Teams serviced, assuming 20% penetration (2 * 20 * .20)
	40		Players services, assuming 10% penetration

	$1,200	Fees per gym (assuming $150 for 20% of teams)
	$2,000	Fees per gym (assuming $50 for 10% of the people)


	~ $2M	Annual revenue from the Gym market.

	Notice, I am getting something like $2K per gym per year.
	(Not $10K or $20K.  I need a crazy 20% penetration and a $100 per player I get = 400 * .3 = 120 * 100 = $8K per gym

	~ $8M	Annual revenue with crazy optomistic per person pentration rate (I think it is not reasonable, but I just tried it out.). And I also double the per person rate to equal our original per team rate.  This also seems unlikely, but I am not a clear about that.  If we add lots of vanity video and such, then while it wont increase the penetration rate, it does seem for those that care, it can increase the rate they will pay.)


	~-~-~
	CALL DETAILS

	Based on my early calls to gyms I *DO* see the same gap in the market gym market.  Their response to the technology is quite different than at the highschool level.  (Hooray!)  They just do not have any technology like this, and HUDL is way too expensive.

	I was not able to get interest from any leagues, but mostly I am just not getting them to talk to me.  (A few did, and they were not in a position to pay for such a thing, but I still just don't understand that market.)

	One manager of a YMCA was *very* bullish on this technology.  Indeed he told me that they were looking for a solution like this to be purchased by the fall time!!  That was shocking to me, how could this be, when this technology doe not exist yet?!?  But when I pushed further it turns out that their goal for the year is to introduce more technology into the gym as a way to differentiate the gym.  (The have had good luck with other kinds of analytic technolgies, for heart, and ideal respiration, etc.)  So he was not looking SPECIFICALLY for this technology, but once he understood it, he wanted it!  He like the stats, and he liked the idea of a system that provides a montague of a players best shots or such even better.  And best of all his inclination was to bundle the cost of this service into his gym fee, instead of charging each team.  He said it would probably increase his membership fee a "couple of bucks", but then eat the rest of his costs himself.  I said so charge each team something like $10 more?  He said yes, but he was fine supplementing this since he sees this as a way to draw more customers.  Still we are talking $10/team!  And I am the one that said this number, he said a couple of bucks, so I think he was thinking less.  So I cannot imagine he is aiming for anything beyond a couple of thousand AT MOST. For covering his gym.

	Also it seems to me that getting this service installed INTO the gym is pretty key.  that way no one needs to do anything, and the game is recorded by default.  I think this will be huge in converting new customers.  I suspect this is the main way to go.  But notice it does mean that we are either developing our own HW.. or there is a solution that we can access that is not part of a competitor's bundle (like HUDL focus).

	~-~~

	In the end I am doubtful that leagues will pay very much per team in order for the whole league to have this service.  I am not sure, but it seems this would happen in the case that a league is competing on tech features against another league for some coveted players.. but is this really the circumstance?

	I *DO* think that gyms want to compete on features, but as I look at the numbers it seems hard to see how a gym would make money by paying $20K extra for this service.  $2K or maybe even $5K makes sense, Still I don't have hard data yet to back that belief up.

	I am not at all clear that one can parlay ones wins in the gym market into other market.  I do see evidence that once one in a single gym, they will want us to provide stats for other sports.  (several have mentioned flag football)  but I am not clear that this will translate into much larger figure per gym.


	~-~~

	So this is an update from the last time we talked.  For the first time I am hitting folks that clearly see the value of this service, and do not already have access to the service.  COOL!

	But so far I am not seeing anything like $20K per gym, nor am I seeing the abilty to jump from the gym/league channel to another channel.

	(I am kind of bundling these two channels together, since anyone who plays in a league often is at one of these gyms, and anyone who plays regularly on a team at one of these gyms is in a league.  Correct me if I am wrong about that.)

	Thus it seems there **IS** a chance to own a market (the gyms) as well as the underlying tech.  so this puts us in the solid eight-figure range, but still the low eight-figures.


	Thoughts?
	--dan














	~-~~-~~
	MY THINKING








	_
#### 2021-03-20 - Jason response #3


	Your research is making me to want try to exhaustively describe the market in NH. I'm going to try to find EVERY AAU team/program, gym with leagues, and nail down the number of private High schools. All of those are sweet pickings for us, I agree the public HS market will be tougher. I also suspect we can pick up JV programs and/or middle school teams too as I doubt many of them use HUDL. I've got some marketing intern candidates lined up, was hoping to do that. 

	Anyway, also found an interesting note... Pixelot is a sports camera manufacturer that I think is seeing the vertical integration happening from HUDL and getting nervous. They bought Vidswap, an "AI" video production/analysis platform to facilitate sharing game analysis and such and I think fend off the HUDL monopoly. Note they have a large amount of customers too, so all those times you're panicking that HUDL owns the entire market this should be slightly reassuring. (Since they don't do automated stats) https://www.pixellot.tv/press-releases/pixellot-announces-acquisition-of-game-analysis-platform-vidswap/

	Jason

	~-~~-~~


	Jason, 

	When you have a moment, I would love to get on the phone to talk about all of this analysis I have been doing lately.  I have never gone this deep on diligence for a company that I didn't yet work for!  I've been going to the mat for this because I really do value co-founding with someone I trust and someone that I believe in.

	Yeah, a couple of of the high schools use and love pixelot... I think the vid-swap company is all about making professional looking video feeds w/o human support.  I asked one coach why have BOTH a HUDL Focus AND a Pixelot.  His answer was ``Because vidslot has a relationship with the NFHS network so you can broadcast your games on cable’’  So I actually think vidslot play is also a network play (but in this case it is an actual TV network :-)  I do think the video feed market pretty fragmented and open.  On the east coast at least there multiple other companies that specialize is video game feeds.  Scarsdale TV auto feeds to cable and to vimeo channels too.  Local Live will record games for cable, but then also auto feed them into HUDL too.  But you are right, there is more openness in that part of the market.

	I have spoken with a couple of vision tech companies as well.  Just as you found, it does seem like this is an area where the tech is up to the task.  I think one can likely build tech that gets up into the high 90s in accuracy, and in that case I think multiple players in the market will be interested in buying the tech from Visio.  I think the tech by itself is an eight-figure exit.  I think you and I agree that companies are not going to pay more than $10M for tech that really exists in many places even if not applied yet to sports.  They can build it or buy it for less.  The real key is building the tech and having significant market share too.

	If we managed to find a nitch that can be dominated with this tech, then it is possible this could be some kind of nine-figure-exit since we are owning both the tech and a market.  But I am become dubious about the viability of this option.  First I am not sure there IS a nine-figure market available to be taken.  Sadly, I think we are now competing on price and not on features; anything an AI can do, a human can do too (Indeed this is what the competition is already doing).  And on price alone I don't think you can gain market share in the HS, College, or Pro markets.  As you see with my early looking at the AAU market below, it really did not feel great at all.  Perhaps I am over-relying on just a couple conversations regarding the gym market, but those conversations really put a web blanket on the idea that rec team players would pay for stats.  Perhaps a low nine-figure exit could be possible based on providing gyms with other services not as focused on stats.

	The other consideration is that the further we get away from AI-based stats, the more it matters that we out innovate others on our business model, our UX, our partnerships, etc.  But I think this is getting further from our core competency.

	Finally a ten-figure exit is completely out of range in my opinion.  In order to hit this, we need to make significant inroads into multiple sports within a moderately big market like high school sports or such.  I just don't see the opportunity to do that.  The network effects of the existing players are too strong.  Breaking that requires that we can do something amazing which they just cannot do.  The SPEED of delivering results seems to be the only real feature I can point to where we could win.  That is a super cool feature, but not a ten-figure feature.  In the highshool market I asked more than a dozen coaches if I gave them amazing features at some stupid cheap price could I get them to leave HUDL for scouting.  Not one of them said yes, unless I could get EVERYONE else to switch too.

	If you have time this weekend or early next week I would love to catch up by phone.

	Cheers, 
	Dan






	~-~~-~~
	Quick Analysis on the AAU market

	This is a quick and very dirty analysis.  I would not trust it as definitive, just a quick sniff test really.  I only managed to connect with a handful of coaches.  This is what I found.



	Their seem to be many coaches that are really doing this as a thin margin business, they are coaching younger kids whose parents what them to have fun, or maybe end up good at sports.  But the coaches seem to be far less interested in really developing the kids in systematic ways.  None of them saw value in stats for the low end.

	There is a second class of team.  The elite team.  These guys have money, but they already have services that record the kids and create highlight reels for them.  Even in this area it seemed that keeping stats and using them was just not a priority.  I notice that one could easily use HUDL in these contexts, but no one is doing that.  For these elite teams the cost of HUDL assist is definitely in range, but there seems to be ZERO penetration into this market.  My tentative conclusion is that they just don't care very much about it.  (and the coaches I spoke to agreed with that.)  One coach did say he uses a team mom to keep stats.








	DETAILS


	Focus seems to be on Highlight Reels not on coaching!

	Instagram is big.  Twitter is big.

	``If a player feels they are not getting enough exposure they may pay a service $300 to come out and record a game and put them in their feed’’

	Example promotional companies:  West coast elite, Simply basketball

	For stats most use max prep (but I think stats are just uploaded into this service, even from HUDL) 

	Looks like there is zero usage of HUDL in the AAU leagues.

	For the elite kids they already have services (but still not stats???) for recording them 
	For the younger kids it is really just about developing them (but not using stats)

	He did not seem to think anyone would really care about stats.

	~-~~

	``A team mom takes stats.''

	DAN ASKS: What would parent's pay
	``$100kid for an 8 week season.''
	(Note:  That would be awesome, but I really don't buy this as this guy mostly spoke about how they don't care about stats...  )

	DAN ASKS: What other fees do you pay
	``teams typically pay $500/month for court gym access''








	  The second bigger concern about the idea that we are going to dominate a market is that neither you nor I are B-to-C market specialists.  We are technologists at heart, and leaders at heart.  I know you figure you can just wing that part, but I disagree on this point.  Already the business is operating in a tech-forward kind of way.  That will get you a tech company, but will not get one to dominate a nitch.

	Even if we manage to get a low nine-figure exit it probably does not make sense for me to make a company back flip for that kind of exit right now.  My likely share of 15-20% means that I might not even beat a google salary even if it all wins!  And I think a ten-figure exit requires that you take a chunk out of a big market.  I don't see the market where that can happen.  I don't think you are going to take any notable chunk of the HS market, or college, or pro with a tech-forward play like we are talking about.  Markets often turn out to be larger than you first thought, so maybe somehow one of these other markets can be a ten-figure exit, and maybe you can dominate one.  But I see all of this as a long shot.


	So best of luck, it is a cool idea, and I think the tech and approach is very valid.  I think running this while doing all the other things you are doing is a win for you.  So I would still do it, if I were sitting where you are sitting.  The logic is just different from where I am sitting.

	Strength and Honor!
	--Dan

 



	_
#### 2021-03-17 - Jason response #2


	Jason,

	calling, calling, calling,  :-)
	Here is my update:

	Market Analysis Update -- The Bad
	--  Part of HUDL's strangle hold on the HS mkt is coach scouting.
		Even coaches that do not care about HUDL or stats *still* get the basic pkg.
		If school A is playing B, they find C that recently played B and ask C for their HUDL of the game.  With a click they get it.  So everyone I asked said they will keep HUDL for this one feature no matter what we provide!!
		(So much for saving money as a feature!  unless somehow our cost is negative.  See the badger league comment below)
	--  Selling to Daddy -- I thought this feature could really differentiate Visio.
		Turns out HUDL assist provides Daddy with a log-on special just for him.
		He can do the same filtering search that the coaches can do.  So Dad already HAS the nifty feature I hoped to give him.  The ability to get a summary of Johnny's play.  Now I agree one can likely build a better UX for Dad than the HUDL interface.  Likely yes.  But enough better that the coach is going to maintain two systems?  Ugh, I am doubtful.  Plus none of this is really building from our core competency of AI auto reco.  That part is just giving us a cheaper price...  which does not help if they need two systems!

	Market Analysis Update -- The Good
	--  The AAU market (dominated by Nike, Adidas, and Under Armor) does not play games against high school teams, so this scouting network is not relevant.  They have money, and some of them really care about winning/player-career/etc.  I am going to try to talk with some AAU coaches from the three major groups and see what tech they use.  Maybe there is a window of opportunity here?  (But notice even if so, HUDL's tech as it exists today applies perfectly here...  It will be a race if they choose to care!)
	-- Several coaches said they would pay for an iPad app that gave them stats at half time, and would pay for stats on their practice games, and after one-on-one sessions with a player.  But for this use case, they were espeically senstive to complexity.  They just want to push a button, so a fixed mounted camera seems required.  But this is just stuff that HUDL *cannot* do at all!



	DAN-O CONCLUSIONS ON THE HIGH SCHOOL MARKET
	-- When it comes to the High School market I am reminded of a bike lock that I had when I was in NYC...  It was called the Fuggedaboutit.  The manufacter never claimed that it was impossible to steal your bike... just that it was so crazy hard that any sane bike thief would simply look elsewhere.  I think it does not matter how perfect our tech is, we are now reduced to competing on price in a very unfair way.  We are not providing stats that HUDL does not provide.  Or maybe we create some UX, or clever usage hack.  I am doubtful these will work, but even if they did, that is not the core tech we are hoping to develop.
	-- The one gap I still see for Visio is SPEED and VOLUME.  coaches that have the money to spend and who love stats, really want those stats FASTER, and they want them on all play even on practice games -- Using HUDL for that starts to get really expensive.  For these fat-cats I bet you could sell them a special second service, on top of HUDL ...  especially if we can somehow hack the HUDL focus camera which they cerrtainly are already using so it also sends video to us too.
	-- But overall my judgement is this is just a Fuggedaboutit market.  
	Lets just look somewhere else for a viable market.  (E.G. maybe the AAU market, or rec gyms are the good market to hit.)






	THE DETAILS


	SCOUTING
	HUDL has a pretty clever pricing model.  For $400/team/year one can get video game tracking...  This is actually their feature of greatest value for most coaches I have talked to.  So why give it away so cleaply?  In order to have total domination of the market.  They get extra value since their HUDL assist is actually able to give stats to the AWAY team without any complexity for the paying AWAY team since the home team also has a hudl video.  It locks the whole market into their ecosystem.  No one wants to mess w. game video that is not in HUDL.  F-ing brillant!

	As one coach with a heavy Italian Brox accent explained:  I know my team, I don't need no spreadsheet to tell me who can rebound and who starts, but I don't know THE OTHER team.  So HUDL scouting is really the key for me.  And getting the game video outside the app from another coach is a hassle for that coach, so that coach probably wont bother -- then I am stuck w/o a scouting video!

	I think this one feature is total killer.  Notice, we CAN provide value that coaches will value and pay for, but we have to do it assuming they are already using HUDL... and probably a HUDL focus.  (most dont seem to really want to screw with video capture from phone, but I don't have hard stats on that... it is more a sense I get.)

	What this means is that if we want to undercut the HUDL price we need to live in the delta between HUDL video @ $400/team/season and HUDL assist @ $900.  Personally I think this is a bad place to be.  If I were HUDL I would raise my video capture to $500 as I consolidate the market and maybe drop my assist price to $800 if Visio were challenging their total dominance even a little bit.  Now @ $300 per season we are at the same price delta as Assist is already at.  
	And notice, for the coas far less complex to get all service from one company.

	Personally I think the network effect from this one feature is just KILLER for this market.  We can sell tech to HUDL, but I dont think we will penetrate this market.  Of course the HS market is not the only market.  but this looks like climbing a nearly smooth cliff face.  A think best just not attempted!




	AN EXAMPLE OF A TEACHER AT A LOW BUDGET SCHOOL
	--> still notice their entire conference mandates that everyone uploads to HUDL
	--> this simplifies the scouting for everyone

	Tyler Kuehl (608-316-1865) at Monona Grove High School

	This school has less money, so they don’t have a hudl focus, but most schools in their conference do.

	They use both hudl and hudl assist.
	-- on Assist:  ``It is huge; it frees me up to focus on the kids’’

	!!!  ``The badger conference in has a rule that all games must be uploaded to HUDL’’

	``Every dollar really counts at our high school.  So it if were cheaper then that would be interesting.’’

	``would definitely be important to have stats at half time.  For both my team and the opposing team’’

	``maybe it would be valuable during the game if it can track; minute breakdown of the players (too see if I am using them too much), to so show me we are taking too many three point shots, and need to get into the pain’’

	``Most schools in our conference have HUDL focus, but we don’t.  It costs too much.’’
	``It is a hassle to have a JV players sit by the iPad I use to record our games and move it around.’’


	DAN ASKS:  ``Any other feature that would really help.’’  after thinking long he says
	``Maybe some body mechanics tracking like the home court app built in could be nice, so I could tell the kid when an angle change on his 3 pointer is causes misses’’





	_
#### 2021-03-17 - Jason response #1



	Jason,

	Some quick comments at the top then lots of details below:
	- Dan's risk aversion.  I think it is the same DanO, but this is a larger commitment AND having Analytics Fire means I have a larger opportunity cost as well.  Still I am down with working for years w/o pay for a good shot at the glory!  (I just want it to ACTUALLY BE a good shot!)
	- I do agree HUDL would likely buy instead of building.  And I do agree if we have this tech, they will be very interested.
	- But I think we are not operating with excellence when it comes to the analysis of the market or nor our analysis of HUDL... details below.




	MARKET ANALYSIS
	- I don't feel we are operating with excellence here.  We are having folks quit jobs, committing $200K, etc.  and we have not gotten 30 customers to tell us they would buy this, what it needs to do, and how much they would pay.
	- Precisely ZERO of the more than a dozen B2C, company owners I have spoken in the past two weeks have given me a passing score in this area.  Most have just given me shit about being amaturish.  (I do admit it HAS been hard for me to get coaches on the phone, but just had two more today...  slowly it goes)
	- Everything I am hearing from existing HUDL users says that you are mis-informated about what HUDL does.
		- It DOES offer video clips of all plays.
		- It DOES break those out by court location, player number, and shot type
		- See the 0:58 mark here:
			  https://www.hudl.com/products/assist/basketball
		  - And this has been confirmed by three different coaches too


	- I hear what you are saying about you and Sam... two scrappy dudes and you made siege!  But I don't think we are really properly thinking about the mechanics of entering a market with **ONE** super dominant player.
	- Yes, they will want to buy us... they have already bought everything in sight, so they want no competition.  But without a second buyer at the highschool level with ANY real penetration, we will not have a good counter bid.  (I challenge you to find the second player with more than 10% penetration, or even 5%!  Just show me ONE.  Notice, HUDL seems to have huge penetration...  e.g. above 80% in at least some states)
	- NETWORK EFFECTS -- All users of HUDL that I have spoken to have mentioned that they have a HUDL package so getting a lower price on one sport will not help them since they already are paying for the whole package.
	- NETWORK EFFECTS -- All users of HUDL also noticed how much they use the sharing features of HUDL and it would be painful to have to work with a non-HUDL coach.


	GLIMMERS OF HOPE -- Here are things of value that HUDL does not do well... and at least one of them, it CANNOT do well w/o our AI:


	``HUDL cannot do real time analytics, and they don’t do analytics on my practice games, if your service could provide those that would be really valuable.’’

	``HUDL does not have long term storage for the videos.  So if a player needs a record of their play for later advancement, I have to tell them I don’t have it.  That is a real loss.” 

	DAN ASKS:  ``what if I could sell parent a $200 package for the season to see just their kids plays at each game’’
	``That sounds really promising.  Right now I always get advertisements for recruiting services that build marketing tapes for your child.
	Given the price of scholarships these days, that is worth real money to the parent’’



	OTHER STUFF I HAVE HEARD

	HUDL USAGE:
	-- Illinois -- ``I don’t know anyone in Illinois that does NOT use HUDL’’ 
	-- Main -- “I am guessing  80 - 85% of the schools in Maine are using HUDL for Basketball ”

	``We have a HUDL package that covers all of our sports, so if you offer Basketball we would need to buy it in addition to this.’’

	``HUDL tags each play with the player, location and shot type so I can filter to see all shots of a certain type by a certain player’’

	``You cannot drop in accuracy relative to HUDL.  At this point I trust HUDL’s scoring record more than I trust my own guys that keep records.’’

	``Your probably going to have to sell at the individual team level… but you should target an entire state or conference at once since the ability to share videos between coaches is key, and if I have to go outside the app to get that video to someone because I am the only one using Dan’s app, that won’t work.  Probably want to give this away for free for a period to get a whole group on board’’

	DAN ASKS:  ``So if I am ⅓ the price of HUDL, but I am 2% less accurate, is that going to be good enough?’’
	``Yeah, maybe for the high school level.  Obviously if you can get to 99.8% accurate that is much better, but maybe that would be ok’’

	DAN ASKS:  ``So what other features would be important to have… ones that would really make a difference’’
	``The more advanced the statistics the better.  Look at what they are doing at the NBA level now.’’
	(NOTE: this was Brian Godzisewski who told me he was a junior coach who does not regularly use HUDL at his level anymore.  So I am a bit skeptical of this answer… seems he doesn’t even use the stats he’s got!)



	REC LEAGUE TEAM MARKET -- The two sales reps I managed to talk to about this were both not bullish on this market... at least not as a stats app.  They claimed that few players would care enough about their stats in order to pay for them -- they did think the premium gyms would pay something for a extra fancy feature.  But they were not clear how much, and it would need dedicated HW.
	==> So if you think this assessment is wrong, then lets find some rec team captains to say it is wrong -- since they are the ones we are claiming will be paying at the gym.  (so far I have only focused on High School since that seemed the better market.)



	BUSINESS MODEL
	- I can tell you like the "fly by the seat of your pant" approach here.  But you are working on at least a back of the envelope financial model right?!?!
	- At a miniumum I would really encourage a financial model estimating the cost of customer acquisition (assuming this must be done one team at a time as I it seems it will need to be)
	- As well as a simple break even model for the business that estimates the number of customers required to hit this point.


	Cheers, 
	Dan









		scheduled another for this morning.)

	HUDL

	RISK AVERSION
	- Its still the same Dan, but I am taking this a larger commitment.  And having Analytics Fire means a larger opportunity cost too.
	


	- We agree having other players in the market is generally a good thing.  
		It proves the market.  and their $500M valuation also sizes the market.
	- Having a _single_ dominant player with a large penetration, and a decade old nation wide sales force is NOT ideal.  It is very hard to take that market from them, and fewer folks to completitively sell to (in that same market)
	- Not comparable to the siege situation much at all.
	- Still, I agree with you, likely HUDL cannot build this tech with their existing team, so likely they will buy it, or less likely build a new team to do it.  (so selling to them could happen, better if they had competition for highschools)
	- Is the Visio MVP a SUBSET of HUDL?
		- HUDL does offer video clips (see the 0:58 mark)
			  https://www.hudl.com/products/assist/basketball
		- BUT the stats are NOT broken out by player; 
			instead by lineup grouping (see 1:20 mark)
		- In my estimation (which could be WRONG :-)  Visio MVP's extra feature of tying the shot to a player is of modest advantage to the coach.
			- In reviewing a game with the team as a whole, looking at all key clips as broken out by turn over, etc. is actually ideal.
			- In one-on-one coaching, having a player-focused summary seems nice.
	- BIG ADVANTAGE -- for me the big (unproven) advantage is that Visio could be sold to each kid's parents, while hudl has more limited value in that context.  (see next "sell to dad" section)
		- I think HUDL could tie shots to players too, but likely would add something like a 50% additional labor cost -- increasing to $6 per game or about $180 per season using Phillippine labor.


	OTHER MODELS
	- SELL TO DAD -- Perhaps the most innovative approach is the "sell to dad" approach which focuses on the lower-accuracy-required task of
		"show me Johnny's highlights" 
		I like this one best since it is quite different in both technology and usage model to HUDL...  but if this is the one we are going for we should validate this market and what the requirements look like.
	- REC LEAGUE TEAM MARKET -- The two sales reps I managed to talk to about this were both not bullish on this... at least not as a stats app.  They claimed that few players would care enough about their stats in order to pay for them -- they did thing premium guys would pay something for a extra fancy feature.
		==> If you think that is wrong, then lets find some rec team captains to say it is wrong. 



	_
#### 2021-02-21 - Jason's Basketball Thing


	- With multiple cameras placed high it seems with work we can approach human performance at labelling shot

	- Likely will at least be sellable to HUDL

	- Working with CEO Jason is likely valuable

	- Most other CTO roles would have $100K salary, better team, <= equity, trust
		This situation is probably better

	- Reason to NOT do it, is so I can do my own thing starting in next year.


	-- Tech is tricky, but probably can be made to work almost as good as people.
	-- Market

	-- Consulting is looking like a fail.  Even if it works, its far from 300K/600K
	-- z
	-- Try the BB thing, then google


	- Accuracies need to be quite high (on shots, not assists)
	- Could ask coach to review tricky calls.
	- Shot tracker will be up our tail pipe

	- Not happy to have it be my only iron in the fire.



	Tradeoffs
	-- Value: $3.6M/yr schools * 3x for all other sources = $11M /year  * 4x = 43M 
	    43M * 15% = 6.5M to me...   Discounted by chance of failure 3x == 2M
	-- Could not ferris the situation at all, since I do not control it.
	-- Psychologically I dont own the idea
	~-~~




	Jason, 
	Sorry I am not back to you quicker on this.  I am really struggling to figure out how to make this work.  Here are some parts to give you a glimpse in to my thinking:

	It may not seem like it to you, but the recent year has shown me that I really am in a qualitatively different place than you are.  I have about 3x less cash in the bank than you do, and I am at the moment not able to move from a market that is probably like 2x more expensive than where you are.  I am not gripeing about all of this.  My financial life (and all of my life) is much better than most have it, I am grateful for this!  It just means that I am not "done" earning for the family in the way that you are.

	-- I can put skin in the game in the sense that I *am* in a position to operate w/o a salary for a year or so...  And even after this as a founder I don't expect to just jump to a normal salary, it really depends on our fund raising & income as to how that plays out.  Still I won't be able to keep pace with "zero-income" Syversen in this department.  And dropping cash in at the start is not a healthy move on the family front, and not logical thing to do given my family situation.  (Right now I can take out 80-90K per year out of my saving w/o draining it, and that is just not enough for us to live on in SF.)  The gap in our situations is perhaps the biggest reason to NOT do this.  It could setup a source of friction.

	Still there seem to be some real advantages one might work from here.  I could not tell from your email, but it seemed that your Vision PhD guy flaked on you?  In a certain way I almost hope so, since I think it can lead to a larger synergy.  You mentioned $400K for an MVP, maybe less.  Wow.  I feel that if one does the MVP and does NOT hire a PhD in AI, plus expensive US-based tech talent, that one can get to an MVP for a WHOLE lot less!  This could be a way I can really earn my keep.


	Another approach on this is to use some AnalyticsFire talent for the balance of the system, e.g. the mobile apps, the web interfaces, cloud infrastrucutre, the embedded code on device, etc. (which is well more than 90% of the code of the MVP), AND use AF+DanO for about 3/4ths of the AI vision work.   We have several guys who spent 2+ years delivering a very cutting edge vision-system system to Sunpower using satalight images to build very accurate models of house roofs from really really crappy images -- it was cutting edge work.  We had external vision PhD guys that we contracted with to up our game, but then we did most of the work in-house.  That same team is building a vision-based system for Enphase that automatically checks and validates visual scenes provided by contractors (can give you the details on these projects)  Anyway, there would be details, bandwidth, convincing Nick etc.  But it is a very different model of getting there.

	Just as I did for the Sunpower project, this would require hiring some $70-$100/hr vision specialists...  but really just dozens of their hours per month can really ensure that we are using all of the latest algorithms and approaches.

	Anyway, I am just thinking out loud here, and I really don't know if you already have the vison side covered...  I think it changes the picture some.

	I just have the sense that we should be able to get a prototype which can validate the core idea (maybe even w/ paying customers) in a very time/money compressed way... and that seems valuable for all involved.

	Tell me more about what is up with the current team...  so much of this does depend upon that.

	--dan




	I don't think it would be top-shelf to rely ONLY on this team for the PhD stuff, but my experiences is that you can get a really strong PhD specializing is just the kind of ML/Vision you are looking for in Europe for <$100/hr.  The you use them judiciously  and you can use them 

	-- 

	-- The is the biggest reason to not do this.  With AF my expected ROI is lower unless I create a new-co within AF (which of course is a great idea).  But the thing about AF is that it really does throw off cash, and with a bit more growth it is a lifestyle business.  so if I flush that away, and I say hey honey I will be contributing again above 80K/yr in a decade...  it is just not a good situation.

	-- This is the largest reason to aim for a business that I have some asic control over it, even if in the end I am not making the same level of impact.


	so I am just thinking thru 






	PRO
	- Simple Customer Validation; Customer saying YES
	- Now, technical, ML

	CONS
	- Control


	Suppose consulting co raises to $6M/yr @ 15% profit = $900K/yr   250+450 + 200
		=> Then I stay & do side stuff too

	Suppose we get bat swing w market signal
		=> Then I chase it, over-time for two years, likely longer.
		=> then start pulling salary and dropping operational activities.
		=> After 4 years I am able to slide towards lifestyle business.
	
	Suppose no consulting doubling & no batswing by end of 2022?
		=> Go work for google



	Two choices:
	- Basketball w. Jason, then after 4 years can transition to part time
	- Some Google ML or Robotics team



	FUTURES
	- My Bat Swing
	- Lifestyle consulting gig?
	- Google - as team lead or individual contributor





	Probablility of landing a Strong Customer Signal Batswing in 2 years?



	_

### _




_