# Symbio LLM Usage: Specific Models for Specific Tasks

## Overview of LLM Role in Report Generation Pipeline

LLMs are particularly useful for tasks requiring:
- Semantic understanding of visual and textual content
- Natural language generation from structured data
- Cross-referencing and reasoning across documents
- Extracting structured information from unstructured sources
- Quality checking and validation

Let's examine 2-3 specific high-value use cases with concrete model recommendations.

## Use Case 1: Multi-Modal Document Understanding & Grounding

### Task
Process visual documents (site plans, aerial photos, engineering drawings) and extract:
- Semantic labels for regions (wetland, building, road, etc.)
- Spatial relationships ("Building A is 50ft from wetland boundary")
- Text callouts linked to visual features
- Cross-reference information across multiple images

### Recommended Models

#### Primary: Claude 3.5 Sonnet or Opus
**Why Claude:**
- Excellent vision capabilities with high accuracy on technical drawings
- Strong spatial reasoning (understands "northwest corner", "adjacent to", etc.)
- Long context window (200K tokens) - can process many documents in single call
- Good at following structured output formats (JSON)
- Current model you're using for this conversation

**How to use:**
```python
import anthropic

client = anthropic.Anthropic(api_key="...")

# Process site plan + aerial photo together
response = client.messages.create(
    model="claude-3-5-sonnet-20241022",
    max_tokens=4096,
    messages=[{
        "role": "user",
        "content": [
            {
                "type": "image",
                "source": {
                    "type": "base64",
                    "media_type": "image/jpeg",
                    "data": base64_site_plan
                }
            },
            {
                "type": "image",
                "source": {
                    "type": "base64",
                    "media_type": "image/jpeg",
                    "data": base64_aerial_photo
                }
            },
            {
                "type": "text",
                "text": """Analyze these two images of the same site: image 1 is a site plan, image 2 is an aerial photograph.

For each distinct region/feature visible in BOTH images:
1. Identify what it represents (building, wetland, road, parking, vegetation, etc.)
2. Provide bounding boxes in both images [x, y, width, height] as percentages
3. Extract any text labels from the site plan
4. Describe spatial relationships (distances, directions)
5. Note any discrepancies between the two images

Return JSON:
{
  "features": [
    {
      "id": "feature_1",
      "type": "building",
      "label": "Building A (from site plan)",
      "site_plan_bbox": [25, 30, 15, 20],
      "aerial_bbox": [28, 32, 14, 18],
      "confidence": 0.92,
      "spatial_relationships": [
        {"relates_to": "feature_2", "relationship": "50ft north of", "source": "site plan label"}
      ]
    },
    ...
  ],
  "discrepancies": [
    {"issue": "Site plan shows detention pond but not visible in aerial photo",
     "explanation": "Aerial may be older or pond not yet constructed"}
  ]
}"""
            }
        ]
    }
)

# Parse structured output
features = json.loads(response.content[0].text)
```

**Prompt Engineering Tips:**
- Show multiple images in one call for cross-referencing
- Request specific JSON structure for easy parsing
- Ask for confidence scores
- Request spatial information in consistent units (percentages, pixels, or real-world coords)
- Include domain context ("this is for environmental compliance report")

**Cost considerations:**
- Sonnet: ~$3 per million input tokens, ~$15 per million output tokens
- Opus (more accurate): ~$15 per million input tokens, ~$75 per million output
- Images cost based on tokens (roughly 1600 tokens per image)
- Typical site plan analysis: $0.10-0.50 per document pair

#### Alternative: GPT-4o (OpenAI)
**When to use:**
- Need faster processing (GPT-4o is quicker than Claude)
- Integration with existing OpenAI infrastructure
- Slightly lower cost for similar quality

**Differences from Claude:**
- Shorter context (128K tokens vs Claude's 200K)
- Different strengths in spatial reasoning (test both for your use case)
- Structured output mode with JSON schema validation

```python
from openai import OpenAI
client = OpenAI(api_key="...")

response = client.chat.completions.create(
    model="gpt-4o",
    messages=[
        {
            "role": "user",
            "content": [
                {"type": "text", "text": prompt},
                {"type": "image_url", "image_url": {"url": f"data:image/jpeg;base64,{base64_image}"}}
            ]
        }
    ],
    response_format={
        "type": "json_schema",
        "json_schema": {
            "name": "site_features",
            "schema": feature_schema  # Define expected JSON structure
        }
    }
)
```

#### Secondary: Gemini 1.5 Pro (Google)
**When to use:**
- Need to process videos (time-lapse, drone footage)
- Very long context needed (2M tokens)
- Multimodal data including audio (site inspection recordings)

**Unique strengths:**
- Native video understanding (not just frame-by-frame)
- Extremely long context for processing many documents at once
- Good at temporal reasoning

## Use Case 2: Structured Information Extraction from Reports

### Task
Extract key data from existing environmental reports, surveys, and regulatory documents:
- Parse Phase I ESA reports for contamination findings
- Extract soil boring data from geotechnical reports
- Pull compliance information from regulatory letters
- Standardize information into structured database

### Recommended Models

#### Primary: GPT-4o-mini (OpenAI)
**Why mini:**
- Cost-effective for high-volume extraction ($0.15 per million input tokens)
- Fast processing (important for batch operations)
- Sufficient capability for structured extraction tasks
- Can process 128K token context (entire reports)

**How to use:**
```python
from openai import OpenAI
client = OpenAI(api_key="...")

# Extract from Phase I ESA report
with open("phase_i_esa_report.pdf", "rb") as f:
    # First convert PDF to text (using PyPDF2, pdfplumber, etc.)
    report_text = extract_text_from_pdf(f)

response = client.chat.completions.create(
    model="gpt-4o-mini",
    messages=[
        {
            "role": "system",
            "content": """You are an expert at extracting structured data from environmental reports.
Extract information accurately and indicate confidence levels. Use null for missing data."""
        },
        {
            "role": "user",
            "content": f"""Extract key findings from this Phase I Environmental Site Assessment:

{report_text}

Return JSON with this structure:
{{
  "site_info": {{
    "address": "...",
    "parcel_number": "...",
    "site_area_acres": float,
    "current_use": "...",
    "surrounding_land_use": "..."
  }},
  "historical_uses": [
    {{"period": "1950-1985", "use": "battery manufacturing", "source": "Sanborn maps"}},
    ...
  ],
  "recognized_environmental_conditions": [
    {{
      "type": "soil_contamination",
      "contaminant": "lead",
      "location": "northwest corner",
      "severity": "significant|moderate|minor",
      "requires_phase_ii": true,
      "rationale": "..."
    }},
    ...
  ],
  "database_findings": [
    {{
      "database": "EPA CERCLIS",
      "finding": "Site not listed",
      "date_checked": "2024-03-15"
    }},
    ...
  ],
  "recommendations": [
    "Conduct Phase II ESA with soil sampling in northwest area",
    ...
  ],
  "confidence": 0.92,
  "missing_information": ["Phase II data not available", ...]
}}"""
        }
    ],
    response_format={"type": "json_object"}
)

extracted_data = json.loads(response.choices[0].message.content)
```

**Why this works well:**
- Repetitive task with clear structure → doesn't need most powerful model
- High volume → cost matters
- Mini is surprisingly good at extraction when given clear schema
- Can process hundreds of reports per hour at low cost

**Batch Processing for Scale:**
```python
# Process 100 reports overnight
from openai import OpenAI
client = OpenAI(api_key="...")

# Create batch job
batch_requests = []
for report_path in report_paths:
    report_text = extract_text_from_pdf(report_path)
    batch_requests.append({
        "custom_id": report_path,
        "method": "POST",
        "url": "/v1/chat/completions",
        "body": {
            "model": "gpt-4o-mini",
            "messages": [...],  # Same as above
            "response_format": {"type": "json_object"}
        }
    })

# Submit batch (50% cost discount)
batch_file = client.files.create(
    file=jsonl_file_with_requests,
    purpose="batch"
)

batch_job = client.batches.create(
    input_file_id=batch_file.id,
    endpoint="/v1/chat/completions",
    completion_window="24h"
)

# Retrieve results later
# Cost: ~$0.001-0.01 per report depending on length
```

#### Alternative: Claude 3 Haiku
**When to use:**
- Need fast, cheap extraction
- Prefer Anthropic ecosystem
- Processing very high volumes

**Comparison to GPT-4o-mini:**
- Similar performance on extraction tasks
- Lower cost ($0.25 per million input vs $0.15 for GPT-4o-mini, but 5x faster)
- Good for real-time applications

## Use Case 3: Report Generation from Structured Data

### Task
Take structured data (extracted features, database queries, analysis results) and generate:
- Natural language report sections
- Executive summaries
- Findings and recommendations
- Regulatory compliance narratives

### Recommended Models

#### Primary: GPT-4o or Claude 3.5 Sonnet
**Why full-power models:**
- Need coherent long-form writing
- Must maintain technical accuracy
- Should adapt style/tone for audience
- Integrate multiple data sources into narrative

**Example: Generate wetlands assessment section**

```python
# Prepare structured context
context = {
    "site_info": {
        "address": "123 Main St",
        "parcel": "APN 123-456-789",
        "area_acres": 5.2
    },
    "wetland_features": [
        {
            "id": "wetland-001",
            "type": "freshwater_emergent",
            "area_acres": 0.8,
            "location": "northwest corner",
            "nwi_classification": "PEM1C",
            "jurisdictional_status": "likely_jurisdictional",
            "visual_evidence": ["aerial_2024.jpg shows standing water", "vegetation consistent with wetland species"],
            "database_evidence": ["USFWS NWI polygon NWI-445523", "matches location and classification"],
            "buffer_required_feet": 50,
            "setback_compliance": {
                "proposed_building": "compliant",
                "distance_feet": 75
            }
        }
    ],
    "historical_context": "No wetlands visible in 1995 aerial, appeared by 2010",
    "regulatory_framework": "USACE Section 404 permit likely required"
}

response = client.messages.create(
    model="claude-3-5-sonnet-20241022",
    max_tokens=8192,
    messages=[{
        "role": "user",
        "content": f"""You are writing the Wetlands Assessment section of an Environmental Impact Report for a proposed commercial development.

Generate a professional, technically accurate section based on this data:

{json.dumps(context, indent=2)}

Requirements:
1. Start with executive summary paragraph
2. Describe each wetland feature with:
   - Location and characteristics
   - Evidence supporting classification (visual + database)
   - Jurisdictional determination
3. Discuss regulatory requirements
4. Assess project impacts and compliance
5. State recommendations
6. Cite all sources with figure/table references
7. Use professional environmental consulting tone
8. Include uncertainty where appropriate
9. Approximately 800-1200 words

Format in markdown with:
- ## Section heading
- ### Subsection headings
- Figure callouts: (Figure X: description)
- Table references: (Table X)
- Citations: superscript numbers [1]
"""
    }]
)

report_section = response.content[0].text
```

**Expected output quality:**
```markdown
## Wetlands Assessment

### Overview
One freshwater emergent wetland (Wetland-001) totaling approximately 0.8 acres
was identified within the project site in the northwest corner. This wetland
appears to be jurisdictional under U.S. Army Corps of Engineers Section 404
authority and will require appropriate permitting prior to project construction.

### Wetland Identification and Delineation

**Wetland-001** is located in the northwest portion of the 5.2-acre project site
(Figure 3). Visual analysis of current aerial imagery (Figure 4) shows characteristic
wetland indicators including standing water and hydrophytic vegetation. This finding
is corroborated by U.S. Fish and Wildlife Service National Wetlands Inventory data,
which identifies a Palustrine Emergent wetland (classification PEM1C) at this location
(NWI polygon ID: NWI-445523).

Historical aerial photograph review indicates this wetland feature was not present
in 1995 imagery but appeared by 2010, suggesting establishment within the past
15-20 years...

[continues with detailed technical narrative]

### Regulatory Compliance and Project Impacts

The proposed building maintains a 75-foot setback from the wetland boundary,
exceeding the 50-foot buffer required under local ordinance (Municipal Code
Section 18.42). No direct impacts to the wetland are anticipated from the proposed
development. However, because this wetland appears to be jurisdictional, a
Section 404 permit application to the U.S. Army Corps of Engineers is recommended...

### Recommendations
1. Conduct formal wetland delineation by qualified biologist during growing season
2. Submit jurisdictional determination request to USACE
3. If jurisdictional, prepare Section 404 permit application
4. Implement erosion control measures during construction
...
```

**Why this works:**
- Full model handles complex narrative structure
- Integrates multiple data points coherently
- Maintains technical accuracy while being readable
- Adapts style appropriately (formal environmental report)
- Handles nuance ("appears to be jurisdictional" vs definitive statements)

#### Technique: Iterative Refinement

For critical sections, use multi-pass approach:

```python
# Pass 1: Generate draft
draft = generate_section(context, model="claude-3-5-sonnet")

# Pass 2: Self-critique and revise
critique_prompt = f"""Review this draft report section for an Environmental Impact Report:

{draft}

Original data: {json.dumps(context, indent=2)}

Check for:
1. Technical accuracy - are all facts correct per the data?
2. Completeness - did we address all required elements?
3. Clarity - is it understandable to regulators and stakeholders?
4. Compliance - does it meet EIR standards?
5. Appropriate uncertainty - are confidence levels clear?

List any issues found, then provide a revised version that addresses them.
"""

revision = client.messages.create(
    model="claude-3-5-sonnet-20241022",
    messages=[{"role": "user", "content": critique_prompt}]
)

# Pass 3: Technical validation (optional - use mini for cost)
validation_prompt = f"""You are a QA reviewer. Check this environmental report section for errors:

{revision.content[0].text}

Data it's based on: {json.dumps(context, indent=2)}

Verify:
- All numerical values match the source data exactly
- All citations reference provided evidence
- No hallucinated facts (only state what's in the data)
- Regulatory requirements are accurate
- Recommendations follow logically from findings

List any errors found. If none, respond "VALIDATED".
"""

validation = client.messages.create(
    model="gpt-4o-mini",  # Cheaper for validation
    messages=[{"role": "user", "content": validation_prompt}]
)
```

## Model Selection Summary Table

| Task | Model | Cost per Call | Speed | When to Use |
|------|-------|--------------|-------|-------------|
| **Multi-modal document analysis** | Claude 3.5 Sonnet | $0.20-0.50 | Medium | Primary visual understanding, cross-referencing |
| | GPT-4o | $0.15-0.40 | Fast | Alternative, good for speed |
| | Gemini 1.5 Pro | $0.30-0.60 | Medium | Video, very long context |
| **Structured extraction** | GPT-4o-mini | $0.001-0.01 | Very Fast | High-volume extraction, batch processing |
| | Claude 3 Haiku | $0.001-0.01 | Very Fast | Alternative, real-time applications |
| | GPT-4o | $0.05-0.20 | Fast | Complex extraction needing reasoning |
| **Report generation** | Claude 3.5 Sonnet | $0.30-1.00 | Medium | Primary for long-form technical writing |
| | GPT-4o | $0.25-0.80 | Fast | Alternative, good for speed |
| | GPT-4o-mini | $0.05-0.15 | Very Fast | Draft generation, simple sections |

## Specialized Use Cases

### Use Case 4: Regulatory Compliance Checking

**Task:** Verify report meets jurisdiction-specific requirements

**Model:** GPT-4o or Claude 3.5 Sonnet with RAG

```python
# Load regulatory requirements into vector database
# Then query with report content

from openai import OpenAI
client = OpenAI(api_key="...")

# Retrieve relevant regulations
relevant_regs = vector_db.query(
    "What are requirements for wetland assessments in California EIR?",
    top_k=10
)

# Check compliance
response = client.chat.completions.create(
    model="gpt-4o",
    messages=[
        {"role": "system", "content": "You are an environmental regulatory compliance expert."},
        {"role": "user", "content": f"""
        Check if this wetlands assessment section complies with California CEQA requirements.

        Relevant regulations:
        {relevant_regs}

        Report section:
        {report_section}

        Return:
        - Compliant items (✓)
        - Missing items (✗)
        - Recommendations to achieve compliance
        """}
    ]
)
```

### Use Case 5: Cross-Document Fact Checking

**Task:** Validate consistency across multiple documents

**Model:** Claude 3.5 Sonnet (long context advantage)

```python
# Load all relevant documents into single context
documents = {
    "site_plan": site_plan_analysis,
    "aerial_photo": aerial_analysis,
    "phase_i_esa": esa_extract,
    "survey": survey_data,
    "wetland_report": wetland_findings
}

response = client.messages.create(
    model="claude-3-5-sonnet-20241022",
    max_tokens=4096,
    messages=[{
        "role": "user",
        "content": f"""Cross-reference these documents and identify any contradictions or inconsistencies:

{json.dumps(documents, indent=2)}

For each inconsistency found, provide:
1. What the contradiction is
2. Which documents conflict
3. Possible explanations (temporal difference, measurement error, etc.)
4. Recommendation for resolution
5. Severity (critical / moderate / minor)

Return as JSON array of issues.
"""
    }]
)

inconsistencies = json.loads(response.content[0].text)
```

## Cost Optimization Strategies

### 1. Cascade: Start Cheap, Escalate if Needed
```python
def analyze_document_cascading(document):
    # Try with mini first
    result = analyze_with_mini(document)

    if result['confidence'] > 0.85:
        return result  # Good enough

    # Not confident, escalate to full model
    result = analyze_with_sonnet(document)
    return result
```

### 2. Cache Common Contexts
```python
# Claude offers prompt caching for repeated content
response = client.messages.create(
    model="claude-3-5-sonnet-20241022",
    system=[
        {
            "type": "text",
            "text": "You are an environmental report analysis expert...",
            "cache_control": {"type": "ephemeral"}  # Cache this system prompt
        }
    ],
    # ... rest of call
)
# Subsequent calls with same system prompt are 90% cheaper
```

### 3. Batch Processing
- Use OpenAI Batch API for 50% discount on non-urgent processing
- Process reports overnight instead of real-time

### 4. Smart Chunking
- Don't send entire 100-page report to LLM
- Extract relevant sections first (traditional NLP/search)
- Send only pertinent pages

## Practical Recommendations

**Start with:**
1. **Claude 3.5 Sonnet** for visual document understanding (site plans, aerials)
2. **GPT-4o-mini** for structured data extraction from text reports
3. **Claude 3.5 Sonnet** for final report generation

**Scale up as needed:**
- Add Gemini for video/temporal analysis
- Use batching for high-volume extraction
- Implement caching for repeated analysis patterns
- Build RAG system for regulatory compliance checking

**Monitor and optimize:**
- Track cost per report
- Measure accuracy by task
- A/B test different models for specific tasks
- Adjust model selection based on performance data

The key is matching model capability (and cost) to task complexity. Don't use Opus where Haiku will do, but don't skimp on critical analysis tasks where accuracy matters most.
