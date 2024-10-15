.**NOW**.
  [DT](spot://dt): 	^ [Homo](https://www.notion.so/sportsvisio/STORY-Homography-Data-Martin-4ab7ed5c5a6b46b1ac7de9821b3a5c76?pvs=4), ^ [BB](https://www.notion.so/sportsvisio/STORY-BB-Data-105719d6f2bd809d9170c84a0c8d762d?pvs=4), 
  [Min](spot:://AImin):	[Alpha](https://www.notion.so/sportsvisio/AI-MIN-Alpha-Testing-James-118719d6f2bd80bb8280fdeafac784f3?pvs=4), 
  QA:	[QAmin](https://software-engineering-team.atlassian.net/jira/software/projects/AIT/boards/25?selectedIssue=AIT-1069), [QAfb](https://software-engineering-team.atlassian.net/jira/software/projects/AIT/boards/25?selectedIssue=AIT-1070) 
  [Fball](spot://fireball):	>Matias DRW; recording
  Eval:  ^VALmin, !gt20>run>chk
  
  look at PRs
  P2P process
  [[2024-08 Backend Architect|Arch]]:	Brian
  [[FIN.]]	Schwab
  [[NJ]]:	>[[War|War]], rewrite, mentor
  GCP:		[Brian plan](https://docs.google.com/spreadsheets/d/1W38uHew6aaqyeolKzbO9NbaA96Q-jzyDb7gYoUst0qw/edit?gid=0#gid=0). 
  
## TRACK
  ^Preston 
  =GCP availability
 - patents
  [SW cort](https://www.notion.so/sportsvisio/HOMO-SW-integration-plan-77fb63f7cac7433bb0d2e2cd4d374e4b?pvs=4): Get plan
  VidQ:		^Neg entropy gallery
  SW proc:	[db](https://www.notion.so/sportsvisio/cf65339c09c6486296f8adaa084c141a?v=f8a45e385eea4c2fb590502835f4453f&pvs=4),[doc](https://docs.google.com/document/d/1aHnoqgQ3H_0abUtlVWoMPHDA7nQhzoq0JRqZLujORc0/edit?usp=drive_web&ouid=105281685541312342727)  ^Jasu 
  Assess	> Assess plan for BB

## SOON [[mgt]] [[CVP]] 
FILIP: ask him again?
[[2023 CV team new agenda metrics]]:		/Create money metrics page
[[DAT]]:			Dynamic Dat loading
[[journaling]] 


## PAUSE , , , ,
:	^J Provisional filed
:	Populate product matrix
:		^TestRecall; FixRecall
:
:		/Grz Results
Fireballs:		^Dispatcher; ^Maxim
[[SV]]
:			>do & metrics
:			!! timeline
:	> tell team
   -- 3on3 games
:		>look
:		Pie-chart
:	Cost Per Game Analysis

  DELS: [bsheet](https://docs.google.com/spreadsheets/d/1C1eYv2Ks2GCCQ9_xZ2Pg7eEr33bX1KOdN8Xf8X6nxSY/edit#gid=0), [hi](https://software-engineering-team.atlassian.net/jira/dashboards/10022), [tst](https://docs.google.com/spreadsheets/d/1Rc07oadsjZgg6_2ZZ-qIXJ9n9ws3LT0FFraLZHnrf9w/edit?usp=sharing), [epic](https://software-engineering-team.atlassian.net/jira/software/projects/MOB/boards/24/timeline), [[patents]], [AI_mat](spot://aiprodmatrix), [RP](spot://rp), [Belief](https://docs.google.com/document/d/1H9krXpxS2_Cr9T68pAksb6QGYqpW28DiF2OSw40g0pI/edit?usp=drive_link), [[@Shreshth Saxena|Saxena]], [[2023 Least Commitment Model For CV Pipe]], [Pivo](https://docs.google.com/document/d/16ee8QGaTCChGW7TC3k0W0sQNyyByygOGzwIw_aUkkpU/edit), [T28Enhance](https://docs.google.com/document/d/1uOdu0gDLiMXK0d_GQ7kr874PBHVgssaY8D521cvdtQw/edit), [Assess](spot://assess), [[Work/SV/CV/Data/Data]], [Cost Analysis](https://docs.google.com/document/d/1WYVwgCfHYk6wC8pGFo68t_vAju_oczbSeBTS-zba-XI/edit?usp=drive_link), [[Work Jump]], 


@Scott Byers / @matias:  We are thinking of a version of halos where we would dynamically recompute the halos depending upon the roster as dynamically edit

Homography  (physics model)

- Ask Greg what was his idea for testing if basketball stats will ever work.  give ground truth

- inst templated folder
- cube construction

## _


.[[Work]].
  [[SV]],
































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