#!/bin/bash
set -euo pipefail

# Demo: non-marketing (record-at-a-time)
cd "$(dirname "$0")/.."

cargo run -p naive-pipe --bin pipe-run-rat --release -- \
    ../specs/non-marketing.pipe \
    ../specs/input-fixed-80.data \
    -o work/sample-pipe-outputs/non-marketing.out 2>&1
