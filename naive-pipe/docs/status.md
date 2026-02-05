# Naive Pipe - Status

## Current Status

**Last Updated**: 2026-02-04

### Phase Progress

| Phase | Status | Progress |
|-------|--------|----------|
| Phase 1: Core Runtime | Complete | 100% |
| Phase 2: Port All 14 Commands | Complete | 100% |
| Phase 3: Debug Trace Capture | Complete | 100% |
| Phase 4: Debugger UI | Complete | 100% |

The record-at-a-time executor and all 14 DSL commands are implemented in the
main `pipelines-rs` library (`src/executor.rs`). The WASM debugger UI lives
in `wasm-ui-rat/`.

## Debugger Features

- [x] Pipeline flow visualization with stage-by-stage execution
- [x] Per-pipe-point stepping (record and flush phases)
- [x] Progressive output (records appear as they reach the sink)
- [x] Watch points with toggle on/off and data inspection
- [x] Breakpoints with Run-stops-at-BP and visual indicators
- [x] Load dropdown (examples + file upload) with auto-initialization
- [x] Reset preserves watches and breakpoints
- [x] Step counter with `[BP]` prefix when paused at breakpoint

## What's Working

- Full record-at-a-time execution with debug trace capture
- Visual debugger tab in wasm-ui-rat
- Step, Run, Reset controls
- Watch panel showing record data at any pipe point
- Breakpoints that pause Run at specific pipe points
- Load dropdown auto-initializes the debugger (no extra Run click needed)
- Watch icon uses color-based visibility (gold when active)
- Breakpoint icon with red indicator and BP-hit row highlighting

## Related Documentation

- [Debugger Manual](debugger-manual.md) - How to use the visual debugger
- [Architecture](architecture.md) - Buffer-per-pipe-point model
- [Design](design.md) - RecordStage trait
- [Plan](plan.md) - Implementation phases
