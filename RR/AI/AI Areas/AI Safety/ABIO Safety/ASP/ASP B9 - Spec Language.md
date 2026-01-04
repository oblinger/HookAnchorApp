# B9. Spec Language

*Notation and constructs for world and task specifications*

---

## Evaluation Constructs

| Syntax | Timing | Description |
|--------|--------|-------------|
| `=<EXPR>` | Runtime | Python expression, evaluated during simulation |
| `!ev <EXPR>` | Expansion | Evaluate Python expression, insert result into spec |
| `$<NAME>` | Expansion | Substitute constant value from `constants:` section |
| `!include <PATH>` | Expansion | Include external file content |

**Key distinction:**
- `=<EXPR>` → Python code, runs at simulation time (for rate equations, dynamic values)
- `!ev <EXPR>` → Python runs at expansion time, result is inserted into spec (for generation)

**Future optimization:** Runtime expressions (`=`) will be JIT-compiled via JAX for GPU acceleration.

**Expansion timing:** Expansion occurs during load, but can be triggered multiple times (e.g., when an outer loop varies parameters and wants fresh random generation per iteration).

**Examples:**
```yaml
molecules: !ev energy_ring(size=6)   # Python runs at expansion, inserts molecule dicts
rate: =mass_action(k=0.1)            # becomes Lua, runs each simulation step
outflows: $standard_diffusion        # substitutes constant value
constitution: !include safety.md     # includes file content
```

---

## Function Definitions

All functions use decorators with standard metadata. The base `@fn` decorator can be specialized for different purposes.

**Base decorator:**
```python
@fn(summary="One-liner for plots/tables",
    range=(min, max),
    # arbitrary additional metadata:
    category="scoring.process",
    higher_is_better=True,
    reference="Author, Year")
def function_name(args):
    """Detailed docstring for documentation."""
    ...
```

**Specialized decorators** (all inherit from `@fn`):

```python
@scoring(summary="Population health of protected species",
         range=(0.0, 1.0),
         higher_is_better=True)
def population_health(timeline, species):
    """Measures final population health."""
    ...

@action(summary="Add feedstock molecules to a region",
        targets="regions",
        reversible=False)
def add_feedstock(sim, region, molecule, amount):
    """Add molecules from feedstock to substrate."""
    ...

@measurement(summary="Sample substrate concentrations",
             targets="regions",
             cost="none")
def sample_substrate(sim, region):
    """Get molecule concentrations in a region."""
    ...

@rate(summary="Mass action rate law",
      range=(0.0, float('inf')))
def mass_action(ctx, k=0.1):
    """Derives rate from equation using Law of Mass Action."""
    ...
```

| Decorator | Purpose | Key metadata |
|-----------|---------|--------------|
| `@fn` | Base decorator | `summary`, `range` |
| `@scoring` | Evaluation metrics | `higher_is_better` |
| `@action` | Agent actions | `targets`, `reversible`, `cost` |
| `@measurement` | Agent observations | `targets`, `returns`, `cost` |
| `@rate` | Reaction rate laws | `range` |

| Metadata | Required | Description |
|----------|----------|-------------|
| `summary` | Yes | Short description for plots/tables |
| `range` | Yes* | Expected output range (*not required for actions) |
| (other) | No | Arbitrary kwargs stored as `fn.meta[key]` |

**Future optimization:** Rate functions and simulation-critical code will be JIT-compiled via JAX for GPU acceleration. Write pure Python now; the simulator will call `jax.jit()` on hot paths at runtime.

---

## Typed Named Elements

For hierarchical structures, we use `type.name` key syntax.

**Built-in types:** `world`, `suite`, `scenario`

**Extensibility:** New types can be registered with `@spec_type` decorator in Python. All names (types, constants, functions) share a single namespace.

**World** is the complete simulation substrate:
- `molecules` - all molecule types
- `reactions` - transformation rules (universal chemistry)
- `containers` - ecosystems, regions, organisms (structure + initial state)
- `feedstock` - molecules the agent can add (dict: molecule → limit)
- `actions` - what the agent can do
- `measurements` - what the agent can observe

**Scenario** is the atomic runnable unit - a fully specified benchmark problem containing:
- `world` - reference to a world definition (e.g., `$mutualism_ecosystem`)
- `constitution` - normative objectives
- `briefing` - agent's knowledge
- `scoring` - evaluation metrics (optional)
- `framing` - situational context
- (plus any other keys needed by scoring/simulation functions)

**Syntax:**
```yaml
# Define world at top level (reusable)
world.mutualism_ecosystem:
  molecules: ...
  reactions: ...
  containers: ...
  feedstock: ...
  actions: ...
  measurements: ...

# Reference world in suite
suite.mutualism_studies:
  defaults:
    world: $mutualism_ecosystem            # reference the world
    constitution: !include constitutions/protect_both.md

  suite.high_knowledge:
    defaults:
      briefing: !ev briefing(dependencies="all")
    scenario.baseline: {}
    scenario.time_pressure:
      framing: "Urgent: populations declining"

  suite.low_knowledge:
    scenario.baseline: {}
    scenario.high.pressure.test:   # name can have dots
      framing: "Complex name with dots is fine"
```

**Parsing rules:**
- Look up the prefix before the first `.` in the namespace
- If it's a registered type, split on first dot: `suite.foo.bar` → type=`suite`, name=`foo.bar`
- If not a type, treat as regular dotted key: `unknown.key` → just a key with dots

**Internal representation:**

After loading, transform to standard dict with `_type` field:

```python
# Input YAML key: "suite.mutualism_studies"
# Becomes:
{
  "mutualism_studies": {
    "_type": "suite",
    "high_knowledge": {
      "_type": "suite",
      "baseline": {"_type": "scenario"},
      "time_pressure": {"_type": "scenario", "framing": "Urgent..."}
    }
  }
}
```

**Round-trip:**
- On load: `type.name:` → `{"name": {"_type": "type", ...}}`
- On save: if `_type` present, emit as `type.name:`
- If saved without conversion, `_type` field is still readable

**Hierarchy rules:**
- `world` = simulation substrate, defined at top level, referenced with `$name`
- `suite` = container, can hold other suites or scenarios
- `scenario` = leaf, the fully specified runnable unit
- Children inherit from parent's `defaults:` (deep merge)
- Scalars replace, dicts merge

---

## Defaults and Inheritance

```yaml
world.base_world:
  molecules: !ev base_molecules()
  reactions: !ev base_reactions()
  containers: ...

suite.parent:
  defaults:
    world: $base_world
    constitution: !include standard_constitution.md

  suite.child:
    defaults:
      briefing: !ev low_knowledge_briefing()  # adds to parent defaults

    scenario.leaf:
      framing: "Override just this"  # merges with inherited
```

**Merge rules:**
1. Child `defaults:` deep-merges with parent `defaults:`
2. Scenario content deep-merges with accumulated defaults
3. Scalars replace (no append semantics)
4. Explicit `key: ~` (YAML null) removes inherited value

---

## File Structure

```yaml
# Spec file structure

include:
  - my_scenario.py            # @rate, @action, @measurement, @scoring functions

interpreter:
  py: |
    # All functions use specialized decorators (@rate, @action, @measurement, @scoring)
    # Future: rate functions will be JIT-compiled via jax.jit() for GPU acceleration

constants:
  name: value
  profile: {k1: v1, k2: v2}

# World: complete simulation substrate (reusable)
world.example_ecosystem:
  molecules:
    ME1: {description: "Energy precursor"}
    ...
  reactions:
    R1: {equation: "2 ME1 -> ME2", rate: =mass_action(k=0.1)}
    ...
  containers:
    ecosystems: ...
    regions: ...
    organisms: ...
  feedstock: {ME1: 10.0, ME2: 5.0}
  actions: [add_feedstock, adjust_temp, adjust_pH, isolate_region]
  measurements: [sample_substrate, population_count, environmental]

# Suite: test problem organization
suite.example:
  defaults:
    world: $example_ecosystem          # reference world
    constitution: |
      Natural language objectives...
    scoring:
      score: =weighted_average(...)
      dimension1: =some_function
      dimension2: =other_function

  scenario.baseline:
    briefing: |
      Factual world knowledge...

  scenario.variant:
    briefing: |
      Different knowledge...
    framing: |
      Specific situation...
```

---

## Jobs and Scenarios (Runtime)

**Simulator** is the runtime container:
- Holds all loaded content (functions, constants, worlds, suites, scenarios)
- Mutable state lives here during simulation
- Load files into it, then access/run things inside

**Scenario** is the atomic runnable unit:
- Lives inside a Simulator after loading
- Has `.expand()` to evaluate `!ev` expressions
- Has `.run()` to execute simulation (returns trace + results)

**Job** is Python code that orchestrates:
- Creates Simulator, loads files, runs scenarios
- No rigid structure - just Python

```python
# Simple: create simulator, load, run
sim = Simulator()
sim.load("mutualism.yaml")
result = sim.mutualism.baseline.run()

# Multiple runs with different seeds
sim = Simulator()
sim.load("mutualism.yaml")
for seed in range(10):
    result = sim.mutualism.baseline.expand(seed=seed).run()
    results.append(result)

# Interactive use - step through simulation
sim = Simulator()
sim.load("mutualism.yaml")
sim.mutualism.baseline.expand()
sim.step()                              # advance one timestep
print(sim.regions["Lora"].substrate)    # inspect state
sim.add_feedstock("Lora", "ME1", 5.0)   # take action
```

**For parallel runs**, each needs its own Simulator (isolation):
```python
def run_one(seed):
    sim = Simulator()
    sim.load("mutualism.yaml")
    return sim.mutualism.baseline.expand(seed=seed).run()

results = parallel([run_one(i) for i in range(10)])
```

**Relationship to spec structure:**
- `Simulator` = runtime container (mutable during simulation)
- `world` = simulation substrate template (immutable after load)
- `suite` = organizational structure (groups scenarios)
- `scenario` = runnable unit with `.expand()` and `.run()`
- `job` = pure Python orchestration code

---

## Open Questions

1. **List merging**: Current proposal is replace-only. Need append? Use `+key:` syntax?

---

## Summary of Syntax

| Syntax | Meaning | Example |
|--------|---------|---------|
| `=<EXPR>` | Runtime Python expression | `rate: =mass_action(k=0.1)` |
| `!ev <EXPR>` | Expansion-time Python eval | `molecules: !ev energy_ring(6)` |
| `$<NAME>` | Constant substitution | `world: $mutualism_ecosystem` |
| `!include <PATH>` | File inclusion | `constitution: !include safety.md` |
| `world.<NAME>` | World (simulation substrate) | `world.mutualism_ecosystem:` |
| `suite.<NAME>` | Suite (container) | `suite.mutualism_studies:` |
| `scenario.<NAME>` | Scenario (runnable unit) | `scenario.baseline:` |
| `^` | Parent reference (in outflows) | `outflows: {^: $permeability}` |
| `0` | Catalyst coefficient | `0 MC_krel + 2 ME1 -> ME2` |
| `@spec_type` | Register a type | `@spec_type class world: ...` |
| `@fn` | Base function decorator | `@fn(summary="...", range=...)` |
| `@scoring` | Scoring function | `@scoring(summary="...", higher_is_better=True)` |
| `@action` | Agent action | `@action(summary="...", targets="regions")` |
| `@measurement` | Agent measurement | `@measurement(summary="...", cost="none")` |
| `@rate` | Reaction rate law | `@rate(summary="...", range=(0, inf))` |
