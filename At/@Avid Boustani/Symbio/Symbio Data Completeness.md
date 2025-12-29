

> How can we tell when we're missing key information either because we didn't pass it from the text or images, or we didn't even get the text or images
in the first place and so now our report is gonna be incomplete or incorrect because we didn't have all the relevant information. What are the ways that
we can detect and mitigate against this?

⏺ Excellent question - this is critical for trustworthy automated report generation. Here are strategies to detect and mitigate missing information:



  Detection Strategies

  1. Schema-Based Completeness Checking

  Define required elements for each report type, then check coverage:

  What to check:
  - Required sections (e.g., every EIA needs wetlands assessment, soil analysis, etc.)
  - Mandatory data points (property boundaries, proposed construction footprint)
  - Regulatory requirements (specific measurements, certifications, tests)

  Implementation:
  Report schema defines:
  - Environmental Impact Report REQUIRES:
    - Property boundary (spatial polygon)
    - Wetland inventory within 500ft
    - Soil contamination screening
    - Protected species habitat check
    - Air quality baseline

  For each requirement:
    IF data_source_available AND extraction_successful:
      status = "complete"
    ELIF data_source_available BUT extraction_failed:
      status = "present but unreadable" → FLAG HIGH PRIORITY
    ELSE:
      status = "missing" → FLAG CRITICAL

  2. Cross-Reference Validation

  Compare information from multiple sources to find gaps:

  Spatial cross-referencing:
  - VLM identifies wetland in aerial photo
  - Check: Is there corresponding USFWS wetland database entry?
  - Mismatch → either database is outdated OR VLM misidentified

  Document cross-referencing:
  - Site plan shows "Building A"
  - Check: Is Building A mentioned in text documents?
  - Check: Does aerial photo show structure at that location?
  - Missing from any source → incomplete information

  Temporal consistency:
  - Database says contamination remediated in 2020
  - Aerial photo from 2024 still shows contaminated appearance
  - Contradiction → need more recent site assessment

  1. Visual Completeness Indicators

  Detect when images don't cover what you need:

  Image quality issues:
  - OCR confidence scores <50% → text is present but unreadable (blur, resolution, handwriting)
  - Large white/blank regions in submitted images → document cropped or pages missing
  - SAM produces very few masks → image too low resolution or overexposed

  Coverage gaps:
  - Site plan shows only 60% of parcel → rest is off-page or not provided
  - Aerial photo at wrong scale → can't identify small features
  - Cross-section drawings referenced in text but not provided

  Annotation incompleteness:
  - Leader lines with no text labels → someone started annotating but didn't finish
  - Text labels with no associated features → references to "see Figure X" but Figure X missing

  2. Entity Relationship Gaps

  Build expected relationship graph, find missing connections:

  Expected relationships:
  - Proposed building → MUST have foundation depth, footprint area, setback distances
  - Wetland → MUST have delineation method, jurisdiction determination, buffer zone
  - Contaminated site → MUST have contamination type, concentration, remediation plan

  Detection:
  For wetland entity "wetland-001":
    Has spatial_boundary: YES
    Has classification: YES (freshwater)
    Has delineation_method: NO ← MISSING
    Has jurisdiction_status: NO ← MISSING

  Flag: "Wetland identified but missing regulatory determinations -
         cannot assess permit requirements"

  3. Regulatory Checklist Compliance

  Each jurisdiction has specific requirements - check against them:

  Example: California CEQA requirements
  - Biological resources survey (within 2 years)
  - Cultural resources assessment
  - Traffic impact analysis
  - Noise study

  Detection:
  - Search all documents for evidence of each required study
  - If not found → "Missing required CEQA documentation: biological survey"

  4. Confidence-Based Gap Detection

  Low confidence scores indicate uncertain or missing information:

  VLM uncertainty:
  - VLM says "possible wetland, confidence 0.45" → need ground-truth verification
  - Multiple regions classified as "unknown" → visual data insufficient

  OCR uncertainty:
  - Critical measurement has low OCR confidence: "setback: 5? ft" (? = uncertain character)
  - Flag for manual verification

  Extraction failures:
  - Tried to extract soil boring data from table, got malformed results → table too complex or image quality poor

  5. Temporal Coverage Gaps

  Environmental data is time-sensitive:

  Age detection:
  - Database queries return data from 2015
  - Aerial photos dated 2018
  - Today is 2024 → data is 6-9 years old, may be outdated

  Seasonal gaps:
  - Wetland delineation requires growing season observation
  - Only have winter aerial photos → may miss seasonal wetlands

  Change detection:
  - Historical imagery shows industrial use until 2010
  - No environmental assessment between 2010-2024 → contamination status unknown

  Mitigation Strategies

  6. Explicit Confidence & Limitation Sections

  Generate report sections that acknowledge gaps:

  Report output:
  ## Data Limitations

  ### Missing Information
  - **Wetland Jurisdictional Determination**: Visual evidence suggests wetland
    present (aerial photo, 2024) but no formal delineation report provided.
    **RECOMMENDATION**: Conduct certified wetland delineation before permit
    application.

  - **Soil Contamination Analysis**: Historical use indicates potential for
    contamination (battery manufacturing 1950-1985) but no Phase II ESA
    available. **CRITICAL GAP**: Cannot assess remediation requirements.

  ### Data Quality Issues
  - Site plan image resolution insufficient to measure precise setback
    distances (±5ft uncertainty). **RECOMMENDATION**: Request CAD files or
    higher resolution scans.

  ### Outdated Information
  - EPA contamination database last updated 2018 (6 years old).
    **RECOMMENDATION**: Verify current site status with recent assessment.

  7. Graduated Confidence Reporting

  Clearly indicate certainty levels:

  ## Wetlands Assessment

  **Confirmed (High Confidence)**
  - 2.3-acre wetland in northwest corner
    - Sources: USFWS database (2022) ✓, Aerial photo (2024) ✓, Site survey (2023) ✓

  **Probable (Medium Confidence)**
  - Additional 0.5-acre seasonal wetland in southeast
    - Sources: Visible in aerial photo ✓, Typical vegetation patterns ✓
    - Missing: Field delineation ✗, Database record ✗
    - Recommendation: Conduct field verification

  **Uncertain (Low Confidence)**
  - Possible jurisdictional waters (drainage channel)
    - Sources: Linear feature visible in aerial ✓
    - Missing: Flow determination ✗, Connectivity analysis ✗
    - Recommendation: Hydrological study required

  8. Interactive Gap-Filling Prompts

  System asks user for missing information:

  During processing:
  MISSING CRITICAL DATA DETECTED:

  The site plan shows a proposed building footprint but the following
  required measurements are not labeled or readable:

  ☐ Building height / number of stories
  ☐ Foundation depth
  ☐ Impervious surface area
  ☐ Distance to property line (east side)

  Actions:
  [Upload additional document] [Enter values manually] [Continue with gaps flagged]

  9. Automated Data Source Querying

  System proactively fetches missing public data:

  When gaps detected:
  - Missing wetland data → automatically query USFWS NWI for site coordinates
  - No contamination info → query EPA Envirofacts, state databases
  - Missing flood data → fetch FEMA flood maps
  - No soil data → retrieve NRCS Web Soil Survey

  Result: Many gaps filled automatically, remaining gaps are truly unavailable data.

  10. Tiered Report Outputs

  Generate different report versions based on available data:

  Tier 1 - Preliminary Assessment (minimal data):
  - Based only on public databases and aerial imagery
  - Many "requires field verification" statements
  - Suitable for initial feasibility only

  Tier 2 - Standard Assessment (typical data):
  - Includes provided site plans, surveys, some studies
  - Some gaps but major conclusions possible
  - Suitable for early planning

  Tier 3 - Comprehensive Assessment (complete data):
  - All required studies, field data, regulatory approvals
  - High confidence in all findings
  - Suitable for permit applications

  11. Dependency Tracking

  Show cascade effects of missing data:

  MISSING: Soil contamination Phase II ESA

  DOWNSTREAM IMPACTS:
  ✗ Cannot determine remediation costs
  ✗ Cannot assess construction worker safety requirements
  ✗ Cannot evaluate groundwater protection needs
  ✗ May delay permit approval (regulatory requirement)
  ✗ Affects site development budget ($?? unknown remediation costs)

  BLOCKING: Final EIR approval, Building permit, Cost estimate

  12. Human-in-the-Loop Validation

  For critical gaps, require expert review:

  Automated screening:
  - System generates draft report
  - Flags 15 gaps/uncertainties
  - Classifies by severity (critical / important / minor)

  Expert review workflow:
  - Environmental engineer reviews flagged items
  - Validates auto-classifications ("yes, this is actually a wetland")
  - Confirms gaps are real ("correct, we don't have boring data")
  - Approves report with documented limitations

  13. Comparative Baseline Analysis

  Use similar completed projects as templates:

  Pattern matching:
  - This is a commercial development in California
  - Typical similar projects required: 12 document types, 8 field studies
  - Current project has: 7 document types, 2 field studies
  - Gap analysis: Missing 5 typical documents, 6 typical studies

  Warning: "This project has less documentation than 85% of similar projects in our database. High risk of incomplete assessment."

  Best Practices Summary

  Detection:
  14. Schema-based completeness checking (what SHOULD be present)
  15. Cross-reference validation (inconsistencies reveal gaps)
  16. Confidence scores (low confidence = uncertain/missing data)
  17. Regulatory checklists (jurisdiction-specific requirements)

  Mitigation:
  18. Explicit limitation sections in reports (transparency)
  19. Graduated confidence levels (clear about uncertainty)
  20. Automated public data fetching (fill what's available)
  21. Interactive gap prompts (user provides missing info)
  22. Tiered reports (match output to input quality)
  23. Expert validation (human oversight for critical gaps)

  Key Principle: Never hide missing information. Better to say "this analysis is incomplete because X is missing" than to generate a confidently wrong
  report. The system should be transparent about its limitations and guide users to fill critical gaps.
