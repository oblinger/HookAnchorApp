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
