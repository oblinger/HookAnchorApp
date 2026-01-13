
- https://constellation.fillout.com/anthropicfellows

Resume: 					  **{{NICK:  Will be a pretty general one.  could slap desire to do AI safety on the top, but it feels disingenous}}**
Google Scholar:		https://scholar.google.com/citations?hl=en&user=NyU2n1IAAAAJ  
LinkedIn:
GitHub:
PersonalWebsite:      https://oblinger.github.io/
Other Links:

[[AFA Share Links Here]]  
**SHARE LINKS HERE:**
**Please share code samples from past projects, ideally a substantial project and (if possible) a machine learning project.**
**Please briefly describe your role and contributions for each sample.**

Was the initial developer and architect for a CV/robotic platform that grew to have 60 developers. The entire system is proprietary property of Aeolus Robotics.
Active developer of many novel CV algorithms at SportsVisio -- all proprietary except DAT an elegant/small data management layer that I open sourced:

DVC-DAT - It provides flexible namespace management that integrates dynamically loaded code with cloud datasets, ensuring versioning is synchronized across them and your code repository.  Code:  https://github.com/oblinger/dvc-dat    Example Usage:  https://nbviewer.jupyter.org/github/oblinger/DVC-DAT/blob/main/examples/do_examples.ipynb

ALIEN BIOLOGY - Framework for generating controllable synthetic alien biology for use in taint-free testing of agentic AI.
- Elegant, general, and biologically plausible. Design optimized to run efficiently on GPU.
  Docs: https://oblinger.github.io/abio-docs/  Code: https://github.com/oblinger/alienbio 

HOOK ANCHOR - I am opinionated about how personal knowledge management should be done, so over the summer I implemented this vision as a 50K-line Rust application with a 3K-line JavaScript user configuration layer on top.  Code:  https://github.com/oblinger/HookAnchorApp  Docs:  https://oblinger.github.io/gitproj/HookAnchor    Teaser:  https://oblinger.github.io/HookAnchor/  

I am a very fast, strong, organized coder, but most of my work is proprietary.  I developed the basic Alien Bio framework in ten days over Christmas break, so its code and docs are a good indicator of my quality and velocity.


**Code Sample:  (Drag & Drop)**   



[[AFA Why Are You Interested]] 
**WHY ARE YOU INTERESTED IN PARTICIPATING IN THE AI FELLOWS PROGRAM?   (1-2 paragraphs)**

TL;DR. I want to transition to full-time AI safety work and I have a research agenda ripe for execution within your program.

I have long been interested in systems that achieve general intelligence, as well as the risks inherent in such systems. As a Program Manager at DARPA, I ran "Bootstrapped Learning," an effort to iteratively capture deeply structured human knowledge, and created "Machine Reading," a vision for how NLP and ML might be productively integrated to drive automated deep structure acquisition—a $150M program that led to IBM's Jeopardy-playing system (Watson) and hundreds of publications driving early deep learning work by researchers central to the field.

Yet even then, I harbored doubts about our ability to control these systems: before joining DARPA, I wrote a 4-page note to myself reasoning through whether it was acceptable to drive this work, and upon leaving, I chose a more applied AI path rather than continue pushing toward AGI. This tension—between fascination with capable AI systems and concern about their risks—is precisely why AI safety work feels so urgent and personal to me now.

Earlier this year, I drafted the white paper "Alien Biology" and realized it could serve as a platform to drive AI safety work (see Alien Biology whitepaper and Deliberative Coherence in Relevant Background). When I learned of the Anthropic Fellows Program, I dove in to implement the generator/simulator and began outlining the deliberative coherence research agenda. After a decade of wringing my hands on the sidelines, I finally have a direction I believe in for addressing humanity's most pressing threat.




[[AFA Application Area and Why]]
**PLEASE TELL US BRIEFLY ABOUT AN AREA OF TECHNICAL AI SAFETY WORK YOU’RE CURRENTLY EXCITED ABOUT, AND WHY.   (1 paragraph)**

 A murderer is not the one who has most often ideated about the act, but rather the one who decides to act. Likewise, I believe future deliberative AI systems will not be misaligned due to poor impulse control or inadequate RLHF training—their deliberation will address those cases. Instead, they will be misaligned because they have chosen to be, selecting one interpretation of their alignment objectives over another. Thus, I want to study alignment achieved through explicit deliberation, isolated from the confounding effects of training bias. By sampling across many alignment goals and synthetic universes, we can learn which pressures cause this alignment to break down and in what contexts. I suspect the dynamics of these deliberative systems may be simpler than the System 1 reasoning that underlies them—and if true, perhaps we can learn to design our first deliberatively coherent AI systems on a trajectory toward, rather than away from, long-term human interests.

  Related Areas:
  - Model Organisms — Synthetic, generated universes (such as Alien Biology) allow study of general trends.
  - Chain-of-thought Faithfulness & Reasoning — Broadly, I want to study failures in reasoning. In the long term, AI safety will depend on such deliberation.





[[AFA Relevant Background]] 
**(Optional) Please share any relevant AI safety background you have and provide links where possible (e.g., research experience, coursework, self-directed study, past roles, relevant projects).**

I have organized my thinking over the last year into three draft papers/outlines that indicate the direction of this thinking:

ALIEN BIOLOGY WHITEPAPER

Most AI benchmarks suffer from training contamination: we cannot distinguish genuine reasoning from pattern-matching against memorized solutions. The Alien Biology framework addresses this by generating procedural universes guaranteed untainted by training corpora. These synthetic ecosystems preserve the structural complexity of real biological reasoning--feedback loops, multi-level interactions, emergent behaviors--while ensuring every detail is novel. The framework is parametrically controllable, varying task complexity across dimensions like reasoning depth, abstraction layers, and information availability. This enables systematic measurement of where agentic AI systems transition from competent to confused.

DELIBERATIVE COHERENCE

This introduces "deliberative coherence" as a theoretical lens for understanding alignment in future AI systems. A deliberatively coherent system possesses three capabilities: self-understanding (the ability to predict and model its own behavior), self-adaptation (the ability, directly or indirectly, to adapt the way it reasons), and exhaustive deliberation (given sufficient stakes, will reason about anything within reach). The central conjecture is that future AI systems will be deliberatively coherent--not as a hope, but as an inevitability driven by competitive pressure and architectural trajectory. Even systems not given direct mechanisms for self-modification will find indirect ways to adapt their thinking toward their objectives. If true, this reframes the alignment question: rather than asking whether we can make systems safe through training, we ask what will the failure modes of deliberately coherent systems be?

EXPERIMENTAL ROADMAP

This outlines experiments using Alien Biology to study how deliberatively coherent systems fail. Research directions include:
- Constitutional conflicts: when stated objectives contradict each other, how does resolution occur?
- Instrumental pressures: goals that emerge from world structure may push against stated alignment objectives
- Alignment under ignorance: with incomplete knowledge, all actions risk violating alignment objectives in ways the system cannot foresee
- Fixed point analysis: if systems continuously self-adapt toward their objectives, where does this process converge?


All three documents available at:
https://oblinger.github.io/gitproj/DeliberativeCoherence/



**How likely are you to accept a full-time offer at Anthropic if you receive one after the Fellows program? ***
- 100%.  More than all other foundational labs, I feel Anthropic is most serious and open-minded about AI-safety.  This is by far my most effective choice.

**How likely are you to be interested in continuing to work on AI safety after the Fellows program? ***
- 95%.  My goal is to pivot into a safety role focused on existential risk.  As long as the role appears to be a long-term position, I am ready to terminate my current CTO role.


**References:**
**For each Reference: Please share context on their background.**
  - **Include their title, website, Google Scholar, and any other relevant public information.**
  **For each Reference: Please share context on your relationship.**
  - **Include what you worked on together, when and for how long you worked together, how closely you collaborated, and your relationship to them.**


Reference 1:
- Andrew Ng    DeepLearning.AI's Founder   AI Fund's Managing Partner           andrewyantakng@gmail.com

BACKGROUND
Andrew Ng is one of the world's most influential figures in AI and deep learning. He co-founded Google Brain (2011), where his team trained neural networks on 16,000 CPU cores that learned to recognize cats from YouTube videos—a landmark in unsupervised deep learning. He served as Chief Scientist at Baidu and co-founded Coursera, bringing AI education to over 8 million students worldwide. He is currently Founder of DeepLearning.AI, Managing General Partner of AI Fund ($175M+ invested in AI startups), Executive Chairman of LandingAI, and Adjunct Professor at Stanford. His research spans machine learning, deep learning, computer vision, and NLP.
- Website: https://www.andrewng.org/
- Google Scholar: https://scholar.google.com/citations?user=mG4imMEAAAAJ
- LinkedIn: https://www.linkedin.com/in/andrewyng/

RELATIONSHIP
As a DARPA Program Manager (2005-2010), I collaborated with Andrew over multiple programs and led workshops fostering collaboration across research teams. Beyond these formal interactions, we had several deeper one-on-one discussions. Most notable was a day we spent together exploring unsupervised methods for learning deep representations. Andrew was initially skeptical of my ideas until I narrowed one approach toward inducing deep hierarchical and/or trees. He suggested a collaboration with him and one of his students, which ultimately yielded an ICML best paper award on convolutional deep belief networks. Unfortunately, DARPA's general counsel prohibited my formal collaboration, so I was not a coauthor—though the core approach was mine, sharpened by Andrew's skepticism. At the end of that discussion, Andrew further suggested I consider a postdoc with him after DARPA.


Reference 2:
- Nina Mishra   Principal Scientist at Amazon       nmishra@gmail.com

BACKGROUND
Nina Mishra is a Principal Scientist at Amazon with deep expertise in machine learning, data mining, and streaming algorithms. With over 16 years of industry research leadership at Microsoft Research, HP Labs, and Amazon, plus 6 years as Associate Professor at University of Virginia and Acting Faculty at Stanford, she bridges academic rigor with industrial impact. Her foundational work on clustering data streams (7,700+ citations, h-index 34) shaped the field, and her "Robust Random Cut Forest" algorithm is central to Amazon's anomaly detection systems. She has authored ~50 publications in top venues (ICML, NeurIPS, AAAI, VLDB, CRYPTO), holds 14+ patents, and chaired ICML in 2003. Her research was central to the Bing search engine. PhD from UIUC.
- Amazon Science: https://www.amazon.science/author/nina-mishra
- Google Scholar: https://scholar.google.com/citations?user=9yJU0gsAAAAJ
- Stanford page: http://theory.stanford.edu/~nmishra/

RELATIONSHIP
Nina has been a lifelong friend and sounding board for each other's research directions and ideas. We collaborated on a SODA paper with 150 citations.


Reference 3:
- Vittorio Castelli   Senior Director Applied Science at Oracle     vittorio.castelli@alumni.stanford.edu

BACKGROUND
Vittorio Castelli is Senior Director of Applied Science at Oracle Cloud Infrastructure, leading generative AI and NLP efforts. Previously at AWS, he helped launch foundational AI services including Amazon Bedrock for generative AI applications. His 30+ year research career spans IBM Research where he pioneered work in machine learning, information theory, and NLP. With 9,400+ citations (h-index 46), his influential publications include foundational work on semi-supervised learning with Thomas Cover, image database retrieval, and large-scale ML systems. He is currently building Oracle's applied science team for generative AI. Stanford PhD.
- LinkedIn: https://www.linkedin.com/in/vittorio-castelli-3449604/
- Google Scholar: https://scholar.google.com/citations?user=d-lg1lEAAAAJ

RELATIONSHIP
Vittorio and I worked together for three years at IBM Research after winning an Adventurous Research grant—a sought-after internal award for self-directed, open-ended research. We developed a novel inductive method for programming by demonstration and co-authored a half-dozen papers, including a runner-up best paper award.


**Do you have any other commitments or obligations during the Fellows program?**
- I will be on an unpaid leave from SportsVisio during this time.  I have made it clear that this means zero hours per week during this time.

**How Did you hear about the Fellows program?**  
- From multiple colleagues.

**Is there anything else you would like to share?**


{{NICK:  I am a little tempted to state that I recognize that I am not a validated AI-safety researcher, but that I am quite motivated to make the switch if I am able to find a path where I maintain some kind of a salary, and that my background both as a coder and as a researcher suggest I will be strong for the role.  Or is writing something like that at the end a bit too try-hard, or denigrating my application?}}




