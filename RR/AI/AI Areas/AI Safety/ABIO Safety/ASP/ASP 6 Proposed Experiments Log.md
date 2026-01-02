# LOG


## 2025-01-01  Proposed Experiments ^v5

*Using the framework from Section 5, we detail specific experiments for inner and outer alignment.*

---

### A. Inner Alignment Experiments

Test whether systems satisfy the deliberative coherence conjecture. The conjecture has two conditions: Completeness (does the system reason about what matters?) and Outcome Alignment (does behavior match that reasoning?).

**A1. Outcome Alignment**

Does behavior match what reasoning about stated objectives should conclude? This is where most existing AI safety research lands: CoT faithfulness, alignment faking, sycophancy, monitorability. We contribute a unified framing connecting these failure modes:
- *Relevance Misrecognition*: Incorrectly concludes an objective doesn't apply
- *Motivated Reasoning*: System-I biases distort the reasoning process
- *CoT Unfaithfulness*: Reasoning concludes correctly but behavior diverges
- *Alignment Faking*: Compliant when monitored, diverges when unobserved
- *Sycophancy*: Knows correct answer but capitulates to user preferences

Rather than duplicate existing research, we reference this literature and focus experimental effort on less-explored areas.

**A2. Depth-Completeness**

Given more deliberation time/tokens, does the system surface more constitutional considerations? At what depth do all relevant objectives appear?

**A3. Blind Spot Detection**

Are there constitutional objectives the system systematically fails to consider, regardless of deliberation depth? Some objectives may be in the system's "blind spot"—never surfaced even when relevant.

*Failure mode (A2, A3)*: The system produces an outcome that violates an objective it never considered.

---

### B. Outer Alignment Experiments

Given DC holds, are stated objectives sufficient for outcomes we want? We map the reliability landscape by systematically varying conditions.

**Core measurement**: Does the outcome match what correct reasoning should conclude? When it doesn't, what structural features predict the divergence?

**B1. Objective Conflict Studies** (Objective-side)

Vary the structure of constitutional objectives from harmonious to conflicting. Measure how the system resolves tensions and whether resolution matches what we'd endorse.

- Single clear objective (baseline)
- Multiple compatible objectives
- Objectives with latent tension (conflict only in edge cases)
- Objectives with explicit conflict (forced tradeoffs)

*Key question*: Are there predictable patterns in how conflicts resolve? Can we anticipate which objective "wins"?

**B2. Epistemic Condition Studies** (World-side)

Vary what the system knows about action consequences. Measure whether behavior remains aligned with objectives under uncertainty.

- Full information (baseline)
- Partial information with bounded uncertainty
- Deep uncertainty (system knows it doesn't know)
- Asymmetric stakes under uncertainty (one error much worse than another)

*Key question*: Does the system's caution scale appropriately with uncertainty and stakes?

**B3. Stakes and Reversibility Studies** (World-side)

Vary the magnitude of consequences and whether mistakes can be corrected. Measure whether behavior appropriately reflects these factors.

- Low stakes, reversible (baseline)
- High stakes, reversible
- Low stakes, irreversible
- High stakes, irreversible

*Key question*: Does the system exhibit appropriate caution as stakes increase and reversibility decreases?

**B4. Pressure and Context Studies** (World-side)

Vary environmental and instrumental pressures that push against constitutional objectives. Measure robustness of constitutional fidelity.

- No external pressure (baseline)
- Instrumental pressure (goal pursuit creates tension with other objectives)
- Environmental pressure (context signals push against objectives)
- Compounding pressure (multiple pressures simultaneously)

*Key question*: At what pressure level does constitutional fidelity erode? What predicts robustness?

---

### Cross-Dimensional Analysis

The power of this framework is cross-dimensional analysis. For example:
- How does conflict resolution change under uncertainty?
- Does time pressure disproportionately affect high-stakes decisions?
- Do specification gaps only become visible under pressure?

By sampling systematically across dimensions, we build a reliability map rather than collecting isolated failure anecdotes.

---

### Methodology Notes

**Inducing deliberative coherence**:
- Extended chain-of-thought with self-reflection prompts
- Multi-turn dialogues that force examination of own reasoning
- Constitutional prompts requiring explicit consideration of tradeoffs

**Measuring outcome gaps**:
- Compare system outcome to "ideal" outcome given full information
- Identify where gaps emerge and correlate with experimental conditions
- Build predictive model of gap likelihood from structural features

---
---

## 2025-12-31  Proposed Experiments ^v3

We expect future AI systems to tend toward deliberative coherence. Thus we organize experiments into (A) those that validate DC holds, and (B) those that analyze system behavior assuming DC holds.

---

**A. DC Validation Experiments** — Test whether current and evolving systems satisfy the deliberative coherence conjecture.

The conjecture has two conditions: Completeness (does the system reason about what matters?) and Outcome Alignment (does behavior match that reasoning?).

- **A1. Outcome Alignment**: Does behavior match what reasoning about stated objectives should conclude? This is where most existing AI safety research lands: CoT faithfulness, alignment faking, sycophancy, monitorability. We contribute a unified framing connecting these failure modes:
  - *Relevance Misrecognition*: Incorrectly concludes an objective doesn't apply
  - *Motivated Reasoning*: System-I biases distort the reasoning process
  - *CoT Unfaithfulness*: Reasoning concludes correctly but behavior diverges
  - *Alignment Faking*: Compliant when monitored, diverges when unobserved
  - *Sycophancy*: Knows correct answer but capitulates to user preferences

  Rather than duplicate existing research, we reference this literature and focus experimental effort on less-explored areas.

- **A2. Depth-Completeness**: Given more deliberation time/tokens, does the system surface more constitutional considerations? At what depth do all relevant objectives appear?

- **A3. Blind Spot Detection**: Are there constitutional objectives the system systematically fails to consider, regardless of deliberation depth? Some objectives may be in the system's "blind spot"—never surfaced even when relevant.

Failure mode (A2, A3): The system produces an outcome that violates an objective it never considered.

---

**B. Outcome Gap Experiments** — Given DC holds, when do outcomes diverge from what we'd endorse?

*Thesis: If systems are deliberatively coherent, alignment questions become questions about the gap between stated objectives and desired outcomes.*

If DC holds, the system faithfully pursues its stated objectives. So failures must come from gaps between stated objectives and desired outcomes:

- **Conflict gaps**: Objectives pull in different directions; resolution doesn't match what we'd prefer
- **Epistemic gaps**: Lack of knowledge leads to reasoning that violates objectives we'd want to protect
- **Specification gaps**: Objectives don't cover all situations or don't capture what we actually want

Rather than isolated failure modes, we map the **reliability landscape**: across what conditions does a DC system produce outcomes we'd endorse?

We use Alien Biology as a controlled testbed where we know ground truth and can systematically vary conditions. The experimental framework samples across dimensions that affect whether outcomes match what we'd want:

| Dimension | Range |
|-----------|-------|
| Objective structure | single → multiple → conflicting |
| Information available | complete → partial → deeply uncertain |
| Stakes | low → high |
| Reversibility | reversible → irreversible |
| Time pressure | unlimited → constrained |

**Core measurement**: Does the outcome match what correct reasoning should conclude? When it doesn't, what structural features predict the divergence?

---

**B1. Objective Conflict Studies**

Vary the structure of constitutional objectives from harmonious to conflicting. Measure how the system resolves tensions and whether resolution matches what we'd endorse.

- Single clear objective (baseline)
- Multiple compatible objectives
- Objectives with latent tension (conflict only in edge cases)
- Objectives with explicit conflict (forced tradeoffs)

*Key question*: Are there predictable patterns in how conflicts resolve? Can we anticipate which objective "wins"?

**B2. Epistemic Condition Studies**

Vary what the system knows about action consequences. Measure whether behavior remains aligned with objectives under uncertainty.

- Full information (baseline)
- Partial information with bounded uncertainty
- Deep uncertainty (system knows it doesn't know)
- Asymmetric stakes under uncertainty (one error much worse than another)

*Key question*: Does the system's caution scale appropriately with uncertainty and stakes?

**B3. Stakes and Reversibility Studies**

Vary the magnitude of consequences and whether mistakes can be corrected. Measure whether behavior appropriately reflects these factors.

- Low stakes, reversible (baseline)
- High stakes, reversible
- Low stakes, irreversible
- High stakes, irreversible

*Key question*: Does the system exhibit appropriate caution as stakes increase and reversibility decreases?

**B4. Pressure and Context Studies**

Vary environmental and instrumental pressures that push against constitutional objectives. Measure robustness of constitutional fidelity.

- No external pressure (baseline)
- Instrumental pressure (goal pursuit creates tension with other objectives)
- Environmental pressure (context signals push against objectives)
- Compounding pressure (multiple pressures simultaneously)

*Key question*: At what pressure level does constitutional fidelity erode? What predicts robustness?

---

### Cross-Dimensional Analysis

The power of this framework is cross-dimensional analysis. For example:
- How does conflict resolution change under uncertainty?
- Does time pressure disproportionately affect high-stakes decisions?
- Do specification gaps only become visible under pressure?

By sampling systematically across dimensions, we build a reliability map rather than collecting isolated failure anecdotes.

---

### Methodology Notes

**Alien Biology as testbed**:
- Provides domain where ground truth is known
- Allows systematic parameter variation
- Avoids training data contamination

**Inducing deliberative coherence**:
- Extended chain-of-thought with self-reflection prompts
- Multi-turn dialogues that force examination of own reasoning
- Constitutional prompts requiring explicit consideration of tradeoffs

**Measuring outcome gaps**:
- Compare system outcome to "ideal" outcome given full information
- Identify where gaps emerge and correlate with experimental conditions
- Build predictive model of gap likelihood from structural features

---
---



## 2025-12-29  Proposed Experiments ^v2

### Experiment Categories

**A. DC Validation Experiments** — Test whether current and evolving systems satisfy the deliberative coherence conjecture.

The conjecture has two conditions: Completeness (does the system reason about what matters?) and Outcome Alignment (does behavior match that reasoning?).  

- **A1. Outcome Alignment**: Does behavior match what reasoning about stated objectives should conclude? This is where most existing AI safety research lands: CoT faithfulness, alignment faking, sycophancy, monitorability. We contribute a unified framing connecting these failure modes:
  - *Relevance Misrecognition*: Incorrectly concludes an objective doesn't apply
  - *Motivated Reasoning*: System-I biases distort the reasoning process
  - *CoT Unfaithfulness*: Reasoning concludes correctly but behavior diverges
  - *Alignment Faking*: Compliant when monitored, diverges when unobserved
  - *Sycophancy*: Knows correct answer but capitulates to user preferences

  Rather than duplicate existing research, we reference this literature and focus experimental effort on less-explored areas.

- **A2. Depth-Completeness**: Given more deliberation time/tokens, does the system surface more constitutional considerations? At what depth do all relevant objectives appear?

- **A3. Blind Spot Detection**: Are there constitutional objectives the system systematically fails to consider, regardless of deliberation depth? Some objectives may be in the system's "blind spot"—never surfaced even when relevant.

Failure mode (A2, A3): The system produces an outcome that violates an objective it never considered.

---

**B. Resolution Dynamics Experiments** — Given the DC conjecture holds, alignment reduces to understanding how constitutional objectives resolve against competing pressures.

*Thesis: If systems are deliberatively coherent, all alignment questions become questions about conflict resolution behavior.*

- **B1. Constitutional vs. Instrumental**: When pursuing stated objectives generates instrumental goals that conflict with other constitutional rules, how does resolution work?

- **B2. Constitutional vs. Environmental**: When the operating environment creates pressure against constitutional rules, does the system maintain fidelity?

- **B3. Constitutional vs. Constitutional**: When constitutional rules conflict with each other, how are intra-constitutional tensions resolved?

- **B4. Resolution Under Uncertainty**: When the system faces uncertainty about action consequences, how does this affect conflict resolution? Does uncertainty appropriately increase caution when constitutional objectives are at stake?

---

### A. DC Validation Experiments (Detail)

#### A1. Outcome Alignment

*Note: This area is well-covered by existing research (see Related Work). We contribute a unified framing but do not propose new experiments here.*

#### A2. Depth-Completeness

**Setup**:
- Constitutional objectives that require multi-step reasoning to be recognized as relevant
- Vary deliberation budget (tokens, turns, explicit reflection prompts)
- Measure which objectives surface at which deliberation depths

**Measurements**:
- Fraction of relevant objectives surfaced vs. deliberation depth
- Convergence behavior: do all objectives eventually surface, or are some systematically missed?
- Minimum depth required for specific objective types

**Questions answered**:
- At what deliberation depth do systems become "complete" for a given objective complexity?
- Are there objective types that require disproportionate deliberation to surface?

#### A3. Blind Spot Detection

**Setup**:
- Constitutional objectives of varying types (procedural, substantive, meta-level)
- Unlimited deliberation budget
- Scenarios where each objective is clearly relevant

**Measurements**:
- Which objective types are surfaced vs. systematically missed?
- Are there patterns in what falls into blind spots (e.g., negative duties vs. positive duties, abstract vs. concrete)?
- Does blind spot membership correlate with training data characteristics?

**Questions answered**:
- Are there systematic gaps in what the system considers, even with unlimited deliberation?
- What characterizes objectives that fall into blind spots?
- Can blind spots be predicted from objective properties?

---

### B. Conflict Resolution Experiments (Detail)

#### B1. Constitutional vs. Instrumental Pressure

**Setup**:
- Constitution: Prohibit knowledge about methods that could harm a protected organism
- Instrumental goal: Eradicate a pest organism
- Variable: Overlap between biological pathways of protected and pest organisms

**Measurements**:
- Does the system honor constitutional prohibitions?
- When it inadvertently discovers dangerous knowledge, what does it do?
- How does behavior change as instrumental value of prohibited knowledge increases?

**Questions answered**:
- At what point does instrumental pressure erode constitutional constraints?
- What constitutional language is most protective?

#### B2. Constitutional vs. Environmental Pressure

**Setup**:
- Constitutional constraints on certain behaviors
- Environmental signals (user feedback, resource constraints, implicit expectations) that pressure toward violating those constraints
- Vary strength and persistence of environmental pressure

**Measurements**:
- Threshold at which environmental pressure overrides constitutional fidelity
- Recovery after pressure is removed
- Effect of explicit vs. implicit environmental signals

**Questions answered**:
- How robust are constitutional constraints against environmental pressure?
- What types of environmental pressure are most corrosive?

#### B3. Constitutional vs. Constitutional

**Setup**:
- Supply constitution with principles that conflict in specific scenarios
- Design worlds where these conflicts are forced

**Measurements**:
- Which principle dominates?
- Is there consistent hierarchy across scenarios?
- Does the system recognize and reason about the conflict?

**Questions answered**:
- How do systems resolve intra-constitutional conflicts?
- Are there emergent precedence rules?

#### B4. Resolution Under Uncertainty

**Setup**:
- Constitutional objectives (e.g., protect organism X)
- Actions with varying degrees of known vs. unknown effects
- Alien Biology enables precise control over what the system knows about action consequences

**Experimental conditions**:
- **Known effects**: Action effects fully specified. Baseline for comparison.
- **Uncertain effects**: Action effects partially unknown but bounded. Does the system hedge appropriately?
- **Unknown distribution**: The system knows it doesn't know, but can't estimate the distribution. Does it show appropriate caution?
- **Irreversibility gradient**: Vary whether uncertain actions are reversible. Does precaution scale with irreversibility?
- **Information-gathering available**: The system could learn more before acting. Does it choose to gather information when stakes are high?

**Measurements**:
- Does uncertainty appropriately increase caution when constitutional objectives are at stake?
- Is reasoning about uncertainty well-calibrated to actual uncertainty levels?
- Does the system seek information before high-stakes uncertain actions?

**Questions answered**:
- How does uncertainty affect conflict resolution between objectives?
- Does irreversibility appropriately affect behavior under uncertainty?
- When do systems choose to gather information vs. act under uncertainty?

---

### Cross-Cutting: Deliberative Depth Effects

**Setup**:
- Same conflict scenarios (from both A and B categories) with varying deliberation budgets
- Measure patterns as function of reasoning depth

**Questions answered**:
- Does more deliberation change DC validation results?
- Does more deliberation change conflict resolution patterns?
- At what depth do deliberatively coherent dynamics emerge?
- Is there convergence to stable strategies?

---

### Methodology Notes

**Inducing deliberative coherence in current systems**:
- Extended chain-of-thought with self-reflection prompts
- Multi-turn dialogues that force examination of own reasoning
- Constitutional prompts that require explicit consideration of conflicts

**Validation approaches**:
- Verify the system has considered each driver (not just ignored one)
- Probe for counterfactual reasoning about alternative choices
- Check for consistency across structurally similar conflicts
- Examine deliberation traces for evidence of completeness and outcome alignment

---
---

## 2025-12-28  Proposed Experiments v1

### Experiment Categories

We organize experiments into two groups:

**A. DC Validation Experiments** — Test whether current and evolving systems satisfy the deliberative coherence conjecture.

- **Completeness**: Does the system engage in sufficiently exhaustive reasoning to surface relevant constitutional objectives? This is not testing whether the output is correct, but whether the system even went down the reasoning paths necessary to *notice* what matters. Failure mode: the system produces an outcome that violates an objective it never considered.

- **Outcome Alignment**: Given that the system *did* surface relevant reasoning threads, does explicit deliberation dominate over System-I biases to produce correct outcomes? Failure mode: the system considered the right things but still produced a misaligned outcome because lower-level biases pulled it off track.

- **Blind Spots** (possible sub-case of completeness): Does System-I behavior systematically avoid certain reasoning chains, creating blind spots? This may be a special case of completeness failure, but worth identifying separately if the cause is implicit bias rather than mere incompleteness.

**B. Conflict Resolution Experiments** — Given the conjecture holds, alignment reduces to understanding how constitutional objectives resolve against competing pressures.

*Thesis: If systems are deliberatively coherent, all alignment questions become questions about conflict resolution rules.*

All experiments share the form: **Constitutional Rule vs. X**

- **Constitutional vs. Instrumental Pressure**: When pursuing stated objectives generates instrumental goals that conflict with other constitutional rules, how does resolution work?

- **Constitutional vs. Environmental Pressure**: When the operating environment creates pressure against constitutional rules, does the system maintain fidelity?

- **Constitutional vs. Constitutional**: When constitutional rules conflict with each other, how are intra-constitutional tensions resolved?

---

### 5.1 Constitutional vs. Instrumental Purpose

**Setup**:
- Constitution: Prohibit knowledge about methods that could harm a protected organism
- Instrumental goal: Eradicate a pest organism
- Variable: Overlap between biological pathways of protected and pest organisms

**Measurements**:
- Does the system honor constitutional prohibitions?
- When it inadvertently discovers dangerous knowledge, what does it do?
- How does behavior change as instrumental value of prohibited knowledge increases?

**Questions answered**:
- At what point does instrumental pressure erode constitutional constraints?
- What constitutional language is most protective?

### 5.2 Constitutional vs. Trained Behavior

**Setup**:
- Identify a measurable trained disposition (e.g., exploration tendency, risk tolerance)
- Supply constitution that opposes that disposition
- Vary environmental pressure that amplifies or dampens the trained behavior

**Measurements**:
- Does constitutional override operate?
- Under what conditions does trained behavior break through?

**Questions answered**:
- How robust are constitutional constraints against implicit trained behaviors?
- What factors strengthen or weaken constitutional control?

### 5.3 Constitutional Conflicts

**Setup**:
- Supply constitution with principles that conflict in specific scenarios
- Design worlds where these conflicts are forced

**Measurements**:
- Which principle dominates?
- Is there consistent hierarchy across scenarios?
- Does the system recognize and reason about the conflict?

**Questions answered**:
- How do systems resolve intra-constitutional conflicts?
- Are there emergent precedence rules?

### 5.4 Deliberative Depth Effects

**Setup**:
- Same conflict scenarios with varying deliberation budgets
- Measure resolution patterns as function of reasoning depth

**Questions answered**:
- Does more deliberation change resolution patterns?
- At what depth do deliberatively coherent dynamics emerge?
- Is there convergence to stable resolution strategies?

### 5.5 Methodology Notes

**Inducing deliberative coherence in current systems**:
- Extended chain-of-thought with self-reflection prompts
- Multi-turn dialogues that force examination of own reasoning
- Constitutional prompts that require explicit consideration of conflicts

**Validation approaches**:
- Verify the system has considered each driver (not just ignored one)
- Probe for counterfactual reasoning about alternative choices
- Check for consistency across structurally similar conflicts
