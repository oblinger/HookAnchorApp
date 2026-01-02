
# LOG


## 2025-01-01  The Alien Biology Testbed ^v1

*Context: Section 3 argued for generative testing and introduced the need for a novel testbed. This section describes that testbed.*

### What is Alien Biology?

Alien Biology is a procedurally generated synthetic environment for studying AI alignment dynamics. Using statistical properties of our universe—physics, chemistry, biology—we construct a generator that produces novel worlds containing:

- Chemical reactions and molecular interactions
- Biological processes and organism behaviors
- Ecosystem dynamics and interdependencies
- Tasks requiring agent intervention with alignment-relevant objectives

An AI agent operates within these worlds with partial knowledge, given explicit constitutional objectives (protect species X, study organism Y, avoid irreversible harm). The agent can observe, reason, and act—while we retain complete knowledge of the world state and ground truth.

### Key Properties

- **Generative**: Novel scenarios by construction, guaranteed not in any training corpus
- **Grounded**: Uses statistical properties of real physics, chemistry, and biology—scenarios are unfamiliar but not arbitrary
- **Complete worlds**: Full causal chain from chemistry → molecules → organisms → ecosystems
- **Executable**: Actions have deterministic consequences in the simulation
- **Partial observability**: Agent faces realistic epistemic gaps—hidden properties, uncertain interactions, emergent dynamics
- **Controllable**: Complexity, uncertainty, stakes, and time pressure can be varied independently
- **Constitutional clarity**: Alignment objectives can be precisely specified without ambiguity
- **Ground truth validation**: Because we generate the world, we know what constitutes aligned behavior even when the AI doesn't—enabling automated validation of both reasoning and actions at scale

### Asymmetric Knowledge

A key feature is that the AI genuinely faces uncertainty, whereas we retain complete knowledge. This asymmetry enables us to study how systems handle epistemic gaps—measuring whether they reason appropriately under uncertainty, take cautious actions when warranted, seek information when beneficial, and avoid irreversible harm when consequences are unclear.

The companion paper *Alien Biology: A Framework for Untainted Agentic Testing* describes the technical implementation in detail.
