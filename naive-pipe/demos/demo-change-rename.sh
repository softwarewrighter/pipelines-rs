#!/bin/bash
set -euo pipefail

# Demo: change-rename (record-at-a-time)
cd "$(dirname "$0")/.."

cargo run -p naive-pipe --bin pipe-run-rat --release -- \
    ../specs/change-rename.pipe \
    ../specs/input-fixed-80.data \
    -o work/sample-pipe-outputs/change-rename.out 2>&1
