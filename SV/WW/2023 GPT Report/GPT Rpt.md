## Sports Visio Competitive Technical Risk Analysis for GPT-4


### Executive Summary

My overall conclusion is that GPT-4-like technologies are a potentially transformative technologies for related business models and uses cases that could be very important for Sports Visio. At the same time, there are a number of inherent aspects of this technology that make it unlikely to be directly applicable as drop in replacements for many of the detailed CV tasks that we are currently working on.  In that way they are less of an immediate risk, and are less of an immediate benefit to the narrow way that we are framing the Basketball stats problem  (e.g. to just getting the box score stats).

Broadly, however, deep learning and related algorithms do represent an immediate threat to our business.  Our mote is just not that deep from a pure technology perspective, so we do need to worry about competitors replicating (or beating) our basic tech, and coupled with a better go to market strategy eclipsing us in the market place.  This is not a GPT-4 mediated threat, but it remains a large threat.

The bigger area for our focus with respect to GPT-4 in particular are multiple, as yet undemonstrated, abilities to drive a much more intelligent AI-based coach assistant.  I think this possibility should be on our radar, as we could provide capabilities that would unmatched by ANY automated systems today, and really could only be replicated by having a dedicated expert level set of coaches analyzing games. That kind of technology could be a HUDL-killer, and even a collegiate level killer, allow us to penetrate markets where offshore analysis simply could not touch us.  Very beginnings of that thinking is included at the end of this doc.



### How to think about Large-Parameter ML Models like GPT-4
- These systems compress colossal amounts of semantic knowledge latent within their training data into their model.  They are really good at surfacing this semantic knowledge at the right time in the right place when presented with novel data from similar spaces.
- Unlike previous incarnations of GPT, GPT-4 is explicitly trained of large VISUAL corpora, thus is much closer and much greater risk for Sports Visio.
- Still when thinking about this risk and opportunity, one should really focus on places where deep semantic knowledge will have transformative value, since these are the places where the key advantage of these large parameter models have greatest value.  
- One can still productively use systems like GPT-4 in places where little background semantics knowledge is needed, but it is not clear one would gain any advantage in such places, AND likely one will pay practical penalties for using a complex/heavy system like GPT-4 in places where it prior training is not adding lots of value.  Thus these would mostly be counter indicated as a likely area of productive application.



### What exactly do we mean by "Semantic Knowledge"?

For our purposes here, Semantic knowledge refers to the general knowledge and features that make up concepts can be acquired and abstracted from their experiences large amounts of generalized experience.  This is in contract to knowledge that is gathered from a given situation, or knowledge that is gathered specifically about a very narrow task domain.  In our case, detailed models of a basketball hoop that is gathered from thousands of images of a basketball hoop would not be semantic knowledge, but strategies and approaches for playing ball sports and basketball generally would be an example of semantic knowledge.  The difference is about how narrow the concept being learned and how tuned the training data is to the specific task being done.

This distinction is quite relevant for SportsVisio-GPT4 discussion since GPT-4 is centrally concerned with acquisition of Semantics-rich knowledge from data, while SportsVisio to date has been more focused on learning at most lower levels of semantic content.



### Levels Of Application

This is my own decomposition of the levels and types of semantic knowledge we might try to use GPT-4-like technologies to inject in to our basketball stats. I give examples of each kind of knowledge as well as a quick assessment of how likely it would be that we could get value from GPT-4 at that level.  The remainder of the document dives in a bit deeper to justify the claims made here.


#### 1 - Relatively Semantic-Free Visual Analysis

Examples include Ball Tracking, Court calibration, Hoop detection, and probably Jersey Detection.

We call these "relatively-semantic-free" learning because there are limited opportunities to productively use a broad background knowledge learned from disparate learning contexts. But there is SOME semantic knowledge in utilization in these tasks.

For example, simple tracking algorithms will model the object's velocity in two dimensions directly in the pixel coordinates of the camera, while a more sophisticated tracker can estimate and dynamically adjust ones expectations of the tracked objects motion considering it relative distance to the camera knowing that apparent 2D motion will increase as the tracked object gets closer to the camera.  We can consider this a kind of semantic knowledge about how objects operate within the 3D world.  It is plausible that the semantic models learned by GPT-4 would productively contain such bits of semantics knowledge.  Still it is more likely that we can train systems to accommodate such small amounts semantic knowledge by explicitly encoding 3D tracking into our tracking systems, or gathering this knowledge latently in a deep learned tracking algorithm that captures this from by narrowly training on tracking directly in the context where we hope to perform tracking.

==> GPT-4 is unlikely to provide significant value for the semantics-free tasks in this category.  This a key finding for this report, since these semantics-free tasks dominate the "box score" stats problem at the heart of our current product offering.

The reason GPT-4 will have little value for these tasks is two fold:
(1) These tasks all have low semantic content, thus there is just less value to be obtained from 
here is a combination of the fact that deep semantics are less valuable for these narrow tasks, 

(2) Second there are a number of limitations that for the foreseeable future will be inherent in any system that builds upon GPT-4 like systems, such that even if there is some residual semantic knowledge that GPT-4 can bring to bear on these problems, the cost of embedding GPT-4 will likely far out weigh 


#### 2- Micro-Visual Analysis

Certain aspects of game play come down to detailed analysis of "close in" features of the video footage.  These would include both aspects required for some box-score stats as well as many details that would be relevant for tasks beyond our current scope.

Examples: Blocking

Certain detailed aspects of a visual image or video can be crucial in making basketball stats assessment.  For example determining if a successful block has occurred requires determining if a player touched the ball.   In many cases on can simply look at the ball's trajectory to determine if a block as occurred, but in other cases on needs to get very granular with the images and try to assess if a player has touched the ball.

This micro visual analysis has notably more semantics in it since it involves how balls typically move and how players typically block.

==> GPT-4 is unlikely to provide significant value for this level of task since the 




#### Macro Strategic Visual Analysis

#### Micro Strategic Lingual Analysis


#### Macro Strategic Lingual Analysis


### Inherent Limitations of Large Parameter Pre-trained ML Models

There are a couple of limitations that are likely to significantly limit the usage of GPT-like solutions in the near and mid term.

(1) COMPUTE - These semantic-rich models are built from hundreds of billions or likely trillions of parameters.  Significant work has gone into approximate and simplified version of these larger models that only require a fraction of the compute of the full model.  But this can only go so far, they will likely never compete at parity with models that are optimized for perform on low semantic-learning tasks.  One semantically rich learning tasks their performance will be superior, thus it may be work paying the price, but a compute price will always need to be paid.

(2) BANDWIDTH - Presently the GPT-4 like systems are not configured to process large amounts of per user execution data.  This is likely to change somewhat from given the shift from 3.5 that does not take visual data to GPT-4 which does.  (Visual data is typically more than 100x larger than the textual data accepted by 3.5)

(3) PHOTO VS VIDEO INPUTS - GPT-4 does not yet accept visual data, and little concrete information has been supplied regarding exactly what it will accept.  Still given the constraints on compute and bandwidth listed above it seems unlikely that GPT-4 will accept video data.  If so, this means our most central data form will not even be accepted by the GPT-4 system. 

(4) ALTERNATE USE PRICING - There are many semantic-rich domains where GPT-like systems offer performance that comparatively better than any alternative approach automated approaches which also deliver high value per data instance analyzed.  This may well dominate the early pricing of GPT-like systems.  For example is if other uses are to deliver a revision on the text of a resume, or understand what is misconfigured in the assembly of some product.  The comparative value derived from analyzing a single layup might be many orders of magnitude less as both price point and number of instances required are quite different for our domain.  Even once compute and bandwidth prices drop these likely will be an incentive for OpenAI and others to maintain elevated prices in order to reap value from these higher margin usages of this technology.

(5) BLACK BOX / STAND ALONE COMPONENT INTEGRATION - OpenAI has already indicated they intent to maintain an extremely insular product model.  For security and profit reasons are not intending to releasing a version of their AI system to be run outside of their facilities.  This means that many optimizations in solution design (e.g. edge computing) are inherently off the table once one has adopted a GPT-like system early in ones data processing pipe.

For all of these reasons one will need to be obtaining transformatively better performance from a system before one is likely to be introducing GPT-like components into ones system.



### The Big Unknown Detail
The GPT-4 report from open AI indicates that has have visual analytics built into its model even though they have yet to release this capability to us with their current GPT-4 interface.  Of course this is the most important data form for our domain, this it makes it hard absolutely certain of anything regarding just how much power on might obtain from GPT-4 on SportsVisio-like tasks.  Despite this limitation we can draw some tentative conclusions that are likely to stand the test of time.

- For reasons listed above it is likely that GPT-4 will be limited to still image analysis and will not support video analysis.  (though one should expect such video analysis will indeed be available for future version of the GPT line.)

- Macro vs Micro visual analysis.  Will GPT-4 has now been trained on visual as well as lingual data, most of the history of this OpenAI project has been focused on that capture of very general purpose lingual-knowledge.  Thus it seem most likely that their first steps into multi-model understanding will skew towards macro level image understanding which connects to a lingual understanding of the image, rather than a micro-level understanding of some aspect of the image.

For example it seems quite likely the system would be able to recognize a player that way blocking another player in a shot (a macro assessment of an image), but would not be trained to recognize if a blockers stance and hand position was optimized for blocking given the relative motion and position of the approaching player (a micro assessment).

So while both of these kinds of assessment will be relevant for Sports Visio only the former seems plausibly output my the GPT-4 system, the latter seems relatively unlikely to be delivered by the system.

But we really wont be sure about much of anything with high confidence until they release the visual processing.

#### Areas of potentially valuable semantic knowledge.
The devil will be in the details in these areas, it is far from clear that GPT-4 can productively be used in these areas, below we list a few of the considerations that will be relevant. This is really just a list of things that MIGHT be interested, valuable, transformative, when using GPT-4.  As these things become crucial from a business perspective, we should invest time/energy in assessing which if any really could be productively done using GPT-4, or other large trained systems.  Here are a few of the considerations that would affect whether GPT-4 would have value:
- Was it trained on relevant materials
- Are the details of data capture and usage context close enough that one expect transfer from the training context - topview side view
- Can semantics free models to a pretty good job



ACTION RECOGNITION - 

BLOCKING - 


LINGUAL DESCRIPTORS FOR GAME PLAY, GAME STRATEGY, PLAYER/COACH THINKING - 


, and precisely how we


### We AINT talking!

- blah blah blah  GPT-4 is a transformer-style pre-trained model fine-tuned with human-feedback based reinforcement learning  blah blah blah.  
  And that is all we are gonna say.

Excerpt from the GPT-4 Tech report: https://arxiv.org/abs/2303.08774
This report focuses on the capabilities, limitations, and safety properties of GPT-4. GPT-4 is a Transformer-style model [39] pre-trained to predict the next token in a document, using both publicly available data (such as internet data) and data licensed from third-party providers. The model was then fine-tuned using Reinforcement Learning from Human Feedback (RLHF) [40]. Given both the competitive landscape and the safety implications of large-scale models like GPT-4, this report contains no further details about the architecture (including model size), hardware, training compute, dataset construction, training method, or similar.


Below is a discussion on secrecy that Open-AI has about its system.  The short summary is that from 2017 thru 2023 openAI has become progressively more secretive about its system.  At this point they are basically telling us NOTHING. I think we can expect this to continue, they can use the valid excuse that this tech is very dangerous and needs to be contained.  But of course it is also nearly impossible to protect this tech fully using patents and such, so secrecy is also economically quite valuable for them.  Thus I expect Sports Visio and its competitors will only have indirect access to these technologies INDEFINITELY.  Of course they will make APIs available for usage of the tech as a kind of black box component within our systems.

https://www.zdnet.com/article/with-gpt-4-openai-opts-for-secrecy-versus-disclosure/



### Productization Velocity

In general I don't agree with Gary Marcus on this assessment of AI's existential risk (an area where it he written a bunch).  Still I do think this quote from him does capture my feeling about how chat GPT will hit the market place:

	let’s temper our initial enthusiasm by realizing we have seen this movie before. It’s always easy to make a demo of something; making it into a real product is hard.


### Conclusion

These are very early days with this new kind of technology, and OpenAI is keeping its cards very close to its chest.  Thus any conclusions we draw in this moment need to be somewhat tentative in nature.  Still the broad outlines of how this tech is most likely to evolve is taking shape:

My guess is that there will be certain tasks that are very lingual in nature to which GPT-4 can be adapted somewhat "out of the box" by performing simple transformations on the text that goes into the black box, and transformations on text that comes out.  Those applications will come almost immediately, and will not represent much of a technical moat for the companies doing them, since all companies will have access to the same black box.  But those will be a flash in the pan, though a some will go on to become new kinds of businesses.  e. g. automated resume writing.  These businesses will not be constructed on large moats of technical differentiation, so instead it will be about building the right partnerships, the right UX / business framings, and go to market strategies in order to build a differentiated business.  There will be winners, successful brands will become known and they will get the lions share of business in order to fleshout more features around their core product, and build more and more partnership relations, thus cementing their business within a low margin market where it becomes difficult for followers to profitably replicate what they have built.


For many other businesses like ours, it will start out as an augmentation of our core value proposition.

My low-confidence guess is that we are not imminently in a position to begin that augmentation pathway in 2023 for a couple of reasons:
- the Vision-based input not released
- the availability and pricing is not at a point where one could use it
- Likely the level of 






# APPENDIX A - A chat with GPT-4


Here we explore the ability for GPT-4 to strategic Basketball analysis.  We are focused on the system's ability to provide dynamic basketball advise.  Providing a generalized basketball 'tutorial' will offer little over some online tutorial covering Basketball strategy.  The transformational opportunity is if GPT-4 could be used to (1) provide a strategically rich understanding of the dynamics inherent in the nature of the specific players on the two teams playing, (2) as well as a strategically rich understanding of the game actually being played.  (3) then to suggest strategies optimized to perform well in the context of this dynamically understood context, and (4) finally to diagnose and replan/refine strategy in light of real-time execution of these strategies.

This is well beyond capabilities currently provided by Sports Visio and our nearest competitors, still they seem of transformative value, and might really catapult us into markets that we otherwise could not attack.


As discussed above, it is likely that GPT-4 will not be able to process video steams, though later GPT-like systems will be able to do.  Even still present day deep-learned action recognition IS able to process video streams and extract high-level descriptions of the video streams.  Likely deep learn models already have been used or can be used to detect an opponents use of pick and roll as discussed below.  Building a system that would be capable of recognizing all semantic features needed in the use case below is within the realm of plausible, still it would be an ambitious R&D project, not likely a out of the box capability for present day deep learned systems.

It does seem plausible that "push button" trainable systems could be on the horizon for must of that level of action recognition as listed below, still such systems are not here today, and it not clear that generalized systems like GPT-4 would ever have the specialized concepts required for this task, these deep learned action models might be directly learned from many hours of training on the sport of basketball specifically.


## Example 1 - The Pick And Roll

With all of these limitations noted, it is really quite stunning just how deep GPT-4 is able to go in understanding basketball strategy and play.  This first line of questioning is aimed at assessing how well GPT-4 might be able to understand, strategize-about, execute, diagnose practical basketball game play.  We start by digging into a most basic game play strategy the pick and roll:


#### DETECTING THAT ONES OPPONENT IS USING THE PICK AND ROLL

![[Pick and Roll 1 (Detecting it).png]]



#### DEFENDING AGAINST THE PICK AND ROLL

Ok, so we no see our opponent is using pick and roll offensively against us, so what we do about that?

![[Pick and Roll 2 (Defending Against).png]]


#### DIAGNOSING FAILURES IN OUR DEFENSE

Here we assume that we opted to use hedging in defending against the pick and roll, how can we tell of our hedging strategy is working, and if it is not working how do we diagnose the failures we are seeing?

![[Pick and Roll 3 (Diagnosing Defense Failures in use of hedging).png]]



#### STRATEGIZING ABOUT OBSERVED FAILURES IN OUR HEDGING DEFENSE

Given repeated observations of the opposing team dribbling between our hedging defender and our screener what might we do to strengthen or adjust our hedging approach?

![[Pick and Roll 4 (Strategizing on hedging failures).png]]



## Example 2 - Strategic Analysis of a Hyper-Specific Game context

Example demonstrated that GPT-4 is quite capable of "going deep" into the details of just one of many different basket ball strategies.  Really impressive stuff!  Still, it is also very GENERALIZED stuff.  Indeed it is the kind of material that might even be compiled into a super detailed tutorial on basketball strategy.  It would apply to ALL basket ball games.  But what about asking GPT-4 strategy questions about the very specific basketball game being played right now?  Is GPT-4 able to contextualize its understanding of the game so it can provide advice appropriate for a single concrete game context?  

To test this we described a specific game context to GPT-4 and then began asking it strategy advice for this particular game.  The results were quite impressive, and it seemed that we could have provided similar descriptors for all ten players on the court and it would have given us competent advice accounting for all the details of these ten players:

#### Our Game Context

Here we describe our players and the opponent's players and immediately GPT-4 has got ideas about what we might do:


![[Strategy Example 1.png]]


#### EFFECTIVE USE OF OUR BALL-HANLDER IN A FULL COURT PRESS

So we are choosing your option #2 the full court press.  Exactly how do we leverage our ball-hander?

![[Strategy Example 1a1 using ball-handler with full court pressure.png]]


#### FULL COURT PRESS: QUESTION 2

A slightly more advance version of full court press question:  Given that we are executing a full court press as indicated how can we best combine the skill of our ball-handler with our well rested fast-break player?


![[Strategy Example 1a1 using both fast-break and ball handling in full-court pressure.png]]


#### DEFENSIVE STRATEGY QUESTION

Given their taller players and better shooting abilities, what strategies should we consider?

![[Strategy Example 1d1 defend against tallest opponents.png]]


#### CAN GPT-4 MAKE QUANTITATIVE STRATEGIC ASSESSMENTS

Above we have been presenting qualitative assessments of game context, and GPT-4 has been providing qualitative descriptions of correct actions in response.  What happens if we provide quantitative information in our context, can GPT-4 use these quantities in providing appropriate advise?

Case 1 - N=2
![[Strategy Example 1d2 quantative assessment of strat failure.png]]


Case 2 - N=15

![[Strategy Example 1d3 larger quantity same question.png]]



WOW! WOW! WOW!
That sure looks like some next-level S * * T!  And I think it **IS** some next level stuff!!!   I think there will be solutions new solutions that will blow the sox off of coaches, and will be very disruptive at all or many levels of the sport right up to the NBA!  I think these quick examples just touched on what could be possible.  Indeed I think real time deep strategic analysis would be transformative.

BUT, these examples are assuming that pretty sophisticated video action recognition that is well beyond what vanilla action recognition gives us today.  And since GPT-4 in it present form does not accept video input, the only way one could get these results today would be human annotated game transcripts.  This would be prohibitively expensive, so expensive it is likely unworkable.

Still it is challenging but plausible that one could use deep learned action models to produce the inputs that GPT-4 would need in order to perform this kind of analysis.  This is challenging, but not out of the question given todays technologies.  Indeed an important area of research exploration will be an assessment regarding the plausibility and difficulty in developing such a hybrid system using today's technologies.







## REFERENCES

The GPT-4 Technical Report
- https://arxiv.org/abs/2303.08774