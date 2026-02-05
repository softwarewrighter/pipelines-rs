#!/bin/bash
set -euo pipefail

# Demo: multi-locate-select (record-at-a-time)
cd "$(dirname "$0")/.."

cargo run -p naive-pipe --bin pipe-run-rat --release -- \
    ../specs/multi-locate-select.pipe \
    ../specs/input-fixed-80.data \
    -o work/sample-pipe-outputs/multi-locate-select.out 2>&1
