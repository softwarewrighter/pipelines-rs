#!/bin/bash
set -euo pipefail

# Demo: multi-filter-count (record-at-a-time)
cd "$(dirname "$0")/.."

cargo run -p naive-pipe --bin pipe-run-rat --release -- \
    ../specs/multi-filter-count.pipe \
    ../specs/input-fixed-80.data \
    -o work/sample-pipe-outputs/multi-filter-count.out 2>&1
