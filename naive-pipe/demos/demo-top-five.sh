#!/bin/bash
set -euo pipefail

# Demo: top-five (record-at-a-time)
cd "$(dirname "$0")/.."

cargo run -p naive-pipe --bin pipe-run-rat --release -- \
    ../specs/top-five.pipe \
    ../specs/input-fixed-80.data \
    -o work/sample-pipe-outputs/top-five.out 2>&1
