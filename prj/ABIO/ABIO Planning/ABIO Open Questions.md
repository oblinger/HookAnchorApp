# ABIO Open Questions

Design decisions needed for M1.11 documentation sync.

## 1. Two Context Classes

There are two different `Context` classes causing confusion:

- **`infra/context.py: Context`** - Runtime pegboard (config, io, do, create)
- **`spec_lang/eval.py: EvalContext`** - Spec evaluation (rng, bindings, functions, path)

Plus there's an alias `Context = EvalContext` in eval.py!

**Options:**
- A) Rename EvalContext to something else (e.g., `EvalScope`) and remove alias
- B) Keep both, remove the confusing alias
- C) Merge them into one unified Context

### ✅ RESOLVED

**Architecture:**

| Role | Class | Notes |
|------|-------|-------|
| **User-facing sandbox** | `Bio` | Multiple instances allowed. Contains root Scope. Users interact here for loading files, running sims. |
| **Loaded definitions** | `Scope` | Hierarchical namespace for YAML definitions. Root scope attached to Bio; users don't interact directly. |
| **Per-eval-thread** | `EvalEnv` | Renamed from `EvalContext`. Holds rng, bindings for `!ev` evaluation. Name avoids "context" ambiguity. |
| **Simulation** | `Simulator` | No separate context needed - Simulator itself holds simulation state. |

**Actions:**
1. Rename `EvalContext` → `EvalEnv` in `spec_lang/eval.py`
2. Remove the confusing `Context = EvalContext` alias
3. Merge `infra/context.py: Context` functionality into `Bio` (or deprecate if redundant)
4. Ensure Bio has a `_root_scope: Scope` that holds loaded definitions

---

## 2. Bio API Style

The `Bio` class has multiple patterns:
- Instance methods on singleton: `bio.fetch()`, `bio.sim()`
- Pegboard attributes: `bio._simulator_factory`

**Question:** Should Bio methods also be available as module-level functions?
```python
from alienbio import fetch, sim  # Alternative to bio.fetch(), bio.sim()
```

### ✅ RESOLVED

**Pattern:** Singleton instance + class for sandboxes

```python
from alienbio import bio    # singleton for simple usage
from alienbio import Bio    # class for creating sandboxes

# Simple usage - singleton
bio.fetch("scenarios/baseline")
bio.run("experiments/sweep")

# Sandbox usage - isolated instance
sandbox = Bio()
sandbox.fetch("scenarios/test")
```

**Design decisions:**
- `Bio.__init__` stays lightweight (empty pegboard, null root scope) - no lazy singleton needed
- Singleton created eagerly at import time since construction is fast
- Thread safety: not a priority - evaluation happens in EvalEnv, not Bio
- Tests should create their own `Bio()` instances for isolation
- No module-level wrapper functions needed - the singleton IS the API

---

## 3. construct/deconstruct Removal Cleanup

We removed `construct`/`deconstruct` but:
- `@biotype` decorator still exists and registers classes
- Some code may still expect `_type` field handling

**Question:** Is `@biotype` still needed? What's it used for now?

### ✅ RESOLVED

**Pattern:** Entity base class replaces @biotype decorator

- Subclassing `Entity` makes a class a biotype - no decorator needed
- Entity subclasses are automatically hydratable/dehydratable via `to_dict()`/`from_dict()` methods
- Rare non-hydratable entities raise an error if hydration/dehydration is attempted
- Remove `@biotype` decorator entirely

**Actions:**
1. Remove `@biotype` decorator from `decorators.py`
2. Ensure `Entity` base class provides default `to_dict()`/`from_dict()` implementations
3. Update any code that checks `@biotype` registry to use `isinstance(obj, Entity)` instead

---

## 4. Storage and Persistence

- Does `bio run` automatically persist results, or is storage explicit?
- Where do built scenarios go? Default data path for `bio build` output?
- Result aggregation: Scan result folders vs maintain an index?

### ✅ RESOLVED

**Architecture:**

- **Bio has a current DAT** - Set via `cd()` command, used as default for operations
- **`bio.dat` accessor** - Returns current DAT; creates anonymous DAT on first access if needed (lazy creation avoids clutter)
- **Anonymous DAT spec** - Config constant determines where anonymous DATs are created
- **`bio.build(dat_spec)`** - Creates new DAT folder, returns DAT name (not path)
- **`bio.build(non_dat_spec)`** - Evaluates in context of current DAT; writes go there
- **`bio.eval()`** - Creates EvalEnv using this Bio's scope and current DAT

**fetch() Always Returns Data:**

- `bio.fetch(dat_name)` returns the DAT object (data), not a Bio
- `bio.fetch(component_name)` returns the component (Scenario, Chemistry, etc.)
- Internally uses **ORM pattern** - only one version of each DAT exists in memory
- First fetch loads DAT into memory; subsequent fetches return cached instance

**Creating Bio from DAT:**

Use the `dat` constructor parameter:
```python
# From DAT name (string) - fetches internally if needed
sandbox = Bio(dat="experiments/baseline")

# From DAT object directly
dat_obj = bio.fetch("experiments/baseline")
sandbox = Bio(dat=dat_obj)

# No DAT - anonymous DAT created lazily via bio.dat accessor
sandbox = Bio()
```

The `dat` parameter accepts either a string (DAT name) or a DAT object. If a string, Bio fetches the DAT internally when needed.

**Multiple Bios on Same DAT:**

- Discouraged but not disallowed
- Each Bio has its own scope chain
- ORM ensures they share the same underlying DAT data
- Use case: different views/queries on same data

**DAT Name Convention:**

- All commands (`cd`, `fetch`, `build`, etc.) use **DAT names** (full names), not filesystem paths
- Paths starting with `/` are recognized as filesystem paths (escape hatch)
- Internal code may convert DAT name → path for implementation, but:
  - Cross-component APIs use DAT names
  - Persisted data uses DAT names (paths not guaranteed stable)
- DAT class handles the mapping from DAT name to filesystem location

**Persistence behavior:**
- Operations in context of a DAT persist to that DAT's folder
- Future: DATs can sync to cloud storage

---

## 5. DAT Integration

- Reuse DAT's `do` function? It has useful directory creation logic, but Bio needs custom YAML parsing
- Loader hooks: If DAT adds `register_loader()`, should Bio use it?
- `DAT.get/set`: Expose to users or hide behind Bio API?

---

## 6. Naming Conventions

- Recipe names: Must have at least one dot, no slashes? (e.g., `scenarios.baseline`)
- Scenario data paths: Must have slashes? (e.g., `data/scenarios/baseline_42/`)
- Experiment output paths: Template tokens like `{date}`, `{seq}`?

---

## 7. Bio.run() Return Type

What should `Bio.run()` return?
- A) Result object with typed fields
- B) Tuple `(success, metadata)`
- C) Dict with standard keys

---

## 8. Bio.build() Return Type

What should `Bio.build()` return?
- A) DAT object
- B) Path string
- C) Scenario wrapper object

---

## 9. Protocol vs Impl Naming

The codebase uses `*Impl` suffix for implementations:
- `ChemistryImpl`, `StateImpl`, `ReferenceSimulatorImpl`

**Question:** Document this as the standard pattern?

---

## 10. hydrate/dehydrate in eval.py

`eval.py` has `hydrate`/`dehydrate` functions that handle YAML tags.
These are different from the removed `construct`/`deconstruct`.

**Question:** Are these names confusing? Should they be renamed to:
- `parse_tags` / `unparse_tags`
- `expand_placeholders` / `collapse_placeholders`

---

## 11. Simulator Factory Pattern

We just added `_simulator_factory` to Bio. Should other factories follow?
- `_chemistry_factory`
- `_state_factory`

---

## 12. Module Exports

`__init__.py` exports many symbols. Should we:
- A) Keep comprehensive exports for convenience
- B) Require explicit imports from submodules
- C) Use `__all__` to limit star imports

---

## TODOs

### DAT Name Convention Verification

- [ ] Review all documentation to ensure DAT names (full names) are used, not filesystem paths
- [ ] Review code to verify cross-component APIs use DAT names
- [ ] Verify persisted data stores DAT names, not paths
- [ ] Check that paths starting with `/` are handled as filesystem path escape hatch
- [ ] **If any ambiguity about implementation approach, consult user before proceeding**

### ORM Pattern Implementation

- [ ] Document DAT ORM pattern (single in-memory instance per DAT)
- [ ] Implement DAT caching layer for fetch()
- [ ] Implement `Bio(dat=...)` constructor parameter (accepts string or DAT object)
- [ ] Document `bio.dat` accessor with lazy anonymous DAT creation
- [ ] Define anonymous DAT spec constant location in config
