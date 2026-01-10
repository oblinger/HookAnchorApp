# ABIO MISCELLANEOUS NOTES

## 2026-01-09  DAT Refactor Idea




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
