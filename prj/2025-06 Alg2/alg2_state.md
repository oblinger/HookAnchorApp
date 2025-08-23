# State Management in MCProc Sports Analytics Pipeline

This document provides a comprehensive analysis of how state is structured and managed in the mcproc system, the core processing engine for the sports analytics pipeline.

## Overview

The mcproc system implements a sophisticated multi-algorithm processing pipeline with centralized state management. State is managed through a hierarchical system of data structures, configuration-driven execution, and complex dependency management between processing algorithms.

## Core State Architecture

### Central State Class (`synch/state.py`)

The `State` class serves as the central repository for all pipeline data and implements the core state management patterns:

```python
class State:
    _version = '3.6.1'
    
    # Core data structures
    self._data = {v: {} for v in views}          # data[view][frame_nbr][key].Data
    self._algtriggers = {}                       # algtriggers[alg][tid].Trigger  
    self._reqspecs = {}                          # reqspecs[tspec][alg][tid][fn].set(dspec)
    self._frames = {v: deque() for v in views}   # frame tracking for memory limits
    self._max_frame_nbr = {v: 0 for v in views}
    self._min_frame_nbr = {v: 0 for v in views}
```

#### State Storage Patterns

**Data Organization**: State is organized in a 4-dimensional hierarchy:
- **View**: Camera perspective (`'view1'`, `'view2'`, `'mview'` for merged)
- **Frame Number**: Temporal sequence identifier (integer)
- **Key**: Data type identifier (defined in `synch/keys.py`)
- **Data**: Wrapped in `Data = namedtuple('Data', 'data alg')` to track ownership

**Example Access Pattern**:
```python
# Store data: state._data[view][frame_nbr][key] = Data(data_value, algorithm_owner)
# Retrieve: state.getitem(view, frame_nbr, key) -> data_value
```

#### Memory Management

**Frame-Based Limits**:
- Default maximum: 1,000,000 frames per view
- Configurable via `cfg['main']['max_frames']`
- Automatic cleanup when limits exceeded
- Uses `collections.deque` for efficient frame tracking

**Cleanup Strategy**:
```python
# Automatic cleanup when max frames exceeded
while len(self._frames[v]) > self.max_frames:
    frame_nbrs.append(self._frames[v].pop())
if frame_nbrs:
    self.cleanup({0: (v, frame_nbrs, '*')})
```

## Configuration-Driven State Management

### Configuration Loading (`synch/cfgloader.py`)

The pipeline uses cascading configuration from multiple sources:

**Configuration Sources**:
1. CSV files with key-value pairs
2. JSON files with nested structures  
3. Command-line parameter overrides
4. Runtime dynamic updates

**Deep Configuration Merging**:
```python
def update_dict_deep(d, u):
    """Updates multi-level dictionary recursively"""
    for k, v in u.items():
        if isinstance(v, collections.abc.Mapping):
            d[k] = update_dict_deep(d.get(k, {}), v)
        else:
            d[k] = v
    return d
```

**Type Conversion**: Automatic casting of string values to appropriate types (bool, int, float, lists).

### Algorithm Instantiation and State Binding

**Dynamic Algorithm Loading** (`mcproc.py:544-556`):
```python
for alg_name in algs_keys_sorted:
    on = cfg.get('algs')[alg_name]
    if not on: continue
    algviews = [v for v in cfg['views'] if alg_name.startswith(cfg.get('alg_views')[v])]
    for view in algviews:
        cfg['view'] = view
        alg = globals()[alg_name](cfg)  # Instantiate algorithm with view-specific config
        cls.algs.append(alg)
```

Each algorithm receives:
- View-specific configuration
- Access to shared state object
- Unique identifier for data ownership
- Trigger and request specifications

## Inter-Algorithm Communication

### Trigger-Based Communication

**Trigger System**:
```python
Trigger = namedtuple('Trigger', 'tspecset rdicts')
# Example: self._algtriggers[alg_id] = {trigger_id: (trigger_spec_set, request_dict)}
```

**Trigger Specifications**:
- **Trigger Spec Set**: Conditions that activate the trigger (e.g., `('view', 'shot-start')`)
- **Request Dictionary**: Data requests when trigger fires (e.g., `{1: ('*', '*', '*')}`)

**Data Flow Example**:
1. Algorithm A adds data with key `'shot-start'`
2. State checks for registered triggers on `'shot-start'`
3. Matching algorithms receive triggered data automatically
4. Downstream processing begins

### Queue-Based Messaging (`synch/algqueues.py`)

**Queue Types**:
```python
QNAMES = ['main2alg_trg',  # triggered data to algs
          'main2alg_req',  # requested data to algs  
          'alg2main_add',  # data items from algs
          'alg2main_req']  # data requests from algs
```

**Message Processing**:
- Non-blocking queue operations
- Batched processing (up to `NCHECKS=5` per iteration)
- Automatic retry and error handling
- Support for both multiprocessing and threading

## State Persistence and Recovery

### Writer System (`synch/writer.py`)

**Serialization Options**:
```python
# Supports multiple formats
if self.pickle:
    pickle.dump(data_to_save, self._fp, pickle.HIGHEST_PROTOCOL)
if self.json:
    jdata = json.dumps(list([list(k), v]), cls=NpEncoder)
```

**Data Filtering**:
```python
# Keys excluded from persistence
_DWKS = set([ky.FRAME, ky.END, ky.SET, ky.HOLD, ky.TRIGGERS, ky.NORESULTS, ky.STATUS])
```

**Progressive Writing**:
- Real-time data streaming to disk
- Frame-by-frame writing with configurable buffering
- Support for both pickled and JSON formats
- Custom NumPy encoder for JSON serialization

### Hold System for Dependency Management (`synch/holds.py`)

**Complex Dependency Graph**:
```python
HOLDS = {
    'holdFI': (None, 0, True),         # file_input
    'holdTR': ('holdFI', 0, False),    # tracker_boxes  
    'holdSH': ('holdFI', 0, True),     # shot_detect
    'holdSR': ('holdSH', 96, True),    # shooter_finder
    'holdMM': ('holdSH', 8, True),     # make_miss
    'holdRR': ('holdMM', 0,            # rebounder_finder
               (ky.SHOT_MADE_MISS_CLASSIFICATION, lambda x, y: operator.is_(x.classification, y), False)),
    # ... complex conditional dependencies
}
```

**Hold Specifications**:
- **Upstream Dependency**: Which hold this depends on
- **Frame Offset**: Additional frames to hold beyond dependency
- **Condition**: Boolean or conditional test for release

**Conditional Release Example**:
```python
# Rebound finder only runs if shot was missed
'holdRR': ('holdMM', 0, (ky.SHOT_MADE_MISS_CLASSIFICATION, 
                        lambda x, y: operator.is_(x.classification, y), False))
```

## Advanced State Features

### Thread Pool Execution Support

**Delayed State Updates**:
```python
# Thread-safe state updates for concurrent execution
if self._updates_queue and can_be_queued:
    self._updates_queue.put((alg, results, override, dbg))
    return
```

**Queue Flushing**:
- Batch processing of queued updates
- Thread-safe state modifications
- Maintains data consistency across concurrent algorithms

### Visualization Integration (`synch/viz.py`)

**Real-Time Visualization**:
```python
# Automatic visualization during cleanup
if self.viz and k == ky.FRAME:
    try:
        self._vz[v].viz(fn, self._data[v][fn][k].data)
    except Exception as e:
        # Visualization errors don't stop processing
        self.log('e', f'exception in viz: {e}')
```

**Multi-View Support**:
- Separate visualization instances per camera view
- Real-time frame rendering with overlays
- Court mapping and top-view projections
- Multi-layer rendering (detections, tracks, poses)

### Data Ownership and Validation

**Ownership Enforcement**:
```python
# Prevents unauthorized data overwrites
if owner != alg and not str(owner).startswith('State') and \
   not k.startswith(ky.HOLD) and self._data[v][fn][k].data != data:
    e = f'non-owner {alg} overwriting {owner, spec2}'
    if self.waiveoverwrite:
        if dbg: self.log('d', e)
    else:
        self.log('e', e)
        raise Exception(e)
```

**Key Validation**:
- All data keys must be defined in `synch/keys.py`
- Runtime validation during data operations
- Prefix matching for wildcard operations

## Multiprocessing Architecture

### Process Coordination

**Subprocess Management** (`mcproc.py:777-813`):
```python
# Algorithm assignment to subprocesses
subprocs, subproc_algs = cls.cfg_subprocs(cfg)
cls.run_algs(cfg, '', subprocs, set(cls.algs) - subproc_algs, cls.aq, cls.dbg)
```

**Queue Distribution**:
- Each algorithm gets dedicated message queues
- Process-safe queue operations
- Coordinated startup and shutdown

**Synchronization Patterns**:
```python
# Main loop coordination
alldone = min([n if n is not None else 0 
               for n in no_results.values()]) >= TH_ALLDONE
```

### Memory Optimization

**Frame-Based Cleanup**:
```python
# Incremental memory management
if len(self._frames[v]) > self.max_frames:
    cleanup_frames = self._frames[v].popleft()
    self.cleanup({0: (v, [cleanup_frames], '*')})
```

**Buffer Management**:
- Configurable `max_buffer_size` for incremental cleanup
- View-specific memory tracking
- Automatic purging of old frames

## Error Handling and Recovery

### State Recovery Mechanisms

**Hold-Based Recovery**:
- Failed algorithms can delegate processing to downstream components
- Conditional hold release based on data conditions
- Automatic purge protection for held frames

**Version Validation**:
```python
def check_version(alg, log, expect: str):
    n, v, ev = alg.__name__, alg.version().split('.')[0], expect
    assert v == ev, f"expected {n} version {ev} but got {v}"
```

**Process Monitoring**:
- Continuous status monitoring of all algorithms
- Automatic recovery from algorithm failures
- Graceful degradation when components fail

### Data Consistency

**Atomic Operations**:
- All state modifications are atomic
- Queue-based updates ensure consistency
- Transaction-like semantics for multi-frame operations

**Rollback Capabilities**:
- Hold system prevents premature data deletion
- Frame-level recovery possible through hold management
- Conditional processing based on upstream results

## Performance Optimizations

### Caching Strategies

**Recent Frame Optimization**:
```python
# Optimized recent frame lookup
if frame_nbr == 'recent':
    for frame_nbr in range(self._max_frame_nbr[view], 
                          self._min_frame_nbr[view] - 1, -1):
        if frame_nbr in view_data:
            got = view_data[frame_nbr].get(key)
            if got: return view_data[frame_nbr][key].data
```

**Lazy Evaluation**:
- Data only computed when explicitly requested
- Trigger-based computation activation
- View-specific processing to reduce overhead

**Memory Pool Management**:
- Efficient frame number tracking with deques
- Batch operations for cleanup
- Configurable memory limits per view

## Sports-Specific State Extensions

### Basketball-Specific State

**Game State Tracking**:
- Shot detection and tracking state
- Player identification and re-identification
- Jersey number recognition state
- Court localization parameters

**Temporal Dependencies**:
- Shot events trigger downstream analysis (96 frames for shooter identification)
- Possession tracking requires ball state history
- Player tracking maintains identity across occlusions

### Multi-Sport Flexibility

**Configurable Algorithms**:
- Sport-specific algorithm sets via configuration
- View-specific processing (basketball court vs. baseball field)
- Extensible key system for new sports

**State Inheritance**:
- Common base state for all sports
- Sport-specific extensions through configuration
- Shared infrastructure with specialized algorithms

This state management system provides a robust, scalable foundation for complex multi-algorithm sports analytics pipelines, with sophisticated dependency management, error recovery, and performance optimization.