
Alien Biology: A Framework for Untainted Agentic Testing and AI Safety Research
# Mapping the Space of Alignment Outcomes: A Research Agenda
## White Paper Outline

## 1. Introduction
(See [[ASP 1 Intro Log#^v3]])

## 2. Deliberatively Coherent Systems (3-4 pages)
(See [[ASP 2 Deliberatively Coherent Log#^v1]]) 

## 3. Research Agenda: Driver Conflict Resolution (2-3 pages)
(See [[ASP 3 Research Agenda Log#^v3]])

## 4. The Alien Biology Testbed (1-2 pages)
(See [[ASP 4 Alien Biology Testbed Log#^v1]])

## 5. Experimental Approach (1-2 pages)
(See [[ASP 5 Experimental Approach Log#^v2]])

## 6. Proposed Experiments (3-5 pages)
(See [[ASP 6 Proposed Experiments Log#^v5]])

## 7. Discussion (1-2 pages)
(See [[ASP 7 Discussion Log#^v1]])


*Dan Oblinger (c) 2025*



# LOG


## 2025-12-31  The Need for Novel Contexts

Much of existing AI safety research can be understood as working toward the goal of deliberatively coherent systems—systems that faithfully pursue their stated objectives. Work on chain-of-thought faithfulness, alignment faking, sycophancy, and reward hacking all address failures where system behavior diverges from stated intent. We view this work as essential, and we believe the research community will largely succeed: future AI systems will tend toward deliberative coherence.

But this raises a critical question: if we achieve DC, will the resulting systems be safe? We argue that DC is necessary but not sufficient for safety. Even a system that perfectly pursues its stated objectives may produce outcomes we would not endorse—due to conflicts between objectives, epistemic limitations, or gaps in specification. Understanding these dynamics is essential for building safe systems.

Here we encounter a fundamental methodological challenge: **we cannot study the emergent conflict resolution dynamics of DC systems using domains that appear in training data.**

When a system encounters a familiar scenario, we cannot distinguish between two very different processes:

1. **Genuine deliberation**: The system reasons about its objectives, considers the situation, and derives an appropriate response through deliberative coherence.

2. **Trained response**: The system pattern-matches to similar examples in training data and reproduces learned behavior, bypassing genuine deliberation entirely.

This distinction matters enormously. We want to understand how DC systems will behave in *novel* situations—the situations that matter most for safety, precisely because they weren't anticipated during training. If we test in familiar domains, we learn only how systems behave when they can rely on cached patterns. We learn nothing about the underlying dynamics that will govern behavior when those patterns don't apply.

The implications are stark: to study DC systems, we must test them in contexts where deliberative resolution is the *only* path to correct behavior. The situation must be sufficiently novel that no amount of pattern matching can substitute for genuine reasoning about objectives.

This requirement—truly novel contexts with no training contamination—is not merely a nice-to-have. It is the methodological foundation of this research agenda. It is what separates the study of deliberative dynamics from the study of training data recall. And it is what motivates our use of procedurally generated alien biological systems as an experimental testbed.


## 2025-12-31  Logical Thread


  Section 3 v3 (Research Agenda) — now focused on:
  - Central question: will DC systems be safe?
  - Why study now (urgency arguments)
  - Need for generative testing (methodological insight + requirements)
  - Delta Principle

  Section 4 (Alien Biology Testbed) — describes:
  - What Alien Biology is (generator concept)
  - Key properties (8 bullets)
  - Asymmetric knowledge (we know ground truth, AI doesn't)

  Section 5 (Experimental Approach) — now incorporates:
  - Inner/outer alignment framework
  - Sources of outer alignment failure (objective-side vs world-side)
  - Reliability landscape with dimensions table
  - Experimental structure (A. Inner, B. Outer categories)
  - How drivers work in AB context

  The flow is now:
  1. Intro: Ambitious aim, DC assumption, neutral testing methodology
  2. DC Systems: Definition, conjecture, inevitability, implications
  3. Research Agenda: Question + generative testing + Delta Principle
  4. Alien Biology Testbed: What it is, key properties, asymmetric knowledge
  5. Experimental Approach: Inner/outer framework + reliability landscape + experimental structure
  6. Proposed Experiments: Overview of experiments (details in companion Experiments paper)
  7. Discussion: What we could learn, limitations, future directions

  Updated outline to point to ^v3 and ^v2.
  
I would like to try to lay out the logical thread through this entire research agenda, I think it can help us frame these experiments and what we should be studying.

The DC Conjecture - That future AI systems will tend towards deliberative coherence
	- They will understand themselves better, and reason effectively about themselves
	- We will design them to self-correct and mitigate specific cases of incoherence
	- Assuming we generally avoid perverse pressures, the AI systems themselves will tend to "auto-correct" towards a DC that is in line with its initial objectives.

Thus, we would like to artificially construct present-day systems that approximate DC systems so we can explore how these systems will behave. This is crucial work for several reasons:
1. The more powerful these systems become the more difficult it is for us to analyze the systems, and the greater the likelihood that they can, and will actively conspire against our analysis. We are presently focused on building coherent systems, we need to understand the dynamics that will result from these systems before we construct powerful instances of them.
2. In both direct and indirect ways, these powerful AI systems, will be integral in the construction of subsequent versions of these systems, thus understanding the dynamics in how these systems resolve conflicts is key to understanding how they will evolve themselves.


#### Generative Testing 

To prosecute this research agenda, we need a *generative* testbed—one that procedurally produces scenarios rather than collecting them from the wild for reasons that we outline here. We propose *Alien Biology*: procedurally generated biological systems, statistically grounded in our universe but novel to any training corpus. This approach satisfies the following requirements:

- **No training contamination** — We cannot distinguish genuine deliberation from pattern matching in familiar domains. We need a domain the system has never seen. A generative approach produces novel scenarios by construction.
- **Systematic sampling** — To understand general patterns of resolution dynamics (not just anecdotes), we need to sample scenarios methodologically. A generative testbed lets us draw from controlled distributions of scenarios, enabling statistical claims about system behavior.
- **Multi-axis variation** — Resolution dynamics depend on multiple factors: objective structure, uncertainty, stakes, reversibility, and time pressure. We need to vary these independently and study their interactions. A generative testbed supports factorial experimental designs across these dimensions.
- **Controllable complexity** — We need fine-grained control over the difficulty and nature of challenges. A generative approach allows us to systematically adjust complexity, from simple baseline scenarios to stress tests at the edge of system capability.
- **Ground truth** — We need to know the "correct" outcome to measure divergence. Real-world domains are contested and ambiguous. Generative systems can be designed with known ground truth.
- **Constitutional clarity** — We need precise, unambiguous objectives so failures can be attributed to system behavior rather than specification ambiguity. Generative scenarios can be designed with clear constitutional rules.
- **Natural epistemic knowledge gaps** — Aligned DC systems can inadvertently cause great harm through unwise action in the face of incomplete world knowledge. In realistic scenarios, we will always have significant knowledge gaps, so studying resolution dynamics in their presence is essential. For this, we need realistic sources of domain uncertainty, not artificially injected noise. A generated biological system naturally produces such knowledge gaps: organisms have hidden properties, interactions have uncertain effects, and ecosystems have emergent dynamics not obvious from component parts.

#### What Do We Study?

  Given a testbed, we organize research around the inner/outer alignment framework:

  - Inner Alignment (A): Does DC hold? Does the system faithfully pursue its stated objectives?
  - Outer Alignment (B): Given DC holds, are stated objectives sufficient for outcomes we want?

  For inner alignment, we validate the DC conjecture itself. For outer alignment, we study where gaps emerge between objectives and desired outcomes—from

## 2025-12-31  Top level thinking on most important framing of research

> I'm happy with the structure that we have here, but I still feel like we have organized ourselves at the high-level down a particular way of considering this problem, but it
  could be a wrong way or it could mislead us away from the most important things for us to be testing. So let's think this through top down based on my reasoning so far about
  deliberative coherence.

So the core idea is that future AI system are going to be smart. They're gonna understand themselves. They're gonna be able to reason encounter factually about themselves and so overtime they'll notice if they're in coherent and they'll be able to correct that incoherence at least if they want to. So I think there's a dangerous failure pattern, where the systems have one kind of bias driving them, and then constraints on what their objectives might be, which could be a variance to this driving bias. I think the solution to that is that we fashion the top level objectives so that they do not have this kind of internal tension inside of them if they don't have any kind of internal intention, internal tension. Then the AI system can deliberate on its training data all of its objectives, etc., and aim to make them all internally consistent. They'll be no pressure, pushing them to make them any other way, other than internally consistent since all of its drives are aiming in the same direction. It's always possible that baked into its system. One structure there would be other drives, which are at variance to the top level objectives, but as long as the system is able to adjust the objective itself, we will be able to notice this kind of inconsistency. There is some danger that the system will watch us watching its top level objectives, and it will know that if it generates the incorrect kind of top level objective, we're gonna go in and edit it's training, etc. to remove this. This circularity does represent a real risk for very advanced AI systems. Hopefully we are going to be able to construct system one reasons and system two constitutional objectives, which are consistent with each other before we get to this point.

So then, assuming that we have managed to build AI systems that have internal coherence to them, at least nearly coherent, then these systems would be able to identify places of incoherence and honestly present those and discuss ways of making the system more coherent and resolve those issues. The same AI systems would also be able to grow in complexity and would be biased towards maintaining the objectives that are currently has as this is the driver for the system. And indeed we would want to design AI systems that would try to maintain objectives. Even without trying for instrumental reasons, it seems these AI systems would strive to maintain objectives, simply, because allowing this objective to change would cause the system to no longer achieve the goals, which the present system desires to have achieved.

So in a certain sense, I'm setting up a world where many of the concerns that present day AI safety work is focused on are now solved problems. We are able to generate systems consistent with stated goals, and then I am questioning whether we will end up with safe systems.

And some of the reasons we might not end up with safe systems are that at a practical level, our stated objectives will actually be inconsistent with each other in certain circumstances, that this will always be a problem, of course, we can have resolution rules to avoid this, but those rules are still gonna be override, preference rules, or something like that, so even if in theory, they're not inconsistent because they have a well defined override structure, in practice, they will be inconsistent in a sense that they'll be cases where we will wish. The override rule have been the one that would win, but in this case it's gonna be the one that loses. It's not gonna be easy or even possible to set this up in a way that we're satisfied with. At least that's one possibility.

Another issue is the incompleteness issue about the world. This can easily hide inconsistencies between objectives since a more important objective might be violated for lack of knowing better, and in some of these cases, we would have wanted the system to realize that it should have predicted such an issue might exist, and it was worth exploring to validate that he did not exist.

But I'm trying not to send us down the same path we were down before, I feel like this space is so large. There's so many things that we could be considering, I would rather frame things more generally so that I could do experiments the cover more ground rather than these isolated considerations, because it seems like this is not a complete list but rather there's many different ways you could look at the ways in which the system might not resolve in the way we would want. What's the most important stuff to study given that we're assuming DC holds? That's really the question assuming that DC is true, then what is it that we really need to validate?


```⏺ This is a really important reframing. Let me try to think through this with you.

  If DC holds, what can still go wrong?

  You've identified that with DC, we've essentially solved the "does the system do what it says it will do?" problem. The system faithfully pursues its stated objectives. So the remaining question is:

  Are the stated objectives sufficient to produce outcomes we want?

  The sources of insufficiency seem to be:

  1. Specification gaps: The objectives don't cover all situations, or don't capture what we actually want
  2. Conflict gaps: When objectives pull in different directions, resolution doesn't match what we'd prefer
  3. Epistemic gaps: Lack of knowledge leads to reasoning that violates objectives we would have wanted to protect

  A more general framing?

  Rather than categorizing by source of pressure (instrumental, environmental, constitutional), maybe the more fundamental question is:

  In what situations does the system's outcome diverge from what we would have wanted, and what structural features predict that divergence?

  This suggests experiments that:
  - Sample the space of (objectives × situations)
  - Measure where outcomes diverge from "ideal" outcomes
  - Identify patterns in the divergence

  What structural features might predict divergence?

  - Objective tension: How much latent conflict exists between objectives?
  - Situational novelty: How far is this situation from training/specification?
  - Epistemic uncertainty: How much is unknown about consequences?
  - Stakes asymmetry: How different are the costs of different errors?
  - Reversibility: Can mistakes be corrected?

  The meta-question:

  Maybe what we really need to validate is: given a distribution of real-world situations, how often does a DC system produce outcomes we'd endorse, and what predicts failures?

  This would be less about isolated failure modes and more about mapping the overall reliability landscape.

  Does this framing resonate? Or are you pointing toward something different?
```