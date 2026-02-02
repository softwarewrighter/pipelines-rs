#!/bin/bash
set -euo pipefail

# Run all pipeline demos
# Outputs go to ./work/sample-pipe-outputs/

cd "$(dirname "$0")/.."

echo "=== Running all pipeline demos ==="
echo

# Build the CLI first
cargo build --bin pipe-run --release 2>/dev/null

# Create output directory
mkdir -p work/sample-pipe-outputs

# Run each demo
for pipe in specs/*.pipe; do
    name=$(basename "$pipe" .pipe)
    echo "Running: $name"
    ./demos/demo-${name}.sh
done

echo
echo "=== All demos complete ==="
echo "Outputs in: work/sample-pipe-outputs/"
