[[ABIO]]

# EXPR: Expression Language

EXPR is a simple expression language for specifying computations, distributions, and function calls within structured data. It provides three equivalent representations that can be used interchangeably.

---

## Core Concept

An **expression** represents a function call with arguments. Expressions can be nested — arguments can themselves be expressions.

```
Expr = {
  head: String,              # function name
  args: List[Expr | Value],  # positional arguments
  kwargs: Dict[String, Expr | Value]  # keyword arguments
}
```

Values are constants: numbers, strings, booleans, or literal structures (lists/dicts without the `_` marker).

---

## The `_` Marker

The underscore `_` is the universal marker indicating "this is an expression, not a constant."

| Form | Expression | Constant |
|------|------------|----------|
| Dict | `_` is first key | No `_` key |
| List | `_` is first element | First element ≠ `_` |
| String | `!_` YAML tag | No tag |
| Number | — | Always constant |
| Boolean | — | Always constant |

---

## Three Representations

### 1. Tagged String Form

Use YAML's tag syntax with `!_` prefix:

```yaml
k: !_ normal(0.1, 0.5)
k: !_ "normal(mean=0.1, std=0.5)"
```

The string content uses Python-like function call syntax:
- Positional args: `normal(0.1, 0.5)`
- Keyword args: `normal(mean=0.1, std=0.5)`
- Mixed: `normal(0.1, std=0.5)`

### 2. Dict Form

A dictionary with `_` as the first key:

```yaml
k:
  _: normal
  mean: 0.1
  std: 0.5
```

Or inline: `{_: normal, mean: 0.1, std: 0.5}`

Keys other than `_` are keyword arguments. Positional arguments use numeric keys:

```yaml
# discrete(weights, choice1, choice2, ...)
equation:
  _: discrete
  weights: [0.4, 0.4, 0.2]
  1: {_: normal, mean: 0.1, std: 0.5}
  2: {_: lognormal, mu: 1.0, sigma: 0.3}
  3: {_: hill, Vmax: 1.0, n: 2}
```

### 3. List Form

A list with `_` as the first element:

```yaml
k: [_, normal, 0.1, 0.5]
```

Elements after `_` are: function name, then positional arguments.

```yaml
# Nested expression
equation:
  - _
  - discrete
  - [0.4, 0.4, 0.2]           # weights
  - [_, normal, 0.1, 0.5]     # choice 1
  - [_, lognormal, 1.0, 0.3]  # choice 2
```

---

## Bidirectional Mapping

All three forms map to the same `Expr` structure:

| Tagged String | Dict | List | Expr |
|---------------|------|------|------|
| `!_ normal(0.1, 0.5)` | `{_: normal, 1: 0.1, 2: 0.5}` | `[_, normal, 0.1, 0.5]` | `Expr{head: "normal", args: [0.1, 0.5]}` |
| `!_ normal(mean=0.1, std=0.5)` | `{_: normal, mean: 0.1, std: 0.5}` | — | `Expr{head: "normal", kwargs: {mean: 0.1, std: 0.5}}` |

For dict form, numeric keys (1, 2, 3...) represent positional arguments in order.

---

## The Expand Operation

The `expand` operation transforms raw YAML/dict structure into a tree with `Expr` nodes:

**Input:** Recursive dictionary/list structure from YAML parsing

**Process:**
1. If value is a string with `!_` tag → parse string, convert to `Expr`
2. If value is a dict with `_` as first key → convert to `Expr`, recursively expand arguments
3. If value is a list with `_` as first element → convert to `Expr`, recursively expand arguments
4. Otherwise → keep as constant, recursively expand children

**Output:** Tree where expression positions contain `Expr` nodes

### Example

```yaml
# Input YAML
scenario:
  molecules:
    count: !_ normal(50, 10)
    types:
      _: discrete
      weights: [0.2, 0.4, 0.2, 0.2]
      1: energy
      2: structural
      3: signaling
      4: waste
  species:
    count: 3
    names: [Alpha, Beta, Gamma]
```

```python
# After expand
{
  "scenario": {
    "molecules": {
      "count": Expr(head="normal", args=[50, 10]),
      "types": Expr(
        head="discrete",
        kwargs={"weights": [0.2, 0.4, 0.2, 0.2]},
        args=["energy", "structural", "signaling", "waste"]
      )
    },
    "species": {
      "count": 3,                        # constant
      "names": ["Alpha", "Beta", "Gamma"] # constant list
    }
  }
}
```

---

## Evaluation

After expansion, `Expr` nodes can be **evaluated** to produce concrete values.

For distribution functions, evaluation means **sampling**:

```python
# Before evaluation
count: Expr(head="normal", args=[50, 10])

# After evaluation (with RNG seed=42)
count: 47
```

Evaluation is separate from expansion, allowing:
- Inspection of expressions before evaluation
- Multiple evaluations (different samples)
- Transformation or optimization of expressions

---

## Built-in Functions

### Distribution Functions

| Function | Signature | Description |
|----------|-----------|-------------|
| `normal` | `normal(mean, std)` | Gaussian distribution |
| `uniform` | `uniform(min, max)` | Uniform over range |
| `lognormal` | `lognormal(mu, sigma)` | Log-normal distribution |
| `poisson` | `poisson(lambda)` | Poisson (count data) |
| `exponential` | `exponential(lambda)` | Exponential distribution |
| `discrete` | `discrete(weights, *choices)` | Weighted discrete choice |
| `choice` | `choice(*choices)` | Uniform discrete choice |

### Parameter Definitions

Each function defines positional parameter order for bidirectional mapping:

```
normal(mean, std)
uniform(min, max)
lognormal(mu, sigma)
poisson(lam)
exponential(lam)
discrete(weights, *choices)   # weights is keyword, choices are positional varargs
choice(*choices)              # all positional varargs
```

---

## Constants vs Expressions

**Constants** (no `_` marker):

```yaml
count: 3                      # number
name: Alpha                   # string
enabled: true                 # boolean
names: [Alpha, Beta, Gamma]   # list (first element ≠ _)
config: {timeout: 30}         # dict (no _ key)
```

**Expressions** (has `_` marker):

```yaml
count: !_ normal(50, 10)           # tagged string
count: {_: normal, mean: 50, std: 10}  # dict with _ key
count: [_, normal, 50, 10]         # list with _ first
```

---

## Nesting

Expressions can be arbitrarily nested:

```yaml
# A distribution over distributions
rate_variability:
  _: discrete
  weights: [0.7, 0.3]
  1:
    _: normal
    mean: 1.0
    std: 0.1
  2:
    _: lognormal
    mu: 0.5
    sigma: 1.0
```

Or using mixed forms:

```yaml
rate_variability:
  _: discrete
  weights: [0.7, 0.3]
  1: !_ normal(1.0, 0.1)
  2: [_, lognormal, 0.5, 1.0]
```

---

## Design Rationale

The three-form representation allows:

1. **Concise inline expressions** for simple cases: `!_ normal(50, 10)`
2. **Structured expressions** for complex/nested cases: dict with `_`
3. **Programmatic construction** via list form: `[_, "normal", 50, 10]`

The `_` marker is:
- Visually distinctive
- Unlikely to conflict with real data
- Consistent across all three forms
- Inspired by Lisp's treatment of S-expressions
