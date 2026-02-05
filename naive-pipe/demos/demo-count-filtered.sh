#!/bin/bash
set -euo pipefail

# Demo: count-filtered (record-at-a-time)
cd "$(dirname "$0")/.."

cargo run -p naive-pipe --bin pipe-run-rat --release -- \
    ../specs/count-filtered.pipe \
    ../specs/input-fixed-80.data \
    -o work/sample-pipe-outputs/count-filtered.out 2>&1
