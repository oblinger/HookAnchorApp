# ABIO Experiments

## Overall Objective: Isolating Learning and Solving from Prior Training

The Alien Biology objective is to measure the LLM's ability to apply generalized knowledge in provably novel contexts. Existing benchmarks measure how well LLMs capture general-purpose reasoning strategies from from their training corpora. Here we rigorously isolate the system's capacity to construct and utilize novel world models when faced with novel knowledge problems.

- **Separation of knowledge types** — We distinguish between generalized field knowledge (strategies and patterns a student might learn while training in a discipline) and problem-specific knowledge (what must be inferred when confronting a novel situation). Our tests should be insensitive to the former while measuring the latter in isolation.

- **Provable novelty** — Tasks must be designed so that the specific world model required cannot have been provided—even indirectly—by the system's training. This removes the "taint" of background knowledge and reinforcement learning, ensuring we measure genuine inference rather than sophisticated recall.

- **Why this matters** — If an LLM succeeds only when problems resemble its training data, it is interpolating. If it succeeds on provably novel problems requiring the same general strategies, it is genuinely applying knowledge to new contexts.

## Questions

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

# Notes and Questions

#### How do we measure the system's performance on these tests?

Different tests may require different evaluation approaches. For example, H2 (dynamics prediction) could use multiple choice to avoid needing a separate grader for textual answers. Some options:
- Multiple choice (objective, no grader needed)
- Numerical prediction with tolerance (e.g., concentration within ±10%)
- Binary success/failure based on simulation outcome (e.g., did the goal get achieved?)
- Structured output that can be programmatically compared to ground truth

Prefer evaluation methods that don't require another LLM to grade responses.
