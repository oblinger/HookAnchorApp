
## GENERATIVE TESTS



## GENERATIVE - SPECIALTY

### [*](https://arxiv.org/abs/2503.13517)2025 CURIE (google sci reasoning)
- [Google Blog Entry](https://research.google/blog/evaluating-progress-of-llms-on-scientific-problem-solving/) 

### 2025 Dynamic Code Eval - **DyCodeEval** - Code writing ability

**Authors:** Simin Chen, Pranav Pusarla, Baishakhi Ray (Columbia University)  
**Description:**  
A benchmark that programmatically mutates programming problems to test for generalization and prevent memorization of coding task solutions.

**References:**
- Chen, S., et al. (2025). *DyCodeEval: Dynamic Contamination-Resistant Evaluation for Code LLMs*. [arXiv:2503.04149](https://arxiv.org/abs/2503.04149)




### 2019 Animal - **Animal-AI Testbed** - Animal-like intelligence testing

**Authors:** Matthew Crosby, Benjamin Beyret, Murray Shanahan, et al. (Imperial College London, Leverhulme CFI)  
**Description:**  
A simulated 3D environment mimicking animal cognition experiments. Tests AI agents on spatial reasoning, object permanence, tool use, and other embodied cognitive skills.

**References:**
- Beyret, B., et al. (2019). *The Animal-AI Environment: Training and Testing AI Agents Using Lessons from Animal Cognition*. [arXiv:1909.07483](https://arxiv.org/abs/1909.07483)
- Beyret, B., et al. (2020). PMLR v123, *Animal-AI Olympics 2019*.




## CONTAMINATION FREE & HAND CRAFTED
### 2019 ARC - **Abstraction and Reasoning Corpus (ARC)**

**Author:** François Chollet (Google Research)  
**Description:**  
A benchmark for testing core abstraction capabilities using grid-based transformation puzzles. Each task involves inferring abstract rules from few-shot examples, requiring generalization to novel problems.

**References:**
- Chollet, F. (2019). *On the Measure of Intelligence*. [arXiv:1911.01547](https://arxiv.org/abs/1911.01547)



### 2024 MMLU-CF - **MMLU-CF (Contamination-Free MMLU)**

**Authors:** Qihao Zhao et al. (Microsoft Research)  
**Description:**  
A decontaminated version of the MMLU benchmark, using new question sources and filtering to eliminate questions present in model training sets.

**References:**
- Zhao, Q., et al. (2024). *MMLU-CF: Benchmarking LLMs on Contamination-Free Multi-task Language Understanding*. [arXiv:2412.15194](https://arxiv.org/abs/2412.15194)



### 2023 Latest Eval - **LatestEval**

**Authors:** Yucheng Li et al. (University of Aberdeen, University of Sheffield)  
**Description:**  
Dynamic benchmark generation from fresh sources (e.g., latest Wikipedia, news, papers) to ensure evaluation content postdates the training cutoff of LLMs.

**References:**
- Li, Y., et al. (2023). *LatestEval: Evaluating Language Models on Recent Knowledge via Reading Comprehension*. [arXiv:2312.12343](https://arxiv.org/abs/2312.12343)
- Published in AAAI-2024: [DOI: 10.1609/aaai.v38i11.26317](https://doi.org/10.1609/aaai.v38i11.26317)



### 2025 Live Bench - **LiveBench**

**Authors:** Colin White, Yann LeCun, Tom Goldstein, et al. (CMU, Meta AI, UMD)  
**Description:**  
A continually updated benchmark of difficult reasoning tasks using fresh data from real-world sources. Designed to measure advanced reasoning without test data leakage.

**References:**
- White, C., et al. (2025). *LiveBench: Real-Time Reasoning Benchmark for LLMs*. ICLR 2025. [OpenReview link](https://openreview.net/forum?id=sKYHBTAxVa)

 

### 2022 BIG - **BIG-bench (Beyond the Imitation Game)**

**Authors:** A. Srivastava et al. (Google and 132-institution collaboration)  
**Description:**  
A community-built multitask benchmark with 204 tasks covering diverse capabilities. Includes mechanisms to test for data contamination via "canary strings" embedded in evaluation tasks.

**References:**
- BIG-bench Collaboration (2022). *Beyond the Imitation Game: Quantifying and Extrapolating the Capabilities of Language Models*. [arXiv:2206.04615](https://arxiv.org/abs/2206.04615)



### 2024 BIG-HARD - **BIG-Bench Hard / Extra Hard (BBH/BBEH)**

**Authors:** Mehran Kazemi, Bahare Fatemi et al. (Google Research)  
**Description:**  
A suite of difficult and newly constructed reasoning tasks derived from BIG-bench, targeting complex logical, symbolic, and multi-step reasoning.

**References:**
- Kazemi, M., et al. (2024). *BIG-Bench Extra Hard: Evaluating the Reasoning Limits of LLMs*. [arXiv:2502.19187](https://arxiv.org/abs/2502.19187)


### 2024 Math - **FrontierMath**

**Authors:** Elliot Glazer, et al. (Epoch, OpenAI collaboration)  
**Description:**  
A benchmark of original, expert-level math problems across disciplines. All problems are unpublished and vetted to guarantee zero overlap with training data.

**References:**
- Glazer, E., et al. (2024). *FrontierMath: Evaluating Frontier LLMs on Hard Math Problems*. [arXiv:2411.04872](https://arxiv.org/abs/2411.04872)





## OTHERS

### 2005 GGP - **General Game Playing (GGP) Competition**

**Authors:** Michael Genesereth (Stanford), Nathaniel Love, Barney Pell  
**Description:**  
A platform where AI agents must play previously unseen games given only a formal rules description, without any game-specific training. Emphasizes learning and reasoning over memorization, ensuring agents generalize to novel game dynamics.

**References:**
- Genesereth, M. R., Love, N., & Pell, B. (2005). *General Game Playing: Overview of the AAAI Competition*. [DOI: 10.1609/aimag.v26i2.1813](https://doi.org/10.1609/aimag.v26i2.1813)



### 2011 WSC - **Winograd Schema Challenge (WSC)**

**Authors:** Hector Levesque (University of Toronto), Ernest Davis (NYU), Leora Morgenstern (SAIC)  
**Description:**  
A commonsense reasoning test using pairs of pronoun resolution problems that can't be solved via statistical correlations. Designed to resist memorization and require true understanding.

**References:**
- Levesque, H. (2011). *The Winograd Schema Challenge*. AAAI Spring Symposium.
- Levesque, H., Davis, E., & Morgenstern, L. (2012). *The Winograd Schema Challenge*. KR 2012.



#### subrua kambampatti


#### Andrew Gordon - accpets his planning is obviated now 
- gofi planning tests


#### SQUAD - stanford test


#### Yoshio bengio
#### Yann LeCun