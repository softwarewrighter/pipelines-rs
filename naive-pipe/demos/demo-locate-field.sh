#!/bin/bash
set -euo pipefail

# Demo: locate-field (record-at-a-time)
cd "$(dirname "$0")/.."

cargo run -p naive-pipe --bin pipe-run-rat --release -- \
    ../specs/locate-field.pipe \
    ../specs/input-fixed-80.data \
    -o work/sample-pipe-outputs/locate-field.out 2>&1
