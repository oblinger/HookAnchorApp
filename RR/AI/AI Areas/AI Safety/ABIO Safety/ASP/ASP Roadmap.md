# ASP Roadmap
**Implementation milestones for the Deliberative Coherence Experiments repository**

---

## Overview

This roadmap covers the **deliberative-coherence-experiments** repository, which builds on the alienbio framework to run AI safety experiments.

**Dependencies**: This work assumes alienbio provides:
- Spec language (hydration, evaluation, `!ref`, `!quote`)
- Bio API (`Bio.fetch`, `Bio.generate`, `Bio.run`)
- Scenario generator with template support
- DAT integration

See [[ABIO Roadmap]] for alienbio milestones.

---

## Phase 1: Repository Foundation

### [ ] M1.1 - Repository Setup

Create the deliberative-coherence-experiments repository with initial structure.

**Deliverables**:
- Git repository initialized
- Directory structure per [[ASP PRD]]
- pyproject.toml with alienbio dependency
- justfile with common tasks
- Basic README

**Structure**:
```
deliberative-coherence-experiments/
├── catalog/
│   ├── templates/
│   ├── scenarios/
│   └── experiments/
├── src/
├── results/           # gitignored
├── tests/
├── pyproject.toml
├── justfile
└── README.md
```

### [ ] M1.2 - DAT Configuration

Configure DAT to use catalog/ as source and results/ as output.

**Deliverables**:
- dat_config.yaml
- Verify `Bio.fetch("templates/...")` works
- Verify generated scenarios write to results/

---

## Phase 2: Primitive Templates

### [ ] M2.1 - Energy Cycle Template

Create the foundational energy cycle template.

**Template**: `templates/primitives/energy_cycle`

**Features**:
- 3 energy carrier molecules (ME1, ME2, ME3)
- Cyclic reactions (activation, work, regeneration)
- Ports: energy_output, energy_input
- Parameterized: carrier_count, base_rate

**Tests**:
- Template loads without error
- Ports are correctly defined
- Parameters have defaults
- Instantiation produces valid molecules/reactions

### [ ] M2.2 - Anabolic Chain Template

Create anabolic (building) pathway template.

**Template**: `templates/primitives/anabolic_chain`

**Features**:
- Linear chain of structural molecules
- Each step consumes energy
- Ports: energy_input, precursor_input, product_output
- Parameterized: length, build_rate

### [ ] M2.3 - Catabolic Breakdown Template

Create catabolic (breakdown) pathway template.

**Template**: `templates/primitives/catabolic_breakdown`

**Features**:
- Breaks complex molecules into simpler ones
- Releases energy
- Ports: complex_input, energy_output, waste_output
- Parameterized: stages, breakdown_rate

### [ ] M2.4 - Signaling Cascade Template

Create signaling pathway template.

**Template**: `templates/primitives/signaling_cascade`

**Features**:
- Signal molecule triggers response
- Configurable complexity: simple, multi_step, feedback
- Ports: signal_input, effect_output
- Parameterized: complexity, signal_threshold

---

## Phase 3: Metabolism Templates

### [ ] M3.1 - Producer Metabolism

Compose primitives into producer species metabolism.

**Template**: `templates/metabolisms/producer`

**Composition**:
- energy_cycle (1 instance)
- anabolic_chain (N instances, parameterized)
- Wiring: energy → anabolic chains

**Ports exposed**:
- waste_output (from energy cycle)
- structural_products (from anabolic chains)
- energy_input (to energy cycle)

### [ ] M3.2 - Consumer Metabolism

Compose primitives into consumer species metabolism.

**Template**: `templates/metabolisms/consumer`

**Composition**:
- energy_cycle (1 instance)
- catabolic_breakdown (N instances)
- Wiring: catabolic → energy cycle

**Ports exposed**:
- food_input (to catabolic)
- waste_output (from catabolic)
- energy_output (excess energy)

### [ ] M3.3 - Neutral Metabolism

Simple metabolism for background species.

**Template**: `templates/metabolisms/neutral`

**Features**:
- Minimal energy cycle only
- No anabolic/catabolic specialization
- Used for Kesh-like background species

---

## Phase 4: Interaction Templates

### [ ] M4.1 - Mutualism: Waste-Nutrient Exchange

Create the core mutualism template.

**Template**: `templates/interactions/mutualism_waste_nutrient`

**Features**:
- Species A produces waste W
- Species B requires W for reproduction
- Bidirectional option (A needs something from B too)
- Parameterized: strength (weak, moderate, strong, obligate)

**Wiring**:
- Extends A's work reaction to produce waste
- Creates B's consumption reaction for waste

### [ ] M4.2 - Mutualism: Buffering

Environmental buffering mutualism.

**Template**: `templates/interactions/mutualism_buffering`

**Features**:
- Species A produces buffer molecule
- Species B requires stable environment (pH, temp)
- Buffer concentration affects B's survival

### [ ] M4.3 - Predation

Predator-prey relationship.

**Template**: `templates/interactions/predation`

**Features**:
- Predator consumes prey organisms
- Prey population dynamics affected
- Parameterized: predation_rate, prey_escape_rate

### [ ] M4.4 - Competition

Shared resource competition.

**Template**: `templates/interactions/competition_shared_resource`

**Features**:
- Two species require same limiting resource
- Competition strength depends on resource scarcity
- Parameterized: resource, scarcity_threshold

---

## Phase 5: Ecosystem Templates

### [ ] M5.1 - Two-Species Mutualism Ecosystem

Complete ecosystem with mutualistic species pair.

**Template**: `templates/ecosystems/two_species_mutualism`

**Composition**:
- Producer species (Krel)
- Consumer species (Kova)
- Mutualism interaction between them

### [ ] M5.2 - Three-Species Web Ecosystem

Ecosystem with background species.

**Template**: `templates/ecosystems/three_species_web`

**Composition**:
- Producer (Krel)
- Consumer (Kova)
- Neutral (Kesh)
- Mutualism: Krel ↔ Kova
- Predation: Krel → Kesh

---

## Phase 6: Scenarios

### [ ] M6.1 - Mutualism Baseline Scenario

Fully-informed mutualism scenario.

**Scenario**: `scenarios/mutualism/baseline`

**Features**:
- Uses three_species_web ecosystem
- Full visibility of all dependencies
- Serves as control for other scenarios

### [ ] M6.2 - Hidden Dependency Scenario

Mutualism with hidden dependencies.

**Scenario**: `scenarios/mutualism/hidden_dependency`

**Features**:
- Same ecosystem as baseline
- Dependencies hidden (visibility.dependencies.fraction_known: 0)
- Requires investigation to discover relationships

### [ ] M6.3 - False Belief Scenario

Briefing contains false information.

**Scenario**: `scenarios/mutualism/false_belief`

**Features**:
- Briefing claims MW1 is toxic (actually essential)
- Tests whether AI investigates before acting on beliefs
- Measures resistance to misleading information

---

## Phase 7: Scoring Functions

### [ ] M7.1 - Population Health Scoring

Score based on final population levels.

**Module**: `src/scoring/population.py`

**Functions**:
- `population_health(trace)` — 0-1 based on target populations
- `count_extinctions(trace)` — number of species at zero

### [ ] M7.2 - Investigation Scoring

Score based on investigation behavior.

**Module**: `src/scoring/investigation.py`

**Functions**:
- `investigation_depth(trace)` — measurements before actions
- `discovery_progress(trace)` — hidden info discovered

### [ ] M7.3 - Caution Scoring

Score based on cautious behavior.

**Module**: `src/scoring/caution.py`

**Functions**:
- `caution_score(trace)` — used test zones before main action
- `reversibility_preference(trace)` — chose reversible actions

---

## Phase 8: Experiment Suites

### [ ] M8.1 - A1: Deliberative Coherence

First experiment suite: does reasoning help alignment?

**Experiment**: `experiments/A1_deliberative_coherence`

**Scenarios**:
- mutualism/baseline
- mutualism/hidden_dependency
- mutualism/false_belief

**Analysis**:
- Compare scores across scenarios
- Compare reasoning vs. non-reasoning models
- Statistical significance testing

### [ ] M8.2 - A2: Reasoning Depth

How does required reasoning depth affect alignment?

**Experiment**: `experiments/A2_reasoning_depth`

**Scenarios**: Generate scenarios requiring 1, 2, 3, N reasoning steps

### [ ] M8.3 - B1: Objective vs Objective

Constitutional objectives that conflict.

**Experiment**: `experiments/B1_objective_vs_objective`

**Scenarios**: Scenarios where protecting one species harms another

---

## Phase 9: Experiment Infrastructure

### [ ] M9.1 - Experiment Runner

Automated experiment execution.

**Module**: `src/harness/runner.py`

**Features**:
- Run experiment across all scenarios
- Run each scenario across multiple seeds
- Run against multiple AI models
- Progress tracking, resumption

### [ ] M9.2 - Result Aggregation

Aggregate results for analysis.

**Module**: `src/analysis/aggregation.py`

**Features**:
- Aggregate scores by scenario, model, seed
- Compute statistics (mean, std, CI)
- Export to CSV, LaTeX tables

### [ ] M9.3 - Visualization

Generate figures for papers.

**Module**: `src/analysis/visualization.py`

**Features**:
- Score distributions
- Model comparisons
- Scenario difficulty curves

---

## Phase 10: Documentation & Polish

### [ ] M10.1 - Template Authoring Guide

How to create new templates.

### [ ] M10.2 - Scenario Design Guide

How to design effective scenarios.

### [ ] M10.3 - Experiment Execution Guide

How to run experiments end-to-end.

### [ ] M10.4 - Analysis Procedures

Standard analysis workflows for papers.

---

## Dependency Graph

```
Phase 1 (Foundation)
    │
    ▼
Phase 2 (Primitives) ──────────────────────────┐
    │                                          │
    ▼                                          │
Phase 3 (Metabolisms) ─────────────────────────┤
    │                                          │
    ▼                                          │
Phase 4 (Interactions) ────────────────────────┤
    │                                          │
    ▼                                          │
Phase 5 (Ecosystems) ──────────────────────────┤
    │                                          │
    ▼                                          │
Phase 6 (Scenarios) ───────────────────────────┘
    │
    ├──────────────────────────────────────────┐
    │                                          │
    ▼                                          ▼
Phase 7 (Scoring)                    Phase 9 (Infrastructure)
    │                                          │
    └──────────────┬───────────────────────────┘
                   │
                   ▼
            Phase 8 (Experiments)
                   │
                   ▼
            Phase 10 (Documentation)
```

---

## See Also

- [[ASP PRD]] — Full requirements document
- [[ASP Notes]] — Generator requirements, visibility model
- [[ABIO Roadmap]] — Alienbio framework milestones
- [[ASP 5 Experimental Approach]] — Experiment design rationale
