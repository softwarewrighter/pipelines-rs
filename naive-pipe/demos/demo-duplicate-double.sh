#!/bin/bash
set -euo pipefail

# Demo: duplicate-double (record-at-a-time)
cd "$(dirname "$0")/.."

cargo run -p naive-pipe --bin pipe-run-rat --release -- \
    ../specs/duplicate-double.pipe \
    ../specs/input-fixed-80.data \
    -o work/sample-pipe-outputs/duplicate-double.out 2>&1
