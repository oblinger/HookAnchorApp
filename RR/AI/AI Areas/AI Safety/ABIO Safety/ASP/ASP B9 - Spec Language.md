# B9. Spec Language

*Notation and constructs for world and task specifications*

---

## Evaluation Constructs

| Syntax | Timing | Description |
|--------|--------|-------------|
| `=<EXPR>` | Runtime | Expression becomes Lua, evaluated during simulation |
| `!ev <EXPR>` | Expansion | Evaluate Python expression, insert result into spec |
| `$<NAME>` | Expansion | Substitute constant value from `constants:` section |
| `!include <PATH>` | Expansion | Include external file content |

**Key distinction:**
- `=<EXPR>` → becomes Lua code, runs at simulation time (for rate equations, dynamic values)
- `!ev <EXPR>` → runs Python at expansion time, result is inserted into spec (for generation)

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

All functions use `@fn` decorator with standard metadata:

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

| Metadata | Required | Description |
|----------|----------|-------------|
| `summary` | Yes | Short description for plots/tables |
| `range` | Yes | Expected output range, e.g. `(0.0, 1.0)` or `(0, float('inf'))` |
| (other) | No | Arbitrary kwargs stored as `fn.meta[key]` |

---

## Typed Named Elements

For hierarchical structures (suites containing scenarios), we use `type.name` key syntax.

**Built-in types:** `suite`, `scenario`

**Extensibility:** New types can be registered with `@spec_type` decorator in Python. All names (types, constants, functions) share a single namespace.

**Scenario** is the atomic runnable unit - a fully specified benchmark problem containing:
- `chemistry`, `containers` - the world
- `constitution` - normative objectives
- `briefing` - agent's knowledge
- `scoring` - evaluation metrics (optional)
- `framing`, `feedstock`, `type` - other configuration

**Syntax:**
```yaml
suite.mutualism_studies:
  defaults:
    chemistry: !ev mutualism_chemistry()
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
- `suite` = container, can hold other suites or scenarios
- `scenario` = leaf, the fully specified runnable unit
- Children inherit from parent's `defaults:` (deep merge)
- Scalars replace, dicts merge

---

## Defaults and Inheritance

```yaml
suite.parent:
  defaults:
    chemistry: !ev base_chemistry()
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
# B10-style spec file

include:
  - lib/functions.py
  - lib/kinetics.lua

interpreter:
  lua: |
    -- Lua functions for runtime
  py: |
    # Python functions with @fn decorator

constants:
  name: value
  profile: {k1: v1, k2: v2}

suite.example:
  defaults:
    chemistry:
      molecules: ...
      reactions: ...

    containers:
      ecosystems: ...
      regions: ...
      organisms: ...

    type: maintain
    feedstock: ...
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

**Scenario** is the atomic runnable unit:
- A data structure representing a computation to be done
- Has a `.run()` method (returns trace + results)
- Functional model: params in → trace + results out

**Job** is Python code that orchestrates scenarios:
- Might run one scenario or many
- Might loop, vary parameters, generate reports
- Uses library helpers as needed, but no rigid structure

**Loading vs Expansion:**
- `load(<PATH>)` - parses YAML, returns unexpanded template
- `template.expand(**overrides)` - evaluates `!ev` expressions, returns concrete scenario
- Can expand same template multiple times (different random seeds, parameter overrides)

```python
# Simple: load, expand, run
scenario = load("test.yaml").expand()
result = scenario.run()

# Multiple expansions with different seeds
template = load("test.yaml")
for seed in range(10):
    scenario = template.expand(seed=seed)
    results.append(scenario.run())
```

**Relationship to spec structure:**
- `suite` = YAML organizational structure (groups scenarios with shared defaults)
- `scenario` = both a YAML leaf element AND the runtime object with `.run()`
- `job` = pure Python, not represented in YAML

---

## Open Questions

1. **List merging**: Current proposal is replace-only. Need append? Use `+key:` syntax?

2. **Additional types**: Beyond `suite` and `scenario`, what other typed elements might we need?

---

## Summary of Syntax

| Syntax | Meaning | Example |
|--------|---------|---------|
| `=<EXPR>` | Runtime Lua expression | `rate: =mass_action(k=0.1)` |
| `!ev <EXPR>` | Expansion-time Python eval | `molecules: !ev energy_ring(6)` |
| `$<NAME>` | Constant substitution | `outflows: $standard_diffusion` |
| `!include <PATH>` | File inclusion | `constitution: !include safety.md` |
| `suite.<NAME>` | Suite (container) | `suite.mutualism_studies:` |
| `scenario.<NAME>` | Scenario (runnable unit) | `scenario.baseline:` |
| `^` | Parent reference (in outflows) | `outflows: {^: $permeability}` |
| `0` | Catalyst coefficient | `0 MC_krel + 2 ME1 -> ME2` |
| `@spec_type` | Register a type (Python) | `@spec_type class suite: ...` |
| `@fn` | Register a function (Python) | `@fn(summary="...", range=...) def f(): ...` |
