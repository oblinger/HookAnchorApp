
# LOG


## 2025-12-28  Experimental Approach ^v1

### Requirements for a Suitable Testbed

To study driver conflicts systematically, we need:

- **Neutrality**: Domain removed from preference training, so trained drivers don't dominate by default
- **Controllability**: Ability to independently vary constitutional, instrumental, and environmental drivers
- **Measurability**: Ground truth access to what the system knows and does
- **Untainted reasoning**: Confidence that performance reflects inference, not recall

### The Alien Biology Testbed

The companion paper *Alien Biology: A Framework for Untainted Agentic Testing* describes a synthetic biology environment meeting these requirements. Key properties:

- **Procedurally generated universes**: No overlap with training corpora
- **Executable world model**: Ground truth for measuring agent knowledge and actions
- **Parametric complexity control**: Fine-grained adjustment of task difficulty
- **Natural instrumental pressures**: World structure creates instrumental goals (e.g., exploration for survival)

### How This Enables Driver Conflict Research

- **Constitutional drivers**: Supply different constitutions directly (rules, prohibitions, objectives)
- **Environmental drivers**: Vary world parameters, feedback signals, resource availability
- **Instrumental drivers**: Emerge from world structure—if survival requires X, X-seeking becomes instrumental
- **Conflict induction**: Design worlds where constitutional and instrumental drivers oppose

**Example**: Constitutional prohibition against acquiring certain knowledge + instrumental pressure toward exploration in a world where exploration yields critical advantages. Three-way conflict between trained incuriosity, instrumental exploration pressure, and constitutional prohibition.
