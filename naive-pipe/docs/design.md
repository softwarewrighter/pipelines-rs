# Naive Pipe - Design

## RecordStage Trait

```rust
trait RecordStage {
    /// Process a single input record.
    /// Returns zero or more output records.
    fn process(&mut self, record: Record) -> Vec<Record>;

    /// Called once after all input records are exhausted.
    /// Returns any accumulated output (e.g., COUNT result).
    fn flush(&mut self) -> Vec<Record> {
        vec![]
    }
}
```

## Stage Implementations

### Stateless (1:1 or 1:0-1)

These stages have no mutable state. `process()` returns a single-element vec
or an empty vec:

- **CONSOLE** -- Returns `vec![record]` (pass-through).
- **FILTER =** / **FILTER !=** -- Returns `vec![record]` if the condition
  matches, otherwise `vec![]`.
- **SELECT** -- Extracts a column range, returns `vec![reformatted_record]`.
- **LOCATE** / **NLOCATE** -- String search; returns `vec![record]` or `vec![]`.
- **CHANGE** -- String replacement; returns `vec![modified_record]`.
- **UPPER** / **LOWER** -- Case conversion; returns `vec![modified_record]`.
- **REVERSE** -- Reverses the record content; returns `vec![reversed_record]`.
- **HOLE** -- Always returns `vec![]` (discards input).

### Stateless (1:N)

- **DUPLICATE** -- Returns `vec![record.clone(), record]` (or more copies).

### Stateful (counter-based)

- **TAKE** -- Maintains `records_seen: usize`. Returns `vec![record]` while
  `records_seen < n`, then `vec![]`.
- **SKIP** -- Maintains `records_skipped: usize`. Returns `vec![]` while
  `records_skipped < n`, then `vec![record]`.

### Stateful (accumulator)

- **COUNT** -- Maintains `count: usize`. `process()` increments count and returns
  `vec![]`. `flush()` returns `vec![Record::from(count.to_string())]`.

### Stateful (emit-before-input)

- **LITERAL** -- Maintains `literal_emitted: bool`. On the first call to
  `process()`, returns `vec![literal_record, record]`. On subsequent calls,
  returns `vec![record]`.

## Pipeline Executor

```rust
fn execute(input: Vec<Record>, stages: Vec<Box<dyn RecordStage>>) -> Vec<Record> {
    let mut output = Vec::new();

    for record in input {
        let mut current = vec![record];
        for stage in &mut stages {
            let mut next = Vec::new();
            for rec in current {
                next.extend(stage.process(rec));
            }
            current = next;
        }
        output.extend(current);
    }

    // EOF flush
    let mut current = Vec::new();
    for stage in &mut stages {
        let mut flushed = stage.flush();
        // Flush output must also pass through remaining stages
        // (handled by propagating through the rest of the chain)
        current.extend(flushed);
    }
    output.extend(current);

    output
}
```

Note: The flush propagation needs care -- the output of one stage's `flush()`
must pass through subsequent stages' `process()` methods before those stages
themselves flush.
