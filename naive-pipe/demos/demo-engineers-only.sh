#!/bin/bash
set -euo pipefail

# Demo: engineers-only (record-at-a-time)
cd "$(dirname "$0")/.."

cargo run -p naive-pipe --bin pipe-run-rat --release -- \
    ../specs/engineers-only.pipe \
    ../specs/input-fixed-80.data \
    -o work/sample-pipe-outputs/engineers-only.out 2>&1
