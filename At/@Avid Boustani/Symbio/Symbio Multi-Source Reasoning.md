# Symbio Multi-Source Reasoning: Representing and Resolving Alternative Hypotheses

## The Core Problem

**Single-document ambiguity:** A region in an aerial photo could be:
- Wetland (if it's natural)
- Detention pond (if it's engineered stormwater)
- Temporary flooding (if it's recent rain)

**Resolution requires cross-referencing:**
- Site plan labels it "Existing Wetland" → hypothesis 1 wins
- Engineering drawing shows "Proposed Detention Basin" → hypothesis 2 wins
- Historical aerial shows it wasn't there last year + recent rain data → hypothesis 3 wins

**We need a representation that:**
1. Maintains multiple competing interpretations
2. Accumulates evidence from different sources
3. Resolves ambiguities as information accrues
4. Tracks provenance (why we believe each hypothesis)
5. Handles contradictions gracefully

## Approach 1: Probabilistic Graphical Model (Bayesian Network)

### Representation

Each visual region is a node with uncertain attributes. Documents provide evidence that updates probabilities.

```python
class SpatialEntity:
    def __init__(self, geometry):
        self.geometry = geometry  # polygon/bbox
        self.hypotheses = {
            'wetland': Hypothesis(prior=0.15, evidence=[]),
            'detention_pond': Hypothesis(prior=0.10, evidence=[]),
            'temporary_water': Hypothesis(prior=0.05, evidence=[]),
            'vegetation': Hypothesis(prior=0.30, evidence=[]),
            'other': Hypothesis(prior=0.40, evidence=[])
        }

    def add_evidence(self, hypothesis_name, evidence, likelihood_ratio):
        """Bayesian update with new evidence"""
        h = self.hypotheses[hypothesis_name]
        h.evidence.append(evidence)

        # Update probability using Bayes rule
        # P(H|E) ∝ P(E|H) * P(H)
        for hyp_name, hyp in self.hypotheses.items():
            if hyp_name == hypothesis_name:
                hyp.posterior *= likelihood_ratio
            else:
                # Competing hypotheses get dampened
                hyp.posterior *= (1.0 / likelihood_ratio) ** 0.5

        # Normalize
        total = sum(h.posterior for h in self.hypotheses.values())
        for h in self.hypotheses.values():
            h.posterior /= total

class Evidence:
    def __init__(self, source, content, confidence, method):
        self.source = source  # which document
        self.content = content  # what it says
        self.confidence = confidence  # how sure
        self.method = method  # how derived (OCR, VLM, etc)
        self.timestamp = datetime.now()
```

### Evidence Accumulation Example

```python
# Initial state: ambiguous water body in aerial photo
region = SpatialEntity(polygon=[(x1,y1), (x2,y2), ...])

# Evidence 1: VLM analyzes aerial photo
region.add_evidence(
    'wetland',
    Evidence(
        source='aerial_2024.jpg',
        content='Visible wetland vegetation, irregular boundary',
        confidence=0.65,
        method='GPT-4V analysis'
    ),
    likelihood_ratio=3.0  # 3x more likely to be wetland than prior
)
# After: P(wetland)=0.38, P(detention_pond)=0.08, P(vegetation)=0.25, ...

# Evidence 2: Site plan OCR finds label
region.add_evidence(
    'detention_pond',
    Evidence(
        source='site_plan_2023.pdf',
        content='Label: "Proposed Detention Basin #2"',
        confidence=0.95,
        method='OCR + text proximity'
    ),
    likelihood_ratio=15.0  # Very strong evidence from explicit label
)
# After: P(wetland)=0.12, P(detention_pond)=0.78, P(vegetation)=0.06, ...

# Evidence 3: Engineering drawing shows outlet structure
region.add_evidence(
    'detention_pond',
    Evidence(
        source='engineering_detail_sheet5.pdf',
        content='Outlet control structure at this location',
        confidence=0.88,
        method='Symbol detection + spatial matching'
    ),
    likelihood_ratio=8.0
)
# After: P(wetland)=0.04, P(detention_pond)=0.92, P(vegetation)=0.02, ...

# Resolution: High confidence it's a detention pond
final_classification = region.get_best_hypothesis()  # 'detention_pond' with 92%
```

### Advantages
- Natural handling of uncertainty
- Probabilistic reasoning with multiple evidence types
- Can quantify confidence in final classification
- Graceful handling of weak/conflicting evidence

### Disadvantages
- Requires careful choice of likelihood ratios (how much to weight each evidence type)
- Assumes independence of evidence sources (often violated)
- Complex to tune and validate

## Approach 2: Structured Hypothesis Graph

### Representation

Maintain explicit alternative interpretations as a graph structure where edges represent support/conflict relationships.

```python
class HypothesisNode:
    def __init__(self, entity_id, interpretation, geometry):
        self.id = uuid4()
        self.entity_id = entity_id  # which spatial region
        self.interpretation = interpretation  # semantic label
        self.geometry = geometry
        self.evidence = []  # supporting evidence
        self.support_score = 0.0
        self.conflict_score = 0.0

class HypothesisGraph:
    def __init__(self):
        self.nodes = []  # all hypotheses
        self.edges = []  # support/conflict relationships

    def add_hypothesis(self, entity, interpretation, evidence):
        """Add new interpretation for an entity"""
        node = HypothesisNode(entity.id, interpretation, entity.geometry)
        node.evidence.append(evidence)
        node.support_score = evidence.confidence
        self.nodes.append(node)

        # Check for support/conflict with existing hypotheses
        for existing in self.nodes:
            if existing.entity_id == entity.id:
                relation = self.assess_relationship(node, existing)
                self.edges.append(relation)

        return node

    def assess_relationship(self, hyp1, hyp2):
        """Determine if hypotheses support or conflict"""

        # Same interpretation = mutual support
        if hyp1.interpretation == hyp2.interpretation:
            return SupportEdge(hyp1, hyp2, strength=0.8)

        # Mutually exclusive interpretations = conflict
        if are_mutually_exclusive(hyp1.interpretation, hyp2.interpretation):
            return ConflictEdge(hyp1, hyp2, strength=1.0)

        # Compatible but different = weak support
        if are_compatible(hyp1.interpretation, hyp2.interpretation):
            return SupportEdge(hyp1, hyp2, strength=0.3)

        return NeutralEdge(hyp1, hyp2)

    def resolve_entity(self, entity_id):
        """Find best interpretation given all evidence"""

        # Get all hypotheses for this entity
        candidates = [n for n in self.nodes if n.entity_id == entity_id]

        # Aggregate support scores
        for candidate in candidates:
            # Direct evidence
            score = sum(e.confidence for e in candidate.evidence)

            # Support from other hypotheses
            for edge in self.edges:
                if edge.target == candidate and isinstance(edge, SupportEdge):
                    score += edge.strength * edge.source.support_score

            # Penalty from conflicts
            for edge in self.edges:
                if edge.target == candidate and isinstance(edge, ConflictEdge):
                    score -= edge.strength * edge.source.support_score

            candidate.final_score = score

        # Return best hypothesis
        best = max(candidates, key=lambda c: c.final_score)
        alternatives = sorted([c for c in candidates if c != best],
                            key=lambda c: c.final_score, reverse=True)

        return best, alternatives
```

### Example Usage

```python
graph = HypothesisGraph()
entity = SpatialEntity(polygon=water_body_polygon)

# Document 1: Aerial photo VLM analysis (ambiguous)
graph.add_hypothesis(
    entity,
    interpretation='wetland',
    evidence=Evidence('aerial.jpg', 'vegetation patterns', confidence=0.6, ...)
)

graph.add_hypothesis(
    entity,
    interpretation='detention_pond',
    evidence=Evidence('aerial.jpg', 'regular shape', confidence=0.4, ...)
)

# Document 2: Site plan labels it
graph.add_hypothesis(
    entity,
    interpretation='detention_pond',
    evidence=Evidence('site_plan.pdf', 'label: detention basin', confidence=0.9, ...)
)

# Now we have 3 hypothesis nodes:
# - H1: wetland (evidence: aerial VLM, score: 0.6)
# - H2: detention_pond (evidence: aerial VLM, score: 0.4)
# - H3: detention_pond (evidence: site plan label, score: 0.9)

# Edges:
# - H2 ↔ H3: SUPPORT (same interpretation, strength: 0.8)
# - H1 ↔ H2: CONFLICT (mutually exclusive, strength: 1.0)
# - H1 ↔ H3: CONFLICT (mutually exclusive, strength: 1.0)

# Resolution:
best, alternatives = graph.resolve_entity(entity.id)
# best = H3: detention_pond (score: 0.9 + 0.8*0.4 = 1.22)
# alternatives = [H1: wetland (0.6 - 1.0*1.22 = -0.62), H2: detention_pond (...)]
```

### Advantages
- Explicit representation of competing hypotheses
- Can trace reasoning (which evidence led to which interpretation)
- Handles both support and conflict naturally
- Easy to visualize and debug

### Disadvantages
- More complex bookkeeping
- Need rules for compatibility/exclusivity of interpretations
- Graph can grow large with many documents

## Approach 3: Semantic Knowledge Base with Uncertainty

### Representation

Use a knowledge graph with uncertain facts and inference rules.

```python
class KnowledgeBase:
    def __init__(self):
        self.facts = []  # (subject, predicate, object, confidence, provenance)
        self.rules = []  # inference rules

    def assert_fact(self, subject, predicate, object, confidence, source):
        """Add a fact with confidence score"""
        fact = Fact(subject, predicate, object, confidence, source)
        self.facts.append(fact)

        # Check for conflicts
        conflicts = self.find_conflicting_facts(fact)
        if conflicts:
            self.resolve_conflict(fact, conflicts)

    def find_conflicting_facts(self, new_fact):
        """Find facts that contradict new fact"""
        conflicts = []
        for existing in self.facts:
            if existing.subject == new_fact.subject and \
               existing.predicate == new_fact.predicate and \
               existing.object != new_fact.object:
                # Same subject and predicate, different object = conflict
                conflicts.append(existing)
        return conflicts

    def resolve_conflict(self, new_fact, conflicts):
        """Decide which fact to believe"""
        all_facts = [new_fact] + conflicts

        # Strategy 1: Trust most recent
        most_recent = max(all_facts, key=lambda f: f.source.timestamp)

        # Strategy 2: Trust highest confidence
        most_confident = max(all_facts, key=lambda f: f.confidence)

        # Strategy 3: Trust most authoritative source
        source_authority = {
            'licensed_survey': 1.0,
            'engineering_drawing': 0.9,
            'site_plan': 0.8,
            'vlm_analysis': 0.6,
            'heuristic': 0.3
        }
        most_authoritative = max(all_facts,
            key=lambda f: source_authority.get(f.source.method, 0.5))

        # Combined: weighted vote
        if most_confident == most_authoritative:
            winner = most_confident
        elif most_confident.confidence > 0.9:
            winner = most_confident
        else:
            winner = most_authoritative

        # Keep winner, mark others as superseded
        for fact in all_facts:
            if fact != winner:
                fact.superseded_by = winner
                fact.status = 'conflict_resolved'

        return winner

    def query(self, subject, predicate):
        """Query facts with confidence"""
        results = [f for f in self.facts
                  if f.subject == subject and f.predicate == predicate
                  and f.status == 'active']

        if not results:
            # Try inference
            results = self.infer(subject, predicate)

        return results

    def infer(self, subject, predicate):
        """Apply inference rules"""
        inferred = []

        for rule in self.rules:
            if rule.can_infer(subject, predicate, self.facts):
                new_fact = rule.apply(subject, self.facts)
                inferred.append(new_fact)

        return inferred
```

### Example with Inference Rules

```python
kb = KnowledgeBase()

# Add facts from different sources
kb.assert_fact(
    subject='region_123',
    predicate='hasType',
    object='water_body',
    confidence=0.7,
    source=Source('aerial.jpg', 'vlm_analysis', timestamp='2024-11-20')
)

kb.assert_fact(
    subject='region_123',
    predicate='hasLabel',
    object='Detention Basin #2',
    confidence=0.95,
    source=Source('site_plan.pdf', 'ocr', timestamp='2024-11-20')
)

kb.assert_fact(
    subject='region_123',
    predicate='hasOutletStructure',
    object='true',
    confidence=0.85,
    source=Source('engineering.pdf', 'symbol_detection', timestamp='2024-11-20')
)

# Inference rule: If has outlet structure AND labeled detention basin → type is detention_pond
class DetentionPondRule(InferenceRule):
    def can_infer(self, subject, predicate, facts):
        if predicate != 'hasType':
            return False

        has_outlet = any(f.subject == subject and
                        f.predicate == 'hasOutletStructure' and
                        f.object == 'true'
                        for f in facts)

        has_detention_label = any(f.subject == subject and
                                 f.predicate == 'hasLabel' and
                                 'detention' in f.object.lower()
                                 for f in facts)

        return has_outlet and has_detention_label

    def apply(self, subject, facts):
        # Get confidence from supporting facts
        outlet_conf = next(f.confidence for f in facts
                          if f.predicate == 'hasOutletStructure')
        label_conf = next(f.confidence for f in facts
                         if 'detention' in str(f.object).lower())

        # Combined confidence (product rule)
        combined_conf = outlet_conf * label_conf

        return Fact(
            subject=subject,
            predicate='hasType',
            object='detention_pond',
            confidence=combined_conf,
            source=Source('inference', 'DetentionPondRule', datetime.now())
        )

kb.add_rule(DetentionPondRule())

# Query
results = kb.query('region_123', 'hasType')
# Returns:
# - water_body (confidence: 0.7, source: VLM)
# - detention_pond (confidence: 0.81, source: inference from outlet+label)

# Resolution: detention_pond wins (higher confidence)
```

### Advantages
- Flexible inference capabilities
- Can encode domain knowledge as rules
- Clear conflict resolution strategies
- Explainable reasoning

### Disadvantages
- Requires defining inference rules (domain expertise)
- Can be computationally expensive for large knowledge bases
- Conflict resolution strategies need tuning

## Approach 4: Multi-Document Consensus with Weighted Voting

### Representation

Simpler approach: collect all interpretations, weight by source reliability, vote.

```python
class EntityInterpretation:
    def __init__(self, entity_id, geometry):
        self.entity_id = entity_id
        self.geometry = geometry
        self.votes = defaultdict(list)  # interpretation -> [Vote]

    def add_vote(self, interpretation, evidence, weight):
        """Record a vote for an interpretation"""
        vote = Vote(
            interpretation=interpretation,
            evidence=evidence,
            weight=weight,
            source_reliability=self.get_source_reliability(evidence.source)
        )
        self.votes[interpretation].append(vote)

    def get_source_reliability(self, source):
        """Reliability scores for different source types"""
        reliability = {
            # Explicit human labels (highest)
            'site_plan_label': 0.95,
            'engineering_drawing_label': 0.95,
            'survey_annotation': 0.98,

            # Professional analysis
            'licensed_engineer_report': 0.90,
            'environmental_assessment': 0.90,

            # Automated with high confidence
            'vlm_high_confidence': 0.75,  # VLM confidence > 0.85
            'ocr_clear_text': 0.80,

            # Automated with medium confidence
            'vlm_medium_confidence': 0.55,
            'feature_matching': 0.60,

            # Heuristics and weak signals
            'color_heuristic': 0.35,
            'shape_heuristic': 0.40,
            'vlm_low_confidence': 0.45,
        }
        return reliability.get(source.method, 0.5)

    def resolve(self):
        """Determine best interpretation via weighted voting"""
        scores = {}

        for interpretation, votes in self.votes.items():
            # Aggregate weighted votes
            score = 0.0
            evidence_list = []

            for vote in votes:
                # Weight by both confidence and source reliability
                contribution = vote.weight * vote.source_reliability
                score += contribution
                evidence_list.append(vote.evidence)

            # Bonus for consensus (multiple independent sources agree)
            if len(votes) > 1:
                # Diminishing returns: 2 sources = 1.5x, 3 sources = 1.8x, etc
                consensus_bonus = 1.0 + 0.5 * math.log(len(votes))
                score *= consensus_bonus

            scores[interpretation] = {
                'score': score,
                'num_sources': len(votes),
                'evidence': evidence_list
            }

        # Sort by score
        ranked = sorted(scores.items(), key=lambda x: x[1]['score'], reverse=True)

        best = ranked[0]
        alternatives = ranked[1:] if len(ranked) > 1 else []

        # Confidence based on margin between top choices
        if len(alternatives) > 0:
            margin = best[1]['score'] - alternatives[0][1]['score']
            confidence = min(0.99, best[1]['score'] / (best[1]['score'] + alternatives[0][1]['score']))
        else:
            confidence = min(0.99, best[1]['score'])

        return {
            'interpretation': best[0],
            'confidence': confidence,
            'score': best[1]['score'],
            'num_sources': best[1]['num_sources'],
            'evidence': best[1]['evidence'],
            'alternatives': [(alt[0], alt[1]['score']) for alt in alternatives]
        }
```

### Example

```python
entity = EntityInterpretation('region_123', polygon)

# Document 1: Aerial photo VLM
entity.add_vote(
    interpretation='wetland',
    evidence=Evidence('aerial.jpg', 'wetland vegetation visible', 0.65, 'vlm_medium_confidence'),
    weight=0.65
)

# Document 2: Site plan OCR
entity.add_vote(
    interpretation='detention_pond',
    evidence=Evidence('site_plan.pdf', 'Label: Detention Basin', 0.90, 'site_plan_label'),
    weight=0.90
)

# Document 3: Engineering drawing
entity.add_vote(
    interpretation='detention_pond',
    evidence=Evidence('engineering.pdf', 'Outlet structure symbol', 0.80, 'ocr_clear_text'),
    weight=0.80
)

# Document 4: Historical aerial (shows it didn't exist before)
entity.add_vote(
    interpretation='detention_pond',  # supports engineered vs natural
    evidence=Evidence('aerial_2010.jpg', 'Not present in 2010', 0.70, 'feature_matching'),
    weight=0.70
)

result = entity.resolve()

# Calculation:
# wetland: 0.65 * 0.55 = 0.36 (single source, no bonus)
# detention_pond:
#   - Vote 1: 0.90 * 0.95 = 0.855
#   - Vote 2: 0.80 * 0.80 = 0.640
#   - Vote 3: 0.70 * 0.60 = 0.420
#   - Subtotal: 1.915
#   - Consensus bonus: 1.915 * (1 + 0.5*log(3)) = 1.915 * 1.55 = 2.97

# Result:
# {
#   'interpretation': 'detention_pond',
#   'confidence': 0.89,
#   'score': 2.97,
#   'num_sources': 3,
#   'evidence': [...],
#   'alternatives': [('wetland', 0.36)]
# }
```

### Advantages
- Simple and intuitive
- Easy to tune source reliability weights
- Clear handling of consensus
- Interpretable results

### Disadvantages
- Doesn't model complex dependencies
- Simple voting may miss subtle reasoning
- No inference capabilities

## Approach 5: Temporal Reasoning with Version Control

### Key Insight
Documents have temporal ordering that helps resolve conflicts.

```python
class TemporalInterpretation:
    def __init__(self, entity_id):
        self.entity_id = entity_id
        self.timeline = []  # chronologically ordered interpretations

    def add_interpretation(self, interpretation, evidence, timestamp):
        """Add interpretation with temporal context"""
        entry = TimelineEntry(
            interpretation=interpretation,
            evidence=evidence,
            timestamp=timestamp,
            supersedes_previous=False
        )

        # Insert in chronological order
        self.timeline.append(entry)
        self.timeline.sort(key=lambda e: e.timestamp)

        # Check if this supersedes previous interpretations
        if self.is_authoritative_update(entry):
            for prev in self.timeline:
                if prev.timestamp < entry.timestamp:
                    prev.superseded = True
                    entry.supersedes_previous = True

    def is_authoritative_update(self, entry):
        """Determine if this is an official update"""
        authoritative_sources = [
            'revised_site_plan',
            'as_built_drawing',
            'amended_permit',
            'field_verification'
        ]
        return entry.evidence.source.type in authoritative_sources

    def resolve_for_date(self, query_date):
        """What was the interpretation as of a specific date?"""
        applicable = [e for e in self.timeline if e.timestamp <= query_date
                     and not e.superseded]

        if not applicable:
            return None

        # Most recent non-superseded interpretation
        return max(applicable, key=lambda e: e.timestamp)

    def get_current_interpretation(self):
        """Current best interpretation"""
        active = [e for e in self.timeline if not e.superseded]

        if not active:
            return self.timeline[-1]  # Latest even if superseded

        # Weight recent interpretations higher
        now = datetime.now()
        scored = []
        for entry in active:
            age_days = (now - entry.timestamp).days
            recency_factor = 1.0 / (1.0 + age_days / 365.0)  # decay over year
            score = entry.evidence.confidence * recency_factor
            scored.append((score, entry))

        return max(scored, key=lambda x: x[0])[1]
```

### Example: Tracking Changes Over Time

```python
entity = TemporalInterpretation('region_123')

# Historical: was wetland
entity.add_interpretation(
    'wetland',
    Evidence('aerial_2015.jpg', 'natural wetland', 0.8, ...),
    timestamp=datetime(2015, 6, 1)
)

# Proposed: plan to convert to detention pond
entity.add_interpretation(
    'detention_pond',
    Evidence('proposed_site_plan_2018.pdf', 'Proposed Detention Basin', 0.9, ...),
    timestamp=datetime(2018, 3, 15)
)

# Constructed: now is detention pond
entity.add_interpretation(
    'detention_pond',
    Evidence('as_built_2020.pdf', 'Constructed Detention Basin', 0.95, 'as_built_drawing'),
    timestamp=datetime(2020, 8, 30)
)

# Query: What was it in 2017?
result_2017 = entity.resolve_for_date(datetime(2017, 1, 1))
# Returns: 'wetland' (only interpretation before that date)

# Query: What is it now?
result_now = entity.get_current_interpretation()
# Returns: 'detention_pond' from as-built (most authoritative + recent)
```

## Recommended Hybrid Approach

Combine multiple strategies:

```python
class MultiSourceReasoner:
    def __init__(self):
        self.entities = {}  # entity_id -> EntityReasoner

    def process_document(self, document):
        """Extract interpretations from a document"""
        regions = extract_regions(document)

        for region in regions:
            entity_id = self.find_or_create_entity(region.geometry)
            reasoner = self.entities[entity_id]

            # Add interpretations with metadata
            for interpretation in region.possible_interpretations:
                reasoner.add_interpretation(
                    interpretation=interpretation.label,
                    confidence=interpretation.confidence,
                    evidence=Evidence(
                        source=document.filename,
                        method=interpretation.method,
                        content=interpretation.supporting_text,
                        timestamp=document.date
                    )
                )

    def resolve_all(self):
        """Resolve interpretations for all entities"""
        results = {}

        for entity_id, reasoner in self.entities.items():
            # Stage 1: Weighted voting (fast, handles most cases)
            voted = reasoner.weighted_vote()

            # Stage 2: If low confidence, try inference
            if voted['confidence'] < 0.7:
                inferred = reasoner.apply_inference_rules()
                if inferred and inferred['confidence'] > voted['confidence']:
                    voted = inferred

            # Stage 3: If still uncertain, flag for human review
            if voted['confidence'] < 0.6:
                voted['requires_review'] = True
                voted['reason'] = 'Low confidence after multi-source reasoning'

            # Stage 4: Check temporal consistency
            if reasoner.has_temporal_data():
                temporal_check = reasoner.verify_temporal_consistency(voted)
                if not temporal_check['consistent']:
                    voted['requires_review'] = True
                    voted['reason'] = f"Temporal inconsistency: {temporal_check['issue']}"

            results[entity_id] = voted

        return results
```

## Practical Implementation Recommendations

### Phase 1: Simple Weighted Voting
Start with Approach 4 (weighted voting):
- Easy to implement
- Intuitive for users to understand
- Covers 70-80% of cases well

### Phase 2: Add Knowledge Base
Incorporate Approach 3 for complex reasoning:
- Define key inference rules for domain
- Handle special cases voting misses

### Phase 3: Full Probabilistic Model
For highest-stakes applications, use Approach 1:
- Rigorous uncertainty quantification
- Publishable/defensible reasoning

### Cross-Cutting: Always Track Provenance
Regardless of approach, maintain:
- What evidence supports each hypothesis
- Where evidence came from
- When it was collected
- How it was derived
- Why alternatives were rejected

This enables:
- Explainability for users
- Debugging when wrong
- Legal defensibility
- Continuous improvement

The key insight you've identified—that semantic understanding requires cross-document reasoning—is fundamental. The representation must support accumulating evidence, resolving conflicts, and maintaining uncertainty until sufficient information arrives.
