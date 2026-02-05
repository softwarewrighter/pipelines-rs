#!/bin/bash
set -euo pipefail

# Demo: nlocate-exclude (record-at-a-time)
cd "$(dirname "$0")/.."

cargo run -p naive-pipe --bin pipe-run-rat --release -- \
    ../specs/nlocate-exclude.pipe \
    ../specs/input-fixed-80.data \
    -o work/sample-pipe-outputs/nlocate-exclude.out 2>&1
