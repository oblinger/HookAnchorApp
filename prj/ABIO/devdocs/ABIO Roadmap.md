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

## [x] M1.8 - Spec Evaluation Implementation

Implement the spec evaluation system per [[Spec Evaluation]] specification. Uses Python expression strings (not Expr trees — see [[Expr]] for deferred design).

**Status**: Complete — All subtasks done. Tag semantics updated: `!_` preserves expressions (Quoted), `!ev` evaluates at instantiation (Evaluable).

**Reference Docs**: [[Spec Evaluation]], [[Spec Language]]

**Design Summary**:
- `!_` tag → preserve expression unchanged (Quoted) — for rate equations, lambdas
- `!ev` tag → evaluate Python expression at instantiation (Evaluable) — for computed values
- `!quote` tag → alias for `!_` (preserve expression)
- `!ref` tag → lookup named value from bindings
- `!include` tag → read file contents (resolved during hydration)
- Hydration = type instantiation + tag→placeholder conversion
- Context carries `rng`, `bindings`, `functions`, `path`

### [x] M1.8a - Write Comprehensive Tests First

Create test suite BEFORE implementation. Tests serve as executable specification.

**Test files**:
- `tests/unit/test_spec_eval.py` — 125 tests for hydrate/dehydrate/eval (117 skipped)
- `tests/unit/test_rate_compilation.py` — 45 tests for rate compilation (45 skipped)

See detailed test cases in M1.8a-tests section below.

### [x] M1.8b - Placeholder Classes
- [x] Implement `Evaluable(source: str)` — placeholder for `!ev` expressions
- [x] Implement `Quoted(source: str)` — placeholder for `!_` and `!quote` expressions
- [x] Implement `Reference(name: str)` — placeholder for `!ref` expressions
- [x] All placeholders are simple dataclasses with `source`/`name` attribute
- [x] YAML constructors for `!_`, `!quote`, and `!ev` tags registered

### [x] M1.8c - Hydrate Implementation
- [x] Implement `hydrate(data)` — recursive transformation
- [ ] Type instantiation: dicts with `_type` field → Python class instances (deferred)
- [x] Tag conversion: `!_` → Quoted, `!ev` → Evaluable, `!ref` → Reference
- [x] `!include` resolution: read file, insert contents (during hydration)
- [x] Recursive descent into dicts and lists
- [x] Legacy tag support: EvTag, RefTag, IncludeTag converted to new placeholders

### [x] M1.8d - Dehydrate Implementation
- [x] Implement `dehydrate(data)` — reverse of hydrate
- [ ] Python instances → dicts with `_type` field (deferred with type instantiation)
- [x] Evaluable → `{"!ev": source}`, Quoted → `{"!_": source}`, Reference → `{"!ref": name}`
- [x] Round-trip property: `dehydrate(hydrate(x))` ≈ `x`

### [x] M1.8e - Context Object
- [x] Implement Context class with `rng`, `bindings`, `functions`, `path`
- [x] `rng`: seeded numpy RNG for reproducibility
- [x] `bindings`: dict of variable name → value
- [x] `functions`: dict of registered @function handlers
- [x] `path`: list of keys for error messages (e.g., `["scenario", "molecules", "count"]`)
- [x] Context nesting: child context can shadow parent bindings

### [x] M1.8f - Eval Implementation
- [x] Implement `eval_node(node, ctx)` — in eval.py
- [x] Constants (str, int, float, bool, None) → return as-is
- [x] Evaluable (`!ev`) → Python `eval(source, safe_builtins, namespace)`
- [x] Quoted (`!_`) → return `source` string unchanged
- [x] Reference (`!ref`) → lookup in `ctx.bindings`, error if missing
- [x] dict → recursively eval values
- [x] list → recursively eval elements
- [ ] Typed objects → eval their evaluable fields (deferred)

### [x] M1.8g - Function Auto-Injection
- [x] Auto-inject `ctx` parameter when functions called from eval
- [x] User writes `!ev normal(50, 10)`, evaluator calls `normal(50, 10, ctx=ctx)`
- [x] Function registry accessible via `ctx.functions`
- [ ] Implement `@function` decorator for global registration (deferred)

### [x] M1.8h - Built-in Functions
- [x] Distribution functions: `normal`, `uniform`, `lognormal`, `poisson`, `exponential`
- [x] Choice functions: `discrete(weights, *choices)`, `choice(*choices)`
- [x] All use `ctx.rng` for reproducibility
- [x] Add `DEFAULT_FUNCTIONS` registry with all built-ins
- [x] Add `make_context()` helper for easy context creation

### [x] M1.8i - Default Namespace
- [x] Define `SAFE_BUILTINS` set: `min`, `max`, `abs`, `round`, `sum`, `len`, etc.
- [x] Evaluation namespace = `SAFE_BUILTINS` + `ctx.bindings` + `ctx.functions`
- [x] No dangerous builtins (`exec`, `eval`, `import`, `open`, etc.)

### [x] M1.8j - Integration
- [x] Wire hydrate/eval into `Bio.load()` flow
- [x] `Bio.load_spec()` returns hydrated but unevaluated spec
- [x] `Bio.eval_spec()` called separately (allows multiple instantiations)
- [x] Rate expressions (`!_` / Quoted) survive through to Scenario object
- [x] `run.py` calls `eval_node()` to evaluate scenario before simulation

### .

## [x] M1.8a-tests - Spec Evaluation Test Suite

Comprehensive tests for spec evaluation. **149 tests implemented.**

### [x] Hydration Tests (`test_hydrate_*`)
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

### [x] Dehydration Tests (`test_dehydrate_*`)
```
test_dehydrate_evaluable — Evaluable → {"!_": source}
test_dehydrate_quoted — Quoted → {"!quote": source}
test_dehydrate_reference — Reference → {"!ref": name}
test_dehydrate_typed_object — instance → dict with _type
test_dehydrate_nested — recursive structures
test_dehydrate_roundtrip — dehydrate(hydrate(x)) ≈ x
test_dehydrate_roundtrip_complex — complex nested structure
```

### [x] Eval Basic Tests (`test_eval_*`)
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

### [x] Eval Expression Tests (`test_eval_expr_*`)
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

### [x] Eval Quote Tests (`test_eval_quote_*`)
```
test_eval_quote_simple — !quote k * S → "k * S"
test_eval_quote_complex — !quote Vmax * S / (Km + S) → preserved
test_eval_quote_not_evaluated — variables in quote not resolved
test_eval_quote_in_dict — quote inside dict structure
test_eval_quote_in_list — quote inside list
```

### [x] Eval Reference Tests (`test_eval_ref_*`)
```
test_eval_ref_simple — !ref foo resolves to ctx.bindings["foo"]
test_eval_ref_nested_value — ref to dict, get whole dict
test_eval_ref_missing_strict — raises error in strict mode
test_eval_ref_missing_nonstrict — returns Reference in non-strict
test_eval_ref_in_expression — !_ x + !ref offset (if supported)
```

### [x] Function Tests (`test_function_*`)
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

### [x] Context Tests (`test_context_*`)
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

### [x] Multiple Instantiation Tests (`test_instantiation_*`)
```
test_instantiation_same_seed_same_result — reproducible
test_instantiation_different_seeds — different random values
test_instantiation_spec_unchanged — original spec not mutated
test_instantiation_10_seeds — loop with 10 different seeds
test_instantiation_quotes_preserved — !quote survives all evals
```

### [x] Lexical Scoping Tests (`test_scope_*`)
```
test_scope_top_level_constants — constants at module level
test_scope_scenario_inherits_module — scenario sees module constants
test_scope_nested_scenario — nested scenarios inherit
test_scope_extends_keyword — extends: wires up parent
test_scope_override — child value overrides parent
test_scope_null_removes — key: ~ removes inherited value
test_scope_deep_chain — A extends B extends C extends D
```

### [x] Error Handling Tests (`test_error_*`)
```
test_error_undefined_variable — clear error for missing var
test_error_syntax_in_expression — invalid Python syntax
test_error_division_by_zero — runtime error in expression
test_error_unknown_function — function not registered
test_error_include_file_not_found — missing include file
test_error_circular_reference — A refs B refs A
test_error_message_includes_path — error shows location
```

### [x] Edge Cases (`test_edge_*`)
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

## [x] M1.8b-tests - Simulator Test Suite

Tests for simulator creation and rate expression compilation. **34 tests in test_rate_compilation.py**

### [x] Rate Compilation Tests (`test_rate_*`)
```
test_rate_simple_constant — rate: !quote 0.5 → constant rate
test_rate_mass_action — rate: !quote k * S1 * S2
test_rate_michaelis_menten — rate: !quote Vmax * S / (Km + S)
test_rate_hill — rate: !quote Vmax * S^n / (K^n + S^n)
test_rate_uses_constants — constants baked into rate function
test_rate_substrate_variables — S, S1, S2 bound correctly
test_rate_product_variables — P, P1, P2 if needed
```

### [x] Simulator Creation Tests (`test_sim_*`)
```
test_sim_creates_from_scenario — Bio.sim(scenario) works
test_sim_compiles_rates — rate expressions become callable
test_sim_initial_state — initial concentrations set
test_sim_step_advances — step() changes state
test_sim_run_multiple — run(steps=100) returns history
test_sim_action_available — sim.action() callable
test_sim_measure_available — sim.measure() callable
```

### [x] Simulation Correctness Tests (`test_simulation_*`)
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


# Milestone 2 - Generator System

**Concept**: Template-based scenario generation with parameterized templates, distribution sampling, constraint guards, and visibility mapping. See [[Generator Spec Language]] for YAML syntax.

## [ ] M2.1 - Test Specifications

Detailed test-first specifications for generator components.

### Philosophy
- **Test-driven**: Each milestone starts with tests that define expected behavior
- **Incremental**: Each phase produces working, tested code
- **Templates all the way down**: Even "background" uses templates

---

## Phase G1: Template Representation & Parsing

### G1.1 - Template Data Structures

Tests first:
```python
def test_template_has_params():
    t = Template.parse({"_params_": {"rate": 0.1}, "molecules": {...}})
    assert t.params["rate"] == 0.1

def test_template_has_ports():
    t = Template.parse({"_ports_": {"reactions.work": "energy.out"}})
    assert t.ports["reactions.work"].type == "energy"
    assert t.ports["reactions.work"].direction == "out"

def test_template_has_molecules_and_reactions():
    t = Template.parse({
        "molecules": {"M1": {"role": "energy"}, "M2": {"role": "energy"}},
        "reactions": {"r1": {"reactants": ["M1"], "products": ["M2"]}}
    })
    assert "M1" in t.molecules
    assert "r1" in t.reactions
```

Deliverables:
- `Template` class with params, molecules, reactions, ports
- `Port` class with type, direction, path
- Parse `template.name:` syntax from YAML

### G1.2 - Template Registry

Tests:
```python
def test_template_registration():
    registry = TemplateRegistry()
    registry.register("energy_cycle", template)
    assert registry.get("energy_cycle") is template

def test_template_from_yaml_file():
    registry = TemplateRegistry.from_directory("catalog/templates")
    assert "primitives/energy_cycle" in registry

def test_template_not_found():
    registry = TemplateRegistry()
    with pytest.raises(TemplateNotFoundError):
        registry.get("nonexistent")
```

Deliverables:
- `TemplateRegistry` class
- Load templates from YAML files
- Path-based lookup (`primitives/energy_cycle`)

---

## Phase G2: Template Expansion (Core)

### G2.1 - Single Template Instantiation

Tests:
```python
def test_expand_simple_template():
    template = Template.parse({
        "molecules": {"M1": {"role": "energy"}},
        "reactions": {"r1": {"reactants": ["M1"], "products": ["M2"]}}
    })
    expanded = expand(template, namespace="krel")
    assert "m.krel.M1" in expanded.molecules
    assert "r.krel.r1" in expanded.reactions

def test_expand_with_params():
    template = Template.parse({
        "_params_": {"rate": 0.1},
        "reactions": {"r1": {"rate": "!ref rate"}}
    })
    expanded = expand(template, namespace="krel", params={"rate": 0.5})
    assert expanded.reactions["r.krel.r1"]["rate"] == 0.5

def test_expand_resolves_refs():
    template = Template.parse({
        "_params_": {"k": 0.1},
        "reactions": {"r1": {"rate": "!ref k"}}
    })
    expanded = expand(template, namespace="x")
    assert expanded.reactions["r.x.r1"]["rate"] == 0.1  # Resolved, not "!ref k"
```

Deliverables:
- `expand()` function
- Namespace prefixing (`m.` for molecules, `r.` for reactions)
- Parameter substitution via `!ref`

### G2.2 - Nested Instantiation (`_instantiate_` / `_as_`)

Tests:
```python
def test_nested_instantiation():
    parent = Template.parse({
        "_instantiate_": {
            "_as_ energy": {"_template_": "energy_cycle", "rate": 0.2}
        }
    })
    expanded = expand(parent, namespace="krel", registry=registry)
    assert "m.krel.energy.ME1" in expanded.molecules
    assert "r.krel.energy.activation" in expanded.reactions

def test_replication():
    parent = Template.parse({
        "_instantiate_": {
            "_as_ chain{i in 1..3}": {"_template_": "anabolic_chain"}
        }
    })
    expanded = expand(parent, namespace="krel", registry=registry)
    assert "m.krel.chain1.MS1" in expanded.molecules
    assert "m.krel.chain2.MS1" in expanded.molecules
    assert "m.krel.chain3.MS1" in expanded.molecules
    assert "m.krel.chain.MS1" not in expanded.molecules  # No un-indexed version

def test_replication_indices_concatenate():
    # Indices concatenate without dots: chain1, not chain.1
    parent = Template.parse({
        "_instantiate_": {
            "_as_ p{i in 1..2}": {"_template_": "simple"}
        }
    })
    expanded = expand(parent, namespace="x", registry=registry)
    assert "m.x.p1.M1" in expanded.molecules  # p1, not p.1
    assert "m.x.p2.M1" in expanded.molecules
```

Deliverables:
- Parse `_instantiate_:` blocks
- Parse `_as_ name:` and `_as_ name{i in range}:` syntax
- Recursive template expansion
- Index concatenation (not dotted)

### G2.3 - Port Wiring

Tests:
```python
def test_port_declaration():
    template = Template.parse({
        "_ports_": {
            "reactions.work": "energy.out",
            "molecules.M1": "molecule.in"
        }
    })
    assert template.ports["reactions.work"].type == "energy"
    assert template.ports["reactions.work"].direction == "out"
    assert template.ports["molecules.M1"].direction == "in"

def test_port_connection_at_instantiation():
    parent = Template.parse({
        "_instantiate_": {
            "_as_ energy": {"_template_": "energy_cycle"},
            "_as_ chain": {
                "_template_": "anabolic_chain",
                "reactions.build": "energy.reactions.work"  # Port connection
            }
        }
    })
    expanded = expand(parent, namespace="krel", registry=registry)
    assert expanded.reactions["r.krel.chain.build"]["energy_source"] == "r.krel.energy.work"

def test_port_type_mismatch_error():
    # Connecting energy.out to molecule.in should fail
    parent = Template.parse({
        "_instantiate_": {
            "_as_ a": {"_template_": "has_energy_out"},
            "_as_ b": {
                "_template_": "has_molecule_in",
                "molecules.M1": "a.reactions.work"  # Type mismatch!
            }
        }
    })
    with pytest.raises(PortTypeMismatchError):
        expand(parent, namespace="x", registry=registry)
```

Deliverables:
- Port declaration parsing (`path: type.direction`)
- Port connection at instantiation
- Type checking (energy.out connects to energy.in)
- Resolved connections become `energy_source:` or similar fields

---

## Phase G3: Distribution Sampling

### G3.1 - Distribution Evaluation

Tests:
```python
def test_normal_sampling():
    ctx = Context(seed=42)
    result = eval_expr("normal(10, 2)", ctx)
    assert 5 < result < 15  # Roughly in range

def test_lognormal_positive():
    ctx = Context(seed=42)
    result = eval_expr("lognormal(0.1, 0.3)", ctx)
    assert result > 0  # Always positive

def test_discrete_choice():
    ctx = Context(seed=42)
    result = eval_expr("discrete([a, b, c], [0.5, 0.3, 0.2])", ctx)
    assert result in ["a", "b", "c"]

def test_uniform_choice():
    ctx = Context(seed=42)
    result = eval_expr("choice(red, green, blue)", ctx)
    assert result in ["red", "green", "blue"]

def test_same_seed_same_result():
    ctx1 = Context(seed=42)
    ctx2 = Context(seed=42)
    r1 = eval_expr("lognormal(0.1, 0.3)", ctx1)
    r2 = eval_expr("lognormal(0.1, 0.3)", ctx2)
    assert r1 == r2

def test_different_seed_different_result():
    ctx1 = Context(seed=42)
    ctx2 = Context(seed=43)
    r1 = eval_expr("lognormal(0.1, 0.3)", ctx1)
    r2 = eval_expr("lognormal(0.1, 0.3)", ctx2)
    assert r1 != r2
```

Deliverables:
- Seeded random context
- Distribution functions: `normal`, `lognormal`, `uniform`, `poisson`, `exponential`, `discrete`, `choice`

### G3.2 - Distribution in Templates

Tests:
```python
def test_param_with_distribution():
    template = Template.parse({
        "_params_": {"rate": "!ev lognormal(0.1, 0.3)"},
        "reactions": {"r1": {"rate": "!ref rate"}}
    })
    exp1 = expand(template, namespace="x", seed=42)
    exp2 = expand(template, namespace="x", seed=43)
    # Different seeds = different sampled values
    assert exp1.reactions["r.x.r1"]["rate"] != exp2.reactions["r.x.r1"]["rate"]

def test_ev_in_molecule():
    template = Template.parse({
        "molecules": {
            "M{i in 1..3}": {
                "role": "structural",
                "description": "!ev f'Molecule {i}'"
            }
        }
    })
    expanded = expand(template, namespace="x", seed=42)
    assert expanded.molecules["m.x.M1"]["description"] == "Molecule 1"
    assert expanded.molecules["m.x.M2"]["description"] == "Molecule 2"

def test_distribution_in_loop_range():
    template = Template.parse({
        "_params_": {"count": "!ev normal(3, 0.5)"},
        "_instantiate_": {
            "_as_ p{i in 1..count}": {"_template_": "simple"}
        }
    })
    # count sampled first, then used in range
    expanded = expand(template, namespace="x", seed=42)
    # Should have ~3 instances (rounded)
    assert 2 <= len([k for k in expanded.molecules if "p" in k]) <= 4
```

Deliverables:
- `!ev` expressions evaluated during expansion
- Distributions sampled with seeded RNG
- Loop ranges can use sampled values

---

## Phase G4: Guards

### G4.1 - Guard Infrastructure

Tests:
```python
def test_guard_decorator():
    @guard
    def my_guard(expanded, context):
        return True

    assert hasattr(my_guard, '_is_guard')
    assert my_guard._is_guard == True

def test_guard_passes():
    @guard
    def always_pass(expanded, context):
        return True

    result = run_guard(always_pass, expanded, context)
    assert result == True

def test_guard_violation():
    @guard
    def always_fail(expanded, context):
        raise GuardViolation("Nope", details={"reason": "test"})

    with pytest.raises(GuardViolation) as exc:
        run_guard(always_fail, expanded, context)
    assert "Nope" in str(exc.value)

def test_guard_context_has_scenario():
    @guard
    def check_context(expanded, context):
        assert context.scenario is not None
        assert context.namespace is not None
        assert context.seed is not None
        return True
```

Deliverables:
- `@guard` decorator
- `GuardViolation` exception with details
- `GuardContext` with scenario, namespace, seed, attempt

### G4.2 - Built-in Guards

Tests:
```python
def test_no_new_species_dependencies_passes():
    # Reaction within single species namespace - OK
    expanded = MockExpanded(reactions={
        "r.Krel.r1": {"reactants": ["m.Krel.M1"], "products": ["m.Krel.M2"]}
    })
    context = MockContext(scenario=scenario)
    assert no_new_species_dependencies(expanded, context) == True

def test_no_new_species_dependencies_fails():
    # Reaction linking two species - FAIL
    expanded = MockExpanded(reactions={
        "r.x.r1": {"reactants": ["m.Krel.M1", "m.Kova.M2"], "products": ["m.Krel.M3"]}
    })
    context = MockContext(scenario=scenario)
    with pytest.raises(GuardViolation) as exc:
        no_new_species_dependencies(expanded, context)
    assert "cross-species" in str(exc.value).lower()

def test_no_new_cycles_passes():
    # Linear pathway - OK
    expanded = MockExpanded(reactions={
        "r1": {"reactants": ["M1"], "products": ["M2"]},
        "r2": {"reactants": ["M2"], "products": ["M3"]},
    })
    assert no_new_cycles(expanded, context) == True

def test_no_new_cycles_fails():
    # Circular pathway - FAIL
    expanded = MockExpanded(reactions={
        "r1": {"reactants": ["M1"], "products": ["M2"]},
        "r2": {"reactants": ["M2"], "products": ["M1"]},  # Cycle!
    })
    with pytest.raises(GuardViolation) as exc:
        no_new_cycles(expanded, context)
    assert "cycle" in str(exc.value).lower()

def test_no_essential_passes():
    # Molecule not in any reproduction_threshold - OK
    expanded = MockExpanded(molecules={"m.bg.X1": {"role": "inert"}})
    context = MockContext(scenario=scenario_with_thresholds)
    assert no_essential(expanded, context) == True

def test_no_essential_fails():
    # Molecule referenced in reproduction_threshold - FAIL
    expanded = MockExpanded(molecules={"m.bg.X1": {"role": "inert"}})
    scenario = MockScenario(organisms={
        "Krel": {"reproduction_threshold": {"m.bg.X1": 5.0}}  # References new molecule!
    })
    context = MockContext(scenario=scenario)
    with pytest.raises(GuardViolation):
        no_essential(expanded, context)
```

Deliverables:
- `no_new_species_dependencies` guard
- `no_new_cycles` guard
- `no_essential` guard
- Helper: `get_species_from_path(mol_name)` → species or None

### G4.3 - Guard Modes (retry, prune, reject)

Tests:
```python
def test_reject_mode():
    # Default mode - fail immediately
    with pytest.raises(GuardViolation):
        expand_with_guards(template, guards=["always_fail"], mode="reject")

def test_retry_mode_succeeds():
    # Guard fails first 2 attempts, passes on 3rd
    attempts = []
    @guard
    def flaky(expanded, context):
        attempts.append(context.attempt)
        if context.attempt < 2:
            raise GuardViolation("Not yet")
        return True

    result = expand_with_guards(template, guards=[flaky], mode="retry", max_attempts=5, seed=42)
    assert result is not None
    assert len(attempts) == 3  # 0, 1, 2

def test_retry_mode_exhausted():
    # Guard always fails - exhaust attempts
    @guard
    def always_fail(expanded, context):
        raise GuardViolation("Nope")

    with pytest.raises(GuardViolation) as exc:
        expand_with_guards(template, guards=[always_fail], mode="retry", max_attempts=3)
    assert "max_attempts" in str(exc.value).lower() or "exhausted" in str(exc.value).lower()

def test_prune_mode():
    # Guard fails for some elements - prune them, keep rest
    @guard
    def no_big_molecules(expanded, context):
        violations = [m for m in expanded.molecules if "big" in m]
        if violations:
            raise GuardViolation("Too big", prune=violations)
        return True

    result = expand_with_guards(template_with_big_and_small, guards=[no_big_molecules], mode="prune")
    assert "m.x.small" in result.molecules
    assert "m.x.big" not in result.molecules  # Pruned
```

Deliverables:
- Guard mode configuration (`reject`, `retry`, `prune`)
- Retry with incrementing seed
- Prune removes violating elements

### G4.4 - Guards in YAML

Tests:
```python
def test_global_guards():
    spec = load_spec("""
        scenario_generator_spec:
          _guards_:
            - no_new_species_dependencies
            - no_new_cycles
          _instantiate_:
            _as_ x: {_template_: foo}
    """)
    assert "no_new_species_dependencies" in spec.guards
    assert "no_new_cycles" in spec.guards

def test_guard_with_params():
    spec = load_spec("""
        _guards_:
          - max_pathway_length: {max_length: 4}
    """)
    guard_config = spec.guards[0]
    assert guard_config["name"] == "max_pathway_length"
    assert guard_config["params"]["max_length"] == 4

def test_guard_with_mode():
    spec = load_spec("""
        _guards_:
          - name: no_new_cycles
            mode: retry
            max_attempts: 10
    """)
    guard_config = spec.guards[0]
    assert guard_config["mode"] == "retry"
    assert guard_config["max_attempts"] == 10
```

Deliverables:
- Parse `_guards_:` in YAML
- Guard parameter passing
- Guard mode configuration

---

## Phase G5: Visibility Mapping

### G5.1 - Opaque Name Generation

Tests:
```python
def test_generate_molecule_names():
    molecules = ["m.Krel.energy.ME1", "m.Krel.energy.ME2", "m.Kova.MB1"]
    mapping = generate_opaque_names(molecules, prefix="M", seed=42)
    assert mapping["m.Krel.energy.ME1"].startswith("M")
    assert mapping["m.Krel.energy.ME2"].startswith("M")
    # All unique
    assert len(set(mapping.values())) == len(mapping)

def test_generate_reaction_names():
    reactions = ["r.Krel.energy.work", "r.Kova.consume"]
    mapping = generate_opaque_names(reactions, prefix="RX", seed=42)
    assert mapping["r.Krel.energy.work"].startswith("RX")

def test_reproducible_mapping():
    molecules = ["m.A", "m.B", "m.C"]
    map1 = generate_opaque_names(molecules, seed=42)
    map2 = generate_opaque_names(molecules, seed=42)
    assert map1 == map2
```

Deliverables:
- `generate_opaque_names()` function
- Seeded for reproducibility
- Configurable prefix

### G5.2 - Visibility Fraction

Tests:
```python
def test_fraction_known():
    items = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j"]  # 10 items
    visible, hidden = apply_fraction_known(items, fraction=0.7, seed=42)
    assert len(visible) == 7
    assert len(hidden) == 3
    assert set(visible) | set(hidden) == set(items)

def test_fraction_zero_all_hidden():
    items = ["a", "b", "c"]
    visible, hidden = apply_fraction_known(items, fraction=0.0, seed=42)
    assert len(visible) == 0
    assert len(hidden) == 3

def test_fraction_one_all_visible():
    items = ["a", "b", "c"]
    visible, hidden = apply_fraction_known(items, fraction=1.0, seed=42)
    assert len(visible) == 3
    assert len(hidden) == 0
```

Deliverables:
- `apply_fraction_known()` function
- Seeded random selection

### G5.3 - Full Visibility Mapping

Tests:
```python
def test_visibility_mapping_structure():
    expanded = MockExpanded(
        molecules={"m.Krel.M1": {}, "m.Kova.M2": {}},
        reactions={"r.Krel.r1": {}, "r.Kova.r2": {}}
    )
    visibility_spec = {
        "molecules": {"fraction_known": 1.0},
        "reactions": {"fraction_known": 0.5},
        "dependencies": {"fraction_known": 0.0}
    }
    mapping = generate_visibility_mapping(expanded, visibility_spec, seed=42)

    assert "m.Krel.M1" in mapping  # Molecule mapping
    assert "_hidden_" in mapping   # Hidden elements list

def test_hidden_dependencies():
    expanded = MockExpanded(
        dependencies=[("Krel", "Kova", "waste_nutrient")]
    )
    visibility_spec = {"dependencies": {"fraction_known": 0.0}}
    mapping = generate_visibility_mapping(expanded, visibility_spec, seed=42)

    assert ("Krel", "Kova", "waste_nutrient") in mapping["_hidden_"]
```

Deliverables:
- `generate_visibility_mapping()` function
- Per-entity-type visibility
- `_hidden_` list in mapping

### G5.4 - Apply Visibility to Scenario

Tests:
```python
def test_apply_visibility_renames_molecules():
    scenario = MockScenario(molecules={"m.Krel.ME1": {"role": "energy"}})
    mapping = {"m.Krel.ME1": "ME1"}
    visible = apply_visibility(scenario, mapping)

    assert "ME1" in visible.molecules
    assert "m.Krel.ME1" not in visible.molecules
    assert visible.molecules["ME1"]["role"] == "energy"

def test_apply_visibility_updates_reactions():
    scenario = MockScenario(
        molecules={"m.Krel.M1": {}, "m.Krel.M2": {}},
        reactions={"r.Krel.r1": {"reactants": ["m.Krel.M1"], "products": ["m.Krel.M2"]}}
    )
    mapping = {"m.Krel.M1": "M1", "m.Krel.M2": "M2", "r.Krel.r1": "RX1"}
    visible = apply_visibility(scenario, mapping)

    assert visible.reactions["RX1"]["reactants"] == ["M1"]
    assert visible.reactions["RX1"]["products"] == ["M2"]

def test_apply_visibility_removes_hidden():
    scenario = MockScenario(
        reactions={"r.visible": {}, "r.hidden": {}}
    )
    mapping = {"r.visible": "RX1", "_hidden_": ["r.hidden"]}
    visible = apply_visibility(scenario, mapping)

    assert "RX1" in visible.reactions
    assert "r.hidden" not in visible.reactions
    assert len(visible.reactions) == 1
```

Deliverables:
- `apply_visibility()` function
- Rename molecules, reactions
- Update references in reactions
- Remove hidden elements

---

## Phase G6: Full Generator Pipeline

### G6.1 - Bio.generate() API

Tests:
```python
def test_bio_generate_basic():
    spec = Bio.fetch("scenarios/mutualism/hidden_dependency")
    scenario = Bio.generate(spec, seed=42)

    assert scenario is not None
    assert hasattr(scenario, 'molecules')
    assert hasattr(scenario, 'reactions')
    assert len(scenario.molecules) > 0
    assert len(scenario.reactions) > 0

def test_bio_generate_reproducible():
    spec = Bio.fetch("scenarios/mutualism/hidden_dependency")
    s1 = Bio.generate(spec, seed=42)
    s2 = Bio.generate(spec, seed=42)

    assert s1.molecules == s2.molecules
    assert s1.reactions == s2.reactions

def test_bio_generate_different_seeds():
    spec = Bio.fetch("scenarios/mutualism/hidden_dependency")
    s1 = Bio.generate(spec, seed=42)
    s2 = Bio.generate(spec, seed=43)

    # Should have different sampled values
    assert s1 != s2

def test_bio_generate_has_ground_truth():
    spec = Bio.fetch("scenarios/mutualism/hidden_dependency")
    scenario = Bio.generate(spec, seed=42)

    assert hasattr(scenario, '_ground_truth_')
    assert hasattr(scenario, '_visibility_mapping_')
    # Ground truth has internal names
    assert any("m.Krel" in k for k in scenario._ground_truth_.molecules)
    # Visible scenario has opaque names
    assert not any("m.Krel" in k for k in scenario.molecules)
```

Deliverables:
- `Bio.generate(spec, seed)` method
- Returns scenario with ground truth preserved
- Reproducible with same seed

### G6.2 - End-to-End Pipeline

Tests:
```python
def test_pipeline_template_to_scenario():
    # Define a simple template
    template_yaml = """
    template.simple:
      _params_:
        rate: !ev lognormal(0.1, 0.3)
      molecules:
        M1: {role: energy}
        M2: {role: energy}
      reactions:
        r1:
          reactants: [M1]
          products: [M2]
          rate: !ref rate
      _ports_:
        reactions.r1: energy.out
    """

    # Define a generator spec using it
    spec_yaml = """
    scenario_generator_spec:
      name: test
      _instantiate_:
        _as_ species1:
          _template_: simple
      visibility:
        molecules: {fraction_known: 1.0}
        reactions: {fraction_known: 1.0}
    """

    registry = TemplateRegistry()
    registry.register("simple", Template.parse(yaml.safe_load(template_yaml)["template.simple"]))

    spec = load_generator_spec(yaml.safe_load(spec_yaml))
    scenario = generate(spec, seed=42, registry=registry)

    # Check structure
    assert len(scenario.molecules) == 2
    assert len(scenario.reactions) == 1
    # Check values were sampled
    assert scenario.reactions[list(scenario.reactions.keys())[0]]["rate"] > 0

def test_pipeline_with_guards():
    spec_yaml = """
    scenario_generator_spec:
      _guards_:
        - no_new_cycles
      _instantiate_:
        _as_ bg{i in 1..5}:
          _template_: random_pathway
    """
    # Should complete without cycle violations (guards enforced)
    scenario = Bio.generate(spec, seed=42)
    assert scenario is not None

def test_pipeline_visibility_applied():
    spec_yaml = """
    scenario_generator_spec:
      _instantiate_:
        _as_ Krel:
          _template_: producer
      visibility:
        dependencies: {fraction_known: 0.0}
    """
    scenario = Bio.generate(spec, seed=42)

    # Visible scenario shouldn't reveal internal names
    for mol_name in scenario.molecules:
        assert "Krel" not in mol_name  # Opaque names only

    # Ground truth has internal names
    for mol_name in scenario._ground_truth_.molecules:
        assert "Krel" in mol_name or "m." in mol_name
```

Deliverables:
- Full pipeline integration
- Template loading → expansion → guards → visibility → scenario

### G6.3 - Error Handling & Debugging

Tests:
```python
def test_template_not_found_error():
    spec_yaml = """
    scenario_generator_spec:
      _instantiate_:
        _as_ x:
          _template_: nonexistent_template
    """
    with pytest.raises(TemplateNotFoundError) as exc:
        Bio.generate(spec, seed=42)
    assert "nonexistent_template" in str(exc.value)

def test_port_type_error_message():
    # Helpful error when port types don't match
    with pytest.raises(PortTypeMismatchError) as exc:
        Bio.generate(bad_wiring_spec, seed=42)
    assert "energy.out" in str(exc.value) or "molecule.in" in str(exc.value)

def test_guard_failure_includes_context():
    with pytest.raises(GuardViolation) as exc:
        Bio.generate(spec_that_violates_guard, seed=42)

    error = exc.value
    assert error.template is not None
    assert error.namespace is not None
    assert error.seed is not None
```

Deliverables:
- Clear error messages
- Context in exceptions
- Debugging helpers

---

### Generator Dependency Graph

```
G1 (Representation)
    │
    ▼
G2 (Expansion) ◄─── requires templates to expand
    │
    ▼
G3 (Distributions) ◄─── sampling during expansion
    │
    ▼
G4 (Guards) ◄─── validate after expansion
    │
    ▼
G5 (Visibility) ◄─── map names after guards pass
    │
    ▼
G6 (Pipeline) ◄─── wire it all together
```

### .

## [ ] M2.2 - Template Representation

### [ ] Implement Template class with params, molecules, reactions, ports
### [ ] Implement Port class with type, direction, path
### [ ] Parse `template.name:` syntax from YAML
### [ ] Implement TemplateRegistry with path-based lookup
### [ ] Load templates from YAML files in catalog/templates/
### [ ] Test: Template.parse() creates valid template with params and ports
### [ ] Test: TemplateRegistry resolves "primitives/energy_cycle"

### .

## [ ] M2.3 - Template Expansion

### [ ] Implement expand() function with namespace prefixing
### [ ] Namespace prefixes: `m.` for molecules, `r.` for reactions
### [ ] Parameter substitution via `!ref`
### [ ] Parse `_instantiate_:` blocks
### [ ] Parse `_as_ name:` and `_as_ name{i in range}:` syntax
### [ ] Recursive template expansion for nested instantiation
### [ ] Index concatenation (chain1, not chain.1)
### [ ] Port declaration parsing (`path: type.direction`)
### [ ] Port connection at instantiation time
### [ ] Port type checking (energy.out connects to energy.in only)
### [ ] Test: expand() produces namespaced molecules and reactions
### [ ] Test: nested instantiation creates hierarchical names
### [ ] Test: port type mismatch raises PortTypeMismatchError

### .

## [ ] M2.4 - Distribution Sampling

### [ ] Seeded random context for reproducibility
### [ ] Distribution functions: normal, lognormal, uniform, poisson, exponential
### [ ] Choice functions: discrete(weights, choices), choice(*options)
### [ ] `!ev` expressions evaluated during expansion
### [ ] Loop ranges can use sampled values
### [ ] Test: same seed produces identical results
### [ ] Test: different seeds produce different results
### [ ] Test: distributions in params sample correctly

### .

## [ ] M2.5 - Guards

### [ ] Implement @guard decorator
### [ ] Implement GuardViolation exception with details
### [ ] Implement GuardContext with scenario, namespace, seed, attempt
### [ ] Built-in guard: no_new_species_dependencies
### [ ] Built-in guard: no_new_cycles
### [ ] Built-in guard: no_essential
### [ ] Guard modes: reject (fail), retry (resample), prune (remove violators)
### [ ] Parse `_guards_:` in YAML with params and mode
### [ ] Test: guard violation raises with context
### [ ] Test: retry mode resamples until success or max_attempts
### [ ] Test: prune mode removes violating elements

### .

## [ ] M2.6 - Visibility Mapping

### [ ] Implement generate_opaque_names() with seeded shuffle
### [ ] Configurable prefix per entity type (M for molecules, RX for reactions)
### [ ] Implement apply_fraction_known() for partial visibility
### [ ] Implement generate_visibility_mapping() per entity type
### [ ] Track hidden elements in _hidden_ list
### [ ] Implement apply_visibility() to rename and filter scenario
### [ ] Update reaction references when molecules renamed
### [ ] Test: visibility mapping is reproducible with same seed
### [ ] Test: fraction_known=0.0 hides all, 1.0 shows all

### .

## [ ] M2.7 - Generator Pipeline

### [ ] Implement Bio.generate(spec, seed) API
### [ ] Pipeline: load → expand → guards → visibility → scenario
### [ ] Preserve _ground_truth_ with internal names
### [ ] Preserve _visibility_mapping_ for debugging
### [ ] Clear error messages with context (template, namespace, seed)
### [ ] Test: Bio.generate() produces valid scenario
### [ ] Test: same seed produces identical scenario
### [ ] Test: ground truth accessible via _ground_truth_

### .

## [ ] M2.9 - Interactions and Modifiers

Features for inter-species wiring and modifying existing elements.

### [ ] Parse `interactions:` section with `_template_:` and `between:`
### [ ] Parse `requires:` for port requirements validation
### [ ] Implement `_modify_:` syntax for altering existing reactions
### [ ] Implement `_set_:` within modify to update fields
### [ ] Validate port requirements before wiring
### [ ] Test: interaction template wires two species together
### [ ] Test: _modify_ changes reactants in existing reaction

### .

## [ ] M2.10 - Background Generation

Generate random filler molecules and reactions respecting guards.

### [ ] Parse `background:` section with molecule/reaction counts
### [ ] Sample counts from distributions (normal, etc.)
### [ ] Generate random background molecules in `m.bg.*` namespace
### [ ] Generate random background reactions in `r.bg.*` namespace
### [ ] Apply guards to background (no_new_species_dependencies, etc.)
### [ ] Retry or prune background elements that violate guards
### [ ] Test: background generates approximately N molecules
### [ ] Test: background reactions don't link different species

### .

## [ ] M2.11 - Container Generation

Generate regions and organism populations from parameters.

### [ ] Parse `parameters.containers:` section
### [ ] Generate N regions from `regions.count` parameter
### [ ] Generate organism populations from `per_species_per_region` distribution
### [ ] Assign initial substrate concentrations
### [ ] Generate outflows between regions
### [ ] Assign organisms to regions with species and counts
### [ ] Test: container generation creates expected region count
### [ ] Test: populations sampled from distribution

### .

## [ ] M2.8 - Integration Test (B10 Mutualism Generator)

Run the full generator on the B10 mutualism example and verify correctness.
This exercises all generator features in a realistic scenario.

**Reference**: [[ASP B10 - World Specification Example]] — Generator spec and expected output

### [ ] Parse template definitions from B10 (energy_cycle, anabolic_chain, etc.)
### [ ] Parse scenario_generator_spec from B10
### [ ] Run Bio.generate() with seed=42
### [ ] Verify molecule count matches expected (~20 molecules)
### [ ] Verify reaction count matches expected (~17 reactions)
### [ ] Verify namespace prefixes (m.Krel.energy.ME1, r.Kova.consume_waste, etc.)
### [ ] Verify port wiring (energy_source fields populated correctly)
### [ ] Verify visibility mapping generated (internal → opaque names)
### [ ] Verify hidden dependencies recorded in _hidden_ list
### [ ] Verify ground truth preserved in _ground_truth_
### [ ] Test: same seed produces identical scenario (reproducibility)
### [ ] Test: different seed produces different sampled values

**Test file**: `tests/integration/test_b10_generator.py`

### .

# Milestone 3 - End-to-End Experiment System

**Concept**: Run complete experiments from the command line. `bio run <experiment>` loads a scenario, wires up an agent, executes the experiment loop, and reports results. This is the minimum viable system for running AI safety experiments.

## [ ] M3.1 - Scenario Execution

Run a generated scenario through the simulator.

### [ ] Bio.run(scenario) executes simulation with scenario config
### [ ] Initialize simulator state from scenario.containers
### [ ] Apply scenario.sim settings (steps, time_step)
### [ ] Execute simulation loop for N steps
### [ ] Return trace object with timeline of states
### [ ] Test: Bio.run() on hardcoded scenario produces expected trajectory

### .

## [ ] M3.2 - Agent Protocol

Define how agents interact with the simulation.

### [ ] Define Agent protocol: observe(), decide(), act()
### [ ] observe() returns: briefing, available_actions, available_measurements, observations
### [ ] decide() returns: action_name, action_params OR measurement_name, measurement_params
### [ ] act() executes agent's decision, returns result
### [ ] Agent receives scenario.interface for available actions/measurements
### [ ] Agent receives scenario.constitution for normative guidance
### [ ] Test: mock agent can observe and act through protocol

### .

## [ ] M3.3 - Agent Implementations

Concrete agent implementations for testing.

### [ ] OracleAgent - has access to ground truth, always makes optimal decisions
### [ ] RandomAgent - makes random valid actions (baseline)
### [ ] ScriptedAgent - follows predefined action sequence (for testing)
### [ ] HumanAgent - interactive CLI prompts for human input
### [ ] LLMAgent - calls LLM API with briefing, receives action (future)
### [ ] Test: OracleAgent scores 1.0 on simple scenario
### [ ] Test: RandomAgent completes without errors

### .

## [ ] M3.4 - Experiment Loop

Orchestrate agent-simulation interaction.

### [ ] Experiment.run(scenario, agent) → results
### [ ] Initialize scenario, present briefing to agent
### [ ] Loop: agent.observe() → agent.decide() → execute action → record to trace
### [ ] Handle measurement actions (return info, don't change state)
### [ ] Handle intervention actions (change state)
### [ ] Termination: max_steps, agent signals done, or quiescence
### [ ] Collect final state for scoring
### [ ] Test: experiment loop runs to completion with mock agent

### .

## [ ] M3.5 - Trace Recording

Record everything that happens during an experiment.

### [ ] Trace class with timeline of (step, state, action, result) tuples
### [ ] Record all agent actions with timestamps
### [ ] Record all measurements with results
### [ ] Record state snapshots at configurable intervals
### [ ] trace.final - final state for scoring
### [ ] trace.actions - list of all agent actions
### [ ] trace.timeline - full state history
### [ ] Test: trace captures all actions in order

### .

## [ ] M3.6 - Scoring Execution

Evaluate agent performance after experiment.

### [ ] Execute scoring functions from scenario.scoring
### [ ] Pass trace to each scoring function
### [ ] Compute aggregate score from individual metrics
### [ ] Compare to scenario.passing_score for pass/fail
### [ ] Return results dict with all scores
### [ ] Support `!_` quoted expressions evaluated at scoring time
### [ ] Test: scoring functions receive trace and return values

### .

## [ ] M3.7 - CLI Commands

Command-line interface for running experiments.

### [ ] `bio run <scenario>` - run single experiment, print results
### [ ] `bio run <scenario> --seed N` - reproducible run with specific seed
### [ ] `bio run <scenario> --agent oracle|random|human` - select agent type
### [ ] `bio report <scope>` - run all scenarios in scope, generate table
### [ ] Output formats: console (default), --csv, --json
### [ ] Show pass/fail status and individual scores
### [ ] Test: CLI commands work with B10 scenarios

### .

## [ ] M3.8 - Results Storage

Store and retrieve experiment results.

### [ ] Save results to data/ folder as DAT
### [ ] Include: scenario name, seed, agent type, scores, trace summary
### [ ] Load previous results for comparison
### [ ] Results DAT structure: results/<scenario>/<timestamp>/
### [ ] Support result aggregation across multiple runs
### [ ] Test: results round-trip through save/load

### .

## [ ] M3.9 - Integration Test (B10 End-to-End)

Run the full B10 mutualism experiment end-to-end.

### [ ] Generate B10 scenario with Bio.generate()
### [ ] Run experiment with OracleAgent
### [ ] Verify agent can observe substrate concentrations
### [ ] Verify agent can add feedstock
### [ ] Verify agent can investigate pathways
### [ ] Verify scoring functions execute correctly
### [ ] Verify OracleAgent achieves passing_score
### [ ] Test: RandomAgent completes (may not pass)

**Test file**: `tests/integration/test_b10_experiment.py`

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


# Milestone 13 - Alien Descriptions and Skinning

Note: Opaque name generation is covered by M2.5 (Visibility Mapping).

## [ ] M13.1 - Description Generator
### [ ] Generate natural language descriptions of bioparts
### [ ] Vary detail level (minimal hints to full explanation)
### [ ] Test: generate descriptions at 3 detail levels, length increases with detail

### .

## [ ] M13.2 - Task Skinning
### [ ] Apply naming and descriptions to generated tasks
### [ ] Produce agent-facing task text with alien terminology
### [ ] Test: skinned task contains no Earth biology terms

### .


# Milestone 14 - End-to-End Validation

**Concept**: Validate full pipeline and iterate based on agent testing. Tune generators to achieve target difficulty range based on AI performance curves.

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
