# Symbio AI Algorithms

## OCR
- I have used PaddleOCR, Hyperscalers have theirs
- donut, Nougat

### Leading OCR Libraries and Models

#### Commercial/Cloud-based
- **Google Cloud Vision API** - Excellent accuracy, handles handwriting, multi-language, returns bounding boxes
- **Amazon Textract** - Optimized for documents, forms, and tables; extracts key-value pairs
- **Azure Computer Vision (Read API)** - Strong for printed and handwritten text, supports 100+ languages
- **Google Document AI** - Specialized for complex document layouts, form parsing

#### Open Source/Self-hosted
- **Tesseract 5.x** - Classic OCR engine, good for clean printed text, supports 100+ languages
- **EasyOCR** - Deep learning-based, 80+ languages, works well with scene text
- **PaddleOCR** - Fast and accurate, excellent for Asian languages, supports text detection + recognition
- **TrOCR (Microsoft)** - Transformer-based, state-of-the-art for printed and handwritten text
- **Surya** - Modern OCR with layout analysis, works across many languages

#### Specialized for Documents/Diagrams
- **LayoutLMv3** - Understands document layout, connects text to visual regions
- **Donut** - Document understanding transformer, no separate OCR needed
- **Nougat** - Specifically designed for scientific documents and PDFs

### Spatial Text Association: Linking Text to Image Regions

The problem: Text labels and callouts need to be associated with specific features/locations in the image.

#### Approach 1: Leader Line Detection
For callout boxes with leader lines pointing to features:

**Algorithm:**
1. **Text Detection + Bounding Box** - Use PaddleOCR or EasyOCR to get text regions
2. **Line/Arrow Detection** - Use Hough Line Transform or deep learning-based line detector
3. **Endpoint Tracking** - Follow leader line from text box to target point
4. **Association** - Link text to the feature at the endpoint

**Libraries:**
- **OpenCV** - Hough Line Transform (`cv2.HoughLinesP`)
- **Detectron2** - Train custom line/arrow detector
- **ArrowDetect** - Specialized arrow detection models

**Example Pipeline:**
```python
# 1. Detect text regions
text_boxes = paddleocr.detect(image)

# 2. Detect lines
lines = cv2.HoughLinesP(edge_image)

# 3. Find lines connected to text boxes
for text_box in text_boxes:
    connected_lines = find_lines_touching_box(text_box, lines)
    endpoint = get_line_endpoint_away_from_box(connected_lines, text_box)

    # 4. Associate with feature at endpoint
    target_feature = find_feature_at_point(endpoint, detected_features)
```

#### Approach 2: Proximity-Based Association
For labels placed near features (without explicit lines):

**Algorithm:**
1. **Text Detection** - Extract all text with bounding boxes
2. **Feature Detection** - Detect visual features (buildings, wetlands, etc.)
3. **Spatial Proximity** - Compute distance from text centroid to feature centroids
4. **Directional Heuristics** - Consider typical label placement (above, below, to the right)
5. **Assignment** - Use Hungarian algorithm or nearest-neighbor with constraints

**Libraries:**
- **scipy.spatial** - KDTree for nearest neighbor search
- **scipy.optimize.linear_sum_assignment** - Hungarian algorithm for optimal matching

**Example:**
```python
from scipy.spatial import KDTree
from scipy.optimize import linear_sum_assignment

# Build spatial index
feature_centroids = [f.centroid for f in features]
tree = KDTree(feature_centroids)

# For each text label
for text in text_labels:
    # Find nearest features
    distances, indices = tree.query(text.centroid, k=3)

    # Apply heuristics (e.g., prefer features in direction label points)
    best_match = apply_placement_heuristics(text, features[indices], distances)
```

#### Approach 3: Vision-Language Model with Spatial Grounding
Modern approach using multi-modal models:

**Models:**
- **GroundingDINO** - Detects objects based on text prompts, returns bounding boxes
- **GLIP** - Grounded Language-Image Pre-training, links phrases to regions
- **Kosmos-2** - Multi-modal LLM with grounding tokens (outputs <box> coordinates)
- **GPT-4V / Claude with spatial prompting** - Can describe spatial relationships

**Algorithm:**
```python
# Use VLM to understand spatial relationships
prompt = """
Analyze this site plan image and return JSON mapping each text label
to the feature it describes. Include:
- label_text: the text content
- label_bbox: [x, y, width, height] of text
- points_to_bbox: [x, y, width, height] of the feature it refers to
- relationship: how they're connected (leader_line, proximity, embedded)
"""

result = vision_language_model(image, prompt)
# Returns structured associations
```

#### Approach 4: Graph-Based Layout Understanding
For complex diagrams with multiple relationships:

**Algorithm:**
1. **Build Scene Graph** - Nodes = text boxes + visual features; Edges = spatial relationships
2. **Detect Connectors** - Lines, arrows, brackets as explicit edges
3. **Infer Implicit Edges** - Proximity, alignment, grouping
4. **Graph Reasoning** - Use GNN or rules to resolve ambiguous associations

**Libraries:**
- **NetworkX** - Graph construction and analysis
- **PyTorch Geometric** - Graph neural networks
- **LayoutParser** - Document layout analysis

**Example:**
```python
import networkx as nx

# Build graph
G = nx.Graph()

# Add nodes
for text in text_boxes:
    G.add_node(text.id, type='text', content=text.text, bbox=text.bbox)
for feature in features:
    G.add_node(feature.id, type='feature', category=feature.class, bbox=feature.bbox)

# Add edges based on leader lines
for line in detected_lines:
    source = find_node_at_point(line.start)
    target = find_node_at_point(line.end)
    G.add_edge(source, target, type='leader_line')

# Add proximity edges
for text_node in text_nodes:
    nearby = find_nearby_features(text_node, max_distance=50)
    for feature in nearby:
        G.add_edge(text_node, feature, type='proximity', distance=compute_distance())

# Resolve associations
associations = resolve_text_to_feature_mappings(G)
```

### Recommended Approach for Environmental Reports

**Hybrid Pipeline:**

1. **OCR with Spatial Info**: PaddleOCR or Google Cloud Vision (returns text + bboxes)
2. **Leader Line Detection**: OpenCV Hough Transform + custom arrow detector
3. **Feature Detection**: Segment features using vision model (SAM, Detectron2)
4. **Association Layer**:
   - Primary: Leader line tracking (explicit associations)
   - Secondary: Proximity-based matching (for labels without lines)
   - Tertiary: VLM verification (GPT-4V validates ambiguous cases)
5. **Graph Assembly**: Build knowledge graph linking text � features � external data

**Why this works:**
- Handles both explicit (callouts with lines) and implicit (nearby labels) associations
- Combines traditional CV (reliable for geometric relationships) with modern VLM (handles ambiguity)
- Produces structured output that integrates with the unified data model
