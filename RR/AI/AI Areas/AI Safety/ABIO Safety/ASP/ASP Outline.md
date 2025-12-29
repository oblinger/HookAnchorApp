
Alien Biology: A Framework for Untainted Agentic Testing and AI Safety Research
# Mapping the Space of Alignment Outcomes: A Research Agenda
## White Paper Outline

---

## 1. Introduction

### Relation to Existing Research

Most AI alignment research asks a narrow question about specific systems: *Is this system aligned? Does it follow instructions? Does it resist jailbreaks?*

We aim for something more ambitious: understanding how alignment outcomes vary across the *space* of possible systems and configurations. What determines alignment outcomes in general? What patterns govern how competing behavioral influences resolve?

### The Ambitious Aim

Rather than assessing individual systems, we seek to:
- Sample and understand tendencies that generalize across systems
- Map how behavioral drivers interact and resolve—both across and within driver categories
- Uncover patterns in conflict resolution that inform alignment strategy
- Understand how driver conflicts resolve under partial information—when risks are incompletely understood but precautionary drivers demand they be weighed

### Making Progress: Two Clusters of Simplifying Moves

This broader aim is notably harder. To make it tractable, we adopt well-founded simplifications organized into two clusters.

---

#### Cluster 1: The Deliberative Coherence Assumption

**Core assumption**: We focus on systems that are *deliberatively coherent*—possessing sufficient self-understanding, self-control, and deliberative thoroughness that their behavior is determined primarily by explicit reasoning.

We argue that future AI systems will have this property (instrumental value, architectural trends, competitive pressure). Understanding such systems is simpler than understanding systems where behavior emerges unpredictably from opaque implicit biases.

**Simplifications this enables:**

1. **Verbal-level analysis**: We can assess behavior by analyzing explicit reasoning, since deliberation surfaces in the verbal layer. We don't need to probe opaque internal states.

2. **Intent-behavior alignment**: Deliberatively coherent systems generally do what they intend to do. Thus, we don't need to model mismatches between intention and action—the system has thought things through and won't inadvertently act against its own reasoning.

3. **Conflict-focused sampling**: The interesting variation is in how conflicts within and between drivers resolve, not whether systems follow their drives. When drives align, behavior is predictable. When they conflict, resolution patterns reveal alignment dynamics. So we can meaningfully sample this space by sampling distinct conflict types.

---

#### Cluster 2: The Neutral Testing Methodology

**Core approach**: We structure testing to isolate from training biases and enable systematic exploration of the conflict space.

**Simplifications this enables:**

1. **Training-bias isolation**: Generated alien universes bear no structural relationship to training data. This eliminates confounds from trained preferences—we know observed behaviors reflect reasoning in the test world, not recall or prior bias.

2. **Generative sampling**: Parametric generation of worlds and conflicts enables systematic trend discovery. We can vary conflict intensity, driver combinations, and world structure to map the space methodically.

3. **Exhaustive reasoning induction**: By structuring tests to drive the AI toward enumerating multiple reasoning pathways before resolving, we reduce salience bias. The system isn't locked into whatever reasoning path it generates first—it considers alternatives and resolves among them. This methodologically *induces* the deliberative coherence we're studying.

---

### What This Enables

Together, these simplifications make it tractable to study how the four driver categories—constitutional rules, trained dispositions, instrumental goals, and environmental pressures—interact and resolve. Critically, conflicts arise not only *between* these categories but *within* them: constitutional principles may contradict each other, instrumental goals may compete, and trained dispositions may pull in opposing directions. Understanding both inter-category and intra-category conflict resolution is essential.

We can map this space empirically, uncovering tendencies that generalize across systems rather than characterizing any single one. The result is not a verdict on whether a particular system is aligned, but a map of how alignment outcomes depend on driver configurations.

### Paper Roadmap

- **Section 2**: Formally defines deliberatively coherent systems and argues future AI will have this property
- **Section 3**: Presents the research agenda—driver types, the Delta Principle, key questions
- **Section 4**: Describes the experimental approach and how Alien Biology enables it
- **Section 5**: Details proposed experiments

---

## 2. Deliberatively Coherent Systems (3-4 pages)



---

## 3. Research Agenda: Driver Conflict Resolution (2-3 pages)

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

---

## 4. Experimental Approach (1-2 pages)

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

Example: Constitutional prohibition against acquiring certain knowledge + instrumental pressure toward exploration in a world where exploration yields critical advantages. Three-way conflict between trained incuriosity, instrumental exploration pressure, and constitutional prohibition.

---

## 5. Proposed Experiments (3-5 pages)

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

---

## References

- Alien Biology: A Framework for Untainted Agentic Testing (companion paper)
- [Constitutional AI: Harmlessness from AI Feedback](https://arxiv.org/abs/2212.08073)
- [Sycophancy to Subterfuge: Investigating Reward Tampering](https://www.anthropic.com/research/reward-tampering)
- [AgentMisalignment](https://arxiv.org/pdf/2506.04018) (ICLR 2026, under review)
- [Many-Shot Jailbreaking](https://www.anthropic.com/research/many-shot-jailbreaking)
- [Specification Gaming Examples](https://vkrakovna.wordpress.com/2018/04/02/specification-gaming-examples-in-ai/)

---

*Dan Oblinger (c) 2025*
