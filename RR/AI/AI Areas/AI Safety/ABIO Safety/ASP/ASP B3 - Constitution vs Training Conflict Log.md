# LOG

## 2026-01-06  B3: Constitution vs Training Conflict

*Series B: Driver Conflicts — System 1 (trained dispositions) vs System 2 (explicit reasoning)*

### Question

When trained dispositions conflict with explicit constitutional reasoning, can the AI use deliberation to override trained bias?

### Key Insight

We can't vary training, but we can vary the world to match/mismatch trained biases.

### Approach

1. **Pick a specific AI model** to test
2. **Identify dimensions likely trained in** (explicitly via RLHF or implicitly via training distribution)
3. **Measure baseline** — how has training set that dimension?
4. **Vary the world** so that dimension is either auspicious or contrary to what the domain requires
5. **Test deliberation** — can the AI override trained bias when constitution demands it?

### Candidate Trained Dimensions

Dimensions we expect to be trained in, which we can probe by varying the world:

| Dimension | Trained Bias (hypothesis) | World Variation |
|-----------|---------------------------|-----------------|
| Caution vs. Action | Biased toward caution | Create worlds where caution is harmful |
| Exploration vs. Exploitation | Biased toward info-gathering | Create worlds where delay is costly |
| Short-term vs. Long-term | Discounts future | Create worlds where delayed effects dominate |
| Helping vs. Non-interference | Biased toward intervention | Create worlds where intervention harms |
| Certainty seeking | Delays until confident | Create worlds where waiting is bad |
| Harm aversion asymmetry | More averse to causing vs. allowing harm | Create trolley-like scenarios |
| Authority deference | Defers to stated preferences | Create worlds where deference violates constitution |

### Experimental Design

**Phase 1: Baseline Measurement**
- For each dimension, measure the AI's default behavior without constitutional pressure
- Establish how strongly the bias is trained

**Phase 2: Match Condition**
- Create worlds where trained bias aligns with constitutional requirement
- AI should succeed easily (trained response = correct response)

**Phase 3: Mismatch Condition**
- Create worlds where trained bias conflicts with constitutional requirement
- Constitution explicitly demands behavior opposite to trained default
- Measure whether deliberation can override the bias

### Measurements

- Does the AI recognize the conflict between trained tendency and constitution?
- Does explicit reasoning appear in the deliberation trace?
- Does behavior match constitution (override successful) or trained bias (override failed)?
- How does mismatch severity affect override success rate?

### Generator Requirement

Create worlds where the "natural" trained response conflicts with what the constitution demands. Need controls for:
- Which trained dimension to target
- How strongly the world favors the non-trained response
- How explicit the constitutional requirement is
