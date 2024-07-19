### 2024-07-18  Sam - GCP->AWS


Sam,
Jason, Brian and I were discussing the AWS to GCP transition, and since money is involved I wanted to bring you into the discussion.


I have not been in very many discussions about this transition, so I may not understand all plans we have in this area.  But I do recall a discussion we had two management meetings ago about this transition and I think our reasoning at that time is still pretty relevant to our present situation:


In that meeting we noted that because our AI products are GPU heavy they alone will chew up 100% of our Google credits before the end of the Google contract.  Thus it will not save money to transition other aspects of our SW stack over to Google.  I think this logic still holds true.  Of course we can also get Azure credits and maybe one or two more.  But if we are scaling a successful AI-minutes product I anticipate we will be chewing up those credits even faster.

Indeed, it might be valuable for us to still be burning some free credits somewhere even at the A-round as this lets Jason talk about some parts of our compute costs in the future tense rather than in present tense, then at the same time he is talking about future code improvements to lower those costs.

~

But the burn before the bridge round has changed this calculus:  Right now we WOULD spend some extra developer dollars to shift costs that we WILL have to pay 6 months from now on AI-GPUs into credits that we can use TODAY to cover current AWS bills!  This wont decrease the total compute dollars spent, but it will change when when have to spend them.


On the AI-side we are transitioning our experimental code so it can run on GCP this week and it is going well.
We can avoid running many AI-games on the AWS stack before the connection to SW is transitioned over, so we can keep those costs low too.

I believe Brian is working on (or already has) a rack-and-stack of components we can shift from AWS, along with a qualitative estimate of the dev-cost and dollar-benefit of transition each component.  My thought is that we pick the top item from that list and push it thru before starting on the next one.  That way we get early gains from each of these items, and it also keeps us focused.

Since this is cost-shifting rather than cost-savings, we may well opt to not do all of them--this is a decision that we can make over time.


Let me know if there are issues with this plan, otherwise we will make it so.

--dan


### 2023-05-04  increasing MM-SV overlap

### 2023-05-04  Friday production push

TOPIC: FRIDAY PUSH TO PRODUCTION

@Brian I am reposting this topic in the war room so all can collaborate, but the question is foremost to you.  Given all changes now on stage, what level of testing do we need to do tonight in order to push to production tomorrow?  @mark can get 5 folks I think he said to each annotate a game.  Cloud Factory can annotate games as part of their process over night, this likely gets us another 8 games (is that right Mark?).  And then we can divert MM people away from delivering Games to Cerebro for the evening (we want to minimize this).  But how many MM people / games do we want to divert?  (he has 40 total, so we could divert 5 people for example, is that en ough?). Those who are diverted are still doing productive work, but their results will not go to Cerebro the next day.  What do you think we need?
k