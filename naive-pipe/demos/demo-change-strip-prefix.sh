#!/bin/bash
set -euo pipefail

# Demo: change-strip-prefix (record-at-a-time)
cd "$(dirname "$0")/.."

cargo run -p naive-pipe --bin pipe-run-rat --release -- \
    ../specs/change-strip-prefix.pipe \
    ../specs/input-fixed-80.data \
    -o work/sample-pipe-outputs/change-strip-prefix.out 2>&1
