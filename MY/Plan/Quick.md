- [ ] Call Hassler; Urologist; 
- [ ] taxes plan
- [ ] pull trigger on iOS contrators (CreateThrive)
- [ ] finish doc for exportable stats

- [ ] - [ ] snack list

- [ ] - [ ] Jira exportable stats https://software-engineering-team.atlassian.net/browse/WU-64
- [ ] update calendar
- [ ] - [ ] Call Trees who gets called
- [ ] - [ ] look at schedule for conference   https://sportsvisio.atlassian.net/wiki/spaces/SP/pages/21135361/March+SV+All+Hands+Planning+Management+Only+Version+Script
- [ ] iOS dev choice
- [ ] look @ tournament flow from Jack   https://docs.google.com/document/d/1U4Ex458F2EYhMWpI2vnqUlovS82FzJgmkD5UaGhWVbM/edit
- [ ] extra $75

- [ ] 
- [ ] Didn’t want to keep hammering on questions about Cerebro in public, so I will ask it here.  Understood they don’t do or care about video…  but in my experience companies can become VERY VERY elbows out when it comes to customer lists.  Not just because they worry you will somehow steal business, but also because they worry you will damage their hundred plus million dollar brand by association with some small-Co’s unproven business.  At PayByGroup we spent more than three years on an active relationship with VRBO before they would allow our name to appear directly to their customers… it was all about their brand.  So in addition to telling you that they don’t care about video, did Cerebro also indicate  “sure, we could put a ‘click here to see your video highlights’ on our stats which links to a Sports Visio app.”   I am just triple checking on this point as it bit me so hard in the past.  But if that is possible, it could be a super exciting and rapid path for adoption. 

- [ ] 
- [ ] - [ ] find out about getting money exchanged
- [ ] Vijaya email. 
- [ ] https://sportsvisio.atlassian.net/wiki/spaces/_/pages/21135361
- [ ] Ground Truth stats calculation
- [ ] jason todd roos
- [ ] Prescious 415-502-7305
Todo / InProgress / Dev / QA / Stage / Done  

- [ ] Kathleen
- Argentina - Rent car?
- priority on bug board  
- click this link: https://meet.google.com/cvy-ognr-eey
Otherwise, to join by phone, dial +1 607-414-3060 and enter this PIN: 867 884 136#
To view more phone numbers, click this link: https://tel.meet/cvy-ognr-eey?hs=5


- Final federal
- Final Utah
- final 
- K1 goes into both fed and state in 2023. 


~~
Argentina help:  My understanding is that money is “funky” down south, and we should NOT translate money from USD into the Argentine dollar, and when we are there we should try NOT using our own credit cards.  Is this all correct?  And if so, should we just be showing up loaded with USD greenbacks?  and you guys are going to shuttle us to some guy (in some dark back room :laughing:) who will help us convert them?   Is that the plan?

And are there any other things for us to get / bring before we show up in Argentina?

~~







TIME DEPENDENCY & DATA DEPENDENCY - Once we introduce the idea of editing stats at multiple levels the interaction between these edits becomes truly mind bending to get all corner cases for the exponential number of temporal orders that these edits can be applied and later edited and removed.  This will be a source of never-ending bugs if we further cache computed data allowing the possibility that computed results will differ depending upon which part of the caching occurred before/after the subsequently applied edit.

Further the semantics for how these potentially conflicting edits override each other can easily cause the behavior of our App to become very hard to understand.  The user may have difficulty understand what other things will and wont change and consequences of their edits propagate in unexpected ways.

KINDS OF EDITS FOR CONSIDERATION
- AI ANNOTATION - Annotation computed by our AI system.
- APPROVAL - An approval / rejection of AI-annotation by a user.
- MANUAL ANNOTATION - A user-supplied game play annotation.
- IMPORTED STAT APPROVAL - Coaches might opt to accept annotations / edits players make on their version of the game into the master game record.
  (Motivated players may not want to wait for the coaches to fix their plays if they want an accurate rendering of their game they might do it themselves.)
- INTERMEDIATE STAT EDIT - Game stats sometimes are derived from other game stats.  (For example the number of points in a game is the sum of all the points made by all players in the game.)  Editing an intermediate stat could have indirect effect on derived game stats. (see intermediate stat use case for an example)
- TOP-LEVEL STAT EDIT - Top level stats provide summary for the whole game 
- CHANGES - Subsequent edits to any of these annotations / Updates of manually provided annotations / approvals etc.



PROGRESSIVE IMPLEMENTATION - This document attempts to capture the full generality of sports stats computation with multi-level intermediate user-based stats editing.   It introduces a lot of complexity that we don't need to deal with right away.  We expect we will only implement part of this full approach initially, but we consider all of this complexity up front in to avoid "painting in a corner" needing a rewrite, and also to head off a host of timing related bugs that will bite us if we don't think very carefully about our system's semantics.





USE CASE FROM HELL
This use case family jams all kinds of edits into a single game as a way of exercising the space of corner cases.  Much of the complexity stems from the fact that users might supply these many kinds of stats updates in different orders, and they might subsequently edit their edits also in differing orders.  This family of use cases attempts to layout a unified semantics for handling this mess.

In this annotation example from hell the:
- 321 shot attempt AI Annotations.
- 43 user shot annotations. The coach and two players add 43 shot annotations.
- 25 coach approvals. The coach approves 25 shot annotations that the two players added to be part of his master stats for that game.
- 1 intermediate stat edit was applied by a player 31 to increase his final number of baskets made by one to match that stats reported during the playoff game by the league, the coach approved this update.
- 1 final state update was applied to adjust the final score for the opposing team to lower the their final score to match the recorded finals score for the game w/o adjusting any underlying shot annotations.


INTERMEDIATE STATS UPDATE
- When manually increasing the shots made by player 31 from 8 to 9 should the system automatically update to total score for the game so that it is the sum of the shots from all players.
  (which could then be edited back down if desired)
- Alternatively the system might not make any other updates except the score for player 31, and leave the final score unchanged.  (so the number don't add up by default after an edit)

==> what do we want the system to do?


PROBLEMATIC ORDERING FOR CONSIDERATION
- SIMPLE EDIT - The coach adds an annotation for a missed shot attempts.    
  ==> This DOES update the intermediate game stats for that player, and final stats too.



LAYERED EDITS - DATA PROPAGATES
- The coach adds an annotation for a two point shot for player 31
  ==> The final score for the game increases 7-->79 points as a result

LAYEREDEDITS - DATA DOES NOT PROPAGATE
- The coach edits the number of shots for player 35   8-->9
  ==> This further increases game score 79-->81
- The coach adds an annotation for a two point shot for player 35
  ==> The player's score and final score are NOT adjusted since an explicit edit for player 35 is in effect.
- The 
- The player annotated this as



, coaches etc. will want both intermediate and final stats to be correct, but editing ALL stats will be tedious, thus in some cases they will prefer to directly edit intermediate game stats



~~

New kid on the block, and only one member of the MGT team;
So what follows is my opinion about Startup living and working remote.

~
Work harder for less pay
~

Hat and Heads / Heads and Hats
~

Team:

Jack alerted me tonight that he thinks the annotation tool that Chris is using will be valuable for the annotation team going forward.  He asked if I could develop a plan for this tool to be transitioned as Chris' last day is on Friday.  I am hopeful this will be an easy exercise.  Here is my thought for a start:
(1) Jack will own the task of gathering knowledge about how the tool operates directly from Chris and capture this in written form for out team.
(2) At the tail end of Michael's standup tomorrow, he can alert myself and Jack to join (assuming folks are free at that time) for a quick discussion.

Michael if that works for your team, you can decide if Vishal should remain, and if Brian should attend (if there is a chance his team would end up owning that code, this would be a good idea, else we can update him after).  And the end of your meeting is not ideal, just let us all know in this chat.  (FYI Michael's standup time is from 8-8:30 tomorrow)

