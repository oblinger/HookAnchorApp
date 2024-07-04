### 2023-11-07  Notes never sent or OBE


Jason,
I think I came off in our conversation as pushing back on your core points.
But really I am in agreement with your core points:
**—>   The CV team suffered for my lack of attention while attending to issues in the SW team.**  

Overall I think this is right.  There are no easy answers over there, and Michael is a smart guy who aims the team.
~~
  
But he does not operate like the life and death of our company is tied up in the velocity of our CV team (which it is!)

So it’s not the case that I (or you) can dive in and quickly “fix” the CV team’s execution.  We can’t, still by being in there 

In the weeds on a day by day basis.  In dozens of ways either you or I could just ask.  “Hey couldn’t we do this instead”

  

And that would shave a few days here and a week there.  But over and over that stuff really adds up to a huge velocity difference.

  

  

—> Sometimes the CV team is just thinking about things the wrong way, and a fresh perspective would help.

  

Again this is right.

DEV TEAM FOCUS
Last sprints we have been focused >90% on bugs, and finally are turning the corner.  This sprint I hear we are >90% focused on new features, and last week we had ZERO external field testers test the app.  I am concerned.

As Mark Lichty points out, customers often wont bother to report and error, so we cannot assume they don't exist.  And I am certain there still are a number of high priority bugs.  As an org we tend to jump from one fire to the next without putting the last fire out and I think this hurts our stability.  Further, having customers find our high priority bugs for us hurts our credibility.

One thought here is that we keep a floor on our bug fixing time.  Maybe it is 1/3 time, keep struggling to find enough bugs in order to spend 1/3 of the sprint on bugs until QA, James, Jack, and Mark say... we have POUNDED on this app and cannot find any more bugs.  After that point we should have very few latent bug finds since most bugs were removed.

Perhaps the team feels we are already at this point?  Maybe, but I would love to hear the evidence.

And finally I will say, the decision to shift our strategy on bugs should be an explicit discussion happening at the management level, its not a low level tactical decison, it is strategic for the company.





Mark, 
You have not spammed me yet!  I am surprised you can get the tooling done for such modest amounts of money.  I am down in principle for getting one of these beasts and being on the very bleeding edge of chair technology.  (usually I want to wait for a generation or two for the product to get mature, but I think my health should not wait.). Couple of questions:

- Will I need to pay for a recurring fee for an app to use it?
- I think I may want to code a controller myself, I suspect I will not be able to get my desired behavior from the standard app.  how crazy will this be?  (These days I seem to have so little time for such play)...
- What is your best guess for how long this chair will last, and what will give out?
- 
dkdjf



Conversion from web - share button
Google ads - smooth process - 
- link game id with 
==> Don't know games are there
==> Loosing them on the product - don't know where there game there
==> Difficult to onboard
==> deep links don't work if app not downloaded.


League
- points per game for division-season / player 
- win / loss / win percentage


Program ->
	Division -> 
	    Events -> 
			Teams ->
		        Game ->
			        Player ->
			            Points

League(Division) level rollup
- TEAMS - win% streak Win/loss count ranking
- PLAYERS -   point/rebounds/assists  /per game


Add Game
	Player
		Points




[@Jasu](https://sportsvisioworkspace.slack.com/team/U04CKE0AZ39) / [@Brian French](https://sportsvisioworkspace.slack.com/team/U01PU20H56G) when we finally get to the bottom of this highlights showing issue, it will be great to post back to this channel (1) if this was a regression - I am guessing it was a regression.  and (2) if the regression occurred with this Friday push - again I bet it was.  If both of these are true, then [@Mark Lichty](https://sportsvisioworkspace.slack.com/team/U02DMMHDX5F) [@Jack Ryan Potvin](https://sportsvisioworkspace.slack.com/team/U03J2R27DUZ) [@Brian French](https://sportsvisioworkspace.slack.com/team/U01PU20H56G) this is some evidence that we should resist the temptation to push before the weekend in order to get a desired feature (last names) live.  (I am not calling anyone out as having done a wrong thing.... I am just trying to learn what our best practices should be).  Whatever we learn, this is a very interesting case!








inbounding 
ref image. (hand above the head - works for those that need min)
whistle
ball/player


MOST CRITICAL FEATURES FIRST
Yesterday we agreed that the hoops app becoming stable enough that we can start doing some highest priority smaller system tweaks in order to support sales and operations.  (hooray!)  I proposed breaking these tweaks out into the following groups (though this breakdown is very much up for discussion/change):
1. SALES - ONBOARDING - Onboarding is a key sub-area of sales support that Mark called out, so for now we should probably maintain a dedicated sublist.
2. SALES SUPPORT - Other small features that aid selling/closing
3. RETENTION - INGEST - Ingest was identified as a key usability (retention) related feature, this key-sub-are  
4. RETENTION - Features that improve customer retention (the user experience associated with our system).
5. OPERATIONS - Features needed to support operations and customer success.

It was surprising news to both myself and Jack that Mark listed customer onboarding frictions as the number one impediment to closing new sales!!  Thus for the time being, I created a special onboarding sub-section of sales support for the category.  As a soft informal prioritization I proposed Sales-related features above retention features above operations features (though of course this is just a general heuristic, not a hard rule for Jack to follow)

Jack/Mark I assume eventually these features will end up as Jira tickets.  Perhaps you organize them as epics with a growing backlog organized to express your prioritization of each?  Or maybe some other way?  In any case it seems valuable to capture the top ten or so in each category as text titles, just so we are sure to be working on the most important stuff first.  [[HERE]] is a simple google doc we might use for this purpose (or any other way you might want to capture this).


OPS VS SW
I proposed that for velocity sake we should strive to first use the ops team to provide many capabilities that get rolled into SW over time.  For each critical feature that we propose for our team, it is critical to ask ourselves if it is at all possible to handle the feature using our ops or customer success team.  We should get creative with it!  Many times are are ways which are workable even if inferior.  Here are the ticket designations I imagined for each ticket:
- OPS-FIRST - There are no large user experience reasons when this feature could not be implemented via the ops team first.  (though it may not be cost effective to leave it there)
- OPS-INFERIOR - This feature can be implemented using an ops-based approach, but the result is dramatically worse than having a SW based solution from a user's perspective.
- OPS-IMPOSSIBLE - These functions are just not possible to implement AT ALL, using an ops/success team.  SW is the only possible approach.

Generally we will use op, for ops-first and SW for the ops-impossible cases.  For the ops-inferior we would either live without the feature for a period of time then implement in SW, or we would implement with the ops team and switch to SW when we could