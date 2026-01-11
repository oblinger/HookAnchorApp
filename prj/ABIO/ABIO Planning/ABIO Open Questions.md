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

---

## 2. Bio API Style

The `Bio` class has multiple patterns:
- Instance methods on singleton: `bio.fetch()`, `bio.sim()`
- Pegboard attributes: `bio._simulator_factory`

**Question:** Should Bio methods also be available as module-level functions?
```python
from alienbio import fetch, sim  # Alternative to bio.fetch(), bio.sim()
```

---

## 3. construct/deconstruct Removal Cleanup

We removed `construct`/`deconstruct` but:
- `@biotype` decorator still exists and registers classes
- Some code may still expect `_type` field handling

**Question:** Is `@biotype` still needed? What's it used for now?

---

## 4. Storage and Persistence

- Does `bio run` automatically persist results, or is storage explicit?
- Where do built scenarios go? Default data path for `bio build` output?
- Result aggregation: Scan result folders vs maintain an index?

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

