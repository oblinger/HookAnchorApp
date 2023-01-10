=[[START]]    [Product & Strategy Exec](https://www.linkedin.com/in/franklangston/)  



# LOG
### m2022-11-05  Discussions before Reference call

- Group transaction-integrity needed to be 100% perfect.

- AUDITS - Specialized logging and dashboards so we could manually audit a group transaction.
- FORMAL PROCEDURES - Formalized and refined branch management & "push to production" procedures:
	- who came online when, what order the pull requests were merged, test on stage, quick fix vs rollback.  test on production.
	- hot fix procedures
- ARCHITECTURE - restartable.  
	- Ensured integrity even for transactions that span DB migrations
TESTING:
- MANUAL - Build & maintained documented manual testing steps designed based on code coverage analysis.
- SCRIPTED - Migrated many tests into automated integration and unit tests.
- TRIGGERED - Setup our git-based automatic branch testing.


### t2022-11-03  Reference for Sports Visio

Frank,
How goes it?  I see from linked-in it looks like you have a new gig.  Congrats.

I have had success in selling Analytics Fire to one of our former clients, they basically bought our entire 30-person team.  (they raised $170M so fast they never had a chance to really build a SW team.)  Their CEO is was very satisfied client  of ours (during his last startup), so we got a better exit than you normally get for a consulting company.  

I have a favor to ask.  I am in the final stages of interviewing for a CTO role for a computer vision startup that is aimed at automatically capturing basketball stats from video.  My PayByGroup experience has a bit of relevance as an example of pushing to production attention to quality was relevant.  If you have time for a quick call in the next days, I can give you a bit of context.

and I would love to hear anything you can tell me about the stealth startup.

Cheers, 
Dan

~


This is a sweet spot for me, since it really involved PhD computer vision work in deep learning, and they really need a scrappy startup oriented (late joining) co-founder.  Jason, the CEO knows me well, and has tried to get me as a co-founder several times now, and he is pretty sold on me as a partner.  But he recognizes that ensuring production quality is one dimension that will be relevant in this new role, in addition to the more reserachy aspects of the role.  He is wanting to look at cases where I was helping to push code to production.  In my robotics startup I was also doing this, but I think PayByGroup is the simplest case where we were regularly pushing code to production, and because it was a live payments platform, we needed 
