# Comparing Pipeline Implementations

This document compares the two pipeline executor implementations in
pipelines-rs: **batch (stage-at-a-time)** and **RAT (record-at-a-time)**.

## Execution Models

### Batch (stage-at-a-time)

The original executor. All input records are collected into a `Vec<Record>`,
passed through stage 1, the entire output collected, then passed through
stage 2, and so on. Each stage sees the complete record set at once.

Entry point: `execute_pipeline()`

### RAT (record-at-a-time)

Each input record flows through the entire stage chain (stage 1 -> stage 2
-> ... -> stage N) before the next input record is read. After all records
are processed, stages are flushed in order to emit accumulated results
(e.g. COUNT).

Entry point: `execute_pipeline_rat()`

## Correctness

Both executors produce identical output for all supported pipelines.
This is enforced by 23 equivalence tests (one per spec file in `specs/`)
that run the same input and pipeline through both executors and assert
equal output.

## Performance

Benchmarked on Apple Silicon (M-series), `--release` build, averaged over
10-50 iterations per configuration.

### Pipeline 1: LOCATE + CHANGE + UPPER + DUPLICATE 2 + TAKE 50000

A 6-stage pipeline with record expansion (DUPLICATE doubles each record).

```
Records     Batch (us)    RAT (us)    Ratio
-------     ----------    --------    -----
  1,000            246         619     2.5x
 10,000          1,955       5,610     2.9x
100,000         15,509      42,225     2.7x
```

### Pipeline 2: FILTER + COUNT

A 2-stage pipeline exercising the flush path (COUNT accumulates during
processing, emits a single summary on flush).

```
Records     Batch (us)    RAT (us)    Ratio
-------     ----------    --------    -----
  1,000             30          64     2.1x
 10,000            309         626     2.0x
100,000          4,122       7,465     1.8x
```

### Pipeline 3: NLOCATE + LOCATE + CHANGE + LOWER + REVERSE

A 5-stage pure transform/filter chain with no record expansion.

```
Records     Batch (us)    RAT (us)    Ratio
-------     ----------    --------    -----
  1,000            290         598     2.1x
 10,000          2,921       5,849     2.0x
100,000         29,751      62,046     2.1x
```

### Summary

| Metric | Batch | RAT |
|---|---|---|
| Typical overhead | 1x (baseline) | 2-3x |
| Worst case (DUPLICATE) | 1x | ~2.9x |
| Best case (FILTER+COUNT) | 1x | ~1.8x |
| Scaling | Linear | Linear |

Both executors scale linearly with input size. The constant-factor gap
stays stable as dataset size grows.

## Why RAT is Slower

The overhead comes from per-record dynamic dispatch and allocation:

- **Vec allocation per record per stage**: Each record flowing through N
  stages creates N small `Vec<Record>` results. Batch processes entire
  vectors through each stage using bulk iterator operations.
- **Dynamic dispatch**: `Box<dyn RecordStage>` requires a vtable lookup
  per record per stage. Batch uses monomorphized closures via the
  `Pipeline` iterator adapter.
- **Record expansion amplifies cost**: Stages like DUPLICATE that produce
  multiple output records per input compound the per-record overhead,
  explaining the higher ratio (~2.9x) for those pipelines.

## Design Tradeoff

RAT exists for **debugger observability**, not throughput. The
record-at-a-time model lets the debugger UI show exactly how each
individual record flows through the pipeline, with pipe-point snapshots
between every stage. This is not possible with the batch executor, which
only captures before/after snapshots per stage.

For production processing of large datasets, use the batch executor.
For interactive debugging and visualization, use the RAT executor.

## Correctness Contract

Same input + same pipeline = same output, regardless of executor.
This invariant is tested and must hold for any future executor
implementation as well.
