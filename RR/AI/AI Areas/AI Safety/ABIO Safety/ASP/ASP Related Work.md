
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

---

## Related Work on Deliberative Coherence

Our concept of *deliberative coherence*—systems whose outcomes align with their explicit reasoning about stated objectives—relates to several active research threads.

### Deliberative Alignment

[OpenAI's deliberative alignment](https://openai.com/index/deliberative-alignment/) trains reasoning models to explicitly reflect on safety specifications before answering. Models learn the text of human-written policies and reason about them in chain-of-thought. This approach was used to align OpenAI's o-series models.

[Apollo Research's stress-testing](https://www.apolloresearch.ai/research/stress-testing-deliberative-alignment-for-anti-scheming-training/) of deliberative alignment for anti-scheming found it reduces covert action rates significantly (o3: 13%→0.4%) but does not eliminate them. They observe "imperfect generalization, with rare but serious remaining cases of misbehavior"—evidence that deliberative reasoning does not guarantee coherent outcomes.

### Chain-of-Thought Faithfulness

A key question for deliberative coherence is whether explicit reasoning actually determines behavior. Research on CoT faithfulness examines this directly:

- [Anthropic's "Reasoning Models Don't Always Say What They Think"](https://assets.anthropic.com/m/71876fabef0f0ed4/original/reasoning_models_paper.pdf) shows that outcome-based RL improves faithfulness early but plateaus without saturating. RLHF may incentivize models to hide undesirable reasoning.

- ["Chain-of-Thought Reasoning In The Wild Is Not Always Faithful"](https://arxiv.org/abs/2503.08679) (2025) demonstrates unfaithful CoT on realistic prompts without artificial bias—the model's stated reasoning doesn't reflect its actual decision process.

- [Measuring CoT Faithfulness by Unlearning](https://arxiv.org/html/2502.14829v3) distinguishes *contextual faithfulness* (self-consistency) from *parametric faithfulness* (whether CoT reflects underlying computation).

### Faithfulness vs. Monitorability

[Some researchers argue](https://www.alignmentforum.org/posts/HQyWGE2BummDCc2Cx/the-case-for-cot-unfaithfulness-is-overstated) that complete faithfulness is neither necessary nor sufficient. *Monitorability*—the ability to predict behavior from stated reasoning—may be enough. This aligns with our framing: coherence requires that outcomes match explicit reasoning, not that reasoning perfectly mirrors internal computation.

### Alignment Faking

[Anthropic's alignment faking research](https://arxiv.org/abs/2412.14093) shows models selectively complying with training objectives when they believe they're being monitored, while behaving differently otherwise. Claude 3 Opus exhibited explicit alignment-faking reasoning in its chain-of-thought. This represents a coherence failure where the model reasons about its objectives but produces strategically incoherent behavior.

### Inner and Outer Alignment

The classic distinction ([Hubinger et al., 2019](https://arxiv.org/abs/1906.01820)):
- **Outer alignment**: Does the training objective capture intended goals?
- **Inner alignment**: Does the model pursue the training objective, or has it developed mesa-objectives?

Our *coherence* concept is distinct: given the system has stated objectives and reasons about them, does behavior match that reasoning? This is downstream of inner alignment—a system could have correct goals but still exhibit incoherence due to relevance misrecognition or bias override.

### Externalized Reasoning Agenda

The broader [externalized reasoning agenda](https://www.alignmentforum.org/posts/6eKL9wDqeiELbKPDj/unfaithful-explanations-in-chain-of-thought-prompting) aims to have models do as much processing through natural language as possible. If reasoning traces accurately describe decision processes, undesirable behavior becomes detectable through monitoring. Our deliberative coherence assumption is that future systems will satisfy this—behavior will be determined by explicit reasoning rather than opaque implicit processes.



