#!/bin/bash
set -euo pipefail

# Demo: filter-sales
cd "$(dirname "$0")/.."

cargo run --bin pipe-run --release -- \
    specs/filter-sales.pipe \
    specs/input-fixed-80.data \
    -o work/sample-pipe-outputs/filter-sales.out 2>&1
