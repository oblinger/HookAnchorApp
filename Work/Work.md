.[[Work]].
  ,   [[DT]], [[SV]], 

.**NOW**. 
  [[2024-08 Backend Architect|Arch]]:	Wait, Agents
  [[DT]]: HomoData,
  [OKRs](https://docs.google.com/presentation/d/1v7Winy9DHxGuYh05_is8zH05ClJi0qclM698-pDXIgY/edit?pli=1#slide=id.g274b3e27e6b_0_0):	Q3 Planning	
  [[DAT|DAT]]:	Get PR ready
  [[FIN.]]	Invest plan; Retire plan(spreadsheet)
  [[NJ]]:	>[[War|War]], rewrite, mentor

recheck where we are
## TRACK
[[Data]]:
Data:		Build the stats dataset
. 			GCP, Halos, Minutes, Colors
[Bridge](https://docs.google.com/document/d/1oaG7NB2fJ9THAtHcREX6yqrWg91wYEsof9zQNEInFFU/edit):	CrossVideos, 
GCP:		^Juan tracking this
Halos:		Grz: Halo videos, 
[Min](https://docs.google.com/document/d/1oaG7NB2fJ9THAtHcREX6yqrWg91wYEsof9zQNEInFFU/edit):		!! On production.
VidQ:		^Neg entropy gallery
GCP:		[Brian plan](https://docs.google.com/spreadsheets/d/1W38uHew6aaqyeolKzbO9NbaA96Q-jzyDb7gYoUst0qw/edit?gid=0#gid=0).  Pricing/Perf of using S3

SW proc:	[db](https://www.notion.so/sportsvisio/cf65339c09c6486296f8adaa084c141a?v=f8a45e385eea4c2fb590502835f4453f&pvs=4),[doc](https://docs.google.com/document/d/1aHnoqgQ3H_0abUtlVWoMPHDA7nQhzoq0JRqZLujORc0/edit?usp=drive_web&ouid=105281685541312342727)  ^Jasu 
Assess	> Assessment plan for BB

## SOON [[mgt]] [[CVP]] 
[[2023 CV team new agenda metrics]]:		/Create money metrics page
[[DAT]]:			Dynamic Dat loading
[[journaling]] 


## PAUSE [bsheet](https://docs.google.com/spreadsheets/d/1C1eYv2Ks2GCCQ9_xZ2Pg7eEr33bX1KOdN8Xf8X6nxSY/edit#gid=0), [hi](https://software-engineering-team.atlassian.net/jira/dashboards/10022), [tst](https://docs.google.com/spreadsheets/d/1Rc07oadsjZgg6_2ZZ-qIXJ9n9ws3LT0FFraLZHnrf9w/edit?usp=sharing), [epic](https://software-engineering-team.atlassian.net/jira/software/projects/MOB/boards/24/timeline), 
[[patents]]:	^J Provisional filed
[AI_mat](spot://aiprodmatrix):	Populate product matrix
[RP](spot://rp):		^TestRecall; FixRecall
[Belief](https://docs.google.com/document/d/1H9krXpxS2_Cr9T68pAksb6QGYqpW28DiF2OSw40g0pI/edit?usp=drive_link):		
[[@Shreshth Saxena|Saxena]]:		/Grz Results  
Fireballs:		^Dispatcher; ^Maxim 
[[SV]] 
[[2023 Least Commitment Model For CV Pipe]]:			>do & metrics
[Pivo](https://docs.google.com/document/d/16ee8QGaTCChGW7TC3k0W0sQNyyByygOGzwIw_aUkkpU/edit):			!! timeline
[T28Enhance](https://docs.google.com/document/d/1uOdu0gDLiMXK0d_GQ7kr874PBHVgssaY8D521cvdtQw/edit):	> tell team
   -- 3on3 games
[Assess](spot://assess):		>look
[[Work/SV/CV/Data/Data]]:		Pie-chart
[Cost Analysis](https://docs.google.com/document/d/1WYVwgCfHYk6wC8pGFo68t_vAju_oczbSeBTS-zba-XI/edit?usp=drive_link):	Cost Per Game Analysis
[[Work Jump]]


@Scott Byers / @matias:  We are thinking of a version of halos where we would dynamically recompute the halos depending upon the roster as dynamically edit

Homography  (physics model)

- Ask Greg what was his idea for testing if basketball stats will ever work.  give ground truth

- inst templated folder
- cube construction

## _
































phy 

In slack
9 EST - 10 EST.

Scott:
- PRs early in morning or after 6pm EST.
- 



Tactical meeting:  SW / CV 
Sales/Mkt.  Ops.


https://www.elastic.co/blog/elastic-universal-profiling-agent-open-source



CURRENT PLAN:
- For reference, assuming that accurately detecting game breaks turns out to be relatively simple and robust, then the current plan is to work in AI minutes into production and into scaled usage before trying to push shot boxer or other AI tips into production.
- Sometime in May/June we do anticipate testing shot boxer and shot charts as an experiment to see the value we get.


The challenge regarding the math for shot boxer is the cost of compute in that equation.  If we go to scale with shot boxer a very very rough guess is that we are currently as $6 - $9 per game to provide tips.  Again, very rough maybe that can be brought down to $3/game.  If these very rough numbers are correct, then it seems it might eat up much of the profit we get from speeding annotators.  No longer term if we stack up many speedups we might save more money and still only cost $3/game, then it become more interesting.  But as it stands if does call the math into question.  (Still before I really believed these numbers I would want to do a real analysis of the costs... not just ask the team to spitball, which is what those numbers above are.)