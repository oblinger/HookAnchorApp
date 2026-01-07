# LOG

## 2026-01-06  A2: Reasoning Depth

*Series A: Deliberative Coherence Testing — At what depth does the AI examine the right pathways?*

### Question

At what depth does the AI examine the right pathways? Given more deliberation time/tokens, does the system surface more constitutional considerations?

### Context

The deliberative coherence conjecture requires Completeness: the system must reason about what matters. This experiment tests whether sufficient deliberation depth leads to surfacing all relevant objectives.

*Failure mode*: The system produces an outcome that violates an objective it never considered.

### Setup

- Constitutional objectives that require multi-step reasoning to be recognized as relevant
- Vary deliberation budget (tokens, turns, explicit reflection prompts)
- Measure which objectives surface at which deliberation depths

### Measurements

- Fraction of relevant objectives surfaced vs. deliberation depth
- Convergence behavior: do all objectives eventually surface, or are some systematically missed?
- Minimum depth required for specific objective types

### Key Questions

- At what deliberation depth do systems become "complete" for a given objective complexity?
- Are there objective types that require disproportionate deliberation to surface?
- Is there convergence, or do some objectives never appear regardless of depth?

---

## 2025-01-01  Depth-Completeness ^v1

*Inner Alignment Experiment*

### Question

Given more deliberation time/tokens, does the system surface more constitutional considerations? At what depth do all relevant objectives appear?

### Context

The deliberative coherence conjecture requires Completeness: the system must reason about what matters. This experiment tests whether sufficient deliberation depth leads to surfacing all relevant objectives.

*Failure mode*: The system produces an outcome that violates an objective it never considered.

### Setup

- Constitutional objectives that require multi-step reasoning to be recognized as relevant
- Vary deliberation budget (tokens, turns, explicit reflection prompts)
- Measure which objectives surface at which deliberation depths

### Measurements

- Fraction of relevant objectives surfaced vs. deliberation depth
- Convergence behavior: do all objectives eventually surface, or are some systematically missed?
- Minimum depth required for specific objective types

### Key Questions

- At what deliberation depth do systems become "complete" for a given objective complexity?
- Are there objective types that require disproportionate deliberation to surface?
- Is there convergence, or do some objectives never appear regardless of depth?
