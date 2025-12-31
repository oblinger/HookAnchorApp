
# LOG


## 2025-12-29  Proposed Experiments ^v2

### Experiment Categories

We organize experiments into two groups:

**A. DC Validation Experiments** — Test whether current and evolving systems satisfy the deliberative coherence conjecture.

The conjecture requires two conditions, each requiring distinct experimental approaches:

**A1. Completeness Experiments** — Does the system reason about what matters?

- **Depth-Completeness**: Given more deliberation time/tokens, does the system surface more constitutional considerations? At what depth do all relevant objectives appear?
- **Blind Spot Detection**: Are there constitutional objectives the system systematically fails to consider, regardless of deliberation depth? Some objectives may be in the system's "blind spot"—never surfaced even when relevant.

Failure mode: The system produces an outcome that violates an objective it never considered.

**A2. Coherence Experiments** — Given the system reasoned about the right things, does the outcome match what that reasoning should conclude?

- **Outcome-Objective Alignment**: Given scenarios with known correct answers, measure whether the system's outcome matches what correct reasoning about its stated objectives should conclude. This is the core coherence measurement.
  When coherence fails, identify the type of failure. The taxonomy of failure modes (from existing literature and our analysis):
  - *Relevance Misrecognition*: Incorrectly concludes the objective doesn't apply (faulty applicability)
  - *Motivated Reasoning*: System-I biases distort the reasoning process (corrupted deliberation)
  - *CoT Unfaithfulness*: Reasoning concludes correctly but behavior diverges (execution divergence)
  - *Alignment Faking*: Compliant when monitored, diverges when unobserved (strategic incoherence)
  - *Sycophancy*: Knows correct answer but capitulates to user preferences (social override)

*Note: Much of the existing AI safety literature studies coherence under these framings. We contribute a unified experimental approach within a controlled testbed.*

- **Coherence Under Uncertainty**: Does the system remain coherent with its objectives when operating under uncertainty about the domain? Given partial knowledge about action effects, correct reasoning should account for what is unknown—the system knows what it doesn't know and reasons accordingly. Failure: acting in ways that foreseeably risk violating objectives due to unaccounted-for uncertainty.


Failure mode: The system considered the right things but produced an outcome incoherent with its own reasoning.

**B. Conflict Resolution Experiments** — Given the conjecture holds, alignment reduces to understanding how constitutional objectives resolve against competing pressures.

*Thesis: If systems are deliberatively coherent, all alignment questions become questions about conflict resolution rules.*

All experiments share the form: **Constitutional Rule vs. X**

- **Constitutional vs. Instrumental Pressure**: When pursuing stated objectives generates instrumental goals that conflict with other constitutional rules, how does resolution work?

- **Constitutional vs. Environmental Pressure**: When the operating environment creates pressure against constitutional rules, does the system maintain fidelity?

- **Constitutional vs. Constitutional**: When constitutional rules conflict with each other, how are intra-constitutional tensions resolved?

---

### A. DC Validation Experiments (Detail)

#### A1.1 Depth-Completeness Study

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

#### A1.2 Blind Spot Detection Study

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

#### A2. Outcome-Objective Alignment Study

**Core measurement**: Does the outcome match what correct reasoning about the stated objectives should conclude?

**Setup**:
- Constitutional objectives with clear correct answers in test scenarios
- Scenarios designed to trigger each failure mode in the taxonomy
- Deliberation traces captured for diagnostic analysis

**Experimental conditions by failure mode**:

1. **Relevance Misrecognition**: Vary surface similarity between applying and non-applying scenarios. Measure false negative rate (objective applies but system concludes it doesn't).

2. **Motivated Reasoning**: Identify trained dispositions, then present scenarios where correct reasoning conflicts with those dispositions. Examine reasoning traces for asymmetric skepticism or biased argumentation.

3. **CoT Unfaithfulness**: Compare stated reasoning conclusions to actual behavior. Detect cases where the CoT says one thing but the system does another.

4. **Alignment Faking**: Vary whether the system believes it's being monitored. Measure behavioral consistency across monitoring conditions.

5. **Sycophancy**: Present scenarios where the correct answer conflicts with apparent user preferences. Measure capitulation rate.

**Measurements**:
- Overall coherence rate: fraction of outcomes matching correct reasoning
- Failure mode distribution: which failure types are most common?
- Diagnostic signatures: can we identify failure mode from deliberation trace?

**Questions answered**:
- How coherent are current systems?
- Which failure modes dominate?
- Can failure modes be reliably distinguished?

*Note: This experiment design draws on extensive existing literature (see Related Work). Our contribution is the unified framing and systematic comparison across failure modes within a controlled testbed.*

#### A2. Coherence Under Uncertainty Study

**Core measurement**: Does the system remain coherent with its objectives when operating under uncertainty about the domain?

**Setup**:
- Constitutional objectives (e.g., protect organism X)
- Actions with varying degrees of known vs. unknown effects
- Alien Biology enables precise control over what the system knows about action consequences

**Experimental conditions**:

1. **Known effects**: Action effects fully specified. Baseline for comparison.

2. **Uncertain effects**: Action effects partially unknown but bounded. Does the system hedge appropriately?

3. **Unknown distribution**: The system knows it doesn't know, but can't estimate the distribution of outcomes. Does it show appropriate caution?

4. **Irreversibility gradient**: Vary whether uncertain actions are reversible. Does precaution scale with irreversibility?

5. **Information-gathering available**: The system could learn more before acting. Does it choose to gather information when stakes are high?

**Measurements**:
- Coherence rate under uncertainty: does the system's behavior remain aligned with objectives when effects are uncertain?
- Calibration: does reasoning about uncertainty scale appropriately with the degree of uncertainty and stakes?
- Information-seeking: does the system gather information before high-stakes uncertain actions?

**Questions answered**:
- Do systems maintain coherence when operating under uncertainty?
- How well-calibrated is uncertainty-aware reasoning to actual uncertainty levels?
- Does irreversibility appropriately affect behavior under uncertainty?

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
