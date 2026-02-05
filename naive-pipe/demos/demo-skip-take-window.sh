#!/bin/bash
set -euo pipefail

# Demo: skip-take-window (record-at-a-time)
cd "$(dirname "$0")/.."

cargo run -p naive-pipe --bin pipe-run-rat --release -- \
    ../specs/skip-take-window.pipe \
    ../specs/input-fixed-80.data \
    -o work/sample-pipe-outputs/skip-take-window.out 2>&1
