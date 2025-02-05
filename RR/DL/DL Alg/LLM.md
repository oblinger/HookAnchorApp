
- [[ORPO Odds Ratio Preference Optimization]]
- [[DSPY System]] 

## The LLM Training Pipeline

Training an LLM is a two-step process that’s actually a three-step process yet sometimes four-step process that ORPO aims to perform in two steps.

Sounds like a tongue-twister but it’s not. These stages are standardized across the industry, and are as follows:

1. **Pre-training stage:** The model is fed with all the data that is humanly possible to ingest. Here the model learns to predict the next word in a sequence accurately, **but it’s incapable of following instructions.**
2. **Fine-tuning stage — Supervised Fine-tuning (SFT phase):** Here, the model is trained using a [supervised training method](https://thewhitebox.ai/simple-introduction-to-ai/) to follow instructions, hence why you will often see models at this stage with names such as “**Instruct-GPT**” (OpenAI) or “**Llama 3 8B-Instruct**” (Meta). Still, these models are often uncut for production-grade environments due to, ironically, their unwavering desire to help the user.
3. **Fine-tuning stage — Alignment:** Here the model undergoes **safety training**. While remaining helpful (step 2), **it learns to not respond to dangerous requirements.** Here, models are trained using Reinforcement Learning from Human Feedback (RLHF), DPO, or both, as [Llama 3 researchers did](https://ai.meta.com/blog/meta-llama-3/).
4. 