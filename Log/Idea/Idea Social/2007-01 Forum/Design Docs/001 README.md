# Political Discussion Synthesis Platform - Documentation

## Overview

A collaborative platform for diverse political viewpoints to synthesize position statements through structured debate. Built on NodeBB with custom event sourcing and real-time features for political discourse.

## Documentation Index

### Core Design Documents

- **[002 User Stories](./002%20User%20Stories.md)** - Six user types and their interactions with the platform
- **[003 UX Design](./003%20UX%20Design.md)** - Interface designs, wireframes, and user experience elements
- **[004 UX Actions](./004%20UX%20Actions.md)** - Complete catalog of user actions with JavaScript identifiers
- **[005 Forum Mechanics](./005%20Forum%20Mechanics.md)** - Core forum interaction patterns and discussion flow
- **[006 Data Ontology](./006%20Data%20Ontology.md)** - Unified NOTE data model and NodeBB mapping strategy
- **[007 APIs and Key Methods](./007%20APIs%20and%20Key%20Methods.md)** - API endpoints and method signatures
- **[008 NodeBB Design](./008%20NodeBB%20Design.md)** - Complete system architecture with event sourcing implementation

### Implementation Documentation

- **[021 Detail Design](./021%20Detail%20Design.md)** - Component specifications, versions, and infrastructure details

### Supporting Documentation

- **[010 LATER](./010%20LATER.md)** - Future features and missing functionality for later phases

## Key Features

- **Event Sourcing Architecture** - All actions logged as immutable events with derived state reconstruction
- **Real-time Collaboration** - Socket.io integration for live updates and notifications  
- **Political Neutrality** - Tribal affiliation tracking with balanced representation
- **Structured Debate** - Support/oppose stance system with evidence classification
- **NodeBB Foundation** - Leverages proven forum infrastructure with custom extensions

## Quick Start

1. **User Stories** - Start here to understand who uses the platform and why
2. **UX Design** - Review the interface designs and user flows
3. **NodeBB Design** - Comprehensive technical architecture and implementation details
4. **APIs and Key Methods** - Technical API specifications for developers

## Implementation Status

- ✅ **Design Phase** - Complete system design and user experience
- 🔄 **Development Phase** - Ready for NodeBB plugin implementation
- ⏳ **Later Features** - Advanced analytics, moderation tools, and personalization

## Technology Stack

- **Backend:** Node.js, NodeBB forum platform
- **Database:** MongoDB/PostgreSQL with event sourcing
- **Real-time:** Socket.io for live updates
- **Frontend:** Custom templates and JavaScript components
- **Infrastructure:** Docker, t3.small AWS instance (~$20/month)

## Contributing

This documentation represents the complete design specification for a political discussion synthesis platform. All technical decisions and user experience choices are documented across the linked files above.