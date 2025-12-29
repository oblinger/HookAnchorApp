# Visual-Textual Data Fusion for Environmental Reports

Excellent point - visual data (site plans, aerial photos, sketches, maps) is absolutely central to these reports. Let me think through the visual-textual data fusion problem:

## Option 1: Vision-Language Model with Spatial Anchors

Use a multi-modal LLM (GPT-4V, Claude, Gemini) that processes both together, but **explicitly annotate spatial correspondence**:

```python
{
  "visual_document": {
    "id": "site-plan-001",
    "type": "site_plan|aerial_photo|sketch|cross_section",
    "image_url": "...",
    "scale": "1:500",
    "orientation": "north_up",
    "coordinate_system": "EPSG:4326",

    # Spatial calibration points
    "geo_registration": [
      {"pixel": [100, 200], "coordinates": [lat, lng]},
      {"pixel": [500, 600], "coordinates": [lat, lng]}
    ],

    # Annotated regions of interest
    "regions": [
      {
        "id": "region-1",
        "bbox": [x, y, width, height],  # pixel coordinates
        "polygon": [[x1,y1], [x2,y2], ...],
        "label": "proposed_building_footprint",
        "geometry_world": {...}  # real-world coordinates
      }
    ]
  },

  "annotations": [
    {
      "visual_anchor": {
        "region_id": "region-1",
        "point": [x, y]  # pixel coords
      },
      "textual_data": {
        "description": "Proposed 3-story commercial building",
        "attributes": {
          "floor_area": "15000 sq ft",
          "foundation_depth": "12 ft"
        },
        "linked_entities": ["entity-456"]  # links to unified data model
      },
      "source": "architect_drawings"
    }
  ]
}
```

## Option 2: Detect-Then-Link Pipeline

Multi-stage approach:
1. **Vision model detects/segments** visual features
2. **OCR extracts** text from image
3. **Spatial reasoning** links text callouts to regions
4. **Fusion model** combines with external data

```python
# Stage 1: Vision extraction
{
  "detected_features": [
    {
      "type": "building_outline",
      "confidence": 0.92,
      "bbox": [...],
      "segmentation_mask": "...",
      "visual_embedding": [...]  # for similarity search
    }
  ],

  # Stage 2: Text extraction with locations
  "ocr_results": [
    {
      "text": "Wetland boundary - 50ft setback required",
      "bbox": [x, y, w, h],
      "leader_line": [[x1,y1], [x2,y2]],  # if it points somewhere
      "points_to_region": "region-3"
    }
  ],

  # Stage 3: Spatial relationships
  "spatial_relationships": [
    {
      "subject": "proposed_building",
      "predicate": "is_within_distance",
      "object": "wetland",
      "distance": "50ft",
      "extracted_from": "ocr_results[0]"
    }
  ]
}
```

## Option 3: Embedded Visual References in Text

Store visual data as **first-class entities** that text can reference:

```python
{
  "entity_id": "wetland-area-1",
  "entity_type": "ecological_feature",

  # Standard attributes
  "attributes": {
    "type": "freshwater_wetland",
    "area": "2.3 acres",
    "protected_status": "jurisdictional_wetland"
  },

  # Visual evidence
  "visual_evidence": [
    {
      "document_id": "aerial-2023",
      "regions": ["region-5", "region-6"],
      "visual_description": "Visible standing water and wetland vegetation in northwest corner",
      "confidence": "confirmed",
      "image_patch": "base64..."  # cropped relevant section
    },
    {
      "document_id": "site-sketch",
      "regions": ["hand-drawn-polygon-1"],
      "annotation": "Field delineated wetland boundary per surveyor notes"
    }
  ],

  # Link back to text sources
  "textual_evidence": [
    {
      "source": "USFWS_wetland_inventory",
      "confirms": true
    }
  ]
}
```

## My Recommended Approach: Hybrid Multi-Modal Fusion

**Architecture:**

```
┌─────────────────────────────────────────────────┐
│     Input: Visual + Text Documents              │
└─────────────────┬───────────────────────────────┘
                  │
        ┌─────────┴─────────┐
        │                   │
        ▼                   ▼
  ┌──────────┐      ┌──────────────┐
  │ Vision   │      │ Text         │
  │ Encoder  │      │ Processor    │
  │ (VLM)    │      │ (NER, etc)   │
  └─────┬────┘      └──────┬───────┘
        │                  │
        └────────┬─────────┘
                 ▼
    ┌────────────────────────┐
    │  Spatial Grounding     │
    │  Module                │
    │  - Match text→visual   │
    │  - Geo-register        │
    │  - Resolve references  │
    └──────────┬─────────────┘
               │
               ▼
    ┌────────────────────────┐
    │  Unified Knowledge     │
    │  Graph                 │
    │  (entities + visuals)  │
    └──────────┬─────────────┘
               │
               ▼
    ┌────────────────────────┐
    │  Report Generation     │
    │  LLM with grounded     │
    │  visual references     │
    └────────────────────────┘
```

**Concrete Implementation:**

```python
class MultiModalEnvironmentalData:
    def __init__(self):
        self.visual_index = {}  # indexed visual features
        self.text_index = {}    # structured text data
        self.spatial_graph = {} # spatial relationships

    def ingest_visual(self, image, metadata):
        """Process visual input"""
        # 1. Use VLM to understand content
        vlm_analysis = vision_model.analyze(
            image,
            prompt="""Identify and locate:
            - Building footprints and structures
            - Water bodies and wetlands
            - Vegetation and terrain features
            - Infrastructure (roads, utilities)
            - Any text annotations or callouts
            Return JSON with bounding boxes and geo-coordinates if scale bar present.
            """
        )

        # 2. Extract specific features
        features = segment_and_classify(image)

        # 3. OCR with spatial awareness
        text_elements = spatial_ocr(image)  # includes leader lines

        # 4. Geo-register if possible
        if metadata.get('scale') or metadata.get('geo_markers'):
            transform = compute_geo_transform(image, metadata)

        return {
            "visual_features": features,
            "text_elements": text_elements,
            "geo_transform": transform,
            "vlm_interpretation": vlm_analysis
        }

    def fuse_data(self, visual_data, textual_data, site_location):
        """Link visual and textual information"""

        # Create spatial entities
        entities = []

        # From visual
        for feature in visual_data['visual_features']:
            entity = {
                "id": generate_id(),
                "type": feature['class'],
                "geometry": transform_to_world(feature['bbox'],
                                               visual_data['geo_transform']),
                "visual_evidence": {
                    "image_id": visual_data['id'],
                    "region": feature['bbox'],
                    "confidence": feature['confidence']
                },
                "attributes": {}
            }
            entities.append(entity)

        # Match textual data to visual entities
        for text_item in textual_data:
            if text_item.get('geometry'):
                # Find overlapping visual entities
                matches = find_spatial_overlap(
                    text_item['geometry'],
                    entities
                )

                for match in matches:
                    # Enrich visual entity with text data
                    match['attributes'].update(text_item['attributes'])
                    match['textual_evidence'] = text_item['source']

        return entities

    def generate_report_section(self, entities, section_type):
        """Use LLM to generate report with grounded references"""

        # Prepare context with both modalities
        context = {
            "entities": entities,
            "visual_references": [
                {
                    "id": e['visual_evidence']['image_id'],
                    "crops": crop_image(e['visual_evidence'])
                }
                for e in entities if 'visual_evidence' in e
            ]
        }

        prompt = f"""
        Generate the {section_type} section of the environmental report.

        You have access to:
        - Structured data about site features
        - Visual evidence (site photos, maps, sketches)
        - Government database records

        For each finding, cite both textual sources AND visual evidence.
        Use format: "Wetland area identified in northwest corner
        (visible in aerial photo, Figure 3, confirmed by USFWS database)"

        Data: {json.dumps(context['entities'])}
        """

        # Multi-modal LLM call
        response = llm.generate(
            text=prompt,
            images=context['visual_references']
        )

        return response
```

**Key Innovations:**

1. **Bidirectional Linking**: Visual features link to text data, text data links to visual evidence
2. **Spatial Common Ground**: Everything resolved to real-world coordinates
3. **VLM as First-Class Processor**: Not just OCR, but semantic understanding of drawings
4. **Provenance Through Both Modalities**: Can trace findings to either visual or textual sources
5. **Multi-Modal Context for Generation**: Report-writing LLM sees both structured data AND relevant image crops

**Example Output:**

```markdown
## Wetlands Assessment

A 2.3-acre freshwater wetland is located in the northwest portion of the
site (see Figure 3, aerial photo dated 2023-08-15). The wetland boundary,
field-delineated by certified wetland specialist (red line in Figure 4,
site survey), is confirmed by USFWS National Wetlands Inventory (ID:
NWI-445523).

The proposed building footprint maintains a 75-foot setback from the
wetland boundary (blue outline in Figure 4), exceeding the 50-foot
minimum required by local ordinance.

[Figure 3: Aerial photo with wetland visible]
[Figure 4: Site plan showing wetland boundary and building setback]
```

Does this approach align with how you're thinking about the problem? The key question is whether to do early fusion (VLM processes everything together) vs late fusion (separate processing then link)?
