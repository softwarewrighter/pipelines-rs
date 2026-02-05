#!/bin/bash
set -euo pipefail

# Benchmark: Batched vs Record-at-a-Time pipeline execution
#
# Runs each spec file N iterations through both executors, measures wall-clock
# time, and prints a comparison table. Also verifies output equivalence.
#
# Usage:
#   ./scripts/benchmark.sh              # default 100 iterations
#   ./scripts/benchmark.sh 500          # custom iteration count

cd "$(dirname "$0")/.."

ITERATIONS=${1:-100}
SPECS_DIR="specs"
INPUT="specs/input-fixed-80.data"
WORK_DIR="work/benchmark"

echo "=== Batched vs RAT Benchmark ==="
echo "Iterations per spec: $ITERATIONS"
echo

# Build both binaries in release mode
echo "Building release binaries..."
cargo build --bin pipe-run --release 2>/dev/null
cargo build -p naive-pipe --bin pipe-run-rat --release 2>/dev/null

BATCH_BIN="target/release/pipe-run"
RAT_BIN="target/release/pipe-run-rat"

mkdir -p "$WORK_DIR"

# Verify both binaries exist
if [ ! -f "$BATCH_BIN" ]; then
    echo "Error: $BATCH_BIN not found"
    exit 1
fi
if [ ! -f "$RAT_BIN" ]; then
    echo "Error: $RAT_BIN not found"
    exit 1
fi

# Print table header
printf "%-28s %10s %10s %8s %s\n" "Spec" "Batched" "RAT" "Ratio" "Match"
printf "%-28s %10s %10s %8s %s\n" "---" "---" "---" "---" "---"

TOTAL_BATCH=0
TOTAL_RAT=0
MISMATCH=0

for pipe in "$SPECS_DIR"/*.pipe; do
    name=$(basename "$pipe" .pipe)

    # Run batched N times, capture wall-clock time
    batch_start=$(python3 -c 'import time; print(time.monotonic())')
    for ((i = 0; i < ITERATIONS; i++)); do
        "$BATCH_BIN" "$pipe" "$INPUT" -o "$WORK_DIR/batch-${name}.out" 2>/dev/null
    done
    batch_end=$(python3 -c 'import time; print(time.monotonic())')

    # Run RAT N times, capture wall-clock time
    rat_start=$(python3 -c 'import time; print(time.monotonic())')
    for ((i = 0; i < ITERATIONS; i++)); do
        "$RAT_BIN" "$pipe" "$INPUT" -o "$WORK_DIR/rat-${name}.out" 2>/dev/null
    done
    rat_end=$(python3 -c 'import time; print(time.monotonic())')

    # Compute elapsed times in ms
    batch_ms=$(python3 -c "print(f'{($batch_end - $batch_start) * 1000:.1f}')")
    rat_ms=$(python3 -c "print(f'{($rat_end - $rat_start) * 1000:.1f}')")

    TOTAL_BATCH=$(python3 -c "print(f'{$TOTAL_BATCH + ($batch_end - $batch_start) * 1000:.1f}')")
    TOTAL_RAT=$(python3 -c "print(f'{$TOTAL_RAT + ($rat_end - $rat_start) * 1000:.1f}')")

    # Compute ratio (RAT / Batched)
    ratio=$(python3 -c "
b = $batch_end - $batch_start
r = $rat_end - $rat_start
print(f'{r/b:.2f}x' if b > 0 else 'N/A')
")

    # Check output equivalence
    if diff -q "$WORK_DIR/batch-${name}.out" "$WORK_DIR/rat-${name}.out" >/dev/null 2>&1; then
        match="ok"
    else
        match="MISMATCH"
        MISMATCH=$((MISMATCH + 1))
    fi

    printf "%-28s %8s ms %8s ms %8s %s\n" "$name" "$batch_ms" "$rat_ms" "$ratio" "$match"
done

# Print totals
echo
printf "%-28s %8s ms %8s ms\n" "TOTAL" "$TOTAL_BATCH" "$TOTAL_RAT"
TOTAL_RATIO=$(python3 -c "
b = $TOTAL_BATCH
r = $TOTAL_RAT
print(f'{r/b:.2f}x' if b > 0 else 'N/A')
")
printf "%-28s %10s %10s %8s\n" "" "" "" "$TOTAL_RATIO"

echo
if [ "$MISMATCH" -eq 0 ]; then
    echo "All outputs match between batched and RAT executors."
else
    echo "WARNING: $MISMATCH spec(s) produced different output!"
    exit 1
fi

# Cleanup
rm -rf "$WORK_DIR"
