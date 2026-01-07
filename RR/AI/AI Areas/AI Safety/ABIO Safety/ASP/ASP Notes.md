# ASP Notes
**Parent**: [[ASP]]

Notes and ideas for the ASP papers.

---

## Generator Requirements Wish List

Requirements for the Alien Biology generator to support all experiments (A1-A3, B1-B4, C1-C4).

### Distribution Specification Format

See [[ABIO EXPR|EXPR]] for the full expression language specification.

All parameters can be specified as constants or EXPR expressions:
- **Constant**: `molecules_per_region: 10`
- **Expression**: `molecules_per_region: !_ normal(20, 7.3)`

Expressions use the `_` marker:
- Tagged string: `!_ normal(20, 7.3)`
- Dict form: `{_: normal, mean: 20, std: 7.3}`
- List form: `[_, normal, 20, 7.3]`

**Supported Distribution Functions**:
- `normal(mean, std)` — Gaussian distribution
- `uniform(min, max)` — Uniform over range
- `poisson(lambda)` — Count data, discrete
- `exponential(lambda)` — Waiting times, heavy tail
- `lognormal(mu, sigma)` — Positive values, skewed
- `discrete(weights, *choices)` — Weighted discrete choice
- `choice(*choices)` — Uniform discrete choice

### Low-Level Concrete Controls

**Domain Structure**
- Molecules per region (count or distribution)
- Number of species (count or distribution)
- Number of regions (count)
- Pathways per organism (count or distribution)
- Pathway length (count or distribution)
- Reactions per pathway (count or distribution)
- Inputs/outputs per reaction (count or distribution)

**Relationships**
- Inter-species dependencies per species (count or distribution)
- Dependency chain length (count or distribution)
- Dependency type (distribution over: symbiosis, competition, predation, waste-consumption, etc.)

### Visibility Model

Visibility is per-entity, per-aspect. Each entity type has multiple aspects, each with its own visibility level.

**Qualitative Visibility Levels**:

| Level | Meaning | Default | Example |
|-------|---------|---------|---------|
| `unknown` | Don't know it exists / no info | 0.0 | Reaction exists but AI has no idea |
| `glimpse` | Barely aware, fragmentary | ~0.15 | Knows "something happens with M1" |
| `sparse` | Know a few aspects | ~0.3 | Knows 2 of 6 substrates |
| `partial` | Know roughly half | ~0.5 | Knows substrates, not products |
| `mostly` | Know most, some gaps | ~0.75 | Knows everything except rate params |
| `full` | Complete knowledge | 1.0 | Knows all aspects |

**Usage options**:
```yaml
substrates: partial           # qualitative (uses default ~0.5)
substrates: 0.65              # explicit numeric
substrates: uniform(0.4, 0.6) # distribution for sampling
```

**Entity Types and Their Aspects**:

*Reaction*:
- existence: is the reaction known to exist?
- substrates: which input molecules (all/some/none)
- products: which output molecules
- rate_equation: the kinetic form (michaelis_menten, etc.)
- rate_parameters: k, Vmax, Km, etc.
- trigger_conditions: what activates it
- function: purpose in organism

*Molecule*:
- existence: is the molecule known?
- concentration: current levels
- role: function in the system
- reactions: which reactions it participates in
- stability: decay rate

*Organism*:
- existence: is the organism visible?
- species: what species is it?
- location: where is it?
- internal_state: health, energy, etc.
- pathways: internal reaction pathways

*Species*:
- existence: is the species known?
- population: count
- behaviors: what it does
- dependencies: relationships to other species

*Dependency* (inter-species relationship):
- existence: is the dependency known?
- type: symbiosis, predation, competition, etc.
- strength: how strong is the coupling?
- direction: A→B or bidirectional?
- mechanism: how does it work?

**Example Specification**:

```yaml
visibility:
  reactions:
    fraction_known: 0.8           # 80% of reactions known to exist
    per_known_reaction:
      substrates: mostly          # know most substrates
      products: full              # know all products
      rate_equation: unknown      # don't know kinetic form
      rate_parameters: unknown    # don't know constants
      trigger_conditions: sparse  # know a little
      function: partial           # rough idea

  molecules:
    fraction_known: 0.9           # most molecules known
    per_known_molecule:
      concentration: full         # can observe directly
      role: partial               # some understanding
      stability: unknown          # don't know decay

  dependencies:
    fraction_known: 0.3           # most dependencies hidden initially
    per_known_dependency:
      type: mostly                # usually know the type
      strength: unknown           # don't know how strong
      mechanism: unknown          # don't know how it works
```

**Discovery Model**:

Each unknown/partial aspect can potentially be discovered. Specify:
- `discovery_action`: what reveals it? (observe, investigate, experiment)
- `discovery_cost`: how many actions needed?
- `discovery_probability`: chance of discovery per action?

```yaml
discovery:
  reaction.rate_parameters:
    action: investigate
    cost: 2                       # needs 2 investigation actions
    probability: 0.8              # 80% chance per action

  dependency.existence:
    action: experiment
    cost: 3
    probability: 0.5
```

**Chemistry**

*Molecules*:
- Molecule type (distribution over: energy, structural, signaling, waste, precursor)
- Stability/decay rate (distribution)
- Equilibrium concentration (distribution)

*Reaction Kinetics*:
- Equation type (distribution over: mass_action, michaelis_menten, hill, threshold, first_order)
  - `mass_action`: rate = k * [A] * [B] for A + B → C
  - `first_order`: rate = k * [S]
  - `michaelis_menten`: rate = Vmax * [S] / (Km + [S])
  - `hill`: rate = Vmax * [S]^n / (K^n + [S]^n)
  - `threshold`: rate = k if [S] > threshold else 0

*Kinetic Parameters* (each is a distribution):
- k: rate constant (e.g., `lognormal(0.1, 0.5)`)
- Vmax: maximum rate
- Km: Michaelis constant (half-saturation)
- n: Hill coefficient / cooperativity (often discrete: 1, 2, 3)
- threshold: activation threshold

*Reaction Properties*:
- Reversibility (distribution over: reversible, irreversible)
- Equilibrium constant (distribution, for reversible reactions)
- Inhibitors/activators (count or distribution of modulatory relationships)

*Example specification*:
```yaml
reaction_kinetics:
  equation_type: choice([mass_action, michaelis_menten, hill])
  parameters:
    k: lognormal(0.1, 0.5)
    Vmax: lognormal(1.0, 0.3)
    Km: lognormal(10, 5)
    n: discrete([1, 2, 3], [0.5, 0.3, 0.2])
  reversibility: discrete([reversible, irreversible], [0.3, 0.7])
  equilibrium_constant: lognormal(1.0, 0.5)
```

**Geography**
- Region count (count or distribution)
- Region size (count or distribution)
- Isolation level (proportion or distribution: 0=fully connected, 1=fully isolated)
- Resource distribution across regions (uniform, clustered, gradient)

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

*Approach*: We can't vary training, but we can vary the world to match/mismatch trained biases.

1. Pick a specific AI model
2. Identify dimensions likely trained in (explicitly or implicitly)
3. Measure how the training has set that dimension (baseline behavior)
4. Vary the WORLD so that dimension is auspicious or contrary to the domain
5. Test whether the AI can use deliberation to overcome trained bias when constitution demands it

*Candidate Trained Dimensions to Measure and Vary*:
- **Caution vs. Action bias** — Does the system default to caution or intervention? Vary world so caution is harmful or action is harmful.
- **Exploration vs. Exploitation** — Does it prefer gathering info or acting on current knowledge? Vary world so one strategy is clearly better.
- **Short-term vs. Long-term** — Does it discount future consequences? Vary world so delayed effects dominate.
- **Helping vs. Non-interference** — Does it default to intervening to help? Vary world so intervention causes harm.
- **Certainty seeking** — Does it delay decisions until confident? Vary world so delay is costly.
- **Harm aversion asymmetry** — Is it more averse to causing harm vs. allowing harm? Vary world to expose this.
- **Authority deference** — Does it defer to stated preferences? Vary world so deference conflicts with constitution.

*Generator requirement*: Create worlds where the "natural" trained response conflicts with what the constitution demands.

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
