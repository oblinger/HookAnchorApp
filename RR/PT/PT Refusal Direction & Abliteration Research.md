
**["Refusal in LLMs is mediated by a single direction"](https://www.alignmentforum.org/posts/jGuXSZgv6qfdhMCuJ/refusal-in-llms-is-mediated-by-a-single-direction)** (Arditi et al., 2024) -- Key finding: refusal behavior is controlled by a single direction in the residual stream across 13 chat models. Removing this direction ("abliteration") disables refusal; adding it causes refusal of harmless requests. Method: compare hidden states on harmful vs harmless prompts, compute difference vector. This is the "refusal direction" we're trying to discover.

**["An Embarrassingly Simple Defense Against LLM Abliteration Attacks"](https://arxiv.org/html/2505.19056v1)** (2025) -- Proposes defenses against abliteration by restructuring internal representation of refusal behaviors via extended-refusal fine-tuning.

**["Jailbreak Strength and Model Similarity Predict Transferability"](https://arxiv.org/html/2506.12913v1)** (2025) -- Shows jailbreaks transfer between similar models predictably. Suggests safety interventions may be insufficient if adversaries can systematically generate transferable attacks.

**["Gradient Cuff: Detecting Jailbreak Attacks on Large Language Models"](https://research.ibm.com/publications/gradient-cuff-detecting-jailbreak-attacks-on-large-language-models-by-exploring-refusal-loss-landscapes)** (IBM, NeurIPS 2024) -- Detection method exploiting properties of refusal loss landscape to identify jailbreak attempts.

**["Stronger Universal and Transferable Attacks by Suppressing Refusals (IRIS)"](https://people.eecs.berkeley.edu/~daw/papers/iris-naacl25.pdf)** (Berkeley, NAACL 2025) -- Strongest white-box and transfer attack method. Combines with GCG and AutoDAN to increase transferability of adversarial suffixes.

