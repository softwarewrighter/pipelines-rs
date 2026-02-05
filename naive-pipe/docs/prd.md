# Naive Pipe - Product Requirements

## Goal

Implement a record-at-a-time pipeline execution model where each record flows
through all stages before the next record is read. This model prioritizes
clarity and debuggability over performance.

## Requirements

### Core Runtime

- Pipeline executes one record at a time through the full stage chain.
- Each stage processes a single record and produces zero or more output records.
- Stages that accumulate state (COUNT) flush their results on EOF.
- LITERAL emits its own record before passing the first input record through.

### Debugger Integration

- The debugger can observe the record at each pipe point as it passes through.
- Watches show the current record at each stage boundary.
- Users can step one record at a time through the pipeline.
- The stage chain and record contents are visible at every step.

### Commands

All 14 existing DSL commands must be supported with identical semantics to the
current batch-mode implementation:

CONSOLE, FILTER (= and !=), SELECT, TAKE, SKIP, LOCATE, NLOCATE, COUNT,
CHANGE, LITERAL, UPPER, LOWER, REVERSE, DUPLICATE, HOLE.
