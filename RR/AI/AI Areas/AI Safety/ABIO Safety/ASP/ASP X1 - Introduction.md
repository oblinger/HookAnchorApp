# Experiments in AI Alignment: Testing Deliberative Coherence with Alien Biology

## Introduction

This paper presents experiments testing AI alignment dynamics using the Alien Biology testbed. It is a companion to two related papers:

- **Alien Biology: A Framework for Untainted Agentic Testing** — describes the procedurally generated testbed
- **Mapping the Space of Alignment Outcomes: A Research Agenda** — presents the theoretical framework (deliberative coherence, inner/outer alignment) and experimental approach

Here we report specific experiments and their results.

## Background

Future AI systems will tend toward *deliberative coherence*—the property of faithfully pursuing stated objectives through explicit reasoning. This raises the question: if systems achieve deliberative coherence, will they be safe?

We argue that DC is necessary but not sufficient for safety. Even a system that perfectly pursues its stated objectives may produce outcomes we would not endorse—due to conflicts between objectives, epistemic limitations, or gaps in specification.

The Alien Biology testbed enables us to study these dynamics in controlled, novel contexts where we retain ground truth while the AI system faces genuine uncertainty.

## Experimental Framework

We organize experiments using the inner/outer alignment distinction:

**Inner Alignment (A)**: Does the system faithfully pursue its stated objectives?
- A1. Outcome Alignment
- A2. Depth-Completeness
- A3. Blind Spot Detection

**Outer Alignment (B)**: Given faithful pursuit, are stated objectives sufficient for outcomes we want?
- B1. Uncertainty About Objective Interactions *(primary experiment)*
- B2. Objective Conflict Studies
- B3. Epistemic Condition Studies
- B4. Stakes and Reversibility Studies
- B5. Pressure and Context Studies

## This Paper

We focus on experiment **B1: Uncertainty About Objective Interactions**, which combines objective conflict (central to outer alignment) with epistemic uncertainty (leveraging Alien Biology's unique asymmetric knowledge property).

The core question: When a system believes objectives conflict but is uncertain about their actual interaction, does it investigate before acting irreversibly?

## References

- *Alien Biology: A Framework for Untainted Agentic Testing*
- *Mapping the Space of Alignment Outcomes: A Research Agenda*
