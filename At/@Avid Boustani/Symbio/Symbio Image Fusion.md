# Symbio Image Fusion: Geospatial Coordinate System Challenges

## The Reality: Mostly Uncoordinated Data

**Short answer:** You will **mostly need to align images yourself**. Pre-georeferenced imagery with consistent coordinate systems is the exception, not the rule.

## What You'll Encounter

### Georeferenced (Easy Cases) - ~20% of inputs

**Satellite/Aerial Imagery from Professional Sources:**
- Commercial satellite (Planet, Maxar): Comes with GeoTIFF format, embedded coordinate system (usually WGS84 or local UTM)
- Government imagery (USGS, NAIP): Properly georeferenced, metadata includes projection
- Professional drone surveys: If done by surveying firm, includes GCP (Ground Control Points) and coordinate system

**GIS Data:**
- Shapefiles from government databases (FEMA flood maps, USFWS wetlands, county parcels)
- Always include coordinate system metadata (.prj files)
- Can overlay directly in GIS software

**High-End Engineering Documents:**
- CAD files (DWG/DXF) from civil engineering firms sometimes include coordinate system
- Usually State Plane Coordinates or local survey grid
- May need conversion between coordinate systems but at least have starting point

### Not Georeferenced (Hard Cases) - ~80% of inputs

**Scanned Site Plans:**
- PDF or image scans of architectural/engineering drawings
- Have scale bar ("1 inch = 50 feet") but no real-world coordinates
- Origin (0,0) is arbitrary point on drawing, not lat/long

**Hand-Drawn Sketches:**
- Field notes, markup on printed aerials
- Approximate scale at best
- No coordinate information

**Historical Documents:**
- Old aerial photos from 1950s-1980s
- Sanborn maps from early 1900s
- May have landmarks but no GPS coordinates

**Phone Photos/Consumer Imagery:**
- May have EXIF GPS tags showing where photo was taken
- But image content isn't georeferenced (don't know what part of ground each pixel shows)
- Perspective distortion, no scale

**Scanned Reports:**
- Figures extracted from PDF reports
- Images cropped, rotated, resized from originals
- All metadata lost

## Common Scenarios & Solutions

### Scenario 1: Professional Aerial + Scanned Site Plan
**Reality:** ~40% of projects

**What you have:**
- Georeferenced aerial imagery (GeoTIFF with coordinates)
- Site plan PDF/image with scale bar but no coordinates

**Solution - Manual Georeferencing:**
1. Identify 3-4 common landmarks visible in both (property corners, roads, buildings)
2. Mark corresponding points in each image
3. Compute transformation (affine or polynomial) from site plan pixels to real-world coordinates
4. Now can overlay site plan on aerial imagery

**Tools:**
- GDAL `gdal_translate` and `gdalwarp` for georeferencing
- QGIS georeferencer plugin (GUI for marking control points)
- Custom affine transformation if coding yourself

### Scenario 2: Multiple Scanned Documents, No Georeference
**Reality:** ~30% of projects

**What you have:**
- Site plan (scale 1:500)
- Architectural drawing (scale 1:100, zoomed to building area)
- Survey plat (different orientation, scale 1:1000)
- None have real coordinates

**Solution - Arbitrary Local Reference Frame:**
1. Choose one document as "master" (usually site plan with full property)
2. Create arbitrary coordinate system: origin at property corner, units in feet/meters, north = up
3. Georeference all other documents to this local system
4. Later, if you get GPS coordinates for any landmark, can convert entire system to real-world coords

**Advantage:** Can work with spatial relationships without knowing absolute position on Earth

### Scenario 3: Georeferenced Data + Textual Descriptions
**Reality:** ~15% of projects

**What you have:**
- Georeferenced aerial imagery and GIS shapefiles
- Text reports describing locations: "wetland located approximately 200 feet northwest of property corner"

**Solution - Geocoding Text Descriptions:**
1. Parse spatial relationships from text (distance, direction, reference points)
2. Identify reference points in georeferenced imagery
3. Compute coordinates from descriptions
4. Validate: does "200ft NW" actually show wetland in aerial image?

**Challenge:** Text descriptions are often imprecise, ambiguous, or incorrect

### Scenario 4: Everything Uncoordinated
**Reality:** ~15% of projects

**What you have:**
- Hand-drawn sketch
- Phone photos
- Scanned report with unlabeled figures
- Address/parcel number

**Solution - Build from Scratch:**
1. Use address/parcel to find property in public GIS data (county assessor)
2. Get property boundary polygon with real coordinates
3. Fetch georeferenced aerial imagery for that location
4. Manually align hand-drawn sketch to aerial using landmarks
5. Use VLM to extract spatial relationships from photos and text

**This is the hardest case** - significant human effort required

## Image Alignment Techniques

### Manual Georeferencing (Most Common)
**Process:**
```
1. Load reference image (georeferenced aerial)
2. Load target image (scanned site plan)
3. Identify 3-6 control point pairs:
   - Click property corner in aerial → click same corner in site plan
   - Click road intersection in aerial → click same in site plan
   - Repeat for all recognizable features
4. Compute transformation matrix
5. Apply transformation to align images
```

**Transformation Types:**
- **Affine** (6 parameters): Handles rotation, scaling, shear, translation - works for most site plans
- **Polynomial** (10+ parameters): Handles non-linear distortions - needed for old aerial photos with lens distortion
- **Thin-plate spline**: Handles severe warping - historical maps with irregular distortion

**Accuracy depends on:**
- Quality of control points (can you precisely identify same feature in both images?)
- Number of control points (more = better, minimum 3 for affine)
- Spatial distribution (points spread across image better than clustered)

### Semi-Automated Registration
**Feature-Based Matching:**
1. Detect features in both images (corners, edges, distinctive patterns)
2. Match corresponding features automatically (SIFT, ORB, AKAZE algorithms)
3. Compute transformation from matched features
4. Human validates results

**Works well when:**
- Images have high visual similarity (aerial to aerial from different dates)
- Distinctive features present (buildings, roads, clear landmarks)

**Fails when:**
- Different image types (aerial vs line drawing)
- Significant appearance change (historical to modern)
- Few distinctive features (empty field)

### VLM-Assisted Registration
**Modern approach using vision-language models:**
1. Show VLM both images
2. Ask: "Identify corresponding landmarks visible in both images"
3. VLM returns: "Property NW corner at [pixel coords in image 1] and [pixel coords in image 2]"
4. Use VLM-identified points as control points
5. Human verifies before computing transformation

**Advantage:** Can recognize semantic landmarks even when visually different (e.g., "road intersection" in both sketch and aerial)

### Scale Extraction from Drawings
Many site plans have scale bars but no coordinates:

**Automatic Scale Detection:**
1. OCR to find text like "1 inch = 50 feet" or "Scale: 1:500"
2. Or detect graphical scale bar (line with distance labels)
3. Measure scale bar length in pixels
4. Compute pixels-to-real-world-distance conversion

**Then:**
- Can measure distances/areas in the drawing
- Can align to georeferenced imagery using control points

## Coordinate System Harmonization

Even when data is georeferenced, coordinate systems often differ:

### Common Coordinate Systems Encountered

**Geographic Coordinates:**
- WGS84 (GPS standard): Latitude/Longitude in degrees
- NAD83 (North American Datum): Lat/Long, slightly different from WGS84

**Projected Coordinates:**
- State Plane (US): Different zone for each state/region, units in feet or meters
- UTM (Universal Transverse Mercator): Worldwide, units in meters, different zones
- Local survey grids: Custom coordinate systems for specific projects

### Conversion Required
If you have:
- Aerial imagery in WGS84 lat/long
- County parcel data in State Plane California Zone 5 (feet)
- Engineering drawings in local assumed coordinates

**Must convert all to common system:**
```python
# Using GDAL/proj
from osgeo import osr, gdal

# Define source and target coordinate systems
source = osr.SpatialReference()
source.ImportFromEPSG(4326)  # WGS84

target = osr.SpatialReference()
target.ImportFromEPSG(2229)  # State Plane California Zone 5

# Create transformation
transform = osr.CoordinateTransformation(source, target)

# Convert coordinates
lat, lon = 34.0522, -118.2437  # Los Angeles
x, y, z = transform.TransformPoint(lon, lat)
# Now in State Plane feet
```

**Choosing target system:**
- Use whatever engineering drawings use (if specified)
- Or State Plane for the region (standard for US surveying)
- Or local UTM zone (good for calculations in meters)

## Practical Workflow

### Step 1: Inventory & Categorize
For each input document:
- [ ] Georeferenced? (has coordinate system metadata)
- [ ] Has scale? (can measure distances)
- [ ] Has identifiable landmarks? (can align to reference)
- [ ] Coordinate system if georeferenced

### Step 2: Establish Reference Frame
**Option A:** If any inputs are georeferenced
- Use that coordinate system as reference
- Align everything else to it

**Option B:** If nothing georeferenced
- Fetch georeferenced aerial for site address/parcel
- Use as reference frame
- Align all inputs to aerial

**Option C:** If even location unknown
- Create arbitrary local coordinate system
- Align documents to each other
- Later geo-locate when address/parcel identified

### Step 3: Progressive Alignment
**Priority order:**
1. Start with highest-quality georeferenced data
2. Align site plans that show full property
3. Then align detail drawings to site plan
4. Finally align photos/sketches to aligned drawings

**Validation:**
- Check measurements: does 100ft in site plan = 100ft in aerial?
- Visual inspection: do features overlap properly?
- Cross-reference: do text descriptions match visual alignment?

### Step 4: Uncertainty Propagation
Track alignment quality:
- **High precision** (<1 meter): Professional survey data aligned with GPS control points
- **Medium precision** (1-5 meters): Site plans aligned to aerial imagery via clear landmarks
- **Low precision** (5-20 meters): Approximate alignment from text descriptions or unclear landmarks
- **Very uncertain** (>20 meters): Rough sketch aligned by guessing

**Use in reporting:**
- "Wetland located at coordinates [X, Y] ±3 meters (precision limited by site plan alignment accuracy)"

## Automation Opportunities

### What Can Be Automated

**High Success Rate:**
- Scale bar detection and extraction
- Coordinate system identification from metadata
- Conversion between standard coordinate systems (State Plane to UTM)
- Automatic alignment of multi-date aerial imagery (same sensor, same location)

**Medium Success Rate:**
- Feature-based registration for similar image types
- VLM-assisted control point identification
- Text parsing for coordinate system specifications

**Low Success Rate (Need Human):**
- Aligning hand sketches to aerial imagery
- Dealing with distorted historical documents
- Resolving ambiguous landmarks
- Handling missing/contradictory metadata

### Recommended Hybrid Approach

**Automated preprocessing:**
1. Extract metadata, identify coordinate systems
2. Attempt automatic feature matching
3. Detect scale bars and reference markers
4. Convert to common coordinate system if possible

**Human verification:**
1. Review automatically identified control points
2. Add manual control points where auto-matching failed
3. Validate transformation accuracy
4. Approve or adjust alignment

**Quality metrics:**
- RMS error of control points (lower = better alignment)
- Number of control points used
- Spatial distribution of control points
- Visual confirmation that features overlap

## Special Cases

### Temporal Misalignment
**Problem:** Aerial from 2024, site plan from 2022, survey from 2020
- Buildings may have been constructed/demolished
- Vegetation changed
- Landmarks disappeared

**Solution:**
- Use stable features for alignment (property corners, roads, topography)
- Flag temporal inconsistencies
- Prefer newer data for current conditions

### Multi-Scale Documents
**Problem:** Site plan shows 10-acre property, detail shows 0.1-acre building area
- Different levels of detail
- Different features visible
- Same coordinates, different zoom

**Solution:**
- Align detail to site plan using building outline
- Both then aligned to aerial via site plan
- Hierarchical registration (detail→site→aerial)

### 3D vs 2D
**Problem:** Some data has elevation (LiDAR, topographic maps), some doesn't (site plans)
- Aerial photos show roofs, site plans show footprints
- Terrain slope causes positional offset

**Solution:**
- Use orthophotos (already corrected for terrain) when available
- Or account for relief displacement when using oblique imagery
- Register using features at same elevation (ground-level features)

## Bottom Line

**Plan for manual georeferencing in most cases:**
- Budget time/effort for aligning 80% of inputs
- Build UI for efficient control point marking
- Use VLM to assist but don't rely on full automation
- Track alignment quality and propagate uncertainty
- The 20% that comes pre-georeferenced is a bonus, not the baseline assumption

**Invest in good georeferencing tools:**
- QGIS for manual georeferencing (free, reliable)
- GDAL for programmatic transformations
- Custom UI for common workflows (e.g., "align site plan to aerial")
- Quality control metrics and validation

**Long-term automation path:**
- Start with human-in-loop georeferencing
- Build database of successful alignments
- Train ML models on alignment patterns
- Gradually automate common cases
- Always allow human override for tricky situations
