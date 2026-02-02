# Development Plan

## Overview

This document outlines the implementation plan for pipelines-rs, broken into milestones with specific deliverables.

## Current Phase

**Phase**: CLI and Web UI Complete
**Status**: CLI, demos, and tutorial system functional

## Completed Work

### Milestone 1: Core Pipeline (Foundation) - COMPLETE

- [x] Project structure setup
  - [x] Cargo workspace (library + wasm-ui)
  - [x] Module organization
  - [x] Rust 2024 edition
- [x] Core types
  - [x] `Record` type (80-byte fixed-width)
  - [x] `Pipeline` struct with fluent API
  - [x] Iterator-based data flow
- [x] Basic stages
  - [x] `Filter` stage (= and != operators)
  - [x] `Select` stage (field extraction/repositioning)
  - [x] `Take` stage (limit records)
  - [x] `Skip` stage (skip records)
  - [x] `Map` stage (transform records)
  - [x] `Inspect` stage (debugging)
- [x] Web UI
  - [x] Yew/WASM application
  - [x] Three-panel layout (Input, Pipeline, Output)
  - [x] DSL parser for pipeline text
  - [x] Load/Save pipeline files
  - [x] 80-column display with ruler
  - [x] Interactive tutorial system with auto-run mode
  - [x] Clear button for output panel
- [x] Tests
  - [x] 59 tests (33 unit + 26 doc tests)
  - [x] Zero clippy warnings
- [x] Documentation
  - [x] API documentation
  - [x] User manual with examples

### Milestone 2: CLI Interface - COMPLETE

- [x] DSL module in main library
  - [x] `execute_pipeline()` function exposed
  - [x] Full DSL parser (FILTER, SELECT, TAKE, SKIP, LOCATE, COUNT, etc.)
- [x] `pipe-run` CLI binary
  - [x] Run .pipe files against input data
  - [x] Output to file or stdout
  - [x] Record count statistics
- [x] Demo scripts
  - [x] Individual `demo-<name>.sh` for each .pipe file
  - [x] `demo-all.sh` to run all demos
  - [x] Outputs to `work/sample-pipe-outputs/`
- [x] Sample data
  - [x] `specs/input-fixed-80.data` with 8 employee records
  - [x] 23 sample .pipe files in `specs/`

---

## Next Steps

### Immediate (High Priority)

#### Labels and Multiple Streams

Add support for labeled stages and stream branching:

```
PIPE CONSOLE
| a: FILTER 18,10 = "SALES"
| b: SELECT 0,8,0; 28,8,8
| CONSOLE
?
```

**Tasks**:
- [ ] Add label syntax (`label:` prefix) to DSL parser
- [ ] Store labels in parsed Command struct
- [ ] Display labels in UI (for debugging)

#### SPLIT Stage

Route records to different outputs based on conditions:

```
PIPE CONSOLE
| SPLIT 18,10
|   = "SALES": sales_output
|   = "ENGINEER": eng_output
|   OTHERWISE: other_output
?
```

**Tasks**:
- [ ] Design SPLIT syntax
- [ ] Implement SPLIT stage in DSL
- [ ] Add multi-output support to pipeline executor
- [ ] Update UI to show split outputs

#### MERGE Stage

Combine multiple sorted streams:

```
PIPE (
  CONSOLE | SORT 0,8
  ?
  FILE sales.dat | SORT 0,8
)
| MERGE 0,8
| CONSOLE
?
```

**Tasks**:
- [ ] Design MERGE syntax
- [ ] Implement sorted merge algorithm
- [ ] Add multi-input pipeline support
- [ ] Consider memory-efficient streaming merge

#### SORT Stage

Sort records by field:

```
PIPE CONSOLE
| SORT 28,8 DESC
| CONSOLE
?
```

**Tasks**:
- [ ] Implement SORT stage
- [ ] Support ASC/DESC order
- [ ] Support multiple sort keys
- [ ] Consider external sort for large datasets

---

### Short Term (Medium Priority)

#### Debugging Controls

Add ability to inspect data at each stage:

**Tasks**:
- [ ] Add stage-by-stage execution mode
- [ ] Show intermediate results between stages
- [ ] Add record count at each stage
- [ ] Highlight current stage in pipeline editor
- [ ] Add breakpoints (pause at specific stage)

#### Stage Inspector Panel

New UI panel showing data flow:

```
[Input: 8 records]
    ↓
[FILTER: 3 records passed, 5 filtered]
    ↓
[SELECT: 3 records transformed]
    ↓
[Output: 3 records]
```

**Tasks**:
- [ ] Design inspector panel layout
- [ ] Track record counts per stage
- [ ] Show sample records at each stage
- [ ] Add expand/collapse for stage details

#### Additional FILTER Operators

Extend FILTER with more comparison options:

```
FILTER 28,8 > "00050000"     # Greater than
FILTER 28,8 < "00070000"     # Less than
FILTER 28,8 >= "00050000"    # Greater or equal
FILTER 28,8 <= "00070000"    # Less or equal
FILTER 0,8 CONTAINS "SMI"    # Contains substring
FILTER 0,8 STARTSWITH "S"    # Starts with
```

**Tasks**:
- [ ] Add numeric comparison operators
- [ ] Add string operators (CONTAINS, STARTSWITH, ENDSWITH)
- [ ] Update DSL parser
- [ ] Add tests for new operators

---

### Medium Term (Lower Priority)

#### File I/O Stages

Add file-based sources and sinks:

```
PIPE FILE input.dat
| FILTER 18,10 = "SALES"
| FILE output.dat
?
```

**Tasks**:
- [ ] Add FILE stage for reading
- [ ] Add FILE stage for writing
- [ ] Handle file errors gracefully
- [ ] Support relative and absolute paths

#### REFORMAT Stage

Create new records with literal text and field references:

```
PIPE CONSOLE
| REFORMAT "Name: " 0,8 " Salary: $" 28,8
| CONSOLE
?
```

**Tasks**:
- [ ] Design REFORMAT syntax
- [ ] Implement string concatenation with field refs
- [ ] Support escape sequences
- [ ] Add padding/alignment options

#### COUNT and STATS Stages

Aggregate operations:

```
PIPE CONSOLE
| COUNT                     # Output: record count
?

PIPE CONSOLE
| STATS 28,8                # Output: min, max, sum, avg
?
```

**Tasks**:
- [ ] Implement COUNT stage
- [ ] Implement STATS stage for numeric fields
- [ ] Consider GROUP BY functionality

#### Keyboard Shortcuts

Improve UI productivity:

- [ ] Ctrl+Enter to run pipeline
- [ ] Ctrl+S to save pipeline
- [ ] Ctrl+O to load pipeline
- [ ] F5 to run, F6 to step

---

### Long Term (Future Consideration)

#### Parallel Execution

Process records in parallel for performance:

- [ ] Design parallel execution model
- [ ] Implement parallel filter/map stages
- [ ] Add thread pool configuration
- [ ] Measure and optimize performance

#### External Data Sources

Connect to databases and APIs:

- [ ] Database source (SQL query)
- [ ] HTTP/REST source
- [ ] Message queue integration

#### Pipeline Composition

Reusable pipeline fragments:

```
INCLUDE common-filters.pipe
PIPE CONSOLE
| CALL validate_record
| CONSOLE
?
```

#### Visual Pipeline Editor

Drag-and-drop pipeline construction:

- [ ] Node-based visual editor
- [ ] Generate DSL from visual layout
- [ ] Parse DSL to visual layout

---

## Task Tracking

### Current Sprint

| Task | Status | Notes |
|------|--------|-------|
| CLI binary | Complete | `pipe-run` command |
| Demo scripts | Complete | 24 scripts in `demos/` |
| Tutorial system | Complete | Auto-run mode with countdown |
| Labels | Not Started | High priority |
| SPLIT stage | Not Started | High priority |
| Debug controls | Not Started | Medium priority |

### Backlog (Prioritized)

1. Labels for stages
2. SPLIT stage (conditional routing)
3. MERGE stage (combine streams)
4. SORT stage
5. Debug inspector panel
6. Additional FILTER operators
7. REFORMAT stage with field refs
8. Keyboard shortcuts
9. CI/CD pipeline

---

## Risk Register

| Risk | Likelihood | Impact | Mitigation |
|------|------------|--------|------------|
| Complex SPLIT/MERGE syntax | Medium | High | Prototype early, get feedback |
| Performance with large files | Low | Medium | Streaming design, external sort |
| UI complexity with debug features | Medium | Medium | Progressive disclosure |

## Quality Standards

All code must meet these criteria before merge:
- All tests pass (`cargo test`)
- Zero clippy warnings (`cargo clippy -- -D warnings`)
- Formatted (`cargo fmt`)
- Documented (public items)
- User manual updated for new features

## Related Documentation

- [Architecture](architecture.md) - System design
- [Product Requirements](prd.md) - Feature requirements
- [Design Document](design.md) - Technical decisions
- [Status](status.md) - Current progress
- [User Manual](user-manual.md) - Usage guide
