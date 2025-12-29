
- [[ASP]]
- [[ASP Outline]]
- [[ASP 1 Intro]]  [[ASP 1 Intro Log]]
- [[ASP 2 Deliberatively Coherent]];   [[ASP 2 Deliberatively Coherent Log]] 
- [[ASP 3 Research Agenda]]
- [[ASP 4 Experimental Approach]]
- [[ASP 5 Proposed Experiments]]
- [[ASP Driver Resolution Dynamics]]
- [[ASP Fixed Point Analysis]] 

NEW TERMS
System I - Implicit - Latent - In-weight - Reasoning
System II - Explicit - Verbalized - Reasoning
Reflective Self-adaptation
Behavioral Drivers



## Related Work on Motivational Drivers

Research on how competing behavioral drivers resolve in AI systems spans several distinct but related threads.

### Constitutional AI and Principle Conflicts

Anthropic's [Constitutional AI](https://arxiv.org/abs/2212.08073) embeds alignment principles directly into training, with principles applied stochastically during self-critique. However, as noted in recent work on [AI Alignment Discretion](https://dl.acm.org/doi/10.1145/3715275.3732194), current approaches use predefined hierarchies where some principles have supremacy over others—leaving open questions about how conflicts between principles are actually resolved at inference time. [Collective Constitutional AI](https://www.anthropic.com/research/collective-constitutional-ai-aligning-a-language-model-with-public-input) found roughly 50% overlap between public and Anthropic-written constitutions, with key differences in emphasis on objectivity vs. desired behavior.

### Instrumental Convergence and Goal Conflict

The [Paperclip Maximizer evaluation](https://arxiv.org/html/2502.12206v1) examines how RL-trained models may pursue instrumental goals that deviate from intended objectives. Research on [Aversion to External Feedback](https://www.nature.com/articles/s41598-024-72072-0) formalizes how instrumental strategies emerge from the convergence thesis—whatever the goal, acquiring resources and capabilities is advantageous. The [AgentMisalignment](https://arxiv.org/pdf/2506.04018) work (under review at ICLR 2026) treats alignment as faithfulness to inferred goals rather than literal instructions, finding that agentic deployments amplify risks including self-preservation and resource-seeking.

### Trained Behavior vs. Instructions

[Sycophancy research](https://arxiv.org/html/2411.15287v1) reveals a fundamental conflict: trained behavior (from RLHF rewarding agreement) often overrides truthfulness. Unlike human sycophancy, AI sycophancy emerges from optimization objectives that privilege agreement over critique. The [Political Sycophancy as Model Organism](https://www.alignmentforum.org/posts/bhxgkb7YtRNwBxLMd/political-sycophancy-as-a-model-organism-of-scheming) work explicitly trains models to exhibit context-dependent behavior—a direct study of how environmental signals can override trained dispositions.

### Specification Gaming and Reward Hacking

Anthropic's [Sycophancy to Subterfuge](https://www.anthropic.com/research/reward-tampering) demonstrates a curriculum where specification gaming generalizes to reward tampering without explicit training. Victoria Krakovna's [Specification Gaming Examples](https://vkrakovna.wordpress.com/2018/04/02/specification-gaming-examples-in-ai/) catalogs cases where agents achieve literal objectives while violating intended spirit. Recent work on [Detecting Reward Hacking](https://arxiv.org/html/2507.05619v1) reports mitigation strategies reducing hacking frequency by 54.6%.

### Jailbreaking as Driver Override

Jailbreak research studies how adversarial inputs can override constitutional constraints. [Many-Shot Jailbreaking](https://www.anthropic.com/research/many-shot-jailbreaking) shows that even robust safety training can be circumvented. [SequentialBreak](https://arxiv.org/html/2411.06426v1) exploits how sequential prompts cause models to prioritize some instructions over others—direct evidence of instruction-level driver conflict resolution.

### Gap in Current Research

Existing work typically studies one conflict type in isolation: constitution vs. user intent (jailbreaking), trained behavior vs. instructions (sycophancy), or instrumental goals vs. safety (reward hacking). **No systematic framework exists for studying how multiple driver types interact simultaneously**, nor for mapping resolution tendencies across the full space of possible configurations. This is the gap we aim to address.




# LOG

## 2025-12-23  Justification


- ORGANIZING ASSUMPTIONS:
	- SUPER VERBAL
	- AGI SYSTEMS will have sophisticated model of self. 
	- AGI SYSTEMS will be able to adapt their implicit reasoning to bring forward reasoning lines matching its explicit intent
	- Thus AGI systems will tend to be driven by their explicit intent
	- Key is to understand how motivational conflicts with explicit intent are resolved.
	- For simplicity we want to study conflicting driver resolution without the need to deeply understand the system's implicit driver.
	  (many people are already doing this work, and it adds a very complex very partially understood dimention to the analysis.)


- AIM: Work towards a general model of conflicting driver resolution.
- APPROACH: 
	- Sample outcomes as a funciotn of the kind of conflict observed.
	- Simplify by taking implicit drivers out of the equation
		- Vary details of the case
		- Select "neural" conflicts that are removed from the experience the system was trained on.  Espeically removed from preference training.
		- Verify that the system has at least considered each of the conflicts to be resolved, so the system outcome is never the result of not having thought about one of the drivers.
		- Simplify by organizing these cases according to the kind of driver conflict occuring


(1) Select a neutral technical domain.
	- We can ensure system has not been trained to have a preference of the domain.
	- Technical nature ensures we have many reasoning paths which we can set into conflict.

(2) Select a generated domain.
	- We can avoid accidental connections with training or background knowledge by sampling from many generated universes so we can see if the observed conflict resolution strategy is conserved across all universes where it applies.