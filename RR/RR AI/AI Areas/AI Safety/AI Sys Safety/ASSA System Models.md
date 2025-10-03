# AI System Safety Analysis - System Models

## Reference Model
Modern LLM-based systems operate as stateless processors over a context window that accumulates conversation history.

**State:** S = [system_prompt, msg₁, msg₂, ..., msgₙ] where messages alternate user/assistant

**Transition:**
- S' = S + [user_input]
- LLM processes S' to generate output
- S'' = S' + [assistant_output]

**Key Properties:**
- API calls are stateless - full context resent each request
- Context window has fixed maximum size (4k-2M tokens depending on model)
- LLM output feeds back into context for next generation
- No persistent state in model parameters between calls

**Failure Modes:**
- Context overflow when conversation exceeds limit
- Information loss during context management
- Prompt injection affects entire context
- No distinction between instructions and data in state

## Context Management Strategies
When |S| approaches max_tokens, systems apply various strategies to manage the size of the context window:

**Simple Accumulation**
- Append all user + assistant messages until limit reached
- Works for short conversations only
- Eventually fails when context fills

**Sliding Window**
- Keep most recent N tokens, drop oldest
- Fixed memory and cost per response
- Enables indefinite conversations but loses history
- Used by ChatGPT (4k-128k context depending on model)

**Summarization**
- Periodically compress old messages into summary
- Maintain recent messages verbatim
- Structure: [summary(old)] + [recent messages]
- Claude synthesizes history every 24 hours
- Trades accuracy for capacity

**Truncation Strategies**
- Drop middle, keep start + recent end
- Remove least relevant messages by similarity
- Keep only last N conversation turns
- Application-specific heuristics

**External Memory**
- Store full history in database
- Send only relevant context slice to LLM
- Use retrieval (RAG) to pull relevant past context
- Separates storage from inference context

## Related System Modeling Approaches

Theoretical models are used to represent and analyze modern AI reasoning systems for safety analysis.

### Markov Decision Process (MDP) / Partially Observable MDP (POMDP)
Models AI agents as sequential decision-making systems with states, actions, and policies. The LLM itself acts as the policy π that determines actions based on observations.
- Can be "unrolled" into workflow DAGs and finite state machines
- POMDP variant handles cases where agent cannot directly observe underlying state
- Agent operates in perception-reasoning-action loop

**Relevance to ASSA:** Enables formal analysis of decision sequences and state transitions. Failure modes can be analyzed as transitions to unsafe states or incorrect policy decisions.

**Reference:** ["Agents Are Workflows"](https://leehanchung.github.io/blogs/2025/05/09/agent-is-workflow/) (Lee Hanchung, 2025) - demonstrates how LLM agents map to MDPs using Bellman's Equation

### Compositional Component Model
Decomposes AI systems into interacting components: reasoning module, memory systems, tools/actions, and perception.
- Safety guarantees established on individual components
- System safety emerges from component interactions
- Supports hierarchical decomposition of complex systems
- Proofs of safety constructed compositionally

**Relevance to ASSA:** Most directly aligned with ASSA approach. Enables systematic identification of which component combinations must fail to cause system failure. Supports mapping failure trees where components are nodes.

**Reference:** ["Towards Guaranteed Safe AI: A Framework for Ensuring Robust and Reliable AI Systems"](https://arxiv.org/pdf/2405.06624) (arXiv 2405.06624)

### Graph-of-Thought (GoT) / Computational Graph
Represents reasoning process as a directed graph where nodes are reasoning states and edges are transitions between states.
- Captures branching, backtracking, and parallel reasoning paths
- Extends beyond linear chain-of-thought
- Reveals dependencies between reasoning steps
- Enables analysis of reasoning structure

**Relevance to ASSA:** Allows mapping of reasoning failure propagation. Can identify critical nodes whose failure cascades to system failure. Supports analysis of "how many reasoning steps must fail" questions.

**Reference:** ["Beyond Chain-of-Thought, Effective Graph-of-Thought Reasoning in Language Models"](https://arxiv.org/pdf/2305.16582) (arXiv 2305.16582)

### Dual Process Theory (System 1/System 2)
Distinguishes between two modes of cognition: System 1 (fast, automatic, pattern-matching, reactive) and System 2 (slow, deliberate, logical, reflective reasoning).
- Current LLMs primarily exhibit System 1 behavior
- Advanced models (e.g., OpenAI o1) incorporate "thinking time" for System 2
- Different failure modes for each system type
- System 2 reasoning uses reinforcement learning and chain-of-thought

**Relevance to ASSA:** Categorizes failure modes by reasoning type. System 1 failures involve incorrect pattern matching; System 2 failures involve flawed deliberation. Some safety violations may require failures in both systems.

### Layered Safety Architecture
Models safety as multiple defensive layers stacked in depth, each providing partial protection.
- Layers include: input filters, reasoning constraints, output filters, monitoring systems
- Each layer has specific failure modes and coverage gaps
- "Defense in depth" principle from security engineering
- System fails when attack/failure penetrates all layers

**Relevance to ASSA:** This IS the core ASSA model. The fundamental question is: how many layers must fail simultaneously for catastrophic outcome? Each layer is a component in fault tree analysis.

**References:**
- [OpenAI Preparedness Framework](https://cdn.openai.com/pdf/18a02b5d-6b67-4cec-ab64-68cdfbddebcd/preparedness-framework-v2.pdf)
- ["Defense in Depth: An Action Plan to Increase the Safety and Security of Advanced AI"](https://www.ibiblio.org/weidai/Gladstone%20Action%20Plan.pdf) (Gladstone AI, 2024)
- ["AI Control: Improving Safety Despite Intentional Subversion"](https://arxiv.org/pdf/2312.06942) (arXiv 2312.06942)

### Agent Workflow Model
Represents AI agents as workflows with defined states, transitions, and decision points.
- Agents modeled as finite state machines
- Explicit representation of tool use, memory access, and reasoning steps
- Sequential and conditional execution paths
- Observable state transitions

**Relevance to ASSA:** Provides concrete operational model for analyzing failure propagation through agent workflows. Can identify critical decision points where failures have cascading effects.

### Model Selection Guidance

**For ASSA purposes, we recommend:**

1. **Primary Model: Compositional Component Model**
   - Best alignment with FTA/FMEA methodology
   - Natural mapping to component failures and system-level effects
   - Supports systematic enumeration of failure combinations

2. **Secondary Model: Layered Safety Architecture**
   - Directly captures defense-in-depth concept
   - Clear mapping to "N failures required" questions
   - Well-understood in traditional safety engineering

3. **Supporting Models:**
   - MDP/POMDP for sequential decision analysis
   - Graph-of-Thought for reasoning failure propagation
   - Dual Process Theory for categorizing failure types
