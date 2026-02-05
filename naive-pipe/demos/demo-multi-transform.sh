#!/bin/bash
set -euo pipefail

# Demo: multi-transform (record-at-a-time)
cd "$(dirname "$0")/.."

cargo run -p naive-pipe --bin pipe-run-rat --release -- \
    ../specs/multi-transform.pipe \
    ../specs/input-fixed-80.data \
    -o work/sample-pipe-outputs/multi-transform.out 2>&1
