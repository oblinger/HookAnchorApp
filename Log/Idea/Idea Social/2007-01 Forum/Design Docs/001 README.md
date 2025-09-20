# Political Discussion Forum - Design Documentation

## Overview

A NodeBB-based political discussion platform that enables constructive discourse through tribal affiliation tracking, argument synthesis, and consensus-finding mechanisms. The system uses a two-tier architecture with a public NodeBB plugin and proprietary metadata backend.

## Documentation Index

### Requirements & User Experience (002-005)

- **[002 User Stories](./002%20User%20Stories.md)** - Six user personas and their platform interactions
- **[003 UX Design](./003%20UX%20Design.md)** - Interface mockups, wireframes, and user flows
- **[004 UX Actions](./004%20UX%20Actions.md)** - Complete catalog of user actions and system responses
- **[005 Forum Mechanics](./005%20Forum%20Mechanics.md)** - Core discussion patterns and interaction models

### Data Architecture (010-013)

- **[010 Data Ontology](./010%20Data%20Ontology.md)** - Universal NOTE data model with NID addressing system
- **[011 NodeBB DB Tables DataTypes and APIs](./011%20NodeBB%20DB%20Tables%20DataTypes%20and%20APIs.md)** - NodeBB data structures and API mappings
- **[012 Proprietary Data Structures and Event Store](./012%20Proprietary%20Data%20Structures%20and%20Event%20Store.md)** - In-memory cache and SQL event store design
- **[012 Note Types](./012%20Note%20Types.md)** - Complete enumeration of note types in the unified tree
- **[013 APIs and Key Methods](./013%20APIs%20and%20Key%20Methods.md)** - REST API specifications and method signatures

### Integration Architecture (019-021)

- **[019 NodeBB Integration](./019%20NodeBB%20Integration.md)** - Six core commitments for NodeBB integration including categories as issues, unread filtering, and hybrid architecture
- **[020 NodeBB Design](./020%20NodeBB%20Design.md)** - NodeBB-specific implementation details
- **[021 Detail Design](./021%20Detail%20Design.md)** - Component specifications, versions, and infrastructure

### Future Work (030)

- **[030 LATER](./030%20LATER.md)** - Deferred features and future enhancements

## Architecture Highlights

### Two-Tier System Design
1. **Public NodeBB Plugin** - Generic Enhanced Note Viewer (open source)
2. **Proprietary Backend** - Voting synthesis and political analysis server

### Core Innovations
- **Unified Tree Structure** - All content (categories, topics, posts) in single tree with NID addressing
- **Event Sourcing** - Complete audit trail with derived state reconstruction
- **Tribal Affiliations** - Two-tier system: admin-assigned political affiliation + dynamic issue-based tribes
- **Variant Competition** - Democratic evolution of categories, topics, and arguments
- **Synthesis Nodes** - AI-generated consensus summaries

### Key Technical Decisions
- **NID System** - Prefixed IDs (r0, c1, t123, p789) prevent NodeBB ID collisions
- **Hybrid Storage** - NodeBB for presentation, event log for truth, memory cache for performance
- **JWT Authentication** - Leverages NodeBB auth for metadata server access
- **External Versioning** - Complete edit history in metadata server (NodeBB lacks this)

## Implementation Status

### ✅ Completed
- Database snapshot/restore system (`just snapshot`, `just restore`)
- Docker environment with NodeBB, MongoDB, Redis
- Core data model and ontology design
- Integration architecture with 6 commitments
- Event store and cache design

### 🔄 In Progress
- NodeBB plugin structure
- Metadata server implementation

### ⏳ Planned
- Cloud deployment (AWS/other)
- Political affiliation admin tools
- Argument synthesis algorithms
- Enhanced note viewer UI

## Technology Stack

- **Forum Platform:** NodeBB v4.5.1
- **Databases:** MongoDB (NodeBB), PostgreSQL (Event Store)
- **Cache:** Redis + In-Memory JavaScript objects
- **Real-time:** Socket.io
- **Container:** Docker with ARM64 support
- **Development:** `just` command runner

## Quick Start for Developers

1. Review **[002 User Stories](./002%20User%20Stories.md)** for context
2. Study **[010 Data Ontology](./010%20Data%20Ontology.md)** for the data model
3. Read **[019 NodeBB Integration](./019%20NodeBB%20Integration.md)** for architecture
4. Check **[012 Proprietary Data Structures](./012%20Proprietary%20Data%20Structures%20and%20Event%20Store.md)** for implementation details

## Contact & Repository

*Note: This documentation represents the complete design specification for the political discussion forum. The two-tier architecture enables both open-source contribution and proprietary IP protection.*