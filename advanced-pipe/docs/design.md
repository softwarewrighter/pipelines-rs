# Advanced Pipe - Design

## Placeholder

This design will be fleshed out after naive-pipe is complete. The key
differences from naive-pipe are documented here for reference.

## Key Differences from Naive Pipe

### Shared Buffer

Instead of each stage owning input and output buffers, stages share a common
buffer. A stage reads from the buffer, modifies the record in place (or marks
it for removal), and the next stage sees the result.

```rust
// Naive pipe: each stage owns I/O
fn process(&mut self, record: Record) -> Vec<Record>;

// Advanced pipe: stage acts on shared buffer
fn act(&mut self, buffer: &mut PipeBuffer);
```

### In-place Modification

Stages that transform records (UPPER, LOWER, CHANGE, REVERSE) modify the
record in the buffer directly rather than creating a new record. This reduces
allocation and copying.

### Filtering

Stages that filter records (FILTER, LOCATE, TAKE, SKIP) mark records for
removal in the buffer rather than omitting them from output. A compaction
step removes marked records between stage passes.

### Accumulation

Stages like COUNT that accumulate across records maintain internal state and
replace the buffer contents on EOF flush, same as naive-pipe but operating
on the shared buffer.

## Stage Interface (Draft)

```rust
trait BufferStage {
    /// Act on the shared buffer. May modify, remove, or add records.
    fn act(&mut self, buffer: &mut PipeBuffer);

    /// Called on EOF. May replace buffer contents (e.g., COUNT).
    fn flush(&mut self, buffer: &mut PipeBuffer) {}
}
```

## PipeBuffer (Draft)

```rust
struct PipeBuffer {
    records: Vec<Option<Record>>,  // None = removed by filter
}
```

The exact design will be refined during implementation.
