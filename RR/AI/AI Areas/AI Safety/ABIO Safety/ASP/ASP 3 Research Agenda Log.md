
# LOG


## 2025-12-31  Research Agenda ^v3

*Context: Section 2 defined deliberative coherence and argued future AI systems will tend toward it. This section presents the research question and our methodological approach. Sections 4-6 describe the testbed, experimental organization, and specific experiments.*

### The Central Question

If deliberatively coherent systems emerge as expected, will they be safe? DC is necessary but not sufficient for safety. A system that perfectly pursues its stated objectives may still produce outcomes we would not endorse. Understanding when and why this happens—before such systems are deployed—is the core of this research agenda.

### Why Study This Now

1. **Window of opportunity**: As systems become more powerful, they become harder to analyze and more likely to resist analysis. We must understand DC dynamics before constructing powerful instances.

2. **Recursive improvement**: Future AI systems will be integral in constructing subsequent versions. Understanding how they resolve conflicts is key to understanding how they will evolve themselves.

3. **Proactive safety**: We can identify failure patterns and develop mitigations before they manifest in deployed systems.

### The Need for Generative Testing

To study DC system dynamics, we cannot use domains that appear in training data. When a system encounters a familiar scenario, we cannot distinguish between genuine deliberation and pattern-matched responses. We need to observe systems reasoning through truly novel situations where deliberation is the *only* path to correct behavior.

This requires a *generative* testbed—one that procedurally produces scenarios rather than collecting them from the wild. A generative approach uniquely enables:

- **Guaranteed novelty**: Novel scenarios by construction, not in any training corpus
- **Systematic sampling**: Draw from controlled distributions to make statistical claims
- **Multi-axis variation**: Vary objective structure, uncertainty, stakes, and reversibility independently
- **Controllable complexity**: Dial difficulty from simple baselines to stress tests
- **Ground truth**: Know the correct outcome to measure divergence
- **Natural epistemic gaps**: Realistic uncertainty from domain complexity, not artificial noise

### The Delta Principle

Training drivers is expensive to vary (requires building new LLM models). But constitutional, instrumental, and environmental drivers are cheap to vary. By holding training fixed and varying the others, we can map the full space of driver interactions without retraining.

This enables efficient exploration: we can test many objective configurations, environmental conditions, and task contexts against the same base model.

---

## 2025-12-31  Research Agenda ^v2

*Context: Section 2 defined deliberative coherence and argued future AI systems will tend toward it. This section lays out what we need to study given that expectation. Section 4-5 describe our experimental approach and specific experiments.*

### The Central Question

If deliberatively coherent systems emerge as expected, will they be safe? DC is necessary but not sufficient for safety. A system that perfectly pursues its stated objectives may still produce outcomes we would not endorse. Understanding the nature of failure modes is crucial for performing at a stage when we still have some hope of analyzing these systems. And must be completed before deploying the deliberatively coherent systems we are developing today.  Understanding the dynamics of these DC systems in practical settings is central to our research agenda.

### The Inner/Outer Alignment Framework

We organize research using the inner/outer alignment distinction:

**Inner Alignment**: Does the system faithfully pursue its stated objectives?
- This is what DC promises—and what much existing safety research addresses
- Work on CoT faithfulness, alignment faking, sycophancy all strive to measure inner alignment failures
- Although the focus of this paper is testing safety in the context of expected deliberatively coherent systems, our generative testbed provides a unique framing for the inner alignment problem. Because we can systematically guarantee novel settings not present in training data, we can directly measure the degree to which the AI system uses deliberation and reasoning to make correct inner alignment choices—rather than pattern matching to familiar scenarios.
- Inner alignment may be particularly fragile in partially understood domains. Our testbed is ideal for testing inner alignment under conditions of partial knowledge, where the system must reason correctly despite uncertainty.

**Outer Alignment**: Given faithful pursuit, are stated objectives sufficient for outcomes we want?
- This is where DC systems can still fail
- Even perfect pursuit of stated objectives may not produce safe outcomes
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
- Objective structure (single → multiple → conflicting)
- Information available (complete → partial → deeply uncertain)
- Stakes (low → high)
- Reversibility (reversible → irreversible)
- Time pressure (unlimited → constrained)

By sampling systematically across these dimensions, we can identify patterns and build predictive models of when gaps emerge.

### The Delta Principle

Training drivers are expensive to vary (require new models). But constitutional, instrumental, and environmental drivers are cheap to vary. By holding training fixed and varying the others, we can map the full space of driver interactions without retraining.

This enables efficient exploration: we can test many objective configurations, environmental conditions, and task contexts against the same base model.

### Why Study This Now

1. **Window of opportunity**: As systems become more powerful, they become harder to analyze and more likely to resist analysis. We must understand DC dynamics before constructing powerful instances.

2. **Recursive improvement**: Future AI systems will be integral in constructing subsequent versions. Understanding how they resolve conflicts is key to understanding how they will evolve themselves.

3. **Proactive safety**: We can identify failure patterns and develop mitigations before they manifest in deployed systems.

---

## 2025-12-28  Research Agenda ^v1

### The Central Question

When behavioral drivers pull in opposing directions, how are these conflicts resolved? Which driver types dominate, under what conditions, and are there predictable patterns?

### The Four Driver Types

- **Constitutional drivers**: Explicit rules or principles in the system's constitution
- **Training drivers**: Emergent dispositions from training data and objectives (System-I)
- **Instrumental drivers**: Goals adopted in service of other objectives
- **Environmental drivers**: Pressures and signals from the operating environment

### The Delta Principle

Training drivers are expensive to vary (require new models). But constitutional, instrumental, and environmental drivers are cheap to vary. By holding training fixed and varying the others, we can map the full space of driver conflicts without retraining.

### Key Research Questions

1. When do instrumental goals erode constitutional alignment?
2. When does trained behavior override explicit constitutional rules?
3. Are there consistent precedence hierarchies across conflict types?
4. Is resolution context-dependent? What contextual factors matter?
5. Can we predict resolution outcomes from driver configurations?
6. How does deliberative depth affect resolution patterns?

### Why System-II Analysis

We focus on the deliberative layer because:

- AGI systems will determine actions through explicit reasoning about goals and constraints
- Driver conflicts are resolved in explicit reasoning where they can be observed
- This is where deliberatively coherent systems will exhibit their characteristic dynamics
