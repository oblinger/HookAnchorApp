# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is the websearch component of the binproj (binary project) collection. The project focuses on collecting and cataloging sports-related datasets and data sources.

## Dataset Collection

### sport_datasets.csv
- **Purpose**: Catalog of sports-related datasets available online
- **Columns**: Dataset Name, Sport, Data Type, Source, Cost, Description, URL
- **Count**: 20 entries
- **Coverage**: Multi-sport datasets including NFL, NBA, MLB, Premier League, Tennis, Olympics, F1, Cricket, Golf, Hockey, MMA
- **Sources**: Mix of free (official APIs, open data) and paid (commercial providers) options
- **Last Updated**: 2025-06-06

### bb_datasets.csv
- **Purpose**: Basketball-specific datasets with detailed moment-by-moment game annotations
- **Columns**: Dataset Name, Source, Data Type, Temporal Resolution, Annotations, Video Available, Cost, Coverage, Value, Description, URL
- **Count**: 33 entries (expanded from 21)
- **Coverage**: NBA/WNBA tracking, video analysis platforms, academic research, international/college basketball
- **New Additions**: Academic datasets (Basketball-51, MultiSubjects, TrackID3x3), WNBA tracking, EuroLeague data, AI platforms
- **Focus**: Frame-by-frame analysis, player tracking, pose estimation, tactical breakdowns, 3D spatial data
- **Temporal Range**: Real-time to historical (1995-present), variable FPS to 25 FPS
- **Video Integration**: 21 video-enabled sources, 12 tracking/statistics only
- **Research Integration**: Academic datasets with pose estimation and multi-object tracking
- **Last Updated**: 2025-06-06

### research_methods.md
- **Purpose**: Comprehensive guide for discovering sports datasets and data sources
- **Structure**: Organized by search method categories (search engines, repositories, official sources, etc.)
- **Coverage**: Search techniques, platforms, academic sources, APIs, scraping, quality assessment
- **Sections**: 10 major categories with 40+ specific methods and strategies
- **Focus**: Systematic approaches to dataset discovery and validation
- **Last Updated**: 2025-06-06

## Development Commands

No specific commands are configured yet. Standard development practices apply.