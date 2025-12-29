# 1. Introduction

## Outline

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

## Content

(To be fleshed out from the outline above)
