# SportsVisio Response to PlayHQ AI Video Analytics and Highlights RFP

**Submission Date:** October 10, 2025
**Submitted By:** SportsVisio

---

## Executive Summary

SportsVisio is pleased to submit this proposal to provide AI-powered video analytics and highlights services for PlayHQ's basketball ecosystem, supporting 600,000+ games annually across 30,000 clubs.

{{SEAN:  this section is somewhat duplicative with our solutions section.  We could just keep the summary at the top, and delete this capabilities and differentiations section.  I added it so we could brag, but the doc is getting long!  your choice!}}

**Our Solution:**

SportsVisio delivers a fully white-labeled backend API service that processes basketball game videos and returns structured JSON metadata containing shot events, player statistics, and highlight timestamps. PlayHQ retains complete control over the user experience, building all user-facing features on top of SportsVisio's data outputs.

**Key Technical Capabilities:**

- **Comprehensive Statistics:** Field goals attempted (FGA), field goals made (FGM), field goal percentage (FG%), with player-level attribution and shot-by-shot event data
- **Highlight Metadata:** Precise timestamp data enabling PlayHQ to extract highlight clips from source videos
- **Processing Scale:** Infrastructure designed for 600,000+ basketball games annually with sub-6-hour processing times
- **High Accuracy:** >85% accuracy targets for shot detection, outcome classification, and player attribution
- **RESTful API Integration:** Job-based processing model with webhook notifications and real-time status tracking
- **PlayHQ Data Integration:** Integration with PlayHQ scoring webhooks for enhanced accuracy and continuous model improvement

**Key Differentiators:**

**1. AI-First Sports Analytics Company**

SportsVisio is a technology-forward company focused exclusively on AI-powered sports analytics, not constrained by specific hardware partnerships or venue relationships. Our core competency is computer vision and machine learning, enabling us to:
- Adapt to any video source (mobile uploads, fixed cameras, streaming platforms)
- Continuously improve models based on real-world performance data
- Innovate rapidly without dependencies on proprietary hardware ecosystems
- Scale efficiently across diverse video conditions and environments

**2. Multi-Year, Multi-Sport Model Development**

SportsVisio's competitive advantage lies in our complex web of 12+ interconnected computer vision models refined over multiple years across multiple sports:

- **Years of Training Data:** Models trained on thousands of hours of real-world game footage across basketball, volleyball, baseball, and other sports
- **Cross-Sport Intelligence:** Insights and techniques from one sport inform improvements in others (e.g., ball tracking algorithms benefit from cross-sport refinement)
- **Continuous Refinement:** Models have been battle-tested in production environments, not just research labs
- **Grassroots Expertise:** Specific focus on challenging grassroots video conditions (mobile cameras, variable lighting, amateur setups) rather than just professional broadcast quality

This multi-year development investment creates a defensible moat that cannot be replicated quickly by competitors starting from scratch.

**3. Technology Platform, Not a Service**

SportsVisio provides a scalable technology platform via API, not a manual annotation service:
- Fully automated AI processing with no human-in-the-loop requirements for standard games
- Horizontal scalability through cloud infrastructure (0-1000+ GPUs)
- Consistent quality and processing times regardless of volume spikes
- Cost efficiency that improves with scale, not degrades

**4. White-Label Partnership Model**

SportsVisio operates as transparent backend infrastructure:
- No competing user-facing products in PlayHQ markets
- Complete API-based integration with zero SportsVisio branding
- PlayHQ controls 100% of user experience, pricing, and feature roadmap
- Aligned incentives: SportsVisio succeeds when PlayHQ succeeds

**Value Proposition for PlayHQ:**

- **Rapid Time to Market:** No setup fees, consumption-based pricing, and April 2026 go-live target
- **Cost-Effective Scaling:** Tiered pricing that rewards volume growth while covering early-stage costs
- **Grassroots Focus:** Models specifically trained for community-level basketball video conditions
- **Continuous Improvement:** PlayHQ scoring webhooks integration enables ongoing model enhancement
- **Future-Proof Technology:** Multi-sport expansion roadmap (2 additional sports by 2027)
- **Strategic Partnership:** Dedicated resources, transparent communication, and collaborative development approach

**Commercial Terms:**

- Tiered per-game pricing with volume discounts (0-50K, 50K-150K, 150K-300K, 300K+ games)
- No setup fees, no monthly minimums, no platform fees
- Purely consumption-based model aligned with PlayHQ's growth

**Commitment to Excellence:**

SportsVisio commits to 99.9% API uptime, sub-6-hour processing for standard games, >85% accuracy across all detection tasks, and 24/7 support with 90-minute response times for critical issues.

We view this partnership as a multi-year strategic relationship and are committed to investing in PlayHQ-specific optimizations, model training, and collaborative development to ensure mutual success.

---

## 1. Company Information

### 1.1 Corporate Details
- **Company Name:** SportsVisio
- **Registered Name:** [To be completed]
- **ABN/ACN:** [To be completed]
- **Business Address:** [To be completed]
- **Primary Contact:** [To be completed]
- **Email:** [To be completed]
- **Phone:** [To be completed]

### 1.2 Company Overview
[To be completed with company background, mission, and relevant experience]

### 1.3 Governance Documents
[To be completed - attach or reference relevant governance documentation]


---

## 2. Track Record and Experience

### 2.1 Relevant Experience
[To be completed with description of relevant projects and experience in AI video analytics]

### 2.2 Case Studies
[To be completed with 2-3 detailed case studies demonstrating capability]

#### Case Study 1: [Project Name]
- **Client:** Lifetime Fitness
- **Scope:** [Description]
- **Scale:** [Metrics]
- **Outcomes:** ***Followon Contract, Games annotated,?????*** 

#### Case Study 2: [Project Name]
- **Client:** Play On Sports
- **Scope:** [Description]
- **Scale:** [Metrics]
- **Outcomes:** [Results achieved]

### 2.3 References
[To be completed with client references including contact details]


---

## 3. Technical Solution

### 3.1 Solution Architecture Overview

SportsVisio's platform is built on a RESTful API architecture that enables seamless integration with PlayHQ's white-labelled applications. The system operates through a job-based processing model that provides asynchronous video analysis with real-time status tracking and webhook-based notifications.

**Core Architecture Components:**
- **Job Control API:** RESTful API for submitting video processing jobs and tracking status
- **AI Processing Engine:** Computer vision models for player tracking, event recognition, and statistics extraction
- **Results Delivery System:** JSON metadata delivery with shot events, statistics, and highlight timestamps
- **Data Integration Layer:** Roster synchronization and game metadata management
- **Security Layer:** HTTPS communication, bearer token authentication, and webhook signature validation

The architecture is designed for horizontal scalability, enabling processing of 600,000+ games annually while maintaining sub-6-hour processing times and 99.90% uptime.

See **Appendix B: SportsVisio Job Control API Documentation** and **Appendix C: SportsVisio Highlights Job Specification** for complete technical specifications.

### 3.2 PART 1: Platform Integration & API Development

#### 3.2.1 Integration Specifications and Methodology

**API Integration Approach:**

SportsVisio will provide a RESTful API at `https://ai.sportsvisio-api.com` that PlayHQ applications will integrate with for video processing services.

*Integration Flow:*
1. PlayHQ submits video processing jobs via `POST /v1/jobs` with game video URL and roster data
2. SportsVisio returns job ID and estimated completion time
3. PlayHQ tracks job status via `GET /v1/jobs/{job_id}` or receives webhook notifications
4. Upon completion, PlayHQ retrieves highlight clips, player statistics, and event data

*Key Integration Features:*
- **Job Submission:** Submit basketball game videos with optional roster information for enhanced player identification
- **Status Tracking:** Poll job status or configure webhooks for real-time notifications on job completion/failure
- **Batch Monitoring:** Query all active jobs via `GET /v1/jobs` endpoint
- **Flexible Video Input:** Support for direct MP4 downloads and HTTP Live Streaming (HLS) URLs
- **Roster Integration:** Accept team rosters with jersey numbers to improve player identification accuracy

**Single Sign-On (SSO) Implementation:**

For the PlayHQ integration, we propose two authentication models:

*Option 1: Server-to-Server Authentication (Recommended for Initial Phase)*
- PlayHQ backend services authenticate to SportsVisio API using bearer tokens
- End users remain authenticated within PlayHQ applications only
- SportsVisio API calls made server-side on behalf of users
- Simpler implementation path for January 2026 go-live

*Option 2: Federated SSO (Future Enhancement)*
- SAML 2.0 or OAuth 2.0/OIDC integration for user-level authentication
- Enables direct user access to SportsVisio dashboards from PlayHQ apps
- Supports seamless white-label experience
- Can be implemented in Phase 2 based on use case requirements

**Automated Scheduling and Data Sync:**

*Game Scheduling Integration:*
- PlayHQ provides game schedule data via API or webhook events
- SportsVisio pre-loads game metadata (teams, rosters, scheduled time)
- Automatic job submission when video becomes available post-game
- Status updates pushed back to PlayHQ via webhooks

*Participant Data Synchronization:*
- PlayHQ pushes roster data (team IDs, player jersey numbers) at job submission
- SportsVisio maps detected players to roster entries for accurate attribution
- Player statistics linked to PlayHQ participant IDs for seamless integration
- Support for roster updates during season (trades, number changes)

#### 3.2.2 Authentication and Data Management Approach

**Authentication Security:**
- All API communication over HTTPS/TLS 1.3
- Bearer token authentication using API keys in Authorization header
- Webhook requests signed with HMAC-SHA256 following GitHub's webhook security standard
- API key rotation support with configurable expiration
- Rate limiting with transparent headers (`X-RateLimit-Limit`, `X-RateLimit-Remaining`)

**Data Management:**
- Video URLs accessed securely with time-limited signed URLs (if required)
- Processed data retained for configurable period (90 days default, customizable)
- Automatic data purging in compliance with retention policies
- Data export capabilities for PlayHQ archival requirements
- Participant data handling compliant with Privacy Act 1988 and GDPR

**Error Handling:**
- Standardized HTTP status codes (200, 400, 401, 404, 422, 429, 500)
- Detailed error codes for actionable troubleshooting (e.g., `video_invalid_url`, `video_unsupported_format`)
- Webhook retry policy: 5 attempts with exponential backoff (1, 5, 25, 125, 625 minutes)
- Comprehensive logging for support escalation

#### 3.2.3 Scalability Roadmap

**Current Capacity:**
- Processing infrastructure designed for 600,000+ basketball games annually
- Distributed processing cluster with auto-scaling capabilities
- Average processing time: 3-5 hours per game (well below 6-hour SLA)
- Current deployment handles 10,000+ concurrent jobs

**Path to 600,000+ Basketball Games Annually:**

*Infrastructure Scaling Strategy:*
- **Compute:** Kubernetes-based auto-scaling GPU clusters for AI inference
- **Storage:** Cloud-native object storage (AWS S3/Azure Blob) for video and results
- **Database:** Managed PostgreSQL with read replicas for job status queries
- **CDN:** Global content delivery network for highlight video distribution
- **Monitoring:** Real-time performance monitoring with automated alerting

*Capacity Planning:*
- 600,000 games/year = ~1,600 games/day average
- Peak capacity designed for 5x average (8,000 games/day during tournament seasons)
- Sub-6-hour processing time enables same-day delivery
- Infrastructure auto-scales based on queue depth and processing SLAs

**Multi-Sport Expansion Roadmap:**

SportsVisio is committed to expanding beyond basketball to support PlayHQ's multi-sport ecosystem:

*Year 1 (2026):* Basketball focus with foundation for multi-sport architecture
*Year 2 (2027):* Addition of 2 sports (candidates: Soccer, Netball, AFL, Hockey)

*Expansion Approach:*
- Sport-specific AI models trained on each sport's unique characteristics
- Shared infrastructure and API framework (same job submission model)
- Incremental rollout: pilot → beta → production for each sport
- Sport type specified in job submission (`"sport": "basketball"`, `"sport": "soccer"`, etc.)

#### 3.2.4 White-Label Integration

**SportsVisio is a fully white-labeled backend service provider.**

SportsVisio operates as a transparent data processing layer with no user-facing interfaces. Our integration model is:

**API-Only Architecture:**
- SportsVisio provides RESTful API endpoints for job submission and results retrieval
- All user interactions occur within PlayHQ/MyHoops applications
- PlayHQ controls 100% of the user experience, branding, and presentation
- End users never see or interact with SportsVisio systems directly

**Data Output Format:**
- SportsVisio returns structured JSON data containing:
  - Event timestamps (shot attempts, makes/misses)
  - Player identifications and statistics
  - Video clip URLs hosted on SportsVisio CDN
- PlayHQ builds all UI/UX components using this data
- PlayHQ determines how to display statistics, highlights, and content to users

**White-Label Commitment:**
- SportsVisio will not be mentioned in any user-facing materials (unless PlayHQ chooses otherwise)
- No SportsVisio branding, logos, or attribution in video outputs
- Video URLs can be served from PlayHQ domains (via CNAME/DNS configuration)
- All content presented to users appears as native PlayHQ/MyHoops features

**Video Output Customization:**
- Highlight video clips delivered as unbranded MP4 files
- PlayHQ can add their own branding overlays, intros, outros if desired
- No watermarks or SportsVisio identification in generated content
- CDN delivery supports custom domain configuration for seamless white-labeling

SportsVisio understands that PlayHQ seeks a technology partner, not a co-branded solution. Our role is to provide accurate AI analysis and data that powers PlayHQ's user experience, while remaining invisible to end users.

#### 3.2.6 Performance Commitment

**API Uptime:** 99.90% measured monthly (excluding scheduled maintenance)
- Redundant infrastructure across multiple availability zones
- Automated failover and health monitoring
- Monthly uptime reports provided to PlayHQ

**Processing Capacity:** ≥600,000 basketball games per annum
- Designed for 5x peak capacity headroom
- Auto-scaling infrastructure to handle seasonal spikes
- Queue management with priority processing options

**Multi-Sport Readiness:** 2 additional sports by Year 2 (2027)
- Research and development roadmap established
- Sport selection in collaboration with PlayHQ based on strategic priorities
- Pilot programs in Q3 2026 for sport expansion candidates

### 3.3 PART 2: AI Video Processing & Analytics

#### 3.3.1 AI Technology Specifications

**Computer Vision Technology:**

SportsVisio employs state-of-the-art deep learning models for basketball video analysis, utilizing a sophisticated network of 12+ interconnected algorithms and models:

**Multi-Model Architecture:**
- SportsVisio's system integrates a dozen-plus specialized AI models and algorithms working in concert
- Models feed data to each other in a coordinated pipeline to synthesize accurate basketball statistics and highlights
- Each model is trained specifically for basketball video conditions and grassroots-level game scenarios
- Continuous model updates and refinements based on real-world performance data

**Core Capabilities:**
- **Object Detection & Tracking:** Deep learning models detect and track players, ball, and court elements across video frames
- **Player Identification:** Computer vision algorithms recognize jersey colors and numbers for player attribution
- **Event Recognition:** Temporal action recognition models identify shots, makes, misses, and other basketball events
- **Spatial Analysis:** Court position tracking and player movement pattern analysis
- **Video Processing:** Supports 30 fps video with ability to downsample 60 fps content
- **Game Duration:** Handles games up to 2 hours in length

**Player Tracking System:**

- **Bounding Box Tracking:** Frame-by-frame player location tracking with normalized coordinates (0-1 scale)
- **Identity Persistence:** Maintains player identity throughout game despite occlusions and camera movements
- **Roster Mapping:** When roster provided, maps detected players to specific jersey numbers automatically
- **Color-Based Identification:** Distinguishes teams by jersey color (supports 16 discrete colors including grey, black, green, purple, blue, brown, orange, red, white, yellow, pink, navy, fuchsia, lime, aqua, silver)
- **Trajectory Analysis:** Tracks player movement patterns for shot attempts and defensive positioning

**Event Recognition Capabilities:**

Current basketball event detection includes:
- **Shot Detection:** Identifies all field goal attempts with start/end timestamps
- **Shot Outcome:** Classifies each shot as made or missed (>85% accuracy)
- **Player Attribution:** Links shot attempts to specific players via jersey number recognition
- **Shot Tracking Data:** Provides frame-by-frame bounding boxes for shooter and ball during shot sequence
- **Temporal Precision:** Event timestamps accurate to 0.1 second resolution

**Statistical Analysis Engine:**

From detected shot events, SportsVisio computes comprehensive basketball statistics:
- Field Goals Attempted (FGA) - per player and per team
- Field Goals Made (FGM) - per player and per team
- Field Goal Percentage (FG%) - calculated from FGA/FGM
- Field Goal Types - [requires additional shot classification - 2PT vs 3PT detection]

*Statistics Roadmap (for full PlayHQ requirements):*
- **Current:** FGA, FGM, FG%, player-level shot tracking
- **Phase 2 (Q2 2026):** Assists, rebounds, steals, blocks detection
- **Phase 3 (Q3 2026):** Advanced analytics (shot charts, heat maps, possession stats)

#### 3.3.2 Integration with PlayHQ Data

**Roster and Game Metadata Integration:**

SportsVisio will integrate with PlayHQ's game metadata to improve player identification accuracy:

*Data Flow:*
1. PlayHQ provides game metadata at job submission:
   - Game ID and scheduled time
   - Team names/IDs
   - Player rosters with jersey numbers
2. SportsVisio uses roster data to map detected players to specific participant IDs
3. Results returned include PlayHQ game/player IDs for seamless integration
4. Statistics automatically attributed to correct participants in PlayHQ database

*Data Validation Methods:*
- Cross-reference AI-detected player jersey numbers with provided roster
- Validate detected team colors against expected team uniforms
- Flag anomalies (unexpected jersey numbers, player count mismatches) for review
- Human-in-the-loop validation for edge cases with <80% confidence scores

**PlayHQ Scoring Webhooks and Events Data Integration:**

**SportsVisio will integrate with PlayHQ's scoring webhooks and game API data** as specified in the RFP requirements. This integration provides significant value for both accuracy and continuous improvement:

*Integration Approach:*
- PlayHQ sends real-time or post-game scoring events via webhooks (shot attempts, makes/misses, etc.)
- SportsVisio receives and processes this authoritative scoring data alongside AI video analysis
- Bi-directional data flow enables validation and accuracy enhancement

*Key Benefits:*

1. **Enhanced Accuracy Through Data Fusion:**
   - Combine AI computer vision detections with official scoring data
   - Cross-validate AI predictions against ground truth from scorekeepers
   - Resolve ambiguous cases using authoritative scoring events
   - Achieve higher overall accuracy than AI-only or manual-only approaches

2. **Quality Assurance and Validation:**
   - Real-time accuracy monitoring by comparing AI detections vs. official scores
   - Automatic flagging of discrepancies for review
   - Performance dashboards showing AI accuracy metrics by game, team, venue
   - Continuous quality reporting to PlayHQ

3. **Continuous Model Improvement:**
   - Use PlayHQ's scoring data as ground truth for model training
   - Identify systematic errors and edge cases for targeted improvements
   - Fine-tune models specifically for PlayHQ's video conditions and leagues
   - Build PlayHQ-specific models optimized for their ecosystem

4. **Handling Edge Cases:**
   - When AI confidence is low, defer to official scoring data
   - Fill gaps where video quality prevents AI detection
   - Ensure complete statistics even for challenging video conditions

*Data We Will Accept from PlayHQ:*
- Shot attempt events (timestamp, player, location)
- Shot outcomes (made/missed)
- Game events (fouls, timeouts, substitutions - for future expansion)
- Final box score statistics for validation

*Implementation:*
- Webhook endpoint for real-time event ingestion
- Asynchronous processing of scoring data alongside video analysis
- Reconciliation engine to merge AI and manual data sources
- Confidence-weighted fusion algorithm for optimal accuracy

**Fully Automated AI Capability:**

While the PlayHQ scoring integration provides significant benefits, SportsVisio's system can also operate in **fully autonomous mode** using only video input:

- All shot detection, player tracking, and statistics derived from computer vision AI
- No dependency on manual scoring, shot clocks, or scoreboard data
- Suitable for games without official scorers (e.g., practices, informal games)
- Provides backup capability if scoring data unavailable

This dual-mode approach gives PlayHQ maximum flexibility: enhanced accuracy when scoring data is available, plus autonomous operation when it's not.

#### 3.3.3 Video Processing Workflow

**Mobile Video Upload:**

PlayHQ applications upload videos to their own infrastructure, then submit processing jobs with video URLs:

*Workflow:*
1. User records game video on mobile device via PlayHQ/MyHoops app
2. Video uploads to PlayHQ's video storage (or third-party like Vimeo/YouTube)
3. PlayHQ backend submits processing job to SportsVisio API with video URL and roster
4. SportsVisio fetches video via provided URL (supports MP4 direct links and HLS streams)
5. Processing begins asynchronously

*Video Format Support:*
- Direct MP4 download URLs
- HTTP Live Streaming (HLS) URLs
- Maximum duration: 2 hours
- Recommended resolution: 720p or higher (HD)
- Supported codecs: H.264, H.265

**Automated Processing Pipeline:**

Once a job is submitted, SportsVisio's automated pipeline executes:

1. **Video Ingestion:** Download and validate video format/quality
2. **Preprocessing:** Frame extraction, resolution normalization, quality assessment
3. **AI Analysis:**
   - Player detection and tracking
   - Jersey color/number recognition
   - Shot event detection
   - Ball tracking during shots
4. **Event Extraction:** Generate shot events with timestamps, player IDs, outcomes
5. **Statistics Computation:** Aggregate player and team statistics from events
6. **Results Packaging:** Compile JSON metadata with all events, tracking data, and statistics
7. **Results Delivery:** Return structured JSON results, trigger webhook notification

**Quality Control Mechanisms:**

- **Input Validation:** Check video accessibility, format, duration before processing
- **Confidence Scoring:** Each detected event includes confidence score; low-confidence events flagged
- **Automated QA Checks:**
  - Verify expected number of players detected
  - Check for roster mismatches
  - Validate statistics consistency (e.g., FGM ≤ FGA)
- **Error Recovery:** Automatic retry on transient failures; clear error reporting on permanent failures
- **Human Review Queue:** Low-confidence jobs flagged for optional manual review

#### 3.3.4 Highlight Metadata and Event Data

**SportsVisio Output Format:**

SportsVisio delivers structured JSON metadata that enables PlayHQ to create highlight videos. The system does not generate video files directly; instead, it provides precise event data that PlayHQ can use to extract highlight clips from the source video.

**Event Metadata Provided:**

For each shot event detected, SportsVisio returns:
- **Event ID:** Unique identifier for each shot
- **Timestamps:** Start and end time (in seconds) for the shot sequence
- **Player Attribution:** Player ID linked to the shot attempt
- **Outcome:** Made or missed classification
- **Tracking Data:** Frame-by-frame bounding boxes for shooter and ball (optional)
- **Confidence Scores:** AI confidence level for each detection

**Highlight Creation Approach:**

PlayHQ can use SportsVisio's metadata to create highlights in two ways:

*Option 1: PlayHQ-Generated Highlights (Recommended)*
- PlayHQ uses event timestamps to extract video clips from source video
- Full control over video format, branding, resolution, and presentation
- Can generate horizontal (16:9), vertical (9:16), or square (1:1) formats as needed
- Supports PlayHQ's white-label requirements with complete UX control

*Option 2: Video Clip Generation Service (Future Enhancement)*
- SportsVisio could add video rendering capability to deliver pre-cut highlight clips
- Would require additional development and infrastructure
- Higher per-game processing cost
- Can be discussed if PlayHQ prefers turnkey video delivery

**Data Granularity:**

The JSON output includes:
- **Per-Event Data:** Individual shot events with precise timestamps
- **Per-Player Data:** All events grouped by player for individual highlight creation
- **Per-Team Data:** All events for each team for team highlight reels
- **Aggregate Statistics:** FGA, FGM, FG% computed from detected events

This metadata-driven approach gives PlayHQ maximum flexibility to create highlight experiences tailored to their user interface and brand requirements.

#### 3.3.5 Advanced Statistics

**Current Statistics Capabilities:**

Based on shot event detection, SportsVisio currently generates:

| Statistic | Description | Granularity |
|-----------|-------------|-------------|
| **Field Goals Attempted (FGA)** | Total shot attempts | Per player, per team |
| **Field Goals Made (FGM)** | Successful shots | Per player, per team |
| **Field Goal Percentage (FG%)** | FGM / FGA | Per player, per team |
| **Shot Events** | Individual shot details with timestamps | Per event |
| **Shot Tracking Data** | Ball and shooter bounding boxes during shot | Per frame |

**Roadmap for Full PlayHQ Requirements:**

***{{SEAN:  Please consider this section.  Can we "get away" with delivering more advanced stats later}}***

To meet PlayHQ's requirement for comprehensive statistics (assists, rebounds, steals, blocks), SportsVisio proposes a phased approach:

*Phase 1 (Go-Live - April 2026):*
- FGA, FGM, FG% (current capability)
- Shot-level detail with player attribution
- Highlight videos (team and individual)

*Phase 2 (Q2-Q3 2026):*
- **2PT vs 3PT Detection:** Classify shot distance from court calibration
- **Free Throws:** Detect free throw events separately
- **Assists:** Detect passes leading to made baskets (requires pass detection models)
- **Rebounds:** Detect defensive and offensive rebounds (requires miss → recovery detection)

*Phase 3 (Q4 2026):*
- **Steals:** Detect change of possession events
- **Blocks:** Detect shot block events
- **Turnovers:** Classify turnover types
- **Plus/Minus:** Calculate player impact metrics

This phased approach allows April 2026 go-live with core capabilities while continuously expanding to full feature parity.

#### 3.3.6 Processing Infrastructure

**Computing Infrastructure:**

- **Cloud Platform:** Google Cloud Platform (GCP) as primary platform with multi-region deployment
- **GPU Computing:** NVIDIA GPU infrastructure for AI inference
- **Auto-Scaling:** Currently scales 0-1000 GPUs; will be extended to support PlayHQ's 600,000+based upon agreed upon game volume
- **Container Orchestration:** Kubernetes for auto-scaling and workload management
- **Job Queue:** Distributed job queue with priority scheduling
- **Geographic Distribution:** Multi-region deployment for redundancy and performance

**Data Storage Approach:**

- **Video Storage:** Temporary caching of source videos during processing (auto-deleted post-processing)
- **Results Storage:** Cloud object storage for JSON results and event metadata
- **Database:** Managed database service for job metadata, status tracking, and statistics
- **CDN:** Global content delivery network for low-latency data delivery
- **Retention Policy:** Configurable retention (default 90 days, extendable per PlayHQ requirements)

**Redundancy and Backup:**

- **Multi-Region Deployment:** Infrastructure distributed across multiple geographic regions
- **Database Replication:** Automated backups with point-in-time recovery
- **Storage Redundancy:** Cloud object storage with 99.999999999% durability (11 nines)
- **Disaster Recovery:** Automated failover with <5 minute RTO (Recovery Time Objective)
- **Monitoring:** 24/7 system monitoring with automated alerting

#### 3.3.7 Data Security and Privacy

**GDPR Compliance:**
- Data processing agreements (DPA) provided to PlayHQ
- Privacy-by-design architecture with minimal data collection
- Right to erasure: API endpoint for data deletion requests
- Data portability: Export capabilities in standard formats
- Breach notification procedures within 72 hours

**Privacy Act 1988 Compliance:**
- Australian Privacy Principles (APPs) adherence
- Transparent data handling practices documented
- No collection of biometric identifiers beyond jersey numbers
- Age-appropriate handling of minor participants' data
- Privacy impact assessment conducted and available for review

**Privacy-by-Design Principles:**
- Minimal data collection: Only process data necessary for service delivery
- Purpose limitation: Data used only for video analysis and statistics
- Storage limitation: Automatic data deletion per retention policies
- Transparency: Clear documentation of what data is collected and how it's used

**No Facial Recognition Commitment:**

**SportsVisio explicitly does NOT use facial recognition technology.**

Player identification is performed exclusively through:
- Jersey color detection
- Jersey number recognition (OCR on visible numbers)
- Court position tracking
- Roster matching based on jersey numbers
- Whole-body player identification is trained per-game but these models are not retained nor integrated across games.

No biometric facial data is collected, processed, or stored at any point in the pipeline.

**Australian Data Residency:**

SportsVisio commits to Australian data residency for PlayHQ data:

*Proposed Approach:*
- **Option 1:** Deploy dedicated infrastructure in AWS Sydney region (ap-southeast-2)
  - All video processing, storage, and databases hosted in Australia
  - Data never leaves Australian borders
  - Higher infrastructure cost (reflected in pricing)

*Option 2:* Hybrid approach with AU metadata storage
  - Processing may occur in global infrastructure for cost efficiency
  - All participant PII and game metadata stored in AU region only
  - Transient video processing data auto-deleted

PlayHQ can select preferred approach based on compliance requirements and budget considerations.

#### 3.3.8 Quality Assurance and Error Handling

**Quality Assurance Process:**

1. **Automated Testing:**
   - Regression test suite on curated basketball video library
   - Accuracy benchmarks measured against ground truth annotations
   - Performance testing for processing time SLAs

2. **Continuous Monitoring:**
   - Real-time accuracy metrics tracked per job
   - Anomaly detection for unusual patterns (e.g., zero shots detected)
   - A/B testing of model improvements before production deployment

3. **Human Validation:**
   - Sample-based spot checks by QA team
   - Validation of low-confidence predictions
   - Feedback loop for continuous model training

**Error Detection and Handling:**

- **Input Validation Errors:** Immediate rejection with clear error messages (e.g., `video_invalid_url`, `video_unsupported_format`)
- **Processing Failures:** Automatic retry (up to 3 attempts) with exponential backoff
- **Partial Failures:** Best-effort processing (e.g., if some events detected but full game not processed, return partial results with warning)
- **Timeout Protection:** Jobs exceeding expected processing time flagged and escalated to engineering team

**Continuous Improvement:**

- Monthly accuracy reports shared with PlayHQ
- Quarterly model updates based on new training data
- Customer feedback integration (reported inaccuracies used to improve models)
- PlayHQ-specific model fine-tuning using their video conditions and feedback

#### 3.3.9 Performance Commitment

**Processing Time:** Less than 6 hours average turnaround job submission to completion.
(6-hour turn around assumes active game time is less than 60 minutes.  Longer active games will require additional time.)

**Event Recognition Accuracy:** >85% for shot detection
- **Current Performance:** 88-92% shot detection accuracy on test datasets
- **Measurement:** We are measuring precision, recall, and F1 score against human-annotated ground truth.  All three must be above 85%
- **Continuous Improvement:** We will be performing model updates at least quarterly to maintain/improve accuracy.

**Statistics Accuracy:** >85% for player attribution and shot outcomes
- **Current Performance:** 90% player attribution accuracy when roster provided
- **Validation:** Cross-validation with PlayHQ scoring webhooks when available
- **Quality Guarantee:** Accuracy guarantees with remediation clauses per SLA

**Data Output Quality:** Accurate event metadata matching source video
- **Input Support:** 480p minimum, 1080p recommended source video
- **Timestamp Precision:** Event times accurate to 0.1 second resolution
- **Coordinate System:** Normalized bounding boxes (0-1 scale) relative to source video dimensions
- **Metadata Format:** Structured JSON conforming to published schema (see Appendix C)



#### 3.3.10 Input Video Quality Requirements

**SportsVisio's performance guarantees (processing time, accuracy thresholds, event detection) are predicated on input video meeting minimum quality standards.**

This section defines the video quality requirements necessary to achieve the specified SLA targets. SportsVisio's system is designed to degrade gracefully when input quality falls below these thresholds, but accuracy and completeness guarantees only apply to videos meeting these criteria.

**Minimum Video Quality Requirements:**

{{JUAN: Please fill in the following minimum requirements:}}
- **Resolution:** [e.g., 480p minimum, 720p or higher recommended]
- **Frame Rate:** [e.g., 24 fps minimum, 30 fps recommended]
- **Bitrate/Compression:** [e.g., minimum bitrate requirements to avoid excessive compression artifacts]
- **Lighting Conditions:** [e.g., sufficient court lighting to distinguish players and ball]
- **Camera Position:** [e.g., elevated side-court view covering full court preferred; avoid floor-level or behind-backboard angles]
- **Camera Stability:** [e.g., fixed camera position preferred; handheld video acceptable with moderate stabilization]
- **Video Codec Support:** [e.g., H.264, H.265 supported; other codecs may not process correctly]
- **Audio Requirements:** [None - audio not required for processing]

**Degraded Performance Scenarios:**

When input video quality falls below minimum requirements, the following may occur:
- **Reduced Accuracy:** Shot detection and player identification accuracy may drop below 85% targets
- **Incomplete Event Detection:** Some events may not be detected (e.g., shots obscured by poor lighting or camera angles)
- **Processing Delays:** Lower quality video may require additional processing time or manual review
- **Increased Confidence Warnings:** More events flagged as low-confidence requiring validation

**Graceful Degradation:**

SportsVisio's system will attempt to process all submitted videos and return best-effort results even when quality requirements are not fully met. However:
- Quality warnings will be included in API responses indicating detected quality issues
- Low-confidence events will be flagged for optional human review
- No SLA penalties apply when video quality issues are root cause of accuracy or processing delays

**Quality Assessment:**

- Automated quality assessment performed at job intake
- Quality metrics included in job status responses
- PlayHQ notified of quality concerns via webhook or API response
- Guidance provided on corrective actions (e.g., re-recording with better lighting)


### 3.4 PART 3: Data Outputs for User Engagement & Content Distribution

**SportsVisio's Role:**

SportsVisio provides backend AI analysis and structured data outputs that enable PlayHQ to build user engagement features. All user-facing interfaces, dashboards, highlight videos, and content distribution systems are owned, built, and operated by PlayHQ using SportsVisio's data.

**Note:** A detailed list of potential user engagement features that PlayHQ could build using SportsVisio's data has been extracted to a separate reference document for ideation purposes.

#### 3.4.1 JSON Data Outputs

For each basketball game processed, SportsVisio returns structured JSON metadata containing:

**Shot Event Data:**
- Unique event ID for each detected shot
- Start and end timestamps (in seconds) for video clipping
- Player ID attribution (linked to PlayHQ participant database)
- Shot outcome: made or missed
- Optional: Frame-by-frame tracking data (shooter and ball bounding boxes)
- Confidence scores for each detection

**Player and Team Data:**
- Player roster with jersey numbers and colors
- Team identifications
- Player-to-team mappings

**Aggregate Statistics:**
- Per-player statistics: FGA, FGM, FG%
- Per-team statistics: FGA, FGM, FG%
- Game-level summary data

#### 3.4.2 PlayHQ Integration Points

**What PlayHQ Will Build:**

Using SportsVisio's JSON outputs, PlayHQ will develop:

1. **Video Clip Extraction:** Use event timestamps to extract highlight clips from source videos
2. **User Interfaces:** Build web and mobile UIs to display statistics, highlights, and player profiles
3. **Content Distribution:** Create systems for sharing, downloading, and embedding highlights
4. **Engagement Features:** Implement gamification, badges, dashboards, and social features
5. **Analytics:** Track user engagement, viewership, and content performance

**SportsVisio Support:**

- API documentation and integration support
- JSON schema specifications (see Appendix C)
- Technical consultation on optimal use of tracking data
- Example implementations and best practices
- Ongoing API updates and enhancements

#### 3.4.3 Data Retention and Access

**Data Storage:**
- JSON results stored for configurable retention period (default 90 days, extendable)
- PlayHQ can download and archive data for long-term storage
- API provides query access to historical game data within retention window

**Data Ownership:**
- Source videos remain property of PlayHQ/users
- SportsVisio-generated metadata licensed to PlayHQ for unrestricted use within their platform
- PlayHQ controls all aspects of data presentation, privacy settings, and user access
- No SportsVisio branding or attribution required in PlayHQ's user-facing features


---

## 4. Commercial Proposal

### 4.1 Pricing Structure

#### 4.1.1 Per-Game Processing Fees

SportsVisio proposes a tiered volume-based pricing model designed to support PlayHQ's growth while ensuring cost recovery during initial deployment phases:

**Volume Tier Structure:**

| Annual Volume | Per-Game Rate | Notes |
|---------------|---------------|-------|
| 0 - 50,000 games | $[PRICE_TIER_1] | Higher initial pricing to cover infrastructure setup and early-stage costs |
| 50,001 - 150,000 games | $[PRICE_TIER_2] | Mid-tier pricing as volume scales |
| 150,001 - 300,000 games | $[PRICE_TIER_3] | Reduced rate reflecting operational efficiencies |
| 300,001+ games | $[PRICE_TIER_4] | Volume discount for scale operations |

**Pricing Rationale:**

The tiered structure recognizes that:
- Initial deployment requires significant infrastructure investment and integration effort
- Early-stage volumes may not achieve full economies of scale
- Higher per-game pricing in early tiers ensures cost recovery while PlayHQ builds adoption
- Progressive discounts incentivize growth and reward volume commitment
- At target scale (600,000+ games), pricing reaches highly competitive rates

**Billing:**
- Monthly invoicing based on actual games processed
- Volume tiers calculated on rolling 12-month basis
- Automatic tier progression as cumulative volume increases

#### 4.1.2 Setup and Implementation Costs

**No Setup Fees:**

SportsVisio will absorb all setup and implementation costs as part of this partnership, including:
- API integration development and testing
- Webhook endpoint configuration
- PlayHQ-specific customizations
- Quality assurance and acceptance testing
- Technical documentation and training
- Onboarding support

This approach demonstrates SportsVisio's commitment to the PlayHQ partnership and removes barriers to rapid deployment.

#### 4.1.3 Ongoing Costs

**No Monthly Platform Fees:**

Pricing is purely consumption-based (per-game processing fees only). There are no:
- Monthly minimum fees
- Platform access fees
- API usage fees
- Support or maintenance fees
- Infrastructure surcharges

**Additional Services (Optional):**

The following services are available on request at additional cost:
- Custom model training for specific venues or leagues: [Quote on request]
- Priority processing (< 3 hour turnaround): [Premium pricing TBD]
- Enhanced support SLAs (24/7 coverage): [Quote on request]
- Data archival beyond standard retention: [Storage fees TBD]

### 4.2 Commercial Model Alignment

**Alignment with PlayHQ's Objectives:**

SportsVisio's pricing model is designed to support PlayHQ's strategic goals:

1. **Low Barrier to Entry:** No setup fees or monthly minimums enable rapid launch without upfront capital investment

2. **Risk Sharing:** Consumption-based pricing means costs scale directly with PlayHQ's adoption and success

3. **Incentivized Growth:** Volume discounts reward PlayHQ for driving adoption across their 30,000 club network

4. **Cost Predictability:** Simple per-game pricing with transparent tiers enables accurate forecasting

5. **Grassroots Focus:** Pricing structure supports PlayHQ's mission to democratize advanced analytics at community level

**Revenue Model:**

SportsVisio charges PlayHQ on a per-game basis for AI analysis and metadata delivery. PlayHQ independently determines pricing for their end users (clubs, participants, associations) and retains all revenue from their value-added services built on SportsVisio data.

**Volume Commitment:**

While this proposal uses tiered pricing, SportsVisio does not require minimum volume commitments. PlayHQ pays only for games actually processed, providing maximum flexibility during launch and growth phases.

---

## 5. Service Level Agreements

### 5.1 Performance Guarantees

#### 5.1.1 Processing Time

**Guaranteed Delivery Time:**
- **Games ≤60 minutes active game time:** Processing completed within 6 hours of job submission
- **Games >60 minutes active game time:** Processing time not guaranteed (best effort basis)

**Calculation:**
- Processing time measured from job submission (via `POST /v1/jobs`) to webhook notification of completion
- 6-hour guarantee applies to games with active game footage of 60 minutes or less
- Longer games (e.g., overtime, extended tournaments) processed on best-effort basis with estimated completion times provided

**Scope:**
- Guarantee assumes valid video format and accessibility
- Excludes delays caused by video upload issues, invalid URLs, or corrupted source files
- Excludes force majeure events (cloud provider outages, natural disasters, etc.)

#### 5.1.2 System Uptime

**API Availability Guarantee:**
- **99.9% uptime** for SportsVisio API endpoints
- Measured monthly across all API operations (job submission, status queries, result retrieval)

**Infrastructure Resilience:**
- Multi-region GPU compute availability across all major cloud regions (Google Cloud Platform primary)
- Automatic failover between geographic regions to ensure compute capacity
- Dynamic resource allocation across global cloud infrastructure to maintain availability

**Uptime Measurement:**
- Calculated as: `(Total Minutes in Month - Downtime Minutes) / Total Minutes in Month × 100`
- Excludes scheduled maintenance windows (announced 48 hours in advance during periods of low usage, max 8 hours/quarter)
- Excludes downtime caused by PlayHQ's infrastructure or third-party services

**GPU Availability Commitment:**
- SportsVisio commits to sourcing GPU compute capacity from all major geographic cloud regions
- Maintains relationships with multiple cloud providers to maximize GPU availability
- Automatic workload distribution across available regions to maintain processing capacity


{{SEAN:  I think we do NOT put these into the contract.}}
<!-- DECISION NEEDED: Should we include service credit remediation for uptime violations?
     PROPOSAL: Do not offer service credits or financial remediation for SLA violations in initial contract.
     Rationale: Maintains pricing competitiveness; focuses on technical excellence over contractual penalties.
     Alternative: Could offer 10% monthly credit if uptime falls below 99.9%, subject to discussion. -->

#### 5.1.3 Support Response Times

**Support Channels:**

SportsVisio provides multi-channel support access:
- Email: support@sportsvisio.com
- Voice/Telephone: [Phone number TBD]
- WhatsApp: [WhatsApp business number TBD]
- Slack: Dedicated shared channel for PlayHQ integration support

**Response Time Commitments:**

| Severity Level | Definition                                                                    | Response Time | Support Hours  |
| -------------- | ----------------------------------------------------------------------------- | ------------- | -------------- |
| **Critical**   | API outage, complete service disruption, data loss                            | 90 minutes    | 24/7           |
| **High**       | Significant performance degradation, processing delays affecting >10% of jobs | 4 hours       | 24/7           |
| **Medium**     | Isolated processing failures, accuracy concerns, feature requests             | 24 hours      | Business hours |
| **Low**        | General inquiries, documentation requests, minor issues                       | 48 hours      | Business hours |

**Support Coverage:**

- **24/7 Support:** Available during weekends and any period where significant portal usage is occurring
- **Business Hours:** Monday-Friday, 9am-6pm AEST (for non-critical issues)
- **Escalation:** Critical issues automatically escalate to engineering team with 90-minute response SLA
- **On-Call:** Dedicated on-call engineer for critical incidents during off-hours

**Response vs. Resolution:**
- Response time = Initial acknowledgment and triage
- Resolution time depends on issue complexity; estimated timeline provided in initial response
- Critical issues prioritized for same-day resolution when possible

#### 5.1.4 Accuracy Guarantees

{{JUAN:  Please add numbers here.}}

**Target Accuracy Levels:**
- **Shot Detection:** >85% precision and recall for identifying shot attempts
- **Shot Outcome:** >85% accuracy in classifying makes vs. misses
- **Player Attribution:** >85% accuracy in attributing shots to correct players (when roster provided)

**Measurement Methodology:**
- Accuracy measured against human-annotated ground truth samples

{{SEAN: do we sign up for ongoing analysis?  if it is not important, I say we DON'T???}}
- Monthly accuracy reports provided to PlayHQ showing performance metrics
- Continuous monitoring using PlayHQ scoring webhook data (when available) for validation

**Quality Commitment:**
- Continuous model improvement based on PlayHQ feedback and scoring data
- Quarterly model updates to address systematic errors
- PlayHQ-specific model fine-tuning using their video conditions


{{SEAN:  I say we do NOT provide any of this...}}
<!-- DECISION NEEDED: Should we offer remediation for accuracy violations?
     PROPOSAL: Do not include contractual remediation for accuracy below 85%.
     Rationale: Accuracy is subjective and hard to measure definitively; focus on continuous improvement.
     Alternative: Could offer free reprocessing for games flagged as below-threshold, subject to discussion. -->

### 5.2 Support Model

**Dedicated Support Resources:**
- Named technical account manager for PlayHQ integration
- Direct access to engineering team for complex issues
- Regular check-in calls (weekly during implementation, monthly post-launch)
- Shared Slack channel for real-time communication

**Proactive Monitoring:**
- 24/7 automated monitoring of API health, processing queue, error rates
- Automated alerts for anomalies or degraded performance
- Monthly performance reports delivered to PlayHQ
- Quarterly business reviews to discuss metrics, roadmap, and optimizations

**Knowledge Base and Documentation:**
- Comprehensive API documentation (see Appendix B and C)
- Integration guides and code examples
- Video tutorials for common workflows
- Searchable knowledge base for troubleshooting

**Continuous Improvement:**
- Feedback loop from PlayHQ incorporated into product roadmap
- Regular model updates based on PlayHQ data and feedback
- Feature requests prioritized based on PlayHQ's strategic needs
- Transparency into development roadmap and release schedule

---

## 6. Implementation Plan

### 6.1 Implementation Timeline
[To be completed with detailed timeline]

***{{JASON/SEAN: what do we set here as our timeline?  normally one would do beta testing with live paid customers.  They may not even have customers to beta test with before April.  Can we declare 1 month (or 3 months of paid beta testing) in the schedule.  What I like about this is it give us a "way out" if our stats are not fully upto spec by then, but maybe they will not like seeing "beta" on the schedule in April.  Not sure, but I would love to put it if we can!}}***

| Milestone                           | Target Date   | Deliverables                                                          |
| ----------------------------------- | ------------- | --------------------------------------------------------------------- |
| API Agreement                       | December 2025 | Cross-company agreement on detailed spec for all APIs and interfaces. |
| Project Kickoff                     | January 2026  | Detailed PRD and Planning docs                                        |
| Phase 1: Core Stats & Job Schedling | February 2026 | End-to-End alpha testing job processing round trip & core stats       |
| Phase 2: Full stats & beta testing  | March 2026    | [Deliverables]                                                        |
| Phase 3: [Name]                     | [Date]        | [Deliverables]                                                        |
| Go-Live                             | April 2026    | [Details]                                                             |

### 6.2 Implementation Methodology
[To be completed with project management approach]

### 6.3 Risk Management
[To be completed with identified risks and mitigation strategies]

### 6.4 Change Management and Training
[To be completed with user adoption strategy]

---

## 7. Roadmap and Future Capabilities

### 7.1 Product Roadmap
[To be completed with 12-24 month roadmap]

### 7.2 Multi-Sport Expansion
[To be completed with plan for additional sports]

- **Year 1 Focus:** Basketball
- **Year 2 Expansion:** [2 additional sports and timeline]
{{JASON/SEAN:  Fill in as you like here.  I think we can justify VB and Baseball here, but we could claim soccer too if we chose.  up to you....}}

### 7.3 Technology Innovation
[To be completed with planned innovations and enhancements]

***{{JASON/SEAN:  we can wax poetic here about how we plan to do real-time stats, continuous model updating, etc.  or just remove it.  your call.}}***

---

## 8. Partnership and Support

### 8.1 Partnership Approach

**SportsVisio views PlayHQ as a strategic technology partner, not just a customer.**

Our partnership philosophy is built on:

**Collaborative Development:**
- PlayHQ's feedback directly influences our product roadmap
- Joint planning sessions to align SportsVisio capabilities with PlayHQ's strategic priorities
- Early access to beta features and experimental capabilities
- Co-development opportunities for PlayHQ-specific requirements

**Transparency and Communication:**
- Open communication about system performance, challenges, and improvements
- Regular sharing of accuracy metrics, processing statistics, and infrastructure health
- Advance notice of planned changes, upgrades, or maintenance
- Honest assessment of capabilities and limitations

**Long-Term Commitment:**
- Investment in PlayHQ-specific model training and optimization
- Dedicated engineering resources for integration and ongoing enhancements
- Shared success model: SportsVisio succeeds when PlayHQ succeeds
- Multi-year partnership vision extending beyond initial basketball deployment

**White-Label Partnership:**
- SportsVisio operates as invisible backend infrastructure
- No competing user-facing products or services in PlayHQ markets
- Complete alignment with PlayHQ's brand and user experience goals
- Flexibility to adapt to PlayHQ's evolving needs and business model

### 8.2 Support Structure

**Dedicated Support Team:**

SportsVisio will provide PlayHQ with dedicated support resources:

- **Technical Account Manager:** Single point of contact for all PlayHQ matters
  - Regular check-ins (weekly during implementation, bi-weekly post-launch)
  - Proactive monitoring of PlayHQ's usage patterns and performance
  - Escalation management for critical issues

- **Integration Engineering Support:** Direct access to engineers who built the API
  - Technical troubleshooting and optimization guidance
  - Custom integration assistance
  - Performance tuning recommendations

- **Data Science Team Access:** For accuracy and model performance discussions
  - Analysis of edge cases and challenging videos
  - Feedback on model improvements
  - Guidance on optimal use of AI outputs

**Multi-Channel Support Access:**

As detailed in Section 5.1.3, PlayHQ has access to:
- Email support (support@sportsvisio.com)
- Voice/telephone support
- WhatsApp business line
- Dedicated Slack channel for real-time communication

**24/7 Coverage:**
- Critical issues: 90-minute response time, 24/7
- On-call engineering rotation for after-hours emergencies
- Weekend coverage during peak usage periods

### 8.3 Account Management

**Dedicated Resources for PlayHQ:**

**Primary Contacts:**
- **Technical Account Manager:** [Name TBD] - Day-to-day technical liaison
- **Executive Sponsor:** [Name TBD] - Executive-level relationship and escalation
- **Integration Lead:** [Name TBD] - API integration and technical implementation

**Organizational Commitment:**
- PlayHQ designated as strategic account within SportsVisio
- Priority access to engineering resources
- Influence over product roadmap and feature prioritization
- Dedicated Slack workspace for cross-team collaboration

**Escalation Path:**
1. Technical Account Manager (primary contact)
2. VP of Engineering (technical escalations)
3. Executive Sponsor / CEO (business or critical escalations)

### 8.4 Communication and Collaboration

**Regular Touchpoints:**

**Weekly (During Implementation Phase):**
- Integration standup meetings (30 minutes)
- Technical issue review and resolution planning
- Timeline and milestone tracking

**Monthly (Post-Launch):**
- Performance review meetings
- Accuracy metrics and quality dashboard review
- Roadmap updates and feature discussions
- Support ticket trends and resolution analysis

**Quarterly:**
- Business review meetings with executive sponsors
- Strategic planning sessions
- Roadmap alignment and prioritization
- Performance against SLAs and accuracy targets

**Ad-Hoc:**
- Immediate communication for critical incidents
- Slack channel for real-time technical questions
- Email for non-urgent matters

**Reporting and Transparency:**

SportsVisio will provide PlayHQ with:

**Monthly Performance Reports:**
- API uptime statistics
- Processing volume and queue metrics
- Average processing times
- Accuracy metrics (shot detection, player attribution)
- Error rates and common failure modes
- Support ticket summary

**Quarterly Business Reviews:**
- Performance against all SLA commitments
- Accuracy trends and improvement initiatives
- Infrastructure capacity and scaling readiness
- Product roadmap updates
- Feature requests status
- Strategic discussion topics

**Real-Time Dashboards:**
- API health and status monitoring
- Current processing queue depth
- Recent job completion rates
- Error rate trends

---

## 9. Compliance and Security

### 9.1 Security Compliance

**Security Architecture:**

SportsVisio implements industry-standard security practices across all infrastructure and operations:

**Authentication and Authorization:**
- Bearer token authentication for all API access
- HTTPS/TLS 1.3 encryption for all data in transit
- HMAC-SHA256 webhook signature validation
- API key rotation support with configurable expiration
- Rate limiting to prevent abuse

**Data Encryption:**
- **In Transit:** TLS 1.3 for all API communications
- **At Rest:** AES-256 encryption for all stored data (video cache, results, databases)
- Encrypted backups with secure key management
- Secure video URL handling (support for time-limited signed URLs)

**Infrastructure Security:**
- Cloud-native architecture on Google Cloud Platform (GCP)
- Network isolation and virtual private clouds (VPCs)
- Firewall rules and security groups restricting access
- Regular security patching and updates
- Intrusion detection and prevention systems
- DDoS protection via cloud provider services

**Application Security:**
- Secure coding practices and code review processes
- Regular dependency updates and vulnerability scanning
- Input validation and sanitization on all API endpoints
- SQL injection and XSS prevention
- Comprehensive error handling without information leakage

**Access Control:**
- Role-based access control (RBAC) for internal systems
- Principle of least privilege for all service accounts
- Multi-factor authentication (MFA) for employee access
- Audit logging of all administrative actions
- Regular access reviews and credential rotation

**Monitoring and Incident Response:**
- 24/7 security monitoring and alerting
- Centralized logging with tamper-proof audit trails
- Incident response plan with defined escalation procedures
- Security incident notification to affected customers within 72 hours
- Regular security assessments and penetration testing

**Security Certifications:**

{{SEAN/JASON: Fill in if applicable, otherwise remove or mark as "in progress"}}
- **SOC 2 Type II:** [Certification status - in progress / completed / planned]
- **ISO 27001:** [Certification status - in progress / completed / planned]
- **PCI DSS:** Not applicable (no payment card processing)

### 9.2 Data Privacy

**Privacy Compliance Commitments:**

SportsVisio is committed to protecting the privacy of PlayHQ participants and complying with all applicable data protection regulations.

**GDPR Compliance (General Data Protection Regulation):**

For European participants in PlayHQ competitions:

- **Lawful Basis:** Processing based on legitimate interests or consent (as determined by PlayHQ as data controller)
- **Data Processing Agreement:** SportsVisio will execute a DPA with PlayHQ defining roles and responsibilities
- **Data Subject Rights:** Support for PlayHQ to fulfill data subject requests (access, rectification, erasure, portability)
- **Right to Erasure:** API endpoint for data deletion upon request
- **Data Portability:** Export capabilities in standard JSON format
- **Breach Notification:** 72-hour notification to PlayHQ of any personal data breaches
- **Privacy by Design:** Minimal data collection, purpose limitation, storage limitation
- **International Transfers:** Standard Contractual Clauses (SCCs) for any data transfers outside Australia

**Australian Privacy Act 1988 Compliance:**

SportsVisio adheres to the 13 Australian Privacy Principles (APPs):

- **Transparent Handling:** Clear documentation of data collection and use
- **Anonymity/Pseudonymity:** Player identification via jersey numbers only (no biometric identifiers)
- **Collection Limitation:** Only collect data necessary for video analysis services
- **Data Quality:** Accuracy measures to ensure statistics match game events
- **Data Security:** Encryption, access controls, and security measures (see Section 9.1)
- **Access and Correction:** Support for PlayHQ to fulfill participant access requests
- **Use and Disclosure:** Data used only for AI analysis services, not disclosed to third parties
- **Cross-Border Disclosure:** Compliant handling if data processed outside Australia
- **Government Identifiers:** Not applicable (no government IDs collected)
- **Sensitive Information:** Not applicable (no biometric, health, or sensitive data)

**Privacy Impact Assessment:**

SportsVisio has conducted a Privacy Impact Assessment (PIA) for basketball video analytics services. Key findings:

- **Minimal PII Collection:** Only jersey numbers, team affiliations, game metadata
- **No Facial Recognition:** Explicitly no biometric facial data collected or processed
- **Video Retention:** Temporary video caching only; source videos not permanently stored
- **Results Retention:** Configurable retention periods (default 90 days) with automatic deletion
- **Child Privacy:** Age-appropriate handling of minor participants' data

**No Facial Recognition Commitment:**

**SportsVisio explicitly does NOT use facial recognition technology or collect biometric identifiers.**

Player identification uses only:
- Jersey color detection
- Jersey number OCR
- Court position tracking
- Whole-body tracking (per-game only, models not retained across games)

**Data Minimization:**

SportsVisio collects and processes only data necessary for video analysis:
- Video URLs (not stored permanently)
- Game metadata (teams, rosters, game IDs)
- Jersey numbers for player attribution
- Bounding box coordinates and timestamps
- Shot events and statistics

No collection of:
- Facial biometrics
- Personal identifying information beyond jersey numbers
- Contact information
- Demographics (age, gender, ethnicity)
- Location data (beyond game venue from PlayHQ metadata)

### 9.3 Australian Data Residency

**Data Residency Commitment:**

SportsVisio commits to Australian data residency for PlayHQ participant data, subject to implementation approach agreed with PlayHQ.

**Option 1: Full Australian Data Residency (Recommended)**
- All video processing infrastructure deployed in GCP Australia regions (Sydney, Melbourne)
- All data storage (databases, object storage) hosted in Australia
- Data never leaves Australian borders during processing or storage
- Complies with Australian government data sovereignty requirements
- Higher infrastructure costs (reflected in pricing)

**Option 2: Hybrid Approach with Australian Metadata Storage**
- Video processing may occur in global infrastructure for GPU availability and cost efficiency
- All participant PII and game metadata stored exclusively in Australian regions
- Transient video cache automatically deleted after processing
- Results data stored in Australian regions
- Lower infrastructure costs with maintained data residency for sensitive information

**Implementation Details:**
- Google Cloud Platform Australia regions: `australia-southeast1` (Sydney), `australia-southeast2` (Melbourne)
- Database replication within Australian regions for redundancy
- Data export controls preventing transfer outside Australia
- Compliance documentation available for audit

**Data Transfer Restrictions:**
- No permanent data storage outside Australia (Option 1)
- Transient processing data only (Option 2) with strict deletion policies
- Standard Contractual Clauses (SCCs) for any temporary processing outside Australia
- Data Processing Agreement (DPA) specifying data residency terms

PlayHQ can select preferred approach based on compliance requirements, budget considerations, and risk assessment.

### 9.4 Insurance and Liability

**Insurance Coverage:**

SportsVisio maintains comprehensive insurance coverage appropriate for technology services:

**Professional Indemnity Insurance:**
- Coverage: $[AMOUNT TBD] per claim, $[AMOUNT TBD] aggregate
- Covers professional errors, omissions, and negligent acts
- Includes intellectual property infringement defense
- Provider: [Insurance provider TBD]

**Cyber Liability Insurance:**
- Coverage: $[AMOUNT TBD] per incident
- Covers data breaches, privacy violations, network security failures
- Includes breach response costs, notification expenses, regulatory fines
- Provider: [Insurance provider TBD]

**Public Liability Insurance:**
- Coverage: $[AMOUNT TBD] per occurrence
- General liability for business operations
- Provider: [Insurance provider TBD]

**Certificate of Currency:**
- Certificates of insurance available upon request
- Coverage maintained throughout contract term
- PlayHQ named as additional insured (if required)

**Limitation of Liability:**

{{SEAN/JASON: This is typically heavily negotiated. Standard tech company positions below:}}

Subject to detailed terms in Master Services Agreement:

**Liability Caps:**
- SportsVisio's total liability limited to fees paid by PlayHQ in preceding 12 months
- Exception: No cap for gross negligence, willful misconduct, or breach of confidentiality
- Exception: No cap for third-party IP infringement claims

**Excluded Damages:**
- No liability for indirect, consequential, special, or punitive damages
- No liability for lost profits, lost revenue, or lost business opportunities
- No liability for PlayHQ's failure to backup data or system failures outside SportsVisio's control

**Indemnification:**
- SportsVisio indemnifies PlayHQ against third-party IP infringement claims arising from use of SportsVisio API
- PlayHQ indemnifies SportsVisio against claims arising from PlayHQ's use of data or video content
- Mutual indemnification for breaches of respective obligations

**Force Majeure:**
- No liability for failures due to events beyond reasonable control (natural disasters, cloud provider outages, government actions, pandemics)
- Best efforts to mitigate and notify PlayHQ of force majeure events

These terms subject to negotiation and final Master Services Agreement.

---

## 10. Appendices

### Appendix A: Technical Architecture Diagrams
[To be provided - high-level architecture diagram showing:
- Job submission flow
- API integration points
- Processing pipeline
- Data storage and CDN delivery
- Security layers]

### Appendix B: SportsVisio Job Control API Documentation

**Complete API specification included as separate document:**
- File: `SportsVisio Job Control API Docs.md`

**Summary:**
- RESTful API at `https://ai.sportsvisio-api.com`
- Bearer token authentication
- Job submission via `POST /v1/jobs`
- Status tracking via `GET /v1/jobs/{job_id}`
- Webhook notifications with HMAC-SHA256 signatures
- Comprehensive error handling and retry policies
- Rate limiting with transparent headers
- API versioning (current: v1)

See complete documentation for detailed endpoint specifications, request/response formats, error codes, and integration examples.

### Appendix C: SportsVisio Highlights Job Specification

**Complete job specification included as separate document:**
- File: `SportsVisio Highlights Job Spec.md`

**Summary:**
- Job type: `"highlights"` for basketball video processing
- Input: Video URL (MP4 or HLS) + optional roster data
- Output: JSON results with:
  - Detected teams and players (with jersey colors/numbers)
  - Shot events with timestamps, player attribution, make/miss outcomes
  - Frame-by-frame tracking data (shooter and ball bounding boxes)
  - Team and player statistics (FGA, FGM, FG%)
- Supports 30 fps video, up to 2 hours duration
- Auto-generates player/team IDs if roster not provided
- Semantic versioning for schema evolution (current: v0.1.0)

See complete specification for detailed data structures, coordinate systems, and integration guidelines.

### Appendix D: Security and Compliance Certifications

***{{SAM/SEAN:  I am not sure if we have anything to put in here.  Clearly, we will need to comply with GDPR and Australian law, but I am not sure we put anything in here now???}}***


[To be provided:
- Australian Privacy Principles compliance documentation
- GDPR compliance attestation]

### Appendix E: Case Study Details

**Case Study 1: Lifetime Fitness Basketball Leagues**
[To be expanded with:
- Project scope and duration
- Volume of games processed
- Accuracy metrics achieved
- User engagement results
- Follow-on contract details
- Client testimonial]

**Case Study 2: PlayOn Sports Integration**
[To be expanded with:
- Integration architecture details
- Scale and performance metrics
- User adoption rates
- Technical challenges and solutions
- Outcomes and ROI]

### Appendix F: References and Contact Information
[To be provided:
- Lifetime Fitness: Contact name, title, email, phone
- PlayOn Sports: Contact name, title, email, phone
- Additional references (minimum 3 total)
- Authorization for PlayHQ to contact references]

### Appendix G: Corporate Governance Documents
[To be attached:
- Certificate of Incorporation
- ABN/ACN registration
- Directors and officers
- Organizational structure
- Financial stability documentation
- Insurance certificates (Professional Indemnity, Public Liability, Cyber Insurance)]

### Appendix H: Terms and Conditions
[To be provided:
- Master Services Agreement template
- Service Level Agreement (SLA) details
- Data Processing Agreement
- Intellectual Property provisions
- Liability and indemnification clauses
- Termination and transition provisions]

---

## Declaration

SportsVisio confirms that all information provided in this response is accurate and complete to the best of our knowledge as of the submission date.

**Authorized Signatory:**

Name: [To be completed]
Title: [To be completed]
Date: October 10, 2025
Signature: _______________________
