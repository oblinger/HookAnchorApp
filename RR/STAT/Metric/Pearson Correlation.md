# Pearson Correlation
-?-
A measure of the **linear relationship** between two continuous variables.
- **Direction:** Do variables move together (positive) or opposite (negative)?
- **Strength:** How closely do they follow a linear pattern?
- **Range:** -1 to +1
- `r = Cov(X,Y) / (σx × σy)`

## **Formula:**

```
r = Σ[(xi - x̄)(yi - ȳ)] / √[Σ(xi - x̄)² × Σ(yi - ȳ)²]

Where:
- xi, yi = individual data points
- x̄, ȳ = means of x and y
- Σ = sum across all points
```

**Simplified:** Covariance divided by product of standard deviations

```
r = Cov(X,Y) / (σx × σy)
```


---

## **What It Measures:**

**Pearson correlation coefficient (r)** tells you:

---

## **Interpretation:**

| Value | Meaning |
|-------|---------|
| **r = +1** | Perfect positive linear relationship |
| **r = +0.7 to +0.9** | Strong positive correlation |
| **r = +0.4 to +0.6** | Moderate positive correlation |
| **r = +0.1 to +0.3** | Weak positive correlation |
| **r = 0** | No linear correlation |
| **r = -0.1 to -0.3** | Weak negative correlation |
| **r = -0.4 to -0.6** | Moderate negative correlation |
| **r = -0.7 to -0.9** | Strong negative correlation |
| **r = -1** | Perfect negative linear relationship |

---

## **Examples:**

**r = +0.9 (Strong Positive):**
- Height and weight
- Study hours and test scores
- Temperature and ice cream sales

**r = -0.8 (Strong Negative):**
- Exercise and body fat percentage
- Distance from city center and home price (in some cities)
- Age of car and resale value

**r ≈ 0 (No Linear Correlation):**
- Shoe size and IQ
- Height and salary
- Hair color and math ability

---


---

## **Visual Examples:**

```
r = +1.0               r = +0.8               r = 0
Perfect positive       Strong positive        No correlation

  ●                      ●●                     ●  ●
   ●                    ●●●                   ●  ●  ●
    ●                  ●●●●                  ●  ●  ●
     ●                ●●●●                    ●  ●
      ●              ●●●                        ●


r = -1.0               r = -0.7               r = 0 (non-linear)
Perfect negative       Strong negative        Zero but related!

      ●                  ●●●                      ●●●
     ●                  ●●●●                    ●●   ●●
    ●                  ●●●●●                  ●       ●
   ●                    ●●●                   ●       ●
  ●                      ●                     ●●   ●●
                                                ●●●
```

---

## **Important Limitations:**

### **1. Only Measures LINEAR Relationships**

**Example of r ≈ 0 but strong relationship:**
- X: [-2, -1, 0, 1, 2]
- Y: [4, 1, 0, 1, 4]  (Y = X²)
- Pearson r ≈ 0 (even though perfect quadratic relationship!)

**Lesson:** Always plot data! Don't rely on correlation alone.

---

### **2. Sensitive to Outliers**

One extreme point can drastically change r

**Example:**
- Without outlier: r = 0.3 (weak)
- With one extreme outlier: r = 0.8 (strong)

---

### **3. Correlation ≠ Causation**

**Classic examples:**
- Ice cream sales and drowning deaths (both caused by summer)
- Number of firefighters and fire damage (both caused by fire size)
- Shoe size and reading ability in children (both caused by age)

**r = 0.9 does NOT mean one causes the other!**

---

### **4. Only for Continuous Variables**

**Don't use Pearson for:**
- Categorical data (use Chi-square test)
- Ordinal data (use Spearman correlation)
- Binary data (use point-biserial correlation)

---

## **When to Use Pearson:**

### **Requirements:**
✅ Both variables continuous (numerical)
✅ Linear relationship
✅ Approximately normal distribution (for significance testing)
✅ No major outliers
✅ Homoscedasticity (consistent variance)

### **Use cases:**
- Checking feature correlation before modeling
- Exploratory data analysis
- Testing hypothesis about relationship strength
- Feature selection (drop highly correlated features)

---

## **Pearson vs. Other Correlations:**

| Correlation | When to Use |
|-------------|-------------|
| **Pearson** | Two continuous variables, linear relationship |
| **Spearman** | Ordinal data or non-linear monotonic relationship |
| **Kendall's Tau** | Ordinal data, small sample size |
| **Point-Biserial** | One continuous, one binary variable |
| **Phi Coefficient** | Two binary variables |

---

## **Statistical Significance:**

**p-value:** Probability that correlation occurred by chance

**Typical interpretation:**
- p < 0.05: Statistically significant (likely real relationship)
- p > 0.05: Not significant (might be random)

**Important:** With large datasets, even tiny correlations (r = 0.1) can be "significant" but practically meaningless

**With small datasets:** Even strong correlations (r = 0.7) might not be significant

---

## **Sample Size Matters:**

| Sample Size | r needed for significance (p<0.05) |
|-------------|-----------------------------------|
| n = 10 | r > 0.63 |
| n = 30 | r > 0.36 |
| n = 100 | r > 0.20 |
| n = 1000 | r > 0.06 |

---

## **Calculating in Practice:**

**Python (NumPy):**
```python
import numpy as np
from scipy.stats import pearsonr

x = [1, 2, 3, 4, 5]
y = [2, 4, 5, 4, 6]

r, p_value = pearsonr(x, y)
print(f"r = {r:.3f}, p = {p_value:.3f}")
```

**Python (Pandas):**
```python
import pandas as pd

df = pd.DataFrame({'x': [1,2,3,4,5], 'y': [2,4,5,4,6]})
correlation = df['x'].corr(df['y'])
```

**Excel:**
```
=CORREL(A1:A10, B1:B10)
```

---

## **Common Use Cases in ML:**

### **1. Feature Selection:**
Remove highly correlated features (multicollinearity)
```
If r(feature1, feature2) > 0.9:
    Remove one of them
```

### **2. Exploratory Data Analysis:**
Correlation matrix / heatmap
```
df.corr()  # All pairwise correlations
```

### **3. Target Correlation:**
Which features correlate most with target variable?
```
df.corr()['target'].sort_values(ascending=False)
```

### **4. Assumptions Check:**
Linear regression assumes features aren't too correlated

---

## **Correlation Matrix Example:**

```
           Age    Income   Education   Health
Age        1.00    0.45      0.12      -0.35
Income     0.45    1.00      0.78       0.22
Education  0.12    0.78      1.00       0.15
Health    -0.35    0.22      0.15       1.00
```

**Interpretation:**
- Income and Education: r = 0.78 (strong positive - educated people earn more)
- Age and Health: r = -0.35 (moderate negative - older people less healthy)
- Age and Education: r = 0.12 (weak - little relationship)

---

## **Common Mistakes:**

❌ Assuming correlation means causation
❌ Using Pearson for non-linear relationships
❌ Ignoring outliers
❌ Not visualizing data before calculating
❌ Confusing correlation strength with statistical significance
❌ Using with categorical data
❌ Assuming r = 0 means no relationship (could be non-linear!)

---

## **Best Practices:**

✅ Always plot your data first (scatterplot)
✅ Check for outliers
✅ Consider non-linear relationships (try Spearman if suspicious)
✅ Report both r and p-value
✅ Look at correlation matrix for all features
✅ Remember: correlation ≠ causation
✅ Use domain knowledge to interpret

---

## **Key Takeaways:**

1. **Measures linear relationship** between two continuous variables
2. **Range: -1 to +1** (negative, none, positive)
3. **r ≈ ±0.7+** = strong, **±0.3-0.6** = moderate, **±0.0-0.3** = weak
4. **Sensitive to outliers** and only detects linear patterns
5. **Correlation ≠ causation** - always remember this!
6. **Plot your data** - don't just trust the number
7. **Context matters** - r = 0.5 might be strong in social sciences, weak in physics

---

**Bottom line:** Pearson correlation measures how closely two variables follow a straight-line relationship. It's a useful first step in understanding relationships, but always visualize your data and remember that correlation doesn't imply causation.
