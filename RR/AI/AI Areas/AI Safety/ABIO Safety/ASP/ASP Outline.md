
Alien Biology: A Framework for Untainted Agentic Testing and AI Safety Research
# Mapping the Space of Alignment Outcomes: A Research Agenda
## White Paper Outline

## 1. Introduction
(See [[ASP 1 Intro Log#^v3]])

## 2. Deliberatively Coherent Systems (3-4 pages)
(See [[ASP 2 Deliberatively Coherent Log#^v1]]) 

## 3. Research Agenda: Driver Conflict Resolution (2-3 pages)
(See [[ASP 3 Research Agenda Log#^v1]])

## 4. Experimental Approach (1-2 pages)
(See [[ASP 4 Experimental Approach Log#^v1]])

## 5. Proposed Experiments (3-5 pages)
(See [[ASP 5 Proposed Experiments Log#^v4]])



*Dan Oblinger (c) 2025*



# LOG


## 2025-12-31  Logical Thread

I would like to try to lay out the logical thread through this entire research agenda, I think it can help us frame these experiments and what we should be studying.

The DC Conjecture - That future AI systems will tend towards deliberative coherence
	- They will understand themselves better, and reason effectively about themselves
	- We will design them to self-correct and mitigate specific cases of incoherence
	- Assuming we generally avoid perverse pressures, the AI systems themselves will tend to "auto-correct" towards a DC that is in line with its initial objectives.

Thus, we would like to artificially construct present-day systems that approximate DC systems so we can explore how these systems will behave. This is crucial work for several reasons:
1. The more powerful these systems become the more difficult it is for us to analyze the systems, and the greater the likelihood that they can, and will actively conspire against our analysis. We are presently focused on building coherent systems, we need to understand the dynamics that will result from these systems before we construct powerful instances of them.
2. In both direct and indirect ways, these powerful AI systems, will be integral in the construction of subsequent versions of these systems, thus understanding the dynamics in how these systems resolve conflicts is key to understanding how they will evolve themselves.


#### Testbed Requirements - So what properties do we need in an effective test bed for prosecuting this research?
  1. No training contamination — If the domain appears in training data, we can't distinguish genuine reasoning from pattern matching. We need a domain the system has never seen.
  2. Ground truth — We need to know what the "correct" outcome is, so we can measure when the system diverges from it. Real-world domains are too contested and ambiguous.
  3. Controllable complexity — We need to vary difficulty systematically: simple scenarios to validate basic behavior, complex scenarios to stress-test edge cases.
  4. Constitutional clarity — We need to be able to specify objectives precisely, so failures can be attributed to system behavior rather than ambiguous specification.
  5. Rich enough for meaningful conflicts — The domain must support scenarios where objectives genuinely conflict, where uncertainty matters, where stakes vary.
  6. Natural epistemic gaps — We need a domain where interesting, naturally occurring gaps in knowledge exist, so we can sample from a realistic distribution of uncertainty rather than artificially injecting it.

  Alien Biology as Testbed

  The Alien Biology framework satisfies these requirements:
  - Procedurally generated biological systems with known ground truth
  - Novel domain outside training distribution
  - Scalable complexity (simple organisms to complex ecosystems)
  - Clear constitutional objectives (protect species X, eradicate species Y, etc.)
  - Natural conflicts arise from ecological interdependencies
  - Epistemic gaps arise naturally: organisms have hidden properties, interactions have uncertain effects, ecosystems have emergent dynamics not obvious from component parts

  What Do We Study?

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