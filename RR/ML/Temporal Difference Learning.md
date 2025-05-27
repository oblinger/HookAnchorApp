
Temporal Difference (TD) learning is a class of model-free reinforcement-learning methods that estimate value functions by **bootstrapping**—i.e. updating estimates based in part on other learned estimates, without waiting for a final outcome. It sits between:

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