#!/bin/bash
set -euo pipefail

# Demo: sales-report (record-at-a-time)
cd "$(dirname "$0")/.."

cargo run -p naive-pipe --bin pipe-run-rat --release -- \
    ../specs/sales-report.pipe \
    ../specs/input-fixed-80.data \
    -o work/sample-pipe-outputs/sales-report.out 2>&1
