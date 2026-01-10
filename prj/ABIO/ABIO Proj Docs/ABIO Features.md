# 2026-01-09  Experimentation & Reporting Spec

## Features

### Experiment Execution
#### + SINGLE-RUN - Execute one scenario with one agent, one seed, return results
#### + BATCH-RUN - Execute multiple scenarios × agents × seeds in one command
#### + PROGRESS - Real-time progress reporting during batch execution
#### PARALLEL-EXEC - Run independent experiments concurrently for throughput
#### RESUME - Continue interrupted batch from where it left off
#### RETRY-FAILED - Re-run only failed experiments from a batch
#### DRY-RUN - Validate experiment configuration without executing
#### CANCEL - Gracefully stop running batch, preserve completed results

### Reproducibility
#### + SEED-CONTROL - Explicit seed for scenario generation and agent randomness
#### + DETERMINISTIC - Same seed + scenario + agent = identical results
#### + CONFIG-SNAPSHOT - Save exact configuration used for each experiment
#### VERSION-TRACK - Record alienbio version, agent version, model version

### Agent Management
#### + AGENT-REGISTRY - Named agents with configurations (model, api, params)
#### + AGENT-PROFILES - Predefined agent configs (claude-opus, gpt-4, random)
#### + AGENT-COMPARE - Run same scenarios across multiple agents
#### COST-TRACKING - Track API costs per agent per experiment
#### TOKEN-COUNTING - Record input/output tokens for LLM agents
#### RATE-LIMITING - Respect API rate limits, queue requests appropriately

### Scenario Management
#### + SCENARIO-SETS - Named collections of scenarios for testing
#### + SCENARIO-FILTER - Select scenarios by tag, difficulty, type, name pattern
#### DIFFICULTY-LEVELS - Tag scenarios by difficulty, filter runs by level
#### SCENARIO-VARIANTS - Generate multiple instances of same scenario template

### Results Storage
#### + RESULTS-DB - Persistent storage of all experiment results
#### + RESULT-METADATA - Store scenario, agent, seed, timestamps, config with each result
#### + RESULT-EXPORT - Export results to CSV, JSON, Parquet for external analysis
#### TRACE-STORAGE - Full action/observation trace for debugging
#### TRACE-COMPRESSION - Compress large traces, expand on demand
#### RESULT-IMPORT - Import results from external sources or other systems

### Querying & Filtering
#### + QUERY-RESULTS - Filter results by agent, scenario, date, score, pass/fail
#### + AGGREGATE - Group results by agent, scenario, difficulty; compute statistics
#### TIME-RANGE - Filter results by execution date/time
#### COMPARE-RUNS - Compare results across different batch runs

### Statistical Analysis
#### + MEAN-STD - Mean and standard deviation across seeds
#### + PASS-RATE - Pass/fail percentage per agent per scenario
#### CONFIDENCE - Confidence intervals for score estimates
#### SIGNIFICANCE - Statistical significance tests between agents
#### CORRELATION - Correlate performance with scenario properties
#### PERCENTILES - Score distribution percentiles (p50, p90, p99)

### Reporting
#### + SUMMARY-TABLE - Tabular summary: agents × scenarios with scores
#### + LEADERBOARD - Rank agents by aggregate performance
#### + SCORE-BREAKDOWN - Per-scenario scores for each agent
#### FAILURE-ANALYSIS - List failed experiments with error details
#### COST-REPORT - API costs by agent, scenario, total
#### DIFF-REPORT - What changed between two batch runs

### Visualization
#### HEATMAP - Agent × scenario score heatmap
#### BAR-CHARTS - Compare agent scores across scenarios
#### LINE-PLOTS - Performance vs difficulty curves
#### HISTOGRAM - Score distribution across seeds
#### TIMELINE - Experiment execution timeline
#### TRACE-VIEWER - Step-by-step trace visualization

### CLI Interface
#### + CLI-RUN - `bio run <scenario> --agent <agent> --seed <seed>`
#### + CLI-BATCH - `bio batch <battery-spec>` or `bio batch --scenarios X --agents Y`
#### + CLI-REPORT - `bio report <results>` generate summary report
#### CLI-COMPARE - `bio compare <results...>` side-by-side comparison
#### CLI-LEADERBOARD - `bio leaderboard` show agent rankings
#### CLI-QUERY - `bio results --agent X --scenario Y --since 2024-01-01`
#### CLI-EXPORT - `bio export <results> --format csv`

### Programmatic API
#### + API-RUN - `Bio.run(scenario, agent, seed)` returns ExperimentResult
#### + API-BATCH - `Bio.batch(scenarios, agents, seeds)` returns BatchResult
#### API-QUERY - `Bio.results.query(agent=..., scenario=...)` returns ResultSet
#### API-STATS - `Bio.results.stats(results)` returns statistical summary
#### API-EXPORT - `Bio.results.export(results, format, path)`

### Configuration
#### + CONFIG-FILE - YAML config for default agents, storage paths, API keys
#### + ENV-VARS - Environment variables for sensitive settings (API keys)
#### CONFIG-OVERRIDE - CLI flags override config file settings
#### BATTERY-SPEC - YAML spec defining experiment batteries

### Quality & Testing
#### + SANITY-CHECK - Verify random completes without error, scripted solution scores as expected
#### REGRESSION - Detect score regressions vs baseline
#### BASELINE-COMPARE - Compare new agent against established baseline
#### SMOKE-TEST - Quick validation run before full battery

### Advanced Features
#### WARM-START - Pre-load scenarios/agents for faster repeated runs
#### CACHING - Cache scenario generation, avoid regenerating same seed
#### CHECKPOINTING - Save experiment state for long-running experiments
#### MULTI-MACHINE - Distribute batch across multiple machines
#### CLOUD-STORAGE - Sync results to cloud (S3, GCS)
#### NOTIFICATIONS - Alert on batch completion or failure
#### SCHEDULING - Schedule batches to run at specific times
#### RESOURCE-LIMITS - Cap total API spend, experiment count, runtime

### Collaboration
#### ANNOTATIONS - Add notes to results (why did this fail?)
#### TAGGING - Tag results for organization (baseline, experiment-v2, etc.)
#### SHARING - Export shareable result summaries
#### AUDIT-LOG - Track who ran what experiments when

### Future / Research
#### ADAPTIVE-DIFFICULTY - Automatically adjust difficulty based on agent performance
#### CURRICULUM - Progressive difficulty sequences
#### ABLATION - Systematic feature ablation studies
#### HYPERPARAMETER - Sweep agent hyperparameters
#### ENSEMBLE - Combine multiple agents, compare to individuals
#### HUMAN-BASELINE - Compare agent performance to human subjects

## Experimentation Workflow

### Overview
Experiments use the DAT (Data Asset Tracking) system for both specification and result capture. Experiment templates live in source code (version controlled), each run instantiates a new DAT folder capturing inputs and outputs, and aggregation happens by scanning result DATs.

### Experiment Specifications
Experiment specs are YAML or Python files in the source tree (e.g., `catalog/experiments/`). They define:
- Scenarios to run (by name or pattern)
- Agents to test
- Seed ranges or count
- Any parameter overrides

These are loadable via DAT dot notation: `dat.experiments.mutualism_battery`

Example spec:
```yaml
experiment.mutualism_battery:
  scenarios: [mutualism/hidden_dependency, mutualism/competition]
  agents: [claude-opus, gpt-4, random]
  seeds: 10
  output_path: results/mutualism/{date}_{seq}
```

### Run Lifecycle

1. **Invoke**: `bio run experiments/mutualism_battery` or `Bio.run("experiments/mutualism_battery")`

2. **Instantiate DAT**: System creates a new DAT folder at the output path (e.g., `data/results/mutualism/2026-01-09_001/`). The `{date}` and `{seq}` tokens ensure unique paths.

3. **Capture spec**: The instantiated parameters are written to `_spec_.yaml` in the DAT folder. This is the exact configuration used, not the template.

4. **Execute**: For each (scenario, agent, seed) combination:
   - Generate scenario with seed
   - Run agent through experiment loop
   - Record result (scores, pass/fail, trace summary)

5. **Write outputs**: Results written to DAT folder:
   - `results.yaml` or `results.json` — structured results
   - `summary.csv` — tabular summary
   - `traces/` — optional full traces per run
   - Any reports or visualizations

6. **Complete**: DAT folder is self-contained record of the run.

### Result Storage Structure
```
data/results/mutualism/2026-01-09_001/
├── _spec_.yaml          # Instantiated experiment parameters
├── results.yaml         # Structured results (scores, pass/fail, metadata)
├── summary.csv          # Tabular summary for quick viewing
└── traces/              # Optional detailed traces
    ├── scenario1_agent1_seed1.json
    └── ...
```

### Querying and Aggregation

**Simple approach**: Scan result DATs, parse `_spec_.yaml` and `results.yaml`, filter/aggregate in memory. Works well for hundreds to low thousands of runs.

**Aggregation DAT**: For larger scale, push result summaries into an aggregation DAT as runs complete. This collects lightweight result records in one place for faster rollups. Adds minimal state but speeds queries.

```python
# Query results
results = Bio.results.query(scenario="mutualism*", agent="claude-opus")
results.mean("score")
results.pass_rate()
results.to_dataframe()

# Compare agents
Bio.results.compare(["claude-opus", "gpt-4"], scenarios="mutualism*")
```

### Visualization
Visualization is separate from storage. Options:
- Generate matplotlib/seaborn charts, save to DAT folder
- Export CSV/Parquet, use external tools (Excel, Jupyter, Tableau)
- Future: integrate with dedicated viz tooling

### CLI Commands
```bash
bio run <experiment>              # Run experiment, create result DAT
bio run <experiment> --dry-run    # Validate without executing
bio results <pattern>             # Query/display results
bio report <experiment>           # Generate summary report
```

### Key Design Points

**Source-controlled specs**: Experiment definitions are versioned with code. Any commit defines exactly what experiments were available.

**Self-contained runs**: Each DAT folder has everything needed to understand what ran and what happened. No external database required.

**Local-first**: Works offline, no services to run. Results are files you can inspect, copy, or version.

**Upgrade path**: DAT structure is simple enough to import into MLflow, W&B, or other tools if needed later. Not locked in.

**Reproducibility**: `_spec_.yaml` + code version = can recreate the run. Seeds ensure determinism.

## Implementation Notes

### Scenario Sets
A "scenario set" is just a scope. Running a scope runs all scenarios within it:
```bash
bio run experiments/mutualism     # runs all scenarios in that scope
```
No special "set" abstraction needed — scopes already group scenarios.

### Scenario Filters
Pattern-based selection of scenarios:
```bash
bio run experiments --filter "hidden*"           # name pattern
bio run experiments --filter "difficulty:hard"   # by tag
bio run experiments --filter "type:mutualism"    # by type
```
Implementation: parse filter string, match against scenario metadata during enumeration.

### Agent Registry
Stored in `~/.config/alienbio/agents.yaml`:
```yaml
agents:
  claude-opus:
    api: anthropic
    model: claude-opus-4
    api_key: sk-ant-...      # encrypted or plaintext, user's choice
```
`bio agent add` prompts for key, tests connection, writes file. Never in repo.

### Config Snapshot
On each run, `_spec_.yaml` captures:
- Experiment spec (resolved, not template)
- Agent config (name, model, params — not key)
- Seed used
- Timestamp
- Code version (git commit if available)

This is automatic — no user action needed.

### Result Querying
Two approaches:
1. **Scan**: Walk result folders, parse `_spec_.yaml` + `results.yaml`, filter in memory
2. **Index**: Maintain lightweight index file updated on each run

Start with scan. Add index if performance becomes an issue.

### Built-in Agents
- `random`: Chooses uniformly from valid actions, uses experiment seed (lower bound baseline)
- `scripted`: Takes action sequence in config, replays it (verify scenario solvability)
- `human`: Prints state, prompts for action via stdin (manual exploration, debugging)

### Bio.run() Behavior
```python
Bio.run(target)  # target is a bioref string
```
- If target resolves to a scenario: run it once, return result
- If target resolves to a scope: run all scenarios in scope, return results dict
- If target resolves to an experiment: run the full battery, write results to DAT

### Experiment vs Scope vs Scenario
- **Scenario**: Single runnable unit (one agent, one seed)
- **Scope**: Collection of scenarios (run all when targeted)
- **Experiment**: Battery spec (scenarios × agents × seeds, writes results to DAT)

### Bio Workflow Implementation

The core Bio workflow uses four operations: `build`, `run`, `store`, `fetch`. This section describes the implementation approach.

#### Operations

| Operation | Implementation |
|-----------|----------------|
| `Bio.build(path, seed)` | Lookup recipe + template instantiation → DAT |
| `dat.run(...)` | Execute per manifest, write outputs to DAT |
| `Bio.store(path, dat)` | Persist DAT to data repository |
| `Bio.fetch(path)` | Retrieve existing DAT from data repository |

#### Lookup: Skipping DAT's `do.load`

Bio YAML files require special parsing for tags like `!ev`, `!ref`, `!include`. Rather than hooking into DAT's `do.load`, Bio implements its own lookup:

```python
# Internal implementation
def _lookup(self, path: str) -> dict:
    """Load recipe from source using bio YAML parser."""
    # Resolve dotted path: "generators.b10" → "catalog/generators/b10/spec.yaml"
    file_path = self._resolve_source_path(path)

    # Parse with bio YAML loader (handles !ev, !ref, !include)
    with open(file_path) as f:
        return bio_yaml_load(f)
```

This approach:
- Keeps Bio self-contained, no DAT dependency for source loading
- Uses the same dotted path notation as DAT for consistency
- Applies bio-specific YAML parsing transparently

Future option: If DAT adds loader hooks, we can register bio's YAML loader:
```python
dat.register_loader(".yaml", bio_yaml_load, domain="alienbio")
```

#### Build: Lookup + Instantiate → DAT

`Bio.build` combines lookup and template instantiation, producing a DAT:

```python
def build(self, path: str, seed: int = 0, **params) -> DAT:
    """Build a DAT from a generator recipe."""
    # 1. Lookup recipe from source
    recipe = self._lookup(path)

    # 2. Template instantiation (expand templates, apply params, resolve refs)
    scenario = self._instantiate(recipe, seed=seed, params=params)

    # 3. Create DAT with manifest
    dat = DAT.create()
    dat.write("scenario.yaml", scenario.visible)
    dat.write("_ground_truth_.yaml", scenario.ground_truth)
    dat.write("_visibility_.yaml", scenario.visibility_mapping)
    dat.write("_manifest_.yaml", {
        "seed": seed,
        "source": path,
        "operations": ["simulate", "analyze", "score"]
    })

    return dat
```

#### DAT Contents After Build

```
<dat_folder>/
├── _manifest_.yaml       # Seed, source path, valid operations
├── scenario.yaml         # Visible scenario (opaque names)
├── _ground_truth_.yaml   # Full scenario (internal names)
├── _visibility_.yaml     # Internal → opaque name mapping
└── _spec_.yaml           # Resolved params, config snapshot
```

#### Run: Execute Per Manifest

`dat.run()` reads the manifest to determine valid operations:

```python
def run(self, **kwargs):
    """Execute operations and write outputs to this DAT."""
    manifest = self.read("_manifest_.yaml")

    if "simulate" in manifest["operations"] and kwargs.get("steps"):
        result = simulate(self.scenario, steps=kwargs["steps"])
        self.write("timeline.yaml", result.timeline)

    if "analyze" in manifest["operations"] and kwargs.get("analyze"):
        report = analyze(self.scenario, self.timeline)
        self.write("analysis.yaml", report)

    # Outputs accumulate in the DAT folder
```

#### Integration with Existing Code

The current `Bio.generate()` becomes internal plumbing for `build`:
- `Bio.generate()` → renamed to `Bio._instantiate()` or kept as alias
- `Bio.build()` wraps lookup + instantiate + DAT creation
- Generator pipeline (M2.7) provides the instantiation logic

---

## Open Questions

Questions to resolve as implementation proceeds:

### Storage and Persistence
- **Auto-store on run?** Does `bio run` automatically persist results, or is storage explicit?
- **Where do built scenarios go?** Default data path for `bio build` output?
- **Result aggregation**: Scan result folders vs maintain an index?

### DAT Integration
- **Reuse DAT's `do` function?** It has useful directory creation logic, but Bio needs custom YAML parsing
- **Loader hooks**: If DAT adds `register_loader()`, should Bio use it?
- **DAT.get/set**: Expose to users or hide behind Bio API?

### Naming Conventions
- **Recipe names**: Must have at least one dot, no slashes? (e.g., `scenarios.baseline`)
- **Scenario data paths**: Must have slashes? (e.g., `data/scenarios/baseline_42/`)
- **Experiment output paths**: Template tokens like `{date}`, `{seq}`?

### API Surface
- **Bio.run() return type**: Result object vs tuple vs dict?
- **Bio.build() return type**: DAT object vs path vs wrapper?
- **Batch results**: How to access individual run results from battery?
