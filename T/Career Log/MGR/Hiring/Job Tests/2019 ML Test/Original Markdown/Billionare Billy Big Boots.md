# === THINKING ===
## OUR ML/AI HIRING STRATEGY
**TARGETED HIRES**

- **Data Engineer** -- idea: supporting role, also serves as full stack dev

- **Expert Consultant** -- idea: part time spiky, super specialist, pricy

- **Expertise areas** --
- STATY - Low dimensional, numeric, large data
- DEEP -
- NLP - 
- GOFI - 
- VIDEO - 
- TEMPORAL - 


**STRATEGY**

- Hire 1-3 Data Engineers as we can find them, but ensure each are also strong full stack dev.
- Contract with 3-8 consultants in different specialty areas.  
- Redundancy is desired since we cannot ensure availability
- Solo consultants are best since we can expose them to the client in a more straightforward way.

## SKILLS TO TEST FOR
- **FRAMING** -- Ability to uncover and frame ML/AI problem given loose buisiness objectives and loose problem / data specifications.
- VALUE PROP -- Ability to articulate the business value of the proposed work.
- APPROACH -- Ability to develop an end-to-end approach for solving the problems identified.
- **MANAGEMENT**
- **SOW** -- Ability to produce a strong statement of work.
- **FACE-CUSTOMER** -- Wisdom and presence to serve as the face of AF for the customer.
- **MGT-EXECUTION** -- Manage the execution of the project.  Manage the other AF team members, manage the evolving requirements from the customer, manage the schedule, provide early detection of issues and failures, and provide effective remediation for those issues/failures.
- **DATA-TRIAGE** -- 
- **ML APPLICATION AND ANALYSIS** -- 
- **CODING ABILITY** -- 
- **TECHNICAL KNOWLEDGE** -- 
- **OVERALL VELOCITY** -- We are not so focused on raw coding or document production speed, but we do need to see that candidate is able to rapidly approach, assess, and make progress against novel problems.  Our ability to win and keep business is tied to how we attach customer problems.  Candidate will have large impact on our perceived and actual velocity.

## Other stuff

- **Expertise areas** --
- AI/ML
- Platform (cloud/backend & embedded)
- Safety critical
- Polished Interfaces
- **Expert spikes of SW Dev**
- Hardest problems  (think PhD specialists)
- Easy:   integrate well / manage well / frame problems well 
- multiply:   in house value/IP, in house capability, smooth-supported handoffs


KEY SKILLS WE WANT TO TEST

PROBLEM FRAMING / APPROACH GENERATION

DATA CLEANING





OUTPUTS
— Propose 5 analyses we might perform for BB


# === ML-TEST === BIG BOOTS WINERY ===
## Background Story

**PART ONE -- CUSTOMER BACKGROUND**

Billionaire Billy Big Boots wants to invest heavily in creating a new premium wine brand name.  His long time friend and money manager Charlie C. Curmudgeon has hired us to perform some preliminary analyses to support and direct these efforts.  For as much as Big Boots likes freely spending money, Charlie C. does not!  At least not unless he knows it is going to land even more money in return.

So we have a small contract to perform some preliminary analyses for Carlie C.  The contract is quite open ended, Charlie C. is not sure how to approach this, and even what questions to answer. Big boots has provided a data set and wants to build his next billion dollar business here — Charlie C just wants to not loose their collective shirts. 

They have a team working the financials around such a business — our task is to identify the best targeting and strategy to create an award winning line of wines in order to anchor the brand — where to buy land, what kinds of grapes to target, which specific land parcels to buy, in what countries, etc. 

He has cash to burn, and our job is to enable his dream, but have him not loose his shirt!  He has no idea what to buy, where to grow, what to market, etc.  It clear if we provide value in this early contract, that Charlie C is quite open to much more work from us ... we could potentially expand our operations to cover his larger business operations -- his adult diaper empire… worth billions in annual revenue.

Help us, Help him out !

- where should he be buying land?  What should he buy?
- which critics most influence price?  What do they say?
(We have a big marketing budget to woo them)
- what words should he use to market his different varietals to get the right thoughts placed into customer & reviewers heads.
- What makes an award winner?  What makes a high price seller?
- What should we do for him?

He doesn’t know, and we don’t know...



**PART TWO -- CUSTOMER BACKGROUND**

Well C. Curmudgeon is quite cross this monday morning.  It seems that Big Boots just bought a vinyard over the weekend without ANY due diligence.   The silver lining is that Vinny's Vines has more than 15 years of data on their wine making operations.  C C thinks we might be able to both improve Vinny vine's operations and also learn important lessons about making great wines which we can use in deciding how to create our award winning brand.  

Vinny was an early adopter of digital sensing, and has over 15 years of detailed data on the condition of his grapes in each part of his vinyards, weather data, and data about the grape/wine batches that resulting from these over the years.

A few years later he started contracting with a service using remote controlled airplanes to give him fly over video of his vinyards.  He just used this data for irrigation and harvest planning, but it seems that statistically one might be able to use subtle changes in coloration to detect bee polination and blight.

Vinny pays for bees to polinate the cover crops they use in the winery to replenish the soil, but these days he is often not getting his monies worth since the hives are so much weaker over the last decade, also over the years both insect and disease blights have really damaged his vines at different points.  In each case the blight started small and then progressed to cover greater fractions of his vines.  If we could detect it early, then it could be stopped by chemical or other means, but once it affects more than a handful of percent of the vinyard, it is really too late to stop the spread.

Both of these cases are very tricky to detect.  They involve small changes in color and luminance, much smaller than the changes which occur based on sun and weather in any given data.  Further, there is very little labelled data.  We know once a blight has progressed that earlier images should also be viewed as positive examples, but exactly what subset has been covered at each point in the past is not known.  Also with the bees, we have many hundreds of specific data points where we can definitely say at this momenet in this location there definitely were or were not bees on the cover vegetation, but these points are a drop in the bucket.  so in both cases we need to develop some iterated or semi-supervised approach for building these models.


## **TEST PART ONE -- PROBLEM FRAMING AND INITIAL INVESTIGATION**

- **PART ONE A -- DATASET INVESTIGATION REPORT**

	Analyze dataset and generate short report (2-4 pages of text excluding any figure).  
	
	Report should include quantitive and qualitative assessment of the data (custering/regression etc.) to support your proposal for deeper analysis that AF will pitch to the client.  Pay zero attention to polishing the formatting of the figures / tables / graphs.  Whatever is easiest.  The focus is showing us you can quickly assess avaialble in order to justify (and rule out) approaches for their targeted task.


- **PART ONE B -- WRITE UP A TWO-STAGE APPROACH**

Write up 3-5 page summary of a short bit of work (1-6 weeks) that could be done to show the customer progress-towards and viabilty-of the larger approach proposal.  The larger proposal should include the range of analysis and alternatives you would propose for a 6mo to 1year engagement with the customer that involves 2-3 developers.

- The report can simply be a list of bullet points that indicate the work to be done.  The focus should be on the TECHNICAL DETAILS of the approach that you are proposing.  
- Imagine this note is an internally proposal with technical details document which you and the engineers will refer back to as we execute your proposed approach. 
- This internal proposal should also include your reasoning about why this approach is correct, and should clearly indicate the critera for any decision points, and critera of compleness for each step.
- CREATE SOW PAGE -- Create three detailed items from a SOW that might be provided to the customer to underly this approach.  Each item should provide some estimated LOE (level of effort) required to complete the step, as well as criteria specifying completness and quality of step as appropriate.


## TEST PART TWO -- HANDS ON MODEL BUILDING

- **PART TWO A -- HANDS ON MODEL BUILDING**
	
	_AIM: use dataset to justify brand building choices: geographic locations, grape varieties, descriptive adjectectives or multi-word concepts that the data suggests  might serve as the target for a distinctive, value laden brand_.
	DIRECTIONS:  Treat this hands-on work, as an abbreviated bit of model building for our client.  Perform the work as you would if it were part of an ongoing analysis (so document the work well enough that you or others could perform follow on analysis using datasets, scripts etc. from this work.)  All parts of these deliverables should be in a form that we could share with the client.  (That said, this is intended to be professional analysis output, but NOT code intended to production-ready releases or documentation.)

- **FRAME TASK** -- Specify a set of brand-building questions that the data could be used to inform.  Then define the specific learning / analysis that is to be performed in order to inform those decisions.
- **DATA CLEANUP** -- Output:  cleaned data, list of steps used to process the data, and summary of the detailed reasons why specific steps were taken.
- **RUN MODELS** -- Write scripts to run multiple ML and analytics models against cleaned data.  Provide intermediate and final datasets as appropriate. 
- **PROVIDE WRITEUP** -- Provide writeup of the conclusions from the modeling work performed.  Writeup must include:
	- SCRIPT EXPLANATION -- Explanation of *what* modeling was done, sufficient to allow someone else to replicate.
	- SCRIPT JUSTIFICATION -- Explanaiton of _why_ the modeling was done as it was.  e.g. choice of algorithms, of preprocessing steps.  If iteration was done, please provide explanation of each step in the iterative investigation.
	- CONCLUSIONS -- What conclusions do we draw regarding their brand building?  What recommendations can we make, and how confident are we in each recommendation?  What is the justification for each?


- **PART TWO B -- ANSWERS MATRIX** 
	
	_AIM: Test candidates technical knowledge across broad range of problem and approach types rapidly._
	Directions:  Write 5-10 pages of bullet point answers covering many ideas and approaches for the cells in the matrix below.  Your aim should be to show your breadth and your depth, by the volume AND quality of ideas regarding how to approach these problems.  Don't waste time/space stating the obvious, focus on most important observations/ideas/issues to raise across each of these problem areas.  For each area summarize your depth of experience with that kinds of ML.  You don't need to devote equal energy to these problems, if some of them are too far from your experience areas you can even just skip them.  Show us what you know from overall Quality and Volume of your observations, and approach ideas.  Organize your response as a list of bullet points organized by Problem task (shown below) as well by the question being answered (also shown below).

- **PROBLEM TASKS**
	- **SENSOR-DATA** [TEMPORAL/STATS-Y] Vinny's Vines 15 years of sensor data:
		- Data collected every 10-minutes continuously.
		- Wind/sun/rain
		- For each 10 square yards we have 4-times-per-year dirt chemical composition (30 parameters)
		- Vinny keeps records of which plots of land were used for each batch of wine
		- For each batch of wine we have hourly data during fermentation, and weekly numbers during aging.  (15 properties:  temperature, viscosity, sugar levels, tannins, etc.)
		- Each batch has a receipe expressed in meta data as a set of steps with fixed parameters on each step.  E.g. press wine.  15psi for 30minutes, then 30psi for 5 minutes, then settle 2 days. etc.
		- Vinny keeps track of the lineage of each of his vines, he knows which where grafted from others in a digitally encoded "family tree"
		- And of course he has captured data on the ratings that he has received on each of the batches he has produced over the years.
	- **VISUAL-DATA** -- Vinny vine's 12 years of vinyard images.
		- FLY OVERS -- Company performing the data orgaize it into a series of "fly overs".  Each flyover is a set of images that over the whole vinard as a set of overlapping images that each have accufrate geolocgion that are always show as the craft flies from west to east at 100 feet altitude.
		- TRAINING DATA -- Associated training data is associated with 
		- Semi-supervised training.  (small number of blight occurances, training data from weeks prior.)
		- Sporadic positive instances.  (not all blights captured.
		- Data at different times of day
		- Continuous progression.
		- Heavily skewed data -- much more "no bees, and no blight" than examples of each.
		- False Negatives -- No false positives (if we say its a blight, it is)  but there are many. cases of false negatives (blights that occurred by are not annotated in the data).
- **ASPECTS OF OUR ASSESSMENT**
	- YOUR EXPERIENCE -- Your experience addressing similar problems.  (number of months fulltime work, and setence describing up to three background experience tasks)
	- FRAME QUESTION & VALUE -- Write a paragraph framing the business questions you proposed to answer for the client, and the value this will provide to them.  
- col approach summary (data clean&transform/metrics/outs; alg choice; steps)
- col data-triage -- what kinds of data cleaning and data rerepresentation should be performed or considered for this task.
- col less-obvious problems + less-obvious amelioration strategies -- Secondary issues that are likely to come up in solving this task, along with possible amelioriation strategies for these issues.

## TASK DETAILS

DATASETS

— Wine Reviews
https://www.kaggle.com/zynicide/wine-reviews


# === OTHER POSSIBLE TASKS ===
## Black Friday Sales
## Corsara course reviews
https://www.kaggle.com/septa97/100k-courseras-course-reviews-dataset#reviews.csv
## Airport Conjestion modeling
https://www.kaggle.com/cityofLA/los-angeles-international-airport-data
## Parking data
https://www.kaggle.com/cityofLA/los-angeles-parking-citations

? Make most money
? Long term trends
?
