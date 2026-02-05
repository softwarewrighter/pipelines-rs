# Naive Pipe Architecture

## Buffer-per-pipe-point Model

Each pipe point (the connection between two stages) has its own buffer. The
execution model is record-at-a-time:

1. The **source** produces one record.
2. Each **stage** receives the record from its input buffer, processes it, and
   places zero or more result records in its output buffer.
3. The **sink** consumes the final record(s).
4. Repeat for the next input record until EOF.

```text
Source --> [buf] --> Stage1 --> [buf] --> Stage2 --> [buf] --> Sink
```

## Record Flow

A single input record flows through the entire stage chain before the next
input record is read. This makes the data flow easy to visualize and debug:
at any point in time, you can see exactly which record is at which pipe point.

## EOF Handling

When the source is exhausted, an EOF signal propagates through the chain. Stages
that accumulate state (e.g., COUNT) use the `flush()` method to emit their
final output on EOF.

## Stage Cardinalities

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
