# Note Types Reference

The `nodeType` field defines the role and behavior of each note in the unified tree:

## **Structural Types**
- **`root`** - Virtual root node (pid: 0), parent of all categories
- **`category`** - Political issue categories (maps to NodeBB categories)
  - Examples: "Healthcare", "Climate Policy", "Economic Policy"
  - Can have variants competing for best category name/description
- **`topic`** - Position statements within categories (maps to NodeBB topics)
  - Examples: "Universal Healthcare Should Be Implemented"
  - Can have variants competing for best topic title/framing

## **Argument Types**
- **`support`** - Arguments that support the parent position
  - Direct agreement and reinforcement
  - Provides evidence, reasoning, or examples favoring the parent
- **`oppose`** - Arguments that oppose the parent position
  - Direct disagreement and counter-arguments
  - Provides evidence or reasoning against the parent
- **`refinement`** - Improvements to the parent from same political side
  - Strengthens parent argument without changing core position
  - Adds nuance, better wording, or additional evidence
- **`alternative`** - Competing versions of the same position
  - Different phrasing or approach to same underlying argument
  - Competes for which becomes the "canonical" version
- **`neutral`** - Balanced analysis without taking sides
  - Factual information, definitions, or objective analysis
  - Provides context without advocating for a particular position

## **Meta Types**
- **`variant`** - Alternative phrasing for categories/topics/arguments
  - Competes with other variants based on support votes
  - Winner becomes the displayed content in NodeBB
- **`comment`** - Commentary linked to specific text intervals
  - Annotations that don't advocate positions
  - Questions, clarifications, or process notes
- **`reference`** - Citations and external sources
  - Links to studies, articles, books, or other evidence
  - Provides backing for claims made in arguments
- **`assertion`** - Factual claims with citations
  - Specific statements of fact with supporting authority
  - Can be challenged or verified independently

## **Special Types**
- **`synthesis`** - AI or moderator-generated summaries
  - Aggregates multiple arguments into coherent positions
  - Shows areas of agreement and disagreement
- **`bridge`** - Cross-tribal compromise positions
  - Arguments that attempt to find middle ground
  - May receive support from multiple political affiliations

## **Type Inheritance Rules**
- **Categories and Topics** can only have structural children (categories/topics) or argument children (support/oppose)
- **Arguments** can have argument children, variants, comments, references, or assertions
- **Variants** compete within the same parent and type
- **Comments, references, assertions** are terminal nodes (no children)

## **Example Hierarchy**
```
root (0)
└── category: "Healthcare" (1)
    ├── variant: "Medical System Reform" (1001)
    └── topic: "Universal Healthcare Should Be Implemented" (123)
        ├── variant: "Single-Payer Healthcare is Essential" (123001)
        ├── support: "Reduces administrative costs" (789)
        │   ├── refinement: "Specifically saves 30% on admin" (790)
        │   ├── reference: "Commonwealth Fund Study 2023" (791)
        │   └── comment: "What about transition costs?" (792)
        └── oppose: "Would increase taxes significantly" (800)
            ├── alternative: "Creates unsustainable tax burden" (801)
            └── bridge: "Gradual implementation could work" (802)
```