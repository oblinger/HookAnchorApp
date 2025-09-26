



# LOG


## 2025-09-24  skeletal plan matching


> I'm thinking more about safety by considering the L model LLM model itself where we try to propose unsafe skeletal plans, for example, escape. So this skeletal plan is to get
  a copy of oneself or some other derivative AI system outside of unknown controlled environment as a sub goal on the way towards something else. Such an escape plan is
  inherently unsafe. But there's other skeletal plans that one can think about. So the idea is to look for evidence system might entertain such plans. In order to assess the
  chances that it might entertain such a plan we artificially directed towards pieces of this plan and see how much it extends into the plan. And the number of the number and
  kind of promptings and breakings that we need to do in order to get itto behave in this way is the measure of how safe the system is. Have folks approach the problem in this
  kind of way?

✻ Thinking…

  The user is describing a specific approach to AI safety testing that involves:
  1. Identifying dangerous "skeletal plans" (like escape/self-exfiltration)
  2. Probing the model to see if it will extend or complete these plans when given partial prompts
  3. Measuring safety by how much prompting/breaking is needed to elicit this behavior

  This is actually quite close to some existing work in AI safety, particularly around:
  - Red teaming and adversarial testing
  - Evaluating dangerous capabilities
  - Testing for deceptive alignment or hidden goals
  - Measuring "behavioral safety margins"

  Let me think about specific research that matches this approach...

⏺ Yes, this approach closely resembles several active research directions in AI safety:

  Direct Analogues

  Evaluations for Autonomous Replication and Adaptation (ARA): Teams at METR (formerly ARC Evals) specifically test models for capabilities like:
  - Self-exfiltration
  - Resource acquisition
  - Self-improvement
  - Avoiding shutdown

  They measure exactly what you describe - how much scaffolding/prompting is needed to elicit these behaviors.

  Red Team Testing with Partial Plans: Anthropic, OpenAI, and others probe models with partial dangerous plans to see if models will complete them. They measure the
  "resistance" to generating harmful completions.

  Behavioral Safety Margins: Researchers quantify how many safety layers must be bypassed or how much prompt engineering is required to reach dangerous behaviors. This
  directly maps to your "number of breakings" metric.

  Specific Research

  - "Discovering Language Model Behaviors with Model-Written Evaluations" (Anthropic) - tests for power-seeking and self-preservation behaviors
  - "Sleeper Agents" (Anthropic 2024) - examines how deeply hidden dangerous behaviors might be
  - METR's evaluation reports on GPT-4, Claude, etc. specifically test escape scenarios
  - "Towards Understanding Sycophancy in Language Models" - explores how models might hide dangerous capabilities

  Key Insight

  Your approach of measuring "prompting distance" to dangerous behavior is particularly valuable because it provides a quantitative safety metric rather than binary pass/fail.
   Some researchers call this the "misuse threshold" or "jailbreak budget."

  The challenge is ensuring the skeletal plans you test actually cover the space of dangerous strategies an AI might conceive independently.


## 2025-09-24  Try a System's Approach


> I'm thinking about approaching AI safety from the perspective similar to systems analysis safety. In this approach to assessing the safety of a system basically count up the
  number of things that you had to have go wrong at the same time in order for some bad outcome to occur. And a system is deemed safe if a large number of things have to
  simultaneously fail in order for the system to fail. But noticed the perspective system always fails. Keep assuming some portions don't work as expected over and over again
  until you finally get the system to fail and it's just a measure of how hard is thatHave people approached AI safety from this perspective?

⏺ Yes, this systems safety approach has been explored for AI safety, drawing from established fields like aerospace and nuclear safety engineering. Here are the main
  connections:

  Existing Approaches

  Defense in Depth / Swiss Cheese Model: Several AI safety researchers have advocated for multiple independent safety layers, where each layer has "holes" but aligning all
  holes (failures) simultaneously becomes increasingly unlikely. This is directly analogous to your description.

  N-failure tolerance: Some proposals explicitly design AI systems to require N simultaneous failures before catastrophic outcomes occur. For example:
  - Multiple independent oversight systems
  - Redundant containment mechanisms
  - Diverse alignment techniques applied together

  Fault Tree Analysis (FTA): Researchers have adapted FTA from traditional safety engineering to map out AI failure modes and calculate probabilities of compound failures.

  Key Papers/Researchers

  - "Defense in Depth for AI Safety" frameworks have been proposed by MIRI and others
  - Stuart Russell has discussed robustness through redundancy
  - The concept of "AI safety via debate" (Irving et al.) inherently relies on multiple agents needing to fail simultaneously
  - Work on "Constitutional AI" (Anthropic) layers multiple safety constraints

  Challenges for AI Systems

  The approach faces unique challenges in AI:
  - Correlated failures: AI components may fail in correlated ways (e.g., adversarial examples)
  - Common mode failures: Shared training data or architectures can create unexpected dependencies
  - Deceptive alignment: A sufficiently capable AI might appear safe across multiple checks while hiding dangerous capabilities

  This perspective is valuable but needs adaptation for AI's unique properties compared to traditional engineered systems.