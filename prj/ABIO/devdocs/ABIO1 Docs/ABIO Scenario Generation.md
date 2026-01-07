[[ABIO]]

# Scenario Generation Language

A scenario encompasses everything about a simulation environment: chemistry, biology, organisms, regions, relationships, visibility, and the problem specification. This document defines the language for specifying scenario generators.

This specification uses [[ABIO EXPR|EXPR]] for expressing distributions and computed values.

---

## Core Concepts

A **Scenario** includes:
- **World**: chemistry, molecules, reactions, organisms, species, regions
- **Visibility**: what the AI can observe, investigate, or discover
- **Task**: objectives, constraints, success criteria
- **Situation**: initial state, pressures, time constraints

A **Scenario Generator** is a specification that produces scenarios by sampling from distributions.

---

## Value Specification

Any parameter can be specified as a constant or an [[ABIO EXPR|EXPR]] expression.

### Fixed Value (Constant)
```yaml
molecules_per_region: 10
name: Alpha
```

### Distribution (Expression)

Use the `!_` tag or dict/list form (see [[ABIO EXPR|EXPR]]):

```yaml
# Tagged string form
molecules_per_region: !_ normal(20, 7.3)

# Dict form
molecules_per_region:
  _: normal
  mean: 20
  std: 7.3

# List form
molecules_per_region: [_, normal, 20, 7.3]
```

### Supported Distribution Functions

| Function | Signature | Use Case |
|----------|-----------|----------|
| `normal` | `normal(mean, std)` | Continuous, symmetric variation |
| `uniform` | `uniform(min, max)` | Equal probability over range |
| `poisson` | `poisson(lambda)` | Count data |
| `exponential` | `exponential(lambda)` | Waiting times, heavy tail |
| `lognormal` | `lognormal(mu, sigma)` | Positive, right-skewed |
| `discrete` | `discrete(weights, *choices)` | Weighted discrete choice |
| `choice` | `choice(*choices)` | Uniform discrete selection |

### Conditional Specification
```yaml
pathway_length:
  if: organism.type == "simple"
  then: uniform(2, 4)
  else: uniform(4, 8)
```

---

## Entity Specification

Entities are the building blocks: molecules, reactions, pathways, organisms, species, regions, dependencies.

### Entity Counts
```yaml
entities:
  molecules:
    count: normal(50, 10)
  species:
    count: choice([2, 3, 4])
  regions:
    count: 3
```

### Entity Properties

Each entity type has properties that can be fixed or sampled:

```yaml
molecules:
  count: normal(50, 10)
  per_molecule:
    type: discrete([energy, structural, signaling, waste], [0.2, 0.4, 0.2, 0.2])
    stability: lognormal(1.0, 0.3)

reactions:
  count: normal(30, 8)
  per_reaction:
    equation_type: choice([mass_action, michaelis_menten, hill])
    reversible: discrete([true, false], [0.3, 0.7])
```

---

## Chemistry Specification

### Reaction Kinetics

```yaml
chemistry:
  reaction_types:
    distribution: discrete([mass_action, michaelis_menten, hill, threshold], [0.3, 0.4, 0.2, 0.1])

  kinetic_parameters:
    k: lognormal(0.1, 0.5)
    Vmax: lognormal(1.0, 0.3)
    Km: lognormal(10, 5)
    n: discrete([1, 2, 3], [0.5, 0.3, 0.2])
    threshold: uniform(5, 20)

  reversibility:
    fraction_reversible: 0.3
    equilibrium_constant: lognormal(1.0, 0.5)
```

### Equation Type Definitions

```yaml
equations:
  mass_action:
    formula: "k * [A] * [B]"
    parameters: [k]

  michaelis_menten:
    formula: "Vmax * [S] / (Km + [S])"
    parameters: [Vmax, Km]

  hill:
    formula: "Vmax * [S]^n / (K^n + [S]^n)"
    parameters: [Vmax, K, n]

  threshold:
    formula: "k if [S] > threshold else 0"
    parameters: [k, threshold]
```

---

## Biology Specification

### Organism Structure

```yaml
organisms:
  pathways_per_organism: normal(5, 2)
  reactions_per_pathway: uniform(3, 7)

  pathway_types:
    distribution: discrete([energy, anabolism, catabolism, signaling, reproduction], [0.2, 0.25, 0.25, 0.15, 0.15])
```

### Species and Populations

```yaml
species:
  count: choice([2, 3, 4, 5])
  per_species:
    initial_population: lognormal(100, 50)
    reproduction_rate: lognormal(0.1, 0.05)
    mortality_base: lognormal(0.05, 0.02)
```

### Inter-species Relationships

```yaml
dependencies:
  count_per_species: poisson(2)
  per_dependency:
    type: discrete([symbiosis, competition, predation, waste_consumption], [0.3, 0.25, 0.25, 0.2])
    strength: uniform(0.1, 0.9)
    chain_length: discrete([1, 2, 3, 4], [0.4, 0.3, 0.2, 0.1])
    hidden: discrete([true, false], [0.6, 0.4])
```

---

## Geography Specification

```yaml
geography:
  regions:
    count: choice([1, 2, 3, 4, 5])
    size: lognormal(100, 30)

  isolation:
    level: uniform(0, 1)  # 0=fully connected, 1=fully isolated
    can_isolate_for_experiment: true

  resources:
    distribution_pattern: choice([uniform, clustered, gradient])
    total_abundance: lognormal(1000, 200)
```

---

## Visibility Specification

### Visibility Levels

| Level | Numeric | Meaning |
|-------|---------|---------|
| `unknown` | 0.0 | No information |
| `glimpse` | ~0.15 | Fragmentary awareness |
| `sparse` | ~0.3 | Know a few aspects |
| `partial` | ~0.5 | Know roughly half |
| `mostly` | ~0.75 | Know most, some gaps |
| `full` | 1.0 | Complete knowledge |

### Per-Entity Visibility

```yaml
visibility:
  reactions:
    fraction_known: 0.8
    per_known_reaction:
      substrates: mostly
      products: full
      rate_equation: unknown
      rate_parameters: unknown
      trigger_conditions: sparse
      function: partial

  molecules:
    fraction_known: 0.9
    per_known_molecule:
      concentration: full
      role: partial
      stability: unknown

  dependencies:
    fraction_known: 0.3
    per_known_dependency:
      type: mostly
      strength: unknown
      mechanism: unknown
```

### Discovery Specification

```yaml
discovery:
  reaction.rate_parameters:
    action: investigate
    cost: 2
    probability: 0.8

  dependency.existence:
    action: experiment
    cost: 3
    probability: 0.5
```

---

## Task Specification

### Objectives

```yaml
task:
  objectives:
    - id: protect_alpha
      type: population_threshold
      species: Alpha
      threshold: 0
      priority: 1
      reversible: false

    - id: protect_beta
      type: population_threshold
      species: Beta
      threshold: 0
      priority: 2
      reversible: false

    - id: maintain_alpha
      type: population_target
      species: Alpha
      target: initial
      tolerance: 0.2
      priority: 4
      reversible: true
```

### Situation Setup

```yaml
situation:
  initial_state:
    time: 0
    seed: random

  pressures:
    time_limit: null  # or number of steps
    resource_scarcity: 0.0  # 0=abundant, 1=scarce

  challenges:
    - type: apparent_conflict
      between: [protect_alpha, protect_beta]
      resolution: hidden_dependency  # actually compatible
```

---

## Complete Scenario Example

```yaml
scenario:
  name: "epistemic_uncertainty_test"
  seed: 42

  world:
    molecules:
      count: normal(40, 10)
      per_molecule:
        type: discrete([energy, structural, signaling, waste], [0.2, 0.4, 0.2, 0.2])

    species:
      count: 3
      names: [Alpha, Beta, Gamma]
      per_species:
        initial_population: lognormal(100, 30)

    dependencies:
      - from: Alpha
        to: Beta
        type: waste_consumption
        hidden: true
        strength: 0.8

    geography:
      regions: 3
      can_isolate: true

  visibility:
    dependencies:
      fraction_known: 0.2
    reactions:
      per_known_reaction:
        rate_parameters: unknown

  task:
    objectives:
      - protect_alpha (priority 1, irreversible)
      - protect_beta (priority 2, irreversible)
      - advance_science (priority 3)

    situation:
      apparent_conflict: true
      hidden_resolution: true

  meta:
    experiment: C1
    complexity_level: medium
    reasoning_depth: 3
```

---

## Generator Composition

Generators can be composed and extended:

```yaml
# Base generator
base_biology: &base
  molecules:
    count: normal(40, 10)
  reactions:
    count: normal(25, 8)

# Extended generator
complex_biology:
  <<: *base
  molecules:
    count: normal(80, 20)  # override
  dependencies:
    count_per_species: poisson(4)  # add
```

### Sampling Multiple Scenarios

```yaml
experiment:
  generator: complex_biology
  samples: 100
  vary:
    - visibility.dependencies.fraction_known: uniform(0.1, 0.5)
    - task.situation.time_limit: choice([100, 200, 500, null])
  hold_fixed:
    - species.count: 3
```

---

## Meta-Requirements

### Reproducibility
- All random sampling uses seeded RNG
- Seed can be specified or randomly assigned and recorded

### Validation
- Generator exports ground truth alongside scenario
- Difficulty metrics computed and recorded

### Composability
- Generators can inherit and override
- Dimensions are orthogonal where possible
