# ABIO Roadmap
**Related**: [[alienbio]], [[Testing]]
Implementation roadmap for the alienbio project.

---

# Milestone 1 - Minimal Simulation Loop

Core architecture: entities, protocols, and basic simulation.

## M1.0 - Environment Setup
### Create alienbio repository parallel to abio1
### Initialize uv environment with pyproject.toml
### Create justfile with standard commands
### Set up folder structure per architecture spec
### Configure pytest for tests/ folder
### Add rust/ subfolder with Cargo.toml placeholder
### Test: `just test` runs and passes with empty test suite

### .

## M1.1 - Infrastructure Operators

Top-level operations for working with alienbio. See [[alienbio]] module topic for full specification.

### Implement do(name) - resolve dotted names to objects
### Implement create(spec) - instantiate from prototypes
### Implement load(path) / save(obj, path) - entity persistence
### Implement print format - PREFIX:name display
### Implement parse(string) - reconstruct from string
### Implement ctx() - access runtime context
### Test: do("catalog.kegg1") resolves (with stub)
### Test: print(Entity) produces PREFIX:name format
### Test: parse(str(entity)) round-trips correctly
### Test: ctx() returns Context with ContextVar (not plain global)

### .

## M1.2 - Core Protocols
### Define BioMolecule protocol with name, properties
### Define BioReaction protocol with reactants, products, rate
### Define State protocol as dict of molecule concentrations
### Define Simulator protocol with step() method
### Test: pyright passes on all protocol definitions

### .

## M1.3 - Entity Infrastructure
### Implement Entity base with string serialization (repr/str)
### Implement YAML serialization for complex entities
### Implement deserialization from string and YAML
### Integrate dvc_dat for named entity persistence (data/ folder)
### All bio classes (BioMolecule, BioReaction, etc.) inherit from Entity
### Test: round-trip Entity to string and back, assert equality
### Test: save entity to data/, load by name, assert equality

### .

## M1.4 - Hardcoded Test System
### Create 3-4 molecules by hand (A, B, C, D)
### Create 2 reactions by hand (A + B → C, C → D)
### Define fixed rate functions returning constants
### Instantiate initial state with known concentrations
### Test: instantiate system, assert molecule and reaction counts correct

### .

## M1.5 - Python Simulator v0
### Implement step() applying all reactions once
### Implement run() looping step() for N iterations
### Return timeline of states
### Test: run 10 steps, assert timeline has 11 states (initial + 10)

### .

## M1.6 - Verification
### Run simulation on hardcoded system
### Assert concentrations change as expected
### Plot concentration curves over time
### Test: after 100 steps, A and B depleted, C and D increased

### .


# Milestone 2 - KEGG Data Integration

**Concept**: Extract statistical distributions from KEGG biochemistry to serve as templates for synthetic biology generation. Capture molecule properties (atom counts, functional groups, molecular weight), reaction patterns (reactant/product counts, delta-depth), and connectivity (in/out degree per biosynthetic depth layer).

## M2.1 - KEGG Parsing
### Parse compound flat files into BioMolecule instances
### Parse reaction flat files into BioReaction instances
### Build molecule registry and reaction registry
### Test: parse sample KEGG files, assert expected molecule/reaction counts

### .

## M2.2 - DAG Construction
### Compute bdepth for each molecule
### Link molecules via produced_by/consumed_by
### Validate DAG connectivity
### Test: bdepth=0 molecules have no produced_by, all others have at least one

### .

## M2.3 - Statistical Extraction
### Count molecules per bdepth layer
### Compute in-degree/out-degree histograms
### Compute reaction template distributions (n_reactants, n_products)
### Extract delta-depth distributions
### Test: histograms sum to total molecule/reaction counts

### .

## M2.4 - BioChemistryModel
### Populate MoleculeLayerStats from KEGG data
### Populate ReactionClassStats (anabolic, catabolic, energy)
### Serialize model to JSON for inspection
### Test: round-trip serialize/deserialize, assert equality

### .


# Milestone 3 - Synthetic Molecule Generation

**Concept**: Generate alien molecules bottom-up. Start with primitives (bdepth=0) from alien atoms, then build higher-depth molecules via synthetic reactions. Generate alien nomenclature via markov or diffusion model.

## M3.1 - Molecule Generator Protocol
### Define interface for generating molecules from stats
### Accept layer, count, degree distributions as parameters
### Test: protocol definition passes type check

### .

## M3.2 - Primitive Molecules
### Generate bdepth=0 molecules with random names
### Assign synthetic properties (placeholder atoms)
### Test: generate 10 primitives, all have bdepth=0 and unique names

### .

## M3.3 - Layered Generation
### Generate molecules layer by layer using captured distributions
### Sample in-degree from histogram to determine how many reactions produce each
### Build produced_by/consumed_by links as placeholders
### Test: generate 5 layers, molecule counts per layer match requested distribution

### .

## M3.4 - Verification
### Compare synthetic molecule stats to KEGG stats
### Assert distributions match within tolerance
### Test: KS-test on degree distributions, p > 0.05

### .


# Milestone 4 - Synthetic Reaction Generation

**Concept**: Generate alien biochemistry with anabolic (build-up), catabolic (breakdown), and energy reactions. Assign rate functions from parametric templates. Create closed-loop energy carriers (alien ATP/NADH analogs).

## M4.1 - Reaction Generator Protocol
### Define interface for generating reactions linking molecules
### Accept template distribution, depth deltas as parameters
### Test: protocol definition passes type check

### .

## M4.2 - Basic Reactions
### Sample (n_reactants, n_products) from template distribution
### Select reactants from layer L, products from layer L+1
### Create BioReaction instances
### Test: generate 20 reactions, all have valid reactant/product references

### .

## M4.3 - Rate Functions
### Define parametric rate function templates
### Assign random parameters within plausible ranges
### Attach rate functions to reactions
### Test: call rate function with sample state, returns positive float

### .

## M4.4 - Verification
### Generate small synthetic chemistry (50 molecules, 30 reactions)
### Run simulation, confirm no runaway concentrations
### Compare reaction stats to KEGG stats
### Test: after 1000 steps, all concentrations remain in [0, 1e6] range

### .


# Milestone 5 - Single Compartment Simulation

## M5.1 - BioSystem Assembly
### Combine synthetic molecules and reactions into BioSystem
### Initialize state with random concentrations
### Test: BioSystem has correct molecule and reaction counts

### .

## M5.2 - Equilibrium Testing
### Run simulation until concentrations stabilize
### Identify and fix unstable reaction rates
### Define homeostasis targets
### Test: variance of concentrations over last 100 steps < threshold

### .

## M5.3 - Perturbation Testing
### Inject concentration spike, observe recovery
### Remove a reaction, observe divergence
### Validate system responds to interventions
### Test: spike recovery within 50 steps; removal causes measurable drift

### .


# Milestone 6 - Measurements and Actions

**Concept**: Agent-facing API for observations and actions. Measurements observe limited aspects of system state. Actions perturb system state.

## M6.1 - Measurement Protocol
### Define interface: measure(state, params) → value
### Implement concentration measurement
### Implement rate measurement
### Test: measure known state, assert correct concentration returned

### .

## M6.2 - Action Protocol
### Define interface: act(state, params) → new_state
### Implement add_molecule action
### Implement adjust_rate action
### Test: add_molecule increases concentration by expected amount

### .

## M6.3 - Agent Interface
### Bundle measurements and actions into agent-facing API
### Define text descriptions for each measurement/action
### Test agent can query and act via API
### Test: call API methods, assert valid responses and state changes

### .


# Milestone 7 - Task Framework

**Concept**: Tasks specify goals with scoring criteria. Types include predict (forecast behavior), diagnose (identify disease), and cure (restore health).

## M7.1 - Task Protocol
### Define Task with setup, goal, scoring, criteria
### Implement predict task: forecast concentration after N steps
### Test: protocol definition passes type check

### .

## M7.2 - Experiment Protocol
### Define Experiment combining world, task, agent
### Implement run_experiment returning score
### Test: run experiment with mock agent, returns numeric score

### .

## M7.3 - Hardcoded Predict Task
### Create simple system with known dynamics
### Define prediction task with ground truth
### Score agent's prediction accuracy
### Test: perfect prediction scores 1.0, random prediction scores < 0.5

### .


# Milestone 8 - Multi-Compartment Systems

**Concept**: Organism construction with compartmentalized systems. Generate organs as DAGs of bioparts with transport reactions between compartments. Build nested hierarchy from organelles to cells to organs to organism. Establish homeostasis via feedback loops maintaining concentration targets.

## M8.1 - Compartment Model
### Extend State to support multiple compartments
### Define transport reactions moving molecules between compartments
### Test: state with 2 compartments, each has independent concentrations

### .

## M8.2 - Organ Generator
### Generate compartments with local reactions
### Generate transport reactions between compartments
### Assemble into BioOrganism
### Test: generated organism has expected compartment count and transport links

### .

## M8.3 - Cross-Compartment Simulation
### Extend Simulator to handle multi-compartment state
### Verify transport moves molecules correctly
### Test homeostasis across compartments
### Test: molecule injected in compartment A appears in compartment B after N steps

### .


# Milestone 9 - Disease and Variation

**Concept**: Define healthy baseline (equilibria and acceptable ranges). Generate perturbations: mutations (altered rates, removed reactions) and deficiencies (reduced enzyme concentrations). Diseases produce measurable symptoms as deviations from baseline.

## M9.1 - Baseline Definition
### Define healthy steady-state for organism
### Define acceptable ranges for each homeostatic target
### Test: healthy organism stays within ranges for 1000 steps

### .

## M9.2 - Perturbation Generator
### Generate mutations: altered rate, removed reaction
### Generate deficiencies: reduced enzyme concentration
### Apply perturbation to create diseased organism
### Test: perturbation changes at least one reaction rate or removes reaction

### .

## M9.3 - Symptom Measurement
### Define symptom as deviation from baseline
### Implement symptom detection measurements
### Verify diseases produce observable symptoms
### Test: diseased organism has at least one measurement outside healthy range

### .


# Milestone 10 - Diagnosis and Cure Tasks

**Concept**: Diagnosis tasks provide diseased organism with limited measurements; agent identifies which perturbation was applied. Cure tasks provide action toolkit; agent restores homeostatic targets to baseline. Difficulty scales with number of possible diseases and measurement limitations.

## M10.1 - Diagnosis Task
### Provide diseased organism with limited measurements
### Agent must identify which perturbation was applied
### Score based on correct identification
### Test: oracle agent with full info scores 1.0

### .

## M10.2 - Cure Task
### Provide diseased organism with action toolkit
### Agent must restore homeostatic targets to baseline
### Score based on recovery quality
### Test: applying correct cure returns organism to healthy range

### .

## M10.3 - Task Difficulty Scaling
### Parameterize number of possible diseases
### Parameterize measurement limitations
### Generate tasks at varying difficulty levels
### Test: difficulty=1 task easier than difficulty=5 (measured by oracle performance)

### .


# Milestone 11 - Test Harness

## M11.1 - Test Definition
### Define Test as batch of experiments
### Support parameter sweeps (worlds, agents, settings)
### Test: create Test with 10 experiments, assert count matches

### .

## M11.2 - Execution Runner
### Run experiments in sequence or parallel
### Log all agent actions and measurements
### Aggregate scores across experiments
### Test: run batch of 5 experiments, receive 5 scores

### .

## M11.3 - Result Analysis
### Compute pass/fail rates per difficulty level
### Plot performance curves
### Export results for comparison
### Test: export to JSON, re-import, assert data integrity

### .


# Milestone 12 - Rust Simulator

## M12.1 - Rust Core
### Implement State, step(), run() in Rust
### Match Python simulator semantics exactly
### Test: Rust unit tests pass for step() and run()

### .

## M12.2 - PyO3 Binding
### Expose RustSimulator to Python via PyO3
### Implement same Simulator protocol as Python version
### Test: import RustSimulator in Python, call step()

### .

## M12.3 - Verification
### Run identical simulations on both simulators
### Assert outputs match within floating-point tolerance
### Benchmark performance difference
### Test: max difference between Python and Rust outputs < 1e-9

### .


# Milestone 13 - Alien Naming and Skinning

## M13.1 - Name Generator
### Build markov model from biological nomenclature
### Generate plausible alien names for molecules, reactions
### Test: generate 100 names, all unique, all pronounceable (no triple consonants)

### .

## M13.2 - Description Generator
### Generate natural language descriptions of bioparts
### Vary detail level (minimal hints to full explanation)
### Test: generate descriptions at 3 detail levels, length increases with detail

### .

## M13.3 - Task Skinning
### Apply naming and descriptions to generated tasks
### Produce agent-facing task text with alien terminology
### Test: skinned task contains no Earth biology terms

### .


# Milestone 14 - End-to-End Validation

**Concept**: Validate full pipeline, calibrate against KEGG ground truth, and iterate based on agent testing. Tune generators to achieve target difficulty range based on AI performance curves.

## M14.1 - Full Pipeline Test
### Generate alien biology from scratch
### Generate organism with disease
### Generate diagnosis and cure tasks
### Run LLM agent through tasks
### Score and analyze results
### Test: pipeline completes without error, produces valid scores

### .

## M14.2 - Difficulty Calibration
### Sweep complexity parameters
### Identify where agent performance degrades
### Tune generators to target difficulty range
### Test: performance curve shows expected degradation with difficulty

### .

## M14.3 - Documentation
### Document generated biology format
### Document task specification format
### Document agent interface API
### Test: all public functions have docstrings, sphinx builds without warnings

### .
