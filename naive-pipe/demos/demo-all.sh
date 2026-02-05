#!/bin/bash
set -euo pipefail

# Run all pipeline demos using the record-at-a-time executor
# Outputs go to ./work/sample-pipe-outputs/

cd "$(dirname "$0")/.."

echo "=== Running all RAT pipeline demos ==="
echo

# Build the CLI first
cargo build -p naive-pipe --bin pipe-run-rat --release 2>/dev/null

# Create output directory
mkdir -p work/sample-pipe-outputs

# Run each demo
for pipe in ../specs/*.pipe; do
    name=$(basename "$pipe" .pipe)
    echo "Running: $name"
    ./demos/demo-${name}.sh
done

echo
echo "=== All RAT demos complete ==="
echo "Outputs in: naive-pipe/work/sample-pipe-outputs/"
