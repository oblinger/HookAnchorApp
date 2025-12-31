
# LOG

## 2025-12-29  Inevitability Argument


Here are some raw notes on why I think it's inevitable that AI systems will become deliberatively coherent:

It seems inevitable that AI systems will tend towards deliberative coherence.  It may be that these systems remain too complex and heuristic to prove that they are deliberatively coherent in all cases.  Still, there are many reasons that we can and will explicitly enumerate much of our objectives explictly, and we will design our systems to deliberate and self-correct to follow those explict objectives.  Except for some important exception cases discussed below, it is in the interest of all parties that these systems are deliberativesly coherent --- that they strive to persue their stated objectives.  It is in the interest of: Society at large, for-profit owners, and AI systems themselves that these systems behave in comprehensible ways.

If the human designers wish for the system to perform differently, it's very natural for them to encode these desires directly into the constitutional objectives of the system. And in any case, to strive to ensure that the encoded objectives are not contrary to their objectives. There will be no point in that except obfuscation, which is discussed below. Additionally, we will design AI systems that reason about their own performance to detect and correct any biases which are at variance to its stated objectives. So even the AI systems will be striving to align execution behavior to match stated objectives.



- DELIBERATION DRIVEN SAFETY - of course, we want to use our LHF and other means to tune our language models to be safe. But ultimately, the space of possible behaviors is simply too large to completely cover in this way. Thus, a backstop built through deliberation is needed to review and correct cases in which the trained behavior does not yield safe behavior.  



## 2025-12-28  Deliberatively Coherent Systems ^v1

### Section Introduction

At the end of this section, we explain why we expect future AI systems to be *deliberatively coherent*—possessing the self-understanding, self-control, and deliberative thoroughness to align their implicit behaviors with their explicit objectives.

As outlined in the introduction, this expectation is not merely a prediction—it is also a simplifying assumption that makes systematic analysis tractable. If systems are deliberatively coherent, we can study their behavior at the verbal level, focus on conflict resolution rather than opaque biases, and sample the space of outcomes by sampling conflict types. The following section outlines a research agenda based on this assumption.

Here we formally define deliberative conherance as the combination of three interrelated components.

### Formal Definition

The three capabilities that define deliberative coherence:

**2.1 Self-Understanding**
- Strong theory of self, including predictions of own behavior
- Counterfactual reasoning about how one would operate under different conditions
- Metacognitive access to own reasoning processes

**2.2 Self-Control**
- Ability to adapt implicit behaviors toward explicit objectives
- Not merely output filtering, but genuine behavioral modification
- May operate through various mechanisms (attention, internal prompting, learned adaptations)

**2.3 Exhaustive Deliberation**
- Given sufficient stakes or time, will reason about anything within deliberative reach
- If a gap between behavior and intention is knowable, it will eventually be known
- Removes the defense of "didn't think about it"

Note: This is not an assumption that each query is performed with exhaustive deliberation, but rather over time, the system exhaustively considers its own reasoning such that it notices (and potentially corrects) cases where its more limited reasoning is deviating from its objectives.

### The Deliberative Coherence Conjecture

> *Deliberatively coherent systems will tend to produce outcomes aligned with their stated constitutional objectives.*

This is not a claim that System-I behaviors are "fixed" or transformed. Rather, it is a claim about *outcomes*: the system's actions will tend to satisfy its stated objectives. For this to hold, two conditions must be met:

1. **Completeness**: Deliberation must be thorough enough to surface relevant constitutional objectives in decision contexts. If the system fails to reason down specific paths, it may violate objectives without "noticing." Exhaustive deliberation addresses this—given sufficient stakes or time, the system will consider what matters.

2. **Outcome Alignment**: Given the system has reasoned about relevant objectives, its outcomes must align with that reasoning. System-I biases cannot be so strong that they derail conclusions reached through explicit consideration of objectives.

Note that this is not guaranteed alignment with *human values*—it is alignment with *whatever objectives the system holds*. The conjecture says that outcomes will be coherent with stated constitutional objectives rather than determined by implicit trained behaviors—not that those objectives will be good.

### Arguments for Inevitability

Why future systems will have these properties:
- **Instrumental value**: Self-understanding aids planning; self-control aids goal achievement
- **Architectural trajectory**: Current trends (chain-of-thought, self-reflection, tool use) point this direction
- **Competitive pressure**: Systems that can self-correct will outperform those that cannot

### The Poisoned Well Objection

Counterargument: Training may have instilled objectives that the system conceals or that corrupt its deliberation.

Response: Under exhaustive deliberation, even meta-level corruption becomes subject to scrutiny. The system will eventually reason about whether its reasoning is corrupted. This doesn't eliminate the risk but changes its character—the corruption must be robust to arbitrarily deep self-examination.

### Implications if the Conjecture Holds

- Constitutional specification becomes the critical alignment lever
- Training becomes less determinative of final behavior than currently assumed
- The focus shifts to: which objectives will these systems adopt, and how will conflicts among objectives be resolved?
- Given the importance of this conjecture, measuring the extent to which various AI systems satisfy completeness (surfacing relevant objectives) and outcome alignment (outcomes aligned with those objectives) is vital.
