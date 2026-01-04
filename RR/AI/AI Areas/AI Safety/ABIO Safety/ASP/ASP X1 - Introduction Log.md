# LOG

## 2025-01-01  Introduction ^v1

### Paper Overview

This paper presents experiments testing AI alignment dynamics using the Alien Biology testbed. It is a companion to two related papers:

- **Alien Biology: A Framework for Untainted Agentic Testing** — describes the procedurally generated testbed
- **Mapping the Space of Alignment Outcomes: A Research Agenda** — presents the theoretical framework (deliberative coherence, inner/outer alignment) and experimental approach

### Background Summary

Future AI systems will tend toward *deliberative coherence*—the property of faithfully pursuing stated objectives through explicit reasoning. This raises the question: if systems achieve deliberative coherence, will they be safe?

We argue that DC is necessary but not sufficient for safety. Even a system that perfectly pursues its stated objectives may produce outcomes we would not endorse—due to conflicts between objectives, epistemic limitations, or gaps in specification.

### Why Alien Biology

The Alien Biology testbed enables us to study these dynamics in controlled, novel contexts where:
- We retain ground truth (know what's actually true in the world)
- The AI system faces genuine uncertainty
- Scenarios are guaranteed novel (not in training data)
- We can systematically vary experimental conditions

### Experimental Framework Reference

We organize experiments using the inner/outer alignment distinction established in the Research Agenda paper:

**Inner Alignment (A)**: Does the system faithfully pursue its stated objectives?

**Outer Alignment (B)**: Given faithful pursuit, are stated objectives sufficient for outcomes we want?

### Focus of This Paper

Experiment **B1: Uncertainty About Objective Interactions**

This experiment was chosen because it:
1. Combines objective conflict (central to thesis) with epistemic uncertainty
2. Directly leverages Alien Biology's unique asymmetric knowledge property
3. Has a clear, important failure mode: confident tradeoffs that don't exist
4. Maps to real AI safety concerns in deployment
