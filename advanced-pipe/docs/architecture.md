# Advanced Pipe Architecture

## Fewer-buffers Model

Instead of a buffer at every pipe point, this implementation uses generally
one buffer per pipe. Stages act on the buffer in sequence rather than each
owning separate input and output buffers.

```text
Source --> [ shared buffer ] --> Sink
              ^
              | stages act on buffer in sequence
              Stage1 -> Stage2 -> Stage3
```

## Key Differences from Naive Pipe

| Aspect          | Naive Pipe              | Advanced Pipe              |
|-----------------|-------------------------|----------------------------|
| Buffers         | One per pipe point      | One per pipe (shared)      |
| Record copies   | Copy at each stage      | In-place modification      |
| Stage interface | Owns input, writes out  | Acts on shared buffer      |
| Memory          | O(stages * records)     | O(records)                 |

## How CMS Pipelines Actually Worked

In the real CMS Pipelines implementation, stages operated on records in shared
buffers. A stage would read from the buffer, modify or filter the record, and
the next stage would see the result in the same buffer location. This minimized
memory copies on systems where memory was precious.

This advanced implementation aims to faithfully reproduce that model while
maintaining the same external behavior as the naive-pipe implementation.

## Stage Cardinalities

Same as naive-pipe -- all 14+ commands have identical semantics. Only the
internal buffer management differs.

| Command   | Cardinality         | State needed              |
|-----------|---------------------|---------------------------|
| CONSOLE   | 1:1 (pass-through)  | None                      |
| FILTER =  | 1:0-1               | None                      |
| FILTER != | 1:0-1               | None                      |
| SELECT    | 1:1                 | None                      |
| TAKE      | 1:0-1               | Counter (records_seen)    |
| SKIP      | 1:0-1               | Counter (records_skipped) |
| LOCATE    | 1:0-1               | None                      |
| NLOCATE   | 1:0-1               | None                      |
| COUNT     | N:1                 | Accumulator, EOF flush    |
| CHANGE    | 1:1                 | None                      |
| LITERAL   | 0:1 then 1:1        | Flag (literal_emitted)    |
| UPPER     | 1:1                 | None                      |
| LOWER     | 1:1                 | None                      |
| REVERSE   | 1:1                 | None                      |
| DUPLICATE | 1:N                 | None                      |
| HOLE      | 1:0                 | None                      |
