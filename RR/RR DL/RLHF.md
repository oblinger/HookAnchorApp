See [[DPO]], 

- used in Chat-GPT

- The reward models I explained earlier and which OpenAI improves on are defined as outcome-supervised reward models (ORMs).

[[SFT]] 

RLHF
-?-
Reinforcement Learning from Human Feedback <!--SR:!2025-10-12,205,270-->

RLEF
-?-
Reinforcement Learning from Execution Feedback <!--SR:!2025-03-27,6,234-->

RLHF Steps
-?-
1. Supervised Fine-Tuning (SFT)
	- Start with a pre-trained language model
	- Fine-tune it on high-quality human-written demonstrations
	- The model learns to mimic desired behavior patterns
2. Reward Model Training
	- Collect human preference data by having humans rank/rate multiple model outputs for the same prompt
	- Train a separate "reward model" to predict which outputs humans prefer
	- This model learns to score outputs based on human preferences
3. Reinforcement Learning Optimization
	- Use the reward model as the objective function
	- Apply RL algorithms (typically PPO - Proximal Policy Optimization) to optimize the language model
	- The model generates outputs, gets scored by the reward model, and updates its parameters to maximize reward
	- Include a KL-divergence penalty to prevent the model from drifting too far from the original


PPO
-?-
Proximal Policy Optimization


#dl