#!/bin/bash
set -euo pipefail

# Demo: lower-case (record-at-a-time)
cd "$(dirname "$0")/.."

cargo run -p naive-pipe --bin pipe-run-rat --release -- \
    ../specs/lower-case.pipe \
    ../specs/input-fixed-80.data \
    -o work/sample-pipe-outputs/lower-case.out 2>&1
