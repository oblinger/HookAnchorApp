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

### ✅ RESOLVED

**DAT's `do` function:** Don't use internally.

- `do.load()` uses `yaml.safe_load()` which doesn't handle Bio's custom tags (`!ref`, `!ev`, `!include`)
- No scope inheritance semantics (`extends:`)
- Different caching model than Bio's ORM-style DAT caching
- Bio implements its own loader for YAML tag resolution, scope inheritance, and type hydration

**DAT get/set:** Forward directly to Bio class.

```python
from dvc_dat import Dat

class Bio:
    get = Dat.get    # direct assignment, no trampoline
    set = Dat.set
```

Users interact with Bio only - no need to import or think about DAT class for nested dict access.

**Loader hooks:** Not needed. Bio handles its own YAML loading pipeline.

---

## 6. Fetch String Resolution

How does `bio.fetch(string)` resolve its argument?

### ✅ RESOLVED

**Three access patterns:**

| Pattern | Syntax | Example |
|---------|--------|---------|
| **DAT access** | Contains `/` | `experiments/baseline.scenario` |
| **Module access** | All dots, matches loaded module | `alienbio.catalog.worlds` |
| **Source root access** | All dots, no module match | `scenarios.mutualism.test1` |

---

**Routing Algorithm:**

```
fetch(string):
    if "/" in string:
        → DAT access
    else:
        if first_segment matches a loaded Python module:
            → Module access
        else:
            → Source root access
```

---

**Pattern 1: DAT Access** (string contains `/`)

```
experiments/baseline/run_42.results.scores
└─────────DAT name─────────┘ └──dig path──┘
```

1. Split at first `.` that follows the DAT name (everything up to and including last `/` segment)
2. Load the DAT (via ORM cache)
3. Use **dig operation** with remaining dotted path into DAT's content

**Examples:**
```python
fetch("experiments/baseline")           # → DAT object itself
fetch("experiments/baseline.scenario")  # → scenario field from DAT
fetch("experiments/baseline.results.scores.health")  # → nested access
```

**Constraint:** DAT names must contain `/`. This disambiguates from module/source access.

---

**Pattern 2: Module Access** (all dots, first segment is loaded module)

```
alienbio.catalog.worlds.ecosystem
└──module name──┘ └─attribute path─┘
```

1. Import the module (first segment, or first N segments that resolve)
2. Get the global variable (next segment)
3. Dig into the variable with remaining segments using `Bio.get()`

**Examples:**
```python
fetch("alienbio.catalog.worlds")        # → worlds module/dict
fetch("alienbio.catalog.worlds.ecosystem")  # → ecosystem from worlds
fetch("mymodule.CONFIG.timeout")        # → CONFIG["timeout"] or CONFIG.timeout
```

---

**Pattern 3: Source Root Access** (all dots, no module match)

```
scenarios.mutualism.test1.interface
└───filesystem scan───┘ └─dig path─┘
```

1. For each configured source root (in order):
   - Use **filesystem dig operation** to find YAML file
   - If found, load and hydrate it
   - Dig into structure with remaining path
2. Error if not found in any root

**Filesystem dig operation:**

For path `a.b.c.d` starting at root `/src`:
```
Try: /src/a.yaml           → if exists, load, dig into ["b"]["c"]["d"]
Try: /src/a/b.yaml         → if exists, load, dig into ["c"]["d"]
Try: /src/a/b/c.yaml       → if exists, load, dig into ["d"]
Try: /src/a/b/c/d.yaml     → if exists, load, return root
Try: /src/a/index.yaml     → if exists, load, dig into ["b"]["c"]["d"]
Try: /src/a/b/index.yaml   → if exists, load, dig into ["c"]["d"]
... (prefer explicit name over index.yaml at each level)
```

**Examples:**
```python
fetch("scenarios.mutualism")           # finds scenarios/mutualism.yaml or scenarios/mutualism/index.yaml
fetch("scenarios.mutualism.test1")     # loads mutualism.yaml, digs into ["test1"]
fetch("worlds.ecosystem.molecules.ME1")  # nested access
```

---

**Shared Dig Operation:**

Both DAT access and source root access use the same dig semantics once a root is established:

```python
def dig(root: dict, path: list[str]) -> Any:
    """Dig into a loaded structure by dotted path segments."""
    result = root
    for segment in path:
        if isinstance(result, dict):
            result = result[segment]
        elif hasattr(result, segment):
            result = getattr(result, segment)
        else:
            raise KeyError(f"Cannot find {segment} in {type(result)}")
    return result
```

---

**Source Roots Configuration:**

```python
# In Bio or config
source_roots = [
    "/path/to/project/catalog",    # project-specific specs
    "/path/to/alienbio/catalog",   # library defaults
]
```

Searched in order; first match wins.

---

**Edge Cases:**

| Input | Resolution |
|-------|------------|
| `""` (empty) | Error |
| `"foo"` (single segment, no module match) | Source root scan for `foo.yaml` or `foo/index.yaml` |
| `"foo"` (single segment, is module) | Return the module |
| `"foo/bar"` | DAT access (has `/`) |
| `"foo.bar"` | Module or source root (no `/`) |

---

**Design Rationale:**

- `/` unambiguously signals DAT access (filesystem paths use slashes)
- Module access gets priority for dotted names (explicit imports)
- Source root is fallback for spec files in the repository
- Same dig semantics for DAT and source roots reduces cognitive load
- `index.yaml` convention allows directory-as-module pattern

---

## 7. Run Method Design

What should `Bio.run()` return? How does run relate to entities?

### ✅ RESOLVED

**Two-level design:**

**Entity.run() → Any**

Each entity subclass defines its own `run()` that returns whatever makes sense:

```python
class Entity:
    def run(self) -> Any:
        """Override in runnable subclasses."""
        raise NotImplementedError(f"{type(self).__name__} is not runnable")

class Scenario(Entity):
    def run(self) -> SimulationTrace:
        """Run simulation, return traces."""
        ...

class Report(Entity):
    def run(self) -> Path:
        """Generate report, return output path."""
        ...

class Experiment(Entity):
    def run(self) -> list[dict]:
        """Run scenarios, return list of results."""
        ...
```

**Bio.run() → dict**

Bio wraps entity execution with metadata:

```python
class Bio:
    def run(self, target, **kwargs) -> dict:
        entity = self.build(target, **kwargs) if isinstance(target, (str, dict)) else target
        result = entity.run()
        return {
            "result": result,      # whatever entity returned
            "success": True,       # completed without error
            "dat": "...",          # where results stored
            "elapsed": 12.3,       # timing
            # ... extensible
        }
```

**Design rationale:**
- Entities stay simple - return their natural domain-specific result
- Bio provides uniform dict wrapper for CLI/tooling
- Dict is extensible (add fields without breaking callers)
- Clear separation: Entity does work, Bio orchestrates
- No forced structure on entities that don't need it

---

## 8. Bio.build() Return Type

What should `Bio.build()` return?

### ✅ RESOLVED

**Return type determined by spec content:**

- Spec has `_type` → returns instance of that type
- Spec has `dat` key (no `_type`) → returns Dat object
- Neither → returns hydrated dict (error to have both `_type` and `dat`)

**Hydration algorithm (bottom-up):**

1. Depth-first traversal - always recurse into children
2. On the way back up at each node:
   - If `_type` present → construct that class, passing hydrated children
   - If no `_type` → stays as dict
3. No special markers needed - absence of `_type` means "don't hydrate this level"

```python
def hydrate(data: dict) -> Any:
    """Bottom-up hydration based on _type field."""
    # First, recurse into all dict children
    result = {}
    for key, value in data.items():
        if isinstance(value, dict):
            result[key] = hydrate(value)  # recurse
        elif isinstance(value, list):
            result[key] = [hydrate(v) if isinstance(v, dict) else v for v in value]
        else:
            result[key] = value

    # Then, hydrate this level if _type present
    if "_type" in result:
        cls = get_type_class(result["_type"])
        return cls(**{k: v for k, v in result.items() if k != "_type"})
    return result
```

**Design rationale:**
- Simple rule: `_type` present means hydrate, absent means dict
- No special markers or annotations needed
- Natural behavior handles "raw" cases - just omit `_type`
- Consistent with typed key syntax in YAML

---

## 9. Protocol vs Impl Naming & Factory Pattern

The codebase uses `*Impl` suffix for implementations:
- `ChemistryImpl`, `StateImpl`, `ReferenceSimulatorImpl`

### ✅ RESOLVED

**Standard naming pattern:**
- Protocol: `Simulator`, `Chemistry`, `State` (abstract interface)
- Implementation: `*Impl` suffix - `ReferenceSimulatorImpl`, `FastSimulatorImpl`
- Multiple implementations allowed per protocol

---

### ✅ RESOLVED: Factory Pattern

**Decorator to register implementations:**

```python
@factory(name="reference", protocol=Simulator)
class ReferenceSimulatorImpl(Simulator):
    """Reference implementation - accurate but slow."""
    ...

@factory(name="fast", protocol=Simulator)
class FastSimulatorImpl(Simulator):
    """Optimized implementation - faster but approximations."""
    ...
```

**Registry on Bio:**

```python
class Bio:
    # Populated by @factory decorators at import time
    _factories: dict[type, dict[str, type]]  # protocol → {name → impl_class}
    _factory_defaults: dict[type, str]       # protocol → default_name (from config)
```

**Entry point is `build()`, not a separate `create()`:**

Everything goes through specs - objects have too many parameters to create from nothing.

```python
def build(self, spec, impl: str = None, **kwargs) -> Any:
    """Build from spec with optional implementation override.

    Args:
        spec: Spec dict or specifier string
        impl: Override implementation name (optional)
    """
    # impl flows down to hydration
    return self._hydrate(spec, impl_override=impl)
```

**Implementation resolution order:**

1. `impl` parameter to `build()` → programmatic override
2. `impl` field in spec → spec-defined choice
3. Default from config registry → user's configured default
4. None of above → runtime error

**Spec fields (no underscores - regular fields):**

```yaml
scenario.test:
  chemistry:
    type: Chemistry    # protocol/class name
    impl: fast         # implementation name (optional)
    molecules: ...
```

- `type` - which protocol/class (required for typed objects)
- `impl` - which implementation of that type (optional, defaults from registry)

**Validation:**
- If `impl` passed to `build()`, verify it's valid for the spec's `type`
- Error if mismatch: "impl 'fast' is not valid for type 'Scenario'"

**Defaults in config (not in decorator):**

```yaml
# ~/.config/alienbio/config.yaml
defaults:
  Simulator: reference
  Chemistry: standard
```

**Design rationale:**
- Specs always source of truth (handles complex params)
- Programmatic override via `build(spec, impl="fast")`
- No `default=True` in decorators - config controls defaults
- Single path: build → hydrate → construct

---

## 10. hydrate/dehydrate Location and Naming

`eval.py` has `hydrate`/`dehydrate` functions that handle YAML tags.

### ✅ RESOLVED

**Location:** Module-level functions in `alienbio`, not on Bio class.

```python
from alienbio import bio, hydrate, dehydrate  # if needed

# Most users just use fetch (hydrates by default):
scenario = bio.fetch("...")
```

**Rationale:**
- Bio class stays focused on main operations (fetch, build, run, report)
- hydrate/dehydrate rarely called directly (fetch does it)
- Still accessible for advanced use cases (tooling, debugging)
- Cleaner API surface

**Names:** Keep `hydrate`/`dehydrate` - they're clear and standard terminology.

**Pipeline clarity:**

| Stage | What Happens | Tags Resolved |
|-------|--------------|---------------|
| **fetch()** | Load YAML, hydrate by default | — |
| **hydrate()** | Resolve structural tags, construct types | `!include`, `!ref` |
| **build()** | Template expansion | — |
| **eval()** | Execute expressions | `!ev` |

- `!_` (Quoted) preserved for runtime (rates, scoring)
- fetch() hydrates by default; `hydrate=False` to skip

---

## 11. Simulator Factory Pattern

We just added `_simulator_factory` to Bio. Should other factories follow?

### ✅ RESOLVED (see #9)

Yes - unified factory pattern via `@factory` decorator applies to all protocols:
- Simulator, Chemistry, State, etc.
- Single registry on Bio: `_factories` and `_factory_defaults`
- No separate `_simulator_factory`, `_chemistry_factory` - one unified system

See Question #9 for full factory pattern design.

---

## 12. Module Exports

`__init__.py` exports many symbols. Should we:
- A) Keep comprehensive exports for convenience
- B) Require explicit imports from submodules
- C) Use `__all__` to limit star imports

### ✅ RESOLVED

**Option C with curation** - use `__all__` to define clean public API.

```python
# alienbio/__init__.py

# Main user API
from .spec_lang.bio import Bio
bio = Bio()  # singleton

# Module-level functions
from .spec_lang.eval import hydrate, dehydrate

# Core protocols (what users type-hint against)
from .protocols import Simulator, Chemistry, State, Entity, Scenario

# Curated public API for star imports
__all__ = [
    "bio", "Bio",
    "hydrate", "dehydrate",
    "Entity", "Scenario", "Chemistry", "Simulator", "State",
]

# Impl classes available but NOT in __all__
from .impl import ReferenceSimulatorImpl, ChemistryImpl  # etc.
```

**Result:**
- `from alienbio import bio` → clean main API
- `from alienbio import *` → curated set only
- `from alienbio import ReferenceSimulatorImpl` → works for advanced users
- Impl classes not pushed on beginners

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

### Fetch String Resolution Implementation

Test file created: `tests/unit/test_fetch_resolution.py` (50+ test cases)

- [ ] Implement routing logic (`/` → DAT, dots → module or source root)
- [ ] Implement DAT name parsing (extract name + dig path)
- [ ] Implement module access (import + attribute dig)
- [ ] Implement source root scanning (YAML file discovery)
- [ ] Implement shared dig operation (dict key / attribute access)
- [ ] Implement ORM caching for DAT access
- [ ] Implement source root configuration
- [ ] Enable and pass all tests in `test_fetch_resolution.py`
- [ ] Handle edge cases (empty string, unicode, whitespace, etc.)

### Fetch User Documentation

- [ ] Merge fetch string resolution spec (from Question #6 above) into user docs at `docs/architecture/commands/ABIO Fetch.md`
- [ ] Structure: concise overview at top explaining the three access patterns
- [ ] Add extensive examples section covering corner cases:
  - DAT access with/without dig paths
  - Module access with nested attributes
  - Source root scanning with index.yaml fallback
  - Edge cases (single segment, dots in DAT names, etc.)
- [ ] Polish for clarity and readability

### Documentation Integration Audit

- [ ] Scan all resolved questions (#1-12) in this file
- [ ] For each resolution, verify details are integrated into relevant system docs:
  - Bio.md, Scope.md, Entity docs, command docs, etc.
- [ ] Ensure no design decisions are lost in this planning doc only
- [ ] Cross-reference: planning doc should point to where details live in real docs

### Factory Pattern Documentation

- [ ] Create new doc: `docs/architecture/Factory Pattern.md`
- [ ] Document `@factory` decorator usage and registration
- [ ] Document implementation resolution order (build param → spec field → config default)
- [ ] Document config file format for defaults
- [ ] Add examples for creating custom implementations
- [ ] Update Bio.md to reference factory pattern doc

### Code Updates from Design Decisions

- [ ] Move `hydrate`/`dehydrate` from Bio class to module-level functions in `alienbio/__init__.py`
- [ ] Update Bio.fetch() to call module-level hydrate()
- [ ] Implement `@factory` decorator in `alienbio/decorators.py`
- [ ] Implement factory registry on Bio (`_factories`, `_factory_defaults`)
- [ ] Update existing `*Impl` classes with `@factory` decorators
- [ ] Add `impl` parameter to `build()`
- [ ] Implement `Entity.run()` base method with NotImplementedError
- [ ] Add `run()` methods to Scenario, Experiment, Report classes

### Pipeline Documentation Consistency

- [ ] Verify all docs use consistent pipeline: fetch → hydrate → build → eval
- [ ] Update Bio.md methods table to NOT include hydrate/dehydrate (they're module-level now)
- [ ] Update ABIO Hydrate.md to note it's module-level and called by fetch by default
- [ ] Ensure Spec Language Reference matches Core Spec on tag resolution timing
- [ ] Add note to hydrate docs: "Called by fetch() by default - most users don't need this directly"

### Module Exports Cleanup

- [ ] Refactor `alienbio/__init__.py` to use curated `__all__`
- [ ] Export main API: `bio`, `Bio`, `hydrate`, `dehydrate`
- [ ] Export core protocols: `Entity`, `Scenario`, `Chemistry`, `Simulator`, `State`
- [ ] Keep `*Impl` classes importable but NOT in `__all__`
- [ ] Verify `from alienbio import *` only gets curated set
- [ ] Update any docs that reference old import patterns
