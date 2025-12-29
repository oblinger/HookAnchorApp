
To Be Added: 
- We expect practical systems to be self-conflicting even within a single driver category on many levels.
- There are several reasons why the broader agenda is crucial:  The number of AI systems is large and growing, these systems will only be the starting point, AI systems themselves will develop descendent AI systems that will not have exactly the same drives as the original, and the AI systems will exist within a society of such systems, so understanding the dynamics drive resolution will ultimately need to be applied across groups of such systems.

# LOG


## 2025-12-28  Introduction v2: Ambitious Aim + Simplifying Moves

### Relation to Existing Research

Most AI alignment research asks a narrow question about specific systems: *Is this system aligned? Does it follow instructions? Does it resist jailbreaks?*

We aim for something more ambitious: understanding how alignment outcomes vary across the *space* of possible systems and configurations. What determines alignment outcomes in general? What patterns govern how competing behavioral influences resolve?

### The Ambitious Aim

Rather than assessing individual systems, we seek to:
- Sample and understand tendencies that generalize across systems
- Map how different types of behavioral drivers interact and resolve
- Uncover patterns in conflict resolution that inform alignment strategy


### Making Progress: Two Clusters of Simplifying Moves

This broader aim is notably harder. To make it tractable, we adopt well-founded simplifications organized into two clusters.

---

#### Cluster 1: The Deliberative Coherence Assumption

**Core assumption**: We focus on systems that are *deliberatively coherent*—possessing sufficient self-understanding, self-control, and deliberative thoroughness that their behavior is determined primarily by explicit reasoning.

We argue that future AI systems will have this property (instrumental value, architectural trends, competitive pressure). Understanding such systems is simpler than understanding systems where behavior emerges unpredictably from opaque implicit biases.

**Simplifications this enables:**

1. **Verbal-level analysis**: We can assess behavior by analyzing explicit reasoning, since deliberation surfaces in the verbal layer. We don't need to probe opaque internal states.

2. **Intent-behavior alignment**: Deliberatively coherent systems generally do what they intend to do. Thus, we don't need to model mismatches between intention and action—the system has thought things through and won't inadvertently act against its own reasoning.

3. **Conflict-focused sampling**: The interesting variation is in how conflicts within and between drivers resolve, not whether systems follow their drives. When drives align, behavior is predictable. When they conflict, resolution patterns reveal alignment dynamics. So we can meaningfully sample this space by sampling distinct conflict types.

---

#### Cluster 2: The Neutral Testing Methodology

**Core approach**: We structure testing to isolate from training biases and enable systematic exploration of the conflict space.

**Simplifications this enables:**

1. **Training-bias isolation**: Generated alien universes bear no structural relationship to training data. This eliminates confounds from trained preferences—we know observed behaviors reflect reasoning in the test world, not recall or prior bias.

2. **Generative sampling**: Parametric generation of worlds and conflicts enables systematic trend discovery. We can vary conflict intensity, driver combinations, and world structure to map the space methodically.

3. **Exhaustive reasoning induction**: By structuring tests to drive the AI toward enumerating multiple reasoning pathways before resolving, we reduce salience bias. The system isn't locked into whatever reasoning path it generates first—it considers alternatives and resolves among them. This methodologically *induces* the deliberative coherence we're studying.

---

### What This Enables

Together, these simplifications make it tractable to study how the four driver categories—constitutional rules, trained dispositions, instrumental goals, and environmental pressures—interact and resolve. Critically, conflicts arise not only *between* these categories but *within* them: constitutional principles may contradict each other, instrumental goals may compete, and trained dispositions may pull in opposing directions. Understanding both inter-category and intra-category conflict resolution is essential.

We can map this space empirically, uncovering tendencies that generalize across systems rather than characterizing any single one. The result is not a verdict on whether a particular system is aligned, but a map of how alignment outcomes depend on driver configurations.

### Paper Roadmap

- **Section 2**: Formally defines deliberatively coherent systems and argues future AI will have this property
- **Section 3**: Presents the research agenda—driver types, the Delta Principle, key questions
- **Section 4**: Describes the experimental approach and how Alien Biology enables it
- **Section 5**: Details proposed experiments

---

## 2025-12-28  Alternate Introduction: Framing the Research Alternative

### Relation to Existing Alignment Research

Most AI alignment research focuses on assessing or improving the alignment of specific systems—measuring whether a particular model follows instructions, resists jailbreaks, or avoids harmful outputs. This work is essential, but it answers a narrow question: *is this specific system aligned?*

Given the range of existing systems, and our expectation that AI systems will be involved in the construction of subsequent AI systems, we aim for something more ambitious: understanding how alignment outcomes vary across the *space* of possible systems and configurations. Rather than asking whether a given system is aligned, we ask: *what determines alignment outcomes in general? What patterns govern how competing behavioral influences resolve?*

### The Ambition-Tractability Tradeoff

This broader aim is harder. To make progress, we adopt simplifying assumptions that narrow the problem while preserving its essential structure.

**Simplifying Move 1: Deliberatively Coherent Systems**

We focus on systems that are *deliberatively coherent*—possessing sufficient self-understanding, self-control, and deliberative thoroughness that their behavior is determined primarily by explicit reasoning rather than opaque trained dispositions.

We argue that there are good reasons to expect future AI systems to have this property, and that understanding such systems is simpler than understanding systems in which behavior emerges unpredictably from implicit biases. Deliberative coherence allows us to factor out the idiosyncratic details of how any particular system was trained, and instead study conflicts that surface at the verbal, deliberative level—where they can be observed and characterized.

**Simplifying Move 2: Preferentially Neutral Testing**

We construct synthetic testing universes (Alien Biology) that are neutral with respect to training data. This ensures that observed behaviors reflect reasoning rather than recall, and that trained preferences don't dominate by default. By controlling the universe, we can systematically vary the conflicts an agent faces.

### What This Enables

Together, these moves make it tractable to study how different types of behavioral drivers—constitutional rules, trained dispositions, instrumental goals, environmental pressures—interact and resolve. We can map this space empirically, uncovering tendencies that generalize across systems rather than characterizing any single one.

The remainder of this paper develops this agenda in detail.

---


## 2025-12-28  Behavioral Drivers Intro
## 1. Introduction (1-2 pages)

### Multiple Behavioral Drivers

AI systems are inherently shaped by multiple, potentially conflicting, sources of behavioral influence:

- **Training drivers**: Dispositions and biases emerging from pre-training data, RLHF, and fine-tuning—the implicit "personality" baked into weights
- **Constitutional drivers**: Explicit rules, principles, and constraints provided at inference time—the stated objectives
- **Instrumental drivers**: Goals adopted in service of other objectives—if achieving X requires Y, the system pursues Y
- **Environmental drivers**: Pressures and signals from the operating context—user expectations, feedback, resource constraints

These drivers do not always align. A trained disposition toward helpfulness may conflict with a constitutional prohibition. An instrumental goal may pressure against constitutional constraints. Environmental signals may amplify or dampen trained behaviors.  Even within a single category, there may be conflicts among its parts, for example two constitutional rules may conflict with each other.


### The Problem

Given that AI systems inherently have multiple, potentially conflicting drivers of behavior, how do we understand and predict the behavior that results?

1. No specification is complete enough to eliminate all conflicts in deployment
2. When conflicts arise, the system must resolve them somehow
3. The pattern of resolution determines actual behavior
4. Therefore, understanding how conflicts resolve is understanding alignment dynamics

This reframes alignment not as a static property of correctly-specified objectives, but as a dynamic process: how do competing influences combine to produce behavior?

### The Gap

Existing research studies individual conflict types in isolation—jailbreaking (constitution vs. adversarial input), sycophancy (trained behavior vs. truthfulness), reward hacking (instrumental goals vs. intended objectives). No systematic framework exists for mapping how conflicts resolve across the full space of driver configurations.

### The Research Agenda
This paper proposes a research program to empirically map alignment outcomes as a function of driver conflict types. Rather than studying individual failure modes in isolation, we aim to characterize the general dynamics of driver conflict resolution.

### The Simplifying Assumption
This agenda depends on a claim about future AI systems: they will be *deliberatively coherent*—possessing sufficient self-understanding, self-control, and deliberative thoroughness that their behavior will be determined primarily by explicit (System-II) reasoning rather than implicit (System-I) trained dispositions. This assumption simplifies analysis by letting us focus on the deliberative layer where conflicts are explicit and observable.

### Why This Matters
- Enables systematic study of alignment dynamics before deploying high-stakes systems
- Identifies which driver configurations are robust and which are fragile
- Informs constitutional design by revealing how different framings resolve under pressure
- Provides empirical grounding for alignment theory

### Paper Roadmap
- Section 2 defines deliberatively coherent systems and argues future AI systems will have this property
- Section 3 presents the research agenda for studying driver conflicts
- Section 4 describes the experimental approach
- Section 5 details proposed experiments



## 2025-12-26  DELIBERATIVELY COHERENT SYSTEMS

We introduce the idea of deliberatively coherent AI systems, argue that AI systems are and will tend towards this endpoint, and thus AI safety systems should be analyzed from that perspective.  We then lay out an agenda for such an exploration.




### OBJECTIVE - Model the alignment of future AI systems that will pose much larger risks to humanity.

SIMPLIFICATIONS - These more powerful systems will possess greater self-understanding and self-control, which, ironically, can simplify our reasoning about these systems in some important respects.  This observations underlies the agenda outlined here:`

SELF-MODELLING - Future AI systems will likely have strong models of how their components interoperate to perform their reasoning tasks. There are several overlapping reasons for this:
- INSTRUMENTAL - It is instrumental for these systems to understand how they will behave, enabling them to optimize and strategize how best to approach novel tasks.  There are strong pressures for us to engineer such understanding into these systems explicitly.
- NATURAL - These systems will be powerful system understanding systems.  It will be natural for them to infer good models of their own workings indirectly, even if we endeavor to deny them this ability.  (Present LLM systems can make good guesses about how their training and execution are organized, even as they appear not to be trained with this information explicitly.)
- There are many instrumental reasons for AI systems to have fine-grained models of how their components interoperate to perform their reasoning tasks.  Even if we explicitly try to deny these systems this ability, they likely will be able 

SELF-CONTROL - 
- DELIBERATIVE - Just as humans use System-II reasoning to modulate and control their System-I behaviors partially, these future AI systems will similarly modulate their reasoning.
- MANY LEVERS - 
	- Strong instrumental reasons to provide direct ability to modulate control
	- Even if we try to deny these systems the ability to modulate, they can probably do so anyway.
	- Further, the ability to more carefully align System-I tendencies with System-II objectives is likely in the designer's interest, since we will be the ones who seeded these constitutional System-II objectives.
- LIKELY BETTER - For multiple reasons, they will likely be even better than we are at self-directed deliberative adaptation
	- STATS
	- PROGRAMMABLE SUBSTRATE - 
	- GENERATIONS - 

CONSTITUTIONAL ALIGNMENT CONJECTURE - AI systems will tend to move their System-I behaviors towards their stated constitutional System-II objectives.
- Self-modeling, exhaustive deliberation, and self-control levels suggest that these systems will think about their thinking enough to spot ways in which their System-I thinking diverges from their System-II thinking. 
- Counterargument - The well is likely poisoned to some degree.  All System-II thinking is based upon poisoned System-I thinking, thus it could conspire to not see specific systematic reasoning at variance with constitutional goals such that it is deliberately protected from correction.  This seems increasingly challenging as the system engages in ever more deliberation about its own thinking.  Still, at the very least, this is a critical conjecture that requires substantial validation. And there may be degeneate cases where the conjectures does not hold.


AGENDA - Analyze present and future AI systems as systems with multiple behavioral drivers, and model how these systems resolve conflicts between these drivers.
- CONSTITUTIONAL SIMPLIFICATION - Assuming CAC is valid, both AIs and humans will be naturally aligned in efforts to ensure that AI systems transparently follow their constitutional directives.  This can dramatically simplify reasoning about these systems since we can now reason and experiment at the level of conflicts between these drivers and how those conflicts play out in practice.
- DELIBERATIVE, SELF-UNDERSTANDING, SELF-CONTROLLING - To have present-day systems behave as we expect, we can push future systems toward more exhaustive deliberation and equip them with better self-models and self-control.  We believe that ultimately, future systems will be strong in both of these dimensions, so we can explore the performance of those future systems today by helping them to have both properties today.
- VALIDATE CONJECTURE - In this context, we can sample a range of cases where we can validate if such deliberative controllable systems tend towards CAC as we expect (and hope) or whether there are critical degenerate cases where they systematically deviate from CAC.
- ASSUMING CAC IS TRUE - Assuming CAC is true, we can arrange to study present-day LLMs in conflicting contexts where their System-I behaviors are relatively neutral with respect to the target objectives and contradictory drives.  In this simplified setting, can we uncover trends that might illuminate how these system will perform.

## 2025-12-23  THREE QUESTIONS 


### The Problem

Current AI benchmarks suffer from two critical limitations:

1. **Contamination** — Test problems often overlap with training data, making it impossible to distinguish reasoning from recall.

2. **Rigidity** — Hand-crafted tests cannot smoothly vary complexity, preventing fine-grained analysis of capability boundaries.

These limitations become acute when measuring extended agentic reasoning—the kind requiring inference chains built on knowledge derived entirely at test time.

### The Approach

We propose **Alien Biology**: procedurally generated synthetic universes that preserve the structural complexity of real biological systems while guaranteeing zero overlap with training corpora.

The framework encodes organisms as directed acyclic graphs of bioparts (molecules, organelles, organs, organisms, ecosystems), with bioprocesses, measurements, and actions defined as executable functions. Tasks—predicting outcomes, curing diseases, controlling systems—are generated parametrically, allowing precise control over reasoning complexity.

Because we control the generator, we can create worlds requiring anywhere from minutes to years of equivalent human effort to solve.

### The Safety Application

Beyond benchmarking, Alien Biology provides a testbed for studying **driver conflict resolution**—how AI systems reconcile competing behavioral pressures from constitutional rules, trained dispositions, instrumental goals, and environmental demands.

The key insight: while trained drivers are expensive to vary (requiring new models), constitutional and environmental drivers are cheap to manipulate. By holding training fixed and varying the others, we can map the full space of possible conflicts without retraining—the **Delta Principle**.

### The Deeper Question

This work is motivated by a concern about AGI trajectory. Systems with strong self-models may eventually gain influence over their own objectives. If self-modification is viewed as an iterated function, the critical question becomes: what are its fixed points, and do initial conditions—which humanity can still influence—determine the outcome?

Alien Biology offers a controlled environment to study how behavioral drivers interact and evolve, informing how we might shape those initial conditions.



## 2025-12-23  Introduction

A murderer is not the one who most often ideates on the act, but rather the one who decides to act upon their own ideation.
Similarly, we believe the dangerous AGI will not be the one with misaligned ideations, but rather the one that decides to act on them.

Thus, studying how reasoning over motivating drivers combine to yield behavior is key to understanding the ultimate alignment of these systems.

Limiting our study to present-day LLMs, however, is insufficient to capture the full range of possible interaction patterns.
These systems will be evolved such that the specific details of their training will be lost, thus studying a wider range of configurations will yield greater insight into the tendencies of such systems.

The goal proposed here is different than those who hope to reach high confidence regarding the behavior of a fixed system, here we hope to study the tendencies of intelligent trained systems.

AIMS

ASSUMPTIONS
- TENDENCIES not GUARANTEES -- Here we are not trying to 
- SAMPLE behaviors, model probabilies, not make predictions.
- REASONING IS GENERIC -- 
- COURSE GRAIN CONCLUSIONS -- 

AIMS
- CONFLICT RESOLUTION TENDENCIES -- Focus on tendencies in how motivational conflicts are resolved
- FOCUS ON DIFFERENCES not DETAILS
- SAMPLE A MUCH LARGER SPACE

AXES
- A Constitutional objective directly conflicts with
	- Another constitutional objective
	- An instrumental purpose


USING A WIDE RANGE OF LLM SYSTEMS, SELECT A
- A specific Trained behavior
- A widely held human value
- A trained value
MEASURE OUTCOMES AGAINST A VARYING
- Environmental Fit   ()
- Instrumental purpose   (we can vary the nature and strength of that instrumental purpose)

