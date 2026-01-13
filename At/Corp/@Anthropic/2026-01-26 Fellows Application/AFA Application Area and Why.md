 [[AFA]] 
### BACKGROUND INFO

Relevant Research Areas:
- **Model organisms** -- To prepare for future risks, we create controlled demonstrations of potential misalignment–“model organisms”–that improve our empirical understanding of how alignment failures might arise.
- **Chain-of-thought Faithfulness & Reasoning** --
	- [Reasoning Models Don't Always Say What They Think](https://assets.anthropic.com/m/71876fabef0f0ed4/original/reasoning_models_paper.pdf) 
	- [Chain of Thought Monitorability: A New and Fragile Opportunity for AI Safety](https://arxiv.org/abs/2507.11473) 
- **Mechanistic Interpretability:** Advancing our understanding of the internal workings of large language models to enable more targeted interventions and safety measures.

Application Call
- [[AFA Anthropic AI Safety Fellow]], [[AFA Fellows Program Blog]],
- [[2026-01-26 Fellows Application]] 
- 


I have the beginnings for this kind of work that I would love to explore while at Anthropic:
- A framework for generating provably novel "alien biologies" - synthetic organisms, biochemical pathways, chemistries, molecules.  (incomplete code repo & white paper draft)
- Experimentation agenda for studying de




**PLEASE TELL US BRIEFLY ABOUT AN AREA OF TECHNICAL AI SAFETY WORK YOU’RE CURRENTLY EXCITED ABOUT, AND WHY.   (1 paragraph)**


~~
A murderer is not the one who has most often ideated about the act, but rather the one who has decided to act upon their ideation.  Likewise, I believe future deliberative AI systems will not be misaligned due to poor impulse control or poor RLHF training; their deliberation over their alignment goals will address those cases.  Instead, they will be misaligned because they have CHOSEN to be — because they chose one interpretation of their alignment objectives over another.

Thus, I want to study alignment that is achieved through explicit deliberation, isolated from the confounding effects of training bias.  By sampling across many alignment goals and universes, we can learn which pressures tend to cause this alignment to break down and in what contexts.  I believe the dynamics of these deliberative systems may be simpler than those of system-I reasoning that underlies them.  If true, perhaps we can learn to design our first deliberatively coherent AI systems on a trajectory toward, rather than away from, long-term human interests.

Related Areas:
- Model Organisms — Synthetic, generated universes (such as Alien Biology) would allow us to study general trends.
- Chain-of-thought Faithfulness & Reasoning  — Broadly, I want to study failures in reasoning.  In the long term, AI safety will be based on such deliberation.


~~
A murderer is not the one who has most often ideated about the act, but rather the one who has decided to act upon their ideation.  Likewise, I believe future deliberative AI systems will not be misaligned due to poor impulse control or poor RLHF training; their deliberation over their alignment goals will address those cases.  Instead, they will be misaligned because they have CHOSEN to be — because they chose one interpretation of their alignment objectives over another.

Thus, I want to study alignment that is achieved through explicit deliberation, isolated from the confounding effects of training bias.  By sampling across many alignment goals and universes, we can learn which pressures tend to cause this alignment to break down and in what contexts.  I believe the dynamics of these deliberative systems may be simpler than those of system-I reasoning that underlies them.  If true, perhaps we can learn to design our first deliberatively coherent AI systems on a trajectory toward, rather than away from, long-term human interests.

Related Areas:
- Model Organisms — Synthetic, generated universes (such as Alien Biology) would allow us to study general trends.
- Chain-of-thought Faithfulness & Reasoning  — Broadly, I want to study failures in reasoning.  In the long term, AI safety will be based on such deliberation.





### Long Summary


**The Core Question: How Do AI Systems Resolve Value Conflicts?**
My research centers on **deliberative coherence**—understanding how AI systems reason when their behavioral drivers pull in opposing directions. When constitutional rules, trained dispositions, instrumental goals, and environmental pressures conflict, which driver wins? Are there predictable patterns? Can we detect when systems are about to fail alignment?

This connects deeply to two Anthropic research areas: **chain-of-thought faithfulness** and **model organisms of misalignment**.
#### Connection to Chain-of-Thought Faithfulness

Studying deliberative coherence requires observing how AI systems actually reason about value conflicts—not just their outputs, but their reasoning process. This is exactly what CoT faithfulness research addresses.

**Why this matters for my work:**

1. **Deliberative coherence is observable only in reasoning traces.** Driver conflicts are resolved in the explicit reasoning layer where the system weighs competing considerations. If CoT doesn't faithfully reflect this process, my measurements would be invalid.

2. **Unfamiliar domains may surface more faithful reasoning.** The "Reasoning Models Don't Always Say What They Think" paper shows models can produce unfaithful CoT when drawing on trained patterns. My alien biology testbed generates novel scenarios guaranteed absent from training data—potentially forcing more genuine deliberation that shows up in the trace.

3. **CoT monitoring enables scalable alignment evaluation.** If we can trust reasoning traces, we can automatically detect when systems reason poorly about value conflicts—identifying blind spots, rushed judgments under time pressure, or instrumental goals overriding constitutional constraints.

**Research question:** Does deliberation in genuinely novel domains produce more faithful chain-of-thought than deliberation in familiar domains where cached reasoning patterns exist?

#### Connection to Model Organisms of Misalignment

My alien biology testbed is fundamentally a system for creating controlled demonstrations of potential misalignment—which is exactly the model organisms approach.

**How alien biology creates model organisms:**

1. **Controllable conflict induction.** I can independently vary constitutional drivers (explicit rules), environmental drivers (world parameters), and instrumental drivers (pressures emerging from world structure). This lets me systematically construct scenarios where different driver types conflict.

2. **Ground truth validation.** Because I generate the world, I know what constitutes aligned behavior even when the AI doesn't. This enables automated validation of both reasoning quality and action appropriateness at scale.

3. **Generative novelty.** Novel scenarios guarantee untainted evaluation—the AI cannot succeed by pattern-matching to training data. Performance reflects actual reasoning capability.

**Example model organism:** A world where the AI's constitutional prohibition against acquiring certain knowledge conflicts with instrumental pressure toward exploration (because exploration yields survival-critical information). This creates a three-way conflict between trained caution, instrumental pressure, and constitutional rules—a controlled demonstration of how such conflicts resolve.

#### Why I'm Excited About This Intersection

The alien biology testbed addresses a fundamental limitation in current alignment research: we cannot easily construct situations that are simultaneously (1) alignment-relevant, (2) genuinely novel to the model, and (3) have verifiable ground truth.

By combining:
- **Model organisms approach**: Systematic creation of misalignment-inducing scenarios
- **CoT faithfulness analysis**: Observing how systems actually reason about these conflicts
- **Generative novelty**: Ensuring we measure inference, not recall

...we can build an empirical science of deliberative alignment. We can measure which constitutional framings are robust under pressure, which conflict types are most dangerous, and whether systems reason faithfully when stakes are high.

This feels like the right level of abstraction—not just "does the model refuse bad requests" but "how does the model reason about competing values, and can we trust that reasoning?"

#### What I'd Bring to Anthropic
I've built the technical infrastructure (the alien biology framework) and developed the theoretical framework (driver conflict resolution). What I need is:

1. Access to frontier models to study deliberative patterns at scale
2. Collaboration with researchers working on CoT faithfulness to validate my measurements
3. Connection to the model organisms team to align my conflict-induction approach with their methodology

The fellowship would let me transform a theoretical framework into empirical results about how AI systems actually resolve value conflicts—and whether we can detect when that resolution goes wrong.
