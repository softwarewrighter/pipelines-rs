#!/bin/bash
set -euo pipefail

# Demo: sales-report
cd "$(dirname "$0")/.."

cargo run --bin pipe-run --release -- \
    specs/sales-report.pipe \
    specs/input-fixed-80.data \
    -o work/sample-pipe-outputs/sales-report.out 2>&1
