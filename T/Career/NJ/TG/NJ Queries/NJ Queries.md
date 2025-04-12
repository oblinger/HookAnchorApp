.[[NJ Queries]].  [[6sense]] 


## QUERIES

### GOAL: Gather background on companies

#### Find Mission and Area
Please find a mission statement and area for the companies listed here:

- Provide results in a markdown table and ensure all table content is correctly escaped for Markdown parsing, including newlines as `<br>` and pipes (`|`) as `\|`.
- Use the following columns:  name, area, mission
- 'name' is the company name, and 'area' is one of the following areas or a new area as required: Bio, Robotics, Retail, CoreDL, CoreNLP.    (CoreDL means the company produces Deep learning tech for use across many domains; CoreNLP is across many domains focused on NLP)


[[Cerebras Systems]], [[Adept]], [[Covariant]], [[Aisera]], [[Chroma]], [[6sense]], [[Abnormal Security]], [[Cresta]], [[Genesis Therapeutics]], [[Freenome]], [[Insitro]], [[Skydio]], [[Glean]], [[Uniphore]], [[Arize AI]], [[Deepcell]], [[Groq]], [[Pinecone]], [[Vectara]], [[Weights & Biases]], [[Moveworks]], [[Rasa]], [[Atomwise]], [[Dataiku]], [[Hive AI]], [[Symbio Robotics]],

[[Kodiak Robotics]], [[LangChain]], [[Deepgram]], [[Domino Data Lab]], [[Fiddler AI]], [[Findem]], [[Hover]], [[Landing AI]], [[Lightning AI]], [[Lilt]], [[Primer]], [[Snorkel AI]], [[Woebot]], [[Eightfold AI]], [[Eightfold AI]], [[Graphcore]], [[Instabase]], [[Labelbox]], [[Matroid]], [[Nuro]], [[Orbital Insight]], [[Osaro]], [[Predibase]], [[SambaNova Systems]], [[Scale AI]], [[Snappr]], [[Sourcegraph]], [[Symbio Robotics]], [[Luminous Computing]]

[[ABBYY]], [[Aurora Innovation]], [[Automation Anywhere]], [[CrowdStrike]], [[Cruise]], [[Databricks]], [[Intel]], [[Matterport]], [[Palantir]], [[SRI International]], [[SentinelOne]], [[SoundHound]], [[Upstart]], [[Waymo]], [[Zoox]], [[AI21 Labs]], [[Midjourney]], [[Mashgin]], [[Mythic AI]], [[Standard AI]], [[Sift]], [[Eightfold AI]], [[Symbio Robotics]], [[Perplexity AI]], [[DeepMind]], 


[[Adobe]], [[Alibaba]], [[Baidu]], [[Hugging Face]], [[IBM]], [[OpenAI]], [[Salesforce]]
[[Tome]], [[Tavus]], [[Aqemia]], [[Codeium]], [[ElevenLabs]], [[Hippocratic AI]], [[Iambic Therapeutics]], [[AssemblyAI]], [[PlayHT]], [[Biomatter]], [[Colossyan]], [[DeepBrain AI]], [[Insilico Medicine]], [[Synthetaic]], [[Tabnine]], [[Etcembly]], [[Gridspace]], [[Synthesis AI]], [[Clarifai]], [[CopyAI]], [[MOSTLY AI]], [[Plask]], [[Sudowrite]], [[Syntho]], [[GenRocket]], [[LOVO (Genny)]], [[Latitude]], [[Revery AI]]
[[Lightricks]], [[Mistral AI]], [[Paige AI]]
[[Runway]], [[Synthesia]], [[Anyscale]], [[Inworld AI]], [[Forethought]], [[Eightfold AI]], [[Replika]], [[Symbio Robotics]]
[[Anthropic]], [[Inflection AI]], [[Apple]], [[Google]], [[Meta (Facebook)]], [[Microsoft]], [[NVIDIA]],
[[Cohere]], [[Jasper]], [[Gong]], [[Stability AI]]
[[Amazon]]


### GOAL: Build a list of Bay Area Tier II Interviewing Targets.

#### (1)  Find Bay Area-compatible AI companies.  (SFAI)

"Build a list of up to 500 AI-based companies with offices in the San Francisco Bay Area.  Format this as a list of mark-down links:  - [company name](https://company_url.com)"

"This list should be broad, and should include both established and startup companies in two different lists which can each be up to 500 companies."


#### (2) Within a list of companies find those doing Applied Research in  Deep Learning. (DLP)

Please find projects that involve  Applied Research in Deep Learning / Generative AI.
- Only consider projects that involve one of the following companies:

- Format your output as a tab-separated-values table with columns:  name, kind, dlproj1, dlproj2, ..., where 'name' is the company name and 'kind' is 'Big'
- For each company, provide a list of up to 9 generative AI projects in which they are involved, followed by web links to related papers or descriptions when possible.


#### (3) Identify companies that are growing, well funded.  (CASH)

Please look for signals of growth across the following companies:

- Use 10-K and 10-Q filings along with other sources to determine a 'cash_trend' as: down, flat, upward, or spiking.
- Also, look for recent funding announcements and provide links to those announcements under 'funding'
- For each company, compute a 'cash_available' as the total funding in the last large round divided by years since the funding event (expressed as a floating point number)
- Format your output as a markdown table.  Column heading should be:  "name", "cash_trend", "funding", "cash_available" where cash_available is expressed as an integer number of millions per year in funding


- Summarize other signals about the financial strength of a company as cash_other

yes use media reports. do your best with uncertain funding. no threshold but generally want to pick the funding round that will yield the largest fraction given its time in the past


#### (4) Within a list of companies, find as many AI-related job postings as possible.


Provide all data in a markdown table and ensure all table content is correctly escaped for Markdown parsing, including newlines as `<br>` and pipes (`|`) as `\|`.

Please find as many job postings as possible:
- Requiring technical experience and leadership in AI
- That are either remote jobs or are based in the San Francisco Bay Area.
- Only consider posting from the following companies:

- Use the following columns:  name, job_remote, and job_bay
- 'name' is the company name, 'job_remote' contains remote jobs, while 'job_bay' has Bay Area posting.
- Each job posting should have the posting's title followed by the bare text of a Google search URL (without quotes) that will return the posting.  For example:   "Senior AI Engineer - https://google.com/serach?..."

- Provide data as a markdown table and ensure all table content is correctly escaped for Markdown parsing, including newlines as `<br>` and pipes (`|`) as `\|`.

- Provide the data as a tab-separated values (TSV) table.

- Provide all data in a markdown table and ensure all table content is correctly escaped for Markdown parsing, including newlines as `<br>` and pipes (`|`) as `\|`.


- Each job posting should have the posting's title followed by a markdown URL for a Google search (without quotes) that will return the posting.


Eightfold AI, Labelbox, Landing AI, LangChain, Lightning AI, Lilt, Luminous Computing, Matroid, Moveworks, Nuro, Orbital Insight, Osaro, Pinecone, Predibase, Primer, Rasa, Replika, Runway, 

SambaNova Systems, Scale AI, Skydio, Snappr, Snorkel AI, Sourcegraph, Symbio Robotics, Synthesia, Uniphore, Vectara, Weights & Biases, Woebot

AI21 Labs, Aqemia, AssemblyAI, Biomatter, Clarifai, Codeium, Colossyan, CopyAI, DeepBrain AI, ElevenLabs, Etcembly, GenRocket, Gridspace, 

Hippocratic AI, Iambic Therapeutics, Insilico Medicine, LOVO (Genny), Latitude, MOSTLY AI, Midjourney, Plask, PlayHT, Revery AI, Sudowrite, 

Synthesis AI, Synthetaic, Syntho, Tabnine, Tavus, Tome, Eightfold AI, Symbio Robotics, Mashgin, Mythic AI, Sift, Standard AI




#### .


## Criteria

I am beginning a job search and would like to build up a list of relevant companies that fit my experience and interests.
For each company you identify, please provide your assessment of each company against each criterion as well as a summary of the evidence you used to arrive at that company's score on each criterion.

### CRITERIA FOR AN IDEAL COMPANY
#### **LOCATION**: Bay Area or Remote.  I live in the San Francisco Bay Area and want to work most days from here.  I am open to remote work and some travel, but I want to be based here.
#### * **PAY**: The company typically pays engineering-related roles a total comp above $300-500K.
#### * **INNOVATIVE PROJECTS**: The ideal company has many deeply technical AI projects, ideally tied to recent advances in deep learning and generative AI.
#### * **ASCENDANT**: The company is poised for notable growth in the next years.  At the very least, the company should not be flat, stagnant, or shrinking.
#### * **GOOD CULTURE**:  Ideally, this company is known for having a great corporate culture that is loved by its employees.  At the very least, I would consider it a notable negative if the company had a reputation as being toxic, cutthroat, or sweatshop-like.

#### * **FOUNDATIONAL MODELS**: A company gets "extra credit" if they are investing significantly in building foundational AI models.


### CRITERIA FOR AN IDEAL ROLE (AS FOUND BY CONSIDERING PROJECTS / JOB POSTINGS SEE AT FOR THE COMPANY)
#### * **AI-RELATED**: I have a PhD in AI and I want to be working in a role that notably leverages this background.
#### * **CUTTING EDGE DEEP LEARNING**: Ideally, the role will involve cutting-edge AI building upon the latest algorithms in deep learning and/or involve novel AI algorithm work.  I would like the work to be as close as possible to being research in AI, but the job should not require recent academic publications as mine are quite out of date.
#### * **TECHNICAL**:  My activities on a day-to-day basis should be deeply technical.
- My work might include mentoring or brainstorming with more junior engineers on how they might approach aspects of their problem-solving exploration.
- Additionally, I enjoy API work, novel algorithm development, and brainstorming ways to apply AI technologies in novel ways to solve difficult tasks.
#### * **LEADERSHIP**: The role should leverage my management experience to have a larger scope and pay. Still, I prefer more hands-on technical leadership rather than more people or project management aspects.


### CRITERIA FOR IDEAL PROJECT.  (AS FOUND BY CONSIDERING PROJECTS / JOB POSTINGS SEE AT FOR THE COMPANY)
#### * **GROUNDBREAKING**: Ideally, my team's project is breaking new ground at a technical level such that the correct approach is not known, as the task is so new that an established body of practice has not yet been created.  Our team is making its way on this first-of-its-kind product by trial and error.
#### * **PROTOTYPING**: I enjoy and have the most experience building first-of-a-kind prototypes. I have less experience and interest in scaling up solved solutions for mass adoption.



#### * **COHESIVE PRODUCT:**


- AI-RELATED:  I have a PhD in Computer Science specialized in Machine Learning and would like to use this in doing AI work closely related to the recent revolution in deep learning.
- CUTTING EDGE BUT NOT PURE RESEARCH:  I am most 
- SF BAY AREA: I live in San Francisco and prefer not to relocate, so a job choice needs to be consistent with this.
- I am interested in doing cutting-edge applied AI work.  
- but it has been too long since I was publishing, so it may not be feasible for me to get a pure Research position.



## .



# LOG

### 2025-02-24  [[NJ.1]]


### 2025-02-17  Jobs Search Strategy:

I am interested in digging deep in multiple locations in order to find job postings for myself.
I have a PhD in AI and decades leading teams of up to 60 people. Thus, I am interested in any and all jobs that would be applicable to me.  CTO, VP eng, ML engineer, but really any AI-focused technical role at these companies would be of interest. 

In order to do a through search what web locations should we consider?



Set One (Three Steps):
	1.	Use LinkedIn with detailed keyword searches (e.g., “CTO AI,” “VP Engineering Machine Learning,” “Director of AI Research”) and advanced filters for location, experience level, and company size.
	2.	Check general job boards (e.g., Indeed, Glassdoor, Monster) for comprehensive listings, using filters to target AI or technical leadership positions.
	3.	Explore AngelList and CrunchBoard to find AI-focused startups or smaller tech firms seeking high-level talent.
OK

Set Two (Three Steps):
	1.	Visit company career pages for major tech and AI companies (e.g., Google AI, DeepMind, Meta AI, Microsoft, Amazon, IBM Research) to view unadvertised or newly posted senior roles.
	2.	Monitor specialized AI portals, including communities such as Kaggle (jobs board), Papers With Code (open calls), and Ai-jobs.net (curated AI positions).
	3.	Leverage university research labs or government research organizations (e.g., NASA, DARPA-related projects) that post senior-level AI roles online.
OK

Set Three (Three Steps):
	1.	Search executive-level job sites (e.g., The Ladders, Hired.com) and connect with executive recruiters who specialize in C-level tech placements.
	2.	Utilize professional forums and Slack/Discord communities (e.g., ML collectives, AI meetups) where members often share niche or stealth hiring opportunities.
	3.	Consider remote job boards (e.g., We Work Remotely, Remote.co, Toptal) to uncover AI leadership or expert contract roles that might not appear on mainstream sites.
OK

Places to look:  LinkedIn, Angellist, ZipRecruiter, AI Careers Hub, 

### 2025-02-17  Job Search



- I am interested in digging deep in multiple locations in order to find job postings for myself.
- For context, I have a PhD in AI and decades leading teams of up to 60 people. Thus, I am interested in any and all jobs that would be applicable to me.  CTO, VP eng, ML engineer, but really any AI-focused technical role at these companies would be of interest. 
- I live in San Francisco, so I am only interested in roles within the bay area, or that allow remote work.
- Please provide results in a CSV with two columns: name, jobs.  company name, and a list of job titles as markdown web links in this format:   [short posting descriptive phrase](http://url.to.posting.com/jobpost1)
- Please dig deep and use all avenues you listed above on each company in my list below:

Cohere, Forethought, Gong, Hugging Face, Jasper, Stability AI

- Only consider full-time positions. Those role titles are only examples; any AI-related role is of interest.  Please consider all relevant positions; there is no area focus.
- But infrastructure roles, like dev-ops, ML-ops, or backend engineer, are too general and do not leverage my PhD.  Those are not of interest.



### 2025-02-17  Have Funding

I have a list of generative AI startup companies, and I would like understand which are in best position to be aggressively hiring.  One dimension of that is to look at the level of funding they have and how long ago did they raise this funding.  How can we best investigate this, and how can we summarize this into a kind of numeric score for each company indicating their relative ability / propensity to hire based on funding?

~
base_score = recent_funding / (months_since_last_funding + 1)

if months_since_last_funding < 6:
    bonus_factor = 1.5
elif months_since_last_funding < 12:
    bonus_factor = 1.25
else:
    bonus_factor = 1.0

final_score = base_score * bonus_factor

~

Please use the piece wise method above and calculate over the companyies listed below.  Present the results as a CSV with columns: name, funding, justification where name is the company name, funding is the formula result, and justification provided details about the funding result including any relevant details about the companies funding that are not covered in the formula.  Companies:  


### 2025-02-17  Are Hiring



### 2025-02-15  Toxicity

I want to look online to assess how toxic or supportive the following companies are.  both articles about the companies, reviews like on glass door and other sources should be used to provide a score from 0 to 10 indicating toxic/unsupportive at 0 and not-toxic and very supportive at 10.   Provide your output as a downloadable CSV file with four columns: company name, score, summary of score as a phrase, detailed justification and references for score.

Please use the last 2 years unless you cannot find sufficient information, then you can go back further.

Here is the list of companies to consider:
Adobe, Alibaba, Apple, Baidu, Google, IBM, Meta, Microsoft, NVIDIA, Salesforce



### 2025-02-14  Companies doing innovative AI work

- TASK: Please compile a list of 20 to 50 companies that fit this, along with a summary of the generative AI applications each is working on.
- TASK: Please expand the provided list of companies to a full list of 70 relevant companies and return the list as a CSV as indicated.

- CONTEXT: I am beginning a job search and would like to build a list of relevant companies that fit my experience and interest.
- ABOUT ME: I have a PhD in Computer Science with a specialization in AI / Machine Learning, and have built/lead teams of 60+ PhDs and Engineers.
- GEN AI: I am interested in companies doing research extending recent algorithms in generative AI into novel application areas.
- LOCATION: Since I live in San Francisco, I am only interested in companies with offices in the San Francisco Bay Area or ones that allow remote work.
- TWO LISTS:  I am interested in both startups and large companies; please compile these as two separate lists
- I am more broadly interested in novel applications of generative AI than specific industries.  That said, I have experience in Robotics and do enjoy that field.
- FORMAT:  Please format the output as a CSV with the following columns:  
		Company type(Big or Start), Company name, Company mission,
		GenAI project1, GenAI project2





