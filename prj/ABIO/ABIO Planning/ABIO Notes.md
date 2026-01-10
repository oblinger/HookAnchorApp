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






