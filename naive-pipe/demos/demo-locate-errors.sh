#!/bin/bash
set -euo pipefail

# Demo: locate-errors (record-at-a-time)
cd "$(dirname "$0")/.."

cargo run -p naive-pipe --bin pipe-run-rat --release -- \
    ../specs/locate-errors.pipe \
    ../specs/input-fixed-80.data \
    -o work/sample-pipe-outputs/locate-errors.out 2>&1
