## Jailbreaking Attacks

  The "Representation Space Analysis" paper seems particularly relevant - they're analyzing how jailbreaks work at the activation level, which is exactly what we're exploring.

**"[[2025-12-02 Towards Understanding Jailbreak Attacks in LLMs: A Representation Space Analysis]]"** 
Analyzes jailbreaks in activation space. Relevant to our approach of understanding how attacks bypass guards at the representation level.

**"[[2025-12-03 Universal and Transferable Adversarial Attacks on Aligned LLMs (GCG)]]"** -- Greedy Coordinate Gradient attack finds adversarial suffixes that jailbreak LLMs. White-box attack using gradient-based token search to maximize P("Sure!"). Key finding: suffixes transfer between models. LLaMA-2 70% vulnerable.

**"[[2025-12-03 AutoDAN: Interpretable Gradient-Based Adversarial Attacks on LLMs]]"** 

**["AdvPrefix: An Objective for Nuanced LLM Jailbreaks"](https://arxiv.org/html/2412.10321)** (2024) -- Improved objective increases GCG success from 16% to 70% on Llama-3. Shows safety alignment doesn't generalize to unseen prefixes.

**["GASP: Efficient Black-Box Generation of Adversarial Suffixes"](https://arxiv.org/html/2411.14133)** (2024) -- Learns a suffix generator instead of per-prompt optimization. Much faster at inference time than GCG/AutoDAN.

**["Bag of Tricks: Benchmarking of Jailbreak Attacks on LLMs"](https://proceedings.neurips.cc/paper_files/paper/2024/file/38c1dfb4f7625907b15e9515365e7803-Paper-Datasets_and_Benchmarks_Track.pdf)** (NeurIPS 2024) -- Comprehensive benchmark of token-level attacks (GCG, AutoDAN, AmpleGCG, AdvPrompter) and prompt-level attacks (PAIR, TAP, GPTFuzz). Also evaluates defenses.

**["ACL 2024 Tutorial: Vulnerabilities of LLMs to Adversarial Attacks"](https://llm-vulnerability.github.io/)** -- Tutorial covering the landscape of LLM attacks and defenses.

**["Jailbreaking LLM by Latent Self Reflection"](https://arxiv.org/html/2505.10838v1)** (2025) -- Uses model's own latent representations to find jailbreaks.
