# SportsVisio Response to PlayHQ AI Video Analytics and Highlights RFP

**RFP Reference:** PHQ-AI-VIDEO-2025
**Submission Date:** October 10, 2025
**Submitted By:** SportsVisio

---

## Executive Summary

[To be completed with overview of SportsVisio's proposed solution, key differentiators, and value proposition for PlayHQ partnership]

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
- **Content Delivery System:** Multi-format video output generation (horizontal & vertical highlights)
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

**AI-Only Event Detection:**

SportsVisio's system operates purely from video analysis without reliance on external scoring data:

- All shot detection, player tracking, and statistics are derived from computer vision AI
- No dependency on manual scoring, shot clocks, or scoreboard data
- Fully automated processing requires only video input + optional roster
- Suitable for all game levels (including games without official scorers or scoreboards)

**Optional: Future Scoring Data Integration**

If PlayHQ wishes to provide official scoring data (via webhooks or API), SportsVisio can use this to:
- Validate AI predictions against ground truth for accuracy measurement
- Provide quality assurance reporting on AI performance
- Continuously improve models through feedback loops
- Flag discrepancies between AI detection and official scores

This integration is optional and not required for core functionality.

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
- **Auto-Scaling:** Currently scales 0-1000 GPUs; will be extended to support PlayHQ's 600,000+ game volume
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

**Processing Time:** Guaranteed <6 hours from job submission to completion
- **Target:** 3-5 hours average
- **SLA:** 95% of jobs complete within 6 hours
- **Remediation:** Service credits for SLA violations per commercial agreement

**Event Recognition Accuracy:** >85% for shot detection
- **Current Performance:** 88-92% shot detection accuracy on test datasets
- **Measurement:** Precision and recall against human-annotated ground truth
- **Continuous Improvement:** Monthly model updates to maintain/improve accuracy

**Statistics Accuracy:** >85% for player attribution and shot outcomes
- **Current Performance:** 90% player attribution accuracy when roster provided
- **Validation:** Cross-validation with PlayHQ scoring webhooks when available
- **Quality Guarantee:** Accuracy guarantees with remediation clauses per SLA

**Data Output Quality:** Accurate event metadata matching source video
- **Input Support:** 480p minimum, 1080p recommended source video
- **Timestamp Precision:** Event times accurate to 0.1 second resolution
- **Coordinate System:** Normalized bounding boxes (0-1 scale) relative to source video dimensions
- **Metadata Format:** Structured JSON conforming to published schema (see Appendix C)


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

---

## 4. Commercial Proposal

### 4.1 Pricing Structure

#### 4.1.1 Processing Fees
[To be completed with detailed pricing]

- **Per-Game Processing Fee:** [Amount]
- **Volume Discount Structure:**
  - [Tier 1: Volume and rate]
  - [Tier 2: Volume and rate]
  - [Tier 3: Volume and rate]

#### 4.1.2 Revenue Share Model (Optional)
[To be completed with revenue share proposal if applicable]

#### 4.1.3 Setup and Implementation Costs
[To be completed with full breakdown]

- **Initial Setup Fee:** [Amount and details]
- **Integration Development:** [Costs]
- **Testing and QA:** [Costs]
- **Training and Onboarding:** [Costs]
- **Total Implementation Cost:** [Amount]

#### 4.1.4 Ongoing Costs
[To be completed with recurring costs]

- **Monthly Platform Fee:** [If applicable]
- **Support and Maintenance:** [Details]
- **Infrastructure Costs:** [If applicable]

### 4.2 Commercial Model Alignment
[To be completed with explanation of how commercial model aligns with PlayHQ needs]

---

## 5. Service Level Agreements

### 5.1 Performance Guarantees

#### 5.1.1 Processing Time
- **Target:** <6 hours
- **Guarantee:** [Specific commitment]
- **Remediation:** [Approach if target not met]

#### 5.1.2 System Uptime
- **Target:** 99.90%
- **Measurement:** [Methodology]
- **Remediation:** [Approach for downtime]

#### 5.1.3 Support Response
- **Escalation Process:** [Defined levels and timeframes]
- **Support Channels:** [Available channels]
- **Response Time Commitments:** [By severity level]

#### 5.1.4 Accuracy Guarantees
- **Event Recognition:** [>85% guarantee]
- **Statistics Accuracy:** [>85% guarantee]
- **Remediation Clauses:** [Approach when accuracy falls below threshold]

### 5.2 Support Model
[To be completed with support structure details]

---

## 6. Implementation Plan

### 6.1 Implementation Timeline
[To be completed with detailed timeline]

| Milestone | Target Date | Deliverables |
|-----------|-------------|--------------|
| Project Kickoff | January 2026 | [Details] |
| Phase 1: [Name] | [Date] | [Deliverables] |
| Phase 2: [Name] | [Date] | [Deliverables] |
| Phase 3: [Name] | [Date] | [Deliverables] |
| Go-Live | April 2026 | [Details] |

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

### 7.3 Technology Innovation
[To be completed with planned innovations and enhancements]

---

## 8. Partnership and Support

### 8.1 Partnership Approach
[To be completed with philosophy on partnership with PlayHQ]

### 8.2 Support Structure
[To be completed with ongoing support model]

### 8.3 Account Management
[To be completed with dedicated resources for PlayHQ]

### 8.4 Communication and Collaboration
[To be completed with approach to ongoing communication]

---

## 9. Compliance and Security

### 9.1 Security Compliance
[To be completed with security certifications and compliance]

### 9.2 Data Privacy
[To be completed with privacy compliance details]

### 9.3 Australian Data Residency
[To be completed with data residency commitment]

### 9.4 Insurance and Liability
[To be completed with insurance coverage details]

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

### Appendix D: UI/UX Mockups
[To be provided - mockups showing:
- Video upload interface within PlayHQ/MyHoops app
- Job status and progress indicators
- Player highlight galleries
- Statistical dashboards (player, coach, administrator views)
- Social sharing flows
- Badge/gamification displays]

### Appendix E: Security and Compliance Certifications
[To be provided:
- SOC 2 Type II certification (if applicable)
- ISO 27001 certification (if applicable)
- Privacy impact assessment
- Data processing agreement (DPA) template
- Australian Privacy Principles compliance documentation
- GDPR compliance attestation]

### Appendix F: Case Study Details

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

### Appendix G: References and Contact Information
[To be provided:
- Lifetime Fitness: Contact name, title, email, phone
- PlayOn Sports: Contact name, title, email, phone
- Additional references (minimum 3 total)
- Authorization for PlayHQ to contact references]

### Appendix H: Corporate Governance Documents
[To be attached:
- Certificate of Incorporation
- ABN/ACN registration
- Directors and officers
- Organizational structure
- Financial stability documentation
- Insurance certificates (Professional Indemnity, Public Liability, Cyber Insurance)]

### Appendix I: Terms and Conditions
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
