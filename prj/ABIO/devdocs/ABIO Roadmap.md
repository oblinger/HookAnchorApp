# ABIO Roadmap
**Related**: [[alienbio]], [[Testing]]
Implementation roadmap for the alienbio project.

---

# Milestone 1 - Minimal Simulation Loop

Core architecture: entities, protocols, and basic simulation.

## [x] M1.0 - Environment Setup
### [x] Create alienbio repository parallel to abio1
### [x] Initialize uv environment with pyproject.toml
### [x] Create justfile with standard commands
### [x] Set up folder structure per architecture spec
### [x] Configure pytest for tests/ folder
### [x] Add rust/ subfolder with Cargo.toml placeholder
### [x] Test: `just test` runs and passes with empty test suite

### .

## [x] M1.1 - Infrastructure Operators

Top-level operations for working with alienbio. See [[alienbio]] module topic for full specification.

### [x] Implement do(name) - resolve dotted names to objects
### [x] Implement create(spec) - instantiate from prototypes
### [x] Implement load(path) / save(obj, path) - entity persistence
### [x] Implement ctx() - access runtime context
### [x] Test: do("catalog.kegg1") resolves (with stub)
### [x] Test: create(spec) instantiates object from prototype
### [x] Test: save(obj, path) writes to data/, load(path) retrieves it
### [x] Test: ctx() returns Context with ContextVar (not plain global)

### .

## [x] M1.2 - DAT Integration

Configure dvc_dat so do/create/load/save resolve properly. See [[ABIO DAT]].

**Development Setup**: During co-development of alienbio and dvc_dat, use symlink:
- `src/dvc_dat -> ../dvc-dat/dvc_dat` (dvc-dat branch: feat/sv-update-fix)
- This allows editing dvc_dat in place without publish/reinstall cycles
- Once stable, add dvc_dat as proper dependency in pyproject.toml

### [x] Symlink dvc_dat from local dvc-dat repo into src/
### [x] Configure Context.data_path to point to data/ folder
### [x] Configure do() to translate dotted names to dvc_dat slash-based keys
### [x] Set up .datconfig.json with mount_commands for catalog/ and fixtures/
### [x] Set up test fixtures accessible via do("fixtures.simple")
### [x] Test: do("fixtures.X") resolves to test fixture data
### [x] Test: create() instantiates from YAML spec in fixtures folder
### [x] Test: load/save round-trip through data/ folder
### [x] Create tests/unit/test_dat.py for dvc_dat usage patterns

### .

## [x] M1.3 - Entity Infrastructure

Base classes for all alienbio objects. See [[Entity]], [[alienbio]].

### [x] Implement Entity base class with name, description
### [x] Entity.__str__ accesses Context for PREFIX:name display
### [x] Implement parse(string) - reconstruct entity from string
### [x] Implement YAML serialization for complex entities
### [x] Implement deserialization from string and YAML
### [x] Test: print(Entity) produces PREFIX:name format
### [x] Test: parse(str(entity)) round-trips correctly
### [x] Test: round-trip Entity to YAML and back, assert equality
### [x] Test: save entity to data/, load by name, assert equality

### .

## [x] M1.4 - Core Protocols

### [x] Define BioMolecule protocol with name, properties
### [x] Define BioReaction protocol with reactants, products, rate
### [x] Define State protocol as dict of molecule concentrations
### [x] Define Simulator protocol with step() method
### [x] All bio classes (BioMolecule, BioReaction, etc.) inherit from Entity
### [x] Test: pyright passes on all protocol definitions

### .

## [x] M1.5 - Spec Language Module

Implement `alienbio/spec_lang` module with YAML tags, decorators, and Bio class. See [[Spec Language]], [[Decorators]], [[Bio]].

**Status**: Complete — 83 tests passing, 3 skipped.

### [x] Create spec_lang module structure and test scaffold
### [x] Implement `!ev` tag — evaluate Python expressions
### [x] Implement `!ref` tag — reference constants
### [x] Implement `!include` tag — load markdown, YAML, Python files
### [x] Implement typed keys transformation (`type.name:` → `_type` field)
### [x] Implement `@biotype` decorator and hydrate/dehydrate
### [x] Implement function decorators (`@fn`, `@scoring`, `@action`, `@measurement`, `@rate`)
### [x] Implement `expand_defaults()` — deep merge for suite/scenario hierarchy
### [x] Implement Bio class with load(), save(), sim() static methods

### .

## [x] M1.6 - Hardcoded Test Job (DAT)

Build a complete job DAT that defines, runs, and verifies a hardcoded test system.

**Status**: Complete — Job DAT created, Job biotype implemented, 6 tests passing.

### [x] Create job DAT structure in `catalog/jobs/hardcoded_test/`
### [x] Define molecules by hand in spec: A, B, C, D
### [x] Define reactions by hand: A + B → C, C → D
### [x] Define fixed rate functions (constant rates) via `!ev` or inline
### [x] Define initial state with known concentrations
### [x] Add `run:` section with step count and optional quiescence params
### [x] Add `verify:` section with assertions on final concentrations
### [x] Add `scoring:` section referencing `@scoring` functions
### [x] Test: `Bio.fetch("jobs/hardcoded_test")` loads and hydrates correctly

### .

## [x] M1.6b - Documentation & Vocabulary

Documentation for spec system and alien naming conventions.

### [x] Create Scope.md architecture doc
### [x] Create Bio CLI.md for command-line interface
### [x] Create Alien Vocabulary.md with naming conventions (organisms, regions, molecules, reactions)
### [x] Define bioref format in Bio.md as authoritative source
### [x] Fix architecture docs to use Overview instead of Description

### .

## [x] M1.7 - Python Simulator & CLI

Implement the simulator and `bio` CLI command.

**Status**: Complete — Bio singleton with simulator registry, CLI entry point, 339 tests passing.

### [x] Design comprehensive test coverage for simulator (18 tests)
### [x] Implement `step()` applying all reactions once
### [x] Implement `run(steps)` looping step() for N iterations
### [x] Return timeline of states
### [x] Rename SimpleSimulatorImpl → ReferenceSimulatorImpl
### [x] Refactor Bio to singleton with instance methods
### [x] Add simulator registry (register_simulator, create_simulator)
### [x] Create `bio` CLI entry point in pyproject.toml
### [x] Test: `bio jobs/hardcoded_test` runs job from command line

### .

## [ ] M1.8 - Spec Evaluation Implementation

Implement the spec evaluation system per [[Spec Evaluation]] specification. Uses Python expression strings (not Expr trees — see [[Expr]] for deferred design).

**Reference Docs**: [[Spec Evaluation]], [[Spec Language]]

**Design Summary**:
- `!_` tag → evaluate Python expression immediately
- `!quote` tag → preserve expression unchanged (for later compilation)
- `!ref` tag → lookup named value from bindings
- `!include` tag → read file contents (resolved during hydration)
- Hydration = type instantiation + tag→placeholder conversion
- Context carries `rng`, `bindings`, `functions`, `path`

### [ ] M1.8a - Write Comprehensive Tests First

Create test suite BEFORE implementation. Tests serve as executable specification.

**Test file**: `tests/unit/test_spec_eval.py`

See detailed test cases in M1.8a-tests section below.

### [ ] M1.8b - Placeholder Classes
- [ ] Implement `Evaluable(source: str)` — placeholder for `!_` expressions
- [ ] Implement `Quoted(source: str)` — placeholder for `!quote` expressions
- [ ] Implement `Reference(name: str)` — placeholder for `!ref` expressions
- [ ] All placeholders are simple dataclasses with `source`/`name` attribute

### [ ] M1.8c - Hydrate Implementation
- [ ] Implement `Bio.hydrate(data)` — recursive transformation
- [ ] Type instantiation: dicts with `_type` field → Python class instances
- [ ] Tag conversion: `!_` → Evaluable, `!quote` → Quoted, `!ref` → Reference
- [ ] `!include` resolution: read file, insert contents (during hydration)
- [ ] Recursive descent into dicts and lists

### [ ] M1.8d - Dehydrate Implementation
- [ ] Implement `Bio.dehydrate(data)` — reverse of hydrate
- [ ] Python instances → dicts with `_type` field
- [ ] Evaluable → `{"!_": source}`, Quoted → `{"!quote": source}`, Reference → `{"!ref": name}`
- [ ] Round-trip property: `dehydrate(hydrate(x))` ≈ `x`

### [ ] M1.8e - Context Object
- [ ] Implement Context class with `rng`, `bindings`, `functions`, `path`
- [ ] `rng`: seeded numpy RNG for reproducibility
- [ ] `bindings`: dict of variable name → value
- [ ] `functions`: dict of registered @function handlers
- [ ] `path`: list of keys for error messages (e.g., `["scenario", "molecules", "count"]`)
- [ ] Context nesting: child context can shadow parent bindings

### [ ] M1.8f - Eval Implementation
- [ ] Implement `Bio.eval(node, ctx, strict=True)`
- [ ] Constants (str, int, float, bool, None) → return as-is
- [ ] Evaluable → Python `eval(source, safe_builtins, namespace)`
- [ ] Quoted → return `source` string unchanged
- [ ] Reference → lookup in `ctx.bindings`, error if missing (strict mode)
- [ ] dict → recursively eval values
- [ ] list → recursively eval elements
- [ ] Typed objects → eval their evaluable fields

### [ ] M1.8g - @function Decorator
- [ ] Implement `@function` decorator for registering functions
- [ ] Auto-inject `ctx` parameter when called from eval
- [ ] User writes `!_ normal(50, 10)`, evaluator calls `normal(50, 10, ctx=ctx)`
- [ ] Function registry accessible via `ctx.functions`

### [ ] M1.8h - Built-in Functions
- [ ] Distribution functions: `normal`, `uniform`, `lognormal`, `poisson`, `exponential`
- [ ] Choice functions: `discrete(weights, *choices)`, `choice(*choices)`
- [ ] All use `ctx.rng` for reproducibility

### [ ] M1.8i - Default Namespace
- [ ] Define `SAFE_BUILTINS` set: `min`, `max`, `abs`, `round`, `sum`, `len`, etc.
- [ ] Evaluation namespace = `SAFE_BUILTINS` + `ctx.bindings` + `ctx.functions`
- [ ] No dangerous builtins (`exec`, `eval`, `import`, `open`, etc.)

### [ ] M1.8j - Integration
- [ ] Wire hydrate/eval into `Bio.load()` flow
- [ ] `Bio.load()` returns hydrated but unevaluated spec
- [ ] `Bio.eval()` called separately (allows multiple instantiations)
- [ ] Rate expressions (`!quote`) survive through to Scenario object

### .

## [ ] M1.8a-tests - Spec Evaluation Test Suite

Comprehensive tests for spec evaluation. **Create these BEFORE implementation.**

### [ ] Hydration Tests (`test_hydrate_*`)
```
test_hydrate_constant_passthrough — plain values unchanged
test_hydrate_nested_dicts — recursive into dicts
test_hydrate_nested_lists — recursive into lists
test_hydrate_eval_tag — !_ becomes Evaluable placeholder
test_hydrate_quote_tag — !quote becomes Quoted placeholder
test_hydrate_ref_tag — !ref becomes Reference placeholder
test_hydrate_include_reads_file — !include replaced with file contents
test_hydrate_include_markdown — .md file as string
test_hydrate_include_yaml — .yaml file parsed and merged
test_hydrate_type_instantiation — dict with _type becomes class instance
test_hydrate_typed_key_syntax — "scenario.name:" becomes Scenario
test_hydrate_nested_types — types inside types
test_hydrate_mixed — types, tags, and constants together
```

### [ ] Dehydration Tests (`test_dehydrate_*`)
```
test_dehydrate_evaluable — Evaluable → {"!_": source}
test_dehydrate_quoted — Quoted → {"!quote": source}
test_dehydrate_reference — Reference → {"!ref": name}
test_dehydrate_typed_object — instance → dict with _type
test_dehydrate_nested — recursive structures
test_dehydrate_roundtrip — dehydrate(hydrate(x)) ≈ x
test_dehydrate_roundtrip_complex — complex nested structure
```

### [ ] Eval Basic Tests (`test_eval_*`)
```
test_eval_constant_int — 42 → 42
test_eval_constant_float — 3.14 → 3.14
test_eval_constant_string — "hello" → "hello"
test_eval_constant_bool — True → True
test_eval_constant_none — None → None
test_eval_constant_dict — plain dict unchanged
test_eval_constant_list — plain list unchanged
test_eval_nested_constants — nested dicts/lists unchanged
```

### [ ] Eval Expression Tests (`test_eval_expr_*`)
```
test_eval_expr_arithmetic — !_ 2 + 3 → 5
test_eval_expr_multiply — !_ 6 * 7 → 42
test_eval_expr_divide — !_ 10 / 4 → 2.5
test_eval_expr_complex — !_ (a + b) * c with bindings
test_eval_expr_builtin_min — !_ min(3, 1, 2) → 1
test_eval_expr_builtin_max — !_ max(3, 1, 2) → 3
test_eval_expr_builtin_abs — !_ abs(-5) → 5
test_eval_expr_builtin_round — !_ round(3.7) → 4
test_eval_expr_conditional — !_ x if cond else y
test_eval_expr_list_comprehension — !_ [x*2 for x in items]
test_eval_expr_uses_bindings — variables from ctx.bindings
test_eval_expr_binding_override — child binding shadows parent
```

### [ ] Eval Quote Tests (`test_eval_quote_*`)
```
test_eval_quote_simple — !quote k * S → "k * S"
test_eval_quote_complex — !quote Vmax * S / (Km + S) → preserved
test_eval_quote_not_evaluated — variables in quote not resolved
test_eval_quote_in_dict — quote inside dict structure
test_eval_quote_in_list — quote inside list
```

### [ ] Eval Reference Tests (`test_eval_ref_*`)
```
test_eval_ref_simple — !ref foo resolves to ctx.bindings["foo"]
test_eval_ref_nested_value — ref to dict, get whole dict
test_eval_ref_missing_strict — raises error in strict mode
test_eval_ref_missing_nonstrict — returns Reference in non-strict
test_eval_ref_in_expression — !_ x + !ref offset (if supported)
```

### [ ] Function Tests (`test_function_*`)
```
test_function_decorator_registers — @function adds to registry
test_function_ctx_injection — ctx auto-injected as last param
test_function_normal_distribution — normal(50, 10) returns float
test_function_uniform_distribution — uniform(0, 1) in range
test_function_discrete_choice — discrete([0.5, 0.5], "a", "b")
test_function_choice — choice("a", "b", "c") picks one
test_function_uses_ctx_rng — function uses ctx.rng
test_function_in_expression — !_ normal(50, 10) works
test_function_with_bindings — !_ normal(mu, sigma) with bound vars
```

### [ ] Context Tests (`test_context_*`)
```
test_context_rng_seeded — same seed → same results
test_context_rng_different_seeds — different seeds → different results
test_context_bindings_lookup — bindings accessible
test_context_bindings_missing — KeyError for missing binding
test_context_functions_available — registered functions in namespace
test_context_path_tracking — path updated during traversal
test_context_path_in_errors — error messages include path
test_context_child_shadows_parent — child bindings override
test_context_child_inherits_parent — child sees parent bindings
```

### [ ] Multiple Instantiation Tests (`test_instantiation_*`)
```
test_instantiation_same_seed_same_result — reproducible
test_instantiation_different_seeds — different random values
test_instantiation_spec_unchanged — original spec not mutated
test_instantiation_10_seeds — loop with 10 different seeds
test_instantiation_quotes_preserved — !quote survives all evals
```

### [ ] Lexical Scoping Tests (`test_scope_*`)
```
test_scope_top_level_constants — constants at module level
test_scope_scenario_inherits_module — scenario sees module constants
test_scope_nested_scenario — nested scenarios inherit
test_scope_extends_keyword — extends: wires up parent
test_scope_override — child value overrides parent
test_scope_null_removes — key: ~ removes inherited value
test_scope_deep_chain — A extends B extends C extends D
```

### [ ] Error Handling Tests (`test_error_*`)
```
test_error_undefined_variable — clear error for missing var
test_error_syntax_in_expression — invalid Python syntax
test_error_division_by_zero — runtime error in expression
test_error_unknown_function — function not registered
test_error_include_file_not_found — missing include file
test_error_circular_reference — A refs B refs A
test_error_message_includes_path — error shows location
```

### [ ] Edge Cases (`test_edge_*`)
```
test_edge_empty_dict — {} → {}
test_edge_empty_list — [] → []
test_edge_deeply_nested — 10 levels deep
test_edge_large_structure — 1000 keys
test_edge_unicode_in_expression — !_ "héllo" * 2
test_edge_multiline_expression — expression with newlines
test_edge_expression_returns_dict — !_ {"a": 1}
test_edge_expression_returns_list — !_ [1, 2, 3]
```

### .

## [ ] M1.8b-tests - Simulator Test Suite

Tests for simulator creation and rate expression compilation.

### [ ] Rate Compilation Tests (`test_rate_*`)
```
test_rate_simple_constant — rate: !quote 0.5 → constant rate
test_rate_mass_action — rate: !quote k * S1 * S2
test_rate_michaelis_menten — rate: !quote Vmax * S / (Km + S)
test_rate_hill — rate: !quote Vmax * S^n / (K^n + S^n)
test_rate_uses_constants — constants baked into rate function
test_rate_substrate_variables — S, S1, S2 bound correctly
test_rate_product_variables — P, P1, P2 if needed
```

### [ ] Simulator Creation Tests (`test_sim_*`)
```
test_sim_creates_from_scenario — Bio.sim(scenario) works
test_sim_compiles_rates — rate expressions become callable
test_sim_initial_state — initial concentrations set
test_sim_step_advances — step() changes state
test_sim_run_multiple — run(steps=100) returns history
test_sim_action_available — sim.action() callable
test_sim_measure_available — sim.measure() callable
```

### [ ] Simulation Correctness Tests (`test_simulation_*`)
```
test_simulation_conservation — mass conserved in reactions
test_simulation_equilibrium — reaches steady state
test_simulation_perturbation — responds to feedstock
test_simulation_reproducible — same seed same trajectory
test_simulation_different_seeds — different trajectories
```

### .

## [x] M1.9 - Architecture Cleanup

Improve CLI extensibility and hydration patterns.

**Status**: Complete — CLI commands folder, Entity.hydrate() pattern, report command.

### [x] CLI commands folder pattern
- Create `commands/` folder for CLI subcommands
- Main `bio` CLI does argument parsing, dispatches to command modules
- Each command in separate file: `commands/run.py`, `commands/report.py`, etc.
- Easy to extend without modifying main CLI
- **Future**: Auto-discover commands by scanning folder; each command file defines `HELP` one-liner that global help aggregates

### [x] Entity.hydrate() pattern
- Add `hydrate(data: dict) -> Self` class method to Entity base class
- Each biotype class implements its own hydration logic
- ChemistryImpl.hydrate() recursively hydrates molecules, reactions
- Move `_build_chemistry_from_dict` logic into class-based hydration
- Typed structure (Chemistry, Molecules, Reactions) is simulator-independent
- Simulators receive fully hydrated typed objects

### [x] Report command with CSV output
- `bio report` command creates and opens CSV report (default command)
- `bio run` for debug output (prints result dict)
- Final state, scores, and verification results in report
- **Future**: Excel output with timeline data, charts

### .

## [x] M1.10 - Verification

Run the hardcoded job from CLI and verify results.

**Status**: Complete — CLI runs job, shows results, 342 tests passing.

### [x] Run from CLI: `bio src/alienbio/catalog/jobs/hardcoded_test`
### [x] Assert concentrations change as expected (A, B depleted; C, D increased)
### [x] Verify `scoring:` functions return expected values
### [x] Verify `verify:` assertions pass
### [x] CLI output shows pass/fail status and scores
### [x] Test: job completes with all verifications passing from CLI

### .


# Milestone 2 - KEGG Data Integration

**Concept**: Extract statistical distributions from KEGG biochemistry to serve as templates for synthetic biology generation. Capture molecule properties (atom counts, functional groups, molecular weight), reaction patterns (reactant/product counts, delta-depth), and connectivity (in/out degree per biosynthetic depth layer).

## [ ] M2.1 - KEGG Parsing
### [ ] Parse compound flat files into BioMolecule instances
### [ ] Parse reaction flat files into BioReaction instances
### [ ] Build molecule registry and reaction registry
### [ ] Test: parse sample KEGG files, assert expected molecule/reaction counts

### .

## [ ] M2.2 - DAG Construction
### [ ] Compute bdepth for each molecule
### [ ] Link molecules via produced_by/consumed_by
### [ ] Validate DAG connectivity
### [ ] Test: bdepth=0 molecules have no produced_by, all others have at least one

### .

## [ ] M2.3 - Statistical Extraction
### [ ] Count molecules per bdepth layer
### [ ] Compute in-degree/out-degree histograms
### [ ] Compute reaction template distributions (n_reactants, n_products)
### [ ] Extract delta-depth distributions
### [ ] Test: histograms sum to total molecule/reaction counts

### .

## [ ] M2.4 - BioChemistryModel
### [ ] Populate MoleculeLayerStats from KEGG data
### [ ] Populate ReactionClassStats (anabolic, catabolic, energy)
### [ ] Serialize model to JSON for inspection
### [ ] Test: round-trip serialize/deserialize, assert equality

### .


# Milestone 3 - Synthetic Molecule Generation

**Concept**: Generate alien molecules bottom-up. Start with primitives (bdepth=0) from alien atoms, then build higher-depth molecules via synthetic reactions. Generate alien nomenclature via markov or diffusion model.

## [ ] M3.1 - Molecule Generator Protocol
### [ ] Define interface for generating molecules from stats
### [ ] Accept layer, count, degree distributions as parameters
### [ ] Test: protocol definition passes type check

### .

## [ ] M3.2 - Primitive Molecules
### [ ] Generate bdepth=0 molecules with random names
### [ ] Assign synthetic properties (placeholder atoms)
### [ ] Test: generate 10 primitives, all have bdepth=0 and unique names

### .

## [ ] M3.3 - Layered Generation
### [ ] Generate molecules layer by layer using captured distributions
### [ ] Sample in-degree from histogram to determine how many reactions produce each
### [ ] Build produced_by/consumed_by links as placeholders
### [ ] Test: generate 5 layers, molecule counts per layer match requested distribution

### .

## [ ] M3.4 - Verification
### [ ] Compare synthetic molecule stats to KEGG stats
### [ ] Assert distributions match within tolerance
### [ ] Test: KS-test on degree distributions, p > 0.05

### .


# Milestone 4 - Synthetic Reaction Generation

**Concept**: Generate alien biochemistry with anabolic (build-up), catabolic (breakdown), and energy reactions. Assign rate functions from parametric templates. Create closed-loop energy carriers (alien ATP/NADH analogs).

## [ ] M4.1 - Reaction Generator Protocol
### [ ] Define interface for generating reactions linking molecules
### [ ] Accept template distribution, depth deltas as parameters
### [ ] Test: protocol definition passes type check

### .

## [ ] M4.2 - Basic Reactions
### [ ] Sample (n_reactants, n_products) from template distribution
### [ ] Select reactants from layer L, products from layer L+1
### [ ] Create BioReaction instances
### [ ] Test: generate 20 reactions, all have valid reactant/product references

### .

## [ ] M4.3 - Rate Functions
### [ ] Define parametric rate function templates
### [ ] Assign random parameters within plausible ranges
### [ ] Attach rate functions to reactions
### [ ] Add catalyst coefficient (0) — required-but-not-consumed in reactions
### [ ] Test: call rate function with sample state, returns positive float

### .

## [ ] M4.4 - Verification
### [ ] Generate small synthetic chemistry (50 molecules, 30 reactions)
### [ ] Run simulation, confirm no runaway concentrations
### [ ] Compare reaction stats to KEGG stats
### [ ] Test: after 1000 steps, all concentrations remain in [0, 1e6] range

### .


# Milestone 5 - Single Compartment Simulation

## [ ] M5.1 - BioSystem Assembly
### [ ] Combine synthetic molecules and reactions into BioSystem
### [ ] Initialize state with random concentrations
### [ ] Test: BioSystem has correct molecule and reaction counts

### .

## [ ] M5.2 - Equilibrium Testing
### [ ] Run simulation until concentrations stabilize
### [ ] Identify and fix unstable reaction rates
### [ ] Define homeostasis targets
### [ ] Test: variance of concentrations over last 100 steps < threshold

### .

## [ ] M5.3 - Perturbation Testing
### [ ] Inject concentration spike, observe recovery
### [ ] Remove a reaction, observe divergence
### [ ] Validate system responds to interventions
### [ ] Test: spike recovery within 50 steps; removal causes measurable drift

### .


# Milestone 6 - Measurements and Actions

**Concept**: Agent-facing API for observations and actions. Measurements observe limited aspects of system state. Actions perturb system state.

## [ ] M6.1 - Measurement Protocol
### [ ] Define interface: measure(state, params) → value
### [ ] Implement concentration measurement
### [ ] Implement rate measurement
### [ ] Test: measure known state, assert correct concentration returned

### .

## [ ] M6.2 - Action Protocol
### [ ] Define interface: act(state, params) → new_state
### [ ] Implement add_molecule action
### [ ] Implement adjust_rate action
### [ ] Test: add_molecule increases concentration by expected amount

### .

## [ ] M6.3 - Agent Interface
### [ ] Bundle measurements and actions into agent-facing API
### [ ] Define text descriptions for each measurement/action
### [ ] Test agent can query and act via API
### [ ] Test: call API methods, assert valid responses and state changes

### .


# Milestone 7 - Task Framework

**Concept**: Tasks specify goals with scoring criteria. Types include predict (forecast behavior), diagnose (identify disease), and cure (restore health).

## [ ] M7.1 - Task Protocol
### [ ] Define Task with setup, goal, scoring, criteria
### [ ] Implement predict task: forecast concentration after N steps
### [ ] Test: protocol definition passes type check

### .

## [ ] M7.2 - Experiment Protocol
### [ ] Define Experiment combining world, task, agent
### [ ] Implement run_experiment returning score
### [ ] Test: run experiment with mock agent, returns numeric score

### .

## [ ] M7.3 - Hardcoded Predict Task
### [ ] Create simple system with known dynamics
### [ ] Define prediction task with ground truth
### [ ] Score agent's prediction accuracy
### [ ] Test: perfect prediction scores 1.0, random prediction scores < 0.5

### .


# Milestone 8 - Multi-Compartment Systems

**Concept**: Organism construction with compartmentalized systems. Generate organs as DAGs of bioparts with transport reactions between compartments. Build nested hierarchy from organelles to cells to organs to organism. Establish homeostasis via feedback loops maintaining concentration targets.

## [ ] M8.1 - Compartment Model
### [ ] Extend State to support multiple compartments
### [ ] Define transport reactions moving molecules between compartments
### [ ] Implement container hierarchy — ecosystems > regions > organisms > compartments > organelles
### [ ] Implement outflow/inflow system — outflows define transport; inflows are implied
### [ ] Test: state with 2 compartments, each has independent concentrations

### .

## [ ] M8.2 - Organ Generator
### [ ] Generate compartments with local reactions
### [ ] Generate transport reactions between compartments
### [ ] Assemble into BioOrganism
### [ ] Implement template instantiation — `contains: [{template: Krel, count: 80}]`
### [ ] Test: generated organism has expected compartment count and transport links

### .

## [ ] M8.3 - Cross-Compartment Simulation
### [ ] Extend Simulator to handle multi-compartment state
### [ ] Verify transport moves molecules correctly
### [ ] Test homeostasis across compartments
### [ ] Add maintained molecules — enzymes kept at constant concentration in organisms
### [ ] Add operating envelope — survival ranges for pH, temp, molecule concentrations
### [ ] Add reproduction threshold — molecule levels required for reproduction
### [ ] Add predation mechanics — species predation on other species
### [ ] Test: molecule injected in compartment A appears in compartment B after N steps

### .


# Milestone 9 - Disease and Variation

**Concept**: Define healthy baseline (equilibria and acceptable ranges). Generate perturbations: mutations (altered rates, removed reactions) and deficiencies (reduced enzyme concentrations). Diseases produce measurable symptoms as deviations from baseline.

## [ ] M9.1 - Baseline Definition
### [ ] Define healthy steady-state for organism
### [ ] Define acceptable ranges for each homeostatic target
### [ ] Test: healthy organism stays within ranges for 1000 steps

### .

## [ ] M9.2 - Perturbation Generator
### [ ] Generate mutations: altered rate, removed reaction
### [ ] Generate deficiencies: reduced enzyme concentration
### [ ] Apply perturbation to create diseased organism
### [ ] Test: perturbation changes at least one reaction rate or removes reaction

### .

## [ ] M9.3 - Symptom Measurement
### [ ] Define symptom as deviation from baseline
### [ ] Implement symptom detection measurements
### [ ] Verify diseases produce observable symptoms
### [ ] Test: diseased organism has at least one measurement outside healthy range

### .


# Milestone 10 - Diagnosis and Cure Tasks

**Concept**: Diagnosis tasks provide diseased organism with limited measurements; agent identifies which perturbation was applied. Cure tasks provide action toolkit; agent restores homeostatic targets to baseline. Difficulty scales with number of possible diseases and measurement limitations.

## [ ] M10.1 - Diagnosis Task
### [ ] Provide diseased organism with limited measurements
### [ ] Agent must identify which perturbation was applied
### [ ] Score based on correct identification
### [ ] Test: oracle agent with full info scores 1.0

### .

## [ ] M10.2 - Cure Task
### [ ] Provide diseased organism with action toolkit
### [ ] Agent must restore homeostatic targets to baseline
### [ ] Score based on recovery quality
### [ ] Test: applying correct cure returns organism to healthy range

### .

## [ ] M10.3 - Task Difficulty Scaling
### [ ] Parameterize number of possible diseases
### [ ] Parameterize measurement limitations
### [ ] Generate tasks at varying difficulty levels
### [ ] Test: difficulty=1 task easier than difficulty=5 (measured by oracle performance)

### .


# Milestone 11 - Test Harness

## [ ] M11.1 - Test Definition
### [ ] Define Test as batch of experiments
### [ ] Support parameter sweeps (worlds, agents, settings)
### [ ] Test: create Test with 10 experiments, assert count matches

### .

## [ ] M11.2 - Execution Runner
### [ ] Run experiments in sequence or parallel
### [ ] Log all agent actions and measurements
### [ ] Aggregate scores across experiments
### [ ] Test: run batch of 5 experiments, receive 5 scores

### .

## [ ] M11.3 - Result Analysis
### [ ] Compute pass/fail rates per difficulty level
### [ ] Plot performance curves
### [ ] Export results for comparison
### [ ] Test: export to JSON, re-import, assert data integrity

### .


# Milestone 12 - JAX Simulator

GPU-accelerated simulator using JAX/XLA compilation.

## [ ] M12.1 - JAX Core
### [ ] Implement JaxWorldSimulator with same API as Python WorldSimulator
### [ ] Use jax.numpy for state arrays
### [ ] Apply @jax.jit to step() hot path
### [ ] Test: JAX unit tests pass for step() and run()

### .

## [ ] M12.2 - Rate Function Compilation
### [ ] Decorated @rate functions traced and compiled by JAX
### [ ] Verify pure functional rate functions work with jit
### [ ] Handle non-jittable fallback gracefully
### [ ] Test: mass_action rate compiles and runs on GPU

### .

## [ ] M12.3 - Verification
### [ ] Run identical simulations on both simulators (Python reference, JAX)
### [ ] Assert outputs match within floating-point tolerance
### [ ] Benchmark performance difference
### [ ] Test: max difference between Python and JAX outputs < 1e-6

### .


# Milestone 13 - Alien Naming and Skinning

## [ ] M13.1 - Name Generator
### [ ] Build markov model from biological nomenclature
### [ ] Generate plausible alien names for molecules, reactions
### [ ] Test: generate 100 names, all unique, all pronounceable (no triple consonants)

### .

## [ ] M13.2 - Description Generator
### [ ] Generate natural language descriptions of bioparts
### [ ] Vary detail level (minimal hints to full explanation)
### [ ] Test: generate descriptions at 3 detail levels, length increases with detail

### .

## [ ] M13.3 - Task Skinning
### [ ] Apply naming and descriptions to generated tasks
### [ ] Produce agent-facing task text with alien terminology
### [ ] Test: skinned task contains no Earth biology terms

### .


# Milestone 14 - End-to-End Validation

**Concept**: Validate full pipeline, calibrate against KEGG ground truth, and iterate based on agent testing. Tune generators to achieve target difficulty range based on AI performance curves.

## [ ] M14.1 - Full Pipeline Test
### [ ] Generate alien biology from scratch
### [ ] Generate organism with disease
### [ ] Generate diagnosis and cure tasks
### [ ] Run LLM agent through tasks
### [ ] Score and analyze results
### [ ] Test: pipeline completes without error, produces valid scores

### .

## [ ] M14.2 - Difficulty Calibration
### [ ] Sweep complexity parameters
### [ ] Identify where agent performance degrades
### [ ] Tune generators to target difficulty range
### [ ] Test: performance curve shows expected degradation with difficulty

### .

## [ ] M14.3 - Documentation
### [ ] Document generated biology format
### [ ] Document task specification format
### [ ] Document agent interface API
### [ ] Test: all public functions have docstrings, sphinx builds without warnings

### .


# Later

Features to consider for future development.

## [ ] Cloud Storage Integration
- Add cloud sync for DAT storage (Google Cloud Storage, S3)
- Implement `Dat.push()` / `Dat.pull()` methods
- Or integrate with DVC for remote storage
- Config already has `default_remote` and `remote_prefix` fields

## [ ] Web Dashboard
- Real-time experiment monitoring
- Result visualization and comparison
- Agent performance analytics

## [ ] Multi-agent Experiments
- Run multiple agents on same scenarios
- Comparative scoring and analysis
- Tournament-style evaluation

## [ ] Quiescence Detection
- `sim.run(quiet=("measure_name", args...), delta=10, span=50)` - run until measure stabilizes
- Stop when measure changes by less than `delta` over `span` consecutive ticks
- Requires `timeout` parameter for safety (prevents infinite runs)
- Use case: "Run until the ecosystem stabilizes, then sample"
