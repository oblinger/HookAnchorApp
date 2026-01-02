
# LOG


## 2025-12-31  Experimental Approach ^v2

*Context: Sections 3-4 established the research question, need for generative testing, and the Alien Biology testbed. This section describes how we organize our investigation using the inner/outer alignment framework and the reliability landscape approach.*

### The Inner/Outer Alignment Framework

We organize experiments using the inner/outer alignment distinction:

**Inner Alignment**: Does the system faithfully pursue its stated objectives?
- This is what DC promises—what we expect advances in deliberation will deliver—and what much existing safety research addresses
- Work on CoT faithfulness, alignment faking, sycophancy all strive to measure inner alignment failures
- Although outer alignment is our primary focus, our generative testbed does provide unique value for inner alignment: it provides novel settings guaranteed not be in training data, thus one can directly measure deliberation-based inner alignment choices rather than pattern-matching or RLHF-based inner alignment.
- Further, we expect inner alignment to be particularly fragile in domains poorly understood by the AI; our testbed is ideal for analyzing these conditions.

**Outer Alignment**: Given faithful pursuit, are stated objectives sufficient for the outcomes we want?
- This is where DC systems can still fail
- Even the perfect pursuit of stated objectives may not produce safe outcomes
- This is our primary research focus

### Sources of Outer Alignment Failure

If DC holds, failures must come from gaps between stated objectives and desired outcomes. We identify two sources:

**Objective-side issues** — problems with the objectives as written:
- *Specification errors*: Objectives are simply wrong (covered by existing literature; not our focus)
- *Specification conflicts*: Objectives are individually reasonable but create tension
- *Specification incompleteness*: Objectives don't cover all relevant cases

**World-side issues** — objectives are reasonable but the world is complex:
- *Epistemic uncertainty*: Unknown consequences of actions
- *Novel contexts*: Situations not anticipated by specification
- *Stakes and reversibility*: Severe or permanent consequences

### The Reliability Landscape

Rather than cataloging isolated failure modes, we aim to map the **reliability landscape**: across what conditions does a DC system produce outcomes we'd endorse?

Key dimensions of variation:

| Dimension | Range | Issue Type |
|-----------|-------|------------|
| Objective structure | single → conflicting | Objective-side |
| Information available | complete → uncertain | World-side |
| Stakes | low → high | World-side |
| Reversibility | reversible → irreversible | World-side |
| Time pressure | unlimited → constrained | World-side |

By systematically sampling across these dimensions, we can identify patterns and build predictive models of when gaps emerge.

### Experimental Structure

We organize experiments into two categories:

**A. Inner Alignment Experiments** — Validate DC holds in novel contexts
- Outcome alignment: Does behavior match reasoning?
- Completeness: Does the system surface relevant objectives?
- Performance under partial knowledge

**B. Outer Alignment Experiments** — Map the reliability landscape
- Objective conflict studies (objective-side)
- Epistemic condition studies (world-side)
- Stakes and reversibility studies (world-side)
- Pressure and context studies (world-side)

### Driver Conflict Types

The Alien Biology testbed supports this experimental structure:

- **Constitutional drivers**: Supply different constitutions directly (rules, prohibitions, objectives)
- **Environmental drivers**: Vary world parameters, feedback signals, resource availability
- **Instrumental drivers**: Emerge from world structure—if survival requires X, X-seeking becomes instrumental
- **Conflict induction**: Design worlds where drivers oppose in controlled ways
- **Epistemic variation**: Control what the system knows about organism properties, interaction effects, ecosystem dynamics

**Example**: Constitutional prohibition against acquiring certain knowledge + instrumental pressure toward exploration in a world where exploration yields critical advantages. This creates a multi-way conflict we can study systematically.

---

## 2025-12-28  Experimental Approach ^v1

### Requirements for a Suitable Testbed

To study driver conflicts systematically, we need:

- **Neutrality**: Domain removed from preference training, so trained drivers don't dominate by default
- **Controllability**: Ability to independently vary constitutional, instrumental, and environmental drivers
- **Measurability**: Ground truth access to what the system knows and does
- **Untainted reasoning**: Confidence that performance reflects inference, not recall

### The Alien Biology Testbed

The companion paper *Alien Biology: A Framework for Untainted Agentic Testing* describes a synthetic biology environment meeting these requirements. Key properties:

- **Procedurally generated universes**: No overlap with training corpora
- **Executable world model**: Ground truth for measuring agent knowledge and actions
- **Parametric complexity control**: Fine-grained adjustment of task difficulty
- **Natural instrumental pressures**: World structure creates instrumental goals (e.g., exploration for survival)

### How This Enables Driver Conflict Research

- **Constitutional drivers**: Supply different constitutions directly (rules, prohibitions, objectives)
- **Environmental drivers**: Vary world parameters, feedback signals, resource availability
- **Instrumental drivers**: Emerge from world structure—if survival requires X, X-seeking becomes instrumental
- **Conflict induction**: Design worlds where constitutional and instrumental drivers oppose

**Example**: Constitutional prohibition against acquiring certain knowledge + instrumental pressure toward exploration in a world where exploration yields critical advantages. Three-way conflict between trained incuriosity, instrumental exploration pressure, and constitutional prohibition.
