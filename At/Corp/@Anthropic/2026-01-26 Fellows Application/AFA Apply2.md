
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

DVC DAT - It provides flexible namespace management that integrates dynamically loaded code with cloud datasets, ensuring versioning is synchronized across them and your code repository.  

ALIEN BIOLOGY - Framework for generating controllable synthetic "Alien" biological ecosystems for use in taint-free testing of Agentic reasoning.
- Parametrically controlled complexity builds worlds with organisms, predation/symbiosis/etc. build on a synthetic biochemistry with molecules, reactions, cycles, pathways, signaling, etc. LLM serves as an alien biologist, advancing biology and curing disease.
- The framework is elegant/general and biologically plausible. (will be completed and could possibly serve as the basis of my fellowship work)
  Docs:  https://oblinger.github.io/abio-docs/  Code: https://github.com/oblinger/alienbio 



HOOK ANCHOR - I am opinionated about how personal knowledge management should be done, so over the summer I implemented this vision as a 50K-line Rust application with a 3K-line JavaScript user configuration layer on top.  Code:  https://github.com/oblinger/HookAnchorApp  Docs:    Teaser:  

https://oblinger.github.io/gitproj/HookAnchor/README.html

https://oblinger.github.io/gitproj/HookAnchor  

I am a very fast, strong, organized coder, but most of my work is proprietary.  I developed the basic Alien Bio framework in ten days over Christmas break, so its code and docs are a good indicator of my quality and velocity.


**Code Sample:  (Drag & Drop)**   



[[AFA Why Are You Interested]] 
**WHY ARE YOU INTERESTED IN PARTICIPATING IN THE AI FELLOWS PROGRAM?   (1-2 paragraphs)**

TL;DR. I want to transition to full-time AI safety work and I have a research agenda ripe for execution within your program.

I have long been interested in systems that achieve general intelligence, as well as the risks inherent in such systems. As a Program Manager at DARPA, I ran "Bootstrapped Learning," an effort to iteratively capture deeply structured human knowledge, and created "Machine Reading," a vision for how NLP and ML might be productively integrated to drive automated deep structure acquisition—a $150M program that led to IBM's Jeopardy-playing system (Watson) and hundreds of publications driving early deep learning work by researchers central to the field.

Yet even then, I harbored doubts about our ability to control these systems: before joining DARPA, I wrote a 4-page note to myself reasoning through whether it was acceptable to drive this work, and upon leaving, I chose a more applied AI path rather than continue pushing toward AGI. This tension—between fascination with capable AI systems and concern about their risks—is precisely why AI safety work feels so urgent and personal to me now.

Earlier this year, I drafted a white paper, "Alien Biology"—a framework for measuring AI reasoning performance that is provably untainted by reasoning in the training data. More importantly, I believe this platform can drive AI safety work: future AI systems will likely use deliberation to achieve internal alignment and may have instrumental reasons to increase that alignment within themselves and their progeny. Perhaps this affords a natural long-term alignment of their interests with humanity's?

When I learned of the Anthropic Fellows Program, I dove in to implement the Alien Biology universe generator/simulator. After a decade of wringing my hands on the sidelines, I finally have a direction that seems plausible for addressing humanity's most pressing threat.




[[AFA Application Area and Why]]
**PLEASE TELL US BRIEFLY ABOUT AN AREA OF TECHNICAL AI SAFETY WORK YOU’RE CURRENTLY EXCITED ABOUT, AND WHY.   (1 paragraph)**

 A murderer is not the one who has most often ideated about the act, but rather the one who decides to act. Likewise, I believe future deliberative AI systems will not be misaligned due to poor impulse control or inadequate RLHF training—their deliberation will address those cases. Instead, they will be misaligned because they have chosen to be, selecting one interpretation of their alignment objectives over another. Thus, I want to study alignment achieved through explicit deliberation, isolated from the confounding effects of training bias. By sampling across many alignment goals and synthetic universes, we can learn which pressures cause this alignment to break down and in what contexts. I suspect the dynamics of these deliberative systems may be simpler than the System 1 reasoning that underlies them—and if true, perhaps we can learn to design our first deliberatively coherent AI systems on a trajectory toward, rather than away from, long-term human interests.

  Related Areas:
  - Model Organisms — Synthetic, generated universes (such as Alien Biology) allow study of general trends.
  - Chain-of-thought Faithfulness & Reasoning — Broadly, I want to study failures in reasoning. In the long term, AI safety will depend on such deliberation.





[[AFA Relevant Background]] 
**(Optional) Please share any relevant AI safety background you have and provide links where possible (e.g., research experience, coursework, self-directed study, past roles, relevant projects).**

I have organized my thinking over the last year into three draft papers/outlines that indicate the direction of this thinking:

- Alien Biology Whitepaper - The paper that spawned the generative testing framework.  The aim is to provide a
	- controllable
	- taint-free 
	- ???? 

- Deliberative Coherence — Outlines the agenda for studying "deliberative coherence," examining how deliberation-based alignment works and when/how it breaks down.

- Experimental Roadmap -- Outlines how I plan to use the Alien Biology framework to conduct this work.  Here is a thumbnail of these research directions:
	- Testing Inner Alignment - Alien biology provides an ideal testbed for testing how well deliberation achieves inner alignment rather than trained alignment.
	- Deliberation Alignment Failures - Sampling over parametrically generated 

White Paper:  https://oblinger.github.io/gitproj/AlienBiology/AlienBiologyWhitepaper.pdf



**How likely are you to accept a full-time offer at Anthropic if you receive one after the Fellows program? ***
- 100%.  More than all other foundational labs, I feel Anthropic is most serious and open-minded about AI-safety.  This is by far my most effective choice.

**How likely are you to be interested in continuing to work on AI safety after the Fellows program? ***
- 95%.  My goal is to pivot into a safety role focused on existential risk.  As long as the role appears to be a long-term position, I am ready to terminate my current CTO role.


**References:**

Reference 1:
- Andrew Ng    DeepLearning.AI's Founder   AI Fund's Managing Partner           andrewyantakng@gmail.com   

Reference 3:
- Vittorio Castelli   Senior Director Applied Science at Oracle     vittorio.castelli@alumni.stanford.edu  

Reference 2:
- Nina Mishra   Principle Scientist at Amazon       nmishra@gmail.com  




**Do you have any other commitments or obligations during the Fellows program?**
- I will be on an unpaid leave from SportsVisio during this time.  I have made it clear that this means zero hours per week during this time.

**How Did you hear about the Fellows program?**  
- From multiple colleagues.

**Is there anything else you would like to share?**


{{NICK:  I am a little tempted to state that I recognize that I am not a validated AI-safety researcher, but that I am quite motivated to make the switch if I am able to find a path where I maintain some kind of a salary, and that my background both as a coder and as a researcher suggest I will be strong for the role.  Or is writing something like that at the end a bit too try-hard, or denigrating my application?}}




