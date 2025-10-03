# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Environment Setup

**Conda Environment**: The project uses the `SV_analytics` conda environment with Python 3.10 and PyTorch 2.3.1 with CUDA 12.1 support.

```bash
# Create environment
conda env create -f environment.yml
conda activate SV_analytics

# Setup PYTHONPATH and environment variables
cd setup
source add_pkg_path.sh
source export_environ_vars_local.sh
```

## Core Commands

**Main Pipeline**:
```bash
cd src/synch/
python3 mcproc.py -c my_config.json.cfg
```

**Job Dispatcher** (Docker):
```bash
docker compose run --rm sv_ai_pipeline
docker compose run --rm sv_ai_pipeline ./jobs/ping.py
docker compose run --rm sv_ai_pipeline ./jobs/fireball.py
docker compose run --rm sv_ai_pipeline ./jobs/full.py
docker compose run --rm sv_ai_pipeline ./jobs/player_minutes.py
```

**Testing**:
```bash
pytest tests/  # Run all tests
pytest tests/test_job_dispatcher.py  # Run specific test file
pytest tests/test_baseball_alignments.py  # Baseball-specific tests
```

**Code Quality**:
```bash
ruff check .  # Linting
black .       # Formatting (line length: 88)
```

**Configuration Management**:
```bash
# List available configs
find src/synch/configs -name "*.json.cfg"

# Run specific sport pipeline
python mcproc.py -c base_basketball.json.cfg,full_pipeline/stage_3_process.json.cfg

# Debug configuration
python mcproc.py -c my_config.json.cfg --help  # Show all options
```

## Architecture Overview

**algorithms2** is a sports analytics AI pipeline focusing on basketball, baseball, and volleyball video analysis. The system extracts detailed game metrics including player tracking, shot detection, jersey number recognition, and sports-specific analytics.

### Key Components

**Core Processing**:
- `src/synch/mcproc.py` - Main processing pipeline with configuration-driven execution
- `src/bball_analyze/` - Basketball-specific analysis modules (player tracking, jersey detection, shot analysis, court mapping)
- `src/eval/` - Comprehensive evaluation framework with sports-specific metrics
- `jobs/` - Modular job system (ping, fireball, full, player_minutes)
- `src/utils/` - Supporting infrastructure (data management, annotations, API integration)

**Computer Vision Stack**:
- YOLO/RT-DETR for object detection
- SAM (Segment Anything Model) for segmentation  
- ByteTracker with Re-ID for multi-object tracking
- PaddleOCR integration for jersey number recognition
- Custom neural networks for sports-specific tasks
- Real-time video processing with OpenCV

**Infrastructure**:
- Docker containerization with NVIDIA GPU support
- AWS/GCP cloud integration with automatic artifact management
- Configuration-driven pipeline execution via JSON configs
- Git submodules: EfficientVit, PaddleOCR2PyTorch, DVC-DAT

### Configuration System

**Layered JSON Configuration**:
- **Base configs**: `src/synch/configs/base_{sport}.json.cfg` define sport-specific algorithm sets
- **Pipeline stages**: Multi-stage processing configs (preprocess, training, process, analysis)
- **Composable execution**: Configs combined using comma-separated lists
- **Algorithm toggles**: Each algorithm module can be enabled/disabled via boolean flags

**Example Usage**:
```bash
python mcproc.py -c base_basketball.json.cfg,full_pipeline/stage_1_preprocess.json.cfg,full_pipeline/stage_3_process.json.cfg
```

### Data Flow

1. **Input**: Video feeds from sports events
2. **Configuration**: JSON-driven algorithm selection and parameter tuning
3. **Processing**: Multi-stage pipeline with configurable modules
4. **Analysis**: Sport-specific modules extract metrics (player tracking, shot charts, possession analysis)
5. **Evaluation**: Built-in metrics validation and ground truth comparison
6. **Output**: Structured analytics data, visualizations, and performance metrics
7. **Storage**: Automatic cloud artifact management (S3/GCS)

### Job System

The job dispatcher (`job_dispatcher.py`) provides modular execution with standardized wrapper interface:
- **ping**: Basic connectivity test
- **fireball**: Shot detection and analysis
- **full**: Complete game analysis pipeline  
- **player_minutes**: Player time tracking

**Job Architecture**:
- Each job implements a `wrapper(sv_api, instance_data, ai_config, data_path)` function
- Automatic status reporting and error handling
- Cloud instance management with auto-shutdown
- Artifact upload with structured logging

## Docker Workflow

**Prerequisites**:
```bash
git submodule sync
git submodule update --init --recursive --jobs 8
aws s3 sync "s3://annotation-ar-output/dev/assets/" assets/
```

**Build and Run**:
```bash
docker compose build sv_ai_pipeline
docker compose run --rm sv_ai_pipeline  # Default: job_dispatcher.py
docker compose run --rm sv_ai_pipeline bash  # Debug console
```

## Sports-Specific Features

**Basketball**: Player identification, jersey number OCR, shot detection, court localization, ball tracking, possession analysis
**Baseball**: Field alignment, player tracking
**Volleyball**: Court detection, player analysis

The system uses modular design allowing easy extension to additional sports through the configuration system.

## Development Patterns

### Adding New Jobs
1. Create new job file in `jobs/` directory
2. Implement `wrapper(sv_api, instance_data, ai_config, data_path)` function
3. Follow existing patterns from `jobs/ping.py` or `jobs/fireball.py`
4. Add job to Docker compose configuration if needed

### Modifying Algorithm Pipeline
1. Locate relevant config in `src/synch/configs/base_{sport}.json.cfg`
2. Enable/disable algorithms by setting boolean flags (`true`/`false`)
3. Create custom configs by composing existing stage configs
4. Test locally before deploying to cloud

### Working with Sports-Specific Modules
- **Basketball**: Modify algorithms in `src/bball_analyze/` 
- **Baseball/Volleyball**: Create similar sport-specific directories
- **Evaluation**: Add metrics to `src/eval/` following existing patterns
- **Utilities**: Common functionality goes in `src/utils/`

### Configuration Architecture
- **Base configs**: Define which algorithms are available for each sport
- **Stage configs**: Control multi-stage pipeline execution (preprocess → training → process → analysis)
- **Environment configs**: AWS, GCP, local development variants
- **Composition**: Use comma-separated lists to combine configs