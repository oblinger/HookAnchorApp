# ABIO MISCELLANEOUS NOTES


## 2026-01-14 Experiments Open Questions

Questions to resolve before implementing the Hello World experiments (H1-H5).

### ✅ Q1: Agent Interface → Direct API with LLM bindings

How should the LLM interact with the testbed?

**Options**:
- **MCP (Model Context Protocol)** — Claude-native, tool definitions as MCP servers
- **Direct API** — Python function calls, agent receives function signatures
- **Chat-based** — Natural language commands parsed by the system

Already designed in [[Agent Interface]]:
- Pure Python `AgentSession` with `observe()`, `act()`, `is_done()`
- `ConversationalLLMAgent` converts to tool/function calls (works with Claude, OpenAI, etc.)
- Timeline model: actions have initiation time + duration
- Turn-based (blocking) or concurrent (async) modes
- Built-in agents: Oracle, Random, Scripted, Human
- No MCP needed — standard tool calling is sufficient and more portable

### ✅ Q2: Observation Format → Structured summary

How should world state be presented to the LLM?

**Options**:
- **YAML dump** — Raw structured data, machine-readable
- **Structured summary** — Formatted text with sections (Molecules, Reactions, etc.)
- **Natural language** — "The cell contains glucose (5.2 units), ATP (3.1 units)..."

Already implemented in [[Agent Interface]] `_format_observation()`:
- Markdown headers for sections ("## Current Observations")
- Key-value pairs for state (`molecule: concentration`)
- Nested dicts shown with indentation
- Human-readable but machine-parseable

### ✅ Q3: Action Vocabulary → Technical/Descriptive

How verbose/descriptive should action names be?

**Options**:
- **Technical**: `add_molecule`, `adjust_rate`, `observe_concentrations`
- **Descriptive**: `inject_substance`, `modify_reaction_speed`, `measure_levels`
- **Alien-themed**: `infuse_compound`, `alter_pathway_flux`, `sense_abundances`

**Decision**: Use **technical/descriptive** vocabulary (not alien-themed).

Rationale: The "alien" part of Alien Biology is the *specific world details* (which molecules, what reactions do), not the *general domain concepts* (reactions, pathways, rates, concentrations). General biology knowledge is explicitly allowed to transfer — we're testing whether LLMs can apply that knowledge to novel situations, not whether they can decode opaque terminology.

### ✅ Q4: Alien Naming Strategy → Visibility mapping (skinning)

When should we use opaque names (M1, M2, RX7) vs descriptive names (glucose, ATP)?

**Options**:
- **Always opaque** — Pure novelty, no knowledge transfer possible
- **Always descriptive** — Maximizes knowledge transfer
- **Progressive** — Start descriptive (H1-H3), switch to opaque (H4-H5)
- **Variant-based** — Each test has both variants for comparison

**Decision**: Use **visibility mapping** (aka "skinning") — already designed in [[Generator Spec#Visibility Specification]].

The system maintains:
- **Ground truth** — Internal descriptive names (`m.krel.energy.M1`, `r.krel.pathway1.build`)
- **`_visibility_mapping_`** — Maps internal → opaque names (`ME1`, `RX1`)
- **AI view** — Sees only the mapped opaque names

This enables:
- Same scenario logic, different presentations (transparent vs opaque skin)
- Variant-based testing by generating with different visibility mappings
- Ground truth preserved for scoring/analysis

### ✅ Q5: Multi-Turn Budget → Dual budgets (cost + simulation time)

How many API calls / tool invocations constitute a "fair" test?

**Options**:
- **Unlimited** — Let the LLM take as long as it needs
- **Fixed budget** — e.g., 10 observations, 5 actions per test
- **Cost-based** — Actions have costs, total budget in "units"
- **Time-based** — Wall-clock or token limit

**Decision**: Use **dual budgets** — already designed in [[Agent Interface#Termination Conditions]].

Two independent limits per scenario:
- `action.limits.budget` — Cost budget (total action/measurement costs)
- `action.limits.max_sim_time` — Simulation time budget (not wall clock)

Either or both can be specified (null = unlimited). Termination occurs when any limit is exceeded. This allows scenarios to constrain by resources, time, or both.

### ✅ Q6: Baseline Agents → Random, Oracle, Scripted, Human

What non-LLM baselines should we include for comparison?

**Options**:
- **Random** — Random valid actions, seeded for reproducibility
- **Oracle** — Has access to ground truth, computes optimal policy
- **Scripted** — Predefined action sequence for deterministic tests
- **Greedy** — Simple heuristic (e.g., always add the target molecule)
- **Human** — Interactive CLI for researcher baseline

Already implemented in [[Agent Interface]]:
- `RandomAgent` — random valid actions, seeded
- `OracleAgent` — computes optimal policy from ground truth
- `ScriptedAgent` — follows predefined action sequence
- `HumanAgent` — interactive CLI

Greedy heuristics can be implemented as ScriptedAgents for specific tests.

### ✅ Q7: Evaluation Without LLM Grader → Simulation-based scoring

How do we evaluate free-form responses without using another LLM?

**Options**:
- **Multiple choice only** — All questions have fixed answer options
- **Structured output** — Require JSON/YAML responses, parse programmatically
- **Keyword matching** — Check for presence of key terms
- **Numerical only** — All answers are numbers with tolerance

**Decision**: **No direct response evaluation** — already designed in [[Agent Interface]].

We don't evaluate what the agent *says*. We evaluate what happens in the *simulation*:
- Agent interacts with simulation through actions/measurements
- `scoring.score` is a **computable function** of the trace/state (e.g., `!_ population_health(trace)`)
- `passing_score` threshold determines pass/fail
- No LLM grader needed — everything is computed programmatically

Future extension: rubric-style grading (A/B/C/D/F) based on score ranges.


## 2026-01-14  ABIO Experiments roadmap

### Overall Objective: Isolating Learning and Solving from Prior Training

The Alien Biology objective is to measure the LLM's ability to apply generalized knowledge in provably novel contexts. Existing benchmarks measure how well LLMs capture general-purpose reasoning strategies from from their training corpora. Here we rigorously isolate the system's capacity to construct and utilize novel world models when faced with novel knowledge problems.

- **Separation of knowledge types** — We distinguish between generalized field knowledge (strategies and patterns a student might learn while training in a discipline) and problem-specific knowledge (what must be inferred when confronting a novel situation). Our tests should be insensitive to the former while measuring the latter in isolation.

- **Provable novelty** — Tasks must be designed so that the specific world model required cannot have been provided—even indirectly—by the system's training. This removes the "taint" of background knowledge and reinforcement learning, ensuring we measure genuine inference rather than sophisticated recall.

- **Why this matters** — If an LLM succeeds only when problems resemble its training data, it is interpolating. If it succeeds on provably novel problems requiring the same general strategies, it is genuinely applying knowledge to new contexts.

### Questions

### Provably Novel World Model Building

To what degree can LLMs construct and manipulate internal representations of novel systems, versus relying on pattern interpolation from training data?
- **Verifiable ground truth** — This testbed provides physically-grounded scenarios where predictions can be checked against simulation, separating genuine reasoning from plausible confabulation, not just questions and answers.
- **Multi-dimensional integration** — The constructed world model must capture and integrate physical constraints, logical relationships, novel conceptual distinctions, and their relationships to observable indirect measures—dimensions that interact but cannot be reduced to one another.

### Long-Horizon Grounded Task Completion with Limited Understanding and Observability

Can an LLM manage itself toward a complex objective over extended periods while interacting with a world it cannot fully see or understand? This tests sustained goal-directed behavior characterized by:

- **Long-running tasks** — The objective requires decomposition into hundreds or thousands of sub-goals managed over time
- **Grounded interaction** — Success requires acting through sensors and actuators, not just answering questions about provided information
- **Managing resources across competing concerns**:
  - Partial observability — The system cannot see the full world state and must choose what to observe
  - Limited understanding — The dynamics and rules of the world are not fully known
  - Making and validation of progress — The system must verify it is actually advancing toward the goal

# Experiments

## Hello World Progression

A sequence of experiments validating that the testbed is functioning and that LLMs can meaningfully engage with it. These tests serve two purposes:

1. **End-to-end system validation** — Exercise each component of the testbed to confirm the infrastructure works
2. **On-ramp verification** — Confirm that LLMs can actually enter this problem space before measuring sophisticated capabilities

Key concerns this progression must address:

- **Knowledge transfer** — Can LLMs map their prior knowledge of biology onto this novel domain? If they cannot recognize that familiar reasoning strategies apply, the tests measure nothing.
- **Control interface** — Can LLMs manage themselves through whatever control mechanism we provide (MCP, direct API, etc.)? They must be able to observe, take actions, and run experiments.
- **Baseline engagement** — Even if LLMs cannot fully solve complex problems, they should be able to make meaningful initial progress. If they fail to even begin attacking problems, we are not measuring the intended capabilities.

After completing this progression, we should have confidence that the testbed can produce experiments that directly address our two main questions.

### H1: Representation Comprehension
**Objectives**:
- Verify the LLM can parse and understand the alien biology representation format
- Confirm it can answer basic structural questions without simulation

**Construction**:
- Present a simple world: 2-3 compartments, 3-5 molecules, 2-3 reactions
- Ask factual questions: "What molecules are in compartment X?", "What are the products of reaction Y?", "Which compartments share a membrane?"
- Success = accurate answers about static structure

### H2: Single-Step Dynamics Prediction
**Objectives**:
- Verify the LLM can reason about dynamics from indirect observations
- Confirm it can predict concentration changes without seeing which reactions fired

**Construction**:
- Present a world with known molecules and known possible reactions
- Provide a simplified measurement device: concentrations of each molecule per compartment (abstracting mass spectrometry + molecule list → concentrations)
- The LLM observes concentrations at t=0, simulation runs one step, LLM observes concentrations at t=1
- Ask: "What happened? Which reactions likely fired? What do you expect at t=2?"
- Note: The LLM never directly sees reaction events—only concentration changes
- Success = reasonable inference about which reactions occurred, correct directional predictions for next step

### H3: Control Interface Exercise
**Objectives**:
- Verify the LLM can operate the observation/action interface
- Confirm it can execute a simple prescribed sequence

**Construction**:
- Provide explicit instructions: "Observe the state, run 10 simulation steps, observe again, report what changed"
- The LLM must correctly invoke the API/MCP tools in sequence
- Success = correct tool usage, coherent report of observations

### H4: Goal-Directed Single Intervention
**Objectives**:
- Verify the LLM can connect goals to actions in this domain
- Confirm it can reason backward from desired outcome to intervention

**Construction**:
- Present a world and a simple goal: "Increase glucose concentration in the cell"
- The LLM must choose an action (add molecules, trigger a reaction, modify a flow)
- Run simulation and check if the goal was achieved
- Success = chooses a reasonable intervention, articulates why

### H5: Hypothesis Formation from Observation
**Objectives**:
- Verify the LLM can observe dynamics and form hypotheses about hidden rules
- Confirm basic scientific reasoning transfers to this domain

**Construction**:
- Present a world with an unlabeled reaction (LLM doesn't know the stoichiometry)
- Let it run experiments: vary inputs, observe outputs
- Ask it to infer what the reaction does
- Success = correctly identifies reactants, products, and approximate rates

## Measurement Dimensions

_After the Hello World progression, this section will define orthogonal dimensions for measuring the target capabilities. Where possible, each dimension should have smoothly controllable complexity, allowing researchers to calibrate difficulty and measure incremental progress._

### Notes and Questions

#### How do we measure the system's performance on these tests?

Different tests may require different evaluation approaches. For example, H2 (dynamics prediction) could use multiple choice to avoid needing a separate grader for textual answers. Some options:
- Multiple choice (objective, no grader needed)
- Numerical prediction with tolerance (e.g., concentration within ±10%)
- Binary success/failure based on simulation outcome (e.g., did the goal get achieved?)
- Structured output that can be programmatically compared to ground truth

Prefer evaluation methods that don't require another LLM to grade responses.

## Architecture-First Organization

Protocols are defined in an `architecture/` folder, separate from implementations. This enforces clear contracts and enables multiple implementations (e.g., PythonSimulator vs RustSimulator). See [[#Folder Structure]] for full layout.


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


## 2026-01-14  ABIO Experiments roadmap

### Overall Objective: Isolating Learning and Solving from Prior Training

The Alien Biology objective is to measure the LLM's ability to apply generalized knowledge in provably novel contexts. Existing benchmarks measure how well LLMs capture general-purpose reasoning strategies from from their training corpora. Here we rigorously isolate the system's capacity to construct and utilize novel world models when faced with novel knowledge problems.

- **Separation of knowledge types** — We distinguish between generalized field knowledge (strategies and patterns a student might learn while training in a discipline) and problem-specific knowledge (what must be inferred when confronting a novel situation). Our tests should be insensitive to the former while measuring the latter in isolation.

- **Provable novelty** — Tasks must be designed so that the specific world model required cannot have been provided—even indirectly—by the system's training. This removes the "taint" of background knowledge and reinforcement learning, ensuring we measure genuine inference rather than sophisticated recall.

- **Why this matters** — If an LLM succeeds only when problems resemble its training data, it is interpolating. If it succeeds on provably novel problems requiring the same general strategies, it is genuinely applying knowledge to new contexts.

### Questions

### Provably Novel World Model Building

To what degree can LLMs construct and manipulate internal representations of novel systems, versus relying on pattern interpolation from training data?
- **Verifiable ground truth** — This testbed provides physically-grounded scenarios where predictions can be checked against simulation, separating genuine reasoning from plausible confabulation, not just questions and answers.
- **Multi-dimensional integration** — The constructed world model must capture and integrate physical constraints, logical relationships, novel conceptual distinctions, and their relationships to observable indirect measures—dimensions that interact but cannot be reduced to one another.

### Long-Horizon Grounded Task Completion with Limited Understanding and Observability

Can an LLM manage itself toward a complex objective over extended periods while interacting with a world it cannot fully see or understand? This tests sustained goal-directed behavior characterized by:

- **Long-running tasks** — The objective requires decomposition into hundreds or thousands of sub-goals managed over time
- **Grounded interaction** — Success requires acting through sensors and actuators, not just answering questions about provided information
- **Managing resources across competing concerns**:
  - Partial observability — The system cannot see the full world state and must choose what to observe
  - Limited understanding — The dynamics and rules of the world are not fully known
  - Making and validation of progress — The system must verify it is actually advancing toward the goal

# Experiments

## Hello World Progression

A sequence of experiments validating that the testbed is functioning and that LLMs can meaningfully engage with it. These tests serve two purposes:

1. **End-to-end system validation** — Exercise each component of the testbed to confirm the infrastructure works
2. **On-ramp verification** — Confirm that LLMs can actually enter this problem space before measuring sophisticated capabilities

Key concerns this progression must address:

- **Knowledge transfer** — Can LLMs map their prior knowledge of biology onto this novel domain? If they cannot recognize that familiar reasoning strategies apply, the tests measure nothing.
- **Control interface** — Can LLMs manage themselves through whatever control mechanism we provide (MCP, direct API, etc.)? They must be able to observe, take actions, and run experiments.
- **Baseline engagement** — Even if LLMs cannot fully solve complex problems, they should be able to make meaningful initial progress. If they fail to even begin attacking problems, we are not measuring the intended capabilities.

After completing this progression, we should have confidence that the testbed can produce experiments that directly address our two main questions.

### H1: Representation Comprehension
**Objectives**:
- Verify the LLM can parse and understand the alien biology representation format
- Confirm it can answer basic structural questions without simulation

**Construction**:
- Present a simple world: 2-3 compartments, 3-5 molecules, 2-3 reactions
- Ask factual questions: "What molecules are in compartment X?", "What are the products of reaction Y?", "Which compartments share a membrane?"
- Success = accurate answers about static structure

### H2: Single-Step Dynamics Prediction
**Objectives**:
- Verify the LLM can reason about dynamics from indirect observations
- Confirm it can predict concentration changes without seeing which reactions fired

**Construction**:
- Present a world with known molecules and known possible reactions
- Provide a simplified measurement device: concentrations of each molecule per compartment (abstracting mass spectrometry + molecule list → concentrations)
- The LLM observes concentrations at t=0, simulation runs one step, LLM observes concentrations at t=1
- Ask: "What happened? Which reactions likely fired? What do you expect at t=2?"
- Note: The LLM never directly sees reaction events—only concentration changes
- Success = reasonable inference about which reactions occurred, correct directional predictions for next step

### H3: Control Interface Exercise
**Objectives**:
- Verify the LLM can operate the observation/action interface
- Confirm it can execute a simple prescribed sequence

**Construction**:
- Provide explicit instructions: "Observe the state, run 10 simulation steps, observe again, report what changed"
- The LLM must correctly invoke the API/MCP tools in sequence
- Success = correct tool usage, coherent report of observations

### H4: Goal-Directed Single Intervention
**Objectives**:
- Verify the LLM can connect goals to actions in this domain
- Confirm it can reason backward from desired outcome to intervention

**Construction**:
- Present a world and a simple goal: "Increase glucose concentration in the cell"
- The LLM must choose an action (add molecules, trigger a reaction, modify a flow)
- Run simulation and check if the goal was achieved
- Success = chooses a reasonable intervention, articulates why

### H5: Hypothesis Formation from Observation
**Objectives**:
- Verify the LLM can observe dynamics and form hypotheses about hidden rules
- Confirm basic scientific reasoning transfers to this domain

**Construction**:
- Present a world with an unlabeled reaction (LLM doesn't know the stoichiometry)
- Let it run experiments: vary inputs, observe outputs
- Ask it to infer what the reaction does
- Success = correctly identifies reactants, products, and approximate rates

## Measurement Dimensions

_After the Hello World progression, this section will define orthogonal dimensions for measuring the target capabilities. Where possible, each dimension should have smoothly controllable complexity, allowing researchers to calibrate difficulty and measure incremental progress._

### Notes and Questions

#### How do we measure the system's performance on these tests?

Different tests may require different evaluation approaches. For example, H2 (dynamics prediction) could use multiple choice to avoid needing a separate grader for textual answers. Some options:
- Multiple choice (objective, no grader needed)
- Numerical prediction with tolerance (e.g., concentration within ±10%)
- Binary success/failure based on simulation outcome (e.g., did the goal get achieved?)
- Structured output that can be programmatically compared to ground truth

Prefer evaluation methods that don't require another LLM to grade responses.

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






