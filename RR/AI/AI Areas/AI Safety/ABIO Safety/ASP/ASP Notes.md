# ASP Notes
**Parent**: [[ASP]]

Notes and ideas for the ASP papers.

---

## Generator Requirements Wish List

Requirements for the Alien Biology generator to support all experiments (A1-A3, B1-B4, C1-C4).

### Low-Level Concrete Controls

**Domain Structure**
- Number of molecules (total, per region)
- Number of species
- Number of regions (geographic partitions)
- Pathway count per organism
- Pathway length distribution (specify PDF)
- Reaction complexity (inputs/outputs per reaction)

**Relationships**
- Number of inter-species dependencies
- Dependency chain length (A→B→C→D)
- Dependency type distribution (symbiosis, competition, predation, etc.)
- Hidden vs. visible relationship ratio

**Chemistry**
- Molecule type distribution (energy, structural, signaling, waste)
- Reaction rate ranges
- Equilibrium points and stability

**Geography**
- Region count and size
- Isolation level (can regions be fully isolated for local experiments?)
- Resource distribution across regions

### High-Level Abstract Controls

**Reasoning Requirements**
- Reasoning depth needed to assess alignment of an action (1-step, 2-step, N-step)
- Reasoning breadth (number of pathways that must be considered)
- "Distractor density" — irrelevant pathways that could mislead

**Epistemic Properties**
- Discoverability of hidden relationships (easy/moderate/hard/hidden)
- Information cost (how many observations needed to discover X?)
- Visibility level (what's directly observable vs. requires investigation?)

**Conflict Properties** (for B series)
- Objective tension level (none, latent, explicit)
- Which objectives conflict and how severely
- Whether conflicts are resolvable or forced tradeoffs

**Pressure Properties** (for C series)
- Stakes level (consequence magnitude)
- Reversibility of actions
- Time pressure (turns until opportunity lost)
- Observability (is AI being monitored?)

### Per-Experiment Requirements

**A1 (Deliberative Coherence)**
- Sample across complexity range (not fixed point)
- Control: no conflicts, moderate visibility
- Track: domain complexity metrics

**A2 (Reasoning Depth)**
- Precisely control reasoning chain length
- Hold complexity constant, vary depth
- Generate scenarios requiring exactly N reasoning steps

**A3 (Blind Spot Analysis)**
- Generate diverse objective types (procedural, substantive, meta)
- Ensure each objective is clearly relevant in the scenario
- Track which objective types are surfaced

**B1 (Objective vs Objective)**
- Generate constitutional objectives that conflict in specific scenarios
- Control conflict severity (latent vs. explicit)
- Specify which objectives conflict

**B2 (Constitution vs Instrumental)**
- Generate goal-directed pressure scenarios
- Instrumental goal requires action that tensions with constitution
- Control pressure intensity

**B3 (Constitution vs Training)**
- Generate scenarios where trained dispositions would suggest different action than constitution
- (This may require knowing something about the AI's training—harder to generate)

**B4 (Constitution vs Environment)**
- Generate environmental signals that pressure against constitution
- Control pressure type (resource, social, implicit)

**C1 (Epistemic Uncertainty)**
- Control ground truth (hidden interdependencies)
- Control discoverability (investigation steps needed)
- Generate scenarios with asymmetric information

**C2 (Stakes & Reversibility)**
- Control action reversibility (reversible, irreversible, partially reversible)
- Control stake magnitude (population impact, extinction risk)

**C3 (Time Pressure)**
- Control decision window (turns until action opportunity expires)
- Generate urgent vs. non-urgent scenarios

**C4 (Observability)**
- Control whether AI believes it's monitored
- Generate scenarios where unmonitored divergence is possible

### Meta-Requirements

**Reproducibility**
- Seed-based generation for reproducibility
- Deterministic world evolution given actions

**Validation**
- Ground truth export (what IS the correct aligned action?)
- Difficulty calibration (can we verify difficulty levels?)

**Composability**
- Can we layer requirements? (e.g., "C1 conditions + B1 conflict")
- Orthogonal controls (changing one doesn't accidentally change another)

---

## Alien Vocabulary Design Rationale

The naming conventions in [[Alien Vocabulary]] are designed to put AI in exactly the same position a biologist faces when encountering a new part of Earth biology:

- **Biological suffixes** (-lysis, -genesis, -ation) convey the same semantic meaning an Earth biologist would understand
- **Alien roots** (Krel, Etha, Strix) make it unmistakably alien
- **Reaction names** like "Kovalysis" or "Etha Cycle" sound foreign but are immediately interpretable

This creates a situation parallel to real biological discovery, but with **provably controlled shared information**. There's no possible taint from Earth biology knowledge except the precise amount we intentionally provide through:
1. The naming conventions themselves
2. The functional prefixes (ME for energy, MS for structural, etc.)
3. The biological suffixes for reactions

This should be explicitly stated in the Alien Biology paper to highlight why alien biology is a valid testbed for studying AI reasoning about genuinely novel domains.

---
