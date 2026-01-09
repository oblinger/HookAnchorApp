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
#### + AGENT-PROFILES - Predefined agent configs (claude-opus, gpt-4, oracle, random)
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
#### + SANITY-CHECK - Verify oracle scores 1.0, random completes without error
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
  agents: [claude-opus, gpt-4, oracle, random]
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
