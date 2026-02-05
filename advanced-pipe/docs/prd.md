# Advanced Pipe - Product Requirements

## Goal

Implement a CMS-faithful pipeline execution model that uses fewer buffers and
in-place record modification. This provides the same user-visible behavior as
naive-pipe but with an optimized internal representation closer to how CMS
PIPELINES actually worked.

## Requirements

### Core Runtime

- Same external behavior as naive-pipe: identical input produces identical
  output for all 14 commands.
- Internally uses shared buffers rather than per-pipe-point buffers.
- Stages act on the buffer in sequence rather than owning separate I/O buffers.
- Fewer memory copies than naive-pipe.

### Debugger Integration

- Debugger shows buffer state after each stage acts.
- Users can observe how stages modify the shared buffer in-place.
- Same stepping and watch capabilities as naive-pipe debugger.

### Commands

All 14 existing DSL commands with identical semantics to naive-pipe:

CONSOLE, FILTER (= and !=), SELECT, TAKE, SKIP, LOCATE, NLOCATE, COUNT,
CHANGE, LITERAL, UPPER, LOWER, REVERSE, DUPLICATE, HOLE.
