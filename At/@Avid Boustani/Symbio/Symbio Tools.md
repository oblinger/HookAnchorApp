# Symbio Tools: Specific Technologies for Automated Environmental Report Generation

## Overview

This document lists specific tools, frameworks, and libraries organized by function in the automated environmental report generation pipeline.

---

## Computer Vision Tools

### 1. SAM (Segment Anything Model) - Meta AI
**Purpose:** Visual region segmentation in site plans, aerial photos, drawings

**What it does:**
- Segments any object/region in images without training
- Produces pixel-perfect masks for features (buildings, wetlands, roads)
- Works on diverse image types (photos, sketches, CAD drawings)

**Implementation:**
```python
from segment_anything import sam_model_registry, SamAutomaticMaskGenerator

# Load model
sam = sam_model_registry["vit_h"](checkpoint="sam_vit_h_4b8939.pth")
sam.to(device="cuda")

# Automatic segmentation of entire image
mask_generator = SamAutomaticMaskGenerator(sam)
masks = mask_generator.generate(site_plan_image)

# Result: 50-500 masks representing distinct regions
# Each with: segmentation mask, bbox, area, stability score
for mask in masks:
    polygon = mask_to_polygon(mask['segmentation'])
    area = mask['area']
    bbox = mask['bbox']
```

**Use cases:**
- Extract building footprints from site plans
- Identify wetland boundaries in aerial photos
- Segment parking lots, roads, vegetation areas
- Detect changes between historical and current aerials

**Resources:**
- GitHub: https://github.com/facebookresearch/segment-anything
- Model checkpoint: ~2.4GB download
- GPU recommended (runs on CPU but slower)

**Alternatives:**
- **SAM 2** - Adds video/temporal tracking
- **FastSAM** - YOLO-based, 50x faster but slightly less accurate
- **MobileSAM** - Lightweight for edge devices

---

### 2. PaddleOCR
**Purpose:** Text extraction from technical drawings and scanned documents

**What it does:**
- Detects text regions in images
- Recognizes text (80+ languages)
- Returns text + bounding boxes + confidence scores
- Handles rotated/distorted text

**Implementation:**
```python
from paddleocr import PaddleOCR

# Initialize (download models on first run)
ocr = PaddleOCR(use_angle_cls=True, lang='en', use_gpu=True)

# Process site plan image
result = ocr.ocr(site_plan_path, cls=True)

# Parse results
for line in result[0]:
    bbox = line[0]  # [[x1,y1], [x2,y2], [x3,y3], [x4,y4]]
    text = line[1][0]  # Recognized text
    confidence = line[1][1]  # Confidence score

    print(f"Text: '{text}' at {bbox} (conf: {confidence:.2f})")
    # Example: Text: 'Building A' at [[245, 120], [380, 120], ...] (conf: 0.94)
```

**Use cases:**
- Extract labels from site plans ("Wetland", "Building A", "50ft setback")
- Read dimension annotations
- Parse text from scanned engineering drawings
- Extract data from regulatory documents

**Why PaddleOCR over alternatives:**
- Free and open-source
- Excellent accuracy on technical drawings
- Fast inference
- Handles poor quality scans
- Works offline (local models)

**Resources:**
- GitHub: https://github.com/PaddlePaddle/PaddleOCR
- Models: Auto-downloaded (~100MB)
- CPU/GPU compatible

**Alternatives:**
- **Tesseract 5.x** - Classic OCR, good for clean text
- **EasyOCR** - Similar to PaddleOCR, slightly different strengths
- **Google Cloud Vision API** - Cloud-based, excellent but costs money
- **TrOCR** - Transformer-based, best for handwriting

---

### 3. OpenCV (Open Source Computer Vision Library)
**Purpose:** Image preprocessing, geometric analysis, feature detection

**What it does:**
- Image operations (resize, rotate, threshold, filter)
- Line/shape detection (Hough transform)
- Geometric transformations (align images)
- Feature matching (SIFT, ORB for image registration)

**Implementation:**
```python
import cv2
import numpy as np

# Load site plan and aerial photo
site_plan = cv2.imread('site_plan.jpg')
aerial = cv2.imread('aerial_photo.jpg')

# Example 1: Detect lines (for finding building edges, roads, property lines)
gray = cv2.cvtColor(site_plan, cv2.COLOR_BGR2GRAY)
edges = cv2.Canny(gray, 50, 150)
lines = cv2.HoughLinesP(edges, 1, np.pi/180, threshold=100,
                        minLineLength=50, maxLineGap=10)

# Result: Array of line segments [x1, y1, x2, y2]
for line in lines:
    x1, y1, x2, y2 = line[0]
    cv2.line(site_plan, (x1, y1), (x2, y2), (0, 255, 0), 2)

# Example 2: Image registration (align site plan to aerial photo)
# Detect keypoints
sift = cv2.SIFT_create()
kp1, des1 = sift.detectAndCompute(site_plan_gray, None)
kp2, des2 = sift.detectAndCompute(aerial_gray, None)

# Match features
bf = cv2.BFMatcher()
matches = bf.knnMatch(des1, des2, k=2)

# Filter good matches (Lowe's ratio test)
good_matches = []
for m, n in matches:
    if m.distance < 0.75 * n.distance:
        good_matches.append(m)

# Compute transformation matrix
src_pts = np.float32([kp1[m.queryIdx].pt for m in good_matches])
dst_pts = np.float32([kp2[m.trainIdx].pt for m in good_matches])
matrix, mask = cv2.findHomography(src_pts, dst_pts, cv2.RANSAC, 5.0)

# Align site plan to aerial
aligned = cv2.warpPerspective(site_plan, matrix,
                               (aerial.shape[1], aerial.shape[0]))

# Example 3: Scale bar detection
# Find horizontal lines of specific length, nearby text "scale", etc.
# Extract pixels-to-real-world conversion factor
```

**Use cases:**
- **Image preprocessing:** Denoise scans, enhance contrast, remove shadows
- **Line detection:** Find building edges, property boundaries, roads
- **Image alignment:** Register site plans to aerial photos
- **Scale extraction:** Detect scale bars, compute measurement conversion
- **Leader line tracking:** Follow arrows/lines from labels to features
- **Quality assessment:** Detect blur, check image quality

**Resources:**
- Website: https://opencv.org
- Install: `pip install opencv-python`
- Extensive documentation and tutorials
- C++ library with Python bindings

**Key modules for this project:**
- `cv2.Canny()` - Edge detection
- `cv2.HoughLinesP()` - Line detection
- `cv2.findHomography()` - Image alignment
- `cv2.SIFT_create()` - Feature detection for registration
- `cv2.threshold()` - Image binarization

---

### 4. GDAL (Geospatial Data Abstraction Library)
**Purpose:** Geospatial coordinate transformations, georeferencing, raster/vector processing

**What it does:**
- Read/write 200+ geospatial formats (GeoTIFF, Shapefile, KML, etc.)
- Transform between coordinate systems
- Georeference images (assign real-world coordinates)
- Raster analysis and manipulation

**Implementation:**
```python
from osgeo import gdal, osr, ogr
import numpy as np

# Example 1: Georeference a site plan image
# User has marked 4 control points (pixel coords → lat/long)
control_points = [
    # (pixel_x, pixel_y, world_x, world_y)
    (150, 200, -118.2437, 34.0522),  # NW corner
    (850, 200, -118.2420, 34.0522),  # NE corner
    (850, 600, -118.2420, 34.0510),  # SE corner
    (150, 600, -118.2437, 34.0510),  # SW corner
]

# Create georeferenced GeoTIFF
src_ds = gdal.Open('site_plan.jpg')
dst_ds = gdal.Translate(
    'site_plan_georeferenced.tif',
    src_ds,
    format='GTiff',
    outputSRS='EPSG:4326',  # WGS84
    GCPs=[gdal.GCP(lon, lat, 0, px, py)
          for px, py, lon, lat in control_points]
)

# Warp to apply transformation
gdal.Warp(
    'site_plan_final.tif',
    dst_ds,
    dstSRS='EPSG:4326',
    resampleAlg='cubic'
)

# Example 2: Convert coordinates between systems
source_srs = osr.SpatialReference()
source_srs.ImportFromEPSG(4326)  # WGS84 (lat/long)

target_srs = osr.SpatialReference()
target_srs.ImportFromEPSG(2229)  # State Plane California Zone 5 (feet)

transform = osr.CoordinateTransformation(source_srs, target_srs)

# Convert point
lat, lon = 34.0522, -118.2437
x, y, z = transform.TransformPoint(lon, lat)  # Note: lon, lat order
print(f"State Plane coordinates: {x:.2f}, {y:.2f} feet")

# Example 3: Extract data from georeferenced aerial photo
aerial_ds = gdal.Open('aerial_georeferenced.tif')
geotransform = aerial_ds.GetGeoTransform()

# Convert lat/long to pixel coordinates
def world_to_pixel(lon, lat, geotransform):
    x_origin = geotransform[0]
    y_origin = geotransform[3]
    pixel_width = geotransform[1]
    pixel_height = geotransform[5]

    pixel_x = int((lon - x_origin) / pixel_width)
    pixel_y = int((lat - y_origin) / pixel_height)
    return pixel_x, pixel_y

# Get pixel value at specific location
lon, lat = -118.2430, 34.0518
px, py = world_to_pixel(lon, lat, geotransform)
band = aerial_ds.GetRasterBand(1)
value = band.ReadAsArray(px, py, 1, 1)[0, 0]

# Example 4: Work with vector data (shapefiles)
# Load wetland boundaries from USFWS database
driver = ogr.GetDriverByName('ESRI Shapefile')
wetlands_ds = driver.Open('wetlands.shp', 0)  # 0 = read-only
layer = wetlands_ds.GetLayer()

# Iterate through wetland polygons
for feature in layer:
    geometry = feature.GetGeometryRef()
    area = geometry.GetArea()  # square meters
    wetland_type = feature.GetField('WETLAND_TYPE')

    print(f"Wetland: {wetland_type}, Area: {area:.2f} m²")

    # Check if point is inside this wetland
    point = ogr.Geometry(ogr.wkbPoint)
    point.AddPoint(-118.2430, 34.0518)

    if geometry.Contains(point):
        print("Site intersects this wetland!")
```

**Use cases:**
- **Georeferencing:** Assign real-world coordinates to scanned site plans
- **Coordinate transformation:** Convert between State Plane, UTM, WGS84
- **Spatial analysis:** Check if features overlap, calculate distances
- **Database integration:** Load shapefiles from government databases (wetlands, parcels, flood zones)
- **Format conversion:** Convert between GeoTIFF, KML, Shapefile, etc.

**Resources:**
- Website: https://gdal.org
- Install: `pip install gdal` (can be tricky, use conda: `conda install -c conda-forge gdal`)
- Python bindings for C++ library

**Why essential:**
- Standard for geospatial operations
- Used by all GIS software (QGIS, ArcGIS built on GDAL)
- Handles coordinate system complexity
- Reads government data formats

---

## RAG (Retrieval-Augmented Generation) Frameworks

### 5. LangChain
**Purpose:** Orchestration framework for LLM applications with retrieval

**What it does:**
- Document loading and preprocessing
- Text splitting/chunking
- Vector embeddings integration
- Retrieval chains (query → retrieve → generate)
- LLM integration (OpenAI, Anthropic, etc.)

**Implementation:**
```python
from langchain.document_loaders import PyPDFLoader, DirectoryLoader
from langchain.text_splitter import RecursiveCharacterTextSplitter
from langchain.embeddings import OpenAIEmbeddings
from langchain.vectorstores import Pinecone
from langchain.chat_models import ChatOpenAI
from langchain.chains import RetrievalQA
from langchain.prompts import PromptTemplate
import pinecone

# Step 1: Load regulatory documents
loader = DirectoryLoader(
    'regulations/',
    glob="**/*.pdf",
    loader_cls=PyPDFLoader
)
documents = loader.load()

# Step 2: Split into chunks
text_splitter = RecursiveCharacterTextSplitter(
    chunk_size=1000,
    chunk_overlap=200,
    separators=["\n\n", "\n", ". ", " "]
)
chunks = text_splitter.split_documents(documents)

# Step 3: Create embeddings and store in vector database
embeddings = OpenAIEmbeddings(model="text-embedding-3-small")

pinecone.init(api_key="...", environment="...")
vectorstore = Pinecone.from_documents(
    chunks,
    embeddings,
    index_name="environmental-regulations"
)

# Step 4: Create retrieval chain
llm = ChatOpenAI(model="gpt-4o", temperature=0)

qa_prompt = PromptTemplate(
    template="""You are an environmental regulatory compliance expert.

Use the following regulations to answer the question about report compliance.

Regulations:
{context}

Question: {question}

Answer with specific citations to relevant regulations. Identify compliance
status and any missing requirements.

Answer:""",
    input_variables=["context", "question"]
)

qa_chain = RetrievalQA.from_chain_type(
    llm=llm,
    chain_type="stuff",
    retriever=vectorstore.as_retriever(search_kwargs={"k": 10}),
    chain_type_kwargs={"prompt": qa_prompt},
    return_source_documents=True
)

# Step 5: Query the system
report_section = """
The proposed building maintains a 75-foot setback from the wetland boundary.
A freshwater emergent wetland was identified in the northwest corner...
"""

result = qa_chain({
    "query": f"Does this wetland assessment comply with California CEQA requirements?\n\n{report_section}"
})

print(result['result'])
print("\nSources cited:")
for doc in result['source_documents']:
    print(f"- {doc.metadata['source']}: {doc.page_content[:100]}...")
```

**Advanced features:**
```python
# Multi-query retrieval (generate alternative phrasings)
from langchain.retrievers.multi_query import MultiQueryRetriever

multi_query_retriever = MultiQueryRetriever.from_llm(
    retriever=vectorstore.as_retriever(),
    llm=ChatOpenAI(temperature=0)
)

# Contextual compression (re-rank results)
from langchain.retrievers import ContextualCompressionRetriever
from langchain.retrievers.document_compressors import LLMChainExtractor

compressor = LLMChainExtractor.from_llm(llm)
compression_retriever = ContextualCompressionRetriever(
    base_compressor=compressor,
    base_retriever=vectorstore.as_retriever()
)

# Self-querying retriever (extract metadata filters from query)
from langchain.retrievers.self_query.base import SelfQueryRetriever
from langchain.chains.query_constructor.base import AttributeInfo

metadata_fields = [
    AttributeInfo(
        name="jurisdiction",
        description="Legal jurisdiction (Federal, California, Los Angeles County, etc.)",
        type="string"
    ),
    AttributeInfo(
        name="topic",
        description="Subject area (wetlands, contamination, air quality, etc.)",
        type="string"
    ),
]

self_query_retriever = SelfQueryRetriever.from_llm(
    llm,
    vectorstore,
    "Environmental regulations for site assessments",
    metadata_fields
)

# Query: "What are California wetland setback requirements?"
# Automatically extracts: jurisdiction="California", topic="wetlands"
```

**Use cases:**
- Regulatory compliance checking
- Document Q&A (query historical reports)
- Cross-referencing multiple sources
- Automated citation generation

**Resources:**
- Website: https://langchain.com
- Install: `pip install langchain langchain-openai langchain-anthropic`
- Extensive documentation and cookbook

**Why use LangChain:**
- Abstracts complexity of RAG pipeline
- Integrates with all major vector databases
- Pre-built chains for common patterns
- Active development and community

**Alternatives:**
- **LlamaIndex** - Similar but more focused on data indexing
- **Haystack** - NLP framework with RAG capabilities
- Custom implementation using vector DB directly

---

### 6. Pinecone
**Purpose:** Vector database for semantic search and RAG

**What it does:**
- Store and index vector embeddings at scale
- Fast similarity search (millions of vectors, <100ms query)
- Metadata filtering
- Managed cloud service (serverless)

**Implementation:**
```python
from pinecone import Pinecone, ServerlessSpec
import openai

# Initialize
pc = Pinecone(api_key="...")

# Create index (one-time setup)
index_name = "environmental-regulations"
if index_name not in pc.list_indexes().names():
    pc.create_index(
        name=index_name,
        dimension=1536,  # text-embedding-3-small dimension
        metric='cosine',
        spec=ServerlessSpec(
            cloud='aws',
            region='us-east-1'
        )
    )

index = pc.Index(index_name)

# Insert regulatory text chunks
openai_client = openai.OpenAI(api_key="...")

chunks = [
    {
        "id": "ceqa-15125-1",
        "text": "The environmental setting section shall describe physical environmental conditions...",
        "metadata": {
            "source": "CEQA Guidelines Section 15125",
            "jurisdiction": "California",
            "topic": "environmental_setting",
            "effective_date": "2023-01-01"
        }
    },
    # ... more chunks
]

# Create embeddings and upsert
vectors_to_upsert = []
for chunk in chunks:
    # Generate embedding
    response = openai_client.embeddings.create(
        model="text-embedding-3-small",
        input=chunk["text"]
    )
    embedding = response.data[0].embedding

    vectors_to_upsert.append({
        "id": chunk["id"],
        "values": embedding,
        "metadata": {**chunk["metadata"], "text": chunk["text"]}
    })

# Batch upsert (efficient for large datasets)
index.upsert(vectors=vectors_to_upsert, namespace="regulations")

# Query: Find relevant regulations
query_text = "What are requirements for wetland buffer zones in Los Angeles County?"

query_embedding = openai_client.embeddings.create(
    model="text-embedding-3-small",
    input=query_text
).data[0].embedding

results = index.query(
    vector=query_embedding,
    top_k=10,
    include_metadata=True,
    filter={
        "jurisdiction": {"$in": ["California", "Los Angeles County"]},
        "topic": "wetlands"
    },
    namespace="regulations"
)

# Process results
for match in results.matches:
    print(f"Score: {match.score:.3f}")
    print(f"Source: {match.metadata['source']}")
    print(f"Text: {match.metadata['text'][:200]}...")
    print("---")

# Hybrid search: combine semantic + keyword
# (requires sparse embeddings, separate feature)

# Update/delete vectors
index.update(
    id="ceqa-15125-1",
    metadata={"superseded": "true", "superseded_date": "2024-01-15"}
)

index.delete(ids=["old-reg-1", "old-reg-2"])

# Stats
stats = index.describe_index_stats()
print(f"Total vectors: {stats.total_vector_count}")
print(f"Namespaces: {stats.namespaces}")
```

**Use cases:**
- Store embeddings of all regulatory documents
- Semantic search for relevant regulations
- Filter by jurisdiction, topic, date
- Scale to millions of document chunks

**Pricing:**
- Serverless: Pay per query + storage (~$0.01 per 1M queries)
- Pod-based: Fixed monthly cost (starts ~$70/month for small index)
- Free tier: 100K vectors, limited queries

**Resources:**
- Website: https://www.pinecone.io
- Install: `pip install pinecone-client`
- Documentation and examples

**Why Pinecone:**
- Fully managed (no infrastructure)
- Fast and scalable
- Good filtering capabilities
- Simple API

**Alternatives:**
- **Weaviate** - Open-source, can self-host, hybrid search
- **Qdrant** - Open-source, fast, good for on-premise
- **ChromaDB** - Simple, lightweight, good for prototyping
- **pgvector** - PostgreSQL extension, good if already using Postgres
- **FAISS** - Facebook's library, local/in-memory, fast but not database

---

## Document Processing Tools

### 7. PyMuPDF (fitz)
**Purpose:** PDF text and image extraction

**What it does:**
- Extract text from PDFs (better than PyPDF2)
- Extract images embedded in PDFs
- Get page layout information
- Handle scanned PDFs with OCR

**Implementation:**
```python
import fitz  # PyMuPDF
from PIL import Image
import io

# Open PDF
pdf_document = fitz.open("phase_i_esa_report.pdf")

# Extract text from all pages
full_text = ""
for page_num in range(pdf_document.page_count):
    page = pdf_document[page_num]
    text = page.get_text()
    full_text += f"\n--- Page {page_num + 1} ---\n{text}"

# Extract with layout preservation
page = pdf_document[0]
text_blocks = page.get_text("blocks")  # Returns blocks with coordinates
for block in text_blocks:
    x0, y0, x1, y1, text, block_type, block_no = block
    print(f"Block at ({x0:.1f}, {y0:.1f}): {text[:50]}...")

# Extract images from PDF
for page_num in range(pdf_document.page_count):
    page = pdf_document[page_num]
    image_list = page.get_images()

    for img_index, img in enumerate(image_list):
        xref = img[0]
        base_image = pdf_document.extract_image(xref)
        image_bytes = base_image["image"]
        image_ext = base_image["ext"]  # png, jpg, etc.

        # Save image
        image = Image.open(io.BytesIO(image_bytes))
        image.save(f"page_{page_num}_img_{img_index}.{image_ext}")

        # Or process with CV
        image_array = np.array(image)
        # ... SAM, OCR, etc.

# Extract tables (with layout analysis)
tables = page.find_tables()
for table in tables:
    df = table.to_pandas()
    print(df)

# Get document metadata
metadata = pdf_document.metadata
print(f"Title: {metadata['title']}")
print(f"Author: {metadata['author']}")
print(f"Created: {metadata['creationDate']}")

# Search for text
search_term = "contamination"
for page_num in range(pdf_document.page_count):
    page = pdf_document[page_num]
    text_instances = page.search_for(search_term)

    if text_instances:
        print(f"Found '{search_term}' on page {page_num + 1}")
        for inst in text_instances:
            # inst is a rectangle (x0, y0, x1, y1)
            print(f"  at position: {inst}")

pdf_document.close()
```

**Use cases:**
- Extract text from Phase I ESA reports for LLM processing
- Extract figures/images from engineering drawings
- Parse tabular data (soil boring results, contamination levels)
- Search PDFs for specific terms
- Preserve layout information

**Resources:**
- Documentation: https://pymupdf.readthedocs.io
- Install: `pip install PyMuPDF`
- Fast and feature-rich

**Why PyMuPDF:**
- Much faster than alternatives
- Better text extraction quality
- Image extraction capabilities
- Active development

**Alternatives:**
- **pdfplumber** - Good for tables, slower
- **PyPDF2** - Basic, older
- **pdfminer.six** - Detailed layout info, complex API

---

### 8. Unstructured
**Purpose:** Universal document loader and preprocessor

**What it does:**
- Load 30+ file formats (PDF, DOCX, HTML, images, emails, etc.)
- Automatic element detection (titles, tables, lists, narrative text)
- Chunking with semantic awareness
- OCR integration for images

**Implementation:**
```python
from unstructured.partition.auto import partition
from unstructured.chunking.title import chunk_by_title
from unstructured.staging.base import convert_to_dict

# Auto-detect format and parse
# Works with PDF, DOCX, HTML, images, etc.
elements = partition(filename="phase_ii_esa_report.pdf")

# Elements are typed (Title, NarrativeText, Table, Image, etc.)
for element in elements:
    print(f"{element.category}: {str(element)[:100]}...")

# Example output:
# Title: Phase II Environmental Site Assessment
# NarrativeText: This report presents the findings of subsurface investigations...
# Table: [Table content with soil boring data]
# NarrativeText: Soil samples were collected at depths ranging from...

# Extract only specific element types
tables = [el for el in elements if el.category == "Table"]
narrative = [el for el in elements if el.category == "NarrativeText"]

# Chunk intelligently (respects document structure)
chunks = chunk_by_title(
    elements,
    max_characters=1000,
    combine_text_under_n_chars=100
)

# Convert to dict for storage
chunk_dicts = [convert_to_dict(chunk) for chunk in chunks]

# Save to JSON
import json
with open("parsed_report.json", "w") as f:
    json.dump(chunk_dicts, f, indent=2)

# Process images with OCR
from unstructured.partition.image import partition_image

elements = partition_image(
    filename="site_plan.jpg",
    strategy="hi_res",  # Use OCR
    infer_table_structure=True
)

# Extract metadata
for element in elements:
    metadata = element.metadata
    print(f"Page: {metadata.page_number}")
    print(f"Coordinates: {metadata.coordinates}")
    print(f"Text: {element.text}")
```

**Use cases:**
- Load diverse document types with single API
- Preserve document structure (headings, sections)
- Intelligent chunking for RAG
- Extract tables from scanned reports
- OCR integration

**Resources:**
- Website: https://unstructured.io
- Install: `pip install unstructured[all-docs]`
- Open-source with enterprise version

**Why Unstructured:**
- Handles many formats
- Structure-aware (knows what's a title vs body text)
- Good for heterogeneous document collections
- Active development

---

## Geospatial Analysis Tools

### 9. GeoPandas
**Purpose:** Spatial data analysis in Python (like Pandas but for geographic data)

**What it does:**
- Read/write shapefiles, GeoJSON, etc.
- Spatial operations (intersections, buffers, distance calculations)
- Integration with matplotlib for visualization
- Built on top of Pandas (familiar API)

**Implementation:**
```python
import geopandas as gpd
from shapely.geometry import Point, Polygon
import matplotlib.pyplot as plt

# Load wetland boundaries from USFWS database
wetlands = gpd.read_file("wetlands.shp")

# Load property parcel boundary
parcels = gpd.read_file("county_parcels.shp")
site_parcel = parcels[parcels['APN'] == '123-456-789']

# Check for wetlands on property
wetlands_on_site = gpd.sjoin(wetlands, site_parcel, how='inner', predicate='intersects')

print(f"Found {len(wetlands_on_site)} wetlands intersecting property")

# Calculate areas
wetlands_on_site['area_acres'] = wetlands_on_site.geometry.area / 43560  # sq ft to acres

# Create buffer zones
wetland_buffers = wetlands_on_site.copy()
wetland_buffers['geometry'] = wetlands_on_site.geometry.buffer(50)  # 50-foot buffer

# Check if proposed building is within buffer
building_footprint = Polygon([
    (-118.2435, 34.0520),
    (-118.2432, 34.0520),
    (-118.2432, 34.0518),
    (-118.2435, 34.0518)
])
building_gdf = gpd.GeoDataFrame([1], geometry=[building_footprint], crs="EPSG:4326")

# Convert to same CRS (coordinate reference system)
building_gdf = building_gdf.to_crs(wetlands.crs)

# Check intersection
violations = gpd.sjoin(building_gdf, wetland_buffers, how='inner', predicate='intersects')

if len(violations) > 0:
    print("WARNING: Building violates wetland buffer!")
else:
    print("Building complies with wetland setback requirements")

# Calculate distances
wetlands_on_site['distance_to_building'] = wetlands_on_site.geometry.distance(
    building_gdf.geometry.iloc[0]
)

# Spatial queries
# Find all contaminated sites within 1 mile
contaminated_sites = gpd.read_file("epa_contaminated_sites.shp")
site_center = site_parcel.geometry.centroid.iloc[0]
site_center_buffered = site_center.buffer(5280)  # 1 mile in feet (if using State Plane feet)

nearby_contamination = contaminated_sites[
    contaminated_sites.geometry.intersects(site_center_buffered)
]

# Visualize
fig, ax = plt.subplots(figsize=(12, 8))
site_parcel.boundary.plot(ax=ax, color='black', linewidth=2, label='Property')
wetlands_on_site.plot(ax=ax, color='blue', alpha=0.5, label='Wetlands')
wetland_buffers.boundary.plot(ax=ax, color='blue', linestyle='--', label='50ft Buffer')
building_gdf.plot(ax=ax, color='red', alpha=0.7, label='Proposed Building')
nearby_contamination.plot(ax=ax, color='orange', marker='x', markersize=100, label='Contaminated Sites')
plt.legend()
plt.title('Site Analysis')
plt.savefig('site_analysis_map.png', dpi=300)

# Export results
results = gpd.GeoDataFrame({
    'feature_type': ['wetland'] * len(wetlands_on_site),
    'area_acres': wetlands_on_site['area_acres'],
    'distance_to_building_ft': wetlands_on_site['distance_to_building'],
    'geometry': wetlands_on_site.geometry
})
results.to_file("site_features.geojson", driver='GeoJSON')
```

**Use cases:**
- Load and analyze government GIS data (wetlands, parcels, flood zones)
- Spatial compliance checking (setbacks, buffers)
- Distance calculations
- Overlay analysis (what features intersect property?)
- Generate maps for reports

**Resources:**
- Website: https://geopandas.org
- Install: `pip install geopandas`
- Built on shapely, fiona, pyproj

**Why GeoPandas:**
- Familiar Pandas-like API
- Easy spatial operations
- Integrates with visualization
- Standard tool for Python GIS

---

## Workflow Orchestration

### 10. Prefect
**Purpose:** Workflow orchestration and task scheduling

**What it does:**
- Define data pipelines as Python code
- Handle retries, failures, parallelization
- Monitor task execution
- Schedule recurring jobs (daily database updates, batch processing)

**Implementation:**
```python
from prefect import flow, task
from prefect.tasks import task_input_hash
from datetime import timedelta
import time

# Define tasks (individual operations)
@task(retries=3, retry_delay_seconds=10, cache_key_fn=task_input_hash,
      cache_expiration=timedelta(hours=24))
def download_aerial_imagery(property_address):
    """Download aerial photo for property"""
    print(f"Downloading aerial imagery for {property_address}...")
    # API call to imagery provider
    aerial_image = fetch_from_api(property_address)
    return aerial_image

@task
def process_with_sam(image):
    """Segment image with SAM"""
    print("Running SAM segmentation...")
    masks = run_sam_segmentation(image)
    return masks

@task
def extract_text_with_ocr(image):
    """Extract text labels"""
    print("Running OCR...")
    text_elements = run_paddleocr(image)
    return text_elements

@task(retries=5)  # LLM calls may fail, retry
def classify_regions_with_vlm(image, masks, text_elements):
    """Use VLM to classify regions"""
    print("Classifying regions with Claude...")
    classifications = call_claude_api(image, masks, text_elements)
    return classifications

@task
def query_regulatory_databases(property_location):
    """Search EDR and government databases"""
    print("Querying environmental databases...")
    edr_report = fetch_edr_data(property_location)
    return edr_report

@task
def generate_report_section(classifications, edr_data):
    """Generate report text"""
    print("Generating report with LLM...")
    report = call_gpt4_for_report(classifications, edr_data)
    return report

# Define flow (pipeline)
@flow(name="Environmental Report Generation", log_prints=True)
def generate_environmental_report(property_address, property_location):
    """
    Complete pipeline for generating environmental report
    """
    print(f"Starting report generation for {property_address}")

    # Parallel tasks (run simultaneously)
    aerial_future = download_aerial_imagery.submit(property_address)
    edr_future = query_regulatory_databases.submit(property_location)

    # Wait for aerial, then process
    aerial_image = aerial_future.result()

    # Parallel processing of same image
    masks_future = process_with_sam.submit(aerial_image)
    text_future = extract_text_with_ocr.submit(aerial_image)

    # Wait for both
    masks = masks_future.result()
    text_elements = text_future.result()

    # Sequential: needs previous results
    classifications = classify_regions_with_vlm(aerial_image, masks, text_elements)

    # Wait for EDR data
    edr_data = edr_future.result()

    # Generate final report
    report = generate_report_section(classifications, edr_data)

    print("Report generation complete!")
    return report

# Run the flow
if __name__ == "__main__":
    result = generate_environmental_report(
        property_address="123 Main St, Los Angeles, CA",
        property_location={"lat": 34.0522, "lon": -118.2437}
    )
    print(result)

# Schedule recurring execution
from prefect.deployments import Deployment
from prefect.server.schemas.schedules import CronSchedule

deployment = Deployment.build_from_flow(
    flow=generate_environmental_report,
    name="daily-report-generation",
    schedule=CronSchedule(cron="0 2 * * *"),  # 2 AM daily
    work_queue_name="reports"
)
deployment.apply()
```

**Advanced features:**
```python
# Conditional logic
@flow
def conditional_pipeline(property_type):
    aerial = download_imagery()

    if property_type == "industrial":
        # Extra contamination checks for industrial
        contamination_check = deep_contamination_analysis(aerial)
    else:
        contamination_check = basic_contamination_check(aerial)

    return contamination_check

# Dynamic task mapping (process multiple properties)
@flow
def batch_report_generation(property_list):
    results = process_with_sam.map(property_list)  # Parallel processing
    return results

# Error handling
@task
def fallback_on_error():
    return "Placeholder data when API fails"

@flow
def robust_pipeline():
    try:
        data = expensive_api_call()
    except Exception:
        data = fallback_on_error()
    return data
```

**Use cases:**
- Orchestrate multi-step report generation pipeline
- Handle failures gracefully (retry API calls, fallbacks)
- Parallel processing (download images while querying databases)
- Schedule batch jobs (process 100 reports overnight)
- Monitor pipeline execution

**Resources:**
- Website: https://www.prefect.io
- Install: `pip install prefect`
- Open-source with cloud offering

**Why Prefect:**
- Python-native (define workflows as code)
- Good error handling and retries
- Clear visualization of workflows
- Free tier generous

**Alternatives:**
- **Apache Airflow** - More enterprise, steeper learning curve
- **Dagster** - Data-focused, good for ML pipelines
- **Temporal** - Durable execution, complex workflows

---

## Summary Table

| Category | Tool | Primary Use | Key Strength |
|----------|------|-------------|--------------|
| **Computer Vision** | SAM | Region segmentation | Works on any image type |
| | PaddleOCR | Text extraction | Fast, accurate, free |
| | OpenCV | Image processing | Comprehensive CV toolkit |
| | GDAL | Geospatial operations | Standard for GIS work |
| **RAG** | LangChain | RAG orchestration | Simplifies complex pipelines |
| | Pinecone | Vector database | Managed, scalable |
| **Documents** | PyMuPDF | PDF processing | Fast, feature-rich |
| | Unstructured | Multi-format parsing | Structure-aware |
| **Geospatial** | GeoPandas | Spatial analysis | Pandas-like API |
| **Orchestration** | Prefect | Workflow management | Python-native |

## Typical Tech Stack for Symbio

**Core Pipeline:**
1. **PyMuPDF** - Extract images/text from PDF reports
2. **SAM** - Segment site plans and aerial photos
3. **PaddleOCR** - Extract text labels from drawings
4. **OpenCV** - Image preprocessing and alignment
5. **GDAL** - Georeference images to real-world coordinates
6. **Claude 3.5 Sonnet** - Classify regions, cross-reference documents
7. **LangChain + Pinecone** - Regulatory compliance checking via RAG
8. **GPT-4o-mini** - Extract structured data from text reports
9. **GeoPandas** - Spatial analysis and compliance checking
10. **Prefect** - Orchestrate entire pipeline

**Storage:**
- **PostgreSQL** with **pgvector** - Store structured data + embeddings
- **S3** - Store images, PDFs, generated reports
- **Pinecone** - Regulatory knowledge base

**Deployment:**
- **Docker** - Containerize pipeline
- **FastAPI** - REST API for triggering report generation
- **Prefect Cloud** - Monitor workflows

This stack provides end-to-end automation from document ingestion through spatial analysis to final report generation.
