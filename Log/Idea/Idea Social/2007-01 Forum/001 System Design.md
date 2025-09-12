# Political Discussion Synthesis Framework - System Design

## Overview

A collaborative platform for diverse political viewpoints to synthesize position statements through structured debate. Participants view, comment on, and refine arguments with a focus on presenting the "best" versions of each position and its rebuttals.

## Core Requirements

### MVP Features
- User authentication with political affiliation (liberal/conservative)
- Threaded discussion posting with metadata tagging
- Simple voting system (+1 support, -1 oppose, == alternative)
- Argument tree visualization with margin comments
- "Best content" selection algorithm
- 20-100 participant capacity

### User Interactions
- Post position statements on topics
- Comment/rebut specific text segments
- Vote on arguments and rebuttals
- Propose alternative formulations
- View structured argument trees

## Technology Stack

**Backend:** Python/Flask (rapid development, simple deployment)
**Database:** SQLite → PostgreSQL (start simple, scale later)  
**Frontend:** HTML/CSS/JavaScript with Bootstrap (minimal complexity)
**Analysis:** Python scripts (reuse backend language)
**Deployment:** Single server initially (Docker for consistency)

## System Architecture

```
┌─────────────────┐    ┌──────────────────┐    ┌─────────────────────┐
│  Discussion     │    │   Analysis       │    │  Argument Viewer    │
│  Forum App      │────│   Engine         │────│  Frontend           │
│  (Flask)        │    │   (Python)       │    │  (HTML/JS)          │
└─────────────────┘    └──────────────────┘    └─────────────────────┘
         │                        │                         │
         └────────────────────────┼─────────────────────────┘
                                  │
                    ┌─────────────────────────┐
                    │     SQLite Database     │
                    │   (Forum + Metadata)    │
                    └─────────────────────────┘
```

## Component Details

### 1. Discussion Forum Module
**Purpose:** Core posting and interaction system
**Features:**
- User registration with political affiliation
- Topic creation and management  
- Threaded comment system
- Metadata tagging (post_type: support/oppose/alternative)
- Simple web interface for posting

**Implementation:** Flask app with basic templates

### 2. Database Layer
```sql
-- Core tables
users (id, username, email, political_affiliation, created_at)
topics (id, title, description, created_by, created_at)
posts (id, topic_id, user_id, parent_id, content, post_type, target_post_id, votes_up, votes_down, created_at)
votes (id, post_id, user_id, vote_type, created_at)
```

**Post Types:** 
- `position` - Original position statement
- `support` - Supporting argument
- `oppose` - Counter-argument  
- `alternative` - Alternative formulation

### 3. Analysis Engine
**Purpose:** Process forum data into structured arguments
**Functions:**
- Scan database for new content
- Build argument trees from threaded discussions
- Calculate "best" content using voting + recency
- Generate JSON data files for viewer
- Handle content ranking algorithms

**Algorithm for "Best" Content:**
```
score = (upvotes - downvotes) * recency_factor * author_credibility
```

### 4. Argument Viewer Frontend
**Purpose:** Display structured debates with interactive navigation
**Features:**
- Main text with margin comments (Google Docs style)
- Expandable argument trees
- Hover interactions for sub-arguments
- Clean, readable typography
- Mobile-responsive design

**Layout:** Text panel + collapsible sidebar with argument tree

### 5. Administration Tools
**Purpose:** Content and user management
**Features:**
- Topic creation and moderation
- User management and role assignment
- Content flagging and removal
- System analytics and usage stats

## Development Phases

### Phase 1: Core Forum (2-3 weeks)
- [ ] User registration/login with political affiliation
- [ ] Basic topic and post creation
- [ ] Threaded comments with metadata tags
- [ ] Simple voting system (+1, -1, ==)
- [ ] Basic web interface

### Phase 2: Analysis Engine (1-2 weeks)  
- [ ] Database scanning script
- [ ] Argument tree construction
- [ ] "Best content" ranking algorithm
- [ ] JSON export for viewer

### Phase 3: Argument Viewer (2-3 weeks)
- [ ] Interactive argument display
- [ ] Margin comment system  
- [ ] Tree navigation with hover/click
- [ ] Responsive design
- [ ] Integration with analysis data

### Phase 4: Polish & Testing (1-2 weeks)
- [ ] User testing with small group
- [ ] Performance optimization
- [ ] UI/UX improvements
- [ ] Bug fixes and stability

## Implementation Strategy

### Week 1-2: Foundation
Start with Flask forum using existing patterns:
- Copy/adapt simple forum tutorial code
- Add political affiliation to user model
- Implement basic post metadata system

### Week 3-4: Analysis Pipeline  
Build Python script to process forum database:
- Start with simple tree-walking algorithm
- Implement basic ranking (upvotes - downvotes)
- Output to JSON files for testing

### Week 5-7: Viewer Interface
Create standalone HTML/JS viewer:
- Load JSON data files
- Build interactive tree navigation
- Style for readability and usability

### Week 8: Integration & Testing
- Connect all components
- Test with sample data
- Prepare for user trials

## Deployment Plan

**Minimal Setup:**
- Single cloud server (DigitalOcean droplet)
- SQLite database (no separate DB server needed)
- Nginx reverse proxy
- Automated analysis script (cron job)

**Scaling Considerations:**
- Move to PostgreSQL when approaching 100 users
- Add Redis caching for argument viewer
- Consider CDN for static argument data

## Risk Mitigation

**Technical Risks:**
- Database performance: Start with SQLite, profile early
- Complex argument trees: Limit nesting depth initially  
- User engagement: Focus on simple, clear interactions

**Content Risks:**
- Moderation: Start with small trusted group
- Bias in ranking: Make algorithm transparent and adjustable
- Echo chambers: Ensure cross-partisan visibility

## Success Metrics

- Active daily users (target: 20-50 for MVP)
- Posts per user per week (target: 3-5)
- Cross-partisan engagement rate
- Quality of synthesized positions (qualitative assessment)

## Next Steps

1. Set up development environment
2. Create basic Flask forum structure  
3. Implement user authentication with political affiliation
4. Build simple posting system with metadata
5. Test with small dataset before building analysis engine

Total estimated development time: **6-8 weeks** for full MVP with one developer working part-time.