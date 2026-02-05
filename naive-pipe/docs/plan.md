# Naive Pipe - Implementation Plan

## Phase 1: Core Runtime

- Define the `RecordStage` trait with `process()` and `flush()`.
- Implement the pipeline executor that pushes one record at a time through
  the stage chain.
- Write unit tests for the executor with trivial pass-through stages.

## Phase 2: Port All 14 Commands

Port each DSL command to a `RecordStage` implementation:

1. CONSOLE (pass-through)
2. FILTER = (string match filter)
3. FILTER != (string non-match filter)
4. SELECT (column extraction)
5. TAKE (first N records)
6. SKIP (skip first N records)
7. LOCATE (string search, keep matches)
8. NLOCATE (string search, keep non-matches)
9. COUNT (accumulate count, emit on flush)
10. CHANGE (string replacement)
11. LITERAL (emit literal before input)
12. UPPER (uppercase conversion)
13. LOWER (lowercase conversion)
14. REVERSE (reverse record content)
15. DUPLICATE (emit multiple copies)
16. HOLE (discard all input)

Each command gets its own unit tests verifying single-record and multi-record
behavior.

## Phase 3: Debug Trace Capture

- Add a `DebugTrace` struct that records the state at each pipe point for each
  input record.
- Instrument the pipeline executor to optionally capture traces.
- Write tests verifying trace output for representative pipelines.

## Phase 4: Debugger UI

- Update the WASM debugger to use the record-at-a-time executor.
- Show the current record at each pipe point as it flows through stages.
- Support stepping forward one record at a time.
- Display watch values at each pipe point.
