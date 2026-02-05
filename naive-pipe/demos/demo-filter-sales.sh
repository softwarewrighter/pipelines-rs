#!/bin/bash
set -euo pipefail

# Demo: filter-sales (record-at-a-time)
cd "$(dirname "$0")/.."

cargo run -p naive-pipe --bin pipe-run-rat --release -- \
    ../specs/filter-sales.pipe \
    ../specs/input-fixed-80.data \
    -o work/sample-pipe-outputs/filter-sales.out 2>&1
