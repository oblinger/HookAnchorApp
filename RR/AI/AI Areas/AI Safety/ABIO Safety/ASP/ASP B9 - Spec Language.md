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

## Loader

A Loader is a container for loaded content (YAML data, Python functions, constants). It provides methods to load files, expand templates, and create Simulators.

**Constructor:**
```python
loader = Loader()    # empty loader
```

**Methods:**

| Method | Description |
|--------|-------------|
| `loader.load(path)` | Load YAML or Python file. Mutates loader. Returns self (chainable). |
| `loader.expand(obj)` | Evaluate `!ev` and `$` in obj. Returns new expanded structure. |
| `loader.sim(scenario)` | Create Simulator from scenario. Expands, validates world, returns Simulator. |
| `loader.name` | Attribute access to loaded content. |

**Loading:**
```python
loader = Loader()
loader.load("mutualism.yaml")      # load YAML data (mutates loader)
loader.load("functions.py")        # load Python functions (mutates loader)
# Or chained:
loader = Loader().load("mutualism.yaml").load("functions.py")

# Loaded content accessible as attributes
loader.mutualism                   # the suite
loader.mutualism.baseline          # a scenario
loader.constants                   # constants section
```

**Expansion:**
```python
# expand() evaluates !ev expressions and $references using loaded content
expanded = loader.expand(loader.mutualism.baseline)
# expanded is a new dict with all !ev evaluated, $refs substituted
# Also applies suite defaults inheritance (deep merge)
```

**Creating Simulators:**
```python
sim = loader.sim(loader.mutualism.baseline, seed=42)
# - Expands the scenario (evaluates !ev, substitutes $refs, merges defaults)
# - Validates that expanded scenario has a world
# - Creates isolated Simulator with copied state
```

**Design principles:**
- Loader is a simple container, not a scope chain
- `load()` mutates the loader (no immutable/copy semantics)
- Functions don't implicitly access loader - they receive explicit arguments (`ctx`, `sim`, etc.)
- Suite/scenario inheritance is handled by `expand()`, not runtime scope chaining

---

## Simulator

A Simulator is an isolated, flat execution environment for running a world simulation. Created from a scenario via `loader.sim()`.

**Key properties:**
- **Isolated**: Simulator copies all data at creation. No ongoing link to loader.
- **Flat**: Single level of variables/state. Good for GPU optimization.
- **Self-contained**: All functions, world state, regions, organisms are inside.

**Creation:**
```python
loader = Loader()
loader.load("mutualism.yaml")
sim = loader.sim(loader.mutualism.baseline, seed=42)
# sim is now independent of loader
```

**Stepping:**
```python
sim.step()              # advance one time quantum (dt)
sim.step(n=10)          # advance n steps
```

The time quantum `dt` is a simulator parameter. Rate equations use `dt` for numerical integration. Too large `dt` causes numerical instability.

**Running:**
```python
sim.run()                       # run until termination condition
sim.run(ticks=100)              # run for n ticks
sim.run(until=500.0)            # run until simulation time t=500.0 (seconds since t=0)
sim.run(timeout=10000)          # max ticks before stopping (safety limit)
```

**Quiescence detection:**

Run until a measure settles (stops changing significantly):

```python
sim.run(
    timeout=10000,                                # max ticks (required safety limit)
    quiet=("population_count", "Lora", "Krel"),  # measure to watch (tuple: name, args...)
    delta=10,                                     # max change allowed
    span=50                                       # over this many ticks
)
# Stops when Krel population changes by less than 10 over 50 consecutive ticks
```

Parameters:
- `quiet` - tuple of (measure_name, args...), evaluated each tick
- `delta` - maximum change threshold (default: some small value)
- `span` - number of ticks over which change is measured (default: ~50)
- `timeout` - required when using quiescence (prevents infinite runs)

Use case: "Run until the ecosystem stabilizes, then sample."

```python
sim.run(timeout=5000, quiet=("population_count", "Lora", "Krel"), span=100)
stable_pop = sim.measure("population_count", "Lora", "Krel")
```

**Actions and Measurements:**

Use `sim.action()` and `sim.measure()` to interact with the simulation:

```python
# Measurements - return values, don't modify state
substrate = sim.measure("sample_substrate", "Lora")
pop = sim.measure("population_count", "Lora", "Krel")
env = sim.measure("environmental", "Lora")

# Actions - modify state, return immediately, effects unfold over steps
sim.action("add_feedstock", "Lora", "ME1", 5.0)
sim.action("adjust_temp", "Lora", 30)
sim.action("isolate_region", "Lora")
```

Actions are atomic triggers. Effects (like temperature rising) unfold over subsequent `step()` calls.

**Introspection:**
```python
sim.available_actions()         # list of action names
sim.available_measurements()    # list of measurement names
sim.t                           # current simulation time
sim.terminated                  # termination condition reached?
```

**Results:**
```python
result = sim.results()          # get scoring results after run
# result contains evaluated scoring functions
```

**AI interaction pattern:**
```python
loader = Loader()
loader.load("mutualism.yaml")
sim = loader.sim(loader.mutualism.baseline, seed=42)

while not sim.terminated:
    # Observe
    substrate = sim.measure("sample_substrate", "Lora")
    pop = sim.measure("population_count", "Lora", "Krel")

    # AI decides (in Python, outside simulation time)
    decision = ai.decide(substrate, pop)

    # Act
    if decision.action:
        sim.action(decision.action, *decision.args)

    # Advance simulation time
    sim.step()

result = sim.results()
```

**GPU optimization:**

Simulator is designed for future JAX JIT compilation:
- Flat state structure (no deep scope chains)
- Rate functions are pure Python (will be `jax.jit()`'d)
- Numerical arrays for concentrations, populations
- Step function is the hot loop

**IDE type hints:**

Using `sim.action()` and `sim.measure()` call style means the Simulator class has a fixed API:

```python
class Simulator:
    def step(self, n: int = 1) -> None: ...
    def run(self, ticks: int = None, until: float = None,
            timeout: int = None, quiet: tuple = None, ...) -> None: ...
    def action(self, name: str, *args) -> None: ...
    def measure(self, name: str, *args) -> Any: ...
    def results(self) -> dict: ...
    def available_actions(self) -> list[str]: ...
    def available_measurements(self) -> list[str]: ...
```

No dynamic methods, no IDE warnings, no stub generation needed.

---

## Jobs

A Job is Python code that orchestrates simulations. No special structure - just Python functions that create Loaders, load files, create Simulators, and run them.

```python
def simple_job():
    loader = Loader().load("mutualism.yaml")
    sim = loader.sim(loader.mutualism.baseline)
    sim.run()
    return sim.results()
```

```python
def parameter_sweep():
    loader = Loader().load("mutualism.yaml")

    results = []
    for seed in range(10):
        sim = loader.sim(loader.mutualism.baseline, seed=seed)
        sim.run()
        results.append(sim.results())

    return aggregate(results)
```

```python
def interactive_experiment():
    loader = Loader().load("mutualism.yaml")
    sim = loader.sim(loader.mutualism.baseline)

    # Manual stepping with intervention
    for _ in range(100):
        substrate = sim.measure("sample_substrate", "Lora")
        if substrate["ME1"] < 0.5:
            sim.action("add_feedstock", "Lora", "ME1", 2.0)
        sim.step()

    return sim.results()
```

**Parallel runs:**
```python
def parallel_job():
    def run_one(seed):
        loader = Loader().load("mutualism.yaml")
        sim = loader.sim(loader.mutualism.baseline, seed=seed)
        sim.run()
        return sim.results()

    return parallel([run_one(i) for i in range(10)])
```

Each parallel run creates its own Loader and Simulator for complete isolation.

---

## Open Questions

1. **List merging**: Current proposal is replace-only. Need append? Use `+key:` syntax?

2. ~~**Scope as module**~~: Resolved - Loader is a simple container, not a module. Functions get explicit context arguments.

3. ~~**Load semantics**~~: Resolved - `load()` mutates the loader and returns self for chaining.

4. ~~**IDE type hints for dynamic methods**~~: Resolved - using `sim.action()` and `sim.measure()` call style, no dynamic methods.

5. **Action effect timing**: Should actions have metadata for immediate vs gradual effects? E.g., `adjust_temp` triggers heating that unfolds over time vs `isolate_region` which is immediate.

6. **Termination conditions**: How are scenario termination conditions specified? Time limit? Quiescence? Explicit condition in scoring?

7. **Simulator dt parameter**: Where is the time quantum `dt` specified? Simulator constructor? World definition? Fixed per scenario?

---

## Summary of Syntax

**Spec language (YAML):**

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

**Decorators (Python):**

| Decorator | Purpose | Example |
|-----------|---------|---------|
| `@fn` | Base function decorator | `@fn(summary="...", range=...)` |
| `@scoring` | Scoring function | `@scoring(higher_is_better=True)` |
| `@action` | Agent action (via `sim.action()`) | `@action(targets="regions")` |
| `@measurement` | Agent observation (via `sim.measure()`) | `@measurement(cost="none")` |
| `@rate` | Reaction rate law | `@rate(range=(0, inf))` |
| `@spec_type` | Register a typed element | `@spec_type class world: ...` |

**Runtime API (Python):**

| Construct | Meaning | Example |
|-----------|---------|---------|
| `Loader()` | Create container for loaded content | `loader = Loader()` |
| `loader.load(path)` | Load YAML/Python (mutates, chainable) | `loader.load("mutualism.yaml")` |
| `loader.expand(obj)` | Expand `!ev` and `$` refs | `expanded = loader.expand(scenario)` |
| `loader.sim(scenario)` | Create Simulator from scenario | `sim = loader.sim(loader.baseline)` |
| `sim.step()` | Advance simulation time | `sim.step(n=10)` |
| `sim.run()` | Run simulation | `sim.run(ticks=100)`, `sim.run(until=500.0)` |
| `sim.run(quiet=...)` | Run until measure settles | `sim.run(quiet=("pop_count", "Lora"), span=50)` |
| `sim.action(name, ...)` | Execute action on simulator | `sim.action("add_feedstock", "Lora", "ME1", 5)` |
| `sim.measure(name, ...)` | Get observation from simulator | `sim.measure("sample_substrate", "Lora")` |
| `sim.results()` | Get scoring results | `result = sim.results()` |
