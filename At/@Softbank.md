#pp  

### 2024-12-29  New Note (As Sent)

Asuka-san:
I appreciate the feedback from Tsuji-san and will be very happy to process further games from him as well.
We will schedule a meeting in January to talk through our current status and plans for the new year. 

Our focus in Q4 was on productizing both AI-computed minutes and AI-computed shot charts.  In Q1 we are shifting our attention to using the underlying algorithms developed for producing final game-stats that Softbank and others might consume.  Thus we have not yet produced the JSON end-point that would allow Tsuji-san to access those results directly, but we have built an prototype that does deliver the V1 stats as documented, and so we can share results from that for the game you shared, and for any other games Tsuji-san would like to test.


A couple of notes on the game that you have provided us:  Our system has been tuned for basketball games that are provide from one or two cameras that are simply panning back and forth on the basketball court.  We think that is also the use case that you are imaging for your users.  The video that you provided to us appears to have been professionally processed for purposes of broadcast.
- This means that the game is shown with frequent and sometimes large zooms as well as panning.  During the zooming-in we can loose a lot of the context around the basketball player that is used to determine where the camera is pointing.  (Our system maps out lots of detail in the stands and on the walls of the gym etc in order to know where the camera is pointing.  If the camera zooms in where it only sees a small patch of the court floor it can get lost.). We think this is a feature of the example game you showed us.  We are assuming this will not be true generally for your customers.  Is that an accurate assumption?
- The game you showed us also has a banner at the bottom showing the scores and such.  This banner remains in a fixed location even as the camera image is moving around.  Thus our current system automatically uses that very distinctive banner and gets confused thinking it is a good way to locate the court.   If your games will have such a banner, we can build a filtering subsystem to remove those banners so the AI is not confused, but we are assuming these banners will not be part of your customers games.  Is that a correct assumption?
- Your example video also uses an international basketball.  We have not train our system on such a ball.  It seems the existing ball model is close enough to catch the international some of the time, still we will get improved performance once we have labelled and trained on games with such a ball.  We will be sure to include such a data-labelling activity in Q1 of this year in order to handle Softbank games better.  Still I think we will be able to provide results to Softbank even using the existing system, and they will simply get better once the new ball model is integrated.

I look forward to further discussions in our January meeting, and just as you noted in your email, we are honored and excited to have such a strong partner in Softbank to help us build out this capability in best possible ways.  Looking forward to a great year of partnership with you and your team!

Best,
Dan


P.S. I just received a note from one of our engineers working over the break that he was able to correctly filter out the banner in the video you provided, so I think we will be able to provide results from that game.  Do let us know if such banners will regularly occur in your customer's games.


### 2024-12-29  New Year Note (Never Sent)

Asuka-san:
I appreciate the feedback from Tsuji-san and will be very happy to process further games from him as well.
We will schedule a meeting in January to talk through our current status and plans for the new year.  For now, let me provide a summary of our current system:
Our focus in Q4 was on productizing both AI-computed minutes and AI-computed shot charts.  In Q1 we are shifting our attention to using the underlying algorithms developed for producing the final game-stats that Softbank and others might consume.  (I do anticipate 1/3 of our time in Q1 will continue to be focused on tuning the shot charts and AI-minutes systems as those systems are scaled up.)
A secondary focus in Q4 was improving the accuracy of several sub-systems associated with top view location of players and shots as well as Shooter ID.   (Shooter ID is the determination within the court where a shot originated from, and which offensive player made that shot).  Early signs are that we have made large improvements on this subsystem.  At the very end of the year we have running comprehensive before and after stats in order to assess how much we have improved that.  We will share these results as we have them.  These improvements were essential for shot charts (as shooter and location are both quite important) and they really improve all basketball stats we will provide to Softbank as well.
As previously discussed, our target for Q4 was also to be able to process games into the stats as needed by Softbank and others who want end-to-end stats.  We have done this as well, the V1 stats we described in our documentation are currently produced as a side-effect of running our main shot charts and AI-minutes pipeline into a local file.
We will use this manual process to provide the first few games we process from Softbank.  As we shift towards basketball stats in Q1 we will also be providing this same data using a traditional JSON endpoint.  We will be delighted to have Tsuji-san helping us as a first external customer for this interface!
The current system has two aspects of the game that are still being done via human assistance human annotator, the first manual step is court boundaries.  We have an interface where an annotator uses the mouse to click on regions of a single still image of the left and right hoop areas in order to indicate the boundaries of the court and the three point lines.  Once these master images are annotated, then AI system matches those to all frames of the game in order to locate balls and players on the court.  Eventually we will automate this process first step as well, however the annotator time required per game for this step is quite small, so it has not been a focus for our team.
The second manual step is annotating game breaks in our system (including half time).  Currently this is done via manual annotation.  Annotating this information takes a bit more than 10 minutes per game so it is a larger priority to automate this part of the game analysis.  We believe this is a technically simpler sub-problem, so we have as yet not allocated resources against it.
The JSON end-point we are developing in Q1 of 2025 will trigger a manual annotation step for these one or two manual steps in the background.  We have annotators today that work all throughout the week, so this manual step will delay the result by some amount, but will otherwise be transparent to your team’s usage of this interface.  Both of these manual sub-steps are notably easier than providing the basketball stats themselves, thus our focus has been (and will continue to be) on the hardest parts of the Stats problem first.  During 2025 we anticipate a 100% AI-stated basketball game.
I suspect the speed with which we scale our joint usage of the Softbank interface, will dictate the speed that we achieve in getting to 100% AI-stated games.  Manual processing steps are acceptable at small scale but quickly become too cumbersome and expensive as our usage scale grows.  I am sure we can chart a path that is workable on this front.
A couple of notes on the game that you have provided us:  Our system has been tuned for basketball games that are provide from one or two cameras that are simply panning back and forth on the basketball court.  We think that is also the use case that you are imaging for your users.  The video that you provided to us appears to have been professionally processed for purposes of broadcast.
- This means that the game is shown with frequent and sometimes large zooms as well as panning.  During the zooming-in we can loose a lot of the context around the basketball player that is used to determine where the camera is pointing.  (Our system maps out lots of detail in the stands and on the walls of the gym etc in order to know where the camera is pointing.  If the camera zooms in where it only sees a small patch of the court floor it can get lost.). We think this is a feature of the example game you showed us.  We are assuming this will not be true generally for your customers.  Is that an accurate assumption?
- The game you showed us also has a banner at the bottom showing the scores and such.  This banner remains in a fixed location even as the camera image is moving around.  Thus our current system automatically uses that very distinctive banner and gets confused thinking it is a good way to locate the court.   If your games will have such a banner, we can build a filtering subsystem to remove those banners so the AI is not confused, but we are assuming these banners will not be part of your customers games.  Is that a correct assumption?
- Your example video also uses an international basketball.  We have not train our system on such a ball.  It seems the existing ball model is close enough to catch the international some of the time, still we will get improved performance once we have labelled and trained on games with such a ball.  We will be sure to include such a data-labelling activity in Q1 of this year in order to handle Softbank games better.  Still I think we will be able to provide results to Softbank even using the existing system, and they will simply get better once the new ball model is integrated.
We will provide more detail in our January meeting, but I thought it was nice to give you a quick picture of where we are now, and where we will be going in 2025.
As you can see we are in the process of shifting the Computer Vision team’s focus from player-minutes and shot charts over to basketball stats in the new year.  And just as you noted in your email, we are honored and excited to have such a strong partner in Softbank to help us build out this capability in best possible ways.  Looking forward to a great year of partnership with you and your team!
Best,
Dan (edited) 

### 2024-10-01  Meeting #2




### 2024-08-09  Prep for Softbank call

- Open but a little guarded too

Agenda
- Might really want to partner
- Sniffing around to learn something 
- Looking for Acquisition

PORTFOLIO - Smart Coach - Basketball app in Japanese mkt - Money / Sizing
- Manual stats online annotation product.
- Stat product on court



Jason,

Just had a conversation with Sean about our Softbank call tomorrow.
Sean laid out three possible reasons Softbank might have for reaching out:
(1) They actually want to partner as they claim
(2) They are sniffing around to learn something
(3) They are looking at a possible acquisition


Unfortunately I fear it is #2.  As Tony Tether often said "Don't be your miracle on someone else's miracle."  I find it hard to believe they are wanting to partner with an AI company in order to get the same capacity they already have, just at a lower cost.  Reaching out cold for such a dubious advantage seems quite unlikely.  My most optimistic hope is that they are angling for a strategic partnership where they gain an equity stake in SV on the basis of their access to a market.  Possible, but I feel an outfit like Softbank would either not want to mess with us or would want to eat us whole.
Sean plans to start the call just asking them what type of partnerships we might be considering as a way guiding our conversation.  Based on their answers we might lower our guard some.

Still while I am skeptical, I also think we are relatively safe in being somewhat open here.  We won't be digging into which algs/approaches we are taking.  The main info they will gain, is knowledge about what is possible:

My leading guess about what is going on here is that Softbank is considering investing in CV-based basketball, and they want to know that it is possible to do it, before spending the money.  Even if we don't talk with them, they can still look at our product outputs and our pricing and get a basic idea about what is possible.  Unless I REALLY come off as combative, they will likely leave that meeting with additional support for investing in a company that one day could be a direct competitor.  (or maybe they invest in us, but less likely since this company is directly run by Softbank!). My sense is it still make sense to have the call, and to be open with them in a conservative way.  Looking for your guidance on this.

Concretely I plan to talk in generalities about what we can and cannot do.  and in generalities about the quality levels we achieve, but not exact on anything.  So, for example, if they asked if we can do player ID.  I would say yes, but gaining sufficient accuracy in all contexts remains a work in progress. I do want them to see us as desirable and capable, thus I will talk up the difficulty of doing basketball, and toss of half dozen or so specialty modules that we have build as part of our end-to-end solution.  I think naming these (like short origin analysis via multiple approaches) makes us look super cool, and makes the build vs buy equation tip towards BUY.

If they press us about having things in production, my thought is to be optimistic and talk about AI-minutes and annotator halo speedups as being capabilities that we have built.  If they really press on this, i think I would be honest about their current status.

But I am looking for guidance here.  Is this how you want me to play it?  Any other thoughts for me?

--dan
