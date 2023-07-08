
### 2023-07-07  Nick note (stressed)

Could not sleep…. Stressed.   You are right about me of course, no matter what I am going to think … it is what I do, and it would be sad (but also likely) that nothing will every really come of it.  And you are also right that now is a magic time for generalized AI ideas (starting a new-co using GPT, or somehow being a pundit around the un-align-ability apocalypse).  Presently things are going well at Sports Visio, we just got a $3M term sheet...  but I feel overwhelmed, and I do not feel safe at Sports Visio...  too much fucking around on the side could get me burned.

In my defense, I did NOT try to open this can of worms around EduCorp.  In general I agree I should try to learn from experts, that is what I was trying to do...  Now I wish I could put the worm back in the can for 6 months.  If I were stuck at Google, I would *love* to have an EduCorp thing to noodle on the side.  but right now, my noodling time is spoken for.





### 2023-07-07  Older
I love the drive-by constructive feedback that is possible on Medium.  Unvarnished and to the point, Thanks!!  I do have an ax to grind in the health & medicine space.  On nearly every topic I can find 'keto kills' or 'keto will save you', BOTH with arguments backed by an argument derived from at some level by legitimate science.  This misinformation KILLS people, so I am a bit of Dan Quixote for health claims.

Still if I am honest with myself, as you suspected, this predilection runs deeper than that.  I am always wanting to look at things the "wrong" way to see if I can see something that others don't.  (hmmm, if I dig deep, then post it as a reply to a reply PUBLICLY,  one part of my personal identity is thinking I am good at stepping outside the expected framings of things.)  Interesting.

But I wonder what you meant by "spill over"?  You could mean I should take care since folks at work or play could read my online posts, so be careful.  Or you could mean, if this is your personality online, then likely it is also you personality in life too.  I think that second interpretation has some truth.  In 'selling' calls or meetings I find it extremely difficulty to not provide the counterpoints to my own points.  my co-founder has worked with me on this.  Socially I have pretty good EQ so I usually avoid being a debbie downer when it would not be constructive, and I have a stronger-than-usual need to be liked, so when delivering edgy opinions (which I do) I am angling hard to not loose my audience.  In the work/teaching settings I think my fault is simply allowing things to become too complex (... and on the other hand, blah, blah, blah ... etc.)

Dan, ``you should just talk more'', said no one ever.

:-)


p.s. so who is the Brian Magnus guy?  I got nothing, not even a picture!



https://docs.google.com/presentation/d/1DrSX9Yp6Gth8V7JM7RY3AyMb05xefyTqXbDGme_WyBI/edit#slide=id.g11b0bc3c7fc_0_1337

Amanda, Jill, and Qingling, I am mercifully moving you guys to BCC for the designing part of the Yearbook, but will keep you in the loop as we actually start executing things.

  

Siobhan:

  

I register on a site called [slidego.com](http://slidego.com/)  they have some templates we might consider.  For a nominal fee, I can register there if we choose one of their templates just so we dont have to have a credit to slidego in our yearbook.  Here is one possible template.  It has fun kid colors, thought is it crazy saturated!  They have others:

  

[https://docs.google.com/presentation/d/19WZ8XHd2cdGCpa34FAct7TROEV4M_IqnFTBZeq7UpLc/edit#slide=id.ge8b16a7d74_1_0](https://docs.google.com/presentation/d/19WZ8XHd2cdGCpa34FAct7TROEV4M_IqnFTBZeq7UpLc/edit#slide=id.ge8b16a7d74_1_0)

  

My idea is we would strip most of this away, and just provide 

- the head portion where we had a few pics of the teachers, school, and classroom, 

- followed by the picture of my family as an example.

- at the end we would have a couple of pages of clip art (like the squares and diamonds from this template or others.)

   And some nifty fonts too.  (I am still hoping we can find some unique fonts, since the same old fonts are often seen)

  

Let me know your thoughts, and feel very free to just start editing on this starter doc:
  
  




# # OLD UNSORTED


Objective: Supporting subsequent knoweledge work
(Retrieval, Structuring, Concentrating)

- Global Naming + Local Structure
- External Storage/Formats/Viewer/Editors
- Multi-view
- 

NOTES:
open src/urlhandler.scpt                                                
oblinger@haorui urlhandler % python conf/package_app.py /Users/oblinger/ob/data/MyDesk/urlhandler.app   
Use finder to COPY to /Applications              
oblinger@haorui urlhandler % defaults write com.gvne.urlhandler Script /Users/oblinger/ob/bin/urlhandler/src/script.py
oblinger@haorui urlhandler % open gvne://asdfasdf  


- Copying .app to /Applications folder seems to activate it
- Info.plist indicates that it handles URL
- open XXX://url  -->
- Runs  ...app/Contents/Resources/Script/main.scpt -->
- Runs  ...app/Contents/Resources/Script/urlhandler.py --> 
- Looksup  com.gvne.urlhandler
- Runs   ~/bin/urlhandler/src/script.py -->
- writes to /tmp/...


	do shell script "python " & AppPath & "Contents/Resources/Scripts/urlhandler.py '" & this_URL & "'"


Specialty

Light Matter

Amy's organics
- head of autonomy - 10-12 per day (robots per day)
- head of platform - SW / cloud
- head of hardware - mfg supply chain
- experience shipping real 

having experience shipp

-- how do we architect system so it can roll out to hundreds of different
-- needed to ship devices
-- dependencies  

80/20 seriesA seriesB


~-~-~



def main():
	trans = get_transactions()
	trans = process_transactions(trans)
	result = double_pivot(trans, MONTH)
	# result = pivot(trans, row=MONTH, col=SUB_BKT, cols=SUB_BUCKETS)
	spending_path, transactions_path, today = F'{FOLDER}/transactions.csv', F'{FOLDER}/spending.csv', datetime.today().strftime('%Y-%m-%d')
	view_csv(trans, transactions_path)
	view_csv(result, spending_path)
	#save_excel({'Spending':result, 'Transactions': trans}, f'{FOLDER}/spending')
	os.system(F'cp "{spending_path}" "{BACKUPS_FOLDER}/spending-{today}.csv"')




DATE_CUTOFF = dt.datetime(2020, 2, 1)
FOLDER = "/Users/oblinger/ob/data/MyDesk"
TRANSACTIONS_FILE = F'{FOLDER}/transactions.csv'

TRANS_COLS = ['DATE', 'DESC1', 'DESC2', 'AMT', 'TYPE', 'CAT', 'ACCT', 'LABEL', 'NOTE', 'MONTH', 'QTR', 'YEAR', 'GROUP', 'BKT', 'SUB_BKT']
DATE, DESC1, DESC2, AMT, TYPE, CAT, ACCT, LABEL, NOTE, MONTH, QTR, YEAR, GROUP, BKT, SUB_BKT = TRANS_COLS
TRANSACTIONS_COL_RENAMES = {
	'Date':DATE, 'Description':DESC1, 'Original Description':DESC2, 'Amount':AMT,
	'Transaction Type':TYPE, 'Category':CAT, 'Account Name':ACCT, 'Labels':LABEL, 'Notes':NOTE}


BUCKETS = ['AmzMisc', 'DanO', 'Gr




https://news.yahoo.com/mayorkas-dhs-title-42-immigration-095900696.html
https://www.dhs.gov/news/2021/03/16/statement-homeland-security-secretary-alejandro-n-mayorkas-regarding-situation


https://thehill.com/homenews/administration/573875-mayorkas-defends-biden-policies-on-migrants-in-tense-white-house

9/24/
“We do not conduct ourselves in an unethical way. In fact, we are restoring people by reason of the immorality of the past administration. We are reuniting families that were separated,” he said.

Mayorkas has frequently defended the need for Title 42 on a public health basis, but gave a new caveat Friday.

“Over 600,000 Americans have died. More than 40 U.S. Customs and Border Protection personnel have died. Many migrants have gotten sick," he said. "We are doing this out of a public health need. It is not an immigration policy. It is not an immigration policy that we would embrace.”





Research/Technologist Executive

Leading a product team / or org's delivery 

IPC -- Innovtate product com
Rich Lyons

CPO
CIO
CTO 
Innovation officers


VC
PE
Universities
Lawyers 
Dan valientine. adam draper

- 

~~
Bruce, Sandy, Mike, and Michelle,

Thanks for taking the time for our Thursday call.  There were some exciting ideas that came out of it.  I am especially pleased that we had ideas coming from all different directions, that is the way to find the ones most pressing for action now.  Here are the ideas as we understood them, as well as a few thoughts about what the next steps might be on each idea for consideration in your subsequent team meeting.


~-~

- Contingency search firm
- top ten serach firms:  cornferry, 
- recruiting outbound adverts -- 

- loose the resume land the job.
- Connect to new folks in linked in.
	- Making a career move, wanted to be connected on linked in because of your specialization in XXX.
	- Quietly looking like 

~-~~-~~-~~-~~-~-~

red-bean, fava, kidney-like beans, peas

Tarik, it has been years since I have had occasion to write.  hope all is well.  I am locked down under covid, but have gotten married and have a kid now... so progress, I guess.  I expect you are full up with consulting work, or professorial work either way, and so not looking for fun side projects....  it just occurred for me to ask 
~-~

Frank Questions: 
- what happened, after the roof party I attended?
- how can I talk about PBG?   FEVO? 
- Did PBG sell any assets or clients or anything that I can reference?
~-~~
home away to expedia; key travel; airline volatea
Brian Bradford - fin tech deals
- rec sport league; camp site bookings

did do installments with groups

In 2019 still trying to close
- never got to the top of the list of the need
- involved high level decision

In 2019 
- FEVO is close to buying it; sold assets to FEVO
- 

Lesson: They skipped the middle market; needed to land some 50-150M business



~-~~-~
Clerky:

Email:
- look on corp website, will have email address listed with their provide
- guess the email and google for it:  "daniel oblinger" "email" "*@analyticsfire.com"
- Seamless.ai   



~-~~-~
⇒ Maintenance turnaround time improved by 23%
Created the metric used to drive this improvement
Created comprehensive dashboard allowing team to understand blockage



Mostly for my memory, here are good times for chatting:
3:30-5p PST == 7:30-9a PH
8-11p PST   == noon-3p PH


LR:
- The urgency was really because I am loosing access to James.
- but you guys being friends really simplifies this ALOT.
- Expect pretty small business related work for the moment.
- Maybe some sporadic home things.
- But I really don't know what is next, so it is hard to predict longer term.

Here is the key thing:
- JUDGEMENT:  
	- EFFORT -- Judgement from context that I provide to understand effective action.
		- e.g.  should you go the extra mile, or is Dan looking for 2-hour quick and dirty exploration.
	- APPROACH -- For the impt ones, then it is JUDGEMENT about how to approach task.
		- Is the information being captured going to be ACTIONABLE for dan?
		- Is their OTHER information that would make it actionable.
		- Is their information seen along the way that is key to capture.
	- CONCLUSIONS -- Then JUDGEMENT about the final conclusion.  
		- Which one(s) would you pick?  and why?
		- Of all the info captured which 2-3 points are the ones key ones for deciding?

~-~

FEVO
- I am hoping I can claim PBG was acquired.  But I don't want to be stretching the truth too much.
- Did we sell assets or customers or equity?

- Did any early investors like 500 get anything?  or were did the preferences push it all to the later investors?
- Was Covid the nail in the coffin?  Tilt selling to AirBnB was kind of a killer too

~-~

funding:  500 Global, Amino Capital, Bryan Bradford, Deb Lui, Great Oaks Venture Capital, Greg Kidd, Kima Ventures, Payment Ventures, Russ Fradin, SparkLabs Global Ventures, and Thomas Lehrman.

https://www.sffriendsschool.org/about-us/board-trustees



If it makes sense I can draft a letter that I would send, or perhaps you would


Frank,
Very good hearing from you.  It has been a couple of years since I have looked in on PayByGroup.  My current company is also in the process of being acquired / winding down so this had kept me quite busy!  

I would love to get the documentation to see what happened in the end with PayByGroup.  I have seen a couple of place online mention of its acquisition, but nothing very visible.  Was is just an acquistion of our customers, or did it include our software?  In any case perhaps the final cap table, and the minutes from our board meeting over time 

~-~~-~~-~-~



Happy New Year Nick!
I hope the week reset was restful....

I have two emails to send out... and wanted to get your opinion.  The second one is worth wordsmithing, but the first is not.  I just wanted to get your take on if/how I approach Dave.

I just got note from Frank Langston that requesting that I sign for the dissolution of PayByGroup.  I cannot tell what happened with the assets from the company. So I planned to ask Daves support in this.  I WISH I had gotten our gift into his hands before asking this request, but now it is quite time sensitive.


Dave:
Happy New Year; I hope you were able to escape for a bit during the break.  Thanks so much for the chocolate goodness that you sent my way!  Dandelion is a favorite of both myself and my wife so we are savoring it slowly.


On the first of this year, I received a notification from my former co-Founder indicating the dissolution of PayByGroup (the company I co-founded before my time with Nick).  I am hoping to get some "Dan-only" support regarding how I navigate my response to this.  My complexity here is that I do not fully trust these co-founders--very different than my situation with Nick, and I have no visibility into exactly what happened here.  It seems neither other founder went with the assets from PayByGroup, but I do find just a bit of info online that suggests there might have been some kind of acquisition.  So I hope to use my response to gain this clairity.  I think this is a pretty simple case, but I am not even certain which artefacts are appropriate/important for me to request.

Let me know your thoughts and if you are able to provide support on this,

Thanks,
Dan


~-~~-~~
I got too close to Christmas for it to make sense to send any out reach emails before the break.
But I have places lined up now to send to.  Here is a first draft of a note I could send to Cowboy Ventures.
Amanda is a principle that lists AI as one of her area.  Please give your thoughts on this, this one is worth a bit of polish or thinking, as I plan to use it as a template for others.

Thanks!
--dan


Amanda:
I am completing the sale of my company (Analytics Fire) leaving me with a window to consider other paths before getting pulled into the acquiring company.  I see that Cowboy Ventures invests in a companies like Aviso, Vic.AI and Lightstep where AI is central to their mission and most other investments like Crunchbase, Uno Health, and Lending Home where it is a key enabler.


I am reaching out as the principle focused on AI to see if my technical leadership over hundreds of million in spend within AI/ML might be crucial in this moment for one or more of your affiliated companies.  As a quick thumbnail, I have:
- overseen commercialization of software for over $3B+ in assets.
- invented or overseen 21 AI technologies across ML, Robotics, NLP, etc.
- hired and built multiple organizations upto 60 PhD and engineers in size.
- performed as ML researcher with 20+ patents and 25+ peer reviewed papers
For greater depth, please see my attached resume and addendum.

I am looking for a technology centered leadership role that matches my skills and experience.  Please let me know how I might best make myself available for Cowboy Venture's benefit.

Best Regards,
Dan Oblinger, PhD.
Co-Founder/CTO Analytics Fire








~-~-~


Your attempts a brevity are pretty humorous... I guess you have lot to say.  you know much more than I do about the USSR, so I have nothing to add in that area.

I have to start by calling bull s**t on one idea in your repsonse.  you state that there is "not one area" where the USSR was not trumped by the west.  No need to debate this one, but to my ears it really shows your bias in this discussion.  both nations were trying to get to the moon, that was not a win.  both were definitely driving towards minaturized electronics, but they were comparable.  No value in debating this point, just a surprising perspective from you.

We agree about those wanting to move from communism to capitalism, wanting some kind of affluence vs. guarantees for the basics.  I think we share a general skepticism against consumerism.  This does not look to be an auspicious path to happiness.  Still I suspect we diverge on our sense of life under communism.  For me the drab colors of the east are a kind of proxy for the sameness of life under communism... you get your alloted stuff no more no less, not different.  but as I say, I am not in an authoritative position here... it is just a sense that I have.

If you have any reference to polls from CATO or anywhere, I would love to see them.  They can be a way for me to really know the truth of a thing I am far away from!

you wrote:
all marxist states had exchanges....

Huh, again I have little knowlege here.  I do think owner/worker toxicitity is a real problem, and indeed I have  thought alot about the ills of having society driven by capital.  I feel one can perhaps create a market like mechansim that is not driven by capital...  but I don't love the monolith of a union.  This is a complex idea... too much to fit into a comment to a comment.  let me just leave it at an agreement that worker/owner toxicity is a thing to avoid.

cheers





QL:

Our finances are a bit messy in keeping track of who has what.
We should find the time to clean it up.

I was thinking about your 401K...  that is the messiest one.  somehow we should estimate what you would have had if you did not contribute anything to the account, and how much google money we would have if you started at zero.  Then average these two calculations?  I dunno, I am ok with most any equations we should just pick one.  

I think most everything else can be done without an equation:

-- You own the house in shanghai, and if we get rent money or sale price all of that should be kept in a separte account.
-- You own your 401K and any other moneys before marriage.

-- I own all my vanguard monies since it is from before the marriage.
-- My schwab account has a mixture of cash from before the marriage and AF salary during the marriage, so I am just treating this as all community cash.
-- I own analytics fire since it is from before the marriage.
- Selling AF I get 1/3 of the shares and given by R-zero.
-- My plan is to

~-~-~

Most recently I have focused on productization and commercialization.
I am perhaps most proud of my earlier stage commercialization work
Earlier stage commercialization includes 
Late stage commercialization includes

Before that I spent six years as a program manager at DARPA winning and leading 

~-~-~

Notable examples include leading a 40 PhD team's development of a dexterous home robot capable of complex, open-end 




~-~-~
There are two different elements that both provide a kind of sorting behavior, but they do not derive from a common element.  [orthogonality failure]

When describing part PPP of element E1 Alice could say, ``it is kind of like QQQ of E2 except …’’  Yet PPP & QQQ are not derived from a common source

$200K = $12K (after taxes)  Joint gets $12K/mo now.
$400K = $200K + $200K	= (22K)	10K/mo 	= 2.6M
$500K = $200K + $300K	= (27K)	15K/mo	= 4.0M
$600K = $200K + $400K	= (32K)	20K/mo	= 5.2M


MISSION:  What do I want my life to be about?
- ROOM - To explore technically hard/interesting ideas & live life w family.
- LAUNCH - Launch one or more new ideas that benefit/change the world.


~-~-~



50% - Marco

IDP
RPA


National

DAK - asset group - they are distressed asset purchase - they have equity-in-separate-co in Ampliforce
Title Search - $2.50/report ~   $3M

Jason,
I did a bit of praying about Caleb, and then a bit of thinking about the situation, and it lead to a thought:

You don't really care about the money, either the rent money, nor the college tuition money.  You care that he learns that he needs to have a plan in life, and you care that he obtains the skills to execute that plan.

IDEA:  You offer him a place to live with just some basic rules, and requirement to pay rent.  BUT 100% of the rent money is set aside (in your account) for purposes of his education.

- Now if he has *any* sense at all, he is going to prefer paying rent that can become tuition, rather than paying rent elsewhere.  If he spends the next two years screwing around before getting serious about a career, it will not be the end of the world... not great, but not a wasted life.  There are other paths that do lead to wasted lives.  So having him in your house is net-net a good thing.

- I would set the rent at a solid price, given his nice accomodations.  Partly because hopefully he values saving for college, AND you want to avoid him feeling comefortable with a very modest salary.  


TWEAKS
- SEPARATE ACCOUNT
	- MOTIVATION - It is hard to set the price optimally.  When setting it at a solid for the quality of the place, you probably need to leave room to rebate if his salary is too low, else he cannot pay the bill or would just rather live in a cheaper place.  BUT your place is nice, so if he can comefortably afford it, and have spending money left over.  then:  ``Hey, lets just do this for another 5 years before we grow up.''
	- FORMULA - His rent is R dollars a month, if he makes at least MIN dollars, below that 75% of his salary goes to rent and 25% to him.  He must make at least FLOOR dollars, in order to stay at your place.  (there are other formulas, I just created one)
	- ACCOUNT - You create a saving account IN YOUR NAME with a $1000 float added to the account.  His pay is direct deposited into the account, and you pay him from the float the same day his paycheck comes in, or even just setup an auto pay if his pay is regular.

- CONSEQUENCES -- If he looses his job or goes below 'floor' I would have a well thought out process pre-agreed to.  For example, you have 2 weeks to show documentation for the new job, else you go into lock down.  Initially that is loss of car (he can use your Uber to get to any job interview).  later it would be loss of TV, then phone limited to 60 min per day.  (I think at this point he will choose to live on the street rather than stay!)
	Whatever the consequences are, just pick them to push him to get a job, w/o any complex negotiation from you, AND without any natural ability to cheat.

- MATCHING FUNDS -- without telling him the exact matching amount, you can tell him, the first college monies will be 100% his saved rent money.  But without giving him any exact formula now, you can tell him that you will start matching dollars at some rate if he is succeeding in college.  I think psychologically even if you are matching at 4 to 1 in the end, if some of his dollar are being spent at college I think it is a good thing, both becuase he "earned" it, and becuase he remembered the cost of getting those dollars.  By leaving the formula completely unspecified, you can just play it by ear in order to have whatever money that exists, be the 'right amount'.

- SIX MONTH HARD LIFE -- I like everything above, with one exception.  He can still do everything as above and still not REALLY understand just how much his life with suck eggs if he does not get a plan.  He has a girlfriend, nice place to live, video games, moderately annoying easy job, less money than he might want.  BUT all of this could seem like an "ok" spot "for now" -- where "for now" stretches into most of his 20s.  So, one other idea is to tell him he can come back after 6 months, and start saving for college or a new busness or whatever.  But because of the 'lie' he need to figure it out for 6 months of NOT living on anyones couch.  The months don't count unless he is living in his own place.  This will give him an up-close-and-personal look at the rest of his life if he does not get his shit together.  I would minimize the stuff you let him take from the house (if the video game is his, then he gets to take it.)  We want him looking forward to getting back into your house and on track for some kind of real life.

- SEED MONEY FOR BUSINESS -- I am not sure I would mention this option to him in the beginning.  It is very ripe for abuse, or just poor decision making.  But maybe he builds a business and this is seed money instead of education.  I am doubtful that he has the gumshun and orgnaization to do it.  But maybe the idea should be floated in some way?  I am not sure...  I think I would not mess with it, unless you got the sense that this would excite him.


OVERALL, I like this broad idea because it causes his path of least resistance to become a slower, but still reasonable path towards his future.  He is saving money for college / business and at the same time he is hanging out at home w/o a real plan for some period of time too.  AND there is zero negotiation and zero room for him to game the system (assuming his pay goes into a separated account.)


Yeah, if I were more eloquent, this proposal would have been half the length...

best luck,
--dan












~-~~





~-~
- moths, 
- mealy bugs - alchol soap mix.  tried neem oil mix
- silver fish


JCB35N-110

~-~
Ah, now I understand!  Yes ETH is way ahead, yet many a slip a twix the cup and the lip.

My guess is that for financial transactions we will eventually develop a scaled L1.5 solution (which could still piggy back on second L1 like BTC in order to avoid 51% attacks).  And I think projects like SOL also have a future, since for non-financial blockchains one will need collosal scale that will never be consistent with 


~-~

833-922-0844
- Hub;   14-piece (8contact, 2motion)
- Video Doorbell Pro 2   ($700 everyday pro bundle)
- (2) Floodlight Cam Wired Pro 2000LM
- spotlight cam??  300LM
- Ring Protect Plus. $100/yr
- 
- Works with a schlag lock?  (can be used.)
- Battery life of sensors. contact (3 years). motion (1 year, 180deg, )
- one sensor for bay windows? 
- Siren.  104Db
- rebate on insurance.  
- can add more


IDP
RPA
rafferty holdings 
BPO 


Joe,
I kept thinking about this job assignment optimization task.  Its just fun to think about, even though this is definitely not my specialty.  Please humor me to consider if there is any value in this idea.  Here are the core considerations I tried to account for:

- There is wide variance on how long a task might take, so it could be blocking in one future but not in another future, and just taking the average is not so good since the average might be ok, while even one standard deviation above it could result in a disasterous blockage.
- Over time demand for different task types is "lumpy" so the bottlenecked task type will shift
- Fortunately we often can see the lumps coming in the details of the upcomming tasks already in the queue.
	(I think this is true, correct?)
- Not all tasks of the same type are of equal importance.  For example one road edge finding task might have 3 stop sign tasks depending upon it.  Then another one day LATER in the queue there might be another edge finding task with 20 stop sign tasks depending upon it.  If we might run out of tasks for workers that can do stop signs, it could be better to do the task later in the queue first. 

APPROACH OVERVIEW

- DISPATCHING PLAN -- A "plan" is simple an ordered list of tasks to be completed first.

- DISPATCH ALGORITHM -- When executing a dispatch plan each time a worker becomes available, they are assigned the earliest task in the plan list which they can work on, that has yet to be assigned to any worker, and is not currently blocked by some other non-yet-done task.  If none exist in the plan, then some fixed default assignment algorithm is used as a backup.  (this backup should be rarely used.)


- TASK COMPLETION MODEL -- 
	
- SIMULATION -- Given a list of workers and a prioritized set of outstanding tasks one can simulate time forward by simulating the dispatch algorithm and simulate randomized task completion times.

- PLAN SCORING -- Each plan can be stocastically scored by averaging n simulations using the current workers and current task queue as a starting point.

- BATCHED OPTOMIZATION -- The overall approach is to compute a best plan, and use that plan until most tasks in the plan have been assigned, then compute a new plan for exeuction in the next batch.

- GENETIC ALGORITHM -- In order to arrive at a best plan we use a genetic algorithm to combine and find a high scoring plan.


DETAILS

PLAN -- A plan is a simple list of tasks to be completed.

WORLD STATE -- A list of tasks to be completed along with priorities, and dependencies.  A set of available workers along with the task they are currently working on, when they began working on that task, and possibly a randomly chosen completion time for that task.

STOCASTIC TASK PICKER -- Is used to create a new dispatch plan idea "from nothing."
When a worker becomes free the task picker is used to select a task for them.
The earliest task in the plan that they worker can do is assigned to that worker.
If no plan task can be done, then the default picking algorithm is used to assign a task.

DISPATCHING TASK PICKER -- Is used to simulate or actually really use an existing plan to choose what to do.


SIMULATION -- A simple simulation can be done by repeatedly calling the task picker to assign tasks.  One can use a normal distribution for task completion times in order to simulate the duration of each task that is assigned.  As an advanced feature one could also model the inclusion of new tasks over time, but the simple model would just simulated the existing task queue far enough out that one actually simulated into the bottlenecks latent in the existing task queue.
	SIMULATE(WOLRD_STATE, A_TASK_PICKER, TASK_COMPLETIONS_MODEL) --> List of tasks with simulated completion times

SCORING -- Any function of the completed tasks could be used as a scoring metric.  If seems valuable to combine both the square miles completed as well as the priority levels of the tasks completed.  but any function of the completed tasks could serve as a score.

PLAN CREATION -- One can create a stocastic version of the task picker as a propsoal builder.  It could use historic bottlenecking rates (e.g. how often was this task type required in order to unblock another task that could have been done by an idle worker).  But many other heuristics could be considered as well.

PLAN MIXING -- Given two high scoring plans one can combine them, by treating each as a set of indexing numbers and task numbers.  e.g.  {(1, t1), (2, t2), ...} then a random pair is chosen alternatively from each parent.  Any time a task is chosen from one parent that same task is removed from consideration from the other parent.  The new plan is ordered according to the mixed index numbers, any time the same index number occurs twice the two pairs are randomly ordered.

REBATCHING -- We want to recompute a new batch frequently, long before we have exhausted the current plan.  this allows the plans to account for new tasks being added into the system, and also ensures that future bottlenecks are accounted for in the scoring of the tasks being chosen.

GENETIC ALGORITHM -- The system could perform both plan creation and plan mixing as a means of generating new plans for consideration, and simply pick the best scoring plan after some fix number of plans had been considered.

~-~~

Of course you guys can better pick the details  than I can, I just wondered if simulating the picking process over time might be a good way to take into account the emergent bottlenecks that are latent in the current task backlog.  Because the simulation is operating over the actual tasks in the backlog, and actual list of workers, it can "see" where one task has more dependents that might cause future blockage, while another with fewer dependents or that is saturated with more compatible workers might be less important to prioritize.  It can also "see" that working on certain tasks now,  in many cases will avoid idle workers much later in the task queue.

Second idea was to use randomness to compute over long running timelines involving thousands or tens of thousands of tasks.  My thinking is if a task is involved in a bottleneck it will cause that bottleneck in many simulations, thus even very light sampling will show a difference in performance.


~-~~-~~

MODES OF OPERATION

MODE				PICKER						UPDATE								PURPOSE
PLAN CREATION		STOCASTIC TASK PICKER		Simulated completion times			Creates a new randomized plan
PLAN SCORING		DISPATCHING TASK PICKER		Simulated completion times			Computes a score for existing plan
LIVE DISPATCHING	DISPATCHING TASK PICKER		Live updates from the the workers	Run a plan live in real world

~-~~-~




PREREQUISITES
- TASK COMPLETION MODEL -- Use existing data to compute a pdf over completion times for a given task according to its type.  (or estimate normal distribution, etc.; might be parameterized by important aspect of the task)
- STOCASTIC TASK PICKER -- Use the existing determinstic task picker to select the best task for a given situation, then remove that task from consideration and re-run to obtain the second best task, and so on.  The stocastically select tasks with heavy weighting towards the earlier chosen tasks.


TOPLEVEL ALGORITHM (input: WORLD STATE)

	Repeat forever
		if current_plan is too old:
			current_plan = PLAN_AHEAD_ALGORITHM(WORLD STATE)

		when a worker becomes free
			Use DISPATCH ALGORITHM, WORLD_STATE, and CURRENT_PLAN to select next task

		update WORLD_STATE based on what is actually happening in the world



PLAN AHEAD ALGORITHM

- GENERATE N STARTING PLANS -- Use the SIMULATOR with the input WORLD STATE, and the STOCASTIC TASK PICKER to randomly play time forward (updating simulated WORLD STATE as you go) and randomly select a plan task to consider.  Each simulation run will output one new DISPATCH PLAN.

- GENETIC ALGORITHM -- Runs for a specified number of steps, or until plans are not getting better scores.
	- SCORE PLANS -- Run each plan 'n' times randomly and compute average of resulting plan scores
	- MIX PLANS -- Randomly select plans to mix (weighting towards the higest scoring plans) in order to create new plans
	
-  Return best plan found






just do random simulations pretty far into the future, and 

with fewer 

And I wondered if using genetic combination might be a good way to combing the "good ideas" implicitly contained in each randomly selected





plan LENGTH -- 


Both square mile completed and number of high priority tasks completed both seem 



If any task in the active plan can be done 

- generate starting plans using a bias random process
- plan MIXING -- Two plans can be combined using a genetic algorithm style mixing to obtain new plans for consideration.

- assuming each task completes in a random amount




Avail
Baji Gobburi:
Availabilty:
Cost: 
Times in/out:  8-5:30   $2750/month.  
   4teacher-to-25students.   two outside teachers can add in.
   right now 20students.   max is 25students (3 classrooms)
Kian:
Montessori:
Potty Train:
Meals: types food, times
Activities:
Travel:
bigcityschool@gmail.com
info video/pics travel - email+brightwheel+weeklyreview

Long term teachers.  50% hispanic.  ~20% lots of asian too
2hrs 1 week spanish teacher
1x week art enrichment


~-~-~
### SHANE
- New Monitoring-Testing Methodology based on dynmaic injection
- As prototyped on HSM module

- Test injection w/o affecting performance of target.
- Testing is dynamically manage
	- Without increasing code size.  
		(likely decrease code size since )
	- One version of firmware, since
	- Test inhjecton w/o recompilation.
	- Hooks for centralized test infrastucture control

- Advise 
	- and on how your existing cloud infrastructure can be used to driven centralized control
	- how this capability can tie with Jenkins or other testing frameworks


- After analysis we see we can this provide this capability down to code/memory sizes
	(16-32K ram.  1-2M of Flash.  separated data/project on flash and ram.)

- connect to internet thru LTE network

- Use Jtag when possible

Jfrog; lauterbox

secure boot loader


Support for Software development velocity
software practice and integration effort for building new modules

test side:
- python/Pytest, C-testing, Rust


Leave behind
- code injection stuff
- managing large fleets of embedded devices in very high security context
- test side

_

~-~~
8.333.  8y 4m
3.5.    3y 6m
 

~-~~

Recruited from grad school to apply ML and analytics to K-12 developmental reading comprehension—collaboratively developing 6 products and securing 12 patent filings / publications.

~-~~


Statistics population dynamics (size & how it circulates)

Nobody has a built environment simulation-based 

graph-meta-data layer (rooms)
co-worker and pixelated model picture model.  geo-temoporal-data
occupancy sensors


~-~-~


Mary, 

Nick and I had a couple of questions about how these agreements work.  Let me see if I can expresse them succinctly:

(1) Assuming both Nick and I go to R-zero, and we stay some number of years such that at least some of our EMPLOYMENT options vest we decided to leave.  What happens to those shares?  
-- I know we cannot transfer them, but do we get to keep them?
-- The reason I ask is that I also understand if we are still working at R-zero when they are acquired, then there is some constraint that we need to go to the acquiring company.  Is that right?  How long are we locked in at that point?

(2) Assuming that Nick goes to R-zero and I do not go, we will both get shares associated with the ACQUISITON of AF (not for employment).  Then if later the company gets acquired we assume that all non-clawed back shares will fully vest at that time and will then be converted into cash + shares in the acquiring company.  At that point those shares would no longer have any encumberance on them for either nick or dan.  Is this right?

(3) In the case of an IPO is it the same as an acquisition in that we gain full control of our AF related shares after a short post-IPO lockup?

(4) Is my back of the napkin thinking correct here about the fraction of the company owned:
-- Assuming we have a strong IPO or acquisition this should clear any preferential thresholds that investors on their minimum return.  In that case their shares and common share should be valued equivelantly at IPO/acquisition.
-- Assuming the company has a series B post evaulation of $220M with a strike price of $13.21 this implies a total number of outstanding shares of 16,654,045 = $220M / $13.21.
-- Thus nick's 90K shares is worth .5% of the company = 90,000 / 16,654,045 shares.

Did I reason about this correctly?  We are not worried about getting this precisely right, we are just wondering if there are reasons why this reasoning might be very wrong.






 or into publically traded shares.  Either way, after a modest IPO lockup 
our shares will full vest at that time



~-~~
it is the latest in a line of global science agencies now being modelled on the renowned US Defense Advanced Research Projects Agency (DARPA), whose work a generation ago laid the foundation for the modern Internet.

Quote said laid foundations of internet
https://www.nature.com/articles/d41586-021-01878-z

- Deep tech inventor???
- XYZ interplay between science fiction, emerging reality, and commercial use.
- Call UIUC to get PhD date
- Top secret SCI 2005-2011.




Rudy in victorian house.
~-~
Sadly you *do* likely know the field of KM as well as I do!  

And I took your comments with appropriate salt.  And mostly I took your comments with the appropriate stance.
Your goals is the help me as best you can, by “shooting from the hip” with your opinions and not pulling the punches with your assertions.  This *IS* the best way to provide help…. Your time is very precious in during these days, so taking the time and mental energy to think about this is quite a gift.  So all good.


When I zoom out and think about the three years post Aeolus which is when most of all of the focused KM work occurred I come to a couple of observations:

—1—  my efforts were noticeably diluted by attempts to escape the “quake zone” and with efforts to think about a range of things beyond KM.  So my results are probably divided by three from that, for better or worse.

—2— my efforts would have been improve (will be improved) by involving others in my process, and not letting myself off the hook for gaining various forms of validation regarding the efforts.  (I am not against this, but I have only begun the first bits of this once the heat of AF ending began in earnest.)

—3— my efforts will only have great effect, once they are occurring within the context of others trying to do similar things.  In my wildest dreams that would be thru dog-fooding some of my ideas with just a few others who are similarly inclined.  But short of this, even being within an expert community of folks doing DIFFERENT things, but in related directions would greatly improve the chances that I do something of import.  (I have alot of improving to do on this front.)

Obviously if I worked on AI safety this would be a different story.  I could more easily be within a recognized area of exploration and I could more easily be recognized as an expert in this area.  (Btw when I brought up this point with Nina, she dismissed it out of hand… she is like… yeah, who cares what my thesis was, that was 20 years ago, we are all reinventing ourselves.  True that.  But it is also true that I have multiple things since then which add further weight to my credentials in that area…. There is really just no comparison on this dimension.)


—4— I do understand that I will make the most money by solving the problems that arise within the context of big money operations (e.g. Rzero or FAANG). But I think those operational roles will not drive my passion (almost by construction) since they will be about filling in the details on some “obvious” course of action, and they will be in service of making a rich co, even richer, generally at the expense of some other less rich co.  I am not against this on moral grounds, but I have difficulty mustering PASSION around it.
Also I think the place where I am differentiated is in an ability to operate notably beyond where others choose to or able to imagine.  I cannot prove that I do this activity well… There have been a number of cases (like bitcoin) where imagined what others did not see, then it came to be.  Still objectively speaking, I can only claim this as a passion, not as an area of excellence.  This is yet to be proven.

But it connects to something that I think is lost on you.  You seem to imagine that I can just do my "passion stuff” in any area I might choose.  But I dont think this is really correct.  I am ALWAYS thinking about how things could be transformatively different… and after a lifetime of doing this, I really only have two biggest ideas:

— how consciousness works (and I think others see what I see)
— how computer science could become cumulative in the way that mathematics is today (and I think almost no other human that I have identified seems have this idea.  Of course this means it is more likely to be wrong, but it is also more likely to be transformative if correct.)

So KM is a passion of mine, but the really deep passion in this area is this idea about how to make software as universally combinable just in the way that mathematics is combinable.  Notice a mathematician that proves a theorem written in French is automatically compatible with one written in German (even if it must be translated in order to be understood, it is basically impossible for it to some how not apply…. Theorems are just true in a universal way.  But computer science is not that way.  Software written in Java is not automatically compatible with python, in the worse case it must be rewritten from the ground up.  Uniform computation seems to be a very dan idea for better or worse.

Sure I can add value in nearly any place.  Teach a course, or manage a team to build a robot, but these are all very duplicative roles (many others could do it), and it is also a kind of drudgery work.  Once I see how it all is going to play out, the fun is gone, or at least diminished.  Second there is little ‘art’ to this work.  In most cases the correct answer is the quick and dirty ``get ‘er done’’ actions that slap some stuff together which is good enough, and move on to the next issues.  My passion projects have a different feel to them.  I will spend months thinking about how to crack them, and an incite will come to me.  Last night at 4am in 1/2 sleep a significant simplification in the way uniform packages are formulated… this is one of the nine element of unicorn… and I reduced a significant part of it, to a reformulation of the structure element.  Even after YEARS of understanding this element, I can still find deeper insights about it in that fuzzy border between wake and sleep.

There are no questions any normal “day job” has, which requires pushing ones mind to those levels of abstraction.  To do that, one must follow the problems where they lead, NOT where the market or your manager indicates.


—5— I failed.  I saw this reality ten years ago, and thus had the idea that I would become “post income”
I would create a startup to earn fuck you money, or a lifestyle business that would throw off enough money with little time spent.  At the same time I also worked occasionally on these passion ideas.

But honestly both were done in pretty haphazard way.  There are so many things I see now that could have improved my odds.  Mostly I should have just focused on real-estate or such.  But in any case, it is over, and I don’t have the context to try this again in 2021.

—6— Or I didn’t.  Right now, I have $3M in the bank that provides the equivalent of a $200K salary per year, when added to Qingling’s salary that is either $400K or $500K combined income.  One option is to just say, that IS enough, and I *am* retired now.  That will be something of a fight.  But for me it seems sad to optimize for earning more money, when I could instead optimize for impact.  And optimizing for working on dan problems.

—7— I agree, running towards a KM company type agenda is pretty risky… especially if I am trying to serve two masters.  The other master is this idea of making computing cumulative like mathematics.

—7a— Luis heard my situation and he was like, screw coming to Google, you have a shot to check out your KM idea and your team saver idea both… then go to google if they fail.  **BUT** he did not like the idea of spending 100K or a year getting a demo system together, he thought I could find a way to get market signal in some cheap and dirty way…. And that is the way I should proceed.  I do agree, if that were possible, it would be the best path…. And maybe it could be possible.  He was talking about hacking a one off solution for one company to see its value, or using slides to show how the system would operate, and then ask folks if it has value.

Maybe that could work.







On Nov 1, 2021, at 8:18 AM, Nick Allen <nick@nickballen.com> wrote:

Also, meant to type this: take this all with a pound of salt! I obviously don't understand KM or AI field to the level that you do. This is my reaction as a bystander after our long call the other weekend

On Sun, Oct 31, 2021 at 9:25 AM Nick Allen <nick@nickballen.com> wrote:
Dan

I wrote these notes after our call about KM. They' been sitting here in a draft. I think they match what I said verbally. Sending them in written form just in case it is helpful in any way.

(Also, good luck navigating all of this stuff!)




In a vacuum, I do think KM is conceptually interesting. In fact it's closely related to a lisp-based passion project of mine. 
I don't like KM as something for you to work on right now. The reasons are:
(1) You have put yourself into circumstances -- whether justifiable or not -- where you are giving yourself a hall pass to spend time on doing work based on assumptions that can't be validated/invalidated quickly (eg in under 1 month). This is bad on two fronts:
You can't iterate quickly to wayfind towards product/market fit
You can't "fail fast" (& then move on)
(2) There are big, important problems you could be working on (AI safety? self driving cars? global warming?). KM seems like a "grand problem" of the 1970s. Now it feels to me to be more of an "artisanal problem". You may be on to something hugely useful that that no one saw to make even though it was right in front of them (like the wheelbarrow). But if you are, you will need to finish building it for everyone to recognize it, and that is not looking to me like a satisfactorily fast moving enough process. Also, you may do all of this and end up with something that is better but only an incremental improvement over current solutions that are "good enough", and thus not make the splash you are looking for.
My unsolicited advice:
You want to spend your time working on the most interesting part of interesting, high-impact problems. You seem to have a bias that in order to do that you need to be in a small organization (startup) or no organization at all. However, I think that the best place to do that might in fact be inside of a large organization (eg DARPA, Open AI, FAANG...) where you can be highly specialized & other people will work on the parts of the problem you don't want to. In a small company (or by yourself) you need to wear many hats, do many types of jobs, and bring your own discipline to push to stay as focused on the cool stuff as best as you can. Also, big organizations also have the additional secondary benefits of being able to pay, having high social prestige, and comoin with structures to force you to work in a disciplined way.
==> If I were you, I would put the Open AI / DARPA / FAANGs of the world at the top of my lists of what to do next, and turn KM into a weekend project.
(2) You have a track record of GIANT achievements in AI research. Thus you have internal strengths in this area (expertise, inclination) and also external strengths (credibility, authority). It is also a field of "hotness" right now (problems intersect with societal good, marketability of skills, prestige). But you seem biased towards working on "products". However, you don't have near the same track record in this area. If you walk away from research/AI and fully into product, you are walking away from a lot of strengths and resources.
You have mentioned the pay cut of working at Open AI as a deal killer (making $160k year, eating & breathing interesting projects, & helping save the world from the existential threat of malignant AI). But you have also expressed openness to spending $100k (post tax) to pay devs to make a prototype of KM tool while toiling away at another job. This does not compute for me, especially since, after 2 years at an Open AI you will probably be in a good position to do side consulting at high rates. Whereas after 2 years of paying some devs to work on a KM prototype, you will only make a little part time progress, you will be tired and overworked, and if you don't hit a real home run most people will think you are crazy for doing it.
==> if I were you I would bias towards things strongly connected to my prior outsized successes in order to keep "connecting the dots"
If I were you I would probably do one of the following:

-- Go work at an AI safety research lab
Save the world. (If you truly believe we're all doomed to be destroyed by a malignant AI... don't you have a moral imperative to try to help stop it????)
Leverage your rarified background
Spend all day working on interesting problems. Follow your passions, but within a framework .
Take a temporary pay cut, but hone your AI expertise. MAybe do some ML side consulting starting on the 2nd or 3rd year
2. Go work at a FAANG
Make tons of $
Bounce around internally until you find the right job that you love. Take as long as I needed to decide what you want to do long term.
Once established, slack off and have interesting hobbies.
I would not do either of the following:
3. Working at home on problems of your own choosing like Ramanujan
Every day you spend in a silo of one, the less relevant you have been becoming to the job market. Whereas the more time you spent time inside of a DARPA / Aeolus / etc the more relevant you came.
4. KM startup / product. I am worried that you have too much tied up in the non-business aspects of KM (at least right now) that you will not be able to make decisions dispassionately or quickly enough to navigate the messiness of early stage startup existence. I'm worried if you go down this path you will spend too much time spinning your wheels and be frustrated by the types of choices you will have to make
If you MUST organize your life pursing KM projects, I propose that you double down on failing fast. REALLY FAST. Be disciplined. Be prepared to kill a lot of your darling ideas and make a lot of messy tradeoff decisions to quickly find/achieve product/market fit. Give yourself an uncomfortably aggressive timeline. If it doesn't hit during that timeline, hit stop with prejudice, go to FAANG and come back to it as a weekend project or retirement project.


~-~

Evening Number #1:
- I already feel very at home with flipping the legs of the chair up and down.  it is pretty second nature to pull up to drop a leg, and to smack the touch sensor to bring it back up.  (I find pulling the leg up w/o the sensor to be a big of a pain)
- I want to be able to push on the leg to "tell" it to come up.  That seems natural.
- I like the foam foot pad... I work barefoot.

DAY #2
- did not use it much.  but usage is natural.  I do NOT miss that I cannot move my chair!?!

DAY #3
- I am hacking, and listening to some good trance music at volumes slightly in appropriate.
- I cannot bop around as I usually would .... the back of the chair is right there.  :-(
- .....  hmmm, I CAN bounce against the seat back quite satisfyingly on every third beat...

DAY #4 -- not used as much

DAY #5
- SAFETY #1: My two year old boy wanted to play around the chair... he entered the area with both legs down and started fooling with it.  I jumped in there before he managed to smack the right spot on the chair to lift a leg up and flip himself upside down across the foot rest.  (But I think he absolutely would have been shoved backwards over the foot rest, possibly landing on the back of his head on the hardwood floor on the other side of the foot rest.  Not sure how to fix this!)
- SAFETY #2:  I belatedly learned I can swivel the chair while sitting in it.  (very nice!)
	But when I rotate around to pull the window shades down, my center of gravity starts to move outside the base of the chair, and the whole chair begins to tip.  I don't even need to lean much for this effect to happen.  It happens slowly so I felt in control of the fall, and stopped it.  Still the chair is not stable after much more than a 70 degree turn


I noticed I was in flamingo pose for 1/2 hour before thinking to shift.... it was very natural.

Reaching back to lower the blinds was just a bit un-natural.  But still so far I have not been annoyed to not be able to move the base nearly as much as I expected to be annoyed.

For the moment I am keeping it in manual mode, but I am excited to try the automated mode... I just need the manual mode to be working at the same time (I think).

(Auto mode should have a left-right configuration for it.  I would drop the left side first as a lefty, but that is not an option right now.)

I have the idea that I want it to stay in flamingo mode (left side down) when I leave the desk for fastest re-entry... we will see if that is right.

~-~~

No the chair was a second skin after just minutes of using it.

On day 3 I am no using it in manual mode with the pressure sensor dropping the legs when I leave the chair.
This does naturally increase my standing time.

— I love the super squishy standing pad.  Very important for me since I am very flat footed, so standing is hard on me.  Still I get tired after < 1hr, so quick sitting is awesome.

— I have really optimized the sit - stand motion for my standing desk.  e.g. 
     (1) while typing I can hit the memory reset for standing, and keep typing while it raises, 
     (2) the push the rolling chair forward under the desk, and pull the standing pad from its nook in arms reach.
    Even with all of this careful optimization, this chair is WAY WAY WAY smoother to make that transition.
    ==> On this basis alone I will buy this chair when it is available.

— On one occasion I did have the latch fail.
     - I pulled the right leg up and sat on it.
     - Then the left leg and gave just enough room for it to position itself.
     - I put my full 1/2 weight onto the left leg and after about 1 second it released!
     - my other butt cheek was still on the right leg, so nothing much happened, it was a bit jarring.

    Of course if that were to happen when I only had ONE leg under my ass, it would really be BAD.
    My sense is that in giving the left leg room, I just did not give it quite enough room so that latch was not
    Fully into place.

    ==> I wonder if the metal of the latch could be shaped with two points on each side of the latch so as weight is put on it, those points will slide in one direction firmly catch the latch, or slide the other direction with little force causing the latch to open….  This would eliminate the possibility that static friction could hold that latch in an ALMOST closed position.   (But hey I am a PhD in AI, *not* mech-E… so, you will know better than me!)


~-~~-~






~-~~
100watt 
.12d/kwh * .1kwh/h = .012d/h * 8 * 5 * 50 = 024.
~-~~
- Screen shot.  what is my title-line?
- Number of nations at aeolus.
- PhD after my name?


~-~~
2017 Q3 - Biggest source of new revenue for the company (eddie)
IT automation at multi-gigawatt scale

growth executive; pioneer


~-~~
Sources of Anxiety:
-- HIDING THE WORK:  Many folk have a overblown sense of self-importance.
					 I could be in that crew, and that is ok.  
					 But would like to establish I am not before broadcasting it, proudly
-- Cannot justify the work (yet).
-- Not fitting the expectation model for me:
   Secrecy, Silence, Judgement dare greatly
   
-- Emperor has no clothes:  Fake an ML-researcher, but am no longer.  
   Fake leader, but have not really been one in high functioning normal product management
   Really have NEVER be a high time-on-task kind of guy.  (I am a sprinter)
   Hate many forms of drudgery.  Identify many forms of management as drudgery.
-- Chance is going to be lost, and I will slog at work for the next decade
-- My big ideas have already been well considered and dismissed by academia
-- Things have not gone the best in marriage and me & my choices are partly to blame

-- worked long in isolation on multiple topics that never when anywhere



~-~~

Shame - 

Courage to be imperfect.
Connection as a result of authenticity.
Embraced Vulnerability.

let ourselves be seen.
practice gratitude & joy.
I am enough.


First reaction: he sent me a ted talk on shame and vulnerability... this is kind of a miss, as I don't feel these are my core problems... I have core problems but these aren't that close.

I do agree connection is "why we are here"  or at least it is a big one.

I feel I have a number of things I can be proud of... and listening to others over the years, makes me realize that every human is deeply F-ed up in one area or another.  This gives me a certain equinimity regarding my own person F-ups.  Both to my self, and also as expressed to others.  So it seems I am able to be vulnerable, as a generalization.

But there are aspects of the talk where I felt many more issues to dig into:
Courage to be imperfect (maybe).  Ability to have gratitude (definitely).

And Anxiety (definitely).

For me the huge anxiety over the last several years is:
-- the nearly secret agenda I have worked on will expend years and amount to nothing.
   or even more of a worry it will be cut off because 'real life' will intercede.
-- I know if I talk about the agenda (even with another scientist) it just sounds crazy.
   especially since I really only barely understand it myself.
-- So I have somewhat hidden the work, until I understand it well enough that I can cogently present it.
-- and I have dedicated so much mental energy toward this work, that I have allowed many other areas in life to languish.


But there are other sources of anxiety:
-- I worry that in the last decade I have done things that 'look good on paper' but are running is disparate directions, in a way that has put me out of touch with my core competencies.  ... a bit of a fraud on paper, but if I needed to back it up w/ action, I would struggle.
-- I worry that I dont have the stomach for 'real work' and will really HATE being chained again to a 9 to 5.
-- I worry that the things I am capable of will never be realized for lack of focused action and because a traditional career ate up the time needed.
-- I worry that the ideas I have are far less profound and valuable than I imagine.
but mostly I worry that this thing I sacrified for will never even get tested, it will die before it is born.


In the early hours when I wake, I can quickly begin ruminating in a way that keeps me awake.
I used to allow this, but it can eat up blocks of time.  Now I really limit night time thinking as much as I can.  (I will notice which specific things I worry about over the next days)

But it does not feel like shame... exactly.
there is an embarassment in working so hard on such an impractical agenda... maybe that is a shame.
but it is also such a worthy thing to do too... so it is odd to call it shame.


still the single-mindedness of that work does cause me to loose connections to others... to friends... to my wife... to gratitude for many other things.

I would say that GRATITUDE is an area where I have long struggled.
I have much to be grateful for, but as this speaker notes, being quick with the measuring stick causes us to focus on what is NOT right, rather than what is right.  THIS is clearly an area of concern for me.


very stream of consciousness reaction to the first video!
-dan















~-~~
Low road and High road compatibility:
- LOW ROAD COMPATIBILITY -- Two efforts can be mapped to a bridging language where they become compatible
	(but each continues to run in/on incompatible substrates)
- HIGH ROAD COMPATIBILITY -- All code is directly compatible

Requires
- Starting substrate capable of expressing all computation
- Requires limits on ways to extend substrate and ways to re-align any incompatibilities as they threaten to creep into the growing ediface

Requires
- only one 'right' way to encode parts that where one desired interoperability


Formal approaches
- most folks have tried for formal methods to acheive these goals.  could work, dunno.
- here we take a very 



~-~-~
Turmeric definitely has a bio-availabilty problem.  Still as show in the study below it really can be absorbed into the blood at meaningful levels when eaten in its natural form:

https://pubmed.ncbi.nlm.nih.gov/23378457/




But I agree this availability can be dramatically increased (20x) as discussed here:

https://nutritionfacts.org/video/boosting-the-bioavailability-of-curcumin/




So one could add Pepper for a 20x boost, or Meriva for a 29x boost.  And as you point out, with a suplement you are getting a known quantity, and ideally paired with the boosting agent.  Makes alot of sense!




NOTE:  My original take was quite the opposite.  I misread this article to indicate that dietary Turmeric does not work, but this was just my mis-read of the article.  Thanks for the post!


~-~~

Avraam:
Agreed, merely adding frictions usually does diminish an effect.  In this case I just worry we are too far from the root cause.  Consider the case of an old school business that needs a cash injection to modernize.  Investors project a 20% bump in productivity, easily doubling orders (and almost doubling staff too); they estimate payback in 6 months, and 2x profits over 3 years!  Hooray!  It's a win-win-win!

Oh wait!  The Avraam law says we must wait a decade to reap those profits.  So sad, since markets are so fickle, the investor passes; the returns are too slim and timeline is too long.  Notice, the profit taking here was not evil even when it was quick, because it was causally linked to an INCREASE in staff.

Striking nearer to the root of this problem I would say two things:
(1) we should not TAX a company that provides a job, we should PAY them for the service they are providing society.
(2) businesses serve the needs of capital since they are owned by capital.  If you don't like the former, you must remove the latter.  NOTE: I strongly believe in the distributed intelligence of the market, thus one must somehow retain capitalist-market-distributed-decision-making just without capital ownership.  (I have some half-baked ideas, but they dont fit here...)

anyway, I honor the spirit of your attempt, even as I kick sand into the gears of its workings. :-)





Professional association of res writers

National resume writers association

Career directors international

Jared Redick  the resume studio.com





xxxx:

I am taking the unusual step of writing to about your employment offer for Ben XXXX.  I am the CEO of Analytics Fire where Ben worked from  2017 thru 2020.  In the past weeks we have been working to staff a fast moving opportunity connected with a client that has greatly appreciated Ben's work for them.  Since this new work is not yet signed, nor is it public, we had not yet reach out to Ben regarding this opportunity.  This week we received a security check request for him, and immediately reached out to him to find out what was going on.

In negotiation with this client, we arranged for a generous counter offer which we presented to Ben on Friday October the 1st.  It is our understanding that he plans to accept our offer at the expense of your offer.  It is the uniqueness of Ben for this particular role that has prompted us to do this, and I beleive for him to accept the offer as well.

Of course, providing this back story in no way helps you fill your unmet need.  Still if I were in your shoes I would suspect I had been used by a candidate who was using my offer as a "backup plan" while waiting on a better offer.  That is not the case here, Ben had no knowledge of any possible offer from us prior Friday the 1st.

I hope we have not badly damaged your ability to execute in a timely manner.  If you would like any further information about this please reach out by email or phone.

Regards,

Nick Allen
CEO Analytics Fire







1.5 + 2.5 + 6.5 = 10.

Last First 		Age Parents 				Phone 				Email 						Address
Biketi Caleb 	3 	Amsalu Dabela-Biketi 	c (617) 272-5550 	amsaluandalbert@gmail.com 	6 Misty Harbor Ct
					Albert Biketi 			c (617) 417-0025 								Pacifica, CA 94044
Chung Augustine 3 	Michelle Morales 		h (520) 245-2682 	missmorales@gmail.com 		331 Font Blvd
					Marc Chung 				h (602) 690-6272 	mchung@gmail.com 			San Francisco, CA 94132
Corkery Collins 3½ 	Ashley Kemper Corkery 	c (831) 261-9277 	ashleykcorkery@gmail.com 	637 Darien Way
					KC Richard Corkery 		c (650) 804-6946 	kc.corkery@gmail.com 		San Francisco, CA 94127
DiGiacomo Luke	2½ 	Jennifer Sorrel DiGiacomo c (443) 622-4129 	dermins415@gmail.com 		56 Upland Ave
					Chris DiGiacomo 														Daly City, CA 94015
Du Avery 		3 	Jimmy Du 				c (408) 318-3135 	jim.k.du@gmail.com 			4426 25th St
					Vy Hoang 				c (425) 260-1927	hoangv22@gmail.com 			San Francisco, CA 94114
Escalante Mayreth 3 Wendy Lopez 			c (415) 994-5077 	wendymedal@yahoo.com 		301 Hanover St
					Manor Escalante 														San Francisco, CA 94112
Gupta Reynash 	3 	Vivek Gupta 			c (415) 961-3442 	vivekgupta84@gmail.com 		230 N Lake Merced Hills 1B
					Rashi Bargoti 			c (415) 799-6529 	rashi741@gmail.com 			San Francisco, CA 94132
Holloway Juniper 4 	David Holloway 			c (415) 717-2258 	davidh@slugworth.com 		17 Laussat St
					Rebecca Holloway 		c (707) 540-5584 	rebecca4th@gmail.com 		San Francisco, CA 94102
Ivancie Nolan 	3 	Becky Ivancie 			c (323) 717-2299 	becky.wolk@gmail.com 		10 Country Club Dr
					Mike Ivancie 			c (949) 735-5031	mike.ivancie@gmail.com 		San Francisco, CA 94132
Oblinger Eli 	2 	Daniel Oblinger 		c (415) 494-9499 	oblinger@gmail.com 			101 Bache St
					Qingling Ni Oblinger 	c (408) 480-0508	qingling.ni@gmail.com 		San Francisco, CA 94110
Patnaik Saisha 	2 	Pretti Rani Dabir 		c (856) 689-4733 	preetidabir@gmail.com 		85 Woodacre Dr
					Markondapatnaikuni 														San Francisco, CA 94132
						Samba Patnaik 		c (856) 689-3936 	sambapatnaik@gmail.com
Tang Ellie 		4½	Mabel Lee 				c (415) 516-4321 	leem817@gmail.com 			850 Pointe Pacific, Apt 2
					Jackie Tang 			c (626) 376-3611	jackytwc@hotmail.com 		Daly City, CA 94014
Taraskina Sophia 2½ Anastasia Dryha 		c (415) 889-3672 	anastasiia.dryha@gmail.com 	55 Inverness Dr
					Pavlo Taraskin 			c (618) 213-9514	pahask8@gmail.com 			San Francisco, CA 94132
Venkat Adira 	2½ 	Sundar Venkat 			c (650) 743-4689	sundar.venkat@gmail.com 	701 Faxon Ave
					Pavitra Eshwar 			c (415) 601-8342	pavitra.kth@gmail.com 		San Francisco, CA 94112
Wilson Muriel 	4½ 	Christine Garcia 		c (561) 512-2402	chrissy.n.garcia@gmail.com 	4239 Pacheco St
					Aaron Wilson								wood78@gmail.com 			San Francisco, CA 94116


EXCLUDERS
The performance of services in the fields of health, law, engineering, architecture, accounting, actuarial science, performing arts, consulting, athletics, financial services, brokerage services, or any trade or business where the principal asset of such trade or business is the reputation or skill of one or more of its employees.
- Not consulting
	- BASED ON A DEFINED PROCESS 
	- SERVICE (People ask what they want)  (we provide solution)
- Day2Day Assets / Total Assets > 80%
	- Other assets 2x avg balance
- After 2 year, no more than 50% of cash
	- AR


- Engineering (but SW Engineering does not count)
- Perf services in (consulting, )
- principle

PREP
++ 8oz salt&pepper rubbed one-inch-cubed Salmon
++ 1med peeled-one-inch-cubed Potato

HEAT PAN PAN
++ 
SAUTE












flail in the handoff
going to take too long, and too "big corp" style

got to code; liked us fast; understood their limitations; we were a good match

Gibson knife
dan + [scott & gibson]
adding Gibson into our team


O:
I thought a bit more about how I might fit this role and how this role might fit my situation/goals.
I understand that you are trying to manage a two-way street, and you need to identify blockers from either direction.  Here are some (slightly verbose) thoughts that might help to thread the needle here:


The **FIT** here is obvious and quite exciting.  I am a BUILDER of stuff, I thrive on the chaos and uncertainty that surrounds bringing novel tech into the marketplace.  It is what I did at IBM Research, at DARPA, at Martian Robotics, and at Aeolus.  It is what I was born to do, what I love to do and what I am good at.

I think the big issue is going to be **MARKET VISION**.  We don't have to have everything figure out on day one, but we do need to have identified a market segment and product type with strong evidence that a winning solution awaits us.  The freight-train of commidification has been visibly racing towards Neato's bottom line for the better part of the last DECADE, so while is it never too late when you have a multi-billion dollar company like Vorwerk backing you, one needs to have a dramtic shift in conviction if one is going to win in this space.  Landing a novel robotic-product in the consumer market is HARD!  VERY HARD!  Hundreds of companies have failed even after spending dozens or hundreds of millions each.  Thus, this needs to begin with a many engineer-year commitment from Vorwerk, AND some evidence pointing at a use case area that is destined to win.  In the end this will be an emotional decision and we will all have to really believe...

Assuming vision and conviction are in place, I am guessing that the issue of **COMPENSATION** can be worked out.  Vorwerk understands the salary rates that are needed for the bay area, for the right candidate I think this will not be an issue.  The RSUs will be trickier, but here is a trick that might just work:

Neato/Vorwerk provides a very strong RSU package, but (1) one that is tied entirely to new product milestones and eventaully to sales results, and (2) has a (pretty high) dollar cap on its lifetime payout.  I think this can be structured so that Neato is not paying out large RSU dollars until they have already gotten milestones that are clearly worth much more.  The reason this structure works for me is that it gives me a silicon-valley style upside, and increase the number of worlds in which this path out-performs me simply shooting for a directorship position at a FAANG company, which I think has much lower risks.  The reason why this might work for Neato is that at all levels of RSU-payout they have clearly gained much more in new product potential than they have paid in RSU outputs.  Then, in the case that we deliver a successful new consumer-robotic product category, then even a double digit million dollar cap on my RSUs will be nothing more than a happy rounding error for Neato's bottom line.  Structured correctly, it is a winner in all cases.

As I write all of this, I am excited by it!  Such an effort is JUST my kind of mission---you can tell by my years of already taking on just these kinds of missions.  I think it is all going to come down to Neato's assessment of me, and my assessment of their vision and the commitment.

Looking forward to hearing more!
--Dan



 






~-~~-~~

Scott,

As a business owner I understand the complexity in deciding how to plot a course forward that yields the highest velocity with the lowest risks, so no problem with your present choice regarding this work.  As you say, it would indeed be wonderful if in the future we can work together, so let's stay in touch.

Since we had already done some diligence on this project, I thought it would be valuable to capture that thinking in a form in a short document that might be useful to Dropwater in the mean time.  We were thinking of the work in two pieces, the first I called "operation coding velocity" listed below, and the second is "operation architecture."  I am hoping this information might have some value for your internal team's deliberation and execution.  It is difficult to balance making infrastructure improvments now verses waiting until greater funding in the future.  It is our assessment that your team will repay the costs of these investments in greater coding velocity in well under a years time, so we believe you should execute these changes now.

Best, 
Dan


OPERATION CODING VELOCITY   (we estimate XX calendar weeks and YY total dev weeks)

We belive there is low hanging fruit that will increase your development and debugging velocity to such an extent that it will pay for itself in well under a years worth of time, thus we believe these items should be addressed by your team immediately.  Indeed, had we been doing this work we would have made these changes first in order to significanly lower the costs for the second architecture operation listed below.  Taken as a whole we believe these changes would more than double your software development effiency and would also make your system more robust as it would uncover bugs that likely are latent and not expressing in the running system in ways that you can document and fix.  We believe it is important you execute these actions as soon as it practical.


FUNCTIONAL TESTING (XX engieer weeks)

A system as complex as Dropwater's backend should will greatly benefit from a couple of functional tests that exercise the major pathways throught its operation entirely in software simulation.  

MCU CANNED RESPONSE EMULATION (XX QA engineer weeks)

Writing accurate software simulations is expensive and hard (though often still worth it).  Still at this stage Dropwater will get great benefit at low cost from simple "can response" simulations.  Developers map out specific major pathways thru the execution of the dropwater system  (like authorize payment, or dispense water).  For each of these paths Engineers determine the specific commands that must be sent the MCUs (and expected timings in milliseconds) and the responses that will come back.

This "canned response" simulation is much cheaper to develop, and can have configuration variables that allow it to generate the incorrect responses for specific kinds of failures.  (like le)


ERROR REPORTING

Debugging is notably supported by having multiple levels, and kinds of error/status reporting.  Grouping errors and status into categories will allow developers to remove 10x of the noise while debugging.






OPERATION ARCHITECTURE

This second activity will be 






~-~~-~~

STELAR SLOW DOWN

Scott, 


- Following your plan:  
		Quick fix, well architected system
			ability to debug and extend
- 
- Gibson:  Gjengo, Celery, UX
		SW engineering, C++
~-~~




- DEMO SYSTEM WILL
	- Show the calling of existing functions and variables within the running application
	- Show results/effects of injection both on LEDs attached to the MCU as well as results shown 




~-~~

ND2-inc.  dev tools (SW & employee)

fennel, quinoa, 

ASSET vs STOCK -- Asset sale forces c-Corp double taxation.  We both lose alot to Uncle Sam

PROPORTIONATE vs CLIFF -- Much of our value to R-zero is front-loaded and organizational.  If many leave, the value of having a smaller gelled team, especially in the beginning is huge.  It sets the culture, and delivers value early on.  So the value provided is always above the clawback at all levels.

FRONT LOAD vs FLAT LOADING -- For R-zero the period of greatest concern is Q4 of 2021 and 2022.  This is the period at greatest risk of irrecoverable velocity loss.  Replacing churn over time is much easier.

80%-OVER-3-YEARS -- Is already extremely aggressive as a target, doubly aggressive when you consider that we do not start day one at 100%.  Instead we start day one at what ever number we manage to entice to transfer.



PLACEHOLDER=provider of persistence   ACCESS/ASSIGN=Place operators
++	PLACESPACE=Lexspace[Place]	PLACE = Placeholder   (with subvalue propagation assignment semantics)
	++ AT=1-to-1 vertex to place   PLACEMENT=Sets 'at'  VALUE = subgraph "at" place
	++ PATCH=OrderedMap[Place, Value]	DELTA=Computes patch  ADD=Patch application UPDATE=destructive add
++	TIMELINE=List[Value]



5:31 PST.  August 03rd.


~-~~
XXXX,
I am a founder at Analytics Fire, a boutique consulting company

~-~~
3PL -- intermodal.org 
- Givens Transportation (3rd party, chesapeak)
	- President of the company
	- switcher, 
	- director of logistics
		- 150K /year
	- dispatcher needs to radio.
	- VP of transportation

IBM trade lens. 

fleets of devices.  where extreme concern when 

Angel investing:  https://blog.usejournal.com/real-life-angel-investing-returns-2012-2016-b33425fcb816

~-~-~
build framework (needs work)
testing methodology for injections (w/ affecting performance)
rx-tri-core. full HSM functionality on them


Version two of their OS -- NV OS
-- Shane -- HSM functionality
-- holes
	reference implementation for test injection frame (on linux embedded device)
	TC 37x and TC 39x hardware
	Talk with Morrisio -- devops -- and product owner -- would be super excited 
-- 


~-~-~
27.7  Eli weight

REFI Order Number:  1715682
855-236-1499  Shanda


~-~-~
10K / 42 * 3 = $714
10K / 37 * 3 = $811
10K / 32 * 3 = $937
~-~-~
Architects have become developers; becomes
-- putting out fires; particularly around test;

talking points
vehicle based os

auto-star in around; linux is around; 
the home brew stuff needs help
turning on 
MCUs now; multi-processor SOCs are in the future

trying to push team towards injected tests
existing OSS stuff that they might adopt

~-~
we come in and propose OSS testing stuff that could be added 
-- money is not the issue; just want stuff to get done on time
-- harmonously together
-- they already have pretty good CI/CD system; w/ good build infr; 
-- don't have good hardware in the loop testing; 
   -- hooks and MCU (bcl bones; jtags)

-- qyan;
   morrisio on the devops side good guy to talk with.
   test guy sirini

automotive grade of linux; cell4 board
imx8 - S32G
infineon 
focus on virtualization; microvisors

~-~
- CI/CD directly onto hardware right
- abilty to run testing w/o changing production behavior (space & time)
- over jtag, eth, canbus, using scratch ram space 4k or 8k
- run one tiny test, then the next one later.

~-~

yaroslavvb@gmail.com

-- Intro: Math U; scruffy PhD in ML; 

- Skills of interest.
- Finding clients.
- Southpark commons
- 20K
- Tribe.  Drinks with Tribe.  
- Google AI.  levels FYI.  

~-~~

Evan Hubinger @ MIRI

-- There is a gap in how we are approaching AGI, but when we close that gap we will be alot closer than it feels right now.
-- We likely will loose dominance over these systems, so planning the least risky approach as early as possible seems best.

-- two idea:
	-- poc reserach agenda.  Bootstrapping hardest concepts in presence of teacher.
	-- website debate.


Aja Contra -- report on AGI timelines

FHI; Chai; 


~-~~



[cnn](http://cnn.com) [ob](file:///Users/oblinger/ob) 
[open](open:///Users/oblinger/ob/logs/phone/__Phone__.md) 
Allocation GIGA 0.14.  price $445.  6% mgt fee.  1% annual.  20% carry.
140M

Grace Chen MX finance
~-~~
marcel (at) metaobject.com
~-~
TODOS
-- deep learning reading
-- meetups; meetings
-- ML story ()
-- conversations
-- upwork
-- mgt training

-- info management
	one-on-ones

~-~
if x > sla * fooby:

if x > (sla or sys.maxint) * fooby:
	

Chop in equals sized chunks:
Hard Veggies: 
Leafy Veggies: Gpeppers, Kale, Cabbage
Saute: Onions, Celery, Carrots, 
VegBroth: Cover hard veggies

Italian seasoning, garlic, bay leaf

~-~~
- Go the manager route.  ic-manager 
- Compensations really start going up when you get to the product
	
	L7 or L8 where the money 
	L6 five-six years before L7 
	L6 IC -- $300K total comp ($170k base; deferred comp stock)
	L7 20-25% increase over L6 (his guess)
	L6 impact oversite and impact that spans multiple groups
	L7 spanning multiple products
		mostly about the scope

L6 Senior Mangaer
L7 senior manager
L6 Associated Scientist (IC role)
L7 Senior Associated Scientist (parallel to a manager)
-- experince innovating, understanding 
-- leadership principles, previous;corp value emphasis
good decicion; others trust you; delivers results; back bone;
learn and be curious
	


~-~~

-- explortative
- early leaving
- take the money and run
- hostile take over
- demotivated non-participation
- raise more capital (ohter org)
- too rigid and unable to deal with humans
- someone needs to take 1/2 year off

- 
- markets evolving (rice no longer )


https://upload.wikimedia.org/wikipedia/commons/1/18/Ikigai-EN.svg
~-~
why are messages not comming from mom
~-~



Risk Managed for the 

MyMerrill.com

Alibaba
$9.05 (today $215) $255 call for jan-21 2022


~-~~-~
Rate = USD


CQ3 = local / USD     $1 ~ 16





Of course we can say:

    Hey she invited X and say that is a fact!   And let people draw the conclusions they will draw.


    That is just like BLM saying:   ``Breonna Taylor (a completely innocent woman) was murdered by cops who broke into her house and shot her.
    This shows any black person could be wrongly killed by cops on any random day.  It could happen.’'

    And let people draw the conclusion they will draw from it.


You have a choice pop.  
You can choose to say.  It is ok to say true statements, even if it suggests an unfair conclusion.  Or it is not ok.

I say it is NOT ok.

Thus the BLM statement is NOT ok.
And Cruz’s statement is NOT ok (IF she is not supporting


~-~-~


CREATE STRUCTURE THEN LIVE IN IT:  Example:

- Future of your life (big idea)
	- Work View  (part of Designing your Life)
		- Major rewrites of this. 
			- Minor versioning of this.

~-~~

Thomas, 
Let me take a swing at it.  Since this is medium, you need each post to be independently compelling:  

-- ``Is CRT Racist?''   (this would be an abbreviated definition of CRT and your discussion about if it is racist or not.  This is only medium exciting to me)

-- ``What is CRT, and how is it applied today?''  (many of us kinda have an understanding of what CRT is, but your summary is quite nice and enlightening... at least for me)

-- ``Racism vs. color-consciousness'' and your argument against color-conscousness
	It seems to me your biggest beef is against color-consciousness and CRT is just an example of the larger category.

-- Your list of examples of color-consciousness and your conclusion against them


Take my ideas with a grain of salt.  My advisor had me take a business writing class, as he was frustrated with the need to correct my publications!  I just was left with a sense of exhaustion reading so much in one post, and your thesis seemed to wander between several interesting and important points.

still, overall cogent and compelling.

Cheers,
Dan





BUY cards note 4x6

- Tandem
- Team flow
- where is your wedge?

-MKT-
Architecture of an Industry
-- First payers in an industry are typically existing providers who hope to use mkt to secure new clients and/or to provide software that supports their business
- Industry Architect
- Industry Committee
	  - who is being fair
	- What to build
- Industry Subscription providers
- Industry Products

- Business Services Directory

IDEA:
- Vendors might pay more or less depending on the value mkt provides them, and 
	they might pay for additional visibility etc.  but reviewer compensation is clearly connected to performnace and can be shown to not have connection to reviews of paying vendors.
- Specific users might have used multiple products and thus can directly compare them
- 

~-~~
yeah... you did have me for about a paragraph, I was even gettin' a big torqued in paragraph two... but it was just too over the top.... and they I remembered...  this is Mx. Penguin, and realized it was a con job!  (I am not sure if I fully realized it was a flip, but I knew it was not legit.)   




well made point.  I have argued exactly the same about both race and gender.  (of course being both white and male had throughly poisoned my well before I began speaking)  ah well.....



ATOMIC HABITS

ARE YOU HAPPY WITH YOU
1. HABITS - Procrastinate, Dont Do Core Thing, Don't Decide
2. DESIRE - Effective Person
3. CHARACTERISTICS - Efficient w time, Planner, Accomplishes, 
4. CHANGE TODAY - Plan & Execute Days

~-~~-~
.45 USD/Thash/Day * 838K Thash/Day = 400K
~-~~-~
Covid has accelerated a shift that has been underway
==> Project-based work is rarer, and staff aug is increasing
==> Specialty work continues on driven by lang/framework skills

The whole point of AF is a place where awesome talent works alongside awesome talent using state of the art practice, and with a strong positive inclusive culture.
==> We loose all of that if our guys start working inside of random teams with random (and often low) quality levels.
==> So we vote to stay smaller.  To service our long time clients, and grow in speciality skills as each person wants to develop them, and grow only as we find new interesting project work.

~-~~-~
Problem: 
- Should Provide insights to improve bus ops
- Stuck: dashboarding, building data tables

PROBLEM:
Spending the money but not get the value
--> NEED: data-sophisticated, data-forward  (80% value)

SOLN:
-- focus contractors to data pipelining & some dash
-- roadmap to hire 2-3 junior analyis to float in build dash

ACCOUNTABLE for 5-15% improvement.


https://global-uploads.webflow.com/5f1af76ed86d6771ad48324b/5f6a64711514cb41b91b37d5_Johnson_extracting_player_tracking_data-compressed.pdf


The Hater; The Amazon boys; 

Cali - Duch Crunch - Swiss
Cuban - Rye Bread
City Girl - Dutch Crunch 
Godfather - Dutch Crunch



Put it into price of the league
-- bump up a couple of dollars per team per season.  $10

Rent out

$20-30hr for indivdual per 


Why 
My Zone -- heart rate training.  that was a popular feature
Mobile Fit

Poeple like to see instant feedback.

I want to buy a system 

Main goal this year is to Implement new technology.
-- gives 
-- Can go to the why Y and get more 


~-~-~

Adam,
Well that was an unceremoneous end to a good phone call!  But the cause was appropriate to the topic of the call:  Grid Stabilty.  (the power company cut my power in order to replace an electrical pole!)

We identified two topics to check up on:

You were going to ask what roadmap Oragami has (if any) for OpenADR adoption.
We have an approach for reducing cost and time required for adoption and certification if relevant.

The second was the idea of "control as a service" -- a device that provide PLC-like control securely over the internet to any device to be controlled.  This could possibly complement your own control devices, since this device is shipped to a client and is installable by your client in a fully self service way w/o any truck roll from your own staff.  If this second product is of potential interest to Oragami, I am happy to connect with others.

It 

  The first is just to find out if 

I am working from home (as we all are) and our  

~-~-~




For me simplest argument that time is of the essence is in regards to two rough analogies:  one to human DNA, and the other to a nuclear weapon.


HUMAN DNA
I use human DNA as an existence proof that one can develop a bootstrapping AI that is capable of GAI.  The human organism is an example of such a system, its DNA is its code, and the baby is the bootstrapping system.  In a very (very) hand wavy analysis I came with 5 megabytes as the size of the program that is encoding for the intelligence part of the human.  I tried to do it in two ways, one by counting up distinct brain regions and distinct cells in them, and two my looking at the number base pairs coding for brain structures.  This number could be off by an order of magnitude (or more).  But anyway the idea that in principle one can build a bootstrapping AI seems established to me.

I think the better argument for it, is the several chapters of a book saying…. And this is what consciousness is and this the outline of such a program that achieves it. (Admittedly with several “magic” steps. See this far side:  https://www.flickr.com/photos/jpallan/4633000725))




NUCLEAR WEAPONS
We have so far been capable of thwarting nuclear detonations since WWII.  But we are lucky that such detonations require nuclear material which is exceedingly hard to produce, and thus can be strongly controlled by the international community.  Imagine if I invented a nuclear weapon that could be powered using distilled water and constructed from household pumbing.  Do you think humanity would have been able to avoid nuclear detonations?  No way.  Of course 99.99% of humanity would avoid do such a thing, but the remaining .01% would just do us in…. And we would be utterly powerless to stop them.  It was the difficulty of obtaining nuclear material that was saving us.  And with water that limit is gone.


Well if there does exist a 5MB program that bootstraps intelligence?  then the main open variable is the relative power of available computing relative to the computing of the human mind.  Imagine if I am no smarter than you, but that I am speaking to 10,000 people all over the world at the same time, and between each on of the sentences you say to me, I am able to research and think for a weekend before I respond to you (while your mind is frozen).  I will find 50 ways to trick you or take advantage of you.  Thus an AI much slower than us seems of little threat to us and an AI much faster than us seems uncontrollable by us.

So the "nuclear material" for AI is computation, and the average person’s access to this nuclear material dictates if it is more controllable like nuclear weapons are today, or less controllable like water is today.

It does seem to me prudent to build a consciousness long long long before we reach the point of parity.  We are probably too late for that.  But it seems not too late for the AIs to be at least briefly controllable by us.
And it seems having a window of dominance could help us understand if there is any way to shape how the society of AIs that are coming.  (And I guess we could also decide to outlaw Moores law if things looked dire enough.)



So ironically enough the AI researcher who believe is dooms day is still arguing to do the AI research, since the worst outcomes are like to result from learning the key parts of the bootstrapping algorithms long after Moore’s law has put enough nuclear material into the average smart phone to do us in.







~-~-~


-- building a team; building a sales channel
   Fin Tech, & Insure Tech
   public DB what are poeple paying 
   HIPO - 
  


-- pepsit - flomodine -- try for about two weeks
-- alburteral - 
-- pulminary function testing - 
   exercise induced 
-- CPMC Davies



LOCAL FIRST -- CRTD -- distributed consensus data structures
https://martin.kleppmann.com/papers/local-first.pdf


~-~-~

Hopefully you saw my follow up email with the stats page, my goal would be to do something similar to that that refreshed every minute or so, with something that says "draft" on top and then "final" some number of minutes/hours after the game concludes that gives us time to post process data. But it would be in an app as well as the website. 

Now I get it.  It makes a lot of sense (especially since that is what the coach is expecting!)

As a teaser, and also as a way to immediately differentiate from HUDL I would be tempted to add in on or two GRAPHIC top views with info on them.  Iikely the inclusion of this will not add much complexity at all since it really just presents the same data in a visual form.  
	— Heat map of field goal percentages by location
	— Heat map of field goal percentages by blocking player location

I think they would be super interesting visually, and they immediately suggest corrective action that a coach might give to a player.  And no way HUDL can touch that at any price.


But you are right my markovian thought is definitely next-level stuff.  I wonder if the pros do it?  (I would think so, or maybe I will patent it, and sell it to them ;-) )
A coach will have a general sense that in situation, S, a certain player, P, should do choose action A not action B.  e.g. When he is undefended pass to Hank, but not Jeff, in another situation Shoot.

I am sure the coach’s advice uses very simple concepts that a player can just recognize implicitly as they happen.
BUT the coach just has a general sense of how the play unfolds and certain negative paths to avoid.  But humans are really bad a keeping comparative stats over ranges of combinations.  For us keeping stats over then 20 games would really let one build a really accurate model of what will come in all futures if you pass to Hank in the context right now.  The system could find ideas that statistically are lost in complexity for the coach, but are a clear as day, once they are shown to him.  (With matching video clips to illustrate)

Definitely next level stuff!  We would not get to that for two years minimum.  But it would also provide next level play improvement too….  


Your ideas on insights are right on, but that's not in the MVP. I'm envisioning all the next level insights (what sports people call "analytics") would be things we roll out in sometime in 2022. I think in 1Q2022 we can get the base statistics I described (passes that lead to baskets (AKA, assists), steals, rebounds, shot attempts, and makes/misses) in a table by player as well as highlights for all those items. Blocks, fouls, analytics, coaching suggestions, etc. would all be things we add later that year or in 2023 or whenever. (In addition to adding support for second camera, fix mounted installs, etc.) And then at some point we'd start the process of adding other sports. (2023/2024). 

Right.


I can't remember, did I send you all the research papers I've been collecting on the topic? I have a huge collection of them now, in addition to relevant patents in the domain. Let me know if you have gotten those and have any interest in seeing them if not, happy to send along. And if you want a mockup still of something just let me know what and I can make that happen.

No need to send the mock up, but it would be valuable for you to scan multiple web images and give me a specific #1 and #2 that you think best represents, and I can build the mock from that.  (I already think I understand, but still better not to guess)

Yeah, send me the papers.  Nice to see.  But really I am trying hard NOT to think too much on the tech side and more trying to understand the competition and the specific customer “buy" triggers.

—dan


p.s.  
Not that it matter for us to figure this out now, but I suspect by end of first year, we will have an expensive ‘fixed mount’ system to sell to gyms.  I think we are going to have to build out a “super gym" kind of capability in order to get enough training data, then we are going to want to get a bunch of super gyms out there.  

The only way I am going to get 10,000 hrs or 100,000 hrs of ground-truth-labelled game play is going to be though some kind of automation.  I think it is going to be a gym with eight cameras around the court on all sides, but correlating play across all eight cameras I will be able to get very close to ground truth in a fully automated way.  Then I can use that to train the system that operates from only one camera.  But now I need many gyms to setup this fancy system.  (Just 10 or 100 will do it… but that is already starting to look like a product… sure a product that is hand built by one tech that we hire in his garage, but still it is a thing that we send out there… might as well get the gyms to buy it.  TEN or a hundred will buy the bleeding edge product … just as a differentiator for the ultra fancy super lux high-tech gym mojo they are aiming for.)

This is how Tessa’s robotics company got their early training data and customers.





Jason

On Sat, Mar 6, 2021 at 1:02 PM Dan Oblinger <oblinger@gmail.com> wrote:
Hehe, yeah I just spent some hours building that totally wrong slide as a bit of reverse-psychology….  I wanted to prove my extreme geek nature by showing that really I have NO CLUE about sports.  
Mission complete. ;-)


Yeah, it would help if you did that on Monday, or even just pointed me at 2-3 web links to the closest match for the stats you imagine for the MVP product.

I do agree blocks and personal foul are very very hard.  We might be able to get some info by detecting game stoppage and the configuration as it resumes.

But as one who is probably just as clueless as our algorithms, anytime a foul was called on the few games I have seen, 1/3 of the time I was like, who did what?  There sometimes play continues for a second or two as the call is made, and there is a mob around the ball so you really need to be looking for tell tail signs.

Might be easiest if we have a very good mic setup to LISTEN to the coach call out who the foul was against.


~-~~
Anyway, I have been assuming we would give the stats you showed, but I had imagined going beyond that to really try to stats-out the CONTEXT of the key actions.

e.g. after looking at ten games we see Jerry as a much lower shot percentage whenever he is guarded from the 9 o-clock to 11 o-clock position just prior to the shot.
(This looks like a heat map around the shooter)

Anyway, these kinds of stats would allow Jerry to work on that specific situation.  But a real easy move for Jerry… DONT SHOOT.  Reposition or pass until he is in a more favorable shooting position.

If you look at game play as a Markov chain of moves (pass, advance, shoot), etc. then using the statistics over dozens of games, one can optimize the whole Markov chain (they whole team at once)

Simple per-player  “instant choice rules”.  e.g. Jerry when guarded on the left reposition or pass to Fred if you can

(I am sure the language is all wrong, but I this Markovian analysis is the next level stats… its not harder, its just that a human scorer could not keep track of the counts so it is not done)
BUT since the customer is not expecting that, it can’t be part of the MVP


~-~-~
Final comment:  since the customer is expecting the stats in the form you show, that is what must be done.  Still it is going to be harder to get those stats accurate.  As I was imagining it, as long as my errors tended to cancel out, I could still get good stats out.  e.g.  shot percentage could be made quite good.

But the way these stats list, if you miss a single play, then the stats are just WRONG.  Now I see why you are saying things like 98% accuracy.  But that IS going to be hard to achieve.
Course I bet a human had difficulty getting unto that 98% level

~-~-~

Send me stuff on Monday…. Thanks!
—dan





On Mar 6, 2021, at 6:41 AM, Jason Syversen <jason.syversen@gmail.com> wrote:

Here’s a sample page of a local community college online stats page. OREB means offensive rebound, STL is steal, etc.

https://www.theuscaa.com/sports/mbkb-2/2019-20/boxscores/20191101_jomf.xml?view=boxscore

That would be the base product (but we won’t start with PF (personal fouls) and maybe blocks if they’re too hard. 

Analytics will be much more complex and show a diagram of where all the shots were taken from on the court (by player and the entire team), time of possession, player efficiency ratings, breakdown by different situations, etc.

And I was envisioning the stats page would be hyperlinked to the video highlights. Click on “STL” and it takes you to a page with each steal as a video clip you can watch (maybe they’re all loading and displaying automatically)

Jason

On Mar 6, 2021, at 9:35 AM, Jason Syversen <Jason.syversen@gmail.com> wrote:

﻿Dan,

You make me chuckle. It’s definitely not correct. I can update it, but the easiest thing to do is just Google basketball stats and see examples. 

Are you trying to show the end version when we have analytics and shot charts and all that? (Hopefully at the two year mark?) Or the MVP?

Also important to talk to the right people. As I said HUDL already has a solution that’s solid (and expensive and slow). If the coach doesn’t pay for it he’s not going to care if we have an inferior version. But the district will because we are 1/3 the price. And the video highlights is a feature they don’t offer. And our final version of the product should be at least as good and I believe will likely also be better because of the analytical data we can extract. 

I can take a cut at what those things would look like but it might not be until Monday

Jason

On Mar 6, 2021, at 2:45 AM, Dan Oblinger <oblinger@gmail.com> wrote:

﻿

Jason,

I am doing a quick research spike just getting details about how coaches react to the Visio product.
I created a quick and dirty slide showing an imagined output for a single player.

I am not trying to design anything here, I just want to be concrete when I ask coaches, would you use this, and what is it worth.

But since I am not a basket ball player, I could really be doing something totally stupid here.  So I thought I would show it to you before I use it.
(No need for it to be correct… I just want to avoid STUPID. :-)


Let me know what you think.  They way I am reaching out, I will only get a moment or so with different people, so I may not even show this.
Just good to have.

Thx,
Dan


<Basketball Stats.pptx>


~-~-~
Jason, 
I am going to pass on this opportunity for a second time.

The ROI per year could be quite good, no doubt about that.  And opportunities for awesome collaborators on promising ideas are rare indeed.  So I gave it solid thought.

The problem is really one of mismatched life situations.  You have raised triple the money I have, are 10 years further from retirement, and live in a place with half the cost of living.  At the moment I cannot change any of these things, so it just means that I am not "done" earning in the way that you are.  Putting money in and not being in a position to control when the money comes back out will definitely put strain on the home front.  I am better off having a business where I can control that tradeoff better.

In the end, I may end up with a lifestyle thing that earns notably less per year, or I need to give up the game and just go to Google or Facebook where I can earn 5M+ over 10 years.  On a risk adjusted basis it is hard to argue with that...  And there are still a bit of fight left in me, I can imagine as yet taking the flying leap towards a bigger exit, but likely one with a bit more revenue coming out along the way.  

Further, given the shoot the moon, max return for charity mission for Visio, I don't feel pushing for a "better" deal to try to make this work is even a correct path to consider.  The reality is that potentially I can really boost the business in the mid years as one considers different markets and different services... potentially.  But I think many of those agenda's are things that you can pull off as a solo founder.  If you are not there, then I am really essential, but with you there I am not.  I am a "really nice to have", and a potential business multiplier.  But not essenta







~-~~-~~-~~-~~
Shane,

I was shocked by your idea of an unpickable lock, but then I was the insight, and it is brilliant!
It is clear 





~-~~-~~-~~-~

About Dan

VALUES
-- Gap in the universe

STRENGTHS / WEAKNESSES




~-~-~

-- Find other businesses using Standards as their business
-- Indentify specific standards-related capability that are indicated at searchable/driving capabilities
   indicated by others doing it; or seems like a key capability


QUESTIONS:
-- Framing of offering:  R21.  Energy Standards.
-- Grid-based Energy Standards Experts
-- sunpower is listed in readme for AF R21 client.  (check if ok)
-- should we be a fork of EPRI client on Github.
-- low road strategy:  build a landing page for project.  Provide services links at bottom?
-- case studies strategy:  w R21 being a case
-- high road is standards-based experts.  Grid energy experts.




Case Studies
-- 

Technology Stacks
-- R21   (shows what we are doing; could provide offering that few will use)
-- OpenADR

Our Offering



~-~-~

Social Cost of Carbon

A central estimate of the social cost of carbon (in 2018 dollars) is $51 per ton (Interagency Working Group on Social Cost of Carbon 2016)

[fact sheet](https://www.epa.gov/sites/production/files/2016-12/documents/social_cost_of_carbon_fact_sheet.pdf) 

~-~-~

Leila,

I got no love from the two poeple I checked inside Amazon.  I think it is going to be a pretty hard road if they do not get internal credit, and they don't really know you.  The good news is that knowledge about AWS services is very well known the world over.  I think Fiverr.com is a good place to find super cheap folks.  If you can get a couple of Amazon vounchers, I think they would find it odd, but I bet they would take them as partial payment.  e.g. you pay $50 thru Fiverr.com, then you pay $150 in vouchers, or something like that.

If you decide to go this route (which if it is going to help your career, I would do it!)
Here is my suggestion:

- Only consider folks who have done more than a dozen (ideally many dozen jobs)
- And have a very high star rating.  e.g. 4.8 or even 5.0
- You can consider getting level one or level two... they probably know more.
	But on the other hand maybe this is easy enough for someone who knows AWS to figure out.
- The good news in getting someone with a 4.9 score.... they wont take a job they cannot do well, because it will damage their score, so they will probably be honest with you.
- Get someone with lambda experience since this shows they can do the basics of programming
- Then you need to make your request VERY VERY SPECIFIC.  you cannot say, I need something, but I dont' know what.
	something like this....   That will be hard for them to do, and too dangerous that they will get it wrong.



At the bottom of this email I have a quick sketch as a starting point for the job description... you should think hard about this, and make it better, maybe I did not get the work flow correct or such.  Still this this gives you a starting point.  

Just to get you started here I did a search on Fiverr and found a few folks.  You should spend more time and see if you find others/better.  But this helps you see how to think about the serach.

I typed in "AWS pinpoint" and got back this:
	https://www.fiverr.com/search/gigs?query=AWS%20pinpoint&source=top-bar&search_in=everywhere&search-autocomplete-original-term=aws%20pinpoint

- I liked 'CloudhasIB'  he has 38 jobs and 4.8stars.  When I look at his summary he talks about lamda, and Orchstration stuff, and explicitly mentions pinpoint.   5 years experience.  Bangladesh (this can be a lower quality location, but this task is not too hard, and they are CHEAPER there)  I read his 1star and 3star reviews... they did not scare me.. and most are 5stars.   (https://www.fiverr.com/cloudhasib/provide-any-type-of-aws-services-f629?context_referrer=search_gigs&source=top-bar&ref_ctx_id=fac344e2-82b7-459d-a47d-26f964b82075&pckg_id=1&pos=1&context_type=auto&funnel=0cf29248-2440-44ab-8b4a-b770e7345d08) 
- I liked 'gaurab008' only 16 jobs, but 5.0stars.  Also explicitly mentions pinpoint, and reviews are good
- Here is one where I just searched AWS by itself.  I think pinpoint will not be too hard for a programmer type to figure out, if they already know lots of AWS stuff.  'kkchuahan11' has 481jobs and and 5.0stars and level II.
	THIS GUYS REVIEWS ARE STRONG
- I didn't keep looking under just "AWS"  there are a great many to choose from.
	(Notice I skipped over 'ridtech' even though he is a FIVERR choice, because he lists $60)

I think you should get a quote from 3 to 5 guys and then email or maybe even talk with them before you accept any of them.  You can ignore the prices they list on the page... it does kind of tell you who is more expensive or less.  But really you just want to get a flat price for what you are asking for.  Go with your gut.  if someone does not seem 100% clear about what they will and will not do, or if anything feels fishy, just move to the next.  Still I think for a job like this, many of these guys will be good.

I know this is coming out of your own pocket.  But if this pushes your career forward and it is stressing you out trying to figure it out.  Man I would just pay some dude in india for help.  if you get a good one, you can go back for smaller money to fix this or that if you need help.

Let me know how it goes, and what you decide to do!

Love ya cuz!
--dan






**JOB DESCRIPTION - Wellness Content Distrbution Application**

**OBJECTIVE**: I work at Amazon, and have video content that I want to distribute to a list of employees and vendors that specfically request to have these materials sent.  

Here are details of the desired solution:
- **AWS PIN POINT** - I am told that AWS PinPoint can be used in building your solution.  If so that is great, but it is not required.
- **CONTENT** - I will manage the creation and storage of the content itself.  Your system will be supplied text and email messages that will have links to this content, but otherwise you need not worry about the content.
- **CONTENT SEND** - The core capability of your system is to allow me to specify a message a contact list, and your system will immediately send this message to all contacts.
- **MESSAGE CHANNELS** - Recipients will be able to specify a phone number for text messages, or email address to recieve content.  {[LEILA Do you want any other channels?  Add them here if important.]}
- **MESSAGE FORMAT** - The messages send should be rich text or appropriate for email and text.  The messages must be able to have URL links in them to my content.  Ideally they may also accept images, and/or videos directly in the messages as well.
- **RECIPIENT LIST** - You system has one or more recipient lists.  When I trigger a send, I will specify which list is being sent to.
- **[OPTIONAL] RECIPIENT SELF SIGNUP** - If it does not significantly increase the cost of the job I would like to have a landing page where employees and vendors may sign themselves up, or remove themselves from these recipient lists.
	- SECURITY - Minimial security would be needed for these lists.  Ideally the system would require user to confirm via text or email that they really were the owner of the address before being registered.
	- ADMINSTRATOR SIGNUP - If implementing self-serve signup will add significant cost, we can start with an administrator control page and I will just manually add/remove recipients.
- **[OPTIONAL] TIMED DELIVERY** - If easy it would be valuable to have the ability to send scheduled messages, that way I could log in weekly and setup all messges for the week or month.
- **DELIVERY** - Please explain how I will accept delivery of your solution.  Of course I have my own AWS account, but I want to be sure the setup on my side is not too hard for what ever you deliver.  Ideally you and I can iterate w/o too much trouble as well.







... dissemination of such material evinces an invidious intent to create division within American society.


~-~~
 they tried hard to limit the applicability of impeachment because they knew it could become a political football.  But at the same time they also knew they could not enumerate all the truly heneious things a president might do, so they listed a few very serious and clear cut ones, and then added the words “high crimes” to try to limit the use of this feature to only the most egregious and clear cut bad actions.   That is my interpretation, 
~-~-~
Ajeet:

Here are a couple of ideas that could be built into a blog post


KEY IDEA
Advancements in lowpower high performance embedded processors has enabled a range of new solutions.
The "Trainable Intellgent Sensor" below fits into this category.

Trainable Intelligent Sensor
- FIXED HARDWARE + SOFTWARE DEVICE
	- THOUSANDS OF DIFFERENT SENSORS NEED -- In the manufacturing context there are hundreds or thousands of specific point and specific measurments that one would want to optomizing on even a single manufacturing line.  And there are thousands upon thousands of different kinds of manufactering lines.
	- TOO EXPENSIVE -- Developing a custom sensor with custom hardware enclosure, customer sensor components, and custom sofware is far far to expensive to be supported.
	- TRAINABLE SENSOR -- Deep learning offers a unique approach where a very generic sensor like a single CCD video camera inside a fixed enclosure could be trained to do thousands of different sensing tasks.
	- NO CHANGE IN HARDWARE; NO CHANGE IN SOFTWARE; NO CHANGE IN ENCLOSURE -- The key innovation would be to get these wildly different sensing outcomes from a single device.
	- COST EFFECTIVE -- This would provide a cost effective means of making a great many new specialized and advanced kinds of sensing in a way that could be afforded for the first time ever.


CHALLENGES -- But a number of challenges exist in trying to realize this solution, and solving some of these challenges is facilitated by Redis Gears.  Here are some of the challenges:

- costs of scaling up centralized processing -- deep learning is computational quite expensive.  Performing this in the cloud for many many edge devices would become prohibitivly expensive.  Our solution is to use a low power GPU executining directly at the edge.  (taht is suppored by Redis Edge).
	==> cost effective for processing since these edge GPUs are cheap.  (e.g. Jetson chipset)
	==> bandwidth requirement are significantly reduced since processing is performed at same location as the video input.
	
- security -- obviously one must control the security of these sensors since their outputs are used to control downstream manufacturing processing.  HAcking into these sensor could result in destruction of expensive manufacturing equipment, loss of life, destruction of the facility, etc.

- privacy -- privacy of the sensor is also critical since one can gather very detailed business intelligence regarding the execution of a manufacturing faciiltiy from breaching even just one sensor on a line.  Thus one much have absolute control of the privacy of this data.



ASDVANCED ANALYTICS -- In addition to supporting the basic infrastrcutre required for the trainable sensor the redis platform supports a number of analytics capabilities that can be used provide enhance value derived from the sensor itself

- Redis Edge Tensor plugin -- The redis edge tensor flow plugin provides effective and simple ways to dynamically load new tensor models.  This is critical to the usabitlity of the end device since work flows around training and re-training need to be streamlined in order to handle changes or problems occuring on the line.

- Redis Time series -- connection of the output of the deep learning to redis time series is quite a powerful addition in the context of manufacturing lines.  Many of the issues and optomization one which to track, predict, and improve, are not actions indicated by a single sensed value.  Instead one needs to track the evolving time-based statistics observed over multiple time windows and multiple sensors.  Having all data in redis time series provides a very simple and very powerful means of performing those running calculations.




~-~~-~

I have a thought I have been chewing on thought for a bit.   See what you think...


There are two kinds of information sources:
— those that primarily seek to inform you, and
— those that primarily seek to convince you.


It is easy to tell the difference between these two:
— A source intent on informing you will present information in ways that highlight both support and opposition to any simple overarching world view (e.g.  small government is better, left is better, right is better, pro-business pro-union, etc.)
— Imagine a weather channel that is always highlighting the fact that is it not snowing in Dallas.  This might be true, but it's not news.  News is something that goes against what is expected… it is “new”-s.  If I knew of such a station I would suspect some kind of agenda…. I dunno… they are funded by some real-estate consortium in Dallas or something silly.
— I claim any info outlet that that frames nearly every story as always supporting or always opposing…… ANYTHING.  Is not primarily a source of news, it is primarily a source of propaganda.  It might be factually accurate propaganda, or not factual, but either way, their agenda is clear — they primarily aim to persuade, or reinforce some belief.


I guess there is another possibility.  Perhaps the world view so perfect and so complete, it is such a perfect explanation of the world, there simply are NO exceptions or special cases to report on.  Of the hundreds of millions of human interactions each day there simply IS no contrary information worth reporting.  Yeah, and I have hot land for cheap in the everglades to sell you….


So here is my maxim:

If you don’t have info sources you trust and listen to which regularly highlight examples that challenge or contradict your core beliefs, you are not informed … you are managed.  

Now of course not everyone really wants to be informed and challenged, I get that.  
Either way, it is just good to know where you stand.




What ya think?  You buy it?

—d




~-~~-~

I thought this was a nice website showing both left/right spin of each news source as well as the accuracy of each site.

If you go to the interactive chart (it take a bit to understand how it work)
It will show you the individual article ratings used to rank each org.

Basically they have gotten conservative, liberal, and independent leaning folks to review each article in order to get consensus if it leans left/right and it is making unsupported / incorrect claims etc.

Of course we have to trust Ad Fontes itself….  But kicking the tires on them, they seemed pretty legit, and they have developed a standard way to grade factual support and such in order to allow them to consistently rate these site.
Also their rating seem to match my sense of these outlets.

e.g. They say New York Times is less factually accurate than WSJ, and MSNBC is more left than NY times which is already left.  Its fun to look at, you have to zoom in or search in order to find all news sources
      https://www.adfontesmedia.com/


~-~-~
- Rice: Plain, w meat, 
- Wusun, Gbeans, Peas, Egg Plant, Cauliflower
- FRUIT:  all
~-~-~

????
- lentils, barley

NOT NOW
- Broccoli, Stringy Veggies
- TOO HARD: Edamame
- Egg:
- Yougurt:


My Mission -- 
- ideas in my head out into the world
- in a way that is beneficial


It is what is Expected


Pure Aims can be twisted  


~-~
I am the CTO at Analytics Fire and have been thinking a bit about Open ADR


I am the CTO for Analytics Fire and have been thinking a bit about Open ADR, if you a few minutes for a chat, I would love to get your thoughts as it relates to EVSE.

I am the CTO for Analytics Fire and have been thinking a bit about Open ADR, if you have time for a 10 minute chat I would love to get your thoughts as an expert in EVSE.

I am the CTO for Analytics Fire and have been thinking a bit about Open ADR.  If you have 10 minutes, I would love to chat with an expert in EVSE.




~-~-~
dyslexic example.... even after looking directly at correct and incorrect words I could not see what was wrong.
even after looking for better part of a minute.
once I vertically aligned the correct and incorrect versions I could run down the letters to see where they mismatched:

        in_bar_var_referenece = 
                 ~# reference


~-~~-~
- possible client companies: MfgOfADRables; Strategy Orgs (Energy Sol); NC clean energy; Epri  
- AF Radio
- Podcast; Redis; 
- - video2Krz. AF-slides2Ajeet


~-~~
acid reflux
proton pump inhibitor for a month; prilosec
neurgenic -- relaxing medicine
reactive airway disease -- asma ; pulminary function ; 
alergy medicine ; claritin.; 

~-~~
Enterprise: web-mobile:  talk-to-10-other-dev-shops
Embedded:
Standards business OpenADR:
Product: 
Something Else:


Agree with your stance.  This is my field of study, though my views are not consensus with my colleagues.




I think there are two processes at work, one is a research science getting closer to a general bootstrapping modelling system, capable of using experience to bootstrap models of 'stuff' (including all of human knowlege).  I think we are making good progress towards this but I do think we still have a couple qualitative gaps.  but they will be plugged just as prior gaps, (e.g. the deep learning revolution).




The second process is Moore's Law.  Computation is not even yet using the third dimension (chips are only <20 layers deep), so we have a long way to go.




Think of Moore's Law as the stock piling of nuclear material.  And the algorithms as the blueprints for a nuclear weapon.




And your goal of humanities survival as our non-proliferation treaties and efforts.




Now non-proliferation is a reasonable agenda since obtaining nuclear material is difficult.  But what if it were not?  What if every cell phone had enough nuclear material to flatten the earth?  We could never keep that genie in the bottle!




So this is our challenge, if bootstrapping strong AI is possible (E.g. there is a "small seed program" which becomes known) then the only variable is how powerful are the computers upon which is runs.




Maybe we can build policing AIs to scan for and attack others, but then we will be as ants hoping to maintain control of fighting elephants.  In the gaps between each sentence we utter, they will have lived entire lives of our experience.  (admittedly that large of a chasm will only exist after decades more of Moore's Law...  but I think we have lost control long before that point.)




I am not sure I see any way out... except to stop developing more powerful computers.  This would limit the power of any such AI -- but even networked systems of today might already be pretty dangerous.  We might have to back up to make big computing as rare as uranium is today.




not such a popular view as you can imagine.



~-~~-~

there are numbers that are certified, will be certified, that are ceritified.

he just asserted as if God had told him ``we know we want the won the state''
I don't know if he truely is delusional, or if he just uses such certainty to bend reality to his will.  
He is quite effective at it in any case.


I liked when the got to the part about the 5,000 dead.  the other guys said, we dug into this and found two.
Trump asks his collegue to respond, and she says well we just used birth year and name, that is how we came to that number... we don't have the info you have, and we have asked for it...

So it really does show the fishing trip here...  They their goal was to turn up some kind of argument that somehting happened.  No need to really verify if it DID happen, just assert it as fact: 5000 dead people voted in Georgia.

~-~~
Looking at my rallies there is no way I lost the state.


At 49 minutes in he begins talking about the governor, and he says
	``I was a schmuck for endorsing him and helping him get elected governor...  he is a disaster, what a mess.''

So what changed his mind?  Why is he such a disaster now?  Simple, he is not doing as he was told by Trump.
That's it.  He was fine before this, but not now.

over and over again.  100% loyalty, or I will crush you!  And it has worked well for him.

At 59min again talking about how stupid he was to support someone...  ``what a schmuck I was''
why?  Becuase when he needs them to be 100% loyal, no questions asked, they failed him.

why else, all of the sudden, are these favored poeple "dumb" and such?






~-~-~
-- IP-based consulting sales:
-- Transcend
-- new consulting sales approch


~-~-~
Landfill:
- 2.01B tons annually. The world generates 2.01 billion tonnes of municipal solid waste annually, with at least 33 percent of that—extremely conservatively—not managed in an environmentally safe manner.
- CELL: 2,500 tons. 50x50x14
- ~ 1M cells = 2.01 / 2,500
- 10mi x 10mi = 50K x 50K x 14



### 2020-12-23 - XMAS FAMILY ZOOM
~-~~-~
Adam Brown <Brownadam50@gmail.com>,
Monica Brown <Cruzbugs1@gmail.com>,
Ron Hall <hall.ron121@gmail.com>,
Carolyn Oblinger <coblinger@icloud.com>,
John Brown <jeb5004@gmail.com>,
Ashley <aebrowie@yahoo.com>,
Bunny Brown <bunbun4029@aol.com>,
Kathleen Hall <khall92@aol.com>, 
Leila <pawsitivenrg@yahoo.com>, 
Dan Oblinger <pa.oblinger@icoud.com>,
Dave Oblinger <daveo2112@yahoo.com>,  
Linda Litzler <lalitzler@fuse.net>,
Tina <tinamarie77@yahoo.com>, 
Lisa Parnell <lparnell119@yahoo.com>


Zoom Link:
https://us02web.zoom.us/j/86718148976?pwd=b3M4cFJxUHFnZnpuU3kyWW8vNzg0QT09


Passcode: 1
One tap mobile
+16699006833,,86718148976# US (San Jose)
+12532158782,,86718148976# US (Tacoma)

Call   +1 646 876 9923 US (New York)
Meeting ID: 867 1814 8976


If you have never used Zoom this video you can download the app, 
here is a video that walks thru the setup process.
https://www.youtube.com/watch?v=9isp3qPeQ0E













~-~~-~
### 2020-12-23 - SHANE RESUME
- Should have a stmt about what you WANT that possibly ties in with your experience statement. When hiring you I want to know that you want to do what I am looking for, so try to frame your aim.  (I would edit this objective when applying to different kinds of companies too.)
- without even a single bullet item on the the new dawn job, that section looks suspicious.  (they might learn just how, uh light that job was!  I think you advised us on the development of a trusted hardware IOT platform...   you need to vet this with Nick, but I think that is totally legit, since you did, and if a prospective employer were to question you on this, you could talk at length about the idea, so it would be fine.)
- ENPHASE -- I want to focus on tangable outcomes here.  
	- So 3 to 21 members is GREAT to say.
	- Can you say something quantitative about the key industry partners, how many, or what was the outcome or such?
	- Established cross-domain cohesion ... blah blah.  ok.  So what?  can we say anything about the benfits gained or any outcomes from that?
	- In general I am looking for one or two big thump statements of something you did at enphase along with something about outcome / benefit that happened.  these are the top of this section.  then below are lists of things you did in service of those big goals.. eg. guided security ...

- SIBROS -- 
	- same comment.  you engaged in BD.  so what?  what happen based on your amazingness?


~-~~
you almost have a 2 page resume.  if you can make it 2, that is nice.  but if we end up adding too much, then dont worry.

the big comment is this is just a wall of details... because they might do text search on these there is virtue in putting details with each entry.  BUT it is really hard to look at this and get a sense of who shane is without REALLY putting in many minutes of focused attention. (which they never will)

what you want is a story that is like:
- I whet here to do 'x', but then ended up doing 'X', and wow even XXX happened.
- then I went to do 'y', but they promoted me to do 'Y'. and wow YYY happened.

This narrative is all about the amazing shane dropping into situation after situation and he moves up in responsibility, and awesome outcomes happen... then he moves on and does it again.

this narrative story is in the top sections of the experience, then the bullet details are listed below (with those bullets with outcome assertions in them towards the top of each section.)

~-~-~
this is just one way to slice the resume...  happy to chat about this tomorrow too.



_
### UF.AGENDA.INTRO.ELEMENTAL

The joy of programming is the translation of idea into action, the creation of a new and useful capabilities for yourself and others to enjoy.

Too often, however, the joy of programming is eclipsed by the muck of programming.  Struggling with some fiddly bits of some crazy/unexpected interaction between sub-components that you barely knew existed, and which have absolutely no relevance to the thing you are trying to create.  But it is a hard roadblock, until you or someone understands and mitigates this mal-behavior no creation can happen.

Sadly it is this fiddly part that comes to dominate, and nearly define the whole of programming.  The creative sparks become separated by moutains and months of endless fiddling.  Indeed programming has mostly become a rarefied sport that can only be effectively done by those who have spent years of their lives painstakingly learning thousands of tips and tricks for taming the fiddly parts of hundreds of their most commonly used components.  Like witch doctors of old, they trade "spell books" with incantations long ago discovered that will make this or that work in one situation or another -- no joy here, no understanding here, rather just fiddling 'till it works, then on to the next fiddly bit.

But it need not be this way.  Indeed the fiddly bits are inadvertently of our own making -- so the solution to this problem at its core is simple, just don't create those fiddly bits in the first place!


In practice the actual solution is quite a bit harder, so hard in fact I dont think it is practical to attack the end goal in one step, instead we our sights on this outline of programming without any fiddly bits on the horizon an start marching in that direction, erasing and rewriting fiddly bits as we go.  Here we lay the agenda that will guide our steps as we move towards this gleaming city upon the hill.

First we must start at the bottom, we cannot blindly accept the components that have been handed down to us from generations before, nearly all of them are hopelessly filled with an aggregation of complexity upon complexity.  Instead we find 



out two umbrella objectives that give shape to this outline in the distance, elemental and componential.
we can some this problem by just not adding these fiddly bits


elemental
	unitary
	minimal
componential 
	shimless
### UF.MKT

- 

goal of marketplace

maximize value freely created by the maker, and freely paid for by all consumers.



Background ideas
- A knowledge product is created once and then is infinitely consumed.
- Indeed the value of the product typically does not decrease, but rather increases because of network effects as it is consumed.
- Putting any consideration of money transfer aside BOTH the consumer and producer of knowledge benefit from this transaction:
	- the consumer benefits from
	- and the producer benefits from the very act of producing a thing which has value for others.



Given the infinite consumption with increasing value, and net benefit to the PRODUCER of knowlege 



_
### !The perversion of capitalism -- keeping the best and dispensing the rest

TL;DR.  Capitalism as applied to knowlege work is inherently evil because it incentivises the perversion of human effort.  It rewards innovation in IMPEDING of creation far far more than it rewards innovation in creating value.

I am not one of those new-melinna hippy-types that believe we should just burn all corporations down and somehow live in a kind of post-capitalistic kumbaya.

Or rather I salute and applaud the immensive value that the distributed decsion making capital markets hve bestowed on humanity.  And I firmly beleive that any "improvment" to this system must retain this distributed decsion-making nature, and the incentive-alignment of producer/consmers upon which captialism is based.

but now that my appluse dies down I must also acknowldge their is inherent evil and perversion baked into the nature of capitalism, and have a few thoughts about how to remove them while keeping its best parts



" The Evil Nature of Sustainable Differentiation "
### UF CODE
Convert inline key/value pairs into pair form
        if self.parser.stmt_keys and len(body) > 1:
            i, new_body = 1, body[:1]
            while i < len(body):
                key = body[i]
                if not isinstance(key, Op):
                    new_body.append(self.parser.parse_error(f"Illegal statement key {key}"))
                    break
                elif isinstance(key.head, str) and re.match(_identifier_re, key.head):
                    if i+1 >= len(body):
                        new_body.append(self.parser.parse_error(f"Illegal statement odd number of args", op))
                        break
                    value = body[i+1]
                    new_body.append(Op(self.COLON, -1, children=[self.parser.into_ident(key.head), value]))              # should be parse expr for complex expr
                    i += 2
                elif key_head := self.pull_head_from_expr(key):
                    new_body.append(Op(self.COLON, -1, children=[key_head, key]))
                    i += 1
                else:
                    new_body.append(self.parser.parse_error(f"Illegal Statement Key {key}"))
                    break
            body = new_body


	

rewrite_block_body
- loop body calling rewrite stmt

rewrite_stmt
- if pair 		consider blk processing, but that is all
- if not group	consider pulling head
- if seq			perform seq rewriting
### ELI verbal~~

-- Name things
-- Name actions he is doing, dog is doing, adult is doing, water is doing
-- Ask things/actions
_
### HVAC Email ~-~~-~

CONTEXT:
- Cold email sent to Grid Storage MFG


PROPOSED SKELETON
- HOOK FACT -- We start by saying something about a current happening that will affect their business.

TEMPLATE VARS
- NNNNN -- The person's name
- RRRRR -- The person's role
- CCCCC -- The company's name


NNNNN,

As your sales team likely have already noted, the rollout of XXXXXX as well as focus on customer cost saving are driving a move towards OpenADR compliance for all grid attached storage.  

This presents a critical opportunity to grow market share, and a risk of loosing market share if others certify in front of CCCCCC.

We are boutique team of energy-grid-device experts dedicated to implementation and certification of open  standards like OpenADR.

I would like to spend 15 minutes discussing the CCCCC roadmap for OpenADR and strategies you might consider to get there faster.

Cheers, 
Dan



--- THE CUTTING ROOM FLOOR ---

Your sales team is likely already getting requests for OpenADR compliance, and we believe this demand is going to dramatically increase in 2021 as xxxx comes into force.

We bring testing and scaffolding code to each of our efforts in order to rapidly bring devices into compliance with standards like OpenADR, 2030.5 etc.



CEC title-24 energy code standards

HVAC email


in both the implementation and 


~-~~-~
### !Tucker trump-email

I am a Kentucky boy who got a PhD in AI and worked for DARPA giving away couple hundred million of your tax payer dollars for advanced AI reaserach -- this gives me some kind of combo left/right view of things.  

I was pleasantly shocked when I saw your piece on the hyperventellation of the left regarding Trump's taxes where you (correctly) noticed that this is currently just how the rich roll; but you went on to notice that if things dont change then at some point this shit is gonna get real (bad).

Hyperscaling (e.g. amazonification), Automation and AI both are getting more done with fewer people.  They require larger and larger capital, fewer and fewer people, and produce more and more goods and services per dollar.  The result is very predictable: the rich get richer and the entire working class gets poorer in relative terms.  Worse, silicon valley (where I currently work) is getting better at rolling out society-wide changes at much faster speeds than in previous decades:  iPhone, Uber, Facebook, and Amazon all made big changes in how we live and each wiped out the livelihood of a whole job class of workers.  They each wiped out old ways of doing business at a speed much faster than with prior revolutions.  And I am here to tell you, we technologists are only getting started.  With globalization, it has gotten much EASIER to execute planet-wide rollouts of cheaper tech that save money using fewer people).  You ain't see nothing yet!

You and I share the suspicion that the "free lunch"-loving, circular-firing-squad on the left is not going to plot a good path forward in solving this.  Yet if the right does not get a plan for this, then they left will gain and keep the reigns of power since more and more people will have no stake in the current system.  But right now it seems very few on the right are even talking as you did about the problem, and even further from talking about concrete action towards a solution to the shitstorm heading our way.  

As a conservative I imagine you would see each of these as bedrock for any good path forward:

**-1- EVERYONE WORKS** -- Everybody who can work should be strongly incentivised to do so
    -- it is good for society and it is good for the human spirit.
**-2- INDIVIDUAL LEVEL DECISION MAKING** -- some kind of distributed-decision making system need to be in place
    -- self determination; joy of control/ownership; and failures of central planning are too great to ignore
**-3- PAYING PEOPLE** -- Effects of automation and AI shift fraction of each dollar spent towards capital and away from labor.
    -- This technology can be a net good, since is provides more and better quality per person for society.
    -- But as fewer people are needed to build more widgets per day it naturally creates a run-away effect where the percentage of each corporate dollar spent on capital goes up and percent spend on labor goes down.  Without some kind of limit, this effect will flip the apple cart--I think within the next generation.


Those on the right are fine with my first two points, but the third point it is either ignored, saying those who cant find a job are just good-for-nothings, or they engage in wishful thinking about keeping things "the same."  e.g. by preserving coal mining jobs or some such without addressing the underlying cause.  Neither of these paths are gonna fly and if the right ignores this issue they will begin loosing all elections as fewer folks are holding onto the winning end of the stick.






In the past 


~-~~-~
### Larry Calling bullshit


Fact #1 -- Most people (even old ones with preconditions) don't die on any random given month.  FACT FACT FACT !!!
Fact #2 -- So if you test positive for covid and then you are dead one month later, it is simple math...
	we are 90+% sure it was covid that pushed you over the edge; that you *would* be alive today if you didn't get covid.

These two facts are simple basic math.  No debate here:

    90+% of those who test positive and then die soon after were pushed over edge by covid.



So when when the daily body bag count in canada is 3 and in the USA it is 1,000+ we know by simple counting of body bags that America suck eggs at controlling covid.

Canada is presently LESS locked down than the USA, and Canada has 10x less covid.


compare the USA against many countries and we see just how bad we suck at it.





ttp

x-webdoc://C8D7ACFB-F698-4387-A899-D5119B17835A/I%20finally%20have%20a%20nanny%20that%20I%20can%20might%20be%20able%20to%20work%20with.%20%20I%20found%20this%20old%20email%20from%20My%20Payroll.%20%20It%20seems%20to%20me%20that%20the%20last%20phone%20call%20I%20receive%20from%20your%20org%20was%20from%20another%20person,%20but%20I%20cannot%20find%20any%20info%20on%20that.%20%20Either%20way%20I%20would%20like%20to%20get%20an%20estimate%20on%20the%20total%20taxes%20that%20will%20need%20to%20be%20paid%20if%20possible.%20%20%20%20Are%20you%20the%20person%20I%20should%20connect%20with%20in%20order%20to%20get%20this%20estimate?

~-~-~

				Waymo 	Merch Analytics	
Technically				X
Hours					X
Growth/Lead		X


Pay						X




_
### Steph (vic)

Oh wow this is a lot of honesty for a short email!  Still you are right that I do appreciate candidness and yeah sometime I overuse it too.

In your email you wrote two different things:  "I am not opposed to a call" and "I am reluctant to chat"


In the end if chatting with me isn’t a positive thing, then we just shouldn’t do it, and you don’t owe any explanations.

I feel like you and I have had a couple conversations that were really pretty freaking deep and personal.  I know we are not best friends or any such, I just imagined having to split after so much effort would be pretty huge, and thought it would be nice to chat -- not to fix it, just to talk.


But you would know better than anyone if connecting is a positive thing.  So just let me know...  

Warm thoughts from the west coast,
Dan






angel past, boscaiola, carbonara,   roasted beets, musth garlic
### Kernel Dump (osx)
2020-08-21 ~-~~-~~-~~-~~-~-~

panic(cpu 2 caller 0xffffff7f9a262a8d): watchdog timeout: no checkins from watchdogd in 96 seconds (8446 total checkins since monitoring last enabled)
Backtrace (CPU 2), Frame : Return Address
0xffffff820a083c40 : 0xffffff801911a65d 
0xffffff820a083c90 : 0xffffff8019254a75 
0xffffff820a083cd0 : 0xffffff80192465fe 
0xffffff820a083d20 : 0xffffff80190c0a40 
0xffffff820a083d40 : 0xffffff8019119d27 
0xffffff820a083e40 : 0xffffff801911a117 
0xffffff820a083e90 : 0xffffff80198c1b28 
0xffffff820a083f00 : 0xffffff7f9a262a8d 
0xffffff820a083f10 : 0xffffff7f9a26247b 
0xffffff820a083f50 : 0xffffff7f9a277d9c 
0xffffff820a083fa0 : 0xffffff80190c013e 
      Kernel Extensions in backtrace:
         com.apple.driver.watchdog(1.0)[832CC890-EE61-33E0-8FD4-8D354BCD0921]@0xffffff7f9a261000->0xffffff7f9a269fff
         com.apple.driver.AppleSMC(3.1.9)[AB612149-B321-3B95-8741-B99E79274FCD]@0xffffff7f9a26a000->0xffffff7f9a288fff
            dependency: com.apple.iokit.IOACPIFamily(1.4)[68557A36-4EE1-372A-983B-BB2769FDB8E0]@0xffffff7f9a258000
            dependency: com.apple.driver.watchdog(1)[832CC890-EE61-33E0-8FD4-8D354BCD0921]@0xffffff7f9a261000
            dependency: com.apple.iokit.IOPCIFamily(2.9)[DF219CC1-366A-31FC-B8ED-17C584BA2549]@0xffffff7f99b31000

BSD process name corresponding to current thread: kernel_task

Mac OS version:
19G2021

Kernel version:
Darwin Kernel Version 19.6.0: Thu Jun 18 20:49:00 PDT 2020; root:xnu-6153.141.1~1/RELEASE_X86_64
Kernel UUID: 1D3A0F3D-D908-397B-BD16-8F48C0823A2E
Kernel slide:     0x0000000018e00000
Kernel text base: 0xffffff8019000000
__HIB  text base: 0xffffff8018f00000
System model name: MacBookPro14,3 (Mac-551B86E5744E2388)
System shutdown begun: NO
Panic diags file available: YES (0x0)

System uptime in nanoseconds: 84556068003852
last loaded kext at 83348528323351: >!AXsanScheme	3 (addr 0xffffff7f9e272000, size 32768)
last unloaded kext at 83585267448179: >!AXsanScheme	3 (addr 0xffffff7f9e272000, size 32768)
loaded kexts:
com.google.drivefs.filesystems.dfsfuse	41.0.2
com.bitdefender.TMProtection	5.0.0
com.bitdefender.FileProtect	1
com.bitdefender.SelfProtect	1.2.16
>AudioAUUC	1.70
@fileutil	20.036.15
>!AHIDALSService	1
>AGPM	111.4.4
>!APlatformEnabler	2.7.0d0
>X86PlatformShim	1.0.0
@filesystems.autofs	3.0
>!AUpstreamUserClient	3.6.8
@kext.AMDFramebuffer	3.1.0
@kext.AMDRadeonServiceManager	3.1.0
@kext.AMDRadeonX4000	3.1.0
>!AGraphicsDevicePolicy	5.2.6
@AGDCPluginDisplayMetrics	5.2.6
>!AHDA	283.15
>!AHV	1
|IOUserEthernet	1.0.1
|IO!BSerialManager	7.0.6f7
>pmtelemetry	1
@Dont_Steal_Mac_OS_X	7.0.0
>AGDCBacklightControl	5.2.6
>!A!IKBLGraphics	14.0.7
@kext.AMD9500!C	3.1.0
>!A!IPCHPMC	2.0.1
>!AMuxControl	5.2.6
>SMCMotionSensor	3.0.4d1
>!AThunderboltIP	3.1.4
>!AEmbeddedOSSupportHost	1
>!AGFXHDA	100.1.429
>eficheck	1
>!A!IKBLGraphicsFramebuffer	14.0.7
>!AMCCSControl	1.14
>!A!ISlowAdaptiveClocking	4.0.0
>!AVirtIO	1.0
@filesystems.hfs.kext	522.100.5
@!AFSCompression.!AFSCompressionTypeDataless	1.0.0d1
@BootCache	40
@!AFSCompression.!AFSCompressionTypeZlib	1.0.0
>!ATopCaseHIDEventDriver	3430.1
>AirPort.BrcmNIC	1400.1.1
@filesystems.apfs	1412.141.1
@private.KextAudit	1.0
>!ASmartBatteryManager	161.0.0
>!AACPIButtons	6.1
>!ARTC	2.0
>!ASMBIOS	2.1
>!AACPIEC	6.1
>!AAPIC	1.7
$!AImage4	1
@nke.applicationfirewall	303
$TMSafetyNet	8
@!ASystemPolicy	2.0.0
|EndpointSecurity	1
>!UAudio	323.4
@kext.triggers	1.0
@kext.AMDRadeonX4100HWLibs	1.0
@kext.AMDRadeonX4000HWServices	3.1.0
>DspFuncLib	283.15
@kext.OSvKernDSPLib	529
|IOAVB!F	850.1
|IO!BHost!CUARTTransport	7.0.6f7
|IO!BHost!CTransport	7.0.6f7
@!AGPUWrangler	5.2.6
>!ABacklightExpert	1.1.0
>!AGraphicsControl	5.2.6
|IONDRVSupport	576.1
>!AHDA!C	283.15
|IOHDA!F	283.15
|IOAccelerator!F2	438.7.3
@kext.AMDSupport	3.1.0
@!AGraphicsDeviceControl	5.2.6
>!A!ILpssUARTv1	3.0.60
>!A!ILpssUARTCommon	3.0.60
>!AOnboardSerial	1.0
>!ASMBus!C	1.0.18d1
|IOGraphics!F	576.1
|IOSlowAdaptiveClocking!F	1.0.0
>X86PlatformPlugin	1.0.0
>IOPlatformPlugin!F	6.0.0d8
@plugin.IOgPTPPlugin	840.3
|IOEthernetAVB!C	1.1.0
>usb.cdc.ncm	5.0.0
>usb.!UiBridge	1.0
>usb.cdc	5.0.0
>usb.networking	5.0.0
>usb.!UHostCompositeDevice	1.2
|IOAudio!F	300.2
@vecLib.kext	1.2.0
|IOSerial!F	11
|IOSurface	269.11
@filesystems.hfs.encodings.kext	1
>!AActuatorDriver	3440.1
>!AHIDKeyboard	209
>!AHS!BDriver	3430.1
>IO!BHIDDriver	7.0.6f7
|IO!B!F	7.0.6f7
|IO!BPacketLogger	7.0.6f7
>!AMultitouchDriver	3440.1
>!AInputDeviceSupport	3440.8
>!AHSSPIHIDDriver	59
>!AHSSPISupport	59
|IO80211!F	1200.12.2b1
>mDNSOffloadUserClient	1.0.1b8
>corecapture	1.0.4
|IOSkywalk!F	1
>!A!ILpssSpi!C	3.0.60
>!AThunderboltPCIDownAdapter	2.5.4
>!AThunderboltDPInAdapter	6.2.6
>!AThunderboltDPAdapter!F	6.2.6
|IONVMe!F	2.1.0
>!AThunderboltNHI	5.8.6
>!AHPM	3.4.4
|IOThunderbolt!F	7.6.1
>!A!ILpssI2C!C	3.0.60
>!A!ILpssDmac	3.0.60
>!A!ILpssGspi	3.0.60
>!A!ILpssI2C	3.0.60
>usb.!UXHCIPCI	1.2
>usb.!UXHCI	1.2
|IOUSB!F	900.4.2
>!AEFINVRAM	2.1
>!AEFIRuntime	2.1
|IOSMBus!F	1.1
|IOHID!F	2.0.0
$quarantine	4
$sandbox	300.0
@kext.!AMatch	1.0.0d1
>DiskImages	493.0.0
>!AFDEKeyStore	28.30
>!AEffaceable!S	1.0
>!ASSE	1.0
>!AKeyStore	2
>!UTDM	489.120.1
|IOSCSIBlockCommandsDevice	422.120.3
>!ACredentialManager	1.0
>KernelRelayHost	1
>!ASEPManager	1.0.1
>IOSlaveProcessor	1
|IOUSBMass!SDriver	157.140.1
|IOSCSIArchitectureModel!F	422.120.3
|IO!S!F	2.1
|IOUSBHost!F	1.2
>!UHostMergeProperties	1.2
>usb.!UCommon	1.0
>!ABusPower!C	1.0
|CoreAnalytics!F	1
>!AMobileFileIntegrity	1.0.5
@kext.CoreTrust	1
|IOTimeSync!F	840.3
|IONetworking!F	3.4
|IOReport!F	47
>!AACPIPlatform	6.1
>!ASMC	3.1.9
>watchdog	1
|IOPCI!F	2.9
|IOACPI!F	1.4
@kec.pthread	1
@kec.corecrypto	1.0
@kec.Libm	1







~-~~-~~-~~-~~-~

panic(cpu 2 caller 0xffffff7f84c62a8d): watchdog timeout: no checkins from watchdogd in 96 seconds (3495 total checkins since monitoring last enabled)
Backtrace (CPU 2), Frame : Return Address
0xffffff81f4b83c40 : 0xffffff8003b1a65d 
0xffffff81f4b83c90 : 0xffffff8003c54a75 
0xffffff81f4b83cd0 : 0xffffff8003c465fe 
0xffffff81f4b83d20 : 0xffffff8003ac0a40 
0xffffff81f4b83d40 : 0xffffff8003b19d27 
0xffffff81f4b83e40 : 0xffffff8003b1a117 
0xffffff81f4b83e90 : 0xffffff80042c1b28 
0xffffff81f4b83f00 : 0xffffff7f84c62a8d 
0xffffff81f4b83f10 : 0xffffff7f84c6247b 
0xffffff81f4b83f50 : 0xffffff7f84c77d9c 
0xffffff81f4b83fa0 : 0xffffff8003ac013e 
      Kernel Extensions in backtrace:
         com.apple.driver.watchdog(1.0)[832CC890-EE61-33E0-8FD4-8D354BCD0921]@0xffffff7f84c61000->0xffffff7f84c69fff
         com.apple.driver.AppleSMC(3.1.9)[AB612149-B321-3B95-8741-B99E79274FCD]@0xffffff7f84c6a000->0xffffff7f84c88fff
            dependency: com.apple.iokit.IOACPIFamily(1.4)[68557A36-4EE1-372A-983B-BB2769FDB8E0]@0xffffff7f84c58000
            dependency: com.apple.driver.watchdog(1)[832CC890-EE61-33E0-8FD4-8D354BCD0921]@0xffffff7f84c61000
            dependency: com.apple.iokit.IOPCIFamily(2.9)[DF219CC1-366A-31FC-B8ED-17C584BA2549]@0xffffff7f84531000

BSD process name corresponding to current thread: kernel_task

Mac OS version:
19G2021

Kernel version:
Darwin Kernel Version 19.6.0: Thu Jun 18 20:49:00 PDT 2020; root:xnu-6153.141.1~1/RELEASE_X86_64
Kernel UUID: 1D3A0F3D-D908-397B-BD16-8F48C0823A2E
Kernel slide:     0x0000000003800000
Kernel text base: 0xffffff8003a00000
__HIB  text base: 0xffffff8003900000
System model name: MacBookPro14,3 (Mac-551B86E5744E2388)
System shutdown begun: NO
Panic diags file available: YES (0x0)

System uptime in nanoseconds: 35042464320359
last loaded kext at 23385105302150: >!AXsanScheme	3 (addr 0xffffff7f88c1e000, size 32768)
last unloaded kext at 23733388352016: @filesystems.ntfs	3.14.3 (addr 0xffffff7f88c96000, size 385024)
loaded kexts:
com.bitdefender.TMProtection	5.0.0
com.google.drivefs.filesystems.dfsfuse	41.0.2
com.bitdefender.FileProtect	1
com.bitdefender.SelfProtect	1.2.16
>!U!SCoexistentDriver	489.120.1
>AudioAUUC	1.70
@fileutil	20.036.15
>!APlatformEnabler	2.7.0d0
>AGPM	111.4.4
>X86PlatformShim	1.0.0
>!AHIDALSService	1
@filesystems.autofs	3.0
>!AUpstreamUserClient	3.6.8
>!AHDA	283.15
@kext.AMDFramebuffer	3.1.0
@kext.AMDRadeonServiceManager	3.1.0
@kext.AMDRadeonX4000	3.1.0
>!AGraphicsDevicePolicy	5.2.6
@AGDCPluginDisplayMetrics	5.2.6
>!A!IPCHPMC	2.0.1
>!AHV	1
|IOUserEthernet	1.0.1
>AGDCBacklightControl	5.2.6
>!AThunderboltIP	3.1.4
>!AMuxControl	5.2.6
>SMCMotionSensor	3.0.4d1
|IO!BSerialManager	7.0.6f7
>!AGFXHDA	100.1.429
>!A!IKBLGraphics	14.0.7
>pmtelemetry	1
@Dont_Steal_Mac_OS_X	7.0.0
>!A!ISlowAdaptiveClocking	4.0.0
>!A!IKBLGraphicsFramebuffer	14.0.7
@kext.AMD9500!C	3.1.0
>!AEmbeddedOSSupportHost	1
>eficheck	1
>!AMCCSControl	1.14
>!AVirtIO	1.0
@filesystems.hfs.kext	522.100.5
@!AFSCompression.!AFSCompressionTypeDataless	1.0.0d1
@BootCache	40
@!AFSCompression.!AFSCompressionTypeZlib	1.0.0
>!ATopCaseHIDEventDriver	3430.1
>AirPort.BrcmNIC	1400.1.1
@filesystems.apfs	1412.141.1
@private.KextAudit	1.0
>!ASmartBatteryManager	161.0.0
>!AACPIButtons	6.1
>!ARTC	2.0
>!ASMBIOS	2.1
>!AACPIEC	6.1
>!AAPIC	1.7
$!AImage4	1
@nke.applicationfirewall	303
$TMSafetyNet	8
@!ASystemPolicy	2.0.0
|EndpointSecurity	1
>Core!S	551
|IOUSBMass!SClass	4.0.4
@kext.triggers	1.0
>DspFuncLib	283.15
@kext.OSvKernDSPLib	529
@kext.AMDRadeonX4100HWLibs	1.0
@kext.AMDRadeonX4000HWServices	3.1.0
|IOAVB!F	850.1
|IO!BHost!CUARTTransport	7.0.6f7
|IO!BHost!CTransport	7.0.6f7
>!ABacklightExpert	1.1.0
>!AGraphicsControl	5.2.6
>!AHDA!C	283.15
|IOHDA!F	283.15
|IONDRVSupport	576.1
@!AGPUWrangler	5.2.6
|IOSlowAdaptiveClocking!F	1.0.0
|IOAccelerator!F2	438.7.3
@kext.AMDSupport	3.1.0
@!AGraphicsDeviceControl	5.2.6
>!A!ILpssUARTv1	3.0.60
>!A!ILpssUARTCommon	3.0.60
>!AOnboardSerial	1.0
>!ASMBus!C	1.0.18d1
|IOGraphics!F	576.1
>X86PlatformPlugin	1.0.0
>IOPlatformPlugin!F	6.0.0d8
@plugin.IOgPTPPlugin	840.3
|IOEthernetAVB!C	1.1.0
>usb.cdc.ncm	5.0.0
>usb.!UiBridge	1.0
>usb.cdc	5.0.0
>usb.networking	5.0.0
>usb.!UHostCompositeDevice	1.2
|IOAudio!F	300.2
@vecLib.kext	1.2.0
|IOSerial!F	11
|IOSurface	269.11
@filesystems.hfs.encodings.kext	1
>!AActuatorDriver	3440.1
>!AHIDKeyboard	209
>!AHS!BDriver	3430.1
>IO!BHIDDriver	7.0.6f7
|IO!B!F	7.0.6f7
|IO!BPacketLogger	7.0.6f7
>!AMultitouchDriver	3440.1
>!AInputDeviceSupport	3440.8
>!AHSSPIHIDDriver	59
>!AHSSPISupport	59
|IO80211!F	1200.12.2b1
>mDNSOffloadUserClient	1.0.1b8
>corecapture	1.0.4
|IOSkywalk!F	1
>!A!ILpssSpi!C	3.0.60
>!AThunderboltDPInAdapter	6.2.6
>!AThunderboltDPAdapter!F	6.2.6
>!AThunderboltPCIDownAdapter	2.5.4
|IONVMe!F	2.1.0
>!AThunderboltNHI	5.8.6
>!AHPM	3.4.4
|IOThunderbolt!F	7.6.1
>!A!ILpssI2C!C	3.0.60
>!A!ILpssDmac	3.0.60
>!A!ILpssGspi	3.0.60
>!A!ILpssI2C	3.0.60
>usb.!UXHCIPCI	1.2
>usb.!UXHCI	1.2
|IOUSB!F	900.4.2
>!AEFINVRAM	2.1
>!AEFIRuntime	2.1
|IOSMBus!F	1.1
|IOHID!F	2.0.0
$quarantine	4
$sandbox	300.0
@kext.!AMatch	1.0.0d1
>DiskImages	493.0.0
>!AFDEKeyStore	28.30
>!AEffaceable!S	1.0
>!ASSE	1.0
>!AKeyStore	2
>!UTDM	489.120.1
|IOSCSIBlockCommandsDevice	422.120.3
>!ACredentialManager	1.0
>KernelRelayHost	1
>!ASEPManager	1.0.1
>IOSlaveProcessor	1
|IOUSBMass!SDriver	157.140.1
|IOSCSIArchitectureModel!F	422.120.3
|IO!S!F	2.1
|IOUSBHost!F	1.2
>!UHostMergeProperties	1.2
>usb.!UCommon	1.0
>!ABusPower!C	1.0
|CoreAnalytics!F	1
>!AMobileFileIntegrity	1.0.5
@kext.CoreTrust	1
|IOTimeSync!F	840.3
|IONetworking!F	3.4
|IOReport!F	47
>!AACPIPlatform	6.1
>!ASMC	3.1.9
>watchdog	1
|IOPCI!F	2.9
|IOACPI!F	1.4
@kec.pthread	1
@kec.corecrypto	1.0
@kec.Libm	1









~-~~-~
### Richard Feynman Quote
Richard Feynman said something about scientific integrity that would echo through generations: “The first principle is that you must not fool yourself—and you are the easiest person to fool.” 
~-~~-~
### Signup flow.

- Download; launch; position camera
- Get name and headshot pic
- Getting on site
- >> https://blog.zoom.us/cloud-based-and-peer-peer-meetings/
https://www.theverge.com/2020/7/15/21325542/zoom-dten-me-tablet-video-calls-webcam-home-remote-work

~-~-~
### QL notes
- not sure if I am on the right track or not
- voice wavering.  too many 'ums'
- trying to remember my stats.

~-~~
### Parents OWE me
  $140 = 1/2 of $280 for bed.
  $30  pill cutter
  $8	Sunflower seeds
  $20	Can Opener
  $7	Staples
  $9	Sand screen for drywall
  $11	Dry wall screws
  $250	Handrail
  
  


~-~
### Brodie discussions
BEDROCK == assertions broadly accepted by the nearly the entire population.


e.g.  Trump is president.  (Bedrock).       Trump mishandled corona outbreak.    (Not bedrock, but treated as a fact by CNN)


I have two claims:  
A well functioning democracy needs bedrock.  
America no longer has the bedrock it needs.  

~-~~


There are two distinct ideas:

— we could build a mechanical means of providing an objective measure of media bias.

— we could improve partisan discourse by automatically elevating well argued well defended versions of positions, and 
     Place these positions within the web of assumptions and counter positions that exist around then so as to let anyone quickly get an unbiased summary of best citizen thinking on a topic.

~-~~






Nick,
What do you think of these approaches?
No need for you to polish it.  Just getting your take on the overall strategy.
—d


_
### === HENRY SCHIEN === 

Bruce:

If you recall we had a couple of really interesting conversations last year about innovation in the dental space.  One of the ideas that really caught my imagination was the auto-restocking AI system!  This really is the magic moment for this kind of technology, deep learning and cheap GPU technology make this a robust and cheap capability.  It is no coincidence that Amazon Go is happening now.

Since we have a specialty in vision processing, we did a bit of internal work on this idea.  I think we need to explore the variability which will occur with flexible objects like gloves, and segregation of a live video feed into person (to be excluded) and object (to be detected), is going to be some really novel stuff, which we have yet to explore.  Still the whole direction looks really promising!

If you still have interest in this problem, I would love to get your take on the work we have done.  And of course since our companies are so complementary in strengths, it is always possible that partnering would be a great thing to do too.


Let me know if you have interest in catching up next week.

Cheers, 
Dan
_
### Henry Schein

Bruce, Sandy, Mike, and Michelle,

Thanks for taking the time for our Thursday call.  There were some exciting ideas that came out of it.  I am especially pleased that we had ideas coming from all different directions.  I think this is best way to find the ones most pressing for action now.  Here are the ideas as we understood them, as well as a few thoughts about what the next steps might be on each idea for your consideration in your subsequent team meeting.


~-~~-~
**REMOTE DEVICE DIAGNOSTICS AND FORWARD-LOOKING FAILURE PREDICTION**
- PRO:  We love this problem, it marries our strength in Machine Learning, IOT, device integration.  
- We could take on any/all of these pieces and also could build on top of a device connectivity solution that you are considering for licenceing.
Key to success here will be building a cost effective method to onboard a huge number of devices most with legacy interfaces.

**ASSESSMENT STEP**:  Digging into the data from one or a few key devices is probably the best way to get an early sense of the possiblities and difficulties with this path.
TYPICAL BLOCKER:  Often getting raw dumps of data can be done with relative ease, but then getting data prior to failure with the failure indicated in the history is harder.  (we can work with your IT team to see what exists today, and if we can make an assessment with exsiting data.)


~-~~-~~
**PASSIVE STOCK MONITORING AND AUTO RESUPPLY**
- As a DARPA program manager, I just *love* this problem.  It is aiming at the tough stuff that one really can move the needle on results doing what others have yet to do.  AND you told us that it had huge potential to improve revenue!

- CON:  But often problems like these have not been done for a good reason... becuase they are too hard to do well.  this one might just fail on technical grounds.

Still I have a good feeling about this problem.  In your context it would be worth installing some cameras and assets, so I could potentially hit it with a big hammer!

**ASSESSMENT STEP**:  This one is a roll up your sleeves and think kind of problems.  
- I think a single phone call dedicated to digging into the constraints on this use case would be enough.  
- I would then dedicate some shower time to considering how to best attack it, followed by some academic lit review to find out what levels of efficacy have been acheived doing different parts of the problem.

Doing a small POC just to test specific reco in a specific context would not be time consuming to do, but best to just look at other peoples work thoughtly first.

No matter which of the other ideas we chase, I would love to chew on this one in my spare time too .... as kind of a moon shot project.  life is more fun when there is an active moon shot.



~-~~-~-~
**CHAT BOT APPLICATIONS**
PRO:  Lots of industries are finding value in the always-available, natural text chat interface.
CON:  But the devil is in the details.  Unlike a traditional interface with fields and strcuture that allows a user to know if its context is correct, a chat bot just has an empty box where the user can type ANYTHING.  without proper framing as to its usage, the user will be frustrated that what they wanted was not properly covered, and they dont' know what IS covered.

ASSESSMENT STEP:  One or more conversations about specific targets use cases would be needed here.  If we zero in on a very targeted use case or two, we can have have a strong result.  

For example, I can imagine a chat bot that has access to prior purchases and the vendor ID for an incoming chat, then it has a back and forth coversation about resupply.  Perhaps at the end of this there is a human validator that sees the entire back and forth as well as the proposed order before proceeding in cases where the chat bot had texts it did not understand.  This hybrid might allow one worker to hundreds of orders per hour at a very very high level of reliability on order correctness.


~-~~-~~-~~-~-~
WAR ROOM "NOC"

The military organizes its operations into Network Operation Centers similar to the one you described on the call.  These man-machine hybrids are very good at taking into account and enormous amount of fast moving data and taking optimal course of action.  

PRO:  I have experience at DARPA, and even more building solutions at Analytics Fire.  Further our team is quite strong in building reactive bandwith efficient data processing interfaces so this is a sweet spot for us.

In your case it seems forward knowlege of device failures as well stock supply issues could really improve the overall effectiveness of your operational fleet.  I suspect there may be one or two pieces of software that one might licence inside such a NOC, but overall I expect it would be a proprietary Henry Schein solution that would gain the greatest value.
These man-machine combinations work best when the machine portion is highly optomized for the details of your specific use case details.  Trying to build from generalized compoents w/o specialized software aroudn the edges often leave the people pushing info from one interface to another. (Yuck).

ASSESSMENT STEP:  One or two conversations with a relevant product owner would be enough for us to propose a scope and an approach.  This seems an area where your internal team might also be strong.  So we are happy to take on parts of this problem in the context of a larger solution produced in house, and to hand those part quickly over to the internal team.



I thought it might be valuable to write up a bit of thinking from ourside to aid in your internal discussions.  Happy to re-engage once you guys have connected internally.

Cheers, 
Dan

_
### BOSTON SCIENTIFIC

#### Note

Brian, 


#### Summary


We have been approached by Boston Scientific to provide an interface that will allow clients who have gotten a medical procedure to upload videos as testimonials for marketing purposes.  We are leaning into this engagement since we hope we might end up with other work from Boston Scientific, and becuase the work has great human value is additive for our company.


Here is the proposed roadmap for the engagement:
(1) We writeup a proposal for this work.
(2) They select a vendor (we are not the only one) and confirm that the proposed $100K budget is available
(3) We perform a small amount of work w/o pay in order to:
    -- demonstrate our capabilities as an organization.
    -- demonstrate the core "capture a video for sharing" capability.
(4) They approve this up front capability demonstration, we finalize and sign the full contract.
(5) We move into a traditional agile sprint process with them, taking care that we remain in a postion to deliver the full capability without a large overrun.
(6) We deliver this app and remain engaged with Boston Scientific at a basic level in order to support the app as it is being used.  

NOTES:
- We are leaning into this engagement, and they are cash sensitive, thus we want to create a proposal that offers quite a bit of value for the dollar.  As long as we don't over run badly we will count it as a success.
- Ideally we have not blown all $100K by the time they are in deployment, since it is not clear now if he will be able to get more funds if the app is having strong adoption, but we will get value out of ongoing contact.
	- Ideally we can provide a contract with small and full versions of different add ons to the core offering, this way we can get credit for promising many things while not committing to exactly which ones we will actuall do.  That way they can choose during execution to trim the burn so we can stay longer, if they are not going to be able to get followon funding.  



Expected Scope:
- A solution that supports (1) recording, (2) uploading, (3) editing, (4) publishing, and (5) sharing of testimonial videos regarding the effacacy of the watchman implant:
	https://www.watchman.com/en-us/home.html
- The key value proposition for this implant is that it allows its user to get off of blood thinners that are required to avoid stroke.  They believe this product greatly enhances the quality of life for those who can escape these blood thinners, and so they hope to enable the current users to broadcast their success within their own personal networks.  The theory is that a testamonial provided by a person known to the recipient will have greatest impact, so this would be a way to grow their market penetration.
- Currently they have 80K who have had the procedure, and they would send invitations (or have case workers provide the link for the service) to some subset of this 80K members in order to increase adoption.
- Currently they have about 2% market penetration, so they are interested in anything that can move the needle on this penetration rate.
- The are extremely focused on usability within their target demographic, reduction of friction etc.
- Critical features: 
	- The ability to operate without needed any app download.
	- Allows an untrained user to easily succeed 
	- Ability to operate across the widest possible platforms, ideally including iphones, ipads, android, and laptops/web.
	- The lowest friction possible separating a potential user from recording, editing, publishing, and sharing a testimonial story.
	- A solution designed to operate wide range of less tech savvy users including older users without confusing them.

~-~ ~-~
Proposal should layout scope/timeline/budget for these pieces and addons.
We should do some good thinking about the right way to maximized diversity of platforms with minimum of alternate implementations, and with pretty deep consideration for the frictions involved with each choice.

REPORT MATRIX -- I think they will be impressed with us if we can give them a report that shows a matrix where the rows are different upload approaches and the columns are the covered user cases (e.g. iphone, etc.) as well as imporant considerations of realiability, friction issues, etc.

KEY FRICTION CONCEPTS -- I think they will be impressed if we do some thinking about tactics that we can approach the friction and usability.  E.g. do we use walk thru help highlights, do we have audiory help?  is there wizard help?  what tricks can we propose that would maximize execution success over the great range of most diffiult users?  (perhaps we even open that question up on slack general?)

Lets jump on a call tomorrow to clarify the plan and execution here.

Cheers,
Dan
### Todo
Corsica, CEMEX, Procor, TTX Corp, 
#### Rusty (Corsica) + <<<!!
Todo:  email to lock down conversation @ end of may
Todo:  learn about River Logic software and aims

They want to predict how many zipper covers to order

we talked about phone app on factory floor (but they have erp system with all the data)

If we could find a great Operations Research guy we could build a solution to make predictions.

rkennington@corsicanamattress.com

Rusty,
I really enjoyed our couple of conversations at the GDS summit, seems like your past bit of study and work with AI puts you a bit a head of the curve on understanding the ways one might leverage those kinds of technologies within the work flow on your factory floors, across your supply chain, and in modeling markets/demand.  I am sure the business of both of our day jobs are going to quickly take back over our lives so we should lock down a specific time/date for a followup conversation.  We mentioned the end of May as an ideal time.  My calendar is pretty open in that last week.  Why dont you just pick a time & date for us to work from?

Looking forward!
Dan


P.S. As a placeholder so we remember our disucssions I will summarize a bit here:
You mentioned that you now have much of your data being systematically collected within an ERP system, and that you as yet have not done very much modelling on that data.  Our idea was to look for opportunities for bringing in Machine Learning and/or Operations Research talent in order to look for low hanging fruit in doing predictive modeling.  You mentioned that pre-production of frequently produced components, demand modeling, load balancing of orders across warehouses, and prediction of zipper cover orders were some possible areas that we might consider.



#### Russell (Continental Resources) ???
(hadoop cluster)
Matthew Rorman 
upsteam exploring  Oil&Gas
was at Devon energy

Spoke about full stack IOT.  modelling guys.

Was extremely short

XXX


Russell,
This is "DARPA Dan", the PhD AI/ML/IoT guy.  Thanks for the chat at GDS.  We agreed to put a meeting on the books in the next couple of months to brainstorm concrete ways leverage/build out your IoT infrastructure

to leverage the data you guys are collecting.  You said the CEO is expressly NOT data driven, so I think the only kinds of ideas that have a chance of flying will have to be pretty closely tied some value that he emotionally "gets" -- fun challenge!  My schedule opens up after the next three weeks, maybe we chat on Tuesday May-21 at 11am PST, or you can pick an alternate and we can follow up then.

Looking forward,
Dan


#### Mike LaVista (Caxy Interactive) >>
In Chicago, 
Fin Services (trading/banking/heathcare)
higher Ed, Non-profit(pesudo co)
Mfg  (smaller mfg.)

competitor.

building SW systems.

#### Nate Thomposon (Primier Research) >>
great conversation  (Pharma reserach co)
he mostly builds in house.
Todo: send email about our abilities, but seems a miss.



#### Kevin (Citi) >>
masking data 

K
GDPR-ified their infrastructure.


#### (Tenet Health) healthcare +<<<??
Carolyn Campell

300 IT devs inside
mostly use 3rd party SW w. customization

medical records

Carolyn,

We spoke briefly 

carolyn1.campbell@tenethealth.com

Carolyn,
This is Dan, the PhD AI/ML guy.  Thanks for the chat yesterday at GDS.  We agreed to put a meeting on the books in the next couple of months to brainstorm concrete ways to leverage the medical record data you guys are collecting.  My schedule opens up after the next three weeks, maybe we chat on Thursday May-23 at 11am PST, or you can pick an alternate and we can follow up then.

Looking forward,
Dan



#### Lisa (Fairways) +
FIN:  The onus is on you to reach out with a use case that would fit with grocery stores.
TODO: Send email reminding her that we might follow up.

we have logistics software for trucks, and some for inventory.

CEO is expressly NOT data driven, he signs off on everything.


lkerns@farewaystores.com 

Lisa,
This is "DARPA Dan", the PhD AI/ML guy.  Thanks for the chat yesterday at GDS.  We agreed to put a meeting on the books in the next couple of months to brainstorm concrete ways to leverage the data you guys are collecting.  You said the CEO is expressly NOT data driven, so I think the only kinds of ideas that have a chance of flying will have to be pretty closely tied some value that he emotionally "gets" -- fun challenge!  My schedule opens up after the next three weeks, maybe we chat on Tuesday May-21 at 11am PST, or you can pick an alternate and we can follow up then.

Looking forward,
Dan



#### Bruce Schinelli   (TTX Corp) ???   <<<!!   nice note.  

Use IoT on trains.  aim is first to optomized maintance, but then later could optomize rail yards, etc.
But those are owned elsewhere so not easy inroad.
expect to use off shelf sensors, off shelf IOT layer, and custom ontop of that.



#### Murshid Khan (TDECU) +  <<<!! Fraud data modelling brainstorming....
TODO: Follow up to lock down a time to talk about his roadmap on fraud & on 

fraud detection
uderwriting process
new members
morgage lenders 
Fraud in our Automating Customer onboarding.

mkhan@tdecu.org

Murshid,
This is "DARPA Dan", the PhD AI/ML guy.  Thanks for the chat at GDS.  We agreed to put a meeting on the books in the next couple of months to brainstorm ways to leverage the data you are collecting, I recall fraud detection at many levels was top of your mind, but that was just one of the directions we talked about.  My schedule opens up after the next three weeks, is there a time that works well for you?

let me know,
Dan






#### Robin Atherton (PROCOR) +  <<<!!  Data Science Brainstorming 

>>>>>>>> TODO: write email tonight to find a time to chat about 

Maintanace Car

atherton@procor.com

Robin,
This is "DARPA Dan", the PhD AI/ML/IoT guy.  Thanks for the chatting at GDS.  We agreed to put a meeting on the books in the next couple of months to kick around ways to leverage the data you are already collecting for car maintance, logistics optmization etc.  My schedule opens up after the next three weeks, maybe we chat on Monday May-20 at 11am PST, or you can pick an alternate and we follow up then.

Looking forward,
Dan



#### Walid (Tire) + <<??
STAT:  Agreed to chat about ways that he could use his data.
TODO:  linked in tonight to find time.

SCOP - supply chain optomization
mfg optomization
marketing optomization

wasi.ahmed@yokohamatire.com

Wasi,
This is "DARPA Dan", the PhD AI/ML/IoT guy.  Thanks for the chat at GDS.  We agreed to put a meeting on the books in the next couple of months to kick around ways to leverage the data you are already collecting for optomization (supply chain, marketing, mfg processes, etc.).  My schedule opens up after the next three weeks, what would work best for you? 

Looking forward,
Dan



#### Romeo Siquior (CEMEX)  <<<!!

Todo: lockdown call w. Romeo his buddy, and head of XXX in ontario.

Need: predictive analytics around billings, collections, ...
Need: fancy vision/AI system to notice in real time that someone is dosing off.


romeo.siquijor@cemex.com

Romeo,
I had a great time chatting with you over the whole of the GDS conference, thanks.  We agree to put a call on the books so we could talk about a couple different ideas that might go in front of the shark tank at some point (what a great internal process!)  I think front and center was the use of predictive analytics around billings and collections.  What is a good time for us to connect?  




#### Prabakar Kasu (Aon)   @aon.com  <<!!!

High caster.  immediately started selling me on "other" deals he could put together.... shared IP

Talking of investors he could bring on.   Partner consulting companies we could be connected with to get new clients with.

He talked about joining us, being in front of customers, evagalizing, blah blah.

TODO: Send him short deck.  6 weeks later have collaboration discussion.
### Tarick Fadel  (Alberta blue cross) +  <<??

Two:  
-- high velocity dashboard w. quick handoff
-- ML/Deep Learning application

TODO: Follow Up to get a meeting on the books.

tfadel@ab.bluecross.ca 

Tarick,
This is Dan, the PhD AI/ML guy.  Thanks for the chat yesterday at GDS.  We agreed to put a meeting on the books to brainstorm ML/Deeplearning ways to leverage your client data as well as your marketing data.  My schedule opens up after the next three weeks, maybe we chat on Wed May-20 at 11am PST, or you can pick an alternate date/time.

Looking forward,
Dan
### Sergey Razlivkin (Dito)  >>>
He is a GCP solutions impl company (google partner).

Will introduce us to a partner at E+
E+.com seems to be "lockheed martin" style integrator

TODO: Followup w. Sergey and get intro started.
### Douglas Wilson (CK Telco) +
Spoke at elevator as going to bed.

they are gathering data on mgf for thickening agents... might use it predictive analytics.

they have IT in brazil.... so they are cheap, and the are laggards.  Still maybe they would bring in a Data scientist team to provide targeted predictive analytics.

TODO:  send email... aim for followup call now or maybe in months....  see if they have collected the data.

(MAYBE, this is a lower value lead)


doug.wilson@cpkelco.com 

Douglas,
This is Dan, the PhD AI/ML/IoT guy.  Thanks for the short chat at the elevator on our way to our rooms at GDS.  We agreed to put a meeting on the books to brainstorm a few months out too see where you are in collecting data and brainstorm low hanging ways for a Data scientist to get quick value from your data.  Maybe Mon July-29th at 11am, or really any day of the week that works better for you.

Looking forward,
Dan


#### (CVS)  >>>
-- Need conus employees (not for all things)
-- budget nailed this year
-- 3:1 outsource to W2 work
-- many internal datascientists and believes that    
   vertical specific guys are better (so no room there)
-- very focused on safety critical
   (even 99.5% is not good enough when giving perscriptions)

-- I might not be the right guy to talk with in our org
-- we only have a few simple IoT things.

Todo:  he said 'we will connect' as he was pulled away but not clear.

#### Yuri A Campbell (US ARMY)  yuri.a.campbell.mil@mail.mil  + <<<!!
He heard me talk about medical robots
he does AI and robotics and was interested.

TODO: have a followup call at least
he is very interested 


chani.a.cordero.mil@mail.mil

Yuri,
Glad that my voice carries, and you overheard my talking about robotic surgery on the last day at GDS.  You mentioned that you were standing up some kind of workshop on AI and remote medicine (if I got that right), and that you were also generally interested in driving/funding initiatives around this area.

We agreed to put a call on the books to keep the conversation moving.  It's not clear to me, if your work on the initiative is still in its early stages, maybe we should just put that call on the books 3 months out.  Otherwise we could have it sooner.  What is your thought?  (and feel free to just pick a time/date too)

Thanks,
Dan



And we didn't get to talk about it too much, but I was a Program Manager at DARPA, have PhD in ML/AI, and now have a consulting company focused in the area.  A couple of notes about us to remind you later:
-- We helped build a mobile robotic system capable of dexterous gripper manipulation
-- We currently manage 1/2 billion in IOT assests.
-- developed SW use on a "fly-by-wire" surgical robotic system.

We didn't really have time to get into details on either side, so lets put a time on the books now to have a longer talk, and am quite free the end of May beginning of June, so if you like you can just pick a day and time.
### Lockdown
-- Sue (Gates), Rusty (Corsica), 
-- Ed (Pharma)
-- (VMware)
## Notes
### Roundtable 1
Ed (Pharma) worries about compliance
Marvin (Eli Lily)  data-silo because of sesitivity.  20% time ad hoc granting.
## G-next


Analytics Fire is a team & approach for building custom AI & IoT software solutions. We specialize in building safety-critical & business-critical systems where precision is key. Our custom solutions are automating over $3 billion in fleets of connected IoT devices.



http://aeris.com

Tom,

We spoke at length on Tuesday at G-next.  Did you manage to get away from your booth enough to see the work we did for Sun Power -- I hope so, it is always nice to get a 3rd parties assement of work done.

You told me that Aeris is trying to pair back their partner program to a smaller number of higher quality partners.  We both thought that Analytics Fire might be one of the those higher quality partners.

As we discussed I would like to setup a time for my co-founder and I to chat with you about this.  Perhaps a short intro call will orient us, and understand what (if any) further actions should be taken.  Let us know if that sounds right to you.

Best,
Dan Oblinger
Founder/CTO
Analytics Fire


~-~-~
http://ironmountain.com 

Anke,

Did you happen to get a chance to go over to the Sun Power booth to see our Clients Deep Learning solution for designing Solar Panel installations?  We are proud that our client chose to highlight our work among all that is happening this year for them.

If you recall, we are a boutique Machine Learning solutions provider, and we spoke about the possiblity of partnering on work directly for Iron Mountain, or for its many clients.  My DARPA background in NLP seems like a great match for your document understanding technologies.

You had suggested that a phone conversation with Mark would be the ideal next step.  Is that right, should you and I have a quick call to get more details first, or something else?  Let me know your thoughts.

Cheers, 
Dan Oblinger
Founder/CTO
Analytics Fire


~-~~-~~

http://google.com

Josh, 
The chat at G-next was fun!  I am sure it was a blur for you...   I am the guy that created the DARPA program that lead to IBM's Jeopary play system, created a 60-person robotic team aimed at autonmous manipulation, and had debued our GPU-optomized, geometric reasoning + deep learning system for building consumer roof designed for solar installations.... that guy :-)

My sense is that we are pretty specialized right in your area of expertise, so having a basic understanding of use would let you reach out when Google had need of a partner like us.  We talked about connecting up when you are free and I am next near Google.  Should we put something on the calendar now, circle back in May in order to pick a specific day, or something else?  Let me know your thoughts.

Cheers, 
Dan Oblinger
Founder/CTO
Analytics Fire



~-~~-~~
http://hcltech.com 

Sarat,
Thanks for taking the time to chat at G-next.  I am the guy that was the founder of an analytics company that is managing .5 Billion in IoT assets, and built the custom GPU-optomized, deep learning solution that Sun Power debued at G-Next.  (did you happen to get away from your booth long enough to see that system in action?)

During our conversation you suggested that connecting us with the program manager in your IoT works group would be an ideal next step for seeing if we could provide value internally, or in partnerships.  Independently, you suggested getting connect to the Digital DNA group.  We are excited about both!

What is the right next step here?  Should we have a followup call with you first, or directly with these groups?  We are happy to proceed in any way you see fit.

Thanks,
Dan Oblinger
Founder/CTO
Analytics Fire


  
  [SNIP](__SNIP__.md)
 [G-doc Scratch](https://docs.google.com/document/d/19rapJqTzTSeLPrQPIRligaFna5yc44p-gDhdhlOJLdI/edit),  [[SNIP|Snip]],  

  [SNIP](SNIP.md)






  







