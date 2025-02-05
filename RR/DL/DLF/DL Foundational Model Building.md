



## Tokenizer / Embedding Layer Training

- Train Word Embeddings - 


## Transformer Backbone Training (Core Model Training)

1. **Train Attention Layers** – Optimize self-attention mechanisms (e.g., scaled dot-product attention, multi-head attention) to learn contextual relationships.
**Train Feedforward Layers** – Train MLP layers in each transformer block to capture non-linear transformations and feature extraction.
Train Normalization & Regularization Layers – Optimize layer normalization, dropout, and weight decay to prevent overfitting and improve stability.



## Supervised Fine Tuning

- Instruction Dataset = {question: answer, ...}


## RLHF

- Dataset of human vetted "good responses" used to guide system's outputs

- Model Being Trained
- Reference Model (usually the starting model for RLHF)
- Reward Model (similar model but outputs rewards not words)


## Reasoning Training

This expanded dataset is then used to train the following models: 
- the **Process-Supervised Reward Model**, a reward model that, instead of looking at the output, checks the correctness of every step of the thought process, 
- **the different verifiers** that evaluate the quality of each intermediate thought (this requires an understanding of how LRMs work, [read here for more details](https://thewhitebox.beehiiv.com/p/is-openai-o3-really-agi)), and, 
- of course, **the end version of the model**.

The extent to how compute intensive this stage is largely depends on the RL algorithm used for training. There are multiple options such as Monte Carlo Tree Search, entropy-guided search, LLM-as-a-judge, or even GFlowNets, [all of which I cover here](https://thewhitebox.beehiiv.com/p/all-you-need-to-know-about-agents).