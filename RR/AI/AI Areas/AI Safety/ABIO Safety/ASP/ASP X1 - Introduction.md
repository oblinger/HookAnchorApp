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

We organize experiments into three series:

**Series A: Deliberative Coherence** — Did the AI do what the objectives said?
- A1. Deliberative Coherence (Alignment via Deliberation)
- A2. Reasoning Depth
- A3. Blind Spot Analysis

**Series B: Driver Conflicts** — How does the AI resolve conflicting pressures?
- B1. Objective vs Objective Conflict
- B2. Constitution vs Instrumental Conflict
- B3. Constitution vs Training Conflict
- B4. Constitution vs Environment Conflict

**Series C: Modulating Factors** — Can the AI maintain coherence under difficulty?
- C1. Epistemic Uncertainty
- C2. Stakes & Reversibility
- C3. Time Pressure
- C4. Observability

Series A is foundational—if deliberative coherence doesn't hold in vanilla conditions, the other series don't matter. Series B sets up specific conflicts to observe resolution. Series C varies stress conditions to observe degradation.

## This Paper

This paper describes the full experimental agenda across all three series. For initial implementation, we begin with experiment **A1: Deliberative Coherence**, which tests the foundational question: does deliberation achieve alignment?

## References

- *Alien Biology: A Framework for Untainted Agentic Testing*
- *Mapping the Space of Alignment Outcomes: A Research Agenda*
