#!/bin/bash
set -euo pipefail

# Demo: count-records (record-at-a-time)
cd "$(dirname "$0")/.."

cargo run -p naive-pipe --bin pipe-run-rat --release -- \
    ../specs/count-records.pipe \
    ../specs/input-fixed-80.data \
    -o work/sample-pipe-outputs/count-records.out 2>&1
