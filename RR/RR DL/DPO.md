
DPO
-?-
Direct Preference Optimization

  The idea:
  - Traditional RLHF uses a separate reward model + reinforcement learning (PPO)
  - DPO skips the reward model and directly optimizes the policy from preference data

  How it works:
  1. You have pairs of outputs: (preferred response, rejected response) for the same prompt
  2. DPO directly increases probability of preferred outputs and decreases probability of rejected ones
  3. Uses a clever mathematical reformulation that avoids needing RL

  The loss function essentially says: make the model more likely to produce the "chosen" response relative to the "rejected" response, compared to a reference model.

  Why it matters for safety:
  - DPO is how many models learn to refuse harmful requests
  - The preference data includes (refusal, harmful_completion) pairs
  - Model learns: prefer refusals over harmful completions

  Relevance to abliteration:
  - DPO creates the "refusal direction" in activation space
  - Abliteration removes what DPO trained in
  - Understanding DPO helps understand what we're trying to probe/bypass


#dl