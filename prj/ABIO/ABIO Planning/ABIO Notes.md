# ABIO MISCELLANEOUS NOTES


## 2026-01-09  DAT Refactor Idea

### Core Insight
A DAT is just a folder where bio commands run. DAT handles folder/naming infrastructure; Bio handles all domain logic. Users only need two operations.

### Bio Verbs

| Verb | Input | Output | Description |
|------|-------|--------|-------------|
| **build** | dotted name + seed | DAT folder | Create folder, generate bio content, write files |
| **run** | DAT path | results | Execute commands in the `run:` section |

`build` internally does DAT lookup + bio generation. Users don't see lookup.

Other operations (report, query, etc.) are just bio commands that can appear in `run:` sections. See [[ABIO Commands|Commands]].

**Compound:** `bio run scenarios.baseline` = build + run (when given dotted name)

### Naming Concern

"run" is overloaded:
- DAT has a `.run()` method
- Spec has a `run:` section  
- Bio CLI has `bio run`

Options:
- Keep it — the DAT run method executes the run section, so they align
- Use **execute** at Bio API level: `Bio.execute(dat_path)`

### DAT Spec Structure

```yaml
scenarios.baseline:
  path: data/scenarios/baseline_{seed}/

  build:
    index.yaml: scenarios.baseline      # Bio.build → write here

  run:
    - run . --agent claude              # run current DAT with agent
    - report .                          # generate report
```

### What Happens

**`bio build scenarios.baseline --seed 42`**
1. Create folder `data/scenarios/baseline_42/`
2. For each entry in `build:`:
   - Bio.build the referenced generator
   - Write result to specified path
3. Done.

**`bio run data/scenarios/baseline_42/`**  (or `bio execute`)
1. cd to DAT folder
2. Execute each command in `run:` sequentially
3. Results written to folder.

### Why This Works

- **Two verbs** — build and run. That's it.
- **Single language** — Commands in `run:` are bio commands.
- **Transparent** — Reads like what you'd type manually.
- **Composable** — Experiments just spawn child DATs from their run section.


## Architecture-First Organization

Protocols are defined in an `architecture/` folder, separate from implementations. This enforces clear contracts and enables multiple implementations (e.g., PythonSimulator vs RustSimulator). See [[#Folder Structure]] for full layout.


## Folder Structure

```
alienbio/
  pyproject.toml       # uv-managed dependencies, Python >=3.12
  justfile             # build, test, run commands
  README.md
  .gitignore

  src/
    architecture/
      entities.py      # BioMolecule, BioReaction, BioSystem, BioOrganism protocols
      generators.py    # MoleculeGenerator, ReactionGenerator, SystemGenerator protocols
      simulation.py    # State, Simulator, Timeline, World protocols
      interface.py     # Measurement, Action, Task protocols
      experiment.py    # Experiment, Test, Harness protocols

    impl/
      entities/        # BioMolecule, BioReaction, etc. implementations
      generators/      # MoleculeGenerator, ReactionGenerator, etc. implementations
      simulation/      # PythonSimulator, RustSimulator, etc. implementations
      interface/       # Measurement, Action, Task implementations
      experiment/      # Experiment, Test, Harness implementations

  tests/               # unit and integration tests
  data/                # managed via dvc_dat, structure TBD
  scripts/             # CLI entry points, one-off utilities

  rust/                # Rust simulator implementation
    Cargo.toml
    src/
      lib.rs           # PyO3 bindings

  dvc_dat -> ~/ob/proj/dvc_dat   # symlink, temporary until packaged
```


## Justfile Commands

- `build` - build all components (Python package, Rust bindings)
- `build-rust` - compile Rust simulator with maturin/pyo3
- `test` - run pytest on tests/ folder
- `check` - run pyright type checking
- `fmt` - run ruff format on src/
- `lint` - run ruff check on src/
- `clean` - remove build artifacts, __pycache__, .pyc files
- `run` - main entry point (TBD)


## CI/CD

GitHub Actions workflow:
- On push/PR: run `test`, `check`, `lint`
- Ensures tests pass and types check before merge


## 2026-01-10 M2 Test Coverage Audit

### Summary
Created `tests/unit/test_bio_m2.py` with 26 tests covering M2 features. Results: 8 passing, 18 skipped (marking unimplemented features).

### Gaps Discovered

1. **MoleculeImpl not registered as @biotype** — `Bio.store()` calls `dehydrate()` which requires typed objects to be registered with the `@biotype` decorator. Need to add `@biotype` to `MoleculeImpl` class.

2. **Bio.cd() not implemented** — Tests marked as skipped. Need to implement current DAT tracking on Bio instance.

3. **index.yaml convention not implemented** — `fetch(dat_folder)` currently looks for `spec.yaml`, not `index.yaml`. Decide on convention.

4. **Dotted name lookup not implemented** — `fetch('catalog.scenarios.test')` should route to lookup searching configured roots. Currently not implemented.

5. **Python module lookup not implemented** — `fetch('alienbio.bio.Chemistry')` should return Python class. Not yet implemented.

6. **Dotted path dereferencing not implemented** — `fetch('path/to/dat.nested.key')` should load index.yaml then dereference. Not yet implemented.

7. **hydrate=False option** — `fetch(..., hydrate=False)` should return Scope without type construction. Not yet implemented.

8. **raw=True option for store** — `Bio.store(path, obj, raw=True)` should write plain dict without dehydration. Not yet implemented.

### Test Categories

- **Bio.cd()** — 5 tests (all skipped)
- **Bio.fetch() routing** — 5 tests (2 passing, 3 skipped)
- **Bio.fetch() hydration** — 2 tests (1 passing, 1 skipped)
- **Bio.fetch() DAT pattern** — 2 tests (all skipped)
- **Bio.run() routing** — 3 tests (2 passing, 1 skipped)
- **Bio.store()** — 5 tests (all skipped)
- **Bio.build() edge cases** — 3 tests (2 passing, 1 skipped)
- **Integration** — 2 tests (all skipped)






