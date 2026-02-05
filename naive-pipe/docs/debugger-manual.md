# Visual Debugger Manual

## Overview

The Visual Debugger provides record-at-a-time (RAT) inspection of pipeline
execution. You can step through each record as it flows from stage to stage,
set watches to monitor data at specific pipe points, and set breakpoints to
pause execution automatically.

The debugger is available in the **wasm-ui-rat** web interface via the
**Visual Debugger** tab.

## Getting Started

### Opening the Debugger

1. Build and serve the application:
   ```bash
   ./scripts/build.sh
   ./scripts/serve.sh
   ```
2. Open http://localhost:9952 in a browser.
3. Click the **Visual Debugger** tab in the header.

### Loading a Pipeline

Use the **Load...** dropdown in the debugger header to load a pipeline:

- **Examples** -- Select any built-in tutorial pipeline. The pipeline text and
  default input data are loaded, and the debugger initializes automatically
  at step 0.
- **Upload .pipe file...** -- Upload a `.pipe` file from disk. The debugger
  initializes automatically after the file loads.

You can also type or paste pipeline text directly in the Pipeline Editor tab
and switch to the Visual Debugger tab. Click **Run** to initialize.

### Interface Layout

When the debugger is active, three panels are visible:

| Panel | Purpose |
|-------|---------|
| **Input Records** (left) | Edit input data fed to the pipeline |
| **Visual Debugger** (center) | Stage list, pipe points, watches, controls |
| **Output Records** (right) | Progressive output as records reach the sink |

## Controls

The debugger header contains these controls:

| Control | Action |
|---------|--------|
| **Load...** | Load an example or upload a `.pipe` file |
| **Run** | Initialize the debugger (first click) or continue execution (subsequent clicks). Stops at the next breakpoint or the end. |
| **Step** | Advance one pipe point. Always advances exactly one step regardless of breakpoints. |
| **Reset** | Return to step 0 without changing the pipeline. Watches and breakpoints are preserved. |
| **Step counter** | Shows current position: `Record 2 of 8 (1/3)` or `Flush 1 of 2 (1/1)`. Prefixed with `[BP]` when paused at a breakpoint. |

## Stage List

When the debugger is active, the center panel shows the full pipeline as a
vertical list of stages. Between each pair of stages is a **pipe point** --
the connection where records flow from one stage to the next.

### Stage Highlighting

- **Green left border** -- Stage has been reached by the current record.
- **Dim** -- Stage has not yet been reached.

### Pipe Points

Each pipe point row shows:

- Two icons for **watch** and **breakpoint** (see below).
- Watch labels (e.g., `w1`, `w2`) if watches are set at this point.
- A data summary showing the record(s) passing through (e.g., `1 rec: SMITH...`),
  or `...` if not yet reached.

## Stepping Model

The debugger steps at **pipe-point granularity**:

1. Each **Step** reveals the next pipe point in the current record's journey.
2. When a record is filtered (empty pipe point), stepping stops at that point
   and the next step moves to the next input record.
3. After all input records, stepping continues through **flush traces** for
   stages that accumulate state (e.g., COUNT).
4. As records reach the final stage (sink), they appear progressively in the
   **Output Records** panel.

### Step Counter Format

```
Record 2 of 8 (1/3)
```

- **Record 2 of 8** -- Processing the 2nd input record out of 8 total.
- **(1/3)** -- At pipe point 1 of 3 for this record.

During flush:
```
Flush 1 of 2 (1/1)
```

## Watches

Watches monitor data at a specific pipe point across all stepping.

### Adding a Watch

Click the watch icon (circled w) on any pipe point. The icon turns
**gold** when a watch is active.

### Removing a Watch

- Click the **gold watch icon** again to toggle it off, or
- Click the trash icon next to the watch in the **Watches** panel below
  the stage list.

### Watch Panel

The Watches panel (below the stage list) shows each watch with:

- **Label** (e.g., `w1`) and location (e.g., `after LOCATE -> CHANGE`).
- **Record data** at that pipe point for the current step.
- A delete button to remove the watch.

Watches persist across **Reset** and **Run** (re-initialization). They are
cleared when loading a new pipeline. Out-of-range watches are automatically
removed if the new pipeline has fewer stages.

## Breakpoints

Breakpoints pause execution at a specific pipe point when using **Run**.

### Setting a Breakpoint

Click the breakpoint icon (circled B) on any pipe point. The icon turns
**red** when a breakpoint is active.

### Removing a Breakpoint

Click the **red breakpoint icon** again to toggle it off.

### How Breakpoints Work

- **Run** (continue): Execution advances until a breakpoint is hit or the
  pipeline finishes. When a breakpoint is hit, the pipe point row is
  highlighted with a red background and the step counter shows a `[BP]`
  prefix.
- **Step**: Always advances exactly one pipe point, ignoring breakpoints.
  This lets you step past a breakpoint without removing it.
- **Reset**: Clears the breakpoint-hit state but preserves all breakpoint
  positions. Clicking **Run** after reset will stop at the first breakpoint
  again.

### Breakpoint Indicators

| State | Breakpoint icon color | Pipe point row |
|-------|----------------------|----------------|
| No breakpoint | Gray (`#555`) | Normal |
| Breakpoint set | Red (`#ff4444`) | Normal |
| Breakpoint hit (paused) | Red | Red background highlight, red left border |

### Typical Breakpoint Workflow

1. Load a pipeline (or click **Run** to initialize).
2. Click the breakpoint icon on the pipe point(s) you want to inspect.
3. Click **Run** -- execution pauses at the first breakpoint.
4. Inspect the watch panel and pipe point data.
5. Click **Run** again to continue to the next breakpoint or the end.
6. Use **Step** to advance one pipe point at a time past a breakpoint.
7. Click **Reset** to start over (breakpoints remain set).

## Keyboard Reference

Currently the debugger is mouse-driven. Keyboard shortcuts are planned for a
future release.

## Examples

### Stepping Through a Filter Pipeline

```
PIPE CONSOLE
| LOCATE /SALES/
| CHANGE /SALES/REVENUE/
| CONSOLE
?
```

1. Load the pipeline and click **Run** or select from the Load dropdown.
2. Click **Step** to see the first record enter the LOCATE stage.
3. Step again to see if it passes (SALES match) or is filtered (no match).
4. If it passes, step to see the CHANGE stage transform it.
5. Step to see the record reach CONSOLE (output).
6. Continue stepping for each input record.

### Using Watches to Compare Before/After

1. Add a watch before the CHANGE stage (between LOCATE and CHANGE).
2. Add a watch after the CHANGE stage (between CHANGE and CONSOLE).
3. Step through records to see the original and transformed data side by side
   in the watch panel.

### Breaking at a Specific Stage

1. Set a breakpoint between LOCATE and CHANGE.
2. Click **Run** -- execution pauses each time a record passes LOCATE.
3. Filtered records (no SALES match) skip past without triggering the
   breakpoint.
4. Click **Run** to continue to the next matching record.

## Related Documentation

- [Architecture](architecture.md) -- Buffer-per-pipe-point model
- [Design](design.md) -- RecordStage trait and executor design
- [Plan](plan.md) -- Implementation phases
- [User Manual](../../docs/user-manual.md) -- Pipeline syntax reference
