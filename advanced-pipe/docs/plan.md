# Advanced Pipe - Implementation Plan

## Dependency

This implementation depends on naive-pipe being complete. The naive-pipe
implementation establishes the correct semantics for all commands; advanced-pipe
will refactor the internals without changing external behavior.

## Phase 1: Design Finalization

- Study the completed naive-pipe implementation.
- Finalize the `BufferStage` trait and `PipeBuffer` struct.
- Document the exact buffer lifecycle and compaction strategy.

## Phase 2: Core Runtime

- Implement `PipeBuffer` with shared buffer management.
- Implement the pipeline executor using shared buffers.
- Verify equivalence with naive-pipe using the same test suite.

## Phase 3: Port All Commands

- Refactor each `RecordStage` implementation to a `BufferStage` implementation.
- Run the naive-pipe test suite against advanced-pipe to verify identical
  behavior.

## Phase 4: Debugger Integration

- Update the debugger to show buffer state after each stage acts.
- Show in-place modifications visually (highlight changed fields).
- Verify debugger behavior matches naive-pipe debugger output.
