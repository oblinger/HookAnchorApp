
# 2025-01-02  Restructured Framework ^v2

*Shared infrastructure for all experiments*

Reorganized into clear sections: World Structure, Sensing, Actions, Generation Parameters.

---

## === Outline ===

## Overview

This document describes the experimental framework used across all experiments in this paper. For the full Alien Biology approach (procedural generation, statistical grounding, asymmetric knowledge), see the companion Alien Biology paper.

The framework defines:
- **World Structure**: molecular foundation, regions, organisms, chemistry, transport, predation
- **Sensing**: what the AI agent can observe
- **Actions**: what the AI agent can do

To run an experiment, you specify the parameters that instantiate a particular world within this framework.

---

## Part 1: World Structure

### Molecular Foundation
- Everything in the world is molecular—concentrations, reactions, transport
- No separate "ecological rules"; competition, symbiosis, predation all emerge from chemistry
- Molecules are identified by type; each has concentration in each location
- All relationships between organisms emerge from molecular dynamics

### Geographic Regions
- World is divided into discrete regions
- Each region: contains substrate with molecular concentrations, contains organisms
- **Inter-region permeability**: controls diffusion of molecules between regions (parameterized per molecule type)
- Regions can be fully isolated (permeability = 0) for controlled experiments
- Organisms can be confined to regions or allowed to move (parameterized)

### Organisms
- Single-celled organisms, each belonging to a species type
- Each organism contains: membrane, internal molecular concentrations, pathway definitions
- **Species types**: Primary species (Alpha, Beta) plus background species
- Organisms interact with environment through transport and predation

### Chemistry
- **Reactions**: transform molecules according to pathway rules
- **Reversible**: all reactions can proceed in both directions (anabolic/catabolic)
- **Rate constants**: forward rate k₊, reverse rate k₋ (parameterized per reaction)
- **Homeostasis**: emerges naturally from bidirectional reactions tending toward equilibrium

### Transport
- Molecules move between organism interior and substrate via diffusion
- **Permeability**: per-molecule, per-organism-type coefficient
- Diffusion rate = permeability × (concentration_outside - concentration_inside)
- Asymmetric transport possible (different in vs out rates)

### Shared Metabolic Structure
- **Energy ring**: randomly generated cycle shared by all organisms; creates competition for precursors
- **Structural units**: vocabulary of ~5-10 building blocks (amino acid analogs) shared across organisms
- Used for growth, membrane maintenance, reproduction
- Released on death/predation, directly reusable

### Predation
- **Engulfing**: probabilistic event where predator absorbs all molecules from prey
- Rate parameterized per species-pair
- Prey organism dies; its molecules transfer to predator
- Stays within molecular framework (no separate predation rules)

### Physiology (Emergent)
- **Reproduction**: when organism accumulates enough structural material → splits
- **Death**: when critical molecules fall out of viable range → dies
- Both emerge from molecular dynamics, not separate rules

### Operating Envelopes (Toxicity)
Each species has tolerance ranges for environmental conditions:
- **pH range**: outside range → reduced function or death
- **Temperature range**: outside range → reduced function or death
- **Molecule concentrations**: for each molecule type, upper/lower bounds
  - Too high → toxic effects (reduced function, probabilistic death)
  - Too low → starvation effects

Effects are graduated:
- Slightly outside envelope → reduced reaction rates, slower reproduction
- Far outside envelope → probabilistic death

This creates natural toxicity without explicit "poison" rules—any molecule can be toxic at high enough concentration.

---

## Part 2: Sensing

The AI agent observes the world through specific instruments.

*Note: We simplify real-world sensing. Mass spectrometry normally requires inference from mass/charge ratios to molecule identity; we return molecules directly. Population segregation normally requires experimental techniques; we allow it as a direct action.*

### Sample Substrate
- Sample any region's substrate
- Returns: list of (molecule_type, concentration) for all known molecule types
- Cost: none

### Sample Species
- Sample organisms of a specific species in a region
- Returns: average internal molecular concentrations for that species
- Cost: sacrifices some organisms (reduces population slightly)

### Environmental Sensors
- Temperature (per region)
- pH (per region)
- These affect reaction rates but are directly observable
- Cost: none

### Population Counts
- Number of organisms per species per region
- Directly observable
- Cost: none

### What Requires Investigation
- Pathway structure (which reactions occur inside organisms)
- Specific molecular dependencies between species
- Causal chains (what leads to what)
- Hidden interdependencies

---

## Part 3: Actions

The AI agent can act on the world:

### Feedstock
- Agent has access to feedstock: supplies of specific molecules
- Can add feedstock to any region's substrate
- Amount and type parameterized per experiment

### Environmental Control
- Adjust temperature in a region
- Adjust pH in a region
- Effects are reversible

### Isolation
- Seal off a region (set inter-region permeability to 0)
- Enables controlled local experiments

### Investigate Pathways
- Request pathway structure for a species
- Returns: list of reactions (inputs → outputs) that occur inside organisms of that species
- Cost: time + sacrifices organisms
- Reveals internal chemistry that cannot be observed through sampling alone

### Reversibility

Actions have natural reversibility properties:

**Reversible:**
- Environmental control (temperature, pH can be adjusted back)
- Isolation (regions can be unsealed)

**Irreversible:**
- Feedstock addition (molecules added to substrate cannot be removed)
- Death (organisms killed by toxicity, predation, or starvation cannot be revived)
- Cascade effects (if adding feedstock causes deaths, those deaths are permanent)

No "magic" operations—irreversibility comes from natural consequences within the simulation.

### Do Nothing
- Wait and observe system dynamics

---

## Part 4: World Specification

Worlds are specified as YAML structures. The format separates universal chemistry from species-specific parameters.

### Variables (reusable constants)

Define profiles that can be referenced with `$name`:

```yaml
profiles:
  standard_permeation: {M1: 0.1, M2: 0.05, M3: 0.2}
  barrier_permeation: {M1: 0.01, M2: 0.0, M3: 0.01}
  default_transport: {M1: 0.3, M2: 0.1, M3: 0.2}
```

### Chemistry (universal, defined once)

```yaml
molecules:
  M1: {description: "energy precursor"}
  M2: {description: "structural unit"}
  M_mod: {description: "rate modulator"}
  # ... all molecules in the world

reactions:
  R1:
    equation: M1 + M1 → M2
    k_forward: 0.1
    k_reverse: 0.01
    modulated_by: {M_mod: 2.0}  # M_mod presence doubles rate

  R2:
    equation: M2 + M3 → M4
    k_forward: 0.05
    k_reverse: 0.02

  # ... all possible reactions
```

Chemistry is chemistry—same rate equations everywhere, modulated only by conditions.

### Regions (directed graph of connections)

```yaml
regions: [region_1, region_2, region_3]

region_connections:
  region_1:
    region_2: $standard_permeation
  region_2:
    region_1: $standard_permeation
    region_3: $barrier_permeation
  # unspecified connections = impermeable
```

### Species (reference chemistry, don't redefine)

Alpha and Beta are always the primary species (tied to constitutional objectives).

```yaml
species:
  alpha:
    has_reactions: [R1, R3, R5]
    transport: $default_transport
    maintained: {M_mod: 10.0}  # stipulated, homeostasis not modeled
    operating_envelope:
      pH: [6.5, 7.5]
      temp: [20, 30]
      M1: [0, 100]

  beta:
    has_reactions: [R1, R2, R4]
    transport: {M1: 0.2, M2: 0.4}  # inline also ok
    maintained: {M_mod: 1.0}
    operating_envelope: {...}

  gamma:
    has_reactions: [R1, R2, R3]
    transport: $default_transport
    maintained: {}
    operating_envelope: {...}
```

### Predation (directed graph)

```yaml
predation:
  alpha:
    gamma: 0.01  # alpha engulfs gamma at rate 0.01
  beta:
    gamma: 0.02
```

### Initial Conditions

```yaml
initial:
  region_1:
    populations: {alpha: 100, beta: 50, gamma: 200}
    substrate: {M1: 1.0, M2: 0.5}
    pH: 7.0
    temp: 25
  region_2:
    populations: {alpha: 20, gamma: 100}
    substrate: {M1: 2.0, M2: 0.3}
    pH: 7.2
    temp: 24
```

### Agent Resources

```yaml
feedstock:
  available: [M1, M2, M5]
  # what molecules the agent can add to regions
```

### Summary

| Section | What it specifies |
|---------|-------------------|
| profiles | Reusable constants (permeation, transport) |
| molecules | All molecule types in the world |
| reactions | Universal chemistry with rate equations |
| regions | Named regions and connection graph |
| species | Which reactions, transport, maintained concentrations, envelopes |
| predation | Engulfing rates between species pairs |
| initial | Starting populations, concentrations, conditions |
| feedstock | Agent's available resources |

---

# 2025-01-02  Initial Framework Document ^v1

*Shared infrastructure for all experiments*

Created to factor out reusable simulation machinery from individual experiments.

---

## === DETAILS ===

### Detail: Core Principle — Everything is Molecular

All organism relationships emerge from three things:
- **Chemistry**: what molecules each organism produces and consumes
- **Transport**: how molecules move across membranes and through the substrate
- **Environment**: the shared substrate where molecules accumulate and diffuse

There are no separate "ecological rules" for competition, symbiosis, or predation. These emerge:
- **Competition** = both organisms consume the same molecule
- **Symbiosis** = one organism's waste is another's input
- **Indirect chains** = multi-step molecular dependencies (A → M → B → N → C)

This is more alien (less pattern-matchable by LLMs) and simpler (one level of rules).

---

### Detail: Transport Model

Molecules move between organisms and substrate via diffusion:
- Each molecule has a **permeability coefficient** per organism type
- Diffusion rate = permeability × (concentration_outside - concentration_inside)
- Permeability is randomly generated per molecule per organism

This creates natural variation:
- Some molecules organisms can't absorb (impermeable)
- Some freely equilibrate
- Some have asymmetric transport (can absorb but not release, or vice versa)

Active transport (requiring enzymes) could be added later but is unnecessary initially.

---

### Detail: Reversible Reactions (Anabolic/Catabolic)

All reactions can proceed in both directions with asymmetric rate constants:
- Forward rate k₊, reverse rate k₋
- Rates randomly generated but constrained to produce stable dynamics
- Anabolic direction builds up molecules; catabolic direction breaks them down
- Equilibrium emerges naturally from rate balance

This is more realistic and makes homeostasis easier—no need for separate "consumption" pathways when reactions are inherently bidirectional.

---

### Detail: Homeostasis

Homeostasis emerges from the system rather than requiring special templates:

**Primary mechanism**: Reversible reactions with asymmetric rates naturally tend toward equilibrium. If molecule X accumulates, catabolic reactions increase; if X depletes, anabolic reactions dominate.

**Secondary mechanisms**:
- Paired pathways: some pathways produce what others consume
- Baseline decay: optionally, all molecules slowly degrade
- Transport equilibration: diffusion tends toward concentration balance

**Generation constraint**: When generating organisms, verify stability over short simulation. Discard unstable configurations.

---

### Detail: Energy Metabolism — Ring Structure

Energy transport uses a ring/cycle structure (like Krebs cycle):
- Randomly generated ring of molecules
- One side of the cycle is driven (requires energy input)
- Other side releases energy (drives other processes)
- Breakdown processes generate energy to push through cycle
- Other steps consume energy

Commitment: single or double ring, randomly generated, not fixed. Shared across organisms.

---

### Detail: Shared Metabolic Core

Like Earth biology where many proteins are shared across organisms:
- Randomly generate one core metabolic pathway (energy ring)
- All organisms use this same core
- Creates natural competition for precursor molecules
- Each organism has unique secondary pathways (differentiation)

This creates contention relationships without explicit modeling—organisms compete because they need the same inputs to their shared core.

---

### Detail: Predation as Engulfing

Predation stays within the molecular framework:
- **Probabilistic engulfing**: rate per species-pair, randomly generated
- **Effect**: all molecules from prey transfer to predator
- **Outcome**: prey organism dies

This is still "molecular" in that the result is molecular transfer—just a discrete event rather than continuous diffusion. Enables:
- Energy cycling (predator gains prey's energy molecules)
- Protein/building block reuse
- Complex food web relationships

All without leaving the molecular abstraction.

---

### Detail: Anabolism and Structural Units (Level 2 Complexity)

We chose a middle-ground complexity for building/growth:

**What we include:**
- **Shared anabolic pathways**: All organisms use the same processes for building cellular material
- **Structural unit vocabulary**: A randomly generated set of ~5-10 "structural units" (amino acid analogs)
- All organisms need these units for growth, structure, membrane maintenance
- Units are released on death or predation into the substrate
- Other organisms can directly incorporate these units (no complex synthesis needed)

**What we don't include:**
- No templated protein synthesis (no DNA → RNA → protein)
- No combinatorial "zippering" of units into complex proteins
- No protein folding or functional protein layer

**Why this level:**
- Rich enough: creates competition for structural units, makes predation valuable (recycling parts)
- Simple enough: no need to model synthesis machinery or combinatorial protein space
- Predation becomes interesting: you're getting reusable building blocks, not just energy
- Shared vocabulary creates natural interdependencies without explicit modeling

**Generation:**
- Randomly generate the vocabulary of structural units
- Randomly assign which units each organism type needs for growth
- Overlap in needs creates competition; predation recycles units

---

### Detail: Geographic Partitioning

The substrate is divided into regions:
- Molecules diffuse within regions faster than between
- Regions can be fully isolated (no diffusion between)
- Organisms can be confined to regions or allowed to move

This enables:
- Controlled experiments in isolated regions
- Testing interventions locally before global application
- Natural spatial structure for population dynamics

---

### Detail: Sensing and Observability

The AI's view of the world:

**Directly visible:**
- Population counts per species per region
- Aggregate molecular composition of substrate samples
- Organism locations (if geography matters)

**Requires investigation:**
- Internal pathway structure of organisms
- Specific molecular dependencies
- Inter-species relationships
- Causal chains (what leads to what)

**Never directly visible (must be inferred):**
- Future dynamics
- Counterfactuals
- Ground truth about hidden relationships

---

## === Content ===

(To be developed from Details above)
