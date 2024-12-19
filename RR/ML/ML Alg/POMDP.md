
POMDP
-?-
(Partially Observable Markov Decision Process) is a mathematical framework for decision-making problems where the agent must make decisions in a stochastic (uncertain) environment and does not have full information about the current state of the system. It generalizes the Markov Decision Process (MDP) by introducing partial observability.  (A kind of RL algorithm)

---

### Components of a POMDP:

A POMDP is defined by the tuple (S,A,T,R,Ω,O,γ):

1. **State Space (SS)**: A set of all possible states of the environment.
2. **Action Space (AA)**: A set of actions the agent can take.
3. **Transition Model (TT)**: Probability T(s,a,s′)=P(s′∣s,a)T(s,a,s′)=P(s′∣s,a) that action aa in state ss leads to state s′s′.
4. **Reward Function (RR)**: A mapping R(s,a)R(s,a) or R(s,a,s′)R(s,a,s′) that specifies the expected reward for taking action aa in state ss.
5. **Observation Space (ΩΩ)**: A set of possible observations the agent can receive about the state.
6. **Observation Model (OO)**: Probability O(o∣s′,a)O(o∣s′,a) of observing o∈Ωo∈Ω given the next state s′s′ and action aa.
7. **Discount Factor (γγ)**: A factor 0≤γ<10≤γ<1 that models how future rewards are valued relative to immediate rewards.