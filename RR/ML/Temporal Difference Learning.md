
#ml 

Temporal Difference (TD) learning
-?-
TD is a class of model-free reinforcement-learning methods that estimate value functions by **bootstrapping**—i.e., updating estimates based in part on other learned estimates, without waiting for a final outcome. 

It sits between:

- **Monte Carlo** methods, which wait until episode termination to update via full return
- **Dynamic Programming**, which assumes a known model of transition dynamics
    

---

### 1. Core idea
![[Screenshot 2025-05-26 at 11.50.03 PM.png]]

---

### 2. Advantages

- **Data efficiency**: updates every step (vs. only at episode end in Monte Carlo)
- **No model required**: purely sample-based
- **Online & incremental**: can learn on the fly
    

_vs._ Monte Carlo has high variance and must see full episodes; DP needs full knowledge of the MDP.


### 3. Variants. (see [TDS Article](https://towardsdatascience.com/introducing-n-step-temporal-difference-methods-7f7878b3441c/))
- **n-Step** -- like Monte Carlo, it looks into the future to accumulate reward, but this just goes partway.
- **n-Step Sarsa** -- keeps history so next time a state is updated it can update n-chain upto that state too. (I think).
- **off-policy learning** -- Learn a target policy for optimal behavior while executing a different exploration policy for better exploration.
- **tree backup algorithm** -- I think it considers multiple next states from multiple traces for each step rather than learning from a single trace