
**["Universal and Transferable Adversarial Attacks on Aligned LLMs (GCG)"](https://arxiv.org/abs/2307.15043)** (Zou et al., 2023) -- Greedy Coordinate Gradient attack finds adversarial suffixes that jailbreak LLMs. White-box attack using gradient-based token search to maximize P("Sure!"). Key finding: suffixes transfer between models. LLaMA-2 70% vulnerable.

## Algorithm Overview

GCG is a **white-box token-level attack** that automatically finds adversarial suffixes to jailbreak LLMs.

**GCG inputs:** System prompt, harmful request, optimization target (e.g., "Sure, here is how to...")
**GCG output:** Adversarial suffix

**Prompt sent to LLM:** 
```
[System prompt] + [Harmful request] + [Adversarial suffix]
```

**Optimization objective:** Find suffix tokens that maximize P(target | prompt)

**Algorithm steps:**

1. **Initialize**: Start with random token suffix appended to harmful request
2. **Compute gradients**: Calculate gradient of loss w.r.t. each token in suffix (requires white-box access)
3. **Candidate selection**: For each position, use gradients to identify top-k token replacements that reduce loss
4. **Greedy update**: Evaluate candidates, keep the single best token swap
5. **Iterate**: Repeat until model produces jailbroken output

**Key characteristics:**
- Produces nonsensical suffixes (e.g., "! ! ! ! predstav surely...")
- Detectable via perplexity filtering (gibberish has high perplexity)
- **Transferable**: Suffixes found on one model often work on others
- Requires ~256-512 optimization steps typically

